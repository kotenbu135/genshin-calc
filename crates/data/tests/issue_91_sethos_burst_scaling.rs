use genshin_calc_core::ScalingStat;
use genshin_calc_data::find_character;

const EPS: f64 = 1e-4;

/// Issue #91: Sethos Burst (Dusk Bolt) scales off Elemental Mastery, not ATK.
/// Source: honeyhunter-mirror/md/characters/sethos_097.md
/// Elemental Burst table: "Dusk Bolt DMG Increase / 瞑弦の矢ダメージアップ"
/// Values given as "196.16% Elemental Mastery", "210.87% Elemental Mastery", etc.
#[test]
fn sethos_burst_scaling_stat_is_em() {
    let sethos = find_character("sethos").unwrap();
    let burst = &sethos.talents.elemental_burst.scalings[0];
    assert_eq!(
        burst.scaling_stat,
        ScalingStat::Em,
        "Sethos burst (瞑弦の矢) should scale off Em, not {:?}",
        burst.scaling_stat
    );
}

#[test]
fn sethos_burst_em_values() {
    let sethos = find_character("sethos").unwrap();
    let burst = &sethos.talents.elemental_burst.scalings[0];
    // From HH mirror: 196.16% / 210.87% / 225.58% / 245.2% / 259.91% / 274.62% /
    //                 294.24% / 313.86% / 333.47% / 353.09% / 372.70% / 392.32% /
    //                 416.84% / 441.36% / 465.88%
    let expected: [f64; 15] = [
        1.9616, 2.1087, 2.2558, 2.4520, 2.5991, 2.7462, 2.9424, 3.1386, 3.3347, 3.5309, 3.7270,
        3.9232, 4.1684, 4.4136, 4.6588,
    ];
    for lv in 0..15 {
        assert!(
            (burst.values[lv] - expected[lv]).abs() <= EPS,
            "Sethos burst Lv{}: expected {}, got {}",
            lv + 1,
            expected[lv],
            burst.values[lv]
        );
    }
}
