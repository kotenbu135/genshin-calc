use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Liutian Archery -- Physical --

const GANYU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3173, 0.3432, 0.3690, 0.4059, 0.4317, 0.4613, 0.5018, 0.5424, 0.5830, 0.6273, 0.6780,
        0.7377, 0.7974, 0.8570, 0.9221,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GANYU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3560, 0.3850, 0.4140, 0.4554, 0.4844, 0.5175, 0.5630, 0.6086, 0.6541, 0.7038, 0.7607,
        0.8277, 0.8946, 0.9616, 1.0346,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GANYU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4549, 0.4920, 0.5290, 0.5819, 0.6189, 0.6613, 0.7194, 0.7776, 0.8358, 0.8993, 0.9720,
        1.0576, 1.1431, 1.2287, 1.3220,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GANYU_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4549, 0.4920, 0.5290, 0.5819, 0.6189, 0.6613, 0.7194, 0.7776, 0.8358, 0.8993, 0.9720,
        1.0576, 1.1431, 1.2287, 1.3220,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GANYU_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4825, 0.5217, 0.5610, 0.6171, 0.6564, 0.7013, 0.7630, 0.8247, 0.8864, 0.9537, 1.0308,
        1.1211, 1.2114, 1.3017, 1.4014,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GANYU_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5762, 0.6231, 0.6700, 0.7370, 0.7839, 0.8375, 0.9112, 0.9849, 1.0586, 1.1390, 1.2311,
        1.3395, 1.4478, 1.5561, 1.6743,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Aimed Shot & Frostflake Arrow -- Cryo --

const GANYU_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GANYU_AIMED_CHARGE1: TalentScaling = TalentScaling {
    name: "1段チャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GANYU_FROSTFLAKE: TalentScaling = TalentScaling {
    name: "霜華の矢ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2800, 1.3760, 1.4720, 1.6000, 1.6960, 1.7920, 1.9200, 2.0480, 2.1760, 2.3040, 2.4320,
        2.5600, 2.7200, 2.8800, 3.0400,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GANYU_FROSTFLAKE_BLOOM: TalentScaling = TalentScaling {
    name: "霜華の矢・霜華満開ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        2.1760, 2.3392, 2.5024, 2.7200, 2.8832, 3.0464, 3.2640, 3.4816, 3.6992, 3.9168, 4.1344,
        4.3520, 4.6240, 4.8960, 5.1680,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const GANYU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GANYU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const GANYU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8155, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: Trail of the Qilin -- Cryo --

const GANYU_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.3200, 1.4190, 1.5180, 1.6500, 1.7490, 1.8480, 1.9800, 2.1120, 2.2440, 2.3760, 2.5080,
        2.6400, 2.8050, 2.9700, 3.1350,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: Celestial Shower -- Cryo --

const GANYU_BURST: TalentScaling = TalentScaling {
    name: "氷柱ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.7027, 0.7554, 0.8081, 0.8784, 0.9311, 0.9838, 1.0541, 1.1244, 1.1946, 1.2649, 1.3352,
        1.4054, 1.4933, 1.5811, 1.6690,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const GANYU: CharacterData = CharacterData {
    id: "ganyu",
    name: "Ganyu",
    element: Element::Cryo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [
        763.00, 1978.00, 2632.00, 3939.00, 4403.00, 5066.00, 5686.00, 6355.00, 6820.00, 7495.00,
        7960.00, 8643.00, 9108.00, 9797.00, 9797.00, 10188.88, // Lv95/Lv95+/Lv100
        10188.88, // Lv95/Lv95+/Lv100
        10580.76, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        26.07, 67.62, 89.97, 134.62, 150.50, 173.16, 194.33, 217.22, 233.10, 256.19, 272.07,
        295.43, 311.31, 334.85, 334.85, 348.24, // Lv95/Lv95+/Lv100
        348.24, // Lv95/Lv95+/Lv100
        361.64, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        49.06, 127.26, 169.33, 253.37, 283.26, 325.89, 365.74, 408.82, 438.71, 482.18, 512.07,
        556.02, 585.91, 630.21, 630.21, 655.42, // Lv95/Lv95+/Lv100
        655.42, // Lv95/Lv95+/Lv100
        680.63, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "流天射術",
            hits: &[
                GANYU_NORMAL_1,
                GANYU_NORMAL_2,
                GANYU_NORMAL_3,
                GANYU_NORMAL_4,
                GANYU_NORMAL_5,
                GANYU_NORMAL_6,
            ],
            charged: &[
                GANYU_AIMED,
                GANYU_AIMED_CHARGE1,
                GANYU_FROSTFLAKE,
                GANYU_FROSTFLAKE_BLOOM,
            ],
            plunging: &[GANYU_PLUNGE, GANYU_PLUNGE_LOW, GANYU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "山雀の仮道",
            scalings: &[GANYU_SKILL],
        },
        elemental_burst: TalentData {
            name: "降魔の天華",
            scalings: &[GANYU_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
    passive_scalings: &[],
};

// =============================================================================
// Kaeya
