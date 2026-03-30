use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 断雨 (Cutting Torrent) -- Physical (Bow) --

const TARTAGLIA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4128, 0.4464, 0.4800, 0.5280, 0.5616, 0.6000, 0.6528, 0.7056, 0.7584, 0.8160, 0.8736,
        0.9312, 0.9888, 1.0464, 1.1040,
    ],
};

const TARTAGLIA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4627, 0.5003, 0.5380, 0.5918, 0.6295, 0.6725, 0.7317, 0.7909, 0.8500, 0.9146, 0.9792,
        1.0437, 1.1083, 1.1728, 1.2374,
    ],
};

const TARTAGLIA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5538, 0.5989, 0.6440, 0.7084, 0.7535, 0.8050, 0.8758, 0.9467, 1.0175, 1.0948, 1.1721,
        1.2494, 1.3266, 1.4039, 1.4812,
    ],
};

const TARTAGLIA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5702, 0.6166, 0.6630, 0.7293, 0.7757, 0.8288, 0.9017, 0.9746, 1.0475, 1.1271, 1.2067,
        1.2862, 1.3658, 1.4453, 1.5249,
    ],
};

const TARTAGLIA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6089, 0.6584, 0.7080, 0.7788, 0.8284, 0.8850, 0.9629, 1.0408, 1.1186, 1.2036, 1.2886,
        1.3735, 1.4585, 1.5434, 1.6284,
    ],
};

const TARTAGLIA_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7276, 0.7868, 0.8460, 0.9306, 0.9898, 1.0575, 1.1506, 1.2436, 1.3367, 1.4382, 1.5397,
        1.6412, 1.7428, 1.8443, 1.9458,
    ],
};

// -- Charged Attack -- Bow --

const TARTAGLIA_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const TARTAGLIA_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

// -- Plunging Attack -- Physical --

const TARTAGLIA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8178, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const TARTAGLIA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1853, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const TARTAGLIA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 魔王の武装·荒波 (Foul Legacy: Raging Tide) -- Hydro (Melee stance) --

const TARTAGLIA_MELEE_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3887, 0.4204, 0.4520, 0.4972, 0.5288, 0.5650, 0.6147, 0.6644, 0.7142, 0.7684, 0.8226,
        0.8769, 0.9311, 0.9854, 1.0396,
    ],
};

const TARTAGLIA_MELEE_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.4162, 0.4501, 0.4840, 0.5324, 0.5663, 0.6050, 0.6582, 0.7115, 0.7647, 0.8228, 0.8809,
        0.9390, 0.9970, 1.0551, 1.1132,
    ],
};

const TARTAGLIA_MELEE_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5633, 0.6092, 0.6550, 0.7205, 0.7664, 0.8188, 0.8908, 0.9629, 1.0349, 1.1135, 1.1921,
        1.2707, 1.3493, 1.4279, 1.5065,
    ],
};

const TARTAGLIA_MELEE_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5994, 0.6482, 0.6970, 0.7667, 0.8155, 0.8713, 0.9479, 1.0246, 1.1013, 1.1849, 1.2685,
        1.3522, 1.4358, 1.5195, 1.6031,
    ],
};

const TARTAGLIA_MELEE_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5530, 0.5980, 0.6430, 0.7073, 0.7523, 0.8038, 0.8745, 0.9452, 1.0159, 1.0931, 1.1703,
        1.2474, 1.3246, 1.4017, 1.4789,
    ],
};

const TARTAGLIA_MELEE_6A: TalentScaling = TalentScaling {
    name: "6段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3543, 0.3832, 0.4120, 0.4532, 0.4820, 0.5150, 0.5603, 0.6056, 0.6510, 0.7004, 0.7498,
        0.7993, 0.8487, 0.8982, 0.9476,
    ],
};

const TARTAGLIA_MELEE_6B: TalentScaling = TalentScaling {
    name: "6段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3767, 0.4073, 0.4380, 0.4818, 0.5125, 0.5475, 0.5957, 0.6439, 0.6920, 0.7446, 0.7972,
        0.8497, 0.9023, 0.9548, 1.0074,
    ],
};

const TARTAGLIA_MELEE_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.6020, 0.6510, 0.7000, 0.7700, 0.8190, 0.8750, 0.9520, 1.0290, 1.1060, 1.1900, 1.2740,
        1.3580, 1.4420, 1.5260, 1.6100,
    ],
};

const TARTAGLIA_MELEE_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.7198, 0.7784, 0.8370, 0.9207, 0.9793, 1.0463, 1.1383, 1.2304, 1.3225, 1.4229, 1.5233,
        1.6238, 1.7242, 1.8247, 1.9251,
    ],
};

const TARTAGLIA_RIPTIDE_SLASH: TalentScaling = TalentScaling {
    name: "断流・斬ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.6020, 0.6510, 0.7000, 0.7700, 0.8190, 0.8750, 0.9520, 1.0290, 1.1060, 1.1900, 1.2740,
        1.3580, 1.4420, 1.5260, 1.6100,
    ],
};

// -- Elemental Burst: 極悪技·尽滅閃 (Havoc: Obliteration) -- Hydro --

const TARTAGLIA_BURST_MELEE: TalentScaling = TalentScaling {
    name: "近接スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        4.6400, 4.9880, 5.3360, 5.8000, 6.1480, 6.4960, 6.9600, 7.4240, 7.8880, 8.3520, 8.8160,
        9.2800, 9.8600, 10.4400, 11.0200,
    ],
};

const TARTAGLIA_BURST_RANGED: TalentScaling = TalentScaling {
    name: "遠距離スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        3.7840, 4.0678, 4.3516, 4.7300, 5.0138, 5.2976, 5.6760, 6.0544, 6.4328, 6.8112, 7.1896,
        7.5680, 8.0410, 8.5140, 8.9870,
    ],
};

const TARTAGLIA_BURST_RIPTIDE: TalentScaling = TalentScaling {
    name: "断流・爆ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.2000, 1.2900, 1.3800, 1.5000, 1.5900, 1.6800, 1.8000, 1.9200, 2.0400, 2.1600, 2.2800,
        2.4000, 2.5500, 2.7000, 2.8500,
    ],
};

pub const TARTAGLIA: CharacterData = CharacterData {
    id: "tartaglia",
    name: "Tartaglia",
    element: Element::Hydro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Snezhnaya,
    base_hp: [1020.0, 11561.0, 12182.0, 13103.0],
    base_atk: [23.0, 260.0, 274.0, 301.0],
    base_def: [63.0, 714.0, 753.0, 810.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Hydro, 0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "断雨",
            hits: &[
                TARTAGLIA_NORMAL_1,
                TARTAGLIA_NORMAL_2,
                TARTAGLIA_NORMAL_3,
                TARTAGLIA_NORMAL_4,
                TARTAGLIA_NORMAL_5,
                TARTAGLIA_NORMAL_6,
            ],
            charged: &[TARTAGLIA_AIMED, TARTAGLIA_AIMED_FULL],
            plunging: &[
                TARTAGLIA_PLUNGE,
                TARTAGLIA_PLUNGE_LOW,
                TARTAGLIA_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "魔王の武装·荒波",
            scalings: &[
                TARTAGLIA_MELEE_1,
                TARTAGLIA_MELEE_2,
                TARTAGLIA_MELEE_3,
                TARTAGLIA_MELEE_4,
                TARTAGLIA_MELEE_5,
                TARTAGLIA_MELEE_6A,
                TARTAGLIA_MELEE_6B,
                TARTAGLIA_MELEE_CHARGED_1,
                TARTAGLIA_MELEE_CHARGED_2,
                TARTAGLIA_RIPTIDE_SLASH,
            ],
        },
        elemental_burst: TalentData {
            name: "極悪技·尽滅閃",
            scalings: &[
                TARTAGLIA_BURST_MELEE,
                TARTAGLIA_BURST_RANGED,
                TARTAGLIA_BURST_RIPTIDE,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
