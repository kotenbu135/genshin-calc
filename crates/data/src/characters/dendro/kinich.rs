use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// --- Normal Attack: Nightsun Style (physical) ---

const KINICH_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.98986, 1.07043, 1.151, 1.2661, 1.34667, 1.43875, 1.56536, 1.69197, 1.81858, 1.9567,
        2.09482, 2.23294, 2.37106, 2.50918, 2.6473,
    ],
};

const KINICH_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.82904, 0.89652, 0.964, 1.0604, 1.12788, 1.205, 1.31104, 1.41708, 1.52312, 1.6388,
        1.75448, 1.87016, 1.98584, 2.10152, 2.2172,
    ],
};

const KINICH_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.23496, 1.33548, 1.436, 1.5796, 1.68012, 1.795, 1.95296, 2.11092, 2.26888, 2.4412,
        2.61352, 2.78584, 2.95816, 3.13048, 3.3028,
    ],
};

const KINICH_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.48418, 0.52359, 0.563, 0.6193, 0.65871, 0.70375, 0.76568, 0.82761, 0.88954, 0.9571,
        1.02466, 1.09222, 1.15978, 1.22734, 1.2949,
    ],
};

const KINICH_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.745878, 0.806589, 0.8673, 0.95403, 1.014741, 1.084125, 1.179528, 1.274931, 1.370334,
        1.47441, 1.578486, 1.682562, 1.786638, 1.890714, 1.99479,
    ],
};

const KINICH_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.49144, 1.612836, 1.734233, 1.907656, 2.029052, 2.167791, 2.358556, 2.549322, 2.740087,
        2.948195, 3.156303, 3.364411, 3.572519, 3.780627, 3.988735,
    ],
};

const KINICH_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.862889, 2.01452, 2.16615, 2.382765, 2.534396, 2.707688, 2.945964, 3.184241, 3.422517,
        3.682455, 3.942393, 4.202331, 4.462269, 4.722207, 4.982145,
    ],
};

// --- Elemental Skill: Canopy Hunter: Riding High ---

const KINICH_SKILL_LOOP: TalentScaling = TalentScaling {
    name: "ループショットダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.5728, 0.61576, 0.65872, 0.716, 0.75896, 0.80192, 0.8592, 0.91648, 0.97376, 1.03104,
        1.08832, 1.1456, 1.2172, 1.2888, 1.3604,
    ],
};

const KINICH_SKILL_CANNON: TalentScaling = TalentScaling {
    name: "スケールスパイカー砲ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        6.8744, 7.38998, 7.90556, 8.593, 9.10858, 9.62416, 10.3116, 10.99904, 11.68648, 12.37392,
        13.06136, 13.7488, 14.6081, 15.4674, 16.3267,
    ],
};

// --- Elemental Burst: Hail to the Almighty Dragonlord ---

const KINICH_BURST_DMG: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.34, 1.4405, 1.541, 1.675, 1.7755, 1.876, 2.01, 2.144, 2.278, 2.412, 2.546, 2.68, 2.8475,
        3.015, 3.1825,
    ],
};

const KINICH_BURST_BREATH: TalentScaling = TalentScaling {
    name: "ドラゴンブレスダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.20736, 1.297912, 1.388464, 1.5092, 1.599752, 1.690304, 1.81104, 1.931776, 2.052512,
        2.173248, 2.293984, 2.41472, 2.56564, 2.71656, 2.86748,
    ],
};

// --- Kinich aggregation ---

static KINICH_NA_HITS: &[TalentScaling] = &[KINICH_NA_HIT1, KINICH_NA_HIT2, KINICH_NA_HIT3];
static KINICH_CHARGED_ATTACKS: &[TalentScaling] = &[KINICH_CHARGED];
static KINICH_PLUNGING: &[TalentScaling] = &[KINICH_PLUNGE, KINICH_PLUNGE_LOW, KINICH_PLUNGE_HIGH];
static KINICH_SKILL_SCALINGS: &[TalentScaling] = &[KINICH_SKILL_LOOP, KINICH_SKILL_CANNON];
static KINICH_BURST_SCALINGS: &[TalentScaling] = &[KINICH_BURST_DMG, KINICH_BURST_BREATH];

pub const KINICH: CharacterData = CharacterData {
    id: "kinich",
    name: "Kinich",
    element: Element::Dendro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [1001.0, 11345.0, 11955.0, 12858.0],
    base_atk: [26.0, 293.0, 309.0, 332.0],
    base_def: [62.0, 707.0, 745.0, 802.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "武闘術・白夜",
            hits: KINICH_NA_HITS,
            charged: KINICH_CHARGED_ATTACKS,
            plunging: KINICH_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "懸狩り·宙の遊猟",
            scalings: KINICH_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "偉大なる聖龍を崇拝せよ",
            scalings: KINICH_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
