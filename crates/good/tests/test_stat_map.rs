use genshin_calc_good::stat_map;

const EPS: f64 = 1e-4;

#[test]
fn flat_stats_no_conversion() {
    use stat_map::StatConvertResult::Converted;

    let Converted(field, value) = stat_map::convert_stat("hp", 4780.0) else {
        panic!()
    };
    assert_eq!(field, stat_map::StatField::HpFlat);
    assert!((value - 4780.0).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("atk", 311.0) else {
        panic!()
    };
    assert_eq!(field, stat_map::StatField::AtkFlat);
    assert!((value - 311.0).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("def", 37.0) else {
        panic!()
    };
    assert_eq!(field, stat_map::StatField::DefFlat);
    assert!((value - 37.0).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("eleMas", 187.0) else {
        panic!()
    };
    assert_eq!(field, stat_map::StatField::ElementalMastery);
    assert!((value - 187.0).abs() < EPS);
}

#[test]
fn percent_stats_divide_by_100() {
    use stat_map::StatConvertResult::Converted;

    let Converted(field, value) = stat_map::convert_stat("hp_", 46.6) else {
        panic!()
    };
    assert_eq!(field, stat_map::StatField::HpPercent);
    assert!((value - 0.466).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("critRate_", 31.1) else {
        panic!()
    };
    assert_eq!(field, stat_map::StatField::CritRate);
    assert!((value - 0.311).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("critDMG_", 62.2) else {
        panic!()
    };
    assert_eq!(field, stat_map::StatField::CritDmg);
    assert!((value - 0.622).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("enerRech_", 51.8) else {
        panic!()
    };
    assert_eq!(field, stat_map::StatField::EnergyRecharge);
    assert!((value - 0.518).abs() < EPS);
}

#[test]
fn elemental_dmg_bonus() {
    use stat_map::StatConvertResult::Converted;

    let Converted(field, value) = stat_map::convert_stat("pyro_dmg_", 46.6) else {
        panic!()
    };
    assert_eq!(field, stat_map::StatField::ElementalDmgBonus("pyro"));
    assert!((value - 0.466).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("physical_dmg_", 58.3) else {
        panic!()
    };
    assert_eq!(field, stat_map::StatField::PhysicalDmgBonus);
    assert!((value - 0.583).abs() < EPS);
}

#[test]
fn healing_bonus_returns_skip() {
    assert_eq!(
        stat_map::convert_stat("heal_", 35.9),
        stat_map::StatConvertResult::Skip
    );
}

#[test]
fn unknown_stat_key() {
    assert_eq!(
        stat_map::convert_stat("unknown_stat", 10.0),
        stat_map::StatConvertResult::Unknown
    );
}

#[test]
fn star5_flower_lv20() {
    let value = stat_map::main_stat_value(5, "hp", 20).unwrap();
    assert!((value - 4780.0).abs() < 1.0);
}

#[test]
fn star5_plume_lv20() {
    let value = stat_map::main_stat_value(5, "atk", 20).unwrap();
    assert!((value - 311.0).abs() < 1.0);
}

#[test]
fn star5_hp_percent_lv20() {
    let value = stat_map::main_stat_value(5, "hp_", 20).unwrap();
    assert!((value - 46.6).abs() < 0.1);
}

#[test]
fn star5_crit_rate_lv0() {
    let value = stat_map::main_stat_value(5, "critRate_", 0).unwrap();
    assert!((value - 4.7).abs() < 0.1);
}

#[test]
fn star5_crit_dmg_lv20() {
    let value = stat_map::main_stat_value(5, "critDMG_", 20).unwrap();
    assert!((value - 62.2).abs() < 0.1);
}

#[test]
fn star4_atk_percent_lv16() {
    let value = stat_map::main_stat_value(4, "atk_", 16).unwrap();
    assert!((value - 34.8).abs() < 0.1);
}

#[test]
fn star3_hp_flat_lv12() {
    let value = stat_map::main_stat_value(3, "hp", 12).unwrap();
    assert!((value - 1893.0).abs() < 1.0);
}

#[test]
fn star5_atk_percent_lv10_intermediate() {
    let value = stat_map::main_stat_value(5, "atk_", 10).unwrap();
    assert!((value - 26.8).abs() < 1.0);
}

#[test]
fn invalid_rarity_returns_none() {
    assert!(stat_map::main_stat_value(2, "hp", 0).is_none());
}
