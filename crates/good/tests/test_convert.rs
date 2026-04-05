use genshin_calc_good::{
    ImportOptions, import_good, import_good_with_options, to_team_member_builder,
};

#[test]
fn import_minimal_hu_tao() {
    let json = include_str!("data/minimal.json");
    let result = import_good(json).unwrap();

    assert_eq!(result.source, "TestScanner");
    assert_eq!(result.version, 2);
    assert_eq!(result.builds.len(), 1);
    assert!(
        result.warnings.is_empty(),
        "unexpected warnings: {:?}",
        result.warnings
    );

    let build = &result.builds[0];
    assert_eq!(build.character.id, "hu_tao");
    assert_eq!(build.level, 90);
    assert_eq!(build.constellation, 1);
    assert_eq!(build.talent_levels, [10, 10, 8]);

    let weapon = build.weapon.as_ref().unwrap();
    assert_eq!(weapon.weapon.id, "staff_of_homa");
    assert_eq!(weapon.level, 90);
    assert_eq!(weapon.refinement, 1);

    // 5x CrimsonWitch → 4pc set
    assert_eq!(build.artifacts.sets.len(), 1);
    assert_eq!(build.artifacts.sets[0].set.id, "crimson_witch");
    assert_eq!(build.artifacts.sets[0].piece_count, 4);
}

#[test]
fn artifact_stats_aggregation() {
    let json = include_str!("data/minimal.json");
    let result = import_good(json).unwrap();
    let stats = &result.builds[0].artifacts.stats;

    // Flower main: hp flat 4780
    assert!((stats.hp_flat - 4780.0).abs() < 1.0);
    // Plume main: atk flat 311 + goblet sub atk flat 33
    assert!((stats.atk_flat - 344.0).abs() < 1.0);
    // hp_percent: sands main(0.466) + plume sub(0.099) + goblet sub(0.140) + circlet sub(0.117)
    assert!((stats.hp_percent - 0.822).abs() < 0.01);
    // crit_rate subs: flower(0.105) + sands(0.062) + goblet(0.035) + circlet(0.074)
    assert!((stats.crit_rate - 0.276).abs() < 0.01);
    // crit_dmg: circlet main(0.622) + flower sub(0.210) + plume sub(0.148) + sands sub(0.132) + goblet sub(0.078)
    assert!((stats.crit_dmg - 1.19).abs() < 0.01);
    // pyro_dmg_bonus: goblet main pyro 0.466
    assert!((stats.pyro_dmg_bonus - 0.466).abs() < 0.01);
    // def_flat: sands sub(37)
    assert!((stats.def_flat - 37.0).abs() < 1.0);
}

#[test]
fn two_piece_two_piece_sets() {
    let json = include_str!("data/two_piece_two_piece.json");
    let result = import_good(json).unwrap();
    let build = &result.builds[0];

    // Emblem 3pc + Gladiator 2pc → both qualify for 2pc
    assert_eq!(build.artifacts.sets.len(), 2);
    for entry in &build.artifacts.sets {
        assert_eq!(entry.piece_count, 2, "2pc sets should have piece_count=2");
    }

    let set_ids: Vec<&str> = build.artifacts.sets.iter().map(|s| s.set.id).collect();
    assert!(set_ids.contains(&"emblem_of_severed_fate"));
    assert!(set_ids.contains(&"gladiators_finale"));

    // Convert to TeamMember and verify 2pc buffs are in buffs_provided
    let member = to_team_member_builder(build, &[], &[])
        .unwrap()
        .build()
        .unwrap();

    // Debug: print all buffs
    eprintln!("=== buffs_provided ===");
    for b in &member.buffs_provided {
        eprintln!("  source={}, value={}", b.source, b.value);
    }

    // Emblem 2pc: ER+20% (0.20)
    // Gladiator 2pc: ATK+18% (0.18)
    let has_emblem_2pc = member
        .buffs_provided
        .iter()
        .any(|b| b.source.contains("絶縁の旗印") && b.source.contains("2pc"));
    let has_glad_2pc = member
        .buffs_provided
        .iter()
        .any(|b| b.source.contains("剣闘士のフィナーレ") && b.source.contains("2pc"));
    assert!(
        has_emblem_2pc,
        "should have Emblem 2pc buff in buffs_provided"
    );
    assert!(
        has_glad_2pc,
        "should have Gladiator 2pc buff in buffs_provided"
    );

    // artifact stats should NOT contain 2pc buffs (only main/substats)
    // enerRech_ subs: 5+10+5+5 = 25% → 0.25
    // sands main enerRech_: 51.8% → 0.518
    // total = 0.25 + 0.518 = 0.768
    assert!((build.artifacts.stats.energy_recharge - 0.768).abs() < 0.02);

    // atk_percent subs: 5+5+5+5 = 20% → 0.20
    assert!((build.artifacts.stats.atk_percent - 0.20).abs() < 0.01);
}

#[test]
fn unknown_character_becomes_warning() {
    let json = r#"{
        "format": "GOOD", "source": "Test", "version": 1,
        "characters": [
            { "key": "FutureCharacter", "level": 90, "constellation": 0, "ascension": 6,
              "talent": { "auto": 1, "skill": 1, "burst": 1 } }
        ]
    }"#;
    let result = import_good(json).unwrap();
    assert_eq!(result.builds.len(), 0);
    assert_eq!(result.warnings.len(), 1);
}

#[test]
fn empty_good_returns_empty_builds() {
    let json = r#"{ "format": "GOOD", "source": "Empty", "version": 1 }"#;
    let result = import_good(json).unwrap();
    assert!(result.builds.is_empty());
    assert!(result.warnings.is_empty());
}

#[test]
fn partial_build_no_weapon() {
    let json = r#"{
        "format": "GOOD", "source": "Test", "version": 1,
        "characters": [
            { "key": "Xiangling", "level": 80, "constellation": 0, "ascension": 5,
              "talent": { "auto": 1, "skill": 1, "burst": 1 } }
        ]
    }"#;
    let result = import_good(json).unwrap();
    assert_eq!(result.builds.len(), 1);
    assert!(result.builds[0].weapon.is_none());
    assert!(result.builds[0].artifacts.sets.is_empty());
}

#[test]
fn off_element_goblet_stored_in_per_element_field() {
    // Xiangling is Pyro but equipped with a hydro goblet.
    // The hydro DMG bonus should be stored in hydro_dmg_bonus (not filtered out),
    // and no ElementMismatchGoblet warning should be emitted.
    let json = r#"{
        "format": "GOOD", "source": "Test", "version": 1,
        "characters": [
            { "key": "Xiangling", "level": 90, "constellation": 0, "ascension": 6,
              "talent": { "auto": 1, "skill": 1, "burst": 1 } }
        ],
        "artifacts": [
            {
                "setKey": "GladiatorsFinale", "slotKey": "goblet",
                "level": 20, "rarity": 5, "mainStatKey": "hydro_dmg_",
                "location": "Xiangling", "lock": false, "substats": []
            }
        ]
    }"#;
    let result = import_good(json).unwrap();
    assert!(
        result.warnings.is_empty(),
        "unexpected warnings: {:?}",
        result.warnings
    );
    // hydro DMG stored in its own field (~0.466 at lv20 5★)
    assert!((result.builds[0].artifacts.stats.hydro_dmg_bonus - 0.466).abs() < 0.01);
    // dmg_bonus (generic) unaffected
    assert!((result.builds[0].artifacts.stats.dmg_bonus).abs() < 0.001);
}

#[test]
fn traveler_dendro_with_options() {
    let json = r#"{
        "format": "GOOD", "source": "Test", "version": 1,
        "characters": [
            { "key": "Traveler", "level": 90, "constellation": 0, "ascension": 6,
              "talent": { "auto": 1, "skill": 1, "burst": 1 } }
        ]
    }"#;
    let options = ImportOptions {
        traveler_element: Some(genshin_calc_core::Element::Dendro),
    };
    let result = import_good_with_options(json, &options).unwrap();
    assert_eq!(result.builds.len(), 1);
    assert_eq!(result.builds[0].character.id, "traveler_dendro");
    assert!(result.warnings.is_empty());
}

#[test]
fn traveler_without_options_becomes_warning() {
    let json = r#"{
        "format": "GOOD", "source": "Test", "version": 1,
        "characters": [
            { "key": "Traveler", "level": 90, "constellation": 0, "ascension": 6,
              "talent": { "auto": 1, "skill": 1, "burst": 1 } }
        ]
    }"#;
    let result = import_good(json).unwrap();
    assert_eq!(result.builds.len(), 0);
    assert_eq!(result.warnings.len(), 1);
}

#[test]
fn traveler_unimplemented_element_becomes_warning() {
    let json = r#"{
        "format": "GOOD", "source": "Test", "version": 1,
        "characters": [
            { "key": "Traveler", "level": 90, "constellation": 0, "ascension": 6,
              "talent": { "auto": 1, "skill": 1, "burst": 1 } }
        ]
    }"#;
    let options = ImportOptions {
        traveler_element: Some(genshin_calc_core::Element::Pyro),
    };
    let result = import_good_with_options(json, &options).unwrap();
    // traveler_pyro is not in data crate → UnknownCharacter warning
    assert_eq!(result.builds.len(), 0);
    assert_eq!(result.warnings.len(), 1);
}
