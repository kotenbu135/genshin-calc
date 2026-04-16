use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Zhongli
// =============================================================================

// -- Normal Attack: 岩雨 (Rain of Stone) -- Physical --

const ZHONGLI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3077, 0.3328, 0.3578, 0.3936, 0.4186, 0.4473, 0.4866, 0.5260, 0.5653, 0.6083, 0.6513,
        0.6943, 0.7373, 0.7804, 0.8234,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ZHONGLI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3115, 0.3369, 0.3622, 0.3984, 0.4238, 0.4528, 0.4926, 0.5324, 0.5722, 0.6157, 0.6593,
        0.7028, 0.7463, 0.7898, 0.8334,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ZHONGLI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3858, 0.4172, 0.4486, 0.4935, 0.5249, 0.5608, 0.6101, 0.6594, 0.7088, 0.7626, 0.8165,
        0.8703, 0.9242, 0.9780, 1.0319,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ZHONGLI_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4294, 0.4643, 0.4993, 0.5492, 0.5842, 0.6241, 0.6791, 0.7340, 0.7890, 0.8489, 0.9088,
        0.9688, 1.0288, 1.0887, 1.1487,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ZHONGLI_NORMAL_5_1: TalentScaling = TalentScaling {
    name: "5段ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.1075, 0.1163, 0.1250, 0.1375, 0.1463, 0.1563, 0.1700, 0.1838, 0.1975, 0.2125, 0.2275,
        0.2425, 0.2575, 0.2725, 0.2875,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ZHONGLI_NORMAL_5_2: TalentScaling = TalentScaling {
    name: "5段ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.1075, 0.1163, 0.1250, 0.1375, 0.1463, 0.1563, 0.1700, 0.1838, 0.1975, 0.2125, 0.2275,
        0.2425, 0.2575, 0.2725, 0.2875,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ZHONGLI_NORMAL_5_3: TalentScaling = TalentScaling {
    name: "5段ダメージ3",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.1075, 0.1163, 0.1250, 0.1375, 0.1463, 0.1563, 0.1700, 0.1838, 0.1975, 0.2125, 0.2275,
        0.2425, 0.2575, 0.2725, 0.2875,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ZHONGLI_NORMAL_5_4: TalentScaling = TalentScaling {
    name: "5段ダメージ4",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.1075, 0.1163, 0.1250, 0.1375, 0.1463, 0.1563, 0.1700, 0.1838, 0.1975, 0.2125, 0.2275,
        0.2425, 0.2575, 0.2725, 0.2875,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ZHONGLI_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5451, 0.5894, 0.6337, 0.6971, 0.7414, 0.7922, 0.8620, 0.9318, 1.0017, 1.0775, 1.1533,
        1.2292, 1.3050, 1.3808, 1.4567,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const ZHONGLI_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1103, 1.2008, 1.2913, 1.4204, 1.5109, 1.6141, 1.7559, 1.8978, 2.0396, 2.1949, 2.3502,
        2.5055, 2.6608, 2.8161, 2.9714,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const ZHONGLI_PLUNGE: TalentScaling = TalentScaling {
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

const ZHONGLI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5271, 2.7055,
        2.8840, 3.0624, 3.2409, 3.4193,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ZHONGLI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6021, 3.8249, 4.0478, 4.2706,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 地心 (Dominus Lapidis) -- Geo, HP scaling --

const ZHONGLI_SKILL_STONE_STELE: TalentScaling = TalentScaling {
    name: "岩柱ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Geo),
    values: [
        0.1600, 0.1720, 0.1840, 0.2000, 0.2120, 0.2240, 0.2400, 0.2560, 0.2720, 0.2880, 0.3040,
        0.3200, 0.3400, 0.3600, 0.3800,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ZHONGLI_SKILL_RESONANCE: TalentScaling = TalentScaling {
    name: "共鳴ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Geo),
    values: [
        0.3200, 0.3440, 0.3680, 0.4000, 0.4240, 0.4480, 0.4800, 0.5120, 0.5440, 0.5760, 0.6080,
        0.6400, 0.6800, 0.7200, 0.7600,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ZHONGLI_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Geo),
    values: [
        0.8000, 0.8600, 0.9200, 1.0000, 1.0600, 1.1200, 1.2000, 1.2800, 1.3600, 1.4400, 1.5200,
        1.6000, 1.7000, 1.8000, 1.9000,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 天星 (Planet Befall) -- Geo --

const ZHONGLI_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        4.0108, 4.4444, 4.8780, 5.4200, 5.9078, 6.3956, 7.0460, 7.6964, 8.3468, 8.9972, 9.6476,
        10.2980, 10.8400, 11.3820, 11.9240,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const ZHONGLI: CharacterData = CharacterData {
    id: "zhongli",
    name: "Zhongli",
    element: Element::Geo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [
        1144.00, 2967.00, 3948.00, 5908.00, 6605.00, 7599.00, 8528.00, 9533.00, 10230.00, 11243.00,
        11940.00, 12965.00, 13662.00, 14695.00, 14695.00, 15282.80, // Lv95/Lv95+/Lv100
        15282.80, // Lv95/Lv95+/Lv100
        15870.60, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        19.55, 50.72, 67.48, 100.97, 112.88, 129.87, 145.75, 162.91, 174.82, 192.15, 204.06,
        221.57, 233.48, 251.14, 251.14, 261.19, // Lv95/Lv95+/Lv100
        261.19, // Lv95/Lv95+/Lv100
        271.23, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        57.44, 148.99, 198.24, 296.63, 331.62, 381.53, 428.19, 478.62, 513.61, 564.50, 599.49,
        650.95, 685.95, 737.81, 737.81, 767.32, // Lv95/Lv95+/Lv100
        767.32, // Lv95/Lv95+/Lv100
        796.83, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Geo, 0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "岩雨",
            hits: &[
                ZHONGLI_NORMAL_1,
                ZHONGLI_NORMAL_2,
                ZHONGLI_NORMAL_3,
                ZHONGLI_NORMAL_4,
                ZHONGLI_NORMAL_5_1,
                ZHONGLI_NORMAL_5_2,
                ZHONGLI_NORMAL_5_3,
                ZHONGLI_NORMAL_5_4,
                ZHONGLI_NORMAL_6,
            ],
            charged: &[ZHONGLI_CHARGED],
            plunging: &[ZHONGLI_PLUNGE, ZHONGLI_PLUNGE_LOW, ZHONGLI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "地心",
            scalings: &[
                ZHONGLI_SKILL_STONE_STELE,
                ZHONGLI_SKILL_RESONANCE,
                ZHONGLI_SKILL_HOLD,
            ],
        },
        elemental_burst: TalentData {
            name: "天星",
            scalings: &[ZHONGLI_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

#[cfg(test)]
mod tests {
    use super::*;
    use genshin_calc_core::DamageType;

    const BURST_EXPECTED: [f64; 15] = [
        4.0108, 4.4444, 4.8780, 5.4200, 5.9078, 6.3956, 7.0460, 7.6964, 8.3468, 8.9972, 9.6476,
        10.2980, 10.8400, 11.3820, 11.9240,
    ];

    fn assert_scaling_table(actual: &[f64; 15], expected: &[f64; 15], label: &str) {
        for (index, (&actual, &expected)) in actual.iter().zip(expected.iter()).enumerate() {
            assert!(
                (actual - expected).abs() <= 1e-6,
                "{label} Lv{}: expected {expected}, got {actual}",
                index + 1
            );
        }
    }

    #[test]
    fn zhongli_burst_scalings_match_honeyhunter_mirror() {
        assert_scaling_table(&ZHONGLI_BURST.values, &BURST_EXPECTED, "天星");
    }

    #[test]
    fn zhongli_constellation_pattern_matches_honeyhunter_mirror() {
        assert_eq!(ZHONGLI.effective_talent_level(DamageType::Skill, 9, 2), 9);
        assert_eq!(ZHONGLI.effective_talent_level(DamageType::Burst, 9, 2), 9);
        assert_eq!(ZHONGLI.effective_talent_level(DamageType::Skill, 9, 3), 12);
        assert_eq!(ZHONGLI.effective_talent_level(DamageType::Burst, 9, 3), 9);
        assert_eq!(ZHONGLI.effective_talent_level(DamageType::Skill, 9, 5), 12);
        assert_eq!(ZHONGLI.effective_talent_level(DamageType::Burst, 9, 5), 12);
    }
}
