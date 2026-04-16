use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CYNO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4792, 0.5182, 0.5572, 0.6129, 0.6519, 0.6965, 0.7578, 0.8191, 0.8804, 0.9473, 1.0141,
        1.0810, 1.1479, 1.2147, 1.2816,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CYNO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5858, 0.6335, 0.6812, 0.7493, 0.7970, 0.8515, 0.9264, 1.0013, 1.0762, 1.1586, 1.2404,
        1.3222, 1.4040, 1.4858, 1.5675,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CYNO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7588, 0.8206, 0.8824, 0.9706, 1.0324, 1.1030, 1.2001, 1.2971, 1.3942, 1.5001, 1.6061,
        1.7120, 1.8180, 1.9239, 2.0299,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CYNO_PLUNGE_LOW: TalentScaling = TalentScaling {
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

const CYNO_PLUNGE_HIGH: TalentScaling = TalentScaling {
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

// -- Elemental Skill: 秘儀・律淵の渡し (Secret Rite: Chasmic Soulfarer) -- Electro --

const CYNO_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.3040, 1.4018, 1.4996, 1.6300, 1.7278, 1.8256, 1.9560, 2.0864, 2.2168, 2.3472, 2.4776,
        2.6080, 2.7710, 2.9340, 3.0970,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CYNO_SKILL_MORTUARY: TalentScaling = TalentScaling {
    name: "殯儀の秘儀ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.5680, 1.6856, 1.8032, 1.9600, 2.0776, 2.1952, 2.3520, 2.5088, 2.6656, 2.8224, 2.9792,
        3.1360, 3.3320, 3.5280, 3.7240,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CYNO_BURST_NORMAL_2: TalentScaling = TalentScaling {
    name: "末途の一撃2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.8246, 0.8920, 0.9593, 1.0553, 1.1226, 1.1991, 1.3043, 1.4095, 1.5147, 1.6298, 1.7450,
        1.8601, 1.9752, 2.0904, 2.2055,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CYNO_BURST_NORMAL_3: TalentScaling = TalentScaling {
    name: "末途の一撃3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.0462, 1.1316, 1.2170, 1.3387, 1.4241, 1.5213, 1.6551, 1.7889, 1.9226, 2.0688, 2.2140,
        2.3600, 2.5060, 2.6520, 2.7980,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CYNO_BURST_NORMAL_4: TalentScaling = TalentScaling {
    name: "末途の一撃4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.0340, 1.1184, 1.2028, 1.3231, 1.4075, 1.5035, 1.6358, 1.7680, 1.9003, 2.0447, 2.1896,
        2.3324, 2.4766, 2.6208, 2.7650,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CYNO_BURST_NORMAL_5: TalentScaling = TalentScaling {
    name: "末途の一撃5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.3080, 1.4148, 1.5216, 1.6738, 1.7806, 1.9020, 2.0694, 2.2367, 2.4040, 2.5867, 2.7698,
        2.9516, 3.1347, 3.3165, 3.4990,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CYNO_BURST_CHARGED: TalentScaling = TalentScaling {
    name: "末途の一撃重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.0106, 1.0932, 1.1753, 1.2929, 1.3751, 1.4694, 1.5984, 1.7271, 1.8570, 1.9976, 2.1388,
        2.2794, 2.4206, 2.5618, 2.7030,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const CYNO: CharacterData = CharacterData {
    id: "cyno",
    name: "Cyno",
    element: Element::Electro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Sumeru,
    base_hp: [
        972.00, 2522.00, 3356.00, 5022.00, 5614.00, 6459.00, 7249.00, 8103.00, 8695.00, 9557.00,
        10149.00, 11020.00, 11613.00, 12491.00, 12491.00, 12990.64, // Lv95/Lv95+/Lv100
        12990.64, // Lv95/Lv95+/Lv100
        13490.28, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        24.76, 64.24, 85.47, 127.89, 142.98, 164.50, 184.61, 206.36, 221.44, 243.38, 258.47,
        280.66, 295.74, 318.11, 318.11, 330.83, // Lv95/Lv95+/Lv100
        330.83, // Lv95/Lv95+/Lv100
        343.56, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        66.89, 173.51, 230.87, 345.45, 386.20, 444.33, 498.66, 557.39, 598.14, 657.41, 698.16,
        758.09, 798.84, 859.24, 859.24, 893.61, // Lv95/Lv95+/Lv100
        893.61, // Lv95/Lv95+/Lv100
        927.98, // Lv95/Lv95+/Lv100
    ],
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
