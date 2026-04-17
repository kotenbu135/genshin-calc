use genshin_calc_core::{
    BuffableStat, DamageInput, DamageType, Element, Enemy, ScalingStat, StatProfile, Stats,
    TeamMember, WeaponType, calculate_damage, resolve_team_stats_detailed,
};
use genshin_calc_data::{TeamMemberBuilder, find_character, find_weapon};

fn assert_close(actual: f64, expected: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= 1e-6,
        "{label}: expected {expected}, got {actual}"
    );
}

fn dummy_member(element: Element) -> TeamMember {
    TeamMember {
        element,
        weapon_type: WeaponType::Sword,
        stats: StatProfile {
            base_hp: 10000.0,
            base_atk: 1000.0,
            base_def: 700.0,
            crit_rate: 1.0,
            crit_dmg: 0.50,
            energy_recharge: 1.0,
            ..Default::default()
        },
        buffs_provided: vec![],
        is_moonsign: false,
        can_nightsoul: false,
        moonsign_benediction: None,
    }
}

fn applied_buff<'a>(
    result: &'a genshin_calc_core::TeamResolveResult,
    source: &str,
    stat: BuffableStat,
) -> &'a genshin_calc_core::ResolvedBuff {
    result
        .applied_buffs
        .iter()
        .find(|buff| buff.source == source && buff.stat == stat)
        .unwrap_or_else(|| panic!("missing applied buff: {source:?} / {stat:?}"))
}

fn enemy() -> Enemy {
    Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
        def_ignore: 0.0,
    }
}

fn make_input(stats: Stats, element: Option<Element>) -> DamageInput {
    DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 1.0,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element,
        reaction: None,
        reaction_bonus: 0.0,
        flat_dmg: 0.0,
    }
}

#[test]
fn gorou_c6_applies_geo_crit_dmg_only() {
    let gorou = TeamMemberBuilder::new(
        find_character("gorou").unwrap(),
        find_weapon("favonius_warbow").unwrap(),
    )
    .constellation(6)
    .activate("gorou_c6_geo_crit_dmg")
    .build()
    .unwrap();

    let result = resolve_team_stats_detailed(&[gorou, dummy_member(Element::Geo)], 1, &[]).unwrap();
    let buff = applied_buff(
        &result,
        "gorou_c6_geo_crit_dmg",
        BuffableStat::ElementalCritDmg(Element::Geo),
    );
    assert_close(buff.value, 0.40, "Gorou C6 applied buff");
    assert_close(
        result.final_stats.crit_dmg,
        0.50,
        "Gorou C6 leaves generic crit dmg unchanged",
    );
    assert_close(
        result.final_stats.geo_crit_dmg_bonus,
        0.40,
        "Gorou C6 stores geo crit dmg separately",
    );

    let geo = calculate_damage(
        &make_input(result.final_stats.clone(), Some(Element::Geo)),
        &enemy(),
    )
    .unwrap();
    let pyro = calculate_damage(
        &make_input(result.final_stats, Some(Element::Pyro)),
        &enemy(),
    )
    .unwrap();

    assert_close(geo.crit / geo.non_crit, 1.90, "Geo gets Gorou C6 crit dmg");
    assert_close(
        pyro.crit / pyro.non_crit,
        1.50,
        "Pyro does not get Gorou C6 crit dmg",
    );
}

#[test]
fn shenhe_c2_applies_cryo_crit_dmg_only() {
    let shenhe = TeamMemberBuilder::new(
        find_character("shenhe").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .constellation(2)
    .activate("shenhe_c2_cryo_crit_dmg")
    .build()
    .unwrap();

    let result =
        resolve_team_stats_detailed(&[shenhe, dummy_member(Element::Cryo)], 1, &[]).unwrap();
    let buff = applied_buff(
        &result,
        "shenhe_c2_cryo_crit_dmg",
        BuffableStat::ElementalCritDmg(Element::Cryo),
    );
    assert_close(buff.value, 0.15, "Shenhe C2 applied buff");
    assert_close(
        result.final_stats.crit_dmg,
        0.50,
        "Shenhe C2 leaves generic crit dmg unchanged",
    );
    assert_close(
        result.final_stats.cryo_crit_dmg_bonus,
        0.15,
        "Shenhe C2 stores cryo crit dmg separately",
    );

    let cryo = calculate_damage(
        &make_input(result.final_stats.clone(), Some(Element::Cryo)),
        &enemy(),
    )
    .unwrap();
    let hydro = calculate_damage(
        &make_input(result.final_stats, Some(Element::Hydro)),
        &enemy(),
    )
    .unwrap();

    assert_close(
        cryo.crit / cryo.non_crit,
        1.65,
        "Cryo gets Shenhe C2 crit dmg",
    );
    assert_close(
        hydro.crit / hydro.non_crit,
        1.50,
        "Hydro does not get Shenhe C2 crit dmg",
    );
}
