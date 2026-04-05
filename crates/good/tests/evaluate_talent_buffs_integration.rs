//! Integration tests for evaluate_talent_buffs with realistic GOOD data.

use genshin_calc_core::{BuffTarget, BuffableStat};
use genshin_calc_good::{evaluate_talent_buffs, import_good};

fn good_json_with(character_json: &str, weapon_json: &str) -> String {
    format!(
        r#"{{"format":"GOOD","version":3,"source":"Test","characters":[{}],"weapons":[{}],"artifacts":[]}}"#,
        character_json, weapon_json
    )
}

#[test]
fn test_bennett_burst_atk_scaling() {
    let json = good_json_with(
        r#"{"key":"Bennett","level":90,"constellation":0,"ascension":6,"talent":{"auto":1,"skill":1,"burst":13}}"#,
        r#"{"key":"AquilaFavonia","level":90,"ascension":6,"refinement":1,"location":"Bennett","lock":false}"#,
    );
    let import = import_good(&json).unwrap();
    let build = &import.builds[0];
    let buffs = evaluate_talent_buffs(build, 0, &[1, 1, 13], &[]);

    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
    assert_eq!(buffs[0].target, BuffTarget::Team);
    assert!(buffs[0].source.contains("bennett"));
    assert!(buffs[0].source.contains("burst"));
    assert!(
        buffs[0].value > 500.0,
        "Bennett burst ATK buff should be significant"
    );
    assert!(
        buffs[0].value < 1500.0,
        "Bennett burst ATK buff should be reasonable"
    );
}

#[test]
fn test_build_member_stats_matches_stat_profile() {
    let json = good_json_with(
        r#"{"key":"Bennett","level":90,"constellation":6,"ascension":6,"talent":{"auto":1,"skill":1,"burst":13}}"#,
        r#"{"key":"AquilaFavonia","level":90,"ascension":6,"refinement":1,"location":"Bennett","lock":false}"#,
    );
    let import = import_good(&json).unwrap();
    let build = &import.builds[0];
    let profile = genshin_calc_good::build_stat_profile(build);

    assert!(profile.base_atk > 500.0, "base_atk should include weapon");
    assert!((profile.crit_rate - 0.05).abs() < 1e-6);
    assert!((profile.crit_dmg - 0.50).abs() < 1e-6);
    assert!(
        (profile.energy_recharge - 1.0).abs() > 0.01,
        "Should include ascension ER bonus"
    );
}

#[test]
fn test_pipeline_build_member_stats_to_resolve_team() {
    let json = good_json_with(
        r#"{"key":"Bennett","level":90,"constellation":0,"ascension":6,"talent":{"auto":1,"skill":1,"burst":13}}"#,
        r#"{"key":"AquilaFavonia","level":90,"ascension":6,"refinement":1,"location":"Bennett","lock":false}"#,
    );
    let import = import_good(&json).unwrap();
    let build = &import.builds[0];

    let profile = genshin_calc_good::build_stat_profile(build);
    let base_atk = profile.base_atk;
    let buffs = evaluate_talent_buffs(build, 0, &[1, 1, 13], &[]);

    let member = genshin_calc_core::TeamMember {
        element: build.character.element,
        weapon_type: build.character.weapon_type,
        stats: profile,
        buffs_provided: buffs,
        is_moonsign: false,
    };

    let result = genshin_calc_core::resolve_team_stats(&[member], 0);
    assert!(result.is_ok());
    let resolved = result.unwrap();
    assert!(resolved.final_stats.atk >= base_atk);
}
