use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    dynamic_bonus: None,
};

const XIANYUN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.389064, 0.418244, 0.447424, 0.48633, 0.51551, 0.54469, 0.583596, 0.622502, 0.661407,
        0.700313, 0.739219, 0.778124, 0.826757, 0.87539, 0.924023,
    ],
    dynamic_bonus: None,
};

const XIANYUN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.488784, 0.525443, 0.562101, 0.61098, 0.647639, 0.684297, 0.733176, 0.782054, 0.830933,
        0.879812, 0.92869, 0.977569, 1.038667, 1.099765, 1.160862,
    ],
    dynamic_bonus: None,
};

const XIANYUN_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.649464, 0.698174, 0.746884, 0.81183, 0.86054, 0.90925, 0.974196, 1.039142, 1.104088,
        1.169034, 1.23398, 1.298926, 1.380109, 1.461292, 1.542475,
    ],
    dynamic_bonus: None,
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
    dynamic_bonus: None,
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
    dynamic_bonus: None,
};

const XIANYUN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
    dynamic_bonus: None,
};

const XIANYUN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
    dynamic_bonus: None,
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
    dynamic_bonus: None,
};

const XIANYUN_SKILL_CLOUD_1: TalentScaling = TalentScaling {
    name: "流雲波1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.16, 1.247, 1.334, 1.45, 1.537, 1.624, 1.74, 1.856, 1.972, 2.088, 2.204, 2.32, 2.465,
        2.61, 2.755,
    ],
    dynamic_bonus: None,
};

const XIANYUN_SKILL_CLOUD_2: TalentScaling = TalentScaling {
    name: "流雲波2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.48, 1.591, 1.702, 1.85, 1.961, 2.072, 2.22, 2.368, 2.516, 2.664, 2.812, 2.96, 3.145,
        3.33, 3.515,
    ],
    dynamic_bonus: None,
};

const XIANYUN_SKILL_CLOUD_3: TalentScaling = TalentScaling {
    name: "流雲波3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        3.376, 3.6292, 3.8824, 4.22, 4.4732, 4.7264, 5.064, 5.4016, 5.7392, 6.0768, 6.4144, 6.752,
        7.174, 7.596, 8.018,
    ],
    dynamic_bonus: None,
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
    dynamic_bonus: None,
};

const XIANYUN_BURST_STARWICKER: TalentScaling = TalentScaling {
    name: "星枝ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.392, 0.4214, 0.4508, 0.49, 0.5194, 0.5488, 0.588, 0.6272, 0.6664, 0.7056, 0.7448, 0.784,
        0.8330, 0.882, 0.931,
    ],
    dynamic_bonus: None,
};

pub const XIANYUN: CharacterData = CharacterData {
    id: "xianyun",
    name: "Xianyun",
    element: Element::Anemo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [
        810.00, 9184.00, 9184.00, 9430.50, 9430.50, 9553.75, 9553.75, 9512.67, 9512.67, 10043.00,
        10043.00, 9677.00, 9677.00, 10409.00, 10409.00, 10825.36, // Lv95/Lv95+/Lv100
        10825.36, // Lv95/Lv95+/Lv100
        11241.72, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        26.00, 295.00, 295.00, 303.00, 303.00, 307.00, 307.00, 305.67, 305.67, 323.00, 323.00,
        311.00, 311.00, 335.00, 335.00, 348.40, // Lv95/Lv95+/Lv100
        348.40, // Lv95/Lv95+/Lv100
        361.80, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        45.00, 506.00, 506.00, 519.50, 519.50, 526.25, 526.25, 524.00, 524.00, 553.00, 553.00,
        533.00, 533.00, 573.00, 573.00, 595.92, // Lv95/Lv95+/Lv100
        595.92, // Lv95/Lv95+/Lv100
        618.84, // Lv95/Lv95+/Lv100
    ],
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
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Xiao — 5★ Anemo Polearm (Liyue)
// Source: genshin-db-api
// Normal Attack: Whirlwind Thrust (旋風キック)
// Elemental Skill: Lemniscatic Wind Cycling (風輪両立)
// Elemental Burst: Bane of All Evil (靖妖儺舞)
