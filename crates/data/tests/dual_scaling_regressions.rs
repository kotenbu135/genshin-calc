use genshin_calc_core::{
    BuffableStat, DamageType, Element, Enemy, StatProfile, calculate_damage,
    resolve_team_stats_detailed,
};
use genshin_calc_data::{TeamMemberBuilder, find_character, find_weapon};

fn assert_close(actual: f64, expected: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= 1e-6,
        "{label}: expected {expected}, got {actual}"
    );
}

fn default_enemy() -> Enemy {
    Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
        def_ignore: 0.0,
    }
}

#[test]
fn chiori_dual_scaling_uses_resolved_def_and_a4_buff_is_visible() {
    let chiori = find_character("chiori").unwrap();
    let member = TeamMemberBuilder::new(chiori, find_weapon("freedom_sworn").unwrap())
        .artifact_stats(StatProfile {
            def_percent: 0.50,
            def_flat: 100.0,
            geo_dmg_bonus: 0.466,
            ..Default::default()
        })
        .talent_levels([1, 10, 10])
        .activate("The Finishing Touch Geo DMG Bonus")
        .build()
        .unwrap();

    let team = [member];
    let result = resolve_team_stats_detailed(&team, 0, &[]).unwrap();

    assert!(
        result.applied_buffs.iter().any(|buff| {
            buff.source == "The Finishing Touch Geo DMG Bonus"
                && buff.stat == BuffableStat::ElementalDmgBonus(Element::Geo)
                && (buff.value - 0.20).abs() <= 1e-6
        }),
        "Chiori A4 Geo buff should appear in applied_buffs"
    );

    let expected_def = chiori.base_def_at_level(90) * 1.50 + 100.0;
    assert_close(result.final_stats.def, expected_def, "Chiori final DEF");
    assert_close(
        result.final_stats.geo_dmg_bonus,
        0.666,
        "Chiori final Geo DMG bonus",
    );

    let input = chiori
        .build_damage_input(
            result.final_stats.clone(),
            90,
            DamageType::Skill,
            0,
            10,
            0,
            None,
            0.0,
        )
        .unwrap();
    assert_close(
        input.flat_dmg,
        result.final_stats.def * 1.8468,
        "Chiori Tamoto DEF flat damage",
    );

    let mut atk_only = input.clone();
    atk_only.flat_dmg = 0.0;
    let with_def = calculate_damage(&input, &default_enemy()).unwrap();
    let without_def = calculate_damage(&atk_only, &default_enemy()).unwrap();
    assert!(
        with_def.non_crit > without_def.non_crit,
        "Chiori DEF component should increase skill damage"
    );
}

#[test]
fn dehya_dual_scaling_uses_resolved_hp_for_skill_and_burst() {
    let dehya = find_character("dehya").unwrap();
    let member = TeamMemberBuilder::new(dehya, find_weapon("wolfs_gravestone").unwrap())
        .artifact_stats(StatProfile {
            hp_percent: 0.40,
            hp_flat: 1000.0,
            pyro_dmg_bonus: 0.466,
            ..Default::default()
        })
        .talent_levels([1, 10, 10])
        .build()
        .unwrap();

    let team = [member];
    let result = resolve_team_stats_detailed(&team, 0, &[]).unwrap();

    let expected_hp = dehya.base_hp_at_level(90) * 1.688 + 1000.0;
    assert_close(result.final_stats.hp, expected_hp, "Dehya final HP");

    let field = dehya
        .build_damage_input(
            result.final_stats.clone(),
            90,
            DamageType::Skill,
            2,
            10,
            0,
            None,
            0.0,
        )
        .unwrap();
    assert_close(
        field.flat_dmg,
        result.final_stats.hp * 0.0186,
        "Dehya field HP flat damage",
    );

    let fist = dehya
        .build_damage_input(
            result.final_stats.clone(),
            90,
            DamageType::Burst,
            0,
            10,
            0,
            None,
            0.0,
        )
        .unwrap();
    assert_close(
        fist.flat_dmg,
        result.final_stats.hp * 0.0305,
        "Dehya fist HP flat damage",
    );

    let kick = dehya
        .build_damage_input(
            result.final_stats,
            90,
            DamageType::Burst,
            1,
            10,
            0,
            None,
            0.0,
        )
        .unwrap();
    assert_close(
        kick.flat_dmg,
        expected_hp * 0.0430,
        "Dehya kick HP flat damage",
    );
}
