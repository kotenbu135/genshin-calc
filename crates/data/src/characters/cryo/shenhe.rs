use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Dawnstar Piercer -- Physical --

const SHENHE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4326, 0.4678, 0.5031, 0.5534, 0.5886, 0.6288, 0.6842, 0.7395, 0.7948, 0.8552, 0.9156,
        0.9760, 1.0363, 1.0967, 1.1571,
    ],
};

const SHENHE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4024, 0.4352, 0.4680, 0.5148, 0.5476, 0.5850, 0.6365, 0.6879, 0.7394, 0.7956, 0.8518,
        0.9080, 0.9642, 1.0204, 1.0766,
    ],
};

const SHENHE_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5332, 0.5766, 0.6200, 0.6820, 0.7254, 0.7750, 0.8432, 0.9114, 0.9796, 1.0540, 1.1284,
        1.2028, 1.2772, 1.3516, 1.4260,
    ],
};

const SHENHE_NORMAL_4A: TalentScaling = TalentScaling {
    name: "4段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2632, 0.2846, 0.3060, 0.3366, 0.3580, 0.3825, 0.4162, 0.4498, 0.4834, 0.5202, 0.5570,
        0.5938, 0.6306, 0.6674, 0.7042,
    ],
};

const SHENHE_NORMAL_4B: TalentScaling = TalentScaling {
    name: "4段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2632, 0.2846, 0.3060, 0.3366, 0.3580, 0.3825, 0.4162, 0.4498, 0.4834, 0.5202, 0.5570,
        0.5938, 0.6306, 0.6674, 0.7042,
    ],
};

const SHENHE_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6562, 0.7096, 0.7630, 0.8393, 0.8927, 0.9538, 1.0375, 1.1213, 1.2051, 1.2969, 1.3888,
        1.4806, 1.5724, 1.6642, 1.7560,
    ],
};

// -- Charged Attack -- Physical --

const SHENHE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1068, 1.1969, 1.2870, 1.4157, 1.5058, 1.6088, 1.7502, 1.8917, 2.0331, 2.1878, 2.3424,
        2.4970, 2.6517, 2.8063, 2.9609,
    ],
};

// -- Plunging Attack -- Physical --

const SHENHE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const SHENHE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const SHENHE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: Spring Spirit Summoning -- Cryo --

const SHENHE_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "短押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.3920, 1.4964, 1.6008, 1.7400, 1.8444, 1.9488, 2.0880, 2.2272, 2.3664, 2.5056, 2.6448,
        2.7840, 2.9580, 3.1320, 3.3060,
    ],
};

const SHENHE_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.8880, 2.0296, 2.1712, 2.3600, 2.5016, 2.6432, 2.8320, 3.0208, 3.2096, 3.3984, 3.5872,
        3.7760, 4.0120, 4.2480, 4.4840,
    ],
};

// -- Elemental Burst: Divine Maiden's Deliverance -- Cryo --

const SHENHE_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.0080, 1.0836, 1.1592, 1.2600, 1.3356, 1.4112, 1.5120, 1.6128, 1.7136, 1.8144, 1.9152,
        2.0160, 2.1420, 2.2680, 2.3940,
    ],
};

const SHENHE_BURST_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.3312, 0.3560, 0.3809, 0.4140, 0.4388, 0.4637, 0.4968, 0.5299, 0.5630, 0.5962, 0.6293,
        0.6624, 0.7038, 0.7452, 0.7866,
    ],
};

pub const SHENHE: CharacterData = CharacterData {
    id: "shenhe",
    name: "Shenhe",
    element: Element::Cryo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [
        1011.00, 2624.00, 3491.00, 5224.00, 5840.00, 6719.00, 7540.00, 8429.00, 9045.00, 9941.00,
        10557.00, 11463.00, 12080.00, 12993.00, 12993.00, 13512.72, // Lv95/Lv95+/Lv100
        13512.72, // Lv95/Lv95+/Lv100
        14032.44, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        23.65, 61.34, 81.62, 122.12, 136.53, 157.08, 176.29, 197.05, 211.45, 232.40, 246.81,
        268.00, 282.40, 303.76, 303.76, 315.91, // Lv95/Lv95+/Lv100
        315.91, // Lv95/Lv95+/Lv100
        328.06, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        64.62, 167.61, 223.02, 333.71, 373.07, 429.22, 481.71, 538.45, 577.81, 635.06, 674.43,
        732.32, 771.69, 830.04, 830.04, 863.24, // Lv95/Lv95+/Lv100
        863.24, // Lv95/Lv95+/Lv100
        896.44, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "踏辰摂斗",
            hits: &[
                SHENHE_NORMAL_1,
                SHENHE_NORMAL_2,
                SHENHE_NORMAL_3,
                SHENHE_NORMAL_4A,
                SHENHE_NORMAL_4B,
                SHENHE_NORMAL_5,
            ],
            charged: &[SHENHE_CHARGED],
            plunging: &[SHENHE_PLUNGE, SHENHE_PLUNGE_LOW, SHENHE_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "仰霊威召将役咒",
            scalings: &[SHENHE_SKILL_PRESS, SHENHE_SKILL_HOLD],
        },
        elemental_burst: TalentData {
            name: "神女遣霊真訣",
            scalings: &[SHENHE_BURST, SHENHE_BURST_DOT],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Skirk
