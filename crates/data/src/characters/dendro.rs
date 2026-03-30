// Game data values that happen to approximate mathematical constants (e.g., 0.7071, 0.318)
// are actual talent scaling percentages from datamining, not approximations of pi/sqrt(2)/etc.
#![allow(clippy::approx_constant)]

use genshin_calc_core::{Element, ScalingStat};

use crate::types::*;

// =============================================================================
// Alhaitham — 5★ Dendro Sword (Sumeru)
// Source: genshin-db-api (genshin-db-api.vercel.app)
// Normal Attack: Abductive Reasoning (リトロダクション)
// Elemental Skill: Universality: An Elaboration on Form (共相·イデア模写)
// Elemental Burst: Particular Field: Fetters of Phenomena (殊境·顕象結縛)
// =============================================================================

// --- Normal Attack: Abductive Reasoning ---

const ALHAITHAM_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.495257, 0.535568, 0.57588, 0.633468, 0.67378, 0.71985, 0.783197, 0.846544, 0.90989,
        0.978996, 1.048102, 1.117207, 1.186313, 1.255418, 1.324524,
    ],
};

const ALHAITHAM_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.507495, 0.548802, 0.59011, 0.649121, 0.690429, 0.737638, 0.80255, 0.867462, 0.932374,
        1.003187, 1.074, 1.144813, 1.215627, 1.28644, 1.357253,
    ],
};

const ALHAITHAM_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.341785, 0.369605, 0.397425, 0.437167, 0.464987, 0.496781, 0.540498, 0.584215, 0.627931,
        0.675622, 0.723313, 0.771004, 0.818696, 0.866387, 0.914077,
    ],
};

const ALHAITHAM_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.667678, 0.722024, 0.77637, 0.854007, 0.908353, 0.970463, 1.055863, 1.141264, 1.226665,
        1.319829, 1.412993, 1.506158, 1.599322, 1.692487, 1.785651,
    ],
};

const ALHAITHAM_NA_HIT5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.838509, 0.906759, 0.97501, 1.072511, 1.140762, 1.218762, 1.326014, 1.433265, 1.540516,
        1.657517, 1.774518, 1.891519, 2.008521, 2.125522, 2.242523,
    ],
};

const ALHAITHAM_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.55255, 0.597525, 0.6425, 0.70675, 0.751725, 0.803125, 0.8738, 0.944475, 1.01515, 1.09225,
        1.16935, 1.24645, 1.32355, 1.40065, 1.47775,
    ],
};

const ALHAITHAM_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572,
        1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982,
    ],
};

const ALHAITHAM_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646,
        2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915,
    ],
};

const ALHAITHAM_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586,
        3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041,
    ],
};

// --- Elemental Skill: Universality: An Elaboration on Form (ATK portion) ---

const ALHAITHAM_SKILL_THRUST: TalentScaling = TalentScaling {
    name: "突進攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.936, 2.0812, 2.2264, 2.42, 2.5652, 2.7104, 2.904, 3.0976, 3.2912, 3.4848, 3.6784, 3.872,
        4.114, 4.356, 4.598,
    ],
};

// --- Elemental Burst: Particular Field: Fetters of Phenomena (ATK portion) ---

const ALHAITHAM_BURST_HIT: TalentScaling = TalentScaling {
    name: "1回のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.216, 1.3072, 1.3984, 1.52, 1.6112, 1.7024, 1.824, 1.9456, 2.0672, 2.1888, 2.3104, 2.432,
        2.584, 2.736, 2.888,
    ],
};

// --- Alhaitham aggregation ---

static ALHAITHAM_NA_HITS: &[TalentScaling] = &[
    ALHAITHAM_NA_HIT1,
    ALHAITHAM_NA_HIT2,
    ALHAITHAM_NA_HIT3,
    ALHAITHAM_NA_HIT4,
    ALHAITHAM_NA_HIT5,
];
static ALHAITHAM_CHARGED_ATTACKS: &[TalentScaling] = &[ALHAITHAM_CHARGED];
static ALHAITHAM_PLUNGING: &[TalentScaling] = &[
    ALHAITHAM_PLUNGE,
    ALHAITHAM_PLUNGE_LOW,
    ALHAITHAM_PLUNGE_HIGH,
];
static ALHAITHAM_SKILL_SCALINGS: &[TalentScaling] = &[ALHAITHAM_SKILL_THRUST];
static ALHAITHAM_BURST_SCALINGS: &[TalentScaling] = &[ALHAITHAM_BURST_HIT];

pub const ALHAITHAM: CharacterData = CharacterData {
    id: "alhaitham",
    name: "Alhaitham",
    element: Element::Dendro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Sumeru,
    base_hp: [1039.0, 11777.0, 12410.0, 13348.0],
    base_atk: [24.0, 276.0, 291.0, 313.0],
    base_def: [61.0, 690.0, 727.0, 782.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Dendro, 0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "リトロダクション",
            hits: ALHAITHAM_NA_HITS,
            charged: ALHAITHAM_CHARGED_ATTACKS,
            plunging: ALHAITHAM_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "共相·イデア模写",
            scalings: ALHAITHAM_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "殊境·顕象結縛",
            scalings: ALHAITHAM_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Baizhu — 5★ Dendro Catalyst (Liyue)
// Source: genshin-db-api
// Normal Attack: The Classics of Acupuncture (金匱鍼解)
// Elemental Skill: Universal Diagnosis (太素診要)
// Elemental Burst: Holistic Revivification (癒気全形論)
// =============================================================================

// --- Normal Attack: The Classics of Acupuncture (all Dendro — catalyst) ---

const BAIZHU_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.373704, 0.401732, 0.42976, 0.46713, 0.495158, 0.523186, 0.560556, 0.597926, 0.635297,
        0.672667, 0.710038, 0.747408, 0.794121, 0.840834, 0.887547,
    ],
};

const BAIZHU_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.364248, 0.391567, 0.418885, 0.45531, 0.482629, 0.509947, 0.546372, 0.582797, 0.619222,
        0.655646, 0.692071, 0.728496, 0.774027, 0.819558, 0.865089,
    ],
};

const BAIZHU_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.225416, 0.242322, 0.259228, 0.28177, 0.298676, 0.315582, 0.338124, 0.360666, 0.383207,
        0.405749, 0.42829, 0.450832, 0.479009, 0.507186, 0.535363,
    ],
};

const BAIZHU_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.541376, 0.581979, 0.622582, 0.67672, 0.717323, 0.757926, 0.812064, 0.866202, 0.920339,
        0.974477, 1.028614, 1.082752, 1.150424, 1.218096, 1.285768,
    ],
};

const BAIZHU_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.2104, 1.30118, 1.39196, 1.513, 1.60378, 1.69456, 1.8156, 1.93664, 2.05768, 2.17872,
        2.29976, 2.4208, 2.5721, 2.7234, 2.8747,
    ],
};

const BAIZHU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
};

const BAIZHU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const BAIZHU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
};

// --- Elemental Skill: Universal Diagnosis ---

const BAIZHU_SKILL_DMG: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.792, 0.8514, 0.9108, 0.99, 1.0494, 1.1088, 1.188, 1.2672, 1.3464, 1.4256, 1.5048, 1.584,
        1.683, 1.782, 1.881,
    ],
};

// --- Elemental Burst: Holistic Revivification ---

const BAIZHU_BURST_SPIRIT: TalentScaling = TalentScaling {
    name: "霊気棘ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.97064, 1.043438, 1.116236, 1.2133, 1.286098, 1.358896, 1.45596, 1.553024, 1.650088,
        1.747152, 1.844216, 1.94128, 2.06261, 2.18394, 2.30527,
    ],
};

// --- Baizhu aggregation ---

static BAIZHU_NA_HITS: &[TalentScaling] = &[
    BAIZHU_NA_HIT1,
    BAIZHU_NA_HIT2,
    BAIZHU_NA_HIT3,
    BAIZHU_NA_HIT4,
];
static BAIZHU_CHARGED_ATTACKS: &[TalentScaling] = &[BAIZHU_CHARGED];
static BAIZHU_PLUNGING: &[TalentScaling] = &[BAIZHU_PLUNGE, BAIZHU_PLUNGE_LOW, BAIZHU_PLUNGE_HIGH];
static BAIZHU_SKILL_SCALINGS: &[TalentScaling] = &[BAIZHU_SKILL_DMG];
static BAIZHU_BURST_SCALINGS: &[TalentScaling] = &[BAIZHU_BURST_SPIRIT];

pub const BAIZHU: CharacterData = CharacterData {
    id: "baizhu",
    name: "Baizhu",
    element: Element::Dendro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [1039.0, 11777.0, 12410.0, 13348.0],
    base_atk: [15.0, 170.0, 179.0, 193.0],
    base_def: [39.0, 441.0, 464.0, 500.0],
    ascension_stat: AscensionStat::Hp(0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "金匱鍼解",
            hits: BAIZHU_NA_HITS,
            charged: BAIZHU_CHARGED_ATTACKS,
            plunging: BAIZHU_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "太素診要",
            scalings: BAIZHU_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "癒気全形論",
            scalings: BAIZHU_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Collei — 4★ Dendro Bow (Sumeru)
// Source: genshin-db-api
// Normal Attack: Supplicant's Bowmanship (祈りの射技)
// Elemental Skill: Floral Brush (花触葉讃)
// Elemental Burst: Trump-Card Kitty (ニャンコトレジャー)
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
};

const COLLEI_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.42656, 0.46128, 0.496, 0.5456, 0.58032, 0.62, 0.67456, 0.72912, 0.78368, 0.8432, 0.90272,
        0.96224, 1.02176, 1.08128, 1.1408,
    ],
};

const COLLEI_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.54094, 0.58497, 0.629, 0.6919, 0.73593, 0.78625, 0.85544, 0.92463, 0.99382, 1.0693,
        1.14478, 1.22026, 1.29574, 1.37122, 1.4467,
    ],
};

const COLLEI_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.68026, 0.73563, 0.791, 0.8701, 0.92547, 0.98875, 1.07576, 1.16277, 1.24978, 1.3447,
        1.43962, 1.53454, 1.62946, 1.72438, 1.8193,
    ],
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
};

const COLLEI_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635,
        2.79, 2.945,
    ],
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
};

const COLLEI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const COLLEI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
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
};

const COLLEI_BURST_LEAP: TalentScaling = TalentScaling {
    name: "跳躍ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.43248, 0.464916, 0.497352, 0.5406, 0.573036, 0.605472, 0.64872, 0.691968, 0.735216,
        0.778464, 0.821712, 0.86496, 0.91902, 0.97308, 1.02714,
    ],
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
    base_hp: [821.0, 8674.0, 9110.0, 9787.0],
    base_atk: [17.0, 177.0, 186.0, 200.0],
    base_def: [50.0, 532.0, 559.0, 601.0],
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
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Emilie — 5★ Dendro Polearm (Fontaine)
// Source: genshin-db-api
// Normal Attack: Shadow-Hunting Spear: Modified (影追いの槍術・改)
// Elemental Skill: Fragrance Extraction (フレグランス·アコード)
// Elemental Burst: Aromatic Explication (アロマティック·アナライズ)
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
};

const EMILIE_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.448954, 0.485497, 0.52204, 0.574244, 0.610787, 0.65255, 0.709974, 0.767399, 0.824823,
        0.887468, 0.950113, 1.012758, 1.075402, 1.138047, 1.200692,
    ],
};

const EMILIE_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.593004, 0.641272, 0.68954, 0.758494, 0.806762, 0.861925, 0.937774, 1.013624, 1.089473,
        1.172218, 1.254963, 1.337708, 1.420452, 1.503197, 1.585942,
    ],
};

const EMILIE_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.751029, 0.81216, 0.87329, 0.960619, 1.021749, 1.091613, 1.187674, 1.283736, 1.379798,
        1.484593, 1.589388, 1.694183, 1.798977, 1.903772, 2.008567,
    ],
};

const EMILIE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.91332, 0.98766, 1.062, 1.1682, 1.24254, 1.3275, 1.44432, 1.56114, 1.67796, 1.8054,
        1.93284, 2.06028, 2.18772, 2.31516, 2.4426,
    ],
};

const EMILIE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572,
        1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982,
    ],
};

const EMILIE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646,
        2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915,
    ],
};

const EMILIE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586,
        3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041,
    ],
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
};

const EMILIE_SKILL_LV1: TalentScaling = TalentScaling {
    name: "Lv.1香液ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.396, 0.4257, 0.4554, 0.495, 0.5247, 0.5544, 0.594, 0.6336, 0.6732, 0.7128, 0.7524, 0.792,
        0.8415, 0.891, 0.9405,
    ],
};

const EMILIE_SKILL_LV2: TalentScaling = TalentScaling {
    name: "Lv.2香液ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.84, 0.903, 0.966, 1.05, 1.113, 1.176, 1.26, 1.344, 1.428, 1.512, 1.596, 1.68, 1.785,
        1.89, 1.995,
    ],
};

const EMILIE_SKILL_THORN: TalentScaling = TalentScaling {
    name: "霊息棘ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.3852, 0.41409, 0.44298, 0.4815, 0.51039, 0.53928, 0.5778, 0.61632, 0.65484, 0.69336,
        0.73188, 0.7704, 0.81855, 0.8667, 0.91485,
    ],
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
    base_hp: [1056.0, 11971.0, 12615.0, 13568.0],
    base_atk: [26.0, 295.0, 311.0, 335.0],
    base_def: [57.0, 644.0, 679.0, 730.0],
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
};

// =============================================================================
// Kaveh — 4★ Dendro Claymore (Sumeru)
// Source: genshin-db-api
// Normal Attack: Schematic Setup (円規定則)
// Elemental Skill: Artistic Ingenuity (精緻なる絵図)
// Elemental Burst: Painted Dome (ムカルナスの描像)
// =============================================================================

// --- Normal Attack: Schematic Setup (physical) ---

const KAVEH_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.761857, 0.823868, 0.88588, 0.974468, 1.03648, 1.10735, 1.204797, 1.302244, 1.39969,
        1.505996, 1.612302, 1.718607, 1.824913, 1.931218, 2.037524,
    ],
};

const KAVEH_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.696385, 0.753068, 0.80975, 0.890725, 0.947407, 1.012188, 1.10126, 1.190333, 1.279405,
        1.376575, 1.473745, 1.570915, 1.668085, 1.765255, 1.862425,
    ],
};

const KAVEH_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.842611, 0.911195, 0.97978, 1.077758, 1.146343, 1.224725, 1.332501, 1.440277, 1.548052,
        1.665626, 1.7832, 1.900773, 2.018347, 2.13592, 2.253494,
    ],
};

const KAVEH_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.026883, 1.110467, 1.19405, 1.313455, 1.397039, 1.492563, 1.623908, 1.755254, 1.886599,
        2.029885, 2.173171, 2.316457, 2.459743, 2.603029, 2.746315,
    ],
};

const KAVEH_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.53148, 0.57474, 0.618, 0.6798, 0.72306, 0.7725, 0.84048, 0.90846, 0.97644, 1.0506,
        1.12476, 1.19892, 1.27308, 1.34724, 1.4214,
    ],
};

const KAVEH_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.96148, 1.03974, 1.118, 1.2298, 1.30806, 1.3975, 1.52048, 1.64346, 1.76644, 1.9006,
        2.03476, 2.16892, 2.30308, 2.43724, 2.5714,
    ],
};

const KAVEH_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.745878, 0.806589, 0.8673, 0.95403, 1.014741, 1.084125, 1.179528, 1.274931, 1.370334,
        1.47441, 1.578486, 1.682562, 1.786638, 1.890714, 1.99479,
    ],
};

const KAVEH_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.49144, 1.612836, 1.734233, 1.907656, 2.029052, 2.167791, 2.358556, 2.549322, 2.740087,
        2.948195, 3.156303, 3.364411, 3.572519, 3.780627, 3.988735,
    ],
};

const KAVEH_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.862889, 2.01452, 2.16615, 2.382765, 2.534396, 2.707688, 2.945964, 3.184241, 3.422517,
        3.682455, 3.942393, 4.202331, 4.462269, 4.722207, 4.982145,
    ],
};

// --- Elemental Skill: Artistic Ingenuity ---

const KAVEH_SKILL_DMG: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        2.04, 2.193, 2.346, 2.55, 2.703, 2.856, 3.06, 3.264, 3.468, 3.672, 3.876, 4.08, 4.335,
        4.59, 4.845,
    ],
};

// --- Elemental Burst: Painted Dome ---

const KAVEH_BURST_DMG: TalentScaling = TalentScaling {
    name: "元素爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.6, 1.72, 1.84, 2.0, 2.12, 2.24, 2.4, 2.56, 2.72, 2.88, 3.04, 3.2, 3.4, 3.6, 3.8,
    ],
};

// --- Kaveh aggregation ---

static KAVEH_NA_HITS: &[TalentScaling] =
    &[KAVEH_NA_HIT1, KAVEH_NA_HIT2, KAVEH_NA_HIT3, KAVEH_NA_HIT4];
static KAVEH_CHARGED_ATTACKS: &[TalentScaling] = &[KAVEH_CHARGED_SPINNING, KAVEH_CHARGED_FINAL];
static KAVEH_PLUNGING: &[TalentScaling] = &[KAVEH_PLUNGE, KAVEH_PLUNGE_LOW, KAVEH_PLUNGE_HIGH];
static KAVEH_SKILL_SCALINGS: &[TalentScaling] = &[KAVEH_SKILL_DMG];
static KAVEH_BURST_SCALINGS: &[TalentScaling] = &[KAVEH_BURST_DMG];

pub const KAVEH: CharacterData = CharacterData {
    id: "kaveh",
    name: "Kaveh",
    element: Element::Dendro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Sumeru,
    base_hp: [1003.0, 10602.0, 11134.0, 11962.0],
    base_atk: [20.0, 207.0, 217.0, 234.0],
    base_def: [63.0, 665.0, 699.0, 751.0],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "円規定則",
            hits: KAVEH_NA_HITS,
            charged: KAVEH_CHARGED_ATTACKS,
            plunging: KAVEH_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "精緻なる絵図",
            scalings: KAVEH_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "ムカルナスの描像",
            scalings: KAVEH_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Kinich — 5★ Dendro Claymore (Natlan)
// Source: genshin-db-api
// Normal Attack: Nightsun Style (武闘術・白夜)
// Elemental Skill: Canopy Hunter: Riding High (懸狩り·宙の遊猟)
// Elemental Burst: Hail to the Almighty Dragonlord (偉大なる聖龍を崇拝せよ)
// =============================================================================

// --- Normal Attack: Nightsun Style (physical) ---

const KINICH_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.98986, 1.07043, 1.151, 1.2661, 1.34667, 1.43875, 1.56536, 1.69197, 1.81858, 1.9567,
        2.09482, 2.23294, 2.37106, 2.50918, 2.6473,
    ],
};

const KINICH_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.82904, 0.89652, 0.964, 1.0604, 1.12788, 1.205, 1.31104, 1.41708, 1.52312, 1.6388,
        1.75448, 1.87016, 1.98584, 2.10152, 2.2172,
    ],
};

const KINICH_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.23496, 1.33548, 1.436, 1.5796, 1.68012, 1.795, 1.95296, 2.11092, 2.26888, 2.4412,
        2.61352, 2.78584, 2.95816, 3.13048, 3.3028,
    ],
};

const KINICH_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.48418, 0.52359, 0.563, 0.6193, 0.65871, 0.70375, 0.76568, 0.82761, 0.88954, 0.9571,
        1.02466, 1.09222, 1.15978, 1.22734, 1.2949,
    ],
};

const KINICH_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.745878, 0.806589, 0.8673, 0.95403, 1.014741, 1.084125, 1.179528, 1.274931, 1.370334,
        1.47441, 1.578486, 1.682562, 1.786638, 1.890714, 1.99479,
    ],
};

const KINICH_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.49144, 1.612836, 1.734233, 1.907656, 2.029052, 2.167791, 2.358556, 2.549322, 2.740087,
        2.948195, 3.156303, 3.364411, 3.572519, 3.780627, 3.988735,
    ],
};

const KINICH_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.862889, 2.01452, 2.16615, 2.382765, 2.534396, 2.707688, 2.945964, 3.184241, 3.422517,
        3.682455, 3.942393, 4.202331, 4.462269, 4.722207, 4.982145,
    ],
};

// --- Elemental Skill: Canopy Hunter: Riding High ---

const KINICH_SKILL_LOOP: TalentScaling = TalentScaling {
    name: "ループショットダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.5728, 0.61576, 0.65872, 0.716, 0.75896, 0.80192, 0.8592, 0.91648, 0.97376, 1.03104,
        1.08832, 1.1456, 1.2172, 1.2888, 1.3604,
    ],
};

const KINICH_SKILL_CANNON: TalentScaling = TalentScaling {
    name: "スケールスパイカー砲ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        6.8744, 7.38998, 7.90556, 8.593, 9.10858, 9.62416, 10.3116, 10.99904, 11.68648, 12.37392,
        13.06136, 13.7488, 14.6081, 15.4674, 16.3267,
    ],
};

// --- Elemental Burst: Hail to the Almighty Dragonlord ---

const KINICH_BURST_DMG: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.34, 1.4405, 1.541, 1.675, 1.7755, 1.876, 2.01, 2.144, 2.278, 2.412, 2.546, 2.68, 2.8475,
        3.015, 3.1825,
    ],
};

const KINICH_BURST_BREATH: TalentScaling = TalentScaling {
    name: "ドラゴンブレスダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.20736, 1.297912, 1.388464, 1.5092, 1.599752, 1.690304, 1.81104, 1.931776, 2.052512,
        2.173248, 2.293984, 2.41472, 2.56564, 2.71656, 2.86748,
    ],
};

// --- Kinich aggregation ---

static KINICH_NA_HITS: &[TalentScaling] = &[KINICH_NA_HIT1, KINICH_NA_HIT2, KINICH_NA_HIT3];
static KINICH_CHARGED_ATTACKS: &[TalentScaling] = &[KINICH_CHARGED];
static KINICH_PLUNGING: &[TalentScaling] = &[KINICH_PLUNGE, KINICH_PLUNGE_LOW, KINICH_PLUNGE_HIGH];
static KINICH_SKILL_SCALINGS: &[TalentScaling] = &[KINICH_SKILL_LOOP, KINICH_SKILL_CANNON];
static KINICH_BURST_SCALINGS: &[TalentScaling] = &[KINICH_BURST_DMG, KINICH_BURST_BREATH];

pub const KINICH: CharacterData = CharacterData {
    id: "kinich",
    name: "Kinich",
    element: Element::Dendro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [1001.0, 11345.0, 11955.0, 12858.0],
    base_atk: [26.0, 293.0, 309.0, 332.0],
    base_def: [62.0, 707.0, 745.0, 802.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "武闘術・白夜",
            hits: KINICH_NA_HITS,
            charged: KINICH_CHARGED_ATTACKS,
            plunging: KINICH_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "懸狩り·宙の遊猟",
            scalings: KINICH_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "偉大なる聖龍を崇拝せよ",
            scalings: KINICH_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Kirara — 4★ Dendro Sword (Inazuma)
// Source: genshin-db-api
// Normal Attack: Boxcutter (段板紙・切り裂き術)
// Elemental Skill: Meow-teor Kick (にゃんにゃん町飛脚)
// Elemental Burst: Secret Art: Surprise Dispatch (秘法·サプライズ特別配送)
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
};

const KIRARA_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.46354, 0.50127, 0.539, 0.5929, 0.63063, 0.67375, 0.73304, 0.79233, 0.85162, 0.9163,
        0.98098, 1.04566, 1.11034, 1.17502, 1.2397,
    ],
};

const KIRARA_NA_HIT3A: TalentScaling = TalentScaling {
    name: "3段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.254216, 0.274908, 0.2956, 0.32516, 0.345852, 0.3695, 0.402016, 0.434532, 0.467048,
        0.50252, 0.537992, 0.573464, 0.608936, 0.644408, 0.67988,
    ],
};

const KIRARA_NA_HIT3B: TalentScaling = TalentScaling {
    name: "3段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.381324, 0.412362, 0.4434, 0.48774, 0.518778, 0.55425, 0.603024, 0.651798, 0.700572,
        0.75378, 0.806988, 0.860196, 0.913404, 0.966612, 1.01982,
    ],
};

const KIRARA_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.73272, 0.79236, 0.852, 0.9372, 0.99684, 1.065, 1.15872, 1.25244, 1.34616, 1.4484,
        1.55064, 1.65288, 1.75512, 1.85736, 1.9596,
    ],
};

const KIRARA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ(x3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.223772, 0.241986, 0.2602, 0.28622, 0.304434, 0.32525, 0.353872, 0.382494, 0.411116,
        0.44234, 0.473564, 0.504788, 0.536012, 0.567236, 0.59846,
    ],
};

const KIRARA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572,
        1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982,
    ],
};

const KIRARA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646,
        2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915,
    ],
};

const KIRARA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586,
        3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041,
    ],
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
};

const KIRARA_BURST_EXPLOSION: TalentScaling = TalentScaling {
    name: "猫草豪雨ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.3564, 0.38313, 0.40986, 0.4455, 0.47223, 0.49896, 0.5346, 0.57024, 0.60588, 0.64152,
        0.67716, 0.7128, 0.75735, 0.8019, 0.84645,
    ],
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
static KIRARA_SKILL_SCALINGS: &[TalentScaling] = &[KIRARA_SKILL_KICK];
static KIRARA_BURST_SCALINGS: &[TalentScaling] = &[KIRARA_BURST_DMG, KIRARA_BURST_EXPLOSION];

pub const KIRARA: CharacterData = CharacterData {
    id: "kirara",
    name: "Kirara",
    element: Element::Dendro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Inazuma,
    base_hp: [1021.0, 10794.0, 11336.0, 12179.0],
    base_atk: [19.0, 198.0, 208.0, 223.0],
    base_def: [46.0, 484.0, 508.0, 546.0],
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

// =============================================================================
// Nahida — 5★ Dendro Catalyst (Sumeru)
// Source: genshin-db-api
// Normal Attack: Akara (行相)
// Elemental Skill: All Schemes to Know (所聞遍計)
// Elemental Burst: Illusory Heart (心景幻成)
// =============================================================================

// --- Normal Attack: Akara (all Dendro — catalyst) ---

const NAHIDA_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.403048, 0.433277, 0.463505, 0.50381, 0.534039, 0.564267, 0.604572, 0.644877, 0.685182,
        0.725486, 0.765791, 0.806096, 0.856477, 0.906858, 0.957239,
    ],
};

const NAHIDA_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.369744, 0.397475, 0.425206, 0.46218, 0.489911, 0.517642, 0.554616, 0.59159, 0.628565,
        0.665539, 0.702514, 0.739488, 0.785706, 0.831924, 0.878142,
    ],
};

const NAHIDA_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.458744, 0.49315, 0.527556, 0.57343, 0.607836, 0.642242, 0.688116, 0.73399, 0.779865,
        0.825739, 0.871614, 0.917488, 0.974831, 1.032174, 1.089517,
    ],
};

const NAHIDA_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.584064, 0.627869, 0.671674, 0.73008, 0.773885, 0.81769, 0.876096, 0.934502, 0.992909,
        1.051315, 1.109722, 1.168128, 1.241136, 1.314144, 1.387152,
    ],
};

const NAHIDA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.66, 0.7095, 0.759, 0.825, 0.8745, 0.924, 0.99, 1.056, 1.122, 1.188, 1.254, 1.32, 1.4025,
        1.485, 1.5675,
    ],
};

const NAHIDA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
};

const NAHIDA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const NAHIDA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
};

// --- Elemental Skill: All Schemes to Know (ATK portion of Tri-Karma Purification) ---

const NAHIDA_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "滅浄三業ダメージ(攻撃力)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.032, 1.1094, 1.1868, 1.29, 1.3674, 1.4448, 1.548, 1.6512, 1.7544, 1.8576, 1.9608, 2.064,
        2.193, 2.322, 2.451,
    ],
};

// --- Elemental Burst: Illusory Heart ---
// Nahida's burst provides buffs — no direct damage scalings

// --- Nahida aggregation ---

static NAHIDA_NA_HITS: &[TalentScaling] = &[
    NAHIDA_NA_HIT1,
    NAHIDA_NA_HIT2,
    NAHIDA_NA_HIT3,
    NAHIDA_NA_HIT4,
];
static NAHIDA_CHARGED_ATTACKS: &[TalentScaling] = &[NAHIDA_CHARGED];
static NAHIDA_PLUNGING: &[TalentScaling] = &[NAHIDA_PLUNGE, NAHIDA_PLUNGE_LOW, NAHIDA_PLUNGE_HIGH];
static NAHIDA_SKILL_SCALINGS: &[TalentScaling] = &[NAHIDA_SKILL_PRESS];
static NAHIDA_BURST_SCALINGS: &[TalentScaling] = &[];

pub const NAHIDA: CharacterData = CharacterData {
    id: "nahida",
    name: "Nahida",
    element: Element::Dendro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Sumeru,
    base_hp: [807.0, 9141.0, 9632.0, 10360.0],
    base_atk: [23.0, 264.0, 278.0, 299.0],
    base_def: [49.0, 556.0, 586.0, 630.0],
    ascension_stat: AscensionStat::ElementalMastery(115.2),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "行相",
            hits: NAHIDA_NA_HITS,
            charged: NAHIDA_CHARGED_ATTACKS,
            plunging: NAHIDA_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "所聞遍計",
            scalings: NAHIDA_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "心景幻成",
            scalings: NAHIDA_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Tighnari — 5★ Dendro Bow (Sumeru)
// Source: genshin-db-api
// Normal Attack: Khanda Barrier-Buster (蔵蘊散悩)
// Elemental Skill: Vijnana-Phala Mine (識果榴弾)
// Elemental Burst: Fashioner's Tanglevine Shaft (造生·蔓纏いの矢)
// =============================================================================

// --- Normal Attack: Khanda Barrier-Buster (physical) ---

const TIGHNARI_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.44634, 0.48267, 0.519, 0.5709, 0.60723, 0.64875, 0.70584, 0.76293, 0.82002, 0.8823,
        0.94458, 1.00686, 1.06914, 1.13142, 1.1937,
    ],
};

const TIGHNARI_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.41968, 0.45384, 0.488, 0.5368, 0.57096, 0.61, 0.66368, 0.71736, 0.77104, 0.8296, 0.88816,
        0.94672, 1.00528, 1.06384, 1.1224,
    ],
};

const TIGHNARI_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.26445, 0.285975, 0.3075, 0.33825, 0.359775, 0.384375, 0.4182, 0.452025, 0.48585, 0.52275,
        0.55965, 0.59655, 0.63345, 0.67035, 0.70725,
    ],
};

const TIGHNARI_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.68628, 0.74214, 0.798, 0.8778, 0.93366, 0.9975, 1.08528, 1.17306, 1.26084, 1.3566,
        1.45236, 1.54812, 1.64388, 1.73964, 1.8354,
    ],
};

// --- Aimed/Charged Attacks ---

const TIGHNARI_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894,
        1.0506, 1.1118, 1.173,
    ],
};

const TIGHNARI_AIMED_LV1: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635,
        2.79, 2.945,
    ],
};

const TIGHNARI_WREATH_ARROW: TalentScaling = TalentScaling {
    name: "花筐矢ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.872, 0.9374, 1.0028, 1.09, 1.1554, 1.2208, 1.308, 1.3952, 1.4824, 1.5696, 1.6568, 1.744,
        1.853, 1.962, 2.071,
    ],
};

const TIGHNARI_CLUSTERBLOOM: TalentScaling = TalentScaling {
    name: "蔵蘊花矢ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.386, 0.41495, 0.4439, 0.4825, 0.51145, 0.5404, 0.579, 0.6176, 0.6562, 0.6948, 0.7334,
        0.772, 0.82025, 0.8685, 0.91675,
    ],
};

// --- Plunging Attack (physical) ---

const TIGHNARI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
};

const TIGHNARI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const TIGHNARI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
};

// --- Elemental Skill: Vijnana-Phala Mine ---

const TIGHNARI_SKILL_DMG: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.496, 1.6082, 1.7204, 1.87, 1.9822, 2.0944, 2.244, 2.3936, 2.5432, 2.6928, 2.8424, 2.992,
        3.179, 3.366, 3.553,
    ],
};

// --- Elemental Burst: Fashioner's Tanglevine Shaft ---

const TIGHNARI_BURST_PRIMARY: TalentScaling = TalentScaling {
    name: "蔓纏い矢ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.5562, 0.597915, 0.63963, 0.69525, 0.736965, 0.77868, 0.8343, 0.88992, 0.94554, 1.00116,
        1.05678, 1.1124, 1.181925, 1.25145, 1.320975,
    ],
};

const TIGHNARI_BURST_SECONDARY: TalentScaling = TalentScaling {
    name: "次段蔓纏い矢ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.6798, 0.730785, 0.78177, 0.84975, 0.900735, 0.95172, 1.0197, 1.08768, 1.15566, 1.22364,
        1.29162, 1.3596, 1.444575, 1.52955, 1.614525,
    ],
};

// --- Tighnari aggregation ---

static TIGHNARI_NA_HITS: &[TalentScaling] = &[
    TIGHNARI_NA_HIT1,
    TIGHNARI_NA_HIT2,
    TIGHNARI_NA_HIT3,
    TIGHNARI_NA_HIT4,
];
static TIGHNARI_CHARGED_ATTACKS: &[TalentScaling] = &[
    TIGHNARI_AIMED,
    TIGHNARI_AIMED_LV1,
    TIGHNARI_WREATH_ARROW,
    TIGHNARI_CLUSTERBLOOM,
];
static TIGHNARI_PLUNGING: &[TalentScaling] =
    &[TIGHNARI_PLUNGE, TIGHNARI_PLUNGE_LOW, TIGHNARI_PLUNGE_HIGH];
static TIGHNARI_SKILL_SCALINGS: &[TalentScaling] = &[TIGHNARI_SKILL_DMG];
static TIGHNARI_BURST_SCALINGS: &[TalentScaling] =
    &[TIGHNARI_BURST_PRIMARY, TIGHNARI_BURST_SECONDARY];

pub const TIGHNARI: CharacterData = CharacterData {
    id: "tighnari",
    name: "Tighnari",
    element: Element::Dendro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Sumeru,
    base_hp: [845.0, 9573.0, 10087.0, 10850.0],
    base_atk: [21.0, 236.0, 249.0, 268.0],
    base_def: [49.0, 556.0, 586.0, 630.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Dendro, 0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "蔵蘊散悩",
            hits: TIGHNARI_NA_HITS,
            charged: TIGHNARI_CHARGED_ATTACKS,
            plunging: TIGHNARI_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "識果榴弾",
            scalings: TIGHNARI_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "造生·蔓纏いの矢",
            scalings: TIGHNARI_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Traveler (Dendro) — 5★ Dendro Sword (Other)
// Source: genshin-db-api
// Normal Attack: Foreign Fieldcleaver (異邦の草薙)
// Elemental Skill: Razorgrass Blade (草縁剣)
// Elemental Burst: Surgent Manifestation (臥草若化)
// =============================================================================

// --- Normal Attack: Foreign Fieldcleaver (physical) ---

const TRAVELER_DENDRO_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.44462, 0.48081, 0.517, 0.5687, 0.60489, 0.64625, 0.70312, 0.75999, 0.81686, 0.8789,
        0.94094, 1.00298, 1.06502, 1.12706, 1.1891,
    ],
};

const TRAVELER_DENDRO_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4343, 0.46965, 0.505, 0.5555, 0.59085, 0.63125, 0.6868, 0.74235, 0.7979, 0.8585, 0.9191,
        0.9797, 1.0403, 1.1009, 1.1615,
    ],
};

const TRAVELER_DENDRO_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.52976, 0.57288, 0.616, 0.6776, 0.72072, 0.77, 0.83776, 0.90552, 0.97328, 1.0472, 1.12112,
        1.19504, 1.26896, 1.34288, 1.4168,
    ],
};

const TRAVELER_DENDRO_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.58308, 0.63054, 0.678, 0.7458, 0.79326, 0.8475, 0.92208, 0.99666, 1.07124, 1.1526,
        1.23396, 1.31532, 1.39668, 1.47804, 1.5594,
    ],
};

const TRAVELER_DENDRO_NA_HIT5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.70778, 0.76539, 0.823, 0.9053, 0.96291, 1.02875, 1.11928, 1.20981, 1.30034, 1.3991,
        1.49786, 1.59662, 1.69538, 1.79414, 1.8929,
    ],
};

const TRAVELER_DENDRO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.559, 0.6045, 0.65, 0.715, 0.7605, 0.8125, 0.884, 0.9555, 1.027, 1.105, 1.183, 1.261,
        1.339, 1.417, 1.495,
    ],
};

const TRAVELER_DENDRO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572,
        1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982,
    ],
};

const TRAVELER_DENDRO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646,
        2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915,
    ],
};

const TRAVELER_DENDRO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586,
        3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041,
    ],
};

// --- Elemental Skill: Razorgrass Blade ---

const TRAVELER_DENDRO_SKILL_DMG: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        2.304, 2.4768, 2.6496, 2.88, 3.0528, 3.2256, 3.456, 3.6864, 3.9168, 4.1472, 4.3776, 4.608,
        4.896, 5.184, 5.472,
    ],
};

// --- Elemental Burst: Surgent Manifestation ---

const TRAVELER_DENDRO_BURST_LAMP: TalentScaling = TalentScaling {
    name: "蓮灯ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.8016, 0.86172, 0.92184, 1.002, 1.06212, 1.12224, 1.2024, 1.28256, 1.36272, 1.44288,
        1.52304, 1.6032, 1.7034, 1.8036, 1.9038,
    ],
};

const TRAVELER_DENDRO_BURST_EXPLOSION: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        4.008, 4.3086, 4.6092, 5.01, 5.3106, 5.6112, 6.012, 6.4128, 6.8136, 7.2144, 7.6152, 8.016,
        8.517, 9.018, 9.519,
    ],
};

// --- Traveler (Dendro) aggregation ---

static TRAVELER_DENDRO_NA_HITS: &[TalentScaling] = &[
    TRAVELER_DENDRO_NA_HIT1,
    TRAVELER_DENDRO_NA_HIT2,
    TRAVELER_DENDRO_NA_HIT3,
    TRAVELER_DENDRO_NA_HIT4,
    TRAVELER_DENDRO_NA_HIT5,
];
static TRAVELER_DENDRO_CHARGED_ATTACKS: &[TalentScaling] = &[TRAVELER_DENDRO_CHARGED];
static TRAVELER_DENDRO_PLUNGING: &[TalentScaling] = &[
    TRAVELER_DENDRO_PLUNGE,
    TRAVELER_DENDRO_PLUNGE_LOW,
    TRAVELER_DENDRO_PLUNGE_HIGH,
];
static TRAVELER_DENDRO_SKILL_SCALINGS: &[TalentScaling] = &[TRAVELER_DENDRO_SKILL_DMG];
static TRAVELER_DENDRO_BURST_SCALINGS: &[TalentScaling] =
    &[TRAVELER_DENDRO_BURST_LAMP, TRAVELER_DENDRO_BURST_EXPLOSION];

pub const TRAVELER_DENDRO: CharacterData = CharacterData {
    id: "traveler_dendro",
    name: "Traveler (Dendro)",
    element: Element::Dendro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Other,
    base_hp: [912.0, 9638.0, 10122.0, 10874.0],
    base_atk: [18.0, 188.0, 198.0, 212.0],
    base_def: [57.0, 605.0, 635.0, 682.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "異邦の草薙",
            hits: TRAVELER_DENDRO_NA_HITS,
            charged: TRAVELER_DENDRO_CHARGED_ATTACKS,
            plunging: TRAVELER_DENDRO_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "草縁剣",
            scalings: TRAVELER_DENDRO_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "臥草若化",
            scalings: TRAVELER_DENDRO_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Yaoyao — 4★ Dendro Polearm (Liyue)
// Source: genshin-db-api
// Normal Attack: Toss 'N' Turn Spear (顛撲連撃槍)
// Elemental Skill: Raphanus Sky Cluster (祥雲団々落清白)
// Elemental Burst: Moonjade Descent (玉顆珊々月中落)
// =============================================================================

// --- Normal Attack: Toss 'N' Turn Spear (physical) ---

const YAOYAO_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.510014, 0.551527, 0.59304, 0.652344, 0.693857, 0.7413, 0.806534, 0.871769, 0.937003,
        1.008168, 1.079333, 1.150498, 1.221662, 1.292827, 1.363992,
    ],
};

const YAOYAO_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.474428, 0.513044, 0.55166, 0.606826, 0.645442, 0.689575, 0.750258, 0.81094, 0.871623,
        0.937822, 1.004021, 1.07022, 1.13642, 1.202619, 1.268818,
    ],
};

const YAOYAO_NA_HIT3A: TalentScaling = TalentScaling {
    name: "3段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.313771, 0.339311, 0.36485, 0.401335, 0.426874, 0.456063, 0.496196, 0.53633, 0.576463,
        0.620245, 0.664027, 0.707809, 0.751591, 0.795373, 0.839155,
    ],
};

const YAOYAO_NA_HIT3B: TalentScaling = TalentScaling {
    name: "3段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.329457, 0.356274, 0.38309, 0.421399, 0.448215, 0.478862, 0.521002, 0.563142, 0.605282,
        0.651253, 0.697224, 0.743195, 0.789165, 0.835136, 0.881107,
    ],
};

const YAOYAO_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.779315, 0.842747, 0.90618, 0.996798, 1.060231, 1.132725, 1.232405, 1.332085, 1.431764,
        1.540506, 1.649248, 1.757989, 1.866731, 1.975472, 2.084214,
    ],
};

const YAOYAO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1266, 1.2183, 1.31, 1.441, 1.5327, 1.6375, 1.7816, 1.9257, 2.0698, 2.227, 2.3842, 2.5414,
        2.6986, 2.8558, 3.013,
    ],
};

const YAOYAO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572,
        1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982,
    ],
};

const YAOYAO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646,
        2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915,
    ],
};

const YAOYAO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586,
        3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041,
    ],
};

// --- Elemental Skill: Raphanus Sky Cluster ---

const YAOYAO_SKILL_RADISH: TalentScaling = TalentScaling {
    name: "白玉大根ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.2992, 0.32164, 0.34408, 0.374, 0.39644, 0.41888, 0.4488, 0.47872, 0.50864, 0.53856,
        0.56848, 0.5984, 0.6358, 0.6732, 0.7106,
    ],
};

// --- Elemental Burst: Moonjade Descent ---

const YAOYAO_BURST_RADISH: TalentScaling = TalentScaling {
    name: "白玉大根(爆発)ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.7216, 0.77572, 0.82984, 0.902, 0.95612, 1.01024, 1.0824, 1.15456, 1.22672, 1.29888,
        1.37104, 1.4432, 1.5334, 1.6236, 1.7138,
    ],
};

// --- Yaoyao aggregation ---

static YAOYAO_NA_HITS: &[TalentScaling] = &[
    YAOYAO_NA_HIT1,
    YAOYAO_NA_HIT2,
    YAOYAO_NA_HIT3A,
    YAOYAO_NA_HIT3B,
    YAOYAO_NA_HIT4,
];
static YAOYAO_CHARGED_ATTACKS: &[TalentScaling] = &[YAOYAO_CHARGED];
static YAOYAO_PLUNGING: &[TalentScaling] = &[YAOYAO_PLUNGE, YAOYAO_PLUNGE_LOW, YAOYAO_PLUNGE_HIGH];
static YAOYAO_SKILL_SCALINGS: &[TalentScaling] = &[YAOYAO_SKILL_RADISH];
static YAOYAO_BURST_SCALINGS: &[TalentScaling] = &[YAOYAO_BURST_RADISH];

pub const YAOYAO: CharacterData = CharacterData {
    id: "yaoyao",
    name: "Yaoyao",
    element: Element::Dendro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [1030.0, 10891.0, 11438.0, 12288.0],
    base_atk: [18.0, 188.0, 198.0, 212.0],
    base_def: [63.0, 665.0, 699.0, 751.0],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "顛撲連撃槍",
            hits: YAOYAO_NA_HITS,
            charged: YAOYAO_CHARGED_ATTACKS,
            plunging: YAOYAO_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "祥雲団々落清白",
            scalings: YAOYAO_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "玉顆珊々月中落",
            scalings: YAOYAO_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Lauma — 5★ Dendro Catalyst (Nod-Krai)
// Source: genshin-db-api (genshin-db-api.vercel.app)
// Normal Attack: Peregrination of Linnunrata
// Elemental Skill: Runo: Dawnless Rest of Karsikko
// Elemental Burst: Runo: All Hearts Become the Beating Moon
// =============================================================================

// --- Normal Attack: Peregrination of Linnunrata (all Dendro — catalyst) ---

const LAUMA_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.337024, 0.362301, 0.387578, 0.42128, 0.446557, 0.471834, 0.505536, 0.539238, 0.572941,
        0.606643, 0.640346, 0.674048, 0.716176, 0.758304, 0.800432,
    ],
};

const LAUMA_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.318048, 0.341902, 0.365755, 0.39756, 0.421414, 0.445267, 0.477072, 0.508877, 0.540682,
        0.572486, 0.604291, 0.636096, 0.675852, 0.715608, 0.755364,
    ],
};

const LAUMA_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.444968, 0.478341, 0.511713, 0.55621, 0.589583, 0.622955, 0.667452, 0.711949, 0.756446,
        0.800942, 0.845439, 0.889936, 0.945557, 1.001178, 1.056799,
    ],
};

const LAUMA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.2904, 1.38718, 1.48396, 1.613, 1.70978, 1.80656, 1.9356, 2.06464, 2.19368, 2.32272,
        2.45176, 2.5808, 2.7421, 2.9034, 3.0647,
    ],
};

const LAUMA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
};

const LAUMA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const LAUMA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
};

// --- Elemental Skill: Runo: Dawnless Rest of Karsikko ---

const LAUMA_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "狩りの讃歌ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.216, 1.3072, 1.3984, 1.52, 1.6112, 1.7024, 1.824, 1.9456, 2.0672, 2.1888, 2.3104, 2.432,
        2.584, 2.736, 2.888,
    ],
};

const LAUMA_SKILL_HOLD_HIT1: TalentScaling = TalentScaling {
    name: "永眠の讃歌ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.5808, 1.69936, 1.81792, 1.976, 2.09456, 2.21312, 2.3712, 2.52928, 2.68736, 2.84544,
        3.00352, 3.1616, 3.3592, 3.5568, 3.7544,
    ],
};

const LAUMA_SKILL_HOLD_HIT2: TalentScaling = TalentScaling {
    name: "月開花ダメージ",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        1.52, 1.634, 1.748, 1.9, 2.014, 2.128, 2.28, 2.432, 2.584, 2.736, 2.888, 3.04, 3.23, 3.42,
        3.61,
    ],
};

const LAUMA_SKILL_SANCTUARY_ATK: TalentScaling = TalentScaling {
    name: "霜林聖域攻撃ダメージ・攻撃力",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.96, 1.032, 1.104, 1.2, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16,
        2.28,
    ],
};

const LAUMA_SKILL_SANCTUARY_EM: TalentScaling = TalentScaling {
    name: "霜林聖域攻撃ダメージ・元素熟知",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        1.92, 2.064, 2.208, 2.4, 2.544, 2.688, 2.88, 3.072, 3.264, 3.456, 3.648, 3.84, 4.08, 4.32,
        4.56,
    ],
};

// --- Elemental Burst: Runo: All Hearts Become the Beating Moon ---

const LAUMA_BURST_BLOOM_INCREASE: TalentScaling = TalentScaling {
    name: "開花/超開花/烈開花ダメージ増加",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        2.7776, 2.98592, 3.19424, 3.472, 3.68032, 3.88864, 4.1664, 4.44416, 4.72192, 4.99968,
        5.27744, 5.5552, 5.9024, 6.2496, 6.5968,
    ],
};

const LAUMA_BURST_LUNAR_BLOOM_INCREASE: TalentScaling = TalentScaling {
    name: "月開花ダメージ増加",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        2.2224, 2.38908, 2.55576, 2.778, 2.94468, 3.11136, 3.3336, 3.55584, 3.77808, 4.00032,
        4.22256, 4.4448, 4.7226, 5.0004, 5.2782,
    ],
};

// --- Lauma aggregation ---

static LAUMA_NA_HITS: &[TalentScaling] = &[LAUMA_NA_HIT1, LAUMA_NA_HIT2, LAUMA_NA_HIT3];
static LAUMA_CHARGED_ATTACKS: &[TalentScaling] = &[LAUMA_CHARGED];
static LAUMA_PLUNGING: &[TalentScaling] = &[LAUMA_PLUNGE, LAUMA_PLUNGE_LOW, LAUMA_PLUNGE_HIGH];
static LAUMA_SKILL_SCALINGS: &[TalentScaling] = &[
    LAUMA_SKILL_PRESS,
    LAUMA_SKILL_HOLD_HIT1,
    LAUMA_SKILL_HOLD_HIT2,
    LAUMA_SKILL_SANCTUARY_ATK,
    LAUMA_SKILL_SANCTUARY_EM,
];
static LAUMA_BURST_SCALINGS: &[TalentScaling] =
    &[LAUMA_BURST_BLOOM_INCREASE, LAUMA_BURST_LUNAR_BLOOM_INCREASE];

pub const LAUMA: CharacterData = CharacterData {
    id: "lauma",
    name: "Lauma",
    element: Element::Dendro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::NodKrai,
    base_hp: [829.0, 2151.0, 9400.0, 10654.0],
    base_atk: [19.85, 51.49, 224.95, 254.96],
    base_def: [52.05, 135.02, 589.93, 668.64],
    ascension_stat: AscensionStat::ElementalMastery(115.2),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "リンヌンラタ巡礼",
            hits: LAUMA_NA_HITS,
            charged: LAUMA_CHARGED_ATTACKS,
            plunging: LAUMA_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "ルノ：カルシッコの陽なき安息",
            scalings: LAUMA_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "ルノ：万心鼓動たる月",
            scalings: LAUMA_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
