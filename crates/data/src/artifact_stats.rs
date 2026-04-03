//! Artifact main stat definitions.
//!
//! Provides main stat values for each artifact slot, rarity, and level.
//! Levels supported: 0-20 (5-star), 0-16 (4-star)

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

pub fn available_levels() -> [u8; 21] {
    [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ]
}

fn is_valid_level(rarity: ArtifactRarity, level: u8) -> bool {
    match rarity {
        ArtifactRarity::Star5 => level <= 20,
        ArtifactRarity::Star4 => level <= 16,
        ArtifactRarity::Star4And5 => false,
    }
}

const STAR5_HP_FLOWER: [f64; 21] = [
    717.0, 920.0, 1123.0, 1326.0, 1530.0, 1733.0, 1936.0, 2139.0, 2342.0, 2545.0, 2749.0, 2952.0,
    3155.0, 3358.0, 3561.0, 3764.0, 3967.0, 4171.0, 4374.0, 4577.0, 4780.0,
];
const STAR5_ATK_PLUME: [f64; 21] = [
    47.0, 60.0, 73.0, 86.0, 100.0, 113.0, 126.0, 139.0, 152.0, 166.0, 179.0, 192.0, 205.0, 219.0,
    232.0, 245.0, 258.0, 272.0, 285.0, 298.0, 311.0,
];

const STAR5_HP_PERCENT: [f64; 21] = [
    7.0, 9.0, 11.0, 12.9, 14.9, 16.9, 18.9, 20.9, 22.8, 24.8, 26.8, 28.8, 30.8, 32.8, 34.7, 36.7,
    38.7, 40.7, 42.7, 44.6, 46.6,
];
const STAR5_ATK_PERCENT: [f64; 21] = [
    7.0, 9.0, 11.0, 12.9, 14.9, 16.9, 18.9, 20.9, 22.8, 24.8, 26.8, 28.8, 30.8, 32.8, 34.7, 36.7,
    38.7, 40.7, 42.7, 44.6, 46.6,
];
const STAR5_DEF_PERCENT: [f64; 21] = [
    8.7, 11.2, 13.7, 16.2, 18.6, 21.1, 23.6, 26.1, 28.6, 31.0, 33.5, 36.0, 38.5, 40.9, 43.4, 45.9,
    48.4, 50.8, 53.3, 55.8, 58.3,
];
const STAR5_EM: [f64; 21] = [
    28.0, 35.9, 43.8, 51.8, 59.7, 67.6, 75.5, 83.5, 91.4, 99.3, 107.2, 115.2, 123.1, 131.0, 138.9,
    146.9, 154.8, 162.7, 170.6, 178.6, 186.5,
];
const STAR5_ER: [f64; 21] = [
    7.8, 10.0, 12.2, 14.4, 16.6, 18.8, 21.0, 23.2, 25.4, 27.6, 29.8, 32.0, 34.2, 36.4, 38.6, 40.8,
    43.0, 45.2, 47.4, 49.6, 51.8,
];
const STAR5_ELEMENTAL_DMG: [f64; 21] = [
    7.0, 9.0, 11.0, 12.9, 14.9, 16.9, 18.9, 20.9, 22.8, 24.8, 26.8, 28.8, 30.8, 32.8, 34.7, 36.7,
    38.7, 40.7, 42.7, 44.6, 46.6,
];
const STAR5_PHYSICAL_DMG: [f64; 21] = [
    8.7, 11.2, 13.7, 16.2, 18.6, 21.1, 23.6, 26.1, 28.6, 31.0, 33.5, 36.0, 38.5, 40.9, 43.4, 45.9,
    48.4, 50.8, 53.3, 55.8, 58.3,
];
const STAR5_CRIT_RATE: [f64; 21] = [
    4.7, 6.0, 7.3, 8.6, 9.9, 11.3, 12.6, 13.9, 15.2, 16.6, 17.9, 19.2, 20.5, 21.8, 23.2, 24.5,
    25.8, 27.1, 28.4, 29.8, 31.1,
];
const STAR5_CRIT_DMG: [f64; 21] = [
    9.3, 12.0, 14.6, 17.3, 19.9, 22.5, 25.2, 27.8, 30.5, 33.1, 35.7, 38.4, 41.0, 43.7, 46.3, 49.0,
    51.6, 54.2, 56.9, 59.5, 62.2,
];
const STAR5_HEALING: [f64; 21] = [
    5.4, 6.9, 8.4, 10.0, 11.5, 13.0, 14.5, 16.1, 17.6, 19.1, 20.6, 22.1, 23.7, 25.2, 26.7, 28.2,
    29.8, 31.3, 32.8, 34.3, 35.9,
];

const STAR4_HP_FLOWER: [f64; 17] = [
    645.0, 828.0, 1011.0, 1194.0, 1377.0, 1559.0, 1742.0, 1925.0, 2108.0, 2291.0, 2474.0, 2657.0,
    2839.0, 3022.0, 3205.0, 3388.0, 3571.0,
];
const STAR4_ATK_PLUME: [f64; 17] = [
    42.0, 54.0, 66.0, 78.0, 90.0, 102.0, 113.0, 125.0, 137.0, 149.0, 161.0, 173.0, 185.0, 197.0,
    209.0, 221.0, 232.0,
];

const STAR4_HP_PERCENT: [f64; 17] = [
    6.3, 8.1, 9.9, 11.6, 13.4, 15.2, 17.0, 18.8, 20.6, 22.3, 24.1, 25.9, 27.7, 29.5, 31.3, 33.0,
    34.8,
];
const STAR4_ATK_PERCENT: [f64; 17] = [
    6.3, 8.1, 9.9, 11.6, 13.4, 15.2, 17.0, 18.8, 20.6, 22.3, 24.1, 25.9, 27.7, 29.5, 31.3, 33.0,
    34.8,
];
const STAR4_DEF_PERCENT: [f64; 17] = [
    7.9, 10.1, 12.3, 14.6, 16.8, 19.0, 21.2, 23.5, 25.7, 27.9, 30.2, 32.4, 34.6, 36.8, 39.1, 41.3,
    43.5,
];
const STAR4_EM: [f64; 17] = [
    25.2, 32.3, 39.4, 46.6, 53.7, 60.8, 68.0, 75.1, 82.2, 89.4, 96.5, 103.6, 110.8, 117.9, 125.0,
    132.2, 139.3,
];
const STAR4_ER: [f64; 17] = [
    7.0, 9.0, 11.0, 12.9, 14.9, 16.9, 18.9, 20.9, 22.8, 24.8, 26.8, 28.8, 30.8, 32.8, 34.7, 36.7,
    38.7,
];
const STAR4_ELEMENTAL_DMG: [f64; 17] = [
    6.3, 8.1, 9.9, 11.6, 13.4, 15.2, 17.0, 18.8, 20.6, 22.3, 24.1, 25.9, 27.7, 29.5, 31.3, 33.0,
    34.8,
];
const STAR4_PHYSICAL_DMG: [f64; 17] = [
    7.9, 10.1, 12.3, 14.6, 16.8, 19.0, 21.2, 23.5, 25.7, 27.9, 30.2, 32.4, 34.6, 36.8, 39.1, 41.3,
    43.5,
];
const STAR4_CRIT_RATE: [f64; 17] = [
    4.2, 5.4, 6.6, 7.8, 9.0, 10.1, 11.3, 12.5, 13.7, 14.9, 16.1, 17.3, 18.5, 19.7, 20.8, 22.0, 23.2,
];
const STAR4_CRIT_DMG: [f64; 17] = [
    8.4, 10.8, 13.1, 15.5, 17.9, 20.3, 22.7, 25.0, 27.4, 29.8, 32.2, 34.5, 36.9, 39.3, 41.7, 44.1,
    46.4,
];
const STAR4_HEALING: [f64; 17] = [
    4.8, 6.2, 7.6, 9.0, 10.3, 11.7, 13.1, 14.4, 15.8, 17.2, 18.6, 19.9, 21.3, 22.7, 24.0, 25.4,
    26.8,
];

fn get_flat_table(rarity: ArtifactRarity) -> Option<(&'static [f64], &'static [f64])> {
    match rarity {
        ArtifactRarity::Star5 => Some((&STAR5_HP_FLOWER, &STAR5_ATK_PLUME)),
        ArtifactRarity::Star4 => Some((&STAR4_HP_FLOWER, &STAR4_ATK_PLUME)),
        ArtifactRarity::Star4And5 => None,
    }
}

fn get_percent_table(rarity: ArtifactRarity, stat: &BuffableStat) -> Option<&'static [f64]> {
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
    if !is_valid_level(rarity, level) {
        return None;
    }

    match slot {
        ArtifactSlot::Flower => {
            if matches!(stat, BuffableStat::HpFlat) {
                if let Some((hp_table, _)) = get_flat_table(rarity) {
                    return Some(hp_table[level as usize]);
                }
            }
        }
        ArtifactSlot::Plume => {
            if matches!(stat, BuffableStat::AtkFlat) {
                if let Some((_, atk_table)) = get_flat_table(rarity) {
                    return Some(atk_table[level as usize]);
                }
            }
        }
        ArtifactSlot::Sands | ArtifactSlot::Goblet | ArtifactSlot::Circlet => {
            if let Some(table) = get_percent_table(rarity, stat) {
                return Some(table[level as usize]);
            }
        }
    }
    None
}

pub fn available_levels_by_rarity(rarity: ArtifactRarity) -> &'static [u8] {
    match rarity {
        ArtifactRarity::Star5 => &[
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
        ],
        ArtifactRarity::Star4 => &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        ArtifactRarity::Star4And5 => &[],
    }
}

pub fn artifact_main_stat_value_by_key(
    rarity: ArtifactRarity,
    slot: ArtifactSlot,
    stat_key: &str,
    level: u8,
) -> Option<f64> {
    let stat = match stat_key {
        "hp" => Some(BuffableStat::HpFlat),
        "atk" => Some(BuffableStat::AtkFlat),
        "def" => Some(BuffableStat::DefFlat),
        "hp_" => Some(BuffableStat::HpPercent),
        "atk_" => Some(BuffableStat::AtkPercent),
        "def_" => Some(BuffableStat::DefPercent),
        "eleMas" => Some(BuffableStat::ElementalMastery),
        "enerRech_" => Some(BuffableStat::EnergyRecharge),
        "critRate_" => Some(BuffableStat::CritRate),
        "critDMG_" => Some(BuffableStat::CritDmg),
        "physical_dmg_" => Some(BuffableStat::PhysicalDmgBonus),
        "pyro_dmg_" => Some(BuffableStat::ElementalDmgBonus(Element::Pyro)),
        "hydro_dmg_" => Some(BuffableStat::ElementalDmgBonus(Element::Hydro)),
        "electro_dmg_" => Some(BuffableStat::ElementalDmgBonus(Element::Electro)),
        "cryo_dmg_" => Some(BuffableStat::ElementalDmgBonus(Element::Cryo)),
        "dendro_dmg_" => Some(BuffableStat::ElementalDmgBonus(Element::Dendro)),
        "anemo_dmg_" => Some(BuffableStat::ElementalDmgBonus(Element::Anemo)),
        "geo_dmg_" => Some(BuffableStat::ElementalDmgBonus(Element::Geo)),
        _ => None,
    }?;

    artifact_main_stat_value(rarity, slot, &stat, level)
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

    #[test]
    fn test_star5_all_levels() {
        for level in 0..=20 {
            let hp = artifact_main_stat_value(
                ArtifactRarity::Star5,
                ArtifactSlot::Flower,
                &BuffableStat::HpFlat,
                level,
            );
            assert!(hp.is_some(), "Level {} should be valid for Star5", level);
        }
    }

    #[test]
    fn test_star4_levels() {
        for level in 0..=16 {
            let hp = artifact_main_stat_value(
                ArtifactRarity::Star4,
                ArtifactSlot::Flower,
                &BuffableStat::HpFlat,
                level,
            );
            assert!(hp.is_some(), "Level {} should be valid for Star4", level);
        }
        let hp = artifact_main_stat_value(
            ArtifactRarity::Star4,
            ArtifactSlot::Flower,
            &BuffableStat::HpFlat,
            17,
        );
        assert!(hp.is_none(), "Level 17 should be invalid for Star4");
    }

    #[test]
    fn test_flower_hp_star5_lv0() {
        let value = artifact_main_stat_value(
            ArtifactRarity::Star5,
            ArtifactSlot::Flower,
            &BuffableStat::HpFlat,
            0,
        );
        assert!((value.unwrap() - 717.0).abs() < 1e-6);
    }
}
