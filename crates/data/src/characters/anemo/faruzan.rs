use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Heizou — 4★ Anemo Catalyst (Inazuma)
// Source: genshin-db-api
// Normal Attack: Fudou Style Martial Arts
// Elemental Skill: Heartstopper Strike
// Elemental Burst: Windmuster Kick
