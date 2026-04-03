use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Navia
// =============================================================================

// -- Normal Attack: 多様性を守る銃弾 (Blunt Refusal) -- Physical --

const NAVIA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9352, 1.0114, 1.0877, 1.1964, 1.2727, 1.3596, 1.4789, 1.5983, 1.7176, 1.8487, 1.9797,
        2.1108, 2.2418, 2.3729, 2.5039,
    ],
};

const NAVIA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8651, 0.9357, 1.0063, 1.1069, 1.1775, 1.2579, 1.3683, 1.4788, 1.5892, 1.7104, 1.8315,
        1.9527, 2.0738, 2.1950, 2.3161,
    ],
};

const NAVIA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ1",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3485, 0.3769, 0.4053, 0.4458, 0.4742, 0.5066, 0.5511, 0.5956, 0.6401, 0.6889, 0.7377,
        0.7865, 0.8353, 0.8841, 0.9329,
    ],
};

const NAVIA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.3334, 1.4420, 1.5506, 1.7057, 1.8143, 1.9383, 2.1088, 2.2794, 2.4499, 2.6360, 2.8222,
        3.0083, 3.1945, 3.3806, 3.5668,
    ],
};

// -- Charged Attack -- Physical --

const NAVIA_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6252, 0.6762, 0.7271, 0.7998, 0.8508, 0.9089, 0.9888, 1.0688, 1.1487, 1.2361, 1.3234,
        1.4107, 1.4981, 1.5854, 1.6727,
    ],
};

const NAVIA_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1309, 1.2231, 1.3152, 1.4468, 1.5389, 1.6441, 1.7887, 1.9333, 2.0779, 2.2359, 2.3939,
        2.5519, 2.7099, 2.8679, 3.0259,
    ],
};

// -- Plunging Attack -- Physical --

const NAVIA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9541, 1.0148, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
};

const NAVIA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
};

const NAVIA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9459, 3.1841, 3.4223, 3.6824, 3.9424,
        4.2024, 4.4625, 4.7225, 4.9826,
    ],
};

// -- Elemental Skill: 大キラインの式典装弾 (Ceremonial Crystalshot) -- Geo --

const NAVIA_SKILL_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ (基礎)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        3.9480, 4.2441, 4.5402, 4.9350, 5.2311, 5.5272, 5.9220, 6.3168, 6.7116, 7.1064, 7.5012,
        7.8960, 8.3895, 8.8830, 9.3765,
    ],
};

const NAVIA_SKILL_CRYSTAL_SHRAPNEL: TalentScaling = TalentScaling {
    name: "結晶弾片ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.3600, 0.3870, 0.4140, 0.4500, 0.4770, 0.5040, 0.5400, 0.5760, 0.6120, 0.6480, 0.6840,
        0.7200, 0.7650, 0.8100, 0.8550,
    ],
};

// -- Elemental Burst: 裁判のキ光弾 (As the Sunlit Sky's Singing Salute) -- Geo --

const NAVIA_BURST_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.7524, 0.8088, 0.8653, 0.9405, 0.9970, 1.0534, 1.1286, 1.2038, 1.2791, 1.3543, 1.4296,
        1.5048, 1.5989, 1.6929, 1.7870,
    ],
};

const NAVIA_BURST_CANNON: TalentScaling = TalentScaling {
    name: "砲弾ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        0.4309, 0.4632, 0.4956, 0.5386, 0.5710, 0.6033, 0.6463, 0.6894, 0.7324, 0.7755, 0.8185,
        0.8616, 0.9155, 0.9693, 1.0232,
    ],
};

pub const NAVIA: CharacterData = CharacterData {
    id: "navia",
    name: "Navia",
    element: Element::Geo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [
        985.00, 2555.00, 3399.00, 5086.00, 5686.00, 6542.00, 7341.00, 8206.00, 8806.00, 9679.00,
        10278.00, 11161.00, 11761.00, 12650.00, 12650.00, 13156.00, // Lv95/Lv95+/Lv100
        13156.00, // Lv95/Lv95+/Lv100
        13662.00, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        27.37, 71.00, 94.47, 141.36, 158.03, 181.81, 204.05, 228.08, 244.75, 269.00, 285.68,
        310.20, 326.88, 351.59, 351.59, 365.65, // Lv95/Lv95+/Lv100
        365.65, // Lv95/Lv95+/Lv100
        379.72, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        61.74, 160.17, 213.11, 318.88, 356.49, 410.15, 460.30, 514.51, 552.13, 606.84, 644.45,
        699.78, 737.39, 793.15, 793.15, 824.88, // Lv95/Lv95+/Lv100
        824.88, // Lv95/Lv95+/Lv100
        856.60, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "多様性を守る銃弾",
            hits: &[
                NAVIA_NORMAL_1,
                NAVIA_NORMAL_2,
                NAVIA_NORMAL_3,
                NAVIA_NORMAL_4,
            ],
            charged: &[NAVIA_CHARGED_SPINNING, NAVIA_CHARGED_FINAL],
            plunging: &[NAVIA_PLUNGE, NAVIA_PLUNGE_LOW, NAVIA_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "大キラインの式典装弾",
            scalings: &[NAVIA_SKILL_DAMAGE, NAVIA_SKILL_CRYSTAL_SHRAPNEL],
        },
        elemental_burst: TalentData {
            name: "裁判のキ光弾",
            scalings: &[NAVIA_BURST_DAMAGE, NAVIA_BURST_CANNON],
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
