use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// --- Normal Attack: Schematic Setup (physical) ---

const KAVEH_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.761857, 0.823868, 0.88588, 0.974468, 1.03648, 1.10735, 1.204797, 1.302244, 1.39969,
        1.505996, 1.612302, 1.718607, 1.824913, 1.931218, 2.037524,
    ],
};

const KAVEH_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.696385, 0.753068, 0.80975, 0.890725, 0.947407, 1.012188, 1.10126, 1.190333, 1.279405,
        1.376575, 1.473745, 1.570915, 1.668085, 1.765255, 1.862425,
    ],
};

const KAVEH_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.842611, 0.911195, 0.97978, 1.077758, 1.146343, 1.224725, 1.332501, 1.440277, 1.548052,
        1.665626, 1.7832, 1.900773, 2.018347, 2.13592, 2.253494,
    ],
};

const KAVEH_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.026883, 1.110467, 1.19405, 1.313455, 1.397039, 1.492563, 1.623908, 1.755254, 1.886599,
        2.029885, 2.173171, 2.316457, 2.459743, 2.603029, 2.746315,
    ],
};

const KAVEH_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.53148, 0.57474, 0.618, 0.6798, 0.72306, 0.7725, 0.84048, 0.90846, 0.97644, 1.0506,
        1.12476, 1.19892, 1.27308, 1.34724, 1.4214,
    ],
};

const KAVEH_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.96148, 1.03974, 1.118, 1.2298, 1.30806, 1.3975, 1.52048, 1.64346, 1.76644, 1.9006,
        2.03476, 2.16892, 2.30308, 2.43724, 2.5714,
    ],
};

const KAVEH_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.745878, 0.806589, 0.8673, 0.95403, 1.014741, 1.084125, 1.179528, 1.274931, 1.370334,
        1.47441, 1.578486, 1.682562, 1.786638, 1.890714, 1.99479,
    ],
};

const KAVEH_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.49144, 1.612836, 1.734233, 1.907656, 2.029052, 2.167791, 2.358556, 2.549322, 2.740087,
        2.948195, 3.156303, 3.364411, 3.572519, 3.780627, 3.988735,
    ],
};

const KAVEH_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.862889, 2.01452, 2.16615, 2.382765, 2.534396, 2.707688, 2.945964, 3.184241, 3.422517,
        3.682455, 3.942393, 4.202331, 4.462269, 4.722207, 4.982145,
    ],
};

// --- Elemental Skill: Artistic Ingenuity ---

const KAVEH_SKILL_DMG: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        2.04, 2.193, 2.346, 2.55, 2.703, 2.856, 3.06, 3.264, 3.468, 3.672, 3.876, 4.08, 4.335,
        4.59, 4.845,
    ],
};

// --- Elemental Burst: Painted Dome ---

const KAVEH_BURST_DMG: TalentScaling = TalentScaling {
    name: "元素爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.6, 1.72, 1.84, 2.0, 2.12, 2.24, 2.4, 2.56, 2.72, 2.88, 3.04, 3.2, 3.4, 3.6, 3.8,
    ],
};

// --- Kaveh aggregation ---

static KAVEH_NA_HITS: &[TalentScaling] =
    &[KAVEH_NA_HIT1, KAVEH_NA_HIT2, KAVEH_NA_HIT3, KAVEH_NA_HIT4];
static KAVEH_CHARGED_ATTACKS: &[TalentScaling] = &[KAVEH_CHARGED_SPINNING, KAVEH_CHARGED_FINAL];
static KAVEH_PLUNGING: &[TalentScaling] = &[KAVEH_PLUNGE, KAVEH_PLUNGE_LOW, KAVEH_PLUNGE_HIGH];
static KAVEH_SKILL_SCALINGS: &[TalentScaling] = &[KAVEH_SKILL_DMG];
static KAVEH_BURST_SCALINGS: &[TalentScaling] = &[KAVEH_BURST_DMG];

pub const KAVEH: CharacterData = CharacterData {
    id: "kaveh",
    name: "Kaveh",
    element: Element::Dendro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Sumeru,
    base_hp: [1003.0, 10602.0, 11134.0, 11962.0],
    base_atk: [20.0, 207.0, 217.0, 234.0],
    base_def: [63.0, 665.0, 699.0, 751.0],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "円規定則",
            hits: KAVEH_NA_HITS,
            charged: KAVEH_CHARGED_ATTACKS,
            plunging: KAVEH_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "精緻なる絵図",
            scalings: KAVEH_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "ムカルナスの描像",
            scalings: KAVEH_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
