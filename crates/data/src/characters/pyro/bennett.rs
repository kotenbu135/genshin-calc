use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Bennett
// =============================================================================

// -- Normal Attack: 好運の剣 (Strike of Fortune) -- Physical --

const BENNETT_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4455, 0.4817, 0.5180, 0.5698, 0.6061, 0.6475, 0.7045, 0.7615, 0.8184, 0.8806, 0.9428,
        1.0049, 1.0671, 1.1292, 1.1914,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BENNETT_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4274, 0.4622, 0.4970, 0.5467, 0.5815, 0.6213, 0.6759, 0.7306, 0.7853, 0.8449, 0.9045,
        0.9642, 1.0238, 1.0835, 1.1431,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BENNETT_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5461, 0.5906, 0.6350, 0.6985, 0.7430, 0.7938, 0.8636, 0.9335, 1.0033, 1.0795, 1.1557,
        1.2319, 1.3081, 1.3843, 1.4605,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BENNETT_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5968, 0.6454, 0.6940, 0.7634, 0.8120, 0.8675, 0.9438, 1.0202, 1.0965, 1.1798, 1.2631,
        1.3464, 1.4296, 1.5129, 1.5962,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BENNETT_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7190, 0.7775, 0.8360, 0.9196, 0.9781, 1.0450, 1.1370, 1.2289, 1.3209, 1.4212, 1.5215,
        1.6218, 1.7222, 1.8225, 1.9228,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const BENNETT_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ 1段",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5590, 0.6045, 0.6500, 0.7150, 0.7605, 0.8125, 0.8840, 0.9555, 1.0270, 1.1050, 1.1830,
        1.2610, 1.3390, 1.4170, 1.4950,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BENNETT_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ 2段",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6072, 0.6566, 0.7060, 0.7766, 0.8260, 0.8825, 0.9602, 1.0378, 1.1155, 1.2002, 1.2849,
        1.3696, 1.4544, 1.5391, 1.6238,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const BENNETT_PLUNGE: TalentScaling = TalentScaling {
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

const BENNETT_PLUNGE_LOW: TalentScaling = TalentScaling {
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

const BENNETT_PLUNGE_HIGH: TalentScaling = TalentScaling {
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

// -- Elemental Skill: 溢れる情熱 (Passion Overload) -- Pyro --

const BENNETT_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "1回押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.3760, 1.4792, 1.5824, 1.7200, 1.8232, 1.9264, 2.0640, 2.2016, 2.3392, 2.4768, 2.6144,
        2.7520, 2.9240, 3.0960, 3.2680,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BENNETT_SKILL_HOLD_1: TalentScaling = TalentScaling {
    name: "長押し1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.8400, 0.9030, 0.9660, 1.0500, 1.1130, 1.1760, 1.2600, 1.3440, 1.4280, 1.5120, 1.5960,
        1.6800, 1.7850, 1.8900, 1.9950,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BENNETT_SKILL_HOLD_2: TalentScaling = TalentScaling {
    name: "長押し2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9200, 0.9890, 1.0580, 1.1500, 1.2190, 1.2880, 1.3800, 1.4720, 1.5640, 1.6560, 1.7480,
        1.8400, 1.9550, 2.0700, 2.1850,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BENNETT_SKILL_EXPLOSION: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.3200, 1.4190, 1.5180, 1.6500, 1.7490, 1.8480, 1.9800, 2.1120, 2.2440, 2.3760, 2.5080,
        2.6400, 2.8050, 2.9700, 3.1350,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 素晴らしい旅 (Fantastic Voyage) -- Pyro --

const BENNETT_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        2.3280, 2.5026, 2.6772, 2.9100, 3.0846, 3.2592, 3.4920, 3.7248, 3.9576, 4.1904, 4.4232,
        4.6560, 4.9470, 5.2380, 5.5290,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const BENNETT: CharacterData = CharacterData {
    id: "bennett",
    name: "Bennett",
    element: Element::Pyro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [
        1039.00, 2670.00, 3447.00, 5163.00, 5715.00, 6573.00, 7309.00, 8168.00, 8719.00, 9577.00,
        10129.00, 10987.00, 11539.00, 12397.00, 12397.00, 12892.88, // Lv95/Lv95+/Lv100
        12892.88, // Lv95/Lv95+/Lv100
        13388.76, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        16.03, 41.17, 53.15, 79.61, 88.12, 101.35, 112.70, 125.94, 134.44, 147.67, 156.17, 169.41,
        177.92, 191.16, 191.16, 198.81, // Lv95/Lv95+/Lv100
        198.81, // Lv95/Lv95+/Lv100
        206.45, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        64.66, 166.12, 214.43, 321.19, 355.51, 408.93, 454.69, 508.10, 542.43, 595.78, 630.10,
        683.51, 717.84, 771.25, 771.25, 802.10, // Lv95/Lv95+/Lv100
        802.10, // Lv95/Lv95+/Lv100
        832.95, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::EnergyRecharge(0.2668),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "好運の剣",
            hits: &[
                BENNETT_NORMAL_1,
                BENNETT_NORMAL_2,
                BENNETT_NORMAL_3,
                BENNETT_NORMAL_4,
                BENNETT_NORMAL_5,
            ],
            charged: &[BENNETT_CHARGED_1, BENNETT_CHARGED_2],
            plunging: &[BENNETT_PLUNGE, BENNETT_PLUNGE_LOW, BENNETT_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "溢れる情熱",
            scalings: &[
                BENNETT_SKILL_PRESS,
                BENNETT_SKILL_HOLD_1,
                BENNETT_SKILL_HOLD_2,
                BENNETT_SKILL_EXPLOSION,
            ],
        },
        elemental_burst: TalentData {
            name: "素晴らしい旅",
            scalings: &[BENNETT_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
    scaling_modifiers: &[],
};
