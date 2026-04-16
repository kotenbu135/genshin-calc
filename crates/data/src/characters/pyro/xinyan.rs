use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Xinyan
// =============================================================================

// -- Normal Attack: 炎舞 (Dance on Fire) -- Physical --

const XINYAN_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7654, 0.8277, 0.8900, 0.9790, 1.0413, 1.1125, 1.2104, 1.3083, 1.4062, 1.5130, 1.6198,
        1.7266, 1.8334, 1.9402, 2.0470,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XINYAN_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7396, 0.7998, 0.8600, 0.9460, 1.0062, 1.0750, 1.1696, 1.2642, 1.3588, 1.4620, 1.5652,
        1.6684, 1.7716, 1.8748, 1.9780,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XINYAN_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9546, 1.0323, 1.1100, 1.2210, 1.2987, 1.3875, 1.5096, 1.6317, 1.7538, 1.8870, 2.0202,
        2.1534, 2.2866, 2.4198, 2.5530,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XINYAN_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1584, 1.2527, 1.3470, 1.4817, 1.5760, 1.6838, 1.8319, 1.9801, 2.1283, 2.2899, 2.4515,
        2.6132, 2.7748, 2.9365, 3.0981,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack -- Physical --

const XINYAN_CHARGED_SPINNING: TalentScaling = TalentScaling {
    name: "連続重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6255, 0.6764, 0.7273, 0.8000, 0.8509, 0.9091, 0.9891, 1.0691, 1.1491, 1.2364, 1.3236,
        1.4109, 1.4982, 1.5855, 1.6727,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XINYAN_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "重撃終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1309, 1.2230, 1.3150, 1.4465, 1.5386, 1.6438, 1.7884, 1.9331, 2.0777, 2.2355, 2.3933,
        2.5511, 2.7089, 2.8667, 3.0245,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const XINYAN_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7459, 0.8066, 0.8673, 0.9540, 1.0147, 1.0841, 1.1795, 1.2749, 1.3703, 1.4744, 1.5785,
        1.6826, 1.7866, 1.8907, 1.9948,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XINYAN_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.4914, 1.6128, 1.7342, 1.9077, 2.0291, 2.1678, 2.3586, 2.5493, 2.7401, 2.9482, 3.1563,
        3.3644, 3.5725, 3.7806, 3.9887,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XINYAN_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.8629, 2.0145, 2.1662, 2.3828, 2.5344, 2.7077, 2.9460, 3.1842, 3.4225, 3.6825, 3.9424,
        4.2023, 4.4623, 4.7222, 4.9821,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 情熱のスイーパー (Sweeping Fervor) -- Pyro --

const XINYAN_SKILL_SWING: TalentScaling = TalentScaling {
    name: "振り回しダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        1.6960, 1.8232, 1.9504, 2.1200, 2.2472, 2.3744, 2.5440, 2.7136, 2.8832, 3.0528, 3.2224,
        3.3920, 3.6040, 3.8160, 4.0280,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XINYAN_SKILL_DOT: TalentScaling = TalentScaling {
    name: "継続ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.3360, 0.3612, 0.3864, 0.4200, 0.4452, 0.4704, 0.5040, 0.5376, 0.5712, 0.6048, 0.6384,
        0.6720, 0.7140, 0.7560, 0.7980,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 叛逆の弾き (Riff Revolution) -- Pyro / Physical --

const XINYAN_BURST: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        3.4080, 3.6636, 3.9192, 4.2600, 4.5156, 4.7712, 5.1120, 5.4528, 5.7936, 6.1344, 6.4752,
        6.8160, 7.2420, 7.6680, 8.0940,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const XINYAN_BURST_DOT: TalentScaling = TalentScaling {
    name: "炎のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Pyro),
    values: [
        0.4000, 0.4300, 0.4600, 0.5000, 0.5300, 0.5600, 0.6000, 0.6400, 0.6800, 0.7200, 0.7600,
        0.8000, 0.8500, 0.9000, 0.9500,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const XINYAN: CharacterData = CharacterData {
    id: "xinyan",
    name: "Xinyan",
    element: Element::Pyro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Liyue,
    base_hp: [
        939.00, 2413.00, 3114.00, 4665.00, 5163.00, 5939.00, 6604.00, 7379.00, 7878.00, 8653.00,
        9151.00, 9927.00, 10425.00, 11201.00, 11201.00, 11649.04, // Lv95/Lv95+/Lv100
        11649.04, // Lv95/Lv95+/Lv100
        12097.08, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        20.84, 53.53, 69.09, 103.49, 114.55, 131.76, 146.51, 163.72, 174.78, 191.97, 203.03,
        220.24, 231.29, 248.50, 248.50, 258.44, // Lv95/Lv95+/Lv100
        258.44, // Lv95/Lv95+/Lv100
        268.38, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        66.95, 172.00, 222.02, 332.56, 368.10, 423.40, 470.79, 526.09, 561.63, 616.87, 652.40,
        707.71, 743.25, 798.55, 798.55, 830.49, // Lv95/Lv95+/Lv100
        830.49, // Lv95/Lv95+/Lv100
        862.43, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::Atk(0.24),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "炎舞",
            hits: &[
                XINYAN_NORMAL_1,
                XINYAN_NORMAL_2,
                XINYAN_NORMAL_3,
                XINYAN_NORMAL_4,
            ],
            charged: &[XINYAN_CHARGED_SPINNING, XINYAN_CHARGED_FINAL],
            plunging: &[XINYAN_PLUNGE, XINYAN_PLUNGE_LOW, XINYAN_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "情熱のスイーパー",
            scalings: &[XINYAN_SKILL_SWING, XINYAN_SKILL_DOT],
        },
        elemental_burst: TalentData {
            name: "叛逆の弾き",
            scalings: &[XINYAN_BURST, XINYAN_BURST_DOT],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
    scaling_modifiers: &[],
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xinyan_burst_main_hit_is_physical_and_dot_is_pyro() {
        let burst = XINYAN.talents.elemental_burst.scalings;
        assert_eq!(burst.len(), 2);
        assert_eq!(burst[0].name, "スキルダメージ");
        assert_eq!(burst[0].damage_element, None);
        assert_eq!(burst[1].name, "炎のダメージ");
        assert_eq!(burst[1].damage_element, Some(Element::Pyro));
    }
}
