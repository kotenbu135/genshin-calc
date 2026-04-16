use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// --- Normal Attack: Shadow-Hunting Spear: Modified (physical) ---

const EMILIE_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.485608, 0.525134, 0.56466, 0.621126, 0.660652, 0.705825, 0.767938, 0.83005, 0.892163,
        0.959922, 1.027681, 1.09544, 1.1632, 1.230959, 1.298718,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EMILIE_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.448954, 0.485497, 0.52204, 0.574244, 0.610787, 0.65255, 0.709974, 0.767399, 0.824823,
        0.887468, 0.950113, 1.012758, 1.075402, 1.138047, 1.200692,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EMILIE_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.593004, 0.641272, 0.68954, 0.758494, 0.806762, 0.861925, 0.937774, 1.013624, 1.089473,
        1.172218, 1.254963, 1.337708, 1.420452, 1.503197, 1.585942,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EMILIE_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.751029, 0.81216, 0.87329, 0.960619, 1.021749, 1.091613, 1.187674, 1.283736, 1.379798,
        1.484593, 1.589388, 1.694183, 1.798977, 1.903772, 2.008567,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EMILIE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.91332, 0.98766, 1.062, 1.1682, 1.24254, 1.3275, 1.44432, 1.56114, 1.67796, 1.8054,
        1.93284, 2.06028, 2.18772, 2.31516, 2.4426,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EMILIE_PLUNGE: TalentScaling = TalentScaling {
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

const EMILIE_PLUNGE_LOW: TalentScaling = TalentScaling {
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

const EMILIE_PLUNGE_HIGH: TalentScaling = TalentScaling {
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

// --- Elemental Skill: Fragrance Extraction ---

const EMILIE_SKILL_DMG: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.4708, 0.50611, 0.54142, 0.5885, 0.62381, 0.65912, 0.7062, 0.75328, 0.80036, 0.84744,
        0.89452, 0.9416, 1.00045, 1.0593, 1.11815,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EMILIE_SKILL_LV1: TalentScaling = TalentScaling {
    name: "Lv.1香液ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.396, 0.4257, 0.4554, 0.495, 0.5247, 0.5544, 0.594, 0.6336, 0.6732, 0.7128, 0.7524, 0.792,
        0.8415, 0.891, 0.9405,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EMILIE_SKILL_LV2: TalentScaling = TalentScaling {
    name: "Lv.2香液ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.84, 0.903, 0.966, 1.05, 1.113, 1.176, 1.26, 1.344, 1.428, 1.512, 1.596, 1.68, 1.785,
        1.89, 1.995,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EMILIE_SKILL_THORN: TalentScaling = TalentScaling {
    name: "霊息棘ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.3852, 0.41409, 0.44298, 0.4815, 0.51039, 0.53928, 0.5778, 0.61632, 0.65484, 0.69336,
        0.73188, 0.7704, 0.81855, 0.8667, 0.91485,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Elemental Burst: Aromatic Explication ---

const EMILIE_BURST_LV3: TalentScaling = TalentScaling {
    name: "Lv.3香液ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        2.172, 2.3349, 2.4978, 2.715, 2.8779, 3.0408, 3.258, 3.4752, 3.6924, 3.9096, 4.1268, 4.344,
        4.6155, 4.887, 5.1585,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Emilie aggregation ---

static EMILIE_NA_HITS: &[TalentScaling] = &[
    EMILIE_NA_HIT1,
    EMILIE_NA_HIT2,
    EMILIE_NA_HIT3,
    EMILIE_NA_HIT4,
];
static EMILIE_CHARGED_ATTACKS: &[TalentScaling] = &[EMILIE_CHARGED];
static EMILIE_PLUNGING: &[TalentScaling] = &[EMILIE_PLUNGE, EMILIE_PLUNGE_LOW, EMILIE_PLUNGE_HIGH];
static EMILIE_SKILL_SCALINGS: &[TalentScaling] = &[
    EMILIE_SKILL_DMG,
    EMILIE_SKILL_LV1,
    EMILIE_SKILL_LV2,
    EMILIE_SKILL_THORN,
];
static EMILIE_BURST_SCALINGS: &[TalentScaling] = &[EMILIE_BURST_LV3];

pub const EMILIE: CharacterData = CharacterData {
    id: "emilie",
    name: "Emilie",
    element: Element::Dendro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [
        1056.00, 2740.00, 3646.00, 5455.00, 6099.00, 7016.00, 7874.00, 8802.00, 9445.00, 10381.00,
        11025.00, 11971.00, 12615.00, 13568.00, 13568.00, 14110.72, // Lv95/Lv95+/Lv100
        14110.72, // Lv95/Lv95+/Lv100
        14653.44, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        26.07, 67.62, 89.97, 134.62, 150.50, 173.16, 194.33, 217.22, 233.10, 256.19, 272.07,
        295.43, 311.31, 334.85, 334.85, 348.24, // Lv95/Lv95+/Lv100
        348.24, // Lv95/Lv95+/Lv100
        361.64, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        56.84, 147.44, 196.17, 293.54, 328.17, 377.56, 423.73, 473.63, 508.26, 558.62, 593.25,
        644.17, 678.80, 730.13, 730.13, 759.34, // Lv95/Lv95+/Lv100
        759.34, // Lv95/Lv95+/Lv100
        788.54, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "影追いの槍術・改",
            hits: EMILIE_NA_HITS,
            charged: EMILIE_CHARGED_ATTACKS,
            plunging: EMILIE_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "フレグランス·アコード",
            scalings: EMILIE_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "アロマティック·アナライズ",
            scalings: EMILIE_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
};
