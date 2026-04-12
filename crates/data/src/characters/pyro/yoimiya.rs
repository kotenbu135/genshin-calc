use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Yoimiya
// =============================================================================

// -- Normal Attack: 打ち上げ花火 (Firework Flare-Up) -- Physical --

const YOIMIYA_NORMAL_1A: TalentScaling = TalentScaling {
    name: "1段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3564, 0.3807, 0.4050, 0.4374, 0.4617, 0.4901, 0.5265, 0.5630, 0.5994, 0.6359, 0.6723,
        0.7088, 0.7452, 0.7817, 0.8181,
    ],
    dynamic_bonus: None,
};

const YOIMIYA_NORMAL_1B: TalentScaling = TalentScaling {
    name: "1段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3564, 0.3807, 0.4050, 0.4374, 0.4617, 0.4901, 0.5265, 0.5630, 0.5994, 0.6359, 0.6723,
        0.7088, 0.7452, 0.7817, 0.8181,
    ],
    dynamic_bonus: None,
};

const YOIMIYA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6838, 0.7304, 0.7770, 0.8392, 0.8858, 0.9402, 1.0101, 1.0800, 1.1500, 1.2199, 1.2898,
        1.3598, 1.4297, 1.4996, 1.5695,
    ],
    dynamic_bonus: None,
};

const YOIMIYA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8889, 0.9495, 1.0101, 1.0909, 1.1515, 1.2222, 1.3131, 1.4040, 1.4949, 1.5859, 1.6768,
        1.7677, 1.8586, 1.9495, 2.0404,
    ],
    dynamic_bonus: None,
};

const YOIMIYA_NORMAL_4A: TalentScaling = TalentScaling {
    name: "4段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4642, 0.4959, 0.5275, 0.5697, 0.6014, 0.6383, 0.6858, 0.7332, 0.7807, 0.8282, 0.8757,
        0.9231, 0.9706, 1.0181, 1.0656,
    ],
    dynamic_bonus: None,
};

const YOIMIYA_NORMAL_4B: TalentScaling = TalentScaling {
    name: "4段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4642, 0.4959, 0.5275, 0.5697, 0.6014, 0.6383, 0.6858, 0.7332, 0.7807, 0.8282, 0.8757,
        0.9231, 0.9706, 1.0181, 1.0656,
    ],
    dynamic_bonus: None,
};

const YOIMIYA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0586, 1.1308, 1.2030, 1.2992, 1.3714, 1.4556, 1.5639, 1.6722, 1.7804, 1.8887, 1.9970,
        2.1053, 2.2135, 2.3218, 2.4301,
    ],
    dynamic_bonus: None,
};

// -- Aimed Shot -- Pyro (charged) --

const YOIMIYA_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
    dynamic_bonus: None,
};

const YOIMIYA_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
    dynamic_bonus: None,
};

const YOIMIYA_AIMED_KINDLING: TalentScaling = TalentScaling {
    name: "焔硝の矢ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.1640, 0.1763, 0.1886, 0.2050, 0.2173, 0.2296, 0.2460, 0.2624, 0.2788, 0.2952, 0.3116,
        0.3280, 0.3485, 0.3690, 0.3895,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const YOIMIYA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
};

const YOIMIYA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
};

const YOIMIYA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: 庭火焔硝 (Niwabi Fire-Dance) -- Pyro --
// Note: Skill infuses normal attacks with Pyro. The damage bonus multiplier
// is stored here as a scaling reference. Callers handle the infusion mechanic.

const YOIMIYA_SKILL_DMG: TalentScaling = TalentScaling {
    name: "炎硝矢ダメージ増加",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.3791, 0.4018, 0.4245, 0.4540, 0.4767, 0.4994, 0.5289, 0.5584, 0.5879, 0.6174, 0.6470,
        0.6765, 0.7060, 0.7355, 0.7650,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 琉金雲間草 (Ryuukin Saxifrage) -- Pyro --

const YOIMIYA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2720, 1.3674, 1.4628, 1.5900, 1.6854, 1.7808, 1.9080, 2.0352, 2.1624, 2.2896, 2.4168,
        2.5440, 2.7030, 2.8620, 3.0210,
    ],
    dynamic_bonus: None,
};

const YOIMIYA_BURST_EXPLOSION: TalentScaling = TalentScaling {
    name: "琉金の炎爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2200, 1.3115, 1.4030, 1.5250, 1.6165, 1.7080, 1.8300, 1.9520, 2.0740, 2.1960, 2.3180,
        2.4400, 2.5925, 2.7450, 2.8975,
    ],
    dynamic_bonus: None,
};

pub const YOIMIYA: CharacterData = CharacterData {
    id: "yoimiya",
    name: "Yoimiya",
    element: Element::Pyro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [
        791.00, 2053.00, 2731.00, 4086.00, 4568.00, 5256.00, 5899.00, 6593.00, 7076.00, 7777.00,
        8259.00, 8968.00, 9450.00, 10164.00, 10164.00, 10570.56, // Lv95/Lv95+/Lv100
        10570.56, // Lv95/Lv95+/Lv100
        10977.12, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        25.14, 65.21, 86.76, 129.82, 145.13, 166.97, 187.39, 209.46, 224.77, 247.04, 262.36,
        284.88, 300.19, 322.89, 322.89, 335.81, // Lv95/Lv95+/Lv100
        335.81, // Lv95/Lv95+/Lv100
        348.72, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        47.86, 124.16, 165.20, 247.19, 276.35, 317.94, 356.82, 398.85, 428.01, 470.42, 499.58,
        542.46, 571.62, 614.84, 614.84, 639.43, // Lv95/Lv95+/Lv100
        639.43, // Lv95/Lv95+/Lv100
        664.03, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "打ち上げ花火",
            hits: &[
                YOIMIYA_NORMAL_1A,
                YOIMIYA_NORMAL_1B,
                YOIMIYA_NORMAL_2,
                YOIMIYA_NORMAL_3,
                YOIMIYA_NORMAL_4A,
                YOIMIYA_NORMAL_4B,
                YOIMIYA_NORMAL_5,
            ],
            charged: &[YOIMIYA_AIMED, YOIMIYA_AIMED_FULL, YOIMIYA_AIMED_KINDLING],
            plunging: &[YOIMIYA_PLUNGE, YOIMIYA_PLUNGE_LOW, YOIMIYA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "庭火焔硝",
            scalings: &[YOIMIYA_SKILL_DMG],
        },
        elemental_burst: TalentData {
            name: "琉金雲間草",
            scalings: &[YOIMIYA_BURST, YOIMIYA_BURST_EXPLOSION],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
