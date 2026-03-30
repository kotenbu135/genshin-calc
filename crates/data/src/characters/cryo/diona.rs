use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Kätzlein Style -- Physical --

const DIONA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3612, 0.3906, 0.4200, 0.4620, 0.4914, 0.5250, 0.5712, 0.6174, 0.6636, 0.7140, 0.7718,
        0.8397, 0.9076, 0.9755, 1.0496,
    ],
};

const DIONA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3354, 0.3627, 0.3900, 0.4290, 0.4563, 0.4875, 0.5304, 0.5733, 0.6162, 0.6630, 0.7166,
        0.7797, 0.8428, 0.9058, 0.9746,
    ],
};

const DIONA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4558, 0.4929, 0.5300, 0.5830, 0.6201, 0.6625, 0.7208, 0.7791, 0.8374, 0.9010, 0.9739,
        1.0596, 1.1453, 1.2310, 1.3245,
    ],
};

const DIONA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4300, 0.4650, 0.5000, 0.5500, 0.5850, 0.6250, 0.6800, 0.7350, 0.7900, 0.8500, 0.9188,
        0.9996, 1.0805, 1.1613, 1.2495,
    ],
};

const DIONA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5375, 0.5813, 0.6250, 0.6875, 0.7313, 0.7813, 0.8500, 0.9188, 0.9875, 1.0625, 1.1484,
        1.2495, 1.3506, 1.4516, 1.5619,
    ],
};

// -- Aimed Shot -- Cryo (charged) --

const DIONA_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9371,
        1.0196, 1.1021, 1.1845, 1.2745,
    ],
};

const DIONA_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3610,
        2.5296, 2.6982, 2.8669, 3.0355,
    ],
};

// -- Plunging Attack -- Physical --

const DIONA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const DIONA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const DIONA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: Icy Paws -- Cryo --

const DIONA_SKILL: TalentScaling = TalentScaling {
    name: "猫の爪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.4192, 0.4506, 0.4821, 0.5240, 0.5554, 0.5869, 0.6288, 0.6707, 0.7126, 0.7546, 0.7965,
        0.8384, 0.8908, 0.9432, 0.9956,
    ],
};

// -- Elemental Burst: Signature Mix -- Cryo --

const DIONA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.8000, 0.8600, 0.9200, 1.0000, 1.0600, 1.1200, 1.2000, 1.2800, 1.3600, 1.4400, 1.5200,
        1.6000, 1.7000, 1.8000, 1.9000,
    ],
};

const DIONA_BURST_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5264, 0.5659, 0.6054, 0.6580, 0.6975, 0.7370, 0.7896, 0.8422, 0.8949, 0.9475, 1.0002,
        1.0528, 1.1186, 1.1844, 1.2502,
    ],
};

pub const DIONA: CharacterData = CharacterData {
    id: "diona",
    name: "Diona",
    element: Element::Cryo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [802.0, 8481.0, 8907.0, 9570.0],
    base_atk: [18.0, 188.0, 198.0, 212.0],
    base_def: [50.0, 532.0, 559.0, 601.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Cryo, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "ケッツェンシュタイル",
            hits: &[
                DIONA_NORMAL_1,
                DIONA_NORMAL_2,
                DIONA_NORMAL_3,
                DIONA_NORMAL_4,
                DIONA_NORMAL_5,
            ],
            charged: &[DIONA_AIMED, DIONA_AIMED_FULL],
            plunging: &[DIONA_PLUNGE, DIONA_PLUNGE_LOW, DIONA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "猫の爪フリーズ",
            scalings: &[DIONA_SKILL],
        },
        elemental_burst: TalentData {
            name: "特製カクテル",
            scalings: &[DIONA_BURST, DIONA_BURST_DOT],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Escoffier
