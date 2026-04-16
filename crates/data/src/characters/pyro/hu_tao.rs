use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Hu Tao
// =============================================================================

// -- Normal Attack: 往生秘伝槍法 (Secret Spear of Wangsheng) -- Physical --

const HU_TAO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4689, 0.5008, 0.5328, 0.5754, 0.6074, 0.6447, 0.6926, 0.7406, 0.7885, 0.8365, 0.8844,
        0.9324, 0.9804, 1.0283, 1.0763,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const HU_TAO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4825, 0.5154, 0.5483, 0.5922, 0.6251, 0.6635, 0.7128, 0.7622, 0.8115, 0.8609, 0.9102,
        0.9596, 1.0089, 1.0583, 1.1076,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const HU_TAO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6105, 0.6521, 0.6938, 0.7493, 0.7909, 0.8394, 0.9019, 0.9643, 1.0268, 1.0892, 1.1516,
        1.2141, 1.2765, 1.3389, 1.4014,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const HU_TAO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6564, 0.7012, 0.7459, 0.8056, 0.8503, 0.9026, 0.9697, 1.0368, 1.1040, 1.1711, 1.2382,
        1.3054, 1.3725, 1.4396, 1.5068,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const HU_TAO_NORMAL_5A: TalentScaling = TalentScaling {
    name: "5段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3327, 0.3554, 0.3781, 0.4084, 0.4310, 0.4575, 0.4915, 0.5256, 0.5596, 0.5936, 0.6277,
        0.6617, 0.6957, 0.7298, 0.7638,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const HU_TAO_NORMAL_5B: TalentScaling = TalentScaling {
    name: "5段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3520, 0.3760, 0.4000, 0.4320, 0.4560, 0.4840, 0.5200, 0.5560, 0.5920, 0.6280, 0.6640,
        0.7000, 0.7360, 0.7720, 0.8080,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const HU_TAO_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8596, 0.9182, 0.9768, 1.0549, 1.1136, 1.1819, 1.2698, 1.3578, 1.4457, 1.5336, 1.6215,
        1.7094, 1.7973, 1.8852, 1.9731,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const HU_TAO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.3596, 1.4523, 1.5450, 1.6686, 1.7613, 1.8695, 2.0085, 2.1476, 2.2866, 2.4257, 2.5647,
        2.7038, 2.8428, 2.9819, 3.1209,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const HU_TAO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6542, 0.6988, 0.7434, 0.8029, 0.8475, 0.8995, 0.9664, 1.0333, 1.1002, 1.1671, 1.2340,
        1.3010, 1.3679, 1.4348, 1.5017,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const HU_TAO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.3081, 1.3973, 1.4865, 1.6054, 1.6946, 1.7986, 1.9324, 2.0662, 2.2000, 2.3338, 2.4676,
        2.6013, 2.7351, 2.8689, 3.0027,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const HU_TAO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.6339, 1.7453, 1.8567, 2.0052, 2.1166, 2.2466, 2.4137, 2.5808, 2.7479, 2.9150, 3.0821,
        3.2492, 3.4163, 3.5834, 3.7505,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 蝶導来世 (Guide to Afterlife) -- Pyro --

const HU_TAO_SKILL_BLOOD_BLOSSOM: TalentScaling = TalentScaling {
    name: "血梅香ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Pyro),
    values: [
        0.6400, 0.6880, 0.7360, 0.8000, 0.8480, 0.8960, 0.9600, 1.0240, 1.0880, 1.1520, 1.2160,
        1.2800, 1.3600, 1.4400, 1.5200,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 安神秘法 (Spirit Soother) -- Pyro --

const HU_TAO_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        3.0327, 3.2143, 3.3959, 3.6320, 3.8136, 3.9952, 4.2313, 4.4674, 4.7034, 4.9395, 5.1756,
        5.4117, 5.6478, 5.8838, 6.1199,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const HU_TAO_BURST_LOW_HP: TalentScaling = TalentScaling {
    name: "低HPスキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        3.7909, 4.0179, 4.2449, 4.5400, 4.7670, 4.9940, 5.2891, 5.5842, 5.8793, 6.1744, 6.4695,
        6.7646, 7.0597, 7.3548, 7.6499,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const HU_TAO: CharacterData = CharacterData {
    id: "hu_tao",
    name: "Hu Tao",
    element: Element::Pyro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [
        1211.00, 3141.00, 4179.00, 6253.00, 6990.00, 8042.00, 9026.00, 10089.00, 10826.00,
        11899.00, 12637.00, 13721.00, 14459.00, 15552.00, 15552.00,
        16174.08, // Lv95/Lv95+/Lv100
        16174.08, // Lv95/Lv95+/Lv100
        16796.16, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        8.29, 21.49, 28.60, 42.79, 47.84, 55.04, 61.77, 69.04, 74.09, 81.43, 86.48, 93.90, 98.95,
        106.43, 106.43, 110.69, // Lv95/Lv95+/Lv100
        110.69, // Lv95/Lv95+/Lv100
        114.94, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        68.21, 176.93, 235.41, 352.25, 393.80, 453.07, 508.47, 568.36, 609.91, 670.34, 711.90,
        773.01, 814.56, 876.15, 876.15, 911.20, // Lv95/Lv95+/Lv100
        911.20, // Lv95/Lv95+/Lv100
        946.24, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "往生秘伝槍法",
            hits: &[
                HU_TAO_NORMAL_1,
                HU_TAO_NORMAL_2,
                HU_TAO_NORMAL_3,
                HU_TAO_NORMAL_4,
                HU_TAO_NORMAL_5A,
                HU_TAO_NORMAL_5B,
                HU_TAO_NORMAL_6,
            ],
            charged: &[HU_TAO_CHARGED],
            plunging: &[HU_TAO_PLUNGE, HU_TAO_PLUNGE_LOW, HU_TAO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "蝶導来世",
            scalings: &[HU_TAO_SKILL_BLOOD_BLOSSOM],
        },
        elemental_burst: TalentData {
            name: "安神秘法",
            scalings: &[HU_TAO_BURST, HU_TAO_BURST_LOW_HP],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
    scaling_modifiers: &[],
};
