use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Cool-Color Capture -- Cryo (Catalyst) --

const CHARLOTTE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.4985, 0.5358, 0.5732, 0.6231, 0.6605, 0.6978, 0.7477, 0.7975, 0.8474, 0.8972, 0.9471,
        0.9969, 1.0592, 1.1215, 1.1838,
    ],
};

const CHARLOTTE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.4338, 0.4663, 0.4988, 0.5422, 0.5747, 0.6073, 0.6506, 0.6940, 0.7374, 0.7808, 0.8241,
        0.8675, 0.9217, 0.9759, 1.0302,
    ],
};

const CHARLOTTE_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.6460, 0.6945, 0.7429, 0.8075, 0.8560, 0.9044, 0.9690, 1.0336, 1.0982, 1.1628, 1.2274,
        1.2920, 1.3728, 1.4535, 1.5343,
    ],
};

// -- Charged Attack -- Cryo (Catalyst) --

const CHARLOTTE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.0051, 1.0805, 1.1559, 1.2564, 1.3318, 1.4072, 1.5077, 1.6082, 1.7087, 1.8092, 1.9097,
        2.0102, 2.1359, 2.2615, 2.3872,
    ],
};

// -- Plunging Attack -- Cryo (Catalyst) --

const CHARLOTTE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const CHARLOTTE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const CHARLOTTE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: Framing: Freezing Point Composition -- Cryo --

const CHARLOTTE_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "撮影ダメージ (短押し)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.6720, 0.7224, 0.7728, 0.8400, 0.8904, 0.9408, 1.0080, 1.0752, 1.1424, 1.2096, 1.2768,
        1.3440, 1.4280, 1.5120, 1.5960,
    ],
};

const CHARLOTTE_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "撮影ダメージ (長押し)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.3920, 1.4964, 1.6008, 1.7400, 1.8444, 1.9488, 2.0880, 2.2272, 2.3664, 2.5056, 2.6448,
        2.7840, 2.9580, 3.1320, 3.3060,
    ],
};

const CHARLOTTE_SKILL_MARK_SNAPPY: TalentScaling = TalentScaling {
    name: "スナップマークダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.3920, 0.4214, 0.4508, 0.4900, 0.5194, 0.5488, 0.5880, 0.6272, 0.6664, 0.7056, 0.7448,
        0.7840, 0.8330, 0.8820, 0.9310,
    ],
};

const CHARLOTTE_SKILL_MARK_FOCUSED: TalentScaling = TalentScaling {
    name: "集中マークダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.4060, 0.4365, 0.4669, 0.5075, 0.5380, 0.5684, 0.6090, 0.6496, 0.6902, 0.7308, 0.7714,
        0.8120, 0.8628, 0.9135, 0.9643,
    ],
};

// -- Elemental Burst: Still Photo: Comprehensive Confirmation -- Cryo --

const CHARLOTTE_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.7762, 0.8344, 0.8926, 0.9702, 1.0284, 1.0866, 1.1642, 1.2419, 1.3195, 1.3971, 1.4747,
        1.5523, 1.6493, 1.7464, 1.8434,
    ],
};

const CHARLOTTE_BURST_KAMERA: TalentScaling = TalentScaling {
    name: "カメラ継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.0647, 0.0695, 0.0744, 0.0809, 0.0857, 0.0906, 0.0970, 0.1035, 0.1100, 0.1164, 0.1229,
        0.1294, 0.1374, 0.1455, 0.1536,
    ],
};

pub const CHARLOTTE: CharacterData = CharacterData {
    id: "charlotte",
    name: "Charlotte",
    element: Element::Cryo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    region: Region::Fontaine,
    base_hp: [
        903.00, 2319.00, 2993.00, 4484.00, 4963.00, 5708.00, 6347.00, 7093.00, 7572.00, 8317.00,
        8796.00, 9541.00, 10021.00, 10766.00, 10766.00, 11196.64, // Lv95/Lv95+/Lv100
        11196.64, // Lv95/Lv95+/Lv100
        11627.28, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        14.51, 37.29, 48.13, 72.09, 79.79, 91.78, 102.05, 114.04, 121.75, 133.72, 141.42, 153.41,
        161.12, 173.10, 173.10, 180.02, // Lv95/Lv95+/Lv100
        180.02, // Lv95/Lv95+/Lv100
        186.95, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        45.78, 117.61, 151.81, 227.39, 251.69, 289.51, 321.91, 359.72, 384.02, 421.79, 446.09,
        483.90, 508.20, 546.02, 546.02, 567.86, // Lv95/Lv95+/Lv100
        567.86, // Lv95/Lv95+/Lv100
        589.70, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "クールカラーキャプチャー",
            hits: &[CHARLOTTE_NORMAL_1, CHARLOTTE_NORMAL_2, CHARLOTTE_NORMAL_3],
            charged: &[CHARLOTTE_CHARGED],
            plunging: &[
                CHARLOTTE_PLUNGE,
                CHARLOTTE_PLUNGE_LOW,
                CHARLOTTE_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "フレーミング・氷点構図",
            scalings: &[
                CHARLOTTE_SKILL_PRESS,
                CHARLOTTE_SKILL_HOLD,
                CHARLOTTE_SKILL_MARK_SNAPPY,
                CHARLOTTE_SKILL_MARK_FOCUSED,
            ],
        },
        elemental_burst: TalentData {
            name: "スチルフォト・総合確認",
            scalings: &[CHARLOTTE_BURST, CHARLOTTE_BURST_KAMERA],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Chongyun
