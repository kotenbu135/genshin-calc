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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NOELLE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7336, 0.7933, 0.8530, 0.9383, 0.9980, 1.0663, 1.1601, 1.2539, 1.3478, 1.4501, 1.5525,
        1.6549, 1.7573, 1.8597, 1.9620,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NOELLE_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8626, 0.9328, 1.0030, 1.1033, 1.1735, 1.2538, 1.3640, 1.4743, 1.5846, 1.7050, 1.8255,
        1.9459, 2.0663, 2.1867, 2.3072,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NOELLE_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1340, 1.2263, 1.3186, 1.4505, 1.5428, 1.6483, 1.7933, 1.9384, 2.0834, 2.2417, 2.3999,
        2.5582, 2.7164, 2.8747, 3.0329,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const NOELLE_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5074, 0.5487, 0.5900, 0.6490, 0.6903, 0.7375, 0.8024, 0.8673, 0.9322, 1.0030, 1.0738,
        1.1446, 1.2154, 1.2862, 1.3570,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NOELLE_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9047, 0.9784, 1.0520, 1.1572, 1.2308, 1.3150, 1.4307, 1.5464, 1.6622, 1.7884, 1.9146,
        2.0409, 2.1671, 2.2934, 2.4196,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NOELLE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NOELLE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9459, 3.1841, 3.4223, 3.6824, 3.9424,
        4.2024, 4.4625, 4.7225, 4.9826,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NOELLE_SKILL_HEAL: TalentScaling = TalentScaling {
    name: "回復量 (DEF基準)",
    scaling_stat: ScalingStat::Def,
    damage_element: None,
    values: [
        2.1280, 2.2876, 2.4472, 2.6600, 2.8196, 2.9792, 3.1920, 3.4048, 3.6176, 3.8304, 4.0432,
        4.2560, 4.5220, 4.7880, 5.0540,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
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
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NOELLE_BURST_SPINNING: TalentScaling = TalentScaling {
    name: "連続スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.9280, 0.9976, 1.0672, 1.1600, 1.2296, 1.2992, 1.3920, 1.4848, 1.5776, 1.6704, 1.7632,
        1.8560, 1.9720, 2.0880, 2.2040,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const NOELLE_BURST_DEF_BONUS: TalentScaling = TalentScaling {
    name: "ATK追加 (DEF基準)",
    scaling_stat: ScalingStat::Def,
    damage_element: None,
    values: [
        0.4000, 0.4300, 0.4600, 0.5000, 0.5300, 0.5600, 0.6000, 0.6400, 0.6800, 0.7200, 0.7600,
        0.8000, 0.8500, 0.9000, 0.9500,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const NOELLE: CharacterData = CharacterData {
    id: "noelle",
    name: "Noelle",
    element: Element::Geo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [
        1012.00, 2600.00, 3356.00, 5027.00, 5564.00, 6400.00, 7117.00, 7953.00, 8490.00, 9325.00,
        9862.00, 10698.00, 11235.00, 12071.00, 12071.00, 12488.50, // Lv95/Lv95+/Lv100
        12488.50, // Lv95/Lv95+/Lv100
        12906.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        16.03, 41.17, 53.15, 79.61, 88.12, 101.35, 112.70, 125.94, 134.44, 147.67, 156.17, 169.41,
        177.92, 191.16, 191.16, 215.55, // Lv95/Lv95+/Lv100
        215.55, // Lv95/Lv95+/Lv100
        239.93, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        66.95, 172.00, 222.02, 332.56, 368.10, 423.40, 470.79, 526.09, 561.63, 616.87, 652.40,
        707.71, 743.25, 798.55, 798.55, 826.17, // Lv95/Lv95+/Lv100
        826.17, // Lv95/Lv95+/Lv100
        853.79, // Lv95/Lv95+/Lv100
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
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

#[cfg(test)]
mod tests {
    use super::*;

    const BASE_HP_EXPECTED: [f64; 18] = [
        1012.00, 2600.00, 3356.00, 5027.00, 5564.00, 6400.00, 7117.00, 7953.00, 8490.00, 9325.00,
        9862.00, 10698.00, 11235.00, 12071.00, 12071.00, 12488.50, 12488.50, 12906.00,
    ];
    const BASE_ATK_EXPECTED: [f64; 18] = [
        16.03, 41.17, 53.15, 79.61, 88.12, 101.35, 112.70, 125.94, 134.44, 147.67, 156.17, 169.41,
        177.92, 191.16, 191.16, 215.55, 215.55, 239.93,
    ];
    const BASE_DEF_EXPECTED: [f64; 18] = [
        66.95, 172.00, 222.02, 332.56, 368.10, 423.40, 470.79, 526.09, 561.63, 616.87, 652.40,
        707.71, 743.25, 798.55, 798.55, 826.17, 826.17, 853.79,
    ];
    const CHARGED_SPINNING_EXPECTED: [f64; 15] = [
        0.5074, 0.5487, 0.5900, 0.6490, 0.6903, 0.7375, 0.8024, 0.8673, 0.9322, 1.0030, 1.0738,
        1.1446, 1.2154, 1.2862, 1.3570,
    ];
    const CHARGED_FINAL_EXPECTED: [f64; 15] = [
        0.9047, 0.9784, 1.0520, 1.1572, 1.2308, 1.3150, 1.4307, 1.5464, 1.6622, 1.7884, 1.9146,
        2.0409, 2.1671, 2.2934, 2.4196,
    ];

    const BASE_STAT_BREAKPOINTS: [u32; 18] = [
        1, 20, 20, 40, 40, 49, 50, 60, 60, 70, 70, 80, 80, 90, 90, 95, 95, 100,
    ];

    fn assert_close(actual: f64, expected: f64, label: &str) {
        assert!(
            (actual - expected).abs() <= 0.01,
            "{label}: expected {expected}, got {actual}"
        );
    }

    fn assert_scaling_table(actual: &[f64; 15], expected: &[f64; 15], label: &str) {
        for (index, (&actual, &expected)) in actual.iter().zip(expected.iter()).enumerate() {
            assert!(
                (actual - expected).abs() <= 1e-6,
                "{label} Lv{}: expected {expected}, got {actual}",
                index + 1
            );
        }
    }

    fn expected_stat_at_level(keypoints: &[f64; 18], level: u32) -> f64 {
        let level = level.clamp(1, 100);

        if level <= 1 {
            return keypoints[0];
        }
        if level >= 100 {
            return keypoints[17];
        }

        for index in 0..17 {
            let start_level = BASE_STAT_BREAKPOINTS[index];
            let end_level = BASE_STAT_BREAKPOINTS[index + 1];
            if level < start_level || level > end_level {
                continue;
            }

            let t = if start_level == end_level {
                0.0
            } else {
                (level - start_level) as f64 / (end_level - start_level) as f64
            };
            return keypoints[index] + (keypoints[index + 1] - keypoints[index]) * t;
        }

        unreachable!("level {level} should be covered by breakpoints");
    }

    #[test]
    fn noelle_base_stat_keypoints_match_honeyhunter_mirror() {
        for (index, (&actual, &expected)) in NOELLE
            .base_hp
            .iter()
            .zip(BASE_HP_EXPECTED.iter())
            .enumerate()
        {
            assert_close(actual, expected, &format!("base_hp[{index}]"));
        }
        for (index, (&actual, &expected)) in NOELLE
            .base_atk
            .iter()
            .zip(BASE_ATK_EXPECTED.iter())
            .enumerate()
        {
            assert_close(actual, expected, &format!("base_atk[{index}]"));
        }
        for (index, (&actual, &expected)) in NOELLE
            .base_def
            .iter()
            .zip(BASE_DEF_EXPECTED.iter())
            .enumerate()
        {
            assert_close(actual, expected, &format!("base_def[{index}]"));
        }
    }

    #[test]
    fn noelle_base_stats_interpolate_correctly_for_all_levels() {
        for level in 1..=100 {
            assert_close(
                NOELLE.base_hp_at_level(level),
                expected_stat_at_level(&BASE_HP_EXPECTED, level),
                &format!("base_hp_at_level({level})"),
            );
            assert_close(
                NOELLE.base_atk_at_level(level),
                expected_stat_at_level(&BASE_ATK_EXPECTED, level),
                &format!("base_atk_at_level({level})"),
            );
            assert_close(
                NOELLE.base_def_at_level(level),
                expected_stat_at_level(&BASE_DEF_EXPECTED, level),
                &format!("base_def_at_level({level})"),
            );
        }
    }

    #[test]
    fn noelle_charged_attack_scalings_match_honeyhunter_mirror() {
        assert_scaling_table(
            &NOELLE_CHARGED_SPINNING.values,
            &CHARGED_SPINNING_EXPECTED,
            "連続重撃ダメージ",
        );
        assert_scaling_table(
            &NOELLE_CHARGED_FINAL.values,
            &CHARGED_FINAL_EXPECTED,
            "重撃終了ダメージ",
        );
    }
}
