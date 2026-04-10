use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 西風剣術・祭儀 -- Physical (Sword) --

const DAHLIA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4355, 0.4709, 0.5064, 0.5570, 0.5924, 0.6330, 0.6887, 0.7443, 0.8000, 0.8608, 0.9216,
        0.9823, 1.0431, 1.1039, 1.1646,
    ],
    dynamic_bonus: None,
};

const DAHLIA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4010, 0.4336, 0.4663, 0.5129, 0.5455, 0.5829, 0.6341, 0.6854, 0.7367, 0.7927, 0.8486,
        0.9046, 0.9605, 1.0165, 1.0724,
    ],
    dynamic_bonus: None,
};

const DAHLIA_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2374, 0.2568, 0.2761, 0.3037, 0.3230, 0.3451, 0.3755, 0.4059, 0.4362, 0.4694, 0.5025,
        0.5356, 0.5688, 0.6019, 0.6350,
    ],
    dynamic_bonus: None,
};

const DAHLIA_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2902, 0.3138, 0.3374, 0.3711, 0.3948, 0.4218, 0.4589, 0.4960, 0.5331, 0.5736, 0.6141,
        0.6546, 0.6950, 0.7355, 0.7760,
    ],
    dynamic_bonus: None,
};

const DAHLIA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6566, 0.7100, 0.7635, 0.8398, 0.8932, 0.9543, 1.0383, 1.1223, 1.2063, 1.2979, 1.3895,
        1.4811, 1.5727, 1.6643, 1.7560,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Physical --

const DAHLIA_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3988, 0.4312, 0.4637, 0.5100, 0.5425, 0.5796, 0.6306, 0.6816, 0.7326, 0.7883, 0.8439,
        0.8995, 0.9552, 1.0108, 1.0665,
    ],
    dynamic_bonus: None,
};

const DAHLIA_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ2",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5507, 0.5955, 0.6403, 0.7044, 0.7492, 0.8004, 0.8708, 0.9413, 1.0117, 1.0885, 1.1654,
        1.2422, 1.3191, 1.3959, 1.4727,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const DAHLIA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
};

const DAHLIA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
};

const DAHLIA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: 受洗の礼典 -- Hydro --

const DAHLIA_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        2.3280, 2.5026, 2.6772, 2.9100, 3.0846, 3.2592, 3.4920, 3.7248, 3.9576, 4.1904, 4.4232,
        4.6560, 4.9470, 5.2380, 5.5290,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 純光の祈り -- Hydro --

const DAHLIA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        4.0640, 4.3688, 4.6736, 5.0800, 5.3848, 5.6896, 6.0960, 6.5024, 6.9088, 7.3152, 7.7216,
        8.1280, 8.6360, 9.1440, 9.6520,
    ],
    dynamic_bonus: None,
};

pub const DAHLIA: CharacterData = CharacterData {
    id: "dahlia",
    name: "Dahlia",
    element: Element::Hydro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Snezhnaya,
    base_hp: [
        1049.00, 2694.00, 3477.00, 5208.00, 5765.00, 6631.00, 7373.00, 8239.00, 8796.00, 9661.00,
        10217.00, 11083.00, 11640.00, 12506.00, 12506.00, 13006.24, // Lv95/Lv95+/Lv100
        13006.24, // Lv95/Lv95+/Lv100
        13371.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        15.85, 40.72, 52.56, 78.72, 87.14, 100.23, 111.45, 124.54, 132.95, 146.02, 154.44, 167.53,
        175.94, 189.03, 189.03, 196.59, // Lv95/Lv95+/Lv100
        196.59, // Lv95/Lv95+/Lv100
        237.26, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        46.92, 120.55, 155.60, 233.08, 257.98, 296.74, 329.95, 368.71, 393.62, 432.33, 457.24,
        496.00, 520.91, 559.67, 559.67, 582.06, // Lv95/Lv95+/Lv100
        582.06, // Lv95/Lv95+/Lv100
        598.38, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "西風剣術・祭儀",
            hits: &[
                DAHLIA_NORMAL_1,
                DAHLIA_NORMAL_2,
                DAHLIA_NORMAL_3A,
                DAHLIA_NORMAL_3B,
                DAHLIA_NORMAL_4,
            ],
            charged: &[DAHLIA_CHARGED_1, DAHLIA_CHARGED_2],
            plunging: &[DAHLIA_PLUNGE, DAHLIA_PLUNGE_LOW, DAHLIA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "受洗の礼典",
            scalings: &[DAHLIA_SKILL],
        },
        elemental_burst: TalentData {
            name: "純光の祈り",
            scalings: &[DAHLIA_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
