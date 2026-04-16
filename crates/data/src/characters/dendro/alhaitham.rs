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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ALHAITHAM_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.507495, 0.548802, 0.59011, 0.649121, 0.690429, 0.737638, 0.80255, 0.867462, 0.932374,
        1.003187, 1.074, 1.144813, 1.215627, 1.28644, 1.357253,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ALHAITHAM_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.341785, 0.369605, 0.397425, 0.437167, 0.464987, 0.496781, 0.540498, 0.584215, 0.627931,
        0.675622, 0.723313, 0.771004, 0.818696, 0.866387, 0.914077,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ALHAITHAM_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.667678, 0.722024, 0.77637, 0.854007, 0.908353, 0.970463, 1.055863, 1.141264, 1.226665,
        1.319829, 1.412993, 1.506158, 1.599322, 1.692487, 1.785651,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ALHAITHAM_NA_HIT5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.838509, 0.906759, 0.97501, 1.072511, 1.140762, 1.218762, 1.326014, 1.433265, 1.540516,
        1.657517, 1.774518, 1.891519, 2.008521, 2.125522, 2.242523,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ALHAITHAM_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.55255, 0.597525, 0.6425, 0.70675, 0.751725, 0.803125, 0.8738, 0.944475, 1.01515, 1.09225,
        1.16935, 1.24645, 1.32355, 1.40065, 1.47775,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ALHAITHAM_PLUNGE: TalentScaling = TalentScaling {
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

const ALHAITHAM_PLUNGE_LOW: TalentScaling = TalentScaling {
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

const ALHAITHAM_PLUNGE_HIGH: TalentScaling = TalentScaling {
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

// --- Elemental Skill: Universality: An Elaboration on Form (ATK portion) ---

const ALHAITHAM_SKILL_THRUST: TalentScaling = TalentScaling {
    name: "突進攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.936, 2.0812, 2.2264, 2.42, 2.5652, 2.7104, 2.904, 3.0976, 3.2912, 3.4848, 3.6784, 3.872,
        4.114, 4.356, 4.598,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Elemental Skill: Universality: An Elaboration on Form (1-Mirror Projection) ---

const ALHAITHAM_SKILL_PROJECTION_ATK: TalentScaling = TalentScaling {
    name: "1枚光幕攻撃のダメージ(ATK)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.6720, 0.7224, 0.7728, 0.8400, 0.8904, 0.9408, 1.0080, 1.0752, 1.1424, 1.2096, 1.2768,
        1.3440, 1.4280, 1.5120, 1.5960,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ALHAITHAM_SKILL_PROJECTION_EM: TalentScaling = TalentScaling {
    name: "1枚光幕攻撃のダメージ(EM)",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        1.3440, 1.4448, 1.5456, 1.6800, 1.7808, 1.8816, 2.0160, 2.1504, 2.2848, 2.4192, 2.5536,
        2.6880, 2.8560, 3.0240, 3.1920,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Elemental Skill: Rush Attack EM portion ---

const ALHAITHAM_SKILL_THRUST_EM: TalentScaling = TalentScaling {
    name: "突進攻撃のダメージ(EM)",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        1.5488, 1.6650, 1.7811, 1.9360, 2.0522, 2.1683, 2.3232, 2.4781, 2.6330, 2.7878, 2.9427,
        3.0976, 3.2912, 3.4848, 3.6784,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Elemental Burst: Particular Field: Fetters of Phenomena (ATK portion) ---

const ALHAITHAM_BURST_HIT: TalentScaling = TalentScaling {
    name: "1回のダメージ(ATK)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.216, 1.3072, 1.3984, 1.52, 1.6112, 1.7024, 1.824, 1.9456, 2.0672, 2.1888, 2.3104, 2.432,
        2.584, 2.736, 2.888,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Elemental Burst: Particular Field: Fetters of Phenomena (EM portion) ---

const ALHAITHAM_BURST_HIT_EM: TalentScaling = TalentScaling {
    name: "1回のダメージ(EM)",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        0.9728, 1.0458, 1.1187, 1.2160, 1.2890, 1.3619, 1.4592, 1.5565, 1.6538, 1.7510, 1.8483,
        1.9456, 2.0672, 2.1888, 2.3104,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
static ALHAITHAM_SKILL_SCALINGS: &[TalentScaling] = &[
    ALHAITHAM_SKILL_THRUST,
    ALHAITHAM_SKILL_PROJECTION_ATK,
    ALHAITHAM_SKILL_PROJECTION_EM,
    ALHAITHAM_SKILL_THRUST_EM,
];
static ALHAITHAM_BURST_SCALINGS: &[TalentScaling] = &[ALHAITHAM_BURST_HIT, ALHAITHAM_BURST_HIT_EM];

pub const ALHAITHAM: CharacterData = CharacterData {
    id: "alhaitham",
    name: "Alhaitham",
    element: Element::Dendro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Sumeru,
    base_hp: [
        1039.00, 2695.00, 3586.00, 5366.00, 5999.00, 6902.00, 7747.00, 8659.00, 9292.00, 10213.00,
        10846.00, 11777.00, 12410.00, 13348.00, 13348.00, 13822.50, // Lv95/Lv95+/Lv100
        13822.50, // Lv95/Lv95+/Lv100
        14297.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        24.39, 63.27, 84.19, 125.97, 140.83, 162.03, 181.84, 203.25, 218.11, 239.72, 254.58,
        276.44, 291.30, 313.32, 313.32, 348.57, // Lv95/Lv95+/Lv100
        348.57, // Lv95/Lv95+/Lv100
        383.82, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        60.85, 157.84, 210.01, 314.24, 351.31, 404.19, 453.61, 507.04, 544.11, 598.02, 635.09,
        689.61, 726.67, 781.62, 781.62, 809.40, // Lv95/Lv95+/Lv100
        809.40, // Lv95/Lv95+/Lv100
        837.17, // Lv95/Lv95+/Lv100
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
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
};
