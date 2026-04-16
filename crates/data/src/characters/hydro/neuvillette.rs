use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 如水カノン (As Water Seeks Equilibrium) -- All Hydro (Catalyst) --

const NEUVILLETTE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5458, 0.5867, 0.6276, 0.6822, 0.7231, 0.7641, 0.8187, 0.8732, 0.9278, 0.9824, 1.0370,
        1.0915, 1.1598, 1.2280, 1.2962,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NEUVILLETTE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.4625, 0.4971, 0.5318, 0.5781, 0.6128, 0.6474, 0.6937, 0.7399, 0.7862, 0.8324, 0.8787,
        0.9249, 0.9827, 1.0405, 1.0983,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NEUVILLETTE_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.7234, 0.7776, 0.8319, 0.9042, 0.9585, 1.0127, 1.0851, 1.1574, 1.2297, 1.3021, 1.3744,
        1.4468, 1.5372, 1.6276, 1.7180,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Hydro (Catalyst) --

const NEUVILLETTE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.3680, 1.4706, 1.5732, 1.7100, 1.8126, 1.9152, 2.0520, 2.1888, 2.3256, 2.4624, 2.5992,
        2.7360, 2.9070, 3.0780, 3.2490,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack: 衡平な裁量 (Equitable Judgment) -- Hydro (HP scaling) --

const NEUVILLETTE_CHARGED_JUDGMENT: TalentScaling = TalentScaling {
    name: "衡平な裁量ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0732, 0.0791, 0.0851, 0.0936, 0.0996, 0.1064, 0.1157, 0.1251, 0.1345, 0.1447, 0.1549,
        0.1651, 0.1753, 0.1855, 0.1957,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Hydro (Catalyst) --

const NEUVILLETTE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NEUVILLETTE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NEUVILLETTE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 遺したき裁き (O Tears, I Shall Repay) -- Hydro --

const NEUVILLETTE_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1286, 0.1383, 0.1479, 0.1608, 0.1704, 0.1801, 0.1930, 0.2058, 0.2187, 0.2316, 0.2444,
        0.2573, 0.2734, 0.2894, 0.3055,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NEUVILLETTE_SKILL_THORN: TalentScaling = TalentScaling {
    name: "霊息の棘ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.2080, 0.2236, 0.2392, 0.2600, 0.2756, 0.2912, 0.3120, 0.3328, 0.3536, 0.3744, 0.3952,
        0.4160, 0.4420, 0.4680, 0.4940,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 潮よ、我に懲罰を委ねよ (O Tides, I Have Returned) -- Hydro --

const NEUVILLETTE_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.2226, 0.2393, 0.2560, 0.2782, 0.2949, 0.3116, 0.3339, 0.3561, 0.3784, 0.4006, 0.4229,
        0.4452, 0.4730, 0.5008, 0.5286,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NEUVILLETTE_BURST_WATERFALL: TalentScaling = TalentScaling {
    name: "滝ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0911, 0.0979, 0.1047, 0.1138, 0.1206, 0.1275, 0.1366, 0.1457, 0.1548, 0.1639, 0.1730,
        0.1821, 0.1935, 0.2049, 0.2163,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const NEUVILLETTE: CharacterData = CharacterData {
    id: "neuvillette",
    name: "Neuvillette",
    element: Element::Hydro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [
        1144.00, 2967.00, 3948.00, 5908.00, 6605.00, 7599.00, 8528.00, 9533.00, 10230.00, 11243.00,
        11940.00, 12965.00, 13662.00, 14695.00, 14695.00, 15282.80, // Lv95/Lv95+/Lv100
        15282.80, // Lv95/Lv95+/Lv100
        15870.60, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        16.22, 42.07, 55.97, 83.76, 93.63, 107.73, 120.90, 135.14, 145.02, 159.39, 169.27, 183.80,
        193.68, 208.32, 208.32, 216.65, // Lv95/Lv95+/Lv100
        216.65, // Lv95/Lv95+/Lv100
        224.99, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        44.87, 116.40, 154.87, 231.74, 259.08, 298.07, 334.52, 373.92, 401.26, 441.02, 468.35,
        508.56, 535.90, 576.42, 576.42, 599.48, // Lv95/Lv95+/Lv100
        599.48, // Lv95/Lv95+/Lv100
        622.53, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "如水カノン",
            hits: &[
                NEUVILLETTE_NORMAL_1,
                NEUVILLETTE_NORMAL_2,
                NEUVILLETTE_NORMAL_3,
            ],
            charged: &[NEUVILLETTE_CHARGED, NEUVILLETTE_CHARGED_JUDGMENT],
            plunging: &[
                NEUVILLETTE_PLUNGE,
                NEUVILLETTE_PLUNGE_LOW,
                NEUVILLETTE_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "遺したき裁き",
            scalings: &[NEUVILLETTE_SKILL, NEUVILLETTE_SKILL_THORN],
        },
        elemental_burst: TalentData {
            name: "潮よ、我に懲罰を委ねよ",
            scalings: &[NEUVILLETTE_BURST, NEUVILLETTE_BURST_WATERFALL],
        },
    },
    constellation_pattern: ConstellationPattern::C3NormalC5Burst,
    passive_scalings: &[],
    scaling_modifiers: &[],
};
