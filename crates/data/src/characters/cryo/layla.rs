use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Sword of the Radiant Path -- Physical --

const LAYLA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5120, 0.5538, 0.5955, 0.6551, 0.6968, 0.7444, 0.8098, 0.8753, 0.9407, 1.0123, 1.0838,
        1.1553, 1.2269, 1.2984, 1.3700,
    ],
    dynamic_bonus: None,
};

const LAYLA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4848, 0.5243, 0.5639, 0.6203, 0.6598, 0.7049, 0.7669, 0.8290, 0.8910, 0.9587, 1.0264,
        1.0940, 1.1617, 1.2293, 1.2970,
    ],
    dynamic_bonus: None,
};

const LAYLA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7298, 0.7892, 0.8486, 0.9335, 0.9929, 1.0608, 1.1541, 1.2475, 1.3408, 1.4427, 1.5445,
        1.6464, 1.7480, 1.8500, 1.9520,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Physical --

const LAYLA_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4773, 0.5162, 0.5551, 0.6106, 0.6495, 0.6939, 0.7549, 0.8159, 0.8769, 0.9436, 1.0103,
        1.0770, 1.1437, 1.2104, 1.2771,
    ],
    dynamic_bonus: None,
};

const LAYLA_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5254, 0.5682, 0.6110, 0.6721, 0.7149, 0.7638, 0.8310, 0.8981, 0.9653, 1.0387, 1.1121,
        1.1855, 1.2589, 1.3200, 1.4050,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const LAYLA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
};

const LAYLA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
};

const LAYLA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: Nights of Formal Focus -- Cryo --

const LAYLA_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Cryo),
    values: [
        0.1280, 0.1376, 0.1472, 0.1600, 0.1696, 0.1792, 0.1920, 0.2048, 0.2176, 0.2304, 0.2432,
        0.2560, 0.2720, 0.2880, 0.3040,
    ],
    dynamic_bonus: None,
};

const LAYLA_SKILL_STAR: TalentScaling = TalentScaling {
    name: "流星ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Cryo),
    values: [
        0.1472, 0.1582, 0.1693, 0.1840, 0.1950, 0.2061, 0.2208, 0.2355, 0.2502, 0.2650, 0.2797,
        0.2944, 0.3128, 0.3312, 0.3496,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: Dream of the Star-Stream Shaker -- Cryo --

const LAYLA_BURST: TalentScaling = TalentScaling {
    name: "星光弾ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Cryo),
    values: [
        0.0465, 0.0500, 0.0535, 0.0581, 0.0616, 0.0651, 0.0697, 0.0744, 0.0790, 0.0837, 0.0883,
        0.0930, 0.0988, 0.1046, 0.1104,
    ],
    dynamic_bonus: None,
};

pub const LAYLA: CharacterData = CharacterData {
    id: "layla",
    name: "Layla",
    element: Element::Cryo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Sumeru,
    base_hp: [
        930.00, 2389.00, 3084.00, 4619.00, 5113.00, 5881.00, 6540.00, 7308.00, 7801.00, 8569.00,
        9062.00, 9831.00, 10324.00, 11092.00, 11092.00, 11535.68, // Lv95/Lv95+/Lv100
        11535.68, // Lv95/Lv95+/Lv100
        11979.36, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        18.16, 46.66, 60.23, 90.22, 99.86, 114.87, 127.72, 142.73, 152.37, 167.35, 177.00, 192.00,
        201.64, 216.65, 216.65, 225.32, // Lv95/Lv95+/Lv100
        225.32, // Lv95/Lv95+/Lv100
        233.98, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        54.94, 141.13, 182.17, 272.87, 302.03, 347.41, 386.29, 431.66, 460.82, 506.15, 535.31,
        580.68, 609.84, 655.22, 655.22, 681.43, // Lv95/Lv95+/Lv100
        681.43, // Lv95/Lv95+/Lv100
        707.64, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "熠輝の剣",
            hits: &[LAYLA_NORMAL_1, LAYLA_NORMAL_2, LAYLA_NORMAL_3],
            charged: &[LAYLA_CHARGED_1, LAYLA_CHARGED_2],
            plunging: &[LAYLA_PLUNGE, LAYLA_PLUNGE_LOW, LAYLA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "格式の夜",
            scalings: &[LAYLA_SKILL, LAYLA_SKILL_STAR],
        },
        elemental_burst: TalentData {
            name: "星流揺動の夢",
            scalings: &[LAYLA_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Mika
