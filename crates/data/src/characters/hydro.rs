#![allow(clippy::approx_constant)]
use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Ayato
// =============================================================================

// -- Normal Attack: 神里流・転 (Kamisato Art: Marobashi) -- Physical --

const AYATO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4496, 0.4862, 0.5228, 0.5751, 0.6117, 0.6535, 0.7110, 0.7685, 0.8260, 0.8888, 0.9515,
        1.0143, 1.0770, 1.1397, 1.2025,
    ],
};

const AYATO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4716, 0.5099, 0.5483, 0.6032, 0.6416, 0.6854, 0.7457, 0.8061, 0.8664, 0.9322, 0.9980,
        1.0638, 1.1296, 1.1954, 1.2612,
    ],
};

const AYATO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5861, 0.6338, 0.6815, 0.7497, 0.7974, 0.8519, 0.9269, 1.0019, 1.0768, 1.1586, 1.2404,
        1.3222, 1.4040, 1.4858, 1.5675,
    ],
};

const AYATO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ(×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2945, 0.3185, 0.3424, 0.3767, 0.4006, 0.4280, 0.4657, 0.5034, 0.5410, 0.5821, 0.6232,
        0.6643, 0.7054, 0.7465, 0.7876,
    ],
};

const AYATO_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7560, 0.8176, 0.8791, 0.9670, 1.0286, 1.0989, 1.1956, 1.2923, 1.3890, 1.4945, 1.6000,
        1.7055, 1.8110, 1.9165, 2.0220,
    ],
};

// -- Charged Attack -- Physical --

const AYATO_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2953, 1.4007, 1.5062, 1.6568, 1.7622, 1.8827, 2.0484, 2.2141, 2.3797, 2.5605, 2.7412,
        2.9219, 3.1027, 3.2834, 3.4642,
    ],
};

// -- Plunging Attack -- Physical --

const AYATO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const AYATO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const AYATO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 神里流·鏡花 (Kamisato Art: Kyouka) -- Hydro --

const AYATO_SKILL_1HIT: TalentScaling = TalentScaling {
    name: "瞬水剣1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5289, 0.5719, 0.6150, 0.6765, 0.7195, 0.7687, 0.8364, 0.9040, 0.9717, 1.0455, 1.1193,
        1.1931, 1.2669, 1.3407, 1.4145,
    ],
};

const AYATO_SKILL_2HIT: TalentScaling = TalentScaling {
    name: "瞬水剣2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5891, 0.6371, 0.6850, 0.7535, 0.8015, 0.8562, 0.9316, 1.0070, 1.0823, 1.1645, 1.2467,
        1.3289, 1.4111, 1.4933, 1.5755,
    ],
};

const AYATO_SKILL_3HIT: TalentScaling = TalentScaling {
    name: "瞬水剣3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.6493, 0.7022, 0.7550, 0.8305, 0.8834, 0.9437, 1.0268, 1.0999, 1.1929, 1.2835, 1.3741,
        1.4647, 1.5553, 1.6459, 1.7365,
    ],
};

const AYATO_SKILL_ILLUSION: TalentScaling = TalentScaling {
    name: "水の幻影ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.0148, 1.0974, 1.1800, 1.2980, 1.3806, 1.4750, 1.6048, 1.7346, 1.8644, 2.0060, 2.1476,
        2.2892, 2.4308, 2.5724, 2.7140,
    ],
};

// -- Elemental Burst: 神里流·水囿 (Kamisato Art: Suiyuu) -- Hydro --

const AYATO_BURST: TalentScaling = TalentScaling {
    name: "水沫剣ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.6646, 0.7144, 0.7642, 0.8307, 0.8805, 0.9304, 0.9968, 1.0633, 1.1298, 1.1962, 1.2626,
        1.3291, 1.4122, 1.4953, 1.5783,
    ],
};

pub const AYATO: CharacterData = CharacterData {
    id: "ayato",
    name: "Ayato",
    element: Element::Hydro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [1068.0, 11106.0, 11705.0, 13715.0],
    base_atk: [23.0, 243.0, 256.0, 299.0],
    base_def: [60.0, 628.0, 662.0, 769.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "神里流・転",
            hits: &[
                AYATO_NORMAL_1,
                AYATO_NORMAL_2,
                AYATO_NORMAL_3,
                AYATO_NORMAL_4,
                AYATO_NORMAL_5,
            ],
            charged: &[AYATO_CHARGED],
            plunging: &[AYATO_PLUNGE, AYATO_PLUNGE_LOW, AYATO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "神里流·鏡花",
            scalings: &[
                AYATO_SKILL_1HIT,
                AYATO_SKILL_2HIT,
                AYATO_SKILL_3HIT,
                AYATO_SKILL_ILLUSION,
            ],
        },
        elemental_burst: TalentData {
            name: "神里流·水囿",
            scalings: &[AYATO_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Barbara
// =============================================================================

// -- Normal Attack: 水の囁き (Whisper of Water) -- All Hydro (Catalyst) --

const BARBARA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3784, 0.4068, 0.4352, 0.4730, 0.5014, 0.5298, 0.5676, 0.6054, 0.6433, 0.6811, 0.7205,
        0.7719, 0.8234, 0.8749, 0.9263,
    ],
};

const BARBARA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3552, 0.3818, 0.4085, 0.4440, 0.4706, 0.4973, 0.5328, 0.5683, 0.6038, 0.6394, 0.6763,
        0.7246, 0.7729, 0.8212, 0.8695,
    ],
};

const BARBARA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.4104, 0.4412, 0.4720, 0.5130, 0.5438, 0.5746, 0.6156, 0.6566, 0.6977, 0.7387, 0.7814,
        0.8372, 0.8930, 0.9488, 1.0047,
    ],
};

const BARBARA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5520, 0.5934, 0.6348, 0.6900, 0.7314, 0.7728, 0.8280, 0.8832, 0.9384, 0.9936, 1.0510,
        1.1261, 1.2012, 1.2762, 1.3513,
    ],
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
};

const BARBARA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const BARBARA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
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
};

pub const BARBARA: CharacterData = CharacterData {
    id: "barbara",
    name: "Barbara",
    element: Element::Hydro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    region: Region::Mondstadt,
    base_hp: [821.0, 8676.0, 9110.0, 9787.0],
    base_atk: [13.0, 142.0, 149.0, 159.0],
    base_def: [56.0, 593.0, 623.0, 669.0],
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
};

// =============================================================================
// Candace
// =============================================================================

// -- Normal Attack: 流耀槍術・守勢 (Gleaming Spear - Guardian Stance) -- Physical --

const CANDACE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6080, 0.6580, 0.7070, 0.7780, 0.8270, 0.8840, 0.9620, 1.0390, 1.1170, 1.2020, 1.2870,
        1.3720, 1.4560, 1.5410, 1.6260,
    ],
};

const CANDACE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6110, 0.6610, 0.7110, 0.7820, 0.8320, 0.8890, 0.9700, 1.0450, 1.1230, 1.2090, 1.2940,
        1.3790, 1.4650, 1.5500, 1.6350,
    ],
};

const CANDACE_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3550, 0.3840, 0.4130, 0.4540, 0.4830, 0.5160, 0.5610, 0.6070, 0.6520, 0.7020, 0.7510,
        0.8010, 0.8500, 0.9000, 0.9490,
    ],
};

const CANDACE_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4340, 0.4690, 0.5040, 0.5550, 0.5900, 0.6300, 0.6860, 0.7410, 0.7970, 0.8570, 0.9180,
        0.9780, 1.0390, 1.0990, 1.1600,
    ],
};

const CANDACE_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9490, 1.0270, 1.1040, 1.2140, 1.2920, 1.3800, 1.5010, 1.6230, 1.7440, 1.8770, 2.0090,
        2.1420, 2.2740, 2.4070, 2.5390,
    ],
};

// -- Charged Attack -- Physical --

const CANDACE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2420, 1.3430, 1.4440, 1.5880, 1.6890, 1.8050, 1.9640, 2.1230, 2.2820, 2.4550, 2.6280,
        2.8010, 2.9750, 3.1480, 3.3210,
    ],
};

// -- Plunging Attack -- Physical --

const CANDACE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const CANDACE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const CANDACE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 聖儀·蒼鷺による庇護 (Sacred Rite: Heron's Sanctum) -- Hydro --

const CANDACE_SKILL_PRESS: TalentScaling = TalentScaling {
    name: "一段チャージダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1200, 0.1200, 0.1200, 0.1200, 0.1200, 0.1200, 0.1200, 0.1200, 0.1200, 0.1200, 0.1200,
        0.1200, 0.1200, 0.1200, 0.1200,
    ],
};

const CANDACE_SKILL_HOLD: TalentScaling = TalentScaling {
    name: "二段チャージダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1904, 0.2047, 0.2190, 0.2380, 0.2523, 0.2666, 0.2856, 0.3046, 0.3237, 0.3427, 0.3618,
        0.3808, 0.4046, 0.4284, 0.4522,
    ],
};

// -- Elemental Burst: 聖儀·灰鴒の呼び潮 (Sacred Rite: Wagtail's Tide) -- Hydro --

const CANDACE_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0661, 0.0711, 0.0760, 0.0826, 0.0876, 0.0925, 0.0992, 0.1058, 0.1124, 0.1190, 0.1256,
        0.1322, 0.1405, 0.1487, 0.1570,
    ],
};

const CANDACE_BURST_WAVE: TalentScaling = TalentScaling {
    name: "波衝撃ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0661, 0.0711, 0.0760, 0.0826, 0.0876, 0.0925, 0.0992, 0.1058, 0.1124, 0.1190, 0.1256,
        0.1322, 0.1405, 0.1487, 0.1570,
    ],
};

pub const CANDACE: CharacterData = CharacterData {
    id: "candace",
    name: "Candace",
    element: Element::Hydro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Sumeru,
    base_hp: [912.0, 9642.0, 10129.0, 10875.0],
    base_atk: [18.0, 186.0, 196.0, 210.0],
    base_def: [57.0, 607.0, 638.0, 683.0],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "流耀槍術・守勢",
            hits: &[
                CANDACE_NORMAL_1,
                CANDACE_NORMAL_2,
                CANDACE_NORMAL_3A,
                CANDACE_NORMAL_3B,
                CANDACE_NORMAL_4,
            ],
            charged: &[CANDACE_CHARGED],
            plunging: &[CANDACE_PLUNGE, CANDACE_PLUNGE_LOW, CANDACE_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "聖儀·蒼鷺による庇護",
            scalings: &[CANDACE_SKILL_PRESS, CANDACE_SKILL_HOLD],
        },
        elemental_burst: TalentData {
            name: "聖儀·灰鴒の呼び潮",
            scalings: &[CANDACE_BURST, CANDACE_BURST_WAVE],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Dahlia
// =============================================================================

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

// =============================================================================
// Furina
// =============================================================================

// -- Normal Attack: 独舞の招待 (Soloist's Solicitation) -- Physical (Sword) --

const FURINA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4839, 0.5232, 0.5626, 0.6189, 0.6583, 0.7033, 0.7652, 0.8271, 0.8890, 0.9565, 1.0240,
        1.0915, 1.1590, 1.2265, 1.2940,
    ],
};

const FURINA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4373, 0.4729, 0.5085, 0.5593, 0.5949, 0.6356, 0.6915, 0.7475, 0.8034, 0.8644, 0.9254,
        0.9865, 1.0475, 1.1085, 1.1695,
    ],
};

const FURINA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5512, 0.5961, 0.6409, 0.7050, 0.7499, 0.8012, 0.8717, 0.9422, 1.0127, 1.0896, 1.1665,
        1.2434, 1.3203, 1.3972, 1.4741,
    ],
};

const FURINA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7330, 0.7926, 0.8523, 0.9375, 0.9972, 1.0654, 1.1591, 1.2529, 1.3466, 1.4489, 1.5512,
        1.6535, 1.7557, 1.8580, 1.9603,
    ],
};

// -- Charged Attack -- Physical (Sword) --

const FURINA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7422, 0.8026, 0.8630, 0.9493, 1.0097, 1.0788, 1.1737, 1.2686, 1.3635, 1.4671, 1.5707,
        1.6742, 1.7778, 1.8813, 1.9849,
    ],
};

// -- Plunging Attack -- Physical (Sword) --

const FURINA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const FURINA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const FURINA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: サロン・ソリティア (Salon Solitaire) -- Hydro --

const FURINA_SKILL_BUBBLE: TalentScaling = TalentScaling {
    name: "ウーシア泡沫ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0786, 0.0845, 0.0904, 0.0983, 0.1042, 0.1101, 0.1180, 0.1258, 0.1337, 0.1416, 0.1494,
        0.1573, 0.1671, 0.1769, 0.1868,
    ],
};

const FURINA_SKILL_USHER: TalentScaling = TalentScaling {
    name: "紳士のヒステリックダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0596, 0.0641, 0.0685, 0.0745, 0.0790, 0.0834, 0.0894, 0.0954, 0.1013, 0.1073, 0.1132,
        0.1192, 0.1267, 0.1341, 0.1416,
    ],
};

const FURINA_SKILL_CHEVALMARIN: TalentScaling = TalentScaling {
    name: "騎士のシュヴァルマランダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0323, 0.0347, 0.0372, 0.0404, 0.0428, 0.0452, 0.0485, 0.0517, 0.0549, 0.0582, 0.0614,
        0.0646, 0.0687, 0.0727, 0.0768,
    ],
};

const FURINA_SKILL_CRABALETTA: TalentScaling = TalentScaling {
    name: "マドモワゼルクラバレッタダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0829, 0.0891, 0.0953, 0.1036, 0.1098, 0.1160, 0.1243, 0.1326, 0.1409, 0.1492, 0.1575,
        0.1658, 0.1761, 0.1865, 0.1968,
    ],
};

// -- Elemental Burst: 万民のカーニバル (Let the People Rejoice) -- Hydro --

const FURINA_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1141, 0.1226, 0.1312, 0.1426, 0.1511, 0.1597, 0.1711, 0.1825, 0.1939, 0.2053, 0.2167,
        0.2281, 0.2424, 0.2566, 0.2709,
    ],
};

pub const FURINA: CharacterData = CharacterData {
    id: "furina",
    name: "Furina",
    element: Element::Hydro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [1192.0, 13471.0, 14198.0, 15307.0],
    base_atk: [19.0, 210.0, 221.0, 244.0],
    base_def: [54.0, 615.0, 648.0, 696.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "独舞の招待",
            hits: &[
                FURINA_NORMAL_1,
                FURINA_NORMAL_2,
                FURINA_NORMAL_3,
                FURINA_NORMAL_4,
            ],
            charged: &[FURINA_CHARGED],
            plunging: &[FURINA_PLUNGE, FURINA_PLUNGE_LOW, FURINA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "サロン・ソリティア",
            scalings: &[
                FURINA_SKILL_BUBBLE,
                FURINA_SKILL_USHER,
                FURINA_SKILL_CHEVALMARIN,
                FURINA_SKILL_CRABALETTA,
            ],
        },
        elemental_burst: TalentData {
            name: "万民のカーニバル",
            scalings: &[FURINA_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Kokomi
// =============================================================================

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

// =============================================================================
// Mona
// =============================================================================

// -- Normal Attack: 因果点破 (Ripple of Fate) -- All Hydro (Catalyst) --

const MONA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3760, 0.4042, 0.4324, 0.4700, 0.4982, 0.5264, 0.5640, 0.6016, 0.6392, 0.6768, 0.7144,
        0.7520, 0.7990, 0.8460, 0.8930,
    ],
};

const MONA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3600, 0.3870, 0.4140, 0.4500, 0.4770, 0.5040, 0.5400, 0.5760, 0.6120, 0.6480, 0.6840,
        0.7200, 0.7650, 0.8100, 0.8550,
    ],
};

const MONA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.4480, 0.4816, 0.5152, 0.5600, 0.5936, 0.6272, 0.6720, 0.7168, 0.7616, 0.8064, 0.8512,
        0.8960, 0.9520, 1.0080, 1.0640,
    ],
};

const MONA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5616, 0.6037, 0.6458, 0.7020, 0.7441, 0.7862, 0.8424, 0.8986, 0.9547, 1.0109, 1.0670,
        1.1232, 1.1934, 1.2636, 1.3338,
    ],
};

// -- Charged Attack -- Hydro (Catalyst) --

const MONA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4972, 1.6095, 1.7218, 1.8715, 1.9838, 2.0961, 2.2458, 2.3955, 2.5452, 2.6950, 2.8507,
        3.0543, 3.2579, 3.4615, 3.6651,
    ],
};

// -- Plunging Attack -- Hydro (Catalyst) --

const MONA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const MONA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const MONA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 水中幻願 (Reflection of Doom) -- Hydro --

const MONA_SKILL_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3200, 0.3440, 0.3680, 0.4000, 0.4240, 0.4480, 0.4800, 0.5120, 0.5440, 0.5760, 0.6080,
        0.6400, 0.6800, 0.7200, 0.7600,
    ],
};

const MONA_SKILL_EXPLOSION: TalentScaling = TalentScaling {
    name: "爆発ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.3280, 1.4276, 1.5272, 1.6600, 1.7596, 1.8592, 1.9920, 2.1248, 2.2576, 2.3904, 2.5232,
        2.6560, 2.8220, 2.9880, 3.1540,
    ],
};

// -- Elemental Burst: 星命定軌 (Stellaris Phantasm) -- Hydro --

const MONA_BURST_BUBBLE: TalentScaling = TalentScaling {
    name: "泡影破裂ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        4.4240, 4.7558, 5.0876, 5.5300, 5.8618, 6.1936, 6.6360, 7.0784, 7.5208, 7.9632, 8.4056,
        8.8480, 9.4010, 9.9540, 10.5070,
    ],
};

pub const MONA: CharacterData = CharacterData {
    id: "mona",
    name: "Mona",
    element: Element::Hydro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [810.0, 9184.0, 9677.0, 10409.0],
    base_atk: [22.0, 253.0, 267.0, 287.0],
    base_def: [51.0, 573.0, 604.0, 653.0],
    ascension_stat: AscensionStat::EnergyRecharge(0.32),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "因果点破",
            hits: &[MONA_NORMAL_1, MONA_NORMAL_2, MONA_NORMAL_3, MONA_NORMAL_4],
            charged: &[MONA_CHARGED],
            plunging: &[MONA_PLUNGE, MONA_PLUNGE_LOW, MONA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "水中幻願",
            scalings: &[MONA_SKILL_DOT, MONA_SKILL_EXPLOSION],
        },
        elemental_burst: TalentData {
            name: "星命定軌",
            scalings: &[MONA_BURST_BUBBLE],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Mualani
// =============================================================================

// -- Normal Attack: 降温処理 (Cooling Treatment) -- All Hydro (Catalyst) --

const MUALANI_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5140, 0.5530, 0.5910, 0.6420, 0.6810, 0.7200, 0.7710, 0.8220, 0.8740, 0.9250, 0.9770,
        1.0280, 1.0920, 1.1560, 1.2210,
    ],
};

const MUALANI_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.4460, 0.4800, 0.5130, 0.5580, 0.5910, 0.6250, 0.6690, 0.7140, 0.7590, 0.8030, 0.8480,
        0.8930, 0.9480, 1.0040, 1.0600,
    ],
};

const MUALANI_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.7000, 0.7530, 0.8050, 0.8750, 0.9280, 0.9800, 1.0510, 1.1210, 1.1910, 1.2610, 1.3310,
        1.4010, 1.4880, 1.5760, 1.6630,
    ],
};

// -- Charged Attack -- Hydro (Catalyst) --

const MUALANI_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4288, 1.5360, 1.6431, 1.7860, 1.8932, 2.0003, 2.1432, 2.2861, 2.4290, 2.5718, 2.7147,
        2.8576, 3.0362, 3.2148, 3.3934,
    ],
};

// -- Plunging Attack -- Hydro (Catalyst) --

const MUALANI_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const MUALANI_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const MUALANI_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 鯊鯊衝浪 (Surfshark Wavebreaker) -- Hydro (HP scaling) --

const MUALANI_SKILL_BITE: TalentScaling = TalentScaling {
    name: "鯊鯊バイトダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0868, 0.0933, 0.0998, 0.1085, 0.1150, 0.1215, 0.1302, 0.1389, 0.1476, 0.1562, 0.1649,
        0.1736, 0.1845, 0.1953, 0.2062,
    ],
};

const MUALANI_SKILL_STACK: TalentScaling = TalentScaling {
    name: "波乗りチャージ/層",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0434, 0.0467, 0.0499, 0.0543, 0.0575, 0.0608, 0.0651, 0.0694, 0.0738, 0.0781, 0.0825,
        0.0868, 0.0922, 0.0977, 0.1031,
    ],
};

const MUALANI_SKILL_BIG_WAVE: TalentScaling = TalentScaling {
    name: "巨浪追加ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.2170, 0.2333, 0.2496, 0.2713, 0.2875, 0.3038, 0.3255, 0.3472, 0.3689, 0.3906, 0.4123,
        0.4340, 0.4611, 0.4883, 0.5154,
    ],
};

// -- Elemental Burst: 爆瀑飛弾 (Boomsharka-laka) -- Hydro (HP scaling) --

const MUALANI_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.5844, 0.6282, 0.6721, 0.7305, 0.7743, 0.8181, 0.8766, 0.9350, 0.9935, 1.0519, 1.1103,
        1.1688, 1.2418, 1.3149, 1.3879,
    ],
};

pub const MUALANI: CharacterData = CharacterData {
    id: "mualani",
    name: "Mualani",
    element: Element::Hydro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [1182.0, 13397.0, 14117.0, 15185.0],
    base_atk: [14.0, 159.0, 168.0, 181.0],
    base_def: [44.0, 500.0, 527.0, 567.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "降温処理",
            hits: &[MUALANI_NORMAL_1, MUALANI_NORMAL_2, MUALANI_NORMAL_3],
            charged: &[MUALANI_CHARGED],
            plunging: &[MUALANI_PLUNGE, MUALANI_PLUNGE_LOW, MUALANI_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "鯊鯊衝浪",
            scalings: &[
                MUALANI_SKILL_BITE,
                MUALANI_SKILL_STACK,
                MUALANI_SKILL_BIG_WAVE,
            ],
        },
        elemental_burst: TalentData {
            name: "爆瀑飛弾",
            scalings: &[MUALANI_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Neuvillette
// =============================================================================

// -- Normal Attack: 如水カノン (As Water Seeks Equilibrium) -- All Hydro (Catalyst) --

const NEUVILLETTE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5458, 0.5867, 0.6276, 0.6822, 0.7231, 0.7641, 0.8187, 0.8732, 0.9278, 0.9824, 1.0370,
        1.0915, 1.1598, 1.2280, 1.2962,
    ],
};

const NEUVILLETTE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.4625, 0.4971, 0.5318, 0.5781, 0.6128, 0.6474, 0.6937, 0.7399, 0.7862, 0.8324, 0.8787,
        0.9249, 0.9827, 1.0405, 1.0983,
    ],
};

const NEUVILLETTE_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.7234, 0.7776, 0.8319, 0.9042, 0.9585, 1.0127, 1.0851, 1.1574, 1.2297, 1.3021, 1.3744,
        1.4468, 1.5372, 1.6276, 1.7180,
    ],
};

// -- Charged Attack -- Hydro (Catalyst) --

const NEUVILLETTE_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.3680, 1.4706, 1.5732, 1.7100, 1.8126, 1.9152, 2.0520, 2.1888, 2.3256, 2.4624, 2.5992,
        2.7360, 2.9070, 3.0780, 3.2490,
    ],
};

// -- Charged Attack: 衡平な裁量 (Equitable Judgment) -- Hydro (HP scaling) --

const NEUVILLETTE_CHARGED_JUDGMENT: TalentScaling = TalentScaling {
    name: "衡平な裁量ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0732, 0.0791, 0.0851, 0.0936, 0.0996, 0.1064, 0.1157, 0.1251, 0.1345, 0.1447, 0.1549,
        0.1651, 0.1753, 0.1855, 0.1957,
    ],
};

// -- Plunging Attack -- Hydro (Catalyst) --

const NEUVILLETTE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const NEUVILLETTE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const NEUVILLETTE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 遺したき裁き (O Tears, I Shall Repay) -- Hydro --

const NEUVILLETTE_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1286, 0.1383, 0.1479, 0.1608, 0.1704, 0.1801, 0.1930, 0.2058, 0.2187, 0.2316, 0.2444,
        0.2573, 0.2734, 0.2894, 0.3055,
    ],
};

const NEUVILLETTE_SKILL_THORN: TalentScaling = TalentScaling {
    name: "霊息の棘ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.2080, 0.2236, 0.2392, 0.2600, 0.2756, 0.2912, 0.3120, 0.3328, 0.3536, 0.3744, 0.3952,
        0.4160, 0.4420, 0.4680, 0.4940,
    ],
};

// -- Elemental Burst: 潮よ、我に懲罰を委ねよ (O Tides, I Have Returned) -- Hydro --

const NEUVILLETTE_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.2226, 0.2393, 0.2560, 0.2782, 0.2949, 0.3116, 0.3339, 0.3561, 0.3784, 0.4006, 0.4229,
        0.4452, 0.4730, 0.5008, 0.5286,
    ],
};

const NEUVILLETTE_BURST_WATERFALL: TalentScaling = TalentScaling {
    name: "滝ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0911, 0.0979, 0.1047, 0.1138, 0.1206, 0.1275, 0.1366, 0.1457, 0.1548, 0.1639, 0.1730,
        0.1821, 0.1935, 0.2049, 0.2163,
    ],
};

pub const NEUVILLETTE: CharacterData = CharacterData {
    id: "neuvillette",
    name: "Neuvillette",
    element: Element::Hydro,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [1144.0, 12956.0, 13655.0, 14695.0],
    base_atk: [16.0, 183.0, 193.0, 208.0],
    base_def: [45.0, 509.0, 537.0, 577.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "如水カノン",
            hits: &[
                NEUVILLETTE_NORMAL_1,
                NEUVILLETTE_NORMAL_2,
                NEUVILLETTE_NORMAL_3,
            ],
            charged: &[NEUVILLETTE_CHARGED, NEUVILLETTE_CHARGED_JUDGMENT],
            plunging: &[
                NEUVILLETTE_PLUNGE,
                NEUVILLETTE_PLUNGE_LOW,
                NEUVILLETTE_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "遺したき裁き",
            scalings: &[NEUVILLETTE_SKILL, NEUVILLETTE_SKILL_THORN],
        },
        elemental_burst: TalentData {
            name: "潮よ、我に懲罰を委ねよ",
            scalings: &[NEUVILLETTE_BURST, NEUVILLETTE_BURST_WATERFALL],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Nilou
// =============================================================================

// -- Normal Attack: 弦月のダンス (Dance of Samser) -- Physical (Sword) --

const NILOU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5031, 0.5441, 0.5850, 0.6435, 0.6845, 0.7313, 0.7956, 0.8599, 0.9242, 0.9945, 1.0648,
        1.1351, 1.2054, 1.2757, 1.3460,
    ],
};

const NILOU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4544, 0.4914, 0.5284, 0.5812, 0.6182, 0.6605, 0.7189, 0.7773, 0.8356, 0.8986, 0.9616,
        1.0246, 1.0876, 1.1506, 1.2136,
    ],
};

const NILOU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7035, 0.7608, 0.8182, 0.9000, 0.9574, 1.0228, 1.1127, 1.2026, 1.2925, 1.3909, 1.4893,
        1.5876, 1.6860, 1.7844, 1.8828,
    ],
};

// -- Charged Attack -- Physical (Sword) --

const NILOU_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5022, 0.5431, 0.5840, 0.6424, 0.6833, 0.7300, 0.7942, 0.8585, 0.9227, 0.9928, 1.0630,
        1.1331, 1.2032, 1.2734, 1.3435,
    ],
};

const NILOU_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5444, 0.5887, 0.6330, 0.6963, 0.7406, 0.7913, 0.8608, 0.9304, 1.0000, 1.0760, 1.1520,
        1.2280, 1.3040, 1.3800, 1.4560,
    ],
};

// -- Plunging Attack -- Physical (Sword) --

const NILOU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const NILOU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const NILOU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 七域のダンス (Dance of Haftkarsvar) -- Hydro (HP scaling) --

const NILOU_SKILL_STEP: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0334, 0.0359, 0.0384, 0.0417, 0.0442, 0.0467, 0.0501, 0.0534, 0.0568, 0.0601, 0.0634,
        0.0668, 0.0710, 0.0751, 0.0793,
    ],
};

const NILOU_SKILL_SWORD_1: TalentScaling = TalentScaling {
    name: "剣舞ステップ1ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0455, 0.0489, 0.0524, 0.0569, 0.0603, 0.0637, 0.0683, 0.0728, 0.0774, 0.0819, 0.0865,
        0.0911, 0.0967, 0.1024, 0.1081,
    ],
};

const NILOU_SKILL_SWORD_2: TalentScaling = TalentScaling {
    name: "剣舞ステップ2ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0514, 0.0553, 0.0592, 0.0643, 0.0682, 0.0720, 0.0772, 0.0823, 0.0875, 0.0926, 0.0977,
        0.1029, 0.1093, 0.1158, 0.1222,
    ],
};

const NILOU_SKILL_WATER_WHEEL: TalentScaling = TalentScaling {
    name: "水月ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0717, 0.0771, 0.0824, 0.0896, 0.0950, 0.1004, 0.1075, 0.1147, 0.1219, 0.1290, 0.1362,
        0.1434, 0.1523, 0.1613, 0.1703,
    ],
};

// -- Elemental Burst: 浮蓮のダンス (Dance of Abzendegi: Distant Dreams, Listening Spring) -- Hydro --

const NILOU_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1843, 0.1981, 0.2120, 0.2304, 0.2442, 0.2580, 0.2765, 0.2949, 0.3133, 0.3318, 0.3502,
        0.3686, 0.3917, 0.4147, 0.4378,
    ],
};

const NILOU_BURST_LINGERING: TalentScaling = TalentScaling {
    name: "永続ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.2253, 0.2422, 0.2591, 0.2816, 0.2985, 0.3154, 0.3379, 0.3604, 0.3830, 0.4055, 0.4280,
        0.4506, 0.4787, 0.5069, 0.5350,
    ],
};

pub const NILOU: CharacterData = CharacterData {
    id: "nilou",
    name: "Nilou",
    element: Element::Hydro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Sumeru,
    base_hp: [1182.0, 13397.0, 14117.0, 15185.0],
    base_atk: [18.0, 204.0, 215.0, 230.0],
    base_def: [57.0, 642.0, 677.0, 729.0],
    ascension_stat: AscensionStat::Hp(0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "弦月のダンス",
            hits: &[NILOU_NORMAL_1, NILOU_NORMAL_2, NILOU_NORMAL_3],
            charged: &[NILOU_CHARGED_1, NILOU_CHARGED_2],
            plunging: &[NILOU_PLUNGE, NILOU_PLUNGE_LOW, NILOU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "七域のダンス",
            scalings: &[
                NILOU_SKILL_STEP,
                NILOU_SKILL_SWORD_1,
                NILOU_SKILL_SWORD_2,
                NILOU_SKILL_WATER_WHEEL,
            ],
        },
        elemental_burst: TalentData {
            name: "浮蓮のダンス",
            scalings: &[NILOU_BURST, NILOU_BURST_LINGERING],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Sigewinne
// =============================================================================

// -- Normal Attack: ターゲットセラピー (Targeted Treatment) -- Physical (Bow) --

const SIGEWINNE_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5260, 0.5700, 0.6120, 0.6730, 0.7160, 0.7650, 0.8320, 0.8990, 0.9670, 1.0400, 1.1130,
        1.1870, 1.2600, 1.3340, 1.4070,
    ],
};

const SIGEWINNE_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5110, 0.5520, 0.5940, 0.6530, 0.6950, 0.7420, 0.8080, 0.8730, 0.9380, 1.0090, 1.0810,
        1.1520, 1.2230, 1.2950, 1.3660,
    ],
};

const SIGEWINNE_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7830, 0.8470, 0.9100, 1.0010, 1.0650, 1.1400, 1.2380, 1.3380, 1.4380, 1.5480, 1.6570,
        1.7660, 1.8750, 1.9850, 2.0940,
    ],
};

// -- Charged Attack -- Bow --

const SIGEWINNE_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const SIGEWINNE_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

const SIGEWINNE_MINI_BUBBLE: TalentScaling = TalentScaling {
    name: "ミニ泡沫弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.2480, 0.2670, 0.2850, 0.3100, 0.3290, 0.3470, 0.3720, 0.3970, 0.4220, 0.4460, 0.4710,
        0.4960, 0.5270, 0.5580, 0.5890,
    ],
};

// -- Plunging Attack -- Physical --

const SIGEWINNE_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const SIGEWINNE_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const SIGEWINNE_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: ぴょんぴょん水療法 (Bouncy Hydro Therapy) -- Hydro (HP scaling) --

const SIGEWINNE_SKILL_BUBBLE: TalentScaling = TalentScaling {
    name: "バウンドバブルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0228, 0.0245, 0.0262, 0.0285, 0.0302, 0.0319, 0.0342, 0.0365, 0.0388, 0.0410, 0.0433,
        0.0456, 0.0485, 0.0513, 0.0542,
    ],
};

const SIGEWINNE_SKILL_BLADE: TalentScaling = TalentScaling {
    name: "飛散する水刃ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.00684, 0.00735, 0.00787, 0.00855, 0.00906, 0.00958, 0.01026, 0.01094, 0.01163, 0.01231,
        0.01300, 0.01368, 0.01454, 0.01539, 0.01625,
    ],
};

// -- Elemental Burst: 過飽和心優し注射 (Super Saturated Syringing) -- Hydro (HP scaling) --

const SIGEWINNE_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1180, 0.1270, 0.1350, 0.1470, 0.1560, 0.1650, 0.1770, 0.1880, 0.2000, 0.2120, 0.2240,
        0.2350, 0.2500, 0.2650, 0.2800,
    ],
};

pub const SIGEWINNE: CharacterData = CharacterData {
    id: "sigewinne",
    name: "Sigewinne",
    element: Element::Hydro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [1039.0, 11761.0, 12397.0, 13348.0],
    base_atk: [15.0, 165.0, 174.0, 188.0],
    base_def: [39.0, 445.0, 469.0, 505.0],
    ascension_stat: AscensionStat::Hp(0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "ターゲットセラピー",
            hits: &[SIGEWINNE_NORMAL_1, SIGEWINNE_NORMAL_2, SIGEWINNE_NORMAL_3],
            charged: &[SIGEWINNE_AIMED, SIGEWINNE_AIMED_FULL, SIGEWINNE_MINI_BUBBLE],
            plunging: &[
                SIGEWINNE_PLUNGE,
                SIGEWINNE_PLUNGE_LOW,
                SIGEWINNE_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "ぴょんぴょん水療法",
            scalings: &[SIGEWINNE_SKILL_BUBBLE, SIGEWINNE_SKILL_BLADE],
        },
        elemental_burst: TalentData {
            name: "過飽和心優し注射",
            scalings: &[SIGEWINNE_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Tartaglia
// =============================================================================

// -- Normal Attack: 断雨 (Cutting Torrent) -- Physical (Bow) --

const TARTAGLIA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4128, 0.4464, 0.4800, 0.5280, 0.5616, 0.6000, 0.6528, 0.7056, 0.7584, 0.8160, 0.8736,
        0.9312, 0.9888, 1.0464, 1.1040,
    ],
};

const TARTAGLIA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4627, 0.5003, 0.5380, 0.5918, 0.6295, 0.6725, 0.7317, 0.7909, 0.8500, 0.9146, 0.9792,
        1.0437, 1.1083, 1.1728, 1.2374,
    ],
};

const TARTAGLIA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5538, 0.5989, 0.6440, 0.7084, 0.7535, 0.8050, 0.8758, 0.9467, 1.0175, 1.0948, 1.1721,
        1.2494, 1.3266, 1.4039, 1.4812,
    ],
};

const TARTAGLIA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5702, 0.6166, 0.6630, 0.7293, 0.7757, 0.8288, 0.9017, 0.9746, 1.0475, 1.1271, 1.2067,
        1.2862, 1.3658, 1.4453, 1.5249,
    ],
};

const TARTAGLIA_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6089, 0.6584, 0.7080, 0.7788, 0.8284, 0.8850, 0.9629, 1.0408, 1.1186, 1.2036, 1.2886,
        1.3735, 1.4585, 1.5434, 1.6284,
    ],
};

const TARTAGLIA_NORMAL_6: TalentScaling = TalentScaling {
    name: "6段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7276, 0.7868, 0.8460, 0.9306, 0.9898, 1.0575, 1.1506, 1.2436, 1.3367, 1.4382, 1.5397,
        1.6412, 1.7428, 1.8443, 1.9458,
    ],
};

// -- Charged Attack -- Bow --

const TARTAGLIA_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const TARTAGLIA_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

// -- Plunging Attack -- Physical --

const TARTAGLIA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8178, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const TARTAGLIA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1853, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const TARTAGLIA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 魔王の武装·荒波 (Foul Legacy: Raging Tide) -- Hydro (Melee stance) --

const TARTAGLIA_MELEE_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3887, 0.4204, 0.4520, 0.4972, 0.5288, 0.5650, 0.6147, 0.6644, 0.7142, 0.7684, 0.8226,
        0.8769, 0.9311, 0.9854, 1.0396,
    ],
};

const TARTAGLIA_MELEE_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.4162, 0.4501, 0.4840, 0.5324, 0.5663, 0.6050, 0.6582, 0.7115, 0.7647, 0.8228, 0.8809,
        0.9390, 0.9970, 1.0551, 1.1132,
    ],
};

const TARTAGLIA_MELEE_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5633, 0.6092, 0.6550, 0.7205, 0.7664, 0.8188, 0.8908, 0.9629, 1.0349, 1.1135, 1.1921,
        1.2707, 1.3493, 1.4279, 1.5065,
    ],
};

const TARTAGLIA_MELEE_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5994, 0.6482, 0.6970, 0.7667, 0.8155, 0.8713, 0.9479, 1.0246, 1.1013, 1.1849, 1.2685,
        1.3522, 1.4358, 1.5195, 1.6031,
    ],
};

const TARTAGLIA_MELEE_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5530, 0.5980, 0.6430, 0.7073, 0.7523, 0.8038, 0.8745, 0.9452, 1.0159, 1.0931, 1.1703,
        1.2474, 1.3246, 1.4017, 1.4789,
    ],
};

const TARTAGLIA_MELEE_6A: TalentScaling = TalentScaling {
    name: "6段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3543, 0.3832, 0.4120, 0.4532, 0.4820, 0.5150, 0.5603, 0.6056, 0.6510, 0.7004, 0.7498,
        0.7993, 0.8487, 0.8982, 0.9476,
    ],
};

const TARTAGLIA_MELEE_6B: TalentScaling = TalentScaling {
    name: "6段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.3767, 0.4073, 0.4380, 0.4818, 0.5125, 0.5475, 0.5957, 0.6439, 0.6920, 0.7446, 0.7972,
        0.8497, 0.9023, 0.9548, 1.0074,
    ],
};

const TARTAGLIA_MELEE_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.6020, 0.6510, 0.7000, 0.7700, 0.8190, 0.8750, 0.9520, 1.0290, 1.1060, 1.1900, 1.2740,
        1.3580, 1.4420, 1.5260, 1.6100,
    ],
};

const TARTAGLIA_MELEE_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.7198, 0.7784, 0.8370, 0.9207, 0.9793, 1.0463, 1.1383, 1.2304, 1.3225, 1.4229, 1.5233,
        1.6238, 1.7242, 1.8247, 1.9251,
    ],
};

const TARTAGLIA_RIPTIDE_SLASH: TalentScaling = TalentScaling {
    name: "断流・斬ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.6020, 0.6510, 0.7000, 0.7700, 0.8190, 0.8750, 0.9520, 1.0290, 1.1060, 1.1900, 1.2740,
        1.3580, 1.4420, 1.5260, 1.6100,
    ],
};

// -- Elemental Burst: 極悪技·尽滅閃 (Havoc: Obliteration) -- Hydro --

const TARTAGLIA_BURST_MELEE: TalentScaling = TalentScaling {
    name: "近接スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        4.6400, 4.9880, 5.3360, 5.8000, 6.1480, 6.4960, 6.9600, 7.4240, 7.8880, 8.3520, 8.8160,
        9.2800, 9.8600, 10.4400, 11.0200,
    ],
};

const TARTAGLIA_BURST_RANGED: TalentScaling = TalentScaling {
    name: "遠距離スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        3.7840, 4.0678, 4.3516, 4.7300, 5.0138, 5.2976, 5.6760, 6.0544, 6.4328, 6.8112, 7.1896,
        7.5680, 8.0410, 8.5140, 8.9870,
    ],
};

const TARTAGLIA_BURST_RIPTIDE: TalentScaling = TalentScaling {
    name: "断流・爆ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.2000, 1.2900, 1.3800, 1.5000, 1.5900, 1.6800, 1.8000, 1.9200, 2.0400, 2.1600, 2.2800,
        2.4000, 2.5500, 2.7000, 2.8500,
    ],
};

pub const TARTAGLIA: CharacterData = CharacterData {
    id: "tartaglia",
    name: "Tartaglia",
    element: Element::Hydro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Snezhnaya,
    base_hp: [1020.0, 11561.0, 12182.0, 13103.0],
    base_atk: [23.0, 260.0, 274.0, 301.0],
    base_def: [63.0, 714.0, 753.0, 810.0],
    ascension_stat: AscensionStat::ElementalDmgBonus(Element::Hydro, 0.288),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "断雨",
            hits: &[
                TARTAGLIA_NORMAL_1,
                TARTAGLIA_NORMAL_2,
                TARTAGLIA_NORMAL_3,
                TARTAGLIA_NORMAL_4,
                TARTAGLIA_NORMAL_5,
                TARTAGLIA_NORMAL_6,
            ],
            charged: &[TARTAGLIA_AIMED, TARTAGLIA_AIMED_FULL],
            plunging: &[
                TARTAGLIA_PLUNGE,
                TARTAGLIA_PLUNGE_LOW,
                TARTAGLIA_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "魔王の武装·荒波",
            scalings: &[
                TARTAGLIA_MELEE_1,
                TARTAGLIA_MELEE_2,
                TARTAGLIA_MELEE_3,
                TARTAGLIA_MELEE_4,
                TARTAGLIA_MELEE_5,
                TARTAGLIA_MELEE_6A,
                TARTAGLIA_MELEE_6B,
                TARTAGLIA_MELEE_CHARGED_1,
                TARTAGLIA_MELEE_CHARGED_2,
                TARTAGLIA_RIPTIDE_SLASH,
            ],
        },
        elemental_burst: TalentData {
            name: "極悪技·尽滅閃",
            scalings: &[
                TARTAGLIA_BURST_MELEE,
                TARTAGLIA_BURST_RANGED,
                TARTAGLIA_BURST_RIPTIDE,
            ],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};

// =============================================================================
// Xingqiu
// =============================================================================

// -- Normal Attack: 古華剣法 (Guhua Sword: Fatal Rainscreen) -- Physical (Sword) --

const XINGQIU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4661, 0.5041, 0.5420, 0.5962, 0.6341, 0.6775, 0.7371, 0.7967, 0.8564, 0.9214, 0.9959,
        1.0836, 1.1712, 1.2588, 1.3545,
    ],
};

const XINGQIU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4764, 0.5152, 0.5540, 0.6094, 0.6482, 0.6925, 0.7534, 0.8144, 0.8753, 0.9418, 1.0180,
        1.1076, 1.1971, 1.2867, 1.3844,
    ],
};

const XINGQIU_NORMAL_3A: TalentScaling = TalentScaling {
    name: "3段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2855, 0.3088, 0.3320, 0.3652, 0.3884, 0.4150, 0.4515, 0.4880, 0.5246, 0.5644, 0.6101,
        0.6637, 0.7174, 0.7711, 0.8297,
    ],
};

const XINGQIU_NORMAL_3B: TalentScaling = TalentScaling {
    name: "3段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.2855, 0.3088, 0.3320, 0.3652, 0.3884, 0.4150, 0.4515, 0.4880, 0.5246, 0.5644, 0.6101,
        0.6637, 0.7174, 0.7711, 0.8297,
    ],
};

const XINGQIU_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5599, 0.6054, 0.6510, 0.7161, 0.7617, 0.8138, 0.8854, 0.9570, 1.0286, 1.1067, 1.1962,
        1.3015, 1.4067, 1.5120, 1.6268,
    ],
};

const XINGQIU_NORMAL_5A: TalentScaling = TalentScaling {
    name: "5段ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3586, 0.3878, 0.4170, 0.4587, 0.4879, 0.5213, 0.5671, 0.6130, 0.6589, 0.7089, 0.7662,
        0.8337, 0.9011, 0.9685, 1.0421,
    ],
};

const XINGQIU_NORMAL_5B: TalentScaling = TalentScaling {
    name: "5段ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3586, 0.3878, 0.4170, 0.4587, 0.4879, 0.5213, 0.5671, 0.6130, 0.6589, 0.7089, 0.7662,
        0.8337, 0.9011, 0.9685, 1.0421,
    ],
};

// -- Charged Attack -- Physical (Sword) --

const XINGQIU_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4730, 0.5115, 0.5500, 0.6050, 0.6435, 0.6875, 0.7480, 0.8085, 0.8690, 0.9350, 1.0106,
        1.0996, 1.1885, 1.2774, 1.3745,
    ],
};

const XINGQIU_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5616, 0.6073, 0.6530, 0.7183, 0.7640, 0.8163, 0.8881, 0.9599, 1.0317, 1.1101, 1.1999,
        1.3055, 1.4111, 1.5167, 1.6318,
    ],
};

// -- Plunging Attack -- Physical (Sword) --

const XINGQIU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const XINGQIU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const XINGQIU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: 古華剣·画雨籠山 (Guhua Sword: Fatal Rainscreen) -- Hydro --

const XINGQIU_SKILL_1: TalentScaling = TalentScaling {
    name: "スキルダメージ(1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.6800, 1.8060, 1.9320, 2.1000, 2.2260, 2.3520, 2.5200, 2.6880, 2.8560, 3.0240, 3.1920,
        3.3600, 3.5700, 3.7800, 3.9900,
    ],
};

const XINGQIU_SKILL_2: TalentScaling = TalentScaling {
    name: "スキルダメージ(2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.9120, 2.0554, 2.1988, 2.3900, 2.5334, 2.6768, 2.8680, 3.0592, 3.2504, 3.4416, 3.6328,
        3.8240, 4.0630, 4.3020, 4.5410,
    ],
};

// -- Elemental Burst: 古華剣·裁雨留虹 (Guhua Sword: Raincutter) -- Hydro --

const XINGQIU_BURST: TalentScaling = TalentScaling {
    name: "剣雨のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        0.5427, 0.5834, 0.6241, 0.6784, 0.7191, 0.7598, 0.8141, 0.8684, 0.9226, 0.9769, 1.0312,
        1.0854, 1.1533, 1.2211, 1.2890,
    ],
};

pub const XINGQIU: CharacterData = CharacterData {
    id: "xingqiu",
    name: "Xingqiu",
    element: Element::Hydro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [857.0, 9060.0, 9514.0, 10222.0],
    base_atk: [17.0, 178.0, 187.0, 202.0],
    base_def: [64.0, 673.0, 707.0, 758.0],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "古華剣法",
            hits: &[
                XINGQIU_NORMAL_1,
                XINGQIU_NORMAL_2,
                XINGQIU_NORMAL_3A,
                XINGQIU_NORMAL_3B,
                XINGQIU_NORMAL_4,
                XINGQIU_NORMAL_5A,
                XINGQIU_NORMAL_5B,
            ],
            charged: &[XINGQIU_CHARGED_1, XINGQIU_CHARGED_2],
            plunging: &[XINGQIU_PLUNGE, XINGQIU_PLUNGE_LOW, XINGQIU_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "古華剣·画雨籠山",
            scalings: &[XINGQIU_SKILL_1, XINGQIU_SKILL_2],
        },
        elemental_burst: TalentData {
            name: "古華剣·裁雨留虹",
            scalings: &[XINGQIU_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Yelan
// =============================================================================

// -- Normal Attack: 匿影隠曜の弓 (Stealthy Bowshot) -- Physical (Bow) --

const YELAN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4068, 0.4399, 0.4730, 0.5203, 0.5534, 0.5913, 0.6433, 0.6953, 0.7473, 0.8041, 0.8609,
        0.9176, 0.9744, 1.0310, 1.0879,
    ],
};

const YELAN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3904, 0.4222, 0.4540, 0.4994, 0.5312, 0.5675, 0.6174, 0.6674, 0.7173, 0.7718, 0.8263,
        0.8808, 0.9352, 0.9897, 1.0442,
    ],
};

const YELAN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5160, 0.5580, 0.6000, 0.6600, 0.7020, 0.7500, 0.8160, 0.8820, 0.9480, 1.0200, 1.0920,
        1.1640, 1.2360, 1.3080, 1.3800,
    ],
};

const YELAN_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ(×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3251, 0.3515, 0.3780, 0.4158, 0.4423, 0.4725, 0.5141, 0.5557, 0.5972, 0.6426, 0.6880,
        0.7333, 0.7787, 0.8240, 0.8694,
    ],
};

// -- Charged Attack -- Bow (HP scaling for Breakthrough Barb) --

const YELAN_AIMED: TalentScaling = TalentScaling {
    name: "狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4386, 0.4743, 0.5100, 0.5610, 0.5967, 0.6375, 0.6936, 0.7497, 0.8058, 0.8670, 0.9282,
        0.9894, 1.0506, 1.1118, 1.1730,
    ],
};

const YELAN_AIMED_FULL: TalentScaling = TalentScaling {
    name: "フルチャージ狙い撃ち",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.2400, 1.3330, 1.4260, 1.5500, 1.6430, 1.7360, 1.8600, 1.9840, 2.1080, 2.2320, 2.3560,
        2.4800, 2.6350, 2.7900, 2.9450,
    ],
};

const YELAN_BREAKTHROUGH: TalentScaling = TalentScaling {
    name: "破局の矢ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.1158, 0.1244, 0.1331, 0.1447, 0.1534, 0.1621, 0.1736, 0.1852, 0.1968, 0.2084, 0.2199,
        0.2315, 0.2460, 0.2605, 0.2749,
    ],
};

// -- Plunging Attack -- Physical --

const YELAN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const YELAN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0390,
    ],
};

const YELAN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: 絡み合う命の糸 (Lingering Lifeline) -- Hydro (HP scaling) --

const YELAN_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.2261, 0.2431, 0.2601, 0.2827, 0.2996, 0.3166, 0.3392, 0.3618, 0.3844, 0.4070, 0.4297,
        0.4523, 0.4805, 0.5088, 0.5371,
    ],
};

// -- Elemental Burst: 深謀玲瓏賽 (Depth-Clarion Dice) -- Hydro (HP scaling) --

const YELAN_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0731, 0.0786, 0.0840, 0.0914, 0.0968, 0.1023, 0.1096, 0.1169, 0.1242, 0.1315, 0.1389,
        0.1462, 0.1553, 0.1644, 0.1736,
    ],
};

const YELAN_BURST_EXQUISITE_THROW: TalentScaling = TalentScaling {
    name: "玲瓏一擲ダメージ(×3)",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Hydro),
    values: [
        0.0487, 0.0524, 0.0560, 0.0609, 0.0646, 0.0682, 0.0731, 0.0780, 0.0828, 0.0877, 0.0926,
        0.0974, 0.1035, 0.1096, 0.1157,
    ],
};

pub const YELAN: CharacterData = CharacterData {
    id: "yelan",
    name: "Yelan",
    element: Element::Hydro,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    region: Region::Liyue,
    base_hp: [1125.0, 12749.0, 13434.0, 14450.0],
    base_atk: [19.0, 214.0, 225.0, 244.0],
    base_def: [43.0, 489.0, 515.0, 548.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "匿影隠曜の弓",
            hits: &[
                YELAN_NORMAL_1,
                YELAN_NORMAL_2,
                YELAN_NORMAL_3,
                YELAN_NORMAL_4,
            ],
            charged: &[YELAN_AIMED, YELAN_AIMED_FULL, YELAN_BREAKTHROUGH],
            plunging: &[YELAN_PLUNGE, YELAN_PLUNGE_LOW, YELAN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "絡み合う命の糸",
            scalings: &[YELAN_SKILL],
        },
        elemental_burst: TalentData {
            name: "深謀玲瓏賽",
            scalings: &[YELAN_BURST, YELAN_BURST_EXQUISITE_THROW],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};

// =============================================================================
// Aino — 4★ Hydro Claymore (Nod-Krai / v6.0)
// Placeholder values — update from Genshin Impact Wiki when available
// =============================================================================

// -- Normal Attack -- Physical --

const AINO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7032, 0.7604, 0.8176, 0.8994, 0.9566, 1.0220, 1.1120, 1.2019, 1.2918, 1.3899, 1.4880,
        1.5862, 1.6843, 1.7824, 1.8805,
    ],
};

const AINO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6588, 0.7124, 0.7660, 0.8426, 0.8962, 0.9575, 1.0417, 1.1260, 1.2102, 1.3022, 1.3942,
        1.4862, 1.5782, 1.6702, 1.7622,
    ],
};

const AINO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7920, 0.8564, 0.9208, 1.0128, 1.0772, 1.1510, 1.2524, 1.3538, 1.4552, 1.5658, 1.6764,
        1.7870, 1.8976, 2.0082, 2.1188,
    ],
};

const AINO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9636, 1.0421, 1.1206, 1.2327, 1.3112, 1.4007, 1.5239, 1.6470, 1.7702, 1.9043, 2.0385,
        2.1726, 2.3068, 2.4409, 2.5751,
    ],
};

// -- Charged Attack --

const AINO_CHARGED: TalentScaling = TalentScaling {
    name: "チャージ攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.2780, 1.3820, 1.4861, 1.6347, 1.7387, 1.8576, 2.0211, 2.1845, 2.3480, 2.5263, 2.7047,
        2.8830, 3.0614, 3.2397, 3.4181,
    ],
};

// -- Plunging Attack --

const AINO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2748, 1.3702, 1.4742, 1.5783,
        1.6823, 1.7864, 1.8904, 1.9944,
    ],
};

const AINO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9076, 2.0289, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5726, 3.7807, 3.9888,
    ],
};

const AINO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1661, 2.3827, 2.5342, 2.7076, 2.9459, 3.1842, 3.4225, 3.6826, 3.9428,
        4.2029, 4.4631, 4.7232, 4.9834,
    ],
};

// -- Elemental Skill -- Hydro --

const AINO_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        1.68, 1.806, 1.932, 2.10, 2.226, 2.352, 2.52, 2.688, 2.856, 3.024, 3.192, 3.36, 3.57, 3.78,
        3.99,
    ],
};

// -- Elemental Burst -- Hydro --

const AINO_BURST: TalentScaling = TalentScaling {
    name: "バーストダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Hydro),
    values: [
        2.016, 2.1672, 2.3184, 2.52, 2.6712, 2.8224, 3.024, 3.2256, 3.4272, 3.6288, 3.8304, 4.032,
        4.284, 4.536, 4.788,
    ],
};

pub const AINO: CharacterData = CharacterData {
    id: "aino",
    name: "Aino",
    element: Element::Hydro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Snezhnaya,
    base_hp: [1090.0, 9570.0, 10017.0, 10768.0],
    base_atk: [18.0, 162.0, 170.0, 183.0],
    base_def: [67.0, 589.0, 617.0, 664.0],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "Placeholder Normal Attack",
            hits: &[AINO_NORMAL_1, AINO_NORMAL_2, AINO_NORMAL_3, AINO_NORMAL_4],
            charged: &[AINO_CHARGED],
            plunging: &[AINO_PLUNGE, AINO_PLUNGE_LOW, AINO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "Placeholder Skill",
            scalings: &[AINO_SKILL],
        },
        elemental_burst: TalentData {
            name: "Placeholder Burst",
            scalings: &[AINO_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
