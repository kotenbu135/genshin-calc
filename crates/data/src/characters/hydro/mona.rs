use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 因果点破 (Ripple of Fate) -- All Hydro (Catalyst) --

const MONA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3760, 0.4042, 0.4324, 0.4700, 0.4982, 0.5264, 0.5640, 0.6016, 0.6392, 0.6768, 0.7144,
        0.7520, 0.7990, 0.8460, 0.8930,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const MONA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3600, 0.3870, 0.4140, 0.4500, 0.4770, 0.5040, 0.5400, 0.5760, 0.6120, 0.6480, 0.6840,
        0.7200, 0.7650, 0.8100, 0.8550,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const MONA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.4480, 0.4816, 0.5152, 0.5600, 0.5936, 0.6272, 0.6720, 0.7168, 0.7616, 0.8064, 0.8512,
        0.8960, 0.9520, 1.0080, 1.0640,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const MONA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5616, 0.6037, 0.6458, 0.7020, 0.7441, 0.7862, 0.8424, 0.8986, 0.9547, 1.0109, 1.0670,
        1.1232, 1.1934, 1.2636, 1.3338,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Hydro (Catalyst) --

const MONA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4972, 1.6095, 1.7218, 1.8715, 1.9838, 2.0961, 2.2458, 2.3955, 2.5452, 2.6950, 2.8507,
        3.0543, 3.2579, 3.4615, 3.6651,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Hydro (Catalyst) --

const MONA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const MONA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const MONA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 水中幻願 (Reflection of Doom) -- Hydro --

const MONA_SKILL_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3200, 0.3440, 0.3680, 0.4000, 0.4240, 0.4480, 0.4800, 0.5120, 0.5440, 0.5760, 0.6080,
        0.6400, 0.6800, 0.7200, 0.7600,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const MONA_SKILL_EXPLOSION: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.3280, 1.4276, 1.5272, 1.6600, 1.7596, 1.8592, 1.9920, 2.1248, 2.2576, 2.3904, 2.5232,
        2.6560, 2.8220, 2.9880, 3.1540,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 星命定軌 (Stellaris Phantasm) -- Hydro --

const MONA_BURST_BUBBLE: TalentScaling = TalentScaling {
    name: "泡影破裂ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        4.4240, 4.7558, 5.0876, 5.5300, 5.8618, 6.1936, 6.6360, 7.0784, 7.5208, 7.9632, 8.4056,
        8.8480, 9.4010, 9.9540, 10.5070,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const MONA: CharacterData = CharacterData {
    id: "mona",
    name: "Mona",
    element: Element::Hydro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [
        810.00, 2102.00, 2797.00, 4185.00, 4678.00, 5383.00, 6041.00, 6752.00, 7246.00, 7964.00,
        8458.00, 9184.00, 9677.00, 10409.00, 10409.00, 10825.36, // Lv95/Lv95+/Lv100
        10825.36, // Lv95/Lv95+/Lv100
        11241.72, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        22.34, 57.96, 77.12, 115.39, 129.00, 148.42, 166.57, 186.19, 199.80, 219.59, 233.21,
        253.23, 266.84, 287.01, 287.01, 298.49, // Lv95/Lv95+/Lv100
        298.49, // Lv95/Lv95+/Lv100
        309.97, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        50.86, 131.92, 175.52, 262.64, 293.62, 337.82, 379.13, 423.78, 454.76, 499.82, 530.80,
        576.37, 607.35, 653.27, 653.27, 679.40, // Lv95/Lv95+/Lv100
        679.40, // Lv95/Lv95+/Lv100
        705.53, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::EnergyRecharge(0.32),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "因果点破",
            hits: &[MONA_NORMAL_1, MONA_NORMAL_2, MONA_NORMAL_3, MONA_NORMAL_4],
            charged: &[MONA_CHARGED],
            plunging: &[MONA_PLUNGE, MONA_PLUNGE_LOW, MONA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "水中幻願",
            scalings: &[MONA_SKILL_DOT, MONA_SKILL_EXPLOSION],
        },
        elemental_burst: TalentData {
            name: "星命定軌",
            scalings: &[MONA_BURST_BUBBLE],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
    passive_scalings: &[],
    scaling_modifiers: &[],
};
