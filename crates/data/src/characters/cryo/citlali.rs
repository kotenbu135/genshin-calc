use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Shadow-Stealing Spirit Vessel -- Cryo (Catalyst) --

const CITLALI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.4341, 0.4666, 0.4992, 0.5426, 0.5751, 0.6077, 0.6511, 0.6945, 0.7379, 0.7813, 0.8247,
        0.8681, 0.9224, 0.9767, 1.0309,
    ],
    dynamic_bonus: None,
};

const CITLALI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.3881, 0.4172, 0.4464, 0.4852, 0.5143, 0.5434, 0.5822, 0.6210, 0.6598, 0.6986, 0.7375,
        0.7763, 0.8248, 0.8733, 0.9218,
    ],
    dynamic_bonus: None,
};

const CITLALI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5377, 0.5780, 0.6184, 0.6721, 0.7125, 0.7528, 0.8066, 0.8603, 0.9141, 0.9679, 1.0217,
        1.0754, 1.1426, 1.2099, 1.2771,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Cryo (Catalyst) --

const CITLALI_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.9920, 1.0664, 1.1408, 1.2400, 1.3144, 1.3888, 1.4880, 1.5872, 1.6864, 1.7856, 1.8848,
        1.9840, 2.1080, 2.2320, 2.3560,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Cryo (Catalyst) --

const CITLALI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
};

const CITLALI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
};

const CITLALI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: Dawnfrost Darkstar -- Cryo --

const CITLALI_SKILL_TZITZIMITL: TalentScaling = TalentScaling {
    name: "黒曜ツィツィミトルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.7296, 0.7843, 0.8390, 0.9120, 0.9667, 1.0214, 1.0944, 1.1674, 1.2403, 1.3133, 1.3862,
        1.4592, 1.5504, 1.6416, 1.7328,
    ],
    dynamic_bonus: None,
};

const CITLALI_SKILL_STORM: TalentScaling = TalentScaling {
    name: "霜降嵐ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.1703, 0.1830, 0.1958, 0.2128, 0.2256, 0.2383, 0.2554, 0.2724, 0.2894, 0.3064, 0.3235,
        0.3405, 0.3618, 0.3830, 0.4043,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: Edict of Entwined Splendor -- Cryo --

const CITLALI_BURST_ICE_STORM: TalentScaling = TalentScaling {
    name: "氷嵐ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        5.3760, 5.7792, 6.1824, 6.7200, 7.1232, 7.5264, 8.0640, 8.6016, 9.1392, 9.6768, 10.2144,
        10.7520, 11.4240, 12.0960, 12.7680,
    ],
    dynamic_bonus: None,
};

const CITLALI_BURST_SKULL: TalentScaling = TalentScaling {
    name: "霊脈頭蓋ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.3440, 1.4448, 1.5456, 1.6800, 1.7808, 1.8816, 2.0160, 2.1504, 2.2848, 2.4192, 2.5536,
        2.6880, 2.8560, 3.0240, 3.1920,
    ],
    dynamic_bonus: None,
};

pub const CITLALI: CharacterData = CharacterData {
    id: "citlali",
    name: "Citlali",
    element: Element::Cryo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [
        906.00, 2349.00, 3126.00, 4677.00, 5229.00, 6016.00, 6752.00, 7547.00, 8098.00, 8901.00,
        9453.00, 10264.00, 10816.00, 11634.00, 11634.00, 12099.36, // Lv95/Lv95+/Lv100
        12099.36, // Lv95/Lv95+/Lv100
        12564.72, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        9.87, 25.60, 34.06, 50.96, 56.98, 65.55, 73.57, 82.23, 88.24, 96.99, 103.00, 111.84,
        117.85, 126.76, 126.76, 131.83, // Lv95/Lv95+/Lv100
        131.83, // Lv95/Lv95+/Lv100
        136.90, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        59.41, 154.11, 205.05, 306.82, 343.02, 394.65, 442.91, 495.07, 531.27, 583.90, 620.10,
        673.33, 709.53, 763.17, 763.17, 793.70, // Lv95/Lv95+/Lv100
        793.70, // Lv95/Lv95+/Lv100
        824.22, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalMastery(115.2),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "影盗みの霊器",
            hits: &[CITLALI_NORMAL_1, CITLALI_NORMAL_2, CITLALI_NORMAL_3],
            charged: &[CITLALI_CHARGED],
            plunging: &[CITLALI_PLUNGE, CITLALI_PLUNGE_LOW, CITLALI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "夜明霜暗星",
            scalings: &[CITLALI_SKILL_TZITZIMITL, CITLALI_SKILL_STORM],
        },
        elemental_burst: TalentData {
            name: "絢爛交織の勅令",
            scalings: &[CITLALI_BURST_ICE_STORM, CITLALI_BURST_SKULL],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Diona
