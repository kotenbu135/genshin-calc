use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Ningguang
// =============================================================================

// -- Normal Attack: 千金キ擲 (Sparkling Scatter) -- Geo (Catalyst) --

const NINGGUANG_NORMAL: TalentScaling = TalentScaling {
    name: "通常攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.2800, 0.3010, 0.3220, 0.3500, 0.3710, 0.3920, 0.4200, 0.4480, 0.4760, 0.5040, 0.5330,
        0.5712, 0.6094, 0.6475, 0.6857,
    ],
};

// -- Charged Attack -- Geo (Catalyst) --

const NINGGUANG_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.7408, 1.8714, 2.0019, 2.1760, 2.3066, 2.4371, 2.6112, 2.7853, 2.9594, 3.1334, 3.3133,
        3.5500, 3.7866, 4.0233, 4.2600,
    ],
};

const NINGGUANG_CHARGED_STAR_JADE: TalentScaling = TalentScaling {
    name: "星璃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.4960, 0.5332, 0.5704, 0.6200, 0.6572, 0.6944, 0.7440, 0.7936, 0.8432, 0.8928, 0.9443,
        1.0118, 1.0793, 1.1468, 1.2143,
    ],
};

// -- Plunging Attack -- Geo (Catalyst) --

const NINGGUANG_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const NINGGUANG_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const NINGGUANG_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 璇璣キ屏 (Jade Screen) -- Geo --

const NINGGUANG_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        2.3040, 2.4768, 2.6496, 2.8800, 3.0528, 3.2256, 3.4560, 3.6864, 3.9168, 4.1472, 4.3776,
        4.6080, 4.8960, 5.1840, 5.4720,
    ],
};

// -- Elemental Burst: 天キ権キ典 (Starshatter) -- Geo --

const NINGGUANG_BURST: TalentScaling = TalentScaling {
    name: "宝石ダメージ (1個)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.8696, 0.9348, 1.0000, 1.0870, 1.1522, 1.2174, 1.3044, 1.3914, 1.4783, 1.5653, 1.6522,
        1.7392, 1.8479, 1.9566, 2.0653,
    ],
};

pub const NINGGUANG: CharacterData = CharacterData {
    id: "ningguang",
    name: "Ningguang",
    element: Element::Geo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [
        821.00, 2108.00, 2721.00, 4076.00, 4512.00, 5189.00, 5770.00, 6448.00, 6884.00, 7561.00,
        7996.00, 8674.00, 9110.00, 9787.00, 9787.00, 10178.48, // Lv95/Lv95+/Lv100
        10178.48, // Lv95/Lv95+/Lv100
        10569.96, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        17.81, 45.75, 59.05, 88.45, 97.91, 112.62, 125.22, 139.93, 149.38, 164.07, 173.53, 188.24,
        197.69, 212.40, 212.40, 220.90, // Lv95/Lv95+/Lv100
        220.90, // Lv95/Lv95+/Lv100
        229.39, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        48.07, 123.49, 159.40, 238.76, 264.28, 303.98, 338.00, 377.71, 403.22, 442.88, 468.39,
        508.10, 533.61, 573.32, 573.32, 596.25, // Lv95/Lv95+/Lv100
        596.25, // Lv95/Lv95+/Lv100
        619.19, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Geo, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "千金キ擲",
            hits: &[NINGGUANG_NORMAL],
            charged: &[NINGGUANG_CHARGED, NINGGUANG_CHARGED_STAR_JADE],
            plunging: &[
                NINGGUANG_PLUNGE,
                NINGGUANG_PLUNGE_LOW,
                NINGGUANG_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "璇璣キ屏",
            scalings: &[NINGGUANG_SKILL],
        },
        elemental_burst: TalentData {
            name: "天キ権キ典",
            scalings: &[NINGGUANG_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
