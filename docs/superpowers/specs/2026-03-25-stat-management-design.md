# v0.3.0 ステータス管理 設計書

## Goal

キャラ・武器・聖遺物のステータスを合算して最終Statsを生成する計算ロジックと、HP/DEFスケーリング対応をcoreクレートに追加する。

## Scope

- `StatProfile` → `Stats` 合算関数（基礎値 × (1+%) + flat）
- `ScalingStat` enum（Atk/Hp/Def）で天賦倍率の基準ステータスを指定
- `DamageInput`に`scaling_stat`フィールド追加（デフォルトAtk、後方互換）
- バリデーション付き
- data crateには手を入れない（計算ロジックのみ）

## Out of Scope

- 静的ゲームデータ（キャラ基礎値テーブル、武器データ等）
- 聖遺物セット効果の自動適用
- StatProfileBuilder / フルエントAPI
- 混合スケーリング（ATK+HP等の複合倍率）— 呼び出し側で分割計算

## Breaking Changes

- `DamageInput`に`scaling_stat: ScalingStat`フィールド追加

### マイグレーションガイド

1. `ScalingStat`に`impl Default`を実装し、`Atk`を返す
2. `DamageInput`の`scaling_stat`フィールドに`#[serde(default)]`を付与（JSON逆シリアライズの後方互換）
3. 既存テスト・呼び出しコードには`scaling_stat: ScalingStat::Atk`を明示追加
4. クレートはv0.1.0（pre-1.0）のため、semver上はbreaking changeとして許容

---

## データモデル

### ScalingStat enum

```rust
// types.rs に追加
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ScalingStat {
    #[default]
    Atk,
    Hp,
    Def,
}
```

天賦倍率の基準ステータスを指定する。大半のキャラはAtk。夜蘭等はHp、荒瀧一斗等はDef。

### StatProfile struct

```rust
// stat_profile.rs（新規）
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StatProfile {
    // 基礎値
    pub base_hp: f64,       // キャラ基礎HP（キャラのみ）
    pub base_atk: f64,      // キャラ基礎ATK + 武器基礎ATK
    pub base_def: f64,      // キャラ基礎DEF（キャラのみ）

    // パーセントボーナス（全ソース合算、0.466 = 46.6%）
    pub hp_percent: f64,
    pub atk_percent: f64,
    pub def_percent: f64,

    // フラットボーナス（全ソース合算）
    pub hp_flat: f64,
    pub atk_flat: f64,
    pub def_flat: f64,

    // その他ステータス（全ソース合算値）
    pub elemental_mastery: f64,
    pub crit_rate: f64,       // 0.05 = 5%（キャラ基礎5%込み）
    pub crit_dmg: f64,        // 0.50 = 50%（キャラ基礎50%込み）
    pub energy_recharge: f64, // 1.00 = 100%（基礎100%込み）
    pub dmg_bonus: f64,       // 元素ダメ%等の合算
}
```

**base_atkの注意点:** 原神ではキャラ基礎ATKと武器基礎ATKが合算されてからパーセントボーナスが乗る。`base_atk`にはこの合算値を入れる。base_hp/base_defはキャラのみ（武器はATKのみ提供）。

### DamageInput の変更

```rust
// damage.rs
pub struct DamageInput {
    pub character_level: u32,
    pub stats: Stats,
    pub talent_multiplier: f64,
    pub scaling_stat: ScalingStat,  // 新規
    pub damage_type: DamageType,
    pub element: Option<Element>,
    pub reaction: Option<Reaction>,
    pub reaction_bonus: f64,
}
```

---

## 合算公式

`combine_stats(&StatProfile) -> Result<Stats, CalcError>`

```
final_hp  = base_hp  × (1 + hp_percent)  + hp_flat
final_atk = base_atk × (1 + atk_percent) + atk_flat
final_def = base_def × (1 + def_percent)  + def_flat
```

その他フィールド（elemental_mastery, crit_rate, crit_dmg, energy_recharge, dmg_bonus）はStatProfileからStatsにそのままコピー。

### ダメージ計算の基礎値変更

`calculate_damage`内:

```
// 旧
base_damage = stats.atk × talent_multiplier + catalyze_flat

// 新
scaling_value = match scaling_stat {
    ScalingStat::Atk => stats.atk,
    ScalingStat::Hp  => stats.hp,
    ScalingStat::Def => stats.def,
};
base_damage = scaling_value × talent_multiplier + catalyze_flat
```

---

## バリデーション

### combine_stats のバリデーション

| フィールド | 条件 | エラー |
|-----------|------|--------|
| base_hp, base_atk, base_def | ≥ 0.0 | `InvalidBaseValue(f64)` |
| hp_percent, atk_percent, def_percent | ≥ -1.0 | `InvalidPercentBonus(f64)` |
| hp_flat, atk_flat, def_flat | ≥ 0.0 | `InvalidFlatBonus(f64)` |
| elemental_mastery | ≥ 0.0 | 既存 `InvalidElementalMastery` |
| crit_rate | ≥ 0.0 | 既存 `InvalidCritRate` |
| crit_dmg | ≥ 0.0 | 新規 `InvalidCritDmg(f64)` |
| energy_recharge | ≥ 0.0 | 新規 `InvalidEnergyRecharge(f64)` |
| dmg_bonus | ≥ -1.0 | 新規 `InvalidDmgBonus(f64)` |

**パーセントの下限が-1.0の理由:** 実用上の下限。flat値が大きい場合は-1.0でも最終値が正になり得るが、ゲーム内でこの下限を超えるケースは存在しないため、安全なガードとして採用。

### calculate_damage のバリデーション

既存バリデーションはそのまま。`scaling_stat`はenum型のため不正値がコンパイル時に排除される。

---

## エラー型追加

```rust
// error.rs に追加
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CalcError {
    // ... 既存バリアント ...

    #[error("invalid base value: {0} (must be >= 0)")]
    InvalidBaseValue(f64),

    #[error("invalid percent bonus: {0} (must be >= -1.0)")]
    InvalidPercentBonus(f64),

    #[error("invalid flat bonus: {0} (must be >= 0)")]
    InvalidFlatBonus(f64),

    #[error("crit_dmg must be >= 0.0, got {0}")]
    InvalidCritDmg(f64),

    #[error("energy_recharge must be >= 0.0, got {0}")]
    InvalidEnergyRecharge(f64),

    #[error("dmg_bonus must be >= -1.0, got {0}")]
    InvalidDmgBonus(f64),
}
```

---

## ファイル構成

| ファイル | 変更内容 |
|---------|---------|
| `crates/core/src/types.rs` | `ScalingStat` enum追加 |
| `crates/core/src/stat_profile.rs` | **新規** — `StatProfile`, `combine_stats`, バリデーション |
| `crates/core/src/damage.rs` | `DamageInput`に`scaling_stat`追加、基礎ダメージ計算分岐 |
| `crates/core/src/error.rs` | 6エラーバリアント追加 |
| `crates/core/src/lib.rs` | `pub mod stat_profile` + re-export |

---

## テスト戦略

### combine_stats テスト（stat_profile.rs内）

| テスト | 内容 |
|-------|------|
| 基本合算 | 全フィールドに値を入れて公式通りに合算 |
| ゼロ入力 | `StatProfile::default()` → HP/ATK/DEF=0, その他=0 |
| パーセントのみ | flat=0でbase×(1+percent) |
| フラットのみ | percent=0でbase+flat |
| goldenテスト | 具体的キャラ構成で手計算と照合 |
| バリデーション | 負base, -1.0未満percent, 負flat, NaN入力はmatches!()で検証 |

### ScalingStat テスト（damage.rs内）

| テスト | 内容 |
|-------|------|
| Atk基準 | 既存テストと同結果（後方互換） |
| Hp基準 | HP×倍率でダメージ計算 |
| Def基準 | DEF×倍率でダメージ計算 |
| goldenテスト | HP/DEFスケーリングの手計算値 |

### 後方互換

既存95テストが全パス（`scaling_stat: ScalingStat::Atk`を明示追加）。

想定追加: 15-20テスト、合計110-115件。

---

## 使用例

### ステータス合算

```rust
use genshin_calc_core::{StatProfile, combine_stats};

let profile = StatProfile {
    base_hp: 9352.0,
    base_atk: 554.0,   // キャラ240 + 武器314
    base_def: 572.0,
    atk_percent: 0.466, // 聖遺物杯メイン
    atk_flat: 311.0,    // 聖遺物羽メイン
    crit_rate: 0.05 + 0.311,  // 基礎5% + 聖遺物31.1%
    crit_dmg: 0.50 + 0.622,   // 基礎50% + 聖遺物62.2%
    ..StatProfile::default()
};

let stats = combine_stats(&profile).unwrap();
// stats.atk = 554 × 1.466 + 311 = 1123.164
```

### HP/DEFスケーリング

```rust
use genshin_calc_core::{calculate_damage, DamageInput, ScalingStat, Stats, Enemy, DamageType};

let input = DamageInput {
    character_level: 90,
    stats: Stats {
        hp: 30000.0,
        atk: 1200.0,
        crit_rate: 0.7,
        crit_dmg: 1.5,
        dmg_bonus: 0.466,
        ..Stats::default()
    },
    talent_multiplier: 0.0589, // 夜蘭のスキル倍率（HP基準）
    scaling_stat: ScalingStat::Hp,
    damage_type: DamageType::Skill,
    element: Some(Element::Hydro),
    reaction: None,
    reaction_bonus: 0.0,
};

let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
let result = calculate_damage(&input, &enemy).unwrap();
// base_damage = 30000 × 0.0589 = 1767.0
```
