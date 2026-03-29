# v0.3.0 ステータス管理 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** キャラ・武器・聖遺物のステータス合算関数(`combine_stats`)とHP/DEFスケーリング(`ScalingStat`)をcoreクレートに追加する。

**Architecture:** `StatProfile`で基礎値+%+flat値を受け取り、`combine_stats`で`Stats`に合算する。`DamageInput`に`scaling_stat`フィールドを追加し、ATK以外(HP/DEF)基準のダメージ計算に対応する。既存の41テストは`scaling_stat: ScalingStat::Atk`を明示追加して後方互換を維持する。

**Tech Stack:** Rust (edition 2024), serde 1, thiserror 2, serde_json 1 (dev)

**Spec:** `docs/superpowers/specs/2026-03-25-stat-management-design.md`

---

## File Structure

| ファイル | 責務 | 変更 |
|---------|------|------|
| `crates/core/src/types.rs` | 型定義 (Element, DamageType, ScalingStat) | `ScalingStat` enum 追加 |
| `crates/core/src/error.rs` | エラー型 | 6バリアント追加 |
| `crates/core/src/stat_profile.rs` | **新規** — StatProfile構造体 + combine_stats関数 | 作成 |
| `crates/core/src/damage.rs` | ダメージ計算 | DamageInputにscaling_stat追加、base計算分岐 |
| `crates/core/src/lib.rs` | 公開API | pub mod + re-export追加 |

---

### Task 1: ScalingStat enum と エラーバリアント追加

**Files:**
- Modify: `crates/core/src/types.rs:1-22`
- Modify: `crates/core/src/error.rs:1-50`

- [ ] **Step 1: ScalingStat enumをtypes.rsに追加**

`crates/core/src/types.rs`の末尾（22行目以降）に追加:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ScalingStat {
    #[default]
    Atk,
    Hp,
    Def,
}
```

- [ ] **Step 2: エラーバリアントをerror.rsに追加**

`crates/core/src/error.rs`の`InvalidReactionLevel`バリアント（49行目）の後に追加:

```rust
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
```

- [ ] **Step 3: ビルド確認**

Run: `cargo build -p genshin-calc-core`
Expected: PASS（未使用警告は出るが、コンパイルは成功）

- [ ] **Step 4: コミット**

```bash
git add crates/core/src/types.rs crates/core/src/error.rs
git commit -m "feat: add ScalingStat enum and new CalcError variants for v0.3.0"
```

---

### Task 2: StatProfile構造体とcombine_stats関数（TDD）

**Files:**
- Create: `crates/core/src/stat_profile.rs`
- Modify: `crates/core/src/lib.rs:1-10` (pub mod追加)

- [ ] **Step 1: stat_profile.rsのスケルトンとlib.rs登録**

`crates/core/src/stat_profile.rs`を作成:

```rust
use serde::{Deserialize, Serialize};

use crate::error::CalcError;
use crate::stats::Stats;

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StatProfile {
    pub base_hp: f64,
    pub base_atk: f64,
    pub base_def: f64,
    pub hp_percent: f64,
    pub atk_percent: f64,
    pub def_percent: f64,
    pub hp_flat: f64,
    pub atk_flat: f64,
    pub def_flat: f64,
    pub elemental_mastery: f64,
    pub crit_rate: f64,
    pub crit_dmg: f64,
    pub energy_recharge: f64,
    pub dmg_bonus: f64,
}

pub fn combine_stats(_profile: &StatProfile) -> Result<Stats, CalcError> {
    todo!()
}
```

`crates/core/src/lib.rs`の`pub mod types;`の後（10行目の後）に追加:

```rust
pub mod stat_profile;
```

re-export行（21行目付近）に追加:

```rust
pub use stat_profile::{StatProfile, combine_stats};
pub use types::{DamageType, Element, ScalingStat};
```

注意: 既存の`pub use types::{DamageType, Element};`を上記に置き換える（`ScalingStat`追加）。

- [ ] **Step 2: ビルド確認**

Run: `cargo build -p genshin-calc-core`
Expected: PASS

- [ ] **Step 3: 基本合算テストを書く（RED）**

`crates/core/src/stat_profile.rs`の末尾に追加:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_combine_basic() {
        let profile = StatProfile {
            base_hp: 10000.0,
            base_atk: 500.0,
            base_def: 600.0,
            hp_percent: 0.466,
            atk_percent: 0.466,
            def_percent: 0.30,
            hp_flat: 4780.0,
            atk_flat: 311.0,
            def_flat: 0.0,
            elemental_mastery: 100.0,
            crit_rate: 0.5,
            crit_dmg: 1.0,
            energy_recharge: 1.2,
            dmg_bonus: 0.466,
        };
        let stats = combine_stats(&profile).unwrap();
        // final_hp  = 10000 × 1.466 + 4780 = 19440.0
        assert!((stats.hp - 19440.0).abs() < EPSILON);
        // final_atk = 500 × 1.466 + 311 = 1044.0
        assert!((stats.atk - 1044.0).abs() < EPSILON);
        // final_def = 600 × 1.30 + 0 = 780.0
        assert!((stats.def - 780.0).abs() < EPSILON);
        // copy-through fields
        assert!((stats.elemental_mastery - 100.0).abs() < EPSILON);
        assert!((stats.crit_rate - 0.5).abs() < EPSILON);
        assert!((stats.crit_dmg - 1.0).abs() < EPSILON);
        assert!((stats.energy_recharge - 1.2).abs() < EPSILON);
        assert!((stats.dmg_bonus - 0.466).abs() < EPSILON);
    }

    #[test]
    fn test_combine_default_profile() {
        let stats = combine_stats(&StatProfile::default()).unwrap();
        assert!((stats.hp).abs() < EPSILON);
        assert!((stats.atk).abs() < EPSILON);
        assert!((stats.def).abs() < EPSILON);
        assert!((stats.elemental_mastery).abs() < EPSILON);
        assert!((stats.crit_rate).abs() < EPSILON);
        assert!((stats.crit_dmg).abs() < EPSILON);
        assert!((stats.energy_recharge).abs() < EPSILON);
        assert!((stats.dmg_bonus).abs() < EPSILON);
    }

    #[test]
    fn test_combine_percent_only() {
        let profile = StatProfile {
            base_atk: 800.0,
            atk_percent: 0.50,
            ..StatProfile::default()
        };
        let stats = combine_stats(&profile).unwrap();
        // 800 × 1.50 + 0 = 1200.0
        assert!((stats.atk - 1200.0).abs() < EPSILON);
    }

    #[test]
    fn test_combine_flat_only() {
        let profile = StatProfile {
            base_atk: 800.0,
            atk_flat: 311.0,
            ..StatProfile::default()
        };
        let stats = combine_stats(&profile).unwrap();
        // 800 × 1.0 + 311 = 1111.0
        assert!((stats.atk - 1111.0).abs() < EPSILON);
    }
}
```

- [ ] **Step 4: テスト失敗を確認**

Run: `cargo test -p genshin-calc-core stat_profile`
Expected: FAIL（`todo!()` panic）

- [ ] **Step 5: combine_stats実装（GREEN）**

`crates/core/src/stat_profile.rs`の`combine_stats`関数を実装に置き換え:

```rust
pub fn combine_stats(profile: &StatProfile) -> Result<Stats, CalcError> {
    validate(profile)?;

    Ok(Stats {
        hp: profile.base_hp * (1.0 + profile.hp_percent) + profile.hp_flat,
        atk: profile.base_atk * (1.0 + profile.atk_percent) + profile.atk_flat,
        def: profile.base_def * (1.0 + profile.def_percent) + profile.def_flat,
        elemental_mastery: profile.elemental_mastery,
        crit_rate: profile.crit_rate,
        crit_dmg: profile.crit_dmg,
        energy_recharge: profile.energy_recharge,
        dmg_bonus: profile.dmg_bonus,
    })
}

fn validate(profile: &StatProfile) -> Result<(), CalcError> {
    if profile.base_hp < 0.0 {
        return Err(CalcError::InvalidBaseValue(profile.base_hp));
    }
    if profile.base_atk < 0.0 {
        return Err(CalcError::InvalidBaseValue(profile.base_atk));
    }
    if profile.base_def < 0.0 {
        return Err(CalcError::InvalidBaseValue(profile.base_def));
    }
    if profile.hp_percent < -1.0 {
        return Err(CalcError::InvalidPercentBonus(profile.hp_percent));
    }
    if profile.atk_percent < -1.0 {
        return Err(CalcError::InvalidPercentBonus(profile.atk_percent));
    }
    if profile.def_percent < -1.0 {
        return Err(CalcError::InvalidPercentBonus(profile.def_percent));
    }
    if profile.hp_flat < 0.0 {
        return Err(CalcError::InvalidFlatBonus(profile.hp_flat));
    }
    if profile.atk_flat < 0.0 {
        return Err(CalcError::InvalidFlatBonus(profile.atk_flat));
    }
    if profile.def_flat < 0.0 {
        return Err(CalcError::InvalidFlatBonus(profile.def_flat));
    }
    if profile.elemental_mastery < 0.0 {
        return Err(CalcError::InvalidElementalMastery(profile.elemental_mastery));
    }
    if profile.crit_rate < 0.0 {
        return Err(CalcError::InvalidCritRate(profile.crit_rate));
    }
    if profile.crit_dmg < 0.0 {
        return Err(CalcError::InvalidCritDmg(profile.crit_dmg));
    }
    if profile.energy_recharge < 0.0 {
        return Err(CalcError::InvalidEnergyRecharge(profile.energy_recharge));
    }
    if profile.dmg_bonus < -1.0 {
        return Err(CalcError::InvalidDmgBonus(profile.dmg_bonus));
    }
    Ok(())
}
```

- [ ] **Step 6: テスト成功を確認**

Run: `cargo test -p genshin-calc-core stat_profile`
Expected: 4 tests PASS

- [ ] **Step 7: コミット**

```bash
git add crates/core/src/stat_profile.rs crates/core/src/lib.rs
git commit -m "feat: add StatProfile and combine_stats with basic tests"
```

---

### Task 3: combine_statsバリデーションテストとgoldenテスト

**Files:**
- Modify: `crates/core/src/stat_profile.rs` (テスト追加)

- [ ] **Step 1: バリデーションテストを追加**

`stat_profile.rs`のテストモジュール末尾に追加:

```rust
    #[test]
    fn test_validate_negative_base() {
        let profile = StatProfile {
            base_atk: -1.0,
            ..StatProfile::default()
        };
        assert!(matches!(
            combine_stats(&profile),
            Err(CalcError::InvalidBaseValue(v)) if v < 0.0
        ));
    }

    #[test]
    fn test_validate_percent_too_low() {
        let profile = StatProfile {
            atk_percent: -1.1,
            ..StatProfile::default()
        };
        assert!(matches!(
            combine_stats(&profile),
            Err(CalcError::InvalidPercentBonus(v)) if v < -1.0
        ));
    }

    #[test]
    fn test_validate_negative_flat() {
        let profile = StatProfile {
            atk_flat: -1.0,
            ..StatProfile::default()
        };
        assert!(matches!(
            combine_stats(&profile),
            Err(CalcError::InvalidFlatBonus(v)) if v < 0.0
        ));
    }

    #[test]
    fn test_validate_negative_em() {
        let profile = StatProfile {
            elemental_mastery: -1.0,
            ..StatProfile::default()
        };
        assert!(matches!(
            combine_stats(&profile),
            Err(CalcError::InvalidElementalMastery(_))
        ));
    }

    #[test]
    fn test_validate_negative_crit_rate() {
        let profile = StatProfile {
            crit_rate: -0.1,
            ..StatProfile::default()
        };
        assert!(matches!(
            combine_stats(&profile),
            Err(CalcError::InvalidCritRate(_))
        ));
    }

    #[test]
    fn test_validate_negative_crit_dmg() {
        let profile = StatProfile {
            crit_dmg: -0.1,
            ..StatProfile::default()
        };
        assert!(matches!(
            combine_stats(&profile),
            Err(CalcError::InvalidCritDmg(_))
        ));
    }

    #[test]
    fn test_validate_negative_energy_recharge() {
        let profile = StatProfile {
            energy_recharge: -0.1,
            ..StatProfile::default()
        };
        assert!(matches!(
            combine_stats(&profile),
            Err(CalcError::InvalidEnergyRecharge(_))
        ));
    }

    #[test]
    fn test_validate_dmg_bonus_too_low() {
        let profile = StatProfile {
            dmg_bonus: -1.1,
            ..StatProfile::default()
        };
        assert!(matches!(
            combine_stats(&profile),
            Err(CalcError::InvalidDmgBonus(_))
        ));
    }

    #[test]
    fn test_validate_nan_base() {
        let profile = StatProfile {
            base_atk: f64::NAN,
            ..StatProfile::default()
        };
        // NaN fails `< 0.0` check (NaN < 0.0 is false), but also fails `>= 0.0`
        // NaN comparison: NaN < 0.0 → false, so validation passes?
        // Actually: `if profile.base_atk < 0.0` → false for NaN, so NaN slips through.
        // This is acceptable — NaN propagates through calculations and is detectable in output.
        // Just verify it doesn't panic.
        let result = combine_stats(&profile);
        assert!(result.is_ok());
        assert!(result.unwrap().atk.is_nan());
    }

    #[test]
    fn test_validate_edge_percent_minus_one() {
        // percent = -1.0 is the boundary (allowed)
        let profile = StatProfile {
            base_atk: 1000.0,
            atk_percent: -1.0,
            ..StatProfile::default()
        };
        let stats = combine_stats(&profile).unwrap();
        // 1000 × (1 + (-1.0)) + 0 = 0.0
        assert!((stats.atk).abs() < EPSILON);
    }

    // =====================================================================
    // Golden test: hand-calculated stat combination
    // =====================================================================

    #[test]
    fn test_golden_typical_dps_build() {
        // Typical DPS: base_atk 674 (char 335 + weapon 339)
        // ATK% from goblet 46.6%, ATK flat from feather 311
        // CR 62.2%, CD 124.4% (base 5%/50% + artifacts)
        // Pyro DMG 46.6%, EM 0, ER 100% base
        let profile = StatProfile {
            base_hp: 13103.0,
            base_atk: 674.0,
            base_def: 763.0,
            hp_percent: 0.0,
            atk_percent: 0.466,
            def_percent: 0.0,
            hp_flat: 0.0,
            atk_flat: 311.0,
            def_flat: 0.0,
            elemental_mastery: 0.0,
            crit_rate: 0.622,
            crit_dmg: 1.244,
            energy_recharge: 1.0,
            dmg_bonus: 0.466,
        };
        let stats = combine_stats(&profile).unwrap();
        // final_atk = 674 × 1.466 + 311 = 988.084 + 311 = 1299.084
        assert!((stats.atk - 1299.084).abs() < 0.01);
        // final_hp = 13103 × 1.0 + 0 = 13103.0
        assert!((stats.hp - 13103.0).abs() < EPSILON);
        // final_def = 763 × 1.0 + 0 = 763.0
        assert!((stats.def - 763.0).abs() < EPSILON);
    }

    #[test]
    fn test_serde_roundtrip() {
        let profile = StatProfile {
            base_hp: 10000.0,
            base_atk: 500.0,
            base_def: 600.0,
            atk_percent: 0.466,
            atk_flat: 311.0,
            crit_rate: 0.5,
            crit_dmg: 1.0,
            ..StatProfile::default()
        };
        let json = serde_json::to_string(&profile).unwrap();
        let deserialized: StatProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(profile, deserialized);
    }
```

- [ ] **Step 2: テスト実行**

Run: `cargo test -p genshin-calc-core stat_profile`
Expected: 16 tests PASS

- [ ] **Step 3: コミット**

```bash
git add crates/core/src/stat_profile.rs
git commit -m "test: add validation and golden tests for combine_stats"
```

---

### Task 4: DamageInputにscaling_stat追加、既存テスト修正、新テスト追加

**Files:**
- Modify: `crates/core/src/damage.rs:1-122` (DamageInput変更、calculate_damage変更)
- Modify: `crates/core/src/damage.rs` (既存テスト全箇所にscaling_stat追加)
- Modify: `crates/core/src/lib.rs` (統合テスト更新)

**重要:** DamageInputにフィールドを追加すると、全てのDamageInputリテラル構築箇所がコンパイルエラーになる。そのため、フィールド追加・valid_input()更新・全直接構築箇所の修正・lib.rs修正を1ステップで行い、コンパイル可能な状態を維持する。

- [ ] **Step 1: DamageInputにscaling_statフィールド追加 + 全構築箇所を一括修正**

`crates/core/src/damage.rs`の変更:

import行（9行目）を変更:
```rust
// 旧
use crate::types::{DamageType, Element};
// 新
use crate::types::{DamageType, Element, ScalingStat};
```

DamageInput構造体（11-20行目）を変更:
```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageInput {
    pub character_level: u32,
    pub stats: Stats,
    pub talent_multiplier: f64,
    #[serde(default)]
    pub scaling_stat: ScalingStat,
    pub damage_type: DamageType,
    pub element: Option<Element>,
    pub reaction: Option<Reaction>,
    pub reaction_bonus: f64,
}
```

`calculate_damage`関数内の107行目を変更:
```rust
    // 旧
    let base = input.stats.atk * input.talent_multiplier + catalyze_flat;
    // 新
    let scaling_value = match input.scaling_stat {
        ScalingStat::Atk => input.stats.atk,
        ScalingStat::Hp => input.stats.hp,
        ScalingStat::Def => input.stats.def,
    };
    let base = scaling_value * input.talent_multiplier + catalyze_flat;
```

`valid_input()`ヘルパー（130-146行目）を更新:
```rust
    fn valid_input() -> DamageInput {
        DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.5,
                crit_dmg: 1.0,
                dmg_bonus: 0.466,
                ..Stats::default()
            },
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
        }
    }
```

damage.rs内で`DamageInput { ... }`を直接構築している全テスト箇所（serde roundtrip, golden tests等）に`scaling_stat: ScalingStat::Atk,`を追加。`valid_input()`を使わず直接構築している箇所を`DamageInput {`で検索して全て修正する。

`crates/core/src/lib.rs`の統合テスト（40-48行目）も修正:
```rust
        let input = DamageInput {
            character_level: 90,
            stats,
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
        };
```

- [ ] **Step 2: 既存テスト全パスを確認**

Run: `cargo test -p genshin-calc-core`
Expected: 95 tests PASS（既存動作を維持）

- [ ] **Step 3: ScalingStatテストを追加**

`damage.rs`のテストモジュール末尾に追加:

```rust
    // =====================================================================
    // ScalingStat tests
    // =====================================================================

    #[test]
    fn test_scaling_stat_atk_matches_default_behavior() {
        let input = DamageInput {
            scaling_stat: ScalingStat::Atk,
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy()).unwrap();
        // Same as existing: ATK 2000 × 1.76 = 3520 base
        let expected_non_crit = 2000.0 * 1.76 * 1.466 * 0.5 * 0.9;
        assert!((result.non_crit - expected_non_crit).abs() < 0.01);
    }

    #[test]
    fn test_scaling_stat_hp() {
        let input = DamageInput {
            stats: Stats {
                hp: 30000.0,
                atk: 1200.0,
                crit_rate: 0.5,
                crit_dmg: 1.0,
                dmg_bonus: 0.466,
                ..Stats::default()
            },
            talent_multiplier: 0.10,
            scaling_stat: ScalingStat::Hp,
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy()).unwrap();
        // base = HP 30000 × 0.10 = 3000
        let expected_non_crit = 30000.0 * 0.10 * 1.466 * 0.5 * 0.9;
        assert!((result.non_crit - expected_non_crit).abs() < 0.01);
    }

    #[test]
    fn test_scaling_stat_def() {
        let input = DamageInput {
            stats: Stats {
                def: 2500.0,
                atk: 1000.0,
                crit_rate: 0.5,
                crit_dmg: 1.0,
                dmg_bonus: 0.466,
                ..Stats::default()
            },
            talent_multiplier: 0.80,
            scaling_stat: ScalingStat::Def,
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy()).unwrap();
        // base = DEF 2500 × 0.80 = 2000
        let expected_non_crit = 2500.0 * 0.80 * 1.466 * 0.5 * 0.9;
        assert!((result.non_crit - expected_non_crit).abs() < 0.01);
    }

    #[test]
    fn test_scaling_stat_hp_with_vaporize() {
        // HP scaling + amplifying reaction
        let input = DamageInput {
            stats: Stats {
                hp: 30000.0,
                atk: 1000.0,
                elemental_mastery: 100.0,
                crit_rate: 0.5,
                crit_dmg: 1.0,
                dmg_bonus: 0.466,
                ..Stats::default()
            },
            talent_multiplier: 0.10,
            scaling_stat: ScalingStat::Hp,
            element: Some(Element::Hydro),
            reaction: Some(crate::reaction::Reaction::Vaporize),
            reaction_bonus: 0.0,
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy()).unwrap();
        // base = HP 30000 × 0.10 = 3000 (no catalyze)
        // Hydro Vaporize = 2.0x
        // em_bonus = 2.78 × 100 / (100 + 1400) = 0.185333
        // amplify = 2.0 × (1 + 0.185333) = 2.370666
        let expected_non_crit = 30000.0 * 0.10 * 1.466 * 0.5 * 0.9 * 2.0 * (1.0 + 2.78 * 100.0 / 1500.0);
        assert!((result.non_crit - expected_non_crit).abs() < 0.1);
    }

    #[test]
    fn test_scaling_stat_serde_default_backward_compat() {
        // JSON without scaling_stat field should deserialize with default Atk
        let json = r#"{
            "character_level": 90,
            "stats": {"hp":0,"atk":2000,"def":0,"elemental_mastery":0,"crit_rate":0.5,"crit_dmg":1.0,"energy_recharge":0,"dmg_bonus":0.466},
            "talent_multiplier": 1.76,
            "damage_type": "Skill",
            "element": "Pyro",
            "reaction": null,
            "reaction_bonus": 0.0
        }"#;
        let input: DamageInput = serde_json::from_str(json).unwrap();
        assert_eq!(input.scaling_stat, ScalingStat::Atk);
    }
```

- [ ] **Step 4: 全テスト実行**

Run: `cargo test -p genshin-calc-core`
Expected: 全テストPASS（95既存 + 21新規 = 116件）
注: 新規内訳 — stat_profile 16件 + scaling_stat 5件 = 21件

- [ ] **Step 5: Clippy + fmt確認**

Run: `cargo clippy -- -D warnings && cargo fmt --check`
Expected: PASS

- [ ] **Step 6: コミット**

```bash
git add crates/core/src/damage.rs crates/core/src/lib.rs
git commit -m "feat: add scaling_stat to DamageInput for HP/DEF scaling damage"
```

---

### Task 5: README更新

**Files:**
- Modify: `README.md`

- [ ] **Step 1: README使用例を更新**

READMEの使用例セクションに以下を反映:
- 既存の使用例に`scaling_stat: ScalingStat::Atk`を追加
- `StatProfile`/`combine_stats`の使用例を追加
- HP/DEFスケーリングの使用例を追加（スペックの使用例を参照）
- API Overviewテーブルに`combine_stats`行を追加

- [ ] **Step 2: CLAUDE.md更新**

`CLAUDE.md`のArchitectureセクションに追加:
```
- `stat_profile.rs`: ステータス合算（StatProfile → Stats）
```

Testingセクションのテスト数を更新。

- [ ] **Step 3: コミット**

```bash
git add README.md CLAUDE.md
git commit -m "docs: update README and CLAUDE.md for v0.3.0 stat management"
```
