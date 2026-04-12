use serde::{Deserialize, Serialize};

use crate::em::{amplifying_em_bonus, catalyze_em_bonus};
use crate::enemy::Enemy;
use crate::error::CalcError;
use crate::level_table::reaction_base_value;
use crate::reaction::{Reaction, ReactionCategory, catalyze_coefficient};
use crate::stats::Stats;
use crate::team::ResolvedBuff;
use crate::types::{DamageType, Element, ScalingStat};

/// Input parameters for standard damage calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageInput {
    /// Character level (1-90).
    pub character_level: u32,
    /// Final character stats.
    pub stats: Stats,
    /// Talent multiplier in decimal form (e.g. 176% = `1.76`).
    pub talent_multiplier: f64,
    /// Stat used for scaling (ATK, HP, or DEF).
    #[serde(default)]
    pub scaling_stat: ScalingStat,
    /// Attack type (normal, charged, skill, burst, plunging).
    pub damage_type: DamageType,
    /// Damage element. `None` for physical damage.
    pub element: Option<Element>,
    /// Elemental reaction. `None` for no reaction.
    pub reaction: Option<Reaction>,
    /// Reaction DMG bonus from artifacts/buffs in decimal form.
    pub reaction_bonus: f64,
    /// Flat damage added to base (e.g. Shenhe quill, weapon flat DMG scaling).
    /// Added after ATK * talent_multiplier in the damage formula.
    #[serde(default)]
    pub flat_dmg: f64,
}

/// Result of standard damage calculation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageResult {
    /// Damage without critical hit.
    pub non_crit: f64,
    /// Damage with critical hit.
    pub crit: f64,
    /// Average damage (weighted by crit rate).
    pub average: f64,
    /// Applied reaction, if any.
    pub reaction: Option<Reaction>,
}

fn validate(input: &DamageInput, enemy: &Enemy) -> Result<(), CalcError> {
    if !(1..=100).contains(&input.character_level) {
        return Err(CalcError::InvalidCharacterLevel(input.character_level));
    }
    if !(1..=200).contains(&enemy.level) {
        return Err(CalcError::InvalidEnemyLevel(enemy.level));
    }
    if !(0.0..=1.0).contains(&input.stats.crit_rate) {
        return Err(CalcError::InvalidCritRate(input.stats.crit_rate));
    }
    if !(0.0..=1.0).contains(&enemy.def_reduction) {
        return Err(CalcError::InvalidDefReduction(enemy.def_reduction));
    }
    if !(0.0..=1.0).contains(&enemy.def_ignore) {
        return Err(CalcError::InvalidDefIgnore(enemy.def_ignore));
    }
    if input.talent_multiplier <= 0.0 {
        return Err(CalcError::InvalidTalentMultiplier(input.talent_multiplier));
    }
    if input.reaction.is_some() && input.reaction_bonus < 0.0 {
        return Err(CalcError::InvalidReactionBonus(input.reaction_bonus));
    }
    Ok(())
}

/// Calculates the resistance multiplier for an enemy.
///
/// If resistance >= 0.75: `1 / (4 * res + 1)`
/// If 0 <= resistance < 0.75: `1 - res`
/// If resistance < 0: `1 - res / 2`
pub(crate) fn resistance_multiplier(enemy: &Enemy) -> f64 {
    let res = enemy.resistance;
    if res < 0.0 {
        1.0 - res / 2.0
    } else if res < 0.75 {
        1.0 - res
    } else {
        1.0 / (4.0 * res + 1.0)
    }
}

fn defense_multiplier(char_level: u32, enemy: &Enemy) -> f64 {
    let char_part = f64::from(char_level) + 100.0;
    let enemy_part =
        (f64::from(enemy.level) + 100.0) * (1.0 - enemy.def_reduction) * (1.0 - enemy.def_ignore);
    char_part / (char_part + enemy_part)
}

/// Calculates standard damage for attacks and skills.
///
/// Supports amplifying reactions (vaporize/melt) and catalyze reactions
/// (spread/aggravate) via the `reaction` field. For transformative reactions
/// use [`calculate_transformative`](crate::calculate_transformative).
/// For lunar reactions use [`calculate_lunar`](crate::calculate_lunar).
///
/// # Errors
///
/// Returns [`CalcError`] if any input parameter is out of valid range.
///
/// # Examples
///
/// ```
/// use genshin_calc_core::*;
///
/// let input = DamageInput {
///     character_level: 90,
///     stats: Stats {
///         atk: 2000.0,
///         crit_rate: 0.75,
///         crit_dmg: 1.50,
///         dmg_bonus: 0.466,
///         ..Default::default()
///     },
///     talent_multiplier: 1.76,
///     scaling_stat: ScalingStat::Atk,
///     damage_type: DamageType::Skill,
///     element: Some(Element::Pyro),
///     reaction: None,
///     reaction_bonus: 0.0,
///     flat_dmg: 0.0,
/// };
/// let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0, def_ignore: 0.0 };
/// let result = calculate_damage(&input, &enemy).unwrap();
/// assert!(result.crit > result.non_crit);
/// ```
pub fn calculate_damage(input: &DamageInput, enemy: &Enemy) -> Result<DamageResult, CalcError> {
    validate(input, enemy)?;

    let mut catalyze_flat = 0.0;
    let mut amplify_mult = 1.0;
    let mut reaction_result = None;

    if let Some(reaction) = input.reaction {
        match reaction.category() {
            ReactionCategory::Amplifying => {
                let trigger = input.element.ok_or(CalcError::AmplifyingRequiresElement)?;
                let base_mult = match (reaction, trigger) {
                    (Reaction::Vaporize, Element::Pyro) => 1.5,
                    (Reaction::Vaporize, Element::Hydro) => 2.0,
                    (Reaction::Melt, Element::Pyro) => 2.0,
                    (Reaction::Melt, Element::Cryo) => 1.5,
                    _ => return Err(CalcError::InvalidAmplifyingCombination(reaction, trigger)),
                };
                let em_bonus = amplifying_em_bonus(input.stats.elemental_mastery);
                amplify_mult = base_mult * (1.0 + em_bonus + input.reaction_bonus);
                reaction_result = Some(reaction);
            }
            ReactionCategory::Catalyze => {
                let coeff =
                    catalyze_coefficient(reaction).expect("validated: reaction is Catalyze");
                let em_bonus = catalyze_em_bonus(input.stats.elemental_mastery);
                let level_base =
                    reaction_base_value(input.character_level).expect("validated: level 1..=90");
                catalyze_flat = coeff * level_base * (1.0 + em_bonus + input.reaction_bonus);
                reaction_result = Some(reaction);
            }
            ReactionCategory::Transformative => {
                return Err(CalcError::UseTransformativeFunction(reaction));
            }
            ReactionCategory::Lunar => {
                return Err(CalcError::UseLunarFunction(reaction));
            }
        }
    }

    let scaling_value = match input.scaling_stat {
        ScalingStat::Atk | ScalingStat::TotalAtk => input.stats.atk,
        ScalingStat::Hp => input.stats.hp,
        ScalingStat::Def => input.stats.def,
        ScalingStat::Em => input.stats.elemental_mastery,
        ScalingStat::CritRate => input.stats.crit_rate,
    };
    let base = scaling_value * input.talent_multiplier + catalyze_flat + input.flat_dmg;
    let non_crit = base
        * (1.0 + input.stats.total_dmg_bonus(input.element))
        * defense_multiplier(input.character_level, enemy)
        * resistance_multiplier(enemy)
        * amplify_mult;
    let crit = non_crit * (1.0 + input.stats.total_crit_dmg(input.element));
    let average = non_crit * (1.0 - input.stats.crit_rate) + crit * input.stats.crit_rate;

    Ok(DamageResult {
        non_crit,
        crit,
        average,
        reaction: reaction_result,
    })
}

/// Collects flat damage bonus from resolved buffs for a specific damage type.
///
/// Matches `NormalAtkFlatDmg`, `ChargedAtkFlatDmg`, `PlungingAtkFlatDmg`,
/// `SkillFlatDmg`, or `BurstFlatDmg` against the damage type and sums values.
pub fn collect_flat_dmg(buffs: &[ResolvedBuff], damage_type: DamageType) -> f64 {
    use crate::buff_types::BuffableStat;
    let target_stat = match damage_type {
        DamageType::Normal => BuffableStat::NormalAtkFlatDmg,
        DamageType::Charged => BuffableStat::ChargedAtkFlatDmg,
        DamageType::Plunging => BuffableStat::PlungingAtkFlatDmg,
        DamageType::Skill => BuffableStat::SkillFlatDmg,
        DamageType::Burst => BuffableStat::BurstFlatDmg,
    };
    buffs
        .iter()
        .filter(|b| b.stat == target_stat)
        .map(|b| b.value)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

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
            flat_dmg: 0.0,
        }
    }

    fn valid_enemy() -> Enemy {
        Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
            def_ignore: 0.0,
        }
    }

    #[test]
    fn test_invalid_character_level_zero() {
        let input = DamageInput {
            character_level: 0,
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert_eq!(result, Err(CalcError::InvalidCharacterLevel(0)));
    }

    #[test]
    fn test_invalid_character_level_too_high() {
        let input = DamageInput {
            character_level: 101,
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert_eq!(result, Err(CalcError::InvalidCharacterLevel(101)));
    }

    #[test]
    fn test_invalid_enemy_level_zero() {
        let enemy = Enemy {
            level: 0,
            ..valid_enemy()
        };
        let result = calculate_damage(&valid_input(), &enemy);
        assert_eq!(result, Err(CalcError::InvalidEnemyLevel(0)));
    }

    #[test]
    fn test_invalid_enemy_level_too_high() {
        let enemy = Enemy {
            level: 201,
            ..valid_enemy()
        };
        let result = calculate_damage(&valid_input(), &enemy);
        assert_eq!(result, Err(CalcError::InvalidEnemyLevel(201)));
    }

    #[test]
    fn test_invalid_crit_rate_negative() {
        let input = DamageInput {
            stats: Stats {
                crit_rate: -0.1,
                ..valid_input().stats
            },
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert_eq!(result, Err(CalcError::InvalidCritRate(-0.1)));
    }

    #[test]
    fn test_invalid_crit_rate_too_high() {
        let input = DamageInput {
            stats: Stats {
                crit_rate: 1.1,
                ..valid_input().stats
            },
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert_eq!(result, Err(CalcError::InvalidCritRate(1.1)));
    }

    #[test]
    fn test_invalid_def_reduction_negative() {
        let enemy = Enemy {
            def_reduction: -0.1,
            ..valid_enemy()
        };
        let result = calculate_damage(&valid_input(), &enemy);
        assert_eq!(result, Err(CalcError::InvalidDefReduction(-0.1)));
    }

    #[test]
    fn test_invalid_def_reduction_too_high() {
        let enemy = Enemy {
            def_reduction: 1.5,
            ..valid_enemy()
        };
        let result = calculate_damage(&valid_input(), &enemy);
        assert_eq!(result, Err(CalcError::InvalidDefReduction(1.5)));
    }

    #[test]
    fn test_invalid_talent_multiplier_zero() {
        let input = DamageInput {
            talent_multiplier: 0.0,
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert_eq!(result, Err(CalcError::InvalidTalentMultiplier(0.0)));
    }

    #[test]
    fn test_invalid_talent_multiplier_negative() {
        let input = DamageInput {
            talent_multiplier: -1.0,
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert_eq!(result, Err(CalcError::InvalidTalentMultiplier(-1.0)));
    }

    #[test]
    fn test_resistance_negative() {
        let enemy = Enemy {
            level: 90,
            resistance: -0.2,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        assert!((resistance_multiplier(&enemy) - 1.1).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_zero() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.0,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        assert!((resistance_multiplier(&enemy) - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_normal() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.1,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        assert!((resistance_multiplier(&enemy) - 0.9).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_boundary_below_75() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.74,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        assert!((resistance_multiplier(&enemy) - 0.26).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_at_75() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.75,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        assert!((resistance_multiplier(&enemy) - 0.25).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_high() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.9,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        assert!((resistance_multiplier(&enemy) - 1.0 / 4.6).abs() < EPSILON);
    }

    #[test]
    fn test_defense_multiplier_same_level() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.0,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        let result = defense_multiplier(90, &enemy);
        assert!((result - 0.5).abs() < EPSILON);
    }

    #[test]
    fn test_defense_multiplier_low_vs_high() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.0,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        let result = defense_multiplier(1, &enemy);
        assert!((result - 101.0 / 291.0).abs() < EPSILON);
    }

    #[test]
    fn test_defense_multiplier_with_def_reduction() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.0,
            def_reduction: 0.3,
            def_ignore: 0.0,
        };
        let result = defense_multiplier(90, &enemy);
        assert!((result - 190.0 / 323.0).abs() < EPSILON);
    }

    #[test]
    fn test_defense_multiplier_full_def_reduction() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.0,
            def_reduction: 1.0,
            def_ignore: 0.0,
        };
        let result = defense_multiplier(90, &enemy);
        assert!((result - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_damage_golden() {
        // base = 2000 * 1.76 + 0 (no catalyze) = 3520
        // non_crit = 3520 * (1 + 0.466) * 0.5 * 0.9 = 3520 * 1.466 * 0.5 * 0.9 = 2322.144
        // crit = 2322.144 * (1 + 1.0) = 4644.288
        // avg = 2322.144 * 0.5 + 4644.288 * 0.5 = 3483.216
        let result = calculate_damage(&valid_input(), &valid_enemy()).unwrap();
        assert!((result.non_crit - 2322.144).abs() < 0.01);
        assert!((result.crit - 4644.288).abs() < 0.01);
        assert!((result.average - 3483.216).abs() < 0.01);
        assert_eq!(result.reaction, None);
    }

    #[test]
    fn test_calculate_damage_no_crit() {
        let input = DamageInput {
            stats: Stats {
                crit_rate: 0.0,
                ..valid_input().stats
            },
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy()).unwrap();
        assert!((result.average - result.non_crit).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_damage_guaranteed_crit() {
        let input = DamageInput {
            stats: Stats {
                crit_rate: 1.0,
                ..valid_input().stats
            },
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy()).unwrap();
        assert!((result.average - result.crit).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_damage_physical() {
        let input = DamageInput {
            element: None,
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert!(result.is_ok());
    }

    #[test]
    fn test_calculate_damage_boundary_levels() {
        let input = DamageInput {
            character_level: 1,
            ..valid_input()
        };
        let enemy = Enemy {
            level: 1,
            resistance: 0.0,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        let result = calculate_damage(&input, &enemy);
        assert!(result.is_ok());
    }

    #[test]
    fn test_damage_result_serde_roundtrip() {
        let input = valid_input();
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: DamageResult = serde_json::from_str(&json).unwrap();
        assert!((result.non_crit - deserialized.non_crit).abs() < EPSILON);
        assert!((result.crit - deserialized.crit).abs() < EPSILON);
        assert!((result.average - deserialized.average).abs() < EPSILON);
        assert_eq!(result.reaction, deserialized.reaction);
    }

    #[test]
    fn test_damage_input_serde_roundtrip() {
        let input = valid_input();
        let json = serde_json::to_string(&input).unwrap();
        let deserialized: DamageInput = serde_json::from_str(&json).unwrap();
        assert!((input.stats.atk - deserialized.stats.atk).abs() < EPSILON);
        assert!(
            (input.stats.elemental_mastery - deserialized.stats.elemental_mastery).abs() < EPSILON
        );
        assert!((input.talent_multiplier - deserialized.talent_multiplier).abs() < EPSILON);
        assert!((input.reaction_bonus - deserialized.reaction_bonus).abs() < EPSILON);
        assert!((input.flat_dmg - deserialized.flat_dmg).abs() < EPSILON);
        assert_eq!(input.character_level, deserialized.character_level);
        assert_eq!(input.scaling_stat, deserialized.scaling_stat);
        assert_eq!(input.damage_type, deserialized.damage_type);
        assert_eq!(input.element, deserialized.element);
        assert_eq!(input.reaction, deserialized.reaction);
    }

    // =====================================================================
    // Verification tests: community data + in-game observed values
    // =====================================================================

    #[test]
    fn test_verified_freminet_normal_lv4_vs_lv85() {
        // Freminet Lv20, Normal Attack 1st hit Lv4 (107.7%), vs Lv85 Hilichurl
        // In-game observed: non-crit 35, crit 53 (game floors to integer)
        let input = DamageInput {
            character_level: 20,
            stats: Stats {
                atk: 94.0,
                crit_rate: 0.05,
                crit_dmg: 0.50,
                dmg_bonus: 0.0,
                ..Stats::default()
            },
            talent_multiplier: 1.077,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: None, // physical
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let enemy = Enemy {
            level: 85,
            resistance: 0.1, // Hilichurl physical resistance
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        let result = calculate_damage(&input, &enemy).unwrap();
        // Game shows floor(35.84) = 35, floor(53.76) = 53
        assert_eq!(result.non_crit.floor() as i64, 35);
        assert_eq!(result.crit.floor() as i64, 53);
    }

    #[test]
    fn test_verified_diluc_searing_onslaught_lv8_vs_lv90() {
        // Diluc Searing Onslaught 1st Hit, Talent Lv8, vs Lv90 Hilichurl
        // Talent multiplier: 151.04% (source: genshin-center.com)
        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 1800.0,
                crit_rate: 0.6,
                crit_dmg: 1.2,
                dmg_bonus: 0.466,
                ..Stats::default()
            },
            talent_multiplier: 1.5104,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let enemy = Enemy {
            level: 90,
            resistance: 0.1,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        let result = calculate_damage(&input, &enemy).unwrap();
        assert!((result.non_crit - 1793.539584).abs() < 0.01);
        assert!((result.crit - 3945.787085).abs() < 0.01);
    }

    #[test]
    fn test_verified_ganyu_bloom_lv10_vs_lv90() {
        // Ganyu Frostflake Arrow Bloom, Talent Lv10, vs Lv90 Hilichurl
        // Talent multiplier: 396.16% (source: genshin-center.com)
        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 2200.0,
                crit_rate: 0.45,
                crit_dmg: 1.8,
                dmg_bonus: 0.616,
                ..Stats::default()
            },
            talent_multiplier: 3.9616,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Charged,
            element: Some(Element::Cryo),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let enemy = Enemy {
            level: 90,
            resistance: 0.1,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        let result = calculate_damage(&input, &enemy).unwrap();
        assert!((result.non_crit - 6337.926144).abs() < 0.01);
        assert!((result.crit - 17746.193203).abs() < 0.01);
    }

    #[test]
    fn test_verified_raiden_initial_slash_lv8_vs_lv100() {
        // Raiden Shogun Musou no Hitotachi, Talent Lv8, vs Lv100 enemy
        // Talent multiplier: 641.28% base, no Resolve stacks
        // Tests asymmetric char/enemy levels (DEF mult != 0.5)
        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.55,
                crit_dmg: 1.1,
                dmg_bonus: 0.466,
                ..Stats::default()
            },
            talent_multiplier: 6.4128,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Burst,
            element: Some(Element::Electro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let enemy = Enemy {
            level: 100,
            resistance: 0.1,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        let result = calculate_damage(&input, &enemy).unwrap();
        // DEF mult = 190/390 = 0.48718 (not 0.5)
        assert!((result.non_crit - 8244.098363).abs() < 0.01);
        assert!((result.crit - 17312.606562).abs() < 0.01);
    }

    // =====================================================================
    // Amplifying reaction tests
    // =====================================================================

    #[test]
    fn test_vaporize_pyro_on_hydro() {
        // Pyro trigger = 1.5x, EM=200 → em_bonus = 0.3475, reaction_bonus = 0.15
        // amplify = 1.5 * (1 + 0.3475 + 0.15) = 1.5 * 1.4975 = 2.24625
        let input = DamageInput {
            stats: Stats {
                atk: 1800.0,
                crit_rate: 0.6,
                crit_dmg: 1.2,
                dmg_bonus: 0.466,
                elemental_mastery: 200.0,
                ..Stats::default()
            },
            talent_multiplier: 1.5104,
            element: Some(Element::Pyro),
            reaction: Some(Reaction::Vaporize),
            reaction_bonus: 0.15,
            ..valid_input()
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        assert_eq!(result.reaction, Some(Reaction::Vaporize));
        // Verify amplified damage > non-amplified
        let no_reaction = calculate_damage(
            &DamageInput {
                reaction: None,
                reaction_bonus: 0.0,
                ..input.clone()
            },
            &enemy,
        )
        .unwrap();
        assert!(result.non_crit > no_reaction.non_crit * 1.4);
    }

    #[test]
    fn test_vaporize_hydro_on_pyro() {
        // Hydro trigger = 2.0x
        let input = DamageInput {
            element: Some(Element::Hydro),
            reaction: Some(Reaction::Vaporize),
            reaction_bonus: 0.0,
            ..valid_input()
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        let no_reaction = calculate_damage(
            &DamageInput {
                reaction: None,
                ..input.clone()
            },
            &enemy,
        )
        .unwrap();
        assert!((result.non_crit / no_reaction.non_crit - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_melt_pyro_on_cryo() {
        // Pyro trigger Melt = 2.0x
        let input = DamageInput {
            element: Some(Element::Pyro),
            reaction: Some(Reaction::Melt),
            reaction_bonus: 0.0,
            ..valid_input()
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        let no_reaction = calculate_damage(
            &DamageInput {
                reaction: None,
                ..input.clone()
            },
            &enemy,
        )
        .unwrap();
        assert!((result.non_crit / no_reaction.non_crit - 2.0).abs() < 0.01);
    }

    // =====================================================================
    // Catalyze reaction tests
    // =====================================================================

    #[test]
    fn test_aggravate_adds_flat_damage() {
        let input = DamageInput {
            element: Some(Element::Electro),
            reaction: Some(Reaction::Aggravate),
            reaction_bonus: 0.0,
            ..valid_input()
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        let no_reaction = calculate_damage(
            &DamageInput {
                reaction: None,
                ..input.clone()
            },
            &enemy,
        )
        .unwrap();
        // Aggravate should add flat damage, making result higher
        assert!(result.non_crit > no_reaction.non_crit);
        assert_eq!(result.reaction, Some(Reaction::Aggravate));
    }

    #[test]
    fn test_spread_adds_flat_damage() {
        let input = DamageInput {
            element: Some(Element::Dendro),
            reaction: Some(Reaction::Spread),
            reaction_bonus: 0.0,
            ..valid_input()
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        let no_reaction = calculate_damage(
            &DamageInput {
                reaction: None,
                ..input.clone()
            },
            &enemy,
        )
        .unwrap();
        assert!(result.non_crit > no_reaction.non_crit);
    }

    // =====================================================================
    // Error case tests
    // =====================================================================

    #[test]
    fn test_transformative_in_calculate_damage_errors() {
        let input = DamageInput {
            reaction: Some(Reaction::Overloaded),
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert!(matches!(
            result,
            Err(CalcError::UseTransformativeFunction(_))
        ));
    }

    #[test]
    fn test_lunar_in_calculate_damage_errors() {
        let input = DamageInput {
            reaction: Some(Reaction::LunarElectroCharged),
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert!(matches!(result, Err(CalcError::UseLunarFunction(_))));
    }

    #[test]
    fn test_amplifying_without_element_errors() {
        let input = DamageInput {
            element: None,
            reaction: Some(Reaction::Vaporize),
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert!(matches!(result, Err(CalcError::AmplifyingRequiresElement)));
    }

    // =====================================================================
    // Golden tests: hand-calculated reaction values
    // =====================================================================

    #[test]
    fn test_golden_vaporize_pyro_trigger() {
        // Diluc-like: ATK 1800, talent 1.5104, Pyro DMG 46.6%, EM 200
        // Vaporize (Pyro on Hydro) = 1.5x, reaction_bonus = 0.15 (Crimson Witch)
        // em_bonus = 2.78 * 200 / (200 + 1400) = 0.3475
        // amplify = 1.5 * (1 + 0.3475 + 0.15) = 2.24625
        // base = 1800 * 1.5104 = 2718.72
        // non_crit = 2718.72 * 1.466 * 0.5 * 0.9 * 2.24625
        //          = 1793.5396 * 2.24625 = 4028.738
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
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: Some(Reaction::Vaporize),
            reaction_bonus: 0.15,
            flat_dmg: 0.0,
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        assert!((result.non_crit - 4028.738).abs() < 0.1);
        assert!((result.crit - 8863.224).abs() < 0.1);
        assert_eq!(result.reaction, Some(Reaction::Vaporize));
    }

    #[test]
    fn test_golden_aggravate() {
        // Electro trigger, ATK 1500, talent 1.2, Electro DMG 46.6%, EM 150
        // catalyze_flat = 1.15 * 1446.8535 * (1 + 5*150/(150+1200))
        //               = 1.15 * 1446.8535 * 1.5556 = 2588.260
        // base = 1500 * 1.2 + 2588.260 = 4388.260
        // non_crit = 4388.260 * 1.466 * 0.5 * 0.9 = 2894.935
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
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Electro),
            reaction: Some(Reaction::Aggravate),
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        assert!((result.non_crit - 2894.935).abs() < 0.1);
        assert!((result.crit - 6947.845).abs() < 0.1);
        assert_eq!(result.reaction, Some(Reaction::Aggravate));
    }

    #[test]
    fn test_golden_melt_reverse_cryo_trigger() {
        // Reverse Melt (Cryo on Pyro) = 1.5x
        // ATK 2000, talent 2.0, Cryo DMG 46.6%, EM 200
        // em_bonus = 2.78 * 200 / (200 + 1400) = 0.3475
        // amplify = 1.5 * (1 + 0.3475) = 2.02125
        // base = 2000 * 2.0 = 4000
        // non_crit = 4000 * 1.466 * 0.5 * 0.9 * 2.02125 = 5333.675
        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.6,
                crit_dmg: 1.2,
                dmg_bonus: 0.466,
                elemental_mastery: 200.0,
                ..Stats::default()
            },
            talent_multiplier: 2.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Burst,
            element: Some(Element::Cryo),
            reaction: Some(Reaction::Melt),
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        assert!((result.non_crit - 5333.675).abs() < 0.1);
        assert!((result.crit - 11734.084).abs() < 0.1);
        assert_eq!(result.reaction, Some(Reaction::Melt));
    }

    #[test]
    fn test_golden_spread_dendro_trigger() {
        // Nahida-like: ATK 1200, talent 1.8576 (Tri-Karma Lv10 ATK%), Dendro DMG 15%, EM 800
        // catalyze_em_bonus = 5.0 * 800 / (800 + 1200) = 2.0
        // catalyze_flat = 1.25 * 1446.8535 * (1 + 2.0) = 5425.701
        // base = 1200 * 1.8576 + 5425.701 = 7654.821
        // non_crit = 7654.821 * 1.15 * 0.5 * 0.9 = 3961.370
        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 1200.0,
                crit_rate: 0.5,
                crit_dmg: 1.0,
                dmg_bonus: 0.15,
                elemental_mastery: 800.0,
                ..Stats::default()
            },
            talent_multiplier: 1.8576,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Dendro),
            reaction: Some(Reaction::Spread),
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        assert!((result.non_crit - 3961.370).abs() < 0.1);
        assert!((result.crit - 7922.739).abs() < 0.1);
        assert_eq!(result.reaction, Some(Reaction::Spread));
    }

    #[test]
    fn test_golden_hp_scaling_yelan_like() {
        // Yelan-like: HP 30000, talent 20% max HP, Hydro DMG 46.6%, ScalingStat::Hp
        // base = 30000 * 0.2 = 6000
        // non_crit = 6000 * 1.466 * 0.5 * 0.9 = 3958.2
        // crit (200% crit dmg) = 3958.2 * 3.0 = 11874.6
        // avg = 3958.2 * 0.3 + 11874.6 * 0.7 = 9499.68
        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                hp: 30000.0,
                atk: 1000.0,
                crit_rate: 0.7,
                crit_dmg: 2.0,
                dmg_bonus: 0.466,
                ..Stats::default()
            },
            talent_multiplier: 0.2,
            scaling_stat: ScalingStat::Hp,
            damage_type: DamageType::Skill,
            element: Some(Element::Hydro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        assert!((result.non_crit - 3958.2).abs() < 0.01);
        assert!((result.crit - 11874.6).abs() < 0.01);
        assert!((result.average - 9499.68).abs() < 0.01);
    }

    #[test]
    fn test_golden_def_scaling_itto_like() {
        // Itto-like: DEF 2500, Kesagiri Lv10 = 178.84%, Geo DMG 46.6%, ScalingStat::Def
        // (source: Project Amber API, KQM)
        // base = 2500 * 1.7884 = 4471.0
        // non_crit = 4471.0 * 1.466 * 0.5 * 0.9 = 2949.519
        // crit (180% crit dmg) = 2949.519 * 2.8 = 8258.652
        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                def: 2500.0,
                atk: 1000.0,
                crit_rate: 0.7,
                crit_dmg: 1.8,
                dmg_bonus: 0.466,
                ..Stats::default()
            },
            talent_multiplier: 1.7884,
            scaling_stat: ScalingStat::Def,
            damage_type: DamageType::Normal,
            element: Some(Element::Geo),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        assert!((result.non_crit - 2949.519).abs() < 0.1);
        assert!((result.crit - 8258.652).abs() < 0.1);
    }

    #[test]
    fn test_golden_negative_resistance_vv_shred() {
        // VV shred: 10% base RES - 40% VV = -30% effective RES
        // RES multiplier for -30%: 1 - (-0.30)/2 = 1.15
        // ATK 1800, talent 1.5, Pyro DMG 46.6%
        // non_crit = 1800 * 1.5 * 1.466 * 0.5 * 1.15 = 2275.965
        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 1800.0,
                crit_rate: 0.6,
                crit_dmg: 1.2,
                dmg_bonus: 0.466,
                ..Stats::default()
            },
            talent_multiplier: 1.5,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let enemy = Enemy {
            level: 90,
            resistance: -0.30,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        let result = calculate_damage(&input, &enemy).unwrap();
        assert!((result.non_crit - 2275.965).abs() < 0.01);
        assert!((result.crit - 5007.123).abs() < 0.01);
    }

    #[test]
    fn test_golden_def_reduction() {
        // 30% DEF reduction (e.g. Lisa A4 + Raiden C2 etc.)
        // DEF mult = 190 / (190 + 190*(1-0.30)) = 190/323 = 0.588235
        // ATK 1500, talent 1.5, Electro DMG 46.6%
        // non_crit = 1500 * 1.5 * 1.466 * (190/323) * 0.9 = 1746.265
        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 1500.0,
                crit_rate: 0.5,
                crit_dmg: 1.0,
                dmg_bonus: 0.466,
                ..Stats::default()
            },
            talent_multiplier: 1.5,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Electro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.30,
            def_ignore: 0.0,
        };
        let result = calculate_damage(&input, &enemy).unwrap();
        assert!((result.non_crit - 1746.265).abs() < 0.1);
    }

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
        let expected_non_crit = 2500.0 * 0.80 * 1.466 * 0.5 * 0.9;
        assert!((result.non_crit - expected_non_crit).abs() < 0.01);
    }

    #[test]
    fn test_scaling_stat_hp_with_vaporize() {
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
        let expected_non_crit =
            30000.0 * 0.10 * 1.466 * 0.5 * 0.9 * 2.0 * (1.0 + 2.78 * 100.0 / 1500.0);
        assert!((result.non_crit - expected_non_crit).abs() < 0.1);
    }

    #[test]
    fn test_flat_dmg_added_to_base() {
        let mut input = valid_input();
        input.flat_dmg = 500.0;
        input.stats.crit_rate = 0.0;
        input.stats.crit_dmg = 0.0;
        input.stats.dmg_bonus = 0.0;
        input.talent_multiplier = 1.0;
        input.reaction = None;

        let enemy = Enemy {
            level: 90,
            resistance: 0.0,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        let result = calculate_damage(&input, &enemy).unwrap();

        // base = atk * 1.0 + flat_dmg = 2000 + 500 = 2500
        // non_crit = 2500 * 1.0 * 0.5 * 1.0 = 1250
        let expected = (2000.0 + 500.0) * 0.5;
        assert!((result.non_crit - expected).abs() < 1.0);
    }

    #[test]
    fn test_flat_dmg_zero_unchanged() {
        let input = valid_input();
        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };
        let result = calculate_damage(&input, &enemy).unwrap();

        let mut input_zero = valid_input();
        input_zero.flat_dmg = 0.0;
        let result_zero = calculate_damage(&input_zero, &enemy).unwrap();
        assert!((result.non_crit - result_zero.non_crit).abs() < 1e-6);
    }

    #[test]
    fn test_collect_flat_dmg() {
        use crate::buff_types::BuffableStat;
        use crate::team::{BuffTarget, ResolvedBuff};

        let buffs = vec![
            ResolvedBuff {
                source: "Weapon".into(),
                stat: BuffableStat::NormalAtkFlatDmg,
                value: 200.0,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "Weapon2".into(),
                stat: BuffableStat::NormalAtkFlatDmg,
                value: 100.0,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "Other".into(),
                stat: BuffableStat::SkillFlatDmg,
                value: 500.0,
                target: BuffTarget::Team,
                origin: None,
            },
        ];
        assert!((collect_flat_dmg(&buffs, DamageType::Normal) - 300.0).abs() < 1e-6);
        assert!((collect_flat_dmg(&buffs, DamageType::Skill) - 500.0).abs() < 1e-6);
        assert!((collect_flat_dmg(&buffs, DamageType::Charged) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_scaling_stat_serde_default_backward_compat() {
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

    #[test]
    fn element_specific_dmg_bonus_applies() {
        // Test that pyro_dmg_bonus is applied when element is Pyro
        let input = DamageInput {
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.75,
                crit_dmg: 1.50,
                dmg_bonus: 0.0,
                pyro_dmg_bonus: 0.466,
                ..Stats::default()
            },
            element: Some(Element::Pyro),
            ..valid_input()
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        // non_crit = 2000 * (1 + 0.466) * def_mult * res_mult
        // Pyro bonus should be applied
        assert!(result.non_crit > 0.0);
        // Verify pyro bonus is used (not zero dmg_bonus)
        let input_no_bonus = DamageInput {
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.75,
                crit_dmg: 1.50,
                dmg_bonus: 0.0,
                ..Stats::default()
            },
            ..input.clone()
        };
        let result_no_bonus = calculate_damage(&input_no_bonus, &enemy).unwrap();
        assert!(result.non_crit > result_no_bonus.non_crit);
    }

    #[test]
    fn wrong_element_dmg_bonus_does_not_apply() {
        // Test that pyro_dmg_bonus is NOT applied when element is Hydro
        let input = DamageInput {
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.75,
                crit_dmg: 1.50,
                dmg_bonus: 0.0,
                pyro_dmg_bonus: 0.466,
                ..Stats::default()
            },
            element: Some(Element::Hydro), // Hydro damage, NOT Pyro
            ..valid_input()
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        // Pyro bonus should NOT apply to Hydro damage
        let input_no_bonus = DamageInput {
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.75,
                crit_dmg: 1.50,
                dmg_bonus: 0.0,
                ..Stats::default()
            },
            ..input.clone()
        };
        let result_no_bonus = calculate_damage(&input_no_bonus, &enemy).unwrap();
        assert!((result.non_crit - result_no_bonus.non_crit).abs() < 1e-6);
    }
}
