use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// --- Normal Attack: Toss 'N' Turn Spear (physical) ---

const YAOYAO_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.510014, 0.551527, 0.59304, 0.652344, 0.693857, 0.7413, 0.806534, 0.871769, 0.937003,
        1.008168, 1.079333, 1.150498, 1.221662, 1.292827, 1.363992,
    ],
    dynamic_bonus: None,
};

const YAOYAO_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.474428, 0.513044, 0.55166, 0.606826, 0.645442, 0.689575, 0.750258, 0.81094, 0.871623,
        0.937822, 1.004021, 1.07022, 1.13642, 1.202619, 1.268818,
    ],
    dynamic_bonus: None,
};

const YAOYAO_NA_HIT3A: TalentScaling = TalentScaling {
    name: "3段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.313771, 0.339311, 0.36485, 0.401335, 0.426874, 0.456063, 0.496196, 0.53633, 0.576463,
        0.620245, 0.664027, 0.707809, 0.751591, 0.795373, 0.839155,
    ],
    dynamic_bonus: None,
};

const YAOYAO_NA_HIT3B: TalentScaling = TalentScaling {
    name: "3段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.329457, 0.356274, 0.38309, 0.421399, 0.448215, 0.478862, 0.521002, 0.563142, 0.605282,
        0.651253, 0.697224, 0.743195, 0.789165, 0.835136, 0.881107,
    ],
    dynamic_bonus: None,
};

const YAOYAO_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.779315, 0.842747, 0.90618, 0.996798, 1.060231, 1.132725, 1.232405, 1.332085, 1.431764,
        1.540506, 1.649248, 1.757989, 1.866731, 1.975472, 2.084214,
    ],
    dynamic_bonus: None,
};

const YAOYAO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1266, 1.2183, 1.31, 1.441, 1.5327, 1.6375, 1.7816, 1.9257, 2.0698, 2.227, 2.3842, 2.5414,
        2.6986, 2.8558, 3.013,
    ],
    dynamic_bonus: None,
};

const YAOYAO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.639324, 0.691362, 0.7434, 0.81774, 0.869778, 0.92925, 1.011024, 1.092798, 1.174572,
        1.26378, 1.352988, 1.442196, 1.531404, 1.620612, 1.70982,
    ],
    dynamic_bonus: None,
};

const YAOYAO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.278377, 1.382431, 1.486485, 1.635134, 1.739187, 1.858106, 2.02162, 2.185133, 2.348646,
        2.527025, 2.705403, 2.883781, 3.062159, 3.240537, 3.418915,
    ],
    dynamic_bonus: None,
};

const YAOYAO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.596762, 1.726731, 1.8567, 2.04237, 2.172339, 2.320875, 2.525112, 2.729349, 2.933586,
        3.15639, 3.379194, 3.601998, 3.824802, 4.047606, 4.27041,
    ],
    dynamic_bonus: None,
};

// --- Elemental Skill: Raphanus Sky Cluster ---

const YAOYAO_SKILL_RADISH: TalentScaling = TalentScaling {
    name: "白玉大根ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.2992, 0.32164, 0.34408, 0.374, 0.39644, 0.41888, 0.4488, 0.47872, 0.50864, 0.53856,
        0.56848, 0.5984, 0.6358, 0.6732, 0.7106,
    ],
    dynamic_bonus: None,
};

// --- Elemental Burst: Moonjade Descent ---

const YAOYAO_BURST_RADISH: TalentScaling = TalentScaling {
    name: "白玉大根(爆発)ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.7216, 0.77572, 0.82984, 0.902, 0.95612, 1.01024, 1.0824, 1.15456, 1.22672, 1.29888,
        1.37104, 1.4432, 1.5334, 1.6236, 1.7138,
    ],
    dynamic_bonus: None,
};

// --- Yaoyao aggregation ---

static YAOYAO_NA_HITS: &[TalentScaling] = &[
    YAOYAO_NA_HIT1,
    YAOYAO_NA_HIT2,
    YAOYAO_NA_HIT3A,
    YAOYAO_NA_HIT3B,
    YAOYAO_NA_HIT4,
];
static YAOYAO_CHARGED_ATTACKS: &[TalentScaling] = &[YAOYAO_CHARGED];
static YAOYAO_PLUNGING: &[TalentScaling] = &[YAOYAO_PLUNGE, YAOYAO_PLUNGE_LOW, YAOYAO_PLUNGE_HIGH];
static YAOYAO_SKILL_SCALINGS: &[TalentScaling] = &[YAOYAO_SKILL_RADISH];
static YAOYAO_BURST_SCALINGS: &[TalentScaling] = &[YAOYAO_BURST_RADISH];

pub const YAOYAO: CharacterData = CharacterData {
    id: "yaoyao",
    name: "Yaoyao",
    element: Element::Dendro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [
        1030.00, 2647.00, 3417.00, 5118.00, 5665.00, 6516.00, 7245.00, 8096.00, 8643.00, 9493.00,
        10040.00, 10891.00, 11438.00, 12289.00, 12289.00, 12780.56, // Lv95/Lv95+/Lv100
        12780.56, // Lv95/Lv95+/Lv100
        13272.12, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        17.81, 45.75, 59.05, 88.45, 97.91, 112.62, 125.22, 139.93, 149.38, 164.07, 173.53, 188.24,
        197.69, 212.40, 212.40, 220.90, // Lv95/Lv95+/Lv100
        220.90, // Lv95/Lv95+/Lv100
        229.39, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        62.95, 161.71, 208.74, 312.66, 346.08, 398.07, 442.62, 494.62, 528.03, 579.96, 613.37,
        665.37, 698.78, 750.77, 750.77, 780.80, // Lv95/Lv95+/Lv100
        780.80, // Lv95/Lv95+/Lv100
        810.83, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "顛撲連撃槍",
            hits: YAOYAO_NA_HITS,
            charged: YAOYAO_CHARGED_ATTACKS,
            plunging: YAOYAO_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "祥雲団々落清白",
            scalings: YAOYAO_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "玉顆珊々月中落",
            scalings: YAOYAO_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
