use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Havoc: Sunder -- Physical --

const SKIRK_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5452, 0.5896, 0.6340, 0.6974, 0.7418, 0.7925, 0.8622, 0.9320, 1.0017, 1.0778, 1.1539,
        1.2300, 1.3060, 1.3821, 1.4582,
    ],
    dynamic_bonus: None,
};

const SKIRK_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4979, 0.5385, 0.5790, 0.6369, 0.6774, 0.7238, 0.7874, 0.8511, 0.9148, 0.9843, 1.0538,
        1.1233, 1.1927, 1.2622, 1.3317,
    ],
    dynamic_bonus: None,
};

const SKIRK_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3242, 0.3506, 0.3770, 0.4147, 0.4411, 0.4713, 0.5127, 0.5542, 0.5957, 0.6409, 0.6861,
        0.7314, 0.7766, 0.8219, 0.8671,
    ],
    dynamic_bonus: None,
};

const SKIRK_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6080, 0.6575, 0.7070, 0.7777, 0.8272, 0.8838, 0.9615, 1.0393, 1.1171, 1.2019, 1.2867,
        1.3716, 1.4564, 1.5413, 1.6261,
    ],
    dynamic_bonus: None,
};

const SKIRK_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8290, 0.8965, 0.9640, 1.0604, 1.1279, 1.2050, 1.3110, 1.4171, 1.5231, 1.6388, 1.7545,
        1.8702, 1.9858, 2.1015, 2.2172,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Physical --

const SKIRK_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6682, 0.7226, 0.7770, 0.8547, 0.9091, 0.9713, 1.0567, 1.1422, 1.2277, 1.3209, 1.4141,
        1.5074, 1.6006, 1.6939, 1.7871,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const SKIRK_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
};

const SKIRK_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
};

const SKIRK_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: Havoc: Warp -- Cryo --

const SKIRK_SKILL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.3282, 1.4364, 1.5445, 1.6989, 1.8070, 1.9306, 2.1005, 2.2704, 2.4403, 2.6256, 2.8109,
        2.9963, 3.1816, 3.3669, 3.5523,
    ],
    dynamic_bonus: None,
};

const SKIRK_SKILL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.1980, 1.2955, 1.3930, 1.5323, 1.6298, 1.7413, 1.8945, 2.0477, 2.2010, 2.3681, 2.5353,
        2.7025, 2.8696, 3.0368, 3.2039,
    ],
    dynamic_bonus: None,
};

const SKIRK_SKILL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.7572, 0.8189, 0.8805, 0.9686, 1.0302, 1.1006, 1.1975, 1.2943, 1.3912, 1.4969, 1.6025,
        1.7082, 1.8138, 1.9195, 2.0252,
    ],
    dynamic_bonus: None,
};

const SKIRK_SKILL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.8054, 0.8709, 0.9365, 1.0302, 1.0957, 1.1706, 1.2736, 1.3767, 1.4797, 1.5921, 1.7044,
        1.8168, 1.9292, 2.0416, 2.1540,
    ],
    dynamic_bonus: None,
};

const SKIRK_SKILL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.9662, 2.1263, 2.2863, 2.5150, 2.6750, 2.8579, 3.1094, 3.3609, 3.6124, 3.8868, 4.1611,
        4.4548, 4.7098, 4.9842, 5.2586,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: Havoc: Ruin -- Cryo --

const SKIRK_BURST_SLASH: TalentScaling = TalentScaling {
    name: "スラッシュダメージ (×5)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2276, 1.3197, 1.4117, 1.5345, 1.6266, 1.7186, 1.8414, 1.9642, 2.0869, 2.2097, 2.3324,
        2.4552, 2.6087, 2.7621, 2.9156,
    ],
    dynamic_bonus: None,
};

const SKIRK_BURST_FINAL: TalentScaling = TalentScaling {
    name: "最終スラッシュダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        2.0460, 2.1995, 2.3529, 2.5575, 2.7110, 2.8644, 3.0690, 3.2736, 3.4782, 3.6828, 3.8874,
        4.0920, 4.3478, 4.6035, 4.8593,
    ],
    dynamic_bonus: None,
};

pub const SKIRK: CharacterData = CharacterData {
    id: "skirk",
    name: "Skirk",
    element: Element::Cryo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Snezhnaya,
    base_hp: [
        967.00, 2508.00, 3336.00, 4992.00, 5581.00, 6421.00, 7206.00, 8055.00, 8644.00, 9501.00,
        10089.00, 10956.00, 11544.00, 12417.00, 12417.00, 12913.68, // Lv95/Lv95+/Lv100
        12913.68, // Lv95/Lv95+/Lv100
        13410.36, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        27.93, 72.45, 96.40, 144.24, 161.25, 185.53, 208.21, 232.73, 249.75, 274.49, 291.51,
        316.53, 333.55, 358.77, 358.77, 373.12, // Lv95/Lv95+/Lv100
        373.12, // Lv95/Lv95+/Lv100
        387.47, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        62.76, 162.80, 216.62, 324.13, 362.36, 416.90, 467.89, 522.99, 561.23, 616.83, 655.07,
        711.30, 749.54, 806.21, 806.21, 838.46, // Lv95/Lv95+/Lv100
        838.46, // Lv95/Lv95+/Lv100
        870.71, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "大破・裂壊",
            hits: &[
                SKIRK_NORMAL_1,
                SKIRK_NORMAL_2,
                SKIRK_NORMAL_3,
                SKIRK_NORMAL_4,
                SKIRK_NORMAL_5,
            ],
            charged: &[SKIRK_CHARGED],
            plunging: &[SKIRK_PLUNGE, SKIRK_PLUNGE_LOW, SKIRK_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "大破・歪曲",
            scalings: &[
                SKIRK_SKILL_1,
                SKIRK_SKILL_2,
                SKIRK_SKILL_3,
                SKIRK_SKILL_4,
                SKIRK_SKILL_5,
            ],
        },
        elemental_burst: TalentData {
            name: "大破・崩壊",
            scalings: &[SKIRK_BURST_SLASH, SKIRK_BURST_FINAL],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Wriothesley
