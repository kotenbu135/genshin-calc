use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Amber
// =============================================================================

// -- Normal Attack: シャープシューター (Sharpshooter) -- Physical --

const AMBER_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3612, 0.3906, 0.4200, 0.4620, 0.4914, 0.5250, 0.5712, 0.6174, 0.6636, 0.7140, 0.7644,
        0.8148, 0.8652, 0.9156, 0.9660,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const AMBER_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3612, 0.3906, 0.4200, 0.4620, 0.4914, 0.5250, 0.5712, 0.6174, 0.6636, 0.7140, 0.7644,
        0.8148, 0.8652, 0.9156, 0.9660,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const AMBER_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4644, 0.5022, 0.5400, 0.5940, 0.6318, 0.6750, 0.7344, 0.7938, 0.8532, 0.9180, 0.9828,
        1.0476, 1.1124, 1.1772, 1.2420,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const AMBER_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4730, 0.5115, 0.5500, 0.6050, 0.6435, 0.6875, 0.7480, 0.8085, 0.8690, 0.9350, 1.0010,
        1.0670, 1.1330, 1.1990, 1.2650,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const AMBER_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5934, 0.6417, 0.6900, 0.7590, 0.8073, 0.8625, 0.9384, 1.0143, 1.0902, 1.1730, 1.2558,
        1.3386, 1.4214, 1.5042, 1.5870,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Aimed Shot -- Pyro (charged) --

const AMBER_AIMED: TalentScaling = TalentScaling {
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

const AMBER_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const AMBER_PLUNGE: TalentScaling = TalentScaling {
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

const AMBER_PLUNGE_LOW: TalentScaling = TalentScaling {
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

const AMBER_PLUNGE_HIGH: TalentScaling = TalentScaling {
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

// -- Elemental Skill: 爆弾人形 (Explosive Puppet) -- Pyro --

const AMBER_SKILL_EXPLOSION: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2320, 1.3244, 1.4168, 1.5400, 1.6324, 1.7248, 1.8480, 1.9712, 2.0944, 2.2176, 2.3408,
        2.4640, 2.6180, 2.7720, 2.9260,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 矢の雨 (Fiery Rain) -- Pyro --

const AMBER_BURST_WAVE: TalentScaling = TalentScaling {
    name: "1回のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.2808, 0.3019, 0.3229, 0.3510, 0.3721, 0.3931, 0.4212, 0.4493, 0.4774, 0.5054, 0.5335,
        0.5616, 0.5967, 0.6318, 0.6669,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const AMBER_BURST_TOTAL: TalentScaling = TalentScaling {
    name: "合計ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        5.0544, 5.4335, 5.8126, 6.3180, 6.6971, 7.0762, 7.5816, 8.0870, 8.5925, 9.0979, 9.6034,
        10.1088, 10.7406, 11.3724, 12.0042,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const AMBER: CharacterData = CharacterData {
    id: "amber",
    name: "Amber",
    element: Element::Pyro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [
        793.26, 2037.88, 2630.48, 3940.15, 4361.22, 5016.45, // index 5: Lv50 突破前
        5577.86, 6233.09, 6654.15, 7308.59, 7729.65, 8384.88, // index 6-11
        8805.94, 9461.18, 9461.18, 9788.00, // index 12-15: Lv80+ - Lv90+
        9788.00, 10115.61, // index 16-17: Lv95/Lv95+
    ],
    base_atk: [
        18.70, 48.04, 62.01, 92.88, 102.80, 118.25, // index 5: Lv50 突破前
        131.48, 146.93, 156.85, 172.28, 182.20, 197.65, // index 6-11
        207.57, 223.02, 223.02, 231.94, // index 12-15: Lv80+ - Lv90+
        231.94, 240.86, // index 16-17: Lv95/Lv95+
    ],
    base_def: [
        50.36, 129.37, 166.99, 250.13, 276.86, 318.46, // index 5: Lv50 突破前
        354.10, 395.69, 422.42, 463.97, 490.70, 532.29, // index 6-11
        559.02, 600.62, 600.62, 621.37, // index 12-15: Lv80+ - Lv90+
        621.37, 642.16, // index 16-17: Lv95/Lv95+
    ],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "シャープシューター",
            hits: &[
                AMBER_NORMAL_1,
                AMBER_NORMAL_2,
                AMBER_NORMAL_3,
                AMBER_NORMAL_4,
                AMBER_NORMAL_5,
            ],
            charged: &[AMBER_AIMED, AMBER_AIMED_FULL],
            plunging: &[AMBER_PLUNGE, AMBER_PLUNGE_LOW, AMBER_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "爆弾人形",
            scalings: &[AMBER_SKILL_EXPLOSION],
        },
        elemental_burst: TalentData {
            name: "矢の雨",
            scalings: &[AMBER_BURST_WAVE, AMBER_BURST_TOTAL],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
    passive_scalings: &[],
    scaling_modifiers: &[],
};
