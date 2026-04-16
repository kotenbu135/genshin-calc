use crate::types::*;
use genshin_calc_core::{Element, Reaction, ScalingStat};

// -- Normal Attack: Moondew Cascade -- All Hydro (Catalyst) --

const COLUMBINA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1-Hit DMG",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.4679, 0.5030, 0.5381, 0.5849, 0.6200, 0.6551, 0.7019, 0.7487, 0.7955, 0.8423, 0.8890,
        0.9358, 0.9943, 1.0528, 1.1113,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLUMBINA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2-Hit DMG",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3663, 0.3937, 0.4212, 0.4578, 0.4853, 0.5128, 0.5494, 0.5860, 0.6226, 0.6593, 0.6959,
        0.7325, 0.7783, 0.8241, 0.8699,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLUMBINA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3-Hit DMG",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5848, 0.6287, 0.6726, 0.7311, 0.7749, 0.8188, 0.8773, 0.9357, 0.9942, 1.0527, 1.1112,
        1.1697, 1.2428, 1.3159, 1.3890,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLUMBINA_CHARGED: TalentScaling = TalentScaling {
    name: "Charged Attack DMG",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.1608, 1.2479, 1.3349, 1.4510, 1.5381, 1.6251, 1.7412, 1.8573, 1.9734, 2.0894, 2.2055,
        2.3216, 2.4667, 2.6118, 2.7569,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLUMBINA_MOONDEW_CLEANSE: TalentScaling = TalentScaling {
    name: "Moondew Cleanse DMG",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0151, 0.0162, 0.0174, 0.0189, 0.0200, 0.0212, 0.0227, 0.0242, 0.0257, 0.0272, 0.0287,
        0.0302, 0.0321, 0.0340, 0.0359,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::DirectLunar(Reaction::LunarBloom),
};

// -- Plunging Attack -- Hydro (Catalyst) --

const COLUMBINA_PLUNGE: TalentScaling = TalentScaling {
    name: "Plunge DMG",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLUMBINA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "Low Plunge DMG",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6516, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLUMBINA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "High Plunge DMG",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: Eternal Tides -- Hydro --

const COLUMBINA_SKILL_DMG: TalentScaling = TalentScaling {
    name: "Skill DMG",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1672, 0.1797, 0.1923, 0.2090, 0.2215, 0.2341, 0.2508, 0.2675, 0.2842, 0.3010, 0.3177,
        0.3344, 0.3553, 0.3762, 0.3971,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLUMBINA_SKILL_RIPPLE: TalentScaling = TalentScaling {
    name: "Gravity Ripple Continuous DMG",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0936, 0.1006, 0.1076, 0.1170, 0.1240, 0.1310, 0.1404, 0.1498, 0.1591, 0.1685, 0.1778,
        0.1872, 0.1989, 0.2106, 0.2223,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const COLUMBINA_SKILL_LUNAR_CHARGED: TalentScaling = TalentScaling {
    name: "Gravity Interference: Lunar-Charged DMG",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0470, 0.0506, 0.0541, 0.0588, 0.0623, 0.0659, 0.0706, 0.0753, 0.0800, 0.0847, 0.0894,
        0.0941, 0.1000, 0.1058, 0.1117,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::DirectLunar(Reaction::LunarElectroCharged),
};

const COLUMBINA_SKILL_LUNAR_BLOOM: TalentScaling = TalentScaling {
    name: "Gravity Interference: Lunar-Bloom DMG",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0141, 0.0151, 0.0162, 0.0176, 0.0187, 0.0197, 0.0211, 0.0225, 0.0239, 0.0253, 0.0268,
        0.0282, 0.0299, 0.0317, 0.0334,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::DirectLunar(Reaction::LunarBloom),
};

const COLUMBINA_SKILL_LUNAR_CRYSTALLIZE: TalentScaling = TalentScaling {
    name: "Gravity Interference: Lunar-Crystallize DMG",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0882, 0.0949, 0.1015, 0.1103, 0.1169, 0.1235, 0.1324, 0.1412, 0.1500, 0.1588, 0.1677,
        0.1765, 0.1875, 0.1985, 0.2096,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::DirectLunar(Reaction::LunarCrystallize),
};

// -- Elemental Burst: Moonlit Melancholy -- Hydro --

const COLUMBINA_BURST_DMG: TalentScaling = TalentScaling {
    name: "Skill DMG",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.3224, 0.3466, 0.3708, 0.4030, 0.4272, 0.4514, 0.4836, 0.5158, 0.5481, 0.5803, 0.6126,
        0.6448, 0.6851, 0.7254, 0.7657,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const COLUMBINA: CharacterData = CharacterData {
    id: "columbina",
    name: "Columbina",
    element: Element::Hydro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::NodKrai,
    base_hp: [
        1144.00, 2967.00, 3948.00, 5908.00, 6605.00, 7599.00, 8528.00, 9533.00, 10230.00, 11243.00,
        11940.00, 12965.00, 13662.00, 14695.00, 14695.00, 15282.80, // Lv95/Lv95+/Lv100
        15282.80, // Lv95/Lv95+/Lv100
        15870.60, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        7.45, 19.32, 25.71, 38.46, 43.00, 49.47, 55.52, 62.06, 66.60, 73.20, 77.74, 84.41, 88.95,
        95.67, 95.67, 99.50,  // Lv95/Lv95+/Lv100
        99.50,  // Lv95/Lv95+/Lv100
        103.32, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        40.09, 103.98, 138.35, 207.02, 231.44, 266.28, 298.84, 334.04, 358.46, 393.97, 418.40,
        454.31, 478.73, 514.93, 514.93, 535.53, // Lv95/Lv95+/Lv100
        535.53, // Lv95/Lv95+/Lv100
        556.12, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "Moondew Cascade",
            hits: &[COLUMBINA_NORMAL_1, COLUMBINA_NORMAL_2, COLUMBINA_NORMAL_3],
            charged: &[COLUMBINA_CHARGED, COLUMBINA_MOONDEW_CLEANSE],
            plunging: &[
                COLUMBINA_PLUNGE,
                COLUMBINA_PLUNGE_LOW,
                COLUMBINA_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "Eternal Tides",
            scalings: &[
                COLUMBINA_SKILL_DMG,
                COLUMBINA_SKILL_RIPPLE,
                COLUMBINA_SKILL_LUNAR_CHARGED,
                COLUMBINA_SKILL_LUNAR_BLOOM,
                COLUMBINA_SKILL_LUNAR_CRYSTALLIZE,
            ],
        },
        elemental_burst: TalentData {
            name: "Moonlit Melancholy",
            scalings: &[COLUMBINA_BURST_DMG],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
