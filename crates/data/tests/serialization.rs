use genshin_calc_core::Element;
use genshin_calc_data::buff::*;
use genshin_calc_data::types::*;

const EPSILON: f64 = 1e-10;

// Roundtrip tests for Deserialize-capable types
#[test]
fn weapon_type_roundtrip() {
    let original = WeaponType::Claymore;
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: WeaponType = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn rarity_roundtrip() {
    let original = Rarity::Star5;
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Rarity = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn region_roundtrip() {
    let original = Region::Mondstadt;
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Region = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn ascension_stat_roundtrip() {
    let original = AscensionStat::CritRate(0.192);
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: AscensionStat = serde_json::from_str(&json).unwrap();
    match (original, deserialized) {
        (AscensionStat::CritRate(a), AscensionStat::CritRate(b)) => {
            assert!((a - b).abs() < EPSILON);
        }
        _ => panic!("variant mismatch"),
    }
}

#[test]
fn ascension_stat_elemental_roundtrip() {
    let original = AscensionStat::ElementalDmgBonus(Element::Pyro, 0.288);
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: AscensionStat = serde_json::from_str(&json).unwrap();
    match (original, deserialized) {
        (AscensionStat::ElementalDmgBonus(e1, a), AscensionStat::ElementalDmgBonus(e2, b)) => {
            assert_eq!(e1, e2);
            assert!((a - b).abs() < EPSILON);
        }
        _ => panic!("variant mismatch"),
    }
}

#[test]
fn stat_buff_roundtrip() {
    let original = StatBuff {
        stat: BuffableStat::AtkPercent,
        value: 0.20,
        refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: StatBuff = serde_json::from_str(&json).unwrap();
    assert_eq!(original.stat, deserialized.stat);
    assert!((original.value - deserialized.value).abs() < EPSILON);
    let orig_rv = original.refinement_values.unwrap();
    let deser_rv = deserialized.refinement_values.unwrap();
    for (a, b) in orig_rv.iter().zip(deser_rv.iter()) {
        assert!((a - b).abs() < EPSILON);
    }
}

#[test]
fn weapon_sub_stat_roundtrip() {
    let original = WeaponSubStat::AtkPercent([0.108, 0.406, 0.444, 0.496]);
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: WeaponSubStat = serde_json::from_str(&json).unwrap();
    match (original, deserialized) {
        (WeaponSubStat::AtkPercent(a), WeaponSubStat::AtkPercent(b)) => {
            for (x, y) in a.iter().zip(b.iter()) {
                assert!((x - y).abs() < EPSILON);
            }
        }
        _ => panic!("variant mismatch"),
    }
}

#[test]
fn resistance_template_serializes() {
    let original = genshin_calc_data::enemies::ALL_10;
    let json = serde_json::to_string(&original).unwrap();
    assert!(json.contains("\"pyro\":0.1"));
    assert!(json.contains("\"physical\":0.1"));
}

// Serialize-only tests for &'static ref types
#[test]
fn character_data_serializes() {
    let diluc = genshin_calc_data::find_character("diluc").unwrap();
    let json = serde_json::to_string(diluc).unwrap();
    assert!(json.contains("\"id\":\"diluc\""));
    assert!(json.contains("\"name\":\"Diluc\""));
}

#[test]
fn weapon_data_serializes() {
    let wgs = genshin_calc_data::find_weapon("wolfs_gravestone").unwrap();
    let json = serde_json::to_string(wgs).unwrap();
    assert!(json.contains("\"id\":\"wolfs_gravestone\""));
}

#[test]
fn artifact_set_serializes() {
    let cw = genshin_calc_data::find_artifact_set("crimson_witch").unwrap();
    let json = serde_json::to_string(cw).unwrap();
    assert!(json.contains("\"id\":\"crimson_witch\""));
}
