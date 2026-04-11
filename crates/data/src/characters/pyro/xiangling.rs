use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Xiangling
// =============================================================================

// -- Normal Attack: 旋火棍法 (Dough-Fu) -- Physical --

const XIANGLING_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4205, 0.4548, 0.4890, 0.5379, 0.5721, 0.6113, 0.6650, 0.7188, 0.7726, 0.8313, 0.8985,
        0.9776, 1.0567, 1.1358, 1.2220,
    ],
    dynamic_bonus: None,
};

const XIANGLING_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4214, 0.4557, 0.4900, 0.5390, 0.5733, 0.6125, 0.6664, 0.7203, 0.7742, 0.8330, 0.9004,
        0.9796, 1.0588, 1.1381, 1.2245,
    ],
    dynamic_bonus: None,
};

const XIANGLING_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2606, 0.2818, 0.3030, 0.3333, 0.3545, 0.3788, 0.4121, 0.4454, 0.4787, 0.5151, 0.5568,
        0.6058, 0.6548, 0.7037, 0.7572,
    ],
    dynamic_bonus: None,
};

const XIANGLING_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2606, 0.2818, 0.3030, 0.3333, 0.3545, 0.3788, 0.4121, 0.4454, 0.4787, 0.5151, 0.5568,
        0.6058, 0.6548, 0.7037, 0.7572,
    ],
    dynamic_bonus: None,
};

const XIANGLING_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (×4)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7104, 0.7682, 0.8260, 0.9086, 0.9664, 1.0325, 1.1234, 1.2142, 1.3051, 1.4042, 1.5178,
        1.6513, 1.7849, 1.9185, 2.0642,
    ],
    dynamic_bonus: None,
};

const XIANGLING_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2169, 1.3160, 1.4150, 1.5565, 1.6556, 1.7688, 1.9244, 2.0801, 2.2357, 2.4055, 2.6001,
        2.8289, 3.0567, 3.2865, 3.5361,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Physical --

const XIANGLING_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2169, 1.3160, 1.4150, 1.5565, 1.6556, 1.7688, 1.9244, 2.0801, 2.2357, 2.4055, 2.5753,
        2.7451, 2.9149, 3.0847, 3.2545,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const XIANGLING_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
};

const XIANGLING_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
};

const XIANGLING_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: グゥオパァー出撃 (Guoba Attack) -- Pyro --

const XIANGLING_SKILL: TalentScaling = TalentScaling {
    name: "噴火ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1128, 1.1963, 1.2797, 1.3910, 1.4745, 1.5579, 1.6692, 1.7805, 1.8918, 2.0030, 2.1143,
        2.2256, 2.3647, 2.5038, 2.6429,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 旋火輪 (Pyronado) -- Pyro --

const XIANGLING_BURST_1: TalentScaling = TalentScaling {
    name: "1段旋火輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.7200, 0.7740, 0.8280, 0.9000, 0.9540, 1.0080, 1.0800, 1.1520, 1.2240, 1.2960, 1.3680,
        1.4400, 1.5300, 1.6200, 1.7100,
    ],
    dynamic_bonus: None,
};

const XIANGLING_BURST_2: TalentScaling = TalentScaling {
    name: "2段旋火輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.8800, 0.9460, 1.0120, 1.1000, 1.1660, 1.2320, 1.3200, 1.4080, 1.4960, 1.5840, 1.6720,
        1.7600, 1.8700, 1.9800, 2.0900,
    ],
    dynamic_bonus: None,
};

const XIANGLING_BURST_3: TalentScaling = TalentScaling {
    name: "3段旋火輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.0960, 1.1782, 1.2604, 1.3700, 1.4522, 1.5344, 1.6440, 1.7536, 1.8632, 1.9728, 2.0824,
        2.1920, 2.3290, 2.4660, 2.6030,
    ],
    dynamic_bonus: None,
};

const XIANGLING_BURST_PYRONADO: TalentScaling = TalentScaling {
    name: "旋火輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1200, 1.2040, 1.2880, 1.4000, 1.4840, 1.5680, 1.6800, 1.7920, 1.9040, 2.0160, 2.1280,
        2.2400, 2.3800, 2.5200, 2.6600,
    ],
    dynamic_bonus: None,
};

pub const XIANGLING: CharacterData = CharacterData {
    id: "xiangling",
    name: "Xiangling",
    element: Element::Pyro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [
        912.00, 2342.00, 3024.00, 4529.00, 5013.00, 5766.00, 6411.00, 7164.00, 7648.00, 8401.00,
        8885.00, 9638.00, 10122.00, 10875.00, 10875.00, 11310.00, // Lv95/Lv95+/Lv100
        11310.00, // Lv95/Lv95+/Lv100
        11745.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        18.88, 48.49, 62.60, 93.76, 103.78, 119.37, 132.73, 148.32, 158.34, 173.92, 183.94, 199.53,
        209.55, 225.14, 225.14, 234.15, // Lv95/Lv95+/Lv100
        234.15, // Lv95/Lv95+/Lv100
        243.15, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        56.08, 144.07, 185.97, 278.55, 308.32, 354.64, 394.33, 440.66, 470.42, 516.69, 546.46,
        592.78, 622.55, 668.87, 668.87, 695.62, // Lv95/Lv95+/Lv100
        695.62, // Lv95/Lv95+/Lv100
        722.38, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "旋火棍法",
            hits: &[
                XIANGLING_NORMAL_1,
                XIANGLING_NORMAL_2,
                XIANGLING_NORMAL_3A,
                XIANGLING_NORMAL_3B,
                XIANGLING_NORMAL_4,
                XIANGLING_NORMAL_5,
            ],
            charged: &[XIANGLING_CHARGED],
            plunging: &[
                XIANGLING_PLUNGE,
                XIANGLING_PLUNGE_LOW,
                XIANGLING_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "グゥオパァー出撃",
            scalings: &[XIANGLING_SKILL],
        },
        elemental_burst: TalentData {
            name: "旋火輪",
            scalings: &[
                XIANGLING_BURST_1,
                XIANGLING_BURST_2,
                XIANGLING_BURST_3,
                XIANGLING_BURST_PYRONADO,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

#[cfg(test)]
mod tests {
    use super::*;

    const NORMAL_3B_EXPECTED: [f64; 15] = [
        0.2606, 0.2818, 0.3030, 0.3333, 0.3545, 0.3788, 0.4121, 0.4454, 0.4787, 0.5151, 0.5568,
        0.6058, 0.6548, 0.7037, 0.7572,
    ];

    #[test]
    fn xiangling_normal_3b_matches_honeyhunter_mirror() {
        for (index, (&actual, &expected)) in XIANGLING_NORMAL_3B
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
