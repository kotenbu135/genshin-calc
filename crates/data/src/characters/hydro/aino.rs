use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack -- Physical --

const AINO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6650, 0.7191, 0.7732, 0.8506, 0.9047, 0.9666, 1.0516, 1.1367, 1.2217, 1.3145, 1.4073,
        1.5001, 1.5929, 1.6857, 1.7785,
    ],
    dynamic_bonus: None,
};

const AINO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6619, 0.7158, 0.7697, 0.8466, 0.9005, 0.9621, 1.0468, 1.1314, 1.2161, 1.3084, 1.4008,
        1.4932, 1.5855, 1.6779, 1.7702,
    ],
    dynamic_bonus: None,
};

const AINO_NORMAL_3_1: TalentScaling = TalentScaling {
    name: "3段ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4922, 0.5322, 0.5723, 0.6295, 0.6696, 0.7154, 0.7783, 0.8413, 0.9042, 0.9729, 1.0416,
        1.1102, 1.1789, 1.2476, 1.3163,
    ],
    dynamic_bonus: None,
};

const AINO_NORMAL_3_2: TalentScaling = TalentScaling {
    name: "3段ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4922, 0.5322, 0.5723, 0.6295, 0.6696, 0.7154, 0.7783, 0.8413, 0.9042, 0.9729, 1.0416,
        1.1102, 1.1789, 1.2476, 1.3163,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack --

const AINO_CHARGED_LOOP: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6252, 0.6761, 0.7270, 0.7997, 0.8506, 0.9088, 0.9887, 1.0687, 1.1487, 1.2359, 1.3231,
        1.4104, 1.4976, 1.5849, 1.6721,
    ],
    dynamic_bonus: None,
};

const AINO_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1309, 1.2230, 1.3150, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933,
        2.5511, 2.7089, 2.8667, 3.0245,
    ],
    dynamic_bonus: None,
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
    dynamic_bonus: None,
};

const AINO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9076, 2.0289, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5726, 3.7807, 3.9888,
    ],
    dynamic_bonus: None,
};

const AINO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1661, 2.3827, 2.5342, 2.7076, 2.9459, 3.1842, 3.4225, 3.6826, 3.9428,
        4.2029, 4.4631, 4.7232, 4.9834,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill -- Hydro --

const AINO_SKILL_STAGE_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.6560, 0.7052, 0.7544, 0.8200, 0.8692, 0.9184, 0.9840, 1.0496, 1.1152, 1.1808, 1.2464,
        1.3120, 1.3940, 1.4760, 1.5580,
    ],
    dynamic_bonus: None,
};

const AINO_SKILL_STAGE_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.8880, 2.0296, 2.1712, 2.3600, 2.5016, 2.6432, 2.8320, 3.0208, 3.2096, 3.3984, 3.5872,
        3.7760, 4.0120, 4.2480, 4.4840,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst -- Hydro --

const AINO_BURST: TalentScaling = TalentScaling {
    name: "バーストダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.2011, 0.2162, 0.2313, 0.2514, 0.2665, 0.2816, 0.3017, 0.3218, 0.3419, 0.3620, 0.3821,
        0.4022, 0.4274, 0.4525, 0.4777,
    ],
    dynamic_bonus: None,
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
            hits: &[
                AINO_NORMAL_1,
                AINO_NORMAL_2,
                AINO_NORMAL_3_1,
                AINO_NORMAL_3_2,
            ],
            charged: &[AINO_CHARGED_LOOP, AINO_CHARGED_FINAL],
            plunging: &[AINO_PLUNGE, AINO_PLUNGE_LOW, AINO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "Musecatcher",
            scalings: &[AINO_SKILL_STAGE_1, AINO_SKILL_STAGE_2],
        },
        elemental_burst: TalentData {
            name: "Precision Hydronic Cooler",
            scalings: &[AINO_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
