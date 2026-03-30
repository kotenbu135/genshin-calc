use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Shuumatsuban Ninja Blade -- Physical --

const SAYU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7224, 0.7812, 0.84, 0.924, 0.9828, 1.05, 1.1424, 1.2348, 1.3272, 1.428, 1.5435, 1.679328,
        1.815156, 1.950984, 2.09916,
    ],
};

const SAYU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7138, 0.7719, 0.83, 0.913, 0.9711, 1.0375, 1.1288, 1.2201, 1.3114, 1.411, 1.525125,
        1.659336, 1.793547, 1.927758, 2.07417,
    ],
};

const SAYU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4343, 0.46965, 0.505, 0.5555, 0.59085, 0.63125, 0.6868, 0.74235, 0.7979, 0.8585,
        0.927937, 1.009596, 1.091255, 1.172913, 1.261995,
    ],
};

const SAYU_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.98126, 1.06113, 1.141, 1.2551, 1.33497, 1.42625, 1.55176, 1.67727, 1.80278, 1.9397,
        2.096588, 2.281087, 2.465587, 2.650087, 2.851359,
    ],
};

// -- Charged Attack -- Physical --

const SAYU_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.625455, 0.676364, 0.727273, 0.8, 0.850909, 0.909091, 0.989091, 1.069091, 1.149091,
        1.236364, 1.336364, 1.453964, 1.571564, 1.689164, 1.817455,
    ],
};

const SAYU_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1309, 1.22295, 1.315, 1.4465, 1.53855, 1.64375, 1.7884, 1.93305, 2.0777, 2.2355,
        2.416313, 2.628948, 2.841584, 3.054219, 3.286185,
    ],
};

// -- Plunging Attack -- Physical --

const SAYU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.745878, 0.806589, 0.8673, 0.95403, 1.014741, 1.084125, 1.179528, 1.274931, 1.370334,
        1.47441, 1.578486, 1.682562, 1.786638, 1.890714, 1.99479,
    ],
};

const SAYU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.49144, 1.612836, 1.734233, 1.907656, 2.029052, 2.167791, 2.358556, 2.549322, 2.740087,
        2.948195, 3.156303, 3.364411, 3.572519, 3.780627, 3.988735,
    ],
};

const SAYU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.862889, 2.01452, 2.16615, 2.382765, 2.534396, 2.707688, 2.945964, 3.184241, 3.422517,
        3.682455, 3.942393, 4.202331, 4.462269, 4.722207, 4.982145,
    ],
};

// -- Elemental Skill: Yoohoo Art: Fuuin Dash -- Anemo --

const SAYU_SKILL_WINDWHEEL: TalentScaling = TalentScaling {
    name: "風風輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.36, 0.387, 0.414, 0.45, 0.477, 0.504, 0.54, 0.576, 0.612, 0.648, 0.684, 0.72, 0.765,
        0.81, 0.855,
    ],
};

const SAYU_SKILL_WINDWHEEL_ELEM: TalentScaling = TalentScaling {
    name: "風風輪元素ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.168, 0.1806, 0.1932, 0.21, 0.2226, 0.2352, 0.252, 0.2688, 0.2856, 0.3024, 0.3192, 0.336,
        0.357, 0.378, 0.399,
    ],
};

const SAYU_SKILL_KICK_PRESS: TalentScaling = TalentScaling {
    name: "旋風キック短押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.584, 1.7028, 1.8216, 1.98, 2.0988, 2.2176, 2.376, 2.5344, 2.6928, 2.8512, 3.0096, 3.168,
        3.366, 3.564, 3.762,
    ],
};

const SAYU_SKILL_KICK_HOLD: TalentScaling = TalentScaling {
    name: "旋風キック長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.176, 2.3392, 2.5024, 2.72, 2.8832, 3.0464, 3.264, 3.4816, 3.6992, 3.9168, 4.1344, 4.352,
        4.624, 4.896, 5.168,
    ],
};

const SAYU_SKILL_KICK_ELEM: TalentScaling = TalentScaling {
    name: "旋風キック元素ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.7616, 0.81872, 0.87584, 0.952, 1.00912, 1.06624, 1.1424, 1.21856, 1.29472, 1.37088,
        1.44704, 1.5232, 1.6184, 1.7136, 1.8088,
    ],
};

// -- Elemental Burst: Yoohoo Art: Mujina Flurry -- Anemo --

const SAYU_BURST_ACTIVATION: TalentScaling = TalentScaling {
    name: "発動ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.168, 1.2556, 1.3432, 1.46, 1.5476, 1.6352, 1.752, 1.8688, 1.9856, 2.1024, 2.2192, 2.336,
        2.482, 2.628, 2.774,
    ],
};

const SAYU_BURST_DARUMA: TalentScaling = TalentScaling {
    name: "ムジムジだるまダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.52, 0.559, 0.598, 0.65, 0.689, 0.728, 0.78, 0.832, 0.884, 0.936, 0.988, 1.04, 1.105,
        1.17, 1.235,
    ],
};

pub const SAYU: CharacterData = CharacterData {
    id: "sayu",
    name: "Sayu",
    element: Element::Anemo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Inazuma,
    base_hp: [994.0, 10505.0, 11033.0, 11854.0],
    base_atk: [20.0, 216.0, 227.0, 244.0],
    base_def: [62.0, 660.0, 693.0, 745.0],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "終末番忍刀",
            hits: &[SAYU_NORMAL_1, SAYU_NORMAL_2, SAYU_NORMAL_3, SAYU_NORMAL_4],
            charged: &[SAYU_CHARGED_SPINNING, SAYU_CHARGED_FINAL],
            plunging: &[SAYU_PLUNGE, SAYU_PLUNGE_LOW, SAYU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "風風輪・旋旋",
            scalings: &[
                SAYU_SKILL_WINDWHEEL,
                SAYU_SKILL_WINDWHEEL_ELEM,
                SAYU_SKILL_KICK_PRESS,
                SAYU_SKILL_KICK_HOLD,
                SAYU_SKILL_KICK_ELEM,
            ],
        },
        elemental_burst: TalentData {
            name: "呼び出しムジナの群れ",
            scalings: &[SAYU_BURST_ACTIVATION, SAYU_BURST_DARUMA],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Sucrose — 4★ Anemo Catalyst (Mondstadt)
// Source: genshin-db-api
// Normal Attack: Wind Spirit Creation (風霊作成)
// Elemental Skill: Astable Anemohypostasis Creation - 6308
// Elemental Burst: Forbidden Creation - Isomer 75 / Type II
