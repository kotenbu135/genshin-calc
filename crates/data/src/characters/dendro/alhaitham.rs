use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    base_hp: [
        1039.00, 11777.00, 11777.00, 12093.50, 12093.50, 12251.75, 12251.75, 12199.00, 12199.00,
        12879.00, 12879.00, 12410.00, 12410.00, 13348.00, 13348.00,
        13881.92, // Lv95/Lv95+/Lv100
        13881.92, // Lv95/Lv95+/Lv100
        14415.84, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        24.00, 276.00, 276.00, 283.50, 283.50, 287.25, 287.25, 286.00, 286.00, 302.00, 302.00,
        291.00, 291.00, 313.00, 313.00, 325.52, // Lv95/Lv95+/Lv100
        325.52, // Lv95/Lv95+/Lv100
        338.04, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        61.00, 690.00, 690.00, 708.50, 708.50, 717.75, 717.75, 714.67, 714.67, 754.50, 754.50,
        727.00, 727.00, 782.00, 782.00, 813.28, // Lv95/Lv95+/Lv100
        813.28, // Lv95/Lv95+/Lv100
        844.56, // Lv95/Lv95+/Lv100
    ],
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
