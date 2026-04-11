use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Raiden Shogun
// =============================================================================

// -- Normal Attack: 源流 (Origin) -- Physical --

const RAIDEN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3965, 0.4287, 0.4610, 0.5071, 0.5394, 0.5763, 0.6270, 0.6777, 0.7284, 0.7837, 0.8471,
        0.9216, 0.9962, 1.0707, 1.1520,
    ],
    dynamic_bonus: None,
};

const RAIDEN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3973, 0.4297, 0.4620, 0.5082, 0.5405, 0.5775, 0.6283, 0.6791, 0.7299, 0.7854, 0.8489,
        0.9236, 0.9983, 1.0730, 1.1545,
    ],
    dynamic_bonus: None,
};

const RAIDEN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4988, 0.5394, 0.5800, 0.6380, 0.6786, 0.7250, 0.7888, 0.8526, 0.9164, 0.9860, 1.0658,
        1.1595, 1.2533, 1.3471, 1.4494,
    ],
    dynamic_bonus: None,
};

const RAIDEN_NORMAL_4A: TalentScaling = TalentScaling {
    name: "4段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2898, 0.3134, 0.3370, 0.3707, 0.3943, 0.4213, 0.4583, 0.4954, 0.5325, 0.5729, 0.6192,
        0.6737, 0.7282, 0.7827, 0.8422,
    ],
    dynamic_bonus: None,
};

const RAIDEN_NORMAL_4B: TalentScaling = TalentScaling {
    name: "4段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2898, 0.3134, 0.3370, 0.3707, 0.3943, 0.4213, 0.4583, 0.4954, 0.5325, 0.5729, 0.6192,
        0.6737, 0.7282, 0.7827, 0.8422,
    ],
    dynamic_bonus: None,
};

const RAIDEN_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6545, 0.7077, 0.7610, 0.8371, 0.8904, 0.9513, 1.0350, 1.1187, 1.2024, 1.2937, 1.3983,
        1.5214, 1.6444, 1.7675, 1.9017,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Physical --

const RAIDEN_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9959, 1.0769, 1.1580, 1.2738, 1.3549, 1.4475, 1.5746, 1.7023, 1.8296, 1.9686, 2.1278,
        2.3151, 2.5023, 2.6896, 2.8938,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const RAIDEN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
};

const RAIDEN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
};

const RAIDEN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: 神変・悪曜開眼 (Transcendence: Baleful Omen) -- Electro --

const RAIDEN_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.1720, 1.2599, 1.3478, 1.4650, 1.5529, 1.6408, 1.7580, 1.8752, 1.9924, 2.1096, 2.2268,
        2.3440, 2.4905, 2.6370, 2.7835,
    ],
    dynamic_bonus: None,
};

const RAIDEN_SKILL_COORDINATED: TalentScaling = TalentScaling {
    name: "協同攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4200, 0.4515, 0.4830, 0.5250, 0.5565, 0.5880, 0.6300, 0.6720, 0.7140, 0.7560, 0.7980,
        0.8400, 0.8925, 0.9450, 0.9975,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 奥義・夢想真説 (Secret Art: Musou Shinsetsu) -- Electro --

static RAIDEN_RESOLVE_MUSOU: DynamicTalentBonus = DynamicTalentBonus {
    name: "諸願百目の輪",
    max_stacks: 60,
    per_stack: [
        0.03888, 0.041796, 0.044712, 0.0486, 0.051516, 0.054432, 0.05832, 0.062208, 0.066096,
        0.069984, 0.073872, 0.07776, 0.08262, 0.08748, 0.09234,
    ],
};

static RAIDEN_RESOLVE_NORMAL: DynamicTalentBonus = DynamicTalentBonus {
    name: "諸願百目の輪",
    max_stacks: 60,
    per_stack: [
        0.007262, 0.007806, 0.008351, 0.009077, 0.009622, 0.010166, 0.010892, 0.011619, 0.012345,
        0.013071, 0.013797, 0.014523, 0.015431, 0.016339, 0.017246,
    ],
};

const RAIDEN_BURST_MUSOU: TalentScaling = TalentScaling {
    name: "夢想の一太刀基礎ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        4.0080, 4.3086, 4.6092, 5.0100, 5.3106, 5.6112, 6.0120, 6.4128, 6.8136, 7.2144, 7.6152,
        8.0160, 8.5170, 9.0180, 9.5190,
    ],
    dynamic_bonus: Some(&RAIDEN_RESOLVE_MUSOU),
};

const RAIDEN_BURST_N1: TalentScaling = TalentScaling {
    name: "夢想一心1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4474, 0.4779, 0.5084, 0.5491, 0.5796, 0.6151, 0.6609, 0.7066, 0.7524, 0.7982, 0.8439,
        0.8897, 0.9354, 0.9812, 1.0269,
    ],
    dynamic_bonus: Some(&RAIDEN_RESOLVE_NORMAL),
};

const RAIDEN_BURST_N2: TalentScaling = TalentScaling {
    name: "夢想一心2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.4396, 0.4695, 0.4995, 0.5395, 0.5694, 0.6044, 0.6494, 0.6943, 0.7393, 0.7842, 0.8292,
        0.8741, 0.9191, 0.9640, 1.0090,
    ],
    dynamic_bonus: Some(&RAIDEN_RESOLVE_NORMAL),
};

const RAIDEN_BURST_N3: TalentScaling = TalentScaling {
    name: "夢想一心3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5382, 0.5749, 0.6116, 0.6605, 0.6972, 0.7400, 0.7951, 0.8501, 0.9052, 0.9600, 1.0153,
        1.0703, 1.1254, 1.1804, 1.2355,
    ],
    dynamic_bonus: Some(&RAIDEN_RESOLVE_NORMAL),
};

const RAIDEN_BURST_N4A: TalentScaling = TalentScaling {
    name: "夢想一心4段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3089, 0.3299, 0.3510, 0.3791, 0.4001, 0.4247, 0.4563, 0.4879, 0.5195, 0.5511, 0.5827,
        0.6143, 0.6458, 0.6774, 0.7090,
    ],
    dynamic_bonus: Some(&RAIDEN_RESOLVE_NORMAL),
};

const RAIDEN_BURST_N4B: TalentScaling = TalentScaling {
    name: "夢想一心4段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3098, 0.3309, 0.3520, 0.3802, 0.4013, 0.4259, 0.4576, 0.4893, 0.5210, 0.5526, 0.5843,
        0.6160, 0.6477, 0.6794, 0.7110,
    ],
    dynamic_bonus: Some(&RAIDEN_RESOLVE_NORMAL),
};

const RAIDEN_BURST_N5: TalentScaling = TalentScaling {
    name: "夢想一心5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7394, 0.7899, 0.8403, 0.9075, 0.9579, 1.0167, 1.0924, 1.1680, 1.2436, 1.3192, 1.3948,
        1.4705, 1.5461, 1.6217, 1.6973,
    ],
    dynamic_bonus: Some(&RAIDEN_RESOLVE_NORMAL),
};

const RAIDEN_BURST_CHARGED_1: TalentScaling = TalentScaling {
    name: "夢想一心重撃ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.6160, 0.6580, 0.7000, 0.7560, 0.7980, 0.8470, 0.9100, 0.9730, 1.0360, 1.0990, 1.1620,
        1.2250, 1.2880, 1.3510, 1.4140,
    ],
    dynamic_bonus: Some(&RAIDEN_RESOLVE_NORMAL),
};

const RAIDEN_BURST_CHARGED_2: TalentScaling = TalentScaling {
    name: "夢想一心重撃ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7436, 0.7943, 0.8450, 0.9126, 0.9633, 1.0225, 1.0985, 1.1746, 1.2506, 1.3267, 1.4027,
        1.4788, 1.5548, 1.6309, 1.7069,
    ],
    dynamic_bonus: Some(&RAIDEN_RESOLVE_NORMAL),
};

pub const RAIDEN_SHOGUN: CharacterData = CharacterData {
    id: "raiden_shogun",
    name: "Raiden Shogun",
    element: Element::Electro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [
        1005.00, 2606.00, 3468.00, 5189.00, 5801.00, 6675.00, 7491.00, 8373.00, 8985.00, 9875.00,
        10487.00, 11388.00, 12000.00, 12907.00, 12907.00, 13423.28, // Lv95/Lv95+/Lv100
        13423.28, // Lv95/Lv95+/Lv100
        13939.56, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        26.25, 68.10, 90.61, 135.59, 151.58, 174.39, 195.72, 218.77, 234.76, 258.02, 274.02,
        297.54, 313.53, 337.24, 337.24, 350.73, // Lv95/Lv95+/Lv100
        350.73, // Lv95/Lv95+/Lv100
        364.22, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        61.45, 159.39, 212.07, 317.33, 354.76, 408.16, 458.07, 512.02, 549.46, 603.90, 641.33,
        696.39, 733.82, 789.31, 789.31, 820.88, // Lv95/Lv95+/Lv100
        820.88, // Lv95/Lv95+/Lv100
        852.45, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::EnergyRecharge(0.32),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "源流",
            hits: &[
                RAIDEN_NORMAL_1,
                RAIDEN_NORMAL_2,
                RAIDEN_NORMAL_3,
                RAIDEN_NORMAL_4A,
                RAIDEN_NORMAL_4B,
                RAIDEN_NORMAL_5,
            ],
            charged: &[RAIDEN_CHARGED],
            plunging: &[RAIDEN_PLUNGE, RAIDEN_PLUNGE_LOW, RAIDEN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "神変・悪曜開眼",
            scalings: &[RAIDEN_SKILL, RAIDEN_SKILL_COORDINATED],
        },
        elemental_burst: TalentData {
            name: "奥義・夢想真説",
            scalings: &[
                RAIDEN_BURST_MUSOU,
                RAIDEN_BURST_N1,
                RAIDEN_BURST_N2,
                RAIDEN_BURST_N3,
                RAIDEN_BURST_N4A,
                RAIDEN_BURST_N4B,
                RAIDEN_BURST_N5,
                RAIDEN_BURST_CHARGED_1,
                RAIDEN_BURST_CHARGED_2,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

#[cfg(test)]
mod tests {
    use super::*;

    const BURST_N4B_EXPECTED: [f64; 15] = [
        0.3098, 0.3309, 0.3520, 0.3802, 0.4013, 0.4259, 0.4576, 0.4893, 0.5210, 0.5526, 0.5843,
        0.6160, 0.6477, 0.6794, 0.7110,
    ];

    #[test]
    fn raiden_musou_isshin_n4b_matches_honeyhunter_mirror() {
        for (index, (&actual, &expected)) in RAIDEN_BURST_N4B
            .values
            .iter()
            .zip(BURST_N4B_EXPECTED.iter())
            .enumerate()
        {
            assert!(
                (actual - expected).abs() <= 1e-6,
                "N4B Lv{}: expected {expected}, got {actual}",
                index + 1
            );
            assert!(
                (actual - RAIDEN_BURST_N4A.values[index]).abs() > 1e-6,
                "N4B Lv{} should differ from N4A",
                index + 1
            );
        }
    }
}
