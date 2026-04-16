use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Spear of Favonius - Arrow's Passage -- Physical --

const MIKA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4326, 0.4678, 0.5031, 0.5534, 0.5886, 0.6288, 0.6842, 0.7395, 0.7948, 0.8552, 0.9156,
        0.9759, 1.0363, 1.0967, 1.1570,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const MIKA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4150, 0.4488, 0.4826, 0.5308, 0.5646, 0.6032, 0.6563, 0.7094, 0.7625, 0.8204, 0.8783,
        0.9362, 0.9941, 1.0520, 1.1099,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const MIKA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5450, 0.5894, 0.6338, 0.6971, 0.7415, 0.7922, 0.8619, 0.9316, 1.0013, 1.0774, 1.1534,
        1.2295, 1.3055, 1.3816, 1.4576,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const MIKA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2761, 0.2986, 0.3211, 0.3532, 0.3757, 0.4014, 0.4367, 0.4720, 0.5073, 0.5459, 0.5844,
        0.6229, 0.6615, 0.7000, 0.7385,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const MIKA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7087, 0.7664, 0.8241, 0.9065, 0.9642, 1.0302, 1.1208, 1.2115, 1.3021, 1.4010, 1.4999,
        1.5988, 1.6977, 1.7966, 1.8955,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const MIKA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1275, 1.2192, 1.3110, 1.4421, 1.5339, 1.6388, 1.7830, 1.9272, 2.0714, 2.2287, 2.3860,
        2.5433, 2.7007, 2.8580, 3.0153,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const MIKA_PLUNGE: TalentScaling = TalentScaling {
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

const MIKA_PLUNGE_LOW: TalentScaling = TalentScaling {
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

const MIKA_PLUNGE_HIGH: TalentScaling = TalentScaling {
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

// -- Elemental Skill: Starfrost Swirl -- Cryo --

const MIKA_SKILL_ARROW: TalentScaling = TalentScaling {
    name: "霜流矢ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.6720, 0.7224, 0.7728, 0.8400, 0.8904, 0.9408, 1.0080, 1.0752, 1.1424, 1.2096, 1.2768,
        1.3440, 1.4280, 1.5120, 1.5960,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const MIKA_SKILL_FLARE: TalentScaling = TalentScaling {
    name: "氷星フレアダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.8400, 0.9030, 0.9660, 1.0500, 1.1130, 1.1760, 1.2600, 1.3440, 1.4280, 1.5120, 1.5960,
        1.6800, 1.7850, 1.8900, 1.9950,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const MIKA_SKILL_SHARD: TalentScaling = TalentScaling {
    name: "氷星の欠片ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.2520, 0.2709, 0.2898, 0.3150, 0.3339, 0.3528, 0.3780, 0.4032, 0.4284, 0.4536, 0.4788,
        0.5040, 0.5355, 0.5670, 0.5985,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: Skyfeather Song -- Cryo (healing, no damage scalings) --
// Note: Mika's burst is healing-only with no damage component

pub const MIKA: CharacterData = CharacterData {
    id: "mika",
    name: "Mika",
    element: Element::Cryo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [
        1049.00, 2694.00, 3477.00, 5208.00, 5765.00, 6631.00, 7373.00, 8239.00, 8796.00, 9661.00,
        10217.00, 11083.00, 11640.00, 12506.00, 12506.00, 13006.24, // Lv95/Lv95+/Lv100
        13006.24, // Lv95/Lv95+/Lv100
        13506.48, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        18.70, 48.04, 62.01, 92.88, 102.80, 118.25, 131.48, 146.93, 156.85, 172.28, 182.20, 197.65,
        207.57, 223.02, 223.02, 231.94, // Lv95/Lv95+/Lv100
        231.94, // Lv95/Lv95+/Lv100
        240.86, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        59.80, 153.63, 198.30, 297.03, 328.77, 378.17, 420.49, 469.88, 501.63, 550.96, 582.70,
        632.10, 663.84, 713.23, 713.23, 741.76, // Lv95/Lv95+/Lv100
        741.76, // Lv95/Lv95+/Lv100
        770.29, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "西風槍術・矢の道",
            hits: &[
                MIKA_NORMAL_1,
                MIKA_NORMAL_2,
                MIKA_NORMAL_3,
                MIKA_NORMAL_4,
                MIKA_NORMAL_5,
            ],
            charged: &[MIKA_CHARGED],
            plunging: &[MIKA_PLUNGE, MIKA_PLUNGE_LOW, MIKA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "星霜の渦",
            scalings: &[MIKA_SKILL_ARROW, MIKA_SKILL_FLARE, MIKA_SKILL_SHARD],
        },
        elemental_burst: TalentData {
            name: "天羽の歌",
            scalings: &[],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
    passive_scalings: &[],
};

// =============================================================================
// Qiqi
