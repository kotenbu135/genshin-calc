use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// --- Normal Attack: Foreign Fieldcleaver (physical) ---

const TRAVELER_DENDRO_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.44462, 0.48081, 0.517, 0.5687, 0.60489, 0.64625, 0.70312, 0.75999, 0.81686, 0.8789,
        0.94094, 1.00298, 1.06502, 1.12706, 1.1891,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const TRAVELER_DENDRO_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4343, 0.46965, 0.505, 0.5555, 0.59085, 0.63125, 0.6868, 0.74235, 0.7979, 0.8585, 0.9191,
        0.9797, 1.0403, 1.1009, 1.1615,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const TRAVELER_DENDRO_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.52976, 0.57288, 0.616, 0.6776, 0.72072, 0.77, 0.83776, 0.90552, 0.97328, 1.0472, 1.12112,
        1.19504, 1.26896, 1.34288, 1.4168,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const TRAVELER_DENDRO_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.58308, 0.63054, 0.678, 0.7458, 0.79326, 0.8475, 0.92208, 0.99666, 1.07124, 1.1526,
        1.23396, 1.31532, 1.39668, 1.47804, 1.5594,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const TRAVELER_DENDRO_NA_HIT5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.70778, 0.76539, 0.823, 0.9053, 0.96291, 1.02875, 1.11928, 1.20981, 1.30034, 1.3991,
        1.49786, 1.59662, 1.69538, 1.79414, 1.8929,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const TRAVELER_DENDRO_CHARGED_HIT1: TalentScaling = TalentScaling {
    name: "重撃ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.559, 0.6045, 0.65, 0.715, 0.7605, 0.8125, 0.884, 0.9555, 1.027, 1.105, 1.183, 1.261,
        1.339, 1.417, 1.495,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const TRAVELER_DENDRO_CHARGED_HIT2: TalentScaling = TalentScaling {
    name: "重撃ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6072, 0.6566, 0.7060, 0.7766, 0.8260, 0.8825, 0.9602, 1.0378, 1.1155, 1.2002, 1.2849,
        1.3696, 1.4544, 1.5391, 1.6238,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const TRAVELER_DENDRO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572,
        1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const TRAVELER_DENDRO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646,
        2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const TRAVELER_DENDRO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586,
        3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Elemental Skill: Razorgrass Blade ---

const TRAVELER_DENDRO_SKILL_DMG: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        2.304, 2.4768, 2.6496, 2.88, 3.0528, 3.2256, 3.456, 3.6864, 3.9168, 4.1472, 4.3776, 4.608,
        4.896, 5.184, 5.472,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Elemental Burst: Surgent Manifestation ---

const TRAVELER_DENDRO_BURST_LAMP: TalentScaling = TalentScaling {
    name: "蓮灯ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.8016, 0.86172, 0.92184, 1.002, 1.06212, 1.12224, 1.2024, 1.28256, 1.36272, 1.44288,
        1.52304, 1.6032, 1.7034, 1.8036, 1.9038,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const TRAVELER_DENDRO_BURST_EXPLOSION: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        4.008, 4.3086, 4.6092, 5.01, 5.3106, 5.6112, 6.012, 6.4128, 6.8136, 7.2144, 7.6152, 8.016,
        8.517, 9.018, 9.519,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Traveler (Dendro) aggregation ---

static TRAVELER_DENDRO_NA_HITS: &[TalentScaling] = &[
    TRAVELER_DENDRO_NA_HIT1,
    TRAVELER_DENDRO_NA_HIT2,
    TRAVELER_DENDRO_NA_HIT3,
    TRAVELER_DENDRO_NA_HIT4,
    TRAVELER_DENDRO_NA_HIT5,
];
static TRAVELER_DENDRO_CHARGED_ATTACKS: &[TalentScaling] =
    &[TRAVELER_DENDRO_CHARGED_HIT1, TRAVELER_DENDRO_CHARGED_HIT2];
static TRAVELER_DENDRO_PLUNGING: &[TalentScaling] = &[
    TRAVELER_DENDRO_PLUNGE,
    TRAVELER_DENDRO_PLUNGE_LOW,
    TRAVELER_DENDRO_PLUNGE_HIGH,
];
static TRAVELER_DENDRO_SKILL_SCALINGS: &[TalentScaling] = &[TRAVELER_DENDRO_SKILL_DMG];
static TRAVELER_DENDRO_BURST_SCALINGS: &[TalentScaling] =
    &[TRAVELER_DENDRO_BURST_LAMP, TRAVELER_DENDRO_BURST_EXPLOSION];

pub const TRAVELER_DENDRO: CharacterData = CharacterData {
    id: "traveler_dendro",
    name: "Traveler (Dendro)",
    element: Element::Dendro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Other,
    base_hp: [
        912.00, 2342.00, 3024.00, 4529.00, 5013.00, 5766.00, 6411.00, 7164.00, 7648.00, 8401.00,
        8885.00, 9638.00, 10122.00, 10875.00, 10875.00, 11251.00, // Lv95/Lv95+/Lv100
        11251.00, // Lv95/Lv95+/Lv100
        11627.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        17.81, 45.75, 59.05, 88.45, 97.91, 112.62, 125.22, 139.93, 149.38, 164.07, 173.53, 188.24,
        197.69, 212.40, 212.40, 239.50, // Lv95/Lv95+/Lv100
        239.50, // Lv95/Lv95+/Lv100
        266.59, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        57.23, 147.01, 189.76, 284.24, 314.61, 361.88, 402.38, 449.65, 480.03, 527.24, 557.61,
        604.88, 635.25, 682.52, 682.52, 706.13, // Lv95/Lv95+/Lv100
        706.13, // Lv95/Lv95+/Lv100
        729.73, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "異邦の草薙",
            hits: TRAVELER_DENDRO_NA_HITS,
            charged: TRAVELER_DENDRO_CHARGED_ATTACKS,
            plunging: TRAVELER_DENDRO_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "草縁剣",
            scalings: TRAVELER_DENDRO_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "臥草若化",
            scalings: TRAVELER_DENDRO_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
};

#[cfg(test)]
mod tests {
    use super::*;

    const CHARGED_HIT_1_EXPECTED: [f64; 15] = [
        0.5590, 0.6045, 0.6500, 0.7150, 0.7605, 0.8125, 0.8840, 0.9555, 1.0270, 1.1050, 1.1830,
        1.2610, 1.3390, 1.4170, 1.4950,
    ];
    const CHARGED_HIT_2_EXPECTED: [f64; 15] = [
        0.6072, 0.6566, 0.7060, 0.7766, 0.8260, 0.8825, 0.9602, 1.0378, 1.1155, 1.2002, 1.2849,
        1.3696, 1.4544, 1.5391, 1.6238,
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
    fn traveler_dendro_charged_hits_match_honeyhunter_mirror() {
        let charged = TRAVELER_DENDRO.talents.normal_attack.charged;
        assert_eq!(charged.len(), 2);
        assert_eq!(charged[0].name, "重撃ダメージ (1)");
        assert_eq!(charged[1].name, "重撃ダメージ (2)");
        assert_scaling_table(&charged[0].values, &CHARGED_HIT_1_EXPECTED, "Charged 1");
        assert_scaling_table(&charged[1].values, &CHARGED_HIT_2_EXPECTED, "Charged 2");
    }
}
