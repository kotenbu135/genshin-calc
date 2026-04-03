use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    base_hp: [
        845.00, 2191.00, 2915.00, 4362.00, 4877.00, 5611.00, 6297.00, 7038.00, 7553.00, 8301.00,
        8816.00, 9573.00, 10087.00, 10850.00, 10850.00, 11284.00, // Lv95/Lv95+/Lv100
        11284.00, // Lv95/Lv95+/Lv100
        11718.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        20.85, 54.10, 71.98, 107.70, 120.40, 138.53, 155.46, 173.77, 186.48, 204.96, 217.66,
        236.34, 249.05, 267.88, 267.88, 278.60, // Lv95/Lv95+/Lv100
        278.60, // Lv95/Lv95+/Lv100
        289.31, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        49.06, 127.26, 169.33, 253.37, 283.26, 325.89, 365.74, 408.82, 438.71, 482.18, 512.07,
        556.02, 585.91, 630.21, 630.21, 655.42, // Lv95/Lv95+/Lv100
        655.42, // Lv95/Lv95+/Lv100
        680.63, // Lv95/Lv95+/Lv100
    ],
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
