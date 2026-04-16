use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CLORINDE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5163, 0.5583, 0.6003, 0.6604, 0.7024, 0.7504, 0.8164, 0.8825, 0.9485, 1.0206, 1.0926,
        1.1646, 1.2367, 1.3087, 1.3808,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CLORINDE_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3419, 0.3697, 0.3975, 0.4373, 0.4651, 0.4969, 0.5406, 0.5843, 0.6281, 0.6758, 0.7235,
        0.7712, 0.8189, 0.8666, 0.9143,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CLORINDE_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3419, 0.3697, 0.3975, 0.4373, 0.4651, 0.4969, 0.5406, 0.5843, 0.6281, 0.6758, 0.7235,
        0.7712, 0.8189, 0.8666, 0.9143,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CLORINDE_NORMAL_4A: TalentScaling = TalentScaling {
    name: "4段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2313, 0.2502, 0.2690, 0.2959, 0.3147, 0.3363, 0.3658, 0.3954, 0.4250, 0.4573, 0.4896,
        0.5219, 0.5541, 0.5864, 0.6187,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CLORINDE_NORMAL_4B: TalentScaling = TalentScaling {
    name: "4段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2313, 0.2502, 0.2690, 0.2959, 0.3147, 0.3363, 0.3658, 0.3954, 0.4250, 0.4573, 0.4896,
        0.5219, 0.5541, 0.5864, 0.6187,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CLORINDE_NORMAL_4C: TalentScaling = TalentScaling {
    name: "4段ダメージ (3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2313, 0.2502, 0.2690, 0.2959, 0.3147, 0.3363, 0.3658, 0.3954, 0.4250, 0.4573, 0.4896,
        0.5219, 0.5541, 0.5864, 0.6187,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CLORINDE_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9001, 0.9734, 1.0466, 1.1513, 1.2246, 1.3083, 1.4234, 1.5385, 1.6537, 1.7793, 1.9049,
        2.0305, 2.1561, 2.2817, 2.4072,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CLORINDE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CLORINDE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CLORINDE_SKILL_SWIFT_HUNT_PIERCE: TalentScaling = TalentScaling {
    name: "迅捷の追撃貫通ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3879, 0.4194, 0.4510, 0.4961, 0.5277, 0.5638, 0.6134, 0.6630, 0.7126, 0.7667, 0.8208,
        0.8749, 0.9291, 0.9832, 1.0373,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CLORINDE_SKILL_IMPALE: TalentScaling = TalentScaling {
    name: "夜巡りの一刺しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4396, 0.4754, 0.5112, 0.5623, 0.5981, 0.6390, 0.6952, 0.7515, 0.8077, 0.8690, 0.9304,
        0.9917, 1.0531, 1.1144, 1.1758,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CLORINDE_SKILL_SURGING_BLADE: TalentScaling = TalentScaling {
    name: "流湧の刃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4320, 0.4644, 0.4968, 0.5400, 0.5724, 0.6048, 0.6480, 0.6912, 0.7344, 0.7776, 0.8208,
        0.8640, 0.9180, 0.9720, 1.0260,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const CLORINDE: CharacterData = CharacterData {
    id: "clorinde",
    name: "Clorinde",
    element: Element::Electro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [
        1009.00, 2616.00, 3481.00, 5209.00, 5823.00, 6700.00, 7519.00, 8405.00, 9019.00, 9913.00,
        10527.00, 11431.00, 12045.00, 12956.00, 12956.00, 13474.24, // Lv95/Lv95+/Lv100
        13474.24, // Lv95/Lv95+/Lv100
        13992.48, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        26.25, 68.10, 90.61, 135.59, 151.58, 174.39, 195.72, 218.77, 234.76, 258.02, 274.02,
        297.54, 313.53, 337.24, 337.24, 350.73, // Lv95/Lv95+/Lv100
        350.73, // Lv95/Lv95+/Lv100
        364.22, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        61.03, 158.30, 210.63, 315.17, 352.35, 405.38, 454.95, 508.53, 545.71, 599.78, 636.96,
        691.64, 728.82, 783.93, 783.93, 815.29, // Lv95/Lv95+/Lv100
        815.29, // Lv95/Lv95+/Lv100
        846.64, // Lv95/Lv95+/Lv100
    ],
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
    passive_scalings: &[],
    scaling_modifiers: &[],
};
