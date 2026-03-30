use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: [Placeholder] -- Physical --

const JAHODA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3612, 0.3906, 0.4200, 0.4620, 0.4914, 0.5250, 0.5712, 0.6174, 0.6636, 0.7140, 0.7718,
        0.8397, 0.9076, 0.9755, 1.0496,
    ],
};

const JAHODA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3354, 0.3627, 0.3900, 0.4290, 0.4563, 0.4875, 0.5304, 0.5733, 0.6162, 0.6630, 0.7166,
        0.7797, 0.8428, 0.9058, 0.9746,
    ],
};

const JAHODA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4558, 0.4929, 0.5300, 0.5830, 0.6201, 0.6625, 0.7208, 0.7791, 0.8374, 0.9010, 0.9739,
        1.0596, 1.1453, 1.2310, 1.3245,
    ],
};

const JAHODA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4300, 0.4650, 0.5000, 0.5500, 0.5850, 0.6250, 0.6800, 0.7350, 0.7900, 0.8500, 0.9188,
        0.9996, 1.0805, 1.1613, 1.2495,
    ],
};

const JAHODA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5375, 0.5813, 0.6250, 0.6875, 0.7313, 0.7813, 0.8500, 0.9188, 0.9875, 1.0625, 1.1484,
        1.2495, 1.3506, 1.4516, 1.5619,
    ],
};

// -- Aimed Shot / Charged --

const JAHODA_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9371,
        1.0196, 1.1021, 1.1845, 1.2745,
    ],
};

const JAHODA_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3610,
        2.5296, 2.6982, 2.8669, 3.0355,
    ],
};

// -- Plunging Attack --

const JAHODA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const JAHODA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const JAHODA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill -- Anemo --

const JAHODA_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.12, 1.204, 1.288, 1.40, 1.484, 1.568, 1.68, 1.792, 1.904, 2.016, 2.128, 2.24, 2.38, 2.52,
        2.66,
    ],
};

// -- Elemental Burst -- Anemo --

const JAHODA_BURST: TalentScaling = TalentScaling {
    name: "バーストダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.728, 1.8576, 1.9872, 2.16, 2.2896, 2.4192, 2.592, 2.7648, 2.9376, 3.1104, 3.2832, 3.456,
        3.672, 3.888, 4.104,
    ],
};

pub const JAHODA: CharacterData = CharacterData {
    id: "jahoda",
    name: "Jahoda",
    element: Element::Anemo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Snezhnaya,
    base_hp: [802.0, 8481.0, 8907.0, 9570.0],
    base_atk: [18.0, 188.0, 198.0, 212.0],
    base_def: [50.0, 532.0, 559.0, 601.0],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "Placeholder Normal Attack",
            hits: &[
                JAHODA_NORMAL_1,
                JAHODA_NORMAL_2,
                JAHODA_NORMAL_3,
                JAHODA_NORMAL_4,
                JAHODA_NORMAL_5,
            ],
            charged: &[JAHODA_AIMED, JAHODA_AIMED_FULL],
            plunging: &[JAHODA_PLUNGE, JAHODA_PLUNGE_LOW, JAHODA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "Placeholder Skill",
            scalings: &[JAHODA_SKILL],
        },
        elemental_burst: TalentData {
            name: "Placeholder Burst",
            scalings: &[JAHODA_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Varka — 5★ Anemo Claymore (Mondstadt)
// Source: datamined v6.4
// Normal Attack: Favonius Bladework: Dancing Radiance
// Elemental Skill: Windbound Execution
// Elemental Burst: Northwind Avatar
