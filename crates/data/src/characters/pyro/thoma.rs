use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Thoma
// =============================================================================

// -- Normal Attack: 迅烈な槍 (Swiftshatter Spear) -- Physical --

const THOMA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4439, 0.4801, 0.5162, 0.5678, 0.6040, 0.6453, 0.7020, 0.7588, 0.8156, 0.8775, 0.9395,
        1.0014, 1.0634, 1.1253, 1.1873,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const THOMA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4363, 0.4718, 0.5073, 0.5580, 0.5935, 0.6341, 0.6899, 0.7457, 0.8015, 0.8624, 0.9233,
        0.9842, 1.0450, 1.1059, 1.1668,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const THOMA_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2679, 0.2897, 0.3115, 0.3427, 0.3645, 0.3894, 0.4236, 0.4579, 0.4922, 0.5296, 0.5669,
        0.6043, 0.6417, 0.6791, 0.7165,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const THOMA_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2679, 0.2897, 0.3115, 0.3427, 0.3645, 0.3894, 0.4236, 0.4579, 0.4922, 0.5296, 0.5669,
        0.6043, 0.6417, 0.6791, 0.7165,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const THOMA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6736, 0.7284, 0.7832, 0.8615, 0.9163, 0.9790, 1.0652, 1.1513, 1.2375, 1.3314, 1.4254,
        1.5194, 1.6134, 1.7074, 1.8014,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const THOMA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1275, 1.2192, 1.3110, 1.4421, 1.5339, 1.6388, 1.7830, 1.9272, 2.0714, 2.2287, 2.3860,
        2.5433, 2.7007, 2.8580, 3.0153,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const THOMA_PLUNGE: TalentScaling = TalentScaling {
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

const THOMA_PLUNGE_LOW: TalentScaling = TalentScaling {
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

const THOMA_PLUNGE_HIGH: TalentScaling = TalentScaling {
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

// -- Elemental Skill: 烈焔侍立 (Blazing Blessing) -- Pyro --

const THOMA_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.4640, 1.5738, 1.6836, 1.8300, 1.9398, 2.0496, 2.1960, 2.3424, 2.4888, 2.6352, 2.7816,
        2.9280, 3.1110, 3.2940, 3.4770,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 真紅熾炎の大鎧 (Crimson Ooyoroi) -- Pyro --

const THOMA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.8800, 0.9460, 1.0120, 1.1000, 1.1660, 1.2320, 1.3200, 1.4080, 1.4960, 1.5840, 1.6720,
        1.7600, 1.8700, 1.9800, 2.0900,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const THOMA_BURST_FIERY: TalentScaling = TalentScaling {
    name: "炎の援護ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5800, 0.6235, 0.6670, 0.7250, 0.7685, 0.8120, 0.8700, 0.9280, 0.9860, 1.0440, 1.1020,
        1.1600, 1.2325, 1.3050, 1.3775,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const THOMA: CharacterData = CharacterData {
    id: "thoma",
    name: "Thoma",
    element: Element::Pyro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Inazuma,
    base_hp: [
        866.00, 2225.00, 2872.00, 4302.00, 4762.00, 5478.00, 6091.00, 6806.00, 7266.00, 7981.00,
        8440.00, 9156.00, 9616.00, 10331.00, 10331.00, 10744.24, // Lv95/Lv95+/Lv100
        10744.24, // Lv95/Lv95+/Lv100
        11046.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        16.92, 43.46, 56.10, 84.03, 93.01, 106.98, 118.96, 132.93, 141.91, 155.87, 164.85, 178.82,
        187.80, 201.78, 201.78, 209.85, // Lv95/Lv95+/Lv100
        209.85, // Lv95/Lv95+/Lv100
        253.26, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        62.95, 161.71, 208.74, 312.66, 346.08, 398.07, 442.62, 494.62, 528.03, 579.96, 613.37,
        665.37, 698.78, 750.77, 750.77, 780.80, // Lv95/Lv95+/Lv100
        780.80, // Lv95/Lv95+/Lv100
        802.71, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "迅烈な槍",
            hits: &[
                THOMA_NORMAL_1,
                THOMA_NORMAL_2,
                THOMA_NORMAL_3A,
                THOMA_NORMAL_3B,
                THOMA_NORMAL_4,
            ],
            charged: &[THOMA_CHARGED],
            plunging: &[THOMA_PLUNGE, THOMA_PLUNGE_LOW, THOMA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "烈焔侍立",
            scalings: &[THOMA_SKILL],
        },
        elemental_burst: TalentData {
            name: "真紅熾炎の大鎧",
            scalings: &[THOMA_BURST, THOMA_BURST_FIERY],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
    scaling_modifiers: &[],
};

#[cfg(test)]
mod tests {
    use super::*;

    const NORMAL_3B_EXPECTED: [f64; 15] = [
        0.2679, 0.2897, 0.3115, 0.3427, 0.3645, 0.3894, 0.4236, 0.4579, 0.4922, 0.5296, 0.5669,
        0.6043, 0.6417, 0.6791, 0.7165,
    ];

    #[test]
    fn thoma_normal_3b_matches_honeyhunter_mirror() {
        for (index, (&actual, &expected)) in THOMA_NORMAL_3B
            .values
            .iter()
            .zip(NORMAL_3B_EXPECTED.iter())
            .enumerate()
        {
            assert!(
                (actual - expected).abs() <= 1e-6,
                "N3B Lv{}: expected {expected}, got {actual}",
                index + 1
            );
        }
    }
}
