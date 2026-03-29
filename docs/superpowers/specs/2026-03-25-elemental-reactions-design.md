# genshin-calc v0.2.0 元素反応 設計ドキュメント

## 概要

v0.1.0のダメージ計算エンジンに全元素反応を追加する。増幅反応（蒸発・溶解）、激化反応（超激化・草激化）、固定反応（過負荷・超電導・感電・拡散・開花系・燃焼）、月反応（月感電・月開花・月結晶）を網羅する。

## スコープ

### v0.2.0
- 増幅反応: 蒸発（1.5x/2.0x）、溶解（1.5x/2.0x）
- 激化反応: 超激化（1.15）、草激化（1.25）
- 固定反応: 過負荷、超電導、感電、氷砕き、拡散、開花、超開花、烈開花、燃焼
- 月反応: 月感電、月開花、月結晶
- 元素熟知ボーナス計算（4種の式）
- レベル基礎値テーブル（Lv1〜100、データマイニング値）
- 反応ボーナス（装備効果等）を`reaction_bonus: f64`で受け取る

### スコープ外
- 元素付着ゲージ管理
- ICD（内部クールダウン）
- 結晶シールドの吸収量計算（通常の結晶反応はダメージを与えないため対象外。月結晶のみ対応）
- 凍結の持続時間計算

## 破壊的変更（Breaking Changes）

v0.2.0は以下の破壊的変更を含む:

1. **`DamageInput` にフィールド追加**: `reaction: Option<Reaction>`, `reaction_bonus: f64` — 全ての構築箇所に追加が必要
2. **`DamageResult` にフィールド追加**: `reaction: Option<Reaction>` — パターンマッチや分割代入に影響

### 移行方法
```rust
// v0.1.0
DamageInput { character_level: 90, stats, talent_multiplier: 1.76, damage_type, element }
// v0.2.0（反応なし = v0.1.0と同じ動作）
DamageInput { character_level: 90, stats, talent_multiplier: 1.76, damage_type, element,
              reaction: None, reaction_bonus: 0.0 }
```

## 設計方針

- **既存v0.1.0との後方互換**: `reaction: None`で従来と同じ計算結果
- **enumマッピング**: `Reaction` enumで反応タイプを直接指定。増幅反応のみ `determine_reaction(trigger, aura)` でも判定可能
- **API分離**: 増幅/激化は`calculate_damage`拡張、固定反応は`calculate_transformative`、月反応は`calculate_lunar`
- **激化は直接指定**: Quicken（原激化）は元素ではなく状態なので、`Reaction::Aggravate`/`Spread`を直接指定する
- **耐性は呼び出し側責任**: `Enemy.resistance`は単一値。固定反応や月反応ではダメージ元素が変わるため、呼び出し側が適切な元素の耐性値を設定する
- **既存原則を維持**: イミュータブル、純粋関数、serde対応、WASM互換

## プロジェクト構成（差分）

```
crates/core/src/
├── lib.rs              # 新モジュール・再エクスポート追加
├── types.rs            # 変更なし
├── stats.rs            # 変更なし（elemental_mastery が使われるようになる）
├── enemy.rs            # 変更なし
├── error.rs            # 新バリアント追加
├── damage.rs           # DamageInput拡張、激化/増幅対応
├── reaction.rs         # 【新規】反応タイプ判定、Reaction enum
├── em.rs               # 【新規】元素熟知ボーナス計算
├── level_table.rs      # 【新規】レベル基礎値テーブル
├── transformative.rs   # 【新規】固定反応ダメージ計算
└── lunar.rs            # 【新規】月反応ダメージ計算
```

## 元素反応の分類と計算式

### 増幅反応（Amplifying）

既存ダメージに倍率を乗算する。

| トリガー | 付着(aura) | 反応 | 基本倍率 |
|---------|-----------|------|---------|
| Pyro | Hydro | 蒸発 | 1.5 |
| Hydro | Pyro | 蒸発 | 2.0 |
| Pyro | Cryo | 溶解 | 2.0 |
| Cryo | Pyro | 溶解 | 1.5 |

```
最終ダメージ = 通常ダメージ × 基本倍率 × (1 + EM_bonus + reaction_bonus)
EM_bonus = 2.78 × EM / (EM + 1400)
```

### 激化反応（Catalyze）

基礎ダメージにフラット加算してから通常パイプラインを通す。

| 条件 | 反応 | 倍率係数 |
|------|------|---------|
| Electro攻撃 on 原激化(Quicken)状態 | 超激化（Aggravate） | 1.15 |
| Dendro攻撃 on 原激化(Quicken)状態 | 草激化（Spread） | 1.25 |

原激化状態（Quicken）は Dendro + Electro の付着で発生する。**Quickenは元素ではなく状態**であるため、`determine_reaction`では判定できない。呼び出し側が`Reaction::Aggravate`または`Reaction::Spread`を直接指定する。

```
加算量 = 倍率係数 × レベル基礎値 × (1 + EM_bonus + reaction_bonus)
基礎ダメージ = (ATK × 天賦倍率 + 加算量) × (1 + dmg_bonus) × 防御補正 × 耐性補正 × 会心
EM_bonus = 5.0 × EM / (EM + 1200)
```

### 固定反応（Transformative）

ATK・天賦倍率を使わない。キャラレベルと元素熟知のみで算出。

| 反応 | 倍率 | ダメージ元素 |
|------|------|------------|
| 過負荷（Overloaded） | 2.75 | Pyro |
| 超電導（Superconduct） | 1.5 | Cryo |
| 感電（Electro-Charged） | 2.0 | Electro |
| 氷砕き（Shattered） | 3.0 | Physical（None） |
| 拡散（Swirl） | 0.6 | 拡散された元素 |
| 開花（Bloom） | 2.0 | Dendro |
| 超開花（Hyperbloom） | 3.0 | Dendro |
| 烈開花（Burgeon） | 3.0 | Dendro |
| 燃焼（Burning） | 0.25 | Pyro |

**拡散の有効元素**: Swirl(Element)のElementはPyro, Hydro, Electro, Cryoのみ有効。Dendro, Anemo, Geoは無効（バリデーションエラー）。

```
固定ダメージ = レベル基礎値 × 反応倍率 × (1 + EM_bonus + reaction_bonus) × 耐性補正
EM_bonus = 16.0 × EM / (EM + 2000)
```

会心なし、防御補正なし。

**耐性について**: `enemy.resistance`はダメージ元素に対する耐性を設定する。例: 過負荷なら炎耐性、超電導なら氷耐性。呼び出し側が適切な値を設定する責任を持つ。

### 月反応（Lunar）

| 反応 | 倍率 | 関連元素 |
|------|------|---------|
| 月感電（LunarElectroCharged） | 1.8 | Hydro + Electro |
| 月開花（LunarBloom） | 1.0 | Hydro + Dendro |
| 月結晶・反応（LunarCrystallize） | 0.96 | Hydro + Geo |
| 月結晶・派生（LunarCrystallizeSecondary） | 1.6 | Hydro + Geo |

```
月反応ダメージ = 倍率 × レベル基礎値 × (1 + EM_bonus + reaction_bonus) × 会心 × 耐性補正
EM_bonus = 6.0 × EM / (EM + 2000)
```

**会心あり**（固定反応と異なる）、防御補正なし。

## データモデル

### reaction.rs

```rust
use serde::{Deserialize, Serialize};
use crate::types::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Reaction {
    // 増幅
    Vaporize,
    Melt,
    // 激化
    Aggravate,
    Spread,
    // 固定
    Overloaded,
    Superconduct,
    ElectroCharged,
    Shattered,
    Swirl(Element),
    Bloom,
    Hyperbloom,
    Burgeon,
    Burning,
    // 月反応
    LunarElectroCharged,
    LunarBloom,
    LunarCrystallize,
    LunarCrystallizeSecondary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReactionCategory {
    Amplifying,
    Catalyze,
    Transformative,
    Lunar,
}

impl Reaction {
    pub fn category(&self) -> ReactionCategory { ... }
}

/// trigger × aura から増幅/固定/月反応を判定。
/// 激化（Aggravate/Spread）はQuicken状態に依存するためこの関数では判定できない。
/// 存在しない組み合わせはNone。
pub fn determine_reaction(trigger: Element, aura: Element) -> Option<Reaction>

/// 増幅反応の基本倍率を返す。Vaporize/Melt以外はNone。
/// trigger: 攻撃元素, aura: 付着元素
pub fn amplifying_multiplier(trigger: Element, aura: Element) -> Option<f64>

/// 激化反応の倍率係数を返す。Aggravate→1.15, Spread→1.25, その他→None。
pub fn catalyze_coefficient(reaction: Reaction) -> Option<f64>

/// 固定反応の倍率を返す。Transformativeカテゴリ以外はNone。
pub fn transformative_multiplier(reaction: Reaction) -> Option<f64>

/// 月反応の倍率を返す。Lunarカテゴリ以外はNone。
pub fn lunar_multiplier(reaction: Reaction) -> Option<f64>

/// 固定反応のダメージ元素を返す。Shattered→None（物理）、Swirl(e)→Some(e)。
pub fn transformative_element(reaction: Reaction) -> Option<Option<Element>>
```

### em.rs

```rust
/// 増幅反応（蒸発・溶解）の元素熟知ボーナス
pub fn amplifying_em_bonus(em: f64) -> f64 {
    2.78 * em / (em + 1400.0)
}

/// 激化反応（超激化・草激化）の元素熟知ボーナス
pub fn catalyze_em_bonus(em: f64) -> f64 {
    5.0 * em / (em + 1200.0)
}

/// 固定反応の元素熟知ボーナス
pub fn transformative_em_bonus(em: f64) -> f64 {
    16.0 * em / (em + 2000.0)
}

/// 月反応の元素熟知ボーナス
pub fn lunar_em_bonus(em: f64) -> f64 {
    6.0 * em / (em + 2000.0)
}
```

EM=0のとき全式で0.0を返す（ゼロ除算なし、分母は常に正）。

### level_table.rs

```rust
/// キャラレベルごとの反応基礎値（Lv1〜100）
/// ソース: DimbreathBot/AnimeGameData v6.0.0 playerElementLevelCo
/// 無効なレベル（0, 101以上）はNoneを返す
pub fn reaction_base_value(level: u32) -> Option<f64>
```

完全なテーブル（100エントリ）を定数配列として埋め込む。

### DamageInput の拡張（damage.rs）

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageInput {
    // v0.1.0 既存
    pub character_level: u32,
    pub stats: Stats,
    pub talent_multiplier: f64,
    pub damage_type: DamageType,
    pub element: Option<Element>,
    // v0.2.0 追加
    pub reaction: Option<Reaction>,   // None = 反応なし（v0.1.0互換）
    pub reaction_bonus: f64,          // 装備ボーナス（魔女4セット等）。0.0 = なし
}
```

`reaction`は`Reaction` enumを直接指定。増幅反応の倍率判定には`element`（トリガー元素）が必要。

`reaction`に指定できるカテゴリ: `Amplifying`, `Catalyze`のみ。`Transformative`/`Lunar`を指定した場合は`Err(UseTransformativeFunction)`/`Err(UseLunarFunction)`を返す。

### DamageResult の拡張（damage.rs）

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageResult {
    pub non_crit: f64,
    pub crit: f64,
    pub average: f64,
    pub reaction: Option<Reaction>,  // v0.2.0 追加
}
```

### TransformativeInput / TransformativeResult（transformative.rs）

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformativeInput {
    pub character_level: u32,   // 1..=100
    pub elemental_mastery: f64, // >= 0.0
    pub reaction: Reaction,     // Transformativeカテゴリのみ
    pub reaction_bonus: f64,    // >= 0.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformativeResult {
    pub damage: f64,
    pub damage_element: Option<Element>,  // None = 物理（Shattered）
}

pub fn calculate_transformative(
    input: &TransformativeInput,
    enemy: &Enemy,
) -> Result<TransformativeResult, CalcError>
```

### LunarInput / LunarResult（lunar.rs）

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LunarInput {
    pub character_level: u32,   // 1..=100
    pub elemental_mastery: f64, // >= 0.0
    pub reaction: Reaction,     // Lunarカテゴリのみ
    pub reaction_bonus: f64,    // >= 0.0
    pub crit_rate: f64,         // 0.0..=1.0
    pub crit_dmg: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LunarResult {
    pub non_crit: f64,
    pub crit: f64,
    pub average: f64,
    pub damage_element: Option<Element>,
}

pub fn calculate_lunar(
    input: &LunarInput,
    enemy: &Enemy,
) -> Result<LunarResult, CalcError>
```

## 入力バリデーション

### calculate_damage（既存 + 拡張）

| フィールド | 有効範囲 | エラー |
|-----------|---------|--------|
| `character_level` | 1..=90 | `InvalidCharacterLevel` |
| `enemy.level` | 1..=100 | `InvalidEnemyLevel` |
| `stats.crit_rate` | 0.0..=1.0 | `InvalidCritRate` |
| `enemy.def_reduction` | 0.0..=1.0 | `InvalidDefReduction` |
| `talent_multiplier` | > 0.0 | `InvalidTalentMultiplier` |
| `reaction` | Amplifying/Catalyzeのみ | `UseTransformativeFunction`/`UseLunarFunction` |

増幅反応で`element`がNone（物理）の場合は`InvalidReaction`。

### calculate_transformative

| フィールド | 有効範囲 | エラー |
|-----------|---------|--------|
| `character_level` | 1..=100 | `InvalidCharacterLevel` |
| `enemy.level` | 1..=100 | `InvalidEnemyLevel` |
| `elemental_mastery` | >= 0.0 | `InvalidElementalMastery` |
| `reaction_bonus` | >= 0.0 | `InvalidReactionBonus` |
| `reaction` | Transformativeのみ | `NotTransformative` |
| `Swirl(elem)` | Pyro/Hydro/Electro/Cryoのみ | `InvalidSwirlElement` |

### calculate_lunar

| フィールド | 有効範囲 | エラー |
|-----------|---------|--------|
| `character_level` | 1..=100 | `InvalidCharacterLevel` |
| `enemy.level` | 1..=100 | `InvalidEnemyLevel` |
| `elemental_mastery` | >= 0.0 | `InvalidElementalMastery` |
| `reaction_bonus` | >= 0.0 | `InvalidReactionBonus` |
| `crit_rate` | 0.0..=1.0 | `InvalidCritRate` |
| `reaction` | Lunarのみ | `NotLunar` |

Note: `character_level`は`calculate_damage`では1..=90（プレイアブルキャラ上限）、`calculate_transformative`/`calculate_lunar`では1..=100（レベルテーブルの範囲）。

## エラー型の拡張（error.rs）

```rust
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CalcError {
    // v0.1.0 既存
    #[error("character level must be 1..=90, got {0}")]
    InvalidCharacterLevel(u32),
    #[error("enemy level must be 1..=100, got {0}")]
    InvalidEnemyLevel(u32),
    #[error("crit_rate must be 0.0..=1.0, got {0}")]
    InvalidCritRate(f64),
    #[error("def_reduction must be 0.0..=1.0, got {0}")]
    InvalidDefReduction(f64),
    #[error("talent_multiplier must be > 0.0, got {0}")]
    InvalidTalentMultiplier(f64),
    // v0.2.0 追加
    #[error("amplifying reaction requires an element, but element is None (physical)")]
    AmplifyingRequiresElement,
    #[error("elemental_mastery must be >= 0.0, got {0}")]
    InvalidElementalMastery(f64),
    #[error("reaction_bonus must be >= 0.0, got {0}")]
    InvalidReactionBonus(f64),
    #[error("reaction {0:?} is not amplifying or catalyze; use calculate_transformative")]
    UseTransformativeFunction(Reaction),
    #[error("reaction {0:?} is not amplifying or catalyze; use calculate_lunar")]
    UseLunarFunction(Reaction),
    #[error("reaction {0:?} is not a transformative reaction")]
    NotTransformative(Reaction),
    #[error("reaction {0:?} is not a lunar reaction")]
    NotLunar(Reaction),
    #[error("swirl element must be Pyro, Hydro, Electro, or Cryo, got {0:?}")]
    InvalidSwirlElement(Element),
    #[error("character level must be 1..=100 for reaction calculations, got {0}")]
    InvalidReactionLevel(u32),
}
```

Note: `InvalidCharacterLevel`はcalculate_damage用（1..=90）、`InvalidReactionLevel`はcalculate_transformative/lunar用（1..=100）。

## 計算フロー

### calculate_damage（増幅/激化対応）

```
1. validate(input, enemy) — v0.1.0バリデーション
2. catalyze_flat = 0.0
3. amplify_mult = 1.0
4. reaction_result = None
5. if reaction あり:
     match reaction.category():
       Amplifying →
         element が None なら Err(AmplifyingRequiresElement)
         multiplier = amplifying_multiplier(element, ...) // trigger=element
         em_bonus = amplifying_em_bonus(stats.elemental_mastery)
         amplify_mult = multiplier × (1 + em_bonus + reaction_bonus)
         reaction_result = Some(reaction)
       Catalyze →
         coeff = catalyze_coefficient(reaction)
         em_bonus = catalyze_em_bonus(stats.elemental_mastery)
         level_base = reaction_base_value(character_level)
         catalyze_flat = coeff × level_base × (1 + em_bonus + reaction_bonus)
         reaction_result = Some(reaction)
       Transformative →
         return Err(UseTransformativeFunction)
       Lunar →
         return Err(UseLunarFunction)
6. base = ATK × talent_mult + catalyze_flat
7. non_crit = base × (1 + dmg_bonus) × def_mult × res_mult × amplify_mult
8. crit = non_crit × (1 + crit_dmg)
9. average = non_crit × (1 - crit_rate) + crit × crit_rate
10. return DamageResult { non_crit, crit, average, reaction: reaction_result }
```

`reaction: None`ならステップ5をスキップし、v0.1.0と同じ結果。

### calculate_transformative（固定反応）

```
1. validate(input, enemy) — level 1..=100, EM >= 0, reaction_bonus >= 0
2. reaction が Transformative カテゴリか検証
3. Swirl(elem)のelemバリデーション（Pyro/Hydro/Electro/Cryoのみ）
4. level_base = reaction_base_value(character_level)
5. reaction_mult = transformative_multiplier(reaction)
6. em_bonus = transformative_em_bonus(elemental_mastery)
7. damage_elem = transformative_element(reaction)
8. damage = level_base × reaction_mult × (1 + em_bonus + reaction_bonus) × res_mult
9. return TransformativeResult { damage, damage_element }
```

### calculate_lunar（月反応）

```
1. validate(input, enemy) — level 1..=100, EM >= 0, crit_rate 0..=1, reaction_bonus >= 0
2. reaction が Lunar カテゴリか検証
3. level_base = reaction_base_value(character_level)
4. reaction_mult = lunar_multiplier(reaction)
5. em_bonus = lunar_em_bonus(elemental_mastery)
6. non_crit = level_base × reaction_mult × (1 + em_bonus + reaction_bonus) × res_mult
7. crit = non_crit × (1 + crit_dmg)
8. average = non_crit × (1 - crit_rate) + crit × crit_rate
9. return LunarResult { non_crit, crit, average, damage_element }
```

## レベル基礎値テーブル

ソース: `DimbreathBot/AnimeGameData` v6.0.0 `playerElementLevelCo`
ゲームバージョン: 6.0.0（2025年9月時点）

```rust
const LEVEL_TABLE: [f64; 100] = [
    17.165606,   18.535048,   19.904854,   21.274902,   22.6454,     // Lv1-5
    24.649612,   26.640642,   28.868587,   31.36768,    34.143345,   // Lv6-10
    37.201,      40.66,       44.446667,   48.56352,    53.74848,    // Lv11-15
    59.081898,   64.420044,   69.72446,    75.12314,    80.58478,    // Lv16-20
    86.11203,    91.70374,    97.24463,    102.812645,  108.40956,   // Lv21-25
    113.20169,   118.102905,  122.97932,   129.72733,   136.29291,   // Lv26-30
    142.67085,   149.02902,   155.41699,   161.8255,    169.10631,   // Lv31-35
    176.51808,   184.07274,   191.70952,   199.55692,   207.38205,   // Lv36-40
    215.3989,    224.16566,   233.50217,   243.35057,   256.06308,   // Lv41-45
    268.5435,    281.52606,   295.01364,   309.0672,    323.6016,    // Lv46-50
    336.75754,   350.5303,    364.4827,    378.61917,   398.6004,    // Lv51-55
    416.39825,   434.387,     452.95105,   472.60623,   492.8849,    // Lv56-60
    513.56854,   539.1032,    565.51056,   592.53876,   624.4434,    // Lv61-65
    651.47015,   679.4968,    707.79407,   736.67145,   765.64026,   // Lv66-70
    794.7734,    824.67737,   851.1578,    877.74207,   914.2291,    // Lv71-75
    946.74677,   979.4114,    1011.223,    1044.7917,   1077.4437,   // Lv76-80
    1109.9976,   1142.9766,   1176.3695,   1210.1844,   1253.8357,   // Lv81-85
    1288.9528,   1325.4841,   1363.4569,   1405.0974,   1446.8535,   // Lv86-90
    1462.788,    1475.6956,   1497.9644,   1516.9423,   1561.468,    // Lv91-95
    1593.5062,   1621.0258,   1643.8679,   1662.1382,   1674.8092,   // Lv96-100
];
```

## 公開API

### 既存の拡張
```rust
pub use damage::{calculate_damage, DamageInput, DamageResult};
```

### 新規追加
```rust
pub use reaction::{determine_reaction, Reaction, ReactionCategory};
pub use transformative::{calculate_transformative, TransformativeInput, TransformativeResult};
pub use lunar::{calculate_lunar, LunarInput, LunarResult};
pub use em::{amplifying_em_bonus, catalyze_em_bonus, transformative_em_bonus, lunar_em_bonus};
pub use level_table::reaction_base_value;
```

## テスト戦略

### テストカテゴリ

| カテゴリ | テスト内容 | テスト数目安 |
|---------|----------|------------|
| reaction.rs | `determine_reaction` 全組み合わせ、`category()` | ~15 |
| em.rs | 4種のEM式 + EM=0エッジケース | ~10 |
| level_table.rs | テーブル参照 + 境界値(Lv1,90,100) + 無効値 | ~5 |
| damage.rs (増幅) | 蒸発1.5x/2.0x、溶解1.5x/2.0x + EM | ~6 |
| damage.rs (激化) | 超激化、草激化 + EMあり/なし | ~6 |
| damage.rs (後方互換) | reaction:None でv0.1.0と同じ結果 | ~2 |
| damage.rs (エラー) | Transformative/Lunar指定時のエラー | ~3 |
| transformative.rs | 全9反応 + Swirlバリデーション | ~12 |
| lunar.rs | 月感電・月開花・月結晶 + 会心テスト | ~6 |
| バリデーション | 不正入力のエラーテスト | ~5 |

### 方針
- 浮動小数点の比較は `const EPSILON: f64 = 1e-6` を統一使用
- ゴールデンテストは `0.01` 許容誤差
- v0.1.0の既存33テストは `reaction: None, reaction_bonus: 0.0` を追加して全て通ること
- wiki掲載の計算例と照合するゴールデンテストを各カテゴリに最低1つ

## 依存crate（変更なし）

| crate | バージョン | 用途 |
|-------|-----------|------|
| `serde` | 1, features = ["derive"] | シリアライズ/デシリアライズ |
| `serde_json` | 1 | JSONサポート（dev-dependencies） |
| `thiserror` | 2 | エラー型derive |

新しい依存は追加しない。

## 利用イメージ

### 増幅反応（蒸発）

```rust
use genshin_calc_core::*;

let input = DamageInput {
    character_level: 90,
    stats: Stats {
        atk: 1800.0,
        crit_rate: 0.6,
        crit_dmg: 1.2,
        dmg_bonus: 0.466,
        elemental_mastery: 200.0,
        ..Stats::default()
    },
    talent_multiplier: 1.5104,
    damage_type: DamageType::Skill,
    element: Some(Element::Pyro),
    reaction: Some(Reaction::Vaporize),
    reaction_bonus: 0.15, // 魔女4セット
};
let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
let result = calculate_damage(&input, &enemy).unwrap();
// 蒸発1.5x（Pyro trigger on Hydro）× (1 + EM_bonus + 0.15) が適用済み
```

### 激化反応（超激化）

```rust
let input = DamageInput {
    character_level: 90,
    stats: Stats {
        atk: 1500.0,
        crit_rate: 0.7,
        crit_dmg: 1.4,
        dmg_bonus: 0.466,
        elemental_mastery: 150.0,
        ..Stats::default()
    },
    talent_multiplier: 1.2,
    damage_type: DamageType::Skill,
    element: Some(Element::Electro),
    reaction: Some(Reaction::Aggravate),
    reaction_bonus: 0.0,
};
let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
let result = calculate_damage(&input, &enemy).unwrap();
// 超激化フラット加算が基礎ダメージに加算された結果
```

### 固定反応（過負荷）

```rust
let input = TransformativeInput {
    character_level: 90,
    elemental_mastery: 800.0,
    reaction: Reaction::Overloaded,
    reaction_bonus: 0.0,
};
let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
let result = calculate_transformative(&input, &enemy).unwrap();
// result.damage: 過負荷ダメージ
// result.damage_element: Some(Element::Pyro)
```

### 月反応（月感電）

```rust
let input = LunarInput {
    character_level: 90,
    elemental_mastery: 300.0,
    reaction: Reaction::LunarElectroCharged,
    reaction_bonus: 0.0,
    crit_rate: 0.5,
    crit_dmg: 1.0,
};
let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
let result = calculate_lunar(&input, &enemy).unwrap();
// result.non_crit, result.crit, result.average
```

### 反応なし（v0.1.0互換）

```rust
let input = DamageInput {
    character_level: 90,
    stats: Stats { atk: 2000.0, crit_rate: 0.5, crit_dmg: 1.0, dmg_bonus: 0.466, ..Stats::default() },
    talent_multiplier: 1.76,
    damage_type: DamageType::Skill,
    element: Some(Element::Pyro),
    reaction: None,        // 反応なし
    reaction_bonus: 0.0,
};
// v0.1.0と同じ結果
```
