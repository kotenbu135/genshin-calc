use genshin_calc_data::{find_character, types::AscensionStat};

const EPS: f64 = 1e-4;

/// Issue #90: Ineffa ascension stat should be CritRate, not CritDmg
/// Source: honeyhunter-mirror/md/characters/ineffa_116.md
/// Base Stats table header: "Bonus CritRate%", max value at Lv90+: 19.2%
#[test]
fn ineffa_ascension_stat_is_crit_rate() {
    let ineffa = find_character("ineffa").unwrap();
    match ineffa.ascension_stat {
        AscensionStat::CritRate(v) => {
            assert!(
                (v - 0.192).abs() <= EPS,
                "Ineffa CritRate ascension should be 0.192 (19.2%), got {}",
                v
            );
        }
        other => panic!(
            "Ineffa ascension_stat should be CritRate(0.192), got {:?}",
            other
        ),
    }
}
