use genshin_calc_data::{TeamMemberBuilder, find_character, find_weapon, types::AscensionStat};

fn assert_close(actual: f64, expected: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= 1e-6,
        "{label}: expected {expected}, got {actual}"
    );
}

#[test]
fn xilonen_ascension_def_is_36_percent_and_applies_to_built_stats() {
    let xilonen = find_character("xilonen").expect("Xilonen character data should exist");
    assert_eq!(xilonen.ascension_stat, AscensionStat::Def(0.36));

    let member = TeamMemberBuilder::new(xilonen, find_weapon("favonius_sword").unwrap())
        .build()
        .unwrap();

    assert_close(member.stats.def_percent, 0.36, "Xilonen DEF ascension stat");
    assert_close(
        member.stats.base_def * (1.0 + member.stats.def_percent),
        member.stats.base_def * 1.36,
        "Xilonen final DEF uses 36% ascension stat",
    );
}
