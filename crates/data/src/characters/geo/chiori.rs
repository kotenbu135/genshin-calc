use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
