use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Rapid Fire -- Physical --

const ALOY_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2112, 0.2256, 0.2400, 0.2592, 0.2736, 0.2904, 0.3120, 0.3336, 0.3552, 0.3768, 0.3984,
        0.4200, 0.4416, 0.4632, 0.4848,
    ],
    dynamic_bonus: None,
};

const ALOY_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2376, 0.2538, 0.2700, 0.2916, 0.3078, 0.3267, 0.3510, 0.3753, 0.3996, 0.4239, 0.4482,
        0.4725, 0.4968, 0.5211, 0.5454,
    ],
    dynamic_bonus: None,
};

const ALOY_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4312, 0.4606, 0.4900, 0.5292, 0.5586, 0.5929, 0.6370, 0.6811, 0.7252, 0.7693, 0.8134,
        0.8575, 0.9016, 0.9457, 0.9898,
    ],
    dynamic_bonus: None,
};

const ALOY_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5280, 0.5640, 0.6000, 0.6480, 0.6840, 0.7260, 0.7800, 0.8340, 0.8880, 0.9420, 0.9960,
        1.0500, 1.1040, 1.1580, 1.2120,
    ],
    dynamic_bonus: None,
};

// -- Aimed Shot -- Cryo (charged) --

const ALOY_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
    dynamic_bonus: None,
};

const ALOY_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const ALOY_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
};

const ALOY_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
};

const ALOY_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: Frozen Wilds -- Cryo --

const ALOY_SKILL_BOMB: TalentScaling = TalentScaling {
    name: "凍結爆弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.7760, 1.9092, 2.0424, 2.2200, 2.3532, 2.4864, 2.6640, 2.8416, 3.0192, 3.1968, 3.3744,
        3.5520, 3.7740, 3.9960, 4.2180,
    ],
    dynamic_bonus: None,
};

const ALOY_SKILL_BOMBLET: TalentScaling = TalentScaling {
    name: "チルウォーター爆弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.4000, 0.4300, 0.4600, 0.5000, 0.5300, 0.5600, 0.6000, 0.6400, 0.6800, 0.7200, 0.7600,
        0.8000, 0.8500, 0.9000, 0.9500,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: Prophecies of Dawn -- Cryo --

const ALOY_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        3.5920, 3.8614, 4.1308, 4.4900, 4.7594, 5.0288, 5.3880, 5.7472, 6.1064, 6.4656, 6.8248,
        7.1840, 7.6330, 8.0820, 8.5310,
    ],
    dynamic_bonus: None,
};

pub const ALOY: CharacterData = CharacterData {
    id: "aloy",
    name: "Aloy",
    element: Element::Cryo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Other,
    base_hp: [
        848.00, 2201.00, 2928.00, 4382.00, 4899.00, 5636.00, 6325.00, 7070.00, 7587.00, 8339.00,
        8856.00, 9616.00, 10133.00, 10899.00, 10899.00, 11334.96, // Lv95/Lv95+/Lv100
        11334.96, // Lv95/Lv95+/Lv100
        11770.92, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        18.21, 47.24, 62.85, 94.04, 105.14, 120.96, 135.75, 151.74, 162.84, 178.97, 190.06, 206.38,
        217.47, 233.92, 233.92, 243.28, // Lv95/Lv95+/Lv100
        243.28, // Lv95/Lv95+/Lv100
        252.63, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        52.65, 136.58, 181.72, 271.91, 303.99, 349.74, 392.51, 438.73, 470.81, 517.46, 549.53,
        596.71, 628.78, 676.33, 676.33, 703.38, // Lv95/Lv95+/Lv100
        703.38, // Lv95/Lv95+/Lv100
        730.44, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Cryo, 0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "ラピッドファイア",
            hits: &[ALOY_NORMAL_1, ALOY_NORMAL_2, ALOY_NORMAL_3, ALOY_NORMAL_4],
            charged: &[ALOY_AIMED, ALOY_AIMED_FULL],
            plunging: &[ALOY_PLUNGE, ALOY_PLUNGE_LOW, ALOY_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "凍てつくワイルド",
            scalings: &[ALOY_SKILL_BOMB, ALOY_SKILL_BOMBLET],
        },
        elemental_burst: TalentData {
            name: "夜明けの予言",
            scalings: &[ALOY_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Ayaka (Kamisato Ayaka)
