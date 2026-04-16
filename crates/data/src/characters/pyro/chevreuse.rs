use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Chevreuse
// =============================================================================

// -- Normal Attack: ラインバヨネット突撃EX (Line Bayonet Thrust EX) -- Physical --

const CHEVREUSE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5313, 0.5745, 0.6178, 0.6796, 0.7228, 0.7722, 0.8402, 0.9082, 0.9761, 1.0502, 1.1244,
        1.1985, 1.2726, 1.3468, 1.4209,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHEVREUSE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4931, 0.5332, 0.5734, 0.6307, 0.6709, 0.7167, 0.7798, 0.8429, 0.9059, 0.9747, 1.0436,
        1.1124, 1.1812, 1.2500, 1.3188,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHEVREUSE_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2764, 0.2990, 0.3215, 0.3536, 0.3761, 0.4018, 0.4372, 0.4725, 0.5079, 0.5465, 0.5850,
        0.6236, 0.6622, 0.7008, 0.7393,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHEVREUSE_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3245, 0.3509, 0.3774, 0.4151, 0.4415, 0.4717, 0.5132, 0.5547, 0.5962, 0.6415, 0.6868,
        0.7321, 0.7774, 0.8226, 0.8679,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHEVREUSE_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7726, 0.8355, 0.8984, 0.9882, 1.0511, 1.1230, 1.2218, 1.3206, 1.4195, 1.5273, 1.6351,
        1.7429, 1.8507, 1.9585, 2.0663,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const CHEVREUSE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2169, 1.3160, 1.4150, 1.5565, 1.6556, 1.7688, 1.9244, 2.0801, 2.2357, 2.4055, 2.5753,
        2.7451, 2.9149, 3.0847, 3.2545,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const CHEVREUSE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHEVREUSE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHEVREUSE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 近距離急速射撃 (Short-Range Rapid Interdiction Fire) -- Pyro --

const CHEVREUSE_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "1回押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1520, 1.2384, 1.3248, 1.4400, 1.5264, 1.6128, 1.7280, 1.8432, 1.9584, 2.0736, 2.1888,
        2.3040, 2.4480, 2.5920, 2.7360,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHEVREUSE_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.7280, 1.8576, 1.9872, 2.1600, 2.2896, 2.4192, 2.5920, 2.7648, 2.9376, 3.1104, 3.2832,
        3.4560, 3.6720, 3.8880, 4.1040,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHEVREUSE_SKILL_OVERCHARGED: TalentScaling = TalentScaling {
    name: "過充填弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        2.8240, 3.0358, 3.2476, 3.5300, 3.7418, 3.9536, 4.2360, 4.5184, 4.8008, 5.0832, 5.3656,
        5.6480, 6.0010, 6.3540, 6.7070,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 榴弾リング (Ring of Bursting Grenades) -- Pyro --

const CHEVREUSE_BURST_GRENADE: TalentScaling = TalentScaling {
    name: "爆発グレネードダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        3.6816, 3.9577, 4.2338, 4.6020, 4.8781, 5.1542, 5.5224, 5.8906, 6.2587, 6.6269, 6.9950,
        7.3632, 7.8234, 8.2836, 8.7438,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const CHEVREUSE_BURST_SECONDARY: TalentScaling = TalentScaling {
    name: "二次爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.4909, 0.5277, 0.5645, 0.6136, 0.6504, 0.6872, 0.7363, 0.7854, 0.8345, 0.8836, 0.9327,
        0.9818, 1.0431, 1.1045, 1.1658,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const CHEVREUSE: CharacterData = CharacterData {
    id: "chevreuse",
    name: "Chevreuse",
    element: Element::Pyro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Fontaine,
    base_hp: [
        1003.00, 2577.00, 3326.00, 4982.00, 5514.00, 6343.00, 7052.00, 7881.00, 8413.00, 9241.00,
        9773.00, 10602.00, 11134.00, 11962.00, 11962.00, 12440.48, // Lv95/Lv95+/Lv100
        12440.48, // Lv95/Lv95+/Lv100
        12918.96, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        16.21, 41.63, 53.74, 80.49, 89.09, 102.48, 113.95, 127.34, 135.94, 149.31, 157.91, 171.29,
        179.90, 193.28, 193.28, 201.01, // Lv95/Lv95+/Lv100
        201.01, // Lv95/Lv95+/Lv100
        208.74, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        50.70, 130.25, 168.13, 251.84, 278.75, 320.63, 356.51, 398.39, 425.30, 467.13, 494.04,
        535.92, 562.83, 604.71, 604.71, 628.90, // Lv95/Lv95+/Lv100
        628.90, // Lv95/Lv95+/Lv100
        653.09, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "ラインバヨネット突撃EX",
            hits: &[
                CHEVREUSE_NORMAL_1,
                CHEVREUSE_NORMAL_2,
                CHEVREUSE_NORMAL_3A,
                CHEVREUSE_NORMAL_3B,
                CHEVREUSE_NORMAL_4,
            ],
            charged: &[CHEVREUSE_CHARGED],
            plunging: &[
                CHEVREUSE_PLUNGE,
                CHEVREUSE_PLUNGE_LOW,
                CHEVREUSE_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "近距離急速射撃",
            scalings: &[
                CHEVREUSE_SKILL_PRESS,
                CHEVREUSE_SKILL_HOLD,
                CHEVREUSE_SKILL_OVERCHARGED,
            ],
        },
        elemental_burst: TalentData {
            name: "榴弾リング",
            scalings: &[CHEVREUSE_BURST_GRENADE, CHEVREUSE_BURST_SECONDARY],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
    scaling_modifiers: &[],
};
