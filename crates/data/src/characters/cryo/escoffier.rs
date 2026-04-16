use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ESCOFFIER_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4759, 0.5147, 0.5534, 0.6088, 0.6475, 0.6918, 0.7526, 0.8135, 0.8744, 0.9408, 1.0072,
        1.0736, 1.1400, 1.2064, 1.2728,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ESCOFFIER_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3300, 0.3569, 0.3837, 0.4221, 0.4489, 0.4796, 0.5219, 0.5641, 0.6063, 0.6523, 0.6984,
        0.7444, 0.7905, 0.8365, 0.8825,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ESCOFFIER_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4033, 0.4362, 0.4690, 0.5159, 0.5487, 0.5862, 0.6378, 0.6894, 0.7410, 0.7973, 0.8536,
        0.9098, 0.9661, 1.0224, 1.0787,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ESCOFFIER_PLUNGE_LOW: TalentScaling = TalentScaling {
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

const ESCOFFIER_PLUNGE_HIGH: TalentScaling = TalentScaling {
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

// -- Elemental Skill: Low-Temperature Cooking -- Cryo --

const ESCOFFIER_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5040, 0.5418, 0.5796, 0.6300, 0.6678, 0.7056, 0.7560, 0.8064, 0.8568, 0.9072, 0.9576,
        1.0080, 1.0710, 1.1340, 1.1970,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ESCOFFIER_SKILL_PARFAIT: TalentScaling = TalentScaling {
    name: "フロスティパフェダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2000, 1.2900, 1.3800, 1.5000, 1.5900, 1.6800, 1.8000, 1.9200, 2.0400, 2.1600, 2.2800,
        2.4000, 2.5500, 2.7000, 2.8500,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ESCOFFIER_SKILL_SURGING: TalentScaling = TalentScaling {
    name: "サージングブレードダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.3360, 0.3612, 0.3864, 0.4200, 0.4452, 0.4704, 0.5040, 0.5376, 0.5712, 0.6048, 0.6384,
        0.6720, 0.7140, 0.7560, 0.7980,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const ESCOFFIER: CharacterData = CharacterData {
    id: "escoffier",
    name: "Escoffier",
    element: Element::Cryo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [
        1039.00, 2695.00, 3586.00, 5366.00, 5999.00, 6902.00, 7747.00, 8659.00, 9292.00, 10213.00,
        10846.00, 11777.00, 12410.00, 13348.00, 13348.00, 13881.92, // Lv95/Lv95+/Lv100
        13881.92, // Lv95/Lv95+/Lv100
        14415.84, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        27.00, 70.04, 93.18, 139.43, 155.88, 179.34, 201.27, 224.98, 241.42, 265.34, 281.79,
        305.98, 322.43, 346.81, 346.81, 360.68, // Lv95/Lv95+/Lv100
        360.68, // Lv95/Lv95+/Lv100
        374.55, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        56.96, 147.75, 196.59, 294.16, 328.86, 378.35, 424.62, 474.63, 509.33, 559.80, 594.50,
        645.53, 680.23, 731.66, 731.66, 760.93, // Lv95/Lv95+/Lv100
        760.93, // Lv95/Lv95+/Lv100
        790.19, // Lv95/Lv95+/Lv100
    ],
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
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
};

// =============================================================================
// Eula
