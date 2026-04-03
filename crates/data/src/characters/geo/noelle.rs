use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
    base_hp: [
        1012.00, 10698.00, 10698.00, 10966.50, 10966.50, 11100.75, 11100.75, 11056.00, 11056.00,
        11653.00, 11653.00, 11235.00, 11235.00, 12071.00, 12071.00,
        12553.84, // Lv95/Lv95+/Lv100
        12553.84, // Lv95/Lv95+/Lv100
        13036.68, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        16.00, 172.00, 172.00, 176.00, 176.00, 178.00, 178.00, 177.33, 177.33, 187.00, 187.00,
        180.00, 180.00, 194.00, 194.00, 201.76, // Lv95/Lv95+/Lv100
        201.76, // Lv95/Lv95+/Lv100
        209.52, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        67.00, 709.00, 709.00, 727.00, 727.00, 736.00, 736.00, 733.00, 733.00, 772.00, 772.00,
        745.00, 745.00, 799.00, 799.00, 830.96, // Lv95/Lv95+/Lv100
        830.96, // Lv95/Lv95+/Lv100
        862.92, // Lv95/Lv95+/Lv100
    ],
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
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
