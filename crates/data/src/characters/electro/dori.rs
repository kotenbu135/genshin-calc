use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Dori
// =============================================================================

// -- Normal Attack: マーベラスソードダンス (Marvelous Sword-Dance) -- Physical --

const DORI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9021, 0.9756, 1.0490, 1.1539, 1.2274, 1.3113, 1.4266, 1.5420, 1.6574, 1.7833, 1.9093,
        2.0353, 2.1612, 2.2872, 2.4132,
    ],
    dynamic_bonus: None,
};

const DORI_NORMAL_2A: TalentScaling = TalentScaling {
    name: "2段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4107, 0.4441, 0.4775, 0.5253, 0.5587, 0.5969, 0.6493, 0.7017, 0.7542, 0.8116, 0.8690,
        0.9264, 0.9839, 1.0413, 1.0987,
    ],
    dynamic_bonus: None,
};

const DORI_NORMAL_2B: TalentScaling = TalentScaling {
    name: "2段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4313, 0.4665, 0.5017, 0.5519, 0.5871, 0.6272, 0.6822, 0.7373, 0.7924, 0.8528, 0.9132,
        0.9735, 1.0339, 1.0943, 1.1546,
    ],
    dynamic_bonus: None,
};

const DORI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2836, 1.3883, 1.4930, 1.6423, 1.7470, 1.8663, 2.0303, 2.1944, 2.3584, 2.5379, 2.7174,
        2.8969, 3.0764, 3.2559, 3.4354,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Physical --

const DORI_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6254, 0.6764, 0.7275, 0.8002, 0.8513, 0.9094, 0.9893, 1.0692, 1.1491, 1.2366, 1.3365,
        1.4541, 1.5717, 1.6893, 1.8173,
    ],
    dynamic_bonus: None,
};

const DORI_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1314, 1.2235, 1.3156, 1.4472, 1.5393, 1.6445, 1.7893, 1.9341, 2.0789, 2.2366, 2.4164,
        2.6293, 2.8422, 3.0551, 3.2861,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const DORI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9541, 1.0148, 1.0841, 1.1795, 1.2749, 1.3702, 1.4744, 1.5786,
        1.6827, 1.7869, 1.8910, 1.9952,
    ],
    dynamic_bonus: None,
};

const DORI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9076, 2.0290, 2.1678, 2.3586, 2.5494, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
    dynamic_bonus: None,
};

const DORI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5345, 2.7078, 2.9460, 3.1843, 3.4225, 3.6824, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: 鎮霊のランプ・トラブルシューター (Spirit-Warding Lamp: Troubleshooter Cannon) -- Electro --

const DORI_SKILL_SHOT: TalentScaling = TalentScaling {
    name: "トラブルシューター弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.4728, 1.5833, 1.6937, 1.8410, 1.9515, 2.0619, 2.2092, 2.3565, 2.5038, 2.6510, 2.7983,
        2.9456, 3.1297, 3.3138, 3.4979,
    ],
    dynamic_bonus: None,
};

const DORI_SKILL_AFTER_SALES: TalentScaling = TalentScaling {
    name: "アフターサービス弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3156, 0.3393, 0.3629, 0.3945, 0.4182, 0.4418, 0.4734, 0.5050, 0.5365, 0.5681, 0.5996,
        0.6312, 0.6707, 0.7101, 0.7496,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 卸カガラカの法契 (Alcazarzaray's Exactitude) -- Electro --

const DORI_BURST_CONNECTOR: TalentScaling = TalentScaling {
    name: "コネクターダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.1592, 0.1711, 0.1831, 0.1990, 0.2110, 0.2229, 0.2388, 0.2547, 0.2706, 0.2866, 0.3025,
        0.3184, 0.3383, 0.3582, 0.3781,
    ],
    dynamic_bonus: None,
};

pub const DORI: CharacterData = CharacterData {
    id: "dori",
    name: "Dori",
    element: Element::Electro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Sumeru,
    base_hp: [
        1039.00, 2670.00, 3447.00, 5163.00, 5715.00, 6573.00, 7309.00, 8168.00, 8719.00, 9577.00,
        10129.00, 10987.00, 11539.00, 12397.00, 12397.00, 12892.88, // Lv95/Lv95+/Lv100
        12892.88, // Lv95/Lv95+/Lv100
        13388.76, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        18.70, 48.04, 62.01, 92.88, 102.80, 118.25, 131.48, 146.93, 156.85, 172.28, 182.20, 197.65,
        207.57, 223.02, 223.02, 231.94, // Lv95/Lv95+/Lv100
        231.94, // Lv95/Lv95+/Lv100
        240.86, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        60.66, 155.83, 201.15, 301.29, 333.49, 383.60, 426.53, 476.63, 508.83, 558.87, 591.07,
        641.17, 673.37, 723.47, 723.47, 752.41, // Lv95/Lv95+/Lv100
        752.41, // Lv95/Lv95+/Lv100
        781.35, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "マーベラスソードダンス改",
            hits: &[DORI_NORMAL_1, DORI_NORMAL_2A, DORI_NORMAL_2B, DORI_NORMAL_3],
            charged: &[DORI_CHARGED_SPINNING, DORI_CHARGED_FINAL],
            plunging: &[DORI_PLUNGE, DORI_PLUNGE_LOW, DORI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "鎮霊のランプ・トラブルシューター",
            scalings: &[DORI_SKILL_SHOT, DORI_SKILL_AFTER_SALES],
        },
        elemental_burst: TalentData {
            name: "卸カガラカの法契",
            scalings: &[DORI_BURST_CONNECTOR],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
