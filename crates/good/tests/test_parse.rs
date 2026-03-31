use genshin_calc_good::GoodFormat;

#[test]
fn parse_minimal_good_json() {
    let json = include_str!("data/minimal.json");
    let good: GoodFormat = serde_json::from_str(json).unwrap();

    assert_eq!(good.format, "GOOD");
    assert_eq!(good.source, "TestScanner");
    assert_eq!(good.version, 2);

    let chars = good.characters.as_ref().unwrap();
    assert_eq!(chars.len(), 1);
    assert_eq!(chars[0].key, "HuTao");
    assert_eq!(chars[0].level, 90);
    assert_eq!(chars[0].constellation, 1);
    assert_eq!(chars[0].ascension, 6);
    assert_eq!(chars[0].talent.auto, 10);
    assert_eq!(chars[0].talent.skill, 10);
    assert_eq!(chars[0].talent.burst, 8);

    let weapons = good.weapons.as_ref().unwrap();
    assert_eq!(weapons.len(), 1);
    assert_eq!(weapons[0].key, "StaffOfHoma");
    assert_eq!(weapons[0].level, 90);
    assert_eq!(weapons[0].refinement, 1);
    assert_eq!(weapons[0].location.as_deref(), Some("HuTao"));

    let artifacts = good.artifacts.as_ref().unwrap();
    assert_eq!(artifacts.len(), 5);
    assert_eq!(artifacts[0].set_key, "CrimsonWitchOfFlames");
    assert_eq!(artifacts[0].slot_key, "flower");
    assert_eq!(artifacts[0].level, 20);
    assert_eq!(artifacts[0].rarity, 5);
    assert_eq!(artifacts[0].main_stat_key, "hp");
    assert_eq!(artifacts[0].substats.len(), 4);
    assert_eq!(artifacts[0].substats[0].key, "critRate_");
    assert!((artifacts[0].substats[0].value - 10.5).abs() < 1e-6);
}

#[test]
fn parse_empty_fields() {
    let json = r#"{ "format": "GOOD", "source": "Empty", "version": 1 }"#;
    let good: GoodFormat = serde_json::from_str(json).unwrap();
    assert!(good.characters.is_none());
    assert!(good.weapons.is_none());
    assert!(good.artifacts.is_none());
}

#[test]
fn parse_empty_location_is_unequipped() {
    let json = r#"{
        "format": "GOOD", "source": "Test", "version": 1,
        "weapons": [{
            "key": "DullBlade", "level": 1, "ascension": 0,
            "refinement": 1, "location": "", "lock": false
        }]
    }"#;
    let good: GoodFormat = serde_json::from_str(json).unwrap();
    let weapons = good.weapons.unwrap();
    assert_eq!(weapons[0].location.as_deref(), Some(""));
}
