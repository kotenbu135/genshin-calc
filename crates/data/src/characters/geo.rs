#![allow(clippy::approx_constant)]
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
};

const ALBEDO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3674, 0.3973, 0.4273, 0.4700, 0.4999, 0.5341, 0.5811, 0.6281, 0.6751, 0.7264, 0.7777,
        0.8290, 0.8803, 0.9316, 0.9829,
    ],
};

const ALBEDO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4745, 0.5132, 0.5518, 0.6070, 0.6456, 0.6898, 0.7503, 0.8109, 0.8715, 0.9379, 1.0044,
        1.0708, 1.1372, 1.2037, 1.2701,
    ],
};

const ALBEDO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4975, 0.5380, 0.5786, 0.6365, 0.6770, 0.7233, 0.7868, 0.8504, 0.9139, 0.9836, 1.0532,
        1.1228, 1.1925, 1.2621, 1.3317,
    ],
};

const ALBEDO_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6207, 0.6712, 0.7218, 0.7940, 0.8445, 0.9022, 0.9815, 1.0609, 1.1402, 1.2269, 1.3137,
        1.4004, 1.4872, 1.5739, 1.6607,
    ],
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
};

const ALBEDO_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6020, 0.6510, 0.7000, 0.7700, 0.8190, 0.8750, 0.9520, 1.0290, 1.1060, 1.1900, 1.2740,
        1.3580, 1.4420, 1.5260, 1.6100,
    ],
};

// -- Plunging Attack -- Physical --

const ALBEDO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 0.1011, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const ALBEDO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5271, 2.7055,
        2.8840, 3.0624, 3.2409, 3.4193,
    ],
};

const ALBEDO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6021, 3.8249, 4.0478, 4.2706,
    ],
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
};

const ALBEDO_SKILL_TRANSIENT_BLOSSOM: TalentScaling = TalentScaling {
    name: "刹那の花ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        1.3360, 1.4362, 1.5364, 1.6700, 1.7702, 1.8704, 2.0040, 2.1376, 2.2712, 2.4048, 2.5384,
        2.6720, 2.8390, 3.0060, 3.1730,
    ],
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
};

const ALBEDO_BURST_FATAL_BLOSSOM: TalentScaling = TalentScaling {
    name: "生滅の花ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.7200, 0.7740, 0.8280, 0.9000, 0.9540, 1.0080, 1.0800, 1.1520, 1.2240, 1.2960, 1.3680,
        1.4400, 1.5300, 1.6200, 1.7100,
    ],
};

pub const ALBEDO: CharacterData = CharacterData {
    id: "albedo",
    name: "Albedo",
    element: Element::Geo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [1030.0, 10309.0, 11435.0, 12296.0],
    base_atk: [20.0, 200.0, 222.0, 233.0],
    base_def: [68.0, 680.0, 755.0, 815.0],
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
};

// =============================================================================
// Chiori
// =============================================================================

// -- Normal Attack: 裁錦キ法 (Weaving Blade) -- Physical --

const CHIORI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4941, 0.5343, 0.5745, 0.6320, 0.6722, 0.7181, 0.7813, 0.8445, 0.9077, 0.9766, 1.0456,
        1.1145, 1.1835, 1.2524, 1.3214,
    ],
};

const CHIORI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4685, 0.5066, 0.5448, 0.5993, 0.6374, 0.6810, 0.7409, 0.8009, 0.8609, 0.9262, 0.9914,
        1.0567, 1.1220, 1.1873, 1.2525,
    ],
};

const CHIORI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3048, 0.3296, 0.3545, 0.3899, 0.4148, 0.4431, 0.4821, 0.5211, 0.5601, 0.6026, 0.6451,
        0.6876, 0.7301, 0.7726, 0.8151,
    ],
};

const CHIORI_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7513, 0.8124, 0.8735, 0.9609, 1.0220, 1.0919, 1.1881, 1.2843, 1.3805, 1.4852, 1.5898,
        1.6944, 1.7991, 1.9037, 2.0083,
    ],
};

// -- Charged Attack -- Physical --

const CHIORI_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5425, 0.5867, 0.6310, 0.6941, 0.7383, 0.7887, 0.8582, 0.9277, 0.9972, 1.0728, 1.1483,
        1.2239, 1.2995, 1.3750, 1.4506,
    ],
};

const CHIORI_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5425, 0.5867, 0.6310, 0.6941, 0.7383, 0.7887, 0.8582, 0.9277, 0.9972, 1.0728, 1.1483,
        1.2239, 1.2995, 1.3750, 1.4506,
    ],
};

// -- Plunging Attack -- Physical --

const CHIORI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 0.1011, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const CHIORI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5271, 2.7055,
        2.8840, 3.0624, 3.2409, 3.4193,
    ],
};

const CHIORI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6021, 3.8249, 4.0478, 4.2706,
    ],
};

// -- Elemental Skill: 羽袖キ法・糸結 (Fluttering Hasode) -- Geo --

const CHIORI_SKILL_TAMOTO: TalentScaling = TalentScaling {
    name: "袂自動攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.8208, 0.8824, 0.9439, 1.0260, 1.0876, 1.1491, 1.2312, 1.3133, 1.3954, 1.4774, 1.5595,
        1.6416, 1.7442, 1.8468, 1.9494,
    ],
};

const CHIORI_SKILL_TURRET: TalentScaling = TalentScaling {
    name: "袂飛び道具ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.4880, 1.5996, 1.7112, 1.8600, 1.9716, 2.0832, 2.2320, 2.3808, 2.5296, 2.6784, 2.8272,
        2.9760, 3.1620, 3.3480, 3.5340,
    ],
};

const CHIORI_SKILL_UPWARD_SWEEP: TalentScaling = TalentScaling {
    name: "上拂スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.4928, 1.6048, 1.7168, 1.8660, 1.9780, 2.0899, 2.2392, 2.3885, 2.5378, 2.6870, 2.8363,
        2.9856, 3.1722, 3.3588, 3.5454,
    ],
};

// -- Elemental Burst: 二刀キ法・緋反 (Hiyoku: Twin Blades) -- Geo --

const CHIORI_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        2.5656, 2.7580, 2.9504, 3.2070, 3.3994, 3.5918, 3.8484, 4.1050, 4.3615, 4.6181, 4.8747,
        5.1312, 5.4519, 5.7726, 6.0933,
    ],
};

pub const CHIORI: CharacterData = CharacterData {
    id: "chiori",
    name: "Chiori",
    element: Element::Geo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [890.0, 8905.0, 9881.0, 10624.0],
    base_atk: [25.0, 252.0, 280.0, 301.0],
    base_def: [74.0, 745.0, 827.0, 888.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "裁錦キ法",
            hits: &[
                CHIORI_NORMAL_1,
                CHIORI_NORMAL_2,
                CHIORI_NORMAL_3,
                CHIORI_NORMAL_4,
            ],
            charged: &[CHIORI_CHARGED_1, CHIORI_CHARGED_2],
            plunging: &[CHIORI_PLUNGE, CHIORI_PLUNGE_LOW, CHIORI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "羽袖キ法・糸結",
            scalings: &[
                CHIORI_SKILL_TAMOTO,
                CHIORI_SKILL_TURRET,
                CHIORI_SKILL_UPWARD_SWEEP,
            ],
        },
        elemental_burst: TalentData {
            name: "二刀キ法・緋反",
            scalings: &[CHIORI_BURST],
        },
    },
};

// =============================================================================
// Gorou
// =============================================================================

// -- Normal Attack: 踏鞴ケ射法 (Ripping Fang Fletching) -- Physical --

const GOROU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3777, 0.4085, 0.4393, 0.4832, 0.5140, 0.5491, 0.5975, 0.6458, 0.6942, 0.7469, 0.7996,
        0.8523, 0.9049, 0.9576, 1.0103,
    ],
};

const GOROU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3715, 0.4018, 0.4320, 0.4752, 0.5054, 0.5400, 0.5875, 0.6350, 0.6826, 0.7344, 0.7862,
        0.8381, 0.8899, 0.9418, 0.9936,
    ],
};

const GOROU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4945, 0.5348, 0.5750, 0.6325, 0.6728, 0.7188, 0.7820, 0.8453, 0.9085, 0.9775, 1.0465,
        1.1155, 1.1845, 1.2535, 1.3225,
    ],
};

const GOROU_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5900, 0.6382, 0.6863, 0.7549, 0.8031, 0.8579, 0.9333, 1.0088, 1.0842, 1.1667, 1.2491,
        1.3316, 1.4140, 1.4965, 1.5789,
    ],
};

// -- Aimed Shot -- Geo (charged) --

const GOROU_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const GOROU_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

// -- Plunging Attack -- Physical --

const GOROU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const GOROU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const GOROU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 犬坂ケ方陣 (Inuzaka All-Round Defense) -- Geo --

const GOROU_SKILL_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.0720, 1.1524, 1.2328, 1.3400, 1.4204, 1.5008, 1.6080, 1.7152, 1.8224, 1.9296, 2.0368,
        2.1440, 2.2780, 2.4120, 2.5460,
    ],
};

// -- Elemental Burst: 獣牙キ突撃 (Juuga: Forward Unto Victory) -- Geo --

const GOROU_BURST_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.9822, 1.0559, 1.1296, 1.2278, 1.3014, 1.3751, 1.4733, 1.5716, 1.6698, 1.7680, 1.8663,
        1.9645, 2.0873, 2.2101, 2.3329,
    ],
};

const GOROU_BURST_CRYSTAL: TalentScaling = TalentScaling {
    name: "岩晶崩破ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.6132, 0.6592, 0.7052, 0.7665, 0.8126, 0.8586, 0.9199, 0.9812, 1.0425, 1.1038, 1.1651,
        1.2264, 1.3031, 1.3797, 1.4563,
    ],
};

pub const GOROU: CharacterData = CharacterData {
    id: "gorou",
    name: "Gorou",
    element: Element::Geo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Inazuma,
    base_hp: [802.0, 8481.0, 8907.0, 9570.0],
    base_atk: [15.0, 158.0, 166.0, 183.0],
    base_def: [54.0, 572.0, 601.0, 648.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Geo, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "踏鞴ケ射法",
            hits: &[
                GOROU_NORMAL_1,
                GOROU_NORMAL_2,
                GOROU_NORMAL_3,
                GOROU_NORMAL_4,
            ],
            charged: &[GOROU_AIMED, GOROU_AIMED_FULL],
            plunging: &[GOROU_PLUNGE, GOROU_PLUNGE_LOW, GOROU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "犬坂ケ方陣",
            scalings: &[GOROU_SKILL_DAMAGE],
        },
        elemental_burst: TalentData {
            name: "獣牙キ突撃",
            scalings: &[GOROU_BURST_DAMAGE, GOROU_BURST_CRYSTAL],
        },
    },
};

// =============================================================================
// Itto (Arataki Itto)
// =============================================================================

// -- Normal Attack: 喧嘩キ暴走 (Fight Club Legend) -- Physical --

const ITTO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7918, 0.8564, 0.9210, 1.0131, 1.0777, 1.1513, 1.2523, 1.3534, 1.4545, 1.5654, 1.6763,
        1.7872, 1.8981, 2.0090, 2.1199,
    ],
};

const ITTO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7637, 0.8260, 0.8883, 0.9771, 1.0394, 1.1104, 1.2079, 1.3054, 1.4029, 1.5099, 1.6168,
        1.7237, 1.8306, 1.9376, 2.0445,
    ],
};

const ITTO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9161, 0.9907, 1.0653, 1.1718, 1.2464, 1.3316, 1.4490, 1.5664, 1.6838, 1.8113, 1.9387,
        2.0661, 2.1935, 2.3210, 2.4484,
    ],
};

const ITTO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1723, 1.2679, 1.3634, 1.4997, 1.5952, 1.7042, 1.8541, 2.0040, 2.1539, 2.3176, 2.4813,
        2.6450, 2.8087, 2.9724, 3.1361,
    ],
};

// -- Charged Attack (Arataki Kesagiri) -- Physical --

const ITTO_CHARGED_SLASH: TalentScaling = TalentScaling {
    name: "荒瀧キ逆袈裟連斬ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9123, 0.9866, 1.0610, 1.1671, 1.2414, 1.3262, 1.4425, 1.5588, 1.6751, 1.8031, 1.9311,
        2.0591, 2.1872, 2.3152, 2.4432,
    ],
};

const ITTO_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "荒瀧キ逆袈裟終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.9092, 2.0647, 2.2203, 2.4423, 2.5978, 2.7755, 3.0197, 3.2638, 3.5080, 3.7746, 4.0412,
        4.3078, 4.5744, 4.8410, 5.1076,
    ],
};

// -- Plunging Attack -- Physical --

const ITTO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8195, 0.8862, 0.9530, 1.0483, 1.1150, 1.1912, 1.2960, 1.4008, 1.5057, 1.6200, 1.7344,
        1.8488, 1.9631, 2.0775, 2.1918,
    ],
};

const ITTO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.6387, 1.7722, 1.9058, 2.0964, 2.2299, 2.3822, 2.5916, 2.8010, 3.0104, 3.2395, 3.4686,
        3.6978, 3.9269, 4.1560, 4.3851,
    ],
};

const ITTO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        2.0474, 2.2142, 2.3810, 2.6191, 2.7859, 2.9762, 3.2378, 3.4993, 3.7609, 4.0472, 4.3335,
        4.6198, 4.9062, 5.1925, 5.4788,
    ],
};

// -- Elemental Skill: 魔殺キ絶技 (Masatsu Zetsugi: Akaushi Burst!) -- Geo --

const ITTO_SKILL_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        3.0720, 3.3024, 3.5328, 3.8400, 4.0704, 4.3008, 4.6080, 4.9152, 5.2224, 5.5296, 5.8368,
        6.1440, 6.5280, 6.9120, 7.2960,
    ],
};

// -- Elemental Burst: 最キ一番キ虎キ盛 (Royal Descent: Behold, Itto the Evil!) -- Geo infusion --

const ITTO_BURST_ATK_BONUS: TalentScaling = TalentScaling {
    name: "ATK追加 (DEF基準)",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.5760, 0.6192, 0.6624, 0.7200, 0.7632, 0.8064, 0.8640, 0.9216, 0.9792, 1.0368, 1.0944,
        1.1520, 1.2240, 1.2960, 1.3680,
    ],
};

pub const ITTO: CharacterData = CharacterData {
    id: "itto",
    name: "Arataki Itto",
    element: Element::Geo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [1001.0, 10012.0, 11105.0, 12858.0],
    base_atk: [18.0, 176.0, 195.0, 227.0],
    base_def: [75.0, 751.0, 833.0, 959.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "喧嘩キ暴走",
            hits: &[ITTO_NORMAL_1, ITTO_NORMAL_2, ITTO_NORMAL_3, ITTO_NORMAL_4],
            charged: &[ITTO_CHARGED_SLASH, ITTO_CHARGED_FINAL],
            plunging: &[ITTO_PLUNGE, ITTO_PLUNGE_LOW, ITTO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "魔殺キ絶技",
            scalings: &[ITTO_SKILL_DAMAGE],
        },
        elemental_burst: TalentData {
            name: "最キ一番キ虎キ盛",
            scalings: &[ITTO_BURST_ATK_BONUS],
        },
    },
};

// =============================================================================
// Kachina
// =============================================================================

// -- Normal Attack: 石紡ぎの拳 (Cragbiter) -- Physical --

const KACHINA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4940, 0.5342, 0.5744, 0.6318, 0.6720, 0.7180, 0.7811, 0.8443, 0.9074, 0.9764, 1.0454,
        1.1143, 1.1833, 1.2522, 1.3212,
    ],
};

const KACHINA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4466, 0.4829, 0.5193, 0.5712, 0.6076, 0.6491, 0.7064, 0.7636, 0.8208, 0.8830, 0.9451,
        1.0073, 1.0694, 1.1316, 1.1937,
    ],
};

const KACHINA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6505, 0.7034, 0.7563, 0.8319, 0.8848, 0.9454, 1.0285, 1.1117, 1.1948, 1.2857, 1.3765,
        1.4674, 1.5582, 1.6491, 1.7399,
    ],
};

// -- Charged Attack -- Physical --

const KACHINA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1266, 1.2186, 1.3107, 1.4417, 1.5338, 1.6384, 1.7829, 1.9275, 2.0721, 2.2287, 2.3852,
        2.5417, 2.6983, 2.8548, 3.0114,
    ],
};

// -- Plunging Attack -- Physical --

const KACHINA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 0.1011, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const KACHINA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5271, 2.7055,
        2.8840, 3.0624, 3.2409, 3.4193,
    ],
};

const KACHINA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6021, 3.8249, 4.0478, 4.2706,
    ],
};

// -- Elemental Skill: ゴゴキでゴー！(Go, Go Turbo Twirly!) -- Geo --

const KACHINA_SKILL_TURBO: TalentScaling = TalentScaling {
    name: "ゴゴキ回転ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.8768, 0.9426, 1.0083, 1.0960, 1.1618, 1.2275, 1.3152, 1.4029, 1.4906, 1.5782, 1.6659,
        1.7536, 1.8632, 1.9728, 2.0824,
    ],
};

const KACHINA_SKILL_DASH: TalentScaling = TalentScaling {
    name: "ゴゴキ突進ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.6400, 0.6880, 0.7360, 0.8000, 0.8480, 0.8960, 0.9600, 1.0240, 1.0880, 1.1520, 1.2160,
        1.2800, 1.3600, 1.4400, 1.5200,
    ],
};

// -- Elemental Burst: 蝕夜キ絶門（Time to Get Serious!） -- Geo --

const KACHINA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        3.8076, 4.0932, 4.3788, 4.7595, 5.0451, 5.3307, 5.7114, 6.0922, 6.4729, 6.8537, 7.2344,
        7.6152, 8.0912, 8.5671, 9.0431,
    ],
};

pub const KACHINA: CharacterData = CharacterData {
    id: "kachina",
    name: "Kachina",
    element: Element::Geo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Natlan,
    base_hp: [989.0, 10454.0, 10979.0, 11799.0],
    base_atk: [18.0, 186.0, 196.0, 210.0],
    base_def: [66.0, 698.0, 733.0, 788.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Geo, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "石紡ぎの拳",
            hits: &[KACHINA_NORMAL_1, KACHINA_NORMAL_2, KACHINA_NORMAL_3],
            charged: &[KACHINA_CHARGED],
            plunging: &[KACHINA_PLUNGE, KACHINA_PLUNGE_LOW, KACHINA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "ゴゴキでゴー！",
            scalings: &[KACHINA_SKILL_TURBO, KACHINA_SKILL_DASH],
        },
        elemental_burst: TalentData {
            name: "蝕夜キ絶門",
            scalings: &[KACHINA_BURST],
        },
    },
};

// =============================================================================
// Navia
// =============================================================================

// -- Normal Attack: 多様性を守る銃弾 (Blunt Refusal) -- Physical --

const NAVIA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9352, 1.0114, 1.0877, 1.1964, 1.2727, 1.3596, 1.4789, 1.5983, 1.7176, 1.8487, 1.9797,
        2.1108, 2.2418, 2.3729, 2.5039,
    ],
};

const NAVIA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8651, 0.9357, 1.0063, 1.1069, 1.1775, 1.2579, 1.3683, 1.4788, 1.5892, 1.7104, 1.8315,
        1.9527, 2.0738, 2.1950, 2.3161,
    ],
};

const NAVIA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3485, 0.3769, 0.4053, 0.4458, 0.4742, 0.5066, 0.5511, 0.5956, 0.6401, 0.6889, 0.7377,
        0.7865, 0.8353, 0.8841, 0.9329,
    ],
};

const NAVIA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.3334, 1.4420, 1.5506, 1.7057, 1.8143, 1.9383, 2.1088, 2.2794, 2.4499, 2.6360, 2.8222,
        3.0083, 3.1945, 3.3806, 3.5668,
    ],
};

// -- Charged Attack -- Physical --

const NAVIA_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6252, 0.6762, 0.7271, 0.7998, 0.8508, 0.9089, 0.9888, 1.0688, 1.1487, 1.2361, 1.3234,
        1.4107, 1.4981, 1.5854, 1.6727,
    ],
};

const NAVIA_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1309, 1.2231, 1.3152, 1.4468, 1.5389, 1.6441, 1.7887, 1.9333, 2.0779, 2.2359, 2.3939,
        2.5519, 2.7099, 2.8679, 3.0259,
    ],
};

// -- Plunging Attack -- Physical --

const NAVIA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9541, 1.0148, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
};

const NAVIA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const NAVIA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9459, 3.1841, 3.4223, 3.6824, 3.9424,
        4.2024, 4.4625, 4.7225, 4.9826,
    ],
};

// -- Elemental Skill: 大キラインの式典装弾 (Ceremonial Crystalshot) -- Geo --

const NAVIA_SKILL_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ (基礎)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        3.9480, 4.2441, 4.5402, 4.9350, 5.2311, 5.5272, 5.9220, 6.3168, 6.7116, 7.1064, 7.5012,
        7.8960, 8.3895, 8.8830, 9.3765,
    ],
};

const NAVIA_SKILL_CRYSTAL_SHRAPNEL: TalentScaling = TalentScaling {
    name: "結晶弾片ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.3600, 0.3870, 0.4140, 0.4500, 0.4770, 0.5040, 0.5400, 0.5760, 0.6120, 0.6480, 0.6840,
        0.7200, 0.7650, 0.8100, 0.8550,
    ],
};

// -- Elemental Burst: 裁判のキ光弾 (As the Sunlit Sky's Singing Salute) -- Geo --

const NAVIA_BURST_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.7524, 0.8088, 0.8653, 0.9405, 0.9970, 1.0534, 1.1286, 1.2038, 1.2791, 1.3543, 1.4296,
        1.5048, 1.5989, 1.6929, 1.7870,
    ],
};

const NAVIA_BURST_CANNON: TalentScaling = TalentScaling {
    name: "砲弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.4309, 0.4632, 0.4956, 0.5386, 0.5710, 0.6033, 0.6463, 0.6894, 0.7324, 0.7755, 0.8185,
        0.8616, 0.9155, 0.9693, 1.0232,
    ],
};

pub const NAVIA: CharacterData = CharacterData {
    id: "navia",
    name: "Navia",
    element: Element::Geo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [984.0, 9845.0, 10923.0, 11741.0],
    base_atk: [27.0, 272.0, 302.0, 325.0],
    base_def: [62.0, 617.0, 684.0, 736.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "多様性を守る銃弾",
            hits: &[
                NAVIA_NORMAL_1,
                NAVIA_NORMAL_2,
                NAVIA_NORMAL_3,
                NAVIA_NORMAL_4,
            ],
            charged: &[NAVIA_CHARGED_SPINNING, NAVIA_CHARGED_FINAL],
            plunging: &[NAVIA_PLUNGE, NAVIA_PLUNGE_LOW, NAVIA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "大キラインの式典装弾",
            scalings: &[NAVIA_SKILL_DAMAGE, NAVIA_SKILL_CRYSTAL_SHRAPNEL],
        },
        elemental_burst: TalentData {
            name: "裁判のキ光弾",
            scalings: &[NAVIA_BURST_DAMAGE, NAVIA_BURST_CANNON],
        },
    },
};

// =============================================================================
// Ningguang
// =============================================================================

// -- Normal Attack: 千金キ擲 (Sparkling Scatter) -- Geo (Catalyst) --

const NINGGUANG_NORMAL: TalentScaling = TalentScaling {
    name: "通常攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.2800, 0.3010, 0.3220, 0.3500, 0.3710, 0.3920, 0.4200, 0.4480, 0.4760, 0.5040, 0.5330,
        0.5712, 0.6094, 0.6475, 0.6857,
    ],
};

// -- Charged Attack -- Geo (Catalyst) --

const NINGGUANG_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.7408, 1.8714, 2.0019, 2.1760, 2.3066, 2.4371, 2.6112, 2.7853, 2.9594, 3.1334, 3.3133,
        3.5500, 3.7866, 4.0233, 4.2600,
    ],
};

const NINGGUANG_CHARGED_STAR_JADE: TalentScaling = TalentScaling {
    name: "星璃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.4960, 0.5332, 0.5704, 0.6200, 0.6572, 0.6944, 0.7440, 0.7936, 0.8432, 0.8928, 0.9443,
        1.0118, 1.0793, 1.1468, 1.2143,
    ],
};

// -- Plunging Attack -- Geo (Catalyst) --

const NINGGUANG_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const NINGGUANG_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const NINGGUANG_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 璇璣キ屏 (Jade Screen) -- Geo --

const NINGGUANG_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        2.3040, 2.4768, 2.6496, 2.8800, 3.0528, 3.2256, 3.4560, 3.6864, 3.9168, 4.1472, 4.3776,
        4.6080, 4.8960, 5.1840, 5.4720,
    ],
};

// -- Elemental Burst: 天キ権キ典 (Starshatter) -- Geo --

const NINGGUANG_BURST: TalentScaling = TalentScaling {
    name: "宝石ダメージ (1個)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.8696, 0.9348, 1.0000, 1.0870, 1.1522, 1.2174, 1.3044, 1.3914, 1.4783, 1.5653, 1.6522,
        1.7392, 1.8479, 1.9566, 2.0653,
    ],
};

pub const NINGGUANG: CharacterData = CharacterData {
    id: "ningguang",
    name: "Ningguang",
    element: Element::Geo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [821.0, 8674.0, 9110.0, 9787.0],
    base_atk: [18.0, 188.0, 198.0, 212.0],
    base_def: [48.0, 512.0, 538.0, 573.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Geo, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "千金キ擲",
            hits: &[NINGGUANG_NORMAL],
            charged: &[NINGGUANG_CHARGED, NINGGUANG_CHARGED_STAR_JADE],
            plunging: &[
                NINGGUANG_PLUNGE,
                NINGGUANG_PLUNGE_LOW,
                NINGGUANG_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "璇璣キ屏",
            scalings: &[NINGGUANG_SKILL],
        },
        elemental_burst: TalentData {
            name: "天キ権キ典",
            scalings: &[NINGGUANG_BURST],
        },
    },
};

// =============================================================================
// Noelle
// =============================================================================

// -- Normal Attack: 西風キ剣術・メイド (Favonius Bladework - Maid) -- Physical --

const NOELLE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7912, 0.8556, 0.9200, 1.0120, 1.0764, 1.1500, 1.2512, 1.3524, 1.4536, 1.5640, 1.6744,
        1.7848, 1.8952, 2.0056, 2.1160,
    ],
};

const NOELLE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7336, 0.7933, 0.8530, 0.9383, 0.9980, 1.0663, 1.1601, 1.2539, 1.3478, 1.4501, 1.5525,
        1.6549, 1.7573, 1.8597, 1.9620,
    ],
};

const NOELLE_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8626, 0.9328, 1.0030, 1.1033, 1.1735, 1.2538, 1.3640, 1.4743, 1.5846, 1.7050, 1.8255,
        1.9459, 2.0663, 2.1867, 2.3072,
    ],
};

const NOELLE_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1340, 1.2263, 1.3186, 1.4505, 1.5428, 1.6483, 1.7933, 1.9384, 2.0834, 2.2417, 2.3999,
        2.5582, 2.7164, 2.8747, 3.0329,
    ],
};

// -- Charged Attack -- Physical --

const NOELLE_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6252, 0.6762, 0.7271, 0.7998, 0.8508, 0.9089, 0.9888, 1.0688, 1.1487, 1.2361, 1.3234,
        1.4107, 1.4981, 1.5854, 1.6727,
    ],
};

const NOELLE_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.3144, 1.4213, 1.5282, 1.6810, 1.7879, 1.9102, 2.0785, 2.2467, 2.4150, 2.5981, 2.7812,
        2.9643, 3.1474, 3.3305, 3.5136,
    ],
};

// -- Plunging Attack -- Physical --

const NOELLE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9541, 1.0148, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
};

const NOELLE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const NOELLE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9459, 3.1841, 3.4223, 3.6824, 3.9424,
        4.2024, 4.4625, 4.7225, 4.9826,
    ],
};

// -- Elemental Skill: 護心キ鎧 (Breastplate) -- Geo, DEF scaling --

const NOELLE_SKILL_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        1.2000, 1.2900, 1.3800, 1.5000, 1.5900, 1.6800, 1.8000, 1.9200, 2.0400, 2.1600, 2.2800,
        2.4000, 2.5500, 2.7000, 2.8500,
    ],
};

const NOELLE_SKILL_HEAL: TalentScaling = TalentScaling {
    name: "回復量 (DEF基準)",
    scaling_stat: ScalingStat::Def,
    damage_element: None,
    values: [
        2.1280, 2.2876, 2.4472, 2.6600, 2.8196, 2.9792, 3.1920, 3.4048, 3.6176, 3.8304, 4.0432,
        4.2560, 4.5220, 4.7880, 5.0540,
    ],
};

// -- Elemental Burst: 大キ掃除 (Sweeping Time) -- Geo, DEF scaling burst --

const NOELLE_BURST_SLASH: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.6720, 0.7224, 0.7728, 0.8400, 0.8904, 0.9408, 1.0080, 1.0752, 1.1424, 1.2096, 1.2768,
        1.3440, 1.4280, 1.5120, 1.5960,
    ],
};

const NOELLE_BURST_SPINNING: TalentScaling = TalentScaling {
    name: "連続スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.9280, 0.9976, 1.0672, 1.1600, 1.2296, 1.2992, 1.3920, 1.4848, 1.5776, 1.6704, 1.7632,
        1.8560, 1.9720, 2.0880, 2.2040,
    ],
};

const NOELLE_BURST_DEF_BONUS: TalentScaling = TalentScaling {
    name: "ATK追加 (DEF基準)",
    scaling_stat: ScalingStat::Def,
    damage_element: None,
    values: [
        0.4000, 0.4300, 0.4600, 0.5000, 0.5300, 0.5600, 0.6000, 0.6400, 0.6800, 0.7200, 0.7600,
        0.8000, 0.8500, 0.9000, 0.9500,
    ],
};

pub const NOELLE: CharacterData = CharacterData {
    id: "noelle",
    name: "Noelle",
    element: Element::Geo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [1012.0, 10698.0, 11235.0, 12071.0],
    base_atk: [16.0, 172.0, 180.0, 194.0],
    base_def: [67.0, 709.0, 745.0, 799.0],
    ascension_stat: AscensionStat::Def(0.30),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "西風キ剣術・メイド",
            hits: &[
                NOELLE_NORMAL_1,
                NOELLE_NORMAL_2,
                NOELLE_NORMAL_3,
                NOELLE_NORMAL_4,
            ],
            charged: &[NOELLE_CHARGED_SPINNING, NOELLE_CHARGED_FINAL],
            plunging: &[NOELLE_PLUNGE, NOELLE_PLUNGE_LOW, NOELLE_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "護心キ鎧",
            scalings: &[NOELLE_SKILL_DAMAGE, NOELLE_SKILL_HEAL],
        },
        elemental_burst: TalentData {
            name: "大キ掃除",
            scalings: &[
                NOELLE_BURST_SLASH,
                NOELLE_BURST_SPINNING,
                NOELLE_BURST_DEF_BONUS,
            ],
        },
    },
};

// =============================================================================
// Xilonen
// =============================================================================

// -- Normal Attack: エケカトルの音 (Ehecatl's Roar) -- Physical --

const XILONEN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5180, 0.5602, 0.6025, 0.6627, 0.7050, 0.7531, 0.8193, 0.8854, 0.9516, 1.0241, 1.0966,
        1.1691, 1.2416, 1.3142, 1.3867,
    ],
};

const XILONEN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2741, 0.2964, 0.3187, 0.3506, 0.3729, 0.3984, 0.4334, 0.4684, 0.5034, 0.5417, 0.5801,
        0.6184, 0.6567, 0.6951, 0.7334,
    ],
};

const XILONEN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7296, 0.7890, 0.8484, 0.9332, 0.9926, 1.0605, 1.1538, 1.2472, 1.3405, 1.4423, 1.5441,
        1.6459, 1.7477, 1.8495, 1.9513,
    ],
};

// -- Nightsoul Normal Attack -- Geo --

const XILONEN_BLADE_ROLLER_1: TalentScaling = TalentScaling {
    name: "刃ローラー1段",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.5609, 0.6065, 0.6521, 0.7173, 0.7629, 0.8151, 0.8869, 0.9586, 1.0303, 1.1086, 1.1869,
        1.2652, 1.3434, 1.4217, 1.5000,
    ],
};

const XILONEN_BLADE_ROLLER_2: TalentScaling = TalentScaling {
    name: "刃ローラー2段",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.5543, 0.5993, 0.6444, 0.7088, 0.7539, 0.8055, 0.8764, 0.9474, 1.0183, 1.0956, 1.1729,
        1.2502, 1.3275, 1.4048, 1.4821,
    ],
};

const XILONEN_BLADE_ROLLER_3: TalentScaling = TalentScaling {
    name: "刃ローラー3段",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.7303, 0.7897, 0.8491, 0.9340, 0.9934, 1.0614, 1.1549, 1.2484, 1.3419, 1.4437, 1.5455,
        1.6474, 1.7492, 1.8511, 1.9529,
    ],
};

// -- Charged Attack -- Physical --

const XILONEN_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5413, 0.5854, 0.6296, 0.6926, 0.7367, 0.7870, 0.8562, 0.9254, 0.9947, 1.0703, 1.1458,
        1.2214, 1.2969, 1.3725, 1.4480,
    ],
};

const XILONEN_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5413, 0.5854, 0.6296, 0.6926, 0.7367, 0.7870, 0.8562, 0.9254, 0.9947, 1.0703, 1.1458,
        1.2214, 1.2969, 1.3725, 1.4480,
    ],
};

// -- Plunging Attack -- Physical --

const XILONEN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 0.1011, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const XILONEN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5271, 2.7055,
        2.8840, 3.0624, 3.2409, 3.4193,
    ],
};

const XILONEN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6021, 3.8249, 4.0478, 4.2706,
    ],
};

// -- Elemental Skill: イキシュトリの音色 (Yohual's Scratch) -- Geo --

const XILONEN_SKILL_RUSH: TalentScaling = TalentScaling {
    name: "突進ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        1.7920, 1.9264, 2.0608, 2.2400, 2.3744, 2.5088, 2.6880, 2.8672, 3.0464, 3.2256, 3.4048,
        3.5840, 3.8080, 4.0320, 4.2560,
    ],
};

// -- Elemental Burst: オセロトルの音響 (Ocelotlicue Point!) -- Geo --

const XILONEN_BURST_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        2.8132, 3.0242, 3.2352, 3.5165, 3.7275, 3.9385, 4.2198, 4.5011, 4.7824, 5.0637, 5.3450,
        5.6264, 5.9780, 6.3297, 6.6814,
    ],
};

const XILONEN_BURST_FOLLOW_UP: TalentScaling = TalentScaling {
    name: "追撃ダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        2.8132, 3.0242, 3.2352, 3.5165, 3.7275, 3.9385, 4.2198, 4.5011, 4.7824, 5.0637, 5.3450,
        5.6264, 5.9780, 6.3297, 6.6814,
    ],
};

const XILONEN_BURST_HEAL: TalentScaling = TalentScaling {
    name: "回復量 (DEF基準)",
    scaling_stat: ScalingStat::Def,
    damage_element: None,
    values: [
        1.0400, 1.1180, 1.1960, 1.3000, 1.3780, 1.4560, 1.5600, 1.6640, 1.7680, 1.8720, 1.9760,
        2.0800, 2.2100, 2.3400, 2.4700,
    ],
};

pub const XILONEN: CharacterData = CharacterData {
    id: "xilonen",
    name: "Xilonen",
    element: Element::Geo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [966.0, 9667.0, 10723.0, 11527.0],
    base_atk: [21.0, 215.0, 239.0, 257.0],
    base_def: [72.0, 722.0, 801.0, 861.0],
    ascension_stat: AscensionStat::Def(0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "エケカトルの音",
            hits: &[XILONEN_NORMAL_1, XILONEN_NORMAL_2, XILONEN_NORMAL_3],
            charged: &[XILONEN_CHARGED_1, XILONEN_CHARGED_2],
            plunging: &[XILONEN_PLUNGE, XILONEN_PLUNGE_LOW, XILONEN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "イキシュトリの音色",
            scalings: &[
                XILONEN_SKILL_RUSH,
                XILONEN_BLADE_ROLLER_1,
                XILONEN_BLADE_ROLLER_2,
                XILONEN_BLADE_ROLLER_3,
            ],
        },
        elemental_burst: TalentData {
            name: "オセロトルの音響",
            scalings: &[
                XILONEN_BURST_DAMAGE,
                XILONEN_BURST_FOLLOW_UP,
                XILONEN_BURST_HEAL,
            ],
        },
    },
};

// =============================================================================
// Yun Jin
// =============================================================================

// -- Normal Attack: 拂雲キ出手 (Cloud-Grazing Strike) -- Physical --

const YUN_JIN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4051, 0.4381, 0.4710, 0.5181, 0.5510, 0.5888, 0.6406, 0.6924, 0.7443, 0.8008, 0.8573,
        0.9138, 0.9703, 1.0269, 1.0834,
    ],
};

const YUN_JIN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4025, 0.4352, 0.4680, 0.5148, 0.5476, 0.5850, 0.6365, 0.6880, 0.7395, 0.7956, 0.8518,
        0.9079, 0.9641, 1.0202, 1.0764,
    ],
};

const YUN_JIN_NORMAL_3_1: TalentScaling = TalentScaling {
    name: "3段ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2296, 0.2483, 0.2670, 0.2937, 0.3124, 0.3338, 0.3632, 0.3926, 0.4220, 0.4540, 0.4860,
        0.5181, 0.5501, 0.5821, 0.6141,
    ],
};

const YUN_JIN_NORMAL_3_2: TalentScaling = TalentScaling {
    name: "3段ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2752, 0.2977, 0.3202, 0.3522, 0.3747, 0.4003, 0.4355, 0.4708, 0.5060, 0.5444, 0.5828,
        0.6213, 0.6597, 0.6981, 0.7365,
    ],
};

const YUN_JIN_NORMAL_4_1: TalentScaling = TalentScaling {
    name: "4段ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2399, 0.2595, 0.2790, 0.3069, 0.3264, 0.3488, 0.3794, 0.4101, 0.4408, 0.4743, 0.5078,
        0.5413, 0.5748, 0.6082, 0.6417,
    ],
};

const YUN_JIN_NORMAL_4_2: TalentScaling = TalentScaling {
    name: "4段ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2880, 0.3115, 0.3350, 0.3685, 0.3920, 0.4188, 0.4556, 0.4924, 0.5293, 0.5695, 0.6097,
        0.6499, 0.6901, 0.7303, 0.7705,
    ],
};

const YUN_JIN_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6734, 0.7283, 0.7833, 0.8616, 0.9165, 0.9791, 1.0651, 1.1510, 1.2370, 1.3313, 1.4257,
        1.5200, 1.6144, 1.7087, 1.8031,
    ],
};

// -- Charged Attack -- Physical --

const YUN_JIN_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1596, 1.2541, 1.3486, 1.4835, 1.5780, 1.6858, 1.8342, 1.9826, 2.1311, 2.2928, 2.4546,
        2.6163, 2.7781, 2.9399, 3.1016,
    ],
};

// -- Plunging Attack -- Physical --

const YUN_JIN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 0.1011, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const YUN_JIN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5271, 2.7055,
        2.8840, 3.0624, 3.2409, 3.4193,
    ],
};

const YUN_JIN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6021, 3.8249, 4.0478, 4.2706,
    ],
};

// -- Elemental Skill: 旋雲キ開相 (Opening Flourish) -- Geo, DEF scaling --

const YUN_JIN_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "短押しダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        1.4912, 1.6030, 1.7149, 1.8640, 1.9758, 2.0877, 2.2368, 2.3859, 2.5350, 2.6842, 2.8333,
        2.9824, 3.1688, 3.3552, 3.5416,
    ],
};

const YUN_JIN_SKILL_CHARGE_1: TalentScaling = TalentScaling {
    name: "一段チャージダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        2.6080, 2.8036, 2.9992, 3.2600, 3.4556, 3.6512, 3.9120, 4.1728, 4.4336, 4.6944, 4.9552,
        5.2160, 5.5420, 5.8680, 6.1940,
    ],
};

const YUN_JIN_SKILL_CHARGE_2: TalentScaling = TalentScaling {
    name: "二段チャージダメージ",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        3.7248, 4.0042, 4.2835, 4.6560, 4.9354, 5.2147, 5.5872, 5.9597, 6.3322, 6.7046, 7.0771,
        7.4496, 7.9152, 8.3808, 8.8464,
    ],
};

// -- Elemental Burst: 破嶂キ旌儀 (Cliffbreaker's Banner) -- Geo, DEF scaling --

const YUN_JIN_BURST_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        2.4400, 2.6230, 2.8060, 3.0500, 3.2330, 3.4160, 3.6600, 3.9040, 4.1480, 4.3920, 4.6360,
        4.8800, 5.1850, 5.4900, 5.7950,
    ],
};

const YUN_JIN_BURST_DMG_BONUS: TalentScaling = TalentScaling {
    name: "ダメージ増加 (DEF基準)",
    scaling_stat: ScalingStat::Def,
    damage_element: None,
    values: [
        0.3216, 0.3457, 0.3698, 0.4020, 0.4261, 0.4502, 0.4824, 0.5146, 0.5467, 0.5789, 0.6110,
        0.6432, 0.6834, 0.7236, 0.7638,
    ],
};

pub const YUN_JIN: CharacterData = CharacterData {
    id: "yun_jin",
    name: "Yun Jin",
    element: Element::Geo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [894.0, 9445.0, 9919.0, 10657.0],
    base_atk: [16.0, 171.0, 180.0, 191.0],
    base_def: [62.0, 651.0, 684.0, 734.0],
    ascension_stat: AscensionStat::EnergyRecharge(0.2668),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "拂雲キ出手",
            hits: &[
                YUN_JIN_NORMAL_1,
                YUN_JIN_NORMAL_2,
                YUN_JIN_NORMAL_3_1,
                YUN_JIN_NORMAL_3_2,
                YUN_JIN_NORMAL_4_1,
                YUN_JIN_NORMAL_4_2,
                YUN_JIN_NORMAL_5,
            ],
            charged: &[YUN_JIN_CHARGED],
            plunging: &[YUN_JIN_PLUNGE, YUN_JIN_PLUNGE_LOW, YUN_JIN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "旋雲キ開相",
            scalings: &[
                YUN_JIN_SKILL_PRESS,
                YUN_JIN_SKILL_CHARGE_1,
                YUN_JIN_SKILL_CHARGE_2,
            ],
        },
        elemental_burst: TalentData {
            name: "破嶂キ旌儀",
            scalings: &[YUN_JIN_BURST_DAMAGE, YUN_JIN_BURST_DMG_BONUS],
        },
    },
};

// =============================================================================
// Zhongli
// =============================================================================

// -- Normal Attack: 岩雨 (Rain of Stone) -- Physical --

const ZHONGLI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3077, 0.3328, 0.3578, 0.3936, 0.4186, 0.4473, 0.4866, 0.5260, 0.5653, 0.6083, 0.6513,
        0.6943, 0.7373, 0.7804, 0.8234,
    ],
};

const ZHONGLI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3115, 0.3369, 0.3622, 0.3984, 0.4238, 0.4528, 0.4926, 0.5324, 0.5722, 0.6157, 0.6593,
        0.7028, 0.7463, 0.7898, 0.8334,
    ],
};

const ZHONGLI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3858, 0.4172, 0.4486, 0.4935, 0.5249, 0.5608, 0.6101, 0.6594, 0.7088, 0.7626, 0.8165,
        0.8703, 0.9242, 0.9780, 1.0319,
    ],
};

const ZHONGLI_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4294, 0.4643, 0.4993, 0.5492, 0.5842, 0.6241, 0.6791, 0.7340, 0.7890, 0.8489, 0.9088,
        0.9688, 1.0288, 1.0887, 1.1487,
    ],
};

const ZHONGLI_NORMAL_5_1: TalentScaling = TalentScaling {
    name: "5段ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.1075, 0.1163, 0.1250, 0.1375, 0.1463, 0.1563, 0.1700, 0.1838, 0.1975, 0.2125, 0.2275,
        0.2425, 0.2575, 0.2725, 0.2875,
    ],
};

const ZHONGLI_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5451, 0.5894, 0.6337, 0.6971, 0.7414, 0.7922, 0.8620, 0.9318, 1.0017, 1.0775, 1.1533,
        1.2292, 1.3050, 1.3808, 1.4567,
    ],
};

// -- Charged Attack -- Physical --

const ZHONGLI_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1103, 1.2008, 1.2913, 1.4204, 1.5109, 1.6141, 1.7559, 1.8978, 2.0396, 2.1949, 2.3502,
        2.5055, 2.6608, 2.8161, 2.9714,
    ],
};

// -- Plunging Attack -- Physical --

const ZHONGLI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 0.1011, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const ZHONGLI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5271, 2.7055,
        2.8840, 3.0624, 3.2409, 3.4193,
    ],
};

const ZHONGLI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6021, 3.8249, 4.0478, 4.2706,
    ],
};

// -- Elemental Skill: 地心 (Dominus Lapidis) -- Geo, HP scaling --

const ZHONGLI_SKILL_STONE_STELE: TalentScaling = TalentScaling {
    name: "岩柱ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Geo),
    values: [
        0.1600, 0.1720, 0.1840, 0.2000, 0.2120, 0.2240, 0.2400, 0.2560, 0.2720, 0.2880, 0.3040,
        0.3200, 0.3400, 0.3600, 0.3800,
    ],
};

const ZHONGLI_SKILL_RESONANCE: TalentScaling = TalentScaling {
    name: "共鳴ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Geo),
    values: [
        0.3200, 0.3440, 0.3680, 0.4000, 0.4240, 0.4480, 0.4800, 0.5120, 0.5440, 0.5760, 0.6080,
        0.6400, 0.6800, 0.7200, 0.7600,
    ],
};

const ZHONGLI_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Geo),
    values: [
        0.8000, 0.8600, 0.9200, 1.0000, 1.0600, 1.1200, 1.2000, 1.2800, 1.3600, 1.4400, 1.5200,
        1.6000, 1.7000, 1.8000, 1.9000,
    ],
};

// -- Elemental Burst: 天星 (Planet Befall) -- Geo --

const ZHONGLI_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        4.0108, 4.2528, 4.4948, 4.8010, 5.0430, 5.2850, 5.5912, 5.8974, 6.2036, 6.5098, 6.8160,
        7.1222, 7.5548, 7.9875, 8.4201,
    ],
};

pub const ZHONGLI: CharacterData = CharacterData {
    id: "zhongli",
    name: "Zhongli",
    element: Element::Geo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [1144.0, 11453.0, 12707.0, 14695.0],
    base_atk: [20.0, 200.0, 222.0, 251.0],
    base_def: [57.0, 564.0, 626.0, 738.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Geo, 0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "岩雨",
            hits: &[
                ZHONGLI_NORMAL_1,
                ZHONGLI_NORMAL_2,
                ZHONGLI_NORMAL_3,
                ZHONGLI_NORMAL_4,
                ZHONGLI_NORMAL_5_1,
                ZHONGLI_NORMAL_6,
            ],
            charged: &[ZHONGLI_CHARGED],
            plunging: &[ZHONGLI_PLUNGE, ZHONGLI_PLUNGE_LOW, ZHONGLI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "地心",
            scalings: &[
                ZHONGLI_SKILL_STONE_STELE,
                ZHONGLI_SKILL_RESONANCE,
                ZHONGLI_SKILL_HOLD,
            ],
        },
        elemental_burst: TalentData {
            name: "天星",
            scalings: &[ZHONGLI_BURST],
        },
    },
};
