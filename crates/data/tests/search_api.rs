use genshin_calc_core::Element;
use genshin_calc_data::types::*;
use genshin_calc_data::*;

#[test]
fn find_character_by_id() {
    let diluc = find_character("diluc").expect("Diluc should exist");
    assert_eq!(diluc.name, "Diluc");
    assert_eq!(diluc.element, Element::Pyro);
    assert_eq!(diluc.weapon_type, WeaponType::Claymore);
}

#[test]
fn find_character_not_found() {
    assert!(find_character("nonexistent").is_none());
}

#[test]
fn find_weapon_by_id() {
    let wgs = find_weapon("wolfs_gravestone").expect("WGS should exist");
    assert_eq!(wgs.weapon_type, WeaponType::Claymore);
    assert_eq!(wgs.rarity, Rarity::Star5);
}

#[test]
fn find_artifact_set_by_id() {
    let cw = find_artifact_set("crimson_witch").expect("CW should exist");
    assert!(!cw.two_piece.buffs.is_empty());
}

#[test]
fn find_enemy_by_id() {
    let hilichurl = find_enemy("hilichurl").expect("Hilichurl should exist");
    assert_eq!(hilichurl.name, "ヒルチャール");
}

#[test]
fn filter_characters_by_element() {
    let pyro_chars = characters_by_element(Element::Pyro);
    assert!(pyro_chars.iter().all(|c| c.element == Element::Pyro));
    assert!(!pyro_chars.is_empty());
}

#[test]
fn filter_weapons_by_type() {
    let claymores = weapons_by_type(WeaponType::Claymore);
    assert!(
        claymores
            .iter()
            .all(|w| w.weapon_type == WeaponType::Claymore)
    );
    assert!(!claymores.is_empty());
}

#[test]
fn enemy_to_core_conversion() {
    let hilichurl = find_enemy("hilichurl").unwrap();
    let enemy = hilichurl.to_enemy(Some(Element::Pyro), 90, 0.0);
    assert_eq!(enemy.level, 90);
    assert!((enemy.resistance - 0.10).abs() < f64::EPSILON);
    assert!((enemy.def_reduction - 0.0).abs() < f64::EPSILON);
}

#[test]
fn resistance_template_physical() {
    let hilichurl = find_enemy("hilichurl").unwrap();
    let enemy = hilichurl.to_enemy(None, 85, 0.12);
    assert!((enemy.resistance - 0.10).abs() < f64::EPSILON);
    assert!((enemy.def_reduction - 0.12).abs() < f64::EPSILON);
}
