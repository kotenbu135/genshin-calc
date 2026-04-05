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

/// Aggregated enemy debuffs from team buffs.
///
/// Each field represents the total resistance reduction or DEF reduction
/// collected from all team members' buffs. Use [`apply_debuffs_to_enemy`]
/// to apply these to a base [`Enemy`].
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnemyDebuffs {
    /// Pyro resistance reduction total.
    pub pyro_res_reduction: f64,
    /// Hydro resistance reduction total.
    pub hydro_res_reduction: f64,
    /// Electro resistance reduction total.
    pub electro_res_reduction: f64,
    /// Cryo resistance reduction total.
    pub cryo_res_reduction: f64,
    /// Dendro resistance reduction total.
    pub dendro_res_reduction: f64,
    /// Anemo resistance reduction total.
    pub anemo_res_reduction: f64,
    /// Geo resistance reduction total.
    pub geo_res_reduction: f64,
    /// Physical resistance reduction total.
    pub physical_res_reduction: f64,
    /// DEF reduction total.
    pub def_reduction: f64,
}

/// Collects enemy debuffs from resolved team buffs into an [`EnemyDebuffs`].
///
/// Extracts `ElementalResReduction`, `PhysicalResReduction`, and `DefReduction`
/// buffs. All other buff types are ignored.
pub(crate) fn collect_enemy_debuffs(buffs: &[ResolvedBuff]) -> EnemyDebuffs {
    let mut d = EnemyDebuffs::default();
    for buff in buffs {
        match buff.stat {
            BuffableStat::ElementalResReduction(Element::Pyro) => {
                d.pyro_res_reduction += buff.value
            }
            BuffableStat::ElementalResReduction(Element::Hydro) => {
                d.hydro_res_reduction += buff.value
            }
            BuffableStat::ElementalResReduction(Element::Electro) => {
                d.electro_res_reduction += buff.value
            }
            BuffableStat::ElementalResReduction(Element::Cryo) => {
                d.cryo_res_reduction += buff.value
            }
            BuffableStat::ElementalResReduction(Element::Dendro) => {
                d.dendro_res_reduction += buff.value
            }
            BuffableStat::ElementalResReduction(Element::Anemo) => {
                d.anemo_res_reduction += buff.value
            }
            BuffableStat::ElementalResReduction(Element::Geo) => d.geo_res_reduction += buff.value,
            BuffableStat::PhysicalResReduction => d.physical_res_reduction += buff.value,
            BuffableStat::DefReduction => d.def_reduction += buff.value,
            _ => {}
        }
    }
    d
}

/// Applies pre-collected enemy debuffs to a base enemy for a specific damage element.
///
/// Selects the matching resistance reduction by element (or physical if `None`),
/// subtracts from `enemy.resistance`, and adds `def_reduction` (clamped to 1.0).
/// Returns a new `Enemy` (immutable pattern).
pub fn apply_debuffs_to_enemy(
    enemy: &Enemy,
    debuffs: &EnemyDebuffs,
    element: Option<Element>,
) -> Enemy {
    let res_reduction = match element {
        Some(Element::Pyro) => debuffs.pyro_res_reduction,
        Some(Element::Hydro) => debuffs.hydro_res_reduction,
        Some(Element::Electro) => debuffs.electro_res_reduction,
        Some(Element::Cryo) => debuffs.cryo_res_reduction,
        Some(Element::Dendro) => debuffs.dendro_res_reduction,
        Some(Element::Anemo) => debuffs.anemo_res_reduction,
        Some(Element::Geo) => debuffs.geo_res_reduction,
        None => debuffs.physical_res_reduction,
    };
    Enemy {
        level: enemy.level,
        resistance: enemy.resistance - res_reduction,
        def_reduction: f64::min(1.0, enemy.def_reduction + debuffs.def_reduction),
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

    // Task 2: EnemyDebuffs tests
    #[test]
    fn test_collect_enemy_debuffs_empty() {
        let debuffs = collect_enemy_debuffs(&[]);
        assert!((debuffs.pyro_res_reduction - 0.0).abs() < EPSILON);
        assert!((debuffs.def_reduction - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_collect_enemy_debuffs_all_elements() {
        let buffs = vec![
            ResolvedBuff {
                source: "Citlali Q".into(),
                stat: BuffableStat::ElementalResReduction(Element::Pyro),
                value: 0.20,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "Citlali Q".into(),
                stat: BuffableStat::ElementalResReduction(Element::Cryo),
                value: 0.20,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "VV".into(),
                stat: BuffableStat::ElementalResReduction(Element::Hydro),
                value: 0.40,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "Superconduct".into(),
                stat: BuffableStat::PhysicalResReduction,
                value: 0.40,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "Lisa A4".into(),
                stat: BuffableStat::DefReduction,
                value: 0.15,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "Bennett".into(),
                stat: BuffableStat::AtkFlat,
                value: 1000.0,
                target: BuffTarget::Team,
            },
        ];
        let debuffs = collect_enemy_debuffs(&buffs);
        assert!((debuffs.pyro_res_reduction - 0.20).abs() < EPSILON);
        assert!((debuffs.cryo_res_reduction - 0.20).abs() < EPSILON);
        assert!((debuffs.hydro_res_reduction - 0.40).abs() < EPSILON);
        assert!((debuffs.electro_res_reduction - 0.0).abs() < EPSILON);
        assert!((debuffs.dendro_res_reduction - 0.0).abs() < EPSILON);
        assert!((debuffs.anemo_res_reduction - 0.0).abs() < EPSILON);
        assert!((debuffs.geo_res_reduction - 0.0).abs() < EPSILON);
        assert!((debuffs.physical_res_reduction - 0.40).abs() < EPSILON);
        assert!((debuffs.def_reduction - 0.15).abs() < EPSILON);
    }

    #[test]
    fn test_collect_enemy_debuffs_stacks() {
        let buffs = vec![
            ResolvedBuff {
                source: "VV".into(),
                stat: BuffableStat::ElementalResReduction(Element::Pyro),
                value: 0.40,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "Zhongli".into(),
                stat: BuffableStat::ElementalResReduction(Element::Pyro),
                value: 0.20,
                target: BuffTarget::Team,
            },
        ];
        let debuffs = collect_enemy_debuffs(&buffs);
        assert!((debuffs.pyro_res_reduction - 0.60).abs() < EPSILON);
    }

    #[test]
    fn test_apply_debuffs_to_enemy_pyro() {
        let enemy = base_enemy();
        let debuffs = EnemyDebuffs {
            pyro_res_reduction: 0.40,
            ..Default::default()
        };
        let result = apply_debuffs_to_enemy(&enemy, &debuffs, Some(Element::Pyro));
        assert!((result.resistance - (-0.30)).abs() < EPSILON);
        assert!((result.def_reduction - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_apply_debuffs_to_enemy_physical() {
        let enemy = base_enemy();
        let debuffs = EnemyDebuffs {
            physical_res_reduction: 0.40,
            ..Default::default()
        };
        let result = apply_debuffs_to_enemy(&enemy, &debuffs, None);
        assert!((result.resistance - (-0.30)).abs() < EPSILON);
    }

    #[test]
    fn test_apply_debuffs_to_enemy_wrong_element_no_effect() {
        let enemy = base_enemy();
        let debuffs = EnemyDebuffs {
            pyro_res_reduction: 0.40,
            ..Default::default()
        };
        let result = apply_debuffs_to_enemy(&enemy, &debuffs, Some(Element::Cryo));
        assert!((result.resistance - 0.10).abs() < EPSILON);
    }

    #[test]
    fn test_apply_debuffs_to_enemy_def_reduction_clamped() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.80,
        };
        let debuffs = EnemyDebuffs {
            def_reduction: 0.50,
            ..Default::default()
        };
        let result = apply_debuffs_to_enemy(&enemy, &debuffs, Some(Element::Pyro));
        assert!((result.def_reduction - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_apply_debuffs_to_enemy_combined() {
        let enemy = base_enemy();
        let debuffs = EnemyDebuffs {
            pyro_res_reduction: 0.40,
            cryo_res_reduction: 0.20,
            def_reduction: 0.15,
            ..Default::default()
        };
        let result = apply_debuffs_to_enemy(&enemy, &debuffs, Some(Element::Pyro));
        assert!((result.resistance - (-0.30)).abs() < EPSILON);
        assert!((result.def_reduction - 0.15).abs() < EPSILON);
    }

    #[test]
    fn test_full_pipeline_resolve_debuffs_damage() {
        use crate::damage::{DamageInput, calculate_damage};
        use crate::stat_profile::StatProfile;
        use crate::stats::Stats;
        use crate::team::{BuffTarget, ResolvedBuff, TeamMember, resolve_team_stats};
        use crate::types::{DamageType, ScalingStat, WeaponType};

        let dps = TeamMember {
            element: Element::Pyro,
            weapon_type: WeaponType::Claymore,
            stats: StatProfile {
                base_atk: 900.0,
                crit_rate: 0.70,
                crit_dmg: 1.50,
                energy_recharge: 1.0,
                ..Default::default()
            },
            buffs_provided: vec![],
            is_moonsign: false,
        };

        let support = TeamMember {
            element: Element::Cryo,
            weapon_type: WeaponType::Catalyst,
            stats: StatProfile {
                base_atk: 600.0,
                energy_recharge: 1.0,
                ..Default::default()
            },
            buffs_provided: vec![
                ResolvedBuff {
                    source: "Bennett Burst".into(),
                    stat: BuffableStat::AtkFlat,
                    value: 1000.0,
                    target: BuffTarget::Team,
                },
                ResolvedBuff {
                    source: "VV Pyro".into(),
                    stat: BuffableStat::ElementalResReduction(Element::Pyro),
                    value: 0.40,
                    target: BuffTarget::Team,
                },
                ResolvedBuff {
                    source: "Shenhe E".into(),
                    stat: BuffableStat::NormalAtkFlatDmg,
                    value: 2500.0,
                    target: BuffTarget::Team,
                },
                ResolvedBuff {
                    source: "Freedom-Sworn".into(),
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.16,
                    target: BuffTarget::Team,
                },
            ],
            is_moonsign: false,
        };

        let team = vec![dps, support];
        let result = resolve_team_stats(&team, 0).unwrap();

        // Verify stats: base_atk 900 + Bennett 1000 = 1900
        assert!((result.final_stats.atk - 1900.0).abs() < EPSILON);

        // Verify damage_context
        assert!((result.damage_context.normal_atk_dmg_bonus - 0.16).abs() < EPSILON);
        assert!((result.damage_context.normal_atk_flat_dmg - 2500.0).abs() < EPSILON);

        // Verify enemy_debuffs
        assert!((result.enemy_debuffs.pyro_res_reduction - 0.40).abs() < EPSILON);

        // Build DamageInput using the resolved data
        let mut stats = result.final_stats.clone();
        stats.dmg_bonus += result.damage_context.normal_atk_dmg_bonus;

        let input = DamageInput {
            character_level: 90,
            stats,
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: result.damage_context.normal_atk_flat_dmg,
        };

        let base_enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };
        let debuffed_enemy =
            apply_debuffs_to_enemy(&base_enemy, &result.enemy_debuffs, Some(Element::Pyro));

        // Verify debuffed enemy: 0.10 - 0.40 = -0.30
        assert!((debuffed_enemy.resistance - (-0.30)).abs() < EPSILON);

        // Calculate damage with and without the full pipeline
        let result_full = calculate_damage(&input, &debuffed_enemy).unwrap();

        // Compare with no-buff baseline
        let baseline_input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 900.0,
                crit_rate: 0.70,
                crit_dmg: 1.50,
                ..Default::default()
            },
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let _result_baseline = calculate_damage(&baseline_input, &base_enemy).unwrap();

        // Hand-calculated golden values:
        // base = 1900 * 1.76 + 2500 = 5844
        // dmg_bonus = 0.16, res = -0.30 → res_mult = 1.15, def_mult = 0.5
        // non_crit = 5844 * 1.16 * 0.5 * 1.15 = 3897.948
        let expected_non_crit = 3897.948;
        assert!((result_full.non_crit - expected_non_crit).abs() < 0.01);
        assert!(result_full.non_crit > 0.0);
        assert!(result_full.crit > result_full.non_crit);
    }
}
