use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// --- Normal Attack: Striking Serpent ---

const NEFER_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.3807, 0.4093, 0.4378, 0.4759, 0.5044, 0.5330, 0.5711, 0.6091, 0.6472, 0.6853, 0.7234,
        0.7614, 0.8090, 0.8566, 0.9042,
    ],
};

const NEFER_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.3756, 0.4038, 0.4320, 0.4696, 0.4977, 0.5259, 0.5635, 0.6010, 0.6386, 0.6762, 0.7137,
        0.7513, 0.7982, 0.8452, 0.8921,
    ],
};

const NEFER_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.2524, 0.2713, 0.2903, 0.3155, 0.3344, 0.3534, 0.3786, 0.4038, 0.4291, 0.4543, 0.4796,
        0.5048, 0.5364, 0.5679, 0.5994,
    ],
};

const NEFER_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.6099, 0.6557, 0.7014, 0.7624, 0.8082, 0.8539, 0.9149, 0.9759, 1.0369, 1.0979, 1.1589,
        1.2199, 1.2961, 1.3724, 1.4486,
    ],
};

const NEFER_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.3088, 1.4070, 1.5051, 1.6360, 1.7342, 1.8323, 1.9632, 2.0941, 2.2250, 2.3558, 2.4867,
        2.6176, 2.7812, 2.9448, 3.1084,
    ],
};

const NEFER_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const NEFER_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6516, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const NEFER_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// --- Elemental Skill: Senet Strategy: Dance of a Thousand Nights ---

const NEFER_SKILL_DMG_ATK: TalentScaling = TalentScaling {
    name: "スキルダメージ・攻撃力",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.7638, 0.8211, 0.8784, 0.9548, 1.0121, 1.0694, 1.1458, 1.2221, 1.2985, 1.3749, 1.4513,
        1.5277, 1.6232, 1.7186, 1.8141,
    ],
};

const NEFER_SKILL_DMG_EM: TalentScaling = TalentScaling {
    name: "スキルダメージ・元素熟知",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        1.5277, 1.6423, 1.7568, 1.9096, 2.0242, 2.1388, 2.2915, 2.4443, 2.5971, 2.7498, 2.9026,
        3.0554, 3.2463, 3.4373, 3.6282,
    ],
};

const NEFER_PHANTASM1_NEFER_ATK: TalentScaling = TalentScaling {
    name: "幻影演舞1段(Nefer)・攻撃力",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.2464, 0.2649, 0.2834, 0.3080, 0.3265, 0.3450, 0.3696, 0.3942, 0.4189, 0.4435, 0.4682,
        0.4928, 0.5236, 0.5544, 0.5852,
    ],
};

const NEFER_PHANTASM1_NEFER_EM: TalentScaling = TalentScaling {
    name: "幻影演舞1段(Nefer)・元素熟知",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        0.4928, 0.5298, 0.5667, 0.6160, 0.6530, 0.6899, 0.7392, 0.7885, 0.8378, 0.8870, 0.9363,
        0.9856, 1.0472, 1.1088, 1.1704,
    ],
};

const NEFER_PHANTASM2_NEFER_ATK: TalentScaling = TalentScaling {
    name: "幻影演舞2段(Nefer)・攻撃力",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.3203, 0.3443, 0.3684, 0.4004, 0.4244, 0.4484, 0.4805, 0.5125, 0.5445, 0.5766, 0.6086,
        0.6406, 0.6807, 0.7207, 0.7608,
    ],
};

const NEFER_PHANTASM2_NEFER_EM: TalentScaling = TalentScaling {
    name: "幻影演舞2段(Nefer)・元素熟知",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        0.6406, 0.6887, 0.7367, 0.8008, 0.8488, 0.8969, 0.9610, 1.0250, 1.0891, 1.1532, 1.2172,
        1.2813, 1.3614, 1.4414, 1.5215,
    ],
};

const NEFER_PHANTASM1_SHADES: TalentScaling = TalentScaling {
    name: "幻影演舞1段(分身)",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        0.96, 1.032, 1.104, 1.20, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16,
        2.28,
    ],
};

const NEFER_PHANTASM2_SHADES: TalentScaling = TalentScaling {
    name: "幻影演舞2段(分身)",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        0.96, 1.032, 1.104, 1.20, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16,
        2.28,
    ],
};

const NEFER_PHANTASM3_SHADES: TalentScaling = TalentScaling {
    name: "幻影演舞3段(分身)",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        1.28, 1.376, 1.472, 1.60, 1.696, 1.792, 1.92, 2.048, 2.176, 2.304, 2.432, 2.56, 2.72, 2.88,
        3.04,
    ],
};

// --- Elemental Burst: Sacred Vow: True Eye's Phantasm ---

const NEFER_BURST_HIT1_ATK: TalentScaling = TalentScaling {
    name: "1段ダメージ・攻撃力",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        2.2464, 2.4149, 2.5834, 2.8080, 2.9765, 3.1450, 3.3696, 3.5942, 3.8189, 4.0435, 4.2682,
        4.4928, 4.7736, 5.0544, 5.3352,
    ],
};

const NEFER_BURST_HIT1_EM: TalentScaling = TalentScaling {
    name: "1段ダメージ・元素熟知",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        4.4928, 4.8298, 5.1667, 5.6160, 5.9530, 6.2899, 6.7392, 7.1885, 7.6378, 8.0870, 8.5363,
        8.9856, 9.5472, 10.1088, 10.6704,
    ],
};

const NEFER_BURST_HIT2_ATK: TalentScaling = TalentScaling {
    name: "2段ダメージ・攻撃力",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        3.3696, 3.6223, 3.8750, 4.2120, 4.4647, 4.7174, 5.0544, 5.3914, 5.7283, 6.0653, 6.4022,
        6.7392, 7.1604, 7.5816, 8.0028,
    ],
};

const NEFER_BURST_HIT2_EM: TalentScaling = TalentScaling {
    name: "2段ダメージ・元素熟知",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        6.7392, 7.2446, 7.7501, 8.4240, 8.9294, 9.4349, 10.1088, 10.7827, 11.4566, 12.1306,
        12.8045, 13.4784, 14.3208, 15.1632, 16.0056,
    ],
};

// --- Nefer aggregation ---

static NEFER_NA_HITS: &[TalentScaling] =
    &[NEFER_NA_HIT1, NEFER_NA_HIT2, NEFER_NA_HIT3, NEFER_NA_HIT4];
static NEFER_CHARGED_ATTACKS: &[TalentScaling] = &[NEFER_CHARGED];
static NEFER_PLUNGING: &[TalentScaling] = &[NEFER_PLUNGE, NEFER_PLUNGE_LOW, NEFER_PLUNGE_HIGH];
static NEFER_SKILL_SCALINGS: &[TalentScaling] = &[
    NEFER_SKILL_DMG_ATK,
    NEFER_SKILL_DMG_EM,
    NEFER_PHANTASM1_NEFER_ATK,
    NEFER_PHANTASM1_NEFER_EM,
    NEFER_PHANTASM2_NEFER_ATK,
    NEFER_PHANTASM2_NEFER_EM,
    NEFER_PHANTASM1_SHADES,
    NEFER_PHANTASM2_SHADES,
    NEFER_PHANTASM3_SHADES,
];
static NEFER_BURST_SCALINGS: &[TalentScaling] = &[
    NEFER_BURST_HIT1_ATK,
    NEFER_BURST_HIT1_EM,
    NEFER_BURST_HIT2_ATK,
    NEFER_BURST_HIT2_EM,
];

pub const NEFER: CharacterData = CharacterData {
    id: "nefer",
    name: "Nefer",
    element: Element::Dendro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::NodKrai,
    base_hp: [989.0, 2565.0, 11208.0, 12704.0],
    base_atk: [26.81, 69.55, 303.87, 344.42],
    base_def: [62.22, 161.41, 705.2, 799.3],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "ストライキング・サーペント",
            hits: NEFER_NA_HITS,
            charged: NEFER_CHARGED_ATTACKS,
            plunging: NEFER_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "セネト戦略：千夜の舞",
            scalings: NEFER_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "聖約：真眼の幻影",
            scalings: NEFER_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
