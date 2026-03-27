#![allow(clippy::approx_constant)]
use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 西風剣術 (Tempered Sword) -- All physical --

const DILUC_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8974, 0.9704, 1.0434, 1.1477, 1.2207, 1.3042, 1.4191, 1.5340, 1.6489, 1.7739, 1.8989,
        2.0239, 2.1489, 2.2739, 2.3989,
    ],
};

const DILUC_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8764, 0.9477, 1.0190, 1.1209, 1.1922, 1.2737, 1.3858, 1.4979, 1.6100, 1.7323, 1.8546,
        1.9769, 2.0992, 2.2215, 2.3438,
    ],
};

const DILUC_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9882, 1.0687, 1.1493, 1.2642, 1.3447, 1.4366, 1.5628, 1.6891, 1.8153, 1.9535, 2.0917,
        2.2298, 2.3680, 2.5061, 2.6443,
    ],
};

const DILUC_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.3399, 1.4491, 1.5583, 1.7141, 1.8233, 1.9479, 2.1191, 2.2904, 2.4616, 2.6489, 2.8362,
        3.0235, 3.2108, 3.3981, 3.5854,
    ],
};

// -- Charged Attack -- Physical --

const DILUC_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6880, 0.7440, 0.8000, 0.8800, 0.9360, 1.0000, 1.0880, 1.1760, 1.2640, 1.3600, 1.4560,
        1.5520, 1.6480, 1.7440, 1.8400,
    ],
};

const DILUC_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2470, 1.3485, 1.4500, 1.5950, 1.6965, 1.8125, 1.9720, 2.1315, 2.2910, 2.4650, 2.6390,
        2.8130, 2.9870, 3.1610, 3.3350,
    ],
};

// -- Plunging Attack -- Physical --

const DILUC_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8951, 0.9681, 1.0411, 1.1452, 1.2182, 1.3013, 1.4154, 1.5296, 1.6437, 1.7679, 1.8921,
        2.0163, 2.1405, 2.2646, 2.3888,
    ],
};

const DILUC_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.7901, 1.9360, 2.0820, 2.2902, 2.4361, 2.6020, 2.8303, 3.0585, 3.2867, 3.5349, 3.7831,
        4.0313, 4.2795, 4.5277, 4.7759,
    ],
};

const DILUC_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        2.2362, 2.4184, 2.6007, 2.8608, 3.0430, 3.2503, 3.5355, 3.8206, 4.1057, 4.4159, 4.7261,
        5.0363, 5.3465, 5.6567, 5.9669,
    ],
};

// -- Elemental Skill: 逆焔の刃 (Searing Onslaught) -- All Pyro --

const DILUC_SKILL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9440, 1.0148, 1.0856, 1.1800, 1.2508, 1.3216, 1.4160, 1.5104, 1.6048, 1.6992, 1.7936,
        1.8880, 2.0060, 2.1240, 2.2420,
    ],
};

const DILUC_SKILL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9760, 1.0492, 1.1224, 1.2200, 1.2932, 1.3664, 1.4640, 1.5616, 1.6592, 1.7568, 1.8544,
        1.9520, 2.0740, 2.1960, 2.3180,
    ],
};

const DILUC_SKILL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2880, 1.3846, 1.4812, 1.6100, 1.7066, 1.8032, 1.9320, 2.0608, 2.1896, 2.3184, 2.4472,
        2.5760, 2.7370, 2.8980, 3.0590,
    ],
};

// -- Elemental Burst: 黎明 (Dawn) -- All Pyro --

const DILUC_BURST_SLASH: TalentScaling = TalentScaling {
    name: "斬撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        2.0400, 2.1930, 2.3460, 2.5500, 2.7030, 2.8560, 3.0600, 3.2640, 3.4680, 3.6720, 3.8760,
        4.0800, 4.3350, 4.5900, 4.8450,
    ],
};

const DILUC_BURST_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.6000, 0.6450, 0.6900, 0.7500, 0.7950, 0.8400, 0.9000, 0.9600, 1.0200, 1.0800, 1.1400,
        1.2000, 1.2750, 1.3500, 1.4250,
    ],
};

const DILUC_BURST_EXPLOSION: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        2.0400, 2.1930, 2.3460, 2.5500, 2.7030, 2.8560, 3.0600, 3.2640, 3.4680, 3.6720, 3.8760,
        4.0800, 4.3350, 4.5900, 4.8450,
    ],
};

// -- Character Data --

pub const DILUC: CharacterData = CharacterData {
    id: "diluc",
    name: "Diluc",
    element: Element::Pyro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [1011.0, 10122.0, 11243.0, 12068.0],
    base_atk: [26.0, 260.0, 289.0, 311.0],
    base_def: [61.0, 612.0, 680.0, 729.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "西風剣術",
            hits: &[
                DILUC_NORMAL_1,
                DILUC_NORMAL_2,
                DILUC_NORMAL_3,
                DILUC_NORMAL_4,
            ],
            charged: &[DILUC_CHARGED_SPINNING, DILUC_CHARGED_FINAL],
            plunging: &[DILUC_PLUNGE, DILUC_PLUNGE_LOW, DILUC_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "逆焔の刃",
            scalings: &[DILUC_SKILL_1, DILUC_SKILL_2, DILUC_SKILL_3],
        },
        elemental_burst: TalentData {
            name: "黎明",
            scalings: &[DILUC_BURST_SLASH, DILUC_BURST_DOT, DILUC_BURST_EXPLOSION],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

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
};

const AMBER_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3612, 0.3906, 0.4200, 0.4620, 0.4914, 0.5250, 0.5712, 0.6174, 0.6636, 0.7140, 0.7644,
        0.8148, 0.8652, 0.9156, 0.9660,
    ],
};

const AMBER_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4644, 0.5022, 0.5400, 0.5940, 0.6318, 0.6750, 0.7344, 0.7938, 0.8532, 0.9180, 0.9828,
        1.0476, 1.1124, 1.1772, 1.2420,
    ],
};

const AMBER_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4730, 0.5115, 0.5500, 0.6050, 0.6435, 0.6875, 0.7480, 0.8085, 0.8690, 0.9350, 1.0010,
        1.0670, 1.1330, 1.1990, 1.2650,
    ],
};

const AMBER_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5934, 0.6417, 0.6900, 0.7590, 0.8073, 0.8625, 0.9384, 1.0143, 1.0902, 1.1730, 1.2558,
        1.3386, 1.4214, 1.5042, 1.5870,
    ],
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
};

const AMBER_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
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
};

const AMBER_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const AMBER_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
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
};

const AMBER_BURST_TOTAL: TalentScaling = TalentScaling {
    name: "合計ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        5.0544, 5.4335, 5.8126, 6.3180, 6.6971, 7.0762, 7.5816, 8.0870, 8.5925, 9.0979, 9.6034,
        10.1088, 10.7406, 11.3724, 12.0042,
    ],
};

pub const AMBER: CharacterData = CharacterData {
    id: "amber",
    name: "Amber",
    element: Element::Pyro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [793.0, 8385.0, 8806.0, 9461.0],
    base_atk: [19.0, 198.0, 208.0, 223.0],
    base_def: [50.0, 532.0, 559.0, 601.0],
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
};

// =============================================================================
// Arlecchino
// =============================================================================

// -- Normal Attack: 断頭への招待 (Invitation to a Beheading) -- Physical --

const ARLECCHINO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4750, 0.5137, 0.5523, 0.6076, 0.6462, 0.6904, 0.7512, 0.8119, 0.8727, 0.9390, 1.0052,
        1.0715, 1.1378, 1.2041, 1.2704,
    ],
};

const ARLECCHINO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5211, 0.5635, 0.6059, 0.6665, 0.7089, 0.7574, 0.8240, 0.8906, 0.9573, 1.0300, 1.1027,
        1.1754, 1.2481, 1.3208, 1.3935,
    ],
};

const ARLECCHINO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6539, 0.7071, 0.7603, 0.8363, 0.8896, 0.9504, 1.0340, 1.1176, 1.2013, 1.2925, 1.3837,
        1.4750, 1.5662, 1.6575, 1.7487,
    ],
};

const ARLECCHINO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3715, 0.4017, 0.4319, 0.4751, 0.5053, 0.5399, 0.5874, 0.6349, 0.6824, 0.7343, 0.7861,
        0.8379, 0.8898, 0.9416, 0.9934,
    ],
};

const ARLECCHINO_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6998, 0.7568, 0.8137, 0.8951, 0.9521, 1.0172, 1.1067, 1.1962, 1.2857, 1.3834, 1.4810,
        1.5787, 1.6763, 1.7740, 1.8716,
    ],
};

const ARLECCHINO_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8538, 0.9233, 0.9928, 1.0920, 1.1615, 1.2410, 1.3502, 1.4594, 1.5686, 1.6877, 1.8068,
        1.9260, 2.0451, 2.1642, 2.2834,
    ],
};

// -- Charged Attack -- Physical --

const ARLECCHINO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9082, 0.9821, 1.0560, 1.1616, 1.2355, 1.3200, 1.4362, 1.5523, 1.6685, 1.7952, 1.9219,
        2.0486, 2.1754, 2.3021, 2.4288,
    ],
};

// -- Plunging Attack -- Physical --

const ARLECCHINO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const ARLECCHINO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const ARLECCHINO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 万象灰燼 (All Is Ash) -- Pyro --

const ARLECCHINO_SKILL_SPIKE: TalentScaling = TalentScaling {
    name: "スパイクダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.1484, 0.1595, 0.1707, 0.1855, 0.1966, 0.2078, 0.2226, 0.2374, 0.2523, 0.2671, 0.2820,
        0.2968, 0.3154, 0.3339, 0.3525,
    ],
};

const ARLECCHINO_SKILL_CLEAVE: TalentScaling = TalentScaling {
    name: "切り裂きダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.3356, 1.4358, 1.5359, 1.6695, 1.7697, 1.8698, 2.0034, 2.1370, 2.2705, 2.4041, 2.5376,
        2.6712, 2.8382, 3.0051, 3.1721,
    ],
};

const ARLECCHINO_SKILL_DIRECTIVE: TalentScaling = TalentScaling {
    name: "血償の勅令ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.3180, 0.3419, 0.3657, 0.3975, 0.4214, 0.4452, 0.4770, 0.5088, 0.5406, 0.5724, 0.6042,
        0.6360, 0.6758, 0.7155, 0.7553,
    ],
};

// -- Elemental Burst: 厄月昇り (Balemoon Rising) -- Pyro --

const ARLECCHINO_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        3.7040, 3.9818, 4.2596, 4.6300, 4.9078, 5.1856, 5.5560, 5.9264, 6.2968, 6.6672, 7.0376,
        7.4080, 7.8710, 8.3340, 8.7970,
    ],
};

pub const ARLECCHINO: CharacterData = CharacterData {
    id: "arlecchino",
    name: "Arlecchino",
    element: Element::Pyro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Snezhnaya,
    base_hp: [1020.0, 11561.0, 12182.0, 13103.0],
    base_atk: [27.0, 302.0, 318.0, 342.0],
    base_def: [60.0, 675.0, 711.0, 765.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "断頭への招待",
            hits: &[
                ARLECCHINO_NORMAL_1,
                ARLECCHINO_NORMAL_2,
                ARLECCHINO_NORMAL_3,
                ARLECCHINO_NORMAL_4,
                ARLECCHINO_NORMAL_5,
                ARLECCHINO_NORMAL_6,
            ],
            charged: &[ARLECCHINO_CHARGED],
            plunging: &[
                ARLECCHINO_PLUNGE,
                ARLECCHINO_PLUNGE_LOW,
                ARLECCHINO_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "万象灰燼",
            scalings: &[
                ARLECCHINO_SKILL_SPIKE,
                ARLECCHINO_SKILL_CLEAVE,
                ARLECCHINO_SKILL_DIRECTIVE,
            ],
        },
        elemental_burst: TalentData {
            name: "厄月昇り",
            scalings: &[ARLECCHINO_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Bennett
// =============================================================================

// -- Normal Attack: 好運の剣 (Strike of Fortune) -- Physical --

const BENNETT_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4455, 0.4817, 0.5180, 0.5698, 0.6061, 0.6475, 0.7045, 0.7615, 0.8184, 0.8806, 0.9428,
        1.0049, 1.0671, 1.1292, 1.1914,
    ],
};

const BENNETT_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4274, 0.4622, 0.4970, 0.5467, 0.5815, 0.6213, 0.6759, 0.7306, 0.7853, 0.8449, 0.9045,
        0.9642, 1.0238, 1.0835, 1.1431,
    ],
};

const BENNETT_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5461, 0.5906, 0.6350, 0.6985, 0.7430, 0.7938, 0.8636, 0.9335, 1.0033, 1.0795, 1.1557,
        1.2319, 1.3081, 1.3843, 1.4605,
    ],
};

const BENNETT_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5968, 0.6454, 0.6940, 0.7634, 0.8120, 0.8675, 0.9438, 1.0202, 1.0965, 1.1798, 1.2631,
        1.3464, 1.4296, 1.5129, 1.5962,
    ],
};

const BENNETT_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7190, 0.7775, 0.8360, 0.9196, 0.9781, 1.0450, 1.1370, 1.2289, 1.3209, 1.4212, 1.5215,
        1.6218, 1.7222, 1.8225, 1.9228,
    ],
};

// -- Charged Attack -- Physical --

const BENNETT_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ 1段",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5590, 0.6045, 0.6500, 0.7150, 0.7605, 0.8125, 0.8840, 0.9555, 1.0270, 1.1050, 1.1830,
        1.2610, 1.3390, 1.4170, 1.4950,
    ],
};

const BENNETT_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ 2段",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6072, 0.6566, 0.7060, 0.7766, 0.8260, 0.8825, 0.9602, 1.0378, 1.1155, 1.2002, 1.2849,
        1.3696, 1.4544, 1.5391, 1.6238,
    ],
};

// -- Plunging Attack -- Physical --

const BENNETT_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const BENNETT_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const BENNETT_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 溢れる情熱 (Passion Overload) -- Pyro --

const BENNETT_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "1回押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.3760, 1.4792, 1.5824, 1.7200, 1.8232, 1.9264, 2.0640, 2.2016, 2.3392, 2.4768, 2.6144,
        2.7520, 2.9240, 3.0960, 3.2680,
    ],
};

const BENNETT_SKILL_HOLD_1: TalentScaling = TalentScaling {
    name: "長押し1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.8400, 0.9030, 0.9660, 1.0500, 1.1130, 1.1760, 1.2600, 1.3440, 1.4280, 1.5120, 1.5960,
        1.6800, 1.7850, 1.8900, 1.9950,
    ],
};

const BENNETT_SKILL_HOLD_2: TalentScaling = TalentScaling {
    name: "長押し2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9200, 0.9890, 1.0580, 1.1500, 1.2190, 1.2880, 1.3800, 1.4720, 1.5640, 1.6560, 1.7480,
        1.8400, 1.9550, 2.0700, 2.1850,
    ],
};

const BENNETT_SKILL_EXPLOSION: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.3200, 1.4190, 1.5180, 1.6500, 1.7490, 1.8480, 1.9800, 2.1120, 2.2440, 2.3760, 2.5080,
        2.6400, 2.8050, 2.9700, 3.1350,
    ],
};

// -- Elemental Burst: 素晴らしい旅 (Fantastic Voyage) -- Pyro --

const BENNETT_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        2.3280, 2.5026, 2.6772, 2.9100, 3.0846, 3.2592, 3.4920, 3.7248, 3.9576, 4.1904, 4.4232,
        4.6560, 4.9470, 5.2380, 5.5290,
    ],
};

pub const BENNETT: CharacterData = CharacterData {
    id: "bennett",
    name: "Bennett",
    element: Element::Pyro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [1039.0, 10987.0, 11539.0, 12397.0],
    base_atk: [16.0, 169.0, 178.0, 191.0],
    base_def: [65.0, 684.0, 718.0, 771.0],
    ascension_stat: AscensionStat::EnergyRecharge(0.2668),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "好運の剣",
            hits: &[
                BENNETT_NORMAL_1,
                BENNETT_NORMAL_2,
                BENNETT_NORMAL_3,
                BENNETT_NORMAL_4,
                BENNETT_NORMAL_5,
            ],
            charged: &[BENNETT_CHARGED_1, BENNETT_CHARGED_2],
            plunging: &[BENNETT_PLUNGE, BENNETT_PLUNGE_LOW, BENNETT_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "溢れる情熱",
            scalings: &[
                BENNETT_SKILL_PRESS,
                BENNETT_SKILL_HOLD_1,
                BENNETT_SKILL_HOLD_2,
                BENNETT_SKILL_EXPLOSION,
            ],
        },
        elemental_burst: TalentData {
            name: "素晴らしい旅",
            scalings: &[BENNETT_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Chevreuse
// =============================================================================

// -- Normal Attack: ラインバヨネット突撃EX (Line Bayonet Thrust EX) -- Physical --

const CHEVREUSE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5313, 0.5745, 0.6178, 0.6796, 0.7228, 0.7722, 0.8402, 0.9082, 0.9761, 1.0502, 1.1244,
        1.1985, 1.2726, 1.3468, 1.4209,
    ],
};

const CHEVREUSE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4931, 0.5332, 0.5734, 0.6307, 0.6709, 0.7167, 0.7798, 0.8429, 0.9059, 0.9747, 1.0436,
        1.1124, 1.1812, 1.2500, 1.3188,
    ],
};

const CHEVREUSE_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2764, 0.2990, 0.3215, 0.3536, 0.3761, 0.4018, 0.4372, 0.4725, 0.5079, 0.5465, 0.5850,
        0.6236, 0.6622, 0.7008, 0.7393,
    ],
};

const CHEVREUSE_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3245, 0.3509, 0.3774, 0.4151, 0.4415, 0.4717, 0.5132, 0.5547, 0.5962, 0.6415, 0.6868,
        0.7321, 0.7774, 0.8226, 0.8679,
    ],
};

const CHEVREUSE_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7726, 0.8355, 0.8984, 0.9882, 1.0511, 1.1230, 1.2218, 1.3206, 1.4195, 1.5273, 1.6351,
        1.7429, 1.8507, 1.9585, 2.0663,
    ],
};

// -- Charged Attack -- Physical --

const CHEVREUSE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2169, 1.3160, 1.4150, 1.5565, 1.6556, 1.7688, 1.9244, 2.0801, 2.2357, 2.4055, 2.5753,
        2.7451, 2.9149, 3.0847, 3.2545,
    ],
};

// -- Plunging Attack -- Physical --

const CHEVREUSE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const CHEVREUSE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const CHEVREUSE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 近距離急速射撃 (Short-Range Rapid Interdiction Fire) -- Pyro --

const CHEVREUSE_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "1回押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1520, 1.2384, 1.3248, 1.4400, 1.5264, 1.6128, 1.7280, 1.8432, 1.9584, 2.0736, 2.1888,
        2.3040, 2.4480, 2.5920, 2.7360,
    ],
};

const CHEVREUSE_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.7280, 1.8576, 1.9872, 2.1600, 2.2896, 2.4192, 2.5920, 2.7648, 2.9376, 3.1104, 3.2832,
        3.4560, 3.6720, 3.8880, 4.1040,
    ],
};

const CHEVREUSE_SKILL_OVERCHARGED: TalentScaling = TalentScaling {
    name: "過充填弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        2.8240, 3.0358, 3.2476, 3.5300, 3.7418, 3.9536, 4.2360, 4.5184, 4.8008, 5.0832, 5.3656,
        5.6480, 6.0010, 6.3540, 6.7070,
    ],
};

// -- Elemental Burst: 榴弾リング (Ring of Bursting Grenades) -- Pyro --

const CHEVREUSE_BURST_GRENADE: TalentScaling = TalentScaling {
    name: "爆発グレネードダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        3.6816, 3.9577, 4.2338, 4.6020, 4.8781, 5.1542, 5.5224, 5.8906, 6.2587, 6.6269, 6.9950,
        7.3632, 7.8234, 8.2836, 8.7438,
    ],
};

const CHEVREUSE_BURST_SECONDARY: TalentScaling = TalentScaling {
    name: "二次爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.4909, 0.5277, 0.5645, 0.6136, 0.6504, 0.6872, 0.7363, 0.7854, 0.8345, 0.8836, 0.9327,
        0.9818, 1.0431, 1.1045, 1.1658,
    ],
};

pub const CHEVREUSE: CharacterData = CharacterData {
    id: "chevreuse",
    name: "Chevreuse",
    element: Element::Pyro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Fontaine,
    base_hp: [1003.0, 10602.0, 11134.0, 11962.0],
    base_atk: [16.0, 171.0, 180.0, 193.0],
    base_def: [51.0, 536.0, 563.0, 605.0],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "ラインバヨネット突撃EX",
            hits: &[
                CHEVREUSE_NORMAL_1,
                CHEVREUSE_NORMAL_2,
                CHEVREUSE_NORMAL_3A,
                CHEVREUSE_NORMAL_3B,
                CHEVREUSE_NORMAL_4,
            ],
            charged: &[CHEVREUSE_CHARGED],
            plunging: &[
                CHEVREUSE_PLUNGE,
                CHEVREUSE_PLUNGE_LOW,
                CHEVREUSE_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "近距離急速射撃",
            scalings: &[
                CHEVREUSE_SKILL_PRESS,
                CHEVREUSE_SKILL_HOLD,
                CHEVREUSE_SKILL_OVERCHARGED,
            ],
        },
        elemental_burst: TalentData {
            name: "榴弾リング",
            scalings: &[CHEVREUSE_BURST_GRENADE, CHEVREUSE_BURST_SECONDARY],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Dehya
// =============================================================================

// -- Normal Attack: 拳闘術 (Sandstorm Assault) -- Physical --

const DEHYA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6212, 0.6717, 0.7223, 0.7945, 0.8451, 0.9029, 0.9823, 1.0618, 1.1412, 1.2279, 1.3146,
        1.4013, 1.4879, 1.5746, 1.6613,
    ],
};

const DEHYA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6171, 0.6673, 0.7176, 0.7893, 0.8395, 0.8970, 0.9759, 1.0548, 1.1337, 1.2199, 1.3060,
        1.3921, 1.4782, 1.5643, 1.6504,
    ],
};

const DEHYA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7663, 0.8287, 0.8911, 0.9802, 1.0425, 1.1138, 1.2118, 1.3099, 1.4079, 1.5148, 1.6217,
        1.7287, 1.8356, 1.9425, 2.0494,
    ],
};

const DEHYA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9529, 1.0305, 1.1080, 1.2188, 1.2964, 1.3851, 1.5069, 1.6288, 1.7507, 1.8837, 2.0166,
        2.1496, 2.2826, 2.4155, 2.5485,
    ],
};

// -- Charged Attack -- Physical --

const DEHYA_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5633, 0.6092, 0.6550, 0.7205, 0.7664, 0.8188, 0.8908, 0.9629, 1.0349, 1.1135, 1.1921,
        1.2707, 1.3493, 1.4279, 1.5065,
    ],
};

const DEHYA_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0182, 1.1011, 1.1840, 1.3024, 1.3853, 1.4800, 1.6102, 1.7405, 1.8707, 2.0128, 2.1549,
        2.2970, 2.4390, 2.5811, 2.7232,
    ],
};

// -- Plunging Attack -- Physical --

const DEHYA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
};

const DEHYA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const DEHYA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9460, 3.1842, 3.4225, 3.6825, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
};

// -- Elemental Skill: 熔鉄流獄 (Molten Inferno) -- Pyro --

const DEHYA_SKILL_INDOMITABLE: TalentScaling = TalentScaling {
    name: "不屈の炎ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1288, 1.2135, 1.2981, 1.4110, 1.4957, 1.5803, 1.6932, 1.8061, 1.9190, 2.0318, 2.1447,
        2.2576, 2.3987, 2.5398, 2.6809,
    ],
};

const DEHYA_SKILL_RANGE: TalentScaling = TalentScaling {
    name: "領域ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.3280, 1.4276, 1.5272, 1.6600, 1.7596, 1.8592, 1.9920, 2.1248, 2.2576, 2.3904, 2.5232,
        2.6560, 2.8220, 2.9880, 3.1540,
    ],
};

// -- Elemental Burst: 炎獅子の噛み付き (Leonine Bite) -- Pyro --

const DEHYA_BURST_FIST: TalentScaling = TalentScaling {
    name: "拳撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9870, 1.0610, 1.1351, 1.2338, 1.3078, 1.3818, 1.4805, 1.5792, 1.6779, 1.7766, 1.8753,
        1.9740, 2.0974, 2.2208, 2.3441,
    ],
};

const DEHYA_BURST_KICK: TalentScaling = TalentScaling {
    name: "蹴りダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.3930, 1.4975, 1.6020, 1.7413, 1.8457, 1.9502, 2.0895, 2.2288, 2.3681, 2.5074, 2.6467,
        2.7860, 2.9601, 3.1343, 3.3084,
    ],
};

pub const DEHYA: CharacterData = CharacterData {
    id: "dehya",
    name: "Dehya",
    element: Element::Pyro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Sumeru,
    base_hp: [1220.0, 13829.0, 14573.0, 15675.0],
    base_atk: [21.0, 234.0, 247.0, 265.0],
    base_def: [49.0, 554.0, 584.0, 628.0],
    ascension_stat: AscensionStat::Hp(0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "拳闘術",
            hits: &[
                DEHYA_NORMAL_1,
                DEHYA_NORMAL_2,
                DEHYA_NORMAL_3,
                DEHYA_NORMAL_4,
            ],
            charged: &[DEHYA_CHARGED_SPINNING, DEHYA_CHARGED_FINAL],
            plunging: &[DEHYA_PLUNGE, DEHYA_PLUNGE_LOW, DEHYA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "熔鉄流獄",
            scalings: &[DEHYA_SKILL_INDOMITABLE, DEHYA_SKILL_RANGE],
        },
        elemental_burst: TalentData {
            name: "炎獅子の噛み付き",
            scalings: &[DEHYA_BURST_FIST, DEHYA_BURST_KICK],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Gaming
// =============================================================================

// -- Normal Attack: 星辰裂き (Stellar Rend) -- Physical --

const GAMING_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8386, 0.9068, 0.9751, 1.0726, 1.1408, 1.2188, 1.3261, 1.4334, 1.5406, 1.6576, 1.7746,
        1.8916, 2.0086, 2.1257, 2.2427,
    ],
};

const GAMING_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7904, 0.8548, 0.9191, 1.0110, 1.0754, 1.1489, 1.2500, 1.3511, 1.4522, 1.5625, 1.6728,
        1.7831, 1.8934, 2.0037, 2.1140,
    ],
};

const GAMING_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0665, 1.1533, 1.2401, 1.3641, 1.4509, 1.5501, 1.6865, 1.8229, 1.9593, 2.1081, 2.2569,
        2.4057, 2.5545, 2.7034, 2.8522,
    ],
};

const GAMING_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2795, 1.3836, 1.4878, 1.6366, 1.7407, 1.8597, 2.0234, 2.1870, 2.3507, 2.5292, 2.7078,
        2.8863, 3.0648, 3.2434, 3.4219,
    ],
};

// -- Charged Attack -- Physical --

const GAMING_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6252, 0.6761, 0.7270, 0.7997, 0.8506, 0.9088, 0.9887, 1.0687, 1.1487, 1.2359, 1.3231,
        1.4104, 1.4976, 1.5849, 1.6721,
    ],
};

const GAMING_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1309, 1.2230, 1.3150, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933,
        2.5511, 2.7089, 2.8667, 3.0245,
    ],
};

// -- Plunging Attack -- Physical --

const GAMING_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6415, 0.6937, 0.7459, 0.8205, 0.8727, 0.9324, 1.0144, 1.0964, 1.1785, 1.2680, 1.3575,
        1.4470, 1.5365, 1.6260, 1.7155,
    ],
};

const GAMING_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2826, 1.3870, 1.4914, 1.6406, 1.7450, 1.8643, 2.0284, 2.1924, 2.3565, 2.5354, 2.7144,
        2.8934, 3.0724, 3.2513, 3.4303,
    ],
};

const GAMING_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.6021, 1.7325, 1.8629, 2.0492, 2.1796, 2.3286, 2.5335, 2.7384, 2.9434, 3.1669, 3.3905,
        3.6140, 3.8376, 4.0611, 4.2846,
    ],
};

// -- Elemental Skill: 瑞獣登場 (Bestial Ascent) -- Pyro --

const GAMING_SKILL_PLUNGE: TalentScaling = TalentScaling {
    name: "落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        2.3040, 2.4768, 2.6496, 2.8800, 3.0528, 3.2256, 3.4560, 3.6864, 3.9168, 4.1472, 4.3776,
        4.6080, 4.8960, 5.1840, 5.4720,
    ],
};

// -- Elemental Burst: 瑞獣の金舞 (Suanni's Gilded Dance) -- Pyro --

const GAMING_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        3.7040, 3.9818, 4.2596, 4.6300, 4.9078, 5.1856, 5.5560, 5.9264, 6.2968, 6.6672, 7.0376,
        7.4080, 7.8710, 8.3340, 8.7970,
    ],
};

pub const GAMING: CharacterData = CharacterData {
    id: "gaming",
    name: "Gaming",
    element: Element::Pyro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [957.0, 10120.0, 10628.0, 11419.0],
    base_atk: [25.0, 267.0, 281.0, 302.0],
    base_def: [59.0, 623.0, 654.0, 703.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "星辰裂き",
            hits: &[
                GAMING_NORMAL_1,
                GAMING_NORMAL_2,
                GAMING_NORMAL_3,
                GAMING_NORMAL_4,
            ],
            charged: &[GAMING_CHARGED_SPINNING, GAMING_CHARGED_FINAL],
            plunging: &[GAMING_PLUNGE, GAMING_PLUNGE_LOW, GAMING_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "瑞獣登場",
            scalings: &[GAMING_SKILL_PLUNGE],
        },
        elemental_burst: TalentData {
            name: "瑞獣の金舞",
            scalings: &[GAMING_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Hu Tao
// =============================================================================

// -- Normal Attack: 往生秘伝槍法 (Secret Spear of Wangsheng) -- Physical --

const HU_TAO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4689, 0.5008, 0.5328, 0.5754, 0.6074, 0.6447, 0.6926, 0.7406, 0.7885, 0.8365, 0.8844,
        0.9324, 0.9804, 1.0283, 1.0763,
    ],
};

const HU_TAO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4825, 0.5154, 0.5483, 0.5922, 0.6251, 0.6635, 0.7128, 0.7622, 0.8115, 0.8609, 0.9102,
        0.9596, 1.0089, 1.0583, 1.1076,
    ],
};

const HU_TAO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6105, 0.6521, 0.6938, 0.7493, 0.7909, 0.8394, 0.9019, 0.9643, 1.0268, 1.0892, 1.1516,
        1.2141, 1.2765, 1.3389, 1.4014,
    ],
};

const HU_TAO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6564, 0.7012, 0.7459, 0.8056, 0.8503, 0.9026, 0.9697, 1.0368, 1.1040, 1.1711, 1.2382,
        1.3054, 1.3725, 1.4396, 1.5068,
    ],
};

const HU_TAO_NORMAL_5A: TalentScaling = TalentScaling {
    name: "5段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3327, 0.3554, 0.3781, 0.4084, 0.4310, 0.4575, 0.4915, 0.5256, 0.5596, 0.5936, 0.6277,
        0.6617, 0.6957, 0.7298, 0.7638,
    ],
};

const HU_TAO_NORMAL_5B: TalentScaling = TalentScaling {
    name: "5段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3520, 0.3760, 0.4000, 0.4320, 0.4560, 0.4840, 0.5200, 0.5560, 0.5920, 0.6280, 0.6640,
        0.7000, 0.7360, 0.7720, 0.8080,
    ],
};

const HU_TAO_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8596, 0.9182, 0.9768, 1.0549, 1.1136, 1.1819, 1.2698, 1.3578, 1.4457, 1.5336, 1.6215,
        1.7094, 1.7973, 1.8852, 1.9731,
    ],
};

// -- Charged Attack -- Physical --

const HU_TAO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.3596, 1.4523, 1.5450, 1.6686, 1.7613, 1.8695, 2.0085, 2.1476, 2.2866, 2.4257, 2.5647,
        2.7038, 2.8428, 2.9819, 3.1209,
    ],
};

// -- Plunging Attack -- Physical --

const HU_TAO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6542, 0.6988, 0.7434, 0.8029, 0.8475, 0.8995, 0.9664, 1.0333, 1.1002, 1.1671, 1.2340,
        1.3010, 1.3679, 1.4348, 1.5017,
    ],
};

const HU_TAO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.3081, 1.3973, 1.4865, 1.6054, 1.6946, 1.7986, 1.9324, 2.0662, 2.2000, 2.3338, 2.4676,
        2.6013, 2.7351, 2.8689, 3.0027,
    ],
};

const HU_TAO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.6339, 1.7453, 1.8567, 2.0052, 2.1166, 2.2466, 2.4137, 2.5808, 2.7479, 2.9150, 3.0821,
        3.2492, 3.4163, 3.5834, 3.7505,
    ],
};

// -- Elemental Skill: 蝶導来世 (Guide to Afterlife) -- Pyro --

const HU_TAO_SKILL_BLOOD_BLOSSOM: TalentScaling = TalentScaling {
    name: "血梅香ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Pyro),
    values: [
        0.6400, 0.6880, 0.7360, 0.8000, 0.8480, 0.8960, 0.9600, 1.0240, 1.0880, 1.1520, 1.2160,
        1.2800, 1.3600, 1.4400, 1.5200,
    ],
};

// -- Elemental Burst: 安神秘法 (Spirit Soother) -- Pyro --

const HU_TAO_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        3.0327, 3.2143, 3.3959, 3.6320, 3.8136, 3.9952, 4.2313, 4.4674, 4.7034, 4.9395, 5.1756,
        5.4117, 5.6478, 5.8838, 6.1199,
    ],
};

const HU_TAO_BURST_LOW_HP: TalentScaling = TalentScaling {
    name: "低HPスキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        3.7909, 4.0179, 4.2449, 4.5400, 4.7670, 4.9940, 5.2891, 5.5842, 5.8793, 6.1744, 6.4695,
        6.7646, 7.0597, 7.3548, 7.6499,
    ],
};

pub const HU_TAO: CharacterData = CharacterData {
    id: "hu_tao",
    name: "Hu Tao",
    element: Element::Pyro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [1211.0, 13721.0, 14459.0, 15552.0],
    base_atk: [8.0, 94.0, 99.0, 106.0],
    base_def: [68.0, 773.0, 815.0, 876.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "往生秘伝槍法",
            hits: &[
                HU_TAO_NORMAL_1,
                HU_TAO_NORMAL_2,
                HU_TAO_NORMAL_3,
                HU_TAO_NORMAL_4,
                HU_TAO_NORMAL_5A,
                HU_TAO_NORMAL_5B,
                HU_TAO_NORMAL_6,
            ],
            charged: &[HU_TAO_CHARGED],
            plunging: &[HU_TAO_PLUNGE, HU_TAO_PLUNGE_LOW, HU_TAO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "蝶導来世",
            scalings: &[HU_TAO_SKILL_BLOOD_BLOSSOM],
        },
        elemental_burst: TalentData {
            name: "安神秘法",
            scalings: &[HU_TAO_BURST, HU_TAO_BURST_LOW_HP],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Klee
// =============================================================================

// -- Normal Attack: ドッカンはなび (Kaboom!) -- Pyro (Catalyst) --

const KLEE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.7216, 0.7757, 0.8298, 0.9020, 0.9561, 1.0102, 1.0824, 1.1546, 1.2267, 1.2989, 1.3739,
        1.4721, 1.5702, 1.6683, 1.7665,
    ],
};

const KLEE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.6240, 0.6708, 0.7176, 0.7800, 0.8268, 0.8736, 0.9360, 0.9984, 1.0608, 1.1232, 1.1881,
        1.2730, 1.3578, 1.4427, 1.5276,
    ],
};

const KLEE_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.8992, 0.9666, 1.0341, 1.1240, 1.1914, 1.2589, 1.3488, 1.4387, 1.5286, 1.6186, 1.7121,
        1.8344, 1.9567, 2.0790, 2.2012,
    ],
};

// -- Charged Attack -- Pyro (Catalyst) --

const KLEE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.5736, 1.6916, 1.8096, 1.9670, 2.0850, 2.2030, 2.3604, 2.5178, 2.6751, 2.8325, 2.9961,
        3.2101, 3.4242, 3.6382, 3.8522,
    ],
};

// -- Plunging Attack -- Pyro (Catalyst) --

const KLEE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const KLEE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const KLEE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: ボンボン爆弾 (Jumpy Dumpty) -- Pyro --

const KLEE_SKILL_BOUNCE: TalentScaling = TalentScaling {
    name: "ボンボン爆弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9520, 1.0234, 1.0948, 1.1900, 1.2614, 1.3328, 1.4280, 1.5232, 1.6184, 1.7136, 1.8088,
        1.9040, 2.0230, 2.1420, 2.2610,
    ],
};

const KLEE_SKILL_MINE: TalentScaling = TalentScaling {
    name: "ブービートラップダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.3280, 0.3526, 0.3772, 0.4100, 0.4346, 0.4592, 0.4920, 0.5248, 0.5576, 0.5904, 0.6232,
        0.6560, 0.6970, 0.7380, 0.7790,
    ],
};

// -- Elemental Burst: ドッカン花火 (Sparks 'n' Splash) -- Pyro --

const KLEE_BURST: TalentScaling = TalentScaling {
    name: "ドッカン花火ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.4264, 0.4584, 0.4904, 0.5330, 0.5650, 0.5970, 0.6396, 0.6822, 0.7249, 0.7675, 0.8102,
        0.8528, 0.9061, 0.9594, 1.0127,
    ],
};

pub const KLEE: CharacterData = CharacterData {
    id: "klee",
    name: "Klee",
    element: Element::Pyro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [801.0, 9076.0, 9563.0, 10287.0],
    base_atk: [24.0, 274.0, 289.0, 311.0],
    base_def: [48.0, 542.0, 572.0, 615.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Pyro, 0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "ドッカンはなび",
            hits: &[KLEE_NORMAL_1, KLEE_NORMAL_2, KLEE_NORMAL_3],
            charged: &[KLEE_CHARGED],
            plunging: &[KLEE_PLUNGE, KLEE_PLUNGE_LOW, KLEE_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "ボンボン爆弾",
            scalings: &[KLEE_SKILL_BOUNCE, KLEE_SKILL_MINE],
        },
        elemental_burst: TalentData {
            name: "ドッカン花火",
            scalings: &[KLEE_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Lyney
// =============================================================================

// -- Normal Attack: カードフォース・トランスロケーション (Card Force Translocation) -- Physical --

const LYNEY_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3879, 0.4194, 0.4510, 0.4961, 0.5277, 0.5638, 0.6134, 0.6630, 0.7126, 0.7667, 0.8208,
        0.8749, 0.9291, 0.9832, 1.0373,
    ],
};

const LYNEY_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3801, 0.4111, 0.4420, 0.4862, 0.5171, 0.5525, 0.6011, 0.6497, 0.6984, 0.7514, 0.8044,
        0.8575, 0.9105, 0.9636, 1.0166,
    ],
};

const LYNEY_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2726, 0.2948, 0.3170, 0.3487, 0.3709, 0.3963, 0.4311, 0.4660, 0.5009, 0.5389, 0.5769,
        0.6150, 0.6530, 0.6911, 0.7291,
    ],
};

const LYNEY_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2726, 0.2948, 0.3170, 0.3487, 0.3709, 0.3963, 0.4311, 0.4660, 0.5009, 0.5389, 0.5769,
        0.6150, 0.6530, 0.6911, 0.7291,
    ],
};

const LYNEY_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5693, 0.6157, 0.6620, 0.7282, 0.7745, 0.8275, 0.9003, 0.9731, 1.0460, 1.1254, 1.2048,
        1.2843, 1.3637, 1.4432, 1.5226,
    ],
};

// -- Aimed Shot -- Pyro (charged) --

const LYNEY_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const LYNEY_AIMED_CHARGE1: TalentScaling = TalentScaling {
    name: "1段チャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

const LYNEY_AIMED_PROP: TalentScaling = TalentScaling {
    name: "プロップアローダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.7280, 1.8576, 1.9872, 2.1600, 2.2896, 2.4192, 2.5920, 2.7648, 2.9376, 3.1104, 3.2832,
        3.4560, 3.6720, 3.8880, 4.1040,
    ],
};

const LYNEY_AIMED_PYROTECHNIC: TalentScaling = TalentScaling {
    name: "パイロテクニックストライク",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        2.1200, 2.2790, 2.4380, 2.6500, 2.8090, 2.9680, 3.1800, 3.3920, 3.6040, 3.8160, 4.0280,
        4.2400, 4.5050, 4.7700, 5.0350,
    ],
};

// -- Plunging Attack -- Physical --

const LYNEY_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const LYNEY_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const LYNEY_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 眩惑マジック (Bewildering Lights) -- Pyro --

const LYNEY_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.6720, 1.7974, 1.9228, 2.0900, 2.2154, 2.3408, 2.5080, 2.6752, 2.8424, 3.0096, 3.1768,
        3.3440, 3.5530, 3.7620, 3.9710,
    ],
};

const LYNEY_SKILL_HAT: TalentScaling = TalentScaling {
    name: "ハットトリックダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5320, 0.5719, 0.6118, 0.6650, 0.7049, 0.7448, 0.7980, 0.8512, 0.9044, 0.9576, 1.0108,
        1.0640, 1.1305, 1.1970, 1.2635,
    ],
};

// -- Elemental Burst: 大魔術・ミラクルパレード (Wondrous Trick: Miracle Parade) -- Pyro --

const LYNEY_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.5400, 1.6555, 1.7710, 1.9250, 2.0405, 2.1560, 2.3100, 2.4640, 2.6180, 2.7720, 2.9260,
        3.0800, 3.2725, 3.4650, 3.6575,
    ],
};

const LYNEY_BURST_EXPLOSION: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        4.1400, 4.4505, 4.7610, 5.1750, 5.4855, 5.7960, 6.2100, 6.6240, 7.0380, 7.4520, 7.8660,
        8.2800, 8.7975, 9.3150, 9.8325,
    ],
};

pub const LYNEY: CharacterData = CharacterData {
    id: "lyney",
    name: "Lyney",
    element: Element::Pyro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [858.0, 9724.0, 10247.0, 11021.0],
    base_atk: [25.0, 281.0, 296.0, 318.0],
    base_def: [42.0, 475.0, 500.0, 538.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "カードフォース・トランスロケーション",
            hits: &[
                LYNEY_NORMAL_1,
                LYNEY_NORMAL_2,
                LYNEY_NORMAL_3A,
                LYNEY_NORMAL_3B,
                LYNEY_NORMAL_4,
            ],
            charged: &[
                LYNEY_AIMED,
                LYNEY_AIMED_CHARGE1,
                LYNEY_AIMED_PROP,
                LYNEY_AIMED_PYROTECHNIC,
            ],
            plunging: &[LYNEY_PLUNGE, LYNEY_PLUNGE_LOW, LYNEY_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "眩惑マジック",
            scalings: &[LYNEY_SKILL, LYNEY_SKILL_HAT],
        },
        elemental_burst: TalentData {
            name: "大魔術・ミラクルパレード",
            scalings: &[LYNEY_BURST, LYNEY_BURST_EXPLOSION],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Mavuika
// =============================================================================

// -- Normal Attack: 焔は命を紡ぐ (Flames Weave Life) -- Physical --

const MAVUIKA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8004, 0.8655, 0.9306, 1.0237, 1.0888, 1.1633, 1.2657, 1.3680, 1.4704, 1.5821, 1.6938,
        1.8054, 1.9171, 2.0288, 2.1405,
    ],
};

const MAVUIKA_NORMAL_2A: TalentScaling = TalentScaling {
    name: "2段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3648, 0.3945, 0.4242, 0.4666, 0.4963, 0.5302, 0.5769, 0.6236, 0.6702, 0.7211, 0.7720,
        0.8229, 0.8738, 0.9247, 0.9756,
    ],
};

const MAVUIKA_NORMAL_2B: TalentScaling = TalentScaling {
    name: "2段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3322, 0.3593, 0.3863, 0.4249, 0.4520, 0.4829, 0.5254, 0.5679, 0.6104, 0.6567, 0.7031,
        0.7495, 0.7958, 0.8422, 0.8885,
    ],
};

const MAVUIKA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1619, 1.2565, 1.3511, 1.4862, 1.5808, 1.6889, 1.8375, 1.9861, 2.1347, 2.2968, 2.4590,
        2.6211, 2.7832, 2.9454, 3.1075,
    ],
};

// -- Charged Attack -- Physical --

const MAVUIKA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.9384, 2.0962, 2.2540, 2.4794, 2.6372, 2.8175, 3.0654, 3.3134, 3.5613, 3.8318, 4.1023,
        4.3728, 4.6432, 4.9137, 5.1842,
    ],
};

// -- Plunging Attack -- Physical --

const MAVUIKA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
};

const MAVUIKA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const MAVUIKA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9460, 3.1842, 3.4225, 3.6825, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
};

// -- Elemental Skill: その名の瞬間 (The Named Moment) -- Pyro --

const MAVUIKA_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.7440, 0.7998, 0.8556, 0.9300, 0.9858, 1.0416, 1.1160, 1.1904, 1.2648, 1.3392, 1.4136,
        1.4880, 1.5810, 1.6740, 1.7670,
    ],
};

const MAVUIKA_SKILL_RING: TalentScaling = TalentScaling {
    name: "灼熱の輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2800, 1.3760, 1.4720, 1.6000, 1.6960, 1.7920, 1.9200, 2.0480, 2.1760, 2.3040, 2.4320,
        2.5600, 2.7200, 2.8800, 3.0400,
    ],
};

const MAVUIKA_SKILL_FLAMESTRIDER_N1: TalentScaling = TalentScaling {
    name: "炎騎通常1段",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5726, 0.6193, 0.6659, 0.7325, 0.7791, 0.8323, 0.9056, 0.9788, 1.0521, 1.1320, 1.2119,
        1.2918, 1.3717, 1.4516, 1.5315,
    ],
};

const MAVUIKA_SKILL_FLAMESTRIDER_N2: TalentScaling = TalentScaling {
    name: "炎騎通常2段",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5913, 0.6395, 0.6876, 0.7563, 0.8045, 0.8595, 0.9351, 1.0108, 1.0864, 1.1689, 1.2514,
        1.3339, 1.4164, 1.4989, 1.5815,
    ],
};

const MAVUIKA_SKILL_FLAMESTRIDER_N3: TalentScaling = TalentScaling {
    name: "炎騎通常3段",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.6999, 0.7568, 0.8138, 0.8952, 0.9521, 1.0173, 1.1068, 1.1963, 1.2858, 1.3835, 1.4811,
        1.5788, 1.6764, 1.7741, 1.8717,
    ],
};

const MAVUIKA_SKILL_FLAMESTRIDER_N4: TalentScaling = TalentScaling {
    name: "炎騎通常4段",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.6970, 0.7538, 0.8105, 0.8916, 0.9483, 1.0132, 1.1023, 1.1915, 1.2806, 1.3779, 1.4751,
        1.5724, 1.6697, 1.7669, 1.8642,
    ],
};

const MAVUIKA_SKILL_FLAMESTRIDER_N5: TalentScaling = TalentScaling {
    name: "炎騎通常5段",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9100, 0.9841, 1.0582, 1.1640, 1.2381, 1.3227, 1.4391, 1.5555, 1.6719, 1.7989, 1.9259,
        2.0529, 2.1799, 2.3068, 2.4338,
    ],
};

// -- Elemental Burst: 焼天の時 (Hour of Burning Skies) -- Pyro --

const MAVUIKA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        4.4480, 4.7816, 5.1152, 5.5600, 5.8936, 6.2272, 6.6720, 7.1168, 7.5616, 8.0064, 8.4512,
        8.8960, 9.4520, 10.0080, 10.5640,
    ],
};

pub const MAVUIKA: CharacterData = CharacterData {
    id: "mavuika",
    name: "Mavuika",
    element: Element::Pyro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [977.0, 11074.0, 11670.0, 12552.0],
    base_atk: [28.0, 317.0, 334.0, 359.0],
    base_def: [62.0, 698.0, 736.0, 792.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "焔は命を紡ぐ",
            hits: &[
                MAVUIKA_NORMAL_1,
                MAVUIKA_NORMAL_2A,
                MAVUIKA_NORMAL_2B,
                MAVUIKA_NORMAL_3,
            ],
            charged: &[MAVUIKA_CHARGED],
            plunging: &[MAVUIKA_PLUNGE, MAVUIKA_PLUNGE_LOW, MAVUIKA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "その名の瞬間",
            scalings: &[
                MAVUIKA_SKILL,
                MAVUIKA_SKILL_RING,
                MAVUIKA_SKILL_FLAMESTRIDER_N1,
                MAVUIKA_SKILL_FLAMESTRIDER_N2,
                MAVUIKA_SKILL_FLAMESTRIDER_N3,
                MAVUIKA_SKILL_FLAMESTRIDER_N4,
                MAVUIKA_SKILL_FLAMESTRIDER_N5,
            ],
        },
        elemental_burst: TalentData {
            name: "焼天の時",
            scalings: &[MAVUIKA_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Thoma
// =============================================================================

// -- Normal Attack: 迅烈な槍 (Swiftshatter Spear) -- Physical --

const THOMA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4439, 0.4801, 0.5162, 0.5678, 0.6040, 0.6453, 0.7020, 0.7588, 0.8156, 0.8775, 0.9395,
        1.0014, 1.0634, 1.1253, 1.1873,
    ],
};

const THOMA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4363, 0.4718, 0.5073, 0.5580, 0.5935, 0.6341, 0.6899, 0.7457, 0.8015, 0.8624, 0.9233,
        0.9842, 1.0450, 1.1059, 1.1668,
    ],
};

const THOMA_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2679, 0.2897, 0.3115, 0.3427, 0.3645, 0.3894, 0.4236, 0.4579, 0.4922, 0.5296, 0.5669,
        0.6043, 0.6417, 0.6791, 0.7165,
    ],
};

const THOMA_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6736, 0.7284, 0.7832, 0.8615, 0.9163, 0.9790, 1.0652, 1.1513, 1.2375, 1.3314, 1.4254,
        1.5194, 1.6134, 1.7074, 1.8014,
    ],
};

// -- Charged Attack -- Physical --

const THOMA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1275, 1.2192, 1.3110, 1.4421, 1.5339, 1.6388, 1.7830, 1.9272, 2.0714, 2.2287, 2.3860,
        2.5433, 2.7007, 2.8580, 3.0153,
    ],
};

// -- Plunging Attack -- Physical --

const THOMA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const THOMA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const THOMA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 烈焔侍立 (Blazing Blessing) -- Pyro --

const THOMA_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.4640, 1.5738, 1.6836, 1.8300, 1.9398, 2.0496, 2.1960, 2.3424, 2.4888, 2.6352, 2.7816,
        2.9280, 3.1110, 3.2940, 3.4770,
    ],
};

// -- Elemental Burst: 真紅熾炎の大鎧 (Crimson Ooyoroi) -- Pyro --

const THOMA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.8800, 0.9460, 1.0120, 1.1000, 1.1660, 1.2320, 1.3200, 1.4080, 1.4960, 1.5840, 1.6720,
        1.7600, 1.8700, 1.9800, 2.0900,
    ],
};

const THOMA_BURST_FIERY: TalentScaling = TalentScaling {
    name: "炎の援護ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5800, 0.6235, 0.6670, 0.7250, 0.7685, 0.8120, 0.8700, 0.9280, 0.9860, 1.0440, 1.1020,
        1.1600, 1.2325, 1.3050, 1.3775,
    ],
};

pub const THOMA: CharacterData = CharacterData {
    id: "thoma",
    name: "Thoma",
    element: Element::Pyro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Inazuma,
    base_hp: [866.0, 9156.0, 9616.0, 10331.0],
    base_atk: [17.0, 179.0, 188.0, 202.0],
    base_def: [63.0, 665.0, 699.0, 751.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "迅烈な槍",
            hits: &[
                THOMA_NORMAL_1,
                THOMA_NORMAL_2,
                THOMA_NORMAL_3A,
                THOMA_NORMAL_3B,
            ],
            charged: &[THOMA_CHARGED],
            plunging: &[THOMA_PLUNGE, THOMA_PLUNGE_LOW, THOMA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "烈焔侍立",
            scalings: &[THOMA_SKILL],
        },
        elemental_burst: TalentData {
            name: "真紅熾炎の大鎧",
            scalings: &[THOMA_BURST, THOMA_BURST_FIERY],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Xiangling
// =============================================================================

// -- Normal Attack: 旋火棍法 (Dough-Fu) -- Physical --

const XIANGLING_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4205, 0.4548, 0.4890, 0.5379, 0.5721, 0.6113, 0.6650, 0.7188, 0.7726, 0.8313, 0.8985,
        0.9776, 1.0567, 1.1358, 1.2220,
    ],
};

const XIANGLING_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4214, 0.4557, 0.4900, 0.5390, 0.5733, 0.6125, 0.6664, 0.7203, 0.7742, 0.8330, 0.9004,
        0.9796, 1.0588, 1.1381, 1.2245,
    ],
};

const XIANGLING_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2606, 0.2818, 0.3030, 0.3333, 0.3545, 0.3788, 0.4121, 0.4454, 0.4787, 0.5151, 0.5568,
        0.6058, 0.6548, 0.7037, 0.7572,
    ],
};

const XIANGLING_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.1410, 0.1525, 0.1640, 0.1804, 0.1919, 0.2050, 0.2230, 0.2411, 0.2591, 0.2788, 0.3014,
        0.3279, 0.3544, 0.3809, 0.4098,
    ],
};

const XIANGLING_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (×4)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7104, 0.7682, 0.8260, 0.9086, 0.9664, 1.0325, 1.1234, 1.2142, 1.3051, 1.4042, 1.5178,
        1.6513, 1.7849, 1.9185, 2.0642,
    ],
};

const XIANGLING_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2169, 1.3160, 1.4150, 1.5565, 1.6556, 1.7688, 1.9244, 2.0801, 2.2357, 2.4055, 2.6001,
        2.8289, 3.0567, 3.2865, 3.5361,
    ],
};

// -- Charged Attack -- Physical --

const XIANGLING_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2169, 1.3160, 1.4150, 1.5565, 1.6556, 1.7688, 1.9244, 2.0801, 2.2357, 2.4055, 2.5753,
        2.7451, 2.9149, 3.0847, 3.2545,
    ],
};

// -- Plunging Attack -- Physical --

const XIANGLING_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const XIANGLING_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const XIANGLING_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: グゥオパァー出撃 (Guoba Attack) -- Pyro --

const XIANGLING_SKILL: TalentScaling = TalentScaling {
    name: "噴火ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1128, 1.1963, 1.2797, 1.3910, 1.4745, 1.5579, 1.6692, 1.7805, 1.8918, 2.0030, 2.1143,
        2.2256, 2.3647, 2.5038, 2.6429,
    ],
};

// -- Elemental Burst: 旋火輪 (Pyronado) -- Pyro --

const XIANGLING_BURST_1: TalentScaling = TalentScaling {
    name: "1段旋火輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.7200, 0.7740, 0.8280, 0.9000, 0.9540, 1.0080, 1.0800, 1.1520, 1.2240, 1.2960, 1.3680,
        1.4400, 1.5300, 1.6200, 1.7100,
    ],
};

const XIANGLING_BURST_2: TalentScaling = TalentScaling {
    name: "2段旋火輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.8800, 0.9460, 1.0120, 1.1000, 1.1660, 1.2320, 1.3200, 1.4080, 1.4960, 1.5840, 1.6720,
        1.7600, 1.8700, 1.9800, 2.0900,
    ],
};

const XIANGLING_BURST_3: TalentScaling = TalentScaling {
    name: "3段旋火輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.0960, 1.1782, 1.2604, 1.3700, 1.4522, 1.5344, 1.6440, 1.7536, 1.8632, 1.9728, 2.0824,
        2.1920, 2.3290, 2.4660, 2.6030,
    ],
};

const XIANGLING_BURST_PYRONADO: TalentScaling = TalentScaling {
    name: "旋火輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1200, 1.2040, 1.2880, 1.4000, 1.4840, 1.5680, 1.6800, 1.7920, 1.9040, 2.0160, 2.1280,
        2.2400, 2.3800, 2.5200, 2.6600,
    ],
};

pub const XIANGLING: CharacterData = CharacterData {
    id: "xiangling",
    name: "Xiangling",
    element: Element::Pyro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [912.0, 10122.0, 10647.0, 10875.0],
    base_atk: [19.0, 200.0, 210.0, 225.0],
    base_def: [56.0, 593.0, 623.0, 669.0],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "旋火棍法",
            hits: &[
                XIANGLING_NORMAL_1,
                XIANGLING_NORMAL_2,
                XIANGLING_NORMAL_3A,
                XIANGLING_NORMAL_3B,
                XIANGLING_NORMAL_4,
                XIANGLING_NORMAL_5,
            ],
            charged: &[XIANGLING_CHARGED],
            plunging: &[
                XIANGLING_PLUNGE,
                XIANGLING_PLUNGE_LOW,
                XIANGLING_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "グゥオパァー出撃",
            scalings: &[XIANGLING_SKILL],
        },
        elemental_burst: TalentData {
            name: "旋火輪",
            scalings: &[
                XIANGLING_BURST_1,
                XIANGLING_BURST_2,
                XIANGLING_BURST_3,
                XIANGLING_BURST_PYRONADO,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Xinyan
// =============================================================================

// -- Normal Attack: 炎舞 (Dance on Fire) -- Physical --

const XINYAN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7654, 0.8277, 0.8900, 0.9790, 1.0413, 1.1125, 1.2104, 1.3083, 1.4062, 1.5130, 1.6198,
        1.7266, 1.8334, 1.9402, 2.0470,
    ],
};

const XINYAN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7396, 0.7998, 0.8600, 0.9460, 1.0062, 1.0750, 1.1696, 1.2642, 1.3588, 1.4620, 1.5652,
        1.6684, 1.7716, 1.8748, 1.9780,
    ],
};

const XINYAN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9546, 1.0323, 1.1100, 1.2210, 1.2987, 1.3875, 1.5096, 1.6317, 1.7538, 1.8870, 2.0202,
        2.1534, 2.2866, 2.4198, 2.5530,
    ],
};

const XINYAN_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1584, 1.2527, 1.3470, 1.4817, 1.5760, 1.6838, 1.8319, 1.9801, 2.1283, 2.2899, 2.4515,
        2.6132, 2.7748, 2.9365, 3.0981,
    ],
};

// -- Charged Attack -- Physical --

const XINYAN_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6255, 0.6764, 0.7273, 0.8000, 0.8509, 0.9091, 0.9891, 1.0691, 1.1491, 1.2364, 1.3236,
        1.4109, 1.4982, 1.5855, 1.6727,
    ],
};

const XINYAN_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1309, 1.2230, 1.3150, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933,
        2.5511, 2.7089, 2.8667, 3.0245,
    ],
};

// -- Plunging Attack -- Physical --

const XINYAN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
};

const XINYAN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const XINYAN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9460, 3.1842, 3.4225, 3.6825, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
};

// -- Elemental Skill: 情熱のスイーパー (Sweeping Fervor) -- Pyro --

const XINYAN_SKILL_SWING: TalentScaling = TalentScaling {
    name: "振り回しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.6960, 1.8232, 1.9504, 2.1200, 2.2472, 2.3744, 2.5440, 2.7136, 2.8832, 3.0528, 3.2224,
        3.3920, 3.6040, 3.8160, 4.0280,
    ],
};

const XINYAN_SKILL_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.3360, 0.3612, 0.3864, 0.4200, 0.4452, 0.4704, 0.5040, 0.5376, 0.5712, 0.6048, 0.6384,
        0.6720, 0.7140, 0.7560, 0.7980,
    ],
};

// -- Elemental Burst: 叛逆の弾き (Riff Revolution) -- Pyro / Physical --

const XINYAN_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        3.4080, 3.6636, 3.9192, 4.2600, 4.5156, 4.7712, 5.1120, 5.4528, 5.7936, 6.1344, 6.4752,
        6.8160, 7.2420, 7.6680, 8.0940,
    ],
};

const XINYAN_BURST_DOT: TalentScaling = TalentScaling {
    name: "炎のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.4000, 0.4300, 0.4600, 0.5000, 0.5300, 0.5600, 0.6000, 0.6400, 0.6800, 0.7200, 0.7600,
        0.8000, 0.8500, 0.9000, 0.9500,
    ],
};

pub const XINYAN: CharacterData = CharacterData {
    id: "xinyan",
    name: "Xinyan",
    element: Element::Pyro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [939.0, 9927.0, 10425.0, 11201.0],
    base_atk: [21.0, 220.0, 231.0, 249.0],
    base_def: [67.0, 708.0, 743.0, 799.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "炎舞",
            hits: &[
                XINYAN_NORMAL_1,
                XINYAN_NORMAL_2,
                XINYAN_NORMAL_3,
                XINYAN_NORMAL_4,
            ],
            charged: &[XINYAN_CHARGED_SPINNING, XINYAN_CHARGED_FINAL],
            plunging: &[XINYAN_PLUNGE, XINYAN_PLUNGE_LOW, XINYAN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "情熱のスイーパー",
            scalings: &[XINYAN_SKILL_SWING, XINYAN_SKILL_DOT],
        },
        elemental_burst: TalentData {
            name: "叛逆の弾き",
            scalings: &[XINYAN_BURST, XINYAN_BURST_DOT],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Yanfei
// =============================================================================

// -- Normal Attack: 封蝋の印 (Seal of Approval) -- Pyro (Catalyst) --

const YANFEI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5834, 0.6272, 0.6709, 0.7293, 0.7730, 0.8168, 0.8751, 0.9335, 0.9918, 1.0501, 1.1085,
        1.1668, 1.2398, 1.3127, 1.3856,
    ],
};

const YANFEI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5213, 0.5604, 0.5994, 0.6516, 0.6907, 0.7298, 0.7819, 0.8340, 0.8861, 0.9383, 0.9904,
        1.0425, 1.1077, 1.1728, 1.2380,
    ],
};

const YANFEI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.7601, 0.8171, 0.8741, 0.9502, 1.0072, 1.0642, 1.1402, 1.2162, 1.2922, 1.3682, 1.4442,
        1.5203, 1.6153, 1.7103, 1.8053,
    ],
};

// -- Charged Attack -- Pyro (Catalyst, seal-dependent) --

const YANFEI_CHARGED_0: TalentScaling = TalentScaling {
    name: "重撃ダメージ (0印)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9823, 1.0411, 1.0999, 1.1764, 1.2352, 1.2940, 1.3705, 1.4470, 1.5234, 1.5999, 1.6764,
        1.7528, 1.8293, 1.9058, 1.9822,
    ],
};

const YANFEI_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ (1印)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1556, 1.2248, 1.2940, 1.3840, 1.4532, 1.5224, 1.6124, 1.7023, 1.7923, 1.8822, 1.9722,
        2.0622, 2.1521, 2.2421, 2.3320,
    ],
};

const YANFEI_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ (2印)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.3290, 1.4086, 1.4881, 1.5916, 1.6712, 1.7508, 1.8542, 1.9577, 2.0611, 2.1646, 2.2680,
        2.3715, 2.4749, 2.5784, 2.6818,
    ],
};

const YANFEI_CHARGED_3: TalentScaling = TalentScaling {
    name: "重撃ダメージ (3印)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.5023, 1.5923, 1.6823, 1.7992, 1.8892, 1.9791, 2.0961, 2.2130, 2.3300, 2.4469, 2.5639,
        2.6808, 2.7978, 2.9147, 3.0317,
    ],
};

const YANFEI_CHARGED_4: TalentScaling = TalentScaling {
    name: "重撃ダメージ (4印)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.6757, 1.7760, 1.8764, 2.0068, 2.1071, 2.2075, 2.3379, 2.4684, 2.5988, 2.7292, 2.8597,
        2.9901, 3.1206, 3.2510, 3.3815,
    ],
};

// -- Plunging Attack -- Pyro (Catalyst) --

const YANFEI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const YANFEI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const YANFEI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 丹書契約 (Signed Edict) -- Pyro --

const YANFEI_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.6960, 1.8232, 1.9504, 2.1200, 2.2472, 2.3744, 2.5440, 2.7136, 2.8832, 3.0528, 3.2224,
        3.3920, 3.6040, 3.8160, 4.0280,
    ],
};

// -- Elemental Burst: 契約成立 (Done Deal) -- Pyro --

const YANFEI_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.8240, 1.9608, 2.0976, 2.2800, 2.4168, 2.5536, 2.7360, 2.9184, 3.1008, 3.2832, 3.4656,
        3.6480, 3.8760, 4.1040, 4.3320,
    ],
};

pub const YANFEI: CharacterData = CharacterData {
    id: "yanfei",
    name: "Yanfei",
    element: Element::Pyro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [784.0, 8289.0, 8705.0, 9352.0],
    base_atk: [20.0, 213.0, 223.0, 240.0],
    base_def: [49.0, 520.0, 546.0, 587.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Pyro, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "封蝋の印",
            hits: &[YANFEI_NORMAL_1, YANFEI_NORMAL_2, YANFEI_NORMAL_3],
            charged: &[
                YANFEI_CHARGED_0,
                YANFEI_CHARGED_1,
                YANFEI_CHARGED_2,
                YANFEI_CHARGED_3,
                YANFEI_CHARGED_4,
            ],
            plunging: &[YANFEI_PLUNGE, YANFEI_PLUNGE_LOW, YANFEI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "丹書契約",
            scalings: &[YANFEI_SKILL],
        },
        elemental_burst: TalentData {
            name: "契約成立",
            scalings: &[YANFEI_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Yoimiya
// =============================================================================

// -- Normal Attack: 打ち上げ花火 (Firework Flare-Up) -- Physical --

const YOIMIYA_NORMAL_1A: TalentScaling = TalentScaling {
    name: "1段ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3564, 0.3807, 0.4050, 0.4374, 0.4617, 0.4901, 0.5265, 0.5630, 0.5994, 0.6359, 0.6723,
        0.7088, 0.7452, 0.7817, 0.8181,
    ],
};

const YOIMIYA_NORMAL_1B: TalentScaling = TalentScaling {
    name: "1段ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6838, 0.7304, 0.7770, 0.8392, 0.8858, 0.9402, 1.0101, 1.0800, 1.1500, 1.2199, 1.2898,
        1.3598, 1.4297, 1.4996, 1.5695,
    ],
};

const YOIMIYA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8889, 0.9495, 1.0101, 1.0909, 1.1515, 1.2222, 1.3131, 1.4040, 1.4949, 1.5859, 1.6768,
        1.7677, 1.8586, 1.9495, 2.0404,
    ],
};

const YOIMIYA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4642, 0.4959, 0.5275, 0.5697, 0.6014, 0.6383, 0.6858, 0.7332, 0.7807, 0.8282, 0.8757,
        0.9231, 0.9706, 1.0181, 1.0656,
    ],
};

const YOIMIYA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4642, 0.4959, 0.5275, 0.5697, 0.6014, 0.6383, 0.6858, 0.7332, 0.7807, 0.8282, 0.8757,
        0.9231, 0.9706, 1.0181, 1.0656,
    ],
};

const YOIMIYA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0586, 1.1308, 1.2030, 1.2992, 1.3714, 1.4556, 1.5639, 1.6722, 1.7804, 1.8887, 1.9970,
        2.1053, 2.2135, 2.3218, 2.4301,
    ],
};

// -- Aimed Shot -- Pyro (charged) --

const YOIMIYA_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const YOIMIYA_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

const YOIMIYA_AIMED_KINDLING: TalentScaling = TalentScaling {
    name: "焔硝の矢ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.1640, 0.1763, 0.1886, 0.2050, 0.2173, 0.2296, 0.2460, 0.2624, 0.2788, 0.2952, 0.3116,
        0.3280, 0.3485, 0.3690, 0.3895,
    ],
};

// -- Plunging Attack -- Physical --

const YOIMIYA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const YOIMIYA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const YOIMIYA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 庭火焔硝 (Niwabi Fire-Dance) -- Pyro --
// Note: Skill infuses normal attacks with Pyro. The damage bonus multiplier
// is stored here as a scaling reference. Callers handle the infusion mechanic.

const YOIMIYA_SKILL_DMG: TalentScaling = TalentScaling {
    name: "炎硝矢ダメージ増加",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.3791, 0.4018, 0.4245, 0.4540, 0.4767, 0.4994, 0.5289, 0.5584, 0.5879, 0.6174, 0.6470,
        0.6765, 0.7060, 0.7355, 0.7650,
    ],
};

// -- Elemental Burst: 琉金雲間草 (Ryuukin Saxifrage) -- Pyro --

const YOIMIYA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2720, 1.3674, 1.4628, 1.5900, 1.6854, 1.7808, 1.9080, 2.0352, 2.1624, 2.2896, 2.4168,
        2.5440, 2.7030, 2.8620, 3.0210,
    ],
};

const YOIMIYA_BURST_EXPLOSION: TalentScaling = TalentScaling {
    name: "琉金の炎爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2200, 1.3115, 1.4030, 1.5250, 1.6165, 1.7080, 1.8300, 1.9520, 2.0740, 2.1960, 2.3180,
        2.4400, 2.5925, 2.7450, 2.8975,
    ],
};

pub const YOIMIYA: CharacterData = CharacterData {
    id: "yoimiya",
    name: "Yoimiya",
    element: Element::Pyro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [791.0, 8968.0, 9450.0, 10164.0],
    base_atk: [25.0, 285.0, 300.0, 323.0],
    base_def: [48.0, 542.0, 572.0, 615.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "打ち上げ花火",
            hits: &[
                YOIMIYA_NORMAL_1A,
                YOIMIYA_NORMAL_1B,
                YOIMIYA_NORMAL_2,
                YOIMIYA_NORMAL_3,
                YOIMIYA_NORMAL_4,
                YOIMIYA_NORMAL_5,
            ],
            charged: &[YOIMIYA_AIMED, YOIMIYA_AIMED_FULL, YOIMIYA_AIMED_KINDLING],
            plunging: &[YOIMIYA_PLUNGE, YOIMIYA_PLUNGE_LOW, YOIMIYA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "庭火焔硝",
            scalings: &[YOIMIYA_SKILL_DMG],
        },
        elemental_burst: TalentData {
            name: "琉金雲間草",
            scalings: &[YOIMIYA_BURST, YOIMIYA_BURST_EXPLOSION],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
