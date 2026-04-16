use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Black Pheasant Strides on Water -- All Anemo (Catalyst) --

const LAN_YAN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.4144, 0.44548, 0.47656, 0.518, 0.54908, 0.58016, 0.6216, 0.66304, 0.70448, 0.74592,
        0.78736, 0.8288, 0.8806, 0.9324, 0.9842,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAN_YAN_NORMAL_2A: TalentScaling = TalentScaling {
    name: "2段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.20412, 0.219429, 0.234738, 0.25515, 0.270459, 0.285768, 0.30618, 0.326592, 0.347004,
        0.367416, 0.387828, 0.40824, 0.433755, 0.45927, 0.484785,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAN_YAN_NORMAL_2B: TalentScaling = TalentScaling {
    name: "2段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.24948, 0.268191, 0.286902, 0.31185, 0.330561, 0.349272, 0.37422, 0.399168, 0.424116,
        0.449064, 0.474012, 0.49896, 0.530145, 0.56133, 0.592515,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAN_YAN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.2692, 0.28939, 0.30958, 0.3365, 0.35669, 0.37688, 0.4038, 0.43072, 0.45764, 0.48456,
        0.51148, 0.5384, 0.57205, 0.6057, 0.63935,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAN_YAN_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.6456, 0.69402, 0.74244, 0.807, 0.85542, 0.90384, 0.9684, 1.03296, 1.09752, 1.16208,
        1.22664, 1.2912, 1.3719, 1.4526, 1.5333,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Anemo (Catalyst) --

const LAN_YAN_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ(x3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.3784, 0.40678, 0.43516, 0.473, 0.50138, 0.52976, 0.5676, 0.60544, 0.64328, 0.68112,
        0.71896, 0.7568, 0.8041, 0.8514, 0.8987,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Anemo (Catalyst) --

const LAN_YAN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAN_YAN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAN_YAN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: Swallow-Wisp Pinion Dance -- Anemo --

const LAN_YAN_SKILL: TalentScaling = TalentScaling {
    name: "羽月環ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.96256, 1.034752, 1.106944, 1.2032, 1.275392, 1.347584, 1.44384, 1.540096, 1.636352,
        1.732608, 1.828864, 1.92512, 2.04544, 2.16576, 2.28608,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: Lustrous Moonrise -- Anemo --

const LAN_YAN_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ(x3)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.41064, 2.591438, 2.772236, 3.0133, 3.194098, 3.374896, 3.61596, 3.857024, 4.098088,
        4.339152, 4.580216, 4.82128, 5.12261, 5.42394, 5.72527,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const LAN_YAN: CharacterData = CharacterData {
    id: "lan_yan",
    name: "Lan Yan",
    element: Element::Anemo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [
        775.00, 1991.00, 2570.00, 3850.00, 4261.00, 4901.00, 5450.00, 6090.00, 6501.00, 7141.00,
        7552.00, 8192.00, 8604.00, 9244.00, 9244.00, 9613.76, // Lv95/Lv95+/Lv100
        9613.76, // Lv95/Lv95+/Lv100
        9983.52, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        21.01, 53.98, 69.68, 104.38, 115.53, 132.89, 147.76, 165.12, 176.27, 193.61, 204.76,
        222.12, 233.27, 250.63, 250.63, 260.66, // Lv95/Lv95+/Lv100
        260.66, // Lv95/Lv95+/Lv100
        270.68, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        48.64, 124.96, 161.30, 241.60, 267.42, 307.60, 342.02, 382.20, 408.02, 448.15, 473.97,
        514.15, 539.97, 580.14, 580.14, 603.35, // Lv95/Lv95+/Lv100
        603.35, // Lv95/Lv95+/Lv100
        626.55, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "黒雉水上歩",
            hits: &[
                LAN_YAN_NORMAL_1,
                LAN_YAN_NORMAL_2A,
                LAN_YAN_NORMAL_2B,
                LAN_YAN_NORMAL_3,
                LAN_YAN_NORMAL_4,
            ],
            charged: &[LAN_YAN_CHARGED],
            plunging: &[LAN_YAN_PLUNGE, LAN_YAN_PLUNGE_LOW, LAN_YAN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "呑霊羽舞",
            scalings: &[LAN_YAN_SKILL],
        },
        elemental_burst: TalentData {
            name: "朗月昇る",
            scalings: &[LAN_YAN_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
    scaling_modifiers: &[],
};

// =============================================================================
// Lynette — 4★ Anemo Sword (Fontaine)
// Source: genshin-db-api
// Normal Attack: Rapid Ritesword (迅捷な儀刃)
// Elemental Skill: Enigmatic Feint (エニグマティック・フェイント)
// Elemental Burst: Magic Trick: Astonishing Shift (マジック・アストニッシングシフト)
