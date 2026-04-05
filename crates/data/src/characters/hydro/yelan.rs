use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 匿影隠曜の弓 (Stealthy Bowshot) -- Physical (Bow) --

const YELAN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4068, 0.4399, 0.4730, 0.5203, 0.5534, 0.5913, 0.6433, 0.6953, 0.7473, 0.8041, 0.8609,
        0.9176, 0.9744, 1.0310, 1.0879,
    ],
    dynamic_bonus: None,
};

const YELAN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3904, 0.4222, 0.4540, 0.4994, 0.5312, 0.5675, 0.6174, 0.6674, 0.7173, 0.7718, 0.8263,
        0.8808, 0.9352, 0.9897, 1.0442,
    ],
    dynamic_bonus: None,
};

const YELAN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5160, 0.5580, 0.6000, 0.6600, 0.7020, 0.7500, 0.8160, 0.8820, 0.9480, 1.0200, 1.0920,
        1.1640, 1.2360, 1.3080, 1.3800,
    ],
    dynamic_bonus: None,
};

const YELAN_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ(×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3251, 0.3515, 0.3780, 0.4158, 0.4423, 0.4725, 0.5141, 0.5557, 0.5972, 0.6426, 0.6880,
        0.7333, 0.7787, 0.8240, 0.8694,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Bow (HP scaling for Breakthrough Barb) --

const YELAN_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
    dynamic_bonus: None,
};

const YELAN_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
    dynamic_bonus: None,
};

const YELAN_BREAKTHROUGH: TalentScaling = TalentScaling {
    name: "破局の矢ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1158, 0.1244, 0.1331, 0.1447, 0.1534, 0.1621, 0.1736, 0.1852, 0.1968, 0.2084, 0.2199,
        0.2315, 0.2460, 0.2605, 0.2749,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const YELAN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
};

const YELAN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
};

const YELAN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: 絡み合う命の糸 (Lingering Lifeline) -- Hydro (HP scaling) --

const YELAN_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.2261, 0.2431, 0.2601, 0.2827, 0.2996, 0.3166, 0.3392, 0.3618, 0.3844, 0.4070, 0.4297,
        0.4523, 0.4805, 0.5088, 0.5371,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 深謀玲瓏賽 (Depth-Clarion Dice) -- Hydro (HP scaling) --

const YELAN_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0731, 0.0786, 0.0840, 0.0914, 0.0968, 0.1023, 0.1096, 0.1169, 0.1242, 0.1315, 0.1389,
        0.1462, 0.1553, 0.1644, 0.1736,
    ],
    dynamic_bonus: None,
};

const YELAN_BURST_EXQUISITE_THROW: TalentScaling = TalentScaling {
    name: "玲瓏一擲ダメージ(×3)",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0487, 0.0524, 0.0560, 0.0609, 0.0646, 0.0682, 0.0731, 0.0780, 0.0828, 0.0877, 0.0926,
        0.0974, 0.1035, 0.1096, 0.1157,
    ],
    dynamic_bonus: None,
};

pub const YELAN: CharacterData = CharacterData {
    id: "yelan",
    name: "Yelan",
    element: Element::Hydro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [
        1125.00, 2918.00, 3883.00, 5810.00, 6495.00, 7472.00, 8386.00, 9374.00, 10059.00, 11056.00,
        11741.00, 12749.00, 13434.00, 14450.00, 14450.00, 15028.00, // Lv95/Lv95+/Lv100
        15028.00, // Lv95/Lv95+/Lv100
        15606.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        18.99, 49.27, 65.55, 98.08, 109.65, 126.16, 141.58, 158.26, 169.83, 186.66, 198.23, 215.24,
        226.81, 243.96, 243.96, 253.72, // Lv95/Lv95+/Lv100
        253.72, // Lv95/Lv95+/Lv100
        263.48, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        42.66, 110.66, 147.23, 220.31, 246.30, 283.37, 318.02, 355.47, 381.46, 419.26, 445.25,
        483.47, 509.46, 547.98, 547.98, 569.90, // Lv95/Lv95+/Lv100
        569.90, // Lv95/Lv95+/Lv100
        591.82, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "匿影隠曜の弓",
            hits: &[
                YELAN_NORMAL_1,
                YELAN_NORMAL_2,
                YELAN_NORMAL_3,
                YELAN_NORMAL_4,
            ],
            charged: &[YELAN_AIMED, YELAN_AIMED_FULL, YELAN_BREAKTHROUGH],
            plunging: &[YELAN_PLUNGE, YELAN_PLUNGE_LOW, YELAN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "絡み合う命の糸",
            scalings: &[YELAN_SKILL],
        },
        elemental_burst: TalentData {
            name: "深謀玲瓏賽",
            scalings: &[YELAN_BURST, YELAN_BURST_EXQUISITE_THROW],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
