use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 古華剣法 (Guhua Sword: Fatal Rainscreen) -- Physical (Sword) --

const XINGQIU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4661, 0.5041, 0.5420, 0.5962, 0.6341, 0.6775, 0.7371, 0.7967, 0.8564, 0.9214, 0.9959,
        1.0836, 1.1712, 1.2588, 1.3545,
    ],
};

const XINGQIU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4764, 0.5152, 0.5540, 0.6094, 0.6482, 0.6925, 0.7534, 0.8144, 0.8753, 0.9418, 1.0180,
        1.1076, 1.1971, 1.2867, 1.3844,
    ],
};

const XINGQIU_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2855, 0.3088, 0.3320, 0.3652, 0.3884, 0.4150, 0.4515, 0.4880, 0.5246, 0.5644, 0.6101,
        0.6637, 0.7174, 0.7711, 0.8297,
    ],
};

const XINGQIU_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2855, 0.3088, 0.3320, 0.3652, 0.3884, 0.4150, 0.4515, 0.4880, 0.5246, 0.5644, 0.6101,
        0.6637, 0.7174, 0.7711, 0.8297,
    ],
};

const XINGQIU_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5599, 0.6054, 0.6510, 0.7161, 0.7617, 0.8138, 0.8854, 0.9570, 1.0286, 1.1067, 1.1962,
        1.3015, 1.4067, 1.5120, 1.6268,
    ],
};

const XINGQIU_NORMAL_5A: TalentScaling = TalentScaling {
    name: "5段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3586, 0.3878, 0.4170, 0.4587, 0.4879, 0.5213, 0.5671, 0.6130, 0.6589, 0.7089, 0.7662,
        0.8337, 0.9011, 0.9685, 1.0421,
    ],
};

const XINGQIU_NORMAL_5B: TalentScaling = TalentScaling {
    name: "5段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3586, 0.3878, 0.4170, 0.4587, 0.4879, 0.5213, 0.5671, 0.6130, 0.6589, 0.7089, 0.7662,
        0.8337, 0.9011, 0.9685, 1.0421,
    ],
};

// -- Charged Attack -- Physical (Sword) --

const XINGQIU_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4730, 0.5115, 0.5500, 0.6050, 0.6435, 0.6875, 0.7480, 0.8085, 0.8690, 0.9350, 1.0106,
        1.0996, 1.1885, 1.2774, 1.3745,
    ],
};

const XINGQIU_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5616, 0.6073, 0.6530, 0.7183, 0.7640, 0.8163, 0.8881, 0.9599, 1.0317, 1.1101, 1.1999,
        1.3055, 1.4111, 1.5167, 1.6318,
    ],
};

// -- Plunging Attack -- Physical (Sword) --

const XINGQIU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const XINGQIU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const XINGQIU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 古華剣·画雨籠山 (Guhua Sword: Fatal Rainscreen) -- Hydro --

const XINGQIU_SKILL_1: TalentScaling = TalentScaling {
    name: "スキルダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.6800, 1.8060, 1.9320, 2.1000, 2.2260, 2.3520, 2.5200, 2.6880, 2.8560, 3.0240, 3.1920,
        3.3600, 3.5700, 3.7800, 3.9900,
    ],
};

const XINGQIU_SKILL_2: TalentScaling = TalentScaling {
    name: "スキルダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.9120, 2.0554, 2.1988, 2.3900, 2.5334, 2.6768, 2.8680, 3.0592, 3.2504, 3.4416, 3.6328,
        3.8240, 4.0630, 4.3020, 4.5410,
    ],
};

// -- Elemental Burst: 古華剣·裁雨留虹 (Guhua Sword: Raincutter) -- Hydro --

const XINGQIU_BURST: TalentScaling = TalentScaling {
    name: "剣雨のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5427, 0.5834, 0.6241, 0.6784, 0.7191, 0.7598, 0.8141, 0.8684, 0.9226, 0.9769, 1.0312,
        1.0854, 1.1533, 1.2211, 1.2890,
    ],
};

pub const XINGQIU: CharacterData = CharacterData {
    id: "xingqiu",
    name: "Xingqiu",
    element: Element::Hydro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [857.0, 9060.0, 9514.0, 10222.0],
    base_atk: [17.0, 178.0, 187.0, 202.0],
    base_def: [64.0, 673.0, 707.0, 758.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "古華剣法",
            hits: &[
                XINGQIU_NORMAL_1,
                XINGQIU_NORMAL_2,
                XINGQIU_NORMAL_3A,
                XINGQIU_NORMAL_3B,
                XINGQIU_NORMAL_4,
                XINGQIU_NORMAL_5A,
                XINGQIU_NORMAL_5B,
            ],
            charged: &[XINGQIU_CHARGED_1, XINGQIU_CHARGED_2],
            plunging: &[XINGQIU_PLUNGE, XINGQIU_PLUNGE_LOW, XINGQIU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "古華剣·画雨籠山",
            scalings: &[XINGQIU_SKILL_1, XINGQIU_SKILL_2],
        },
        elemental_burst: TalentData {
            name: "古華剣·裁雨留虹",
            scalings: &[XINGQIU_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
