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

const KACHINA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4466, 0.4829, 0.5193, 0.5712, 0.6076, 0.6491, 0.7064, 0.7636, 0.8208, 0.8830, 0.9451,
        1.0073, 1.0694, 1.1316, 1.1937,
    ],
    dynamic_bonus: None,
};

const KACHINA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6505, 0.7034, 0.7563, 0.8319, 0.8848, 0.9454, 1.0285, 1.1117, 1.1948, 1.2857, 1.3765,
        1.4674, 1.5582, 1.6491, 1.7399,
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
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 0.1011, 1.0928, 1.1746, 1.2638, 1.3530,
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
        0.8768, 0.9426, 1.0083, 1.0960, 1.1618, 1.2275, 1.3152, 1.4029, 1.4906, 1.5782, 1.6659,
        1.7536, 1.8632, 1.9728, 2.0824,
    ],
    dynamic_bonus: None,
};

const KACHINA_SKILL_DASH: TalentScaling = TalentScaling {
    name: "ゴゴキ突進ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.6400, 0.6880, 0.7360, 0.8000, 0.8480, 0.8960, 0.9600, 1.0240, 1.0880, 1.1520, 1.2160,
        1.2800, 1.3600, 1.4400, 1.5200,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 蝕夜キ絶門（Time to Get Serious!） -- Geo --

const KACHINA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        3.8076, 4.0932, 4.3788, 4.7595, 5.0451, 5.3307, 5.7114, 6.0922, 6.4729, 6.8537, 7.2344,
        7.6152, 8.0912, 8.5671, 9.0431,
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
            hits: &[KACHINA_NORMAL_1, KACHINA_NORMAL_2, KACHINA_NORMAL_3],
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
