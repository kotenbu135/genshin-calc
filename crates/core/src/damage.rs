use serde::{Deserialize, Serialize};

use crate::enemy::Enemy;
use crate::error::CalcError;
use crate::stats::Stats;
use crate::types::{DamageType, Element};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageInput {
    pub character_level: u32,
    pub stats: Stats,
    pub talent_multiplier: f64,
    pub damage_type: DamageType,
    pub element: Option<Element>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageResult {
    pub non_crit: f64,
    pub crit: f64,
    pub average: f64,
}

fn validate(input: &DamageInput, enemy: &Enemy) -> Result<(), CalcError> {
    if !(1..=90).contains(&input.character_level) {
        return Err(CalcError::InvalidCharacterLevel(input.character_level));
    }
    if !(1..=100).contains(&enemy.level) {
        return Err(CalcError::InvalidEnemyLevel(enemy.level));
    }
    if !(0.0..=1.0).contains(&input.stats.crit_rate) {
        return Err(CalcError::InvalidCritRate(input.stats.crit_rate));
    }
    if !(0.0..=1.0).contains(&enemy.def_reduction) {
        return Err(CalcError::InvalidDefReduction(enemy.def_reduction));
    }
    if input.talent_multiplier <= 0.0 {
        return Err(CalcError::InvalidTalentMultiplier(input.talent_multiplier));
    }
    Ok(())
}

fn resistance_multiplier(enemy: &Enemy) -> f64 {
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
    let enemy_part = (f64::from(enemy.level) + 100.0) * (1.0 - enemy.def_reduction);
    char_part / (char_part + enemy_part)
}

fn base_damage(input: &DamageInput) -> f64 {
    input.stats.atk * input.talent_multiplier * (1.0 + input.stats.dmg_bonus)
}

pub fn calculate_damage(input: &DamageInput, enemy: &Enemy) -> Result<DamageResult, CalcError> {
    validate(input, enemy)?;

    let base = base_damage(input);
    let def_mult = defense_multiplier(input.character_level, enemy);
    let res_mult = resistance_multiplier(enemy);

    let non_crit = base * def_mult * res_mult;
    let crit = non_crit * (1.0 + input.stats.crit_dmg);
    let average = non_crit * (1.0 - input.stats.crit_rate) + crit * input.stats.crit_rate;

    Ok(DamageResult {
        non_crit,
        crit,
        average,
    })
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
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
        }
    }

    fn valid_enemy() -> Enemy {
        Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
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
            character_level: 91,
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert_eq!(result, Err(CalcError::InvalidCharacterLevel(91)));
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
            level: 101,
            ..valid_enemy()
        };
        let result = calculate_damage(&valid_input(), &enemy);
        assert_eq!(result, Err(CalcError::InvalidEnemyLevel(101)));
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
        };
        assert!((resistance_multiplier(&enemy) - 1.1).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_zero() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.0,
            def_reduction: 0.0,
        };
        assert!((resistance_multiplier(&enemy) - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_normal() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.1,
            def_reduction: 0.0,
        };
        assert!((resistance_multiplier(&enemy) - 0.9).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_boundary_below_75() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.74,
            def_reduction: 0.0,
        };
        assert!((resistance_multiplier(&enemy) - 0.26).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_at_75() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.75,
            def_reduction: 0.0,
        };
        assert!((resistance_multiplier(&enemy) - 0.25).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_high() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.9,
            def_reduction: 0.0,
        };
        assert!((resistance_multiplier(&enemy) - 1.0 / 4.6).abs() < EPSILON);
    }

    #[test]
    fn test_defense_multiplier_same_level() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.0,
            def_reduction: 0.0,
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
        };
        let result = defense_multiplier(90, &enemy);
        assert!((result - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_base_damage() {
        let input = valid_input();
        let result = base_damage(&input);
        assert!((result - 2000.0 * 1.76 * 1.466).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_damage_golden() {
        // base = 2000 * 1.76 * (1 + 0.466) = 2000 * 1.76 * 1.466 = 5160.32
        // def = 0.5, res = 1.0 - 0.10 = 0.9
        // non_crit = 5160.32 * 0.5 * 0.9 = 2322.144
        // crit = 2322.144 * (1 + 1.0) = 4644.288
        // avg = 2322.144 * 0.5 + 4644.288 * 0.5 = 3483.216
        let result = calculate_damage(&valid_input(), &valid_enemy()).unwrap();
        assert!((result.non_crit - 2322.144).abs() < 0.01);
        assert!((result.crit - 4644.288).abs() < 0.01);
        assert!((result.average - 3483.216).abs() < 0.01);
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
    }

    #[test]
    fn test_damage_input_serde_roundtrip() {
        let input = valid_input();
        let json = serde_json::to_string(&input).unwrap();
        let deserialized: DamageInput = serde_json::from_str(&json).unwrap();
        assert_eq!(input, deserialized);
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
            damage_type: DamageType::Normal,
            element: None, // physical
        };
        let enemy = Enemy {
            level: 85,
            resistance: 0.1, // Hilichurl physical resistance
            def_reduction: 0.0,
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
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
        };
        let enemy = Enemy {
            level: 90,
            resistance: 0.1,
            def_reduction: 0.0,
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
            damage_type: DamageType::Charged,
            element: Some(Element::Cryo),
        };
        let enemy = Enemy {
            level: 90,
            resistance: 0.1,
            def_reduction: 0.0,
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
            damage_type: DamageType::Burst,
            element: Some(Element::Electro),
        };
        let enemy = Enemy {
            level: 100,
            resistance: 0.1,
            def_reduction: 0.0,
        };
        let result = calculate_damage(&input, &enemy).unwrap();
        // DEF mult = 190/390 = 0.48718 (not 0.5)
        assert!((result.non_crit - 8244.098363).abs() < 0.01);
        assert!((result.crit - 17312.606562).abs() < 0.01);
    }
}
