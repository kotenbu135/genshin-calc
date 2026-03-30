use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Sethos
// =============================================================================

// -- Normal Attack: 王の鉤矢射法 (Royal Reed Archery) -- Physical --

const SETHOS_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5261, 0.5690, 0.6120, 0.6732, 0.7162, 0.7650, 0.8323, 0.8997, 0.9670, 1.0404, 1.1138,
        1.1871, 1.2605, 1.3338, 1.4072,
    ],
};

const SETHOS_NORMAL_2A: TalentScaling = TalentScaling {
    name: "2段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2380, 0.2574, 0.2768, 0.3045, 0.3239, 0.3460, 0.3765, 0.4070, 0.4375, 0.4706, 0.5038,
        0.5369, 0.5700, 0.6031, 0.6362,
    ],
};

const SETHOS_NORMAL_2B: TalentScaling = TalentScaling {
    name: "2段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2662, 0.2879, 0.3095, 0.3405, 0.3621, 0.3869, 0.4210, 0.4551, 0.4891, 0.5263, 0.5634,
        0.6005, 0.6376, 0.6748, 0.7119,
    ],
};

const SETHOS_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7400, 0.8002, 0.8604, 0.9464, 1.0066, 1.0755, 1.1702, 1.2649, 1.3596, 1.4628, 1.5660,
        1.6692, 1.7724, 1.8756, 1.9788,
    ],
};

// -- Aimed Shot -- Electro (charged) --

const SETHOS_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const SETHOS_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

const SETHOS_SHADOWPIERCING: TalentScaling = TalentScaling {
    name: "影刺し狙い撃ちダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.4000, 1.5050, 1.6100, 1.7500, 1.8550, 1.9600, 2.1000, 2.2400, 2.3800, 2.5200, 2.6600,
        2.8000, 2.9750, 3.1500, 3.3250,
    ],
};

// -- Plunging Attack -- Physical --

const SETHOS_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const SETHOS_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const SETHOS_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 古式・甕の秘儀 (Ancient Rite: The Thundering Sands) -- Electro --

const SETHOS_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.1560, 1.2427, 1.3294, 1.4450, 1.5317, 1.6184, 1.7340, 1.8496, 1.9652, 2.0808, 2.1964,
        2.3120, 2.4565, 2.6010, 2.7455,
    ],
};

// -- Elemental Burst: 秘儀・甕の夕暮れ (Secret Rite: Twilight Shadowpiercer) -- Electro --

const SETHOS_BURST: TalentScaling = TalentScaling {
    name: "暮の雷弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.9620, 2.1094, 2.2568, 2.4525, 2.5999, 2.7474, 2.9430, 3.1387, 3.3344, 3.5301, 3.7258,
        3.9214, 4.1665, 4.4116, 4.6567,
    ],
};

pub const SETHOS: CharacterData = CharacterData {
    id: "sethos",
    name: "Sethos",
    element: Element::Electro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Sumeru,
    base_hp: [821.0, 8674.0, 9110.0, 9787.0],
    base_atk: [19.0, 201.0, 212.0, 227.0],
    base_def: [47.0, 496.0, 521.0, 560.0],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "王の鉤矢射法",
            hits: &[
                SETHOS_NORMAL_1,
                SETHOS_NORMAL_2A,
                SETHOS_NORMAL_2B,
                SETHOS_NORMAL_3,
            ],
            charged: &[SETHOS_AIMED, SETHOS_AIMED_FULL, SETHOS_SHADOWPIERCING],
            plunging: &[SETHOS_PLUNGE, SETHOS_PLUNGE_LOW, SETHOS_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "古式・甕の秘儀",
            scalings: &[SETHOS_SKILL],
        },
        elemental_burst: TalentData {
            name: "秘儀・甕の夕暮れ",
            scalings: &[SETHOS_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
