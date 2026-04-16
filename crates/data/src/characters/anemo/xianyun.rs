use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Word of Wind and Flower -- All Anemo (Catalyst) --

const XIANYUN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.403, 0.4333, 0.4635, 0.5038, 0.534, 0.5642, 0.6045, 0.6448, 0.6851, 0.7254, 0.7657,
        0.806, 0.8564, 0.9068, 0.9572,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XIANYUN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.3886, 0.4177, 0.4468, 0.4857, 0.5148, 0.544, 0.5828, 0.6217, 0.6605, 0.6994, 0.7382,
        0.7771, 0.8257, 0.8742, 0.9228,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XIANYUN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.4888, 0.5254, 0.5621, 0.611, 0.6476, 0.6843, 0.7332, 0.782, 0.8309, 0.8798, 0.9287,
        0.9776, 1.0386, 1.0997, 1.1608,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XIANYUN_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.6492, 0.6979, 0.7465, 0.8115, 0.8601, 0.9088, 0.9738, 1.0387, 1.1036, 1.1685, 1.2334,
        1.2983, 1.3795, 1.4606, 1.5418,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Anemo (Catalyst) --

const XIANYUN_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.2312, 1.3235, 1.4159, 1.539, 1.6313, 1.7237, 1.8468, 1.9699, 2.093, 2.2162, 2.3393,
        2.4624, 2.6163, 2.7702, 2.9241,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Anemo (Catalyst) --

const XIANYUN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XIANYUN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XIANYUN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: White Clouds at Dawn -- Anemo --

const XIANYUN_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.248, 0.2666, 0.2852, 0.31, 0.3286, 0.3472, 0.372, 0.3968, 0.4216, 0.4464, 0.4712, 0.496,
        0.527, 0.558, 0.589,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XIANYUN_SKILL_CLOUD_1: TalentScaling = TalentScaling {
    name: "流雲波1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.16, 1.247, 1.334, 1.45, 1.537, 1.624, 1.74, 1.856, 1.972, 2.088, 2.204, 2.32, 2.465,
        2.61, 2.755,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XIANYUN_SKILL_CLOUD_2: TalentScaling = TalentScaling {
    name: "流雲波2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.48, 1.591, 1.702, 1.85, 1.961, 2.072, 2.22, 2.368, 2.516, 2.664, 2.812, 2.96, 3.145,
        3.33, 3.515,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XIANYUN_SKILL_CLOUD_3: TalentScaling = TalentScaling {
    name: "流雲波3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        3.376, 3.6292, 3.8824, 4.22, 4.4732, 4.7264, 5.064, 5.4016, 5.7392, 6.0768, 6.4144, 6.752,
        7.174, 7.596, 8.018,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: Stars Gather at Dusk -- Anemo --

const XIANYUN_BURST: TalentScaling = TalentScaling {
    name: "初回ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.08, 1.161, 1.242, 1.35, 1.431, 1.512, 1.62, 1.728, 1.836, 1.944, 2.052, 2.16, 2.295,
        2.43, 2.565,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XIANYUN_BURST_STARWICKER: TalentScaling = TalentScaling {
    name: "星枝ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.392, 0.4214, 0.4508, 0.49, 0.5194, 0.5488, 0.588, 0.6272, 0.6664, 0.7056, 0.7448, 0.784,
        0.8330, 0.882, 0.931,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const XIANYUN: CharacterData = CharacterData {
    id: "xianyun",
    name: "Xianyun",
    element: Element::Anemo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [
        810.00, 2102.00, 2797.00, 4185.00, 4678.00, 5383.00, 6041.00, 6752.00, 7246.00, 7964.00,
        8458.00, 9184.00, 9677.00, 10409.00, 10409.00, 10825.36, // Lv95/Lv95+/Lv100
        10825.36, // Lv95/Lv95+/Lv100
        11241.72, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        26.07, 67.62, 89.97, 134.62, 150.50, 173.16, 194.33, 217.22, 233.10, 256.19, 272.07,
        295.43, 311.31, 334.85, 334.85, 348.24, // Lv95/Lv95+/Lv100
        348.24, // Lv95/Lv95+/Lv100
        361.64, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        44.57, 115.62, 153.84, 230.20, 257.35, 296.09, 332.29, 371.43, 398.58, 438.08, 465.23,
        505.17, 532.32, 572.57, 572.57, 595.47, // Lv95/Lv95+/Lv100
        595.47, // Lv95/Lv95+/Lv100
        618.38, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "風花の詞",
            hits: &[
                XIANYUN_NORMAL_1,
                XIANYUN_NORMAL_2,
                XIANYUN_NORMAL_3,
                XIANYUN_NORMAL_4,
            ],
            charged: &[XIANYUN_CHARGED],
            plunging: &[XIANYUN_PLUNGE, XIANYUN_PLUNGE_LOW, XIANYUN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "白雲の暁",
            scalings: &[
                XIANYUN_SKILL,
                XIANYUN_SKILL_CLOUD_1,
                XIANYUN_SKILL_CLOUD_2,
                XIANYUN_SKILL_CLOUD_3,
            ],
        },
        elemental_burst: TalentData {
            name: "群星の夕",
            scalings: &[XIANYUN_BURST, XIANYUN_BURST_STARWICKER],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Xiao — 5★ Anemo Polearm (Liyue)
// Source: genshin-db-api
// Normal Attack: Whirlwind Thrust (旋風キック)
// Elemental Skill: Lemniscatic Wind Cycling (風輪両立)
// Elemental Burst: Bane of All Evil (靖妖儺舞)
