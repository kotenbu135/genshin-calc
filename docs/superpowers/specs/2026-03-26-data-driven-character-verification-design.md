# Data-Driven Character Verification テスト設計

## 概要

KQMコミュニティの検証済み計算データを活用し、TOML外部データファイルによるデータ駆動テストで15キャラ・約30ケースのダメージ検証を追加する。

## 目的

- 元素×スケーリング×反応タイプの組み合わせを網羅的にカバー
- KQM TheoryCraftingの検証値で計算エンジンの正確性を裏付ける
- 新キャラ追加をTOMLファイル1つで完結させる拡張性の確保

## ディレクトリ構成

```
crates/core/
├── src/                          # 変更なし
├── tests/
│   ├── data/
│   │   └── characters/
│   │       ├── diluc.toml
│   │       ├── hu_tao.toml
│   │       ├── ganyu.toml
│   │       ├── raiden.toml
│   │       ├── yelan.toml
│   │       ├── ayato.toml
│   │       ├── itto.toml
│   │       ├── nahida.toml
│   │       ├── xiao.toml
│   │       ├── freminet.toml
│   │       ├── tighnari.toml
│   │       ├── kazuha.toml
│   │       ├── fischl.toml
│   │       ├── yae_miko.toml
│   │       └── nilou.toml
│   ├── test_types.rs             # TOML用型定義
│   └── character_verification.rs # テストランナー
└── Cargo.toml                    # dev-dependencies追加
```

## TOMLスキーマ

### 設計方針: Rust型との対応

TOMLフィールドは実際のRust構造体に1対1で対応させる。テストランナーは中間変換を行わず、TOMLから直接API入力を構築する。

- **`Stats` を直接使用する**（`StatProfile` ではなく）: TOMLには最終計算済みステータス（`atk`, `hp`, `def` 等）を記載する。`StatProfile → Stats` の合算はこのテストのスコープ外（既存の `stat_profile.rs` テストでカバー済み）。
- **フィールド名はRust構造体と一致させる**: `elemental_mastery`（`em` ではなく）、`resistance`（`enemy_resistance` ではなく）等。

### Rust型 → TOML対応表

| Rust構造体 | TOMLセクション | 備考 |
|-----------|---------------|------|
| `Stats` | `[cases.stats]` | `hp`, `atk`, `def`, `elemental_mastery`, `crit_rate`, `crit_dmg`, `energy_recharge`, `dmg_bonus` |
| `Enemy` | `[cases.enemy]` | `level`, `resistance`, `def_reduction` |
| `DamageInput` | `[[cases]]` トップレベル | `character_level`, `talent_multiplier`, `scaling_stat`, `damage_type`, `element`, `reaction`, `reaction_bonus` |
| `TransformativeInput` | `[[cases]]` トップレベル | `character_level`, `elemental_mastery`, `reaction`, `reaction_bonus` |
| `LunarInput` | `[[cases]]` トップレベル | `character_level`, `elemental_mastery`, `reaction`, `reaction_bonus`, `crit_rate`, `crit_dmg` |

### Reaction enum のTOML表現

`Reaction` enumは大半が単純バリアントだが、`Swirl(Element)` はタプルバリアント。TOMLでは以下のマッピングを使用し、テストランナー内でカスタムデシリアライズする:

| TOML文字列 | Rust値 |
|-----------|--------|
| `"Vaporize"` | `Reaction::Vaporize` |
| `"Melt"` | `Reaction::Melt` |
| `"Aggravate"` | `Reaction::Aggravate` |
| `"Spread"` | `Reaction::Spread` |
| `"Overloaded"` | `Reaction::Overloaded` |
| `"Superconduct"` | `Reaction::Superconduct` |
| `"ElectroCharged"` | `Reaction::ElectroCharged` |
| `"Shattered"` | `Reaction::Shattered` |
| `"Bloom"` | `Reaction::Bloom` |
| `"Hyperbloom"` | `Reaction::Hyperbloom` |
| `"Burgeon"` | `Reaction::Burgeon` |
| `"Burning"` | `Reaction::Burning` |
| `"SwirlPyro"` | `Reaction::Swirl(Element::Pyro)` |
| `"SwirlHydro"` | `Reaction::Swirl(Element::Hydro)` |
| `"SwirlElectro"` | `Reaction::Swirl(Element::Electro)` |
| `"SwirlCryo"` | `Reaction::Swirl(Element::Cryo)` |
| `"LunarElectroCharged"` | `Reaction::LunarElectroCharged` |
| `"LunarBloom"` | `Reaction::LunarBloom` |
| `"LunarCrystallize"` | `Reaction::LunarCrystallize` |
| `"LunarCrystallizeSecondary"` | `Reaction::LunarCrystallizeSecondary` |

テストランナーの `parse_reaction(s: &str) -> Reaction` ヘルパーで変換。`Swirl*` 以外はserde標準デシリアライズに委譲可能だが、統一性のためすべてこのヘルパーを通す。

### ファイルヘッダ

```toml
# Source: https://keqingmains.com/<character>/

[character]
name = "Diluc"
element = "Pyro"        # Pyro/Hydro/Electro/Cryo/Dendro/Anemo/Geo
                        # 物理キャラ（Freminet等）はelement省略（element無しで通常ダメージ）
```

注: `element` はキャラの代表元素を示すメタデータ。各テストケースの `element` フィールドが実際の `DamageInput.element` にマッピングされる。

### テストケース: normal / amplifying / catalyze（→ `calculate_damage`）

```toml
[[cases]]
name = "Searing Onslaught Lv8 non-reaction"
type = "normal"            # normal / amplifying / catalyze
character_level = 90
talent_multiplier = 1.5104
scaling_stat = "Atk"       # Atk / Hp / Def（ScalingStat enumと一致、省略時 Atk）
damage_type = "Skill"      # Normal / Charged / Plunging / Skill / Burst
element = "Pyro"           # 物理ダメージの場合は省略（None扱い）

# amplifying / catalyze の場合のみ
# reaction = "Vaporize"    # Reaction enumバリアント名（増幅の方向はelementで決定）
# reaction_bonus = 0.0

[cases.stats]              # Stats構造体と1対1対応
hp = 12000.0               # 最終HP（StatProfile合算後の値）
atk = 1800.0               # 最終ATK
def = 800.0                # 最終DEF
elemental_mastery = 0.0
crit_rate = 0.60
crit_dmg = 1.20
energy_recharge = 1.0
dmg_bonus = 0.466

[cases.enemy]              # Enemy構造体と1対1対応
level = 90
resistance = 0.10
def_reduction = 0.0

[cases.expected]
non_crit = 1793.54
crit = 3945.79
average = 2084.89
```

増幅反応の方向性について:
- `element = "Pyro"` + `reaction = "Vaporize"` → 炎→水に蒸発（1.5倍、逆蒸発）
- `element = "Hydro"` + `reaction = "Vaporize"` → 水→炎に蒸発（2.0倍、順蒸発）
- `element = "Pyro"` + `reaction = "Melt"` → 炎→氷に溶解（2.0倍、順溶解）
- `element = "Cryo"` + `reaction = "Melt"` → 氷→炎に溶解（1.5倍、逆溶解）

### テストケース: transformative（→ `calculate_transformative`）

```toml
[[cases]]
name = "Overloaded EM200"
type = "transformative"
character_level = 90
elemental_mastery = 200.0
reaction = "Overloaded"
reaction_bonus = 0.0

[cases.enemy]              # Enemy構造体（level, resistance, def_reduction）
level = 90
resistance = 0.10
def_reduction = 0.0

[cases.expected]
damage = 5765.0
```

### テストケース: lunar（→ `calculate_lunar`）

```toml
[[cases]]
name = "Lunar Electro-Charged EM500"
type = "lunar"
character_level = 90
elemental_mastery = 500.0
reaction = "LunarElectroCharged"
reaction_bonus = 0.0
crit_rate = 0.60
crit_dmg = 1.20

[cases.enemy]              # Enemy構造体（level, resistance, def_reduction）
level = 90
resistance = 0.10
def_reduction = 0.0

[cases.expected]
non_crit = 12000.0
crit = 26400.0
average = 20640.0
```

## テストランナー設計

`tests/character_verification.rs`（Cargo統合テスト）:

1. `tests/data/characters/*.toml` を `glob` で全件読み込み
2. 各ファイルの `[[cases]]` をイテレート
3. `type` フィールドに応じて計算関数を呼び分け:
   - `"normal"` / `"amplifying"` / `"catalyze"` → `DamageInput` 構築 → `calculate_damage()`
   - `"transformative"` → `TransformativeInput` 構築 → `calculate_transformative()`
   - `"lunar"` → `LunarInput` 構築 → `calculate_lunar()`
4. 結果と `expected` を許容誤差で比較
5. 失敗時にキャラ名・ケース名・期待値・実値を出力

### 型定義（`test_types.rs`）

```rust
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CharacterTestData {
    pub character: CharacterInfo,
    pub cases: Vec<TestCase>,
}

#[derive(Deserialize)]
pub struct CharacterInfo {
    pub name: String,
    pub element: Option<String>,  // 物理キャラはNone
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum TestCase {
    #[serde(rename = "normal")]
    Normal(DamageCase),
    #[serde(rename = "amplifying")]
    Amplifying(DamageCase),
    #[serde(rename = "catalyze")]
    Catalyze(DamageCase),
    #[serde(rename = "transformative")]
    Transformative(TransformativeCase),
    #[serde(rename = "lunar")]
    Lunar(LunarCase),
}

#[derive(Deserialize)]
pub struct DamageCase {
    pub name: String,
    pub character_level: u32,
    pub talent_multiplier: f64,
    #[serde(default = "default_atk")]
    pub scaling_stat: String,       // "Atk" / "Hp" / "Def"
    pub damage_type: String,        // "Normal" / "Charged" / "Plunging" / "Skill" / "Burst"
    pub element: Option<String>,    // None = physical
    pub reaction: Option<String>,   // None = no reaction
    #[serde(default)]
    pub reaction_bonus: f64,
    pub stats: StatsData,
    pub enemy: EnemyData,
    pub expected: DamageExpected,
}

#[derive(Deserialize)]
pub struct TransformativeCase {
    pub name: String,
    pub character_level: u32,
    pub elemental_mastery: f64,
    pub reaction: String,
    #[serde(default)]
    pub reaction_bonus: f64,
    pub enemy: EnemyData,
    pub expected: TransformativeExpected,
}

#[derive(Deserialize)]
pub struct LunarCase {
    pub name: String,
    pub character_level: u32,
    pub elemental_mastery: f64,
    pub reaction: String,
    #[serde(default)]
    pub reaction_bonus: f64,
    pub crit_rate: f64,
    pub crit_dmg: f64,
    pub enemy: EnemyData,
    pub expected: DamageExpected,
}

#[derive(Deserialize)]
pub struct StatsData {
    pub hp: f64,
    pub atk: f64,
    pub def: f64,
    pub elemental_mastery: f64,
    pub crit_rate: f64,
    pub crit_dmg: f64,
    #[serde(default = "default_er")]
    pub energy_recharge: f64,
    pub dmg_bonus: f64,
}

#[derive(Deserialize)]
pub struct EnemyData {
    pub level: u32,
    pub resistance: f64,
    #[serde(default)]
    pub def_reduction: f64,
}

#[derive(Deserialize)]
pub struct DamageExpected {
    pub non_crit: f64,
    pub crit: f64,
    pub average: f64,
}

#[derive(Deserialize)]
pub struct TransformativeExpected {
    pub damage: f64,
}

fn default_atk() -> String { "Atk".to_string() }
fn default_er() -> f64 { 1.0 }
```

### Reaction パース処理

```rust
fn parse_reaction(s: &str) -> Reaction {
    match s {
        "Vaporize" => Reaction::Vaporize,
        "Melt" => Reaction::Melt,
        "Aggravate" => Reaction::Aggravate,
        "Spread" => Reaction::Spread,
        "Overloaded" => Reaction::Overloaded,
        "Superconduct" => Reaction::Superconduct,
        "ElectroCharged" => Reaction::ElectroCharged,
        "Shattered" => Reaction::Shattered,
        "Bloom" => Reaction::Bloom,
        "Hyperbloom" => Reaction::Hyperbloom,
        "Burgeon" => Reaction::Burgeon,
        "Burning" => Reaction::Burning,
        "SwirlPyro" => Reaction::Swirl(Element::Pyro),
        "SwirlHydro" => Reaction::Swirl(Element::Hydro),
        "SwirlElectro" => Reaction::Swirl(Element::Electro),
        "SwirlCryo" => Reaction::Swirl(Element::Cryo),
        "LunarElectroCharged" => Reaction::LunarElectroCharged,
        "LunarBloom" => Reaction::LunarBloom,
        "LunarCrystallize" => Reaction::LunarCrystallize,
        "LunarCrystallizeSecondary" => Reaction::LunarCrystallizeSecondary,
        other => panic!("Unknown reaction: {other}"),
    }
}
```

### 依存関係

`crates/core/Cargo.toml` の `[dev-dependencies]` に追加:

```toml
toml = "0.8"
glob = "0.3"
```

`dev-dependencies` のためWASM互換性・プロダクションバイナリに影響なし。

## キャラ選定とカバレッジ

| # | キャラ | 元素 | スケーリング | テストする反応 | ケース数 |
|---|--------|------|-------------|---------------|---------|
| 1 | Diluc | Pyro | Atk | 通常(Skill), 蒸発(Skill), 溶解逆(Skill) | 3 |
| 2 | Hu Tao | Pyro | Hp | 通常(Normal), 蒸発(Normal) | 2 |
| 3 | Ganyu | Cryo | Atk | 通常(Charged), 溶解(Charged), 溶解逆(Charged) | 3 |
| 4 | Raiden | Electro | Atk | 通常(Burst), 超激化(Burst) | 2 |
| 5 | Yelan | Hydro | Hp | 通常(Skill), 蒸発逆(Skill) | 2 |
| 6 | Ayato | Hydro | Atk | 通常(Normal), 蒸発逆(Normal) | 2 |
| 7 | Itto | Geo | Def | 通常(Normal) | 1 |
| 8 | Nahida | Dendro | Atk | 通常(Skill), 草激化(Skill) | 2 |
| 9 | Xiao | Anemo | Atk | 通常(Plunging) | 1 |
| 10 | Freminet | なし(物理) | Atk | 通常(Normal) | 1 |
| 11 | Tighnari | Dendro | Atk | 通常(Charged), 草激化(Charged) | 2 |
| 12 | Kazuha | Anemo | Atk | 拡散(SwirlPyro/Cryo/Electro/Hydro) | 4 |
| 13 | Fischl | Electro | Atk | 超激化(Skill) | 1 |
| 14 | Yae Miko | Electro | Atk | 超激化(Skill), 感電(lunar) | 2 |
| 15 | Nilou | Hydro | Hp | 開花(transformative), 月開花(lunar) | 2 |

**合計: 約30ケース**

### カバレッジまとめ

- **元素**: 7/7 全元素 + 物理（`element: None`）
- **スケーリング**: Atk 9体, Hp 3体, Def 1体
- **DamageType**: Normal 4, Charged 3, Plunging 1, Skill 6, Burst 2（全5種カバー）
- **反応**: Vaporize(正逆), Melt(正逆), Aggravate, Spread, Swirl(4元素), LunarElectroCharged, Bloom, LunarBloom
- **パイプライン**: `calculate_damage` / `calculate_transformative` / `calculate_lunar` 全3種

## 許容誤差

| テスト種別 | 許容誤差 | 理由 |
|-----------|---------|------|
| KQM手計算照合 | `< 0.01` | 浮動小数点演算の丸め |
| ゲーム内検証（既存） | `< 1.0` | ゲーム側のfloor丸め |
| ケース単位で上書き | `tolerance` フィールド（optional） | データソースにより精度が異なる場合 |

## 成功基準

- 全15キャラ・約30ケースがパスする
- `cargo test` で既存127テスト + 新規約30テスト = 約157テストが全パス
- 新キャラ追加がTOMLファイル1つの追加で完結する
- 計算ロジック（`src/`）は一切変更しない

## スコープ外

- 既存のインラインテスト（127個）の変更
- 計算ロジックの修正
- `StatProfile → Stats` 合算のテスト（既存カバー済み）
- `data` crateへの変更
- CI/CDパイプラインの変更（将来対応可能）
