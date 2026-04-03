use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 独舞の招待 (Soloist's Solicitation) -- Physical (Sword) --

const FURINA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4839, 0.5232, 0.5626, 0.6189, 0.6583, 0.7033, 0.7652, 0.8271, 0.8890, 0.9565, 1.0240,
        1.0915, 1.1590, 1.2265, 1.2940,
    ],
};

const FURINA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4373, 0.4729, 0.5085, 0.5593, 0.5949, 0.6356, 0.6915, 0.7475, 0.8034, 0.8644, 0.9254,
        0.9865, 1.0475, 1.1085, 1.1695,
    ],
};

const FURINA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5512, 0.5961, 0.6409, 0.7050, 0.7499, 0.8012, 0.8717, 0.9422, 1.0127, 1.0896, 1.1665,
        1.2434, 1.3203, 1.3972, 1.4741,
    ],
};

const FURINA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7330, 0.7926, 0.8523, 0.9375, 0.9972, 1.0654, 1.1591, 1.2529, 1.3466, 1.4489, 1.5512,
        1.6535, 1.7557, 1.8580, 1.9603,
    ],
};

// -- Charged Attack -- Physical (Sword) --

const FURINA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7422, 0.8026, 0.8630, 0.9493, 1.0097, 1.0788, 1.1737, 1.2686, 1.3635, 1.4671, 1.5707,
        1.6742, 1.7778, 1.8813, 1.9849,
    ],
};

// -- Plunging Attack -- Physical (Sword) --

const FURINA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const FURINA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const FURINA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: サロン・ソリティア (Salon Solitaire) -- Hydro --

const FURINA_SKILL_BUBBLE: TalentScaling = TalentScaling {
    name: "ウーシア泡沫ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0786, 0.0845, 0.0904, 0.0983, 0.1042, 0.1101, 0.1180, 0.1258, 0.1337, 0.1416, 0.1494,
        0.1573, 0.1671, 0.1769, 0.1868,
    ],
};

const FURINA_SKILL_USHER: TalentScaling = TalentScaling {
    name: "紳士のヒステリックダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0596, 0.0641, 0.0685, 0.0745, 0.0790, 0.0834, 0.0894, 0.0954, 0.1013, 0.1073, 0.1132,
        0.1192, 0.1267, 0.1341, 0.1416,
    ],
};

const FURINA_SKILL_CHEVALMARIN: TalentScaling = TalentScaling {
    name: "騎士のシュヴァルマランダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0323, 0.0347, 0.0372, 0.0404, 0.0428, 0.0452, 0.0485, 0.0517, 0.0549, 0.0582, 0.0614,
        0.0646, 0.0687, 0.0727, 0.0768,
    ],
};

const FURINA_SKILL_CRABALETTA: TalentScaling = TalentScaling {
    name: "マドモワゼルクラバレッタダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0829, 0.0891, 0.0953, 0.1036, 0.1098, 0.1160, 0.1243, 0.1326, 0.1409, 0.1492, 0.1575,
        0.1658, 0.1761, 0.1865, 0.1968,
    ],
};

// -- Elemental Burst: 万民のカーニバル (Let the People Rejoice) -- Hydro --

const FURINA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1141, 0.1226, 0.1312, 0.1426, 0.1511, 0.1597, 0.1711, 0.1825, 0.1939, 0.2053, 0.2167,
        0.2281, 0.2424, 0.2566, 0.2709,
    ],
};

pub const FURINA: CharacterData = CharacterData {
    id: "furina",
    name: "Furina",
    element: Element::Hydro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [
        1192.00, 3091.00, 4113.00, 6154.00, 6880.00, 7916.00, 8884.00, 9930.00, 10656.00, 11712.00,
        12438.00, 13505.00, 14231.00, 15307.00, 15307.00, 15919.28, // Lv95/Lv95+/Lv100
        15919.28, // Lv95/Lv95+/Lv100
        16531.56, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        18.99, 49.27, 65.55, 98.08, 109.65, 126.16, 141.58, 158.26, 169.83, 186.66, 198.23, 215.24,
        226.81, 243.96, 243.96, 253.72, // Lv95/Lv95+/Lv100
        253.72, // Lv95/Lv95+/Lv100
        263.48, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        54.15, 140.46, 186.88, 279.63, 312.62, 359.67, 403.66, 451.20, 484.18, 532.16, 565.15,
        613.66, 646.65, 695.54, 695.54, 723.36, // Lv95/Lv95+/Lv100
        723.36, // Lv95/Lv95+/Lv100
        751.18, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "独舞の招待",
            hits: &[
                FURINA_NORMAL_1,
                FURINA_NORMAL_2,
                FURINA_NORMAL_3,
                FURINA_NORMAL_4,
            ],
            charged: &[FURINA_CHARGED],
            plunging: &[FURINA_PLUNGE, FURINA_PLUNGE_LOW, FURINA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "サロン・ソリティア",
            scalings: &[
                FURINA_SKILL_BUBBLE,
                FURINA_SKILL_USHER,
                FURINA_SKILL_CHEVALMARIN,
                FURINA_SKILL_CRABALETTA,
            ],
        },
        elemental_burst: TalentData {
            name: "万民のカーニバル",
            scalings: &[FURINA_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
