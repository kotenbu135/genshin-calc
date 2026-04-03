use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    base_hp: [
        820.00, 2127.00, 2830.00, 4234.00, 4734.00, 5446.00, 6112.00, 6832.00, 7331.00, 8058.00,
        8557.00, 9292.00, 9791.00, 10531.00, 10531.00, 10952.24, // Lv95/Lv95+/Lv100
        10952.24, // Lv95/Lv95+/Lv100
        11373.48, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        20.48, 53.13, 70.69, 105.78, 118.25, 136.05, 152.69, 170.67, 183.15, 201.30, 213.77,
        232.12, 244.60, 263.10, 263.10, 273.62, // Lv95/Lv95+/Lv100
        273.62, // Lv95/Lv95+/Lv100
        284.15, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        52.05, 135.02, 179.65, 268.82, 300.53, 345.76, 388.05, 433.75, 465.46, 511.58, 543.29,
        589.93, 621.64, 668.64, 668.64, 695.39, // Lv95/Lv95+/Lv100
        695.39, // Lv95/Lv95+/Lv100
        722.13, // Lv95/Lv95+/Lv100
    ],
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
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Wanderer — 5★ Anemo Catalyst (Sumeru)
// Source: genshin-db-api
// Normal Attack: Yuuban Meigen (夕番銘詠)
// Elemental Skill: Hanega: Song of the Wind (羽化・風の歌)
// Elemental Burst: Kyougen: Five Ceremonial Plays (狂言・五番の演目)
