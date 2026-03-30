use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

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
