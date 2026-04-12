use genshin_calc_core::{
    BuffableStat, Element, Reaction, ScalingStat, StatProfile, TeamMember, WeaponType,
    resolve_team_stats_detailed,
};
use genshin_calc_data::{
    TalentBuffSource, TeamMemberBuilder, find_character, find_talent_buffs, find_weapon,
};

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
    }
}

fn total_atk(profile: &StatProfile) -> f64 {
    profile.base_atk * (1.0 + profile.atk_percent) + profile.atk_flat
}

#[test]
fn xiangling_c1_c6_only_apply_when_activated_and_never_double_count() {
    let off = TeamMemberBuilder::new(
        find_character("xiangling").unwrap(),
        find_weapon("the_catch").unwrap(),
    )
    .constellation(6)
    .build()
    .unwrap();
    let on = TeamMemberBuilder::new(
        find_character("xiangling").unwrap(),
        find_weapon("the_catch").unwrap(),
    )
    .constellation(6)
    .activate("xiangling_c1_pyro_res_shred")
    .activate("xiangling_c6_pyro_dmg")
    .build()
    .unwrap();

    let off = resolve_team_stats_detailed(&[off, dummy_member(Element::Pyro)], 1, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[on, dummy_member(Element::Pyro)], 1, &[]).unwrap();

    assert_close(
        off.enemy_debuffs.pyro_res_reduction,
        0.0,
        "Xiangling C1 is inactive by default",
    );
    assert_close(
        on.enemy_debuffs.pyro_res_reduction,
        0.15,
        "Xiangling C1 applies exactly once",
    );
    assert_close(
        on.final_stats.pyro_dmg_bonus - off.final_stats.pyro_dmg_bonus,
        0.15,
        "Xiangling C6 applies exactly once",
    );
    assert_eq!(
        on.applied_buffs
            .iter()
            .filter(|buff| buff.stat == BuffableStat::ElementalResReduction(Element::Pyro))
            .count(),
        1,
        "Xiangling C1 should resolve to one Pyro RES shred entry",
    );
    assert_eq!(
        on.applied_buffs
            .iter()
            .filter(|buff| buff.stat == BuffableStat::ElementalDmgBonus(Element::Pyro))
            .count(),
        1,
        "Xiangling C6 should resolve to one Pyro DMG entry",
    );
}

#[test]
fn kirara_c6_uses_element_specific_damage_bonuses_only() {
    let defs = find_talent_buffs("kirara").expect("Kirara talent buffs should exist");
    let c6: Vec<_> = defs
        .iter()
        .filter(|buff| buff.source == TalentBuffSource::Constellation(6))
        .collect();

    assert_eq!(
        c6.len(),
        7,
        "Kirara C6 should fan out to all seven elements"
    );
    assert!(c6.iter().all(|buff| {
        matches!(
            buff.stat,
            BuffableStat::ElementalDmgBonus(Element::Pyro)
                | BuffableStat::ElementalDmgBonus(Element::Hydro)
                | BuffableStat::ElementalDmgBonus(Element::Electro)
                | BuffableStat::ElementalDmgBonus(Element::Cryo)
                | BuffableStat::ElementalDmgBonus(Element::Dendro)
                | BuffableStat::ElementalDmgBonus(Element::Anemo)
                | BuffableStat::ElementalDmgBonus(Element::Geo)
        )
    }));
    assert!(c6.iter().all(|buff| buff.stat != BuffableStat::DmgBonus));

    let off = TeamMemberBuilder::new(
        find_character("kirara").unwrap(),
        find_weapon("favonius_sword").unwrap(),
    )
    .constellation(6)
    .build()
    .unwrap();
    let on = TeamMemberBuilder::new(
        find_character("kirara").unwrap(),
        find_weapon("favonius_sword").unwrap(),
    )
    .constellation(6)
    .activate("kirara_c6_elemental_dmg_bonus")
    .build()
    .unwrap();

    let off = resolve_team_stats_detailed(&[off, dummy_member(Element::Pyro)], 1, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[on, dummy_member(Element::Pyro)], 1, &[]).unwrap();

    assert_close(
        on.final_stats.pyro_dmg_bonus - off.final_stats.pyro_dmg_bonus,
        0.12,
        "Kirara C6 should grant Pyro DMG",
    );
    assert_close(
        on.final_stats.dmg_bonus - off.final_stats.dmg_bonus,
        0.0,
        "Kirara C6 should not leak into generic DMG bonus",
    );
}

#[test]
fn nefer_c6_uses_exact_lunar_bloom_reaction_bonus() {
    let defs = find_talent_buffs("nefer").expect("Nefer talent buffs should exist");
    let c6 = defs
        .iter()
        .find(|buff| buff.name == "nefer_c6_lunar_bloom_dmg")
        .expect("Nefer C6 buff should exist");
    assert_eq!(
        c6.stat,
        BuffableStat::ReactionDmgBonus(Reaction::LunarBloom)
    );

    let off = TeamMemberBuilder::new(
        find_character("nefer").unwrap(),
        find_weapon("favonius_codex").unwrap(),
    )
    .constellation(6)
    .build()
    .unwrap();
    let on = TeamMemberBuilder::new(
        find_character("nefer").unwrap(),
        find_weapon("favonius_codex").unwrap(),
    )
    .constellation(6)
    .activate("nefer_c6_lunar_bloom_dmg")
    .build()
    .unwrap();

    let off = resolve_team_stats_detailed(&[off], 0, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[on], 0, &[]).unwrap();

    assert_close(
        on.damage_context.reaction_bonus_for(Reaction::LunarBloom)
            - off.damage_context.reaction_bonus_for(Reaction::LunarBloom),
        0.15,
        "Nefer C6 should only buff Lunar-Bloom",
    );
    assert_close(
        on.damage_context.transformative_bonus - off.damage_context.transformative_bonus,
        0.0,
        "Nefer C6 should not use broad transformative bonus",
    );
}

#[test]
fn xianyun_a4_is_flat_damage_and_a1_is_marked_as_approximation() {
    let defs = find_talent_buffs("xianyun").expect("Xianyun talent buffs should exist");
    let a1 = defs
        .iter()
        .find(|buff| buff.source == TalentBuffSource::AscensionPassive(1))
        .expect("Xianyun A1 buff should exist");
    assert_eq!(a1.stat, BuffableStat::CritRate);
    assert!(
        a1.description.contains("approximation"),
        "Xianyun A1 should be labeled as a design approximation",
    );

    let a4 = defs
        .iter()
        .find(|buff| buff.source == TalentBuffSource::AscensionPassive(4))
        .expect("Xianyun A4 buff should exist");
    assert_eq!(a4.stat, BuffableStat::PlungingAtkFlatDmg);
    assert_eq!(a4.scales_on, Some(ScalingStat::TotalAtk));
    assert_close(a4.base_value, 2.0, "Xianyun A4 coefficient");
    assert_eq!(a4.cap, Some(9000.0));

    let off = TeamMemberBuilder::new(
        find_character("xianyun").unwrap(),
        find_weapon("favonius_codex").unwrap(),
    )
    .artifact_stats(StatProfile {
        atk_percent: 0.50,
        atk_flat: 100.0,
        ..Default::default()
    })
    .build()
    .unwrap();
    let on = TeamMemberBuilder::new(
        find_character("xianyun").unwrap(),
        find_weapon("favonius_codex").unwrap(),
    )
    .artifact_stats(StatProfile {
        atk_percent: 0.50,
        atk_flat: 100.0,
        ..Default::default()
    })
    .activate("Consider, the Adeptus in Her Realm")
    .build()
    .unwrap();

    let expected = (total_atk(&on.stats) * 2.0).min(9000.0);
    let off = resolve_team_stats_detailed(&[off, dummy_member(Element::Anemo)], 1, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[on, dummy_member(Element::Anemo)], 1, &[]).unwrap();

    assert_close(
        on.damage_context.plunging_atk_flat_dmg - off.damage_context.plunging_atk_flat_dmg,
        expected,
        "Xianyun A4 should use total ATK flat damage",
    );
}

#[test]
fn nahida_a1_is_labeled_as_team_highest_em_approximation() {
    let defs = find_talent_buffs("nahida").expect("Nahida talent buffs should exist");
    let a1 = defs
        .iter()
        .find(|buff| buff.name == "Compassion Illuminated")
        .expect("Nahida A1 buff should exist");
    assert!(
        a1.description.contains("approximation"),
        "Nahida A1 should be labeled as a design approximation",
    );
}
