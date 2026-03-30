use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Beidou
// =============================================================================

// -- Normal Attack: 征霆鑑 (Oceanborne) -- All physical --

const BEIDOU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7112, 0.7692, 0.8272, 0.9099, 0.9679, 1.0340, 1.1249, 1.2158, 1.3067, 1.4061, 1.5055,
        1.6049, 1.7043, 1.8037, 1.9031,
    ],
};

const BEIDOU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7086, 0.7664, 0.8242, 0.9066, 0.9644, 1.0302, 1.1208, 1.2114, 1.3020, 1.4010, 1.5000,
        1.5990, 1.6980, 1.7970, 1.8960,
    ],
};

const BEIDOU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8832, 0.9550, 1.0268, 1.1295, 1.2013, 1.2835, 1.3963, 1.5091, 1.6219, 1.7454, 1.8689,
        1.9924, 2.1159, 2.2394, 2.3629,
    ],
};

const BEIDOU_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8652, 0.9356, 1.0060, 1.1066, 1.1770, 1.2575, 1.3682, 1.4789, 1.5896, 1.7103, 1.8310,
        1.9517, 2.0724, 2.1931, 2.3138,
    ],
};

const BEIDOU_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1214, 1.2126, 1.3038, 1.4342, 1.5254, 1.6298, 1.7733, 1.9168, 2.0603, 2.2166, 2.3729,
        2.5292, 2.6855, 2.8418, 2.9981,
    ],
};

// -- Charged Attack -- Physical --

const BEIDOU_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5624, 0.6082, 0.6540, 0.7194, 0.7652, 0.8175, 0.8894, 0.9614, 1.0333, 1.1118, 1.1903,
        1.2688, 1.3473, 1.4258, 1.5043,
    ],
};

const BEIDOU_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.0182, 1.1012, 1.1842, 1.3026, 1.3856, 1.4800, 1.6102, 1.7404, 1.8706, 2.0128, 2.1550,
        2.2972, 2.4394, 2.5816, 2.7238,
    ],
};

// -- Plunging Attack -- Physical --

const BEIDOU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9541, 1.0148, 1.0841, 1.1795, 1.2749, 1.3702, 1.4744, 1.5786,
        1.6827, 1.7869, 1.8910, 1.9952,
    ],
};

const BEIDOU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9076, 2.0290, 2.1678, 2.3586, 2.5494, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const BEIDOU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5345, 2.7078, 2.9460, 3.1843, 3.4225, 3.6824, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
};

// -- Elemental Skill: 捉浪 (Tidecaller) -- Electro --

const BEIDOU_SKILL_BASE: TalentScaling = TalentScaling {
    name: "基礎ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2160, 1.3072, 1.3984, 1.5200, 1.6112, 1.7024, 1.8240, 1.9456, 2.0672, 2.1888, 2.3104,
        2.4320, 2.5840, 2.7360, 2.8880,
    ],
};

const BEIDOU_SKILL_HIT_BONUS: TalentScaling = TalentScaling {
    name: "受撃時ダメージボーナス",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.6000, 1.7200, 1.8400, 2.0000, 2.1200, 2.2400, 2.4000, 2.5600, 2.7200, 2.8800, 3.0400,
        3.2000, 3.4000, 3.6000, 3.8000,
    ],
};

// -- Elemental Burst: 斫雷 (Stormbreaker) -- Electro --

const BEIDOU_BURST_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2160, 1.3072, 1.3984, 1.5200, 1.6112, 1.7024, 1.8240, 1.9456, 2.0672, 2.1888, 2.3104,
        2.4320, 2.5840, 2.7360, 2.8880,
    ],
};

const BEIDOU_BURST_LIGHTNING: TalentScaling = TalentScaling {
    name: "雷放電ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.9600, 1.0320, 1.1040, 1.2000, 1.2720, 1.3440, 1.4400, 1.5360, 1.6320, 1.7280, 1.8240,
        1.9200, 2.0400, 2.1600, 2.2800,
    ],
};

pub const BEIDOU: CharacterData = CharacterData {
    id: "beidou",
    name: "Beidou",
    element: Element::Electro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [1094.0, 11565.0, 12146.0, 13050.0],
    base_atk: [19.0, 200.0, 210.0, 225.0],
    base_def: [54.0, 575.0, 603.0, 648.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Electro, 0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "征霆鑑",
            hits: &[
                BEIDOU_NORMAL_1,
                BEIDOU_NORMAL_2,
                BEIDOU_NORMAL_3,
                BEIDOU_NORMAL_4,
                BEIDOU_NORMAL_5,
            ],
            charged: &[BEIDOU_CHARGED_SPINNING, BEIDOU_CHARGED_FINAL],
            plunging: &[BEIDOU_PLUNGE, BEIDOU_PLUNGE_LOW, BEIDOU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "捉浪",
            scalings: &[BEIDOU_SKILL_BASE, BEIDOU_SKILL_HIT_BONUS],
        },
        elemental_burst: TalentData {
            name: "斫雷",
            scalings: &[BEIDOU_BURST_SKILL, BEIDOU_BURST_LIGHTNING],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
