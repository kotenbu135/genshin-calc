use serde::{Deserialize, Serialize};

use crate::damage::resistance_multiplier;
use crate::em::lunar_em_bonus;
use crate::enemy::Enemy;
use crate::error::CalcError;
use crate::level_table::reaction_base_value;
use crate::reaction::{Reaction, ReactionCategory, lunar_element, lunar_multiplier};
use crate::types::Element;

/// Input for lunar reaction damage calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LunarInput {
    /// Character level (1-100).
    pub character_level: u32,
    /// Elemental mastery.
    pub elemental_mastery: f64,
    /// Lunar reaction type.
    pub reaction: Reaction,
    /// Reaction DMG bonus in decimal form.
    pub reaction_bonus: f64,
    /// Crit rate in decimal form.
    pub crit_rate: f64,
    /// Crit DMG in decimal form.
    pub crit_dmg: f64,
    /// Base DMG Bonus from Moonsign Benediction passives (decimal form, default 0.0).
    pub base_dmg_bonus: f64,
}

/// Result of lunar reaction damage calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LunarResult {
    /// Damage without critical hit.
    pub non_crit: f64,
    /// Damage with critical hit.
    pub crit: f64,
    /// Average damage (weighted by crit rate).
    pub average: f64,
    /// Element of the reaction damage.
    pub damage_element: Option<Element>,
}

fn validate(input: &LunarInput, enemy: &Enemy) -> Result<(), CalcError> {
    if !(1..=100).contains(&input.character_level) {
        return Err(CalcError::InvalidReactionLevel(input.character_level));
    }
    if !(1..=100).contains(&enemy.level) {
        return Err(CalcError::InvalidEnemyLevel(enemy.level));
    }
    if input.elemental_mastery < 0.0 {
        return Err(CalcError::InvalidElementalMastery(input.elemental_mastery));
    }
    if input.reaction_bonus < 0.0 {
        return Err(CalcError::InvalidReactionBonus(input.reaction_bonus));
    }
    if !(0.0..=1.0).contains(&input.crit_rate) {
        return Err(CalcError::InvalidCritRate(input.crit_rate));
    }
    if input.reaction.category() != ReactionCategory::Lunar {
        return Err(CalcError::NotLunar(input.reaction));
    }
    Ok(())
}

/// Calculates lunar reaction damage.
///
/// Lunar reactions scale with character level and elemental mastery like
/// transformative reactions, but they can also crit. They ignore ATK,
/// talent multipliers, and defense.
///
/// # Errors
///
/// Returns [`CalcError`] if the reaction is not lunar or inputs are invalid.
///
/// # Examples
///
/// ```
/// use genshin_calc_core::*;
///
/// let input = LunarInput {
///     character_level: 90,
///     elemental_mastery: 300.0,
///     reaction: Reaction::LunarElectroCharged,
///     reaction_bonus: 0.0,
///     crit_rate: 0.60,
///     crit_dmg: 1.20,
///     base_dmg_bonus: 0.0,
/// };
/// let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0, def_ignore: 0.0 };
/// let result = calculate_lunar(&input, &enemy).unwrap();
/// assert!(result.crit > result.non_crit);
/// ```
pub fn calculate_lunar(input: &LunarInput, enemy: &Enemy) -> Result<LunarResult, CalcError> {
    validate(input, enemy)?;

    let level_base = reaction_base_value(input.character_level).expect("validated: level 1..=100");
    let reaction_mult = lunar_multiplier(input.reaction).expect("validated: Lunar reaction");
    let em_bonus = lunar_em_bonus(input.elemental_mastery);
    let res_mult = resistance_multiplier(enemy);

    let non_crit = level_base
        * reaction_mult
        * (1.0 + input.base_dmg_bonus)
        * (1.0 + em_bonus + input.reaction_bonus)
        * res_mult;
    let crit = non_crit * (1.0 + input.crit_dmg);
    let average = non_crit * (1.0 - input.crit_rate) + crit * input.crit_rate;

    let damage_element = lunar_element(input.reaction).expect("validated: Lunar reaction");

    Ok(LunarResult {
        non_crit,
        crit,
        average,
        damage_element,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    fn default_enemy() -> Enemy {
        Enemy {
            level: 90,
            resistance: 0.1,
            def_reduction: 0.0,
            def_ignore: 0.0,
        }
    }

    #[test]
    fn test_lunar_electro_charged_no_em() {
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 0.5,
            crit_dmg: 1.0,
            base_dmg_bonus: 0.0,
        };
        // 1446.8535 * 1.8 * 1.0 * 0.9 = 2343.9...
        let expected_non_crit = 1446.8535 * 1.8 * 0.9;
        let result = calculate_lunar(&input, &default_enemy()).unwrap();
        assert!((result.non_crit - expected_non_crit).abs() < 0.01);
        assert!((result.crit - expected_non_crit * 2.0).abs() < 0.01);
        assert_eq!(result.damage_element, Some(Element::Electro));
    }

    #[test]
    fn test_lunar_bloom() {
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarBloom,
            reaction_bonus: 0.0,
            crit_rate: 0.0,
            crit_dmg: 0.0,
            base_dmg_bonus: 0.0,
        };
        let expected = 1446.8535 * 1.0 * 0.9;
        let result = calculate_lunar(&input, &default_enemy()).unwrap();
        assert!((result.non_crit - expected).abs() < 0.01);
        assert!((result.average - result.non_crit).abs() < EPSILON);
    }

    #[test]
    fn test_lunar_crystallize() {
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarCrystallize,
            reaction_bonus: 0.0,
            crit_rate: 0.5,
            crit_dmg: 1.0,
            base_dmg_bonus: 0.0,
        };
        let expected = 1446.8535 * 0.96 * 0.9;
        let result = calculate_lunar(&input, &default_enemy()).unwrap();
        assert!((result.non_crit - expected).abs() < 0.01);
    }

    #[test]
    fn test_lunar_crit_applied() {
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 1.0,
            crit_dmg: 1.0,
            base_dmg_bonus: 0.0,
        };
        let result = calculate_lunar(&input, &default_enemy()).unwrap();
        assert!((result.average - result.crit).abs() < EPSILON);
    }

    // =====================================================================
    // Golden tests: hand-calculated lunar reaction values
    // =====================================================================

    #[test]
    fn test_golden_lunar_electro_charged_em500() {
        // Lv90, EM 500, Lunar EC (1.8), crit_rate 0.6, crit_dmg 1.2
        // em_bonus = 6 * 500 / (500 + 2000) = 3000/2500 = 1.2
        // non_crit = 1446.8535 * 1.8 * (1 + 1.2) * 0.9
        //          = 2604.336 * 2.2 * 0.9 = 5156.586
        // crit = 5156.586 * (1 + 1.2) = 11344.489
        // avg = 5156.586 * 0.4 + 11344.489 * 0.6 = 8869.328
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 500.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 0.6,
            crit_dmg: 1.2,
            base_dmg_bonus: 0.0,
        };
        let result = calculate_lunar(&input, &default_enemy()).unwrap();
        assert!((result.non_crit - 5156.586).abs() < 0.1);
        assert!((result.crit - 11344.489).abs() < 0.1);
        assert!((result.average - 8869.328).abs() < 0.1);
        assert_eq!(result.damage_element, Some(Element::Electro));
    }

    #[test]
    fn test_lunar_not_lunar_error() {
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
            crit_rate: 0.5,
            crit_dmg: 1.0,
            base_dmg_bonus: 0.0,
        };
        assert!(matches!(
            calculate_lunar(&input, &default_enemy()),
            Err(CalcError::NotLunar(_))
        ));
    }

    #[test]
    fn test_lunar_em_applied() {
        let base = LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 0.0,
            crit_dmg: 0.0,
            base_dmg_bonus: 0.0,
        };
        let with_em = LunarInput {
            elemental_mastery: 300.0,
            ..base.clone()
        };
        let r1 = calculate_lunar(&base, &default_enemy()).unwrap();
        let r2 = calculate_lunar(&with_em, &default_enemy()).unwrap();
        let em_bonus = 6.0 * 300.0 / (300.0 + 2000.0);
        assert!((r2.non_crit / r1.non_crit - (1.0 + em_bonus)).abs() < EPSILON);
    }

    #[test]
    fn test_lunar_base_dmg_bonus_applied() {
        let base = LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 0.0,
            crit_dmg: 0.0,
            base_dmg_bonus: 0.0,
        };
        let with_bonus = LunarInput {
            base_dmg_bonus: 0.21,
            ..base.clone()
        };
        let r1 = calculate_lunar(&base, &default_enemy()).unwrap();
        let r2 = calculate_lunar(&with_bonus, &default_enemy()).unwrap();
        assert!((r2.non_crit / r1.non_crit - 1.21).abs() < EPSILON);
    }

    #[test]
    fn test_golden_lunar_ec_with_base_dmg_bonus() {
        // Lv90, EM 500, Lunar EC (1.8), base_dmg_bonus 0.14
        // em_bonus = 6 * 500 / (500 + 2000) = 1.2
        // non_crit = 1446.8535 * 1.8 * (1 + 0.14) * (1 + 1.2) * 0.9
        //          = 2604.336 * 1.14 * 2.2 * 0.9 = 5878.508
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 500.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 0.6,
            crit_dmg: 1.2,
            base_dmg_bonus: 0.14,
        };
        let result = calculate_lunar(&input, &default_enemy()).unwrap();
        assert!((result.non_crit - 5878.508).abs() < 0.1);
    }
}
