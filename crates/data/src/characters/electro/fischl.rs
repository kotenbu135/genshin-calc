use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Fischl
// =============================================================================

// -- Normal Attack: 罪滅ぼしの矢 (Bolts of Downfall) -- Physical --

const FISCHL_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4412, 0.4771, 0.5130, 0.5643, 0.6002, 0.6413, 0.6976, 0.7540, 0.8104, 0.8721, 0.9337,
        0.9954, 1.0568, 1.1183, 1.1799,
    ],
    dynamic_bonus: None,
};

const FISCHL_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4678, 0.5058, 0.5438, 0.5982, 0.6362, 0.6798, 0.7397, 0.7996, 0.8594, 0.9246, 0.9898,
        1.0550, 1.1206, 1.1862, 1.2514,
    ],
    dynamic_bonus: None,
};

const FISCHL_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5814, 0.6289, 0.6764, 0.7440, 0.7916, 0.8455, 0.9198, 0.9942, 1.0685, 1.1498, 1.2310,
        1.3122, 1.3934, 1.4747, 1.5559,
    ],
    dynamic_bonus: None,
};

const FISCHL_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5771, 0.6242, 0.6712, 0.7383, 0.7854, 0.8390, 0.9127, 0.9864, 1.0601, 1.1409, 1.2216,
        1.3024, 1.3831, 1.4639, 1.5446,
    ],
    dynamic_bonus: None,
};

const FISCHL_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7207, 0.7794, 0.8382, 0.9220, 0.9808, 1.0478, 1.1398, 1.2319, 1.3239, 1.4248, 1.5257,
        1.6266, 1.7275, 1.8284, 1.9293,
    ],
    dynamic_bonus: None,
};

// -- Aimed Shot -- Electro (charged) --

const FISCHL_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
    dynamic_bonus: None,
};

const FISCHL_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const FISCHL_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
};

const FISCHL_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
};

const FISCHL_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: 夜巡りの翼 (Nightrider) -- Electro --

const FISCHL_SKILL_OZ: TalentScaling = TalentScaling {
    name: "オズ攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.8880, 0.9546, 1.0212, 1.1100, 1.1766, 1.2432, 1.3320, 1.4208, 1.5096, 1.5984, 1.6872,
        1.7760, 1.8870, 1.9980, 2.1090,
    ],
    dynamic_bonus: None,
};

const FISCHL_SKILL_SUMMON: TalentScaling = TalentScaling {
    name: "召喚ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.1544, 1.2410, 1.3276, 1.4430, 1.5296, 1.6162, 1.7316, 1.8470, 1.9625, 2.0779, 2.1934,
        2.3088, 2.4531, 2.5974, 2.7417,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 夜の幻現 (Midnight Phantasmagoria) -- Electro --

const FISCHL_BURST: TalentScaling = TalentScaling {
    name: "落雷ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        2.0800, 2.2360, 2.3920, 2.6000, 2.7560, 2.9120, 3.1200, 3.3280, 3.5360, 3.7440, 3.9520,
        4.1600, 4.4200, 4.6800, 4.9400,
    ],
    dynamic_bonus: None,
};

pub const FISCHL: CharacterData = CharacterData {
    id: "fischl",
    name: "Fischl",
    element: Element::Electro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [
        770.00, 1979.00, 2555.00, 3827.00, 4236.00, 4872.00, 5418.00, 6054.00, 6463.00, 7099.00,
        7508.00, 8144.00, 8553.00, 9189.00, 9189.00, 9556.56, // Lv95/Lv95+/Lv100
        9556.56, // Lv95/Lv95+/Lv100
        9924.12, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        20.48, 52.61, 67.91, 101.72, 112.59, 129.51, 144.00, 160.92, 171.79, 188.68, 199.55,
        216.47, 227.34, 244.26, 244.26, 254.03, // Lv95/Lv95+/Lv100
        254.03, // Lv95/Lv95+/Lv100
        263.80, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        49.79, 127.90, 165.09, 247.29, 273.71, 314.84, 350.07, 391.20, 417.62, 458.70, 485.12,
        526.24, 552.67, 593.79, 593.79, 617.54, // Lv95/Lv95+/Lv100
        617.54, // Lv95/Lv95+/Lv100
        641.29, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "罪滅ぼしの矢",
            hits: &[
                FISCHL_NORMAL_1,
                FISCHL_NORMAL_2,
                FISCHL_NORMAL_3,
                FISCHL_NORMAL_4,
                FISCHL_NORMAL_5,
            ],
            charged: &[FISCHL_AIMED, FISCHL_AIMED_FULL],
            plunging: &[FISCHL_PLUNGE, FISCHL_PLUNGE_LOW, FISCHL_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "夜巡りの翼",
            scalings: &[FISCHL_SKILL_OZ, FISCHL_SKILL_SUMMON],
        },
        elemental_burst: TalentData {
            name: "夜の幻現",
            scalings: &[FISCHL_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
