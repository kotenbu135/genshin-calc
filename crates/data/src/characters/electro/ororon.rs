use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Ororon
// =============================================================================

// -- Normal Attack: 霊器撮影 (Spiritvessel Snapshot) -- Physical --

const ORORON_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5064, 0.5476, 0.5888, 0.6477, 0.6889, 0.7360, 0.8006, 0.8652, 0.9298, 1.0007, 1.0717,
        1.1426, 1.2136, 1.2845, 1.3554,
    ],
};

const ORORON_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4440, 0.4802, 0.5164, 0.5680, 0.6042, 0.6455, 0.7022, 0.7590, 0.8157, 0.8778, 0.9399,
        1.0020, 1.0637, 1.1253, 1.1870,
    ],
};

const ORORON_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6984, 0.7552, 0.8120, 0.8932, 0.9500, 1.0150, 1.1042, 1.1934, 1.2826, 1.3802, 1.4778,
        1.5754, 1.6730, 1.7706, 1.8682,
    ],
};

// -- Aimed Shot -- Electro (charged) --

const ORORON_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const ORORON_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

// -- Plunging Attack -- Physical --

const ORORON_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const ORORON_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const ORORON_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 夜の投弾 (Night's Sling) -- Electro --

const ORORON_SKILL: TalentScaling = TalentScaling {
    name: "霊の弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.9760, 2.1242, 2.2724, 2.4700, 2.6182, 2.7664, 2.9640, 3.1616, 3.3592, 3.5568, 3.7544,
        3.9520, 4.1990, 4.4460, 4.6930,
    ],
};

// -- Elemental Burst: 闇声の反響 (Dark Voices Echo) -- Electro --

const ORORON_BURST_RITUAL: TalentScaling = TalentScaling {
    name: "儀式ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        1.7438, 1.8746, 2.0054, 2.1798, 2.3106, 2.4414, 2.6158, 2.7901, 2.9645, 3.1389, 3.3133,
        3.4877, 3.7057, 3.9236, 4.1416,
    ],
};

const ORORON_BURST_SOUNDWAVE: TalentScaling = TalentScaling {
    name: "音波衝突ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.3320, 0.3569, 0.3818, 0.4150, 0.4399, 0.4648, 0.4980, 0.5312, 0.5644, 0.5976, 0.6308,
        0.6640, 0.7055, 0.7470, 0.7885,
    ],
};

pub const ORORON: CharacterData = CharacterData {
    id: "ororon",
    name: "Ororon",
    element: Element::Electro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Natlan,
    base_hp: [775.0, 8192.0, 8604.0, 9244.0],
    base_atk: [20.0, 216.0, 227.0, 244.0],
    base_def: [49.0, 520.0, 546.0, 587.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "霊器撮影",
            hits: &[ORORON_NORMAL_1, ORORON_NORMAL_2, ORORON_NORMAL_3],
            charged: &[ORORON_AIMED, ORORON_AIMED_FULL],
            plunging: &[ORORON_PLUNGE, ORORON_PLUNGE_LOW, ORORON_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "夜の投弾",
            scalings: &[ORORON_SKILL],
        },
        elemental_burst: TalentData {
            name: "闇声の反響",
            scalings: &[ORORON_BURST_RITUAL, ORORON_BURST_SOUNDWAVE],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
