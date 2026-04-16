use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Dehya
// =============================================================================

// -- Normal Attack: 拳闘術 (Sandstorm Assault) -- Physical --

const DEHYA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6212, 0.6717, 0.7223, 0.7945, 0.8451, 0.9029, 0.9823, 1.0618, 1.1412, 1.2279, 1.3146,
        1.4013, 1.4879, 1.5746, 1.6613,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const DEHYA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6171, 0.6673, 0.7176, 0.7893, 0.8395, 0.8970, 0.9759, 1.0548, 1.1337, 1.2199, 1.3060,
        1.3921, 1.4782, 1.5643, 1.6504,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const DEHYA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7663, 0.8287, 0.8911, 0.9802, 1.0425, 1.1138, 1.2118, 1.3099, 1.4079, 1.5148, 1.6217,
        1.7287, 1.8356, 1.9425, 2.0494,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const DEHYA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9529, 1.0305, 1.1080, 1.2188, 1.2964, 1.3851, 1.5069, 1.6288, 1.7507, 1.8837, 2.0166,
        2.1496, 2.2826, 2.4155, 2.5485,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const DEHYA_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5633, 0.6092, 0.6550, 0.7205, 0.7664, 0.8188, 0.8908, 0.9629, 1.0349, 1.1135, 1.1921,
        1.2707, 1.3493, 1.4279, 1.5065,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const DEHYA_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0182, 1.1011, 1.1840, 1.3024, 1.3853, 1.4800, 1.6102, 1.7405, 1.8707, 2.0128, 2.1549,
        2.2970, 2.4390, 2.5811, 2.7232,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const DEHYA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const DEHYA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const DEHYA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9460, 3.1842, 3.4225, 3.6825, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 熔鉄流獄 (Molten Inferno) -- Pyro --

const DEHYA_SKILL_INDOMITABLE: TalentScaling = TalentScaling {
    name: "不屈の炎ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1288, 1.2135, 1.2981, 1.4110, 1.4957, 1.5803, 1.6932, 1.8061, 1.9190, 2.0318, 2.1447,
        2.2576, 2.3987, 2.5398, 2.6809,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const DEHYA_SKILL_RANGE: TalentScaling = TalentScaling {
    name: "領域ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.3280, 1.4276, 1.5272, 1.6600, 1.7596, 1.8592, 1.9920, 2.1248, 2.2576, 2.3904, 2.5232,
        2.6560, 2.8220, 2.9880, 3.1540,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const DEHYA_SKILL_FIELD: TalentScaling = TalentScaling {
    name: "領域ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.6020, 0.6472, 0.6923, 0.7525, 0.7977, 0.8428, 0.9030, 0.9632, 1.0234, 1.0836, 1.1438,
        1.2040, 1.2793, 1.3545, 1.4298,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 炎獅子の噛み付き (Leonine Bite) -- Pyro --

const DEHYA_BURST_FIST: TalentScaling = TalentScaling {
    name: "拳撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9870, 1.0610, 1.1351, 1.2338, 1.3078, 1.3818, 1.4805, 1.5792, 1.6779, 1.7766, 1.8753,
        1.9740, 2.0974, 2.2208, 2.3441,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const DEHYA_BURST_KICK: TalentScaling = TalentScaling {
    name: "蹴りダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.3930, 1.4975, 1.6020, 1.7413, 1.8457, 1.9502, 2.0895, 2.2288, 2.3681, 2.5074, 2.6467,
        2.7860, 2.9601, 3.1343, 3.3084,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const DEHYA: CharacterData = CharacterData {
    id: "dehya",
    name: "Dehya",
    element: Element::Pyro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Sumeru,
    base_hp: [
        1220.00, 3165.00, 4212.00, 6302.00, 7045.00, 8106.00, 9097.00, 10168.00, 10912.00,
        11993.00, 12736.00, 13829.00, 14573.00, 15675.00, 15675.00,
        16302.00, // Lv95/Lv95+/Lv100
        16302.00, // Lv95/Lv95+/Lv100
        16929.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        20.67, 53.61, 71.33, 106.74, 119.33, 137.29, 154.08, 172.22, 184.81, 203.13, 215.72,
        234.23, 246.82, 265.49, 265.49, 276.11, // Lv95/Lv95+/Lv100
        276.11, // Lv95/Lv95+/Lv100
        286.73, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        48.88, 126.80, 168.71, 252.44, 282.22, 324.70, 364.41, 407.32, 437.10, 480.41, 510.19,
        553.99, 583.77, 627.91, 627.91, 653.03, // Lv95/Lv95+/Lv100
        653.03, // Lv95/Lv95+/Lv100
        678.14, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Hp(0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "拳闘術",
            hits: &[
                DEHYA_NORMAL_1,
                DEHYA_NORMAL_2,
                DEHYA_NORMAL_3,
                DEHYA_NORMAL_4,
            ],
            charged: &[DEHYA_CHARGED_SPINNING, DEHYA_CHARGED_FINAL],
            plunging: &[DEHYA_PLUNGE, DEHYA_PLUNGE_LOW, DEHYA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "熔鉄流獄",
            scalings: &[
                DEHYA_SKILL_INDOMITABLE,
                DEHYA_SKILL_RANGE,
                DEHYA_SKILL_FIELD,
            ],
        },
        elemental_burst: TalentData {
            name: "炎獅子の噛み付き",
            scalings: &[DEHYA_BURST_FIST, DEHYA_BURST_KICK],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
    passive_scalings: &[],
    scaling_modifiers: &[],
};
