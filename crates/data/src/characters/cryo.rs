#![allow(clippy::approx_constant)]
use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Aloy
// =============================================================================

// -- Normal Attack: Rapid Fire -- Physical --

const ALOY_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2112, 0.2256, 0.2400, 0.2592, 0.2736, 0.2904, 0.3120, 0.3336, 0.3552, 0.3768, 0.3984,
        0.4200, 0.4416, 0.4632, 0.4848,
    ],
};

const ALOY_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2376, 0.2538, 0.2700, 0.2916, 0.3078, 0.3267, 0.3510, 0.3753, 0.3996, 0.4239, 0.4482,
        0.4725, 0.4968, 0.5211, 0.5454,
    ],
};

const ALOY_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4312, 0.4606, 0.4900, 0.5292, 0.5586, 0.5929, 0.6370, 0.6811, 0.7252, 0.7693, 0.8134,
        0.8575, 0.9016, 0.9457, 0.9898,
    ],
};

const ALOY_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5280, 0.5640, 0.6000, 0.6480, 0.6840, 0.7260, 0.7800, 0.8340, 0.8880, 0.9420, 0.9960,
        1.0500, 1.1040, 1.1580, 1.2120,
    ],
};

// -- Aimed Shot -- Cryo (charged) --

const ALOY_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const ALOY_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

// -- Plunging Attack -- Physical --

const ALOY_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const ALOY_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const ALOY_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: Frozen Wilds -- Cryo --

const ALOY_SKILL_BOMB: TalentScaling = TalentScaling {
    name: "凍結爆弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.7760, 1.9092, 2.0424, 2.2200, 2.3532, 2.4864, 2.6640, 2.8416, 3.0192, 3.1968, 3.3744,
        3.5520, 3.7740, 3.9960, 4.2180,
    ],
};

const ALOY_SKILL_BOMBLET: TalentScaling = TalentScaling {
    name: "チルウォーター爆弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.4000, 0.4300, 0.4600, 0.5000, 0.5300, 0.5600, 0.6000, 0.6400, 0.6800, 0.7200, 0.7600,
        0.8000, 0.8500, 0.9000, 0.9500,
    ],
};

// -- Elemental Burst: Prophecies of Dawn -- Cryo --

const ALOY_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        3.5920, 3.8614, 4.1308, 4.4900, 4.7594, 5.0288, 5.3880, 5.7472, 6.1064, 6.4656, 6.8248,
        7.1840, 7.6330, 8.0820, 8.5310,
    ],
};

pub const ALOY: CharacterData = CharacterData {
    id: "aloy",
    name: "Aloy",
    element: Element::Cryo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Other,
    base_hp: [848.0, 9616.0, 10133.0, 10899.0],
    base_atk: [18.0, 206.0, 217.0, 234.0],
    base_def: [53.0, 597.0, 629.0, 676.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Cryo, 0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "ラピッドファイア",
            hits: &[ALOY_NORMAL_1, ALOY_NORMAL_2, ALOY_NORMAL_3, ALOY_NORMAL_4],
            charged: &[ALOY_AIMED, ALOY_AIMED_FULL],
            plunging: &[ALOY_PLUNGE, ALOY_PLUNGE_LOW, ALOY_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "凍てつくワイルド",
            scalings: &[ALOY_SKILL_BOMB, ALOY_SKILL_BOMBLET],
        },
        elemental_burst: TalentData {
            name: "夜明けの予言",
            scalings: &[ALOY_BURST],
        },
    },
};

// =============================================================================
// Ayaka (Kamisato Ayaka)
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
};

const AYAKA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4868, 0.5264, 0.5660, 0.6226, 0.6622, 0.7075, 0.7698, 0.8320, 0.8943, 0.9622, 1.0301,
        1.0981, 1.1660, 1.2340, 1.3019,
    ],
};

const AYAKA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6262, 0.6773, 0.7284, 0.8012, 0.8524, 0.9105, 0.9904, 1.0703, 1.1502, 1.2380, 1.3258,
        1.4136, 1.5014, 1.5892, 1.6770,
    ],
};

const AYAKA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (×3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2261, 0.2445, 0.2630, 0.2893, 0.3077, 0.3287, 0.3576, 0.3865, 0.4154, 0.4470, 0.4787,
        0.5103, 0.5419, 0.5736, 0.6052,
    ],
};

const AYAKA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7818, 0.8455, 0.9093, 1.0002, 1.0639, 1.1366, 1.2365, 1.3364, 1.4363, 1.5456, 1.6549,
        1.7642, 1.8735, 1.9828, 2.0921,
    ],
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
};

const AYAKA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5271, 2.7055,
        2.8840, 3.0624, 3.2409, 3.4189,
    ],
};

const AYAKA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
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
};

const AYAKA_BURST_BLOOM: TalentScaling = TalentScaling {
    name: "開花ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.6845, 1.8109, 1.9373, 2.1056, 2.2320, 2.3584, 2.5267, 2.6950, 2.8634, 3.0321, 3.2005,
        3.3689, 3.5794, 3.7900, 4.0006,
    ],
};

pub const AYAKA: CharacterData = CharacterData {
    id: "ayaka",
    name: "Kamisato Ayaka",
    element: Element::Cryo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [1001.0, 11345.0, 11954.0, 12858.0],
    base_atk: [27.0, 302.0, 318.0, 342.0],
    base_def: [61.0, 692.0, 729.0, 784.0],
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
};

// =============================================================================
// Charlotte
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
    base_hp: [903.0, 9541.0, 10021.0, 10766.0],
    base_atk: [15.0, 153.0, 161.0, 173.0],
    base_def: [46.0, 484.0, 508.0, 546.0],
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
};

// =============================================================================
// Chongyun
// =============================================================================

// -- Normal Attack: Demonbane -- Physical --

const CHONGYUN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7000, 0.7570, 0.8140, 0.8954, 0.9524, 1.0175, 1.1070, 1.1966, 1.2861, 1.3838, 1.4815,
        1.5792, 1.6768, 1.7745, 1.8722,
    ],
};

const CHONGYUN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6312, 0.6826, 0.7340, 0.8074, 0.8588, 0.9175, 0.9982, 1.0790, 1.1597, 1.2478, 1.3359,
        1.4240, 1.5120, 1.6001, 1.6882,
    ],
};

const CHONGYUN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8032, 0.8686, 0.9340, 1.0274, 1.0928, 1.1675, 1.2702, 1.3730, 1.4757, 1.5878, 1.6999,
        1.8120, 1.9240, 2.0361, 2.1482,
    ],
};

const CHONGYUN_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0122, 1.0946, 1.1770, 1.2947, 1.3771, 1.4713, 1.6007, 1.7302, 1.8597, 2.0009, 2.1421,
        2.2834, 2.4246, 2.5659, 2.7071,
    ],
};

// -- Charged Attack -- Physical --

const CHONGYUN_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5629, 0.6087, 0.6545, 0.7199, 0.7657, 0.8181, 0.8901, 0.9621, 1.0341, 1.1126, 1.1912,
        1.2697, 1.3482, 1.4268, 1.5053,
    ],
};

const CHONGYUN_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0178, 1.1007, 1.1835, 1.3019, 1.3847, 1.4794, 1.6096, 1.7397, 1.8699, 2.0120, 2.1540,
        2.2960, 2.4380, 2.5800, 2.7221,
    ],
};

// -- Plunging Attack -- Physical --

const CHONGYUN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
};

const CHONGYUN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const CHONGYUN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9460, 3.1842, 3.4225, 3.6825, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
};

// -- Elemental Skill: Spirit Blade: Chonghua's Layered Frost -- Cryo --

const CHONGYUN_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.7204, 1.8494, 1.9785, 2.1505, 2.2795, 2.4086, 2.5806, 2.7526, 2.9247, 3.0967, 3.2688,
        3.4408, 3.6559, 3.8709, 4.0860,
    ],
};

// -- Elemental Burst: Spirit Blade: Cloud-Parting Star -- Cryo --

const CHONGYUN_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.4240, 1.5308, 1.6376, 1.7800, 1.8868, 1.9936, 2.1360, 2.2784, 2.4208, 2.5632, 2.7056,
        2.8480, 3.0260, 3.2040, 3.3820,
    ],
};

pub const CHONGYUN: CharacterData = CharacterData {
    id: "chongyun",
    name: "Chongyun",
    element: Element::Cryo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [921.0, 9734.0, 10223.0, 10984.0],
    base_atk: [19.0, 198.0, 208.0, 223.0],
    base_def: [54.0, 575.0, 603.0, 648.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "滅魔",
            hits: &[
                CHONGYUN_NORMAL_1,
                CHONGYUN_NORMAL_2,
                CHONGYUN_NORMAL_3,
                CHONGYUN_NORMAL_4,
            ],
            charged: &[CHONGYUN_CHARGED_SPINNING, CHONGYUN_CHARGED_FINAL],
            plunging: &[CHONGYUN_PLUNGE, CHONGYUN_PLUNGE_LOW, CHONGYUN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "霊刃・重華積霜",
            scalings: &[CHONGYUN_SKILL],
        },
        elemental_burst: TalentData {
            name: "霊刃・雲開星落",
            scalings: &[CHONGYUN_BURST],
        },
    },
};

// =============================================================================
// Citlali
// =============================================================================

// -- Normal Attack: Shadow-Stealing Spirit Vessel -- Cryo (Catalyst) --

const CITLALI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.4341, 0.4666, 0.4992, 0.5426, 0.5751, 0.6077, 0.6511, 0.6945, 0.7379, 0.7813, 0.8247,
        0.8681, 0.9224, 0.9767, 1.0309,
    ],
};

const CITLALI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.3881, 0.4172, 0.4464, 0.4852, 0.5143, 0.5434, 0.5822, 0.6210, 0.6598, 0.6986, 0.7375,
        0.7763, 0.8248, 0.8733, 0.9218,
    ],
};

const CITLALI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5377, 0.5780, 0.6184, 0.6721, 0.7125, 0.7528, 0.8066, 0.8603, 0.9141, 0.9679, 1.0217,
        1.0754, 1.1426, 1.2099, 1.2771,
    ],
};

// -- Charged Attack -- Cryo (Catalyst) --

const CITLALI_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.9920, 1.0664, 1.1408, 1.2400, 1.3144, 1.3888, 1.4880, 1.5872, 1.6864, 1.7856, 1.8848,
        1.9840, 2.1080, 2.2320, 2.3560,
    ],
};

// -- Plunging Attack -- Cryo (Catalyst) --

const CITLALI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const CITLALI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const CITLALI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: Dawnfrost Darkstar -- Cryo --

const CITLALI_SKILL_TZITZIMITL: TalentScaling = TalentScaling {
    name: "黒曜ツィツィミトルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.7296, 0.7843, 0.8390, 0.9120, 0.9667, 1.0214, 1.0944, 1.1674, 1.2403, 1.3133, 1.3862,
        1.4592, 1.5504, 1.6416, 1.7328,
    ],
};

const CITLALI_SKILL_STORM: TalentScaling = TalentScaling {
    name: "霜降嵐ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.1703, 0.1830, 0.1958, 0.2128, 0.2256, 0.2383, 0.2554, 0.2724, 0.2894, 0.3064, 0.3235,
        0.3405, 0.3618, 0.3830, 0.4043,
    ],
};

// -- Elemental Burst: Edict of Entwined Splendor -- Cryo --

const CITLALI_BURST_ICE_STORM: TalentScaling = TalentScaling {
    name: "氷嵐ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        5.3760, 5.7792, 6.1824, 6.7200, 7.1232, 7.5264, 8.0640, 8.6016, 9.1392, 9.6768, 10.2144,
        10.7520, 11.4240, 12.0960, 12.7680,
    ],
};

const CITLALI_BURST_SKULL: TalentScaling = TalentScaling {
    name: "霊脈頭蓋ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.3440, 1.4448, 1.5456, 1.6800, 1.7808, 1.8816, 2.0160, 2.1504, 2.2848, 2.4192, 2.5536,
        2.6880, 2.8560, 3.0240, 3.1920,
    ],
};

pub const CITLALI: CharacterData = CharacterData {
    id: "citlali",
    name: "Citlali",
    element: Element::Cryo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [906.0, 10264.0, 10816.0, 11634.0],
    base_atk: [10.0, 112.0, 118.0, 127.0],
    base_def: [59.0, 673.0, 710.0, 763.0],
    ascension_stat: AscensionStat::ElementalMastery(115.2),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "影盗みの霊器",
            hits: &[CITLALI_NORMAL_1, CITLALI_NORMAL_2, CITLALI_NORMAL_3],
            charged: &[CITLALI_CHARGED],
            plunging: &[CITLALI_PLUNGE, CITLALI_PLUNGE_LOW, CITLALI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "夜明霜暗星",
            scalings: &[CITLALI_SKILL_TZITZIMITL, CITLALI_SKILL_STORM],
        },
        elemental_burst: TalentData {
            name: "絢爛交織の勅令",
            scalings: &[CITLALI_BURST_ICE_STORM, CITLALI_BURST_SKULL],
        },
    },
};

// =============================================================================
// Diona
// =============================================================================

// -- Normal Attack: Kätzlein Style -- Physical --

const DIONA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3612, 0.3906, 0.4200, 0.4620, 0.4914, 0.5250, 0.5712, 0.6174, 0.6636, 0.7140, 0.7718,
        0.8397, 0.9076, 0.9755, 1.0496,
    ],
};

const DIONA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3354, 0.3627, 0.3900, 0.4290, 0.4563, 0.4875, 0.5304, 0.5733, 0.6162, 0.6630, 0.7166,
        0.7797, 0.8428, 0.9058, 0.9746,
    ],
};

const DIONA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4558, 0.4929, 0.5300, 0.5830, 0.6201, 0.6625, 0.7208, 0.7791, 0.8374, 0.9010, 0.9739,
        1.0596, 1.1453, 1.2310, 1.3245,
    ],
};

const DIONA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4300, 0.4650, 0.5000, 0.5500, 0.5850, 0.6250, 0.6800, 0.7350, 0.7900, 0.8500, 0.9188,
        0.9996, 1.0805, 1.1613, 1.2495,
    ],
};

const DIONA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5375, 0.5813, 0.6250, 0.6875, 0.7313, 0.7813, 0.8500, 0.9188, 0.9875, 1.0625, 1.1484,
        1.2495, 1.3506, 1.4516, 1.5619,
    ],
};

// -- Aimed Shot -- Cryo (charged) --

const DIONA_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9371,
        1.0196, 1.1021, 1.1845, 1.2745,
    ],
};

const DIONA_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3610,
        2.5296, 2.6982, 2.8669, 3.0355,
    ],
};

// -- Plunging Attack -- Physical --

const DIONA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const DIONA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const DIONA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: Icy Paws -- Cryo --

const DIONA_SKILL: TalentScaling = TalentScaling {
    name: "猫の爪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.4192, 0.4506, 0.4821, 0.5240, 0.5554, 0.5869, 0.6288, 0.6707, 0.7126, 0.7546, 0.7965,
        0.8384, 0.8908, 0.9432, 0.9956,
    ],
};

// -- Elemental Burst: Signature Mix -- Cryo --

const DIONA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.8000, 0.8600, 0.9200, 1.0000, 1.0600, 1.1200, 1.2000, 1.2800, 1.3600, 1.4400, 1.5200,
        1.6000, 1.7000, 1.8000, 1.9000,
    ],
};

const DIONA_BURST_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5264, 0.5659, 0.6054, 0.6580, 0.6975, 0.7370, 0.7896, 0.8422, 0.8949, 0.9475, 1.0002,
        1.0528, 1.1186, 1.1844, 1.2502,
    ],
};

pub const DIONA: CharacterData = CharacterData {
    id: "diona",
    name: "Diona",
    element: Element::Cryo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [802.0, 8481.0, 8907.0, 9570.0],
    base_atk: [18.0, 188.0, 198.0, 212.0],
    base_def: [50.0, 532.0, 559.0, 601.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Cryo, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "ケッツェンシュタイル",
            hits: &[
                DIONA_NORMAL_1,
                DIONA_NORMAL_2,
                DIONA_NORMAL_3,
                DIONA_NORMAL_4,
                DIONA_NORMAL_5,
            ],
            charged: &[DIONA_AIMED, DIONA_AIMED_FULL],
            plunging: &[DIONA_PLUNGE, DIONA_PLUNGE_LOW, DIONA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "猫の爪フリーズ",
            scalings: &[DIONA_SKILL],
        },
        elemental_burst: TalentData {
            name: "特製カクテル",
            scalings: &[DIONA_BURST, DIONA_BURST_DOT],
        },
    },
};

// =============================================================================
// Escoffier
// =============================================================================

// -- Normal Attack: Kitchen Skills -- Physical --

const ESCOFFIER_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5155, 0.5575, 0.5994, 0.6594, 0.7013, 0.7493, 0.8152, 0.8812, 0.9471, 1.0190, 1.0910,
        1.1629, 1.2348, 1.3068, 1.3787,
    ],
};

const ESCOFFIER_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4759, 0.5147, 0.5534, 0.6088, 0.6475, 0.6918, 0.7526, 0.8135, 0.8744, 0.9408, 1.0072,
        1.0736, 1.1400, 1.2064, 1.2728,
    ],
};

const ESCOFFIER_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3300, 0.3569, 0.3837, 0.4221, 0.4489, 0.4796, 0.5219, 0.5641, 0.6063, 0.6523, 0.6984,
        0.7444, 0.7905, 0.8365, 0.8825,
    ],
};

const ESCOFFIER_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4033, 0.4362, 0.4690, 0.5159, 0.5487, 0.5862, 0.6378, 0.6894, 0.7410, 0.7973, 0.8536,
        0.9098, 0.9661, 1.0224, 1.0787,
    ],
};

// -- Charged Attack -- Physical --

const ESCOFFIER_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1541, 1.2481, 1.3420, 1.4762, 1.5701, 1.6775, 1.8251, 1.9727, 2.1204, 2.2814, 2.4424,
        2.6035, 2.7645, 2.9256, 3.0866,
    ],
};

// -- Plunging Attack -- Physical --

const ESCOFFIER_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const ESCOFFIER_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const ESCOFFIER_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: Low-Temperature Cooking -- Cryo --

const ESCOFFIER_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5040, 0.5418, 0.5796, 0.6300, 0.6678, 0.7056, 0.7560, 0.8064, 0.8568, 0.9072, 0.9576,
        1.0080, 1.0710, 1.1340, 1.1970,
    ],
};

const ESCOFFIER_SKILL_PARFAIT: TalentScaling = TalentScaling {
    name: "フロスティパフェダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2000, 1.2900, 1.3800, 1.5000, 1.5900, 1.6800, 1.8000, 1.9200, 2.0400, 2.1600, 2.2800,
        2.4000, 2.5500, 2.7000, 2.8500,
    ],
};

const ESCOFFIER_SKILL_SURGING: TalentScaling = TalentScaling {
    name: "サージングブレードダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.3360, 0.3612, 0.3864, 0.4200, 0.4452, 0.4704, 0.5040, 0.5376, 0.5712, 0.6048, 0.6384,
        0.6720, 0.7140, 0.7560, 0.7980,
    ],
};

// -- Elemental Burst: Scoring Cuts -- Cryo --

const ESCOFFIER_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        5.9280, 6.3726, 6.8172, 7.4100, 7.8546, 8.2992, 8.8920, 9.4848, 10.0776, 10.6704, 11.2632,
        11.8560, 12.5970, 13.3380, 14.0790,
    ],
};

pub const ESCOFFIER: CharacterData = CharacterData {
    id: "escoffier",
    name: "Escoffier",
    element: Element::Cryo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [1039.0, 11777.0, 12410.0, 13348.0],
    base_atk: [27.0, 306.0, 322.0, 347.0],
    base_def: [57.0, 646.0, 680.0, 732.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "厨技",
            hits: &[
                ESCOFFIER_NORMAL_1,
                ESCOFFIER_NORMAL_2,
                ESCOFFIER_NORMAL_3A,
                ESCOFFIER_NORMAL_3B,
            ],
            charged: &[ESCOFFIER_CHARGED],
            plunging: &[
                ESCOFFIER_PLUNGE,
                ESCOFFIER_PLUNGE_LOW,
                ESCOFFIER_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "低温調理",
            scalings: &[
                ESCOFFIER_SKILL,
                ESCOFFIER_SKILL_PARFAIT,
                ESCOFFIER_SKILL_SURGING,
            ],
        },
        elemental_burst: TalentData {
            name: "採点の一刀",
            scalings: &[ESCOFFIER_BURST],
        },
    },
};

// =============================================================================
// Eula
// =============================================================================

// -- Normal Attack: Favonius Bladework - Edel -- Physical --

const EULA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8973, 0.9704, 1.0434, 1.1477, 1.2208, 1.3043, 1.4190, 1.5338, 1.6486, 1.7738, 1.9172,
        2.0860, 2.2547, 2.4234, 2.6075,
    ],
};

const EULA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9355, 1.0117, 1.0878, 1.1966, 1.2727, 1.3598, 1.4792, 1.5988, 1.7183, 1.8490, 1.9988,
        2.1747, 2.3507, 2.5265, 2.7184,
    ],
};

const EULA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5680, 0.6142, 0.6605, 0.7265, 0.7727, 0.8256, 0.8982, 0.9709, 1.0435, 1.1228, 1.2136,
        1.3204, 1.4272, 1.5340, 1.6505,
    ],
};

const EULA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1264, 1.2181, 1.3098, 1.4408, 1.5325, 1.6373, 1.7813, 1.9254, 2.0695, 2.2267, 2.4068,
        2.6186, 2.8304, 3.0421, 3.2732,
    ],
};

const EULA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7183, 0.7768, 0.8353, 0.9188, 0.9773, 1.0441, 1.1360, 1.2278, 1.3197, 1.4199, 1.5348,
        1.6699, 1.8049, 1.9400, 2.0874,
    ],
};

// -- Charged Attack -- Physical --

const EULA_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6880, 0.7440, 0.8000, 0.8800, 0.9360, 1.0000, 1.0880, 1.1760, 1.2640, 1.3600, 1.4700,
        1.5992, 1.7285, 1.8577, 1.9990,
    ],
};

const EULA_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2440, 1.3452, 1.4465, 1.5912, 1.6924, 1.8081, 1.9673, 2.1265, 2.2857, 2.4593, 2.6580,
        2.8918, 3.1257, 3.3595, 3.6148,
    ],
};

// -- Plunging Attack -- Physical --

const EULA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
};

const EULA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const EULA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9460, 3.1842, 3.4225, 3.6825, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
};

// -- Elemental Skill: Icetide Vortex -- Cryo --

const EULA_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "短押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.4640, 1.5738, 1.6836, 1.8300, 1.9398, 2.0496, 2.1960, 2.3424, 2.4888, 2.6352, 2.7816,
        2.9280, 3.1110, 3.2940, 3.4770,
    ],
};

const EULA_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        2.4560, 2.6402, 2.8244, 3.0700, 3.2542, 3.4384, 3.6840, 3.9296, 4.1752, 4.4208, 4.6664,
        4.9120, 5.2190, 5.5260, 5.8330,
    ],
};

const EULA_SKILL_ICEWHIRL: TalentScaling = TalentScaling {
    name: "氷渦旋ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.9600, 1.0320, 1.1040, 1.2000, 1.2720, 1.3440, 1.4400, 1.5360, 1.6320, 1.7280, 1.8240,
        1.9200, 2.0400, 2.1600, 2.2800,
    ],
};

// -- Elemental Burst: Glacial Illumination -- Physical (Lightfall Sword) --

const EULA_BURST_SLASH: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        2.4560, 2.6402, 2.8244, 3.0700, 3.2542, 3.4384, 3.6840, 3.9296, 4.1752, 4.4208, 4.6664,
        4.9120, 5.2190, 5.5260, 5.8330,
    ],
};

const EULA_BURST_LIGHTFALL_BASE: TalentScaling = TalentScaling {
    name: "光臨の剣基礎ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        3.6705, 3.9692, 4.2680, 4.6948, 4.9936, 5.3350, 5.8045, 6.2740, 6.7434, 7.2556, 7.8425,
        8.5326, 9.2227, 9.9129, 10.6657,
    ],
};

const EULA_BURST_LIGHTFALL_STACK: TalentScaling = TalentScaling {
    name: "光臨の剣スタックダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7499, 0.8110, 0.8720, 0.9592, 1.0202, 1.0900, 1.1859, 1.2818, 1.3778, 1.4824, 1.6023,
        1.7433, 1.8843, 2.0253, 2.1791,
    ],
};

pub const EULA: CharacterData = CharacterData {
    id: "eula",
    name: "Eula",
    element: Element::Cryo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [1030.0, 11669.0, 12296.0, 13226.0],
    base_atk: [27.0, 302.0, 318.0, 342.0],
    base_def: [58.0, 662.0, 698.0, 751.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "西風剣術・エーデル",
            hits: &[
                EULA_NORMAL_1,
                EULA_NORMAL_2,
                EULA_NORMAL_3,
                EULA_NORMAL_4,
                EULA_NORMAL_5,
            ],
            charged: &[EULA_CHARGED_SPINNING, EULA_CHARGED_FINAL],
            plunging: &[EULA_PLUNGE, EULA_PLUNGE_LOW, EULA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "氷潮の渦",
            scalings: &[EULA_SKILL_PRESS, EULA_SKILL_HOLD, EULA_SKILL_ICEWHIRL],
        },
        elemental_burst: TalentData {
            name: "氷浪の光",
            scalings: &[
                EULA_BURST_SLASH,
                EULA_BURST_LIGHTFALL_BASE,
                EULA_BURST_LIGHTFALL_STACK,
            ],
        },
    },
};

// =============================================================================
// Freminet
// =============================================================================

// -- Normal Attack: Flowing Eddies -- Physical --

const FREMINET_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8424, 0.9109, 0.9795, 1.0775, 1.1460, 1.2244, 1.3321, 1.4399, 1.5476, 1.6652, 1.7827,
        1.9002, 2.0178, 2.1353, 2.2529,
    ],
};

const FREMINET_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8068, 0.8724, 0.9381, 1.0319, 1.0976, 1.1726, 1.2758, 1.3790, 1.4822, 1.5948, 1.7073,
        1.8199, 1.9325, 2.0450, 2.1576,
    ],
};

const FREMINET_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0190, 1.1020, 1.1849, 1.3034, 1.3864, 1.4812, 1.6115, 1.7418, 1.8722, 2.0144, 2.1566,
        2.2988, 2.4410, 2.5831, 2.7253,
    ],
};

const FREMINET_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2380, 1.3388, 1.4396, 1.5835, 1.6843, 1.7995, 1.9578, 2.1162, 2.2746, 2.4473, 2.6201,
        2.7928, 2.9656, 3.1383, 3.3111,
    ],
};

// -- Charged Attack -- Physical --

const FREMINET_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6252, 0.6761, 0.7270, 0.7997, 0.8506, 0.9088, 0.9887, 1.0687, 1.1487, 1.2359, 1.3231,
        1.4104, 1.4976, 1.5849, 1.6721,
    ],
};

const FREMINET_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1309, 1.2230, 1.3150, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933,
        2.5511, 2.7089, 2.8667, 3.0245,
    ],
};

// -- Plunging Attack -- Physical --

const FREMINET_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
};

const FREMINET_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const FREMINET_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9460, 3.1842, 3.4225, 3.6825, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
};

// -- Elemental Skill: Pressurized Floe -- Cryo --

const FREMINET_SKILL_THRUST: TalentScaling = TalentScaling {
    name: "上突きダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.8304, 0.8927, 0.9550, 1.0380, 1.1003, 1.1626, 1.2456, 1.3286, 1.4117, 1.4947, 1.5778,
        1.6608, 1.7646, 1.8684, 1.9722,
    ],
};

const FREMINET_SKILL_LEVEL4: TalentScaling = TalentScaling {
    name: "Lv4粉砕圧力ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        2.4344, 2.6170, 2.7996, 3.0430, 3.2256, 3.4082, 3.6516, 3.8950, 4.1385, 4.3819, 4.6254,
        4.8688, 5.1731, 5.4774, 5.7817,
    ],
};

// -- Elemental Burst: Shadowhunter's Ambush -- Cryo --

const FREMINET_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        3.1840, 3.4228, 3.6616, 3.9800, 4.2188, 4.4576, 4.7760, 5.0944, 5.4128, 5.7312, 6.0496,
        6.3680, 6.7660, 7.1640, 7.5620,
    ],
};

pub const FREMINET: CharacterData = CharacterData {
    id: "freminet",
    name: "Freminet",
    element: Element::Cryo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Fontaine,
    base_hp: [1012.0, 10698.0, 11235.0, 12071.0],
    base_atk: [21.0, 226.0, 237.0, 255.0],
    base_def: [59.0, 628.0, 659.0, 708.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "流れる渦",
            hits: &[
                FREMINET_NORMAL_1,
                FREMINET_NORMAL_2,
                FREMINET_NORMAL_3,
                FREMINET_NORMAL_4,
            ],
            charged: &[FREMINET_CHARGED_SPINNING, FREMINET_CHARGED_FINAL],
            plunging: &[FREMINET_PLUNGE, FREMINET_PLUNGE_LOW, FREMINET_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "浮氷の圧力",
            scalings: &[FREMINET_SKILL_THRUST, FREMINET_SKILL_LEVEL4],
        },
        elemental_burst: TalentData {
            name: "影の狩人の待ち伏せ",
            scalings: &[FREMINET_BURST],
        },
    },
};

// =============================================================================
// Ganyu
// =============================================================================

// -- Normal Attack: Liutian Archery -- Physical --

const GANYU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3173, 0.3432, 0.3690, 0.4059, 0.4317, 0.4613, 0.5018, 0.5424, 0.5830, 0.6273, 0.6780,
        0.7377, 0.7974, 0.8570, 0.9221,
    ],
};

const GANYU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3560, 0.3850, 0.4140, 0.4554, 0.4844, 0.5175, 0.5630, 0.6086, 0.6541, 0.7038, 0.7607,
        0.8277, 0.8946, 0.9616, 1.0346,
    ],
};

const GANYU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4549, 0.4920, 0.5290, 0.5819, 0.6189, 0.6613, 0.7194, 0.7776, 0.8358, 0.8993, 0.9720,
        1.0576, 1.1431, 1.2287, 1.3220,
    ],
};

const GANYU_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4549, 0.4920, 0.5290, 0.5819, 0.6189, 0.6613, 0.7194, 0.7776, 0.8358, 0.8993, 0.9720,
        1.0576, 1.1431, 1.2287, 1.3220,
    ],
};

const GANYU_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4825, 0.5217, 0.5610, 0.6171, 0.6564, 0.7013, 0.7630, 0.8247, 0.8864, 0.9537, 1.0308,
        1.1211, 1.2114, 1.3017, 1.4014,
    ],
};

const GANYU_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5762, 0.6231, 0.6700, 0.7370, 0.7839, 0.8375, 0.9112, 0.9849, 1.0586, 1.1390, 1.2311,
        1.3395, 1.4478, 1.5561, 1.6743,
    ],
};

// -- Aimed Shot & Frostflake Arrow -- Cryo --

const GANYU_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const GANYU_AIMED_CHARGE1: TalentScaling = TalentScaling {
    name: "1段チャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

const GANYU_FROSTFLAKE: TalentScaling = TalentScaling {
    name: "霜華の矢ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2800, 1.3760, 1.4720, 1.6000, 1.6960, 1.7920, 1.9200, 2.0480, 2.1760, 2.3040, 2.4320,
        2.5600, 2.7200, 2.8800, 3.0400,
    ],
};

const GANYU_FROSTFLAKE_BLOOM: TalentScaling = TalentScaling {
    name: "霜華の矢・霜華満開ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        2.1760, 2.3392, 2.5024, 2.7200, 2.8832, 3.0464, 3.2640, 3.4816, 3.6992, 3.9168, 4.1344,
        4.3520, 4.6240, 4.8960, 5.1680,
    ],
};

// -- Plunging Attack -- Physical --

const GANYU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const GANYU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const GANYU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8155, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: Trail of the Qilin -- Cryo --

const GANYU_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.3200, 1.4190, 1.5180, 1.6500, 1.7490, 1.8480, 1.9800, 2.1120, 2.2440, 2.3760, 2.5080,
        2.6400, 2.8050, 2.9700, 3.1350,
    ],
};

// -- Elemental Burst: Celestial Shower -- Cryo --

const GANYU_BURST: TalentScaling = TalentScaling {
    name: "氷柱ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.7027, 0.7554, 0.8081, 0.8784, 0.9311, 0.9838, 1.0541, 1.1244, 1.1946, 1.2649, 1.3352,
        1.4054, 1.4933, 1.5811, 1.6690,
    ],
};

pub const GANYU: CharacterData = CharacterData {
    id: "ganyu",
    name: "Ganyu",
    element: Element::Cryo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [763.0, 8643.0, 9108.0, 9797.0],
    base_atk: [26.0, 295.0, 311.0, 335.0],
    base_def: [49.0, 556.0, 586.0, 630.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "流天射術",
            hits: &[
                GANYU_NORMAL_1,
                GANYU_NORMAL_2,
                GANYU_NORMAL_3,
                GANYU_NORMAL_4,
                GANYU_NORMAL_5,
                GANYU_NORMAL_6,
            ],
            charged: &[
                GANYU_AIMED,
                GANYU_AIMED_CHARGE1,
                GANYU_FROSTFLAKE,
                GANYU_FROSTFLAKE_BLOOM,
            ],
            plunging: &[GANYU_PLUNGE, GANYU_PLUNGE_LOW, GANYU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "山雀の仮道",
            scalings: &[GANYU_SKILL],
        },
        elemental_burst: TalentData {
            name: "降魔の天華",
            scalings: &[GANYU_BURST],
        },
    },
};

// =============================================================================
// Kaeya
// =============================================================================

// -- Normal Attack: Ceremonial Bladework -- Physical --

const KAEYA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5375, 0.5813, 0.6250, 0.6875, 0.7313, 0.7813, 0.8500, 0.9188, 0.9875, 1.0625, 1.1484,
        1.2495, 1.3506, 1.4516, 1.5619,
    ],
};

const KAEYA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5169, 0.5589, 0.6010, 0.6611, 0.7032, 0.7513, 0.8174, 0.8835, 0.9496, 1.0217, 1.1043,
        1.2015, 1.2987, 1.3959, 1.5019,
    ],
};

const KAEYA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6527, 0.7059, 0.7590, 0.8349, 0.8880, 0.9488, 1.0322, 1.1157, 1.1992, 1.2903, 1.3947,
        1.5174, 1.6401, 1.7629, 1.8967,
    ],
};

const KAEYA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7086, 0.7663, 0.8240, 0.9064, 0.9641, 1.0300, 1.1206, 1.2113, 1.3019, 1.4008, 1.5141,
        1.6473, 1.7806, 1.9138, 2.0592,
    ],
};

const KAEYA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8824, 0.9542, 1.0260, 1.1286, 1.2004, 1.2825, 1.3954, 1.5082, 1.6211, 1.7442, 1.8853,
        2.0512, 2.2171, 2.3830, 2.5640,
    ],
};

// -- Charged Attack -- Physical --

const KAEYA_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5504, 0.5952, 0.6400, 0.7040, 0.7488, 0.8000, 0.8704, 0.9408, 1.0112, 1.0880, 1.1760,
        1.2795, 1.3830, 1.4865, 1.5994,
    ],
};

const KAEYA_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7310, 0.7905, 0.8500, 0.9350, 0.9945, 1.0625, 1.1560, 1.2495, 1.3430, 1.4450, 1.5619,
        1.6993, 1.8368, 1.9742, 2.1242,
    ],
};

// -- Plunging Attack -- Physical --

const KAEYA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const KAEYA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const KAEYA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: Frostgnaw -- Cryo --

const KAEYA_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.9120, 2.0554, 2.1988, 2.3900, 2.5334, 2.6768, 2.8680, 3.0592, 3.2504, 3.4416, 3.6328,
        3.8240, 4.0630, 4.3020, 4.5410,
    ],
};

// -- Elemental Burst: Glacial Waltz -- Cryo --

const KAEYA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.7760, 0.8342, 0.8924, 0.9700, 1.0282, 1.0864, 1.1640, 1.2416, 1.3192, 1.3968, 1.4744,
        1.5520, 1.6490, 1.7460, 1.8430,
    ],
};

pub const KAEYA: CharacterData = CharacterData {
    id: "kaeya",
    name: "Kaeya",
    element: Element::Cryo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [976.0, 10312.0, 10830.0, 11636.0],
    base_atk: [19.0, 198.0, 208.0, 223.0],
    base_def: [66.0, 702.0, 737.0, 792.0],
    ascension_stat: AscensionStat::EnergyRecharge(0.2668),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "儀典剣術",
            hits: &[
                KAEYA_NORMAL_1,
                KAEYA_NORMAL_2,
                KAEYA_NORMAL_3,
                KAEYA_NORMAL_4,
                KAEYA_NORMAL_5,
            ],
            charged: &[KAEYA_CHARGED_1, KAEYA_CHARGED_2],
            plunging: &[KAEYA_PLUNGE, KAEYA_PLUNGE_LOW, KAEYA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "霜の噛みつき",
            scalings: &[KAEYA_SKILL],
        },
        elemental_burst: TalentData {
            name: "氷の輪舞",
            scalings: &[KAEYA_BURST],
        },
    },
};

// =============================================================================
// Layla
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
};

const LAYLA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4848, 0.5243, 0.5639, 0.6203, 0.6598, 0.7049, 0.7669, 0.8290, 0.8910, 0.9587, 1.0264,
        1.0940, 1.1617, 1.2293, 1.2970,
    ],
};

const LAYLA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7298, 0.7892, 0.8486, 0.9335, 0.9929, 1.0608, 1.1541, 1.2475, 1.3408, 1.4427, 1.5445,
        1.6464, 1.7500, 1.8500, 1.9520,
    ],
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
};

const LAYLA_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5254, 0.5682, 0.6110, 0.6721, 0.7149, 0.7638, 0.8310, 0.8981, 0.9653, 1.0387, 1.1121,
        1.1855, 1.2589, 1.3200, 1.4050,
    ],
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
};

const LAYLA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const LAYLA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
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
};

const LAYLA_SKILL_STAR: TalentScaling = TalentScaling {
    name: "流星ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Cryo),
    values: [
        0.1472, 0.1582, 0.1693, 0.1840, 0.1950, 0.2061, 0.2208, 0.2355, 0.2502, 0.2650, 0.2797,
        0.2944, 0.3128, 0.3312, 0.3496,
    ],
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
};

pub const LAYLA: CharacterData = CharacterData {
    id: "layla",
    name: "Layla",
    element: Element::Cryo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Sumeru,
    base_hp: [930.0, 9831.0, 10324.0, 11092.0],
    base_atk: [18.0, 192.0, 202.0, 217.0],
    base_def: [55.0, 581.0, 610.0, 655.0],
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
};

// =============================================================================
// Mika
// =============================================================================

// -- Normal Attack: Spear of Favonius - Arrow's Passage -- Physical --

const MIKA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4326, 0.4678, 0.5031, 0.5534, 0.5886, 0.6288, 0.6842, 0.7395, 0.7948, 0.8552, 0.9156,
        0.9759, 1.0363, 1.0967, 1.1570,
    ],
};

const MIKA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4150, 0.4488, 0.4826, 0.5308, 0.5646, 0.6032, 0.6563, 0.7094, 0.7625, 0.8204, 0.8783,
        0.9362, 0.9941, 1.0520, 1.1099,
    ],
};

const MIKA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5450, 0.5894, 0.6338, 0.6971, 0.7415, 0.7922, 0.8619, 0.9316, 1.0013, 1.0774, 1.1534,
        1.2295, 1.3055, 1.3816, 1.4576,
    ],
};

const MIKA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2761, 0.2986, 0.3211, 0.3532, 0.3757, 0.4014, 0.4367, 0.4720, 0.5073, 0.5459, 0.5844,
        0.6229, 0.6615, 0.7000, 0.7385,
    ],
};

const MIKA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7087, 0.7664, 0.8241, 0.9065, 0.9642, 1.0302, 1.1208, 1.2115, 1.3021, 1.4010, 1.4999,
        1.5988, 1.6977, 1.7966, 1.8955,
    ],
};

// -- Charged Attack -- Physical --

const MIKA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1275, 1.2192, 1.3110, 1.4421, 1.5339, 1.6388, 1.7830, 1.9272, 2.0714, 2.2287, 2.3860,
        2.5433, 2.7007, 2.8580, 3.0153,
    ],
};

// -- Plunging Attack -- Physical --

const MIKA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const MIKA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const MIKA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: Starfrost Swirl -- Cryo --

const MIKA_SKILL_ARROW: TalentScaling = TalentScaling {
    name: "霜流矢ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.6720, 0.7224, 0.7728, 0.8400, 0.8904, 0.9408, 1.0080, 1.0752, 1.1424, 1.2096, 1.2768,
        1.3440, 1.4280, 1.5120, 1.5960,
    ],
};

const MIKA_SKILL_FLARE: TalentScaling = TalentScaling {
    name: "氷星フレアダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.8400, 0.9030, 0.9660, 1.0500, 1.1130, 1.1760, 1.2600, 1.3440, 1.4280, 1.5120, 1.5960,
        1.6800, 1.7850, 1.8900, 1.9950,
    ],
};

const MIKA_SKILL_SHARD: TalentScaling = TalentScaling {
    name: "氷星の欠片ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.2520, 0.2709, 0.2898, 0.3150, 0.3339, 0.3528, 0.3780, 0.4032, 0.4284, 0.4536, 0.4788,
        0.5040, 0.5355, 0.5670, 0.5985,
    ],
};

// -- Elemental Burst: Skyfeather Song -- Cryo (healing, no damage scalings) --
// Note: Mika's burst is healing-only with no damage component

pub const MIKA: CharacterData = CharacterData {
    id: "mika",
    name: "Mika",
    element: Element::Cryo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [1049.0, 11083.0, 11640.0, 12506.0],
    base_atk: [19.0, 198.0, 208.0, 223.0],
    base_def: [60.0, 632.0, 664.0, 713.0],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "西風槍術・矢の道",
            hits: &[
                MIKA_NORMAL_1,
                MIKA_NORMAL_2,
                MIKA_NORMAL_3,
                MIKA_NORMAL_4,
                MIKA_NORMAL_5,
            ],
            charged: &[MIKA_CHARGED],
            plunging: &[MIKA_PLUNGE, MIKA_PLUNGE_LOW, MIKA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "星霜の渦",
            scalings: &[MIKA_SKILL_ARROW, MIKA_SKILL_FLARE, MIKA_SKILL_SHARD],
        },
        elemental_burst: TalentData {
            name: "天羽の歌",
            scalings: &[],
        },
    },
};

// =============================================================================
// Qiqi
// =============================================================================

// -- Normal Attack: Ancient Sword Art -- Physical --

const QIQI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3754, 0.4083, 0.4390, 0.4829, 0.5136, 0.5488, 0.5970, 0.6453, 0.6936, 0.7463, 0.7990,
        0.8517, 0.9043, 0.9570, 1.0097,
    ],
};

const QIQI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3887, 0.4204, 0.4520, 0.4972, 0.5288, 0.5650, 0.6147, 0.6644, 0.7142, 0.7684, 0.8226,
        0.8769, 0.9311, 0.9854, 1.0396,
    ],
};

const QIQI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2417, 0.2613, 0.2810, 0.3091, 0.3288, 0.3513, 0.3822, 0.4131, 0.4440, 0.4777, 0.5114,
        0.5451, 0.5789, 0.6126, 0.6463,
    ],
};

const QIQI_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2468, 0.2669, 0.2870, 0.3157, 0.3358, 0.3588, 0.3903, 0.4219, 0.4535, 0.4879, 0.5223,
        0.5568, 0.5912, 0.6257, 0.6601,
    ],
};

const QIQI_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6304, 0.6817, 0.7330, 0.8063, 0.8576, 0.9163, 0.9969, 1.0775, 1.1581, 1.2461, 1.3341,
        1.4220, 1.5099, 1.5979, 1.6859,
    ],
};

// -- Charged Attack -- Physical --

const QIQI_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6433, 0.6956, 0.7480, 0.8228, 0.8752, 0.9350, 1.0173, 1.0996, 1.1818, 1.2716, 1.3614,
        1.4511, 1.5409, 1.6306, 1.7204,
    ],
};

// -- Plunging Attack -- Physical --

const QIQI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const QIQI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const QIQI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: Adeptus Art: Herald of Frost -- Cryo --

const QIQI_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.9600, 1.0320, 1.1040, 1.2000, 1.2720, 1.3440, 1.4400, 1.5360, 1.6320, 1.7280, 1.8240,
        1.9200, 2.0400, 2.1600, 2.2800,
    ],
};

const QIQI_SKILL_HERALD: TalentScaling = TalentScaling {
    name: "寒病鬼ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.3600, 0.3870, 0.4140, 0.4500, 0.4770, 0.5040, 0.5400, 0.5760, 0.6120, 0.6480, 0.6840,
        0.7200, 0.7650, 0.8100, 0.8550,
    ],
};

// -- Elemental Burst: Adeptus Art: Preserver of Fortune -- Cryo --

const QIQI_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        2.8480, 3.0616, 3.2752, 3.5600, 3.7736, 3.9872, 4.2720, 4.5680, 4.8416, 5.1264, 5.4112,
        5.6960, 6.0520, 6.4080, 6.7640,
    ],
};

pub const QIQI: CharacterData = CharacterData {
    id: "qiqi",
    name: "Qiqi",
    element: Element::Cryo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [963.0, 10912.0, 11499.0, 12368.0],
    base_atk: [22.0, 253.0, 267.0, 287.0],
    base_def: [72.0, 814.0, 857.0, 922.0],
    ascension_stat: AscensionStat::HealingBonus(0.222),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "仙法・雲来古剣",
            hits: &[
                QIQI_NORMAL_1,
                QIQI_NORMAL_2,
                QIQI_NORMAL_3,
                QIQI_NORMAL_4,
                QIQI_NORMAL_5,
            ],
            charged: &[QIQI_CHARGED],
            plunging: &[QIQI_PLUNGE, QIQI_PLUNGE_LOW, QIQI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "仙法・寒病鬼差",
            scalings: &[QIQI_SKILL, QIQI_SKILL_HERALD],
        },
        elemental_burst: TalentData {
            name: "仙法・救苦度厄",
            scalings: &[QIQI_BURST],
        },
    },
};

// =============================================================================
// Rosaria
// =============================================================================

// -- Normal Attack: Spear of the Church -- Physical --

const ROSARIA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5246, 0.5673, 0.6100, 0.6710, 0.7137, 0.7625, 0.8296, 0.8967, 0.9638, 1.0370, 1.1102,
        1.1834, 1.2566, 1.3298, 1.4030,
    ],
};

const ROSARIA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5160, 0.5580, 0.6000, 0.6600, 0.7020, 0.7500, 0.8160, 0.8820, 0.9480, 1.0200, 1.0920,
        1.1640, 1.2360, 1.3080, 1.3800,
    ],
};

const ROSARIA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3182, 0.3441, 0.3700, 0.4070, 0.4329, 0.4625, 0.5032, 0.5439, 0.5846, 0.6290, 0.6734,
        0.7178, 0.7622, 0.8066, 0.8510,
    ],
};

const ROSARIA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6966, 0.7533, 0.8100, 0.8910, 0.9477, 1.0125, 1.1016, 1.1907, 1.2798, 1.3770, 1.4742,
        1.5714, 1.6686, 1.7658, 1.8630,
    ],
};

const ROSARIA_NORMAL_5A: TalentScaling = TalentScaling {
    name: "5段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4162, 0.4501, 0.4840, 0.5324, 0.5663, 0.6050, 0.6582, 0.7115, 0.7647, 0.8228, 0.8809,
        0.9390, 0.9970, 1.0551, 1.1132,
    ],
};

const ROSARIA_NORMAL_5B: TalentScaling = TalentScaling {
    name: "5段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4300, 0.4650, 0.5000, 0.5500, 0.5850, 0.6250, 0.6800, 0.7350, 0.7900, 0.8500, 0.9100,
        0.9700, 1.0300, 1.0900, 1.1500,
    ],
};

// -- Charged Attack -- Physical --

const ROSARIA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.3674, 1.4787, 1.5900, 1.7490, 1.8603, 1.9875, 2.1624, 2.3373, 2.5122, 2.7030, 2.8938,
        3.0846, 3.2754, 3.4662, 3.6570,
    ],
};

// -- Plunging Attack -- Physical --

const ROSARIA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const ROSARIA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const ROSARIA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: Ravaging Confession -- Cryo --

const ROSARIA_SKILL_1: TalentScaling = TalentScaling {
    name: "スキルダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5840, 0.6278, 0.6716, 0.7300, 0.7738, 0.8176, 0.8760, 0.9344, 0.9928, 1.0512, 1.1096,
        1.1680, 1.2410, 1.3140, 1.3870,
    ],
};

const ROSARIA_SKILL_2: TalentScaling = TalentScaling {
    name: "スキルダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.3600, 1.4620, 1.5640, 1.7000, 1.8020, 1.9040, 2.0400, 2.1760, 2.3120, 2.4480, 2.5840,
        2.7200, 2.8900, 3.0600, 3.2300,
    ],
};

// -- Elemental Burst: Rites of Termination -- Cryo --

const ROSARIA_BURST_1: TalentScaling = TalentScaling {
    name: "スキルダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.0400, 1.1180, 1.1960, 1.3000, 1.3780, 1.4560, 1.5600, 1.6640, 1.7680, 1.8720, 1.9760,
        2.0800, 2.2100, 2.3400, 2.4700,
    ],
};

const ROSARIA_BURST_2: TalentScaling = TalentScaling {
    name: "スキルダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.5200, 1.6340, 1.7480, 1.9000, 2.0140, 2.1280, 2.2800, 2.4320, 2.5840, 2.7360, 2.8880,
        3.0400, 3.2300, 3.4200, 3.6100,
    ],
};

const ROSARIA_BURST_DOT: TalentScaling = TalentScaling {
    name: "氷槍継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.3200, 1.4190, 1.5180, 1.6500, 1.7490, 1.8480, 1.9800, 2.1120, 2.2440, 2.3760, 2.5080,
        2.6400, 2.8050, 2.9700, 3.1350,
    ],
};

pub const ROSARIA: CharacterData = CharacterData {
    id: "rosaria",
    name: "Rosaria",
    element: Element::Cryo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [1030.0, 10891.0, 11438.0, 12289.0],
    base_atk: [20.0, 213.0, 223.0, 240.0],
    base_def: [60.0, 629.0, 661.0, 710.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "教会槍術",
            hits: &[
                ROSARIA_NORMAL_1,
                ROSARIA_NORMAL_2,
                ROSARIA_NORMAL_3,
                ROSARIA_NORMAL_4,
                ROSARIA_NORMAL_5A,
                ROSARIA_NORMAL_5B,
            ],
            charged: &[ROSARIA_CHARGED],
            plunging: &[ROSARIA_PLUNGE, ROSARIA_PLUNGE_LOW, ROSARIA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "懺悔の蹂躙",
            scalings: &[ROSARIA_SKILL_1, ROSARIA_SKILL_2],
        },
        elemental_burst: TalentData {
            name: "終命の儀式",
            scalings: &[ROSARIA_BURST_1, ROSARIA_BURST_2, ROSARIA_BURST_DOT],
        },
    },
};

// =============================================================================
// Shenhe
// =============================================================================

// -- Normal Attack: Dawnstar Piercer -- Physical --

const SHENHE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4326, 0.4678, 0.5031, 0.5534, 0.5886, 0.6288, 0.6842, 0.7395, 0.7948, 0.8552, 0.9156,
        0.9760, 1.0363, 1.0967, 1.1571,
    ],
};

const SHENHE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4024, 0.4352, 0.4680, 0.5148, 0.5476, 0.5850, 0.6365, 0.6879, 0.7394, 0.7956, 0.8518,
        0.9080, 0.9642, 1.0204, 1.0766,
    ],
};

const SHENHE_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5332, 0.5766, 0.6200, 0.6820, 0.7254, 0.7750, 0.8432, 0.9114, 0.9796, 1.0540, 1.1284,
        1.2028, 1.2772, 1.3516, 1.4260,
    ],
};

const SHENHE_NORMAL_4A: TalentScaling = TalentScaling {
    name: "4段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2632, 0.2846, 0.3060, 0.3366, 0.3580, 0.3825, 0.4162, 0.4498, 0.4834, 0.5202, 0.5570,
        0.5938, 0.6306, 0.6674, 0.7042,
    ],
};

const SHENHE_NORMAL_4B: TalentScaling = TalentScaling {
    name: "4段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2632, 0.2846, 0.3060, 0.3366, 0.3580, 0.3825, 0.4162, 0.4498, 0.4834, 0.5202, 0.5570,
        0.5938, 0.6306, 0.6674, 0.7042,
    ],
};

const SHENHE_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6562, 0.7096, 0.7630, 0.8393, 0.8927, 0.9538, 1.0375, 1.1213, 1.2051, 1.2969, 1.3888,
        1.4806, 1.5724, 1.6642, 1.7560,
    ],
};

// -- Charged Attack -- Physical --

const SHENHE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1068, 1.1969, 1.2870, 1.4157, 1.5058, 1.6088, 1.7502, 1.8917, 2.0331, 2.1878, 2.3424,
        2.4970, 2.6517, 2.8063, 2.9609,
    ],
};

// -- Plunging Attack -- Physical --

const SHENHE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const SHENHE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const SHENHE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: Spring Spirit Summoning -- Cryo --

const SHENHE_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "短押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.3920, 1.4964, 1.6008, 1.7400, 1.8444, 1.9488, 2.0880, 2.2272, 2.3664, 2.5056, 2.6448,
        2.7840, 2.9580, 3.1320, 3.3060,
    ],
};

const SHENHE_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.8880, 2.0296, 2.1712, 2.3600, 2.5016, 2.6432, 2.8320, 3.0208, 3.2096, 3.3984, 3.5872,
        3.7760, 4.0120, 4.2480, 4.4840,
    ],
};

// -- Elemental Burst: Divine Maiden's Deliverance -- Cryo --

const SHENHE_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.0080, 1.0836, 1.1592, 1.2600, 1.3356, 1.4112, 1.5120, 1.6128, 1.7136, 1.8144, 1.9152,
        2.0160, 2.1420, 2.2680, 2.3940,
    ],
};

const SHENHE_BURST_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.3312, 0.3560, 0.3809, 0.4140, 0.4388, 0.4637, 0.4968, 0.5299, 0.5630, 0.5962, 0.6293,
        0.6624, 0.7038, 0.7452, 0.7866,
    ],
};

pub const SHENHE: CharacterData = CharacterData {
    id: "shenhe",
    name: "Shenhe",
    element: Element::Cryo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [1011.0, 11463.0, 12080.0, 12993.0],
    base_atk: [24.0, 268.0, 282.0, 304.0],
    base_def: [65.0, 732.0, 772.0, 830.0],
    ascension_stat: AscensionStat::Atk(0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "踏辰摂斗",
            hits: &[
                SHENHE_NORMAL_1,
                SHENHE_NORMAL_2,
                SHENHE_NORMAL_3,
                SHENHE_NORMAL_4A,
                SHENHE_NORMAL_4B,
                SHENHE_NORMAL_5,
            ],
            charged: &[SHENHE_CHARGED],
            plunging: &[SHENHE_PLUNGE, SHENHE_PLUNGE_LOW, SHENHE_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "仰霊威召将役咒",
            scalings: &[SHENHE_SKILL_PRESS, SHENHE_SKILL_HOLD],
        },
        elemental_burst: TalentData {
            name: "神女遣霊真訣",
            scalings: &[SHENHE_BURST, SHENHE_BURST_DOT],
        },
    },
};

// =============================================================================
// Skirk
// =============================================================================

// -- Normal Attack: Havoc: Sunder -- Physical --

const SKIRK_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5452, 0.5896, 0.6340, 0.6974, 0.7418, 0.7925, 0.8622, 0.9320, 1.0017, 1.0778, 1.1539,
        1.2300, 1.3060, 1.3821, 1.4582,
    ],
};

const SKIRK_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4979, 0.5385, 0.5790, 0.6369, 0.6774, 0.7238, 0.7874, 0.8511, 0.9148, 0.9843, 1.0538,
        1.1233, 1.1927, 1.2622, 1.3317,
    ],
};

const SKIRK_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3242, 0.3506, 0.3770, 0.4147, 0.4411, 0.4713, 0.5127, 0.5542, 0.5957, 0.6409, 0.6861,
        0.7314, 0.7766, 0.8219, 0.8671,
    ],
};

const SKIRK_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6080, 0.6575, 0.7070, 0.7777, 0.8272, 0.8838, 0.9615, 1.0393, 1.1171, 1.2019, 1.2867,
        1.3716, 1.4564, 1.4413, 1.6261,
    ],
};

const SKIRK_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8290, 0.8965, 0.9640, 1.0604, 1.1279, 1.2050, 1.3110, 1.4171, 1.5231, 1.6388, 1.7545,
        1.8702, 1.9858, 2.1015, 2.2172,
    ],
};

// -- Charged Attack -- Physical --

const SKIRK_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6682, 0.7226, 0.7770, 0.8547, 0.9091, 0.9713, 1.0567, 1.1422, 1.2277, 1.3209, 1.4141,
        1.5074, 1.6006, 1.6939, 1.7871,
    ],
};

// -- Plunging Attack -- Physical --

const SKIRK_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const SKIRK_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const SKIRK_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: Havoc: Warp -- Cryo --

const SKIRK_SKILL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.3282, 1.4364, 1.5445, 1.6989, 1.8070, 1.9306, 2.1005, 2.2704, 2.4403, 2.6256, 2.8109,
        2.9963, 3.1816, 3.3669, 3.5523,
    ],
};

const SKIRK_SKILL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.1980, 1.2955, 1.3930, 1.5323, 1.6298, 1.7413, 1.8945, 2.0477, 2.2010, 2.3681, 2.5353,
        2.7025, 2.8696, 3.0368, 3.2039,
    ],
};

const SKIRK_SKILL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.7572, 0.8189, 0.8805, 0.9686, 1.0302, 1.1006, 1.1975, 1.2943, 1.3912, 1.4969, 1.6025,
        1.7082, 1.8138, 1.9195, 2.0252,
    ],
};

const SKIRK_SKILL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.8054, 0.8709, 0.9365, 1.0302, 1.0957, 1.1706, 1.2736, 1.3767, 1.4797, 1.5921, 1.7044,
        1.8168, 1.9292, 2.0416, 2.1540,
    ],
};

const SKIRK_SKILL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.9662, 2.1263, 2.2863, 2.5150, 2.6750, 2.8579, 3.1094, 3.3609, 3.6124, 3.8868, 4.1611,
        4.4548, 4.7098, 4.9842, 5.2586,
    ],
};

// -- Elemental Burst: Havoc: Ruin -- Cryo --

const SKIRK_BURST_SLASH: TalentScaling = TalentScaling {
    name: "スラッシュダメージ (×5)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2276, 1.3197, 1.4117, 1.5345, 1.6266, 1.7186, 1.8414, 1.9642, 2.0869, 2.2097, 2.3324,
        2.4552, 2.6087, 2.7621, 2.9156,
    ],
};

const SKIRK_BURST_FINAL: TalentScaling = TalentScaling {
    name: "最終スラッシュダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        2.0460, 2.1995, 2.3529, 2.5575, 2.7110, 2.8644, 3.0690, 3.2736, 3.4782, 3.6828, 3.8874,
        4.0920, 4.3478, 4.6035, 4.8593,
    ],
};

pub const SKIRK: CharacterData = CharacterData {
    id: "skirk",
    name: "Skirk",
    element: Element::Cryo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Snezhnaya,
    base_hp: [967.0, 10956.0, 11544.0, 12417.0],
    base_atk: [28.0, 317.0, 334.0, 359.0],
    base_def: [63.0, 711.0, 750.0, 806.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "大破・裂壊",
            hits: &[
                SKIRK_NORMAL_1,
                SKIRK_NORMAL_2,
                SKIRK_NORMAL_3,
                SKIRK_NORMAL_4,
                SKIRK_NORMAL_5,
            ],
            charged: &[SKIRK_CHARGED],
            plunging: &[SKIRK_PLUNGE, SKIRK_PLUNGE_LOW, SKIRK_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "大破・歪曲",
            scalings: &[
                SKIRK_SKILL_1,
                SKIRK_SKILL_2,
                SKIRK_SKILL_3,
                SKIRK_SKILL_4,
                SKIRK_SKILL_5,
            ],
        },
        elemental_burst: TalentData {
            name: "大破・崩壊",
            scalings: &[SKIRK_BURST_SLASH, SKIRK_BURST_FINAL],
        },
    },
};

// =============================================================================
// Wriothesley
// =============================================================================

// -- Normal Attack: Forceful Fists of Frost -- Cryo (Catalyst) --

const WRIOTHESLEY_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5336, 0.5770, 0.6205, 0.6825, 0.7259, 0.7756, 0.8438, 0.9121, 0.9803, 1.0548, 1.1292,
        1.2037, 1.2781, 1.3526, 1.4271,
    ],
};

const WRIOTHESLEY_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5180, 0.5601, 0.6023, 0.6625, 0.7047, 0.7529, 0.8191, 0.8854, 0.9517, 1.0239, 1.0962,
        1.1685, 1.2408, 1.3130, 1.3853,
    ],
};

const WRIOTHESLEY_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.6722, 0.7269, 0.7817, 0.8598, 0.9145, 0.9771, 1.0631, 1.1490, 1.2350, 1.3288, 1.4226,
        1.5164, 1.6102, 1.7040, 1.7978,
    ],
};

const WRIOTHESLEY_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.3790, 0.4099, 0.4407, 0.4848, 0.5157, 0.5509, 0.5994, 0.6479, 0.6964, 0.7493, 0.8022,
        0.8550, 0.9079, 0.9608, 1.0137,
    ],
};

const WRIOTHESLEY_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.9074, 0.9813, 1.0551, 1.1607, 1.2345, 1.3189, 1.4350, 1.5511, 1.6671, 1.7937, 1.9204,
        2.0470, 2.1736, 2.3005, 2.4268,
    ],
};

// -- Charged Attack (Vaulting Fist) -- Cryo (Catalyst) --

const WRIOTHESLEY_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.5296, 1.6443, 1.7590, 1.9120, 2.0267, 2.1414, 2.2944, 2.4736, 2.6003, 2.7533, 2.9062,
        3.0592, 3.2504, 3.4416, 3.6328,
    ],
};

// -- Plunging Attack -- Cryo (Catalyst) --

const WRIOTHESLEY_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const WRIOTHESLEY_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0391,
    ],
};

const WRIOTHESLEY_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: Icefang Rush -- Cryo --

const WRIOTHESLEY_SKILL: TalentScaling = TalentScaling {
    name: "強化パンチダメージ増加",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.4317, 1.4575, 1.4834, 1.5170, 1.5429, 1.5687, 1.6023, 1.6359, 1.6695, 1.7031, 1.7367,
        1.7703, 1.8039, 1.8375, 1.8711,
    ],
};

// -- Elemental Burst: Darkgold Wolfbite -- Cryo --

const WRIOTHESLEY_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ (×5)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2720, 1.3674, 1.4628, 1.5900, 1.6854, 1.7808, 1.9080, 2.0352, 2.1624, 2.2896, 2.4168,
        2.5440, 2.7030, 2.8620, 3.0210,
    ],
};

const WRIOTHESLEY_BURST_SURGING: TalentScaling = TalentScaling {
    name: "サージングブレードダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.4240, 0.4558, 0.4876, 0.5300, 0.5618, 0.5936, 0.6360, 0.6784, 0.7208, 0.7632, 0.8056,
        0.8480, 0.9010, 0.9540, 1.0070,
    ],
};

pub const WRIOTHESLEY: CharacterData = CharacterData {
    id: "wriothesley",
    name: "Wriothesley",
    element: Element::Cryo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [1058.0, 11993.0, 12637.0, 13593.0],
    base_atk: [24.0, 274.0, 289.0, 311.0],
    base_def: [59.0, 673.0, 710.0, 763.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "力の拳撃・霜",
            hits: &[
                WRIOTHESLEY_NORMAL_1,
                WRIOTHESLEY_NORMAL_2,
                WRIOTHESLEY_NORMAL_3,
                WRIOTHESLEY_NORMAL_4,
                WRIOTHESLEY_NORMAL_5,
            ],
            charged: &[WRIOTHESLEY_CHARGED],
            plunging: &[
                WRIOTHESLEY_PLUNGE,
                WRIOTHESLEY_PLUNGE_LOW,
                WRIOTHESLEY_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "氷牙突進",
            scalings: &[WRIOTHESLEY_SKILL],
        },
        elemental_burst: TalentData {
            name: "暗金の狼噛み",
            scalings: &[WRIOTHESLEY_BURST, WRIOTHESLEY_BURST_SURGING],
        },
    },
};
