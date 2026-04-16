use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Chiori
// =============================================================================

// -- Normal Attack: 裁錦キ法 (Weaving Blade) -- Physical --

const CHIORI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4941, 0.5343, 0.5745, 0.6320, 0.6722, 0.7181, 0.7813, 0.8445, 0.9077, 0.9766, 1.0456,
        1.1145, 1.1835, 1.2524, 1.3214,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHIORI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4685, 0.5066, 0.5448, 0.5993, 0.6374, 0.6810, 0.7409, 0.8009, 0.8609, 0.9262, 0.9914,
        1.0567, 1.1220, 1.1873, 1.2525,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHIORI_NORMAL_3_1: TalentScaling = TalentScaling {
    name: "3段ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3042, 0.3289, 0.3537, 0.3890, 0.4138, 0.4421, 0.4810, 0.5199, 0.5588, 0.6013, 0.6437,
        0.6861, 0.7286, 0.7710, 0.8135,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHIORI_NORMAL_3_2: TalentScaling = TalentScaling {
    name: "3段ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3042, 0.3289, 0.3537, 0.3890, 0.4138, 0.4421, 0.4810, 0.5199, 0.5588, 0.6013, 0.6437,
        0.6861, 0.7286, 0.7710, 0.8135,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHIORI_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7513, 0.8124, 0.8735, 0.9609, 1.0220, 1.0919, 1.1881, 1.2843, 1.3805, 1.4852, 1.5898,
        1.6944, 1.7991, 1.9037, 2.0083,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const CHIORI_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5431, 0.5873, 0.6315, 0.6947, 0.7389, 0.7894, 0.8588, 0.9283, 0.9978, 1.0736, 1.1493,
        1.2251, 1.3009, 1.3767, 1.4525,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHIORI_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5431, 0.5873, 0.6315, 0.6947, 0.7389, 0.7894, 0.8588, 0.9283, 0.9978, 1.0736, 1.1493,
        1.2251, 1.3009, 1.3767, 1.4525,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const CHIORI_PLUNGE: TalentScaling = TalentScaling {
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

const CHIORI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5271, 2.7055,
        2.8840, 3.0624, 3.2409, 3.4193,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHIORI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6021, 3.8249, 4.0478, 4.2706,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 羽袖キ法・糸結 (Fluttering Hasode) -- Geo --

const CHIORI_SKILL_TAMOTO: TalentScaling = TalentScaling {
    name: "袂自動攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.8208, 0.8824, 0.9439, 1.0260, 1.0876, 1.1491, 1.2312, 1.3133, 1.3954, 1.4774, 1.5595,
        1.6416, 1.7442, 1.8468, 1.9494,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHIORI_SKILL_TAMOTO_DEF: TalentScaling = TalentScaling {
    name: "袂自動攻撃ダメージ(DEF)",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        1.0260, 1.1030, 1.1799, 1.2825, 1.3595, 1.4364, 1.5390, 1.6416, 1.7442, 1.8468, 1.9494,
        2.0520, 2.1803, 2.3085, 2.4368,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHIORI_SKILL_TURRET: TalentScaling = TalentScaling {
    name: "袂飛び道具ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.4928, 1.6048, 1.7168, 1.8660, 1.9780, 2.0899, 2.2392, 2.3885, 2.5378, 2.6870, 2.8363,
        2.9856, 3.1722, 3.3588, 3.5454,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHIORI_SKILL_UPWARD_SWEEP: TalentScaling = TalentScaling {
    name: "上拂スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.4928, 1.6048, 1.7168, 1.8660, 1.9780, 2.0899, 2.2392, 2.3885, 2.5378, 2.6870, 2.8363,
        2.9856, 3.1722, 3.3588, 3.5454,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHIORI_SKILL_UPWARD_SWEEP_DEF: TalentScaling = TalentScaling {
    name: "上拂スキルダメージ(DEF)",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        1.8660, 2.0060, 2.1459, 2.3325, 2.4725, 2.6124, 2.7990, 2.9856, 3.1722, 3.3588, 3.5454,
        3.7320, 3.9653, 4.1985, 4.4318,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 二刀キ法・緋反 (Hiyoku: Twin Blades) -- Geo --

const CHIORI_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        2.5632, 2.7554, 2.9477, 3.2040, 3.3962, 3.5885, 3.8448, 4.1011, 4.3574, 4.6138, 4.8701,
        5.1264, 5.4468, 5.7672, 6.0876,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHIORI_BURST_DEF: TalentScaling = TalentScaling {
    name: "スキルダメージ(DEF)",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        3.2040, 3.4443, 3.6846, 4.0050, 4.2453, 4.4856, 4.8060, 5.1264, 5.4468, 5.7672, 6.0876,
        6.4080, 6.8085, 7.2090, 7.6095,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const CHIORI: CharacterData = CharacterData {
    id: "chiori",
    name: "Chiori",
    element: Element::Geo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [
        890.00, 2310.00, 3073.00, 4598.00, 5141.00, 5915.00, 6638.00, 7420.00, 7962.00, 8751.00,
        9293.00, 10091.00, 10634.00, 11438.00, 11438.00, 11895.52, // Lv95/Lv95+/Lv100
        11895.52, // Lv95/Lv95+/Lv100
        12353.04, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        25.14, 65.21, 86.76, 129.82, 145.13, 166.97, 187.39, 209.46, 224.77, 247.04, 262.36,
        284.88, 300.19, 322.89, 322.89, 335.81, // Lv95/Lv95+/Lv100
        335.81, // Lv95/Lv95+/Lv100
        348.72, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        74.19, 192.45, 256.06, 383.14, 428.34, 492.81, 553.08, 618.22, 663.41, 729.15, 774.34,
        840.82, 886.01, 953.01, 953.01, 991.13,  // Lv95/Lv95+/Lv100
        991.13,  // Lv95/Lv95+/Lv100
        1029.25, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "裁錦キ法",
            hits: &[
                CHIORI_NORMAL_1,
                CHIORI_NORMAL_2,
                CHIORI_NORMAL_3_1,
                CHIORI_NORMAL_3_2,
                CHIORI_NORMAL_4,
            ],
            charged: &[CHIORI_CHARGED_1, CHIORI_CHARGED_2],
            plunging: &[CHIORI_PLUNGE, CHIORI_PLUNGE_LOW, CHIORI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "羽袖キ法・糸結",
            scalings: &[
                CHIORI_SKILL_TAMOTO,
                CHIORI_SKILL_TAMOTO_DEF,
                CHIORI_SKILL_TURRET,
                CHIORI_SKILL_UPWARD_SWEEP,
                CHIORI_SKILL_UPWARD_SWEEP_DEF,
            ],
        },
        elemental_burst: TalentData {
            name: "二刀キ法・緋反",
            scalings: &[CHIORI_BURST, CHIORI_BURST_DEF],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
