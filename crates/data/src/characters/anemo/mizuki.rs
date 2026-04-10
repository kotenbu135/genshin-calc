use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Pure Heart, Pure Dreams -- All Anemo (Catalyst) --

const MIZUKI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.522768, 0.561976, 0.601183, 0.65346, 0.692668, 0.731875, 0.784152, 0.836429, 0.888706,
        0.940982, 0.993259, 1.045536, 1.110882, 1.176228, 1.241574,
    ],
    dynamic_bonus: None,
};

const MIZUKI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.469144, 0.50433, 0.539516, 0.58643, 0.621616, 0.656802, 0.703716, 0.75063, 0.797545,
        0.844459, 0.891374, 0.938288, 0.996931, 1.055574, 1.114217,
    ],
    dynamic_bonus: None,
};

const MIZUKI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.713688, 0.767215, 0.820741, 0.89211, 0.945637, 0.999163, 1.070532, 1.141901, 1.21327,
        1.284638, 1.356007, 1.427376, 1.516587, 1.605798, 1.695009,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Anemo (Catalyst) --

const MIZUKI_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.3, 1.3975, 1.495, 1.625, 1.7225, 1.82, 1.95, 2.08, 2.21, 2.34, 2.47, 2.6, 2.7625, 2.925,
        3.0875,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Anemo (Catalyst) --

const MIZUKI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
    dynamic_bonus: None,
};

const MIZUKI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
    dynamic_bonus: None,
};

const MIZUKI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: Aisa Utamakura Pilgrimage -- Anemo --

const MIZUKI_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.57744, 0.620748, 0.664056, 0.7218, 0.765108, 0.808416, 0.86616, 0.923904, 0.981648,
        1.039392, 1.097136, 1.15488, 1.22706, 1.29924, 1.37142,
    ],
    dynamic_bonus: None,
};

const MIZUKI_SKILL_CONTINUOUS: TalentScaling = TalentScaling {
    name: "継続攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.44912, 0.482804, 0.516488, 0.5614, 0.595084, 0.628768, 0.67368, 0.718592, 0.763504,
        0.808416, 0.853328, 0.89824, 0.95438, 1.01052, 1.06666,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: Anraku Secret Spring Therapy -- Anemo --

const MIZUKI_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.9408, 1.01136, 1.08192, 1.176, 1.24656, 1.31712, 1.4112, 1.50528, 1.59936, 1.69344,
        1.78752, 1.8816, 1.9992, 2.1168, 2.2344,
    ],
    dynamic_bonus: None,
};

const MIZUKI_BURST_SHOCKWAVE: TalentScaling = TalentScaling {
    name: "夢念衝撃波ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.7056, 0.75852, 0.81144, 0.882, 0.93492, 0.98784, 1.0584, 1.12896, 1.19952, 1.27008,
        1.34064, 1.4112, 1.4994, 1.5876, 1.6758,
    ],
    dynamic_bonus: None,
};

pub const MIZUKI: CharacterData = CharacterData {
    id: "mizuki",
    name: "Mizuki",
    element: Element::Anemo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [
        991.00, 2572.00, 3422.00, 5120.00, 5724.00, 6586.00, 7391.00, 8262.00, 8866.00, 9744.00,
        10348.00, 11236.00, 11840.00, 12736.00, 12736.00, 13245.44, // Lv95/Lv95+/Lv100
        13245.44, // Lv95/Lv95+/Lv100
        13754.88, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        16.76, 43.47, 57.84, 86.54, 96.75, 111.32, 124.93, 139.64, 149.85, 164.70, 174.90, 189.92,
        200.13, 215.26, 215.26, 223.87, // Lv95/Lv95+/Lv100
        223.87, // Lv95/Lv95+/Lv100
        232.48, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        58.93, 152.87, 203.40, 304.35, 340.26, 391.47, 439.34, 491.08, 526.99, 579.20, 615.10,
        667.91, 703.81, 757.03, 757.03, 787.31, // Lv95/Lv95+/Lv100
        787.31, // Lv95/Lv95+/Lv100
        817.59, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalMastery(115.2),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "清心清夢",
            hits: &[MIZUKI_NORMAL_1, MIZUKI_NORMAL_2, MIZUKI_NORMAL_3],
            charged: &[MIZUKI_CHARGED],
            plunging: &[MIZUKI_PLUNGE, MIZUKI_PLUNGE_LOW, MIZUKI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "逢坂歌枕巡り",
            scalings: &[MIZUKI_SKILL, MIZUKI_SKILL_CONTINUOUS],
        },
        elemental_burst: TalentData {
            name: "安楽秘湯療法",
            scalings: &[MIZUKI_BURST, MIZUKI_BURST_SHOCKWAVE],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Sayu — 4★ Anemo Claymore (Inazuma)
// Source: genshin-db-api
// Normal Attack: Shuumatsuban Ninja Blade (終末番忍刀)
// Elemental Skill: Yoohoo Art: Fuuin Dash (風風輪・旋旋)
// Elemental Burst: Yoohoo Art: Mujina Flurry (呼び出しムジナの群れ)
