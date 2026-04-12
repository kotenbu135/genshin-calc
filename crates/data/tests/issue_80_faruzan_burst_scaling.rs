use genshin_calc_data::find_talent_buffs;

const ANEMO_TALENT_BUFFS_SOURCE: &str = include_str!("../src/talent_buffs/anemo.rs");

#[test]
fn faruzan_burst_scaling_matches_mirror_and_drops_wiki_reference() {
    let buffs = find_talent_buffs("faruzan").expect("Faruzan talent buffs should exist");
    let scaling = buffs
        .iter()
        .find(|buff| buff.name == "Prayerful Wind's Benefit")
        .and_then(|buff| buff.talent_scaling)
        .expect("Faruzan burst scaling should exist");

    let expected = [
        0.18, 0.1935, 0.207, 0.225, 0.2385, 0.252, 0.27, 0.288, 0.306, 0.324, 0.342, 0.36, 0.3825,
        0.405, 0.4275,
    ];

    assert_eq!(
        scaling, &expected,
        "Faruzan burst Anemo DMG Bonus should match the honeyhunter mirror at Lv1-15"
    );
    assert!(
        ANEMO_TALENT_BUFFS_SOURCE.contains("honeyhunter-mirror"),
        "Faruzan burst scaling comment should cite honeyhunter-mirror",
    );
    assert!(
        !ANEMO_TALENT_BUFFS_SOURCE.contains("Genshin Wiki"),
        "Faruzan burst scaling comment should no longer mention Genshin Wiki",
    );
}
