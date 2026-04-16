use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// Kuki Shinobu
// =============================================================================

// -- Normal Attack: 忍流飛刃斬り (Shinobu's Shadowsword) -- Physical --

const KUKI_SHINOBU_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4876, 0.5273, 0.5670, 0.6237, 0.6634, 0.7088, 0.7711, 0.8335, 0.8959, 0.9639, 1.0319,
        1.0999, 1.1680, 1.2361, 1.3041,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KUKI_SHINOBU_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4455, 0.4817, 0.5180, 0.5698, 0.6061, 0.6475, 0.7045, 0.7615, 0.8184, 0.8806, 0.9428,
        1.0049, 1.0671, 1.1292, 1.1914,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KUKI_SHINOBU_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5934, 0.6417, 0.6900, 0.7590, 0.8073, 0.8625, 0.9384, 1.0143, 1.0902, 1.1730, 1.2558,
        1.3386, 1.4214, 1.5042, 1.5870,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KUKI_SHINOBU_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7611, 0.8231, 0.8850, 0.9735, 1.0355, 1.1063, 1.2036, 1.3010, 1.3983, 1.5045, 1.6107,
        1.7169, 1.8231, 1.9293, 2.0355,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const KUKI_SHINOBU_CHARGED_1: TalentScaling = TalentScaling {
    name: "重撃ダメージ (1)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.5563, 0.6016, 0.6469, 0.7116, 0.7569, 0.8086, 0.8798, 0.9509, 1.0221, 1.0997, 1.1774,
        1.2550, 1.3326, 1.4102, 1.4879,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KUKI_SHINOBU_CHARGED_2: TalentScaling = TalentScaling {
    name: "重撃ダメージ (2)",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6677, 0.7220, 0.7763, 0.8540, 0.9083, 0.9704, 1.0558, 1.1412, 1.2266, 1.3198, 1.4129,
        1.5061, 1.5993, 1.6924, 1.7856,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const KUKI_SHINOBU_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530,
        1.4422, 1.5314, 1.6206, 1.7098,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KUKI_SHINOBU_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054,
        2.8838, 3.0622, 3.2405, 3.4189,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KUKI_SHINOBU_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792,
        3.6020, 3.8248, 4.0476, 4.2704,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 御咎験式・紫牙 (Sanctifying Ring) -- Electro --

const KUKI_SHINOBU_SKILL: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.7571, 0.8139, 0.8707, 0.9464, 1.0032, 1.0599, 1.1357, 1.2114, 1.2871, 1.3628, 1.4385,
        1.5142, 1.6089, 1.7035, 1.7982,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const KUKI_SHINOBU_SKILL_RING: TalentScaling = TalentScaling {
    name: "越祓草の輪ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro),
    values: [
        0.2524, 0.2713, 0.2903, 0.3155, 0.3344, 0.3534, 0.3786, 0.4038, 0.4291, 0.4543, 0.4796,
        0.5048, 0.5364, 0.5679, 0.5995,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 御影裁式・霊刃 (Gyoei Narukami Kariyama Rite) -- Electro --

const KUKI_SHINOBU_BURST: TalentScaling = TalentScaling {
    name: "単発ダメージ",
    scaling_stat: ScalingStat::Hp,
    damage_element: Some(Element::Electro),
    values: [
        0.0360, 0.0388, 0.0415, 0.0451, 0.0478, 0.0505, 0.0541, 0.0577, 0.0613, 0.0649, 0.0685,
        0.0721, 0.0766, 0.0811, 0.0856,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const KUKI_SHINOBU: CharacterData = CharacterData {
    id: "kuki_shinobu",
    name: "Kuki Shinobu",
    element: Element::Electro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    region: Region::Inazuma,
    base_hp: [
        1030.00, 2647.00, 3417.00, 5118.00, 5665.00, 6516.00, 7245.00, 8096.00, 8643.00, 9493.00,
        10040.00, 10891.00, 11438.00, 12289.00, 12289.00, 12780.56, // Lv95/Lv95+/Lv100
        12780.56, // Lv95/Lv95+/Lv100
        13272.12, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        17.81, 45.75, 59.05, 88.45, 97.91, 112.62, 125.22, 139.93, 149.38, 164.07, 173.53, 188.24,
        197.69, 212.40, 212.40, 220.90, // Lv95/Lv95+/Lv100
        220.90, // Lv95/Lv95+/Lv100
        229.39, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        62.95, 161.71, 208.74, 312.66, 346.08, 398.07, 442.62, 494.62, 528.03, 579.96, 613.37,
        665.37, 698.78, 750.77, 750.77, 780.80, // Lv95/Lv95+/Lv100
        780.80, // Lv95/Lv95+/Lv100
        810.83, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Hp(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "忍流飛刃斬り",
            hits: &[
                KUKI_SHINOBU_NORMAL_1,
                KUKI_SHINOBU_NORMAL_2,
                KUKI_SHINOBU_NORMAL_3,
                KUKI_SHINOBU_NORMAL_4,
            ],
            charged: &[KUKI_SHINOBU_CHARGED_1, KUKI_SHINOBU_CHARGED_2],
            plunging: &[
                KUKI_SHINOBU_PLUNGE,
                KUKI_SHINOBU_PLUNGE_LOW,
                KUKI_SHINOBU_PLUNGE_HIGH,
            ],
        },
        elemental_skill: TalentData {
            name: "越祓雷草の輪",
            scalings: &[KUKI_SHINOBU_SKILL, KUKI_SHINOBU_SKILL_RING],
        },
        elemental_burst: TalentData {
            name: "御影裁式・霊刃",
            scalings: &[KUKI_SHINOBU_BURST],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kuki_burst_scales_with_hp() {
        assert_eq!(KUKI_SHINOBU_BURST.scaling_stat, ScalingStat::Hp);
    }
}
