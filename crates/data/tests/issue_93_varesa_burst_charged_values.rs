use genshin_calc_data::find_character;

const EPS: f64 = 1e-4;

/// Issue #93: Varesa has 3 wrong values in Burst and Charged attacks.
/// Source: honeyhunter-mirror/md/characters/varesa_111.md
///
/// 1. BURST_KICK Lv8: 552.19% = 5.5219 (source has 5.2190 — missing digit)
/// 2. BURST_KICK Lv14: 776.52% = 7.7652 (source has 7.6550)
/// 3. PASSION_CHARGED Lv10: 166.75% = 1.6675 (source has 1.6780)

#[test]
fn varesa_burst_kick_lv8() {
    let varesa = find_character("varesa").unwrap();
    let kick = &varesa.talents.elemental_burst.scalings[0];
    let expected = 5.5219_f64;
    assert!(
        (kick.values[7] - expected).abs() <= EPS,
        "Varesa burst kick Lv8: expected {}, got {}",
        expected,
        kick.values[7]
    );
}

#[test]
fn varesa_burst_kick_lv14() {
    let varesa = find_character("varesa").unwrap();
    let kick = &varesa.talents.elemental_burst.scalings[0];
    let expected = 7.7652_f64;
    assert!(
        (kick.values[13] - expected).abs() <= EPS,
        "Varesa burst kick Lv14: expected {}, got {}",
        expected,
        kick.values[13]
    );
}

#[test]
fn varesa_passion_charged_lv10() {
    let varesa = find_character("varesa").unwrap();
    // scalings[1] is Fiery Passion Charged Attack
    let passion_charged = &varesa.talents.normal_attack.charged[1];
    let expected = 1.6675_f64;
    assert!(
        (passion_charged.values[9] - expected).abs() <= EPS,
        "Varesa Fiery Passion Charged Lv10: expected {}, got {}",
        expected,
        passion_charged.values[9]
    );
}

#[test]
fn varesa_burst_kick_all_values() {
    let varesa = find_character("varesa").unwrap();
    let kick = &varesa.talents.elemental_burst.scalings[0];
    // From HH mirror: 345.12% / 371% / 396.89% / 431.4% / 457.28% / 483.17% /
    //                 517.68% / 552.19% / 586.7% / 621.22% / 655.73% / 690.24% /
    //                 733.38% / 776.52% / 819.66%
    let expected: [f64; 15] = [
        3.4512, 3.7100, 3.9689, 4.3140, 4.5728, 4.8317, 5.1768, 5.5219, 5.8670, 6.2122, 6.5573,
        6.9024, 7.3338, 7.7652, 8.1966,
    ];
    for lv in 0..15 {
        assert!(
            (kick.values[lv] - expected[lv]).abs() <= EPS,
            "Varesa burst kick Lv{}: expected {}, got {}",
            lv + 1,
            expected[lv],
            kick.values[lv]
        );
    }
}
