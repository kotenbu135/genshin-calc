/// Issue #98: Remove Mona C4 CRIT DMG buff (applies only to Hexerei party members, not all)
/// Source: honeyhunter-mirror/md/characters/mona_041.md
use genshin_calc_data::talent_buffs::find_talent_buffs;

/// C4 CRIT DMG buff should NOT exist (Hexerei-only mechanic, not modeled in engine)
#[test]
fn test_mona_c4_crit_dmg_buff_removed() {
    let buffs = find_talent_buffs("mona").expect("mona buffs not found");
    let has_crit_dmg = buffs.iter().any(|b| b.name == "mona_c4_crit_dmg");
    assert!(
        !has_crit_dmg,
        "mona_c4_crit_dmg should be removed: it only applies to Hexerei party members which is not modeled"
    );
}

/// C4 CRIT Rate buff should still exist (applies to all party members)
#[test]
fn test_mona_c4_crit_rate_buff_present() {
    let buffs = find_talent_buffs("mona").expect("mona buffs not found");
    let buff = buffs
        .iter()
        .find(|b| b.name == "mona_c4_crit_rate")
        .expect("mona_c4_crit_rate should still exist");
    assert!(
        (buff.base_value - 0.15).abs() < 1e-6,
        "C4 CR value should be 0.15"
    );
    assert_eq!(buff.min_constellation, 4);
}

/// C2 EM buff should still exist (EM +80 to all party from charged attack)
#[test]
fn test_mona_c2_em_buff_present() {
    let buffs = find_talent_buffs("mona").expect("mona buffs not found");
    let buff = buffs
        .iter()
        .find(|b| b.name == "mona_c2_em")
        .expect("mona_c2_em should exist");
    assert!(
        (buff.base_value - 80.0).abs() < 1e-6,
        "C2 EM value should be 80"
    );
    assert_eq!(buff.min_constellation, 2);
}
