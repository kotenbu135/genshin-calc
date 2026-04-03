use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 流耀槍術・守勢 (Gleaming Spear - Guardian Stance) -- Physical --

const CANDACE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6080, 0.6580, 0.7070, 0.7780, 0.8270, 0.8840, 0.9620, 1.0390, 1.1170, 1.2020, 1.2870,
        1.3720, 1.4560, 1.5410, 1.6260,
    ],
};

const CANDACE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6110, 0.6610, 0.7110, 0.7820, 0.8320, 0.8890, 0.9700, 1.0450, 1.1230, 1.2090, 1.2940,
        1.3790, 1.4650, 1.5500, 1.6350,
    ],
};

const CANDACE_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3550, 0.3840, 0.4130, 0.4540, 0.4830, 0.5160, 0.5610, 0.6070, 0.6520, 0.7020, 0.7510,
        0.8010, 0.8500, 0.9000, 0.9490,
    ],
};

const CANDACE_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4340, 0.4690, 0.5040, 0.5550, 0.5900, 0.6300, 0.6860, 0.7410, 0.7970, 0.8570, 0.9180,
        0.9780, 1.0390, 1.0990, 1.1600,
    ],
};

const CANDACE_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9490, 1.0270, 1.1040, 1.2140, 1.2920, 1.3800, 1.5010, 1.6230, 1.7440, 1.8770, 2.0090,
        2.1420, 2.2740, 2.4070, 2.5390,
    ],
};

// -- Charged Attack -- Physical --

const CANDACE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2420, 1.3430, 1.4440, 1.5880, 1.6890, 1.8050, 1.9640, 2.1230, 2.2820, 2.4550, 2.6280,
        2.8010, 2.9750, 3.1480, 3.3210,
    ],
};

// -- Plunging Attack -- Physical --

const CANDACE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const CANDACE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const CANDACE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 聖儀·蒼鷺による庇護 (Sacred Rite: Heron's Sanctum) -- Hydro --

const CANDACE_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "一段チャージダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1200, 0.1200, 0.1200, 0.1200, 0.1200, 0.1200, 0.1200, 0.1200, 0.1200, 0.1200, 0.1200,
        0.1200, 0.1200, 0.1200, 0.1200,
    ],
};

const CANDACE_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "二段チャージダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1904, 0.2047, 0.2190, 0.2380, 0.2523, 0.2666, 0.2856, 0.3046, 0.3237, 0.3427, 0.3618,
        0.3808, 0.4046, 0.4284, 0.4522,
    ],
};

// -- Elemental Burst: 聖儀·灰鴒の呼び潮 (Sacred Rite: Wagtail's Tide) -- Hydro --

const CANDACE_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0661, 0.0711, 0.0760, 0.0826, 0.0876, 0.0925, 0.0992, 0.1058, 0.1124, 0.1190, 0.1256,
        0.1322, 0.1405, 0.1487, 0.1570,
    ],
};

const CANDACE_BURST_WAVE: TalentScaling = TalentScaling {
    name: "波衝撃ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0661, 0.0711, 0.0760, 0.0826, 0.0876, 0.0925, 0.0992, 0.1058, 0.1124, 0.1190, 0.1256,
        0.1322, 0.1405, 0.1487, 0.1570,
    ],
};

pub const CANDACE: CharacterData = CharacterData {
    id: "candace",
    name: "Candace",
    element: Element::Hydro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Sumeru,
    base_hp: [
        912.00, 2342.00, 3024.00, 4529.00, 5013.00, 5766.00, 6411.00, 7164.00, 7648.00, 8401.00,
        8885.00, 9638.00, 10122.00, 10875.00, 10875.00, 11310.00, // Lv95/Lv95+/Lv100
        11310.00, // Lv95/Lv95+/Lv100
        11745.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        17.81, 45.75, 59.05, 88.45, 97.91, 112.62, 125.22, 139.93, 149.38, 164.07, 173.53, 188.24,
        197.69, 212.40, 212.40, 220.90, // Lv95/Lv95+/Lv100
        220.90, // Lv95/Lv95+/Lv100
        229.39, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        57.22, 147.01, 189.76, 284.24, 314.61, 361.88, 402.38, 449.65, 480.03, 527.24, 557.61,
        604.88, 635.25, 682.52, 682.52, 709.82, // Lv95/Lv95+/Lv100
        709.82, // Lv95/Lv95+/Lv100
        737.12, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "流耀槍術・守勢",
            hits: &[
                CANDACE_NORMAL_1,
                CANDACE_NORMAL_2,
                CANDACE_NORMAL_3A,
                CANDACE_NORMAL_3B,
                CANDACE_NORMAL_4,
            ],
            charged: &[CANDACE_CHARGED],
            plunging: &[CANDACE_PLUNGE, CANDACE_PLUNGE_LOW, CANDACE_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "聖儀·蒼鷺による庇護",
            scalings: &[CANDACE_SKILL_PRESS, CANDACE_SKILL_HOLD],
        },
        elemental_burst: TalentData {
            name: "聖儀·灰鴒の呼び潮",
            scalings: &[CANDACE_BURST, CANDACE_BURST_WAVE],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
