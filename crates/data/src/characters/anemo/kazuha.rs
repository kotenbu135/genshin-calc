use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Garyuu Bladework -- Physical --

const KAZUHA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.44978, 0.48639, 0.523, 0.5753, 0.61191, 0.65375, 0.71128, 0.76881, 0.82634, 0.8891,
        0.961013, 1.045582, 1.130151, 1.21472, 1.306977,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KAZUHA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.45236, 0.48918, 0.526, 0.5786, 0.61542, 0.6575, 0.71536, 0.77322, 0.83108, 0.8942,
        0.966525, 1.051579, 1.136633, 1.221688, 1.314474,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KAZUHA_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.258, 0.279, 0.3, 0.33, 0.351, 0.375, 0.408, 0.441, 0.474, 0.51, 0.55125, 0.59976,
        0.64827, 0.69678, 0.7497,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KAZUHA_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3096, 0.3348, 0.36, 0.396, 0.4212, 0.45, 0.4896, 0.5292, 0.5688, 0.612, 0.6615, 0.719712,
        0.777924, 0.836136, 0.89964,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KAZUHA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.60716, 0.65658, 0.706, 0.7766, 0.82602, 0.8825, 0.96016, 1.03782, 1.11548, 1.2002,
        1.297275, 1.411435, 1.525595, 1.639756, 1.764294,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KAZUHA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ(x3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2537, 0.27435, 0.295, 0.3245, 0.34515, 0.36875, 0.4012, 0.43365, 0.4661, 0.5015,
        0.542063, 0.589764, 0.637465, 0.685167, 0.737205,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const KAZUHA_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.43, 0.465, 0.5, 0.55, 0.585, 0.625, 0.68, 0.735, 0.79, 0.85, 0.91875, 0.9996, 1.08045,
        1.1613, 1.2495,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KAZUHA_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.74648, 0.80724, 0.868, 0.9548, 1.01556, 1.085, 1.18048, 1.27596, 1.37144, 1.4756,
        1.59495, 1.735306, 1.875661, 2.016017, 2.169132,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const KAZUHA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.818335, 0.884943, 0.951552, 1.046707, 1.113316, 1.18944, 1.294111, 1.398781, 1.503452,
        1.617638, 1.731825, 1.846011, 1.960197, 2.074383, 2.18857,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KAZUHA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.636323, 1.769512, 1.902701, 2.092971, 2.22616, 2.378376, 2.587673, 2.79697, 3.006267,
        3.234591, 3.462915, 3.69124, 3.919564, 4.147888, 4.376212,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KAZUHA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        2.043855, 2.210216, 2.376576, 2.614234, 2.780594, 2.97072, 3.232143, 3.493567, 3.75499,
        4.040179, 4.325368, 4.610557, 4.895747, 5.180936, 5.466125,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: Chihayaburu -- Anemo --

const KAZUHA_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "1回押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.92, 2.064, 2.208, 2.4, 2.544, 2.688, 2.88, 3.072, 3.264, 3.456, 3.648, 3.84, 4.08, 4.32,
        4.56,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KAZUHA_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.608, 2.8036, 2.9992, 3.26, 3.4556, 3.6512, 3.912, 4.1728, 4.4336, 4.6944, 4.9552, 5.216,
        5.542, 5.868, 6.194,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: Kazuha Slash -- Anemo --

const KAZUHA_BURST_SLASH: TalentScaling = TalentScaling {
    name: "斬撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.624, 2.8208, 3.0176, 3.28, 3.4768, 3.6736, 3.936, 4.1984, 4.4608, 4.7232, 4.9856, 5.248,
        5.576, 5.904, 6.232,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KAZUHA_BURST_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.2, 1.29, 1.38, 1.5, 1.59, 1.68, 1.8, 1.92, 2.04, 2.16, 2.28, 2.4, 2.55, 2.7, 2.85,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KAZUHA_BURST_ELEM: TalentScaling = TalentScaling {
    name: "付加元素ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.36, 0.387, 0.414, 0.45, 0.477, 0.504, 0.54, 0.576, 0.612, 0.648, 0.684, 0.72, 0.765,
        0.81, 0.855,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const KAZUHA: CharacterData = CharacterData {
    id: "kazuha",
    name: "Kazuha",
    element: Element::Anemo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [
        1039.00, 2695.00, 3586.00, 5366.00, 5999.00, 6902.00, 7747.00, 8659.00, 9292.00, 10213.00,
        10846.00, 11777.00, 12410.00, 13348.00, 13348.00, 13881.92, // Lv95/Lv95+/Lv100
        13881.92, // Lv95/Lv95+/Lv100
        14415.84, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        23.09, 59.89, 79.69, 119.24, 133.30, 153.37, 172.12, 192.39, 206.46, 226.91, 240.98,
        261.67, 275.73, 296.58, 296.58, 308.44, // Lv95/Lv95+/Lv100
        308.44, // Lv95/Lv95+/Lv100
        320.31, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        62.82, 162.96, 216.82, 324.44, 362.71, 417.30, 468.33, 523.49, 561.76, 617.42, 655.69,
        711.98, 750.25, 806.98, 806.98, 839.26, // Lv95/Lv95+/Lv100
        839.26, // Lv95/Lv95+/Lv100
        871.54, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalMastery(115.2),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "我流剣術",
            hits: &[
                KAZUHA_NORMAL_1,
                KAZUHA_NORMAL_2,
                KAZUHA_NORMAL_3A,
                KAZUHA_NORMAL_3B,
                KAZUHA_NORMAL_4,
                KAZUHA_NORMAL_5,
            ],
            charged: &[KAZUHA_CHARGED_1, KAZUHA_CHARGED_2],
            plunging: &[KAZUHA_PLUNGE, KAZUHA_PLUNGE_LOW, KAZUHA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "千早振る",
            scalings: &[KAZUHA_SKILL_PRESS, KAZUHA_SKILL_HOLD],
        },
        elemental_burst: TalentData {
            name: "万葉の一刀",
            scalings: &[KAZUHA_BURST_SLASH, KAZUHA_BURST_DOT, KAZUHA_BURST_ELEM],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
    scaling_modifiers: &[],
};

// =============================================================================
// Lan Yan — 4★ Anemo Catalyst (Liyue)
// Source: genshin-db-api
// Normal Attack: Black Pheasant Strides on Water (黒雉水上歩)
// Elemental Skill: Swallow-Wisp Pinion Dance (呑霊羽舞)
// Elemental Burst: Lustrous Moonrise (朗月昇る)
