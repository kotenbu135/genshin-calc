use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// -- Normal Attack: 水の常形 (The Shape of Water) -- All Hydro (Catalyst) --

const KOKOMI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.6838, 0.7350, 0.7863, 0.8547, 0.9060, 0.9573, 1.0256, 1.0940, 1.1624, 1.2308, 1.2991,
        1.3675, 1.4530, 1.5385, 1.6239,
    ],
};

const KOKOMI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.6154, 0.6615, 0.7077, 0.7692, 0.8154, 0.8615, 0.9231, 0.9846, 1.0462, 1.1077, 1.1692,
        1.2308, 1.3077, 1.3846, 1.4615,
    ],
};

const KOKOMI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.9431, 1.0138, 1.0845, 1.1788, 1.2496, 1.3203, 1.4146, 1.5089, 1.6032, 1.6975, 1.7918,
        1.8861, 2.0040, 2.1219, 2.2398,
    ],
};

// -- Charged Attack -- Hydro (Catalyst) --

const KOKOMI_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4832, 1.5944, 1.7057, 1.8540, 1.9652, 2.0765, 2.2248, 2.3731, 2.5214, 2.6698, 2.8181,
        2.9664, 3.1518, 3.3372, 3.5226,
    ],
};

// -- Plunging Attack -- Hydro (Catalyst) --

const KOKOMI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const KOKOMI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const KOKOMI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 海月の誓い (Kurage's Oath) -- Hydro --

const KOKOMI_SKILL_WAVE: TalentScaling = TalentScaling {
    name: "波紋ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.0919, 1.1738, 1.2557, 1.3649, 1.4468, 1.5287, 1.6379, 1.7471, 1.8562, 1.9654, 2.0746,
        2.1838, 2.3203, 2.4568, 2.5933,
    ],
};

// -- Elemental Burst: 海人の羽衣 (Nereid's Ascension) -- Hydro --

const KOKOMI_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.10416, 0.11197, 0.11978, 0.13020, 0.13801, 0.14582, 0.15624, 0.16666, 0.17707, 0.18749,
        0.19790, 0.20832, 0.22134, 0.23436, 0.24738,
    ],
};

const KOKOMI_BURST_NORMAL_BONUS: TalentScaling = TalentScaling {
    name: "通常攻撃ダメージボーナス",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0484, 0.05203, 0.05566, 0.0605, 0.06413, 0.06776, 0.0726, 0.07744, 0.08228, 0.08712,
        0.09196, 0.0968, 0.10285, 0.10890, 0.11495,
    ],
};

const KOKOMI_BURST_CHARGED_BONUS: TalentScaling = TalentScaling {
    name: "重撃ダメージボーナス",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.06776, 0.07284, 0.07792, 0.0847, 0.08978, 0.09486, 0.10164, 0.10842, 0.11519, 0.12197,
        0.12874, 0.13552, 0.14399, 0.15246, 0.16093,
    ],
};

const KOKOMI_BURST_JELLYFISH_BONUS: TalentScaling = TalentScaling {
    name: "海月ダメージボーナス",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.07096, 0.07629, 0.08161, 0.0887, 0.09403, 0.09935, 0.10645, 0.11354, 0.12064, 0.12773,
        0.13483, 0.14193, 0.15080, 0.15967, 0.16854,
    ],
};

pub const KOKOMI: CharacterData = CharacterData {
    id: "kokomi",
    name: "Kokomi",
    element: Element::Hydro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [1049.0, 11885.0, 12524.0, 13471.0],
    base_atk: [18.0, 203.0, 214.0, 234.0],
    base_def: [51.0, 573.0, 604.0, 657.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Hydro, 0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "水の常形",
            hits: &[KOKOMI_NORMAL_1, KOKOMI_NORMAL_2, KOKOMI_NORMAL_3],
            charged: &[KOKOMI_CHARGED],
            plunging: &[KOKOMI_PLUNGE, KOKOMI_PLUNGE_LOW, KOKOMI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "海月の誓い",
            scalings: &[KOKOMI_SKILL_WAVE],
        },
        elemental_burst: TalentData {
            name: "海人の羽衣",
            scalings: &[
                KOKOMI_BURST,
                KOKOMI_BURST_NORMAL_BONUS,
                KOKOMI_BURST_CHARGED_BONUS,
                KOKOMI_BURST_JELLYFISH_BONUS,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
