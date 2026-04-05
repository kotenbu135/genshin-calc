use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Zibai — 5★ Geo Sword (Liyue)
// Source: Honey Impact (gensh.honeyhunterworld.com) 2026-03-31
// Normal Attack: Golden Blade's Petaled Touch
// Elemental Skill: Heaven and Earth Made Manifest
// Elemental Burst: Tri-Sphere Eminence
// =============================================================================

// --- Normal Attack: Golden Blade's Petaled Touch --- ATK, Physical ---

const ZIBAI_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5055, 0.5467, 0.5878, 0.6466, 0.6878, 0.7348, 0.7995, 0.8641, 0.9288, 0.9993, 1.0699,
        1.1404, 1.2110, 1.2815, 1.3520,
    ],
    dynamic_bonus: None,
};

const ZIBAI_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4655, 0.5034, 0.5413, 0.5954, 0.6333, 0.6766, 0.7362, 0.7957, 0.8553, 0.9202, 0.9852,
        1.0501, 1.1151, 1.1801, 1.2450,
    ],
    dynamic_bonus: None,
};

const ZIBAI_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3089, 0.3340, 0.3592, 0.3951, 0.4202, 0.4490, 0.4885, 0.5280, 0.5675, 0.6106, 0.6537,
        0.6968, 0.7399, 0.7830, 0.8261,
    ],
    dynamic_bonus: None,
};

const ZIBAI_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7790, 0.8424, 0.9058, 0.9963, 1.0597, 1.1322, 1.2318, 1.3315, 1.4311, 1.5398, 1.6485,
        1.7572, 1.8659, 1.9746, 2.0832,
    ],
    dynamic_bonus: None,
};

// --- Charged Attack --- ATK, Physical ---

const ZIBAI_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7366, 0.7965, 0.8565, 0.9421, 1.0021, 1.0706, 1.1648, 1.2591, 1.3533, 1.4561, 1.5588,
        1.6616, 1.7644, 1.8672, 1.9699,
    ],
    dynamic_bonus: None,
};

// --- Plunging Attack --- ATK, Physical ---

const ZIBAI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
};

const ZIBAI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
};

const ZIBAI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
};

// --- Elemental Skill: Heaven and Earth Made Manifest --- DEF, Geo ---

// Lunar Phase Shift mode attacks
const ZIBAI_SKILL_LPS_HIT1: TalentScaling = TalentScaling {
    name: "月相転移1段ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.5658, 0.6082, 0.6507, 0.7072, 0.7497, 0.7921, 0.8487, 0.9053, 0.9618, 1.0184, 1.0750,
        1.1316, 1.2023, 1.2730, 1.3438,
    ],
    dynamic_bonus: None,
};

const ZIBAI_SKILL_LPS_HIT2: TalentScaling = TalentScaling {
    name: "月相転移2段ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.5210, 0.5601, 0.5992, 0.6513, 0.6903, 0.7294, 0.7815, 0.8336, 0.8857, 0.9378, 0.9899,
        1.0420, 1.1071, 1.1723, 1.2374,
    ],
    dynamic_bonus: None,
};

const ZIBAI_SKILL_LPS_HIT3: TalentScaling = TalentScaling {
    name: "月相転移3段ダメージ(x2)",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.3457, 0.3716, 0.3975, 0.4321, 0.4580, 0.4840, 0.5185, 0.5531, 0.5877, 0.6222, 0.6568,
        0.6914, 0.7346, 0.7778, 0.8210,
    ],
    dynamic_bonus: None,
};

const ZIBAI_SKILL_LPS_HIT4: TalentScaling = TalentScaling {
    name: "月相転移4段ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.8718, 0.9372, 1.0026, 1.0897, 1.1551, 1.2205, 1.3077, 1.3949, 1.4820, 1.5692, 1.6564,
        1.7436, 1.8525, 1.9615, 2.0705,
    ],
    dynamic_bonus: None,
};

const ZIBAI_SKILL_LPS_HIT4_EXTRA: TalentScaling = TalentScaling {
    name: "月相転移4段追加ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.2946, 0.3167, 0.3387, 0.3682, 0.3903, 0.4124, 0.4418, 0.4713, 0.5008, 0.5302, 0.5597,
        0.5891, 0.6259, 0.6628, 0.6996,
    ],
    dynamic_bonus: None,
};

const ZIBAI_SKILL_LPS_CHARGED: TalentScaling = TalentScaling {
    name: "月相転移重撃ダメージ(x2)",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.6595, 0.7090, 0.7584, 0.8244, 0.8738, 0.9233, 0.9893, 1.0552, 1.1212, 1.1871, 1.2530,
        1.3190, 1.4014, 1.4839, 1.5663,
    ],
    dynamic_bonus: None,
};

// Spirit Steed's Stride
const ZIBAI_SKILL_STRIDE_HIT1: TalentScaling = TalentScaling {
    name: "霊駿突進1段ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        1.7253, 1.8547, 1.9841, 2.1566, 2.2860, 2.4154, 2.5879, 2.7604, 2.9330, 3.1055, 3.2780,
        3.4506, 3.6662, 3.8819, 4.0975,
    ],
    dynamic_bonus: None,
};

const ZIBAI_SKILL_STRIDE_HIT2: TalentScaling = TalentScaling {
    name: "霊駿突進2段ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        1.4097, 1.5154, 1.6211, 1.7621, 1.8678, 1.9736, 2.1145, 2.2555, 2.3965, 2.5374, 2.6784,
        2.8194, 2.9956, 3.1718, 3.3480,
    ],
    dynamic_bonus: None,
};

// --- Elemental Burst: Tri-Sphere Eminence --- DEF, Geo ---

const ZIBAI_BURST_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        1.2696, 1.3648, 1.4600, 1.5870, 1.6822, 1.7774, 1.9044, 2.0314, 2.1583, 2.2853, 2.4122,
        2.5392, 2.6979, 2.8566, 3.0153,
    ],
    dynamic_bonus: None,
};

const ZIBAI_BURST_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        1.7774, 1.9107, 2.0441, 2.2218, 2.3551, 2.4884, 2.6662, 2.8439, 3.0216, 3.1994, 3.3771,
        3.5549, 3.7771, 3.9992, 4.2214,
    ],
    dynamic_bonus: None,
};

// -- Character Data --

pub const ZIBAI: CharacterData = CharacterData {
    id: "zibai",
    name: "Zibai",
    element: Element::Geo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [
        1006.00, 2609.00, 3471.00, 5194.00, 5807.00, 6681.00, 7498.00, 8381.00, 8994.00, 9885.00,
        10497.00, 11399.00, 12011.00, 12919.00, 12919.00, 13435.76, // Lv95/Lv95+/Lv100
        13435.76, // Lv95/Lv95+/Lv100
        13952.52, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        17.50, 45.40, 60.41, 90.39, 101.05, 116.26, 130.48, 145.85, 156.51, 172.02, 182.68, 198.36,
        209.02, 224.83, 224.83, 233.82, // Lv95/Lv95+/Lv100
        233.82, // Lv95/Lv95+/Lv100
        242.82, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        74.49, 193.22, 257.09, 384.69, 430.07, 494.80, 555.31, 620.71, 666.09, 732.09, 777.47,
        844.21, 889.59, 956.85, 956.85, 995.12,  // Lv95/Lv95+/Lv100
        995.12,  // Lv95/Lv95+/Lv100
        1033.40, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "Golden Blade's Petaled Touch",
            hits: &[ZIBAI_NA_HIT1, ZIBAI_NA_HIT2, ZIBAI_NA_HIT3, ZIBAI_NA_HIT4],
            charged: &[ZIBAI_CHARGED],
            plunging: &[ZIBAI_PLUNGE, ZIBAI_PLUNGE_LOW, ZIBAI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "Heaven and Earth Made Manifest",
            scalings: &[
                ZIBAI_SKILL_LPS_HIT1,
                ZIBAI_SKILL_LPS_HIT2,
                ZIBAI_SKILL_LPS_HIT3,
                ZIBAI_SKILL_LPS_HIT4,
                ZIBAI_SKILL_LPS_HIT4_EXTRA,
                ZIBAI_SKILL_LPS_CHARGED,
                ZIBAI_SKILL_STRIDE_HIT1,
                ZIBAI_SKILL_STRIDE_HIT2,
            ],
        },
        elemental_burst: TalentData {
            name: "Tri-Sphere Eminence",
            scalings: &[ZIBAI_BURST_HIT1, ZIBAI_BURST_HIT2],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
