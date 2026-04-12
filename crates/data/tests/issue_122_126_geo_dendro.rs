use genshin_calc_core::moonsign::{MoonsignLevel, MoonsignTalentEffect};
use genshin_calc_core::{BuffTarget, BuffableStat, Element, Reaction, ScalingStat};
use genshin_calc_data::moonsign_chars::LAUMA_TALENT_ENHANCEMENTS;
/// Tests for issues #122-126: Zibai A1/A4, Baizhu A1, Kinich C1, Lauma A1 moonsign.
use genshin_calc_data::talent_buffs::{TalentBuffSource, find_talent_buffs};

const EPS: f64 = 1e-6;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

// ===== Issue #122: Zibai A1 =====

#[test]
fn zibai_a1_skill_flat_dmg_def_scaling() {
    let buffs = find_talent_buffs("zibai").unwrap();
    let a1 = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::SkillFlatDmg
                && b.source == TalentBuffSource::AscensionPassive(1)
        })
        .expect("Zibai A1 SkillFlatDmg buff missing");
    assert!(
        close(a1.base_value, 0.60),
        "A1 SkillFlatDmg ratio should be 0.60, got {}",
        a1.base_value
    );
    assert_eq!(a1.scales_on, Some(ScalingStat::Def));
    assert_eq!(a1.target, BuffTarget::OnlySelf);
}

// ===== Issue #122: Zibai A4 =====

#[test]
fn zibai_a4_def_percent_per_geo_member() {
    let buffs = find_talent_buffs("zibai").unwrap();
    let a4_def = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::DefPercent && b.source == TalentBuffSource::AscensionPassive(4)
        })
        .expect("Zibai A4 DefPercent buff missing");
    assert!(
        close(a4_def.base_value, 0.15),
        "A4 DefPercent should be 0.15, got {}",
        a4_def.base_value
    );
    assert_eq!(a4_def.target, BuffTarget::OnlySelf);
}

#[test]
fn zibai_a4_em_per_hydro_member() {
    let buffs = find_talent_buffs("zibai").unwrap();
    let a4_em = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::ElementalMastery
                && b.source == TalentBuffSource::AscensionPassive(4)
        })
        .expect("Zibai A4 ElementalMastery buff missing");
    assert!(
        close(a4_em.base_value, 60.0),
        "A4 EM should be 60.0, got {}",
        a4_em.base_value
    );
    assert_eq!(a4_em.target, BuffTarget::OnlySelf);
}

#[test]
fn zibai_buff_count_with_a1_a4() {
    let buffs = find_talent_buffs("zibai").unwrap();
    // Was 1 (C2), now +3 (A1 SkillFlatDmg + A4 DefPercent + A4 EM) = 4
    assert_eq!(buffs.len(), 4, "zibai should have 4 buffs (C2 + A1 + A4x2)");
}

// ===== Issue #123: Baizhu A1 =====

#[test]
fn baizhu_a1_dendro_dmg_bonus() {
    let buffs = find_talent_buffs("baizhu").unwrap();
    let a1 = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::ElementalDmgBonus(Element::Dendro)
                && b.source == TalentBuffSource::AscensionPassive(1)
        })
        .expect("Baizhu A1 ElementalDmgBonus(Dendro) buff missing");
    assert!(
        close(a1.base_value, 0.25),
        "A1 Dendro DMG Bonus should be 0.25, got {}",
        a1.base_value
    );
    assert_eq!(a1.target, BuffTarget::OnlySelf);
}

#[test]
fn baizhu_buff_count_with_a1() {
    let buffs = find_talent_buffs("baizhu").unwrap();
    // Was 9 (7 A4 + 1 C4 + 1 C6), now +1 (A1) = 10
    assert_eq!(buffs.len(), 10, "baizhu should have 10 buffs");
}

// ===== Issue #124: Kinich C1 =====

#[test]
fn kinich_c1_scalespiker_crit_dmg() {
    let buffs = find_talent_buffs("kinich").unwrap();
    let c1 = buffs
        .iter()
        .find(|b| b.stat == BuffableStat::CritDmg && b.source == TalentBuffSource::Constellation(1))
        .expect("Kinich C1 CritDmg buff missing");
    assert!(
        close(c1.base_value, 1.00),
        "C1 CritDmg should be 1.00, got {}",
        c1.base_value
    );
    assert_eq!(c1.min_constellation, 1);
    assert_eq!(c1.target, BuffTarget::OnlySelf);
}

#[test]
fn kinich_buff_count_with_c1() {
    let buffs = find_talent_buffs("kinich").unwrap();
    // Was 3 (C2 Dendro RES, C2 DMG, C4 Burst DMG), now +1 (C1 CritDmg) = 4
    assert_eq!(buffs.len(), 4, "kinich should have 4 buffs");
}

// ===== Issue #125: Lauma A1 moonsign =====

#[test]
fn lauma_a1_nascent_gleam_bloom_crit() {
    let bloom_entry = LAUMA_TALENT_ENHANCEMENTS
        .iter()
        .find(|e| {
            e.required_level == MoonsignLevel::NascentGleam
                && matches!(
                    &e.effect,
                    MoonsignTalentEffect::GrantReactionCrit { reaction, .. }
                    if *reaction == Reaction::Bloom
                )
        })
        .expect("Lauma NascentGleam Bloom GrantReactionCrit entry missing");
    if let MoonsignTalentEffect::GrantReactionCrit {
        crit_rate,
        crit_dmg,
        ..
    } = bloom_entry.effect
    {
        assert!(
            close(crit_rate, 0.15),
            "Bloom crit_rate should be 0.15, got {}",
            crit_rate
        );
        assert!(
            close(crit_dmg, 1.0),
            "Bloom crit_dmg should be 1.0, got {}",
            crit_dmg
        );
    }
}

#[test]
fn lauma_a1_nascent_gleam_hyperbloom_crit() {
    let entry = LAUMA_TALENT_ENHANCEMENTS
        .iter()
        .find(|e| {
            e.required_level == MoonsignLevel::NascentGleam
                && matches!(
                    &e.effect,
                    MoonsignTalentEffect::GrantReactionCrit { reaction, .. }
                    if *reaction == Reaction::Hyperbloom
                )
        })
        .expect("Lauma NascentGleam Hyperbloom GrantReactionCrit entry missing");
    if let MoonsignTalentEffect::GrantReactionCrit {
        crit_rate,
        crit_dmg,
        ..
    } = entry.effect
    {
        assert!(
            close(crit_rate, 0.15),
            "Hyperbloom crit_rate should be 0.15, got {}",
            crit_rate
        );
        assert!(
            close(crit_dmg, 1.0),
            "Hyperbloom crit_dmg should be 1.0, got {}",
            crit_dmg
        );
    }
}

#[test]
fn lauma_a1_nascent_gleam_burgeon_crit() {
    let entry = LAUMA_TALENT_ENHANCEMENTS
        .iter()
        .find(|e| {
            e.required_level == MoonsignLevel::NascentGleam
                && matches!(
                    &e.effect,
                    MoonsignTalentEffect::GrantReactionCrit { reaction, .. }
                    if *reaction == Reaction::Burgeon
                )
        })
        .expect("Lauma NascentGleam Burgeon GrantReactionCrit entry missing");
    if let MoonsignTalentEffect::GrantReactionCrit {
        crit_rate,
        crit_dmg,
        ..
    } = entry.effect
    {
        assert!(
            close(crit_rate, 0.15),
            "Burgeon crit_rate should be 0.15, got {}",
            crit_rate
        );
        assert!(
            close(crit_dmg, 1.0),
            "Burgeon crit_dmg should be 1.0, got {}",
            crit_dmg
        );
    }
}

#[test]
fn lauma_a1_ascendant_gleam_lunarbloom_crit_bonus() {
    let entry = LAUMA_TALENT_ENHANCEMENTS
        .iter()
        .find(|e| {
            e.required_level == MoonsignLevel::AscendantGleam
                && matches!(
                    &e.effect,
                    MoonsignTalentEffect::GrantReactionCrit { reaction, .. }
                    if *reaction == Reaction::LunarBloom
                )
        })
        .expect("Lauma AscendantGleam LunarBloom GrantReactionCrit entry missing");
    if let MoonsignTalentEffect::GrantReactionCrit {
        crit_rate,
        crit_dmg,
        ..
    } = entry.effect
    {
        assert!(
            close(crit_rate, 0.10),
            "LunarBloom crit_rate should be 0.10, got {}",
            crit_rate
        );
        assert!(
            close(crit_dmg, 0.20),
            "LunarBloom crit_dmg should be 0.20, got {}",
            crit_dmg
        );
    }
}

#[test]
fn lauma_moonsign_enhancements_count() {
    // Should have 4 entries: Bloom/Hyperbloom/Burgeon at NascentGleam + LunarBloom at AscendantGleam
    assert_eq!(
        LAUMA_TALENT_ENHANCEMENTS.len(),
        4,
        "Lauma should have 4 moonsign enhancement entries"
    );
}
