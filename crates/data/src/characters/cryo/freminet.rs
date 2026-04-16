use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Flowing Eddies -- Physical --

const FREMINET_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8424, 0.9109, 0.9795, 1.0775, 1.1460, 1.2244, 1.3321, 1.4399, 1.5476, 1.6652, 1.7827,
        1.9002, 2.0178, 2.1353, 2.2529,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const FREMINET_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8068, 0.8724, 0.9381, 1.0319, 1.0976, 1.1726, 1.2758, 1.3790, 1.4822, 1.5948, 1.7073,
        1.8199, 1.9325, 2.0450, 2.1576,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const FREMINET_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0190, 1.1020, 1.1849, 1.3034, 1.3864, 1.4812, 1.6115, 1.7418, 1.8722, 2.0144, 2.1566,
        2.2988, 2.4410, 2.5831, 2.7253,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const FREMINET_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2380, 1.3388, 1.4396, 1.5835, 1.6843, 1.7995, 1.9578, 2.1162, 2.2746, 2.4473, 2.6201,
        2.7928, 2.9656, 3.1383, 3.3111,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const FREMINET_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6252, 0.6761, 0.7270, 0.7997, 0.8506, 0.9088, 0.9887, 1.0687, 1.1487, 1.2359, 1.3231,
        1.4104, 1.4976, 1.5849, 1.6721,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const FREMINET_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1309, 1.2230, 1.3150, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933,
        2.5511, 2.7089, 2.8667, 3.0245,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const FREMINET_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const FREMINET_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const FREMINET_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9460, 3.1842, 3.4225, 3.6825, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: Pressurized Floe -- Cryo --

const FREMINET_SKILL_THRUST: TalentScaling = TalentScaling {
    name: "上突きダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.8304, 0.8927, 0.9550, 1.0380, 1.1003, 1.1626, 1.2456, 1.3286, 1.4117, 1.4947, 1.5778,
        1.6608, 1.7646, 1.8684, 1.9722,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const FREMINET_SKILL_LEVEL4: TalentScaling = TalentScaling {
    name: "Lv4粉砕圧力ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        2.4344, 2.6170, 2.7996, 3.0430, 3.2256, 3.4082, 3.6516, 3.8950, 4.1385, 4.3819, 4.6254,
        4.8688, 5.1731, 5.4774, 5.7817,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: Shadowhunter's Ambush -- Cryo --

const FREMINET_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        3.1840, 3.4228, 3.6616, 3.9800, 4.2188, 4.4576, 4.7760, 5.0944, 5.4128, 5.7312, 6.0496,
        6.3680, 6.7660, 7.1640, 7.5620,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const FREMINET: CharacterData = CharacterData {
    id: "freminet",
    name: "Freminet",
    element: Element::Cryo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Fontaine,
    base_hp: [
        1012.00, 2600.00, 3356.00, 5027.00, 5564.00, 6400.00, 7117.00, 7953.00, 8490.00, 9325.00,
        9862.00, 10698.00, 11235.00, 12071.00, 12071.00, 12553.84, // Lv95/Lv95+/Lv100
        12553.84, // Lv95/Lv95+/Lv100
        13036.68, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        21.37, 54.90, 70.86, 106.14, 117.49, 135.14, 150.26, 167.91, 179.26, 196.89, 208.23,
        225.88, 237.23, 254.88, 254.88, 265.08, // Lv95/Lv95+/Lv100
        265.08, // Lv95/Lv95+/Lv100
        275.27, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        59.40, 152.60, 196.97, 295.04, 326.57, 375.63, 417.67, 466.74, 498.27, 547.27, 578.80,
        627.86, 659.39, 708.46, 708.46, 736.80, // Lv95/Lv95+/Lv100
        736.80, // Lv95/Lv95+/Lv100
        765.14, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "流れる渦",
            hits: &[
                FREMINET_NORMAL_1,
                FREMINET_NORMAL_2,
                FREMINET_NORMAL_3,
                FREMINET_NORMAL_4,
            ],
            charged: &[FREMINET_CHARGED_SPINNING, FREMINET_CHARGED_FINAL],
            plunging: &[FREMINET_PLUNGE, FREMINET_PLUNGE_LOW, FREMINET_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "浮氷の圧力",
            scalings: &[FREMINET_SKILL_THRUST, FREMINET_SKILL_LEVEL4],
        },
        elemental_burst: TalentData {
            name: "影の狩人の待ち伏せ",
            scalings: &[FREMINET_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3NormalC5Skill,
    passive_scalings: &[],
    scaling_modifiers: &[],
};

// =============================================================================
// Ganyu
