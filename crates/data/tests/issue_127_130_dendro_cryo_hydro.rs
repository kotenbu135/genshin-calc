/// Issues #127-130: Traveler(Dendro) A4, Shenhe/Eula RES shred scaling, Sigewinne/Tartaglia values, Skirk confirmation
/// Source: honeyhunter-mirror/md/characters/
use genshin_calc_core::{BuffTarget, BuffableStat, Element};
use genshin_calc_data::{
    find_character,
    talent_buffs::{TalentBuffSource, find_talent_buffs},
};

const EPS: f64 = 1e-4;

// ===== Issue #127: Traveler(Dendro) A1/A4 label swap + A4 unimplemented =====

/// A1 buff should be named "Verdant Overgrowth" (EM+60 in Lamp)
#[test]
fn test_traveler_dendro_a1_buff_name() {
    let buffs = find_talent_buffs("traveler_dendro").expect("traveler_dendro buffs not found");
    let a1_buff = buffs
        .iter()
        .find(|b| b.source == TalentBuffSource::AscensionPassive(1))
        .expect("A1 buff not found");
    assert_eq!(
        a1_buff.name, "Verdant Overgrowth",
        "Traveler Dendro A1 buff name should be 'Verdant Overgrowth', got: {}",
        a1_buff.name
    );
}

/// A4 Skill DMG buff should be named "Verdant Luxury" with SkillDmgBonus stat and ratio 0.0015/EM
#[test]
fn test_traveler_dendro_a4_skill_dmg_buff_exists() {
    let buffs = find_talent_buffs("traveler_dendro").expect("traveler_dendro buffs not found");
    let a4_skill = buffs.iter().find(|b| {
        b.source == TalentBuffSource::AscensionPassive(4) && b.stat == BuffableStat::SkillDmgBonus
    });
    assert!(
        a4_skill.is_some(),
        "Traveler Dendro A4 SkillDmgBonus buff not found"
    );
    let a4_skill = a4_skill.unwrap();
    assert_eq!(
        a4_skill.name, "Verdant Luxury",
        "A4 Skill DMG buff name should be 'Verdant Luxury', got: {}",
        a4_skill.name
    );
    assert!(
        (a4_skill.base_value - 0.0015).abs() < EPS,
        "A4 Skill DMG ratio should be 0.0015 per EM, got {}",
        a4_skill.base_value
    );
}

/// A4 Burst DMG buff should be "Verdant Luxury" with BurstDmgBonus stat and ratio 0.001/EM
#[test]
fn test_traveler_dendro_a4_burst_dmg_buff_exists() {
    let buffs = find_talent_buffs("traveler_dendro").expect("traveler_dendro buffs not found");
    let a4_burst = buffs.iter().find(|b| {
        b.source == TalentBuffSource::AscensionPassive(4) && b.stat == BuffableStat::BurstDmgBonus
    });
    assert!(
        a4_burst.is_some(),
        "Traveler Dendro A4 BurstDmgBonus buff not found"
    );
    let a4_burst = a4_burst.unwrap();
    assert_eq!(
        a4_burst.name, "Verdant Luxury",
        "A4 Burst DMG buff name should be 'Verdant Luxury', got: {}",
        a4_burst.name
    );
    assert!(
        (a4_burst.base_value - 0.001).abs() < EPS,
        "A4 Burst DMG ratio should be 0.001 per EM, got {}",
        a4_burst.base_value
    );
}

/// Total buffs should now be 4 (1 A1 EM + 2 A4 + 1 C6)
#[test]
fn test_traveler_dendro_buff_count() {
    let buffs = find_talent_buffs("traveler_dendro").expect("traveler_dendro buffs not found");
    assert_eq!(
        buffs.len(),
        4,
        "Traveler Dendro should have 4 buffs (A1 EM + 2xA4 + C6), got {}",
        buffs.len()
    );
}

// ===== Issue #128: Eula RES shred talent-level-dependent =====

/// Eula Cryo RES shred should be talent-level dependent (Lv1 = 16%)
#[test]
fn test_eula_cryo_res_shred_scales_with_talent() {
    let buffs = find_talent_buffs("eula").expect("eula buffs not found");
    let cryo_shred = buffs
        .iter()
        .find(|b| b.stat == BuffableStat::ElementalResReduction(Element::Cryo))
        .expect("Eula Cryo RES shred buff not found");
    assert!(
        cryo_shred.scales_with_talent,
        "Eula Cryo RES shred should scale with talent level"
    );
    assert!(
        cryo_shred.talent_scaling.is_some(),
        "Eula Cryo RES shred should have talent_scaling array"
    );
    // Lv1 should be 16% = 0.16
    let lv1 = cryo_shred.talent_scaling.unwrap()[0];
    assert!(
        (lv1 - 0.16).abs() < EPS,
        "Eula Cryo RES shred Lv1 should be 0.16, got {}",
        lv1
    );
    // Lv10 (cap) should be 25% = 0.25
    let lv10 = cryo_shred.talent_scaling.unwrap()[9];
    assert!(
        (lv10 - 0.25).abs() < EPS,
        "Eula Cryo RES shred Lv10 should be 0.25, got {}",
        lv10
    );
}

/// Eula Physical RES shred should be talent-level dependent (Lv1 = 16%)
#[test]
fn test_eula_physical_res_shred_scales_with_talent() {
    let buffs = find_talent_buffs("eula").expect("eula buffs not found");
    let phys_shred = buffs
        .iter()
        .find(|b| b.stat == BuffableStat::PhysicalResReduction)
        .expect("Eula Physical RES shred buff not found");
    assert!(
        phys_shred.scales_with_talent,
        "Eula Physical RES shred should scale with talent level"
    );
    assert!(
        phys_shred.talent_scaling.is_some(),
        "Eula Physical RES shred should have talent_scaling array"
    );
    let lv1 = phys_shred.talent_scaling.unwrap()[0];
    assert!(
        (lv1 - 0.16).abs() < EPS,
        "Eula Physical RES shred Lv1 should be 0.16, got {}",
        lv1
    );
}

// ===== Issue #128: Shenhe burst RES shred missing =====

/// Shenhe should have a Cryo RES shred from Elemental Burst
#[test]
fn test_shenhe_burst_cryo_res_shred_exists() {
    let buffs = find_talent_buffs("shenhe").expect("shenhe buffs not found");
    let cryo_shred = buffs.iter().find(|b| {
        b.source == TalentBuffSource::ElementalBurst
            && b.stat == BuffableStat::ElementalResReduction(Element::Cryo)
    });
    assert!(
        cryo_shred.is_some(),
        "Shenhe should have ElementalBurst Cryo RES shred buff"
    );
    let cryo_shred = cryo_shred.unwrap();
    assert!(
        cryo_shred.scales_with_talent,
        "Shenhe Burst Cryo RES shred should scale with talent"
    );
    // Lv1 = 6%, Lv10 = 15%
    let lv1 = cryo_shred.talent_scaling.unwrap()[0];
    assert!(
        (lv1 - 0.06).abs() < EPS,
        "Shenhe Burst Cryo RES shred Lv1 should be 0.06, got {}",
        lv1
    );
    let lv10 = cryo_shred.talent_scaling.unwrap()[9];
    assert!(
        (lv10 - 0.15).abs() < EPS,
        "Shenhe Burst Cryo RES shred Lv10 should be 0.15, got {}",
        lv10
    );
}

/// Shenhe should have a Physical RES shred from Elemental Burst
#[test]
fn test_shenhe_burst_physical_res_shred_exists() {
    let buffs = find_talent_buffs("shenhe").expect("shenhe buffs not found");
    let phys_shred = buffs.iter().find(|b| {
        b.source == TalentBuffSource::ElementalBurst && b.stat == BuffableStat::PhysicalResReduction
    });
    assert!(
        phys_shred.is_some(),
        "Shenhe should have ElementalBurst Physical RES shred buff"
    );
    let phys_shred = phys_shred.unwrap();
    let lv1 = phys_shred.talent_scaling.unwrap()[0];
    assert!(
        (lv1 - 0.06).abs() < EPS,
        "Shenhe Burst Physical RES shred Lv1 should be 0.06, got {}",
        lv1
    );
}

/// Shenhe total buff count should be 14 (12 existing + 2 burst RES shred)
#[test]
fn test_shenhe_buff_count() {
    let buffs = find_talent_buffs("shenhe").expect("shenhe buffs not found");
    assert_eq!(
        buffs.len(),
        14,
        "Shenhe should have 14 buffs (12 existing + 2 burst RES shred), got {}",
        buffs.len()
    );
}

// ===== Issue #129: Sigewinne burst scaling value errors =====

/// Sigewinne burst Lv1 should be 0.1177
#[test]
fn test_sigewinne_burst_lv1() {
    let char_data = find_character("sigewinne").expect("sigewinne not found");
    let burst = char_data
        .talents
        .elemental_burst
        .scalings
        .first()
        .expect("Sigewinne burst scaling should exist");
    assert!(
        (burst.values[0] - 0.1177).abs() < EPS,
        "Sigewinne burst Lv1 should be 0.1177, got {}",
        burst.values[0]
    );
}

/// Sigewinne burst Lv8 should be 0.1766
#[test]
fn test_sigewinne_burst_lv8() {
    let char_data = find_character("sigewinne").expect("sigewinne not found");
    let burst = char_data
        .talents
        .elemental_burst
        .scalings
        .first()
        .expect("Sigewinne burst scaling should exist");
    assert!(
        (burst.values[7] - 0.1883).abs() < EPS,
        "Sigewinne burst Lv8 should be 0.1883, got {}",
        burst.values[7]
    );
}

/// Sigewinne burst Lv15 should be 0.2796
#[test]
fn test_sigewinne_burst_lv15() {
    let char_data = find_character("sigewinne").expect("sigewinne not found");
    let burst = char_data
        .talents
        .elemental_burst
        .scalings
        .first()
        .expect("Sigewinne burst scaling should exist");
    assert!(
        (burst.values[14] - 0.2796).abs() < EPS,
        "Sigewinne burst Lv15 should be 0.2796, got {}",
        burst.values[14]
    );
}

// ===== Issue #129: Tartaglia plunge value errors =====

/// Tartaglia plunge Lv4 (index 3) should be 0.8177
#[test]
fn test_tartaglia_plunge_lv4() {
    let char_data = find_character("tartaglia").expect("tartaglia not found");
    // plunging[0] = TARTAGLIA_PLUNGE (落下期間のダメージ)
    let plunge = &char_data.talents.normal_attack.plunging[0];
    assert!(
        (plunge.values[3] - 0.8177).abs() < EPS,
        "Tartaglia plunge Lv4 should be 0.8177, got {}",
        plunge.values[3]
    );
}

/// Tartaglia plunge low Lv8 (index 7) should be 2.1851
#[test]
fn test_tartaglia_plunge_low_lv8() {
    let char_data = find_character("tartaglia").expect("tartaglia not found");
    // plunging[1] = TARTAGLIA_PLUNGE_LOW (低空落下攻撃ダメージ)
    let plunge_low = &char_data.talents.normal_attack.plunging[1];
    assert!(
        (plunge_low.values[7] - 2.1851).abs() < EPS,
        "Tartaglia plunge_low Lv8 should be 2.1851, got {}",
        plunge_low.values[7]
    );
}

// ===== Issue #130: Skirk charged value confirmation (already correct) =====

/// Skirk Charged Lv8 (index 7) should be 1.1422
#[test]
fn test_skirk_charged_lv8() {
    let char_data = find_character("skirk").expect("skirk not found");
    let charged = char_data
        .talents
        .normal_attack
        .charged
        .first()
        .expect("Skirk charged attack scaling should exist");
    assert!(
        (charged.values[7] - 1.1422).abs() < EPS,
        "Skirk Charged Lv8 should be 1.1422, got {}",
        charged.values[7]
    );
}

/// Skirk Charged Lv14 (index 13) should be 1.6939
#[test]
fn test_skirk_charged_lv14() {
    let char_data = find_character("skirk").expect("skirk not found");
    let charged = char_data
        .talents
        .normal_attack
        .charged
        .first()
        .expect("Skirk charged attack scaling should exist");
    assert!(
        (charged.values[13] - 1.6939).abs() < EPS,
        "Skirk Charged Lv14 should be 1.6939, got {}",
        charged.values[13]
    );
}
