use genshin_calc_core::{
    BuffableStat, DamageContext, Element, StatProfile, TeamMember, WeaponType,
    resolve_team_stats_detailed,
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

#[test]
fn nahida_a4_applies_skill_bonus_and_crit_rate() {
    let nahida = TeamMemberBuilder::new(
        find_character("nahida").unwrap(),
        find_weapon("sacrificial_fragments").unwrap(),
    )
    .artifact_stats(StatProfile {
        elemental_mastery: 600.0,
        ..Default::default()
    })
    .build()
    .unwrap();

    let result = resolve_team_stats_detailed(&[nahida.clone()], 0, &[]).unwrap();
    let em_above_200 = (nahida.stats.elemental_mastery - 200.0).max(0.0);
    let expected_skill_bonus = (em_above_200 * 0.001).min(0.80);
    let expected_crit_rate = (em_above_200 * 0.0003).min(0.24);

    let skill_buff = applied_buff(
        &result,
        "Awakening Elucidated Skill DMG Bonus",
        BuffableStat::SkillDmgBonus,
    );
    assert_close(
        skill_buff.value,
        expected_skill_bonus,
        "Nahida A4 skill buff",
    );

    let crit_buff = applied_buff(
        &result,
        "Awakening Elucidated Skill CRIT Rate",
        BuffableStat::CritRate,
    );
    assert_close(crit_buff.value, expected_crit_rate, "Nahida A4 crit buff");

    assert_close(
        result.damage_context.skill_dmg_bonus,
        expected_skill_bonus,
        "Nahida final skill dmg bonus",
    );
    assert_close(
        result.final_stats.crit_rate,
        nahida.stats.crit_rate + expected_crit_rate,
        "Nahida final crit rate",
    );
}

#[test]
fn kirara_a4_scales_from_max_hp_for_skill_and_burst() {
    let kirara = TeamMemberBuilder::new(
        find_character("kirara").unwrap(),
        find_weapon("freedom_sworn").unwrap(),
    )
    .artifact_stats(StatProfile {
        hp_percent: 0.46,
        hp_flat: 4780.0,
        ..Default::default()
    })
    .build()
    .unwrap();

    let result = resolve_team_stats_detailed(&[kirara.clone()], 0, &[]).unwrap();
    let expected_hp = result.final_stats.hp;
    let expected_skill_bonus = expected_hp * 0.000004;
    let expected_burst_bonus = expected_hp * 0.000003;

    let skill_buff = applied_buff(
        &result,
        "Pupillary Variance Skill DMG Bonus",
        BuffableStat::SkillDmgBonus,
    );
    assert_close(
        skill_buff.value,
        expected_skill_bonus,
        "Kirara A4 skill buff",
    );

    let burst_buff = applied_buff(
        &result,
        "Pupillary Variance Burst DMG Bonus",
        BuffableStat::BurstDmgBonus,
    );
    assert_close(
        burst_buff.value,
        expected_burst_bonus,
        "Kirara A4 burst buff",
    );

    assert_close(
        result.damage_context.skill_dmg_bonus,
        expected_skill_bonus,
        "Kirara final skill dmg bonus",
    );
    assert_close(
        result.damage_context.burst_dmg_bonus,
        expected_burst_bonus,
        "Kirara final burst dmg bonus",
    );
}

#[test]
fn gorou_a4_team_def_percent_is_visible_in_applied_buffs_and_final_stats() {
    let gorou_off = TeamMemberBuilder::new(
        find_character("gorou").unwrap(),
        find_weapon("favonius_warbow").unwrap(),
    )
    .talent_levels([1, 9, 9])
    .build()
    .unwrap();
    let gorou_on = TeamMemberBuilder::new(
        find_character("gorou").unwrap(),
        find_weapon("favonius_warbow").unwrap(),
    )
    .talent_levels([1, 9, 9])
    .activate("Heedless of the Wind and Weather")
    .build()
    .unwrap();

    let target = dummy_member(Element::Geo, 800.0);
    let off = resolve_team_stats_detailed(&[gorou_off, target.clone()], 1, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[gorou_on, target], 1, &[]).unwrap();

    let buff = applied_buff(
        &on,
        "Heedless of the Wind and Weather",
        BuffableStat::DefPercent,
    );
    assert_close(buff.value, 0.25, "Gorou A4 DEF%");
    assert_close(
        on.final_stats.def - off.final_stats.def,
        800.0 * 0.25,
        "Gorou A4 final DEF delta",
    );
}

#[test]
fn yun_jin_a4_adds_manual_four_element_normal_flat_damage() {
    let yun_jin_off = TeamMemberBuilder::new(
        find_character("yun_jin").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .talent_levels([1, 1, 9])
    .build()
    .unwrap();
    let yun_jin_on = TeamMemberBuilder::new(
        find_character("yun_jin").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .talent_levels([1, 1, 9])
    .activate("Breaking Conventions (4 Element Types)")
    .build()
    .unwrap();

    let target = dummy_member(Element::Pyro, 700.0);
    let off = resolve_team_stats_detailed(&[yun_jin_off, target.clone()], 1, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[yun_jin_on.clone(), target], 1, &[]).unwrap();

    let expected_bonus = yun_jin_on.stats.base_def * 0.115;
    let buff = applied_buff(
        &on,
        "Breaking Conventions (4 Element Types)",
        BuffableStat::NormalAtkFlatDmg,
    );
    assert_close(buff.value, expected_bonus, "Yun Jin A4 flat bonus");
    assert_close(
        on.damage_context.normal_atk_flat_dmg - off.damage_context.normal_atk_flat_dmg,
        expected_bonus,
        "Yun Jin A4 normal flat delta",
    );
}

#[test]
fn xilonen_a4_self_def_percent_is_visible_in_applied_buffs_and_final_stats() {
    let xilonen_off = TeamMemberBuilder::new(
        find_character("xilonen").unwrap(),
        find_weapon("cinnabar_spindle").unwrap(),
    )
    .build()
    .unwrap();
    let xilonen_on = TeamMemberBuilder::new(
        find_character("xilonen").unwrap(),
        find_weapon("cinnabar_spindle").unwrap(),
    )
    .activate("Portable Armored Sheath")
    .build()
    .unwrap();
    let expected_delta = xilonen_on.stats.base_def * 0.20;

    let off = resolve_team_stats_detailed(&[xilonen_off], 0, &[]).unwrap();
    let on = resolve_team_stats_detailed(&[xilonen_on], 0, &[]).unwrap();

    let buff = applied_buff(&on, "Portable Armored Sheath", BuffableStat::DefPercent);
    assert_close(buff.value, 0.20, "Xilonen A4 DEF%");
    assert_close(
        on.final_stats.def - off.final_stats.def,
        expected_delta,
        "Xilonen A4 final DEF delta",
    );
}

#[test]
fn illuga_a4_three_hydro_geo_branch_is_visible_in_applied_buffs_and_damage_context() {
    let illuga = TeamMemberBuilder::new(
        find_character("illuga").unwrap(),
        find_weapon("moonpiercer").unwrap(),
    )
    .activate("Demonhunter's Dusk (3 Hydro/Geo Characters)")
    .build()
    .unwrap();

    let result =
        resolve_team_stats_detailed(&[illuga.clone(), dummy_member(Element::Geo, 700.0)], 1, &[])
            .unwrap();
    let expected_bonus = illuga.stats.elemental_mastery * 0.24;

    let buff = applied_buff(
        &result,
        "Demonhunter's Dusk (3 Hydro/Geo Characters)",
        BuffableStat::BurstFlatDmg,
    );
    assert_close(buff.value, expected_bonus, "Illuga A4 flat bonus");
    assert_close(
        result.damage_context.burst_flat_dmg,
        expected_bonus,
        "Illuga final burst flat dmg",
    );
}

#[test]
fn damage_context_starts_empty_for_issue_70_regressions() {
    assert_eq!(DamageContext::default().normal_atk_flat_dmg, 0.0);
}
