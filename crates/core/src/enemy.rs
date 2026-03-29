use serde::{Deserialize, Serialize};

use crate::buff_types::BuffableStat;
use crate::team::ResolvedBuff;
use crate::types::Element;

/// Enemy parameters for damage calculation.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Enemy {
    /// Enemy level (1-100).
    pub level: u32,
    /// Elemental resistance in decimal form (e.g. 0.10 = 10%).
    pub resistance: f64,
    /// DEF reduction from debuffs in decimal form (0.0 to 1.0).
    pub def_reduction: f64,
}

/// Applies enemy debuffs (resistance reduction, DEF reduction) from resolved buffs.
///
/// Filters buffs by `BuffableStat` type:
/// - `ElementalResReduction(e)`: reduces `resistance` when `element == Some(e)`
/// - `PhysicalResReduction`: reduces `resistance` when `element == None`
/// - `DefReduction`: adds to `def_reduction` (clamped to 1.0)
///
/// Other `BuffableStat` variants are ignored.
/// Returns a new `Enemy` (immutable pattern).
pub fn apply_enemy_debuffs(
    enemy: &Enemy,
    buffs: &[ResolvedBuff],
    element: Option<Element>,
) -> Enemy {
    let mut res_reduction = 0.0;
    let mut def_reduction_add = 0.0;

    for buff in buffs {
        match buff.stat {
            BuffableStat::ElementalResReduction(e) => {
                if element == Some(e) {
                    res_reduction += buff.value;
                }
            }
            BuffableStat::PhysicalResReduction => {
                if element.is_none() {
                    res_reduction += buff.value;
                }
            }
            BuffableStat::DefReduction => {
                def_reduction_add += buff.value;
            }
            _ => {}
        }
    }

    Enemy {
        level: enemy.level,
        resistance: enemy.resistance - res_reduction,
        def_reduction: f64::min(1.0, enemy.def_reduction + def_reduction_add),
    }
}

/// Creates a ResolvedBuff for Superconduct's physical resistance reduction (-40%).
///
/// Consumer adds this to the buff list when Superconduct reaction is active.
pub fn superconduct_debuff() -> ResolvedBuff {
    ResolvedBuff {
        source: "Superconduct".to_string(),
        stat: BuffableStat::PhysicalResReduction,
        value: 0.40,
        target: crate::team::BuffTarget::Team,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::team::BuffTarget;

    const EPSILON: f64 = 1e-6;

    fn base_enemy() -> Enemy {
        Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        }
    }

    fn res_reduction_buff(element: Element, value: f64) -> ResolvedBuff {
        ResolvedBuff {
            source: "Test RES Shred".into(),
            stat: BuffableStat::ElementalResReduction(element),
            value,
            target: BuffTarget::Team,
        }
    }

    #[test]
    fn test_empty_buffs_no_change() {
        let enemy = base_enemy();
        let result = apply_enemy_debuffs(&enemy, &[], Some(Element::Pyro));
        assert!((result.resistance - 0.10).abs() < EPSILON);
        assert!((result.def_reduction - 0.0).abs() < EPSILON);
        assert_eq!(result.level, 90);
    }

    #[test]
    fn test_elemental_res_reduction_matching() {
        let enemy = base_enemy();
        let buffs = vec![res_reduction_buff(Element::Pyro, 0.40)];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        assert!((result.resistance - (-0.30)).abs() < EPSILON);
    }

    #[test]
    fn test_elemental_res_reduction_non_matching_skipped() {
        let enemy = base_enemy();
        let buffs = vec![res_reduction_buff(Element::Pyro, 0.40)];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Cryo));
        assert!((result.resistance - 0.10).abs() < EPSILON);
    }

    #[test]
    fn test_physical_res_reduction() {
        let enemy = base_enemy();
        let buffs = vec![ResolvedBuff {
            source: "Superconduct".into(),
            stat: BuffableStat::PhysicalResReduction,
            value: 0.40,
            target: BuffTarget::Team,
        }];
        let result = apply_enemy_debuffs(&enemy, &buffs, None);
        assert!((result.resistance - (-0.30)).abs() < EPSILON);
    }

    #[test]
    fn test_physical_res_reduction_ignored_for_elemental() {
        let enemy = base_enemy();
        let buffs = vec![ResolvedBuff {
            source: "Superconduct".into(),
            stat: BuffableStat::PhysicalResReduction,
            value: 0.40,
            target: BuffTarget::Team,
        }];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        assert!((result.resistance - 0.10).abs() < EPSILON);
    }

    #[test]
    fn test_multiple_debuffs_stack_additively() {
        let enemy = base_enemy();
        let buffs = vec![
            res_reduction_buff(Element::Pyro, 0.40),
            res_reduction_buff(Element::Pyro, 0.20),
        ];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        assert!((result.resistance - (-0.50)).abs() < EPSILON);
    }

    #[test]
    fn test_negative_resistance_no_floor() {
        let enemy = base_enemy();
        let buffs = vec![res_reduction_buff(Element::Pyro, 0.80)];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        assert!((result.resistance - (-0.70)).abs() < EPSILON);
    }

    #[test]
    fn test_def_reduction() {
        let enemy = base_enemy();
        let buffs = vec![ResolvedBuff {
            source: "Lisa A4".into(),
            stat: BuffableStat::DefReduction,
            value: 0.15,
            target: BuffTarget::Team,
        }];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Electro));
        assert!((result.def_reduction - 0.15).abs() < EPSILON);
    }

    #[test]
    fn test_def_reduction_adds_to_existing() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.20,
        };
        let buffs = vec![ResolvedBuff {
            source: "Lisa A4".into(),
            stat: BuffableStat::DefReduction,
            value: 0.15,
            target: BuffTarget::Team,
        }];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Electro));
        assert!((result.def_reduction - 0.35).abs() < EPSILON);
    }

    #[test]
    fn test_def_reduction_clamped_at_1() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.80,
        };
        let buffs = vec![ResolvedBuff {
            source: "Test".into(),
            stat: BuffableStat::DefReduction,
            value: 0.50,
            target: BuffTarget::Team,
        }];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Electro));
        assert!((result.def_reduction - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_ally_buffs_ignored() {
        let enemy = base_enemy();
        let buffs = vec![
            ResolvedBuff {
                source: "Bennett".into(),
                stat: BuffableStat::AtkFlat,
                value: 1000.0,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "Noblesse".into(),
                stat: BuffableStat::AtkPercent,
                value: 0.20,
                target: BuffTarget::Team,
            },
        ];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        assert!((result.resistance - 0.10).abs() < EPSILON);
        assert!((result.def_reduction - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_physical_attack_zhongli_only_physical_shred() {
        let enemy = base_enemy();
        let buffs = vec![
            res_reduction_buff(Element::Pyro, 0.20),
            res_reduction_buff(Element::Hydro, 0.20),
            res_reduction_buff(Element::Electro, 0.20),
            res_reduction_buff(Element::Cryo, 0.20),
            res_reduction_buff(Element::Dendro, 0.20),
            res_reduction_buff(Element::Anemo, 0.20),
            res_reduction_buff(Element::Geo, 0.20),
            ResolvedBuff {
                source: "Zhongli Shield".into(),
                stat: BuffableStat::PhysicalResReduction,
                value: 0.20,
                target: BuffTarget::Team,
            },
        ];
        let result = apply_enemy_debuffs(&enemy, &buffs, None);
        assert!((result.resistance - (-0.10)).abs() < EPSILON);
    }

    #[test]
    fn test_superconduct_debuff_values() {
        let debuff = superconduct_debuff();
        assert_eq!(debuff.stat, BuffableStat::PhysicalResReduction);
        assert!((debuff.value - 0.40).abs() < EPSILON);
    }

    // Golden tests: hand-calculated resistance multiplier after shred
    #[test]
    fn test_golden_res_10_shred_20() {
        // 10% base - 20% shred = -10% → multiplier = 1 - (-0.10)/2 = 1.05
        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };
        let buffs = vec![res_reduction_buff(Element::Pyro, 0.20)];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        assert!((result.resistance - (-0.10)).abs() < EPSILON);
        let mult = crate::damage::resistance_multiplier(&result);
        assert!((mult - 1.05).abs() < EPSILON);
    }

    #[test]
    fn test_golden_res_10_shred_60() {
        // 10% base - 60% shred = -50% → multiplier = 1 - (-0.50)/2 = 1.25
        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };
        let buffs = vec![
            res_reduction_buff(Element::Pyro, 0.40),
            res_reduction_buff(Element::Pyro, 0.20),
        ];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        assert!((result.resistance - (-0.50)).abs() < EPSILON);
        let mult = crate::damage::resistance_multiplier(&result);
        assert!((mult - 1.25).abs() < EPSILON);
    }

    #[test]
    fn test_golden_res_70_shred_40() {
        // 70% base - 40% shred = 30% → multiplier = 1 - 0.30 = 0.70
        let enemy = Enemy {
            level: 90,
            resistance: 0.70,
            def_reduction: 0.0,
        };
        let buffs = vec![res_reduction_buff(Element::Pyro, 0.40)];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        assert!((result.resistance - 0.30).abs() < EPSILON);
        let mult = crate::damage::resistance_multiplier(&result);
        assert!((mult - 0.70).abs() < EPSILON);
    }

    #[test]
    fn test_integration_vv_zhongli_stacked_damage() {
        // VV (-40%) + Zhongli (-20%) on 10% RES enemy
        // 0.10 - 0.40 - 0.20 = -0.50 → res_mult 1.25
        // original res_mult = 1 - 0.10 = 0.90
        use crate::damage::{DamageInput, calculate_damage};
        use crate::stats::Stats;
        use crate::types::{DamageType, ScalingStat};

        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };
        let buffs = vec![
            res_reduction_buff(Element::Pyro, 0.40), // VV
            res_reduction_buff(Element::Pyro, 0.20), // Zhongli
        ];
        let debuffed = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));

        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.0,
                crit_dmg: 0.0,
                dmg_bonus: 0.0,
                ..Default::default()
            },
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };

        let result_no_debuff = calculate_damage(&input, &enemy).unwrap();
        let result_debuffed = calculate_damage(&input, &debuffed).unwrap();

        assert!(result_debuffed.non_crit > result_no_debuff.non_crit);

        // Ratio: 1.25 / 0.90 = 1.3888...
        let ratio = result_debuffed.non_crit / result_no_debuff.non_crit;
        let expected_ratio = 1.25 / 0.90;
        assert!((ratio - expected_ratio).abs() < 0.001);
    }

    #[test]
    fn test_integration_superconduct_physical() {
        use crate::damage::{DamageInput, calculate_damage};
        use crate::stats::Stats;
        use crate::types::{DamageType, ScalingStat};

        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };
        let buffs = vec![superconduct_debuff()]; // -40% phys
        let debuffed = apply_enemy_debuffs(&enemy, &buffs, None); // physical

        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.0,
                crit_dmg: 0.0,
                dmg_bonus: 0.0,
                ..Default::default()
            },
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: None, // physical
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };

        let result_no_debuff = calculate_damage(&input, &enemy).unwrap();
        let result_debuffed = calculate_damage(&input, &debuffed).unwrap();

        // 0.10 - 0.40 = -0.30 → mult 1.15 vs original 0.90
        let ratio = result_debuffed.non_crit / result_no_debuff.non_crit;
        let expected_ratio = 1.15 / 0.90;
        assert!((ratio - expected_ratio).abs() < 0.001);
    }

    #[test]
    fn test_integration_lisa_def_reduction() {
        use crate::damage::{DamageInput, calculate_damage};
        use crate::stats::Stats;
        use crate::types::{DamageType, ScalingStat};

        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };
        let buffs = vec![ResolvedBuff {
            source: "Lisa A4".into(),
            stat: BuffableStat::DefReduction,
            value: 0.15,
            target: BuffTarget::Team,
        }];
        let debuffed = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Electro));

        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.0,
                crit_dmg: 0.0,
                dmg_bonus: 0.0,
                ..Default::default()
            },
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: Some(Element::Electro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };

        let result_no_debuff = calculate_damage(&input, &enemy).unwrap();
        let result_debuffed = calculate_damage(&input, &debuffed).unwrap();

        assert!(result_debuffed.non_crit > result_no_debuff.non_crit);

        // def_mult_debuffed = 190 / (190 + 190*0.85) = 190/351.5
        // def_mult_original = 190 / (190 + 190) = 0.5
        let def_mult_debuffed = 190.0 / (190.0 + 190.0 * 0.85);
        let def_mult_original = 0.5;
        let expected_ratio = def_mult_debuffed / def_mult_original;
        let ratio = result_debuffed.non_crit / result_no_debuff.non_crit;
        assert!((ratio - expected_ratio).abs() < 0.001);
    }
}
