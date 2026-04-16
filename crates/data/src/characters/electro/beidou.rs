use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Beidou
// =============================================================================

// -- Normal Attack: 征霆鑑 (Oceanborne) -- All physical --

const BEIDOU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7112, 0.7691, 0.8270, 0.9097, 0.9676, 1.0338, 1.1247, 1.2157, 1.3067, 1.4059, 1.5196,
        1.6533, 1.7871, 1.9208, 2.0667,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BEIDOU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7086, 0.7663, 0.8240, 0.9064, 0.9641, 1.0300, 1.1206, 1.2113, 1.3019, 1.4008, 1.5141,
        1.6473, 1.7806, 1.9138, 2.0592,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BEIDOU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8832, 0.9551, 1.0270, 1.1297, 1.2016, 1.2838, 1.3967, 1.5097, 1.6227, 1.7459, 1.8871,
        2.0532, 2.2192, 2.3853, 2.5665,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BEIDOU_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8652, 0.9356, 1.0060, 1.1066, 1.1770, 1.2575, 1.3682, 1.4788, 1.5895, 1.7102, 1.8485,
        2.0112, 2.1739, 2.3365, 2.5140,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BEIDOU_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1214, 1.2127, 1.3040, 1.4344, 1.5257, 1.6300, 1.7734, 1.9169, 2.0604, 2.2168, 2.3961,
        2.6070, 2.8178, 3.0287, 3.2587,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const BEIDOU_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5624, 0.6082, 0.6540, 0.7194, 0.7652, 0.8175, 0.8894, 0.9614, 1.0333, 1.1118, 1.1903,
        1.2688, 1.3473, 1.4258, 1.5043,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BEIDOU_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0182, 1.1012, 1.1842, 1.3026, 1.3856, 1.4800, 1.6102, 1.7404, 1.8706, 2.0128, 2.1550,
        2.2972, 2.4394, 2.5816, 2.7238,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const BEIDOU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9541, 1.0148, 1.0841, 1.1795, 1.2749, 1.3702, 1.4744, 1.5786,
        1.6827, 1.7869, 1.8910, 1.9952,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BEIDOU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9076, 2.0290, 2.1678, 2.3586, 2.5494, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BEIDOU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5345, 2.7078, 2.9460, 3.1843, 3.4225, 3.6824, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 捉浪 (Tidecaller) -- Electro --

const BEIDOU_SKILL_BASE: TalentScaling = TalentScaling {
    name: "基礎ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2160, 1.3072, 1.3984, 1.5200, 1.6112, 1.7024, 1.8240, 1.9456, 2.0672, 2.1888, 2.3104,
        2.4320, 2.5840, 2.7360, 2.8880,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BEIDOU_SKILL_HIT_BONUS: TalentScaling = TalentScaling {
    name: "受撃時ダメージボーナス",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.6000, 1.7200, 1.8400, 2.0000, 2.1200, 2.2400, 2.4000, 2.5600, 2.7200, 2.8800, 3.0400,
        3.2000, 3.4000, 3.6000, 3.8000,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 斫雷 (Stormbreaker) -- Electro --

const BEIDOU_BURST_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2160, 1.3072, 1.3984, 1.5200, 1.6112, 1.7024, 1.8240, 1.9456, 2.0672, 2.1888, 2.3104,
        2.4320, 2.5840, 2.7360, 2.8880,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BEIDOU_BURST_LIGHTNING: TalentScaling = TalentScaling {
    name: "雷放電ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.9600, 1.0320, 1.1040, 1.2000, 1.2720, 1.3440, 1.4400, 1.5360, 1.6320, 1.7280, 1.8240,
        1.9200, 2.0400, 2.1600, 2.2800,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const BEIDOU: CharacterData = CharacterData {
    id: "beidou",
    name: "Beidou",
    element: Element::Electro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [
        1094.00, 2811.00, 3628.00, 5435.00, 6015.00, 6919.00, 7694.00, 8597.00, 9178.00, 10081.00,
        10662.00, 11565.00, 12146.00, 13050.00, 13050.00, 13572.00, // Lv95/Lv95+/Lv100
        13572.00, // Lv95/Lv95+/Lv100
        14094.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        18.88, 48.49, 62.60, 93.76, 103.78, 119.37, 132.73, 148.32, 158.34, 173.92, 183.94, 199.53,
        209.55, 225.14, 225.14, 234.15, // Lv95/Lv95+/Lv100
        234.15, // Lv95/Lv95+/Lv100
        243.15, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        54.36, 139.66, 180.27, 270.03, 298.88, 343.79, 382.26, 427.17, 456.02, 500.87, 529.73,
        574.63, 603.49, 648.40, 648.40, 674.34, // Lv95/Lv95+/Lv100
        674.34, // Lv95/Lv95+/Lv100
        700.27, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Electro, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "征霆鑑",
            hits: &[
                BEIDOU_NORMAL_1,
                BEIDOU_NORMAL_2,
                BEIDOU_NORMAL_3,
                BEIDOU_NORMAL_4,
                BEIDOU_NORMAL_5,
            ],
            charged: &[BEIDOU_CHARGED_SPINNING, BEIDOU_CHARGED_FINAL],
            plunging: &[BEIDOU_PLUNGE, BEIDOU_PLUNGE_LOW, BEIDOU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "捉浪",
            scalings: &[BEIDOU_SKILL_BASE, BEIDOU_SKILL_HIT_BONUS],
        },
        elemental_burst: TalentData {
            name: "斫雷",
            scalings: &[BEIDOU_BURST_SKILL, BEIDOU_BURST_LIGHTNING],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
};

#[cfg(test)]
mod tests {
    use super::*;

    const NORMAL_1_EXPECTED: [f64; 15] = [
        0.7112, 0.7691, 0.8270, 0.9097, 0.9676, 1.0338, 1.1247, 1.2157, 1.3067, 1.4059, 1.5196,
        1.6533, 1.7871, 1.9208, 2.0667,
    ];
    const NORMAL_2_EXPECTED: [f64; 15] = [
        0.7086, 0.7663, 0.8240, 0.9064, 0.9641, 1.0300, 1.1206, 1.2113, 1.3019, 1.4008, 1.5141,
        1.6473, 1.7806, 1.9138, 2.0592,
    ];
    const NORMAL_3_EXPECTED: [f64; 15] = [
        0.8832, 0.9551, 1.0270, 1.1297, 1.2016, 1.2838, 1.3967, 1.5097, 1.6227, 1.7459, 1.8871,
        2.0532, 2.2192, 2.3853, 2.5665,
    ];
    const NORMAL_4_EXPECTED: [f64; 15] = [
        0.8652, 0.9356, 1.0060, 1.1066, 1.1770, 1.2575, 1.3682, 1.4788, 1.5895, 1.7102, 1.8485,
        2.0112, 2.1739, 2.3365, 2.5140,
    ];
    const NORMAL_5_EXPECTED: [f64; 15] = [
        1.1214, 1.2127, 1.3040, 1.4344, 1.5257, 1.6300, 1.7734, 1.9169, 2.0604, 2.2168, 2.3961,
        2.6070, 2.8178, 3.0287, 3.2587,
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
    fn beidou_normal_tables_match_honeyhunter_mirror() {
        assert_scaling_table(&BEIDOU_NORMAL_1.values, &NORMAL_1_EXPECTED, "N1");
        assert_scaling_table(&BEIDOU_NORMAL_2.values, &NORMAL_2_EXPECTED, "N2");
        assert_scaling_table(&BEIDOU_NORMAL_3.values, &NORMAL_3_EXPECTED, "N3");
        assert_scaling_table(&BEIDOU_NORMAL_4.values, &NORMAL_4_EXPECTED, "N4");
        assert_scaling_table(&BEIDOU_NORMAL_5.values, &NORMAL_5_EXPECTED, "N5");
    }
}
