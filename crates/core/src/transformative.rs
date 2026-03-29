use serde::{Deserialize, Serialize};

use crate::damage::resistance_multiplier;
use crate::em::transformative_em_bonus;
use crate::enemy::Enemy;
use crate::error::CalcError;
use crate::level_table::reaction_base_value;
use crate::reaction::{
    Reaction, ReactionCategory, transformative_element, transformative_multiplier,
};
use crate::types::Element;

/// Input for transformative reaction damage calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformativeInput {
    /// Character level (1-100).
    pub character_level: u32,
    /// Elemental mastery.
    pub elemental_mastery: f64,
    /// Transformative reaction type.
    pub reaction: Reaction,
    /// Reaction DMG bonus in decimal form.
    pub reaction_bonus: f64,
}

/// Result of transformative reaction damage calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformativeResult {
    /// Total reaction damage.
    pub damage: f64,
    /// Element of the reaction damage. `None` for physical (e.g. shattered).
    pub damage_element: Option<Element>,
}

fn validate(input: &TransformativeInput, enemy: &Enemy) -> Result<(), CalcError> {
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
    if input.reaction.category() != ReactionCategory::Transformative {
        return Err(CalcError::NotTransformative(input.reaction));
    }
    if let Reaction::Swirl(elem) = input.reaction {
        match elem {
            Element::Pyro | Element::Hydro | Element::Electro | Element::Cryo => {}
            _ => return Err(CalcError::InvalidSwirlElement(elem)),
        }
    }
    Ok(())
}

/// Calculates transformative reaction damage.
///
/// Transformative reactions deal fixed damage based on character level and
/// elemental mastery. They ignore ATK, talent multipliers, crit, and defense.
///
/// # Errors
///
/// Returns [`CalcError`] if the reaction is not transformative or inputs are invalid.
///
/// # Examples
///
/// ```
/// use genshin_calc_core::*;
///
/// let input = TransformativeInput {
///     character_level: 90,
///     elemental_mastery: 200.0,
///     reaction: Reaction::Overloaded,
///     reaction_bonus: 0.0,
/// };
/// let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
/// let result = calculate_transformative(&input, &enemy).unwrap();
/// assert!(result.damage > 0.0);
/// ```
pub fn calculate_transformative(
    input: &TransformativeInput,
    enemy: &Enemy,
) -> Result<TransformativeResult, CalcError> {
    validate(input, enemy)?;

    let level_base = reaction_base_value(input.character_level).unwrap();
    let reaction_mult = transformative_multiplier(input.reaction).unwrap();
    let em_bonus = transformative_em_bonus(input.elemental_mastery);
    let res_mult = resistance_multiplier(enemy);
    let damage_elem = transformative_element(input.reaction).unwrap();

    let damage = level_base * reaction_mult * (1.0 + em_bonus + input.reaction_bonus) * res_mult;

    Ok(TransformativeResult {
        damage,
        damage_element: damage_elem,
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
        }
    }

    #[test]
    fn test_overloaded_lv90_no_em() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
        };
        // 1446.8535 * 2.75 * 1.0 * 0.9 = 3577.9122...
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 1446.8535 * 2.75 * 0.9).abs() < 0.01);
        assert_eq!(result.damage_element, Some(Element::Pyro));
    }

    #[test]
    fn test_overloaded_lv90_em800() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 800.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
        };
        let em_bonus = 16.0 * 800.0 / (800.0 + 2000.0);
        let expected = 1446.8535 * 2.75 * (1.0 + em_bonus) * 0.9;
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - expected).abs() < 0.01);
    }

    #[test]
    fn test_superconduct() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Superconduct,
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 1446.8535 * 1.5 * 0.9).abs() < 0.01);
        assert_eq!(result.damage_element, Some(Element::Cryo));
    }

    #[test]
    fn test_swirl_pyro() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Swirl(Element::Pyro),
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 1446.8535 * 0.6 * 0.9).abs() < 0.01);
        assert_eq!(result.damage_element, Some(Element::Pyro));
    }

    #[test]
    fn test_swirl_invalid_element() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Swirl(Element::Dendro),
            reaction_bonus: 0.0,
        };
        assert!(matches!(
            calculate_transformative(&input, &default_enemy()),
            Err(CalcError::InvalidSwirlElement(Element::Dendro))
        ));
    }

    #[test]
    fn test_shattered_physical() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Shattered,
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert_eq!(result.damage_element, None);
    }

    #[test]
    fn test_bloom() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Bloom,
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 1446.8535 * 2.0 * 0.9).abs() < 0.01);
        assert_eq!(result.damage_element, Some(Element::Dendro));
    }

    #[test]
    fn test_not_transformative_error() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Vaporize,
            reaction_bonus: 0.0,
        };
        assert!(matches!(
            calculate_transformative(&input, &default_enemy()),
            Err(CalcError::NotTransformative(_))
        ));
    }

    #[test]
    fn test_level_100_valid() {
        let input = TransformativeInput {
            character_level: 100,
            elemental_mastery: 0.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy());
        assert!(result.is_ok());
    }

    // =====================================================================
    // Golden tests: hand-calculated reaction values
    // =====================================================================

    #[test]
    fn test_golden_overloaded_em200() {
        // Lv90, EM 200, Overloaded (2.75), vs 10% Pyro RES
        // em_bonus = 16 * 200 / (200 + 2000) = 1.454545
        // damage = 1446.8535 * 2.75 * (1 + 1.454545) * 0.9 = 8789.635
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 200.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 8789.635).abs() < 0.1);
        assert_eq!(result.damage_element, Some(Element::Pyro));
    }

    #[test]
    fn test_golden_swirl_pyro_em800() {
        // Lv90, EM 800, Swirl Pyro (0.6), vs 10% Pyro RES
        // em_bonus = 16 * 800 / (800 + 2000) = 4.571428
        // damage = 1446.8535 * 0.6 * (1 + 4.571428) * 0.9 = 4352.962
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 800.0,
            reaction: Reaction::Swirl(Element::Pyro),
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 4352.962).abs() < 0.1);
        assert_eq!(result.damage_element, Some(Element::Pyro));
    }

    #[test]
    fn test_golden_superconduct_em150() {
        // Lv90, EM 150, Superconduct (1.5), vs 10% Cryo RES
        // em_bonus = 16 * 150 / (150 + 2000) = 2400/2150 = 1.11628
        // damage = 1446.8535 * 1.5 * (1 + 2400/2150) * 0.9
        //        = 2170.280 * (4550/2150) * 0.9 = 4133.627
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 150.0,
            reaction: Reaction::Superconduct,
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 4133.627).abs() < 0.1);
        assert_eq!(result.damage_element, Some(Element::Cryo));
    }

    #[test]
    fn test_golden_electro_charged_em300() {
        // Lv90, EM 300, ElectroCharged (2.0), vs 10% Electro RES
        // em_bonus = 16 * 300 / (300 + 2000) = 4800/2300 = 2.08696
        // damage = 1446.8535 * 2.0 * (1 + 4800/2300) * 0.9
        //        = 2893.707 * (7100/2300) * 0.9 = 8039.473
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 300.0,
            reaction: Reaction::ElectroCharged,
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 8039.473).abs() < 0.1);
        assert_eq!(result.damage_element, Some(Element::Electro));
    }

    #[test]
    fn test_golden_hyperbloom_em800() {
        // Lv90, EM 800, Hyperbloom (3.0), vs 10% Dendro RES
        // em_bonus = 16 * 800 / (800 + 2000) = 12800/2800 = 4.57143
        // damage = 1446.8535 * 3.0 * (1 + 12800/2800) * 0.9
        //        = 4340.561 * (15600/2800) * 0.9 = 21764.811
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 800.0,
            reaction: Reaction::Hyperbloom,
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 21764.811).abs() < 0.1);
        assert_eq!(result.damage_element, Some(Element::Dendro));
    }

    #[test]
    fn test_golden_kazuha_swirl_with_vv() {
        // Kazuha full EM (960), Lv90, Swirl Pyro, 4pc VV (+60% Swirl DMG)
        // em_bonus = 16 * 960 / (960 + 2000) = 15360/2960 = 5.18919
        // damage = 1446.8535 * 0.6 * (1 + 5.18919 + 0.60) * 0.9
        //        = 868.112 * 6.78919 * 0.9 = 5304.306
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 960.0,
            reaction: Reaction::Swirl(Element::Pyro),
            reaction_bonus: 0.60,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 5304.306).abs() < 0.5);
        assert_eq!(result.damage_element, Some(Element::Pyro));
    }

    #[test]
    fn test_reaction_bonus_applied() {
        let base = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
        };
        let with_bonus = TransformativeInput {
            reaction_bonus: 0.4,
            ..base.clone()
        };
        let r1 = calculate_transformative(&base, &default_enemy()).unwrap();
        let r2 = calculate_transformative(&with_bonus, &default_enemy()).unwrap();
        assert!((r2.damage / r1.damage - 1.4).abs() < EPSILON);
    }
}
