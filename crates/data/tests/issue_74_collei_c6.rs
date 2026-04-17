use genshin_calc_core::{
    BuffableStat, Element, StatProfile, TeamMember, WeaponType, resolve_team_stats_detailed,
};
use genshin_calc_data::{TeamMemberBuilder, find_character, find_talent_buffs, find_weapon};

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
            base_atk: 500.0,
            base_def: 700.0,
            crit_rate: 0.05,
            crit_dmg: 0.50,
            energy_recharge: 1.0,
            ..Default::default()
        },
        buffs_provided: vec![],
        is_moonsign: false,
        can_nightsoul: false,
        moonsign_benediction: None,
        moonsign_talent_enhancements: &[],
    }
}

#[test]
fn collei_c6_does_not_register_a_fake_dendro_damage_bonus() {
    let defs = find_talent_buffs("collei").expect("Collei talent buffs should exist");
    assert!(
        defs.iter().all(|buff| buff.name != "collei_c6_dendro_dmg"),
        "Collei C6 should not remain in talent_buffs as a fake Dendro DMG bonus",
    );

    let off = TeamMemberBuilder::new(
        find_character("collei").unwrap(),
        find_weapon("favonius_warbow").unwrap(),
    )
    .constellation(6)
    .build()
    .unwrap();
    let on = TeamMemberBuilder::new(
        find_character("collei").unwrap(),
        find_weapon("favonius_warbow").unwrap(),
    )
    .constellation(6)
    .activate("collei_c6_dendro_dmg")
    .build()
    .unwrap();

    let off = resolve_team_stats_detailed(&[off, dummy_member(Element::Dendro)], 1, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[on, dummy_member(Element::Dendro)], 1, &[]).unwrap();

    assert!(
        on.applied_buffs
            .iter()
            .all(|buff| buff.stat != BuffableStat::ElementalDmgBonus(Element::Dendro)),
        "Collei C6 should not add a Dendro DMG Bonus buff",
    );
    assert_close(
        on.final_stats.dendro_dmg_bonus - off.final_stats.dendro_dmg_bonus,
        0.0,
        "Collei C6 should leave final Dendro DMG bonus unchanged",
    );
}
