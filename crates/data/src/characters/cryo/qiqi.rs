use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    base_hp: [
        963.00, 2498.00, 3323.00, 4973.00, 5559.00, 6396.00, 7178.00, 8023.00, 8610.00, 9463.00,
        10050.00, 10912.00, 11499.00, 12368.00, 12368.00, 12862.72, // Lv95/Lv95+/Lv100
        12862.72, // Lv95/Lv95+/Lv100
        13357.44, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        22.34, 57.96, 77.12, 115.39, 129.00, 148.42, 166.57, 186.19, 199.80, 219.59, 233.21,
        253.23, 266.84, 287.01, 287.01, 298.49, // Lv95/Lv95+/Lv100
        298.49, // Lv95/Lv95+/Lv100
        309.97, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        71.80, 186.24, 247.80, 370.79, 414.53, 476.92, 535.24, 598.27, 642.01, 705.62, 749.36,
        813.69, 857.43, 922.27, 922.27, 959.16, // Lv95/Lv95+/Lv100
        959.16, // Lv95/Lv95+/Lv100
        996.05, // Lv95/Lv95+/Lv100
    ],
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
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Rosaria
