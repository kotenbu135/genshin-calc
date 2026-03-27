use serde::{Deserialize, Serialize};

use crate::error::CalcError;
use crate::stats::Stats;

/// Input for stat calculation with base/percent/flat breakdown.
///
/// Represents character base stats, weapon bonuses, artifact substats, etc.
/// Use [`combine_stats`] to compute final [`Stats`].
///
/// All percentage fields use decimal form (e.g. 46.6% ATK = `0.466`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct StatProfile {
    /// Base HP (character + weapon at current ascension).
    pub base_hp: f64,
    /// Base ATK (character + weapon at current ascension).
    pub base_atk: f64,
    /// Base DEF (character at current ascension).
    pub base_def: f64,
    /// HP% bonus (artifacts, buffs).
    pub hp_percent: f64,
    /// ATK% bonus (artifacts, buffs).
    pub atk_percent: f64,
    /// DEF% bonus (artifacts, buffs).
    pub def_percent: f64,
    /// Flat HP bonus (artifacts, buffs).
    pub hp_flat: f64,
    /// Flat ATK bonus (artifacts, feather).
    pub atk_flat: f64,
    /// Flat DEF bonus (artifacts, buffs).
    pub def_flat: f64,
    /// Elemental mastery.
    pub elemental_mastery: f64,
    /// Crit rate in decimal form.
    pub crit_rate: f64,
    /// Crit DMG in decimal form.
    pub crit_dmg: f64,
    /// Energy recharge in decimal form.
    pub energy_recharge: f64,
    /// DMG bonus in decimal form.
    pub dmg_bonus: f64,
}

/// Combines a [`StatProfile`] into final [`Stats`].
///
/// Formula: `final = base * (1 + percent) + flat`
///
/// # Errors
///
/// Returns [`CalcError`] if any input value is out of valid range
/// (e.g. negative base values, percent bonus below -1.0).
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
        return Err(CalcError::InvalidElementalMastery(
            profile.elemental_mastery,
        ));
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
        let result = combine_stats(&profile);
        assert!(result.is_ok());
        assert!(result.unwrap().atk.is_nan());
    }

    #[test]
    fn test_validate_edge_percent_minus_one() {
        let profile = StatProfile {
            base_atk: 1000.0,
            atk_percent: -1.0,
            ..StatProfile::default()
        };
        let stats = combine_stats(&profile).unwrap();
        assert!((stats.atk).abs() < EPSILON);
    }

    // =====================================================================
    // Golden test: hand-calculated stat combination
    // =====================================================================

    #[test]
    fn test_golden_typical_dps_build() {
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
}
