use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Rite of Dispelling Winds -- Anemo (Catalyst) --

const IFA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.536072, 0.576277, 0.616483, 0.67009, 0.710295, 0.750501, 0.804108, 0.857715, 0.911322,
        0.96493, 1.018537, 1.072144, 1.139153, 1.206162, 1.273171,
    ],
    dynamic_bonus: None,
};

const IFA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.474672, 0.510272, 0.545873, 0.59334, 0.62894, 0.664541, 0.712008, 0.759475, 0.806942,
        0.85441, 0.901877, 0.949344, 1.008678, 1.068012, 1.127346,
    ],
    dynamic_bonus: None,
};

const IFA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.747584, 0.803653, 0.859722, 0.93448, 0.990549, 1.046618, 1.121376, 1.196134, 1.270893,
        1.345651, 1.42041, 1.495168, 1.588616, 1.682064, 1.775512,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Anemo (Catalyst) --

const IFA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.4704, 1.58068, 1.69096, 1.838, 1.94828, 2.05856, 2.2056, 2.35264, 2.49968, 2.64672,
        2.79376, 2.9408, 3.1246, 3.3084, 3.4922,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Anemo (Catalyst) --

const IFA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
    dynamic_bonus: None,
};

const IFA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
    dynamic_bonus: None,
};

const IFA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: Airborne Disease Prevention -- Anemo --

const IFA_SKILL: TalentScaling = TalentScaling {
    name: "トニック弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.3336, 1.43362, 1.53364, 1.667, 1.76702, 1.86704, 2.0004, 2.13376, 2.26712, 2.40048,
        2.53384, 2.6672, 2.8339, 3.0006, 3.1673,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: Compound Sedation Field -- Anemo --

const IFA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        5.0848, 5.46616, 5.84752, 6.356, 6.73736, 7.11872, 7.6272, 8.13568, 8.64416, 9.15264,
        9.66112, 10.1696, 10.8052, 11.4408, 12.0764,
    ],
    dynamic_bonus: None,
};

const IFA_BURST_MARK: TalentScaling = TalentScaling {
    name: "鎮静マークダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.0896, 1.17132, 1.25304, 1.362, 1.44372, 1.52544, 1.6344, 1.74336, 1.85232, 1.96128,
        2.07024, 2.1792, 2.3154, 2.4516, 2.5878,
    ],
    dynamic_bonus: None,
};

pub const IFA: CharacterData = CharacterData {
    id: "ifa",
    name: "Ifa",
    element: Element::Anemo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    region: Region::Natlan,
    base_hp: [
        845.00, 2171.00, 2803.00, 4198.00, 4647.00, 5345.00, 5943.00, 6641.00, 7090.00, 7787.00,
        8236.00, 8934.00, 9383.00, 10081.00, 10081.00, 10484.24, // Lv95/Lv95+/Lv100
        10484.24, // Lv95/Lv95+/Lv100
        10887.48, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        14.96, 38.43, 49.60, 74.30, 82.24, 94.60, 105.18, 117.54, 125.48, 137.82, 145.76, 158.12,
        166.06, 178.41, 178.41, 185.55, // Lv95/Lv95+/Lv100
        185.55, // Lv95/Lv95+/Lv100
        192.68, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        50.76, 130.40, 168.32, 252.12, 279.06, 320.99, 356.91, 398.84, 425.78, 467.66, 494.60,
        536.53, 563.47, 605.40, 605.40, 629.62, // Lv95/Lv95+/Lv100
        629.62, // Lv95/Lv95+/Lv100
        653.83, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "祓風の儀",
            hits: &[IFA_NORMAL_1, IFA_NORMAL_2, IFA_NORMAL_3],
            charged: &[IFA_CHARGED],
            plunging: &[IFA_PLUNGE, IFA_PLUNGE_LOW, IFA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "空中疫病予防",
            scalings: &[IFA_SKILL],
        },
        elemental_burst: TalentData {
            name: "複合鎮静フィールド",
            scalings: &[IFA_BURST, IFA_BURST_MARK],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Jean — 5★ Anemo Sword (Mondstadt)
// Source: genshin-db-api
// Normal Attack: Favonius Bladework (西風剣術)
// Elemental Skill: Gale Blade (風圧剣)
// Elemental Burst: Dandelion Breeze (蒲公英の風)
