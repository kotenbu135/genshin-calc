use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Zhongli
// =============================================================================

// -- Normal Attack: 岩雨 (Rain of Stone) -- Physical --

const ZHONGLI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3077, 0.3328, 0.3578, 0.3936, 0.4186, 0.4473, 0.4866, 0.5260, 0.5653, 0.6083, 0.6513,
        0.6943, 0.7373, 0.7804, 0.8234,
    ],
};

const ZHONGLI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3115, 0.3369, 0.3622, 0.3984, 0.4238, 0.4528, 0.4926, 0.5324, 0.5722, 0.6157, 0.6593,
        0.7028, 0.7463, 0.7898, 0.8334,
    ],
};

const ZHONGLI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3858, 0.4172, 0.4486, 0.4935, 0.5249, 0.5608, 0.6101, 0.6594, 0.7088, 0.7626, 0.8165,
        0.8703, 0.9242, 0.9780, 1.0319,
    ],
};

const ZHONGLI_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4294, 0.4643, 0.4993, 0.5492, 0.5842, 0.6241, 0.6791, 0.7340, 0.7890, 0.8489, 0.9088,
        0.9688, 1.0288, 1.0887, 1.1487,
    ],
};

const ZHONGLI_NORMAL_5_1: TalentScaling = TalentScaling {
    name: "5段ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.1075, 0.1163, 0.1250, 0.1375, 0.1463, 0.1563, 0.1700, 0.1838, 0.1975, 0.2125, 0.2275,
        0.2425, 0.2575, 0.2725, 0.2875,
    ],
};

const ZHONGLI_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5451, 0.5894, 0.6337, 0.6971, 0.7414, 0.7922, 0.8620, 0.9318, 1.0017, 1.0775, 1.1533,
        1.2292, 1.3050, 1.3808, 1.4567,
    ],
};

// -- Charged Attack -- Physical --

const ZHONGLI_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1103, 1.2008, 1.2913, 1.4204, 1.5109, 1.6141, 1.7559, 1.8978, 2.0396, 2.1949, 2.3502,
        2.5055, 2.6608, 2.8161, 2.9714,
    ],
};

// -- Plunging Attack -- Physical --

const ZHONGLI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 0.1011, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const ZHONGLI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5271, 2.7055,
        2.8840, 3.0624, 3.2409, 3.4193,
    ],
};

const ZHONGLI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6021, 3.8249, 4.0478, 4.2706,
    ],
};

// -- Elemental Skill: 地心 (Dominus Lapidis) -- Geo, HP scaling --

const ZHONGLI_SKILL_STONE_STELE: TalentScaling = TalentScaling {
    name: "岩柱ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Geo),
    values: [
        0.1600, 0.1720, 0.1840, 0.2000, 0.2120, 0.2240, 0.2400, 0.2560, 0.2720, 0.2880, 0.3040,
        0.3200, 0.3400, 0.3600, 0.3800,
    ],
};

const ZHONGLI_SKILL_RESONANCE: TalentScaling = TalentScaling {
    name: "共鳴ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Geo),
    values: [
        0.3200, 0.3440, 0.3680, 0.4000, 0.4240, 0.4480, 0.4800, 0.5120, 0.5440, 0.5760, 0.6080,
        0.6400, 0.6800, 0.7200, 0.7600,
    ],
};

const ZHONGLI_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "長押しダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Geo),
    values: [
        0.8000, 0.8600, 0.9200, 1.0000, 1.0600, 1.1200, 1.2000, 1.2800, 1.3600, 1.4400, 1.5200,
        1.6000, 1.7000, 1.8000, 1.9000,
    ],
};

// -- Elemental Burst: 天星 (Planet Befall) -- Geo --

const ZHONGLI_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        4.0108, 4.2528, 4.4948, 4.8010, 5.0430, 5.2850, 5.5912, 5.8974, 6.2036, 6.5098, 6.8160,
        7.1222, 7.5548, 7.9875, 8.4201,
    ],
};

pub const ZHONGLI: CharacterData = CharacterData {
    id: "zhongli",
    name: "Zhongli",
    element: Element::Geo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [1144.0, 11453.0, 12707.0, 14695.0],
    base_atk: [20.0, 200.0, 222.0, 251.0],
    base_def: [57.0, 564.0, 626.0, 738.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Geo, 0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "岩雨",
            hits: &[
                ZHONGLI_NORMAL_1,
                ZHONGLI_NORMAL_2,
                ZHONGLI_NORMAL_3,
                ZHONGLI_NORMAL_4,
                ZHONGLI_NORMAL_5_1,
                ZHONGLI_NORMAL_6,
            ],
            charged: &[ZHONGLI_CHARGED],
            plunging: &[ZHONGLI_PLUNGE, ZHONGLI_PLUNGE_LOW, ZHONGLI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "地心",
            scalings: &[
                ZHONGLI_SKILL_STONE_STELE,
                ZHONGLI_SKILL_RESONANCE,
                ZHONGLI_SKILL_HOLD,
            ],
        },
        elemental_burst: TalentData {
            name: "天星",
            scalings: &[ZHONGLI_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
