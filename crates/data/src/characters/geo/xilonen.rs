use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
