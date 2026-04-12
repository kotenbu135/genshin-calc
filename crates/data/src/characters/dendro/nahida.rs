use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// --- Normal Attack: Akara (all Dendro — catalyst) ---

const NAHIDA_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.403048, 0.433277, 0.463505, 0.50381, 0.534039, 0.564267, 0.604572, 0.644877, 0.685182,
        0.725486, 0.765791, 0.806096, 0.856477, 0.906858, 0.957239,
    ],
    dynamic_bonus: None,
};

const NAHIDA_NA_HIT2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.369744, 0.397475, 0.425206, 0.46218, 0.489911, 0.517642, 0.554616, 0.59159, 0.628565,
        0.665539, 0.702514, 0.739488, 0.785706, 0.831924, 0.878142,
    ],
    dynamic_bonus: None,
};

const NAHIDA_NA_HIT3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.458744, 0.49315, 0.527556, 0.57343, 0.607836, 0.642242, 0.688116, 0.73399, 0.779865,
        0.825739, 0.871614, 0.917488, 0.974831, 1.032174, 1.089517,
    ],
    dynamic_bonus: None,
};

const NAHIDA_NA_HIT4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.584064, 0.627869, 0.671674, 0.73008, 0.773885, 0.81769, 0.876096, 0.934502, 0.992909,
        1.051315, 1.109722, 1.168128, 1.241136, 1.314144, 1.387152,
    ],
    dynamic_bonus: None,
};

const NAHIDA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.3200, 1.4190, 1.5180, 1.6500, 1.7490, 1.8480, 1.9800, 2.1120, 2.2440, 2.3760, 2.5080,
        2.6400, 2.8050, 2.9700, 3.1350,
    ],
    dynamic_bonus: None,
};

const NAHIDA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.568288, 0.614544, 0.6608, 0.72688, 0.773136, 0.826, 0.898688, 0.971376, 1.044064,
        1.12336, 1.202656, 1.281952, 1.361248, 1.440544, 1.51984,
    ],
    dynamic_bonus: None,
};

const NAHIDA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.136335, 1.228828, 1.32132, 1.453452, 1.545944, 1.65165, 1.796995, 1.94234, 2.087686,
        2.246244, 2.404802, 2.563361, 2.721919, 2.880478, 3.039036,
    ],
    dynamic_bonus: None,
};

const NAHIDA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.419344, 1.534872, 1.6504, 1.81544, 1.930968, 2.063, 2.244544, 2.426088, 2.607632,
        2.80568, 3.003728, 3.201776, 3.399824, 3.597872, 3.79592,
    ],
    dynamic_bonus: None,
};

// --- Elemental Skill: All Schemes to Know ---

const NAHIDA_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "一回押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        0.9840, 1.0578, 1.1316, 1.2300, 1.3038, 1.3776, 1.4760, 1.5744, 1.6728, 1.7712, 1.8696,
        1.9680, 2.0910, 2.2140, 2.3370,
    ],
    dynamic_bonus: None,
};

const NAHIDA_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.3040, 1.4018, 1.4996, 1.6300, 1.7278, 1.8256, 1.9560, 2.0864, 2.2168, 2.3472, 2.4776,
        2.6080, 2.7710, 2.9340, 3.0970,
    ],
    dynamic_bonus: None,
};

// --- Elemental Skill: Tri-Karma Purification (ATK + EM dual scaling) ---

const NAHIDA_SKILL_TRI_KARMA_ATK: TalentScaling = TalentScaling {
    name: "滅浄三業ダメージ(ATK)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Dendro),
    values: [
        1.0320, 1.1094, 1.1868, 1.2900, 1.3674, 1.4448, 1.5480, 1.6512, 1.7544, 1.8576, 1.9608,
        2.0640, 2.1930, 2.3220, 2.4510,
    ],
    dynamic_bonus: None,
};

const NAHIDA_SKILL_TRI_KARMA_EM: TalentScaling = TalentScaling {
    name: "滅浄三業ダメージ(EM)",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Dendro),
    values: [
        2.0640, 2.2188, 2.3736, 2.5800, 2.7348, 2.8896, 3.0960, 3.3024, 3.5088, 3.7152, 3.9216,
        4.1280, 4.3860, 4.6440, 4.9020,
    ],
    dynamic_bonus: None,
};

// --- Elemental Burst: Illusory Heart ---
// Nahida's burst provides buffs — no direct damage scalings

// --- Nahida aggregation ---

static NAHIDA_NA_HITS: &[TalentScaling] = &[
    NAHIDA_NA_HIT1,
    NAHIDA_NA_HIT2,
    NAHIDA_NA_HIT3,
    NAHIDA_NA_HIT4,
];
static NAHIDA_CHARGED_ATTACKS: &[TalentScaling] = &[NAHIDA_CHARGED];
static NAHIDA_PLUNGING: &[TalentScaling] = &[NAHIDA_PLUNGE, NAHIDA_PLUNGE_LOW, NAHIDA_PLUNGE_HIGH];
static NAHIDA_SKILL_SCALINGS: &[TalentScaling] = &[
    NAHIDA_SKILL_PRESS,
    NAHIDA_SKILL_HOLD,
    NAHIDA_SKILL_TRI_KARMA_ATK,
    NAHIDA_SKILL_TRI_KARMA_EM,
];
static NAHIDA_BURST_SCALINGS: &[TalentScaling] = &[];

pub const NAHIDA: CharacterData = CharacterData {
    id: "nahida",
    name: "Nahida",
    element: Element::Dendro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Sumeru,
    base_hp: [
        807.00, 2092.00, 2784.00, 4165.00, 4656.00, 5357.00, 6012.00, 6721.00, 7212.00, 7926.00,
        8418.00, 9140.00, 9632.00, 10360.00, 10360.00, 10774.40, // Lv95/Lv95+/Lv100
        10774.40, // Lv95/Lv95+/Lv100
        11188.80, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        23.27, 60.38, 80.33, 120.20, 134.38, 154.60, 173.51, 193.94, 208.12, 228.74, 242.92,
        263.78, 277.96, 298.97, 298.97, 310.93, // Lv95/Lv95+/Lv100
        310.93, // Lv95/Lv95+/Lv100
        322.89, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        49.06, 127.26, 169.33, 253.37, 283.26, 325.89, 365.74, 408.82, 438.71, 482.18, 512.07,
        556.02, 585.91, 630.21, 630.21, 655.42, // Lv95/Lv95+/Lv100
        655.42, // Lv95/Lv95+/Lv100
        680.63, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::ElementalMastery(115.2),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "行相",
            hits: NAHIDA_NA_HITS,
            charged: NAHIDA_CHARGED_ATTACKS,
            plunging: NAHIDA_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "所聞遍計",
            scalings: NAHIDA_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "心景幻成",
            scalings: NAHIDA_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
