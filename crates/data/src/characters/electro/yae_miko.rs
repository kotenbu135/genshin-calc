use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Yae Miko
// =============================================================================

// -- Normal Attack: 狐霊食罪式 (Spiritfox Sin-Eater) -- All Electro (Catalyst) --

const YAE_MIKO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3966, 0.4263, 0.4561, 0.4957, 0.5255, 0.5552, 0.5949, 0.6345, 0.6742, 0.7139, 0.7535,
        0.7932, 0.8427, 0.8923, 0.9419,
    ],
    dynamic_bonus: None,
};

const YAE_MIKO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3852, 0.4141, 0.4430, 0.4815, 0.5104, 0.5393, 0.5778, 0.6163, 0.6548, 0.6933, 0.7319,
        0.7704, 0.8185, 0.8667, 0.9148,
    ],
    dynamic_bonus: None,
};

const YAE_MIKO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5689, 0.6116, 0.6542, 0.7111, 0.7538, 0.7964, 0.8533, 0.9102, 0.9671, 1.0240, 1.0809,
        1.1378, 1.2089, 1.2800, 1.3511,
    ],
    dynamic_bonus: None,
};

// -- Charged Attack -- Electro --

const YAE_MIKO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.4289, 1.5361, 1.6433, 1.7862, 1.8934, 2.0005, 2.1434, 2.2863, 2.4292, 2.5721, 2.7150,
        2.8579, 3.0365, 3.2151, 3.3938,
    ],
    dynamic_bonus: None,
};

// -- Plunging Attack -- Electro (Catalyst) --

const YAE_MIKO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
};

const YAE_MIKO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
};

const YAE_MIKO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
};

// -- Elemental Skill: 野干役呪・殺生桜 (Yakan Evocation: Sesshou Sakura) -- Electro --

const YAE_MIKO_SKILL_LV1: TalentScaling = TalentScaling {
    name: "殺生桜ダメージ・壱階",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.6067, 0.6522, 0.6977, 0.7584, 0.8039, 0.8494, 0.9101, 0.9708, 1.0314, 1.0921, 1.1528,
        1.2134, 1.2893, 1.3651, 1.4410,
    ],
    dynamic_bonus: None,
};

const YAE_MIKO_SKILL_LV2: TalentScaling = TalentScaling {
    name: "殺生桜ダメージ・弐階",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7584, 0.8153, 0.8722, 0.9480, 1.0049, 1.0618, 1.1376, 1.2134, 1.2893, 1.3651, 1.4410,
        1.5168, 1.6116, 1.7064, 1.8012,
    ],
    dynamic_bonus: None,
};

const YAE_MIKO_SKILL_LV3: TalentScaling = TalentScaling {
    name: "殺生桜ダメージ・参階",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.9480, 1.0191, 1.0902, 1.1850, 1.2561, 1.3272, 1.4220, 1.5168, 1.6116, 1.7064, 1.8012,
        1.8960, 2.0145, 2.1330, 2.2150,
    ],
    dynamic_bonus: None,
};

const YAE_MIKO_SKILL_LV4: TalentScaling = TalentScaling {
    name: "殺生桜ダメージ・肆階",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.1850, 1.2739, 1.3628, 1.4813, 1.5701, 1.6590, 1.7775, 1.8960, 2.0145, 2.1330, 2.2515,
        2.3700, 2.5181, 2.6663, 2.8144,
    ],
    dynamic_bonus: None,
};

// -- Elemental Burst: 大密法・天狐顕真 (Great Secret Art: Tenko Kenshin) -- Electro --

const YAE_MIKO_BURST_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        2.6000, 2.7950, 2.9900, 3.2500, 3.4450, 3.6400, 3.9000, 4.1600, 4.4200, 4.6800, 4.9400,
        5.2000, 5.5250, 5.8500, 6.1750,
    ],
    dynamic_bonus: None,
};

const YAE_MIKO_BURST_TENKO: TalentScaling = TalentScaling {
    name: "天狐雷霆ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        3.3382, 3.5885, 3.8389, 4.1727, 4.4231, 4.6734, 5.0072, 5.3411, 5.6749, 6.0087, 6.3425,
        6.6763, 7.0936, 7.5109, 7.9281,
    ],
    dynamic_bonus: None,
};

pub const YAE_MIKO: CharacterData = CharacterData {
    id: "yae_miko",
    name: "Yae Miko",
    element: Element::Electro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [
        807.00, 2095.00, 2787.00, 4170.00, 4662.00, 5364.00, 6020.00, 6729.00, 7220.00, 7936.00,
        8428.00, 9151.00, 9643.00, 10372.00, 10372.00, 10786.88, // Lv95/Lv95+/Lv100
        10786.88, // Lv95/Lv95+/Lv100
        11201.76, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        26.44, 68.59, 91.25, 136.55, 152.65, 175.63, 197.11, 220.32, 236.43, 259.85, 275.96,
        299.65, 315.76, 339.63, 339.63, 353.22, // Lv95/Lv95+/Lv100
        353.22, // Lv95/Lv95+/Lv100
        366.80, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        44.27, 114.85, 152.81, 228.65, 255.62, 294.10, 330.06, 368.94, 395.91, 435.13, 462.11,
        501.78, 528.75, 568.73, 568.73, 591.48, // Lv95/Lv95+/Lv100
        591.48, // Lv95/Lv95+/Lv100
        614.23, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "狐霊食罪式",
            hits: &[YAE_MIKO_NORMAL_1, YAE_MIKO_NORMAL_2, YAE_MIKO_NORMAL_3],
            charged: &[YAE_MIKO_CHARGED],
            plunging: &[YAE_MIKO_PLUNGE, YAE_MIKO_PLUNGE_LOW, YAE_MIKO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "野干役呪・殺生桜",
            scalings: &[
                YAE_MIKO_SKILL_LV1,
                YAE_MIKO_SKILL_LV2,
                YAE_MIKO_SKILL_LV3,
                YAE_MIKO_SKILL_LV4,
            ],
        },
        elemental_burst: TalentData {
            name: "大密法・天狐顕真",
            scalings: &[YAE_MIKO_BURST_SKILL, YAE_MIKO_BURST_TENKO],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
