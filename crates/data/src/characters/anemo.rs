// Game data values that happen to approximate mathematical constants (e.g., 0.7071, 0.318)
// are actual talent scaling percentages from datamining, not approximations of pi/sqrt(2)/etc.
#![allow(clippy::approx_constant)]
use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Chasca — 5★ Anemo Bow (Natlan)
// Source: genshin-db-api (genshin-db-api.vercel.app)
// Normal Attack: Phantom Feather Flurry
// Elemental Skill: Spirit Reins, Shadow Hunt
// Elemental Burst: Soul Reaper's Fatal Round
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
};

// =============================================================================
// Faruzan — 4★ Anemo Bow (Sumeru)
// Source: genshin-db-api
// Normal Attack: Parthian Shot
// Elemental Skill: Wind Realm of Nasamjnin
// Elemental Burst: The Wind's Secret Ways
// =============================================================================

// -- Normal Attack: Parthian Shot -- Physical --

const FARUZAN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.447295, 0.483702, 0.52011, 0.572121, 0.608529, 0.650137, 0.70735, 0.764562, 0.821774,
        0.884187, 0.9466, 1.009013, 1.071427, 1.13384, 1.196253,
    ],
};

const FARUZAN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.421864, 0.456202, 0.49054, 0.539594, 0.573932, 0.613175, 0.667134, 0.721094, 0.775053,
        0.833918, 0.892783, 0.951648, 1.010512, 1.069377, 1.128242,
    ],
};

const FARUZAN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.531635, 0.574907, 0.61818, 0.679998, 0.723271, 0.772725, 0.840725, 0.908725, 0.976724,
        1.050906, 1.125088, 1.199269, 1.273451, 1.347632, 1.421814,
    ],
};

const FARUZAN_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.706206, 0.763688, 0.82117, 0.903287, 0.960769, 1.026463, 1.116791, 1.20712, 1.297449,
        1.395989, 1.494529, 1.59307, 1.69161, 1.790151, 1.888691,
    ],
};

// -- Aimed Shot -- Anemo (charged) --

const FARUZAN_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.9282, 0.9894,
        1.0506, 1.1118, 1.173,
    ],
};

const FARUZAN_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.356, 2.48, 2.635,
        2.79, 2.945,
    ],
};

// -- Plunging Attack -- Physical --

const FARUZAN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
};

const FARUZAN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const FARUZAN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
};

// -- Elemental Skill: Wind Realm of Nasamjnin -- Anemo --

const FARUZAN_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.488, 1.5996, 1.7112, 1.86, 1.9716, 2.0832, 2.232, 2.3808, 2.5296, 2.6784, 2.8272, 2.976,
        3.162, 3.348, 3.534,
    ],
};

const FARUZAN_SKILL_COLLAPSE: TalentScaling = TalentScaling {
    name: "圧潰渦ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.08, 1.161, 1.242, 1.35, 1.431, 1.512, 1.62, 1.728, 1.836, 1.944, 2.052, 2.16, 2.295,
        2.43, 2.565,
    ],
};

// -- Elemental Burst: The Wind's Secret Ways -- Anemo --

const FARUZAN_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        3.776, 4.0592, 4.3424, 4.72, 5.0032, 5.2864, 5.664, 6.0416, 6.4192, 6.7968, 7.1744, 7.552,
        8.024, 8.496, 8.968,
    ],
};

pub const FARUZAN: CharacterData = CharacterData {
    id: "faruzan",
    name: "Faruzan",
    element: Element::Anemo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Sumeru,
    base_hp: [802.0, 8481.0, 8907.0, 9570.0],
    base_atk: [16.0, 173.0, 182.0, 196.0],
    base_def: [53.0, 556.0, 584.0, 628.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "迴身キック射撃",
            hits: &[
                FARUZAN_NORMAL_1,
                FARUZAN_NORMAL_2,
                FARUZAN_NORMAL_3,
                FARUZAN_NORMAL_4,
            ],
            charged: &[FARUZAN_AIMED, FARUZAN_AIMED_FULL],
            plunging: &[FARUZAN_PLUNGE, FARUZAN_PLUNGE_LOW, FARUZAN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "非想風天についての理論",
            scalings: &[FARUZAN_SKILL, FARUZAN_SKILL_COLLAPSE],
        },
        elemental_burst: TalentData {
            name: "烈風波についての秘論",
            scalings: &[FARUZAN_BURST],
        },
    },
};

// =============================================================================
// Heizou — 4★ Anemo Catalyst (Inazuma)
// Source: genshin-db-api
// Normal Attack: Fudou Style Martial Arts
// Elemental Skill: Heartstopper Strike
// Elemental Burst: Windmuster Kick
// =============================================================================

// -- Normal Attack: Fudou Style Martial Arts -- All Anemo (Catalyst) --

const HEIZOU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.374736, 0.402841, 0.430946, 0.46842, 0.496525, 0.52463, 0.562104, 0.599578, 0.637051,
        0.674525, 0.711998, 0.749472, 0.796314, 0.843156, 0.889998,
    ],
};

const HEIZOU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.36852, 0.396159, 0.423798, 0.46065, 0.488289, 0.515928, 0.55278, 0.589632, 0.626484,
        0.663336, 0.700188, 0.73704, 0.783105, 0.82917, 0.875235,
    ],
};

const HEIZOU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.5106, 0.548895, 0.58719, 0.63825, 0.676545, 0.71484, 0.7659, 0.81696, 0.86802, 0.91908,
        0.97014, 1.0212, 1.085025, 1.14885, 1.212675,
    ],
};

const HEIZOU_NORMAL_4A: TalentScaling = TalentScaling {
    name: "4段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.147824, 0.158911, 0.169998, 0.18478, 0.195867, 0.206954, 0.221736, 0.236518, 0.251301,
        0.266083, 0.280866, 0.295648, 0.314126, 0.332604, 0.351082,
    ],
};

const HEIZOU_NORMAL_4B: TalentScaling = TalentScaling {
    name: "4段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.162608, 0.174804, 0.186999, 0.20326, 0.215456, 0.227651, 0.243912, 0.260173, 0.276434,
        0.292694, 0.308955, 0.325216, 0.345542, 0.365868, 0.386194,
    ],
};

const HEIZOU_NORMAL_4C: TalentScaling = TalentScaling {
    name: "4段ダメージ (3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.192176, 0.206589, 0.221002, 0.24022, 0.254633, 0.269046, 0.288264, 0.307482, 0.326699,
        0.345917, 0.365134, 0.384352, 0.408374, 0.432396, 0.456418,
    ],
};

const HEIZOU_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.614496, 0.660583, 0.70667, 0.76812, 0.814207, 0.860294, 0.921744, 0.983194, 1.044643,
        1.106093, 1.167542, 1.228992, 1.305804, 1.382616, 1.459428,
    ],
};

// -- Charged Attack -- Anemo (Catalyst) --

const HEIZOU_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.73, 0.78475, 0.8395, 0.9125, 0.96725, 1.022, 1.095, 1.168, 1.241, 1.314, 1.387, 1.46,
        1.55125, 1.6425, 1.73375,
    ],
};

// -- Plunging Attack -- Anemo (Catalyst) --

const HEIZOU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
};

const HEIZOU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const HEIZOU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
};

// -- Elemental Skill: Heartstopper Strike -- Anemo --

const HEIZOU_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.2752, 2.44584, 2.61648, 2.844, 3.01464, 3.18528, 3.4128, 3.64032, 3.86784, 4.09536,
        4.32288, 4.5504, 4.8348, 5.1192, 5.4036,
    ],
};

const HEIZOU_SKILL_DECLENSION: TalentScaling = TalentScaling {
    name: "変格スタックボーナス",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.5688, 0.61146, 0.65412, 0.711, 0.75366, 0.79632, 0.8532, 0.91008, 0.96696, 1.02384,
        1.08072, 1.1376, 1.2087, 1.2798, 1.3509,
    ],
};

const HEIZOU_SKILL_CONVICTION: TalentScaling = TalentScaling {
    name: "正論ボーナス",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.1376, 1.22292, 1.30824, 1.422, 1.50732, 1.59264, 1.7064, 1.82016, 1.93392, 2.04768,
        2.16144, 2.2752, 2.4174, 2.5596, 2.7018,
    ],
};

// -- Elemental Burst: Windmuster Kick -- Anemo --

const HEIZOU_BURST_VACUUM: TalentScaling = TalentScaling {
    name: "不動流・真空弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        3.14688, 3.382896, 3.618912, 3.9336, 4.169616, 4.405632, 4.72032, 5.035008, 5.349696,
        5.664384, 5.979072, 6.29376, 6.68712, 7.08048, 7.47384,
    ],
};

const HEIZOU_BURST_IRIS: TalentScaling = TalentScaling {
    name: "風威の虹玉ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.21456, 0.230652, 0.246744, 0.2682, 0.284292, 0.300384, 0.32184, 0.343296, 0.364752,
        0.386208, 0.407664, 0.42912, 0.45594, 0.48276, 0.50958,
    ],
};

pub const HEIZOU: CharacterData = CharacterData {
    id: "heizou",
    name: "Heizou",
    element: Element::Anemo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    region: Region::Inazuma,
    base_hp: [894.0, 9445.0, 9919.0, 10657.0],
    base_atk: [19.0, 200.0, 210.0, 225.0],
    base_def: [57.0, 606.0, 636.0, 684.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Anemo, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "不動流格闘術",
            hits: &[
                HEIZOU_NORMAL_1,
                HEIZOU_NORMAL_2,
                HEIZOU_NORMAL_3,
                HEIZOU_NORMAL_4A,
                HEIZOU_NORMAL_4B,
                HEIZOU_NORMAL_4C,
                HEIZOU_NORMAL_5,
            ],
            charged: &[HEIZOU_CHARGED],
            plunging: &[HEIZOU_PLUNGE, HEIZOU_PLUNGE_LOW, HEIZOU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "勠心拳",
            scalings: &[
                HEIZOU_SKILL,
                HEIZOU_SKILL_DECLENSION,
                HEIZOU_SKILL_CONVICTION,
            ],
        },
        elemental_burst: TalentData {
            name: "廻風蹴",
            scalings: &[HEIZOU_BURST_VACUUM, HEIZOU_BURST_IRIS],
        },
    },
};

// =============================================================================
// Ifa — 4★ Anemo Sword (Natlan)
// Source: genshin-db-api
// Normal Attack: Rite of Dispelling Winds
// Elemental Skill: Airborne Disease Prevention
// Elemental Burst: Compound Sedation Field
// =============================================================================

// -- Normal Attack: Rite of Dispelling Winds -- Physical --

const IFA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.536072, 0.576277, 0.616483, 0.67009, 0.710295, 0.750501, 0.804108, 0.857715, 0.911322,
        0.96493, 1.018537, 1.072144, 1.139153, 1.206162, 1.273171,
    ],
};

const IFA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.474672, 0.510272, 0.545873, 0.59334, 0.62894, 0.664541, 0.712008, 0.759475, 0.806942,
        0.85441, 0.901877, 0.949344, 1.008678, 1.068012, 1.127346,
    ],
};

const IFA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.747584, 0.803653, 0.859722, 0.93448, 0.990549, 1.046618, 1.121376, 1.196134, 1.270893,
        1.345651, 1.42041, 1.495168, 1.588616, 1.682064, 1.775512,
    ],
};

// -- Charged Attack -- Physical --

const IFA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4704, 1.58068, 1.69096, 1.838, 1.94828, 2.05856, 2.2056, 2.35264, 2.49968, 2.64672,
        2.79376, 2.9408, 3.1246, 3.3084, 3.4922,
    ],
};

// -- Plunging Attack -- Physical --

const IFA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
};

const IFA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const IFA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
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
};

const IFA_BURST_MARK: TalentScaling = TalentScaling {
    name: "鎮静マークダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.0896, 1.17132, 1.25304, 1.362, 1.44372, 1.52544, 1.6344, 1.74336, 1.85232, 1.96128,
        2.07024, 2.1792, 2.3154, 2.4516, 2.5878,
    ],
};

pub const IFA: CharacterData = CharacterData {
    id: "ifa",
    name: "Ifa",
    element: Element::Anemo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Natlan,
    base_hp: [1039.0, 10987.0, 11539.0, 12397.0],
    base_atk: [19.0, 198.0, 208.0, 223.0],
    base_def: [61.0, 641.0, 673.0, 723.0],
    ascension_stat: AscensionStat::Atk(0.24),
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
};

// =============================================================================
// Jean — 5★ Anemo Sword (Mondstadt)
// Source: genshin-db-api
// Normal Attack: Favonius Bladework (西風剣術)
// Elemental Skill: Gale Blade (風圧剣)
// Elemental Burst: Dandelion Breeze (蒲公英の風)
// =============================================================================

// -- Normal Attack: Favonius Bladework -- Physical --

const JEAN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.48332, 0.52266, 0.562, 0.6182, 0.65754, 0.7025, 0.76432, 0.82614, 0.88796, 0.9554,
        1.032675, 1.12355, 1.214426, 1.305301, 1.404438,
    ],
};

const JEAN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4558, 0.4929, 0.53, 0.583, 0.6201, 0.6625, 0.7208, 0.7791, 0.8374, 0.901, 0.973875,
        1.059576, 1.145277, 1.230978, 1.32447,
    ],
};

const JEAN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.60286, 0.65193, 0.701, 0.7711, 0.82017, 0.87625, 0.95336, 1.03047, 1.10758, 1.1917,
        1.288088, 1.401439, 1.514791, 1.628143, 1.751799,
    ],
};

const JEAN_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.65876, 0.71238, 0.766, 0.8426, 0.89622, 0.9575, 1.04176, 1.12602, 1.21028, 1.3022,
        1.407525, 1.531387, 1.655249, 1.779112, 1.914234,
    ],
};

const JEAN_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.79206, 0.85653, 0.921, 1.0131, 1.07757, 1.15125, 1.25256, 1.35387, 1.45518, 1.5657,
        1.692338, 1.841263, 1.990189, 2.139115, 2.301579,
    ],
};

// -- Charged Attack -- Physical --

const JEAN_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.62024, 1.75212, 1.884, 2.0724, 2.20428, 2.355, 2.56224, 2.76948, 2.97672, 3.2028,
        3.46185, 3.766493, 4.071136, 4.375778, 4.708116,
    ],
};

// -- Plunging Attack -- Physical --

const JEAN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572,
        1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982,
    ],
};

const JEAN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646,
        2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915,
    ],
};

const JEAN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586,
        3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041,
    ],
};

// -- Elemental Skill: Gale Blade -- Anemo --

const JEAN_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.92, 3.139, 3.358, 3.65, 3.869, 4.088, 4.38, 4.672, 4.964, 5.256, 5.548, 5.84, 6.205,
        6.57, 6.935,
    ],
};

// -- Elemental Burst: Dandelion Breeze -- Anemo --

const JEAN_BURST: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        4.248, 4.5666, 4.8852, 5.31, 5.6286, 5.9472, 6.372, 6.7968, 7.2216, 7.6464, 8.0712, 8.496,
        9.027, 9.558, 10.089,
    ],
};

const JEAN_BURST_FIELD: TalentScaling = TalentScaling {
    name: "出入りダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.784, 0.8428, 0.9016, 0.98, 1.0388, 1.0976, 1.176, 1.2544, 1.3328, 1.4112, 1.4896, 1.568,
        1.666, 1.764, 1.862,
    ],
};

pub const JEAN: CharacterData = CharacterData {
    id: "jean",
    name: "Jean",
    element: Element::Anemo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [1144.0, 12965.0, 13662.0, 14695.0],
    base_atk: [19.0, 211.0, 222.0, 239.0],
    base_def: [60.0, 678.0, 715.0, 769.0],
    ascension_stat: AscensionStat::HealingBonus(0.222),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "西風剣術",
            hits: &[
                JEAN_NORMAL_1,
                JEAN_NORMAL_2,
                JEAN_NORMAL_3,
                JEAN_NORMAL_4,
                JEAN_NORMAL_5,
            ],
            charged: &[JEAN_CHARGED],
            plunging: &[JEAN_PLUNGE, JEAN_PLUNGE_LOW, JEAN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "風圧剣",
            scalings: &[JEAN_SKILL],
        },
        elemental_burst: TalentData {
            name: "蒲公英の風",
            scalings: &[JEAN_BURST, JEAN_BURST_FIELD],
        },
    },
};

// =============================================================================
// Kazuha — 5★ Anemo Sword (Inazuma)
// Source: genshin-db-api
// Normal Attack: Garyuu Bladework (我流剣術)
// Elemental Skill: Chihayaburu (千早振る)
// Elemental Burst: Kazuha Slash (万葉の一刀)
// =============================================================================

// -- Normal Attack: Garyuu Bladework -- Physical --

const KAZUHA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.44978, 0.48639, 0.523, 0.5753, 0.61191, 0.65375, 0.71128, 0.76881, 0.82634, 0.8891,
        0.961013, 1.045582, 1.130151, 1.21472, 1.306977,
    ],
};

const KAZUHA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.45236, 0.48918, 0.526, 0.5786, 0.61542, 0.6575, 0.71536, 0.77322, 0.83108, 0.8942,
        0.966525, 1.051579, 1.136633, 1.221688, 1.314474,
    ],
};

const KAZUHA_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.258, 0.279, 0.3, 0.33, 0.351, 0.375, 0.408, 0.441, 0.474, 0.51, 0.55125, 0.59976,
        0.64827, 0.69678, 0.7497,
    ],
};

const KAZUHA_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3096, 0.3348, 0.36, 0.396, 0.4212, 0.45, 0.4896, 0.5292, 0.5688, 0.612, 0.6615, 0.719712,
        0.777924, 0.836136, 0.89964,
    ],
};

const KAZUHA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.60716, 0.65658, 0.706, 0.7766, 0.82602, 0.8825, 0.96016, 1.03782, 1.11548, 1.2002,
        1.297275, 1.411435, 1.525595, 1.639756, 1.764294,
    ],
};

const KAZUHA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ(x3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2537, 0.27435, 0.295, 0.3245, 0.34515, 0.36875, 0.4012, 0.43365, 0.4661, 0.5015,
        0.542063, 0.589764, 0.637465, 0.685167, 0.737205,
    ],
};

// -- Charged Attack -- Physical --

const KAZUHA_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.43, 0.465, 0.5, 0.55, 0.585, 0.625, 0.68, 0.735, 0.79, 0.85, 0.91875, 0.9996, 1.08045,
        1.1613, 1.2495,
    ],
};

const KAZUHA_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.74648, 0.80724, 0.868, 0.9548, 1.01556, 1.085, 1.18048, 1.27596, 1.37144, 1.4756,
        1.59495, 1.735306, 1.875661, 2.016017, 2.169132,
    ],
};

// -- Plunging Attack -- Physical --

const KAZUHA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.818335, 0.884943, 0.951552, 1.046707, 1.113316, 1.18944, 1.294111, 1.398781, 1.503452,
        1.617638, 1.731825, 1.846011, 1.960197, 2.074383, 2.18857,
    ],
};

const KAZUHA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.636323, 1.769512, 1.902701, 2.092971, 2.22616, 2.378376, 2.587673, 2.79697, 3.006267,
        3.234591, 3.462915, 3.69124, 3.919564, 4.147888, 4.376212,
    ],
};

const KAZUHA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        2.043855, 2.210216, 2.376576, 2.614234, 2.780594, 2.97072, 3.232143, 3.493567, 3.75499,
        4.040179, 4.325368, 4.610557, 4.895747, 5.180936, 5.466125,
    ],
};

// -- Elemental Skill: Chihayaburu -- Anemo --

const KAZUHA_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "1回押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.92, 2.064, 2.208, 2.4, 2.544, 2.688, 2.88, 3.072, 3.264, 3.456, 3.648, 3.84, 4.08, 4.32,
        4.56,
    ],
};

const KAZUHA_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.608, 2.8036, 2.9992, 3.26, 3.4556, 3.6512, 3.912, 4.1728, 4.4336, 4.6944, 4.9552, 5.216,
        5.542, 5.868, 6.194,
    ],
};

// -- Elemental Burst: Kazuha Slash -- Anemo --

const KAZUHA_BURST_SLASH: TalentScaling = TalentScaling {
    name: "斬撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.624, 2.8208, 3.0176, 3.28, 3.4768, 3.6736, 3.936, 4.1984, 4.4608, 4.7232, 4.9856, 5.248,
        5.576, 5.904, 6.232,
    ],
};

const KAZUHA_BURST_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.2, 1.29, 1.38, 1.5, 1.59, 1.68, 1.8, 1.92, 2.04, 2.16, 2.28, 2.4, 2.55, 2.7, 2.85,
    ],
};

const KAZUHA_BURST_ELEM: TalentScaling = TalentScaling {
    name: "付加元素ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.36, 0.387, 0.414, 0.45, 0.477, 0.504, 0.54, 0.576, 0.612, 0.648, 0.684, 0.72, 0.765,
        0.81, 0.855,
    ],
};

pub const KAZUHA: CharacterData = CharacterData {
    id: "kazuha",
    name: "Kazuha",
    element: Element::Anemo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [1039.0, 11761.0, 12397.0, 13348.0],
    base_atk: [23.0, 263.0, 277.0, 297.0],
    base_def: [63.0, 710.0, 749.0, 807.0],
    ascension_stat: AscensionStat::ElementalMastery(115.2),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "我流剣術",
            hits: &[
                KAZUHA_NORMAL_1,
                KAZUHA_NORMAL_2,
                KAZUHA_NORMAL_3A,
                KAZUHA_NORMAL_3B,
                KAZUHA_NORMAL_4,
                KAZUHA_NORMAL_5,
            ],
            charged: &[KAZUHA_CHARGED_1, KAZUHA_CHARGED_2],
            plunging: &[KAZUHA_PLUNGE, KAZUHA_PLUNGE_LOW, KAZUHA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "千早振る",
            scalings: &[KAZUHA_SKILL_PRESS, KAZUHA_SKILL_HOLD],
        },
        elemental_burst: TalentData {
            name: "万葉の一刀",
            scalings: &[KAZUHA_BURST_SLASH, KAZUHA_BURST_DOT, KAZUHA_BURST_ELEM],
        },
    },
};

// =============================================================================
// Lan Yan — 4★ Anemo Catalyst (Liyue)
// Source: genshin-db-api
// Normal Attack: Black Pheasant Strides on Water (黒雉水上歩)
// Elemental Skill: Swallow-Wisp Pinion Dance (呑霊羽舞)
// Elemental Burst: Lustrous Moonrise (朗月昇る)
// =============================================================================

// -- Normal Attack: Black Pheasant Strides on Water -- All Anemo (Catalyst) --

const LAN_YAN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.4144, 0.44548, 0.47656, 0.518, 0.54908, 0.58016, 0.6216, 0.66304, 0.70448, 0.74592,
        0.78736, 0.8288, 0.8806, 0.9324, 0.9842,
    ],
};

const LAN_YAN_NORMAL_2A: TalentScaling = TalentScaling {
    name: "2段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.20412, 0.219429, 0.234738, 0.25515, 0.270459, 0.285768, 0.30618, 0.326592, 0.347004,
        0.367416, 0.387828, 0.40824, 0.433755, 0.45927, 0.484785,
    ],
};

const LAN_YAN_NORMAL_2B: TalentScaling = TalentScaling {
    name: "2段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.24948, 0.268191, 0.286902, 0.31185, 0.330561, 0.349272, 0.37422, 0.399168, 0.424116,
        0.449064, 0.474012, 0.49896, 0.530145, 0.56133, 0.592515,
    ],
};

const LAN_YAN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.2692, 0.28939, 0.30958, 0.3365, 0.35669, 0.37688, 0.4038, 0.43072, 0.45764, 0.48456,
        0.51148, 0.5384, 0.57205, 0.6057, 0.63935,
    ],
};

const LAN_YAN_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.6456, 0.69402, 0.74244, 0.807, 0.85542, 0.90384, 0.9684, 1.03296, 1.09752, 1.16208,
        1.22664, 1.2912, 1.3719, 1.4526, 1.5333,
    ],
};

// -- Charged Attack -- Anemo (Catalyst) --

const LAN_YAN_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ(x3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.3784, 0.40678, 0.43516, 0.473, 0.50138, 0.52976, 0.5676, 0.60544, 0.64328, 0.68112,
        0.71896, 0.7568, 0.8041, 0.8514, 0.8987,
    ],
};

// -- Plunging Attack -- Anemo (Catalyst) --

const LAN_YAN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
};

const LAN_YAN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const LAN_YAN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
};

// -- Elemental Skill: Swallow-Wisp Pinion Dance -- Anemo --

const LAN_YAN_SKILL: TalentScaling = TalentScaling {
    name: "羽月環ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.96256, 1.034752, 1.106944, 1.2032, 1.275392, 1.347584, 1.44384, 1.540096, 1.636352,
        1.732608, 1.828864, 1.92512, 2.04544, 2.16576, 2.28608,
    ],
};

// -- Elemental Burst: Lustrous Moonrise -- Anemo --

const LAN_YAN_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ(x3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.41064, 2.591438, 2.772236, 3.0133, 3.194098, 3.374896, 3.61596, 3.857024, 4.098088,
        4.339152, 4.580216, 4.82128, 5.12261, 5.42394, 5.72527,
    ],
};

pub const LAN_YAN: CharacterData = CharacterData {
    id: "lan_yan",
    name: "Lan Yan",
    element: Element::Anemo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [912.0, 9642.0, 10129.0, 10875.0],
    base_atk: [20.0, 209.0, 219.0, 236.0],
    base_def: [57.0, 601.0, 631.0, 678.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "黒雉水上歩",
            hits: &[
                LAN_YAN_NORMAL_1,
                LAN_YAN_NORMAL_2A,
                LAN_YAN_NORMAL_2B,
                LAN_YAN_NORMAL_3,
                LAN_YAN_NORMAL_4,
            ],
            charged: &[LAN_YAN_CHARGED],
            plunging: &[LAN_YAN_PLUNGE, LAN_YAN_PLUNGE_LOW, LAN_YAN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "呑霊羽舞",
            scalings: &[LAN_YAN_SKILL],
        },
        elemental_burst: TalentData {
            name: "朗月昇る",
            scalings: &[LAN_YAN_BURST],
        },
    },
};

// =============================================================================
// Lynette — 4★ Anemo Sword (Fontaine)
// Source: genshin-db-api
// Normal Attack: Rapid Ritesword (迅捷な儀刃)
// Elemental Skill: Enigmatic Feint (エニグマティック・フェイント)
// Elemental Burst: Magic Trick: Astonishing Shift (マジック・アストニッシングシフト)
// =============================================================================

// -- Normal Attack: Rapid Ritesword -- Physical --

const LYNETTE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.430817, 0.465884, 0.50095, 0.551045, 0.586112, 0.626188, 0.681292, 0.736397, 0.791501,
        0.851615, 0.911729, 0.971843, 1.031957, 1.092071, 1.152185,
    ],
};

const LYNETTE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.376121, 0.406736, 0.43735, 0.481085, 0.511699, 0.546687, 0.594796, 0.642904, 0.691013,
        0.743495, 0.795977, 0.848459, 0.900941, 0.953423, 1.005905,
    ],
};

const LYNETTE_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.27864, 0.30132, 0.324, 0.3564, 0.37908, 0.405, 0.44064, 0.47628, 0.51192, 0.5508,
        0.58968, 0.62856, 0.66744, 0.70632, 0.7452,
    ],
};

const LYNETTE_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.215929, 0.233504, 0.25108, 0.276188, 0.293764, 0.31385, 0.341469, 0.369088, 0.396706,
        0.426836, 0.456966, 0.487095, 0.517225, 0.547354, 0.577484,
    ],
};

const LYNETTE_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.631541, 0.682945, 0.73435, 0.807785, 0.859189, 0.917937, 0.998716, 1.079494, 1.160273,
        1.248395, 1.336517, 1.424639, 1.512761, 1.600883, 1.689005,
    ],
};

// -- Charged Attack -- Physical --

const LYNETTE_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.44204, 0.47802, 0.514, 0.5654, 0.60138, 0.6425, 0.69904, 0.75558, 0.81212, 0.8738,
        0.93548, 0.99716, 1.05884, 1.12052, 1.1822,
    ],
};

const LYNETTE_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.61404, 0.66402, 0.714, 0.7854, 0.83538, 0.8925, 0.97104, 1.04958, 1.12812, 1.2138,
        1.29948, 1.38516, 1.47084, 1.55652, 1.6422,
    ],
};

// -- Plunging Attack -- Physical --

const LYNETTE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572,
        1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982,
    ],
};

const LYNETTE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646,
        2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915,
    ],
};

const LYNETTE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586,
        3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041,
    ],
};

// -- Elemental Skill: Enigmatic Feint -- Anemo --

const LYNETTE_SKILL: TalentScaling = TalentScaling {
    name: "エニグマスラストダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.68, 2.881, 3.082, 3.35, 3.551, 3.752, 4.02, 4.288, 4.556, 4.824, 5.092, 5.36, 5.695,
        6.03, 6.365,
    ],
};

const LYNETTE_SKILL_SURGING: TalentScaling = TalentScaling {
    name: "サージングブレードダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.312, 0.3354, 0.3588, 0.39, 0.4134, 0.4368, 0.468, 0.4992, 0.5304, 0.5616, 0.5928, 0.624,
        0.663, 0.702, 0.741,
    ],
};

// -- Elemental Burst: Magic Trick: Astonishing Shift -- Anemo --

const LYNETTE_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.832, 0.8944, 0.9568, 1.04, 1.1024, 1.1648, 1.248, 1.3312, 1.4144, 1.4976, 1.5808, 1.664,
        1.768, 1.872, 1.976,
    ],
};

const LYNETTE_BURST_BOX: TalentScaling = TalentScaling {
    name: "ボグルキャットボックスダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.512, 0.5504, 0.5888, 0.64, 0.6784, 0.7168, 0.768, 0.8192, 0.8704, 0.9216, 0.9728, 1.024,
        1.088, 1.152, 1.216,
    ],
};

const LYNETTE_BURST_VIVID: TalentScaling = TalentScaling {
    name: "ヴィヴィッドショットダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.456, 0.4902, 0.5244, 0.57, 0.6042, 0.6384, 0.684, 0.7296, 0.7752, 0.8208, 0.8664, 0.912,
        0.969, 1.026, 1.083,
    ],
};

pub const LYNETTE: CharacterData = CharacterData {
    id: "lynette",
    name: "Lynette",
    element: Element::Anemo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Fontaine,
    base_hp: [1039.0, 10987.0, 11539.0, 12397.0],
    base_atk: [19.0, 205.0, 215.0, 232.0],
    base_def: [60.0, 630.0, 662.0, 712.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Anemo, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "迅捷な儀刃",
            hits: &[
                LYNETTE_NORMAL_1,
                LYNETTE_NORMAL_2,
                LYNETTE_NORMAL_3A,
                LYNETTE_NORMAL_3B,
                LYNETTE_NORMAL_4,
            ],
            charged: &[LYNETTE_CHARGED_1, LYNETTE_CHARGED_2],
            plunging: &[LYNETTE_PLUNGE, LYNETTE_PLUNGE_LOW, LYNETTE_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "謎めいたフェイント",
            scalings: &[LYNETTE_SKILL, LYNETTE_SKILL_SURGING],
        },
        elemental_burst: TalentData {
            name: "マジック・アストニッシングシフト",
            scalings: &[LYNETTE_BURST, LYNETTE_BURST_BOX, LYNETTE_BURST_VIVID],
        },
    },
};

// =============================================================================
// Mizuki — 5★ Anemo Catalyst (Inazuma)
// Source: genshin-db-api
// Normal Attack: Pure Heart, Pure Dreams (清心清夢)
// Elemental Skill: Aisa Utamakura Pilgrimage (逢坂歌枕巡り)
// Elemental Burst: Anraku Secret Spring Therapy (安楽秘湯療法)
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
};

const MIZUKI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.469144, 0.50433, 0.539516, 0.58643, 0.621616, 0.656802, 0.703716, 0.75063, 0.797545,
        0.844459, 0.891374, 0.938288, 0.996931, 1.055574, 1.114217,
    ],
};

const MIZUKI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.713688, 0.767215, 0.820741, 0.89211, 0.945637, 0.999163, 1.070532, 1.141901, 1.21327,
        1.284638, 1.356007, 1.427376, 1.516587, 1.605798, 1.695009,
    ],
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
};

const MIZUKI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const MIZUKI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
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
};

const MIZUKI_SKILL_CONTINUOUS: TalentScaling = TalentScaling {
    name: "継続攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.44912, 0.482804, 0.516488, 0.5614, 0.595084, 0.628768, 0.67368, 0.718592, 0.763504,
        0.808416, 0.853328, 0.89824, 0.95438, 1.01052, 1.06666,
    ],
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
};

const MIZUKI_BURST_SHOCKWAVE: TalentScaling = TalentScaling {
    name: "夢念衝撃波ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.7056, 0.75852, 0.81144, 0.882, 0.93492, 0.98784, 1.0584, 1.12896, 1.19952, 1.27008,
        1.34064, 1.4112, 1.4994, 1.5876, 1.6758,
    ],
};

pub const MIZUKI: CharacterData = CharacterData {
    id: "mizuki",
    name: "Mizuki",
    element: Element::Anemo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [1005.0, 11388.0, 12000.0, 12907.0],
    base_atk: [26.0, 291.0, 307.0, 330.0],
    base_def: [63.0, 714.0, 753.0, 810.0],
    ascension_stat: AscensionStat::Hp(0.288),
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
};

// =============================================================================
// Sayu — 4★ Anemo Claymore (Inazuma)
// Source: genshin-db-api
// Normal Attack: Shuumatsuban Ninja Blade (終末番忍刀)
// Elemental Skill: Yoohoo Art: Fuuin Dash (風風輪・旋旋)
// Elemental Burst: Yoohoo Art: Mujina Flurry (呼び出しムジナの群れ)
// =============================================================================

// -- Normal Attack: Shuumatsuban Ninja Blade -- Physical --

const SAYU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7224, 0.7812, 0.84, 0.924, 0.9828, 1.05, 1.1424, 1.2348, 1.3272, 1.428, 1.5435, 1.679328,
        1.815156, 1.950984, 2.09916,
    ],
};

const SAYU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7138, 0.7719, 0.83, 0.913, 0.9711, 1.0375, 1.1288, 1.2201, 1.3114, 1.411, 1.525125,
        1.659336, 1.793547, 1.927758, 2.07417,
    ],
};

const SAYU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4343, 0.46965, 0.505, 0.5555, 0.59085, 0.63125, 0.6868, 0.74235, 0.7979, 0.8585,
        0.927937, 1.009596, 1.091255, 1.172913, 1.261995,
    ],
};

const SAYU_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.98126, 1.06113, 1.141, 1.2551, 1.33497, 1.42625, 1.55176, 1.67727, 1.80278, 1.9397,
        2.096588, 2.281087, 2.465587, 2.650087, 2.851359,
    ],
};

// -- Charged Attack -- Physical --

const SAYU_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.625455, 0.676364, 0.727273, 0.8, 0.850909, 0.909091, 0.989091, 1.069091, 1.149091,
        1.236364, 1.336364, 1.453964, 1.571564, 1.689164, 1.817455,
    ],
};

const SAYU_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1309, 1.22295, 1.315, 1.4465, 1.53855, 1.64375, 1.7884, 1.93305, 2.0777, 2.2355,
        2.416313, 2.628948, 2.841584, 3.054219, 3.286185,
    ],
};

// -- Plunging Attack -- Physical --

const SAYU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.745878, 0.806589, 0.8673, 0.95403, 1.014741, 1.084125, 1.179528, 1.274931, 1.370334,
        1.47441, 1.578486, 1.682562, 1.786638, 1.890714, 1.99479,
    ],
};

const SAYU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.49144, 1.612836, 1.734233, 1.907656, 2.029052, 2.167791, 2.358556, 2.549322, 2.740087,
        2.948195, 3.156303, 3.364411, 3.572519, 3.780627, 3.988735,
    ],
};

const SAYU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.862889, 2.01452, 2.16615, 2.382765, 2.534396, 2.707688, 2.945964, 3.184241, 3.422517,
        3.682455, 3.942393, 4.202331, 4.462269, 4.722207, 4.982145,
    ],
};

// -- Elemental Skill: Yoohoo Art: Fuuin Dash -- Anemo --

const SAYU_SKILL_WINDWHEEL: TalentScaling = TalentScaling {
    name: "風風輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.36, 0.387, 0.414, 0.45, 0.477, 0.504, 0.54, 0.576, 0.612, 0.648, 0.684, 0.72, 0.765,
        0.81, 0.855,
    ],
};

const SAYU_SKILL_WINDWHEEL_ELEM: TalentScaling = TalentScaling {
    name: "風風輪元素ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.168, 0.1806, 0.1932, 0.21, 0.2226, 0.2352, 0.252, 0.2688, 0.2856, 0.3024, 0.3192, 0.336,
        0.357, 0.378, 0.399,
    ],
};

const SAYU_SKILL_KICK_PRESS: TalentScaling = TalentScaling {
    name: "旋風キック短押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.584, 1.7028, 1.8216, 1.98, 2.0988, 2.2176, 2.376, 2.5344, 2.6928, 2.8512, 3.0096, 3.168,
        3.366, 3.564, 3.762,
    ],
};

const SAYU_SKILL_KICK_HOLD: TalentScaling = TalentScaling {
    name: "旋風キック長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.176, 2.3392, 2.5024, 2.72, 2.8832, 3.0464, 3.264, 3.4816, 3.6992, 3.9168, 4.1344, 4.352,
        4.624, 4.896, 5.168,
    ],
};

const SAYU_SKILL_KICK_ELEM: TalentScaling = TalentScaling {
    name: "旋風キック元素ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.7616, 0.81872, 0.87584, 0.952, 1.00912, 1.06624, 1.1424, 1.21856, 1.29472, 1.37088,
        1.44704, 1.5232, 1.6184, 1.7136, 1.8088,
    ],
};

// -- Elemental Burst: Yoohoo Art: Mujina Flurry -- Anemo --

const SAYU_BURST_ACTIVATION: TalentScaling = TalentScaling {
    name: "発動ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.168, 1.2556, 1.3432, 1.46, 1.5476, 1.6352, 1.752, 1.8688, 1.9856, 2.1024, 2.2192, 2.336,
        2.482, 2.628, 2.774,
    ],
};

const SAYU_BURST_DARUMA: TalentScaling = TalentScaling {
    name: "ムジムジだるまダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.52, 0.559, 0.598, 0.65, 0.689, 0.728, 0.78, 0.832, 0.884, 0.936, 0.988, 1.04, 1.105,
        1.17, 1.235,
    ],
};

pub const SAYU: CharacterData = CharacterData {
    id: "sayu",
    name: "Sayu",
    element: Element::Anemo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Inazuma,
    base_hp: [994.0, 10505.0, 11033.0, 11854.0],
    base_atk: [20.0, 216.0, 227.0, 244.0],
    base_def: [62.0, 660.0, 693.0, 745.0],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "終末番忍刀",
            hits: &[SAYU_NORMAL_1, SAYU_NORMAL_2, SAYU_NORMAL_3, SAYU_NORMAL_4],
            charged: &[SAYU_CHARGED_SPINNING, SAYU_CHARGED_FINAL],
            plunging: &[SAYU_PLUNGE, SAYU_PLUNGE_LOW, SAYU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "風風輪・旋旋",
            scalings: &[
                SAYU_SKILL_WINDWHEEL,
                SAYU_SKILL_WINDWHEEL_ELEM,
                SAYU_SKILL_KICK_PRESS,
                SAYU_SKILL_KICK_HOLD,
                SAYU_SKILL_KICK_ELEM,
            ],
        },
        elemental_burst: TalentData {
            name: "呼び出しムジナの群れ",
            scalings: &[SAYU_BURST_ACTIVATION, SAYU_BURST_DARUMA],
        },
    },
};

// =============================================================================
// Sucrose — 4★ Anemo Catalyst (Mondstadt)
// Source: genshin-db-api
// Normal Attack: Wind Spirit Creation (風霊作成)
// Elemental Skill: Astable Anemohypostasis Creation - 6308
// Elemental Burst: Forbidden Creation - Isomer 75 / Type II
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
};

const SUCROSE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.30616, 0.329122, 0.352084, 0.3827, 0.405662, 0.428624, 0.45924, 0.489856, 0.520472,
        0.551088, 0.581704, 0.61232, 0.65059, 0.68886, 0.72713,
    ],
};

const SUCROSE_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.38448, 0.413316, 0.442152, 0.4806, 0.509436, 0.538272, 0.57672, 0.615168, 0.653616,
        0.692064, 0.730512, 0.76896, 0.81702, 0.86508, 0.91314,
    ],
};

const SUCROSE_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.479176, 0.515114, 0.551052, 0.59897, 0.634908, 0.670846, 0.718764, 0.766682, 0.814599,
        0.862517, 0.910434, 0.958352, 1.018249, 1.078146, 1.138043,
    ],
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
};

const SUCROSE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const SUCROSE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
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
};

const SUCROSE_BURST_ELEM: TalentScaling = TalentScaling {
    name: "付加元素ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.44, 0.473, 0.506, 0.55, 0.583, 0.616, 0.66, 0.704, 0.748, 0.792, 0.836, 0.88, 0.935,
        0.99, 1.045,
    ],
};

pub const SUCROSE: CharacterData = CharacterData {
    id: "sucrose",
    name: "Sucrose",
    element: Element::Anemo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [775.0, 8192.0, 8604.0, 9244.0],
    base_atk: [14.0, 151.0, 158.0, 170.0],
    base_def: [59.0, 623.0, 654.0, 703.0],
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
};

// =============================================================================
// Venti — 5★ Anemo Bow (Mondstadt)
// Source: genshin-db-api
// Normal Attack: Divine Marksmanship (天賦の射術)
// Elemental Skill: Skyward Sonnet (高天の歌)
// Elemental Burst: Wind's Grand Ode (風神の詩)
// =============================================================================

// -- Normal Attack: Divine Marksmanship -- Physical --

const VENTI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.20382, 0.22041, 0.237, 0.2607, 0.27729, 0.29625, 0.32232, 0.34839, 0.37446, 0.4029,
        0.435487, 0.47381, 0.512133, 0.550456, 0.592263,
    ],
};

const VENTI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.44376, 0.47988, 0.516, 0.5676, 0.60372, 0.645, 0.70176, 0.75852, 0.81528, 0.8772,
        0.94815, 1.031587, 1.115024, 1.198462, 1.289484,
    ],
};

const VENTI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.52374, 0.56637, 0.609, 0.6699, 0.71253, 0.76125, 0.82824, 0.89523, 0.96222, 1.0353,
        1.119037, 1.217513, 1.315988, 1.414463, 1.521891,
    ],
};

const VENTI_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.26058, 0.28179, 0.303, 0.3333, 0.35451, 0.37875, 0.41208, 0.44541, 0.47874, 0.5151,
        0.556762, 0.605758, 0.654753, 0.703748, 0.757197,
    ],
};

const VENTI_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.50654, 0.54777, 0.589, 0.6479, 0.68913, 0.73625, 0.80104, 0.86583, 0.93062, 1.0013,
        1.082288, 1.177529, 1.27277, 1.368011, 1.471911,
    ],
};

const VENTI_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7095, 0.76725, 0.825, 0.9075, 0.96525, 1.03125, 1.122, 1.21275, 1.3035, 1.4025, 1.515937,
        1.64934, 1.782742, 1.916145, 2.061675,
    ],
};

// -- Aimed Shot -- Anemo (charged) --

const VENTI_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.51, 0.561, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.867, 0.937125,
        1.019592, 1.102059, 1.184526, 1.27449,
    ],
};

const VENTI_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.24, 1.333, 1.426, 1.55, 1.643, 1.736, 1.86, 1.984, 2.108, 2.232, 2.36096, 2.5296,
        2.69824, 2.86688, 3.03552,
    ],
};

// -- Plunging Attack -- Physical --

const VENTI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
};

const VENTI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const VENTI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
};

// -- Elemental Skill: Skyward Sonnet -- Anemo --

const VENTI_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "1回押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.76, 2.967, 3.174, 3.45, 3.657, 3.864, 4.14, 4.416, 4.692, 4.968, 5.244, 5.52, 5.865,
        6.21, 6.555,
    ],
};

const VENTI_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        3.8, 4.085, 4.37, 4.75, 5.035, 5.32, 5.7, 6.08, 6.46, 6.84, 7.22, 7.6, 8.075, 8.55, 9.025,
    ],
};

// -- Elemental Burst: Wind's Grand Ode -- Anemo --

const VENTI_BURST_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.376, 0.4042, 0.4324, 0.47, 0.4982, 0.5264, 0.564, 0.6016, 0.6392, 0.6768, 0.7144, 0.752,
        0.799, 0.846, 0.893,
    ],
};

const VENTI_BURST_ELEM: TalentScaling = TalentScaling {
    name: "付加元素ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.188, 0.2021, 0.2162, 0.235, 0.2491, 0.2632, 0.282, 0.3008, 0.3196, 0.3384, 0.3572, 0.376,
        0.3995, 0.423, 0.4465,
    ],
};

pub const VENTI: CharacterData = CharacterData {
    id: "venti",
    name: "Venti",
    element: Element::Anemo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [820.0, 9292.0, 9791.0, 10531.0],
    base_atk: [20.0, 232.0, 245.0, 263.0],
    base_def: [52.0, 590.0, 622.0, 669.0],
    ascension_stat: AscensionStat::EnergyRecharge(0.32),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "天賦の射術",
            hits: &[
                VENTI_NORMAL_1,
                VENTI_NORMAL_2,
                VENTI_NORMAL_3,
                VENTI_NORMAL_4,
                VENTI_NORMAL_5,
                VENTI_NORMAL_6,
            ],
            charged: &[VENTI_AIMED, VENTI_AIMED_FULL],
            plunging: &[VENTI_PLUNGE, VENTI_PLUNGE_LOW, VENTI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "高天の歌",
            scalings: &[VENTI_SKILL_PRESS, VENTI_SKILL_HOLD],
        },
        elemental_burst: TalentData {
            name: "風神の詩",
            scalings: &[VENTI_BURST_DOT, VENTI_BURST_ELEM],
        },
    },
};

// =============================================================================
// Wanderer — 5★ Anemo Catalyst (Sumeru)
// Source: genshin-db-api
// Normal Attack: Yuuban Meigen (夕番銘詠)
// Elemental Skill: Hanega: Song of the Wind (羽化・風の歌)
// Elemental Burst: Kyougen: Five Ceremonial Plays (狂言・五番の演目)
// =============================================================================

// -- Normal Attack: Yuuban Meigen -- All Anemo (Catalyst) --

const WANDERER_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.68714, 0.74307, 0.799, 0.8789, 0.93483, 0.99875, 1.08664, 1.17453, 1.26242, 1.3583,
        1.45418, 1.55006, 1.64594, 1.74182, 1.8377,
    ],
};

const WANDERER_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.65016, 0.70308, 0.756, 0.8316, 0.88452, 0.945, 1.02816, 1.11132, 1.19448, 1.2852,
        1.37592, 1.46664, 1.55736, 1.64808, 1.7388,
    ],
};

const WANDERER_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.47644, 0.51522, 0.554, 0.6094, 0.64818, 0.6925, 0.75344, 0.81438, 0.87532, 0.9418,
        1.00828, 1.07476, 1.14124, 1.20772, 1.2742,
    ],
};

// -- Charged Attack -- Anemo (Catalyst) --

const WANDERER_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.3208, 1.41986, 1.51892, 1.651, 1.75006, 1.84912, 1.9812, 2.11328, 2.24536, 2.37744,
        2.50952, 2.6416, 2.8067, 2.9718, 3.1369,
    ],
};

// -- Plunging Attack -- Anemo (Catalyst) --

const WANDERER_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
};

const WANDERER_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const WANDERER_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
};

// -- Elemental Skill: Hanega: Song of the Wind -- Anemo --

const WANDERER_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.952, 1.0234, 1.0948, 1.19, 1.2614, 1.3328, 1.428, 1.5232, 1.6184, 1.7136, 1.8088, 1.904,
        2.023, 2.142, 2.261,
    ],
};

// -- Elemental Burst: Kyougen: Five Ceremonial Plays -- Anemo --

const WANDERER_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ(x5)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.472, 1.5824, 1.6928, 1.84, 1.9504, 2.0608, 2.208, 2.3552, 2.5024, 2.6496, 2.7968, 2.944,
        3.128, 3.312, 3.496,
    ],
};

pub const WANDERER: CharacterData = CharacterData {
    id: "wanderer",
    name: "Wanderer",
    element: Element::Anemo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Sumeru,
    base_hp: [791.0, 8971.0, 9450.0, 10164.0],
    base_atk: [26.0, 289.0, 305.0, 328.0],
    base_def: [47.0, 536.0, 564.0, 607.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "夕番銘詠",
            hits: &[WANDERER_NORMAL_1, WANDERER_NORMAL_2, WANDERER_NORMAL_3],
            charged: &[WANDERER_CHARGED],
            plunging: &[WANDERER_PLUNGE, WANDERER_PLUNGE_LOW, WANDERER_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "羽化・風の歌",
            scalings: &[WANDERER_SKILL],
        },
        elemental_burst: TalentData {
            name: "狂言・五番の演目",
            scalings: &[WANDERER_BURST],
        },
    },
};

// =============================================================================
// Xianyun — 5★ Anemo Catalyst (Liyue)
// Source: genshin-db-api
// Normal Attack: Word of Wind and Flower (風花の詞)
// Elemental Skill: White Clouds at Dawn (白雲の暁)
// Elemental Burst: Stars Gather at Dusk (群星の夕)
// =============================================================================

// -- Normal Attack: Word of Wind and Flower -- All Anemo (Catalyst) --

const XIANYUN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.403416, 0.433672, 0.463928, 0.50427, 0.534526, 0.564782, 0.605124, 0.645466, 0.685807,
        0.726149, 0.766491, 0.806832, 0.857259, 0.907686, 0.958113,
    ],
};

const XIANYUN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.389064, 0.418244, 0.447424, 0.48633, 0.51551, 0.54469, 0.583596, 0.622502, 0.661407,
        0.700313, 0.739219, 0.778124, 0.826757, 0.87539, 0.924023,
    ],
};

const XIANYUN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.488784, 0.525443, 0.562101, 0.61098, 0.647639, 0.684297, 0.733176, 0.782054, 0.830933,
        0.879812, 0.92869, 0.977569, 1.038667, 1.099765, 1.160862,
    ],
};

const XIANYUN_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.649464, 0.698174, 0.746884, 0.81183, 0.86054, 0.90925, 0.974196, 1.039142, 1.104088,
        1.169034, 1.23398, 1.298926, 1.380109, 1.461292, 1.542475,
    ],
};

// -- Charged Attack -- Anemo (Catalyst) --

const XIANYUN_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.2312, 1.32354, 1.41588, 1.5390, 1.63134, 1.72368, 1.8474, 1.97112, 2.09484, 2.21856,
        2.34228, 2.466, 2.61975, 2.7735, 2.92725,
    ],
};

// -- Plunging Attack -- Anemo (Catalyst) --

const XIANYUN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
};

const XIANYUN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
};

const XIANYUN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
};

// -- Elemental Skill: White Clouds at Dawn -- Anemo --

const XIANYUN_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.248, 0.2666, 0.2852, 0.31, 0.3286, 0.3472, 0.372, 0.3968, 0.4216, 0.4464, 0.4712, 0.496,
        0.527, 0.558, 0.589,
    ],
};

const XIANYUN_SKILL_CLOUD_1: TalentScaling = TalentScaling {
    name: "流雲波1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.16, 1.247, 1.334, 1.45, 1.537, 1.624, 1.74, 1.856, 1.972, 2.088, 2.204, 2.32, 2.465,
        2.61, 2.755,
    ],
};

const XIANYUN_SKILL_CLOUD_2: TalentScaling = TalentScaling {
    name: "流雲波2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.48, 1.591, 1.702, 1.85, 1.961, 2.072, 2.22, 2.368, 2.516, 2.664, 2.812, 2.96, 3.145,
        3.33, 3.515,
    ],
};

const XIANYUN_SKILL_CLOUD_3: TalentScaling = TalentScaling {
    name: "流雲波3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        3.376, 3.6292, 3.8824, 4.22, 4.4732, 4.7264, 5.064, 5.4016, 5.7392, 6.0768, 6.4144, 6.752,
        7.174, 7.596, 8.018,
    ],
};

// -- Elemental Burst: Stars Gather at Dusk -- Anemo --

const XIANYUN_BURST: TalentScaling = TalentScaling {
    name: "初回ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.08, 1.161, 1.242, 1.35, 1.431, 1.512, 1.62, 1.728, 1.836, 1.944, 2.052, 2.16, 2.295,
        2.43, 2.565,
    ],
};

const XIANYUN_BURST_STARWICKER: TalentScaling = TalentScaling {
    name: "星枝ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.392, 0.4214, 0.4508, 0.49, 0.5194, 0.5488, 0.588, 0.6272, 0.6664, 0.7056, 0.7448, 0.784,
        0.8330, 0.882, 0.931,
    ],
};

pub const XIANYUN: CharacterData = CharacterData {
    id: "xianyun",
    name: "Xianyun",
    element: Element::Anemo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [810.0, 9184.0, 9677.0, 10409.0],
    base_atk: [26.0, 295.0, 311.0, 335.0],
    base_def: [45.0, 506.0, 533.0, 573.0],
    ascension_stat: AscensionStat::Atk(0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "風花の詞",
            hits: &[
                XIANYUN_NORMAL_1,
                XIANYUN_NORMAL_2,
                XIANYUN_NORMAL_3,
                XIANYUN_NORMAL_4,
            ],
            charged: &[XIANYUN_CHARGED],
            plunging: &[XIANYUN_PLUNGE, XIANYUN_PLUNGE_LOW, XIANYUN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "白雲の暁",
            scalings: &[
                XIANYUN_SKILL,
                XIANYUN_SKILL_CLOUD_1,
                XIANYUN_SKILL_CLOUD_2,
                XIANYUN_SKILL_CLOUD_3,
            ],
        },
        elemental_burst: TalentData {
            name: "群星の夕",
            scalings: &[XIANYUN_BURST, XIANYUN_BURST_STARWICKER],
        },
    },
};

// =============================================================================
// Xiao — 5★ Anemo Polearm (Liyue)
// Source: genshin-db-api
// Normal Attack: Whirlwind Thrust (旋風キック)
// Elemental Skill: Lemniscatic Wind Cycling (風輪両立)
// Elemental Burst: Bane of All Evil (靖妖儺舞)
// =============================================================================

// -- Normal Attack: Whirlwind Thrust -- Physical --

const XIAO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.27544, 0.29422, 0.313, 0.33804, 0.35682, 0.37873, 0.4069, 0.43507, 0.46324, 0.49141,
        0.51958, 0.54775, 0.57592, 0.60409, 0.63226,
    ],
};

const XIAO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.56936, 0.60818, 0.647, 0.69876, 0.73758, 0.78287, 0.8411, 0.89933, 0.95756, 1.01579,
        1.07402, 1.13225, 1.19048, 1.24871, 1.30694,
    ],
};

const XIAO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.68552, 0.73226, 0.779, 0.84132, 0.88806, 0.94259, 1.0127, 1.08281, 1.15292, 1.22303,
        1.29314, 1.36325, 1.43336, 1.50347, 1.57358,
    ],
};

const XIAO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.37664, 0.40232, 0.428, 0.46224, 0.48792, 0.51788, 0.5564, 0.59492, 0.63344, 0.67196,
        0.71048, 0.749, 0.78752, 0.82604, 0.86456,
    ],
};

const XIAO_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.71544, 0.76422, 0.813, 0.87804, 0.92682, 0.98373, 1.0569, 1.13007, 1.20324, 1.27641,
        1.34958, 1.42275, 1.49592, 1.56909, 1.64226,
    ],
};

const XIAO_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.95832, 1.02366, 1.089, 1.17612, 1.24146, 1.31769, 1.4157, 1.51371, 1.61172, 1.70973,
        1.80774, 1.90575, 2.00376, 2.10177, 2.19978,
    ],
};

// -- Charged Attack -- Physical --

const XIAO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.21088, 1.29344, 1.376, 1.48608, 1.56864, 1.66496, 1.7888, 1.91264, 2.03648, 2.16032,
        2.28416, 2.408, 2.53184, 2.65568, 2.77952,
    ],
};

// -- Plunging Attack -- Physical --

const XIAO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.818335, 0.884943, 0.951552, 1.046707, 1.113316, 1.18944, 1.294111, 1.398781, 1.503452,
        1.617638, 1.731825, 1.846011, 1.960197, 2.074383, 2.18857,
    ],
};

const XIAO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.636323, 1.769512, 1.902701, 2.092971, 2.22616, 2.378376, 2.587673, 2.79697, 3.006267,
        3.234591, 3.462915, 3.69124, 3.919564, 4.147888, 4.376212,
    ],
};

const XIAO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        2.043855, 2.210216, 2.376576, 2.614234, 2.780594, 2.97072, 3.232143, 3.493567, 3.75499,
        4.040179, 4.325368, 4.610557, 4.895747, 5.180936, 5.466125,
    ],
};

// -- Elemental Skill: Lemniscatic Wind Cycling -- Anemo --

const XIAO_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.528, 2.7176, 2.9072, 3.16, 3.3496, 3.5392, 3.792, 4.0448, 4.2976, 4.5504, 4.8032, 5.056,
        5.372, 5.688, 6.004,
    ],
};

// -- Elemental Burst: Bane of All Evil --
// Xiao's burst provides a DMG Bonus buff, not direct damage scalings.
// No damage talent scalings to include.

pub const XIAO: CharacterData = CharacterData {
    id: "xiao",
    name: "Xiao",
    element: Element::Anemo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [991.0, 11236.0, 11840.0, 12736.0],
    base_atk: [27.0, 310.0, 327.0, 349.0],
    base_def: [62.0, 707.0, 745.0, 799.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "旋風槍術",
            hits: &[
                XIAO_NORMAL_1,
                XIAO_NORMAL_2,
                XIAO_NORMAL_3,
                XIAO_NORMAL_4,
                XIAO_NORMAL_5,
                XIAO_NORMAL_6,
            ],
            charged: &[XIAO_CHARGED],
            plunging: &[XIAO_PLUNGE, XIAO_PLUNGE_LOW, XIAO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "風輪両立",
            scalings: &[XIAO_SKILL],
        },
        elemental_burst: TalentData {
            name: "靖妖儺舞",
            scalings: &[],
        },
    },
};
