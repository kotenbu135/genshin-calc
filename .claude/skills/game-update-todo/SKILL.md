---
name: game-update-todo
description: Generate implementation TODO list when a new Genshin Impact version brings new characters, weapons, artifacts, or enemies
version: 1.0.0
source: local-git-analysis
analyzed_commits: 50
---

# Game Update TODO Generator

ゲーム更新（新バージョン）で追加されるコンテンツの実装TODOリストを生成するスキル。

## Usage

```
/game-update-todo <version>
```

**例:**
```
/game-update-todo 5.9
/game-update-todo 6.0 キャラ2体 武器3本 聖遺物1セット
```

## Trigger

- ユーザーが「新バージョン」「ゲーム更新」「v5.9」「新キャラ追加」などに言及したとき
- `/game-update-todo` スラッシュコマンド実行時

## Workflow

### Step 1: 情報収集（Wiki自動リサーチ）

ユーザーが指定したバージョン番号を元に、Web検索とWikiページから新コンテンツ情報を自動収集する。
ユーザーへの質問は不要 — 全てWebから取得する。

#### 1-1. バージョン概要の取得

WebSearchで `"Genshin Impact {version} new characters weapons"` を検索し、
該当バージョンの新コンテンツ一覧を特定する。

主要ソース（優先順）:
1. **Genshin Impact Wiki (Fandom)** — `genshin-impact.fandom.com`
2. **Honey Hunter World** — `ambr.top`
3. **Project Amber** — 天賦倍率・武器ステータスの正確な数値

#### 1-2. キャラクターデータ収集

各新キャラについてWikiページをWebFetchで取得し、以下を抽出:

| データ | 取得元 |
|--------|--------|
| 名前・元素・武器種・星・地域 | Wiki概要 |
| base_hp/atk/def (Lv1/20/80/90) | Wiki "Base Stats" テーブル |
| 突破ステータス (ascension_stat) | Wiki "Ascension" セクション |
| 天賦倍率 (Lv1-15 × 各段) | Wiki "Talent" または Honey Hunter |
| 命の星座パターン (C3/C5) | Wiki "Constellation" セクション |
| 固有天賦 (A1/A4) | Wiki "Passive Talents" |
| チームバフ効果 | Wiki "Passive Talents" + "Constellation" |
| 月兆対応 | Wiki "Moonsign" セクション（あれば） |

#### 1-3. 武器データ収集

各新武器についてWikiページから:

| データ | 取得元 |
|--------|--------|
| 名前・武器種・星 | Wiki概要 |
| base_atk (Lv1/20/80/90) | Wiki "Base Stats" テーブル |
| sub_stat (種類 + 4値) | Wiki "Base Stats" テーブル |
| パッシブ名・効果説明 | Wiki "Passive" セクション |
| R1-R5精錬値 | Wiki "Refinement" テーブル |

#### 1-4. 聖遺物データ収集

| データ | 取得元 |
|--------|--------|
| セット名・レアリティ | Wiki概要 |
| 2pc効果 | Wiki "2-Piece Bonus" |
| 4pc効果 | Wiki "4-Piece Bonus" |

#### 1-5. 敵データ収集

| データ | 取得元 |
|--------|--------|
| 名前 | Wiki "Enemies" |
| 各元素耐性値 | Wiki "Resistance" テーブル |

#### 1-6. 収集結果の提示

全データ収集後、以下をユーザーに提示して確認を求める:

```
## v{VERSION} 新コンテンツ確認

### 新キャラクター ({N}体)
- {名前} ({元素}/{武器種}/★{星}/{地域})

### 新武器 ({N}本)
- {名前} ({武器種}/★{星}/サブ:{ステ})

### 新聖遺物 ({N}セット)
- {セット名}

### 新敵 ({N}体)
- {名前}

この内容でTODOリストを生成しますか？
```

### Step 2: TODO生成

以下のテンプレートを使用してチェックリストを生成する。

---

## TODO Template

````markdown
# v{VERSION} データ更新 TODO

## 概要

| カテゴリ | 追加数 |
|----------|--------|
| キャラクター | {N}体 |
| 武器 | {N}本 |
| 聖遺物セット | {N}セット |
| 敵 | {N}体 |

---

## 1. 新キャラクター

### {キャラ名} ({元素}/{武器種}/★{星})

#### 1-1. キャラデータ定義
- [ ] `crates/data/src/characters/{element}.rs` にconst定義追加
  - `id`, `name`, `element`, `weapon_type`, `rarity`, `region`
  - `base_hp`, `base_atk`, `base_def` (Lv1/Lv20/Lv80/Lv90の4値)
  - `ascension_stat` (突破ステ)
  - `constellation_pattern` (C3Skill or C3Burst)
- [ ] 天賦スケーリング定義
  - 通常攻撃: {N}段 + 重撃 + 落下 (各`[f64; 15]` Lv1-15)
  - 元素スキル: {N}段 (各`[f64; 15]`)
  - 元素爆発: {N}段 (各`[f64; 15]`)
- [ ] `NormalAttackData` + `TalentData` × 2 → `TalentSet` 構築

#### 1-2. 登録
- [ ] `crates/data/src/characters/mod.rs` の `ALL_CHARACTERS` に追加
  - コメント付き元素グループ内に挿入

#### 1-3. テストケース
- [ ] `crates/core/tests/data/characters/{character_id}.toml` 作成
  - 最低1ケース（通常ダメージ）
  - 可能ならゲーム内検証ケース追加
  - 元素反応ケース（該当する場合）

#### 1-4. 天賦バフ（該当キャラのみ）
- [ ] `crates/data/src/talent_buffs.rs` に `TalentBuffDef` 追加
  - チームバフを持つキャラ: `target: BuffTarget::Team`
  - 固有天賦(A1/A4)のステータスバフ
  - 命の星座のバフ（C1-C6で主要なもの）

#### 1-5. 月兆データ（該当キャラのみ）
- [ ] `crates/data/src/moonsign_chars.rs` に `MoonsignBenedictionDef` 追加

---

## 2. 新武器

### {武器名} ({武器種}/★{星})

#### 2-1. 武器データ定義
- [ ] `crates/data/src/weapons/{weapon_type}.rs` にconst定義追加
  - `id`, `name`, `weapon_type`, `rarity`
  - `base_atk` (Lv1/Lv20/Lv80/Lv90の4値)
  - `sub_stat` (ステータス種 + 4値)
  - `passive` (パッシブ名 + 効果)

#### 2-2. パッシブ効果実装
- [ ] 無条件バフ: `StatBuff` (always-active)
  - `refinement_values: Some([R1, R2, R3, R4, R5])`
- [ ] 条件付きバフ: `ConditionalBuff` (判別表を参照)

**ConditionalBuff判別:**

| パッシブ内容 | Activation |
|-------------|------------|
| 常時発動（ステータス依存） | `Auto(StatScaling { stat, offset, cap })` |
| 武器種限定 | `Auto(WeaponTypeRequired(&[...]))` |
| 元素限定 | `Auto(ElementRequired(&[...]))` |
| チーム元素数 | `Auto(TeamElementCount { element, min })` |
| HP閾値/スキル使用/元素反応後 | `Manual(Toggle)` |
| 3秒間x%アップ、最大n重 | `Manual(Stacks(n))` |
| ステータス依存 + トグル | `Both(AutoCondition, ManualCondition)` |
| proc/確率発動/回復系 | `conditional_buffs: &[]` (DescriptionOnly) |

#### 2-3. 登録
- [ ] `crates/data/src/weapons/mod.rs` の `ALL_WEAPONS` に追加

---

## 3. 新聖遺物セット

### {セット名}

#### 3-1. セット定義
- [ ] `crates/data/src/artifacts.rs` にconst定義追加
  - `id`, `name`, `rarity`
  - `two_piece`: `SetEffect { description, buffs, conditional_buffs }`
  - `four_piece`: `SetEffect { description, buffs, conditional_buffs }`

#### 3-2. 効果実装
- [ ] 2pc: 通常は `StatBuff`（無条件バフ）
- [ ] 4pc: `ConditionalBuff` (上記判別表を参照)

#### 3-3. 登録
- [ ] `ALL_ARTIFACT_SETS` に追加

---

## 4. 新敵

### {敵名}

#### 4-1. 敵データ定義
- [ ] `crates/data/src/enemies.rs` に追加
  - 既存 `ResistanceTemplate` を再利用 or 新規作成
  - `EnemyData { id, name, resistance }` const定義
  - `ALL_ENEMIES` 配列に追加

**既存テンプレート一覧:**

| テンプレート | パターン |
|-------------|---------|
| `ALL_10` | 全耐性10% (一般mob) |
| `PHYS_50_ELEM_10` | 物理50%/元素10% (遺跡守衛) |
| `PHYS_70_ELEM_10` | 物理70%/元素10% (遺跡機兵) |
| `{ELEM}_70_REST_10` | 特定元素70%/他10% (ボス) |
| `ALL_MINUS_10` | 全耐性-10% (特殊) |

---

## 5. 検証

- [ ] `cargo build` — コンパイル成功
- [ ] `cargo test` — 全テスト通過
  - データ整合性テスト（ID重複なし、base stats正値、buff名ユニーク）
  - キャラ検証テスト（TOMLケース）
- [ ] `cargo clippy -- -D warnings` — lint通過
- [ ] `cargo fmt --check` — フォーマット確認

---

## 6. ドキュメント更新

- [ ] `docs/data-expansion-todo.md` の現状サマリ表を更新
- [ ] CLAUDE.md の統計情報を更新（テスト数、キャラ数等）
````

---

## データソース参考

数値の取得先（優先順）:

1. **Genshin Impact Wiki (Fandom)** — 公式データマイニング値
2. **Honey Hunter World** — 天賦倍率テーブル
3. **KQM (Keqing Mains)** — 検証済みフォーミュラ
4. **ゲーム内スクリーンショット** — 最終確認

## 数値フォーマット規約

| 項目 | フォーマット | 例 |
|------|------------|-----|
| パーセンテージ | 小数形式 | 10.8% → `0.108` |
| base_hp/atk/def | 4値配列 | `[1000.0, 2500.0, 9000.0, 10000.0]` |
| 天賦スケーリング | 15値配列 | `[0.445, 0.481, ..., 0.823]` |
| 精錬値 | 5値配列 | `[0.20, 0.25, 0.30, 0.35, 0.40]` |
| sub_stat | BuffableStat + 4値 | `CritRate([0.048, 0.201, ..., 0.221])` |

## ConditionalBuff命名規約

`{weapon_or_artifact_id}_{effect_description}` — グローバルでユニーク必須。

例:
- `homa_hp_bonus` — 護摩の杖HP依存ATKバフ
- `crimson_witch_pyro_stacks` — 魔女4pc炎ダメスタック
- `bennett_burst_atk` — ベネット爆発ATKバフ（TalentBuff側）

## 実装パターンまとめ

### キャラクター定義の最小テンプレート

```rust
// 通常攻撃1段目の倍率（Lv1-15）
const CHAR_NORMAL_1: [f64; 15] = [0.445, 0.481, ...省略..., 0.823];

pub const CHARACTER_NAME: CharacterData = CharacterData {
    id: "character_id",
    name: "キャラ名",
    element: Element::Pyro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [1000.0, 2500.0, 9000.0, 10000.0],
    base_atk: [25.0, 250.0, 280.0, 300.0],
    base_def: [60.0, 600.0, 700.0, 750.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet { /* ... */ },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
```

### 武器定義の最小テンプレート

```rust
pub const WEAPON_NAME: WeaponData = WeaponData {
    id: "weapon_id",
    name: "武器名",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat { stat: BuffableStat::CritRate, values: [0.048, 0.201, 0.201, 0.221] }),
    passive: Some(WeaponPassive {
        name: "パッシブ名",
        effect: PassiveEffect {
            description: "効果説明",
            buffs: &[StatBuff { stat: BuffableStat::AtkPercent, value: 0.20, refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]) }],
            conditional_buffs: &[],
        },
    }),
};
```

### キャラ検証TOML最小テンプレート

```toml
[character]
name = "CharacterName"
element = "Pyro"

[[cases]]
type = "normal"
name = "Normal Attack Hit 1 — no reaction"
character_level = 90
talent_multiplier = 0.4452
scaling_stat = "Atk"
damage_type = "Normal"
element = "Pyro"

[cases.stats]
hp = 15000.0
atk = 2000.0
def = 800.0
elemental_mastery = 100.0
crit_rate = 0.50
crit_dmg = 1.00
energy_recharge = 1.2
dmg_bonus = 0.466

[cases.enemy]
level = 90
resistance = 0.10

[cases.expected]
non_crit = 1234.56
crit = 2469.12
average = 1851.84
```
