use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: ターゲットセラピー (Targeted Treatment) -- Physical (Bow) --

const SIGEWINNE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5260, 0.5700, 0.6120, 0.6730, 0.7160, 0.7650, 0.8320, 0.8990, 0.9670, 1.0400, 1.1130,
        1.1870, 1.2600, 1.3340, 1.4070,
    ],
    dynamic_bonus: None,
};

const SIGEWINNE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5110, 0.5520, 0.5940, 0.6530, 0.6950, 0.7420, 0.8080, 0.8730, 0.9380, 1.0090, 1.0810,
        1.1520, 1.2230, 1.2950, 1.3660,
    ],
    dynamic_bonus: None,
};

const SIGEWINNE_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7830, 0.8470, 0.9100, 1.0010, 1.0650, 1.1400, 1.2380, 1.3380, 1.4380, 1.5480, 1.6570,
        1.7660, 1.8750, 1.9850, 2.0940,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Bow --

const SIGEWINNE_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
    dynamic_bonus: None,
};

const SIGEWINNE_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
    dynamic_bonus: None,
};

const SIGEWINNE_MINI_BUBBLE: TalentScaling = TalentScaling {
    name: "ミニ泡沫弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.2480, 0.2670, 0.2850, 0.3100, 0.3290, 0.3470, 0.3720, 0.3970, 0.4220, 0.4460, 0.4710,
        0.4960, 0.5270, 0.5580, 0.5890,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const SIGEWINNE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
};

const SIGEWINNE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
};

const SIGEWINNE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: ぴょんぴょん水療法 (Bouncy Hydro Therapy) -- Hydro (HP scaling) --

const SIGEWINNE_SKILL_BUBBLE: TalentScaling = TalentScaling {
    name: "バウンドバブルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0228, 0.0245, 0.0262, 0.0285, 0.0302, 0.0319, 0.0342, 0.0365, 0.0388, 0.0410, 0.0433,
        0.0456, 0.0485, 0.0513, 0.0542,
    ],
    dynamic_bonus: None,
};

const SIGEWINNE_SKILL_BLADE: TalentScaling = TalentScaling {
    name: "飛散する水刃ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.00684, 0.00735, 0.00787, 0.00855, 0.00906, 0.00958, 0.01026, 0.01094, 0.01163, 0.01231,
        0.01300, 0.01368, 0.01454, 0.01539, 0.01625,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 過飽和心優し注射 (Super Saturated Syringing) -- Hydro (HP scaling) --

const SIGEWINNE_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1177, 0.1265, 0.1354, 0.1471, 0.1560, 0.1648, 0.1766, 0.1883, 0.2001, 0.2119, 0.2236,
        0.2354, 0.2501, 0.2648, 0.2796,
    ],
    dynamic_bonus: None,
};

pub const SIGEWINNE: CharacterData = CharacterData {
    id: "sigewinne",
    name: "Sigewinne",
    element: Element::Hydro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [
        1039.00, 2695.00, 3586.00, 5366.00, 5999.00, 6902.00, 7747.00, 8659.00, 9292.00, 10213.00,
        10846.00, 11777.00, 12410.00, 13348.00, 13348.00, 13881.92, // Lv95/Lv95+/Lv100
        13881.92, // Lv95/Lv95+/Lv100
        14415.84, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        14.99, 38.88, 51.73, 77.41, 86.54, 99.57, 111.74, 124.90, 134.03, 147.31, 156.44, 169.87,
        179.00, 192.54, 192.54, 200.24, // Lv95/Lv95+/Lv100
        200.24, // Lv95/Lv95+/Lv100
        207.94, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        38.89, 100.88, 134.22, 200.84, 224.53, 258.33, 289.92, 324.06, 347.76, 382.21, 405.91,
        440.75, 464.44, 499.56, 499.56, 519.54, // Lv95/Lv95+/Lv100
        519.54, // Lv95/Lv95+/Lv100
        539.52, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Hp(0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "ターゲットセラピー",
            hits: &[SIGEWINNE_NORMAL_1, SIGEWINNE_NORMAL_2, SIGEWINNE_NORMAL_3],
            charged: &[SIGEWINNE_AIMED, SIGEWINNE_AIMED_FULL, SIGEWINNE_MINI_BUBBLE],
            plunging: &[
                SIGEWINNE_PLUNGE,
                SIGEWINNE_PLUNGE_LOW,
                SIGEWINNE_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "ぴょんぴょん水療法",
            scalings: &[SIGEWINNE_SKILL_BUBBLE, SIGEWINNE_SKILL_BLADE],
        },
        elemental_burst: TalentData {
            name: "過飽和心優し注射",
            scalings: &[SIGEWINNE_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
