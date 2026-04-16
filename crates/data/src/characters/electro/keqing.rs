use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Keqing
// =============================================================================

// -- Normal Attack: 雲来剣法 (Yunlai Swordsmanship) -- Physical --

const KEQING_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4102, 0.4436, 0.4770, 0.5247, 0.5581, 0.5963, 0.6487, 0.7012, 0.7537, 0.8109, 0.8682,
        0.9254, 0.9826, 1.0399, 1.0971,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KEQING_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4102, 0.4436, 0.4770, 0.5247, 0.5581, 0.5963, 0.6487, 0.7012, 0.7537, 0.8109, 0.8682,
        0.9254, 0.9826, 1.0399, 1.0971,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KEQING_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5444, 0.5887, 0.6330, 0.6963, 0.7406, 0.7913, 0.8609, 0.9306, 1.0003, 1.0762, 1.1521,
        1.2281, 1.3040, 1.3799, 1.4558,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KEQING_NORMAL_4A: TalentScaling = TalentScaling {
    name: "4段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3148, 0.3404, 0.3660, 0.4026, 0.4282, 0.4575, 0.4978, 0.5380, 0.5783, 0.6222, 0.6662,
        0.7101, 0.7541, 0.7980, 0.8420,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KEQING_NORMAL_4B: TalentScaling = TalentScaling {
    name: "4段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3440, 0.3720, 0.4000, 0.4400, 0.4680, 0.5000, 0.5440, 0.5880, 0.6320, 0.6800, 0.7280,
        0.7760, 0.8240, 0.8720, 0.9200,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KEQING_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6699, 0.7243, 0.7787, 0.8566, 0.9110, 0.9734, 1.0591, 1.1449, 1.2306, 1.3239, 1.4172,
        1.5106, 1.6039, 1.6972, 1.7905,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const KEQING_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7680, 0.8305, 0.8930, 0.9823, 1.0448, 1.1163, 1.2142, 1.3122, 1.4102, 1.5178, 1.6254,
        1.7330, 1.8394, 1.9470, 2.0546,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KEQING_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8600, 0.9300, 1.0000, 1.1000, 1.1700, 1.2500, 1.3600, 1.4700, 1.5800, 1.7000, 1.8200,
        1.9400, 2.0600, 2.1800, 2.3000,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const KEQING_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KEQING_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KEQING_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 星辰帰位 (Stellar Restoration) -- Electro --

const KEQING_SKILL_STILETTO: TalentScaling = TalentScaling {
    name: "雷楔ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5040, 0.5418, 0.5796, 0.6300, 0.6678, 0.7056, 0.7560, 0.8064, 0.8568, 0.9072, 0.9576,
        1.0080, 1.0710, 1.1340, 1.1970,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KEQING_SKILL_SLASH: TalentScaling = TalentScaling {
    name: "斬撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.6800, 1.8060, 1.9320, 2.1000, 2.2260, 2.3520, 2.5200, 2.6880, 2.8560, 3.0240, 3.1920,
        3.3600, 3.5700, 3.7800, 3.9900,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KEQING_SKILL_THUNDERCLAP: TalentScaling = TalentScaling {
    name: "雷鳴斬撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.8400, 0.9030, 0.9660, 1.0500, 1.1130, 1.1760, 1.2600, 1.3440, 1.4280, 1.5120, 1.5960,
        1.6800, 1.7850, 1.8900, 1.9950,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 天街巡遊 (Starward Sword) -- Electro --

const KEQING_BURST_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.8800, 0.9460, 1.0120, 1.1000, 1.1660, 1.2320, 1.3200, 1.4080, 1.4960, 1.5840, 1.6720,
        1.7600, 1.8700, 1.9800, 2.0900,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KEQING_BURST_CONSECUTIVE: TalentScaling = TalentScaling {
    name: "連斬ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.2400, 0.2580, 0.2760, 0.3000, 0.3180, 0.3360, 0.3600, 0.3840, 0.4080, 0.4320, 0.4560,
        0.4800, 0.5100, 0.5400, 0.5700,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KEQING_BURST_LAST: TalentScaling = TalentScaling {
    name: "最後の一撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.8880, 2.0296, 2.1712, 2.3600, 2.5016, 2.6432, 2.8320, 3.0208, 3.2096, 3.3984, 3.5872,
        3.7760, 4.0120, 4.2480, 4.4840,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const KEQING: CharacterData = CharacterData {
    id: "keqing",
    name: "Keqing",
    element: Element::Electro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [
        1020.00, 2646.00, 3521.00, 5268.00, 5889.00, 6776.00, 7604.00, 8500.00, 9121.00, 10025.00,
        10647.00, 11561.00, 12182.00, 13103.00, 13103.00, 13627.12, // Lv95/Lv95+/Lv100
        13627.12, // Lv95/Lv95+/Lv100
        14151.24, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        25.14, 65.21, 86.76, 129.82, 145.13, 166.97, 187.39, 209.46, 224.77, 247.04, 262.36,
        284.88, 300.19, 322.89, 322.89, 335.81, // Lv95/Lv95+/Lv100
        335.81, // Lv95/Lv95+/Lv100
        348.72, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        62.22, 161.41, 214.76, 321.35, 359.26, 413.33, 463.87, 518.50, 556.41, 611.54, 649.45,
        705.20, 743.11, 799.30, 799.30, 831.27, // Lv95/Lv95+/Lv100
        831.27, // Lv95/Lv95+/Lv100
        863.24, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "雲来剣法",
            hits: &[
                KEQING_NORMAL_1,
                KEQING_NORMAL_2,
                KEQING_NORMAL_3,
                KEQING_NORMAL_4A,
                KEQING_NORMAL_4B,
                KEQING_NORMAL_5,
            ],
            charged: &[KEQING_CHARGED_1, KEQING_CHARGED_2],
            plunging: &[KEQING_PLUNGE, KEQING_PLUNGE_LOW, KEQING_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "星辰帰位",
            scalings: &[
                KEQING_SKILL_STILETTO,
                KEQING_SKILL_SLASH,
                KEQING_SKILL_THUNDERCLAP,
            ],
        },
        elemental_burst: TalentData {
            name: "天街巡遊",
            scalings: &[
                KEQING_BURST_SKILL,
                KEQING_BURST_CONSECUTIVE,
                KEQING_BURST_LAST,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
    passive_scalings: &[],
    scaling_modifiers: &[],
};
