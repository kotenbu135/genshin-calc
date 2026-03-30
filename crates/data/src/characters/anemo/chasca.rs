use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Phantom Feather Flurry -- Physical --

const CHASCA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.480078, 0.519154, 0.55823, 0.614053, 0.653129, 0.697788, 0.759193, 0.820598, 0.882003,
        0.948991, 1.015979, 1.082966, 1.149954, 1.216941, 1.283929,
    ],
};

const CHASCA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.445884, 0.482177, 0.51847, 0.570317, 0.60661, 0.648087, 0.705119, 0.762151, 0.819183,
        0.881399, 0.943615, 1.005832, 1.068048, 1.130265, 1.192481,
    ],
};

const CHASCA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.296975, 0.321148, 0.34532, 0.379852, 0.404024, 0.43165, 0.469635, 0.50762, 0.545606,
        0.587044, 0.628482, 0.669921, 0.711359, 0.752798, 0.794236,
    ],
};

const CHASCA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ(x3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.254672, 0.275401, 0.29613, 0.325743, 0.346472, 0.370163, 0.402737, 0.435311, 0.467885,
        0.503421, 0.538957, 0.574492, 0.610028, 0.645563, 0.681099,
    ],
};

// -- Aimed Shot -- Anemo (charged) --

const CHASCA_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894,
        1.0506, 1.1118, 1.173,
    ],
};

const CHASCA_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635,
        2.79, 2.945,
    ],
};

// -- Plunging Attack -- Physical --

const CHASCA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
};

const CHASCA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const CHASCA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
};

// -- Elemental Skill: Spirit Reins, Shadow Hunt -- Anemo --

const CHASCA_SKILL_RESONANCE: TalentScaling = TalentScaling {
    name: "共鳴ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.6, 0.645, 0.69, 0.75, 0.795, 0.84, 0.9, 0.96, 1.02, 1.08, 1.14, 1.2, 1.275, 1.35, 1.425,
    ],
};

const CHASCA_SKILL_TAP: TalentScaling = TalentScaling {
    name: "マルチターゲット射撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.36, 0.387, 0.414, 0.45, 0.477, 0.504, 0.54, 0.576, 0.612, 0.648, 0.684, 0.72, 0.765,
        0.81, 0.855,
    ],
};

const CHASCA_SKILL_SHELL: TalentScaling = TalentScaling {
    name: "影狩シェルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.488, 0.5246, 0.5612, 0.61, 0.6466, 0.6832, 0.732, 0.7808, 0.8296, 0.8784, 0.9272, 0.976,
        1.037, 1.098, 1.159,
    ],
};

const CHASCA_SKILL_SHINING_SHELL: TalentScaling = TalentScaling {
    name: "輝く影狩シェルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.66572, 1.790649, 1.915578, 2.08215, 2.207079, 2.332008, 2.49858, 2.665152, 2.831724,
        2.998296, 3.164868, 3.33144, 3.539655, 3.74787, 3.956085,
    ],
};

// -- Elemental Burst: Soul Reaper's Fatal Round -- Anemo --

const CHASCA_BURST_GALESPLITTING: TalentScaling = TalentScaling {
    name: "裂風シェルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.88, 0.946, 1.012, 1.1, 1.166, 1.232, 1.32, 1.408, 1.496, 1.584, 1.672, 1.76, 1.87, 1.98,
        2.09,
    ],
};

const CHASCA_BURST_SOULSEEKER: TalentScaling = TalentScaling {
    name: "魂狩シェルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.034, 1.11155, 1.1891, 1.2925, 1.37005, 1.4476, 1.551, 1.6544, 1.7578, 1.8612, 1.9646,
        2.068, 2.19725, 2.3265, 2.45575,
    ],
};

const CHASCA_BURST_RADIANT: TalentScaling = TalentScaling {
    name: "輝く魂狩シェルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.068, 2.2231, 2.3782, 2.585, 2.7401, 2.8952, 3.102, 3.3088, 3.5156, 3.7224, 3.9292, 4.136,
        4.3945, 4.653, 4.9115,
    ],
};

pub const CHASCA: CharacterData = CharacterData {
    id: "chasca",
    name: "Chasca",
    element: Element::Anemo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [858.0, 9724.0, 10247.0, 11021.0],
    base_atk: [24.0, 274.0, 289.0, 311.0],
    base_def: [51.0, 580.0, 611.0, 657.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "幻羽の飛翔",
            hits: &[
                CHASCA_NORMAL_1,
                CHASCA_NORMAL_2,
                CHASCA_NORMAL_3,
                CHASCA_NORMAL_4,
            ],
            charged: &[CHASCA_AIMED, CHASCA_AIMED_FULL],
            plunging: &[CHASCA_PLUNGE, CHASCA_PLUNGE_LOW, CHASCA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "霊韋の影狩",
            scalings: &[
                CHASCA_SKILL_RESONANCE,
                CHASCA_SKILL_TAP,
                CHASCA_SKILL_SHELL,
                CHASCA_SKILL_SHINING_SHELL,
            ],
        },
        elemental_burst: TalentData {
            name: "魂狩の必殺弾",
            scalings: &[
                CHASCA_BURST_GALESPLITTING,
                CHASCA_BURST_SOULSEEKER,
                CHASCA_BURST_RADIANT,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Faruzan — 4★ Anemo Bow (Sumeru)
// Source: genshin-db-api
// Normal Attack: Parthian Shot
// Elemental Skill: Wind Realm of Nasamjnin
// Elemental Burst: The Wind's Secret Ways
