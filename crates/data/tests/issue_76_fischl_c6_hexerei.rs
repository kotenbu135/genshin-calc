use genshin_calc_core::{BuffableStat, resolve_team_stats_detailed};
use genshin_calc_data::{
    TalentBuffSource, TeamMemberBuilder, find_character, find_talent_buffs, find_weapon,
};

fn assert_close(actual: f64, expected: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= 1e-6,
        "{label}: expected {expected}, got {actual}"
    );
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
fn fischl_c6_buffed_state_doubles_witchs_eve_rite_buffs() {
    let defs = find_talent_buffs("fischl").expect("Fischl talent buffs should exist");
    let atk_c6 = defs
        .iter()
        .find(|buff| buff.name == "Witch's Eve Rite Hexerei ATK Bonus Buffed State")
        .expect("Fischl C6 ATK Buffed State should exist");
    assert_eq!(atk_c6.source, TalentBuffSource::Constellation(6));
    assert_close(atk_c6.base_value, 0.225, "Fischl C6 ATK base value");

    let em_c6 = defs
        .iter()
        .find(|buff| buff.name == "Witch's Eve Rite Hexerei EM Bonus Buffed State")
        .expect("Fischl C6 EM Buffed State should exist");
    assert_eq!(em_c6.source, TalentBuffSource::Constellation(6));
    assert_close(em_c6.base_value, 90.0, "Fischl C6 EM base value");

    let off = TeamMemberBuilder::new(
        find_character("fischl").unwrap(),
        find_weapon("favonius_warbow").unwrap(),
    )
    .constellation(6)
    .build()
    .unwrap();
    let a4 = TeamMemberBuilder::new(
        find_character("fischl").unwrap(),
        find_weapon("favonius_warbow").unwrap(),
    )
    .constellation(6)
    .activate("Witch's Eve Rite Hexerei ATK Bonus")
    .activate("Witch's Eve Rite Hexerei EM Bonus")
    .build()
    .unwrap();
    let buffed = TeamMemberBuilder::new(
        find_character("fischl").unwrap(),
        find_weapon("favonius_warbow").unwrap(),
    )
    .constellation(6)
    .activate("Witch's Eve Rite Hexerei ATK Bonus")
    .activate("Witch's Eve Rite Hexerei EM Bonus")
    .activate("Witch's Eve Rite Hexerei ATK Bonus Buffed State")
    .activate("Witch's Eve Rite Hexerei EM Bonus Buffed State")
    .build()
    .unwrap();

    let off = resolve_team_stats_detailed(&[off], 0, &[]).unwrap();
    let a4 = resolve_team_stats_detailed(&[a4], 0, &[]).unwrap();
    let buffed = resolve_team_stats_detailed(&[buffed], 0, &[]).unwrap();

    let atk_a4 = applied_buff(
        &a4,
        "Witch's Eve Rite Hexerei ATK Bonus",
        BuffableStat::AtkPercent,
    );
    assert_close(atk_a4.value, 0.225, "Fischl A4 ATK buff");
    let atk_c6 = applied_buff(
        &buffed,
        "Witch's Eve Rite Hexerei ATK Bonus Buffed State",
        BuffableStat::AtkPercent,
    );
    assert_close(atk_c6.value, 0.225, "Fischl C6 extra ATK buff");

    let em_a4 = applied_buff(
        &a4,
        "Witch's Eve Rite Hexerei EM Bonus",
        BuffableStat::ElementalMastery,
    );
    assert_close(em_a4.value, 90.0, "Fischl A4 EM buff");
    let em_c6 = applied_buff(
        &buffed,
        "Witch's Eve Rite Hexerei EM Bonus Buffed State",
        BuffableStat::ElementalMastery,
    );
    assert_close(em_c6.value, 90.0, "Fischl C6 extra EM buff");

    let base_atk_delta = a4.final_stats.atk - off.final_stats.atk;
    let c6_extra_atk_delta = buffed.final_stats.atk - a4.final_stats.atk;
    assert_close(
        c6_extra_atk_delta,
        base_atk_delta,
        "Fischl C6 should add one extra A4-sized ATK chunk",
    );
    assert_close(
        buffed.final_stats.atk - off.final_stats.atk,
        base_atk_delta * 2.0,
        "Fischl C6 should double total Witch's Eve Rite ATK",
    );

    assert_close(
        a4.final_stats.elemental_mastery - off.final_stats.elemental_mastery,
        90.0,
        "Fischl A4 should add 90 EM",
    );
    assert_close(
        buffed.final_stats.elemental_mastery - a4.final_stats.elemental_mastery,
        90.0,
        "Fischl C6 should add one extra 90 EM chunk",
    );
    assert_close(
        buffed.final_stats.elemental_mastery - off.final_stats.elemental_mastery,
        180.0,
        "Fischl C6 should double total Witch's Eve Rite EM",
    );
}
