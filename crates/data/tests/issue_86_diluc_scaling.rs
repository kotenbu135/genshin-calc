use genshin_calc_data::find_character;

const EPS: f64 = 1e-4;

fn assert_close(actual: f64, expected: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= EPS,
        "{label}: expected {expected}, got {actual}"
    );
}

#[test]
fn diluc_normal_attack_lv11_to_15() {
    let diluc = find_character("diluc").unwrap();

    // HH expected values for N1-N4, Lv11-15
    let expected_n1: [f64; 15] = [
        0.8974, 0.9704, 1.0434, 1.1477, 1.2207, 1.3042, 1.4191, 1.5340, 1.6489, 1.7739, 1.9165,
        2.0852, 2.2538, 2.4225, 2.6065,
    ];
    let expected_n2: [f64; 15] = [
        0.8764, 0.9477, 1.0190, 1.1209, 1.1922, 1.2737, 1.3858, 1.4979, 1.6100, 1.7323, 1.8724,
        2.0372, 2.2020, 2.3667, 2.5465,
    ];
    let expected_n3: [f64; 15] = [
        0.9882, 1.0687, 1.1493, 1.2642, 1.3447, 1.4366, 1.5628, 1.6891, 1.8153, 1.9535, 2.1113,
        2.2971, 2.4829, 2.6687, 2.8714,
    ];
    let expected_n4: [f64; 15] = [
        1.3399, 1.4491, 1.5583, 1.7141, 1.8233, 1.9479, 2.1191, 2.2904, 2.4616, 2.6489, 2.8628,
        3.1148, 3.3667, 3.6186, 3.8934,
    ];

    let hits = diluc.talents.normal_attack.hits;
    for (i, (hit, expected)) in [
        (&hits[0], &expected_n1),
        (&hits[1], &expected_n2),
        (&hits[2], &expected_n3),
        (&hits[3], &expected_n4),
    ]
    .iter()
    .enumerate()
    {
        for lv in 0..15 {
            assert_close(
                hit.values[lv],
                expected[lv],
                &format!("Diluc N{} Lv{}", i + 1, lv + 1),
            );
        }
    }
}

#[test]
fn diluc_charged_final_lv11_to_15() {
    let diluc = find_character("diluc").unwrap();

    let expected_final: [f64; 15] = [
        1.2470, 1.3485, 1.4500, 1.5950, 1.6965, 1.8125, 1.9720, 2.1315, 2.2910, 2.4650, 2.6644,
        2.8988, 3.1333, 3.3678, 3.6236,
    ];

    let charged_final = &diluc.talents.normal_attack.charged[1];
    for lv in 0..15 {
        assert_close(
            charged_final.values[lv],
            expected_final[lv],
            &format!("Diluc Charged Final Lv{}", lv + 1),
        );
    }
}
