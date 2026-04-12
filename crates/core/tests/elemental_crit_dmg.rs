use genshin_calc_core::{
    BuffTarget, BuffableStat, DamageInput, DamageType, Element, Enemy, ResolvedBuff, ScalingStat,
    StatProfile, apply_buffs_to_profile, calculate_damage, combine_stats,
};

fn assert_close(actual: f64, expected: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= 1e-6,
        "{label}: expected {expected}, got {actual}"
    );
}

fn enemy() -> Enemy {
    Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
        def_ignore: 0.0,
    }
}

#[test]
fn elemental_crit_dmg_applies_only_to_matching_element() {
    let profile = StatProfile {
        base_atk: 2000.0,
        crit_rate: 1.0,
        crit_dmg: 0.50,
        energy_recharge: 1.0,
        ..Default::default()
    };
    let buffs = vec![
        ResolvedBuff {
            source: "generic_crit_dmg".to_string(),
            stat: BuffableStat::CritDmg,
            value: 0.20,
            target: BuffTarget::OnlySelf,
            origin: None,
        },
        ResolvedBuff {
            source: "geo_crit_dmg".to_string(),
            stat: BuffableStat::ElementalCritDmg(Element::Geo),
            value: 0.40,
            target: BuffTarget::OnlySelf,
            origin: None,
        },
    ];

    let buffed = apply_buffs_to_profile(&profile, &buffs);
    let stats = combine_stats(&buffed).unwrap();

    assert_close(stats.crit_dmg, 0.70, "generic crit dmg remains generic");
    assert_close(
        stats.geo_crit_dmg_bonus,
        0.40,
        "geo crit dmg stored separately",
    );

    let geo = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: stats.clone(),
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Geo),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &enemy(),
    )
    .unwrap();
    let pyro = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: stats.clone(),
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &enemy(),
    )
    .unwrap();
    let physical = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats,
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: None,
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &enemy(),
    )
    .unwrap();

    assert_close(
        geo.crit / geo.non_crit,
        2.10,
        "Geo gets generic + geo crit dmg",
    );
    assert_close(
        pyro.crit / pyro.non_crit,
        1.70,
        "Pyro gets generic crit dmg only",
    );
    assert_close(
        physical.crit / physical.non_crit,
        1.70,
        "Physical ignores geo-only crit dmg",
    );
}
