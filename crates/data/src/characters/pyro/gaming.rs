use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Gaming
// =============================================================================

// -- Normal Attack: 星辰裂き (Stellar Rend) -- Physical --

const GAMING_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8386, 0.9068, 0.9751, 1.0726, 1.1408, 1.2188, 1.3261, 1.4334, 1.5406, 1.6576, 1.7746,
        1.8916, 2.0086, 2.1257, 2.2427,
    ],
    dynamic_bonus: None,
};

const GAMING_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7904, 0.8548, 0.9191, 1.0110, 1.0754, 1.1489, 1.2500, 1.3511, 1.4522, 1.5625, 1.6728,
        1.7831, 1.8934, 2.0037, 2.1140,
    ],
    dynamic_bonus: None,
};

const GAMING_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0665, 1.1533, 1.2401, 1.3641, 1.4509, 1.5501, 1.6865, 1.8229, 1.9593, 2.1081, 2.2569,
        2.4057, 2.5545, 2.7034, 2.8522,
    ],
    dynamic_bonus: None,
};

const GAMING_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2795, 1.3836, 1.4878, 1.6366, 1.7407, 1.8597, 2.0234, 2.1870, 2.3507, 2.5292, 2.7078,
        2.8863, 3.0648, 3.2434, 3.4219,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Physical --

const GAMING_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6252, 0.6761, 0.7270, 0.7997, 0.8506, 0.9088, 0.9887, 1.0687, 1.1487, 1.2359, 1.3231,
        1.4104, 1.4976, 1.5849, 1.6721,
    ],
    dynamic_bonus: None,
};

const GAMING_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1309, 1.2230, 1.3150, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933,
        2.5511, 2.7089, 2.8667, 3.0245,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Physical --

const GAMING_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6415, 0.6937, 0.7459, 0.8205, 0.8727, 0.9324, 1.0144, 1.0964, 1.1785, 1.2680, 1.3575,
        1.4470, 1.5365, 1.6260, 1.7155,
    ],
    dynamic_bonus: None,
};

const GAMING_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2826, 1.3870, 1.4914, 1.6406, 1.7450, 1.8643, 2.0284, 2.1924, 2.3565, 2.5354, 2.7144,
        2.8934, 3.0724, 3.2513, 3.4303,
    ],
    dynamic_bonus: None,
};

const GAMING_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.6021, 1.7325, 1.8629, 2.0492, 2.1796, 2.3286, 2.5335, 2.7384, 2.9434, 3.1669, 3.3905,
        3.6140, 3.8376, 4.0611, 4.2846,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: 瑞獣登場 (Bestial Ascent) -- Pyro --

const GAMING_SKILL_PLUNGE: TalentScaling = TalentScaling {
    name: "落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        2.3040, 2.4768, 2.6496, 2.8800, 3.0528, 3.2256, 3.4560, 3.6864, 3.9168, 4.1472, 4.3776,
        4.6080, 4.8960, 5.1840, 5.4720,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 瑞獣の金舞 (Suanni's Gilded Dance) -- Pyro --

const GAMING_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        3.7040, 3.9818, 4.2596, 4.6300, 4.9078, 5.1856, 5.5560, 5.9264, 6.2968, 6.6672, 7.0376,
        7.4080, 7.8710, 8.3340, 8.7970,
    ],
    dynamic_bonus: None,
};

pub const GAMING: CharacterData = CharacterData {
    id: "gaming",
    name: "Gaming",
    element: Element::Pyro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [
        957.00, 2460.00, 3175.00, 4755.00, 5264.00, 6054.00, 6732.00, 7523.00, 8031.00, 8821.00,
        9329.00, 10120.00, 10628.00, 11419.00, 11419.00, 11875.76, // Lv95/Lv95+/Lv100
        11875.76, // Lv95/Lv95+/Lv100
        12332.52, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        25.29, 64.96, 83.85, 125.60, 139.03, 159.91, 177.81, 198.70, 212.12, 232.98, 246.41,
        267.29, 280.72, 301.60, 301.60, 313.66, // Lv95/Lv95+/Lv100
        313.66, // Lv95/Lv95+/Lv100
        325.73, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        58.94, 151.42, 195.45, 292.77, 324.05, 372.74, 414.45, 463.14, 494.43, 543.05, 574.34,
        623.03, 654.31, 703.00, 703.00, 731.12, // Lv95/Lv95+/Lv100
        731.12, // Lv95/Lv95+/Lv100
        759.24, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "星辰裂き",
            hits: &[
                GAMING_NORMAL_1,
                GAMING_NORMAL_2,
                GAMING_NORMAL_3,
                GAMING_NORMAL_4,
            ],
            charged: &[GAMING_CHARGED_SPINNING, GAMING_CHARGED_FINAL],
            plunging: &[GAMING_PLUNGE, GAMING_PLUNGE_LOW, GAMING_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "瑞獣登場",
            scalings: &[GAMING_SKILL_PLUNGE],
        },
        elemental_burst: TalentData {
            name: "瑞獣の金舞",
            scalings: &[GAMING_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
