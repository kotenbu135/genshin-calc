use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    dynamic_bonus: None,
};

const ROSARIA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5160, 0.5580, 0.6000, 0.6600, 0.7020, 0.7500, 0.8160, 0.8820, 0.9480, 1.0200, 1.0920,
        1.1640, 1.2360, 1.3080, 1.3800,
    ],
    dynamic_bonus: None,
};

const ROSARIA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3182, 0.3441, 0.3700, 0.4070, 0.4329, 0.4625, 0.5032, 0.5439, 0.5846, 0.6290, 0.6734,
        0.7178, 0.7622, 0.8066, 0.8510,
    ],
    dynamic_bonus: None,
};

const ROSARIA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6966, 0.7533, 0.8100, 0.8910, 0.9477, 1.0125, 1.1016, 1.1907, 1.2798, 1.3770, 1.4742,
        1.5714, 1.6686, 1.7658, 1.8630,
    ],
    dynamic_bonus: None,
};

const ROSARIA_NORMAL_5A: TalentScaling = TalentScaling {
    name: "5段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4162, 0.4501, 0.4840, 0.5324, 0.5663, 0.6050, 0.6582, 0.7115, 0.7647, 0.8228, 0.8809,
        0.9390, 0.9970, 1.0551, 1.1132,
    ],
    dynamic_bonus: None,
};

const ROSARIA_NORMAL_5B: TalentScaling = TalentScaling {
    name: "5段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4300, 0.4650, 0.5000, 0.5500, 0.5850, 0.6250, 0.6800, 0.7350, 0.7900, 0.8500, 0.9100,
        0.9700, 1.0300, 1.0900, 1.1500,
    ],
    dynamic_bonus: None,
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
    dynamic_bonus: None,
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
    dynamic_bonus: None,
};

const ROSARIA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
};

const ROSARIA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
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
    dynamic_bonus: None,
};

const ROSARIA_SKILL_2: TalentScaling = TalentScaling {
    name: "スキルダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.3600, 1.4620, 1.5640, 1.7000, 1.8020, 1.9040, 2.0400, 2.1760, 2.3120, 2.4480, 2.5840,
        2.7200, 2.8900, 3.0600, 3.2300,
    ],
    dynamic_bonus: None,
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
    dynamic_bonus: None,
};

const ROSARIA_BURST_2: TalentScaling = TalentScaling {
    name: "スキルダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.5200, 1.6340, 1.7480, 1.9000, 2.0140, 2.1280, 2.2800, 2.4320, 2.5840, 2.7360, 2.8880,
        3.0400, 3.2300, 3.4200, 3.6100,
    ],
    dynamic_bonus: None,
};

const ROSARIA_BURST_DOT: TalentScaling = TalentScaling {
    name: "氷槍継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.3200, 1.4190, 1.5180, 1.6500, 1.7490, 1.8480, 1.9800, 2.1120, 2.2440, 2.3760, 2.5080,
        2.6400, 2.8050, 2.9700, 3.1350,
    ],
    dynamic_bonus: None,
};

pub const ROSARIA: CharacterData = CharacterData {
    id: "rosaria",
    name: "Rosaria",
    element: Element::Cryo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [
        1030.00, 2647.00, 3417.00, 5118.00, 5665.00, 6516.00, 7245.00, 8096.00, 8643.00, 9493.00,
        10040.00, 10891.00, 11438.00, 12289.00, 12289.00, 12780.56, // Lv95/Lv95+/Lv100
        12780.56, // Lv95/Lv95+/Lv100
        13272.12, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        20.12, 51.70, 66.73, 99.95, 110.63, 127.26, 141.50, 158.12, 168.80, 185.40, 196.08, 212.71,
        223.39, 240.01, 240.01, 249.61, // Lv95/Lv95+/Lv100
        249.61, // Lv95/Lv95+/Lv100
        259.21, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        59.51, 152.89, 197.35, 295.61, 327.20, 376.36, 418.48, 467.64, 499.23, 548.33, 579.92,
        629.07, 660.66, 709.82, 709.82, 738.21, // Lv95/Lv95+/Lv100
        738.21, // Lv95/Lv95+/Lv100
        766.61, // Lv95/Lv95+/Lv100
    ],
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
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Shenhe
