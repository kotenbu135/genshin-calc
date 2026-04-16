use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Klee
// =============================================================================

// -- Normal Attack: ドッカンはなび (Kaboom!) -- Pyro (Catalyst) --

const KLEE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.7216, 0.7757, 0.8298, 0.9020, 0.9561, 1.0102, 1.0824, 1.1546, 1.2267, 1.2989, 1.3739,
        1.4721, 1.5702, 1.6683, 1.7665,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KLEE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.6240, 0.6708, 0.7176, 0.7800, 0.8268, 0.8736, 0.9360, 0.9984, 1.0608, 1.1232, 1.1881,
        1.2730, 1.3578, 1.4427, 1.5276,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KLEE_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.8992, 0.9666, 1.0341, 1.1240, 1.1914, 1.2589, 1.3488, 1.4387, 1.5286, 1.6186, 1.7121,
        1.8344, 1.9567, 2.0790, 2.2012,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Pyro (Catalyst) --

const KLEE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.5736, 1.6916, 1.8096, 1.9670, 2.0850, 2.2030, 2.3604, 2.5178, 2.6751, 2.8325, 2.9961,
        3.2101, 3.4242, 3.6382, 3.8522,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Pyro (Catalyst) --

const KLEE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KLEE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KLEE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: ボンボン爆弾 (Jumpy Dumpty) -- Pyro --

const KLEE_SKILL_BOUNCE: TalentScaling = TalentScaling {
    name: "ボンボン爆弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9520, 1.0234, 1.0948, 1.1900, 1.2614, 1.3328, 1.4280, 1.5232, 1.6184, 1.7136, 1.8088,
        1.9040, 2.0230, 2.1420, 2.2610,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KLEE_SKILL_MINE: TalentScaling = TalentScaling {
    name: "ブービートラップダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.3280, 0.3526, 0.3772, 0.4100, 0.4346, 0.4592, 0.4920, 0.5248, 0.5576, 0.5904, 0.6232,
        0.6560, 0.6970, 0.7380, 0.7790,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: ドッカン花火 (Sparks 'n' Splash) -- Pyro --

const KLEE_BURST: TalentScaling = TalentScaling {
    name: "ドッカン花火ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.4264, 0.4584, 0.4904, 0.5330, 0.5650, 0.5970, 0.6396, 0.6822, 0.7249, 0.7675, 0.8102,
        0.8528, 0.9061, 0.9594, 1.0127,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const KLEE: CharacterData = CharacterData {
    id: "klee",
    name: "Klee",
    element: Element::Pyro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [
        801.00, 2077.00, 2764.00, 4136.00, 4623.00, 5319.00, 5970.00, 6673.00, 7161.00, 7870.00,
        8358.00, 9076.00, 9563.00, 10287.00, 10287.00, 10698.48, // Lv95/Lv95+/Lv100
        10698.48, // Lv95/Lv95+/Lv100
        11109.96, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        24.21, 62.79, 83.54, 125.01, 139.75, 160.79, 180.45, 201.70, 216.45, 237.89, 252.64,
        274.33, 289.07, 310.93, 310.93, 323.37, // Lv95/Lv95+/Lv100
        323.37, // Lv95/Lv95+/Lv100
        335.80, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        47.86, 124.16, 165.20, 247.19, 276.35, 317.94, 356.82, 398.85, 428.01, 470.42, 499.58,
        542.46, 571.62, 614.84, 614.84, 639.43, // Lv95/Lv95+/Lv100
        639.43, // Lv95/Lv95+/Lv100
        664.03, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Pyro, 0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "ドッカンはなび",
            hits: &[KLEE_NORMAL_1, KLEE_NORMAL_2, KLEE_NORMAL_3],
            charged: &[KLEE_CHARGED],
            plunging: &[KLEE_PLUNGE, KLEE_PLUNGE_LOW, KLEE_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "ボンボン爆弾",
            scalings: &[KLEE_SKILL_BOUNCE, KLEE_SKILL_MINE],
        },
        elemental_burst: TalentData {
            name: "ドッカン花火",
            scalings: &[KLEE_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
};
