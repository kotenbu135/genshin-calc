use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack -- Physical --

const AINO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7032, 0.7604, 0.8176, 0.8994, 0.9566, 1.0220, 1.1120, 1.2019, 1.2918, 1.3899, 1.4880,
        1.5862, 1.6843, 1.7824, 1.8805,
    ],
};

const AINO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6588, 0.7124, 0.7660, 0.8426, 0.8962, 0.9575, 1.0417, 1.1260, 1.2102, 1.3022, 1.3942,
        1.4862, 1.5782, 1.6702, 1.7622,
    ],
};

const AINO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7920, 0.8564, 0.9208, 1.0128, 1.0772, 1.1510, 1.2524, 1.3538, 1.4552, 1.5658, 1.6764,
        1.7870, 1.8976, 2.0082, 2.1188,
    ],
};

const AINO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9636, 1.0421, 1.1206, 1.2327, 1.3112, 1.4007, 1.5239, 1.6470, 1.7702, 1.9043, 2.0385,
        2.1726, 2.3068, 2.4409, 2.5751,
    ],
};

// -- Charged Attack --

const AINO_CHARGED: TalentScaling = TalentScaling {
    name: "チャージ攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.2780, 1.3820, 1.4861, 1.6347, 1.7387, 1.8576, 2.0211, 2.1845, 2.3480, 2.5263, 2.7047,
        2.8830, 3.0614, 3.2397, 3.4181,
    ],
};

// -- Plunging Attack --

const AINO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2748, 1.3702, 1.4742, 1.5783,
        1.6823, 1.7864, 1.8904, 1.9944,
    ],
};

const AINO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9076, 2.0289, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5726, 3.7807, 3.9888,
    ],
};

const AINO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1661, 2.3827, 2.5342, 2.7076, 2.9459, 3.1842, 3.4225, 3.6826, 3.9428,
        4.2029, 4.4631, 4.7232, 4.9834,
    ],
};

// -- Elemental Skill -- Hydro --

const AINO_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.68, 1.806, 1.932, 2.10, 2.226, 2.352, 2.52, 2.688, 2.856, 3.024, 3.192, 3.36, 3.57, 3.78,
        3.99,
    ],
};

// -- Elemental Burst -- Hydro --

const AINO_BURST: TalentScaling = TalentScaling {
    name: "バーストダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        2.016, 2.1672, 2.3184, 2.52, 2.6712, 2.8224, 3.024, 3.2256, 3.4272, 3.6288, 3.8304, 4.032,
        4.284, 4.536, 4.788,
    ],
};

pub const AINO: CharacterData = CharacterData {
    id: "aino",
    name: "Aino",
    element: Element::Hydro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Snezhnaya,
    base_hp: [
        939.00, 2413.00, 3114.00, 4665.00, 5163.00, 5939.00, 6604.00, 7379.00, 7878.00, 8653.00,
        9151.00, 9927.00, 10425.00, 11201.00, 11201.00, 11649.04, // Lv95/Lv95+/Lv100
        11649.04, // Lv95/Lv95+/Lv100
        12097.08, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        20.30, 52.15, 67.32, 100.84, 111.61, 128.38, 142.75, 159.52, 170.29, 187.04, 197.82,
        214.59, 225.36, 242.13, 242.13, 251.82, // Lv95/Lv95+/Lv100
        251.82, // Lv95/Lv95+/Lv100
        261.50, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        50.93, 130.84, 168.89, 252.97, 280.01, 322.08, 358.12, 400.19, 427.22, 469.24, 496.27,
        538.34, 565.38, 607.44, 607.44, 631.74, // Lv95/Lv95+/Lv100
        631.74, // Lv95/Lv95+/Lv100
        656.04, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "Bish-Bash-Bosh Repair",
            hits: &[AINO_NORMAL_1, AINO_NORMAL_2, AINO_NORMAL_3, AINO_NORMAL_4],
            charged: &[AINO_CHARGED],
            plunging: &[AINO_PLUNGE, AINO_PLUNGE_LOW, AINO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "Musecatcher",
            scalings: &[AINO_SKILL],
        },
        elemental_burst: TalentData {
            name: "Precision Hydronic Cooler",
            scalings: &[AINO_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
