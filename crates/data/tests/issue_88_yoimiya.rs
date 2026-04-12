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
fn yoimiya_na_has_7_hits() {
    let yoimiya = find_character("yoimiya").unwrap();
    let hits = yoimiya.talents.normal_attack.hits;
    assert_eq!(
        hits.len(),
        7,
        "Yoimiya should have 7 NA hits (N1×2, N2, N3, N4×2, N5)"
    );
}

#[test]
fn yoimiya_n1_is_two_equal_hits() {
    let yoimiya = find_character("yoimiya").unwrap();
    let hits = yoimiya.talents.normal_attack.hits;
    // HH: "35.64%×2"
    for lv in 0..15 {
        assert_close(
            hits[0].values[lv],
            hits[1].values[lv],
            &format!("Yoimiya N1A==N1B Lv{}", lv + 1),
        );
    }
    assert_close(hits[0].values[0], 0.3564, "Yoimiya N1A Lv1");
}

#[test]
fn yoimiya_n2_values() {
    let yoimiya = find_character("yoimiya").unwrap();
    let hits = yoimiya.talents.normal_attack.hits;
    // HH N2 Lv1 = 68.38%
    assert_close(hits[2].values[0], 0.6838, "Yoimiya N2 Lv1");
}

#[test]
fn yoimiya_n3_values() {
    let yoimiya = find_character("yoimiya").unwrap();
    let hits = yoimiya.talents.normal_attack.hits;
    // HH N3 Lv1 = 88.89%
    assert_close(hits[3].values[0], 0.8889, "Yoimiya N3 Lv1");
}

#[test]
fn yoimiya_n4_is_two_equal_hits() {
    let yoimiya = find_character("yoimiya").unwrap();
    let hits = yoimiya.talents.normal_attack.hits;
    // HH: "46.42%×2"
    for lv in 0..15 {
        assert_close(
            hits[4].values[lv],
            hits[5].values[lv],
            &format!("Yoimiya N4A==N4B Lv{}", lv + 1),
        );
    }
    assert_close(hits[4].values[0], 0.4642, "Yoimiya N4A Lv1");
}

#[test]
fn yoimiya_n5_values() {
    let yoimiya = find_character("yoimiya").unwrap();
    let hits = yoimiya.talents.normal_attack.hits;
    assert_close(hits[6].values[0], 1.0586, "Yoimiya N5 Lv1");
}

#[test]
fn yoimiya_a1_pyro_dmg_bonus_exists() {
    let buffs = genshin_calc_data::find_talent_buffs("yoimiya").unwrap();
    let a1 = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::ElementalDmgBonus(genshin_calc_core::Element::Pyro)
                && matches!(
                    b.source,
                    genshin_calc_data::TalentBuffSource::AscensionPassive(1)
                )
        })
        .expect("Yoimiya A1 Pyro DMG buff should exist");

    assert!(
        (a1.base_value - 0.02).abs() < 1e-6,
        "A1 base = 2% per stack"
    );
    assert!(
        a1.activation.is_some(),
        "A1 should have Stacks activation (max 10)"
    );
}
