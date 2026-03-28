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
}
