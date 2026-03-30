use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
