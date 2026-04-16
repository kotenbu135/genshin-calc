use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 水の囁き (Whisper of Water) -- All Hydro (Catalyst) --

const BARBARA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3784, 0.4068, 0.4352, 0.4730, 0.5014, 0.5298, 0.5676, 0.6054, 0.6433, 0.6811, 0.7205,
        0.7719, 0.8234, 0.8749, 0.9263,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BARBARA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3552, 0.3818, 0.4085, 0.4440, 0.4706, 0.4973, 0.5328, 0.5683, 0.6038, 0.6394, 0.6763,
        0.7246, 0.7729, 0.8212, 0.8695,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BARBARA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.4104, 0.4412, 0.4720, 0.5130, 0.5438, 0.5746, 0.6156, 0.6566, 0.6977, 0.7387, 0.7814,
        0.8372, 0.8930, 0.9488, 1.0047,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BARBARA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5520, 0.5934, 0.6348, 0.6900, 0.7314, 0.7728, 0.8280, 0.8832, 0.9384, 0.9936, 1.0510,
        1.1261, 1.2012, 1.2762, 1.3513,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Hydro (Catalyst) --

const BARBARA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.6624, 1.7871, 1.9118, 2.0780, 2.2027, 2.3274, 2.4936, 2.6598, 2.8261, 2.9923, 3.1649,
        3.3909, 3.6170, 3.8430, 4.0690,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Hydro (Catalyst) --

const BARBARA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BARBARA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const BARBARA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 公演、開始♪ (Let the Show Begin♪) -- Hydro --
// Barbara's skill primarily heals; the ring does Hydro DMG on contact.
// We include the Droplet DMG scaling.

const BARBARA_SKILL_DROPLET: TalentScaling = TalentScaling {
    name: "水滴ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5840, 0.6278, 0.6716, 0.7300, 0.7738, 0.8176, 0.8760, 0.9344, 0.9928, 1.0512, 1.1096,
        1.1680, 1.2410, 1.3140, 1.3870,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: シャイニングミラクル♪ (Shining Miracle♪) -- Hydro --
// Barbara's burst is healing only (no damage scaling).
// We include it with healing % for completeness.

const BARBARA_BURST_HEAL: TalentScaling = TalentScaling {
    name: "治癒量",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1760, 0.1892, 0.2024, 0.2200, 0.2332, 0.2464, 0.2640, 0.2816, 0.2992, 0.3168, 0.3344,
        0.3520, 0.3740, 0.3960, 0.4180,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const BARBARA: CharacterData = CharacterData {
    id: "barbara",
    name: "Barbara",
    element: Element::Hydro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [
        821.00, 2108.00, 2721.00, 4076.00, 4512.00, 5189.00, 5770.00, 6448.00, 6884.00, 7561.00,
        7996.00, 8674.00, 9110.00, 9787.00, 9787.00, 10178.48, // Lv95/Lv95+/Lv100
        10178.48, // Lv95/Lv95+/Lv100
        10569.96, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        13.36, 34.31, 44.29, 66.34, 73.43, 84.46, 93.91, 104.95, 112.04, 123.05, 130.14, 141.18,
        148.27, 159.30, 159.30, 165.67, // Lv95/Lv95+/Lv100
        165.67, // Lv95/Lv95+/Lv100
        172.04, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        56.08, 144.07, 185.97, 278.55, 308.32, 354.64, 394.33, 440.66, 470.42, 516.69, 546.46,
        592.78, 622.55, 668.87, 668.87, 695.62, // Lv95/Lv95+/Lv100
        695.62, // Lv95/Lv95+/Lv100
        722.38, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "水の囁き",
            hits: &[
                BARBARA_NORMAL_1,
                BARBARA_NORMAL_2,
                BARBARA_NORMAL_3,
                BARBARA_NORMAL_4,
            ],
            charged: &[BARBARA_CHARGED],
            plunging: &[BARBARA_PLUNGE, BARBARA_PLUNGE_LOW, BARBARA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "公演、開始♪",
            scalings: &[BARBARA_SKILL_DROPLET],
        },
        elemental_burst: TalentData {
            name: "シャイニングミラクル♪",
            scalings: &[BARBARA_BURST_HEAL],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
    passive_scalings: &[],
};
