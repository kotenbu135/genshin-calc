use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    base_hp: [
        1144.00, 12965.00, 12965.00, 13313.50, 13313.50, 13487.75, 13487.75, 13429.67, 13429.67,
        14178.50, 14178.50, 13662.00, 13662.00, 14695.00, 14695.00,
        15282.80, // Lv95/Lv95+/Lv100
        15282.80, // Lv95/Lv95+/Lv100
        15870.60, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        19.00, 211.00, 211.00, 216.50, 216.50, 219.25, 219.25, 218.33, 218.33, 230.50, 230.50,
        222.00, 222.00, 239.00, 239.00, 248.56, // Lv95/Lv95+/Lv100
        248.56, // Lv95/Lv95+/Lv100
        258.12, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        60.00, 678.00, 678.00, 696.50, 696.50, 705.75, 705.75, 702.67, 702.67, 742.00, 742.00,
        715.00, 715.00, 769.00, 769.00, 799.76, // Lv95/Lv95+/Lv100
        799.76, // Lv95/Lv95+/Lv100
        830.52, // Lv95/Lv95+/Lv100
    ],
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
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Kazuha — 5★ Anemo Sword (Inazuma)
// Source: genshin-db-api
// Normal Attack: Garyuu Bladework (我流剣術)
// Elemental Skill: Chihayaburu (千早振る)
// Elemental Burst: Kazuha Slash (万葉の一刀)
