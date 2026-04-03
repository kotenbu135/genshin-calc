use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Lisa
// =============================================================================

// -- Normal Attack: 指先の雷 (Lightning Touch) -- All Electro (Catalyst) --

const LISA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3960, 0.4257, 0.4554, 0.4950, 0.5247, 0.5544, 0.5940, 0.6336, 0.6732, 0.7128, 0.7540,
        0.8078, 0.8617, 0.9156, 0.9694,
    ],
};

const LISA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3592, 0.3861, 0.4131, 0.4490, 0.4759, 0.5029, 0.5388, 0.5747, 0.6106, 0.6466, 0.6839,
        0.7328, 0.7816, 0.8305, 0.8793,
    ],
};

const LISA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4280, 0.4601, 0.4922, 0.5350, 0.5671, 0.5992, 0.6420, 0.6848, 0.7276, 0.7704, 0.8149,
        0.8731, 0.9313, 0.9895, 1.0477,
    ],
};

const LISA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5496, 0.5908, 0.6320, 0.6870, 0.7282, 0.7694, 0.8244, 0.8794, 0.9343, 0.9893, 1.0464,
        1.1212, 1.1959, 1.2707, 1.3454,
    ],
};

// -- Charged Attack -- Electro --

const LISA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.7712, 1.9040, 2.0369, 2.2140, 2.3468, 2.4797, 2.6568, 2.8339, 3.0110, 3.1882, 3.3724,
        3.6132, 3.8541, 4.0950, 4.3359,
    ],
};

// -- Plunging Attack -- Electro (Catalyst) --

const LISA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const LISA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const LISA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 蒼雷 (Violet Arc) -- Electro --

const LISA_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "長押しダメージ (0重)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        3.2000, 3.4400, 3.6800, 4.0000, 4.2400, 4.4800, 4.8000, 5.1200, 5.4400, 5.7600, 6.0800,
        6.4000, 6.8000, 7.2000, 7.6000,
    ],
};

const LISA_SKILL_HOLD_3STACK: TalentScaling = TalentScaling {
    name: "長押しダメージ (3重)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        4.8720, 5.2374, 5.6028, 6.0900, 6.4554, 6.8208, 7.3080, 7.9520, 8.2824, 8.7696, 9.2568,
        9.7440, 10.3530, 10.9620, 11.5710,
    ],
};

// -- Elemental Burst: 薔薇の雷光 (Lightning Rose) -- Electro --

const LISA_BURST: TalentScaling = TalentScaling {
    name: "放電ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3656, 0.3930, 0.4204, 0.4570, 0.4844, 0.5118, 0.5484, 0.5850, 0.6215, 0.6581, 0.6946,
        0.7312, 0.7769, 0.8226, 0.8683,
    ],
};

pub const LISA: CharacterData = CharacterData {
    id: "lisa",
    name: "Lisa",
    element: Element::Electro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [
        802.00, 2061.00, 2661.00, 3985.00, 4411.00, 5074.00, 5642.00, 6305.00, 6731.00, 7393.00,
        7819.00, 8481.00, 8907.00, 9570.00, 9570.00, 9952.80,  // Lv95/Lv95+/Lv100
        9952.80,  // Lv95/Lv95+/Lv100
        10335.60, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        19.41, 49.87, 64.37, 96.41, 106.72, 122.75, 136.49, 152.52, 162.83, 178.84, 189.14, 205.18,
        215.48, 231.51, 231.51, 240.77, // Lv95/Lv95+/Lv100
        240.77, // Lv95/Lv95+/Lv100
        250.03, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        48.07, 123.49, 159.40, 238.76, 264.28, 303.98, 338.00, 377.71, 403.22, 442.88, 468.39,
        508.10, 533.61, 573.32, 573.32, 596.25, // Lv95/Lv95+/Lv100
        596.25, // Lv95/Lv95+/Lv100
        619.19, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "指先の雷",
            hits: &[LISA_NORMAL_1, LISA_NORMAL_2, LISA_NORMAL_3, LISA_NORMAL_4],
            charged: &[LISA_CHARGED],
            plunging: &[LISA_PLUNGE, LISA_PLUNGE_LOW, LISA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "蒼雷",
            scalings: &[LISA_SKILL_PRESS, LISA_SKILL_HOLD_3STACK],
        },
        elemental_burst: TalentData {
            name: "薔薇の雷光",
            scalings: &[LISA_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
