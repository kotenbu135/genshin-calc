use genshin_calc_data::find_character;

const EPS: f64 = 1e-4;

/// Issue #92: Iansan Skill Lv9 scaling value is wrong.
/// Source: honeyhunter-mirror/md/characters/iansan_110.md
/// Elemental Skill table: "Skill DMG / スキルダメージ"
/// Lv9 value: 486.88% = 4.8688 (source incorrectly has 4.6880)
#[test]
fn iansan_skill_lv9_value() {
    let iansan = find_character("iansan").unwrap();
    let skill = &iansan.talents.elemental_skill.scalings[0];
    let lv9_expected = 4.8688_f64;
    assert!(
        (skill.values[8] - lv9_expected).abs() <= EPS,
        "Iansan skill Lv9: expected {}, got {}",
        lv9_expected,
        skill.values[8]
    );
}

#[test]
fn iansan_skill_all_values() {
    let iansan = find_character("iansan").unwrap();
    let skill = &iansan.talents.elemental_skill.scalings[0];
    // From HH mirror: 286.4% / 307.88% / 329.36% / 358% / 379.48% / 400.96% /
    //                 429.6% / 458.24% / 486.88% / 515.52% / 544.16% / 572.8% /
    //                 608.6% / 644.4% / 680.2%
    let expected: [f64; 15] = [
        2.8640, 3.0788, 3.2936, 3.5800, 3.7948, 4.0096, 4.2960, 4.5824, 4.8688, 5.1552, 5.4416,
        5.7280, 6.0860, 6.4440, 6.8020,
    ];
    for lv in 0..15 {
        assert!(
            (skill.values[lv] - expected[lv]).abs() <= EPS,
            "Iansan skill Lv{}: expected {}, got {}",
            lv + 1,
            expected[lv],
            skill.values[lv]
        );
    }
}
