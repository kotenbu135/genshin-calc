use genshin_calc_data::find_character;

const EPS: f64 = 1e-4;

#[test]
fn thoma_na_has_5_hits() {
    let thoma = find_character("thoma").unwrap();
    assert_eq!(
        thoma.talents.normal_attack.hits.len(),
        5,
        "Thoma should have 5 NA hits (N1, N2, N3A, N3B, N4)"
    );
}

#[test]
fn thoma_n4_values() {
    let thoma = find_character("thoma").unwrap();
    let n4 = &thoma.talents.normal_attack.hits[4];
    assert_eq!(n4.name, "4段ダメージ");
    let expected: [f64; 15] = [
        0.6736, 0.7284, 0.7832, 0.8615, 0.9163, 0.9790, 1.0652, 1.1513, 1.2375, 1.3314, 1.4254,
        1.5194, 1.6134, 1.7074, 1.8014,
    ];
    for lv in 0..15 {
        assert!(
            (n4.values[lv] - expected[lv]).abs() <= EPS,
            "Thoma N4 Lv{}: expected {}, got {}",
            lv + 1,
            expected[lv],
            n4.values[lv]
        );
    }
}
