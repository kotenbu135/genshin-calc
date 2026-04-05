use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

const DILUC_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8974, 0.9704, 1.0434, 1.1477, 1.2207, 1.3042, 1.4191, 1.5340, 1.6489, 1.7739, 1.8989,
        2.0239, 2.1489, 2.2739, 2.3989,
    ],
    dynamic_bonus: None,
};

const DILUC_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8764, 0.9477, 1.0190, 1.1209, 1.1922, 1.2737, 1.3858, 1.4979, 1.6100, 1.7323, 1.8546,
        1.9769, 2.0992, 2.2215, 2.3438,
    ],
    dynamic_bonus: None,
};

const DILUC_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9882, 1.0687, 1.1493, 1.2642, 1.3447, 1.4366, 1.5628, 1.6891, 1.8153, 1.9535, 2.0917,
        2.2298, 2.3680, 2.5061, 2.6443,
    ],
    dynamic_bonus: None,
};

const DILUC_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.3399, 1.4491, 1.5583, 1.7141, 1.8233, 1.9479, 2.1191, 2.2904, 2.4616, 2.6489, 2.8362,
        3.0235, 3.2108, 3.3981, 3.5854,
    ],
    dynamic_bonus: None,
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
    dynamic_bonus: None,
};

const DILUC_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2470, 1.3485, 1.4500, 1.5950, 1.6965, 1.8125, 1.9720, 2.1315, 2.2910, 2.4650, 2.6390,
        2.8130, 2.9870, 3.1610, 3.3350,
    ],
    dynamic_bonus: None,
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
    dynamic_bonus: None,
};

const DILUC_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.7901, 1.9360, 2.0820, 2.2902, 2.4361, 2.6020, 2.8303, 3.0585, 3.2867, 3.5349, 3.7831,
        4.0313, 4.2795, 4.5277, 4.7759,
    ],
    dynamic_bonus: None,
};

const DILUC_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        2.2362, 2.4184, 2.6007, 2.8608, 3.0430, 3.2503, 3.5355, 3.8206, 4.1057, 4.4159, 4.7261,
        5.0363, 5.3465, 5.6567, 5.9669,
    ],
    dynamic_bonus: None,
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
    dynamic_bonus: None,
};

const DILUC_SKILL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.9760, 1.0492, 1.1224, 1.2200, 1.2932, 1.3664, 1.4640, 1.5616, 1.6592, 1.7568, 1.8544,
        1.9520, 2.0740, 2.1960, 2.3180,
    ],
    dynamic_bonus: None,
};

const DILUC_SKILL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.2880, 1.3846, 1.4812, 1.6100, 1.7066, 1.8032, 1.9320, 2.0608, 2.1896, 2.3184, 2.4472,
        2.5760, 2.7370, 2.8980, 3.0590,
    ],
    dynamic_bonus: None,
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
    dynamic_bonus: None,
};

const DILUC_BURST_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.6000, 0.6450, 0.6900, 0.7500, 0.7950, 0.8400, 0.9000, 0.9600, 1.0200, 1.0800, 1.1400,
        1.2000, 1.2750, 1.3500, 1.4250,
    ],
    dynamic_bonus: None,
};

const DILUC_BURST_EXPLOSION: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        2.0400, 2.1930, 2.3460, 2.5500, 2.7030, 2.8560, 3.0600, 3.2640, 3.4680, 3.6720, 3.8760,
        4.0800, 4.3350, 4.5900, 4.8450,
    ],
    dynamic_bonus: None,
};

// -- Character Data --

pub const DILUC: CharacterData = CharacterData {
    id: "diluc",
    name: "Diluc",
    element: Element::Pyro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [
        1011.0,  // Lv1
        2621.0,  // Lv20
        3488.0,  // Lv20+
        5219.0,  // Lv40
        5834.0,  // Lv40+
        6713.0,  // Lv50
        7533.0,  // Lv50+
        8392.0,  // Lv60
        9036.0,  // Lv60+
        9932.0,  // Lv70
        10547.0, // Lv70+
        11453.0, // Lv80
        12068.0, // Lv80+
        12981.0, // Lv90
        12981.0, // Lv90+
        13500.0, // Lv95 (Lv90*1.04)
        13500.0, // Lv95+
        14019.0, // Lv100 (Lv90*1.08)
    ],
    base_atk: [
        26.07,  // Lv1
        67.62,  // Lv20
        89.97,  // Lv20+
        134.62, // Lv40
        150.5,  // Lv40+
        173.16, // Lv50
        194.33, // Lv50+
        216.22, // Lv60
        233.15, // Lv60+
        256.19, // Lv70
        272.07, // Lv70+
        295.43, // Lv80
        311.31, // Lv80+
        334.85, // Lv90
        334.85, // Lv90+
        348.24, // Lv95 (Lv90*1.04)
        348.24, // Lv95+
        361.64, // Lv100 (Lv90*1.08)
    ],
    base_def: [
        61.03,  // Lv1
        158.3,  // Lv20
        210.63, // Lv20+
        315.17, // Lv40
        352.35, // Lv40+
        405.38, // Lv50
        454.95, // Lv50+
        508.53, // Lv60
        545.71, // Lv60+
        599.78, // Lv70
        636.96, // Lv70+
        691.64, // Lv80
        728.82, // Lv80+
        783.93, // Lv90
        783.93, // Lv90+
        815.29, // Lv95 (Lv90*1.04)
        815.29, // Lv95+
        846.64, // Lv100 (Lv90*1.08)
    ],
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
