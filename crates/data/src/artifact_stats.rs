//! Artifact main stat definitions.
//!
//! Provides main stat values for each artifact slot, rarity, and level.
//! Levels supported: 1, 4, 8, 12, 16, 20 (matching artifact enhancement intervals)

use crate::buff::BuffableStat;
use crate::types::ArtifactRarity;
use genshin_calc_core::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArtifactSlot {
    Flower,
    Plume,
    Sands,
    Goblet,
    Circlet,
}

impl ArtifactSlot {
    pub fn main_stats(&self) -> &'static [BuffableStat] {
        match self {
            ArtifactSlot::Flower => &[BuffableStat::HpFlat],
            ArtifactSlot::Plume => &[BuffableStat::AtkFlat],
            ArtifactSlot::Sands => &[
                BuffableStat::HpPercent,
                BuffableStat::AtkPercent,
                BuffableStat::DefPercent,
                BuffableStat::ElementalMastery,
                BuffableStat::EnergyRecharge,
            ],
            ArtifactSlot::Goblet => &[
                BuffableStat::HpPercent,
                BuffableStat::AtkPercent,
                BuffableStat::DefPercent,
                BuffableStat::ElementalMastery,
                BuffableStat::EnergyRecharge,
                BuffableStat::ElementalDmgBonus(Element::Pyro),
                BuffableStat::ElementalDmgBonus(Element::Hydro),
                BuffableStat::ElementalDmgBonus(Element::Electro),
                BuffableStat::ElementalDmgBonus(Element::Cryo),
                BuffableStat::ElementalDmgBonus(Element::Anemo),
                BuffableStat::ElementalDmgBonus(Element::Geo),
                BuffableStat::ElementalDmgBonus(Element::Dendro),
                BuffableStat::PhysicalDmgBonus,
            ],
            ArtifactSlot::Circlet => &[
                BuffableStat::HpPercent,
                BuffableStat::AtkPercent,
                BuffableStat::DefPercent,
                BuffableStat::ElementalMastery,
                BuffableStat::EnergyRecharge,
                BuffableStat::CritRate,
                BuffableStat::CritDmg,
                BuffableStat::HealingBonus,
            ],
        }
    }
}

const LEVELS: [u8; 6] = [1, 4, 8, 12, 16, 20];

fn level_to_index(level: u8) -> usize {
    match level {
        1 => 0,
        4 => 1,
        8 => 2,
        12 => 3,
        16 => 4,
        20 => 5,
        _ => panic!(
            "Invalid artifact level: {}. Must be 1, 4, 8, 12, 16, or 20.",
            level
        ),
    }
}

const STAR5_HP_FLOWER: [f64; 6] = [430.0, 1893.0, 2645.0, 3571.0, 4717.0, 4780.0];
const STAR5_ATK_PLUME: [f64; 6] = [28.0, 123.0, 234.0, 247.0, 311.0, 311.0];

const STAR5_HP_PERCENT: [f64; 6] = [5.1, 22.5, 30.9, 34.0, 45.3, 46.6];
const STAR5_ATK_PERCENT: [f64; 6] = [5.1, 22.5, 30.9, 34.0, 45.3, 46.6];
const STAR5_DEF_PERCENT: [f64; 6] = [6.3, 28.0, 38.6, 42.4, 56.6, 58.3];
const STAR5_EM: [f64; 6] = [20.5, 89.9, 122.3, 135.8, 181.6, 186.5];
const STAR5_ER: [f64; 6] = [5.6, 24.8, 34.0, 37.5, 50.1, 51.8];
const STAR5_ELEMENTAL_DMG: [f64; 6] = [5.1, 22.5, 30.9, 34.0, 45.3, 46.6];
const STAR5_PHYSICAL_DMG: [f64; 6] = [6.3, 28.0, 38.6, 42.4, 56.6, 58.3];
const STAR5_CRIT_RATE: [f64; 6] = [3.4, 15.0, 20.6, 22.6, 30.1, 31.1];
const STAR5_CRIT_DMG: [f64; 6] = [6.8, 30.0, 41.3, 45.4, 60.5, 62.2];
const STAR5_HEALING: [f64; 6] = [3.9, 17.3, 24.2, 26.6, 35.4, 35.9];

const STAR4_HP_FLOWER: [f64; 6] = [375.0, 1653.0, 2310.0, 3119.0, 4119.0, 4175.0];
const STAR4_ATK_PLUME: [f64; 6] = [24.0, 107.0, 204.0, 216.0, 272.0, 272.0];

const STAR4_HP_PERCENT: [f64; 6] = [4.4, 19.7, 27.0, 29.7, 39.6, 40.7];
const STAR4_ATK_PERCENT: [f64; 6] = [4.4, 19.7, 27.0, 29.7, 39.6, 40.7];
const STAR4_DEF_PERCENT: [f64; 6] = [5.5, 24.4, 33.7, 37.1, 49.4, 50.8];
const STAR4_EM: [f64; 6] = [17.9, 78.5, 106.9, 118.6, 158.6, 162.9];
const STAR4_ER: [f64; 6] = [4.9, 21.7, 29.7, 32.7, 43.7, 45.2];
const STAR4_ELEMENTAL_DMG: [f64; 6] = [4.4, 19.7, 27.0, 29.7, 39.6, 40.7];
const STAR4_PHYSICAL_DMG: [f64; 6] = [5.5, 24.4, 33.7, 37.1, 49.4, 50.8];
const STAR4_CRIT_RATE: [f64; 6] = [3.0, 13.1, 18.0, 19.8, 26.3, 27.2];
const STAR4_CRIT_DMG: [f64; 6] = [5.9, 26.2, 36.1, 39.7, 52.8, 54.3];
const STAR4_HEALING: [f64; 6] = [3.4, 15.1, 21.1, 23.2, 30.9, 31.4];

fn get_flat_table(rarity: ArtifactRarity) -> Option<(&'static [f64; 6], &'static [f64; 6])> {
    match rarity {
        ArtifactRarity::Star5 => Some((&STAR5_HP_FLOWER, &STAR5_ATK_PLUME)),
        ArtifactRarity::Star4 => Some((&STAR4_HP_FLOWER, &STAR4_ATK_PLUME)),
        ArtifactRarity::Star4And5 => None,
    }
}

fn get_percent_table(rarity: ArtifactRarity, stat: &BuffableStat) -> Option<&'static [f64; 6]> {
    match rarity {
        ArtifactRarity::Star5 => match stat {
            BuffableStat::HpPercent => Some(&STAR5_HP_PERCENT),
            BuffableStat::AtkPercent => Some(&STAR5_ATK_PERCENT),
            BuffableStat::DefPercent => Some(&STAR5_DEF_PERCENT),
            BuffableStat::ElementalMastery => Some(&STAR5_EM),
            BuffableStat::EnergyRecharge => Some(&STAR5_ER),
            BuffableStat::ElementalDmgBonus(_) => Some(&STAR5_ELEMENTAL_DMG),
            BuffableStat::PhysicalDmgBonus => Some(&STAR5_PHYSICAL_DMG),
            BuffableStat::CritRate => Some(&STAR5_CRIT_RATE),
            BuffableStat::CritDmg => Some(&STAR5_CRIT_DMG),
            BuffableStat::HealingBonus => Some(&STAR5_HEALING),
            _ => None,
        },
        ArtifactRarity::Star4 => match stat {
            BuffableStat::HpPercent => Some(&STAR4_HP_PERCENT),
            BuffableStat::AtkPercent => Some(&STAR4_ATK_PERCENT),
            BuffableStat::DefPercent => Some(&STAR4_DEF_PERCENT),
            BuffableStat::ElementalMastery => Some(&STAR4_EM),
            BuffableStat::EnergyRecharge => Some(&STAR4_ER),
            BuffableStat::ElementalDmgBonus(_) => Some(&STAR4_ELEMENTAL_DMG),
            BuffableStat::PhysicalDmgBonus => Some(&STAR4_PHYSICAL_DMG),
            BuffableStat::CritRate => Some(&STAR4_CRIT_RATE),
            BuffableStat::CritDmg => Some(&STAR4_CRIT_DMG),
            BuffableStat::HealingBonus => Some(&STAR4_HEALING),
            _ => None,
        },
        ArtifactRarity::Star4And5 => None,
    }
}

pub fn artifact_main_stat_value(
    rarity: ArtifactRarity,
    slot: ArtifactSlot,
    stat: &BuffableStat,
    level: u8,
) -> Option<f64> {
    let idx = level_to_index(level);

    match slot {
        ArtifactSlot::Flower => {
            if matches!(stat, BuffableStat::HpFlat) {
                if let Some((hp_table, _)) = get_flat_table(rarity) {
                    return Some(hp_table[idx]);
                }
            }
        }
        ArtifactSlot::Plume => {
            if matches!(stat, BuffableStat::AtkFlat) {
                if let Some((_, atk_table)) = get_flat_table(rarity) {
                    return Some(atk_table[idx]);
                }
            }
        }
        ArtifactSlot::Sands | ArtifactSlot::Goblet | ArtifactSlot::Circlet => {
            if let Some(table) = get_percent_table(rarity, stat) {
                return Some(table[idx]);
            }
        }
    }
    None
}

pub fn available_levels() -> [u8; 6] {
    LEVELS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flower_hp_star5_lv20() {
        let value = artifact_main_stat_value(
            ArtifactRarity::Star5,
            ArtifactSlot::Flower,
            &BuffableStat::HpFlat,
            20,
        );
        assert!((value.unwrap() - 4780.0).abs() < 1e-6);
    }

    #[test]
    fn test_plume_atk_star5_lv20() {
        let value = artifact_main_stat_value(
            ArtifactRarity::Star5,
            ArtifactSlot::Plume,
            &BuffableStat::AtkFlat,
            20,
        );
        assert!((value.unwrap() - 311.0).abs() < 1e-6);
    }

    #[test]
    fn test_circlet_critdmg_star5_lv20() {
        let value = artifact_main_stat_value(
            ArtifactRarity::Star5,
            ArtifactSlot::Circlet,
            &BuffableStat::CritDmg,
            20,
        );
        assert!((value.unwrap() - 62.2).abs() < 1e-6);
    }

    #[test]
    fn test_sands_er_star5_lv20() {
        let value = artifact_main_stat_value(
            ArtifactRarity::Star5,
            ArtifactSlot::Sands,
            &BuffableStat::EnergyRecharge,
            20,
        );
        assert!((value.unwrap() - 51.8).abs() < 1e-6);
    }

    #[test]
    fn test_invalid_stat_for_slot() {
        let value = artifact_main_stat_value(
            ArtifactRarity::Star5,
            ArtifactSlot::Flower,
            &BuffableStat::AtkPercent,
            20,
        );
        assert!(value.is_none());
    }
}
