#![allow(clippy::approx_constant)]
use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Beidou
// =============================================================================

// -- Normal Attack: 征霆鑑 (Oceanborne) -- All physical --

const BEIDOU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7112, 0.7692, 0.8272, 0.9099, 0.9679, 1.0340, 1.1249, 1.2158, 1.3067, 1.4061, 1.5055,
        1.6049, 1.7043, 1.8037, 1.9031,
    ],
};

const BEIDOU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7086, 0.7664, 0.8242, 0.9066, 0.9644, 1.0302, 1.1208, 1.2114, 1.3020, 1.4010, 1.5000,
        1.5990, 1.6980, 1.7970, 1.8960,
    ],
};

const BEIDOU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8832, 0.9550, 1.0268, 1.1295, 1.2013, 1.2835, 1.3963, 1.5091, 1.6219, 1.7454, 1.8689,
        1.9924, 2.1159, 2.2394, 2.3629,
    ],
};

const BEIDOU_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8652, 0.9356, 1.0060, 1.1066, 1.1770, 1.2575, 1.3682, 1.4789, 1.5896, 1.7103, 1.8310,
        1.9517, 2.0724, 2.1931, 2.3138,
    ],
};

const BEIDOU_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1214, 1.2126, 1.3038, 1.4342, 1.5254, 1.6298, 1.7733, 1.9168, 2.0603, 2.2166, 2.3729,
        2.5292, 2.6855, 2.8418, 2.9981,
    ],
};

// -- Charged Attack -- Physical --

const BEIDOU_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5624, 0.6082, 0.6540, 0.7194, 0.7652, 0.8175, 0.8894, 0.9614, 1.0333, 1.1118, 1.1903,
        1.2688, 1.3473, 1.4258, 1.5043,
    ],
};

const BEIDOU_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0182, 1.1012, 1.1842, 1.3026, 1.3856, 1.4800, 1.6102, 1.7404, 1.8706, 2.0128, 2.1550,
        2.2972, 2.4394, 2.5816, 2.7238,
    ],
};

// -- Plunging Attack -- Physical --

const BEIDOU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9541, 1.0148, 1.0841, 1.1795, 1.2749, 1.3702, 1.4744, 1.5786,
        1.6827, 1.7869, 1.8910, 1.9952,
    ],
};

const BEIDOU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9076, 2.0290, 2.1678, 2.3586, 2.5494, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const BEIDOU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5345, 2.7078, 2.9460, 3.1843, 3.4225, 3.6824, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
};

// -- Elemental Skill: 捉浪 (Tidecaller) -- Electro --

const BEIDOU_SKILL_BASE: TalentScaling = TalentScaling {
    name: "基礎ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2160, 1.3072, 1.3984, 1.5200, 1.6112, 1.7024, 1.8240, 1.9456, 2.0672, 2.1888, 2.3104,
        2.4320, 2.5840, 2.7360, 2.8880,
    ],
};

const BEIDOU_SKILL_HIT_BONUS: TalentScaling = TalentScaling {
    name: "受撃時ダメージボーナス",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.6000, 1.7200, 1.8400, 2.0000, 2.1200, 2.2400, 2.4000, 2.5600, 2.7200, 2.8800, 3.0400,
        3.2000, 3.4000, 3.6000, 3.8000,
    ],
};

// -- Elemental Burst: 斫雷 (Stormbreaker) -- Electro --

const BEIDOU_BURST_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2160, 1.3072, 1.3984, 1.5200, 1.6112, 1.7024, 1.8240, 1.9456, 2.0672, 2.1888, 2.3104,
        2.4320, 2.5840, 2.7360, 2.8880,
    ],
};

const BEIDOU_BURST_LIGHTNING: TalentScaling = TalentScaling {
    name: "雷放電ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.9600, 1.0320, 1.1040, 1.2000, 1.2720, 1.3440, 1.4400, 1.5360, 1.6320, 1.7280, 1.8240,
        1.9200, 2.0400, 2.1600, 2.2800,
    ],
};

pub const BEIDOU: CharacterData = CharacterData {
    id: "beidou",
    name: "Beidou",
    element: Element::Electro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [1094.0, 11565.0, 12146.0, 13050.0],
    base_atk: [19.0, 200.0, 210.0, 225.0],
    base_def: [54.0, 575.0, 603.0, 648.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Electro, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "征霆鑑",
            hits: &[
                BEIDOU_NORMAL_1,
                BEIDOU_NORMAL_2,
                BEIDOU_NORMAL_3,
                BEIDOU_NORMAL_4,
                BEIDOU_NORMAL_5,
            ],
            charged: &[BEIDOU_CHARGED_SPINNING, BEIDOU_CHARGED_FINAL],
            plunging: &[BEIDOU_PLUNGE, BEIDOU_PLUNGE_LOW, BEIDOU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "捉浪",
            scalings: &[BEIDOU_SKILL_BASE, BEIDOU_SKILL_HIT_BONUS],
        },
        elemental_burst: TalentData {
            name: "斫雷",
            scalings: &[BEIDOU_BURST_SKILL, BEIDOU_BURST_LIGHTNING],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Clorinde
// =============================================================================

// -- Normal Attack: 影狩りの誓い (Oath of Hunting Shadows) -- Physical --

const CLORINDE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5406, 0.5846, 0.6286, 0.6915, 0.7355, 0.7858, 0.8549, 0.9240, 0.9932, 1.0686, 1.1441,
        1.2195, 1.2949, 1.3704, 1.4458,
    ],
};

const CLORINDE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5163, 0.5583, 0.6003, 0.6604, 0.7024, 0.7504, 0.8164, 0.8825, 0.9485, 1.0206, 1.0926,
        1.1646, 1.2367, 1.3087, 1.3808,
    ],
};

const CLORINDE_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3419, 0.3697, 0.3975, 0.4373, 0.4651, 0.4969, 0.5406, 0.5843, 0.6281, 0.6758, 0.7235,
        0.7712, 0.8189, 0.8666, 0.9143,
    ],
};

const CLORINDE_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3419, 0.3697, 0.3975, 0.4373, 0.4651, 0.4969, 0.5406, 0.5843, 0.6281, 0.6758, 0.7235,
        0.7712, 0.8189, 0.8666, 0.9143,
    ],
};

const CLORINDE_NORMAL_4A: TalentScaling = TalentScaling {
    name: "4段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2313, 0.2502, 0.2690, 0.2959, 0.3147, 0.3363, 0.3658, 0.3954, 0.4250, 0.4573, 0.4896,
        0.5219, 0.5541, 0.5864, 0.6187,
    ],
};

const CLORINDE_NORMAL_4B: TalentScaling = TalentScaling {
    name: "4段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2313, 0.2502, 0.2690, 0.2959, 0.3147, 0.3363, 0.3658, 0.3954, 0.4250, 0.4573, 0.4896,
        0.5219, 0.5541, 0.5864, 0.6187,
    ],
};

const CLORINDE_NORMAL_4C: TalentScaling = TalentScaling {
    name: "4段ダメージ (3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2313, 0.2502, 0.2690, 0.2959, 0.3147, 0.3363, 0.3658, 0.3954, 0.4250, 0.4573, 0.4896,
        0.5219, 0.5541, 0.5864, 0.6187,
    ],
};

const CLORINDE_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9001, 0.9734, 1.0466, 1.1513, 1.2246, 1.3083, 1.4234, 1.5385, 1.6537, 1.7793, 1.9049,
        2.0305, 2.1561, 2.2817, 2.4072,
    ],
};

// -- Charged Attack -- Physical --

const CLORINDE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2814, 1.3857, 1.4900, 1.6390, 1.7433, 1.8625, 2.0264, 2.1903, 2.3542, 2.5330, 2.7118,
        2.8906, 3.0694, 3.2482, 3.4270,
    ],
};

// -- Plunging Attack -- Physical --

const CLORINDE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const CLORINDE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const CLORINDE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 狩人の眼光 (Hunter's Vigil) -- Electro --

const CLORINDE_SKILL_SWIFT_HUNT: TalentScaling = TalentScaling {
    name: "迅捷の追撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.2676, 0.2894, 0.3112, 0.3423, 0.3641, 0.3890, 0.4232, 0.4575, 0.4917, 0.5290, 0.5664,
        0.6037, 0.6411, 0.6784, 0.7158,
    ],
};

const CLORINDE_SKILL_SWIFT_HUNT_PIERCE: TalentScaling = TalentScaling {
    name: "迅捷の追撃貫通ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3879, 0.4194, 0.4510, 0.4961, 0.5277, 0.5638, 0.6134, 0.6630, 0.7126, 0.7667, 0.8208,
        0.8749, 0.9291, 0.9832, 1.0373,
    ],
};

const CLORINDE_SKILL_IMPALE: TalentScaling = TalentScaling {
    name: "夜巡りの一刺しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4396, 0.4754, 0.5112, 0.5623, 0.5981, 0.6390, 0.6952, 0.7515, 0.8077, 0.8690, 0.9304,
        0.9917, 1.0531, 1.1144, 1.1758,
    ],
};

const CLORINDE_SKILL_SURGING_BLADE: TalentScaling = TalentScaling {
    name: "流湧の刃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4320, 0.4644, 0.4968, 0.5400, 0.5724, 0.6048, 0.6480, 0.6912, 0.7344, 0.7776, 0.8208,
        0.8640, 0.9180, 0.9720, 1.0260,
    ],
};

// -- Elemental Burst: 残光の晩番 (Last Lightfall) -- Electro --

const CLORINDE_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2688, 1.3640, 1.4591, 1.5860, 1.6812, 1.7763, 1.9032, 2.0301, 2.1570, 2.2838, 2.4107,
        2.5376, 2.6962, 2.8548, 3.0134,
    ],
};

pub const CLORINDE: CharacterData = CharacterData {
    id: "clorinde",
    name: "Clorinde",
    element: Element::Electro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [1009.0, 11431.0, 12045.0, 12956.0],
    base_atk: [26.0, 298.0, 314.0, 337.0],
    base_def: [61.0, 692.0, 729.0, 784.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "影狩りの誓い",
            hits: &[
                CLORINDE_NORMAL_1,
                CLORINDE_NORMAL_2,
                CLORINDE_NORMAL_3A,
                CLORINDE_NORMAL_3B,
                CLORINDE_NORMAL_4A,
                CLORINDE_NORMAL_4B,
                CLORINDE_NORMAL_4C,
                CLORINDE_NORMAL_5,
            ],
            charged: &[CLORINDE_CHARGED],
            plunging: &[CLORINDE_PLUNGE, CLORINDE_PLUNGE_LOW, CLORINDE_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "狩人の眼光",
            scalings: &[
                CLORINDE_SKILL_SWIFT_HUNT,
                CLORINDE_SKILL_SWIFT_HUNT_PIERCE,
                CLORINDE_SKILL_IMPALE,
                CLORINDE_SKILL_SURGING_BLADE,
            ],
        },
        elemental_burst: TalentData {
            name: "残光の晩番",
            scalings: &[CLORINDE_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Cyno
// =============================================================================

// -- Normal Attack: 秘儀・律淵渡り (Invoker's Spear) -- Physical --

const CYNO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4926, 0.5327, 0.5728, 0.6301, 0.6702, 0.7160, 0.7789, 0.8419, 0.9048, 0.9737, 1.0425,
        1.1114, 1.1803, 1.2491, 1.3180,
    ],
};

const CYNO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4792, 0.5182, 0.5572, 0.6129, 0.6519, 0.6965, 0.7578, 0.8191, 0.8804, 0.9473, 1.0141,
        1.0810, 1.1479, 1.2147, 1.2816,
    ],
};

const CYNO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5858, 0.6335, 0.6812, 0.7493, 0.7970, 0.8515, 0.9264, 1.0013, 1.0762, 1.5883, 1.2404,
        1.3222, 1.4040, 1.4858, 1.5675,
    ],
};

const CYNO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7588, 0.8206, 0.8824, 0.9706, 1.0324, 1.1030, 1.2001, 1.2971, 1.3942, 1.5001, 1.6061,
        1.7120, 1.8180, 1.9239, 2.0299,
    ],
};

// -- Charged Attack -- Physical --

const CYNO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2238, 1.3234, 1.4230, 1.5653, 1.6649, 1.7788, 1.9353, 2.0918, 2.2483, 2.4191, 2.5899,
        2.7606, 2.9314, 3.1021, 3.2729,
    ],
};

// -- Plunging Attack -- Physical --

const CYNO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const CYNO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const CYNO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 秘儀・律淵の渡し (Secret Rite: Chasmic Soulfarer) -- Electro --

const CYNO_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.3040, 1.4018, 1.4996, 1.6300, 1.7278, 1.8256, 1.9560, 2.0864, 2.2168, 2.3472, 2.4776,
        2.6080, 2.7710, 2.9340, 3.0970,
    ],
};

const CYNO_SKILL_MORTUARY: TalentScaling = TalentScaling {
    name: "殯儀の秘儀ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.5680, 1.6856, 1.8032, 1.9600, 2.0776, 2.1952, 2.3520, 2.5088, 2.6656, 2.8224, 2.9792,
        3.1360, 3.3320, 3.5280, 3.7240,
    ],
};

// -- Elemental Burst: 聖儀・狼駆 (Sacred Rite: Wolf's Swiftness) -- Electro --
// Burst stance normal attacks (Electro infusion)

const CYNO_BURST_NORMAL_1: TalentScaling = TalentScaling {
    name: "末途の一撃1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7826, 0.8465, 0.9104, 1.0015, 1.0654, 1.1380, 1.2380, 1.3380, 1.4380, 1.5475, 1.6570,
        1.7664, 1.8758, 1.9852, 2.0939,
    ],
};

const CYNO_BURST_NORMAL_2: TalentScaling = TalentScaling {
    name: "末途の一撃2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.8246, 0.8920, 0.9593, 1.0553, 1.1226, 1.1991, 1.3043, 1.4095, 1.5147, 1.6298, 1.7450,
        1.8601, 1.9752, 2.0904, 2.2055,
    ],
};

const CYNO_BURST_NORMAL_3: TalentScaling = TalentScaling {
    name: "末途の一撃3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.0462, 1.1316, 1.2170, 1.3387, 1.4241, 1.5213, 1.6551, 1.7889, 1.9226, 2.0688, 2.2140,
        2.3600, 2.5060, 2.6520, 2.7980,
    ],
};

const CYNO_BURST_NORMAL_4: TalentScaling = TalentScaling {
    name: "末途の一撃4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.0340, 1.1184, 1.2028, 1.3231, 1.4075, 1.5035, 1.6358, 1.7680, 1.9003, 2.0447, 2.1896,
        2.3324, 2.4766, 2.6208, 2.7650,
    ],
};

const CYNO_BURST_NORMAL_5: TalentScaling = TalentScaling {
    name: "末途の一撃5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.3080, 1.4148, 1.5216, 1.6738, 1.7806, 1.9020, 2.0694, 2.2367, 2.4040, 2.5867, 2.7698,
        2.9516, 3.1347, 3.3165, 3.4990,
    ],
};

const CYNO_BURST_CHARGED: TalentScaling = TalentScaling {
    name: "末途の一撃重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.0106, 1.0932, 1.1753, 1.2929, 1.3751, 1.4694, 1.5984, 1.7271, 1.8570, 1.9976, 2.1388,
        2.2794, 2.4206, 2.5618, 2.7030,
    ],
};

pub const CYNO: CharacterData = CharacterData {
    id: "cyno",
    name: "Cyno",
    element: Element::Electro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Sumeru,
    base_hp: [972.0, 11020.0, 11613.0, 12491.0],
    base_atk: [25.0, 281.0, 296.0, 318.0],
    base_def: [67.0, 758.0, 799.0, 859.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "秘儀・律淵渡り",
            hits: &[CYNO_NORMAL_1, CYNO_NORMAL_2, CYNO_NORMAL_3, CYNO_NORMAL_4],
            charged: &[CYNO_CHARGED],
            plunging: &[CYNO_PLUNGE, CYNO_PLUNGE_LOW, CYNO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "秘儀・律淵の渡し",
            scalings: &[CYNO_SKILL, CYNO_SKILL_MORTUARY],
        },
        elemental_burst: TalentData {
            name: "聖儀・狼駆",
            scalings: &[
                CYNO_BURST_NORMAL_1,
                CYNO_BURST_NORMAL_2,
                CYNO_BURST_NORMAL_3,
                CYNO_BURST_NORMAL_4,
                CYNO_BURST_NORMAL_5,
                CYNO_BURST_CHARGED,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Dori
// =============================================================================

// -- Normal Attack: マーベラスソードダンス (Marvelous Sword-Dance) -- Physical --

const DORI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9021, 0.9756, 1.0490, 1.1539, 1.2274, 1.3113, 1.4266, 1.5420, 1.6574, 1.7833, 1.9093,
        2.0353, 2.1612, 2.2872, 2.4132,
    ],
};

const DORI_NORMAL_2A: TalentScaling = TalentScaling {
    name: "2段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4107, 0.4441, 0.4775, 0.5253, 0.5587, 0.5969, 0.6493, 0.7017, 0.7542, 0.8116, 0.8690,
        0.9264, 0.9839, 1.0413, 1.0987,
    ],
};

const DORI_NORMAL_2B: TalentScaling = TalentScaling {
    name: "2段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4313, 0.4665, 0.5017, 0.5519, 0.5871, 0.6272, 0.6822, 0.7373, 0.7924, 0.8528, 0.9132,
        0.9735, 1.0339, 1.0943, 1.1546,
    ],
};

const DORI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2836, 1.3883, 1.4930, 1.6423, 1.7470, 1.8663, 2.0303, 2.1944, 2.3584, 2.5379, 2.7174,
        2.8969, 3.0764, 3.2559, 3.4354,
    ],
};

// -- Charged Attack -- Physical --

const DORI_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6254, 0.6764, 0.7275, 0.8002, 0.8513, 0.9094, 0.9893, 1.0692, 1.1491, 1.2366, 1.3365,
        1.4541, 1.5717, 1.6893, 1.8173,
    ],
};

const DORI_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1314, 1.2235, 1.3156, 1.4472, 1.5393, 1.6445, 1.7893, 1.9341, 2.0789, 2.2366, 2.4164,
        2.6293, 2.8422, 3.0551, 3.2861,
    ],
};

// -- Plunging Attack -- Physical --

const DORI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9541, 1.0148, 1.0841, 1.1795, 1.2749, 1.3702, 1.4744, 1.5786,
        1.6827, 1.7869, 1.8910, 1.9952,
    ],
};

const DORI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9076, 2.0290, 2.1678, 2.3586, 2.5494, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const DORI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5345, 2.7078, 2.9460, 3.1843, 3.4225, 3.6824, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
};

// -- Elemental Skill: 鎮霊のランプ・トラブルシューター (Spirit-Warding Lamp: Troubleshooter Cannon) -- Electro --

const DORI_SKILL_SHOT: TalentScaling = TalentScaling {
    name: "トラブルシューター弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.4728, 1.5833, 1.6937, 1.8410, 1.9515, 2.0619, 2.2092, 2.3565, 2.5038, 2.6510, 2.7983,
        2.9456, 3.1297, 3.3138, 3.4979,
    ],
};

const DORI_SKILL_AFTER_SALES: TalentScaling = TalentScaling {
    name: "アフターサービス弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3156, 0.3393, 0.3629, 0.3945, 0.4182, 0.4418, 0.4734, 0.5050, 0.5365, 0.5681, 0.5996,
        0.6312, 0.6707, 0.7101, 0.7496,
    ],
};

// -- Elemental Burst: 卸カガラカの法契 (Alcazarzaray's Exactitude) -- Electro --

const DORI_BURST_CONNECTOR: TalentScaling = TalentScaling {
    name: "コネクターダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.1592, 0.1711, 0.1831, 0.1990, 0.2110, 0.2229, 0.2388, 0.2547, 0.2706, 0.2866, 0.3025,
        0.3184, 0.3383, 0.3582, 0.3781,
    ],
};

pub const DORI: CharacterData = CharacterData {
    id: "dori",
    name: "Dori",
    element: Element::Electro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Sumeru,
    base_hp: [1039.0, 10987.0, 11539.0, 12397.0],
    base_atk: [19.0, 198.0, 208.0, 223.0],
    base_def: [61.0, 641.0, 673.0, 723.0],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "マーベラスソードダンス改",
            hits: &[DORI_NORMAL_1, DORI_NORMAL_2A, DORI_NORMAL_2B, DORI_NORMAL_3],
            charged: &[DORI_CHARGED_SPINNING, DORI_CHARGED_FINAL],
            plunging: &[DORI_PLUNGE, DORI_PLUNGE_LOW, DORI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "鎮霊のランプ・トラブルシューター",
            scalings: &[DORI_SKILL_SHOT, DORI_SKILL_AFTER_SALES],
        },
        elemental_burst: TalentData {
            name: "卸カガラカの法契",
            scalings: &[DORI_BURST_CONNECTOR],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Fischl
// =============================================================================

// -- Normal Attack: 罪滅ぼしの矢 (Bolts of Downfall) -- Physical --

const FISCHL_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4412, 0.4771, 0.5130, 0.5643, 0.6002, 0.6413, 0.6976, 0.7540, 0.8104, 0.8721, 0.9337,
        0.9954, 1.0568, 1.1183, 1.1799,
    ],
};

const FISCHL_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4678, 0.5058, 0.5438, 0.5982, 0.6362, 0.6798, 0.7397, 0.7996, 0.8594, 0.9246, 0.9898,
        1.0550, 1.1206, 1.1862, 1.2514,
    ],
};

const FISCHL_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5814, 0.6289, 0.6764, 0.7440, 0.7916, 0.8455, 0.9198, 0.9942, 1.0685, 1.1498, 1.2310,
        1.3122, 1.3934, 1.4747, 1.5559,
    ],
};

const FISCHL_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5771, 0.6242, 0.6712, 0.7383, 0.7854, 0.8390, 0.9127, 0.9864, 1.0601, 1.1409, 1.2216,
        1.3024, 1.3831, 1.4639, 1.5446,
    ],
};

const FISCHL_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7207, 0.7794, 0.8382, 0.9220, 0.9808, 1.0478, 1.1398, 1.2319, 1.3239, 1.4248, 1.5257,
        1.6266, 1.7275, 1.8284, 1.9293,
    ],
};

// -- Aimed Shot -- Electro (charged) --

const FISCHL_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const FISCHL_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

// -- Plunging Attack -- Physical --

const FISCHL_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const FISCHL_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const FISCHL_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 夜巡りの翼 (Nightrider) -- Electro --

const FISCHL_SKILL_OZ: TalentScaling = TalentScaling {
    name: "オズ攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.8880, 0.9546, 1.0212, 1.1100, 1.1766, 1.2432, 1.3320, 1.4208, 1.5096, 1.5984, 1.6872,
        1.7760, 1.8870, 1.9980, 2.1090,
    ],
};

const FISCHL_SKILL_SUMMON: TalentScaling = TalentScaling {
    name: "召喚ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.1544, 1.2410, 1.3276, 1.4430, 1.5296, 1.6162, 1.7316, 1.8470, 1.9625, 2.0779, 2.1934,
        2.3088, 2.4531, 2.5974, 2.7417,
    ],
};

// -- Elemental Burst: 夜の幻現 (Midnight Phantasmagoria) -- Electro --

const FISCHL_BURST: TalentScaling = TalentScaling {
    name: "落雷ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        2.0800, 2.2360, 2.3920, 2.6000, 2.7560, 2.9120, 3.1200, 3.3280, 3.5360, 3.7440, 3.9520,
        4.1600, 4.4200, 4.6800, 4.9400,
    ],
};

pub const FISCHL: CharacterData = CharacterData {
    id: "fischl",
    name: "Fischl",
    element: Element::Electro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [770.0, 8144.0, 8553.0, 9189.0],
    base_atk: [20.0, 216.0, 227.0, 244.0],
    base_def: [50.0, 526.0, 553.0, 594.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "罪滅ぼしの矢",
            hits: &[
                FISCHL_NORMAL_1,
                FISCHL_NORMAL_2,
                FISCHL_NORMAL_3,
                FISCHL_NORMAL_4,
                FISCHL_NORMAL_5,
            ],
            charged: &[FISCHL_AIMED, FISCHL_AIMED_FULL],
            plunging: &[FISCHL_PLUNGE, FISCHL_PLUNGE_LOW, FISCHL_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "夜巡りの翼",
            scalings: &[FISCHL_SKILL_OZ, FISCHL_SKILL_SUMMON],
        },
        elemental_burst: TalentData {
            name: "夜の幻現",
            scalings: &[FISCHL_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Iansan
// =============================================================================

// -- Normal Attack: 鋲打ちスパイク (Weighted Spike) -- Physical --

const IANSAN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4698, 0.5080, 0.5462, 0.6009, 0.6391, 0.6828, 0.7429, 0.8030, 0.8630, 0.9286, 0.9941,
        1.0597, 1.1252, 1.1908, 1.2563,
    ],
};

const IANSAN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4276, 0.4625, 0.4973, 0.5470, 0.5818, 0.6216, 0.6763, 0.7310, 0.7857, 0.8453, 0.9050,
        0.9647, 1.0244, 1.0841, 1.1437,
    ],
};

const IANSAN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6439, 0.6963, 0.7487, 0.8236, 0.8760, 0.9359, 1.0182, 1.1006, 1.1829, 1.2728, 1.3626,
        1.4525, 1.5423, 1.6322, 1.7220,
    ],
};

// -- Charged Attack -- Physical --

const IANSAN_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0028, 1.0844, 1.1660, 1.2826, 1.3642, 1.4575, 1.5858, 1.7140, 1.8423, 1.9822, 2.1221,
        2.2620, 2.4020, 2.5419, 2.6818,
    ],
};

const IANSAN_SWIFT_STORMFLIGHT: TalentScaling = TalentScaling {
    name: "疾風翔撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8419, 0.9105, 0.9790, 1.0769, 1.1454, 1.2238, 1.3314, 1.4391, 1.5468, 1.6643, 1.7818,
        1.8993, 2.0167, 2.1342, 2.2517,
    ],
};

// -- Plunging Attack -- Physical --

const IANSAN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const IANSAN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const IANSAN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 雷奔撃 (Thunderbolt Rush) -- Electro --

const IANSAN_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        2.8640, 3.0788, 3.2936, 3.5800, 3.7948, 4.0096, 4.2960, 4.5824, 4.6880, 5.1552, 5.4416,
        5.7280, 6.0860, 6.4440, 6.8020,
    ],
};

// -- Elemental Burst: 力の三大原則 (The Three Principles of Power) -- Electro --

const IANSAN_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        4.3040, 4.6268, 4.9496, 5.3800, 5.7028, 6.0256, 6.4560, 6.8864, 7.3168, 7.7472, 8.1776,
        8.6080, 9.1460, 9.6840, 10.2220,
    ],
};

pub const IANSAN: CharacterData = CharacterData {
    id: "iansan",
    name: "Iansan",
    element: Element::Electro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Natlan,
    base_hp: [894.0, 9445.0, 9919.0, 10657.0],
    base_atk: [22.0, 228.0, 239.0, 257.0],
    base_def: [54.0, 566.0, 594.0, 638.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "鋲打ちスパイク",
            hits: &[IANSAN_NORMAL_1, IANSAN_NORMAL_2, IANSAN_NORMAL_3],
            charged: &[IANSAN_CHARGED, IANSAN_SWIFT_STORMFLIGHT],
            plunging: &[IANSAN_PLUNGE, IANSAN_PLUNGE_LOW, IANSAN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "雷奔撃",
            scalings: &[IANSAN_SKILL],
        },
        elemental_burst: TalentData {
            name: "力の三大原則",
            scalings: &[IANSAN_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Ineffa
// =============================================================================

// -- Normal Attack: サイクロン集塵 (Cyclonic Duster) -- All Electro (Catalyst) --

const INEFFA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3484, 0.3767, 0.4051, 0.4456, 0.4739, 0.5063, 0.5509, 0.5954, 0.6400, 0.6886, 0.7372,
        0.7858, 0.8344, 0.8830, 0.9316,
    ],
};

const INEFFA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3422, 0.3701, 0.3979, 0.4377, 0.4656, 0.4974, 0.5412, 0.5849, 0.6287, 0.6765, 0.7242,
        0.7720, 0.8197, 0.8675, 0.9152,
    ],
};

const INEFFA_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.2276, 0.2461, 0.2646, 0.2911, 0.3096, 0.3308, 0.3599, 0.3890, 0.4181, 0.4498, 0.4816,
        0.5133, 0.5451, 0.5768, 0.6086,
    ],
};

const INEFFA_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.2276, 0.2461, 0.2646, 0.2911, 0.3096, 0.3308, 0.3599, 0.3890, 0.4181, 0.4498, 0.4816,
        0.5133, 0.5451, 0.5768, 0.6086,
    ],
};

// -- Charged Attack -- Electro --

const INEFFA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.9494, 1.0267, 1.1040, 1.2144, 1.2917, 1.3800, 1.5014, 1.6229, 1.7443, 1.8768, 2.0093,
        2.1418, 2.2742, 2.4067, 2.5392,
    ],
};

// -- Plunging Attack -- Electro (Catalyst) --

const INEFFA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const INEFFA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const INEFFA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.5968, 1.7267, 1.8657, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 掃除モード・搬送周波数 (Cleaning Mode) -- Electro --

const INEFFA_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.8640, 0.9288, 0.9936, 1.0800, 1.1448, 1.2096, 1.2960, 1.3824, 1.4688, 1.5552, 1.6416,
        1.7280, 1.8360, 1.9440, 2.0520,
    ],
};

const INEFFA_SKILL_DISCHARGE: TalentScaling = TalentScaling {
    name: "ビルギッタ放電ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.9600, 1.0320, 1.1040, 1.2000, 1.2720, 1.3440, 1.4400, 1.5360, 1.6320, 1.7280, 1.8240,
        1.9200, 2.0400, 2.1600, 2.2800,
    ],
};

// -- Elemental Burst: 最高指令・旋風絶滅 (Supreme Instruction) -- Electro --

const INEFFA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        6.7680, 7.2756, 7.7832, 8.4600, 8.9676, 9.4752, 10.1520, 10.8288, 11.5056, 12.1824,
        12.8592, 13.5360, 14.3820, 15.2280, 16.0740,
    ],
};

pub const INEFFA: CharacterData = CharacterData {
    id: "ineffa",
    name: "Ineffa",
    element: Element::Electro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [982.0, 11128.0, 11727.0, 12613.0],
    base_atk: [26.0, 291.0, 307.0, 330.0],
    base_def: [64.0, 730.0, 770.0, 828.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "サイクロン集塵",
            hits: &[
                INEFFA_NORMAL_1,
                INEFFA_NORMAL_2,
                INEFFA_NORMAL_3A,
                INEFFA_NORMAL_3B,
            ],
            charged: &[INEFFA_CHARGED],
            plunging: &[INEFFA_PLUNGE, INEFFA_PLUNGE_LOW, INEFFA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "掃除モード・搬送周波数",
            scalings: &[INEFFA_SKILL, INEFFA_SKILL_DISCHARGE],
        },
        elemental_burst: TalentData {
            name: "最高指令・旋風絶滅",
            scalings: &[INEFFA_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Keqing
// =============================================================================

// -- Normal Attack: 雲来剣法 (Yunlai Swordsmanship) -- Physical --

const KEQING_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4102, 0.4436, 0.4770, 0.5247, 0.5581, 0.5963, 0.6487, 0.7012, 0.7537, 0.8109, 0.8682,
        0.9254, 0.9826, 1.0399, 1.0971,
    ],
};

const KEQING_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4102, 0.4436, 0.4770, 0.5247, 0.5581, 0.5963, 0.6487, 0.7012, 0.7537, 0.8109, 0.8682,
        0.9254, 0.9826, 1.0399, 1.0971,
    ],
};

const KEQING_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5444, 0.5887, 0.6330, 0.6963, 0.7406, 0.7913, 0.8609, 0.9306, 1.0003, 1.0762, 1.1521,
        1.2281, 1.3040, 1.3799, 1.4558,
    ],
};

const KEQING_NORMAL_4A: TalentScaling = TalentScaling {
    name: "4段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3148, 0.3404, 0.3660, 0.4026, 0.4282, 0.4575, 0.4978, 0.5380, 0.5783, 0.6222, 0.6662,
        0.7101, 0.7541, 0.7980, 0.8420,
    ],
};

const KEQING_NORMAL_4B: TalentScaling = TalentScaling {
    name: "4段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3440, 0.3720, 0.4000, 0.4400, 0.4680, 0.5000, 0.5440, 0.5880, 0.6320, 0.6800, 0.7280,
        0.7760, 0.8240, 0.8720, 0.9200,
    ],
};

const KEQING_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6699, 0.7243, 0.7787, 0.8566, 0.9110, 0.9734, 1.0591, 1.1449, 1.2306, 1.3239, 1.4172,
        1.5106, 1.6039, 1.6972, 1.7905,
    ],
};

// -- Charged Attack -- Physical --

const KEQING_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7680, 0.8305, 0.8930, 0.9823, 1.0448, 1.1163, 1.2142, 1.3122, 1.4102, 1.5178, 1.6254,
        1.7330, 1.8394, 1.9470, 2.0546,
    ],
};

const KEQING_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8600, 0.9300, 1.0000, 1.1000, 1.1700, 1.2500, 1.3600, 1.4700, 1.5800, 1.7000, 1.8200,
        1.9400, 2.0600, 2.1800, 2.3000,
    ],
};

// -- Plunging Attack -- Physical --

const KEQING_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const KEQING_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const KEQING_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 星辰帰位 (Stellar Restoration) -- Electro --

const KEQING_SKILL_STILETTO: TalentScaling = TalentScaling {
    name: "雷楔ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5040, 0.5418, 0.5796, 0.6300, 0.6678, 0.7056, 0.7560, 0.8064, 0.8568, 0.9072, 0.9576,
        1.0080, 1.0710, 1.1340, 1.1970,
    ],
};

const KEQING_SKILL_SLASH: TalentScaling = TalentScaling {
    name: "斬撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.6800, 1.8060, 1.9320, 2.1000, 2.2260, 2.3520, 2.5200, 2.6880, 2.8560, 3.0240, 3.1920,
        3.3600, 3.5700, 3.7800, 3.9900,
    ],
};

const KEQING_SKILL_THUNDERCLAP: TalentScaling = TalentScaling {
    name: "雷鳴斬撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.8400, 0.9030, 0.9660, 1.0500, 1.1130, 1.1760, 1.2600, 1.3440, 1.4280, 1.5120, 1.5960,
        1.6800, 1.7850, 1.8900, 1.9950,
    ],
};

// -- Elemental Burst: 天街巡遊 (Starward Sword) -- Electro --

const KEQING_BURST_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.8800, 0.9460, 1.0120, 1.1000, 1.1660, 1.2320, 1.3200, 1.4080, 1.4960, 1.5840, 1.6720,
        1.7600, 1.8700, 1.9800, 2.0900,
    ],
};

const KEQING_BURST_CONSECUTIVE: TalentScaling = TalentScaling {
    name: "連斬ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.2400, 0.2580, 0.2760, 0.3000, 0.3180, 0.3360, 0.3600, 0.3840, 0.4080, 0.4320, 0.4560,
        0.4800, 0.5100, 0.5400, 0.5700,
    ],
};

const KEQING_BURST_LAST: TalentScaling = TalentScaling {
    name: "最後の一撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.8880, 2.0296, 2.1712, 2.3600, 2.5016, 2.6432, 2.8320, 3.0208, 3.2096, 3.3984, 3.5872,
        3.7760, 4.0120, 4.2480, 4.4840,
    ],
};

pub const KEQING: CharacterData = CharacterData {
    id: "keqing",
    name: "Keqing",
    element: Element::Electro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [1020.0, 11561.0, 12182.0, 13103.0],
    base_atk: [25.0, 285.0, 300.0, 323.0],
    base_def: [62.0, 705.0, 743.0, 799.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "雲来剣法",
            hits: &[
                KEQING_NORMAL_1,
                KEQING_NORMAL_2,
                KEQING_NORMAL_3,
                KEQING_NORMAL_4A,
                KEQING_NORMAL_4B,
                KEQING_NORMAL_5,
            ],
            charged: &[KEQING_CHARGED_1, KEQING_CHARGED_2],
            plunging: &[KEQING_PLUNGE, KEQING_PLUNGE_LOW, KEQING_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "星辰帰位",
            scalings: &[
                KEQING_SKILL_STILETTO,
                KEQING_SKILL_SLASH,
                KEQING_SKILL_THUNDERCLAP,
            ],
        },
        elemental_burst: TalentData {
            name: "天街巡遊",
            scalings: &[
                KEQING_BURST_SKILL,
                KEQING_BURST_CONSECUTIVE,
                KEQING_BURST_LAST,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
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
};

const KUJOU_SARA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3870, 0.4185, 0.4500, 0.4950, 0.5265, 0.5625, 0.6120, 0.6615, 0.7110, 0.7650, 0.8190,
        0.8730, 0.9270, 0.9810, 1.0350,
    ],
};

const KUJOU_SARA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4850, 0.5245, 0.5640, 0.6204, 0.6599, 0.7050, 0.7670, 0.8291, 0.8911, 0.9588, 1.0265,
        1.0942, 1.1618, 1.2295, 1.2972,
    ],
};

const KUJOU_SARA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5040, 0.5450, 0.5860, 0.6446, 0.6856, 0.7325, 0.7970, 0.8614, 0.9259, 0.9962, 1.0665,
        1.1368, 1.2072, 1.2775, 1.3478,
    ],
};

const KUJOU_SARA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5805, 0.6278, 0.6750, 0.7425, 0.7898, 0.8438, 0.9180, 0.9923, 1.0665, 1.1475, 1.2285,
        1.3095, 1.3905, 1.4715, 1.5525,
    ],
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
};

const KUJOU_SARA_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
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
};

const KUJOU_SARA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const KUJOU_SARA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
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
};

const KUJOU_SARA_BURST_STORMCLUSTER: TalentScaling = TalentScaling {
    name: "天狗雷球ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3412, 0.3668, 0.3924, 0.4265, 0.4521, 0.4777, 0.5118, 0.5459, 0.5800, 0.6142, 0.6483,
        0.6824, 0.7251, 0.7677, 0.8104,
    ],
};

pub const KUJOU_SARA: CharacterData = CharacterData {
    id: "kujou_sara",
    name: "Kujou Sara",
    element: Element::Electro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Inazuma,
    base_hp: [802.0, 8481.0, 8907.0, 9570.0],
    base_atk: [16.0, 173.0, 182.0, 195.0],
    base_def: [53.0, 556.0, 584.0, 628.0],
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

// =============================================================================
// Kuki Shinobu
// =============================================================================

// -- Normal Attack: 忍流飛刃斬り (Shinobu's Shadowsword) -- Physical --

const KUKI_SHINOBU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4876, 0.5273, 0.5670, 0.6237, 0.6634, 0.7088, 0.7711, 0.8335, 0.8959, 0.9639, 1.0319,
        1.0999, 1.1680, 1.2361, 1.3041,
    ],
};

const KUKI_SHINOBU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4455, 0.4817, 0.5180, 0.5698, 0.6061, 0.6475, 0.7045, 0.7615, 0.8184, 0.8806, 0.9428,
        1.0049, 1.0671, 1.1292, 1.1914,
    ],
};

const KUKI_SHINOBU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5934, 0.6417, 0.6900, 0.7590, 0.8073, 0.8625, 0.9384, 1.0143, 1.0902, 1.1730, 1.2558,
        1.3386, 1.4214, 1.5042, 1.5870,
    ],
};

const KUKI_SHINOBU_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7611, 0.8231, 0.8850, 0.9735, 1.0355, 1.1063, 1.2036, 1.3010, 1.3983, 1.5045, 1.6107,
        1.7169, 1.8231, 1.9293, 2.0355,
    ],
};

// -- Charged Attack -- Physical --

const KUKI_SHINOBU_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5563, 0.6016, 0.6469, 0.7116, 0.7569, 0.8086, 0.8798, 0.9509, 1.0221, 1.0997, 1.1774,
        1.2550, 1.3326, 1.4102, 1.4879,
    ],
};

const KUKI_SHINOBU_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6677, 0.7220, 0.7763, 0.8540, 0.9083, 0.9704, 1.0558, 1.1412, 1.2266, 1.3198, 1.4129,
        1.5061, 1.5993, 1.6924, 1.7856,
    ],
};

// -- Plunging Attack -- Physical --

const KUKI_SHINOBU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const KUKI_SHINOBU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const KUKI_SHINOBU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 御咎験式・紫牙 (Sanctifying Ring) -- Electro --

const KUKI_SHINOBU_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7571, 0.8139, 0.8707, 0.9464, 1.0032, 1.0599, 1.1357, 1.2114, 1.2871, 1.3628, 1.4385,
        1.5142, 1.6089, 1.7035, 1.7982,
    ],
};

const KUKI_SHINOBU_SKILL_RING: TalentScaling = TalentScaling {
    name: "越祓草の輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.2524, 0.2713, 0.2903, 0.3155, 0.3344, 0.3534, 0.3786, 0.4038, 0.4291, 0.4543, 0.4796,
        0.5048, 0.5364, 0.5679, 0.5995,
    ],
};

// -- Elemental Burst: 御影裁式・霊刃 (Gyoei Narukami Kariyama Rite) -- Electro --

const KUKI_SHINOBU_BURST: TalentScaling = TalentScaling {
    name: "単発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.0360, 0.0388, 0.0415, 0.0451, 0.0478, 0.0505, 0.0541, 0.0577, 0.0613, 0.0649, 0.0685,
        0.0721, 0.0766, 0.0811, 0.0856,
    ],
};

pub const KUKI_SHINOBU: CharacterData = CharacterData {
    id: "kuki_shinobu",
    name: "Kuki Shinobu",
    element: Element::Electro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Inazuma,
    base_hp: [1030.0, 10891.0, 11438.0, 12289.0],
    base_atk: [18.0, 188.0, 198.0, 212.0],
    base_def: [63.0, 665.0, 699.0, 751.0],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "忍流飛刃斬り",
            hits: &[
                KUKI_SHINOBU_NORMAL_1,
                KUKI_SHINOBU_NORMAL_2,
                KUKI_SHINOBU_NORMAL_3,
                KUKI_SHINOBU_NORMAL_4,
            ],
            charged: &[KUKI_SHINOBU_CHARGED_1, KUKI_SHINOBU_CHARGED_2],
            plunging: &[
                KUKI_SHINOBU_PLUNGE,
                KUKI_SHINOBU_PLUNGE_LOW,
                KUKI_SHINOBU_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "越祓雷草の輪",
            scalings: &[KUKI_SHINOBU_SKILL, KUKI_SHINOBU_SKILL_RING],
        },
        elemental_burst: TalentData {
            name: "御影裁式・霊刃",
            scalings: &[KUKI_SHINOBU_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
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
    base_hp: [802.0, 8481.0, 8907.0, 9570.0],
    base_atk: [19.0, 205.0, 215.0, 232.0],
    base_def: [48.0, 508.0, 534.0, 573.0],
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

// =============================================================================
// Ororon
// =============================================================================

// -- Normal Attack: 霊器撮影 (Spiritvessel Snapshot) -- Physical --

const ORORON_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5064, 0.5476, 0.5888, 0.6477, 0.6889, 0.7360, 0.8006, 0.8652, 0.9298, 1.0007, 1.0717,
        1.1426, 1.2136, 1.2845, 1.3554,
    ],
};

const ORORON_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4440, 0.4802, 0.5164, 0.5680, 0.6042, 0.6455, 0.7022, 0.7590, 0.8157, 0.8778, 0.9399,
        1.0020, 1.0637, 1.1253, 1.1870,
    ],
};

const ORORON_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6984, 0.7552, 0.8120, 0.8932, 0.9500, 1.0150, 1.1042, 1.1934, 1.2826, 1.3802, 1.4778,
        1.5754, 1.6730, 1.7706, 1.8682,
    ],
};

// -- Aimed Shot -- Electro (charged) --

const ORORON_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const ORORON_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

// -- Plunging Attack -- Physical --

const ORORON_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const ORORON_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const ORORON_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 夜の投弾 (Night's Sling) -- Electro --

const ORORON_SKILL: TalentScaling = TalentScaling {
    name: "霊の弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.9760, 2.1242, 2.2724, 2.4700, 2.6182, 2.7664, 2.9640, 3.1616, 3.3592, 3.5568, 3.7544,
        3.9520, 4.1990, 4.4460, 4.6930,
    ],
};

// -- Elemental Burst: 闇声の反響 (Dark Voices Echo) -- Electro --

const ORORON_BURST_RITUAL: TalentScaling = TalentScaling {
    name: "儀式ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.7438, 1.8746, 2.0054, 2.1798, 2.3106, 2.4414, 2.6158, 2.7901, 2.9645, 3.1389, 3.3133,
        3.4877, 3.7057, 3.9236, 4.1416,
    ],
};

const ORORON_BURST_SOUNDWAVE: TalentScaling = TalentScaling {
    name: "音波衝突ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3320, 0.3569, 0.3818, 0.4150, 0.4399, 0.4648, 0.4980, 0.5312, 0.5644, 0.5976, 0.6308,
        0.6640, 0.7055, 0.7470, 0.7885,
    ],
};

pub const ORORON: CharacterData = CharacterData {
    id: "ororon",
    name: "Ororon",
    element: Element::Electro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Natlan,
    base_hp: [775.0, 8192.0, 8604.0, 9244.0],
    base_atk: [20.0, 216.0, 227.0, 244.0],
    base_def: [49.0, 520.0, 546.0, 587.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "霊器撮影",
            hits: &[ORORON_NORMAL_1, ORORON_NORMAL_2, ORORON_NORMAL_3],
            charged: &[ORORON_AIMED, ORORON_AIMED_FULL],
            plunging: &[ORORON_PLUNGE, ORORON_PLUNGE_LOW, ORORON_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "夜の投弾",
            scalings: &[ORORON_SKILL],
        },
        elemental_burst: TalentData {
            name: "闇声の反響",
            scalings: &[ORORON_BURST_RITUAL, ORORON_BURST_SOUNDWAVE],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Raiden Shogun
// =============================================================================

// -- Normal Attack: 源流 (Origin) -- Physical --

const RAIDEN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3965, 0.4287, 0.4610, 0.5071, 0.5394, 0.5763, 0.6270, 0.6777, 0.7284, 0.7837, 0.8471,
        0.9216, 0.9962, 1.0707, 1.1520,
    ],
};

const RAIDEN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3973, 0.4297, 0.4620, 0.5082, 0.5405, 0.5775, 0.6283, 0.6791, 0.7299, 0.7854, 0.8489,
        0.9236, 0.9983, 1.0730, 1.1545,
    ],
};

const RAIDEN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4988, 0.5394, 0.5800, 0.6380, 0.6786, 0.7250, 0.7888, 0.8526, 0.9164, 0.9860, 1.0658,
        1.1595, 1.2533, 1.3471, 1.4494,
    ],
};

const RAIDEN_NORMAL_4A: TalentScaling = TalentScaling {
    name: "4段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2898, 0.3134, 0.3370, 0.3707, 0.3943, 0.4213, 0.4583, 0.4954, 0.5325, 0.5729, 0.6192,
        0.6737, 0.7282, 0.7827, 0.8422,
    ],
};

const RAIDEN_NORMAL_4B: TalentScaling = TalentScaling {
    name: "4段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2898, 0.3134, 0.3370, 0.3707, 0.3943, 0.4213, 0.4583, 0.4954, 0.5325, 0.5729, 0.6192,
        0.6737, 0.7282, 0.7827, 0.8422,
    ],
};

const RAIDEN_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6545, 0.7077, 0.7610, 0.8371, 0.8904, 0.9513, 1.0350, 1.1187, 1.2024, 1.2937, 1.3983,
        1.5214, 1.6444, 1.7675, 1.9017,
    ],
};

// -- Charged Attack -- Physical --

const RAIDEN_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9959, 1.0769, 1.1580, 1.2738, 1.3549, 1.4475, 1.5746, 1.7023, 1.8296, 1.9686, 2.1278,
        2.3151, 2.5023, 2.6896, 2.8938,
    ],
};

// -- Plunging Attack -- Physical --

const RAIDEN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const RAIDEN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const RAIDEN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 神変・悪曜開眼 (Transcendence: Baleful Omen) -- Electro --

const RAIDEN_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.1720, 1.2599, 1.3478, 1.4650, 1.5529, 1.6408, 1.7580, 1.8752, 1.9924, 2.1096, 2.2268,
        2.3440, 2.4905, 2.6370, 2.7835,
    ],
};

const RAIDEN_SKILL_COORDINATED: TalentScaling = TalentScaling {
    name: "協同攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4200, 0.4515, 0.4830, 0.5250, 0.5565, 0.5880, 0.6300, 0.6720, 0.7140, 0.7560, 0.7980,
        0.8400, 0.8925, 0.9450, 0.9975,
    ],
};

// -- Elemental Burst: 奥義・夢想真説 (Secret Art: Musou Shinsetsu) -- Electro --

const RAIDEN_BURST_MUSOU: TalentScaling = TalentScaling {
    name: "夢想の一太刀基礎ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        4.0080, 4.3086, 4.6092, 5.0100, 5.3106, 5.6112, 6.0120, 6.4128, 6.8136, 7.2144, 7.6152,
        8.0160, 8.5170, 9.0180, 9.5190,
    ],
};

const RAIDEN_BURST_N1: TalentScaling = TalentScaling {
    name: "夢想一心1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4474, 0.4779, 0.5084, 0.5491, 0.5796, 0.6151, 0.6609, 0.7066, 0.7524, 0.7982, 0.8439,
        0.8897, 0.9354, 0.9812, 1.0269,
    ],
};

const RAIDEN_BURST_N2: TalentScaling = TalentScaling {
    name: "夢想一心2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4396, 0.4695, 0.4995, 0.5395, 0.5694, 0.6044, 0.6494, 0.6943, 0.7393, 0.7842, 0.8292,
        0.8741, 0.9191, 0.9640, 1.0090,
    ],
};

const RAIDEN_BURST_N3: TalentScaling = TalentScaling {
    name: "夢想一心3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5382, 0.5749, 0.6116, 0.6605, 0.6972, 0.7400, 0.7951, 0.8501, 0.9052, 0.9600, 1.0153,
        1.0703, 1.1254, 1.1804, 1.2355,
    ],
};

const RAIDEN_BURST_N4A: TalentScaling = TalentScaling {
    name: "夢想一心4段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3089, 0.3299, 0.3510, 0.3791, 0.4001, 0.4247, 0.4563, 0.4879, 0.5195, 0.5511, 0.5827,
        0.6143, 0.6458, 0.6774, 0.7090,
    ],
};

const RAIDEN_BURST_N4B: TalentScaling = TalentScaling {
    name: "夢想一心4段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3089, 0.3299, 0.3510, 0.3791, 0.4001, 0.4247, 0.4563, 0.4879, 0.5195, 0.5511, 0.5827,
        0.6143, 0.6458, 0.6774, 0.7090,
    ],
};

const RAIDEN_BURST_N5: TalentScaling = TalentScaling {
    name: "夢想一心5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7394, 0.7899, 0.8403, 0.9075, 0.9579, 1.0167, 1.0924, 1.1680, 1.2436, 1.3192, 1.3948,
        1.4705, 1.5461, 1.6217, 1.6973,
    ],
};

const RAIDEN_BURST_CHARGED_1: TalentScaling = TalentScaling {
    name: "夢想一心重撃ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.6160, 0.6580, 0.7000, 0.7560, 0.7980, 0.8470, 0.9100, 0.9730, 1.0360, 1.0990, 1.1620,
        1.2250, 1.2880, 1.3510, 1.4140,
    ],
};

const RAIDEN_BURST_CHARGED_2: TalentScaling = TalentScaling {
    name: "夢想一心重撃ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7436, 0.7943, 0.8450, 0.9126, 0.9633, 1.0225, 1.0985, 1.1746, 1.2506, 1.3267, 1.4027,
        1.4788, 1.5548, 1.6309, 1.7069,
    ],
};

pub const RAIDEN_SHOGUN: CharacterData = CharacterData {
    id: "raiden_shogun",
    name: "Raiden Shogun",
    element: Element::Electro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [1005.0, 11388.0, 12000.0, 12907.0],
    base_atk: [26.0, 298.0, 314.0, 337.0],
    base_def: [61.0, 696.0, 734.0, 789.0],
    ascension_stat: AscensionStat::EnergyRecharge(0.32),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "源流",
            hits: &[
                RAIDEN_NORMAL_1,
                RAIDEN_NORMAL_2,
                RAIDEN_NORMAL_3,
                RAIDEN_NORMAL_4A,
                RAIDEN_NORMAL_4B,
                RAIDEN_NORMAL_5,
            ],
            charged: &[RAIDEN_CHARGED],
            plunging: &[RAIDEN_PLUNGE, RAIDEN_PLUNGE_LOW, RAIDEN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "神変・悪曜開眼",
            scalings: &[RAIDEN_SKILL, RAIDEN_SKILL_COORDINATED],
        },
        elemental_burst: TalentData {
            name: "奥義・夢想真説",
            scalings: &[
                RAIDEN_BURST_MUSOU,
                RAIDEN_BURST_N1,
                RAIDEN_BURST_N2,
                RAIDEN_BURST_N3,
                RAIDEN_BURST_N4A,
                RAIDEN_BURST_N4B,
                RAIDEN_BURST_N5,
                RAIDEN_BURST_CHARGED_1,
                RAIDEN_BURST_CHARGED_2,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Razor
// =============================================================================

// -- Normal Attack: 鉄の牙 (Steel Fang) -- Physical --

const RAZOR_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9592, 1.0246, 1.0900, 1.1772, 1.2426, 1.3189, 1.4170, 1.5151, 1.6132, 1.7113, 1.8094,
        1.9075, 2.0056, 2.1037, 2.2018,
    ],
};

const RAZOR_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8263, 0.8827, 0.9390, 1.0141, 1.0705, 1.1362, 1.2207, 1.3052, 1.3897, 1.4742, 1.5587,
        1.6433, 1.7278, 1.8123, 1.8968,
    ],
};

const RAZOR_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0331, 1.1036, 1.1740, 1.2679, 1.3384, 1.4205, 1.5262, 1.6319, 1.7375, 1.8432, 1.9488,
        2.0545, 2.1602, 2.2658, 2.3715,
    ],
};

const RAZOR_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.3605, 1.4532, 1.5460, 1.6697, 1.7624, 1.8707, 2.0098, 2.1489, 2.2881, 2.4272, 2.5664,
        2.7055, 2.8446, 2.9838, 3.1229,
    ],
};

// -- Charged Attack -- Physical --

const RAZOR_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6254, 0.6763, 0.7272, 0.7999, 0.8508, 0.9090, 0.9890, 1.0690, 1.1490, 1.2362, 1.3235,
        1.4108, 1.4980, 1.5853, 1.6726,
    ],
};

const RAZOR_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1309, 1.2230, 1.3150, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933,
        2.5511, 2.7089, 2.8667, 3.0245,
    ],
};

// -- Plunging Attack -- Physical --

const RAZOR_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8205, 0.8872, 0.9540, 1.0494, 1.1162, 1.1925, 1.2975, 1.4024, 1.5074, 1.6219, 1.7363,
        1.8508, 1.9653, 2.0798, 2.1943,
    ],
};

const RAZOR_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.6406, 1.7741, 1.9077, 2.0984, 2.2320, 2.3846, 2.5944, 2.8043, 3.0141, 3.2430, 3.4719,
        3.7009, 3.9298, 4.1587, 4.3876,
    ],
};

const RAZOR_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        2.0492, 2.2160, 2.3828, 2.6210, 2.7878, 2.9785, 3.2406, 3.5027, 3.7648, 4.0507, 4.3366,
        4.6226, 4.9085, 5.1944, 5.4804,
    ],
};

// -- Elemental Skill: 爪雷 (Claw and Thunder) -- Electro --

const RAZOR_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "一回押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.9920, 2.1414, 2.2908, 2.4900, 2.6394, 2.7888, 2.9880, 3.1872, 3.3864, 3.5856, 3.7848,
        3.9840, 4.2330, 4.4820, 4.7310,
    ],
};

const RAZOR_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        2.9520, 3.1734, 3.3948, 3.6900, 3.9114, 4.1328, 4.4280, 4.7232, 5.0184, 5.3136, 5.6088,
        5.9040, 6.2730, 6.6420, 7.0110,
    ],
};

// -- Elemental Burst: 雷牙 (Lightning Fang) -- Electro --

const RAZOR_BURST: TalentScaling = TalentScaling {
    name: "元素爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.6000, 1.7200, 1.8400, 2.0000, 2.1200, 2.2400, 2.4000, 2.5600, 2.7200, 2.8800, 3.0400,
        3.2000, 3.4000, 3.6000, 3.8000,
    ],
};

const RAZOR_BURST_SOUL_COMPANION: TalentScaling = TalentScaling {
    name: "狼魂ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.2400, 0.2580, 0.2760, 0.3000, 0.3180, 0.3360, 0.3600, 0.3840, 0.4080, 0.4320, 0.4560,
        0.4800, 0.5100, 0.5400, 0.5700,
    ],
};

pub const RAZOR: CharacterData = CharacterData {
    id: "razor",
    name: "Razor",
    element: Element::Electro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [1003.0, 10602.0, 11134.0, 11962.0],
    base_atk: [20.0, 207.0, 217.0, 234.0],
    base_def: [63.0, 665.0, 699.0, 751.0],
    ascension_stat: AscensionStat::PhysicalDmgBonus(0.30),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "鉄の牙",
            hits: &[
                RAZOR_NORMAL_1,
                RAZOR_NORMAL_2,
                RAZOR_NORMAL_3,
                RAZOR_NORMAL_4,
            ],
            charged: &[RAZOR_CHARGED_SPINNING, RAZOR_CHARGED_FINAL],
            plunging: &[RAZOR_PLUNGE, RAZOR_PLUNGE_LOW, RAZOR_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "爪雷",
            scalings: &[RAZOR_SKILL_PRESS, RAZOR_SKILL_HOLD],
        },
        elemental_burst: TalentData {
            name: "雷牙",
            scalings: &[RAZOR_BURST, RAZOR_BURST_SOUL_COMPANION],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Sethos
// =============================================================================

// -- Normal Attack: 王の鉤矢射法 (Royal Reed Archery) -- Physical --

const SETHOS_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5261, 0.5690, 0.6120, 0.6732, 0.7162, 0.7650, 0.8323, 0.8997, 0.9670, 1.0404, 1.1138,
        1.1871, 1.2605, 1.3338, 1.4072,
    ],
};

const SETHOS_NORMAL_2A: TalentScaling = TalentScaling {
    name: "2段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2380, 0.2574, 0.2768, 0.3045, 0.3239, 0.3460, 0.3765, 0.4070, 0.4375, 0.4706, 0.5038,
        0.5369, 0.5700, 0.6031, 0.6362,
    ],
};

const SETHOS_NORMAL_2B: TalentScaling = TalentScaling {
    name: "2段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2662, 0.2879, 0.3095, 0.3405, 0.3621, 0.3869, 0.4210, 0.4551, 0.4891, 0.5263, 0.5634,
        0.6005, 0.6376, 0.6748, 0.7119,
    ],
};

const SETHOS_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7400, 0.8002, 0.8604, 0.9464, 1.0066, 1.0755, 1.1702, 1.2649, 1.3596, 1.4628, 1.5660,
        1.6692, 1.7724, 1.8756, 1.9788,
    ],
};

// -- Aimed Shot -- Electro (charged) --

const SETHOS_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const SETHOS_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

const SETHOS_SHADOWPIERCING: TalentScaling = TalentScaling {
    name: "影刺し狙い撃ちダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.4000, 1.5050, 1.6100, 1.7500, 1.8550, 1.9600, 2.1000, 2.2400, 2.3800, 2.5200, 2.6600,
        2.8000, 2.9750, 3.1500, 3.3250,
    ],
};

// -- Plunging Attack -- Physical --

const SETHOS_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const SETHOS_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const SETHOS_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 古式・甕の秘儀 (Ancient Rite: The Thundering Sands) -- Electro --

const SETHOS_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.1560, 1.2427, 1.3294, 1.4450, 1.5317, 1.6184, 1.7340, 1.8496, 1.9652, 2.0808, 2.1964,
        2.3120, 2.4565, 2.6010, 2.7455,
    ],
};

// -- Elemental Burst: 秘儀・甕の夕暮れ (Secret Rite: Twilight Shadowpiercer) -- Electro --

const SETHOS_BURST: TalentScaling = TalentScaling {
    name: "暮の雷弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.9620, 2.1094, 2.2568, 2.4525, 2.5999, 2.7474, 2.9430, 3.1387, 3.3344, 3.5301, 3.7258,
        3.9214, 4.1665, 4.4116, 4.6567,
    ],
};

pub const SETHOS: CharacterData = CharacterData {
    id: "sethos",
    name: "Sethos",
    element: Element::Electro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Sumeru,
    base_hp: [821.0, 8674.0, 9110.0, 9787.0],
    base_atk: [19.0, 201.0, 212.0, 227.0],
    base_def: [47.0, 496.0, 521.0, 560.0],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "王の鉤矢射法",
            hits: &[
                SETHOS_NORMAL_1,
                SETHOS_NORMAL_2A,
                SETHOS_NORMAL_2B,
                SETHOS_NORMAL_3,
            ],
            charged: &[SETHOS_AIMED, SETHOS_AIMED_FULL, SETHOS_SHADOWPIERCING],
            plunging: &[SETHOS_PLUNGE, SETHOS_PLUNGE_LOW, SETHOS_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "古式・甕の秘儀",
            scalings: &[SETHOS_SKILL],
        },
        elemental_burst: TalentData {
            name: "秘儀・甕の夕暮れ",
            scalings: &[SETHOS_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Varesa
// =============================================================================

// -- Normal Attack: 角撃ち (By the Horns) -- All Electro (Catalyst) --
// Standard State

const VARESA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4680, 0.5030, 0.5400, 0.5850, 0.6200, 0.6550, 0.7020, 0.7480, 0.7950, 0.8420, 0.8890,
        0.9360, 0.9940, 1.0530, 1.1110,
    ],
};

const VARESA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4000, 0.4300, 0.4600, 0.5000, 0.5300, 0.5600, 0.6000, 0.6400, 0.6800, 0.7210, 0.7610,
        0.8010, 0.8510, 0.9010, 0.9510,
    ],
};

const VARESA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5630, 0.6050, 0.6480, 0.7040, 0.7460, 0.7880, 0.8450, 0.9010, 0.9570, 1.0140, 1.0700,
        1.1260, 1.1970, 1.2670, 1.3370,
    ],
};

// -- Fiery Passion State --

const VARESA_PASSION_NORMAL_1: TalentScaling = TalentScaling {
    name: "炎情1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5440, 0.5850, 0.6260, 0.6800, 0.7210, 0.7620, 0.8160, 0.8710, 0.9250, 0.9790, 1.0340,
        1.0880, 1.1560, 1.2240, 1.2920,
    ],
};

const VARESA_PASSION_NORMAL_2: TalentScaling = TalentScaling {
    name: "炎情2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5200, 0.5600, 0.5980, 0.6500, 0.6890, 0.7280, 0.7800, 0.8320, 0.8840, 0.9370, 0.9890,
        1.0410, 1.1060, 1.1710, 1.2360,
    ],
};

const VARESA_PASSION_NORMAL_3: TalentScaling = TalentScaling {
    name: "炎情3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7360, 0.7910, 0.8460, 0.9200, 0.9750, 1.0300, 1.1040, 1.1770, 1.2510, 1.3250, 1.3980,
        1.4720, 1.5640, 1.6560, 1.7480,
    ],
};

// -- Charged Attack -- Electro --

const VARESA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.8930, 0.9600, 1.0270, 1.1160, 1.1830, 1.2500, 1.3390, 1.4280, 1.5180, 1.6070, 1.6960,
        1.7860, 1.8970, 2.0090, 2.1200,
    ],
};

const VARESA_PASSION_CHARGED: TalentScaling = TalentScaling {
    name: "炎情重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.9260, 0.9960, 1.0650, 1.1580, 1.2270, 1.2970, 1.3900, 1.4820, 1.5750, 1.6780, 1.7600,
        1.8530, 1.9690, 2.0840, 2.2000,
    ],
};

// -- Plunging Attack -- Electro (Catalyst) --

const VARESA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7459, 0.8066, 0.8673, 0.9541, 1.0148, 1.0841, 1.1795, 1.2749, 1.3702, 1.4744, 1.5786,
        1.6827, 1.7869, 1.8910, 1.9952,
    ],
};

const VARESA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.4914, 1.6128, 1.7342, 1.9076, 2.0290, 2.1678, 2.3586, 2.5494, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const VARESA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5345, 2.7078, 2.9460, 3.1843, 3.4225, 3.6824, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
};

// Fiery Passion plunge variants (higher damage)
const VARESA_PASSION_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "炎情低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        2.2370, 2.4190, 2.6010, 2.8610, 3.0440, 3.2520, 3.5380, 3.8240, 4.1100, 4.4220, 4.7340,
        5.0470, 5.3590, 5.6710, 5.9830,
    ],
};

const VARESA_PASSION_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "炎情高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        2.7940, 3.0220, 3.2490, 3.5740, 3.8020, 4.0620, 4.4190, 4.7760, 5.1340, 5.5240, 5.9140,
        6.3030, 6.6930, 7.0830, 7.4730,
    ],
};

// -- Elemental Skill: 夜虹乗り (Riding the Night-Rainbow) -- Electro --

const VARESA_SKILL_RUSH: TalentScaling = TalentScaling {
    name: "突進ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7450, 0.8010, 0.8570, 0.9310, 0.9870, 1.0430, 1.1170, 1.1920, 1.2660, 1.3410, 1.4150,
        1.4900, 1.5830, 1.6760, 1.7690,
    ],
};

const VARESA_SKILL_PASSION_RUSH: TalentScaling = TalentScaling {
    name: "炎情突進ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.0640, 1.1440, 1.2240, 1.3300, 1.4100, 1.4900, 1.5960, 1.7020, 1.8090, 1.9150, 2.0220,
        2.1280, 2.2610, 2.3940, 2.5270,
    ],
};

// -- Elemental Burst: ガーディアンベント! (Guardian Vent!) -- Electro --

const VARESA_BURST_KICK: TalentScaling = TalentScaling {
    name: "フライングキック",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        3.4510, 3.7100, 3.9690, 4.3140, 4.5730, 4.8320, 5.1770, 5.2190, 5.8670, 6.2120, 6.5570,
        6.9020, 7.3340, 7.6550, 8.1970,
    ],
};

const VARESA_BURST_PASSION_KICK: TalentScaling = TalentScaling {
    name: "炎情フライングキック",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        5.7520, 6.1830, 6.6150, 7.1900, 7.6210, 8.0530, 8.6280, 9.2030, 9.7780, 10.3540, 10.9290,
        11.5040, 12.2230, 12.9420, 13.6610,
    ],
};

const VARESA_BURST_VOLCANO: TalentScaling = TalentScaling {
    name: "ボルケーノカブラム",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        4.0260, 4.3280, 4.6300, 5.0330, 5.3350, 5.6370, 6.0400, 6.4420, 6.8450, 7.2480, 7.6500,
        8.0530, 8.5560, 9.0590, 9.5630,
    ],
};

pub const VARESA: CharacterData = CharacterData {
    id: "varesa",
    name: "Varesa",
    element: Element::Electro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [989.0, 11204.0, 11806.0, 12699.0],
    base_atk: [28.0, 314.0, 331.0, 356.0],
    base_def: [61.0, 690.0, 727.0, 782.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "角撃ち",
            hits: &[
                VARESA_NORMAL_1,
                VARESA_NORMAL_2,
                VARESA_NORMAL_3,
                VARESA_PASSION_NORMAL_1,
                VARESA_PASSION_NORMAL_2,
                VARESA_PASSION_NORMAL_3,
            ],
            charged: &[VARESA_CHARGED, VARESA_PASSION_CHARGED],
            plunging: &[
                VARESA_PLUNGE,
                VARESA_PLUNGE_LOW,
                VARESA_PLUNGE_HIGH,
                VARESA_PASSION_PLUNGE_LOW,
                VARESA_PASSION_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "夜虹乗り",
            scalings: &[VARESA_SKILL_RUSH, VARESA_SKILL_PASSION_RUSH],
        },
        elemental_burst: TalentData {
            name: "ガーディアンベント!",
            scalings: &[
                VARESA_BURST_KICK,
                VARESA_BURST_PASSION_KICK,
                VARESA_BURST_VOLCANO,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Yae Miko
// =============================================================================

// -- Normal Attack: 狐霊食罪式 (Spiritfox Sin-Eater) -- All Electro (Catalyst) --

const YAE_MIKO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3966, 0.4263, 0.4561, 0.4957, 0.5255, 0.5552, 0.5949, 0.6345, 0.6742, 0.7139, 0.7535,
        0.7932, 0.8427, 0.8923, 0.9419,
    ],
};

const YAE_MIKO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3852, 0.4141, 0.4430, 0.4815, 0.5104, 0.5393, 0.5778, 0.6163, 0.6548, 0.6933, 0.7319,
        0.7704, 0.8185, 0.8667, 0.9148,
    ],
};

const YAE_MIKO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5689, 0.6116, 0.6542, 0.7111, 0.7538, 0.7964, 0.8533, 0.9102, 0.9671, 1.0240, 1.0809,
        1.1378, 1.2089, 1.2800, 1.3511,
    ],
};

// -- Charged Attack -- Electro --

const YAE_MIKO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.4289, 1.5361, 1.6433, 1.7862, 1.8934, 2.0005, 2.1434, 2.2863, 2.4292, 2.5721, 2.7150,
        2.8579, 3.0365, 3.2151, 3.3938,
    ],
};

// -- Plunging Attack -- Electro (Catalyst) --

const YAE_MIKO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const YAE_MIKO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const YAE_MIKO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 野干役呪・殺生桜 (Yakan Evocation: Sesshou Sakura) -- Electro --

const YAE_MIKO_SKILL_LV1: TalentScaling = TalentScaling {
    name: "殺生桜ダメージ・壱階",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.6067, 0.6522, 0.6977, 0.7584, 0.8039, 0.8494, 0.9101, 0.9708, 1.0314, 1.0921, 1.1528,
        1.2134, 1.2893, 1.3651, 1.4410,
    ],
};

const YAE_MIKO_SKILL_LV2: TalentScaling = TalentScaling {
    name: "殺生桜ダメージ・弐階",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7584, 0.8153, 0.8722, 0.9480, 1.0049, 1.0618, 1.1376, 1.2134, 1.2893, 1.3651, 1.4410,
        1.5168, 1.6116, 1.7064, 1.8012,
    ],
};

const YAE_MIKO_SKILL_LV3: TalentScaling = TalentScaling {
    name: "殺生桜ダメージ・参階",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.9480, 1.0191, 1.0902, 1.1850, 1.2561, 1.3272, 1.4220, 1.5168, 1.6116, 1.7064, 1.8012,
        1.8960, 2.0145, 2.1330, 2.2150,
    ],
};

const YAE_MIKO_SKILL_LV4: TalentScaling = TalentScaling {
    name: "殺生桜ダメージ・肆階",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.1850, 1.2739, 1.3628, 1.4813, 1.5701, 1.6590, 1.7775, 1.8960, 2.0145, 2.1330, 2.2515,
        2.3700, 2.5181, 2.6663, 2.8144,
    ],
};

// -- Elemental Burst: 大密法・天狐顕真 (Great Secret Art: Tenko Kenshin) -- Electro --

const YAE_MIKO_BURST_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        2.6000, 2.7950, 2.9900, 3.2500, 3.4450, 3.6400, 3.9000, 4.1600, 4.4200, 4.6800, 4.9400,
        5.2000, 5.5250, 5.8500, 6.1750,
    ],
};

const YAE_MIKO_BURST_TENKO: TalentScaling = TalentScaling {
    name: "天狐雷霆ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        3.3382, 3.5885, 3.8389, 4.1727, 4.4231, 4.6734, 5.0072, 5.3411, 5.6749, 6.0087, 6.3425,
        6.6763, 7.0936, 7.5109, 7.9281,
    ],
};

pub const YAE_MIKO: CharacterData = CharacterData {
    id: "yae_miko",
    name: "Yae Miko",
    element: Element::Electro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [807.0, 9151.0, 9643.0, 10372.0],
    base_atk: [26.0, 300.0, 316.0, 340.0],
    base_def: [44.0, 502.0, 529.0, 569.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "狐霊食罪式",
            hits: &[YAE_MIKO_NORMAL_1, YAE_MIKO_NORMAL_2, YAE_MIKO_NORMAL_3],
            charged: &[YAE_MIKO_CHARGED],
            plunging: &[YAE_MIKO_PLUNGE, YAE_MIKO_PLUNGE_LOW, YAE_MIKO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "野干役呪・殺生桜",
            scalings: &[
                YAE_MIKO_SKILL_LV1,
                YAE_MIKO_SKILL_LV2,
                YAE_MIKO_SKILL_LV3,
                YAE_MIKO_SKILL_LV4,
            ],
        },
        elemental_burst: TalentData {
            name: "大密法・天狐顕真",
            scalings: &[YAE_MIKO_BURST_SKILL, YAE_MIKO_BURST_TENKO],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
