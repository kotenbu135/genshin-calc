# 設計: ステータス計算のWASM移管 (`build_stats`)

> Date: 2026-03-31
> Scope: `genshin-calc` — crates/core, crates/good, crates/wasm

## 概要

UI側の `buildStats()` (TypeScript) をWASM側に移管し、`CharacterBuild` → `Stats` 変換をRustで行う。併せて `Stats` / `StatProfile` を元素別DMGボーナスに対応させ、物理DMG%と元素DMG%の混同バグを修正する。

## 背景と動機

### 現状の問題

1. **物理DMGと元素DMGの混同**: `Stats.dmg_bonus` に物理DMG%も元素DMG%も統合されており、物理DMGキャラ（Razor, Eula等）の元素ダメージ計算に物理DMG%が誤って乗る
2. **異元素ダメージの非対応**: ファルカ（風元素）が炎付与で攻撃する場合など、自身の元素DMG%が異なる元素のダメージに誤って適用される
3. **UI側での計算**: ステータス組み立てがTypeScript側にあり、武器サブステのDMG%未反映などのバグが存在

### ゴール

- `Stats` で元素別DMGボーナスを正確にトラッキング
- `build_stats()` WASM関数でUI側の計算ロジックを不要にする
- 既存の `resolve_team_stats()` も自動的に恩恵を受ける

## 設計

### 1. `Stats` 構造体の拡張

```rust
pub struct Stats {
    pub hp: f64,
    pub atk: f64,
    pub def: f64,
    pub elemental_mastery: f64,
    pub crit_rate: f64,
    pub crit_dmg: f64,
    pub energy_recharge: f64,
    pub dmg_bonus: f64,              // 汎用DMGバフ（全ダメージに乗る）
    // 元素別DMGボーナス
    pub pyro_dmg_bonus: f64,
    pub hydro_dmg_bonus: f64,
    pub electro_dmg_bonus: f64,
    pub cryo_dmg_bonus: f64,
    pub dendro_dmg_bonus: f64,
    pub anemo_dmg_bonus: f64,
    pub geo_dmg_bonus: f64,
    pub physical_dmg_bonus: f64,
}
```

`StatProfile` にも同じ8フィールドを追加。

### 2. DMGボーナス解決ヘルパー

```rust
impl Stats {
    pub fn total_dmg_bonus(&self, element: Option<Element>) -> f64 {
        self.dmg_bonus + match element {
            Some(Element::Pyro) => self.pyro_dmg_bonus,
            Some(Element::Hydro) => self.hydro_dmg_bonus,
            Some(Element::Electro) => self.electro_dmg_bonus,
            Some(Element::Cryo) => self.cryo_dmg_bonus,
            Some(Element::Dendro) => self.dendro_dmg_bonus,
            Some(Element::Anemo) => self.anemo_dmg_bonus,
            Some(Element::Geo) => self.geo_dmg_bonus,
            None => self.physical_dmg_bonus,
        }
    }
}
```

### 3. ダメージ計算式の変更

`damage.rs` で:

```rust
// 変更前
let non_crit = base * (1.0 + stats.dmg_bonus) * def_mult * res_mult * amplify;

// 変更後
let total_dmg_bonus = stats.total_dmg_bonus(input.element);
let non_crit = base * (1.0 + total_dmg_bonus) * def_mult * res_mult * amplify;
```

### 4. データ層の振り分け変更

#### `team_builder.rs` — `apply_weapon_sub_stat`

```rust
// 変更前: PhysicalDmgBonus → profile.dmg_bonus
// 変更後:
WeaponSubStat::PhysicalDmgBonus(v) => profile.physical_dmg_bonus += v[3],
```

#### `team_builder.rs` — `apply_ascension_stat`

```rust
// 変更前: ElementalDmgBonus/PhysicalDmgBonus → profile.dmg_bonus
// 変更後:
AscensionStat::ElementalDmgBonus(element, v) => match element {
    Element::Pyro => profile.pyro_dmg_bonus += v,
    Element::Hydro => profile.hydro_dmg_bonus += v,
    Element::Electro => profile.electro_dmg_bonus += v,
    Element::Cryo => profile.cryo_dmg_bonus += v,
    Element::Dendro => profile.dendro_dmg_bonus += v,
    Element::Anemo => profile.anemo_dmg_bonus += v,
    Element::Geo => profile.geo_dmg_bonus += v,
},
AscensionStat::PhysicalDmgBonus(v) => profile.physical_dmg_bonus += v,
```

#### `team_builder.rs` — `merge_artifact_stats`

既存の `merge_artifact_stats()` はフィールドを明示列挙しているため、8つの新フィールドを追加:

```rust
profile.pyro_dmg_bonus += self.artifact_stats.pyro_dmg_bonus;
profile.hydro_dmg_bonus += self.artifact_stats.hydro_dmg_bonus;
// ... 他元素 ...
profile.physical_dmg_bonus += self.artifact_stats.physical_dmg_bonus;
```

#### `good/src/stat_map.rs` — `add_to_profile`

現在の `StatField` enumは `ElementalDmgBonus(&'static str)` で元素を文字列で保持している。書き込み先を元素別フィールドに変更:

```rust
// 変更前: StatField::PhysicalDmgBonus → profile.dmg_bonus
// 変更後:
StatField::PhysicalDmgBonus => profile.physical_dmg_bonus += value,

// 変更前: StatField::ElementalDmgBonus(elem_str) → profile.dmg_bonus (キャラ元素チェック付き)
// 変更後: 文字列マッチで該当元素フィールドに無条件で書き込み（キャラ元素チェック廃止）
StatField::ElementalDmgBonus(elem_str) => match elem_str {
    "pyro" => profile.pyro_dmg_bonus += value,
    "hydro" => profile.hydro_dmg_bonus += value,
    "electro" => profile.electro_dmg_bonus += value,
    "cryo" => profile.cryo_dmg_bonus += value,
    "dendro" => profile.dendro_dmg_bonus += value,
    "anemo" => profile.anemo_dmg_bonus += value,
    "geo" => profile.geo_dmg_bonus += value,
    _ => {} // 未知の元素は無視
},
```

**重要**: 現在のキャラ元素マッチチェック（`element_str_matches`）と `ElementMismatchGoblet` 警告パスは廃止する。元素別フィールドに正しく格納すれば、ダメージ計算時に `total_dmg_bonus(element)` が適切な元素ボーナスのみを選択するため、格納時のフィルタリングは不要。異元素杯は格納されるが、計算時にその元素でダメージを与えない限り使われない。

#### `good/src/convert.rs` — `apply_two_piece_buffs`

2セット効果の `BuffableStat::ElementalDmgBonus(elem)` と `PhysicalDmgBonus` も元素別フィールドに無条件で振り分け（キャラ元素マッチチェック `elem == character.element` は廃止）:

```rust
BuffableStat::ElementalDmgBonus(elem) => match elem {
    Element::Pyro => profile.pyro_dmg_bonus += value,
    // ... 他元素 ...
},
BuffableStat::PhysicalDmgBonus => profile.physical_dmg_bonus += value,
```

理由: `stat_map.rs` と同様、格納時のフィルタリングは不要。`total_dmg_bonus(element)` が計算時に正しい元素を選択する。

#### `core/src/team.rs` — `apply_buffs_to_profile`

`ElementalDmgBonus` と `PhysicalDmgBonus` を `is_unconditional()` に追加し、`apply_buffs_to_profile` で元素別フィールドに振り分け:

```rust
// is_unconditional() に追加:
BuffableStat::ElementalDmgBonus(_) => true,
BuffableStat::PhysicalDmgBonus => true,

// apply_buffs_to_profile で振り分け:
BuffableStat::ElementalDmgBonus(e) => match e { /* 元素別 */ },
BuffableStat::PhysicalDmgBonus => profile.physical_dmg_bonus += value,
```

**振る舞い変更**: 従来これらのバフは `is_unconditional()` に含まれず `apply_buffs_to_profile` でスキップされていた。この変更により、チームバフとして提供される元素/物理DMGボーナス（例: 武器パッシブの「炎元素DMG+20%」）が `StatProfile` の元素別フィールドに正しく反映されるようになる。`total_dmg_bonus(element)` が計算時に適切な元素のみを選択するため、他元素のダメージには影響しない。

#### `BuffableStat` の適用（全体方針）

```rust
BuffableStat::DmgBonus => profile.dmg_bonus += value,           // 汎用
BuffableStat::ElementalDmgBonus(e) => /* 元素別に振り分け */,
BuffableStat::PhysicalDmgBonus => profile.physical_dmg_bonus += value,
```

`NormalAtkDmgBonus` 等のダメージタイプ別バフは変更なし（汎用 `dmg_bonus` のまま）。

### 5. `combine_stats()` の変更

元素別フィールドはそのままコピー:

```rust
pub fn combine_stats(profile: &StatProfile) -> Result<Stats, CalcError> {
    Ok(Stats {
        hp: profile.base_hp * (1.0 + profile.hp_percent) + profile.hp_flat,
        atk: profile.base_atk * (1.0 + profile.atk_percent) + profile.atk_flat,
        def: profile.base_def * (1.0 + profile.def_percent) + profile.def_flat,
        elemental_mastery: profile.elemental_mastery,
        crit_rate: profile.crit_rate,
        crit_dmg: profile.crit_dmg,
        energy_recharge: profile.energy_recharge,
        dmg_bonus: profile.dmg_bonus,
        pyro_dmg_bonus: profile.pyro_dmg_bonus,
        hydro_dmg_bonus: profile.hydro_dmg_bonus,
        electro_dmg_bonus: profile.electro_dmg_bonus,
        cryo_dmg_bonus: profile.cryo_dmg_bonus,
        dendro_dmg_bonus: profile.dendro_dmg_bonus,
        anemo_dmg_bonus: profile.anemo_dmg_bonus,
        geo_dmg_bonus: profile.geo_dmg_bonus,
        physical_dmg_bonus: profile.physical_dmg_bonus,
    })
}
```

### 6. 新WASM関数 `build_stats()`

```rust
// crates/wasm/src/lib.rs
#[wasm_bindgen]
pub fn build_stats(build: JsValue) -> Result<JsValue, JsError> {
    let build: CharacterBuild = serde_wasm_bindgen::from_value(build)?;
    let profile = build_stat_profile(&build);
    let stats = combine_stats(&profile)?;
    to_js(&stats)
}
```

#### `build_stat_profile()` — good crateに配置

`CharacterBuild` → `StatProfile` の変換:

1. `StatProfile::default()` を作成（全フィールド `0.0`）
2. キャラ基礎ステータスをセット: `base_hp = character.base_hp[3]`、`base_atk = character.base_atk[3]`、`base_def = character.base_def[3]`
   - 常に配列インデックス `3`（Lv90/突破6の値）を使用。既存の `TeamMemberBuilder::build_base_profile()` と同じ方針
3. 武器基礎攻撃力を `base_atk` に加算: `weapon.base_atk[3]`（Lv90固定）
4. 武器サブステを適用（`apply_weapon_sub_stat`、index=3）
5. 突破ステータスを適用（`apply_ascension_stat`）— `AscensionStat` はスカラー値（f64）を1つ保持しており、そのまま加算
6. 基礎値を加算: `crit_rate += 0.05`, `crit_dmg += 0.50`, `energy_recharge += 1.0`
7. 聖遺物の `StatProfile` をマージ（`artifacts.stats` の全フィールドを加算、元素別DMGフィールド含む）
8. `StatProfile` を返す

### 7. `resolve_team_stats()` との関係

| 関数 | 用途 | チームバフ |
|------|------|-----------|
| `build_stats()` | 単体キャラのStats表示・簡易計算 | なし |
| `resolve_team_stats()` | 本格ダメージ計算 | あり |

両者とも `StatProfile` → `combine_stats()` を使うため、元素別DMGボーナスの恩恵を自動的に受ける。

## 影響範囲

### 破壊的変更（コンパイルエラー）

- `Stats` を構築している全箇所 — 8フィールド追加
- `StatProfile` 同様
- テストで `Stats { ... }` を直接構築している箇所

### 振る舞い変更

- `damage.rs` の計算式 — `total_dmg_bonus(element)` を使用
- 物理DMG%キャラのテスト結果 — `physical_dmg_bonus` に移動

### `stat_profile.rs` — `validate()`

`validate()` に8つの新DMGボーナスフィールドの検証を追加（`< -1.0` チェック）。既存の `CalcError::InvalidDmgBonus` エラーバリアントを再利用する。

### docコメント更新

`wasm/src/lib.rs` の `resolve_team_stats` docコメントに新フィールドを追記。

### テスト修正方針

- 既存テストの `Stats { dmg_bonus: 0.466, .. }` → 元素杯なら該当元素フィールドに移動
- 物理キャラのテストケース追加: 物理ダメージに `physical_dmg_bonus` が乗り、元素ダメージには乗らないことを検証
- `build_stats()` のテスト: GOOD JSON → `build_stats()` → 期待されるATK/CRIT等をepsilon比較で検証
- `total_dmg_bonus()` のテスト: `element: None` で `physical_dmg_bonus` が使われることを検証

### 設計上の明示ルール

- `total_dmg_bonus(None)` は `dmg_bonus + physical_dmg_bonus` を返す（原神の物理ダメージ計算に対応）
- `total_dmg_bonus(Some(Element::Pyro))` は `dmg_bonus + pyro_dmg_bonus` を返す
- 汎用 `dmg_bonus` は全ダメージに加算される（ダメージタイプ別バフ: NormalAtkDmgBonus等が入る）

### 変更しないもの

- `DamageInput` 構造体 — `Stats` を内包するのみ
- `LunarInput`, `TransformativeInput` — 既存通り
- `TeamMember` — `StatProfile` を内包するため自動的に恩恵を受ける
- `NormalAtkDmgBonus` 等のダメージタイプ別バフ — 汎用 `dmg_bonus` のまま

### smart-paimon（UI側）

- TypeScript型定義の更新が必要（8フィールド追加）
- `buildStats()` → `wasm.build_stats()` に置き換え
- **UI側の修正は今回のスコープ外**
