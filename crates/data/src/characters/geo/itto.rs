use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Itto (Arataki Itto)
// =============================================================================

// -- Normal Attack: 喧嘩キ暴走 (Fight Club Legend) -- Physical --

const ITTO_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7918, 0.8564, 0.9210, 1.0131, 1.0777, 1.1513, 1.2523, 1.3534, 1.4545, 1.5654, 1.6929,
        1.8419, 1.9908, 2.1398, 2.3023,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ITTO_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7637, 0.8260, 0.8883, 0.9771, 1.0394, 1.1104, 1.2079, 1.3054, 1.4029, 1.5099, 1.6317,
        1.7753, 1.9189, 2.0625, 2.2191,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ITTO_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9161, 0.9907, 1.0653, 1.1718, 1.2464, 1.3316, 1.4490, 1.5664, 1.6838, 1.8113, 1.9580,
        2.1303, 2.3027, 2.4750, 2.6629,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ITTO_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1723, 1.2679, 1.3634, 1.4997, 1.5952, 1.7042, 1.8541, 2.0040, 2.1539, 2.3176, 2.5047,
        2.7251, 2.9455, 3.1659, 3.4063,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Charged Attack (Arataki Kesagiri) -- Physical --

const ITTO_CHARGED_SLASH: TalentScaling = TalentScaling {
    name: "荒瀧キ逆袈裟連斬ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9123, 0.9866, 1.0610, 1.1671, 1.2414, 1.3262, 1.4425, 1.5588, 1.6751, 1.8031, 1.9478,
        2.1192, 2.2906, 2.4620, 2.6489,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ITTO_CHARGED_FINAL: TalentScaling = TalentScaling {
    name: "荒瀧キ逆袈裟終了ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.9092, 2.0647, 2.2203, 2.4423, 2.5978, 2.7755, 3.0197, 3.2638, 3.5080, 3.7746, 4.0793,
        4.4382, 4.7972, 5.1562, 5.5478,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ITTO_CHARGED_SAICHIMONJI: TalentScaling = TalentScaling {
    name: "左一文字斬りダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.9047, 0.9784, 1.0520, 1.1572, 1.2308, 1.3150, 1.4307, 1.5464, 1.6622, 1.7884, 1.9331,
        2.1032, 2.2733, 2.4434, 2.6289,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Plunging Attack -- Physical --

const ITTO_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.8195, 0.8862, 0.9530, 1.0483, 1.1150, 1.1912, 1.2960, 1.4008, 1.5057, 1.6200, 1.7344,
        1.8488, 1.9631, 2.0775, 2.1918,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ITTO_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.6387, 1.7722, 1.9058, 2.0964, 2.2299, 2.3822, 2.5916, 2.8010, 3.0104, 3.2395, 3.4686,
        3.6978, 3.9269, 4.1560, 4.3851,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

const ITTO_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        2.0474, 2.2142, 2.3810, 2.6191, 2.7859, 2.9762, 3.2378, 3.4993, 3.7609, 4.0472, 4.3335,
        4.6198, 4.9062, 5.1925, 5.4788,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Skill: 魔殺キ絶技 (Masatsu Zetsugi: Akaushi Burst!) -- Geo --

const ITTO_SKILL_DAMAGE: TalentScaling = TalentScaling {
    name: "スキルダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Geo),
    values: [
        3.0720, 3.3024, 3.5328, 3.8400, 4.0704, 4.3008, 4.6080, 4.9152, 5.2224, 5.5296, 5.8368,
        6.1440, 6.5280, 6.9120, 7.2960,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

// -- Elemental Burst: 最キ一番キ虎キ盛 (Royal Descent: Behold, Itto the Evil!) -- Geo infusion --

const ITTO_BURST_ATK_BONUS: TalentScaling = TalentScaling {
    name: "ATK追加 (DEF基準)",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        0.5760, 0.6192, 0.6624, 0.7200, 0.7632, 0.8064, 0.8640, 0.9216, 0.9792, 1.0368, 1.0944,
        1.1520, 1.2240, 1.2960, 1.3680,
    ],
    dynamic_bonus: None,
    damage_pipeline: DamagePipeline::Standard,
};

pub const ITTO: CharacterData = CharacterData {
    id: "itto",
    name: "Arataki Itto",
    element: Element::Geo,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Inazuma,
    base_hp: [
        1001.00, 2597.00, 3455.00, 5170.00, 5779.00, 6649.00, 7462.00, 8341.00, 8951.00, 9838.00,
        10448.00, 11345.00, 11954.00, 12858.00, 12858.00, 13372.32, // Lv95/Lv95+/Lv100
        13372.32, // Lv95/Lv95+/Lv100
        13886.64, // Lv95/Lv95+/Lv100
    ],
    base_atk: [
        17.69, 45.89, 61.05, 91.35, 102.13, 117.50, 131.87, 147.40, 158.17, 173.85, 184.62, 200.47,
        211.25, 227.22, 227.22, 236.31, // Lv95/Lv95+/Lv100
        236.31, // Lv95/Lv95+/Lv100
        245.40, // Lv95/Lv95+/Lv100
    ],
    base_def: [
        74.67, 193.69, 257.71, 385.62, 431.11, 495.99, 556.65, 622.20, 667.69, 733.85, 779.34,
        846.24, 891.73, 959.16, 959.16, 997.53,  // Lv95/Lv95+/Lv100
        997.53,  // Lv95/Lv95+/Lv100
        1035.89, // Lv95/Lv95+/Lv100
    ],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "喧嘩キ暴走",
            hits: &[ITTO_NORMAL_1, ITTO_NORMAL_2, ITTO_NORMAL_3, ITTO_NORMAL_4],
            charged: &[
                ITTO_CHARGED_SLASH,
                ITTO_CHARGED_FINAL,
                ITTO_CHARGED_SAICHIMONJI,
            ],
            plunging: &[ITTO_PLUNGE, ITTO_PLUNGE_LOW, ITTO_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "魔殺キ絶技",
            scalings: &[ITTO_SKILL_DAMAGE],
        },
        elemental_burst: TalentData {
            name: "最キ一番キ虎キ盛",
            scalings: &[ITTO_BURST_ATK_BONUS],
        },
    },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
    passive_scalings: &[],
};

#[cfg(test)]
mod tests {
    use super::*;

    const SAICHIMONJI_EXPECTED: [f64; 15] = [
        0.9047, 0.9784, 1.0520, 1.1572, 1.2308, 1.3150, 1.4307, 1.5464, 1.6622, 1.7884, 1.9331,
        2.1032, 2.2733, 2.4434, 2.6289,
    ];

    fn assert_scaling_table(actual: &[f64; 15], expected: &[f64; 15], label: &str) {
        for (index, (&actual, &expected)) in actual.iter().zip(expected.iter()).enumerate() {
            assert!(
                (actual - expected).abs() <= 1e-4,
                "{label} Lv{}: expected {expected}, got {actual}",
                index + 1
            );
        }
    }

    #[test]
    fn itto_charged_tables_match_honeyhunter_mirror() {
        let charged = ITTO.talents.normal_attack.charged;
        assert_eq!(charged.len(), 3);
        assert_eq!(charged[2].name, "左一文字斬りダメージ");
        assert_scaling_table(&charged[2].values, &SAICHIMONJI_EXPECTED, "Saichimonji");
    }
}
