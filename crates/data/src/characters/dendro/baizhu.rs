use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// --- Normal Attack: The Classics of Acupuncture (all Dendro — catalyst) ---

const BAIZHU_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.373704, 0.401732, 0.42976, 0.46713, 0.495158, 0.523186, 0.560556, 0.597926, 0.635297,
        0.672667, 0.710038, 0.747408, 0.794121, 0.840834, 0.887547,
    ],
};

const BAIZHU_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.364248, 0.391567, 0.418885, 0.45531, 0.482629, 0.509947, 0.546372, 0.582797, 0.619222,
        0.655646, 0.692071, 0.728496, 0.774027, 0.819558, 0.865089,
    ],
};

const BAIZHU_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.225416, 0.242322, 0.259228, 0.28177, 0.298676, 0.315582, 0.338124, 0.360666, 0.383207,
        0.405749, 0.42829, 0.450832, 0.479009, 0.507186, 0.535363,
    ],
};

const BAIZHU_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.541376, 0.581979, 0.622582, 0.67672, 0.717323, 0.757926, 0.812064, 0.866202, 0.920339,
        0.974477, 1.028614, 1.082752, 1.150424, 1.218096, 1.285768,
    ],
};

const BAIZHU_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.2104, 1.30118, 1.39196, 1.513, 1.60378, 1.69456, 1.8156, 1.93664, 2.05768, 2.17872,
        2.29976, 2.4208, 2.5721, 2.7234, 2.8747,
    ],
};

const BAIZHU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
};

const BAIZHU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const BAIZHU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
};

// --- Elemental Skill: Universal Diagnosis ---

const BAIZHU_SKILL_DMG: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.792, 0.8514, 0.9108, 0.99, 1.0494, 1.1088, 1.188, 1.2672, 1.3464, 1.4256, 1.5048, 1.584,
        1.683, 1.782, 1.881,
    ],
};

// --- Elemental Burst: Holistic Revivification ---

const BAIZHU_BURST_SPIRIT: TalentScaling = TalentScaling {
    name: "霊気棘ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.97064, 1.043438, 1.116236, 1.2133, 1.286098, 1.358896, 1.45596, 1.553024, 1.650088,
        1.747152, 1.844216, 1.94128, 2.06261, 2.18394, 2.30527,
    ],
};

// --- Baizhu aggregation ---

static BAIZHU_NA_HITS: &[TalentScaling] = &[
    BAIZHU_NA_HIT1,
    BAIZHU_NA_HIT2,
    BAIZHU_NA_HIT3,
    BAIZHU_NA_HIT4,
];
static BAIZHU_CHARGED_ATTACKS: &[TalentScaling] = &[BAIZHU_CHARGED];
static BAIZHU_PLUNGING: &[TalentScaling] = &[BAIZHU_PLUNGE, BAIZHU_PLUNGE_LOW, BAIZHU_PLUNGE_HIGH];
static BAIZHU_SKILL_SCALINGS: &[TalentScaling] = &[BAIZHU_SKILL_DMG];
static BAIZHU_BURST_SCALINGS: &[TalentScaling] = &[BAIZHU_BURST_SPIRIT];

pub const BAIZHU: CharacterData = CharacterData {
    id: "baizhu",
    name: "Baizhu",
    element: Element::Dendro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [
        1039.00, 2695.00, 3586.00, 5366.00, 5999.00, 6902.00, 7747.00, 8659.00, 9292.00, 10213.00,
        10846.00, 11777.00, 12410.00, 13348.00, 13348.00, 13881.92, // Lv95/Lv95+/Lv100
        13881.92, // Lv95/Lv95+/Lv100
        14415.84, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        14.99, 38.88, 51.73, 77.41, 86.54, 99.57, 111.74, 124.90, 134.03, 147.31, 156.44, 169.87,
        179.00, 192.54, 192.54, 200.24, // Lv95/Lv95+/Lv100
        200.24, // Lv95/Lv95+/Lv100
        207.94, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        38.89, 100.88, 134.22, 200.84, 224.53, 258.33, 289.92, 324.06, 347.76, 382.21, 405.91,
        440.75, 464.44, 499.56, 499.56, 519.54, // Lv95/Lv95+/Lv100
        519.54, // Lv95/Lv95+/Lv100
        539.52, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Hp(0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "金匱鍼解",
            hits: BAIZHU_NA_HITS,
            charged: BAIZHU_CHARGED_ATTACKS,
            plunging: BAIZHU_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "太素診要",
            scalings: BAIZHU_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "癒気全形論",
            scalings: BAIZHU_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
