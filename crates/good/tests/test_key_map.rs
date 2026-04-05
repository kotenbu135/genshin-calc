use genshin_calc_good::key_map;

#[test]
fn pascal_to_snake_basic() {
    assert_eq!(key_map::pascal_to_snake("HuTao"), "hu_tao");
    assert_eq!(key_map::pascal_to_snake("KamisatoAyaka"), "kamisato_ayaka");
    assert_eq!(key_map::pascal_to_snake("Xiangling"), "xiangling");
    assert_eq!(key_map::pascal_to_snake("TravelerAnemo"), "traveler_anemo");
}

#[test]
fn pascal_to_snake_with_special() {
    assert_eq!(
        key_map::pascal_to_snake("WolfsGravestone"),
        "wolfs_gravestone"
    );
    assert_eq!(key_map::pascal_to_snake("StaffOfHoma"), "staff_of_homa");
    assert_eq!(
        key_map::pascal_to_snake("CrimsonWitchOfFlames"),
        "crimson_witch_of_flames"
    );
}

#[test]
fn lookup_character() {
    let result = key_map::lookup_character("HuTao");
    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "hu_tao");
}

#[test]
fn lookup_character_unknown() {
    assert!(key_map::lookup_character("NonExistentCharacter").is_none());
}

#[test]
fn lookup_character_alias_kazuha() {
    let result = key_map::lookup_character("KaedeharaKazuha");
    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "kazuha");
}

#[test]
fn lookup_character_alias_heizou() {
    let result = key_map::lookup_character("ShikanoinHeizou");
    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "heizou");
}

#[test]
fn lookup_character_alias_mizuki() {
    let result = key_map::lookup_character("YumemizukiMizuki");
    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "mizuki");
}

#[test]
fn lookup_weapon() {
    let result = key_map::lookup_weapon("StaffOfHoma");
    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "staff_of_homa");
}

#[test]
fn lookup_artifact_set_direct() {
    let result = key_map::lookup_artifact_set("GladiatorsFinale");
    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "gladiators_finale");
}

#[test]
fn lookup_artifact_set_alias() {
    let result = key_map::lookup_artifact_set("CrimsonWitchOfFlames");
    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "crimson_witch");
}

#[test]
fn lookup_artifact_set_the_exile() {
    let result = key_map::lookup_artifact_set("TheExile");
    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "exile");
}

#[test]
fn all_data_crate_artifact_sets_are_reachable() {
    use genshin_calc_data::artifacts::ALL_ARTIFACT_SETS;
    for set in ALL_ARTIFACT_SETS {
        let found = genshin_calc_data::find_artifact_set(set.id);
        assert!(
            found.is_some(),
            "artifact set '{}' not findable by id",
            set.id
        );
    }
}
