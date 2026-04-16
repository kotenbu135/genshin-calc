use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Yun Jin
// =============================================================================

// -- Normal Attack: 拂雲キ出手 (Cloud-Grazing Strike) -- Physical --

const YUN_JIN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4051, 0.4381, 0.4710, 0.5181, 0.5510, 0.5888, 0.6406, 0.6924, 0.7443, 0.8008, 0.8573,
        0.9138, 0.9703, 1.0269, 1.0834,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const YUN_JIN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4025, 0.4352, 0.4680, 0.5148, 0.5476, 0.5850, 0.6365, 0.6880, 0.7395, 0.7956, 0.8518,
        0.9079, 0.9641, 1.0202, 1.0764,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const YUN_JIN_NORMAL_3_1: TalentScaling = TalentScaling {
    name: "3段ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2296, 0.2483, 0.2670, 0.2937, 0.3124, 0.3338, 0.3632, 0.3926, 0.4220, 0.4540, 0.4860,
        0.5181, 0.5501, 0.5821, 0.6141,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const YUN_JIN_NORMAL_3_2: TalentScaling = TalentScaling {
    name: "3段ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2752, 0.2977, 0.3202, 0.3522, 0.3747, 0.4003, 0.4355, 0.4708, 0.5060, 0.5444, 0.5828,
        0.6213, 0.6597, 0.6981, 0.7365,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const YUN_JIN_NORMAL_4_1: TalentScaling = TalentScaling {
    name: "4段ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2399, 0.2595, 0.2790, 0.3069, 0.3264, 0.3488, 0.3794, 0.4101, 0.4408, 0.4743, 0.5078,
        0.5413, 0.5748, 0.6082, 0.6417,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const YUN_JIN_NORMAL_4_2: TalentScaling = TalentScaling {
    name: "4段ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2880, 0.3115, 0.3350, 0.3685, 0.3920, 0.4188, 0.4556, 0.4924, 0.5293, 0.5695, 0.6097,
        0.6499, 0.6901, 0.7303, 0.7705,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const YUN_JIN_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6734, 0.7283, 0.7833, 0.8616, 0.9165, 0.9791, 1.0651, 1.1510, 1.2370, 1.3313, 1.4257,
        1.5200, 1.6144, 1.7087, 1.8031,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const YUN_JIN_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2169, 1.3160, 1.4150, 1.5565, 1.6556, 1.7688, 1.9244, 2.0801, 2.2357, 2.4055, 2.6001,
        2.8289, 3.0577, 3.2865, 3.5361,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const YUN_JIN_PLUNGE: TalentScaling = TalentScaling {
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

const YUN_JIN_PLUNGE_LOW: TalentScaling = TalentScaling {
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

const YUN_JIN_PLUNGE_HIGH: TalentScaling = TalentScaling {
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

// -- Elemental Skill: 旋雲キ開相 (Opening Flourish) -- Geo, DEF scaling --

const YUN_JIN_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "短押しダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        1.4912, 1.6030, 1.7149, 1.8640, 1.9758, 2.0877, 2.2368, 2.3859, 2.5350, 2.6842, 2.8333,
        2.9824, 3.1688, 3.3552, 3.5416,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const YUN_JIN_SKILL_CHARGE_1: TalentScaling = TalentScaling {
    name: "一段チャージダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        2.6080, 2.8036, 2.9992, 3.2600, 3.4556, 3.6512, 3.9120, 4.1728, 4.4336, 4.6944, 4.9552,
        5.2160, 5.5420, 5.8680, 6.1940,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const YUN_JIN_SKILL_CHARGE_2: TalentScaling = TalentScaling {
    name: "二段チャージダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        3.7248, 4.0042, 4.2835, 4.6560, 4.9354, 5.2147, 5.5872, 5.9597, 6.3322, 6.7046, 7.0771,
        7.4496, 7.9152, 8.3808, 8.8464,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 破嶂キ旌儀 (Cliffbreaker's Banner) -- Geo, DEF scaling --

const YUN_JIN_BURST_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        2.4400, 2.6230, 2.8060, 3.0500, 3.2330, 3.4160, 3.6600, 3.9040, 4.1480, 4.3920, 4.6360,
        4.8800, 5.1850, 5.4900, 5.7950,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const YUN_JIN_BURST_DMG_BONUS: TalentScaling = TalentScaling {
    name: "ダメージ増加 (DEF基準)",
    scaling_stat: ScalingStat::Def,
    damage_element: None,
    values: [
        0.3216, 0.3457, 0.3698, 0.4020, 0.4261, 0.4502, 0.4824, 0.5146, 0.5467, 0.5789, 0.6110,
        0.6432, 0.6834, 0.7236, 0.7638,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const YUN_JIN: CharacterData = CharacterData {
    id: "yun_jin",
    name: "Yun Jin",
    element: Element::Geo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [
        894.00, 2296.00, 2963.00, 4438.00, 4913.00, 5651.00, 6283.00, 7021.00, 7495.00, 8233.00,
        8707.00, 9445.00, 9919.00, 10657.00, 10657.00, 11083.28, // Lv95/Lv95+/Lv100
        11083.28, // Lv95/Lv95+/Lv100
        11395.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        16.03, 41.17, 53.15, 79.61, 88.12, 101.35, 112.70, 125.94, 134.44, 147.67, 156.17, 169.41,
        177.92, 191.16, 191.16, 198.81, // Lv95/Lv95+/Lv100
        198.81, // Lv95/Lv95+/Lv100
        239.93, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        61.57, 158.18, 204.18, 305.84, 338.53, 389.39, 432.96, 483.82, 516.51, 567.31, 599.99,
        650.85, 683.53, 734.39, 734.39, 763.77, // Lv95/Lv95+/Lv100
        763.77, // Lv95/Lv95+/Lv100
        785.19, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::EnergyRecharge(0.2668),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "拂雲キ出手",
            hits: &[
                YUN_JIN_NORMAL_1,
                YUN_JIN_NORMAL_2,
                YUN_JIN_NORMAL_3_1,
                YUN_JIN_NORMAL_3_2,
                YUN_JIN_NORMAL_4_1,
                YUN_JIN_NORMAL_4_2,
                YUN_JIN_NORMAL_5,
            ],
            charged: &[YUN_JIN_CHARGED],
            plunging: &[YUN_JIN_PLUNGE, YUN_JIN_PLUNGE_LOW, YUN_JIN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "旋雲キ開相",
            scalings: &[
                YUN_JIN_SKILL_PRESS,
                YUN_JIN_SKILL_CHARGE_1,
                YUN_JIN_SKILL_CHARGE_2,
            ],
        },
        elemental_burst: TalentData {
            name: "破嶂キ旌儀",
            scalings: &[YUN_JIN_BURST_DAMAGE, YUN_JIN_BURST_DMG_BONUS],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
    passive_scalings: &[],
    scaling_modifiers: &[],
};
