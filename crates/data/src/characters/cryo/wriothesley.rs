use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================

// -- Normal Attack: Forceful Fists of Frost -- Cryo (Catalyst) --

const WRIOTHESLEY_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5336, 0.5770, 0.6205, 0.6825, 0.7259, 0.7756, 0.8438, 0.9121, 0.9803, 1.0548, 1.1292,
        1.2037, 1.2781, 1.3526, 1.4271,
    ],
};

const WRIOTHESLEY_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5180, 0.5601, 0.6023, 0.6625, 0.7047, 0.7529, 0.8191, 0.8854, 0.9517, 1.0239, 1.0962,
        1.1685, 1.2408, 1.3130, 1.3853,
    ],
};

const WRIOTHESLEY_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.6722, 0.7269, 0.7817, 0.8598, 0.9145, 0.9771, 1.0631, 1.1490, 1.2350, 1.3288, 1.4226,
        1.5164, 1.6102, 1.7040, 1.7978,
    ],
};

const WRIOTHESLEY_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ (×2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.3790, 0.4099, 0.4407, 0.4848, 0.5157, 0.5509, 0.5994, 0.6479, 0.6964, 0.7493, 0.8022,
        0.8550, 0.9079, 0.9608, 1.0137,
    ],
};

const WRIOTHESLEY_NORMAL_5: TalentScaling = TalentScaling {
    name: "5段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.9074, 0.9813, 1.0551, 1.1607, 1.2345, 1.3189, 1.4350, 1.5511, 1.6671, 1.7937, 1.9204,
        2.0470, 2.1736, 2.3005, 2.4268,
    ],
};

// -- Charged Attack (Vaulting Fist) -- Cryo (Catalyst) --

const WRIOTHESLEY_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.5296, 1.6443, 1.7590, 1.9120, 2.0267, 2.1414, 2.2944, 2.4736, 2.6003, 2.7533, 2.9062,
        3.0592, 3.2504, 3.4416, 3.6328,
    ],
};

// -- Plunging Attack -- Cryo (Catalyst) --

const WRIOTHESLEY_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.5683, 0.6145, 0.6608, 0.7269, 0.7731, 0.8260, 0.8987, 0.9714, 1.0441, 1.1234, 1.2027,
        1.2820, 1.3612, 1.4405, 1.5198,
    ],
};

const WRIOTHESLEY_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.1363, 1.2288, 1.3213, 1.4535, 1.5459, 1.6517, 1.7970, 1.9423, 2.0877, 2.2462, 2.4048,
        2.5634, 2.7219, 2.8805, 3.0391,
    ],
};

const WRIOTHESLEY_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.4193, 1.5349, 1.6504, 1.8154, 1.9310, 2.0630, 2.2445, 2.4261, 2.6076, 2.8057, 3.0037,
        3.2018, 3.3998, 3.5979, 3.7959,
    ],
};

// -- Elemental Skill: Icefang Rush -- Cryo --

const WRIOTHESLEY_SKILL: TalentScaling = TalentScaling {
    name: "強化パンチダメージ増加",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.4317, 1.4575, 1.4834, 1.5170, 1.5429, 1.5687, 1.6023, 1.6359, 1.6695, 1.7031, 1.7367,
        1.7703, 1.8039, 1.8375, 1.8711,
    ],
};

// -- Elemental Burst: Darkgold Wolfbite -- Cryo --

const WRIOTHESLEY_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ (×5)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        1.2720, 1.3674, 1.4628, 1.5900, 1.6854, 1.7808, 1.9080, 2.0352, 2.1624, 2.2896, 2.4168,
        2.5440, 2.7030, 2.8620, 3.0210,
    ],
};

const WRIOTHESLEY_BURST_SURGING: TalentScaling = TalentScaling {
    name: "サージングブレードダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Cryo),
    values: [
        0.4240, 0.4558, 0.4876, 0.5300, 0.5618, 0.5936, 0.6360, 0.6784, 0.7208, 0.7632, 0.8056,
        0.8480, 0.9010, 0.9540, 1.0070,
    ],
};

pub const WRIOTHESLEY: CharacterData = CharacterData {
    id: "wriothesley",
    name: "Wriothesley",
    element: Element::Cryo,
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    region: Region::Fontaine,
    base_hp: [1058.0, 11993.0, 12637.0, 13593.0],
    base_atk: [24.0, 274.0, 289.0, 311.0],
    base_def: [59.0, 673.0, 710.0, 763.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "力の拳撃・霜",
            hits: &[
                WRIOTHESLEY_NORMAL_1,
                WRIOTHESLEY_NORMAL_2,
                WRIOTHESLEY_NORMAL_3,
                WRIOTHESLEY_NORMAL_4,
                WRIOTHESLEY_NORMAL_5,
            ],
            charged: &[WRIOTHESLEY_CHARGED],
            plunging: &[
                WRIOTHESLEY_PLUNGE,
                WRIOTHESLEY_PLUNGE_LOW,
                WRIOTHESLEY_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "氷牙突進",
            scalings: &[WRIOTHESLEY_SKILL],
        },
        elemental_burst: TalentData {
            name: "暗金の狼噛み",
            scalings: &[WRIOTHESLEY_BURST, WRIOTHESLEY_BURST_SURGING],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
