use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Kujou Sara
// =============================================================================

// -- Normal Attack: 天狗伝弓術 (Tengu Bowmanship) -- Physical --

const KUJOU_SARA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3689, 0.3990, 0.4290, 0.4719, 0.5019, 0.5363, 0.5834, 0.6306, 0.6778, 0.7293, 0.7808,
        0.8323, 0.8837, 0.9352, 0.9867,
    ],
    dynamic_bonus: None,
};

const KUJOU_SARA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3870, 0.4185, 0.4500, 0.4950, 0.5265, 0.5625, 0.6120, 0.6615, 0.7110, 0.7650, 0.8190,
        0.8730, 0.9270, 0.9810, 1.0350,
    ],
    dynamic_bonus: None,
};

const KUJOU_SARA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4850, 0.5245, 0.5640, 0.6204, 0.6599, 0.7050, 0.7670, 0.8291, 0.8911, 0.9588, 1.0265,
        1.0942, 1.1618, 1.2295, 1.2972,
    ],
    dynamic_bonus: None,
};

const KUJOU_SARA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5040, 0.5450, 0.5860, 0.6446, 0.6856, 0.7325, 0.7970, 0.8614, 0.9259, 0.9962, 1.0665,
        1.1368, 1.2072, 1.2775, 1.3478,
    ],
    dynamic_bonus: None,
};

const KUJOU_SARA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5805, 0.6278, 0.6750, 0.7425, 0.7898, 0.8438, 0.9180, 0.9923, 1.0665, 1.1475, 1.2285,
        1.3095, 1.3905, 1.4715, 1.5525,
    ],
    dynamic_bonus: None,
};

// -- Aimed Shot -- Electro (charged) --

const KUJOU_SARA_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
    dynamic_bonus: None,
};

const KUJOU_SARA_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const KUJOU_SARA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
};

const KUJOU_SARA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
};

const KUJOU_SARA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: 鴉天狗雷鳴 (Tengu Stormcall) -- Electro --

const KUJOU_SARA_SKILL: TalentScaling = TalentScaling {
    name: "天狗の羽撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2576, 1.3519, 1.4462, 1.5720, 1.6663, 1.7606, 1.8864, 2.0122, 2.1379, 2.2637, 2.3894,
        2.5152, 2.6724, 2.8296, 2.9868,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 煌煌千道鎮式 (Subjugation: Koukou Sendou) -- Electro --

const KUJOU_SARA_BURST_TITANBREAKER: TalentScaling = TalentScaling {
    name: "天狗雷撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        4.0960, 4.4032, 4.7104, 5.1200, 5.4272, 5.7344, 6.1440, 6.5536, 6.9632, 7.3728, 7.7824,
        8.1920, 8.7040, 9.2160, 9.7280,
    ],
    dynamic_bonus: None,
};

const KUJOU_SARA_BURST_STORMCLUSTER: TalentScaling = TalentScaling {
    name: "天狗雷球ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3412, 0.3668, 0.3924, 0.4265, 0.4521, 0.4777, 0.5118, 0.5459, 0.5800, 0.6142, 0.6483,
        0.6824, 0.7251, 0.7677, 0.8104,
    ],
    dynamic_bonus: None,
};

pub const KUJOU_SARA: CharacterData = CharacterData {
    id: "kujou_sara",
    name: "Kujou Sara",
    element: Element::Electro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Inazuma,
    base_hp: [
        802.00, 2061.00, 2661.00, 3985.00, 4411.00, 5074.00, 5642.00, 6305.00, 6731.00, 7393.00,
        7819.00, 8481.00, 8907.00, 9570.00, 9570.00, 9952.80,  // Lv95/Lv95+/Lv100
        9952.80,  // Lv95/Lv95+/Lv100
        10335.60, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        16.38, 42.09, 54.33, 81.38, 90.07, 103.61, 115.20, 128.73, 137.43, 150.95, 159.64, 173.18,
        181.87, 195.41, 195.41, 203.23, // Lv95/Lv95+/Lv100
        203.23, // Lv95/Lv95+/Lv100
        211.04, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        52.65, 135.25, 174.58, 261.50, 289.45, 332.93, 370.19, 413.68, 441.62, 485.06, 513.00,
        556.49, 584.43, 627.92, 627.92, 653.04, // Lv95/Lv95+/Lv100
        653.04, // Lv95/Lv95+/Lv100
        678.15, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "天狗伝弓術",
            hits: &[
                KUJOU_SARA_NORMAL_1,
                KUJOU_SARA_NORMAL_2,
                KUJOU_SARA_NORMAL_3,
                KUJOU_SARA_NORMAL_4,
                KUJOU_SARA_NORMAL_5,
            ],
            charged: &[KUJOU_SARA_AIMED, KUJOU_SARA_AIMED_FULL],
            plunging: &[
                KUJOU_SARA_PLUNGE,
                KUJOU_SARA_PLUNGE_LOW,
                KUJOU_SARA_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "鴉天狗雷鳴",
            scalings: &[KUJOU_SARA_SKILL],
        },
        elemental_burst: TalentData {
            name: "煌煌千道鎮式",
            scalings: &[KUJOU_SARA_BURST_TITANBREAKER, KUJOU_SARA_BURST_STORMCLUSTER],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
