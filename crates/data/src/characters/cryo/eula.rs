use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Favonius Bladework - Edel -- Physical --

const EULA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8973, 0.9704, 1.0434, 1.1477, 1.2208, 1.3043, 1.4190, 1.5338, 1.6486, 1.7738, 1.9172,
        2.0860, 2.2547, 2.4234, 2.6075,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EULA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9355, 1.0117, 1.0878, 1.1966, 1.2727, 1.3598, 1.4792, 1.5988, 1.7183, 1.8490, 1.9988,
        2.1747, 2.3507, 2.5265, 2.7184,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EULA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5680, 0.6142, 0.6605, 0.7265, 0.7727, 0.8256, 0.8982, 0.9709, 1.0435, 1.1228, 1.2136,
        1.3204, 1.4272, 1.5340, 1.6505,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EULA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1264, 1.2181, 1.3098, 1.4408, 1.5325, 1.6373, 1.7813, 1.9254, 2.0695, 2.2267, 2.4068,
        2.6186, 2.8304, 3.0421, 3.2732,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EULA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7183, 0.7768, 0.8353, 0.9188, 0.9773, 1.0441, 1.1360, 1.2278, 1.3197, 1.4199, 1.5348,
        1.6699, 1.8049, 1.9400, 2.0874,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const EULA_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6880, 0.7440, 0.8000, 0.8800, 0.9360, 1.0000, 1.0880, 1.1760, 1.2640, 1.3600, 1.4700,
        1.5992, 1.7285, 1.8577, 1.9990,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EULA_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2440, 1.3452, 1.4465, 1.5912, 1.6924, 1.8081, 1.9673, 2.1265, 2.2857, 2.4593, 2.6580,
        2.8918, 3.1257, 3.3595, 3.6148,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const EULA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5937,
        1.7339, 1.8741, 2.0144, 2.1674,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EULA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1867,
        3.4671, 3.7475, 4.0279, 4.3338,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EULA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9460, 3.1842, 3.4225, 3.6825, 3.9803,
        4.3306, 4.6808, 5.0311, 5.4132,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: Icetide Vortex -- Cryo --

const EULA_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "短押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.4640, 1.5738, 1.6836, 1.8300, 1.9398, 2.0496, 2.1960, 2.3424, 2.4888, 2.6352, 2.7816,
        2.9280, 3.1110, 3.2940, 3.4770,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EULA_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        2.4560, 2.6402, 2.8244, 3.0700, 3.2542, 3.4384, 3.6840, 3.9296, 4.1752, 4.4208, 4.6664,
        4.9120, 5.2190, 5.5260, 5.8330,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EULA_SKILL_ICEWHIRL: TalentScaling = TalentScaling {
    name: "氷渦旋ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.9600, 1.0320, 1.1040, 1.2000, 1.2720, 1.3440, 1.4400, 1.5360, 1.6320, 1.7280, 1.8240,
        1.9200, 2.0400, 2.1600, 2.2800,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: Glacial Illumination -- Physical (Lightfall Sword) --

const EULA_BURST_SLASH: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        2.4560, 2.6402, 2.8244, 3.0700, 3.2542, 3.4384, 3.6840, 3.9296, 4.1752, 4.4208, 4.6664,
        4.9120, 5.2190, 5.5260, 5.8330,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EULA_BURST_LIGHTFALL_BASE: TalentScaling = TalentScaling {
    name: "光臨の剣基礎ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        3.6705, 3.9692, 4.2680, 4.6948, 4.9936, 5.3350, 5.8045, 6.2740, 6.7434, 7.2556, 7.8425,
        8.5326, 9.2227, 9.9129, 10.6657,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const EULA_BURST_LIGHTFALL_STACK: TalentScaling = TalentScaling {
    name: "光臨の剣スタックダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7499, 0.8110, 0.8720, 0.9592, 1.0202, 1.0900, 1.1859, 1.2818, 1.3778, 1.4824, 1.6023,
        1.7433, 1.8843, 2.0253, 2.1791,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const EULA: CharacterData = CharacterData {
    id: "eula",
    name: "Eula",
    element: Element::Cryo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [
        1030.00, 2671.00, 3554.00, 5317.00, 5944.00, 6839.00, 7675.00, 8579.00, 9207.00, 10119.00,
        10746.00, 11669.00, 12296.00, 13226.00, 13226.00, 13755.04, // Lv95/Lv95+/Lv100
        13755.04, // Lv95/Lv95+/Lv100
        14284.08, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        26.63, 69.07, 91.90, 137.51, 153.73, 176.87, 198.49, 221.87, 238.09, 261.68, 277.90,
        301.76, 317.98, 342.03, 342.03, 355.71, // Lv95/Lv95+/Lv100
        355.71, // Lv95/Lv95+/Lv100
        369.39, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        58.45, 151.63, 201.75, 301.88, 337.49, 388.29, 435.77, 487.09, 522.71, 574.50, 610.11,
        662.48, 698.09, 750.88, 750.88, 780.92, // Lv95/Lv95+/Lv100
        780.92, // Lv95/Lv95+/Lv100
        810.95, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "西風剣術・エーデル",
            hits: &[
                EULA_NORMAL_1,
                EULA_NORMAL_2,
                EULA_NORMAL_3,
                EULA_NORMAL_4,
                EULA_NORMAL_5,
            ],
            charged: &[EULA_CHARGED_SPINNING, EULA_CHARGED_FINAL],
            plunging: &[EULA_PLUNGE, EULA_PLUNGE_LOW, EULA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "氷潮の渦",
            scalings: &[EULA_SKILL_PRESS, EULA_SKILL_HOLD, EULA_SKILL_ICEWHIRL],
        },
        elemental_burst: TalentData {
            name: "氷浪の光",
            scalings: &[
                EULA_BURST_SLASH,
                EULA_BURST_LIGHTFALL_BASE,
                EULA_BURST_LIGHTFALL_STACK,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
    passive_scalings: &[],
};

// =============================================================================
// Freminet
