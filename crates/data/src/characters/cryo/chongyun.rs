use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Demonbane -- Physical --

const CHONGYUN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7000, 0.7570, 0.8140, 0.8954, 0.9524, 1.0175, 1.1070, 1.1966, 1.2861, 1.3838, 1.4815,
        1.5792, 1.6768, 1.7745, 1.8722,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHONGYUN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6312, 0.6826, 0.7340, 0.8074, 0.8588, 0.9175, 0.9982, 1.0790, 1.1597, 1.2478, 1.3359,
        1.4240, 1.5120, 1.6001, 1.6882,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHONGYUN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8032, 0.8686, 0.9340, 1.0274, 1.0928, 1.1675, 1.2702, 1.3730, 1.4757, 1.5878, 1.6999,
        1.8120, 1.9240, 2.0361, 2.1482,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHONGYUN_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0122, 1.0946, 1.1770, 1.2947, 1.3771, 1.4713, 1.6007, 1.7302, 1.8597, 2.0009, 2.1421,
        2.2834, 2.4246, 2.5659, 2.7071,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const CHONGYUN_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5629, 0.6087, 0.6545, 0.7199, 0.7657, 0.8181, 0.8901, 0.9621, 1.0341, 1.1126, 1.1912,
        1.2697, 1.3482, 1.4268, 1.5053,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHONGYUN_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0178, 1.1007, 1.1835, 1.3019, 1.3847, 1.4794, 1.6096, 1.7397, 1.8699, 2.0120, 2.1540,
        2.2960, 2.4380, 2.5800, 2.7221,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const CHONGYUN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHONGYUN_PLUNGE_LOW: TalentScaling = TalentScaling {
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

const CHONGYUN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9460, 3.1842, 3.4225, 3.6825, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: Spirit Blade: Chonghua's Layered Frost -- Cryo --

const CHONGYUN_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.7204, 1.8494, 1.9785, 2.1505, 2.2795, 2.4086, 2.5806, 2.7526, 2.9247, 3.0967, 3.2688,
        3.4408, 3.6559, 3.8709, 4.0860,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: Spirit Blade: Cloud-Parting Star -- Cryo --

const CHONGYUN_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.4240, 1.5308, 1.6376, 1.7800, 1.8868, 1.9936, 2.1360, 2.2784, 2.4208, 2.5632, 2.7056,
        2.8480, 3.0260, 3.2040, 3.3820,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const CHONGYUN: CharacterData = CharacterData {
    id: "chongyun",
    name: "Chongyun",
    element: Element::Cryo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [
        921.00, 2366.00, 3054.00, 4574.00, 5063.00, 5824.00, 6475.00, 7236.00, 7725.00, 8485.00,
        8974.00, 9734.00, 10223.00, 10984.00, 10984.00, 11423.36, // Lv95/Lv95+/Lv100
        11423.36, // Lv95/Lv95+/Lv100
        11862.72, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        18.70, 48.04, 62.01, 92.88, 102.80, 118.25, 131.48, 146.93, 156.85, 172.28, 182.20, 197.65,
        207.57, 223.02, 223.02, 231.94, // Lv95/Lv95+/Lv100
        231.94, // Lv95/Lv95+/Lv100
        240.86, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        54.36, 139.66, 180.27, 270.03, 298.88, 343.79, 382.26, 427.17, 456.02, 500.87, 529.73,
        574.63, 603.49, 648.40, 648.40, 674.34, // Lv95/Lv95+/Lv100
        674.34, // Lv95/Lv95+/Lv100
        700.27, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "滅魔",
            hits: &[
                CHONGYUN_NORMAL_1,
                CHONGYUN_NORMAL_2,
                CHONGYUN_NORMAL_3,
                CHONGYUN_NORMAL_4,
            ],
            charged: &[CHONGYUN_CHARGED_SPINNING, CHONGYUN_CHARGED_FINAL],
            plunging: &[CHONGYUN_PLUNGE, CHONGYUN_PLUNGE_LOW, CHONGYUN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "霊刃・重華積霜",
            scalings: &[CHONGYUN_SKILL],
        },
        elemental_burst: TalentData {
            name: "霊刃・雲開星落",
            scalings: &[CHONGYUN_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Citlali
