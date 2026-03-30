use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Mavuika
// =============================================================================

// -- Normal Attack: 焔は命を紡ぐ (Flames Weave Life) -- Physical --

const MAVUIKA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8004, 0.8655, 0.9306, 1.0237, 1.0888, 1.1633, 1.2657, 1.3680, 1.4704, 1.5821, 1.6938,
        1.8054, 1.9171, 2.0288, 2.1405,
    ],
};

const MAVUIKA_NORMAL_2A: TalentScaling = TalentScaling {
    name: "2段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3648, 0.3945, 0.4242, 0.4666, 0.4963, 0.5302, 0.5769, 0.6236, 0.6702, 0.7211, 0.7720,
        0.8229, 0.8738, 0.9247, 0.9756,
    ],
};

const MAVUIKA_NORMAL_2B: TalentScaling = TalentScaling {
    name: "2段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3322, 0.3593, 0.3863, 0.4249, 0.4520, 0.4829, 0.5254, 0.5679, 0.6104, 0.6567, 0.7031,
        0.7495, 0.7958, 0.8422, 0.8885,
    ],
};

const MAVUIKA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1619, 1.2565, 1.3511, 1.4862, 1.5808, 1.6889, 1.8375, 1.9861, 2.1347, 2.2968, 2.4590,
        2.6211, 2.7832, 2.9454, 3.1075,
    ],
};

// -- Charged Attack -- Physical --

const MAVUIKA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.9384, 2.0962, 2.2540, 2.4794, 2.6372, 2.8175, 3.0654, 3.3134, 3.5613, 3.8318, 4.1023,
        4.3728, 4.6432, 4.9137, 5.1842,
    ],
};

// -- Plunging Attack -- Physical --

const MAVUIKA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
};

const MAVUIKA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const MAVUIKA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9460, 3.1842, 3.4225, 3.6825, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
};

// -- Elemental Skill: その名の瞬間 (The Named Moment) -- Pyro --

const MAVUIKA_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.7440, 0.7998, 0.8556, 0.9300, 0.9858, 1.0416, 1.1160, 1.1904, 1.2648, 1.3392, 1.4136,
        1.4880, 1.5810, 1.6740, 1.7670,
    ],
};

const MAVUIKA_SKILL_RING: TalentScaling = TalentScaling {
    name: "灼熱の輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2800, 1.3760, 1.4720, 1.6000, 1.6960, 1.7920, 1.9200, 2.0480, 2.1760, 2.3040, 2.4320,
        2.5600, 2.7200, 2.8800, 3.0400,
    ],
};

const MAVUIKA_SKILL_FLAMESTRIDER_N1: TalentScaling = TalentScaling {
    name: "炎騎通常1段",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5726, 0.6193, 0.6659, 0.7325, 0.7791, 0.8323, 0.9056, 0.9788, 1.0521, 1.1320, 1.2119,
        1.2918, 1.3717, 1.4516, 1.5315,
    ],
};

const MAVUIKA_SKILL_FLAMESTRIDER_N2: TalentScaling = TalentScaling {
    name: "炎騎通常2段",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5913, 0.6395, 0.6876, 0.7563, 0.8045, 0.8595, 0.9351, 1.0108, 1.0864, 1.1689, 1.2514,
        1.3339, 1.4164, 1.4989, 1.5815,
    ],
};

const MAVUIKA_SKILL_FLAMESTRIDER_N3: TalentScaling = TalentScaling {
    name: "炎騎通常3段",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.6999, 0.7568, 0.8138, 0.8952, 0.9521, 1.0173, 1.1068, 1.1963, 1.2858, 1.3835, 1.4811,
        1.5788, 1.6764, 1.7741, 1.8717,
    ],
};

const MAVUIKA_SKILL_FLAMESTRIDER_N4: TalentScaling = TalentScaling {
    name: "炎騎通常4段",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.6970, 0.7538, 0.8105, 0.8916, 0.9483, 1.0132, 1.1023, 1.1915, 1.2806, 1.3779, 1.4751,
        1.5724, 1.6697, 1.7669, 1.8642,
    ],
};

const MAVUIKA_SKILL_FLAMESTRIDER_N5: TalentScaling = TalentScaling {
    name: "炎騎通常5段",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9100, 0.9841, 1.0582, 1.1640, 1.2381, 1.3227, 1.4391, 1.5555, 1.6719, 1.7989, 1.9259,
        2.0529, 2.1799, 2.3068, 2.4338,
    ],
};

// -- Elemental Burst: 焼天の時 (Hour of Burning Skies) -- Pyro --

const MAVUIKA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        4.4480, 4.7816, 5.1152, 5.5600, 5.8936, 6.2272, 6.6720, 7.1168, 7.5616, 8.0064, 8.4512,
        8.8960, 9.4520, 10.0080, 10.5640,
    ],
};

pub const MAVUIKA: CharacterData = CharacterData {
    id: "mavuika",
    name: "Mavuika",
    element: Element::Pyro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [977.0, 11074.0, 11670.0, 12552.0],
    base_atk: [28.0, 317.0, 334.0, 359.0],
    base_def: [62.0, 698.0, 736.0, 792.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "焔は命を紡ぐ",
            hits: &[
                MAVUIKA_NORMAL_1,
                MAVUIKA_NORMAL_2A,
                MAVUIKA_NORMAL_2B,
                MAVUIKA_NORMAL_3,
            ],
            charged: &[MAVUIKA_CHARGED],
            plunging: &[MAVUIKA_PLUNGE, MAVUIKA_PLUNGE_LOW, MAVUIKA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "その名の瞬間",
            scalings: &[
                MAVUIKA_SKILL,
                MAVUIKA_SKILL_RING,
                MAVUIKA_SKILL_FLAMESTRIDER_N1,
                MAVUIKA_SKILL_FLAMESTRIDER_N2,
                MAVUIKA_SKILL_FLAMESTRIDER_N3,
                MAVUIKA_SKILL_FLAMESTRIDER_N4,
                MAVUIKA_SKILL_FLAMESTRIDER_N5,
            ],
        },
        elemental_burst: TalentData {
            name: "焼天の時",
            scalings: &[MAVUIKA_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
