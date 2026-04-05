use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Arlecchino
// =============================================================================

// -- Normal Attack: 断頭への招待 (Invitation to a Beheading) -- Physical --

const ARLECCHINO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4750, 0.5137, 0.5523, 0.6076, 0.6462, 0.6904, 0.7512, 0.8119, 0.8727, 0.9390, 1.0052,
        1.0715, 1.1378, 1.2041, 1.2704,
    ],
    dynamic_bonus: None,
};

const ARLECCHINO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5211, 0.5635, 0.6059, 0.6665, 0.7089, 0.7574, 0.8240, 0.8906, 0.9573, 1.0300, 1.1027,
        1.1754, 1.2481, 1.3208, 1.3935,
    ],
    dynamic_bonus: None,
};

const ARLECCHINO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6539, 0.7071, 0.7603, 0.8363, 0.8896, 0.9504, 1.0340, 1.1176, 1.2013, 1.2925, 1.3837,
        1.4750, 1.5662, 1.6575, 1.7487,
    ],
    dynamic_bonus: None,
};

const ARLECCHINO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3715, 0.4017, 0.4319, 0.4751, 0.5053, 0.5399, 0.5874, 0.6349, 0.6824, 0.7343, 0.7861,
        0.8379, 0.8898, 0.9416, 0.9934,
    ],
    dynamic_bonus: None,
};

const ARLECCHINO_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6998, 0.7568, 0.8137, 0.8951, 0.9521, 1.0172, 1.1067, 1.1962, 1.2857, 1.3834, 1.4810,
        1.5787, 1.6763, 1.7740, 1.8716,
    ],
    dynamic_bonus: None,
};

const ARLECCHINO_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8538, 0.9233, 0.9928, 1.0920, 1.1615, 1.2410, 1.3502, 1.4594, 1.5686, 1.6877, 1.8068,
        1.9260, 2.0451, 2.1642, 2.2834,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Physical --

const ARLECCHINO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9082, 0.9821, 1.0560, 1.1616, 1.2355, 1.3200, 1.4362, 1.5523, 1.6685, 1.7952, 1.9219,
        2.0486, 2.1754, 2.3021, 2.4288,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const ARLECCHINO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
};

const ARLECCHINO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
};

const ARLECCHINO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: 万象灰燼 (All Is Ash) -- Pyro --

const ARLECCHINO_SKILL_SPIKE: TalentScaling = TalentScaling {
    name: "スパイクダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.1484, 0.1595, 0.1707, 0.1855, 0.1966, 0.2078, 0.2226, 0.2374, 0.2523, 0.2671, 0.2820,
        0.2968, 0.3154, 0.3339, 0.3525,
    ],
    dynamic_bonus: None,
};

const ARLECCHINO_SKILL_CLEAVE: TalentScaling = TalentScaling {
    name: "切り裂きダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.3356, 1.4358, 1.5359, 1.6695, 1.7697, 1.8698, 2.0034, 2.1370, 2.2705, 2.4041, 2.5376,
        2.6712, 2.8382, 3.0051, 3.1721,
    ],
    dynamic_bonus: None,
};

const ARLECCHINO_SKILL_DIRECTIVE: TalentScaling = TalentScaling {
    name: "血償の勅令ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.3180, 0.3419, 0.3657, 0.3975, 0.4214, 0.4452, 0.4770, 0.5088, 0.5406, 0.5724, 0.6042,
        0.6360, 0.6758, 0.7155, 0.7553,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 厄月昇り (Balemoon Rising) -- Pyro --

const ARLECCHINO_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        3.7040, 3.9818, 4.2596, 4.6300, 4.9078, 5.1856, 5.5560, 5.9264, 6.2968, 6.6672, 7.0376,
        7.4080, 7.8710, 8.3340, 8.7970,
    ],
    dynamic_bonus: None,
};

pub const ARLECCHINO: CharacterData = CharacterData {
    id: "arlecchino",
    name: "Arlecchino",
    element: Element::Pyro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Snezhnaya,
    base_hp: [
        1020.00, 2646.00, 3521.00, 5268.00, 5889.00, 6776.00, 7604.00, 8500.00, 9121.00, 10025.00,
        10647.00, 11561.00, 12182.00, 13103.00, 13103.00, 13627.12, // Lv95/Lv95+/Lv100
        13627.12, // Lv95/Lv95+/Lv100
        14151.24, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        26.63, 69.07, 91.90, 137.51, 153.73, 176.87, 198.49, 221.87, 238.09, 261.68, 277.90,
        301.76, 317.98, 342.03, 342.03, 355.71, // Lv95/Lv95+/Lv100
        355.71, // Lv95/Lv95+/Lv100
        369.39, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        59.53, 154.42, 205.47, 307.44, 343.71, 395.44, 443.80, 496.07, 532.34, 585.08, 621.35,
        674.69, 710.96, 764.71, 764.71, 795.30, // Lv95/Lv95+/Lv100
        795.30, // Lv95/Lv95+/Lv100
        825.89, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "断頭への招待",
            hits: &[
                ARLECCHINO_NORMAL_1,
                ARLECCHINO_NORMAL_2,
                ARLECCHINO_NORMAL_3,
                ARLECCHINO_NORMAL_4,
                ARLECCHINO_NORMAL_5,
                ARLECCHINO_NORMAL_6,
            ],
            charged: &[ARLECCHINO_CHARGED],
            plunging: &[
                ARLECCHINO_PLUNGE,
                ARLECCHINO_PLUNGE_LOW,
                ARLECCHINO_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "万象灰燼",
            scalings: &[
                ARLECCHINO_SKILL_SPIKE,
                ARLECCHINO_SKILL_CLEAVE,
                ARLECCHINO_SKILL_DIRECTIVE,
            ],
        },
        elemental_burst: TalentData {
            name: "厄月昇り",
            scalings: &[ARLECCHINO_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
