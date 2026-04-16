use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Strike While the Arrow's Hot -- Physical --

const JAHODA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4167, 0.4507, 0.4846, 0.533, 0.567, 0.6057, 0.659, 0.7123, 0.7656, 0.8238, 0.8819,
        0.9401, 0.9982, 1.0564, 1.1145,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const JAHODA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ(x2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.1923, 0.208, 0.2236, 0.246, 0.2616, 0.2795, 0.3041, 0.3287, 0.3533, 0.3802, 0.407,
        0.4338, 0.4607, 0.4875, 0.5143,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const JAHODA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.512, 0.5536, 0.5953, 0.6549, 0.6965, 0.7442, 0.8096, 0.8751, 0.9406, 1.012, 1.0835,
        1.1549, 1.2264, 1.2978, 1.3692,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Aimed Shot / Charged --

const JAHODA_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const JAHODA_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack --

const JAHODA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const JAHODA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const JAHODA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: Savvy Strategy: Splitting the Spoils -- Anemo --

const JAHODA_SKILL_SMOKE_BOMB: TalentScaling = TalentScaling {
    name: "発煙弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.59, 1.7093, 1.8285, 1.9875, 2.1068, 2.226, 2.385, 2.544, 2.703, 2.862, 3.021, 3.18,
        3.3788, 3.5775, 3.7763,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const JAHODA_SKILL_UNFILLED_FLASK: TalentScaling = TalentScaling {
    name: "お宝ボトルの通常時ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        1.908, 2.0511, 2.1942, 2.385, 2.5281, 2.6712, 2.862, 3.0528, 3.2436, 3.4344, 3.6252, 3.816,
        4.0545, 4.293, 4.5315,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const JAHODA_SKILL_FILLED_FLASK: TalentScaling = TalentScaling {
    name: "お宝ボトルの満タン時ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.12, 2.279, 2.438, 2.65, 2.809, 2.968, 3.18, 3.392, 3.604, 3.816, 4.028, 4.24, 4.505,
        4.77, 5.035,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// Mirror uses the stored Flask element; expose one selectable row per supported element.
const JAHODA_SKILL_MEOWBALL_PYRO: TalentScaling = TalentScaling {
    name: "ニャンコボールダメージ（炎）",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.28, 1.376, 1.472, 1.6, 1.696, 1.792, 1.92, 2.048, 2.176, 2.304, 2.432, 2.56, 2.72, 2.88,
        3.04,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const JAHODA_SKILL_MEOWBALL_HYDRO: TalentScaling = TalentScaling {
    name: "ニャンコボールダメージ（水）",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.28, 1.376, 1.472, 1.6, 1.696, 1.792, 1.92, 2.048, 2.176, 2.304, 2.432, 2.56, 2.72, 2.88,
        3.04,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const JAHODA_SKILL_MEOWBALL_ELECTRO: TalentScaling = TalentScaling {
    name: "ニャンコボールダメージ（雷）",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.28, 1.376, 1.472, 1.6, 1.696, 1.792, 1.92, 2.048, 2.176, 2.304, 2.432, 2.56, 2.72, 2.88,
        3.04,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const JAHODA_SKILL_MEOWBALL_CRYO: TalentScaling = TalentScaling {
    name: "ニャンコボールダメージ（氷）",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.28, 1.376, 1.472, 1.6, 1.696, 1.792, 1.92, 2.048, 2.176, 2.304, 2.432, 2.56, 2.72, 2.88,
        3.04,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: Hidden Aces: Seven Tools of the Hunter -- Anemo --

const JAHODA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        2.072, 2.2274, 2.3828, 2.59, 2.7454, 2.9008, 3.108, 3.3152, 3.5224, 3.7296, 3.9368, 4.144,
        4.403, 4.662, 4.921,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const JAHODA_BURST_ROBOT: TalentScaling = TalentScaling {
    name: "家庭用肉球アシスタントマシンのダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Anemo),
    values: [
        0.1727, 0.1856, 0.1986, 0.2158, 0.2288, 0.2417, 0.259, 0.2763, 0.2935, 0.3108, 0.3281,
        0.3453, 0.3669, 0.3885, 0.4101,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const JAHODA: CharacterData = CharacterData {
    id: "jahoda",
    name: "Jahoda",
    element: Element::Anemo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Snezhnaya,
    base_hp: [
        809.00, 2078.00, 2682.00, 4017.00, 4446.00, 5114.00, 5687.00, 6355.00, 6784.00, 7451.00,
        7881.00, 8549.00, 8978.00, 9646.00, 9646.00, 10031.84, // Lv95/Lv95+/Lv100
        10031.84, // Lv95/Lv95+/Lv100
        10417.68, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        18.70, 48.04, 62.01, 92.88, 102.80, 118.25, 131.48, 146.93, 156.85, 172.28, 182.20, 197.65,
        207.57, 223.02, 223.02, 231.94, // Lv95/Lv95+/Lv100
        231.94, // Lv95/Lv95+/Lv100
        240.86, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        48.64, 124.96, 161.30, 241.60, 267.42, 307.60, 342.03, 382.20, 408.02, 448.15, 473.97,
        514.15, 539.97, 580.14, 580.14, 603.35, // Lv95/Lv95+/Lv100
        603.35, // Lv95/Lv95+/Lv100
        626.55, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::HealingBonus(0.1846),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "気転の矢",
            hits: &[JAHODA_NORMAL_1, JAHODA_NORMAL_2, JAHODA_NORMAL_3],
            charged: &[JAHODA_AIMED, JAHODA_AIMED_FULL],
            plunging: &[JAHODA_PLUNGE, JAHODA_PLUNGE_LOW, JAHODA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "奇策・財宝分配法",
            scalings: &[
                JAHODA_SKILL_SMOKE_BOMB,
                JAHODA_SKILL_UNFILLED_FLASK,
                JAHODA_SKILL_FILLED_FLASK,
                JAHODA_SKILL_MEOWBALL_PYRO,
                JAHODA_SKILL_MEOWBALL_HYDRO,
                JAHODA_SKILL_MEOWBALL_ELECTRO,
                JAHODA_SKILL_MEOWBALL_CRYO,
            ],
        },
        elemental_burst: TalentData {
            name: "秘器・狩人の七つ道具",
            scalings: &[JAHODA_BURST, JAHODA_BURST_ROBOT],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
    passive_scalings: &[],
};

// =============================================================================
// Varka — 5★ Anemo Claymore (Mondstadt)
// Source: datamined v6.4
// Normal Attack: Favonius Bladework: Dancing Radiance
// Elemental Skill: Windbound Execution
// Elemental Burst: Northwind Avatar
