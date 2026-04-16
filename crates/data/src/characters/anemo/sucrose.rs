use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Wind Spirit Creation -- All Anemo (Catalyst) --

const SUCROSE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.33464, 0.359738, 0.384836, 0.4183, 0.443398, 0.468496, 0.50196, 0.535424, 0.568888,
        0.602352, 0.635816, 0.66928, 0.71111, 0.75294, 0.79477,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const SUCROSE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.30616, 0.329122, 0.352084, 0.3827, 0.405662, 0.428624, 0.45924, 0.489856, 0.520472,
        0.551088, 0.581704, 0.61232, 0.65059, 0.68886, 0.72713,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const SUCROSE_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.38448, 0.413316, 0.442152, 0.4806, 0.509436, 0.538272, 0.57672, 0.615168, 0.653616,
        0.692064, 0.730512, 0.76896, 0.81702, 0.86508, 0.91314,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const SUCROSE_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.479176, 0.515114, 0.551052, 0.59897, 0.634908, 0.670846, 0.718764, 0.766682, 0.814599,
        0.862517, 0.910434, 0.958352, 1.018249, 1.078146, 1.138043,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Anemo (Catalyst) --

const SUCROSE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.2016, 1.29172, 1.38184, 1.502, 1.59212, 1.68224, 1.8024, 1.92256, 2.04272, 2.16288,
        2.28304, 2.4032, 2.5534, 2.7036, 2.8538,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Anemo (Catalyst) --

const SUCROSE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const SUCROSE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const SUCROSE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: Astable Anemohypostasis Creation - 6308 -- Anemo --

const SUCROSE_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.112, 2.2704, 2.4288, 2.64, 2.7984, 2.9568, 3.168, 3.3792, 3.5904, 3.8016, 4.0128, 4.224,
        4.488, 4.752, 5.016,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: Forbidden Creation - Isomer 75 / Type II -- Anemo --

const SUCROSE_BURST_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.48, 1.591, 1.702, 1.85, 1.961, 2.072, 2.22, 2.368, 2.516, 2.664, 2.812, 2.96, 3.145,
        3.33, 3.515,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const SUCROSE_BURST_ELEM: TalentScaling = TalentScaling {
    name: "付加元素ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.44, 0.473, 0.506, 0.55, 0.583, 0.616, 0.66, 0.704, 0.748, 0.792, 0.836, 0.88, 0.935,
        0.99, 1.045,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const SUCROSE: CharacterData = CharacterData {
    id: "sucrose",
    name: "Sucrose",
    element: Element::Anemo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [
        775.00, 1991.00, 2570.00, 3850.00, 4261.00, 4901.00, 5450.00, 6090.00, 6501.00, 7141.00,
        7552.00, 8192.00, 8604.00, 9244.00, 9244.00, 9613.76, // Lv95/Lv95+/Lv100
        9613.76, // Lv95/Lv95+/Lv100
        9983.52, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        14.25, 36.60, 47.24, 70.76, 78.33, 90.09, 100.18, 111.94, 119.51, 131.26, 138.82, 150.59,
        158.15, 169.92, 169.92, 176.72, // Lv95/Lv95+/Lv100
        176.72, // Lv95/Lv95+/Lv100
        183.51, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        58.94, 151.42, 195.45, 292.77, 324.05, 372.74, 414.45, 463.14, 494.43, 543.05, 574.34,
        623.03, 654.31, 703.00, 703.00, 731.12, // Lv95/Lv95+/Lv100
        731.12, // Lv95/Lv95+/Lv100
        759.24, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Anemo, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "風霊作成",
            hits: &[
                SUCROSE_NORMAL_1,
                SUCROSE_NORMAL_2,
                SUCROSE_NORMAL_3,
                SUCROSE_NORMAL_4,
            ],
            charged: &[SUCROSE_CHARGED],
            plunging: &[SUCROSE_PLUNGE, SUCROSE_PLUNGE_LOW, SUCROSE_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "風霊作成・六三〇八",
            scalings: &[SUCROSE_SKILL],
        },
        elemental_burst: TalentData {
            name: "禁・風霊作成・七五同構弐型",
            scalings: &[SUCROSE_BURST_DOT, SUCROSE_BURST_ELEM],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
};

// =============================================================================
// Venti — 5★ Anemo Bow (Mondstadt)
// Source: genshin-db-api
// Normal Attack: Divine Marksmanship (天賦の射術)
// Elemental Skill: Skyward Sonnet (高天の歌)
// Elemental Burst: Wind's Grand Ode (風神の詩)
