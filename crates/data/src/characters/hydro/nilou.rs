use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 弦月のダンス (Dance of Samser) -- Physical (Sword) --

const NILOU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5031, 0.5441, 0.5850, 0.6435, 0.6845, 0.7313, 0.7956, 0.8599, 0.9242, 0.9945, 1.0648,
        1.1351, 1.2054, 1.2757, 1.3460,
    ],
};

const NILOU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4544, 0.4914, 0.5284, 0.5812, 0.6182, 0.6605, 0.7189, 0.7773, 0.8356, 0.8986, 0.9616,
        1.0246, 1.0876, 1.1506, 1.2136,
    ],
};

const NILOU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7035, 0.7608, 0.8182, 0.9000, 0.9574, 1.0228, 1.1127, 1.2026, 1.2925, 1.3909, 1.4893,
        1.5876, 1.6860, 1.7844, 1.8828,
    ],
};

// -- Charged Attack -- Physical (Sword) --

const NILOU_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5022, 0.5431, 0.5840, 0.6424, 0.6833, 0.7300, 0.7942, 0.8585, 0.9227, 0.9928, 1.0630,
        1.1331, 1.2032, 1.2734, 1.3435,
    ],
};

const NILOU_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5444, 0.5887, 0.6330, 0.6963, 0.7406, 0.7913, 0.8608, 0.9304, 1.0000, 1.0760, 1.1520,
        1.2280, 1.3040, 1.3800, 1.4560,
    ],
};

// -- Plunging Attack -- Physical (Sword) --

const NILOU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const NILOU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const NILOU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 七域のダンス (Dance of Haftkarsvar) -- Hydro (HP scaling) --

const NILOU_SKILL_STEP: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0334, 0.0359, 0.0384, 0.0417, 0.0442, 0.0467, 0.0501, 0.0534, 0.0568, 0.0601, 0.0634,
        0.0668, 0.0710, 0.0751, 0.0793,
    ],
};

const NILOU_SKILL_SWORD_1: TalentScaling = TalentScaling {
    name: "剣舞ステップ1ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0455, 0.0489, 0.0524, 0.0569, 0.0603, 0.0637, 0.0683, 0.0728, 0.0774, 0.0819, 0.0865,
        0.0911, 0.0967, 0.1024, 0.1081,
    ],
};

const NILOU_SKILL_SWORD_2: TalentScaling = TalentScaling {
    name: "剣舞ステップ2ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0514, 0.0553, 0.0592, 0.0643, 0.0682, 0.0720, 0.0772, 0.0823, 0.0875, 0.0926, 0.0977,
        0.1029, 0.1093, 0.1158, 0.1222,
    ],
};

const NILOU_SKILL_WATER_WHEEL: TalentScaling = TalentScaling {
    name: "水月ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0717, 0.0771, 0.0824, 0.0896, 0.0950, 0.1004, 0.1075, 0.1147, 0.1219, 0.1290, 0.1362,
        0.1434, 0.1523, 0.1613, 0.1703,
    ],
};

// -- Elemental Burst: 浮蓮のダンス (Dance of Abzendegi: Distant Dreams, Listening Spring) -- Hydro --

const NILOU_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1843, 0.1981, 0.2120, 0.2304, 0.2442, 0.2580, 0.2765, 0.2949, 0.3133, 0.3318, 0.3502,
        0.3686, 0.3917, 0.4147, 0.4378,
    ],
};

const NILOU_BURST_LINGERING: TalentScaling = TalentScaling {
    name: "永続ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.2253, 0.2422, 0.2591, 0.2816, 0.2985, 0.3154, 0.3379, 0.3604, 0.3830, 0.4055, 0.4280,
        0.4506, 0.4787, 0.5069, 0.5350,
    ],
};

pub const NILOU: CharacterData = CharacterData {
    id: "nilou",
    name: "Nilou",
    element: Element::Hydro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Sumeru,
    base_hp: [1182.0, 13397.0, 14117.0, 15185.0],
    base_atk: [18.0, 204.0, 215.0, 230.0],
    base_def: [57.0, 642.0, 677.0, 729.0],
    ascension_stat: AscensionStat::Hp(0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "弦月のダンス",
            hits: &[NILOU_NORMAL_1, NILOU_NORMAL_2, NILOU_NORMAL_3],
            charged: &[NILOU_CHARGED_1, NILOU_CHARGED_2],
            plunging: &[NILOU_PLUNGE, NILOU_PLUNGE_LOW, NILOU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "七域のダンス",
            scalings: &[
                NILOU_SKILL_STEP,
                NILOU_SKILL_SWORD_1,
                NILOU_SKILL_SWORD_2,
                NILOU_SKILL_WATER_WHEEL,
            ],
        },
        elemental_burst: TalentData {
            name: "浮蓮のダンス",
            scalings: &[NILOU_BURST, NILOU_BURST_LINGERING],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
