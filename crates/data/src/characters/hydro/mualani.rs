use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 降温処理 (Cooling Treatment) -- All Hydro (Catalyst) --

const MUALANI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5140, 0.5530, 0.5910, 0.6420, 0.6810, 0.7200, 0.7710, 0.8220, 0.8740, 0.9250, 0.9770,
        1.0280, 1.0920, 1.1560, 1.2210,
    ],
    dynamic_bonus: None,
};

const MUALANI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.4463, 0.4797, 0.5132, 0.5578, 0.5913, 0.6248, 0.6694, 0.7140, 0.7586, 0.8033, 0.8479,
        0.8925, 0.9483, 1.0041, 1.0599,
    ],
    dynamic_bonus: None,
};

const MUALANI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.7000, 0.7530, 0.8050, 0.8750, 0.9280, 0.9800, 1.0510, 1.1210, 1.1910, 1.2610, 1.3310,
        1.4010, 1.4880, 1.5760, 1.6630,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Hydro (Catalyst) --

const MUALANI_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4288, 1.5360, 1.6431, 1.7860, 1.8932, 2.0003, 2.1432, 2.2861, 2.4290, 2.5718, 2.7147,
        2.8576, 3.0362, 3.2148, 3.3934,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Hydro (Catalyst) --

const MUALANI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
};

const MUALANI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
};

const MUALANI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: 鯊鯊衝浪 (Surfshark Wavebreaker) -- Hydro (HP scaling) --

const MUALANI_SKILL_BITE: TalentScaling = TalentScaling {
    name: "鯊鯊バイトダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0868, 0.0933, 0.0998, 0.1085, 0.1150, 0.1215, 0.1302, 0.1389, 0.1476, 0.1562, 0.1649,
        0.1736, 0.1845, 0.1953, 0.2062,
    ],
    dynamic_bonus: None,
};

const MUALANI_SKILL_STACK: TalentScaling = TalentScaling {
    name: "波乗りチャージ/層",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0434, 0.0467, 0.0499, 0.0543, 0.0575, 0.0608, 0.0651, 0.0694, 0.0738, 0.0781, 0.0825,
        0.0868, 0.0922, 0.0977, 0.1031,
    ],
    dynamic_bonus: None,
};

const MUALANI_SKILL_BIG_WAVE: TalentScaling = TalentScaling {
    name: "巨浪追加ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.2170, 0.2333, 0.2496, 0.2713, 0.2875, 0.3038, 0.3255, 0.3472, 0.3689, 0.3906, 0.4123,
        0.4340, 0.4611, 0.4883, 0.5154,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 爆瀑飛弾 (Boomsharka-laka) -- Hydro (HP scaling) --

const MUALANI_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.5844, 0.6282, 0.6721, 0.7305, 0.7743, 0.8181, 0.8766, 0.9350, 0.9935, 1.0519, 1.1103,
        1.1688, 1.2418, 1.3149, 1.3879,
    ],
    dynamic_bonus: None,
};

pub const MUALANI: CharacterData = CharacterData {
    id: "mualani",
    name: "Mualani",
    element: Element::Hydro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [
        1182.00, 3066.00, 4080.00, 6105.00, 6825.00, 7852.00, 8813.00, 9850.00, 10571.00, 11618.00,
        12338.00, 13397.00, 14117.00, 15185.00, 15185.00, 15792.40, // Lv95/Lv95+/Lv100
        15792.40, // Lv95/Lv95+/Lv100
        16399.80, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        14.15, 36.71, 48.84, 73.08, 81.70, 94.00, 105.49, 117.92, 126.54, 139.08, 147.70, 160.38,
        169.00, 181.78, 181.78, 189.05, // Lv95/Lv95+/Lv100
        189.05, // Lv95/Lv95+/Lv100
        196.32, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        44.39, 115.16, 153.22, 229.27, 256.31, 294.89, 330.95, 369.93, 396.98, 436.31, 463.36,
        503.13, 530.18, 570.27, 570.27, 593.08, // Lv95/Lv95+/Lv100
        593.08, // Lv95/Lv95+/Lv100
        615.89, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "降温処理",
            hits: &[MUALANI_NORMAL_1, MUALANI_NORMAL_2, MUALANI_NORMAL_3],
            charged: &[MUALANI_CHARGED],
            plunging: &[MUALANI_PLUNGE, MUALANI_PLUNGE_LOW, MUALANI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "鯊鯊衝浪",
            scalings: &[
                MUALANI_SKILL_BITE,
                MUALANI_SKILL_STACK,
                MUALANI_SKILL_BIG_WAVE,
            ],
        },
        elemental_burst: TalentData {
            name: "爆瀑飛弾",
            scalings: &[MUALANI_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
