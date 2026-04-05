use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Varesa
// =============================================================================

// -- Normal Attack: 角撃ち (By the Horns) -- All Electro (Catalyst) --
// Standard State

const VARESA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4680, 0.5030, 0.5400, 0.5850, 0.6200, 0.6550, 0.7020, 0.7480, 0.7950, 0.8420, 0.8890,
        0.9360, 0.9940, 1.0530, 1.1110,
    ],
    dynamic_bonus: None,
};

const VARESA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4000, 0.4300, 0.4600, 0.5000, 0.5300, 0.5600, 0.6000, 0.6400, 0.6800, 0.7210, 0.7610,
        0.8010, 0.8510, 0.9010, 0.9510,
    ],
    dynamic_bonus: None,
};

const VARESA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5630, 0.6050, 0.6480, 0.7040, 0.7460, 0.7880, 0.8450, 0.9010, 0.9570, 1.0140, 1.0700,
        1.1260, 1.1970, 1.2670, 1.3370,
    ],
    dynamic_bonus: None,
};

// -- Fiery Passion State --

const VARESA_PASSION_NORMAL_1: TalentScaling = TalentScaling {
    name: "炎情1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5440, 0.5850, 0.6260, 0.6800, 0.7210, 0.7620, 0.8160, 0.8710, 0.9250, 0.9790, 1.0340,
        1.0880, 1.1560, 1.2240, 1.2920,
    ],
    dynamic_bonus: None,
};

const VARESA_PASSION_NORMAL_2: TalentScaling = TalentScaling {
    name: "炎情2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5200, 0.5600, 0.5980, 0.6500, 0.6890, 0.7280, 0.7800, 0.8320, 0.8840, 0.9370, 0.9890,
        1.0410, 1.1060, 1.1710, 1.2360,
    ],
    dynamic_bonus: None,
};

const VARESA_PASSION_NORMAL_3: TalentScaling = TalentScaling {
    name: "炎情3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7360, 0.7910, 0.8460, 0.9200, 0.9750, 1.0300, 1.1040, 1.1770, 1.2510, 1.3250, 1.3980,
        1.4720, 1.5640, 1.6560, 1.7480,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Electro --

const VARESA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.8930, 0.9600, 1.0270, 1.1160, 1.1830, 1.2500, 1.3390, 1.4280, 1.5180, 1.6070, 1.6960,
        1.7860, 1.8970, 2.0090, 2.1200,
    ],
    dynamic_bonus: None,
};

const VARESA_PASSION_CHARGED: TalentScaling = TalentScaling {
    name: "炎情重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.9260, 0.9960, 1.0650, 1.1580, 1.2270, 1.2970, 1.3900, 1.4820, 1.5750, 1.6780, 1.7600,
        1.8530, 1.9690, 2.0840, 2.2000,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Electro (Catalyst) --

const VARESA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7459, 0.8066, 0.8673, 0.9541, 1.0148, 1.0841, 1.1795, 1.2749, 1.3702, 1.4744, 1.5786,
        1.6827, 1.7869, 1.8910, 1.9952,
    ],
    dynamic_bonus: None,
};

const VARESA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.4914, 1.6128, 1.7342, 1.9076, 2.0290, 2.1678, 2.3586, 2.5494, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
    dynamic_bonus: None,
};

const VARESA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5345, 2.7078, 2.9460, 3.1843, 3.4225, 3.6824, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
    dynamic_bonus: None,
};

// Fiery Passion plunge variants (higher damage)
const VARESA_PASSION_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "炎情低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        2.2370, 2.4190, 2.6010, 2.8610, 3.0440, 3.2520, 3.5380, 3.8240, 4.1100, 4.4220, 4.7340,
        5.0470, 5.3590, 5.6710, 5.9830,
    ],
    dynamic_bonus: None,
};

const VARESA_PASSION_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "炎情高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        2.7940, 3.0220, 3.2490, 3.5740, 3.8020, 4.0620, 4.4190, 4.7760, 5.1340, 5.5240, 5.9140,
        6.3030, 6.6930, 7.0830, 7.4730,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: 夜虹乗り (Riding the Night-Rainbow) -- Electro --

const VARESA_SKILL_RUSH: TalentScaling = TalentScaling {
    name: "突進ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7450, 0.8010, 0.8570, 0.9310, 0.9870, 1.0430, 1.1170, 1.1920, 1.2660, 1.3410, 1.4150,
        1.4900, 1.5830, 1.6760, 1.7690,
    ],
    dynamic_bonus: None,
};

const VARESA_SKILL_PASSION_RUSH: TalentScaling = TalentScaling {
    name: "炎情突進ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.0640, 1.1440, 1.2240, 1.3300, 1.4100, 1.4900, 1.5960, 1.7020, 1.8090, 1.9150, 2.0220,
        2.1280, 2.2610, 2.3940, 2.5270,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: ガーディアンベント! (Guardian Vent!) -- Electro --

const VARESA_BURST_KICK: TalentScaling = TalentScaling {
    name: "フライングキック",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        3.4510, 3.7100, 3.9690, 4.3140, 4.5730, 4.8320, 5.1770, 5.2190, 5.8670, 6.2120, 6.5570,
        6.9020, 7.3340, 7.6550, 8.1970,
    ],
    dynamic_bonus: None,
};

const VARESA_BURST_PASSION_KICK: TalentScaling = TalentScaling {
    name: "炎情フライングキック",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        5.7520, 6.1830, 6.6150, 7.1900, 7.6210, 8.0530, 8.6280, 9.2030, 9.7780, 10.3540, 10.9290,
        11.5040, 12.2230, 12.9420, 13.6610,
    ],
    dynamic_bonus: None,
};

const VARESA_BURST_VOLCANO: TalentScaling = TalentScaling {
    name: "ボルケーノカブラム",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        4.0260, 4.3280, 4.6300, 5.0330, 5.3350, 5.6370, 6.0400, 6.4420, 6.8450, 7.2480, 7.6500,
        8.0530, 8.5560, 9.0590, 9.5630,
    ],
    dynamic_bonus: None,
};

pub const VARESA: CharacterData = CharacterData {
    id: "varesa",
    name: "Varesa",
    element: Element::Electro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [
        989.00, 2564.00, 3412.00, 5106.00, 5708.00, 6567.00, 7370.00, 8238.00, 8840.00, 9716.00,
        10318.00, 11204.00, 11806.00, 12699.00, 12699.00, 13206.96, // Lv95/Lv95+/Lv100
        13206.96, // Lv95/Lv95+/Lv100
        13714.92, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        27.74, 71.97, 95.75, 143.28, 160.18, 184.29, 206.82, 231.18, 248.08, 272.66, 289.56,
        314.42, 331.32, 356.38, 356.38, 370.64, // Lv95/Lv95+/Lv100
        370.64, // Lv95/Lv95+/Lv100
        384.89, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        60.85, 157.84, 210.01, 314.24, 351.31, 404.19, 453.61, 507.04, 544.11, 598.02, 635.09,
        689.61, 726.67, 781.62, 781.62, 812.88, // Lv95/Lv95+/Lv100
        812.88, // Lv95/Lv95+/Lv100
        844.15, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "角撃ち",
            hits: &[
                VARESA_NORMAL_1,
                VARESA_NORMAL_2,
                VARESA_NORMAL_3,
                VARESA_PASSION_NORMAL_1,
                VARESA_PASSION_NORMAL_2,
                VARESA_PASSION_NORMAL_3,
            ],
            charged: &[VARESA_CHARGED, VARESA_PASSION_CHARGED],
            plunging: &[
                VARESA_PLUNGE,
                VARESA_PLUNGE_LOW,
                VARESA_PLUNGE_HIGH,
                VARESA_PASSION_PLUNGE_LOW,
                VARESA_PASSION_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "夜虹乗り",
            scalings: &[VARESA_SKILL_RUSH, VARESA_SKILL_PASSION_RUSH],
        },
        elemental_burst: TalentData {
            name: "ガーディアンベント!",
            scalings: &[
                VARESA_BURST_KICK,
                VARESA_BURST_PASSION_KICK,
                VARESA_BURST_VOLCANO,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
