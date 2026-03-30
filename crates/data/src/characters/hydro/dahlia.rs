use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 西風弓術・祭儀 -- Physical (Bow) --

const DAHLIA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4350, 0.4710, 0.5060, 0.5670, 0.5920, 0.6330, 0.6890, 0.7440, 0.8000, 0.8610, 0.9220,
        0.9820, 1.0430, 1.1040, 1.1650,
    ],
};

const DAHLIA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4010, 0.4340, 0.4660, 0.5130, 0.5460, 0.5830, 0.6340, 0.6850, 0.7370, 0.7930, 0.8490,
        0.9050, 0.9610, 1.0160, 1.0720,
    ],
};

const DAHLIA_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2370, 0.2570, 0.2760, 0.3040, 0.3230, 0.3450, 0.3750, 0.4060, 0.4360, 0.4690, 0.5030,
        0.5360, 0.5690, 0.6020, 0.6350,
    ],
};

const DAHLIA_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2900, 0.3140, 0.3370, 0.3710, 0.3950, 0.4220, 0.4590, 0.4960, 0.5330, 0.5740, 0.6140,
        0.6550, 0.6950, 0.7360, 0.7760,
    ],
};

const DAHLIA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6570, 0.7100, 0.7630, 0.8400, 0.8930, 0.9540, 1.0380, 1.1220, 1.2060, 1.2980, 1.3890,
        1.4810, 1.5730, 1.6640, 1.7560,
    ],
};

// -- Charged Attack -- Hydro (Bow aimed) --

const DAHLIA_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const DAHLIA_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

// -- Plunging Attack -- Physical --

const DAHLIA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const DAHLIA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const DAHLIA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 受洗の礼典 -- Hydro --

const DAHLIA_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        2.3280, 2.5026, 2.6772, 2.9100, 3.0846, 3.2592, 3.4920, 3.7248, 3.9576, 4.1904, 4.4232,
        4.6560, 4.9470, 5.2380, 5.5290,
    ],
};

// -- Elemental Burst: 純光の祈り -- Hydro --

const DAHLIA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        4.0640, 4.3688, 4.6736, 5.0800, 5.3848, 5.6896, 6.0960, 6.5024, 6.9088, 7.3152, 7.7216,
        8.1280, 8.6360, 9.1440, 9.6520,
    ],
};

pub const DAHLIA: CharacterData = CharacterData {
    id: "dahlia",
    name: "Dahlia",
    element: Element::Hydro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Snezhnaya,
    base_hp: [986.0, 11199.0, 11802.0, 13348.0],
    base_atk: [24.0, 268.0, 283.0, 320.0],
    base_def: [62.0, 700.0, 738.0, 835.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "西風弓術・祭儀",
            hits: &[
                DAHLIA_NORMAL_1,
                DAHLIA_NORMAL_2,
                DAHLIA_NORMAL_3A,
                DAHLIA_NORMAL_3B,
                DAHLIA_NORMAL_4,
            ],
            charged: &[DAHLIA_AIMED, DAHLIA_AIMED_FULL],
            plunging: &[DAHLIA_PLUNGE, DAHLIA_PLUNGE_LOW, DAHLIA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "受洗の礼典",
            scalings: &[DAHLIA_SKILL],
        },
        elemental_burst: TalentData {
            name: "純光の祈り",
            scalings: &[DAHLIA_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
