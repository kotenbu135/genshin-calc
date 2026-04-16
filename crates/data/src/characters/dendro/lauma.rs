use crate::types::*;
use genshin_calc_core::{Element, Reaction, ScalingStat};

// =============================================================================

// --- Normal Attack: Peregrination of Linnunrata (all Dendro — catalyst) ---

const LAUMA_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.337024, 0.362301, 0.387578, 0.42128, 0.446557, 0.471834, 0.505536, 0.539238, 0.572941,
        0.606643, 0.640346, 0.674048, 0.716176, 0.758304, 0.800432,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAUMA_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.318048, 0.341902, 0.365755, 0.39756, 0.421414, 0.445267, 0.477072, 0.508877, 0.540682,
        0.572486, 0.604291, 0.636096, 0.675852, 0.715608, 0.755364,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAUMA_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.444968, 0.478341, 0.511713, 0.55621, 0.589583, 0.622955, 0.667452, 0.711949, 0.756446,
        0.800942, 0.845439, 0.889936, 0.945557, 1.001178, 1.056799,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAUMA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.2904, 1.38718, 1.48396, 1.613, 1.70978, 1.80656, 1.9356, 2.06464, 2.19368, 2.32272,
        2.45176, 2.5808, 2.7421, 2.9034, 3.0647,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAUMA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAUMA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAUMA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Elemental Skill: Runo: Dawnless Rest of Karsikko ---

const LAUMA_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "狩りの讃歌ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.216, 1.3072, 1.3984, 1.52, 1.6112, 1.7024, 1.824, 1.9456, 2.0672, 2.1888, 2.3104, 2.432,
        2.584, 2.736, 2.888,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAUMA_SKILL_HOLD_HIT1: TalentScaling = TalentScaling {
    name: "永眠の讃歌ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.5808, 1.69936, 1.81792, 1.976, 2.09456, 2.21312, 2.3712, 2.52928, 2.68736, 2.84544,
        3.00352, 3.1616, 3.3592, 3.5568, 3.7544,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAUMA_SKILL_HOLD_HIT2: TalentScaling = TalentScaling {
    name: "月開花ダメージ",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        1.52, 1.634, 1.748, 1.9, 2.014, 2.128, 2.28, 2.432, 2.584, 2.736, 2.888, 3.04, 3.23, 3.42,
        3.61,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::DirectLunar(Reaction::LunarBloom),
};

const LAUMA_SKILL_SANCTUARY_ATK: TalentScaling = TalentScaling {
    name: "霜林聖域攻撃ダメージ・攻撃力",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.96, 1.032, 1.104, 1.2, 1.272, 1.344, 1.44, 1.536, 1.632, 1.728, 1.824, 1.92, 2.04, 2.16,
        2.28,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAUMA_SKILL_SANCTUARY_EM: TalentScaling = TalentScaling {
    name: "霜林聖域攻撃ダメージ・元素熟知",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        1.92, 2.064, 2.208, 2.4, 2.544, 2.688, 2.88, 3.072, 3.264, 3.456, 3.648, 3.84, 4.08, 4.32,
        4.56,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Elemental Burst: Runo: All Hearts Become the Beating Moon ---

const LAUMA_BURST_BLOOM_INCREASE: TalentScaling = TalentScaling {
    name: "開花/超開花/烈開花ダメージ増加",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        2.7776, 2.98592, 3.19424, 3.472, 3.68032, 3.88864, 4.1664, 4.44416, 4.72192, 4.99968,
        5.27744, 5.5552, 5.9024, 6.2496, 6.5968,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const LAUMA_BURST_LUNAR_BLOOM_INCREASE: TalentScaling = TalentScaling {
    name: "月開花ダメージ増加",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        2.2224, 2.38908, 2.55576, 2.778, 2.94468, 3.11136, 3.3336, 3.55584, 3.77808, 4.00032,
        4.22256, 4.4448, 4.7226, 5.0004, 5.2782,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// --- Lauma aggregation ---

static LAUMA_NA_HITS: &[TalentScaling] = &[LAUMA_NA_HIT1, LAUMA_NA_HIT2, LAUMA_NA_HIT3];
static LAUMA_CHARGED_ATTACKS: &[TalentScaling] = &[LAUMA_CHARGED];
static LAUMA_PLUNGING: &[TalentScaling] = &[LAUMA_PLUNGE, LAUMA_PLUNGE_LOW, LAUMA_PLUNGE_HIGH];
static LAUMA_SKILL_SCALINGS: &[TalentScaling] = &[
    LAUMA_SKILL_PRESS,
    LAUMA_SKILL_HOLD_HIT1,
    LAUMA_SKILL_HOLD_HIT2,
    LAUMA_SKILL_SANCTUARY_ATK,
    LAUMA_SKILL_SANCTUARY_EM,
];
static LAUMA_BURST_SCALINGS: &[TalentScaling] =
    &[LAUMA_BURST_BLOOM_INCREASE, LAUMA_BURST_LUNAR_BLOOM_INCREASE];

pub const LAUMA: CharacterData = CharacterData {
    id: "lauma",
    name: "Lauma",
    element: Element::Dendro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::NodKrai,
    base_hp: [
        829.00, 2151.00, 2863.00, 4283.00, 4789.00, 5509.00, 6183.00, 6911.00, 7416.00, 8151.00,
        8657.00, 9400.00, 9905.00, 10654.00, 10654.00, 11080.16, // Lv95/Lv95+/Lv100
        11080.16, // Lv95/Lv95+/Lv100
        11506.32, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        19.85, 51.49, 68.51, 102.51, 114.60, 131.85, 147.97, 165.40, 177.49, 195.07, 207.16,
        224.95, 237.04, 254.96, 254.96, 265.16, // Lv95/Lv95+/Lv100
        265.16, // Lv95/Lv95+/Lv100
        275.36, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        52.05, 135.02, 179.65, 268.82, 300.53, 345.76, 388.05, 433.75, 465.46, 511.58, 543.29,
        589.93, 621.64, 668.64, 668.64, 695.39, // Lv95/Lv95+/Lv100
        695.39, // Lv95/Lv95+/Lv100
        722.13, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalMastery(115.2),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "リンヌンラタ巡礼",
            hits: LAUMA_NA_HITS,
            charged: LAUMA_CHARGED_ATTACKS,
            plunging: LAUMA_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "ルノ：カルシッコの陽なき安息",
            scalings: LAUMA_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "ルノ：万心鼓動たる月",
            scalings: LAUMA_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
