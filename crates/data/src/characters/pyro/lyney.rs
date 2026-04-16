use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Lyney
// =============================================================================

// -- Normal Attack: カードフォース・トランスロケーション (Card Force Translocation) -- Physical --

const LYNEY_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3879, 0.4194, 0.4510, 0.4961, 0.5277, 0.5638, 0.6134, 0.6630, 0.7126, 0.7667, 0.8208,
        0.8749, 0.9291, 0.9832, 1.0373,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LYNEY_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3801, 0.4111, 0.4420, 0.4862, 0.5171, 0.5525, 0.6011, 0.6497, 0.6984, 0.7514, 0.8044,
        0.8575, 0.9105, 0.9636, 1.0166,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LYNEY_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2726, 0.2948, 0.3170, 0.3487, 0.3709, 0.3963, 0.4311, 0.4660, 0.5009, 0.5389, 0.5769,
        0.6150, 0.6530, 0.6911, 0.7291,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LYNEY_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2726, 0.2948, 0.3170, 0.3487, 0.3709, 0.3963, 0.4311, 0.4660, 0.5009, 0.5389, 0.5769,
        0.6150, 0.6530, 0.6911, 0.7291,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LYNEY_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5693, 0.6157, 0.6620, 0.7282, 0.7745, 0.8275, 0.9003, 0.9731, 1.0460, 1.1254, 1.2048,
        1.2843, 1.3637, 1.4432, 1.5226,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Aimed Shot -- Pyro (charged) --

const LYNEY_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LYNEY_AIMED_CHARGE1: TalentScaling = TalentScaling {
    name: "1段チャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LYNEY_AIMED_PROP: TalentScaling = TalentScaling {
    name: "プロップアローダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.7280, 1.8576, 1.9872, 2.1600, 2.2896, 2.4192, 2.5920, 2.7648, 2.9376, 3.1104, 3.2832,
        3.4560, 3.6720, 3.8880, 4.1040,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LYNEY_AIMED_PYROTECHNIC: TalentScaling = TalentScaling {
    name: "パイロテクニックストライク",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        2.1200, 2.2790, 2.4380, 2.6500, 2.8090, 2.9680, 3.1800, 3.3920, 3.6040, 3.8160, 4.0280,
        4.2400, 4.5050, 4.7700, 5.0350,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const LYNEY_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LYNEY_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LYNEY_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 眩惑マジック (Bewildering Lights) -- Pyro --

const LYNEY_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.6720, 1.7974, 1.9228, 2.0900, 2.2154, 2.3408, 2.5080, 2.6752, 2.8424, 3.0096, 3.1768,
        3.3440, 3.5530, 3.7620, 3.9710,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LYNEY_SKILL_HAT: TalentScaling = TalentScaling {
    name: "ハットトリックダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5320, 0.5719, 0.6118, 0.6650, 0.7049, 0.7448, 0.7980, 0.8512, 0.9044, 0.9576, 1.0108,
        1.0640, 1.1305, 1.1970, 1.2635,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 大魔術・ミラクルパレード (Wondrous Trick: Miracle Parade) -- Pyro --

const LYNEY_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.5400, 1.6555, 1.7710, 1.9250, 2.0405, 2.1560, 2.3100, 2.4640, 2.6180, 2.7720, 2.9260,
        3.0800, 3.2725, 3.4650, 3.6575,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LYNEY_BURST_EXPLOSION: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        4.1400, 4.4505, 4.7610, 5.1750, 5.4855, 5.7960, 6.2100, 6.6240, 7.0380, 7.4520, 7.8660,
        8.2800, 8.7975, 9.3150, 9.8325,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const LYNEY: CharacterData = CharacterData {
    id: "lyney",
    name: "Lyney",
    element: Element::Pyro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [
        858.00, 2226.00, 2961.00, 4431.00, 4954.00, 5699.00, 6396.00, 7150.00, 7672.00, 8432.00,
        8955.00, 9724.00, 10247.00, 11021.00, 11021.00, 11461.84, // Lv95/Lv95+/Lv100
        11461.84, // Lv95/Lv95+/Lv100
        11805.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        24.76, 64.24, 85.47, 127.89, 142.98, 164.50, 184.61, 206.36, 221.44, 243.38, 258.47,
        280.66, 295.74, 318.11, 318.11, 330.83, // Lv95/Lv95+/Lv100
        330.83, // Lv95/Lv95+/Lv100
        389.68, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        41.88, 108.64, 144.55, 216.29, 241.81, 278.20, 312.22, 348.99, 374.51, 411.61, 437.13,
        474.65, 500.17, 537.99, 537.99, 559.51, // Lv95/Lv95+/Lv100
        559.51, // Lv95/Lv95+/Lv100
        576.23, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "カードフォース・トランスロケーション",
            hits: &[
                LYNEY_NORMAL_1,
                LYNEY_NORMAL_2,
                LYNEY_NORMAL_3A,
                LYNEY_NORMAL_3B,
                LYNEY_NORMAL_4,
            ],
            charged: &[
                LYNEY_AIMED,
                LYNEY_AIMED_CHARGE1,
                LYNEY_AIMED_PROP,
                LYNEY_AIMED_PYROTECHNIC,
            ],
            plunging: &[LYNEY_PLUNGE, LYNEY_PLUNGE_LOW, LYNEY_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "眩惑マジック",
            scalings: &[LYNEY_SKILL, LYNEY_SKILL_HAT],
        },
        elemental_burst: TalentData {
            name: "大魔術・ミラクルパレード",
            scalings: &[LYNEY_BURST, LYNEY_BURST_EXPLOSION],
        },
    },
    constellation_pattern: ConstellationPattern::C3NormalC5Burst,
    passive_scalings: &[],
    scaling_modifiers: &[],
};
