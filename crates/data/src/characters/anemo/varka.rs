use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

const VARKA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6546, 0.7079, 0.7612, 0.8373, 0.8906, 0.9514, 1.0352, 1.1189, 1.2026, 1.2940, 1.3853,
        1.4766, 1.5680, 1.6593, 1.7507,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_NORMAL_2A: TalentScaling = TalentScaling {
    name: "2段ダメージA",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2399, 0.2594, 0.2789, 0.3068, 0.3264, 0.3487, 0.3794, 0.4100, 0.4407, 0.4742, 0.5077,
        0.5411, 0.5746, 0.6081, 0.6416,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_NORMAL_2B: TalentScaling = TalentScaling {
    name: "2段ダメージB",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4455, 0.4818, 0.5180, 0.5698, 0.6061, 0.6475, 0.7045, 0.7615, 0.8185, 0.8806, 0.9428,
        1.0050, 1.0671, 1.1293, 1.1915,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージA",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3244, 0.3508, 0.3772, 0.4149, 0.4413, 0.4715, 0.5129, 0.5544, 0.5959, 0.6412, 0.6864,
        0.7317, 0.7770, 0.8222, 0.8675,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージB",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6024, 0.6514, 0.7005, 0.7705, 0.8195, 0.8756, 0.9526, 1.0297, 1.1067, 1.1908, 1.2748,
        1.3589, 1.4429, 1.5270, 1.6110,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_NORMAL_4A: TalentScaling = TalentScaling {
    name: "4段ダメージA",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5543, 0.5994, 0.6446, 0.7090, 0.7541, 0.8057, 0.8766, 0.9475, 1.0184, 1.0957, 1.1731,
        1.2504, 1.3278, 1.4051, 1.4825,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_NORMAL_4B: TalentScaling = TalentScaling {
    name: "4段ダメージB",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2985, 0.3228, 0.3471, 0.3818, 0.4061, 0.4338, 0.4720, 0.5102, 0.5484, 0.5900, 0.6317,
        0.6733, 0.7150, 0.7566, 0.7983,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_NORMAL_5A: TalentScaling = TalentScaling {
    name: "5段ダメージA",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6975, 0.7543, 0.8110, 0.8921, 0.9489, 1.0138, 1.1030, 1.1922, 1.2815, 1.3788, 1.4761,
        1.5734, 1.6708, 1.7681, 1.8654,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_NORMAL_5B: TalentScaling = TalentScaling {
    name: "5段ダメージB",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3756, 0.4061, 0.4367, 0.4804, 0.5110, 0.5459, 0.5939, 0.6420, 0.6900, 0.7424, 0.7948,
        0.8472, 0.8996, 0.9520, 1.0044,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_CHARGED_A: TalentScaling = TalentScaling {
    name: "重撃ダメージA",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8564, 0.9261, 0.9958, 1.0954, 1.1651, 1.2448, 1.3543, 1.4638, 1.5734, 1.6929, 1.8124,
        1.9319, 2.0513, 2.1708, 2.2903,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_CHARGED_B: TalentScaling = TalentScaling {
    name: "重撃ダメージB",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4611, 0.4987, 0.5362, 0.5898, 0.6274, 0.6702, 0.7292, 0.7882, 0.8472, 0.9115, 0.9759,
        1.0402, 1.1046, 1.1689, 1.2333,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_PLUNGE: TalentScaling = TalentScaling {
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

const VARKA_PLUNGE_LOW: TalentScaling = TalentScaling {
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

const VARKA_PLUNGE_HIGH: TalentScaling = TalentScaling {
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

const VARKA_SKILL_TAP: TalentScaling = TalentScaling {
    name: "スキルDMG",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.784, 2.9928, 3.2016, 3.48, 3.6888, 3.8976, 4.176, 4.4544, 4.7328, 5.0112, 5.2896, 5.568,
        5.916, 6.264, 6.612,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_SD_1: TalentScaling = TalentScaling {
    name: "S&D 1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.8182, 0.8848, 0.9514, 1.0466, 1.1132, 1.1893, 1.2940, 1.3986, 1.5033, 1.6175, 1.7316,
        1.8458, 1.9600, 2.0742, 2.1883,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_SD_2A: TalentScaling = TalentScaling {
    name: "S&D 2段ダメージA",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.2999, 0.3243, 0.3487, 0.3835, 0.4079, 0.4358, 0.4742, 0.5125, 0.5509, 0.5927, 0.6346,
        0.6764, 0.7183, 0.7601, 0.8019,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_SD_2B: TalentScaling = TalentScaling {
    name: "S&D 2段ダメージB",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.5569, 0.6022, 0.6475, 0.7123, 0.7576, 0.8094, 0.8806, 0.9519, 1.0231, 1.1008, 1.1785,
        1.2562, 1.3339, 1.4116, 1.4893,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_SD_3A: TalentScaling = TalentScaling {
    name: "S&D 3段ダメージA",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.4055, 0.4385, 0.4715, 0.5186, 0.5516, 0.5893, 0.6412, 0.6930, 0.7449, 0.8015, 0.8581,
        0.9146, 0.9712, 1.0278, 1.0844,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_SD_3B: TalentScaling = TalentScaling {
    name: "S&D 3段ダメージB",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.7530, 0.8143, 0.8756, 0.9631, 1.0244, 1.0945, 1.1908, 1.2871, 1.3834, 1.4885, 1.5935,
        1.6986, 1.8037, 1.9087, 2.0138,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_SD_4A: TalentScaling = TalentScaling {
    name: "S&D 4段ダメージA",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.6929, 0.7493, 0.8057, 0.8863, 0.9427, 1.0071, 1.0957, 1.1844, 1.2730, 1.3697, 1.4664,
        1.5630, 1.6597, 1.7564, 1.8531,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_SD_4B: TalentScaling = TalentScaling {
    name: "S&D 4段ダメージB",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.3731, 0.4035, 0.4338, 0.4772, 0.5076, 0.5423, 0.5900, 0.6377, 0.6855, 0.7375, 0.7896,
        0.8416, 0.8937, 0.9458, 0.9978,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_SD_5A: TalentScaling = TalentScaling {
    name: "S&D 5段ダメージA",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.8719, 0.9428, 1.0138, 1.1152, 1.1862, 1.2673, 1.3788, 1.4903, 1.6018, 1.7235, 1.8451,
        1.9668, 2.0884, 2.2101, 2.3318,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_SD_5B: TalentScaling = TalentScaling {
    name: "S&D 5段ダメージB",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.4695, 0.5077, 0.5459, 0.6005, 0.6387, 0.6824, 0.7424, 0.8025, 0.8625, 0.9280, 0.9935,
        1.0590, 1.1245, 1.1901, 1.2556,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_SD_CHARGED_A: TalentScaling = TalentScaling {
    name: "S&D 重撃ダメージA",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.0705, 1.1576, 1.2448, 1.3692, 1.4564, 1.5559, 1.6929, 1.8298, 1.9667, 2.1161, 2.2654,
        2.4148, 2.5642, 2.7136, 2.8629,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_SD_CHARGED_B: TalentScaling = TalentScaling {
    name: "S&D 重撃ダメージB",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.5764, 0.6233, 0.6702, 0.7373, 0.7842, 0.8378, 0.9115, 0.9853, 1.0590, 1.1394, 1.2199,
        1.3003, 1.3807, 1.4611, 1.5416,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_FOUR_WINDS_A: TalentScaling = TalentScaling {
    name: "Four Winds' Ascension A",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.7576, 1.8894, 2.0212, 2.1970, 2.3288, 2.4606, 2.6364, 2.8122, 2.9879, 3.1637, 3.3394,
        3.5152, 3.7349, 3.9546, 4.1743,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_FOUR_WINDS_B: TalentScaling = TalentScaling {
    name: "Four Winds' Ascension B",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.9464, 1.0174, 1.0884, 1.1830, 1.2540, 1.3250, 1.4196, 1.5142, 1.6089, 1.7035, 1.7982,
        1.8928, 2.0111, 2.1294, 2.2477,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_AZURE_A: TalentScaling = TalentScaling {
    name: "Azure Devour A (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.936, 1.0062, 1.0764, 1.17, 1.2402, 1.3104, 1.404, 1.4976, 1.5912, 1.6848, 1.7784, 1.872,
        1.989, 2.106, 2.223,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_AZURE_B: TalentScaling = TalentScaling {
    name: "Azure Devour B (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.504, 0.5418, 0.5796, 0.63, 0.6678, 0.7056, 0.756, 0.8064, 0.8568, 0.9072, 0.9576, 1.008,
        1.071, 1.134, 1.197,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_BURST_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        3.3696, 3.6223, 3.8750, 4.2120, 4.4647, 4.7174, 5.0544, 5.3914, 5.7283, 6.0653, 6.4022,
        6.7392, 7.1604, 7.5816, 8.0028,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const VARKA_BURST_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.8144, 1.9505, 2.0866, 2.268, 2.4041, 2.5402, 2.7216, 2.903, 3.0845, 3.2659, 3.4474,
        3.6288, 3.8556, 4.0824, 4.3092,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const VARKA: CharacterData = CharacterData {
    id: "varka",
    name: "Varka",
    element: Element::Anemo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [
        982.00, 2547.00, 3389.00, 5071.00, 5669.00, 6523.00, 7320.00, 8182.00, 8780.00, 9650.00,
        10249.00, 11128.00, 11727.00, 12613.00, 12613.00, 13117.52, // Lv95/Lv95+/Lv100
        13117.52, // Lv95/Lv95+/Lv100
        13622.04, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        27.46, 71.24, 94.79, 141.84, 158.57, 182.43, 204.74, 228.85, 245.58, 269.92, 286.65,
        311.26, 327.99, 352.79, 352.79, 366.90, // Lv95/Lv95+/Lv100
        366.90, // Lv95/Lv95+/Lv100
        381.01, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        61.92, 160.63, 213.73, 319.80, 357.53, 411.34, 461.64, 516.01, 553.74, 608.60, 646.33,
        701.81, 739.54, 795.45, 795.45, 827.27, // Lv95/Lv95+/Lv100
        827.27, // Lv95/Lv95+/Lv100
        859.09, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "Favonius Bladework: Dancing Radiance",
            hits: &[
                VARKA_NORMAL_1,
                VARKA_NORMAL_2A,
                VARKA_NORMAL_2B,
                VARKA_NORMAL_3A,
                VARKA_NORMAL_3B,
                VARKA_NORMAL_4A,
                VARKA_NORMAL_4B,
                VARKA_NORMAL_5A,
                VARKA_NORMAL_5B,
            ],
            charged: &[VARKA_CHARGED_A, VARKA_CHARGED_B],
            plunging: &[VARKA_PLUNGE, VARKA_PLUNGE_LOW, VARKA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "Windbound Execution",
            scalings: &[
                VARKA_SKILL_TAP,
                VARKA_SD_1,
                VARKA_SD_2A,
                VARKA_SD_2B,
                VARKA_SD_3A,
                VARKA_SD_3B,
                VARKA_SD_4A,
                VARKA_SD_4B,
                VARKA_SD_5A,
                VARKA_SD_5B,
                VARKA_SD_CHARGED_A,
                VARKA_SD_CHARGED_B,
                VARKA_FOUR_WINDS_A,
                VARKA_FOUR_WINDS_B,
                VARKA_AZURE_A,
                VARKA_AZURE_B,
            ],
        },
        elemental_burst: TalentData {
            name: "Northwind Avatar",
            scalings: &[VARKA_BURST_1, VARKA_BURST_2],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
};
