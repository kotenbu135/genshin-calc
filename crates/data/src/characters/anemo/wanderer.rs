use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Xianyun — 5★ Anemo Catalyst (Liyue)
// Source: genshin-db-api
// Normal Attack: Word of Wind and Flower (風花の詞)
// Elemental Skill: White Clouds at Dawn (白雲の暁)
// Elemental Burst: Stars Gather at Dusk (群星の夕)
