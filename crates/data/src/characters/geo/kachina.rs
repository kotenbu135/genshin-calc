use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Kachina
// =============================================================================

// -- Normal Attack: 石紡ぎの拳 (Cragbiter) -- Physical --

const KACHINA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4940, 0.5342, 0.5744, 0.6318, 0.6720, 0.7180, 0.7811, 0.8443, 0.9074, 0.9764, 1.0454,
        1.1143, 1.1833, 1.2522, 1.3212,
    ],
    dynamic_bonus: None,
};

const KACHINA_NORMAL_2_1: TalentScaling = TalentScaling {
    name: "2段ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2757, 0.2981, 0.3206, 0.3526, 0.3751, 0.4007, 0.4360, 0.4712, 0.5065, 0.5450, 0.5834,
        0.6219, 0.6604, 0.6988, 0.7373,
    ],
    dynamic_bonus: None,
};

const KACHINA_NORMAL_2_2: TalentScaling = TalentScaling {
    name: "2段ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3063, 0.3313, 0.3562, 0.3918, 0.4167, 0.4452, 0.4844, 0.5236, 0.5628, 0.6055, 0.6483,
        0.6910, 0.7338, 0.7765, 0.8192,
    ],
    dynamic_bonus: None,
};

const KACHINA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7043, 0.7616, 0.8189, 0.9008, 0.9581, 1.0237, 1.1137, 1.2038, 1.2939, 1.3922, 1.4904,
        1.5887, 1.6870, 1.7852, 1.8835,
    ],
    dynamic_bonus: None,
};

const KACHINA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7744, 0.8374, 0.9004, 0.9905, 1.0535, 1.1255, 1.2246, 1.3236, 1.4227, 1.5307, 1.6388,
        1.7468, 1.8549, 1.9629, 2.0710,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Physical --

const KACHINA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1266, 1.2186, 1.3107, 1.4417, 1.5338, 1.6384, 1.7829, 1.9275, 2.0721, 2.2287, 2.3852,
        2.5417, 2.6983, 2.8548, 3.0114,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const KACHINA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
};

const KACHINA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5271, 2.7055,
        2.8840, 3.0624, 3.2409, 3.4193,
    ],
    dynamic_bonus: None,
};

const KACHINA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6021, 3.8249, 4.0478, 4.2706,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: ゴゴキでゴー！(Go, Go Turbo Twirly!) -- Geo --

const KACHINA_SKILL_TURBO: TalentScaling = TalentScaling {
    name: "ゴゴキ回転ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.8776, 0.9434, 1.0092, 1.0970, 1.1628, 1.2286, 1.3164, 1.4042, 1.4919, 1.5797, 1.6674,
        1.7552, 1.8649, 1.9746, 2.0843,
    ],
    dynamic_bonus: None,
};

const KACHINA_SKILL_DASH: TalentScaling = TalentScaling {
    name: "ゴゴキ突進ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.6376, 0.6854, 0.7332, 0.7970, 0.8448, 0.8926, 0.9564, 1.0202, 1.0839, 1.1477, 1.2114,
        1.2752, 1.3549, 1.4346, 1.5143,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 蝕夜キ絶門（Time to Get Serious!） -- Geo --

const KACHINA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        3.8057, 4.1366, 4.4252, 4.8100, 5.0986, 5.3872, 5.7720, 6.1568, 6.5416, 6.9264, 7.3112,
        7.6960, 8.1770, 8.6580, 9.1390,
    ],
    dynamic_bonus: None,
};

pub const KACHINA: CharacterData = CharacterData {
    id: "kachina",
    name: "Kachina",
    element: Element::Geo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Natlan,
    base_hp: [
        989.00, 2541.00, 3281.00, 4914.00, 5439.00, 6256.00, 6956.00, 7773.00, 8299.00, 9115.00,
        9640.00, 10457.00, 10982.00, 11799.00, 11799.00, 12270.96, // Lv95/Lv95+/Lv100
        12270.96, // Lv95/Lv95+/Lv100
        12742.92, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        18.16, 46.66, 60.23, 90.22, 99.86, 114.87, 127.72, 142.73, 152.37, 167.35, 177.00, 192.00,
        201.64, 216.65, 216.65, 225.32, // Lv95/Lv95+/Lv100
        225.32, // Lv95/Lv95+/Lv100
        233.98, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        66.44, 170.68, 220.31, 330.00, 365.27, 420.15, 467.17, 522.04, 557.31, 612.12, 647.39,
        702.26, 737.53, 792.41, 792.41, 824.11, // Lv95/Lv95+/Lv100
        824.11, // Lv95/Lv95+/Lv100
        855.80, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Geo, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "石紡ぎの拳",
            hits: &[
                KACHINA_NORMAL_1,
                KACHINA_NORMAL_2_1,
                KACHINA_NORMAL_2_2,
                KACHINA_NORMAL_3,
                KACHINA_NORMAL_4,
            ],
            charged: &[KACHINA_CHARGED],
            plunging: &[KACHINA_PLUNGE, KACHINA_PLUNGE_LOW, KACHINA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "ゴゴキでゴー！",
            scalings: &[KACHINA_SKILL_TURBO, KACHINA_SKILL_DASH],
        },
        elemental_burst: TalentData {
            name: "蝕夜キ絶門",
            scalings: &[KACHINA_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
