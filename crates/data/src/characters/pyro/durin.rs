use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Durin — 5★ Pyro Sword (Mondstadt)
// Source: Honey Impact (gensh.honeyhunterworld.com) 2026-03-30
// Normal Attack: Radiant Wingslash
// Elemental Skill: Binary Form: Convergence and Division
// Elemental Burst: Principle of Purity / Principle of Darkness
// =============================================================================

// --- Normal Attack: Radiant Wingslash ---

const DURIN_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4565, 0.4937, 0.5308, 0.5839, 0.6211, 0.6635, 0.7219, 0.7803, 0.8387, 0.9024, 0.9661,
        1.0298, 1.0935, 1.1572, 1.2209,
    ],
    dynamic_bonus: None,
};

const DURIN_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4100, 0.4434, 0.4768, 0.5245, 0.5579, 0.5960, 0.6484, 0.7009, 0.7533, 0.8106, 0.8678,
        0.9250, 0.9822, 1.0394, 1.0966,
    ],
    dynamic_bonus: None,
};

const DURIN_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2916, 0.3154, 0.3391, 0.3730, 0.3967, 0.4239, 0.4612, 0.4985, 0.5358, 0.5765, 0.6171,
        0.6578, 0.6985, 0.7392, 0.7799,
    ],
    dynamic_bonus: None,
};

const DURIN_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7115, 0.7694, 0.8274, 0.9101, 0.9680, 1.0342, 1.1252, 1.2162, 1.3072, 1.4065, 1.5058,
        1.6051, 1.7043, 1.8036, 1.9029,
    ],
    dynamic_bonus: None,
};

const DURIN_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1343, 1.2267, 1.3190, 1.4509, 1.5432, 1.6487, 1.7938, 1.9389, 2.0840, 2.2423, 2.4006,
        2.5589, 2.7171, 2.8754, 3.0337,
    ],
    dynamic_bonus: None,
};

const DURIN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
};

const DURIN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
};

const DURIN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
};

// --- Elemental Skill: Binary Form: Convergence and Division ---

const DURIN_SKILL_PURITY: TalentScaling = TalentScaling {
    name: "白炎変換ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.0560, 1.1352, 1.2144, 1.3200, 1.3992, 1.4784, 1.5840, 1.6896, 1.7952, 1.9008, 2.0064,
        2.1120, 2.2440, 2.3760, 2.5080,
    ],
    dynamic_bonus: None,
};

const DURIN_SKILL_DARK_1: TalentScaling = TalentScaling {
    name: "暗闘1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.7224, 0.7766, 0.8308, 0.9030, 0.9572, 1.0114, 1.0836, 1.1558, 1.2281, 1.3003, 1.3726,
        1.4448, 1.5351, 1.6254, 1.7157,
    ],
    dynamic_bonus: None,
};

const DURIN_SKILL_DARK_2: TalentScaling = TalentScaling {
    name: "暗闘2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5320, 0.5719, 0.6118, 0.6650, 0.7049, 0.7448, 0.7980, 0.8512, 0.9044, 0.9576, 1.0108,
        1.0640, 1.1305, 1.1970, 1.2635,
    ],
    dynamic_bonus: None,
};

const DURIN_SKILL_DARK_3: TalentScaling = TalentScaling {
    name: "暗闘3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.6464, 0.6949, 0.7434, 0.8080, 0.8565, 0.9050, 0.9696, 1.0342, 1.0989, 1.1635, 1.2282,
        1.2928, 1.3736, 1.4544, 1.5352,
    ],
    dynamic_bonus: None,
};

// --- Elemental Burst: Principle of Purity / Principle of Darkness ---

const DURIN_BURST_PURITY_1: TalentScaling = TalentScaling {
    name: "白炎1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1896, 1.2788, 1.3680, 1.4870, 1.5762, 1.6654, 1.7844, 1.9034, 2.0223, 2.1413, 2.2602,
        2.3792, 2.5279, 2.6766, 2.8253,
    ],
    dynamic_bonus: None,
};

const DURIN_BURST_PURITY_2: TalentScaling = TalentScaling {
    name: "白炎2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9640, 1.0363, 1.1086, 1.2050, 1.2773, 1.3496, 1.4460, 1.5424, 1.6388, 1.7352, 1.8316,
        1.9280, 2.0485, 2.1690, 2.2895,
    ],
    dynamic_bonus: None,
};

const DURIN_BURST_PURITY_3: TalentScaling = TalentScaling {
    name: "白炎3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1184, 1.2023, 1.2862, 1.3980, 1.4819, 1.5658, 1.6776, 1.7894, 1.9013, 2.0131, 2.1250,
        2.2368, 2.3766, 2.5164, 2.6562,
    ],
    dynamic_bonus: None,
};

const DURIN_BURST_DARK_1: TalentScaling = TalentScaling {
    name: "暗闘1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2544, 1.3485, 1.4426, 1.5680, 1.6621, 1.7562, 1.8816, 2.0070, 2.1325, 2.2579, 2.3834,
        2.5088, 2.6656, 2.8224, 2.9792,
    ],
    dynamic_bonus: None,
};

const DURIN_BURST_DARK_2: TalentScaling = TalentScaling {
    name: "暗闘2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.0176, 1.0939, 1.1702, 1.2720, 1.3483, 1.4246, 1.5264, 1.6282, 1.7299, 1.8317, 1.9334,
        2.0352, 2.1624, 2.2896, 2.4168,
    ],
    dynamic_bonus: None,
};

const DURIN_BURST_DARK_3: TalentScaling = TalentScaling {
    name: "暗闘3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1184, 1.2023, 1.2862, 1.3980, 1.4819, 1.5658, 1.6776, 1.7894, 1.9013, 2.0131, 2.1250,
        2.2368, 2.3766, 2.5164, 2.6562,
    ],
    dynamic_bonus: None,
};

const DURIN_BURST_WHITE_DRAGON: TalentScaling = TalentScaling {
    name: "白炎龍ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9464, 1.0174, 1.0884, 1.1830, 1.2540, 1.3250, 1.4196, 1.5142, 1.6089, 1.7035, 1.7982,
        1.8928, 2.0111, 2.1294, 2.2477,
    ],
    dynamic_bonus: None,
};

const DURIN_BURST_DARK_DRAGON: TalentScaling = TalentScaling {
    name: "暗闘龍ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2984, 1.3958, 1.4932, 1.6230, 1.7204, 1.8178, 1.9476, 2.0774, 2.2073, 2.3371, 2.4670,
        2.5968, 2.7591, 2.9214, 3.0837,
    ],
    dynamic_bonus: None,
};

// -- Character Data --

pub const DURIN: CharacterData = CharacterData {
    id: "durin",
    name: "Durin",
    element: Element::Pyro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [
        968.00, 2510.00, 3340.00, 4997.00, 5587.00, 6428.00, 7214.00, 8063.00, 8653.00, 9510.00,
        10099.00, 10966.00, 11556.00, 12430.00, 12430.00, 12927.20, // Lv95/Lv95+/Lv100
        12927.20, // Lv95/Lv95+/Lv100
        13424.40, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        27.00, 70.04, 93.18, 139.43, 155.88, 179.34, 201.27, 224.98, 241.42, 265.34, 281.79,
        305.98, 322.43, 346.81, 346.81, 360.68, // Lv95/Lv95+/Lv100
        360.68, // Lv95/Lv95+/Lv100
        374.55, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        64.02, 166.06, 220.95, 330.62, 369.62, 425.25, 477.25, 533.46, 572.46, 629.18, 668.18,
        725.54, 764.54, 822.35, 822.35, 855.24, // Lv95/Lv95+/Lv100
        855.24, // Lv95/Lv95+/Lv100
        888.14, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "Radiant Wingslash",
            hits: &[DURIN_NA_HIT1, DURIN_NA_HIT2, DURIN_NA_HIT3, DURIN_NA_HIT4],
            charged: &[DURIN_CHARGED],
            plunging: &[DURIN_PLUNGE, DURIN_PLUNGE_LOW, DURIN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "Binary Form: Convergence and Division",
            scalings: &[
                DURIN_SKILL_PURITY,
                DURIN_SKILL_DARK_1,
                DURIN_SKILL_DARK_2,
                DURIN_SKILL_DARK_3,
            ],
        },
        elemental_burst: TalentData {
            name: "Principle of Purity",
            scalings: &[
                DURIN_BURST_PURITY_1,
                DURIN_BURST_PURITY_2,
                DURIN_BURST_PURITY_3,
                DURIN_BURST_DARK_1,
                DURIN_BURST_DARK_2,
                DURIN_BURST_DARK_3,
                DURIN_BURST_WHITE_DRAGON,
                DURIN_BURST_DARK_DRAGON,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
