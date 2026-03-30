use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Elemental Burst: Ancient Ritual: Cometh the Night
// =============================================================================

// -- Normal Attack: Pocztowy Demonspear -- Physical --

const FLINS_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.44726, 0.48367, 0.52007, 0.57208, 0.60848, 0.65009, 0.70730, 0.76450, 0.82171, 0.88412,
        0.94653, 1.00894, 1.07134, 1.13375, 1.19616,
    ],
};

const FLINS_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.45148, 0.48823, 0.52498, 0.57748, 0.61423, 0.65623, 0.71397, 0.77172, 0.82947, 0.89247,
        0.95546, 1.01846, 1.08146, 1.14446, 1.20745,
    ],
};

const FLINS_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.55920, 0.60471, 0.65023, 0.71525, 0.76077, 0.81279, 0.88431, 0.95584, 1.02736, 1.10539,
        1.18342, 1.26145, 1.33947, 1.41750, 1.49553,
    ],
};

const FLINS_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.32039, 0.34647, 0.37255, 0.40980, 0.43588, 0.46568, 0.50666, 0.54764, 0.58862, 0.63333,
        0.67803, 0.72274, 0.76744, 0.81215, 0.85685,
    ],
};

const FLINS_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.76795, 0.83045, 0.89296, 0.98226, 1.04476, 1.11620, 1.21443, 1.31265, 1.41088, 1.51803,
        1.62519, 1.73234, 1.83950, 1.94665, 2.05381,
    ],
};

// -- Charged Attack -- Physical --

const FLINS_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.03028, 1.11414, 1.19800, 1.31780, 1.40166, 1.49750, 1.62928, 1.76106, 1.89284, 2.03660,
        2.18036, 2.32412, 2.46788, 2.61164, 2.75540,
    ],
};

// -- Plunging Attack -- Physical --

const FLINS_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.63932, 0.69136, 0.74340, 0.81774, 0.86978, 0.92925, 1.01102, 1.09280, 1.17457, 1.26378,
        1.35299, 1.44220, 1.53140, 1.62061, 1.70982,
    ],
};

const FLINS_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.27838, 1.38243, 1.48649, 1.63513, 1.73919, 1.85811, 2.02162, 2.18513, 2.34865, 2.52703,
        2.70540, 2.88378, 3.06216, 3.24054, 3.41892,
    ],
};

const FLINS_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.59676, 1.72673, 1.85670, 2.04237, 2.17234, 2.32088, 2.52511, 2.72935, 2.93359, 3.15639,
        3.37919, 3.60200, 3.82480, 4.04761, 4.27041,
    ],
};

// -- Elemental Skill: Ancient Rite: Arcane Light -- Electro --

const FLINS_SKILL: TalentScaling = TalentScaling {
    name: "北方槍嵐ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.78400, 1.91780, 2.05160, 2.23000, 2.36380, 2.49760, 2.67600, 2.85440, 3.03280, 3.21120,
        3.38960, 3.56800, 3.79100, 4.01400, 4.23700,
    ],
};

// -- Elemental Burst: Ancient Ritual: Cometh the Night -- Electro --

const FLINS_BURST_INITIAL: TalentScaling = TalentScaling {
    name: "初撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        2.59840, 2.79328, 2.98816, 3.24800, 3.44288, 3.63776, 3.89760, 4.15744, 4.41728, 4.67712,
        4.93696, 5.19680, 5.52160, 5.84640, 6.17120,
    ],
};

const FLINS_BURST_MIDDLE_LUNAR: TalentScaling = TalentScaling {
    name: "中盤月感電ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.16240, 0.17458, 0.18676, 0.20300, 0.21518, 0.22736, 0.24360, 0.25984, 0.27608, 0.29232,
        0.30856, 0.32480, 0.34510, 0.36540, 0.38570,
    ],
};

const FLINS_BURST_FINAL_LUNAR: TalentScaling = TalentScaling {
    name: "終盤月感電ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.16928, 1.25698, 1.34467, 1.46160, 1.54930, 1.63699, 1.75392, 1.87085, 1.98778, 2.10470,
        2.22163, 2.33856, 2.48472, 2.63088, 2.77704,
    ],
};

const FLINS_BURST_THUNDER: TalentScaling = TalentScaling {
    name: "雷鳴交響ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.71456, 0.76815, 0.82174, 0.89320, 0.94679, 1.00038, 1.07184, 1.14330, 1.21475, 1.28621,
        1.35766, 1.42912, 1.51844, 1.60776, 1.69708,
    ],
};

pub const FLINS: CharacterData = CharacterData {
    id: "flins",
    name: "Flins",
    element: Element::Electro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::NodKrai,
    base_hp: [972.0, 3356.0, 11020.0, 12491.0],
    base_atk: [27.0, 94.0, 310.0, 352.0],
    base_def: [63.0, 217.0, 713.0, 809.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "ポチトヴィ・デーモンスピア",
            hits: &[
                FLINS_NORMAL_1,
                FLINS_NORMAL_2,
                FLINS_NORMAL_3,
                FLINS_NORMAL_4,
                FLINS_NORMAL_5,
            ],
            charged: &[FLINS_CHARGED],
            plunging: &[FLINS_PLUNGE, FLINS_PLUNGE_LOW, FLINS_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "古儀：アルケイン・ライト",
            scalings: &[FLINS_SKILL],
        },
        elemental_burst: TalentData {
            name: "古儀：夜来たりて",
            scalings: &[
                FLINS_BURST_INITIAL,
                FLINS_BURST_MIDDLE_LUNAR,
                FLINS_BURST_FINAL_LUNAR,
                FLINS_BURST_THUNDER,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
