use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Razor
// =============================================================================

// -- Normal Attack: 鉄の牙 (Steel Fang) -- Physical --

const RAZOR_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9592, 1.0246, 1.0900, 1.1772, 1.2426, 1.3189, 1.4170, 1.5151, 1.6132, 1.7113, 1.8094,
        1.9075, 2.0056, 2.1037, 2.2018,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const RAZOR_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8263, 0.8827, 0.9390, 1.0141, 1.0705, 1.1362, 1.2207, 1.3052, 1.3897, 1.4742, 1.5587,
        1.6433, 1.7278, 1.8123, 1.8968,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const RAZOR_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0331, 1.1036, 1.1740, 1.2679, 1.3384, 1.4205, 1.5262, 1.6319, 1.7375, 1.8432, 1.9488,
        2.0545, 2.1602, 2.2658, 2.3715,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const RAZOR_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.3605, 1.4532, 1.5460, 1.6697, 1.7624, 1.8707, 2.0098, 2.1489, 2.2881, 2.4272, 2.5664,
        2.7055, 2.8446, 2.9838, 3.1229,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const RAZOR_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6254, 0.6763, 0.7272, 0.7999, 0.8508, 0.9090, 0.9890, 1.0690, 1.1490, 1.2362, 1.3235,
        1.4108, 1.4980, 1.5853, 1.6726,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const RAZOR_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1309, 1.2230, 1.3150, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933,
        2.5511, 2.7089, 2.8667, 3.0245,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const RAZOR_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8205, 0.8872, 0.9540, 1.0494, 1.1162, 1.1925, 1.2975, 1.4024, 1.5074, 1.6219, 1.7363,
        1.8508, 1.9653, 2.0798, 2.1943,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const RAZOR_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.6406, 1.7741, 1.9077, 2.0984, 2.2320, 2.3846, 2.5944, 2.8043, 3.0141, 3.2430, 3.4719,
        3.7009, 3.9298, 4.1587, 4.3876,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const RAZOR_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        2.0492, 2.2160, 2.3828, 2.6210, 2.7878, 2.9785, 3.2406, 3.5027, 3.7648, 4.0507, 4.3366,
        4.6226, 4.9085, 5.1944, 5.4804,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 爪雷 (Claw and Thunder) -- Electro --

const RAZOR_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "一回押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.9920, 2.1414, 2.2908, 2.4900, 2.6394, 2.7888, 2.9880, 3.1872, 3.3864, 3.5856, 3.7848,
        3.9840, 4.2330, 4.4820, 4.7310,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const RAZOR_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        2.9520, 3.1734, 3.3948, 3.6900, 3.9114, 4.1328, 4.4280, 4.7232, 5.0184, 5.3136, 5.6088,
        5.9040, 6.2730, 6.6420, 7.0110,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 雷牙 (Lightning Fang) -- Electro --

const RAZOR_BURST: TalentScaling = TalentScaling {
    name: "元素爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.6000, 1.7200, 1.8400, 2.0000, 2.1200, 2.2400, 2.4000, 2.5600, 2.7200, 2.8800, 3.0400,
        3.2000, 3.4000, 3.6000, 3.8000,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const RAZOR_BURST_SOUL_COMPANION: TalentScaling = TalentScaling {
    name: "狼魂ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.2400, 0.2580, 0.2760, 0.3000, 0.3180, 0.3360, 0.3600, 0.3840, 0.4080, 0.4320, 0.4560,
        0.4800, 0.5100, 0.5400, 0.5700,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const RAZOR: CharacterData = CharacterData {
    id: "razor",
    name: "Razor",
    element: Element::Electro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [
        1003.00, 2577.00, 3326.00, 4982.00, 5514.00, 6343.00, 7052.00, 7881.00, 8413.00, 9241.00,
        9773.00, 10602.00, 11134.00, 11962.00, 11962.00, 12440.48, // Lv95/Lv95+/Lv100
        12440.48, // Lv95/Lv95+/Lv100
        12918.96, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        19.59, 50.32, 64.96, 97.30, 107.70, 123.88, 137.74, 153.92, 164.32, 180.48, 190.88, 207.06,
        217.46, 233.64, 233.64, 242.99, // Lv95/Lv95+/Lv100
        242.99, // Lv95/Lv95+/Lv100
        252.33, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        62.95, 161.71, 208.74, 312.66, 346.08, 398.07, 442.62, 494.62, 528.03, 579.96, 613.37,
        665.37, 698.78, 750.77, 750.77, 780.80, // Lv95/Lv95+/Lv100
        780.80, // Lv95/Lv95+/Lv100
        810.83, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::PhysicalDmgBonus(0.30),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "鉄の牙",
            hits: &[
                RAZOR_NORMAL_1,
                RAZOR_NORMAL_2,
                RAZOR_NORMAL_3,
                RAZOR_NORMAL_4,
            ],
            charged: &[RAZOR_CHARGED_SPINNING, RAZOR_CHARGED_FINAL],
            plunging: &[RAZOR_PLUNGE, RAZOR_PLUNGE_LOW, RAZOR_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "爪雷",
            scalings: &[RAZOR_SKILL_PRESS, RAZOR_SKILL_HOLD],
        },
        elemental_burst: TalentData {
            name: "雷牙",
            scalings: &[RAZOR_BURST, RAZOR_BURST_SOUL_COMPANION],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
    passive_scalings: &[],
};
