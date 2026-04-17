use genshin_calc_core::{
    BuffableStat, Element, StatProfile, TeamMember, WeaponType, resolve_team_stats_detailed,
};
use genshin_calc_data::{TeamMemberBuilder, find_character, find_weapon};

fn assert_close(actual: f64, expected: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= 1e-6,
        "{label}: expected {expected}, got {actual}"
    );
}

fn dummy_member(element: Element, base_def: f64) -> TeamMember {
    TeamMember {
        element,
        weapon_type: WeaponType::Sword,
        stats: StatProfile {
            base_hp: 10000.0,
            base_atk: 500.0,
            base_def,
            crit_rate: 0.05,
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

fn total_hp(profile: &StatProfile) -> f64 {
    profile.base_hp * (1.0 + profile.hp_percent) + profile.hp_flat
}

#[test]
fn shenhe_a1_cryo_bonus_is_visible_in_applied_buffs_and_final_stats() {
    let shenhe_off = TeamMemberBuilder::new(
        find_character("shenhe").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .build()
    .unwrap();
    let shenhe_on = TeamMemberBuilder::new(
        find_character("shenhe").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .activate("Deific Embrace")
    .build()
    .unwrap();

    let target = dummy_member(Element::Cryo, 700.0);
    let off = resolve_team_stats_detailed(&[shenhe_off, target.clone()], 1, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[shenhe_on, target], 1, &[]).unwrap();

    let buff = applied_buff(
        &on,
        "Deific Embrace",
        BuffableStat::ElementalDmgBonus(Element::Cryo),
    );
    assert_close(buff.value, 0.15, "Shenhe A1 Cryo DMG");
    assert_close(
        on.final_stats.cryo_dmg_bonus - off.final_stats.cryo_dmg_bonus,
        0.15,
        "Shenhe A1 final Cryo DMG delta",
    );
}

#[test]
fn mika_a1_three_detector_stacks_are_visible_in_applied_buffs_and_final_stats() {
    let mika_off = TeamMemberBuilder::new(
        find_character("mika").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .build()
    .unwrap();
    let mika_on = TeamMemberBuilder::new(
        find_character("mika").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .activate_with_stacks("Suppressive Barrage", 3)
    .build()
    .unwrap();

    let target = dummy_member(Element::Cryo, 700.0);
    let off = resolve_team_stats_detailed(&[mika_off, target.clone()], 1, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[mika_on, target], 1, &[]).unwrap();

    let buff = applied_buff(&on, "Suppressive Barrage", BuffableStat::PhysicalDmgBonus);
    assert_close(buff.value, 0.30, "Mika A1 Physical DMG");
    assert_close(
        on.final_stats.physical_dmg_bonus - off.final_stats.physical_dmg_bonus,
        0.30,
        "Mika A1 final Physical DMG delta",
    );
}

#[test]
fn xiao_a1_max_stacks_are_visible_in_applied_buffs_and_final_stats() {
    let xiao_off = TeamMemberBuilder::new(
        find_character("xiao").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .build()
    .unwrap();
    let xiao_on = TeamMemberBuilder::new(
        find_character("xiao").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .activate_with_stacks("Conqueror of Evil: Tamer of Demons", 5)
    .build()
    .unwrap();

    let off = resolve_team_stats_detailed(&[xiao_off], 0, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[xiao_on], 0, &[]).unwrap();

    let buff = applied_buff(
        &on,
        "Conqueror of Evil: Tamer of Demons",
        BuffableStat::DmgBonus,
    );
    assert_close(buff.value, 0.25, "Xiao A1 DMG");
    assert_close(
        on.final_stats.dmg_bonus - off.final_stats.dmg_bonus,
        0.25,
        "Xiao A1 final DMG delta",
    );
}

#[test]
fn varka_c6_four_azure_fang_stacks_are_visible_in_applied_buffs_and_final_stats() {
    let varka_off = TeamMemberBuilder::new(
        find_character("varka").unwrap(),
        find_weapon("wolfs_gravestone").unwrap(),
    )
    .constellation(6)
    .build()
    .unwrap();
    let varka_on = TeamMemberBuilder::new(
        find_character("varka").unwrap(),
        find_weapon("wolfs_gravestone").unwrap(),
    )
    .constellation(6)
    .activate_with_stacks("Azure Fang's Oath CRIT DMG", 4)
    .build()
    .unwrap();

    let off = resolve_team_stats_detailed(&[varka_off], 0, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[varka_on], 0, &[]).unwrap();

    let buff = applied_buff(&on, "Azure Fang's Oath CRIT DMG", BuffableStat::CritDmg);
    assert_close(buff.value, 0.80, "Varka C6 CRIT DMG");
    assert_close(
        on.final_stats.crit_dmg - off.final_stats.crit_dmg,
        0.80,
        "Varka C6 final CRIT DMG delta",
    );
}

#[test]
fn xingqiu_a4_hydro_bonus_is_visible_in_applied_buffs_and_final_stats() {
    let xingqiu = TeamMemberBuilder::new(
        find_character("xingqiu").unwrap(),
        find_weapon("freedom_sworn").unwrap(),
    )
    .build()
    .unwrap();

    let result = resolve_team_stats_detailed(&[xingqiu.clone()], 0, &[]).unwrap();

    let buff = applied_buff(
        &result,
        "Blades Amidst Raindrops",
        BuffableStat::ElementalDmgBonus(Element::Hydro),
    );
    assert_close(buff.value, 0.20, "Xingqiu A4 Hydro DMG");
    assert_close(
        result.final_stats.hydro_dmg_bonus,
        xingqiu.stats.hydro_dmg_bonus + 0.20,
        "Xingqiu final Hydro DMG bonus",
    );
}

#[test]
fn candace_a4_hp_scaling_is_visible_in_applied_buffs_and_damage_context() {
    let candace_off = TeamMemberBuilder::new(
        find_character("candace").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .artifact_stats(StatProfile {
        hp_percent: 0.46,
        hp_flat: 4780.0,
        ..Default::default()
    })
    .build()
    .unwrap();
    let candace_on = TeamMemberBuilder::new(
        find_character("candace").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .artifact_stats(StatProfile {
        hp_percent: 0.46,
        hp_flat: 4780.0,
        ..Default::default()
    })
    .activate("Celestial Dome of Sand")
    .build()
    .unwrap();

    let target = dummy_member(Element::Hydro, 700.0);
    let off = resolve_team_stats_detailed(&[candace_off, target.clone()], 1, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[candace_on.clone(), target], 1, &[]).unwrap();
    let expected_bonus = total_hp(&candace_on.stats) * 0.000005;

    let buff = applied_buff(
        &on,
        "Celestial Dome of Sand",
        BuffableStat::NormalAtkDmgBonus,
    );
    assert_close(buff.value, expected_bonus, "Candace A4 DMG bonus");
    assert_close(
        on.damage_context.normal_atk_dmg_bonus - off.damage_context.normal_atk_dmg_bonus,
        expected_bonus,
        "Candace A4 normal dmg delta",
    );
}

#[test]
fn beidou_a4_is_visible_in_applied_buffs_and_damage_context() {
    let beidou_off = TeamMemberBuilder::new(
        find_character("beidou").unwrap(),
        find_weapon("wolfs_gravestone").unwrap(),
    )
    .build()
    .unwrap();
    let beidou_on = TeamMemberBuilder::new(
        find_character("beidou").unwrap(),
        find_weapon("wolfs_gravestone").unwrap(),
    )
    .activate("Lightning Storm")
    .build()
    .unwrap();

    let off = resolve_team_stats_detailed(&[beidou_off], 0, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[beidou_on], 0, &[]).unwrap();

    let normal = applied_buff(&on, "Lightning Storm", BuffableStat::NormalAtkDmgBonus);
    assert_close(normal.value, 0.15, "Beidou A4 normal dmg");
    let charged = applied_buff(&on, "Lightning Storm", BuffableStat::ChargedAtkDmgBonus);
    assert_close(charged.value, 0.15, "Beidou A4 charged dmg");

    assert_close(
        on.damage_context.normal_atk_dmg_bonus - off.damage_context.normal_atk_dmg_bonus,
        0.15,
        "Beidou A4 normal dmg delta",
    );
    assert_close(
        on.damage_context.charged_atk_dmg_bonus - off.damage_context.charged_atk_dmg_bonus,
        0.15,
        "Beidou A4 charged dmg delta",
    );
}

#[test]
fn razor_c4_def_reduction_is_visible_in_applied_buffs_and_enemy_debuffs() {
    let razor_off = TeamMemberBuilder::new(
        find_character("razor").unwrap(),
        find_weapon("wolfs_gravestone").unwrap(),
    )
    .constellation(4)
    .build()
    .unwrap();
    let razor_on = TeamMemberBuilder::new(
        find_character("razor").unwrap(),
        find_weapon("wolfs_gravestone").unwrap(),
    )
    .constellation(4)
    .activate("Bite")
    .build()
    .unwrap();

    let off = resolve_team_stats_detailed(&[razor_off], 0, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[razor_on], 0, &[]).unwrap();

    let buff = applied_buff(&on, "Bite", BuffableStat::DefReduction);
    assert_close(buff.value, 0.15, "Razor C4 DEF reduction");
    assert_close(
        on.enemy_debuffs.def_reduction - off.enemy_debuffs.def_reduction,
        0.15,
        "Razor C4 enemy DEF reduction delta",
    );
}

#[test]
fn raiden_a4_er_scaling_is_visible_in_applied_buffs_and_final_stats() {
    let raiden = TeamMemberBuilder::new(
        find_character("raiden_shogun").unwrap(),
        find_weapon("engulfing_lightning").unwrap(),
    )
    .artifact_stats(StatProfile {
        energy_recharge: 0.50,
        ..Default::default()
    })
    .build()
    .unwrap();

    let result = resolve_team_stats_detailed(&[raiden.clone()], 0, &[]).unwrap();
    let expected_bonus = (raiden.stats.energy_recharge - 1.0).max(0.0) * 0.4;

    let buff = applied_buff(
        &result,
        "Enlightened One",
        BuffableStat::ElementalDmgBonus(Element::Electro),
    );
    assert_close(buff.value, expected_bonus, "Raiden A4 Electro DMG");
    assert_close(
        result.final_stats.electro_dmg_bonus,
        raiden.stats.electro_dmg_bonus + expected_bonus,
        "Raiden final Electro DMG bonus",
    );
}

#[test]
fn cyno_a1_skill_bonus_is_visible_in_applied_buffs_and_damage_context() {
    let cyno_off = TeamMemberBuilder::new(
        find_character("cyno").unwrap(),
        find_weapon("staff_of_the_scarlet_sands").unwrap(),
    )
    .build()
    .unwrap();
    let cyno_on = TeamMemberBuilder::new(
        find_character("cyno").unwrap(),
        find_weapon("staff_of_the_scarlet_sands").unwrap(),
    )
    .activate("Featherfall Judgment")
    .build()
    .unwrap();

    let off = resolve_team_stats_detailed(&[cyno_off], 0, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[cyno_on], 0, &[]).unwrap();

    let buff = applied_buff(&on, "Featherfall Judgment", BuffableStat::SkillDmgBonus);
    assert_close(buff.value, 0.35, "Cyno A1 Skill DMG");
    assert_close(
        on.damage_context.skill_dmg_bonus - off.damage_context.skill_dmg_bonus,
        0.35,
        "Cyno A1 skill dmg delta",
    );
}
