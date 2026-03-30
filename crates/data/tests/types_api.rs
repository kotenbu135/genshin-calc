use genshin_calc_core::{DamageType, Stats};

#[test]
fn test_talent_multiplier_zero_level_returns_none() {
    let char_data = genshin_calc_data::find_character("diluc").unwrap();
    assert_eq!(
        char_data.talent_multiplier(DamageType::Normal, 0, 0, 0),
        None
    );
}

#[test]
fn test_build_damage_input_zero_level_returns_none() {
    let char_data = genshin_calc_data::find_character("diluc").unwrap();
    let stats = Stats::default();
    assert_eq!(
        char_data.build_damage_input(stats, 90, DamageType::Normal, 0, 0, 0, None, 0.0),
        None,
    );
}
