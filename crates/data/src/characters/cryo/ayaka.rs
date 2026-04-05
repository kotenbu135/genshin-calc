use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Kamisato Art: Kabuki -- Physical --

const AYAKA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4573, 0.4945, 0.5318, 0.5850, 0.6222, 0.6647, 0.7231, 0.7815, 0.8398, 0.9039, 0.9679,
        1.0319, 1.0960, 1.1600, 1.2240,
    ],
    dynamic_bonus: None,
};

const AYAKA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4868, 0.5264, 0.5660, 0.6226, 0.6622, 0.7075, 0.7698, 0.8320, 0.8943, 0.9622, 1.0301,
        1.0981, 1.1660, 1.2340, 1.3019,
    ],
    dynamic_bonus: None,
};

const AYAKA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6262, 0.6773, 0.7284, 0.8012, 0.8524, 0.9105, 0.9904, 1.0703, 1.1502, 1.2380, 1.3258,
        1.4136, 1.5014, 1.5892, 1.6770,
    ],
    dynamic_bonus: None,
};

const AYAKA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (×3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2261, 0.2445, 0.2630, 0.2893, 0.3077, 0.3287, 0.3576, 0.3865, 0.4154, 0.4470, 0.4787,
        0.5103, 0.5419, 0.5736, 0.6052,
    ],
    dynamic_bonus: None,
};

const AYAKA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7818, 0.8455, 0.9093, 1.0002, 1.0639, 1.1366, 1.2365, 1.3364, 1.4363, 1.5456, 1.6549,
        1.7642, 1.8735, 1.9828, 2.0921,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Physical --

const AYAKA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ (×3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5513, 0.5961, 0.6410, 0.7051, 0.7499, 0.8012, 0.8716, 0.9420, 1.0124, 1.0895, 1.1666,
        1.2437, 1.3208, 1.3979, 1.4750,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const AYAKA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
};

const AYAKA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5271, 2.7055,
        2.8840, 3.0624, 3.2409, 3.4189,
    ],
    dynamic_bonus: None,
};

const AYAKA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: Kamisato Art: Hyouka -- Cryo --

const AYAKA_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        2.3920, 2.5714, 2.7508, 2.9900, 3.1694, 3.3488, 3.5880, 3.8272, 4.0664, 4.3056, 4.5448,
        4.7840, 5.0830, 5.3820, 5.6810,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: Kamisato Art: Soumetsu -- Cryo --

const AYAKA_BURST_CUTTING: TalentScaling = TalentScaling {
    name: "切り裂きダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.1230, 1.2072, 1.2915, 1.4037, 1.4880, 1.5722, 1.6845, 1.7967, 1.9090, 2.0214, 2.1336,
        2.2459, 2.3863, 2.5267, 2.6671,
    ],
    dynamic_bonus: None,
};

const AYAKA_BURST_BLOOM: TalentScaling = TalentScaling {
    name: "開花ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.6845, 1.8109, 1.9373, 2.1056, 2.2320, 2.3584, 2.5267, 2.6950, 2.8634, 3.0321, 3.2005,
        3.3689, 3.5794, 3.7900, 4.0006,
    ],
    dynamic_bonus: None,
};

pub const AYAKA: CharacterData = CharacterData {
    id: "ayaka",
    name: "Kamisato Ayaka",
    element: Element::Cryo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [
        1001.00, 2597.00, 3455.00, 5170.00, 5779.00, 6649.00, 7462.00, 8341.00, 8951.00, 9838.00,
        10448.00, 11345.00, 11954.00, 12858.00, 12858.00, 13372.32, // Lv95/Lv95+/Lv100
        13372.32, // Lv95/Lv95+/Lv100
        13886.64, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        26.63, 69.07, 91.90, 137.51, 153.73, 176.87, 198.49, 221.87, 238.09, 261.68, 277.90,
        301.76, 317.98, 342.03, 342.03, 355.71, // Lv95/Lv95+/Lv100
        355.71, // Lv95/Lv95+/Lv100
        369.39, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        61.03, 158.30, 210.63, 315.17, 352.35, 405.38, 454.95, 508.53, 545.71, 599.78, 636.96,
        691.64, 728.82, 783.93, 783.93, 815.29, // Lv95/Lv95+/Lv100
        815.29, // Lv95/Lv95+/Lv100
        846.64, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "神里流・傾",
            hits: &[
                AYAKA_NORMAL_1,
                AYAKA_NORMAL_2,
                AYAKA_NORMAL_3,
                AYAKA_NORMAL_4,
                AYAKA_NORMAL_5,
            ],
            charged: &[AYAKA_CHARGED],
            plunging: &[AYAKA_PLUNGE, AYAKA_PLUNGE_LOW, AYAKA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "神里流・氷華",
            scalings: &[AYAKA_SKILL],
        },
        elemental_burst: TalentData {
            name: "神里流・霜滅",
            scalings: &[AYAKA_BURST_CUTTING, AYAKA_BURST_BLOOM],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Charlotte
