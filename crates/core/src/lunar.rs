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
    if !(1..=200).contains(&enemy.level) {
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

/// Input for direct lunar damage calculation.
///
/// "Direct lunar damage" refers to talent-originated damage that is *treated
/// as* a lunar reaction (e.g. Ineffa A1, Flins burst middle hit,
/// Columbina skill Gravity, Lauma skill hold, Nefer C6, Zibai skill 2nd hit).
/// The damage scales on a chosen character stat (ATK/HP/DEF/EM) multiplied
/// by a talent multiplier — not on the character level reaction base.
///
/// The damage element is determined by the lunar reaction type.
/// Elemental DMG% bonuses do NOT apply; only the moonsign benediction
/// base DMG bonus and the lunar EM bonus modify the result.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DirectLunarInput {
    /// Character level (1-100). Used for validation only; the damage does
    /// not scale with character level.
    pub character_level: u32,
    /// Talent multiplier in decimal form (e.g. 65% = `0.65`).
    pub talent_multiplier: f64,
    /// The stat value the talent scales on (already selected: ATK, HP, DEF, or EM).
    pub scaling_value: f64,
    /// Elemental mastery for the lunar EM bonus.
    pub elemental_mastery: f64,
    /// Lunar reaction type. Must be one of the `Lunar*` reactions.
    pub reaction: Reaction,
    /// Reaction DMG bonus in decimal form.
    pub reaction_bonus: f64,
    /// Crit rate in decimal form (0.0..=1.0).
    pub crit_rate: f64,
    /// Crit DMG in decimal form.
    pub crit_dmg: f64,
    /// Base DMG Bonus from Moonsign Benediction passives (decimal form).
    pub base_dmg_bonus: f64,
    /// Flat damage added to the base (e.g. weapon flat DMG scaling).
    pub flat_dmg: f64,
}

fn validate_direct(input: &DirectLunarInput, enemy: &Enemy) -> Result<(), CalcError> {
    if !(1..=100).contains(&input.character_level) {
        return Err(CalcError::InvalidReactionLevel(input.character_level));
    }
    if !(1..=200).contains(&enemy.level) {
        return Err(CalcError::InvalidEnemyLevel(enemy.level));
    }
    if input.talent_multiplier <= 0.0 {
        return Err(CalcError::InvalidTalentMultiplier(input.talent_multiplier));
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

/// Calculates direct lunar damage from a talent.
///
/// This pipeline is for talents whose damage is *classified as* lunar-reaction
/// damage while scaling on a character stat and talent multiplier. Examples:
/// Ineffa A1 (ATK-based LunarEC), Lauma skill hold (EM-based LunarBloom),
/// Zibai skill 2nd hit (DEF-based LunarCrystallize).
///
/// Formula:
/// ```text
/// base = talent_multiplier * scaling_value + flat_dmg
/// non_crit = base * (1 + base_dmg_bonus) * (1 + lunar_em_bonus + reaction_bonus) * res_mult
/// crit = non_crit * (1 + crit_dmg)
/// average = non_crit * (1 - crit_rate) + crit * crit_rate
/// ```
///
/// The character level reaction base is NOT used, DEF multiplier is NOT applied,
/// and elemental DMG% is NOT applied.
///
/// # Errors
///
/// Returns [`CalcError`] if the reaction is not lunar or inputs are invalid.
pub fn calculate_direct_lunar(
    input: &DirectLunarInput,
    enemy: &Enemy,
) -> Result<LunarResult, CalcError> {
    validate_direct(input, enemy)?;

    let em_bonus = lunar_em_bonus(input.elemental_mastery);
    let res_mult = resistance_multiplier(enemy);

    let base = input.scaling_value * input.talent_multiplier + input.flat_dmg;
    let non_crit =
        base * (1.0 + input.base_dmg_bonus) * (1.0 + em_bonus + input.reaction_bonus) * res_mult;
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

    // =====================================================================
    // Direct lunar damage tests
    // =====================================================================

    fn direct_valid_input() -> DirectLunarInput {
        DirectLunarInput {
            character_level: 90,
            talent_multiplier: 0.65,
            scaling_value: 2000.0,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 0.5,
            crit_dmg: 1.0,
            base_dmg_bonus: 0.0,
            flat_dmg: 0.0,
        }
    }

    #[test]
    fn test_direct_lunar_golden_atk_scaling() {
        // Ineffa A1-like: ATK 2000, talent 0.65 (65%), LunarEC, no EM
        // base = 2000 * 0.65 = 1300
        // non_crit = 1300 * 1.0 * 1.0 * 0.9 = 1170
        // crit (+100%) = 2340
        // avg = 1170 * 0.5 + 2340 * 0.5 = 1755
        let result = calculate_direct_lunar(&direct_valid_input(), &default_enemy()).unwrap();
        assert!((result.non_crit - 1170.0).abs() < 0.01);
        assert!((result.crit - 2340.0).abs() < 0.01);
        assert!((result.average - 1755.0).abs() < 0.01);
        assert_eq!(result.damage_element, Some(Element::Electro));
    }

    #[test]
    fn test_direct_lunar_em_applied() {
        // EM increases damage via lunar_em_bonus = 6*EM/(EM+2000)
        let base = direct_valid_input();
        let with_em = DirectLunarInput {
            elemental_mastery: 300.0,
            ..base.clone()
        };
        let r1 = calculate_direct_lunar(&base, &default_enemy()).unwrap();
        let r2 = calculate_direct_lunar(&with_em, &default_enemy()).unwrap();
        let em_bonus = 6.0 * 300.0 / (300.0 + 2000.0);
        assert!((r2.non_crit / r1.non_crit - (1.0 + em_bonus)).abs() < EPSILON);
    }

    #[test]
    fn test_direct_lunar_base_dmg_bonus_applied() {
        let base = direct_valid_input();
        let with_bonus = DirectLunarInput {
            base_dmg_bonus: 0.21,
            ..base.clone()
        };
        let r1 = calculate_direct_lunar(&base, &default_enemy()).unwrap();
        let r2 = calculate_direct_lunar(&with_bonus, &default_enemy()).unwrap();
        assert!((r2.non_crit / r1.non_crit - 1.21).abs() < EPSILON);
    }

    #[test]
    fn test_direct_lunar_flat_dmg_applied() {
        // base = scaling_value * talent_multiplier + flat_dmg
        // With flat_dmg = 500: base = 2000*0.65 + 500 = 1800
        // non_crit = 1800 * 0.9 = 1620
        let input = DirectLunarInput {
            flat_dmg: 500.0,
            ..direct_valid_input()
        };
        let result = calculate_direct_lunar(&input, &default_enemy()).unwrap();
        assert!((result.non_crit - 1620.0).abs() < 0.01);
    }

    #[test]
    fn test_direct_lunar_golden_full() {
        // Full combined: EM 300, base_dmg_bonus 0.14, reaction_bonus 0.0, flat 50
        // em_bonus = 6 * 300 / (300 + 2000) = 0.7826086956521739
        // base = 2000 * 0.65 + 50 = 1350
        // non_crit = 1350 * (1 + 0.14) * (1 + 0.7826086956521739) * 0.9 = 2468.635...
        let input = DirectLunarInput {
            elemental_mastery: 300.0,
            base_dmg_bonus: 0.14,
            flat_dmg: 50.0,
            ..direct_valid_input()
        };
        let em_bonus = 6.0 * 300.0 / (300.0 + 2000.0);
        let expected = 1350.0 * 1.14 * (1.0 + em_bonus) * 0.9;
        let result = calculate_direct_lunar(&input, &default_enemy()).unwrap();
        assert!((result.non_crit - expected).abs() < 0.01);
    }

    #[test]
    fn test_direct_lunar_crit_applied() {
        let input = DirectLunarInput {
            crit_rate: 1.0,
            crit_dmg: 1.5,
            ..direct_valid_input()
        };
        let result = calculate_direct_lunar(&input, &default_enemy()).unwrap();
        assert!((result.average - result.crit).abs() < EPSILON);
    }

    #[test]
    fn test_direct_lunar_no_crit_applied() {
        let input = DirectLunarInput {
            crit_rate: 0.0,
            crit_dmg: 1.5,
            ..direct_valid_input()
        };
        let result = calculate_direct_lunar(&input, &default_enemy()).unwrap();
        assert!((result.average - result.non_crit).abs() < EPSILON);
    }

    #[test]
    fn test_direct_lunar_bloom_element() {
        let input = DirectLunarInput {
            reaction: Reaction::LunarBloom,
            ..direct_valid_input()
        };
        let result = calculate_direct_lunar(&input, &default_enemy()).unwrap();
        assert_eq!(result.damage_element, Some(Element::Dendro));
    }

    #[test]
    fn test_direct_lunar_crystallize_element() {
        let input = DirectLunarInput {
            reaction: Reaction::LunarCrystallize,
            ..direct_valid_input()
        };
        let result = calculate_direct_lunar(&input, &default_enemy()).unwrap();
        assert_eq!(result.damage_element, Some(Element::Geo));
    }

    #[test]
    fn test_direct_lunar_not_lunar_error() {
        let input = DirectLunarInput {
            reaction: Reaction::Overloaded,
            ..direct_valid_input()
        };
        assert!(matches!(
            calculate_direct_lunar(&input, &default_enemy()),
            Err(CalcError::NotLunar(_))
        ));
    }

    #[test]
    fn test_direct_lunar_invalid_talent_multiplier() {
        let input = DirectLunarInput {
            talent_multiplier: 0.0,
            ..direct_valid_input()
        };
        assert!(matches!(
            calculate_direct_lunar(&input, &default_enemy()),
            Err(CalcError::InvalidTalentMultiplier(_))
        ));
    }

    #[test]
    fn test_direct_lunar_invalid_character_level() {
        let input = DirectLunarInput {
            character_level: 0,
            ..direct_valid_input()
        };
        assert!(matches!(
            calculate_direct_lunar(&input, &default_enemy()),
            Err(CalcError::InvalidReactionLevel(0))
        ));
    }

    #[test]
    fn test_direct_lunar_negative_em_error() {
        let input = DirectLunarInput {
            elemental_mastery: -1.0,
            ..direct_valid_input()
        };
        assert!(matches!(
            calculate_direct_lunar(&input, &default_enemy()),
            Err(CalcError::InvalidElementalMastery(_))
        ));
    }

    #[test]
    fn test_direct_lunar_input_serde_roundtrip() {
        let input = direct_valid_input();
        let json = serde_json::to_string(&input).unwrap();
        let deserialized: DirectLunarInput = serde_json::from_str(&json).unwrap();
        assert_eq!(input.character_level, deserialized.character_level);
        assert!((input.talent_multiplier - deserialized.talent_multiplier).abs() < EPSILON);
        assert!((input.scaling_value - deserialized.scaling_value).abs() < EPSILON);
        assert_eq!(input.reaction, deserialized.reaction);
    }
}
