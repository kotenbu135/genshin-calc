use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Illuga
// =============================================================================

// -- Normal Attack: Oathkeeper's Spear -- Physical --

const ILLUGA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4737, 0.5122, 0.5508, 0.6058, 0.6444, 0.6885, 0.7490, 0.8096, 0.8702, 0.9363, 1.0024,
        1.0685, 1.1346, 1.2007, 1.2668,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ILLUGA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4853, 0.5248, 0.5642, 0.6207, 0.6602, 0.7053, 0.7674, 0.8294, 0.8915, 0.9592, 1.0269,
        1.0946, 1.1624, 1.2301, 1.2978,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ILLUGA_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3143, 0.3399, 0.3655, 0.4020, 0.4276, 0.4569, 0.4971, 0.5373, 0.5775, 0.6213, 0.6652,
        0.7091, 0.7529, 0.7968, 0.8407,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ILLUGA_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3143, 0.3399, 0.3655, 0.4020, 0.4276, 0.4569, 0.4971, 0.5373, 0.5775, 0.6213, 0.6652,
        0.7091, 0.7529, 0.7968, 0.8407,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ILLUGA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7628, 0.8249, 0.8870, 0.9757, 1.0377, 1.1087, 1.2063, 1.3038, 1.4014, 1.5078, 1.6143,
        1.7207, 1.8271, 1.9336, 2.0400,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const ILLUGA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1103, 1.2006, 1.2910, 1.4201, 1.5105, 1.6137, 1.7558, 1.8978, 2.0398, 2.1947, 2.3496,
        2.5045, 2.6595, 2.8144, 2.9693,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const ILLUGA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ILLUGA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ILLUGA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: Dawnbearing Songbird -- Geo (EM+DEF dual) --

const ILLUGA_SKILL_PRESS_EM: TalentScaling = TalentScaling {
    name: "Press DMG・元素熟知",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Geo),
    values: [
        4.8256, 5.1875, 5.5494, 6.0320, 6.3939, 6.7558, 7.2384, 7.7210, 8.2035, 8.6861, 9.1686,
        9.6512, 10.2544, 10.8576, 11.4608,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ILLUGA_SKILL_PRESS_DEF: TalentScaling = TalentScaling {
    name: "Press DMG・防御力",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        2.4128, 2.5938, 2.7747, 3.0160, 3.1970, 3.3779, 3.6192, 3.8605, 4.1018, 4.3430, 4.5843,
        4.8256, 5.1272, 5.4288, 5.7304,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ILLUGA_SKILL_HOLD_EM: TalentScaling = TalentScaling {
    name: "Hold DMG・元素熟知",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Geo),
    values: [
        6.0320, 6.4844, 6.9368, 7.5400, 7.9924, 8.4448, 9.0480, 9.6512, 10.2544, 10.8576, 11.4608,
        12.0640, 12.8180, 13.5720, 14.3260,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ILLUGA_SKILL_HOLD_DEF: TalentScaling = TalentScaling {
    name: "Hold DMG・防御力",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        3.0160, 3.2422, 3.4684, 3.7700, 3.9962, 4.2224, 4.5240, 4.8256, 5.1272, 5.4288, 5.7304,
        6.0320, 6.4090, 6.7860, 7.1630,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: Shadowless Reflection -- Geo (EM+DEF dual) --

const ILLUGA_BURST_EM: TalentScaling = TalentScaling {
    name: "スキルダメージ・元素熟知",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Geo),
    values: [
        8.2720, 8.8924, 9.5128, 10.3400, 10.9604, 11.5808, 12.4080, 13.2352, 14.0624, 14.8896,
        15.7168, 16.5440, 17.5780, 18.6120, 19.6460,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ILLUGA_BURST_DEF: TalentScaling = TalentScaling {
    name: "スキルダメージ・防御力",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        4.1360, 4.4462, 4.7564, 5.1700, 5.4802, 5.7904, 6.2040, 6.6176, 7.0312, 7.4448, 7.8584,
        8.2720, 8.7890, 9.3060, 9.8230,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ILLUGA_BURST_LUNAR_CRYSTALLIZE: TalentScaling = TalentScaling {
    name: "月結晶化ダメージ増加",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Geo),
    values: [
        2.2592, 2.4286, 2.5981, 2.8240, 2.9934, 3.1629, 3.3888, 3.6147, 3.8406, 4.0666, 4.2925,
        4.5184, 4.8008, 5.0832, 5.3656,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Aggregation --

static ILLUGA_NA_HITS: &[TalentScaling] = &[
    ILLUGA_NORMAL_1,
    ILLUGA_NORMAL_2,
    ILLUGA_NORMAL_3A,
    ILLUGA_NORMAL_3B,
    ILLUGA_NORMAL_4,
];
static ILLUGA_CHARGED_ATTACKS: &[TalentScaling] = &[ILLUGA_CHARGED];
static ILLUGA_PLUNGING: &[TalentScaling] = &[ILLUGA_PLUNGE, ILLUGA_PLUNGE_LOW, ILLUGA_PLUNGE_HIGH];
static ILLUGA_SKILL_SCALINGS: &[TalentScaling] = &[
    ILLUGA_SKILL_PRESS_EM,
    ILLUGA_SKILL_PRESS_DEF,
    ILLUGA_SKILL_HOLD_EM,
    ILLUGA_SKILL_HOLD_DEF,
];
static ILLUGA_BURST_SCALINGS: &[TalentScaling] = &[
    ILLUGA_BURST_EM,
    ILLUGA_BURST_DEF,
    ILLUGA_BURST_LUNAR_CRYSTALLIZE,
];

pub const ILLUGA: CharacterData = CharacterData {
    id: "illuga",
    name: "Illuga",
    element: Element::Geo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Natlan,
    base_hp: [
        1003.00, 2577.00, 3326.00, 4982.00, 5514.00, 6343.00, 7052.00, 7881.00, 8413.00, 9241.00,
        9773.00, 10602.00, 11134.00, 11962.00, 11962.00, 12440.48, // Lv95/Lv95+/Lv100
        12440.48, // Lv95/Lv95+/Lv100
        12918.96, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        16.03, 41.17, 53.15, 79.61, 88.12, 101.35, 112.70, 125.94, 134.44, 147.67, 156.17, 169.41,
        177.92, 191.16, 191.16, 198.81, // Lv95/Lv95+/Lv100
        198.81, // Lv95/Lv95+/Lv100
        206.45, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        68.21, 175.24, 226.20, 338.81, 375.02, 431.36, 479.64, 535.98, 572.19, 628.47, 664.67,
        721.02, 757.22, 813.57, 813.57, 846.11, // Lv95/Lv95+/Lv100
        846.11, // Lv95/Lv95+/Lv100
        878.66, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "Oathkeeper's Spear",
            hits: ILLUGA_NA_HITS,
            charged: ILLUGA_CHARGED_ATTACKS,
            plunging: ILLUGA_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "Dawnbearing Songbird",
            scalings: ILLUGA_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "Shadowless Reflection",
            scalings: ILLUGA_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
    passive_scalings: &[],
};

#[cfg(test)]
mod tests {
    use super::*;

    const NORMAL_3_EXPECTED: [f64; 15] = [
        0.3143, 0.3399, 0.3655, 0.4020, 0.4276, 0.4569, 0.4971, 0.5373, 0.5775, 0.6213, 0.6652,
        0.7091, 0.7529, 0.7968, 0.8407,
    ];

    fn assert_scaling_table(actual: &[f64; 15], expected: &[f64; 15], label: &str) {
        for (index, (&actual, &expected)) in actual.iter().zip(expected.iter()).enumerate() {
            assert!(
                (actual - expected).abs() <= 1e-4,
                "{label} Lv{}: expected {expected}, got {actual}",
                index + 1
            );
        }
    }

    #[test]
    fn illuga_normal_hit_structure_matches_honeyhunter_mirror() {
        let hits = ILLUGA.talents.normal_attack.hits;
        assert_eq!(hits.len(), 5);
        assert_eq!(hits[2].name, "3段ダメージ (1)");
        assert_eq!(hits[3].name, "3段ダメージ (2)");
        assert_scaling_table(&hits[2].values, &NORMAL_3_EXPECTED, "N3-1");
        assert_scaling_table(&hits[3].values, &NORMAL_3_EXPECTED, "N3-2");
    }
}
