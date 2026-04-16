use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 神里流・転 (Kamisato Art: Marobashi) -- Physical --

const AYATO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4496, 0.4862, 0.5228, 0.5751, 0.6117, 0.6535, 0.7110, 0.7685, 0.8260, 0.8888, 0.9515,
        1.0143, 1.0770, 1.1397, 1.2025,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const AYATO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4716, 0.5099, 0.5483, 0.6032, 0.6416, 0.6854, 0.7457, 0.8061, 0.8664, 0.9322, 0.9980,
        1.0638, 1.1296, 1.1954, 1.2612,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const AYATO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5861, 0.6338, 0.6815, 0.7497, 0.7974, 0.8519, 0.9269, 1.0019, 1.0768, 1.1586, 1.2404,
        1.3222, 1.4040, 1.4858, 1.5675,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const AYATO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ(×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2945, 0.3185, 0.3424, 0.3767, 0.4006, 0.4280, 0.4657, 0.5034, 0.5410, 0.5821, 0.6232,
        0.6643, 0.7054, 0.7465, 0.7876,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const AYATO_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7560, 0.8176, 0.8791, 0.9670, 1.0286, 1.0989, 1.1956, 1.2923, 1.3890, 1.4945, 1.6000,
        1.7055, 1.8110, 1.9165, 2.0220,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const AYATO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2953, 1.4007, 1.5062, 1.6568, 1.7622, 1.8827, 2.0484, 2.2141, 2.3797, 2.5605, 2.7412,
        2.9219, 3.1027, 3.2834, 3.4642,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const AYATO_PLUNGE: TalentScaling = TalentScaling {
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

const AYATO_PLUNGE_LOW: TalentScaling = TalentScaling {
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

const AYATO_PLUNGE_HIGH: TalentScaling = TalentScaling {
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

// -- Elemental Skill: 神里流·鏡花 (Kamisato Art: Kyouka) -- Hydro --

const AYATO_SKILL_1HIT: TalentScaling = TalentScaling {
    name: "瞬水剣1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5289, 0.5719, 0.6150, 0.6765, 0.7195, 0.7687, 0.8364, 0.9040, 0.9717, 1.0455, 1.1193,
        1.1931, 1.2669, 1.3407, 1.4145,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const AYATO_SKILL_2HIT: TalentScaling = TalentScaling {
    name: "瞬水剣2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5891, 0.6371, 0.6850, 0.7535, 0.8015, 0.8562, 0.9316, 1.0070, 1.0823, 1.1645, 1.2467,
        1.3289, 1.4111, 1.4933, 1.5755,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const AYATO_SKILL_3HIT: TalentScaling = TalentScaling {
    name: "瞬水剣3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.6493, 0.7022, 0.7550, 0.8305, 0.8834, 0.9437, 1.0268, 1.1099, 1.1929, 1.2835, 1.3741,
        1.4647, 1.5553, 1.6459, 1.7365,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const AYATO_SKILL_ILLUSION: TalentScaling = TalentScaling {
    name: "水の幻影ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.0148, 1.0974, 1.1800, 1.2980, 1.3806, 1.4750, 1.6048, 1.7346, 1.8644, 2.0060, 2.1476,
        2.2892, 2.4308, 2.5724, 2.7140,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 神里流·水囿 (Kamisato Art: Suiyuu) -- Hydro --

const AYATO_BURST: TalentScaling = TalentScaling {
    name: "水沫剣ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.6646, 0.7144, 0.7642, 0.8307, 0.8805, 0.9304, 0.9968, 1.0633, 1.1298, 1.1962, 1.2626,
        1.3291, 1.4122, 1.4953, 1.5783,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const AYATO: CharacterData = CharacterData {
    id: "ayato",
    name: "Ayato",
    element: Element::Hydro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [
        1068.00, 2770.00, 3685.00, 5514.00, 6165.00, 7092.00, 7960.00, 8897.00, 9548.00, 10494.00,
        11144.00, 12101.00, 12751.00, 13715.00, 13715.00, 14263.60, // Lv95/Lv95+/Lv100
        14263.60, // Lv95/Lv95+/Lv100
        14812.20, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        23.27, 60.38, 80.33, 120.20, 134.38, 154.60, 173.51, 193.94, 208.12, 228.74, 242.92,
        263.78, 277.96, 298.97, 298.97, 310.93, // Lv95/Lv95+/Lv100
        310.93, // Lv95/Lv95+/Lv100
        322.89, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        59.83, 155.20, 206.50, 308.99, 345.44, 397.43, 446.03, 498.56, 535.01, 588.02, 624.47,
        678.08, 714.53, 768.55, 768.55, 799.29, // Lv95/Lv95+/Lv100
        799.29, // Lv95/Lv95+/Lv100
        830.03, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "神里流・転",
            hits: &[
                AYATO_NORMAL_1,
                AYATO_NORMAL_2,
                AYATO_NORMAL_3,
                AYATO_NORMAL_4,
                AYATO_NORMAL_5,
            ],
            charged: &[AYATO_CHARGED],
            plunging: &[AYATO_PLUNGE, AYATO_PLUNGE_LOW, AYATO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "神里流·鏡花",
            scalings: &[
                AYATO_SKILL_1HIT,
                AYATO_SKILL_2HIT,
                AYATO_SKILL_3HIT,
                AYATO_SKILL_ILLUSION,
            ],
        },
        elemental_burst: TalentData {
            name: "神里流·水囿",
            scalings: &[AYATO_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
    scaling_modifiers: &[],
};
