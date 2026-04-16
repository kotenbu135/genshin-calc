use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// --- Normal Attack: Supplicant's Bowmanship (physical) ---

const COLLEI_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.43602, 0.47151, 0.507, 0.5577, 0.59319, 0.63375, 0.68952, 0.74529, 0.80106, 0.8619,
        0.92274, 0.98358, 1.04442, 1.10526, 1.1661,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLLEI_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.42656, 0.46128, 0.496, 0.5456, 0.58032, 0.62, 0.67456, 0.72912, 0.78368, 0.8432, 0.90272,
        0.96224, 1.02176, 1.08128, 1.1408,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLLEI_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.54094, 0.58497, 0.629, 0.6919, 0.73593, 0.78625, 0.85544, 0.92463, 0.99382, 1.0693,
        1.14478, 1.22026, 1.29574, 1.37122, 1.4467,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLLEI_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.68026, 0.73563, 0.791, 0.8701, 0.92547, 0.98875, 1.07576, 1.16277, 1.24978, 1.3447,
        1.43962, 1.53454, 1.62946, 1.72438, 1.8193,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Aimed Shot ---

const COLLEI_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894,
        1.0506, 1.1118, 1.173,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLLEI_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635,
        2.79, 2.945,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Plunging Attack (physical) ---

const COLLEI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLLEI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLLEI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Elemental Skill: Floral Brush ---

const COLLEI_SKILL_DMG: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.512, 1.6254, 1.7388, 1.89, 2.0034, 2.1168, 2.268, 2.4192, 2.5704, 2.7216, 2.8728, 3.024,
        3.213, 3.402, 3.591,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Elemental Burst: Trump-Card Kitty ---

const COLLEI_BURST_EXPLOSION: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        2.01824, 2.169608, 2.320976, 2.5228, 2.674168, 2.825536, 3.02736, 3.229184, 3.431008,
        3.632832, 3.834656, 4.03648, 4.28876, 4.54104, 4.79332,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLLEI_BURST_LEAP: TalentScaling = TalentScaling {
    name: "跳躍ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.43248, 0.464916, 0.497352, 0.5406, 0.573036, 0.605472, 0.64872, 0.691968, 0.735216,
        0.778464, 0.821712, 0.86496, 0.91902, 0.97308, 1.02714,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Collei aggregation ---

static COLLEI_NA_HITS: &[TalentScaling] = &[
    COLLEI_NA_HIT1,
    COLLEI_NA_HIT2,
    COLLEI_NA_HIT3,
    COLLEI_NA_HIT4,
];
static COLLEI_CHARGED_ATTACKS: &[TalentScaling] = &[COLLEI_AIMED, COLLEI_AIMED_FULL];
static COLLEI_PLUNGING: &[TalentScaling] = &[COLLEI_PLUNGE, COLLEI_PLUNGE_LOW, COLLEI_PLUNGE_HIGH];
static COLLEI_SKILL_SCALINGS: &[TalentScaling] = &[COLLEI_SKILL_DMG];
static COLLEI_BURST_SCALINGS: &[TalentScaling] = &[COLLEI_BURST_EXPLOSION, COLLEI_BURST_LEAP];

pub const COLLEI: CharacterData = CharacterData {
    id: "collei",
    name: "Collei",
    element: Element::Dendro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Sumeru,
    base_hp: [
        821.00, 2108.00, 2721.00, 4076.00, 4512.00, 5189.00, 5770.00, 6448.00, 6884.00, 7561.00,
        7996.00, 8674.00, 9110.00, 9787.00, 9787.00, 10178.48, // Lv95/Lv95+/Lv100
        10178.48, // Lv95/Lv95+/Lv100
        10569.96, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        16.74, 43.00, 55.51, 83.15, 92.03, 105.86, 117.71, 131.53, 140.42, 154.23, 163.11, 176.94,
        185.83, 199.65, 199.65, 207.64, // Lv95/Lv95+/Lv100
        207.64, // Lv95/Lv95+/Lv100
        215.62, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        50.36, 129.37, 166.99, 250.13, 276.86, 318.46, 354.10, 395.69, 422.42, 463.97, 490.70,
        532.29, 559.02, 600.62, 600.62, 624.64, // Lv95/Lv95+/Lv100
        624.64, // Lv95/Lv95+/Lv100
        648.67, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "祈りの射技",
            hits: COLLEI_NA_HITS,
            charged: COLLEI_CHARGED_ATTACKS,
            plunging: COLLEI_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "花触葉讃",
            scalings: COLLEI_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "ニャンコトレジャー",
            scalings: COLLEI_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
