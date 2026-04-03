use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Whirlwind Thrust -- Physical --

const XIAO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.27544, 0.29422, 0.313, 0.33804, 0.35682, 0.37873, 0.4069, 0.43507, 0.46324, 0.49141,
        0.51958, 0.54775, 0.57592, 0.60409, 0.63226,
    ],
};

const XIAO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.56936, 0.60818, 0.647, 0.69876, 0.73758, 0.78287, 0.8411, 0.89933, 0.95756, 1.01579,
        1.07402, 1.13225, 1.19048, 1.24871, 1.30694,
    ],
};

const XIAO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.68552, 0.73226, 0.779, 0.84132, 0.88806, 0.94259, 1.0127, 1.08281, 1.15292, 1.22303,
        1.29314, 1.36325, 1.43336, 1.50347, 1.57358,
    ],
};

const XIAO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.37664, 0.40232, 0.428, 0.46224, 0.48792, 0.51788, 0.5564, 0.59492, 0.63344, 0.67196,
        0.71048, 0.749, 0.78752, 0.82604, 0.86456,
    ],
};

const XIAO_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.71544, 0.76422, 0.813, 0.87804, 0.92682, 0.98373, 1.0569, 1.13007, 1.20324, 1.27641,
        1.34958, 1.42275, 1.49592, 1.56909, 1.64226,
    ],
};

const XIAO_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.95832, 1.02366, 1.089, 1.17612, 1.24146, 1.31769, 1.4157, 1.51371, 1.61172, 1.70973,
        1.80774, 1.90575, 2.00376, 2.10177, 2.19978,
    ],
};

// -- Charged Attack -- Physical --

const XIAO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.21088, 1.29344, 1.376, 1.48608, 1.56864, 1.66496, 1.7888, 1.91264, 2.03648, 2.16032,
        2.28416, 2.408, 2.53184, 2.65568, 2.77952,
    ],
};

// -- Plunging Attack -- Physical --

const XIAO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.818335, 0.884943, 0.951552, 1.046707, 1.113316, 1.18944, 1.294111, 1.398781, 1.503452,
        1.617638, 1.731825, 1.846011, 1.960197, 2.074383, 2.18857,
    ],
};

const XIAO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.636323, 1.769512, 1.902701, 2.092971, 2.22616, 2.378376, 2.587673, 2.79697, 3.006267,
        3.234591, 3.462915, 3.69124, 3.919564, 4.147888, 4.376212,
    ],
};

const XIAO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        2.043855, 2.210216, 2.376576, 2.614234, 2.780594, 2.97072, 3.232143, 3.493567, 3.75499,
        4.040179, 4.325368, 4.610557, 4.895747, 5.180936, 5.466125,
    ],
};

// -- Elemental Skill: Lemniscatic Wind Cycling -- Anemo --

const XIAO_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.528, 2.7176, 2.9072, 3.16, 3.3496, 3.5392, 3.792, 4.0448, 4.2976, 4.5504, 4.8032, 5.056,
        5.372, 5.688, 6.004,
    ],
};

// -- Elemental Burst: Bane of All Evil --
// Xiao's burst provides a DMG Bonus buff, not direct damage scalings.
// No damage talent scalings to include.

pub const XIAO: CharacterData = CharacterData {
    id: "xiao",
    name: "Xiao",
    element: Element::Anemo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [
        991.00, 2572.00, 3422.00, 5120.00, 5724.00, 6586.00, 7391.00, 8262.00, 8866.00, 9744.00,
        10348.00, 11236.00, 11840.00, 12736.00, 12736.00, 13245.44, // Lv95/Lv95+/Lv100
        13245.44, // Lv95/Lv95+/Lv100
        13754.88, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        27.19, 70.52, 93.83, 140.39, 156.95, 180.58, 202.66, 226.53, 243.09, 267.17, 283.73,
        308.09, 324.65, 349.20, 349.20, 363.17, // Lv95/Lv95+/Lv100
        363.17, // Lv95/Lv95+/Lv100
        377.14, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        62.22, 161.41, 214.76, 321.35, 359.26, 413.33, 463.87, 518.50, 556.41, 611.54, 649.45,
        705.20, 743.11, 799.30, 799.30, 831.27, // Lv95/Lv95+/Lv100
        831.27, // Lv95/Lv95+/Lv100
        863.24, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "旋風槍術",
            hits: &[
                XIAO_NORMAL_1,
                XIAO_NORMAL_2,
                XIAO_NORMAL_3,
                XIAO_NORMAL_4,
                XIAO_NORMAL_5,
                XIAO_NORMAL_6,
            ],
            charged: &[XIAO_CHARGED],
            plunging: &[XIAO_PLUNGE, XIAO_PLUNGE_LOW, XIAO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "風輪両立",
            scalings: &[XIAO_SKILL],
        },
        elemental_burst: TalentData {
            name: "靖妖儺舞",
            scalings: &[],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Jahoda — 4★ Anemo Bow (Nod-Krai / v6.2)
// Placeholder values — update from Genshin Impact Wiki when available
