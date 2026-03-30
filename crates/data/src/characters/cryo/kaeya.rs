use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Ceremonial Bladework -- Physical --

const KAEYA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5375, 0.5813, 0.6250, 0.6875, 0.7313, 0.7813, 0.8500, 0.9188, 0.9875, 1.0625, 1.1484,
        1.2495, 1.3506, 1.4516, 1.5619,
    ],
};

const KAEYA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5169, 0.5589, 0.6010, 0.6611, 0.7032, 0.7513, 0.8174, 0.8835, 0.9496, 1.0217, 1.1043,
        1.2015, 1.2987, 1.3959, 1.5019,
    ],
};

const KAEYA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6527, 0.7059, 0.7590, 0.8349, 0.8880, 0.9488, 1.0322, 1.1157, 1.1992, 1.2903, 1.3947,
        1.5174, 1.6401, 1.7629, 1.8967,
    ],
};

const KAEYA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7086, 0.7663, 0.8240, 0.9064, 0.9641, 1.0300, 1.1206, 1.2113, 1.3019, 1.4008, 1.5141,
        1.6473, 1.7806, 1.9138, 2.0592,
    ],
};

const KAEYA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8824, 0.9542, 1.0260, 1.1286, 1.2004, 1.2825, 1.3954, 1.5082, 1.6211, 1.7442, 1.8853,
        2.0512, 2.2171, 2.3830, 2.5640,
    ],
};

// -- Charged Attack -- Physical --

const KAEYA_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5504, 0.5952, 0.6400, 0.7040, 0.7488, 0.8000, 0.8704, 0.9408, 1.0112, 1.0880, 1.1760,
        1.2795, 1.3830, 1.4865, 1.5994,
    ],
};

const KAEYA_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7310, 0.7905, 0.8500, 0.9350, 0.9945, 1.0625, 1.1560, 1.2495, 1.3430, 1.4450, 1.5619,
        1.6993, 1.8368, 1.9742, 2.1242,
    ],
};

// -- Plunging Attack -- Physical --

const KAEYA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const KAEYA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const KAEYA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: Frostgnaw -- Cryo --

const KAEYA_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.9120, 2.0554, 2.1988, 2.3900, 2.5334, 2.6768, 2.8680, 3.0592, 3.2504, 3.4416, 3.6328,
        3.8240, 4.0630, 4.3020, 4.5410,
    ],
};

// -- Elemental Burst: Glacial Waltz -- Cryo --

const KAEYA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.7760, 0.8342, 0.8924, 0.9700, 1.0282, 1.0864, 1.1640, 1.2416, 1.3192, 1.3968, 1.4744,
        1.5520, 1.6490, 1.7460, 1.8430,
    ],
};

pub const KAEYA: CharacterData = CharacterData {
    id: "kaeya",
    name: "Kaeya",
    element: Element::Cryo,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [976.0, 10312.0, 10830.0, 11636.0],
    base_atk: [19.0, 198.0, 208.0, 223.0],
    base_def: [66.0, 702.0, 737.0, 792.0],
    ascension_stat: AscensionStat::EnergyRecharge(0.2668),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "儀典剣術",
            hits: &[
                KAEYA_NORMAL_1,
                KAEYA_NORMAL_2,
                KAEYA_NORMAL_3,
                KAEYA_NORMAL_4,
                KAEYA_NORMAL_5,
            ],
            charged: &[KAEYA_CHARGED_1, KAEYA_CHARGED_2],
            plunging: &[KAEYA_PLUNGE, KAEYA_PLUNGE_LOW, KAEYA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "霜の噛みつき",
            scalings: &[KAEYA_SKILL],
        },
        elemental_burst: TalentData {
            name: "氷の輪舞",
            scalings: &[KAEYA_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Layla
