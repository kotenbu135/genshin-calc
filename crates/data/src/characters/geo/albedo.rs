use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Albedo
// =============================================================================

// -- Normal Attack: 西風剣術・白 (Favonius Bladework - Weiss) -- Physical --

const ALBEDO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3674, 0.3973, 0.4273, 0.4700, 0.4999, 0.5341, 0.5811, 0.6281, 0.6751, 0.7264, 0.7777,
        0.8290, 0.8803, 0.9316, 0.9829,
    ],
    dynamic_bonus: None,
};

const ALBEDO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3674, 0.3973, 0.4273, 0.4700, 0.4999, 0.5341, 0.5811, 0.6281, 0.6751, 0.7264, 0.7777,
        0.8290, 0.8803, 0.9316, 0.9829,
    ],
    dynamic_bonus: None,
};

const ALBEDO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4745, 0.5132, 0.5518, 0.6070, 0.6456, 0.6898, 0.7503, 0.8109, 0.8715, 0.9379, 1.0044,
        1.0708, 1.1372, 1.2037, 1.2701,
    ],
    dynamic_bonus: None,
};

const ALBEDO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4975, 0.5380, 0.5786, 0.6365, 0.6770, 0.7233, 0.7868, 0.8504, 0.9139, 0.9836, 1.0532,
        1.1228, 1.1925, 1.2621, 1.3317,
    ],
    dynamic_bonus: None,
};

const ALBEDO_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6207, 0.6712, 0.7218, 0.7940, 0.8445, 0.9022, 0.9815, 1.0609, 1.1402, 1.2269, 1.3137,
        1.4004, 1.4872, 1.5739, 1.6607,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Physical --

const ALBEDO_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4730, 0.5115, 0.5500, 0.6050, 0.6435, 0.6875, 0.7480, 0.8085, 0.8690, 0.9350, 1.0010,
        1.0670, 1.1330, 1.1990, 1.2650,
    ],
    dynamic_bonus: None,
};

const ALBEDO_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6020, 0.6510, 0.7000, 0.7700, 0.8190, 0.8750, 0.9520, 1.0290, 1.1060, 1.1900, 1.2740,
        1.3580, 1.4420, 1.5260, 1.6100,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const ALBEDO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
};

const ALBEDO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5271, 2.7055,
        2.8840, 3.0624, 3.2409, 3.4193,
    ],
    dynamic_bonus: None,
};

const ALBEDO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6021, 3.8249, 4.0478, 4.2706,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: 創生術・擬似陽華 (Abiogenesis: Solar Isotoma) -- Geo, DEF scaling --

const ALBEDO_SKILL_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.3040, 1.4018, 1.4996, 1.6300, 1.7278, 1.8256, 1.9560, 2.0864, 2.2168, 2.3472, 2.4776,
        2.6080, 2.7710, 2.9340, 3.0970,
    ],
    dynamic_bonus: None,
};

const ALBEDO_SKILL_TRANSIENT_BLOSSOM: TalentScaling = TalentScaling {
    name: "刹那の花ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        1.3360, 1.4362, 1.5364, 1.6700, 1.7702, 1.8704, 2.0040, 2.1376, 2.2712, 2.4048, 2.5384,
        2.6720, 2.8390, 3.0060, 3.1730,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 誕生式・大地の潮 (Rite of Progeniture: Tectonic Tide) -- Geo --

const ALBEDO_BURST_DAMAGE: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        3.6720, 3.9474, 4.2228, 4.5900, 4.8654, 5.1408, 5.5080, 5.8752, 6.2424, 6.6096, 6.9768,
        7.3440, 7.8030, 8.2620, 8.7210,
    ],
    dynamic_bonus: None,
};

const ALBEDO_BURST_FATAL_BLOSSOM: TalentScaling = TalentScaling {
    name: "生滅の花ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.7200, 0.7740, 0.8280, 0.9000, 0.9540, 1.0080, 1.0800, 1.1520, 1.2240, 1.2960, 1.3680,
        1.4400, 1.5300, 1.6200, 1.7100,
    ],
    dynamic_bonus: None,
};

pub const ALBEDO: CharacterData = CharacterData {
    id: "albedo",
    name: "Albedo",
    element: Element::Geo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [
        1030.00, 2671.00, 3554.00, 5317.00, 5944.00, 6839.00, 7675.00, 8579.00, 9207.00, 10119.00,
        10746.00, 11669.00, 12296.00, 13226.00, 13226.00, 13755.04, // Lv95/Lv95+/Lv100
        13755.04, // Lv95/Lv95+/Lv100
        14284.08, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        19.55, 50.72, 67.48, 100.97, 112.88, 129.87, 145.75, 162.91, 174.82, 192.15, 204.06,
        221.57, 233.48, 251.14, 251.14, 261.19, // Lv95/Lv95+/Lv100
        261.19, // Lv95/Lv95+/Lv100
        271.23, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        68.21, 176.93, 235.41, 352.25, 393.80, 453.07, 508.47, 568.36, 609.91, 670.34, 711.90,
        773.01, 814.56, 876.15, 876.15, 911.20, // Lv95/Lv95+/Lv100
        911.20, // Lv95/Lv95+/Lv100
        946.24, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Geo, 0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "西風剣術・白",
            hits: &[
                ALBEDO_NORMAL_1,
                ALBEDO_NORMAL_2,
                ALBEDO_NORMAL_3,
                ALBEDO_NORMAL_4,
                ALBEDO_NORMAL_5,
            ],
            charged: &[ALBEDO_CHARGED_1, ALBEDO_CHARGED_2],
            plunging: &[ALBEDO_PLUNGE, ALBEDO_PLUNGE_LOW, ALBEDO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "創生術・擬似陽華",
            scalings: &[ALBEDO_SKILL_DAMAGE, ALBEDO_SKILL_TRANSIENT_BLOSSOM],
        },
        elemental_burst: TalentData {
            name: "誕生式・大地の潮",
            scalings: &[ALBEDO_BURST_DAMAGE, ALBEDO_BURST_FATAL_BLOSSOM],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
