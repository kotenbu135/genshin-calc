/// StatKey変換結果のフィールド識別子
#[derive(Debug, Clone, PartialEq)]
pub enum StatField {
    HpFlat,
    HpPercent,
    AtkFlat,
    AtkPercent,
    DefFlat,
    DefPercent,
    ElementalMastery,
    EnergyRecharge,
    CritRate,
    CritDmg,
    DmgBonus,
    ElementalDmgBonus(&'static str), // "pyro", "hydro", etc.
    PhysicalDmgBonus,
}

/// StatKey変換の結果
#[derive(Debug, PartialEq)]
pub enum StatConvertResult {
    Converted(StatField, f64),
    Skip,    // heal_ etc — not in StatProfile
    Unknown, // truly unknown key
}

/// GOOD StatKeyと値を内部形式に変換。パーセント系は÷100済み
pub fn convert_stat(key: &str, value: f64) -> StatConvertResult {
    match key {
        "hp" => StatConvertResult::Converted(StatField::HpFlat, value),
        "atk" => StatConvertResult::Converted(StatField::AtkFlat, value),
        "def" => StatConvertResult::Converted(StatField::DefFlat, value),
        "hp_" => StatConvertResult::Converted(StatField::HpPercent, value / 100.0),
        "atk_" => StatConvertResult::Converted(StatField::AtkPercent, value / 100.0),
        "def_" => StatConvertResult::Converted(StatField::DefPercent, value / 100.0),
        "eleMas" => StatConvertResult::Converted(StatField::ElementalMastery, value),
        "enerRech_" => StatConvertResult::Converted(StatField::EnergyRecharge, value / 100.0),
        "critRate_" => StatConvertResult::Converted(StatField::CritRate, value / 100.0),
        "critDMG_" => StatConvertResult::Converted(StatField::CritDmg, value / 100.0),
        "physical_dmg_" => StatConvertResult::Converted(StatField::PhysicalDmgBonus, value / 100.0),
        "pyro_dmg_" => {
            StatConvertResult::Converted(StatField::ElementalDmgBonus("pyro"), value / 100.0)
        }
        "hydro_dmg_" => {
            StatConvertResult::Converted(StatField::ElementalDmgBonus("hydro"), value / 100.0)
        }
        "electro_dmg_" => {
            StatConvertResult::Converted(StatField::ElementalDmgBonus("electro"), value / 100.0)
        }
        "cryo_dmg_" => {
            StatConvertResult::Converted(StatField::ElementalDmgBonus("cryo"), value / 100.0)
        }
        "dendro_dmg_" => {
            StatConvertResult::Converted(StatField::ElementalDmgBonus("dendro"), value / 100.0)
        }
        "anemo_dmg_" => {
            StatConvertResult::Converted(StatField::ElementalDmgBonus("anemo"), value / 100.0)
        }
        "geo_dmg_" => {
            StatConvertResult::Converted(StatField::ElementalDmgBonus("geo"), value / 100.0)
        }
        "heal_" => StatConvertResult::Skip,
        _ => StatConvertResult::Unknown,
    }
}

/// StatFieldの値をStatProfileに加算する
pub fn add_to_profile(
    profile: &mut genshin_calc_core::StatProfile,
    field: &StatField,
    value: f64,
) -> bool {
    match field {
        StatField::HpFlat => {
            profile.hp_flat += value;
            true
        }
        StatField::HpPercent => {
            profile.hp_percent += value;
            true
        }
        StatField::AtkFlat => {
            profile.atk_flat += value;
            true
        }
        StatField::AtkPercent => {
            profile.atk_percent += value;
            true
        }
        StatField::DefFlat => {
            profile.def_flat += value;
            true
        }
        StatField::DefPercent => {
            profile.def_percent += value;
            true
        }
        StatField::ElementalMastery => {
            profile.elemental_mastery += value;
            true
        }
        StatField::EnergyRecharge => {
            profile.energy_recharge += value;
            true
        }
        StatField::CritRate => {
            profile.crit_rate += value;
            true
        }
        StatField::CritDmg => {
            profile.crit_dmg += value;
            true
        }
        StatField::DmgBonus => {
            profile.dmg_bonus += value;
            true
        }
        StatField::PhysicalDmgBonus => {
            profile.physical_dmg_bonus += value;
            true
        }
        StatField::ElementalDmgBonus(elem_str) => {
            match *elem_str {
                "pyro" => profile.pyro_dmg_bonus += value,
                "hydro" => profile.hydro_dmg_bonus += value,
                "electro" => profile.electro_dmg_bonus += value,
                "cryo" => profile.cryo_dmg_bonus += value,
                "dendro" => profile.dendro_dmg_bonus += value,
                "anemo" => profile.anemo_dmg_bonus += value,
                "geo" => profile.geo_dmg_bonus += value,
                _ => return false,
            }
            true
        }
    }
}

/// 聖遺物メインステの値を返す（生の%値、÷100前）
/// 注: 線形補間による近似値。中間レベルで±0.5%程度の誤差あり
pub fn main_stat_value(rarity: u8, main_stat_key: &str, level: u8) -> Option<f64> {
    let max_level = match rarity {
        5 => 20,
        4 => 16,
        3 => 12,
        _ => return None,
    };
    if level > max_level {
        return None;
    }
    let (base, max_val) = main_stat_base_max(rarity, main_stat_key)?;
    Some(base + (max_val - base) * (level as f64) / (max_level as f64))
}

fn main_stat_base_max(rarity: u8, key: &str) -> Option<(f64, f64)> {
    match (rarity, key) {
        // Star 5 (Lv0 → Lv20)
        (5, "hp") => Some((717.0, 4780.0)),
        (5, "atk") => Some((47.0, 311.0)),
        (5, "hp_") => Some((7.0, 46.6)),
        (5, "atk_") => Some((7.0, 46.6)),
        (5, "def_") => Some((8.7, 58.3)),
        (5, "eleMas") => Some((28.0, 187.0)),
        (5, "enerRech_") => Some((7.8, 51.8)),
        (5, "critRate_") => Some((4.7, 31.1)),
        (5, "critDMG_") => Some((9.3, 62.2)),
        (5, "physical_dmg_") => Some((8.7, 58.3)),
        (5, "pyro_dmg_")
        | (5, "hydro_dmg_")
        | (5, "electro_dmg_")
        | (5, "cryo_dmg_")
        | (5, "dendro_dmg_")
        | (5, "anemo_dmg_")
        | (5, "geo_dmg_") => Some((7.0, 46.6)),
        (5, "heal_") => Some((5.4, 35.9)),
        // Star 4 (Lv0 → Lv16)
        (4, "hp") => Some((645.0, 3571.0)),
        (4, "atk") => Some((42.0, 232.0)),
        (4, "hp_") => Some((6.3, 34.8)),
        (4, "atk_") => Some((6.3, 34.8)),
        (4, "def_") => Some((7.9, 43.5)),
        (4, "eleMas") => Some((25.0, 139.0)),
        (4, "enerRech_") => Some((7.0, 38.7)),
        (4, "critRate_") => Some((4.2, 23.2)),
        (4, "critDMG_") => Some((8.4, 46.4)),
        (4, "physical_dmg_") => Some((7.9, 43.5)),
        (4, "pyro_dmg_")
        | (4, "hydro_dmg_")
        | (4, "electro_dmg_")
        | (4, "cryo_dmg_")
        | (4, "dendro_dmg_")
        | (4, "anemo_dmg_")
        | (4, "geo_dmg_") => Some((6.3, 34.8)),
        (4, "heal_") => Some((4.8, 26.8)),
        // Star 3 (Lv0 → Lv12)
        (3, "hp") => Some((430.0, 1893.0)),
        (3, "atk") => Some((28.0, 123.0)),
        (3, "hp_") => Some((5.2, 24.1)),
        (3, "atk_") => Some((5.2, 24.1)),
        (3, "def_") => Some((6.6, 30.2)),
        (3, "eleMas") => Some((21.0, 94.0)),
        (3, "enerRech_") => Some((5.8, 26.8)),
        (3, "critRate_") => Some((3.5, 16.2)),
        (3, "critDMG_") => Some((7.0, 32.4)),
        (3, "physical_dmg_") => Some((6.6, 30.2)),
        (3, "pyro_dmg_")
        | (3, "hydro_dmg_")
        | (3, "electro_dmg_")
        | (3, "cryo_dmg_")
        | (3, "dendro_dmg_")
        | (3, "anemo_dmg_")
        | (3, "geo_dmg_") => Some((5.2, 24.1)),
        (3, "heal_") => Some((4.0, 18.6)),
        _ => None,
    }
}
