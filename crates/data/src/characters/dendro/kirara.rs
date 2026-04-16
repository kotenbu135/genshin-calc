use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// --- Normal Attack: Boxcutter (physical) ---

const KIRARA_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.47902, 0.51801, 0.557, 0.6127, 0.65169, 0.69625, 0.75752, 0.81879, 0.88006, 0.9469,
        1.01374, 1.08058, 1.14742, 1.21426, 1.2811,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KIRARA_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.46354, 0.50127, 0.539, 0.5929, 0.63063, 0.67375, 0.73304, 0.79233, 0.85162, 0.9163,
        0.98098, 1.04566, 1.11034, 1.17502, 1.2397,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KIRARA_NA_HIT3A: TalentScaling = TalentScaling {
    name: "3段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.254216, 0.274908, 0.2956, 0.32516, 0.345852, 0.3695, 0.402016, 0.434532, 0.467048,
        0.50252, 0.537992, 0.573464, 0.608936, 0.644408, 0.67988,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KIRARA_NA_HIT3B: TalentScaling = TalentScaling {
    name: "3段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.381324, 0.412362, 0.4434, 0.48774, 0.518778, 0.55425, 0.603024, 0.651798, 0.700572,
        0.75378, 0.806988, 0.860196, 0.913404, 0.966612, 1.01982,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KIRARA_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.73272, 0.79236, 0.852, 0.9372, 0.99684, 1.065, 1.15872, 1.25244, 1.34616, 1.4484,
        1.55064, 1.65288, 1.75512, 1.85736, 1.9596,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KIRARA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ(x3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.223772, 0.241986, 0.2602, 0.28622, 0.304434, 0.32525, 0.353872, 0.382494, 0.411116,
        0.44234, 0.473564, 0.504788, 0.536012, 0.567236, 0.59846,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KIRARA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572,
        1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KIRARA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646,
        2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KIRARA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586,
        3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Elemental Skill: Meow-teor Kick ---

const KIRARA_SKILL_KICK: TalentScaling = TalentScaling {
    name: "飛び蹴りダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.04, 1.118, 1.196, 1.3, 1.378, 1.456, 1.56, 1.664, 1.768, 1.872, 1.976, 2.08, 2.21, 2.34,
        2.47,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KIRARA_SKILL_PARCEL_HIT: TalentScaling = TalentScaling {
    name: "ネコ箱急便突進ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.3360, 0.3612, 0.3864, 0.4200, 0.4452, 0.4704, 0.5040, 0.5376, 0.5712, 0.6048, 0.6384,
        0.6720, 0.7140, 0.7560, 0.7980,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KIRARA_SKILL_FLIPCLAW: TalentScaling = TalentScaling {
    name: "くるりん爪撃のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.4400, 1.5480, 1.6560, 1.8000, 1.9080, 2.0160, 2.1600, 2.3040, 2.4480, 2.5920, 2.7360,
        2.8800, 3.0600, 3.2400, 3.4200,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Elemental Burst: Secret Art: Surprise Dispatch ---

const KIRARA_BURST_DMG: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        5.7024, 6.13008, 6.55776, 7.128, 7.55568, 7.98336, 8.5536, 9.12384, 9.69408, 10.26432,
        10.83456, 11.4048, 12.1176, 12.8304, 13.5432,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KIRARA_BURST_EXPLOSION: TalentScaling = TalentScaling {
    name: "猫草豪雨ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.3564, 0.38313, 0.40986, 0.4455, 0.47223, 0.49896, 0.5346, 0.57024, 0.60588, 0.64152,
        0.67716, 0.7128, 0.75735, 0.8019, 0.84645,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Kirara aggregation ---

static KIRARA_NA_HITS: &[TalentScaling] = &[
    KIRARA_NA_HIT1,
    KIRARA_NA_HIT2,
    KIRARA_NA_HIT3A,
    KIRARA_NA_HIT3B,
    KIRARA_NA_HIT4,
];
static KIRARA_CHARGED_ATTACKS: &[TalentScaling] = &[KIRARA_CHARGED];
static KIRARA_PLUNGING: &[TalentScaling] = &[KIRARA_PLUNGE, KIRARA_PLUNGE_LOW, KIRARA_PLUNGE_HIGH];
static KIRARA_SKILL_SCALINGS: &[TalentScaling] = &[
    KIRARA_SKILL_KICK,
    KIRARA_SKILL_PARCEL_HIT,
    KIRARA_SKILL_FLIPCLAW,
];
static KIRARA_BURST_SCALINGS: &[TalentScaling] = &[KIRARA_BURST_DMG, KIRARA_BURST_EXPLOSION];

pub const KIRARA: CharacterData = CharacterData {
    id: "kirara",
    name: "Kirara",
    element: Element::Dendro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Inazuma,
    base_hp: [
        1021.00, 2623.00, 3386.00, 5072.00, 5614.00, 6458.00, 7181.00, 8024.00, 8566.00, 9409.00,
        9951.00, 10794.00, 11336.00, 12180.00, 12180.00, 12601.00, // Lv95/Lv95+/Lv100
        12601.00, // Lv95/Lv95+/Lv100
        13022.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        18.70, 48.04, 62.01, 92.88, 102.80, 118.25, 131.48, 146.93, 156.85, 172.28, 182.20, 197.65,
        207.57, 223.02, 223.02, 251.47, // Lv95/Lv95+/Lv100
        251.47, // Lv95/Lv95+/Lv100
        279.92, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        45.78, 117.61, 151.81, 227.39, 251.69, 289.51, 321.91, 359.72, 384.02, 421.79, 446.09,
        483.90, 508.20, 546.02, 546.02, 564.91, // Lv95/Lv95+/Lv100
        564.91, // Lv95/Lv95+/Lv100
        583.79, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "段板紙・切り裂き術",
            hits: KIRARA_NA_HITS,
            charged: KIRARA_CHARGED_ATTACKS,
            plunging: KIRARA_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "にゃんにゃん町飛脚",
            scalings: KIRARA_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "秘法·サプライズ特別配送",
            scalings: KIRARA_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
