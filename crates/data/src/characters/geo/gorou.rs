use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Gorou
// =============================================================================

// -- Normal Attack: 踏鞴ケ射法 (Ripping Fang Fletching) -- Physical --

const GOROU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3777, 0.4085, 0.4393, 0.4832, 0.5140, 0.5491, 0.5975, 0.6458, 0.6942, 0.7469, 0.7996,
        0.8523, 0.9049, 0.9576, 1.0103,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GOROU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3715, 0.4018, 0.4320, 0.4752, 0.5054, 0.5400, 0.5875, 0.6350, 0.6826, 0.7344, 0.7862,
        0.8381, 0.8899, 0.9418, 0.9936,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GOROU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4945, 0.5348, 0.5750, 0.6325, 0.6728, 0.7188, 0.7820, 0.8453, 0.9085, 0.9775, 1.0465,
        1.1155, 1.1845, 1.2535, 1.3225,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GOROU_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5900, 0.6382, 0.6863, 0.7549, 0.8031, 0.8579, 0.9333, 1.0088, 1.0842, 1.1667, 1.2491,
        1.3316, 1.4140, 1.4965, 1.5789,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Aimed Shot -- Geo (charged) --

const GOROU_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GOROU_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const GOROU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GOROU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GOROU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 犬坂ケ方陣 (Inuzaka All-Round Defense) -- Geo --

const GOROU_SKILL_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.0720, 1.1524, 1.2328, 1.3400, 1.4204, 1.5008, 1.6080, 1.7152, 1.8224, 1.9296, 2.0368,
        2.1440, 2.2780, 2.4120, 2.5460,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 獣牙キ突撃 (Juuga: Forward Unto Victory) -- Geo --

const GOROU_BURST_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.9822, 1.0559, 1.1296, 1.2278, 1.3014, 1.3751, 1.4733, 1.5716, 1.6698, 1.7680, 1.8663,
        1.9645, 2.0873, 2.2101, 2.3329,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GOROU_BURST_CRYSTAL: TalentScaling = TalentScaling {
    name: "岩晶崩破ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.6132, 0.6592, 0.7052, 0.7665, 0.8126, 0.8586, 0.9199, 0.9812, 1.0425, 1.1038, 1.1651,
        1.2264, 1.3031, 1.3797, 1.4563,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const GOROU: CharacterData = CharacterData {
    id: "gorou",
    name: "Gorou",
    element: Element::Geo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Inazuma,
    base_hp: [
        802.00, 2061.00, 2661.00, 3985.00, 4411.00, 5074.00, 5642.00, 6305.00, 6731.00, 7393.00,
        7819.00, 8481.00, 8907.00, 9570.00, 9570.00, 9952.80,  // Lv95/Lv95+/Lv100
        9952.80,  // Lv95/Lv95+/Lv100
        10335.60, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        15.31, 39.34, 50.79, 76.07, 84.20, 96.85, 107.69, 120.34, 128.47, 141.10, 149.23, 161.88,
        170.01, 182.66, 182.66, 189.97, // Lv95/Lv95+/Lv100
        189.97, // Lv95/Lv95+/Lv100
        197.27, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        54.36, 139.66, 180.27, 270.03, 298.88, 343.79, 382.26, 427.17, 456.02, 500.87, 529.73,
        574.63, 603.49, 648.40, 648.40, 674.34, // Lv95/Lv95+/Lv100
        674.34, // Lv95/Lv95+/Lv100
        700.27, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Geo, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "踏鞴ケ射法",
            hits: &[
                GOROU_NORMAL_1,
                GOROU_NORMAL_2,
                GOROU_NORMAL_3,
                GOROU_NORMAL_4,
            ],
            charged: &[GOROU_AIMED, GOROU_AIMED_FULL],
            plunging: &[GOROU_PLUNGE, GOROU_PLUNGE_LOW, GOROU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "犬坂ケ方陣",
            scalings: &[GOROU_SKILL_DAMAGE],
        },
        elemental_burst: TalentData {
            name: "獣牙キ突撃",
            scalings: &[GOROU_BURST_DAMAGE, GOROU_BURST_CRYSTAL],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
    scaling_modifiers: &[],
};
