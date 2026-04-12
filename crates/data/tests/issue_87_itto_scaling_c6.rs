use genshin_calc_core::BuffableStat;
use genshin_calc_data::find_character;

const EPS: f64 = 1e-4;

fn assert_close(actual: f64, expected: f64, label: &str) {
    assert!(
        (actual - expected).abs() <= EPS,
        "{label}: expected {expected}, got {actual}"
    );
}

#[test]
fn itto_normal_attack_lv11_to_15() {
    let itto = find_character("itto").unwrap();
    let hits = itto.talents.normal_attack.hits;

    // HH N1 Lv11-15
    let expected_n1: [f64; 5] = [1.6929, 1.8419, 1.9908, 2.1398, 2.3023];
    // HH N2 Lv11-15
    let expected_n2: [f64; 5] = [1.6317, 1.7753, 1.9189, 2.0625, 2.2191];
    // HH N3 Lv11-15
    let expected_n3: [f64; 5] = [1.9580, 2.1303, 2.3027, 2.4750, 2.6629];
    // HH N4 Lv11-15
    let expected_n4: [f64; 5] = [2.5047, 2.7251, 2.9455, 3.1659, 3.4063];

    for (i, (hit, expected)) in [
        (&hits[0], &expected_n1),
        (&hits[1], &expected_n2),
        (&hits[2], &expected_n3),
        (&hits[3], &expected_n4),
    ]
    .iter()
    .enumerate()
    {
        for lv in 0..5 {
            assert_close(
                hit.values[10 + lv],
                expected[lv],
                &format!("Itto N{} Lv{}", i + 1, 11 + lv),
            );
        }
    }
}

#[test]
fn itto_charged_slash_and_final_lv11_to_15() {
    let itto = find_character("itto").unwrap();
    let charged = itto.talents.normal_attack.charged;

    // HH Charged Slash Lv11-15
    let expected_slash: [f64; 5] = [1.9478, 2.1192, 2.2906, 2.4620, 2.6489];
    // HH Charged Final Lv11-15
    let expected_final: [f64; 5] = [4.0793, 4.4382, 4.7972, 5.1562, 5.5478];

    for lv in 0..5 {
        assert_close(
            charged[0].values[10 + lv],
            expected_slash[lv],
            &format!("Itto Charged Slash Lv{}", 11 + lv),
        );
        assert_close(
            charged[1].values[10 + lv],
            expected_final[lv],
            &format!("Itto Charged Final Lv{}", 11 + lv),
        );
    }
}

#[test]
fn itto_c6_uses_charged_atk_crit_dmg() {
    let buffs = genshin_calc_data::find_talent_buffs("itto").unwrap();
    let c6_buff = buffs
        .iter()
        .find(|b| b.name == "Crimson Oni King's Legacy CA CRIT DMG")
        .expect("C6 buff should exist");

    assert_eq!(c6_buff.stat, BuffableStat::ChargedAtkCritDmg);
    assert!((c6_buff.base_value - 0.70).abs() < 1e-6);
    assert_eq!(c6_buff.min_constellation, 6);
}
