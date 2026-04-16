use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Iansan
// =============================================================================

// -- Normal Attack: 鋲打ちスパイク (Weighted Spike) -- Physical --

const IANSAN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4698, 0.5080, 0.5462, 0.6009, 0.6391, 0.6828, 0.7429, 0.8030, 0.8630, 0.9286, 0.9941,
        1.0597, 1.1252, 1.1908, 1.2563,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const IANSAN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4276, 0.4625, 0.4973, 0.5470, 0.5818, 0.6216, 0.6763, 0.7310, 0.7857, 0.8453, 0.9050,
        0.9647, 1.0244, 1.0841, 1.1437,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const IANSAN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6439, 0.6963, 0.7487, 0.8236, 0.8760, 0.9359, 1.0182, 1.1006, 1.1829, 1.2728, 1.3626,
        1.4525, 1.5423, 1.6322, 1.7220,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const IANSAN_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0028, 1.0844, 1.1660, 1.2826, 1.3642, 1.4575, 1.5858, 1.7140, 1.8423, 1.9822, 2.1221,
        2.2620, 2.4020, 2.5419, 2.6818,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const IANSAN_SWIFT_STORMFLIGHT: TalentScaling = TalentScaling {
    name: "疾風翔撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8419, 0.9105, 0.9790, 1.0769, 1.1454, 1.2238, 1.3314, 1.4391, 1.5468, 1.6643, 1.7818,
        1.8993, 2.0167, 2.1342, 2.2517,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const IANSAN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const IANSAN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const IANSAN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 雷奔撃 (Thunderbolt Rush) -- Electro --

const IANSAN_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        2.8640, 3.0788, 3.2936, 3.5800, 3.7948, 4.0096, 4.2960, 4.5824, 4.8688, 5.1552, 5.4416,
        5.7280, 6.0860, 6.4440, 6.8020,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 力の三大原則 (The Three Principles of Power) -- Electro --

const IANSAN_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        4.3040, 4.6268, 4.9496, 5.3800, 5.7028, 6.0256, 6.4560, 6.8864, 7.3168, 7.7472, 8.1776,
        8.6080, 9.1460, 9.6840, 10.2220,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const IANSAN: CharacterData = CharacterData {
    id: "iansan",
    name: "Iansan",
    element: Element::Electro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Natlan,
    base_hp: [
        894.00, 2296.00, 2963.00, 4438.00, 4913.00, 5651.00, 6283.00, 7021.00, 7495.00, 8233.00,
        8707.00, 9445.00, 9919.00, 10657.00, 10657.00, 11083.28, // Lv95/Lv95+/Lv100
        11083.28, // Lv95/Lv95+/Lv100
        11509.56, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        21.55, 55.36, 71.45, 107.03, 118.47, 136.26, 151.52, 169.31, 180.75, 198.53, 209.97,
        227.76, 239.20, 257.00, 257.00, 267.28, // Lv95/Lv95+/Lv100
        267.28, // Lv95/Lv95+/Lv100
        277.56, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        53.51, 137.46, 177.43, 265.76, 294.16, 338.36, 376.23, 420.42, 448.82, 492.97, 521.37,
        565.56, 593.96, 638.16, 638.16, 663.69, // Lv95/Lv95+/Lv100
        663.69, // Lv95/Lv95+/Lv100
        689.21, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "鋲打ちスパイク",
            hits: &[IANSAN_NORMAL_1, IANSAN_NORMAL_2, IANSAN_NORMAL_3],
            charged: &[IANSAN_CHARGED, IANSAN_SWIFT_STORMFLIGHT],
            plunging: &[IANSAN_PLUNGE, IANSAN_PLUNGE_LOW, IANSAN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "雷奔撃",
            scalings: &[IANSAN_SKILL],
        },
        elemental_burst: TalentData {
            name: "力の三大原則",
            scalings: &[IANSAN_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
    scaling_modifiers: &[],
};
