/// Issues #106-113: Electro/Anemo buff fixes and additions
/// TDD: Tests written first (RED), then implementation (GREEN)
use genshin_calc_core::{BuffTarget, BuffableStat, Element};
use genshin_calc_data::talent_buffs::{
    Activation, ManualCondition, TalentBuffSource, find_talent_buffs,
};

const EPS: f64 = 1e-6;

// ===== Issue #106: Ororon C2 base 8% Electro DMG Bonus =====

/// C2 should have a base 8% Electro DMG Bonus (in addition to the stacks-based 32%)
#[test]
fn test_ororon_c2_has_base_8pct_electro_dmg() {
    let buffs = find_talent_buffs("ororon").expect("ororon buffs not found");
    // There should be a non-stacks Electro DMG bonus from C2 with value 0.08
    let base_buff = buffs.iter().find(|b| {
        b.stat == BuffableStat::ElementalDmgBonus(Element::Electro)
            && b.source == TalentBuffSource::Constellation(2)
            && (b.base_value - 0.08).abs() < EPS
    });
    assert!(
        base_buff.is_some(),
        "Ororon C2 should have a base 8% Electro DMG Bonus entry, but found none"
    );
}

/// The base 8% entry should be OnlySelf and not require Toggle (or is Toggle)
#[test]
fn test_ororon_c2_base_8pct_is_self_toggle() {
    let buffs = find_talent_buffs("ororon").expect("ororon buffs not found");
    let base_buff = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::ElementalDmgBonus(Element::Electro)
                && b.source == TalentBuffSource::Constellation(2)
                && (b.base_value - 0.08).abs() < EPS
        })
        .expect("Ororon C2 base 8% Electro DMG buff not found");
    assert_eq!(base_buff.target, BuffTarget::OnlySelf);
    assert_eq!(base_buff.min_constellation, 2);
}

/// Ororon should now have 3 buffs total (base 8%, stacks 32%, C6 ATK)
#[test]
fn test_ororon_total_buff_count() {
    let buffs = find_talent_buffs("ororon").expect("ororon buffs not found");
    assert_eq!(
        buffs.len(),
        3,
        "Ororon should have 3 buffs (C2 base, C2 stacks, C6)"
    );
}

// ===== Issue #107: Flins A1 "Symphony of Winter" Lunar-Charged DMG +20% =====

/// Flins A1 should grant Lunar-Charged DMG +20% (TransformativeBonus, OnlySelf, Toggle)
#[test]
fn test_flins_a1_symphony_of_winter_lunar_charged_dmg() {
    let buffs = find_talent_buffs("flins").expect("flins buffs not found");
    let a1_buff = buffs
        .iter()
        .find(|b| b.source == TalentBuffSource::AscensionPassive(1))
        .expect("Flins A1 buff not found");
    assert_eq!(
        a1_buff.stat,
        BuffableStat::TransformativeBonus,
        "Flins A1 should use TransformativeBonus for Lunar-Charged DMG"
    );
    assert!(
        (a1_buff.base_value - 0.20).abs() < EPS,
        "Flins A1 should be +20%"
    );
    assert_eq!(a1_buff.target, BuffTarget::OnlySelf);
    assert_eq!(
        a1_buff.activation,
        Some(Activation::Manual(ManualCondition::Toggle))
    );
}

/// Flins should now have 7 buffs (A1 new, A4, C4 ATK, C4 A4 enhance, C2 res, C6 self, C6 team)
#[test]
fn test_flins_total_buff_count() {
    let buffs = find_talent_buffs("flins").expect("flins buffs not found");
    assert_eq!(
        buffs.len(),
        7,
        "Flins should have 7 buffs (A1 + existing 6)"
    );
}

// ===== Issue #108: Kujou Sara C6 CritDmg → ElementalCritDmg(Electro) =====

/// Sara C6 should use ElementalCritDmg(Electro) not generic CritDmg
#[test]
fn test_sara_c6_uses_elemental_crit_dmg_electro() {
    let buffs = find_talent_buffs("kujou_sara").expect("kujou_sara buffs not found");
    let c6_buff = buffs
        .iter()
        .find(|b| b.source == TalentBuffSource::Constellation(6))
        .expect("Sara C6 buff not found");
    assert_eq!(
        c6_buff.stat,
        BuffableStat::ElementalCritDmg(Element::Electro),
        "Sara C6 should use ElementalCritDmg(Electro), not CritDmg"
    );
    assert!(
        (c6_buff.base_value - 0.60).abs() < EPS,
        "Sara C6 should be +60%"
    );
}

/// Sara C6 should NOT use generic CritDmg anymore
#[test]
fn test_sara_c6_no_generic_crit_dmg() {
    let buffs = find_talent_buffs("kujou_sara").expect("kujou_sara buffs not found");
    let generic_crit_dmg = buffs.iter().find(|b| {
        b.stat == BuffableStat::CritDmg && b.source == TalentBuffSource::Constellation(6)
    });
    assert!(
        generic_crit_dmg.is_none(),
        "Sara C6 should not have generic CritDmg entry"
    );
}

// ===== Issue #109: Ifa A4 EM+80 =====

/// Ifa A4 "Mutual Aid Agreement" should grant EM+80 (OnlySelf, Toggle)
#[test]
fn test_ifa_a4_em_bonus() {
    let buffs = find_talent_buffs("ifa").expect("ifa buffs not found");
    let a4_buff = buffs
        .iter()
        .find(|b| b.source == TalentBuffSource::AscensionPassive(4))
        .expect("Ifa A4 buff not found");
    assert_eq!(
        a4_buff.stat,
        BuffableStat::ElementalMastery,
        "Ifa A4 should buff ElementalMastery"
    );
    assert!(
        (a4_buff.base_value - 80.0).abs() < EPS,
        "Ifa A4 should be EM+80"
    );
    assert_eq!(a4_buff.target, BuffTarget::OnlySelf);
    assert_eq!(
        a4_buff.activation,
        Some(Activation::Manual(ManualCondition::Toggle))
    );
}

/// Ifa should now have 2 buffs (A4 + existing C4)
#[test]
fn test_ifa_total_buff_count() {
    let buffs = find_talent_buffs("ifa").expect("ifa buffs not found");
    assert_eq!(buffs.len(), 2, "Ifa should have 2 buffs (A4 + C4)");
}

// ===== Issue #110: Jean C1 Skill DMG+40% =====

/// Jean C1 "Spiraling Tempest" should grant SkillDmgBonus+40% (OnlySelf, Toggle)
#[test]
fn test_jean_c1_skill_dmg_bonus() {
    let buffs = find_talent_buffs("jean").expect("jean buffs not found");
    let c1_buff = buffs
        .iter()
        .find(|b| b.source == TalentBuffSource::Constellation(1))
        .expect("Jean C1 buff not found");
    assert_eq!(
        c1_buff.stat,
        BuffableStat::SkillDmgBonus,
        "Jean C1 should buff SkillDmgBonus"
    );
    assert!(
        (c1_buff.base_value - 0.40).abs() < EPS,
        "Jean C1 should be +40%"
    );
    assert_eq!(c1_buff.target, BuffTarget::OnlySelf);
    assert_eq!(c1_buff.min_constellation, 1);
    assert_eq!(
        c1_buff.activation,
        Some(Activation::Manual(ManualCondition::Toggle))
    );
}

/// Jean should now have 2 buffs (existing C4 RES shred + new C1 Skill DMG)
#[test]
fn test_jean_total_buff_count() {
    let buffs = find_talent_buffs("jean").expect("jean buffs not found");
    assert_eq!(buffs.len(), 2, "Jean should have 2 buffs (C4 + C1)");
}

// ===== Issue #111: Lynette A1 Burst DMG+15% =====

/// Lynette A1 "Loci of Actualization" should grant BurstDmgBonus+15% (OnlySelf, Toggle)
#[test]
fn test_lynette_a1_burst_dmg_bonus() {
    let buffs = find_talent_buffs("lynette").expect("lynette buffs not found");
    let a1_buff = buffs
        .iter()
        .find(|b| b.source == TalentBuffSource::AscensionPassive(1))
        .expect("Lynette A1 buff not found");
    assert_eq!(
        a1_buff.stat,
        BuffableStat::BurstDmgBonus,
        "Lynette A1 should buff BurstDmgBonus"
    );
    assert!(
        (a1_buff.base_value - 0.15).abs() < EPS,
        "Lynette A1 should be +15%"
    );
    assert_eq!(a1_buff.target, BuffTarget::OnlySelf);
    assert_eq!(
        a1_buff.activation,
        Some(Activation::Manual(ManualCondition::Toggle))
    );
}

/// Lynette should now have 3 buffs (A1 + existing A4 + C6)
#[test]
fn test_lynette_total_buff_count() {
    let buffs = find_talent_buffs("lynette").expect("lynette buffs not found");
    assert_eq!(buffs.len(), 3, "Lynette should have 3 buffs (A1 + A4 + C6)");
}

// ===== Issue #112: Sucrose C6 Elemental DMG+20% team (absorbed element) =====

/// Sucrose C6 should have Pyro DMG+20% team entry (Toggle)
#[test]
fn test_sucrose_c6_pyro_dmg_bonus() {
    let buffs = find_talent_buffs("sucrose").expect("sucrose buffs not found");
    let buff = buffs.iter().find(|b| {
        b.stat == BuffableStat::ElementalDmgBonus(Element::Pyro)
            && b.source == TalentBuffSource::Constellation(6)
    });
    assert!(
        buff.is_some(),
        "Sucrose C6 should have Pyro DMG Bonus entry"
    );
    let buff = buff.unwrap();
    assert!((buff.base_value - 0.20).abs() < EPS);
    assert_eq!(buff.target, BuffTarget::Team);
    assert_eq!(buff.min_constellation, 6);
    assert_eq!(
        buff.activation,
        Some(Activation::Manual(ManualCondition::Toggle))
    );
}

/// Sucrose C6 should have Hydro DMG+20% team entry (Toggle)
#[test]
fn test_sucrose_c6_hydro_dmg_bonus() {
    let buffs = find_talent_buffs("sucrose").expect("sucrose buffs not found");
    let buff = buffs.iter().find(|b| {
        b.stat == BuffableStat::ElementalDmgBonus(Element::Hydro)
            && b.source == TalentBuffSource::Constellation(6)
    });
    assert!(
        buff.is_some(),
        "Sucrose C6 should have Hydro DMG Bonus entry"
    );
    let buff = buff.unwrap();
    assert!((buff.base_value - 0.20).abs() < EPS);
    assert_eq!(buff.target, BuffTarget::Team);
    assert_eq!(
        buff.activation,
        Some(Activation::Manual(ManualCondition::Toggle))
    );
}

/// Sucrose C6 should have Cryo DMG+20% team entry (Toggle)
#[test]
fn test_sucrose_c6_cryo_dmg_bonus() {
    let buffs = find_talent_buffs("sucrose").expect("sucrose buffs not found");
    let buff = buffs.iter().find(|b| {
        b.stat == BuffableStat::ElementalDmgBonus(Element::Cryo)
            && b.source == TalentBuffSource::Constellation(6)
    });
    assert!(
        buff.is_some(),
        "Sucrose C6 should have Cryo DMG Bonus entry"
    );
    let buff = buff.unwrap();
    assert!((buff.base_value - 0.20).abs() < EPS);
    assert_eq!(buff.target, BuffTarget::Team);
    assert_eq!(
        buff.activation,
        Some(Activation::Manual(ManualCondition::Toggle))
    );
}

/// Sucrose C6 should have Electro DMG+20% team entry (Toggle)
#[test]
fn test_sucrose_c6_electro_dmg_bonus() {
    let buffs = find_talent_buffs("sucrose").expect("sucrose buffs not found");
    let buff = buffs.iter().find(|b| {
        b.stat == BuffableStat::ElementalDmgBonus(Element::Electro)
            && b.source == TalentBuffSource::Constellation(6)
    });
    assert!(
        buff.is_some(),
        "Sucrose C6 should have Electro DMG Bonus entry"
    );
    let buff = buff.unwrap();
    assert!((buff.base_value - 0.20).abs() < EPS);
    assert_eq!(buff.target, BuffTarget::Team);
    assert_eq!(
        buff.activation,
        Some(Activation::Manual(ManualCondition::Toggle))
    );
}

/// Sucrose should now have 6 buffs (A1 + A4 + 4x C6)
#[test]
fn test_sucrose_total_buff_count() {
    let buffs = find_talent_buffs("sucrose").expect("sucrose buffs not found");
    assert_eq!(
        buffs.len(),
        6,
        "Sucrose should have 6 buffs (A1 + A4 + 4 C6 elemental)"
    );
}

// ===== Issue #113: Venti C6 swirled element RES-20% =====

/// Venti C6 should have Pyro RES-20% entry (Toggle)
#[test]
fn test_venti_c6_pyro_res_shred() {
    let buffs = find_talent_buffs("venti").expect("venti buffs not found");
    let buff = buffs.iter().find(|b| {
        b.stat == BuffableStat::ElementalResReduction(Element::Pyro)
            && b.source == TalentBuffSource::Constellation(6)
    });
    assert!(buff.is_some(), "Venti C6 should have Pyro RES shred entry");
    let buff = buff.unwrap();
    assert!((buff.base_value - 0.20).abs() < EPS);
    assert_eq!(buff.target, BuffTarget::Team);
    assert_eq!(buff.min_constellation, 6);
    assert_eq!(
        buff.activation,
        Some(Activation::Manual(ManualCondition::Toggle))
    );
}

/// Venti C6 should have Hydro RES-20% entry (Toggle)
#[test]
fn test_venti_c6_hydro_res_shred() {
    let buffs = find_talent_buffs("venti").expect("venti buffs not found");
    let buff = buffs.iter().find(|b| {
        b.stat == BuffableStat::ElementalResReduction(Element::Hydro)
            && b.source == TalentBuffSource::Constellation(6)
    });
    assert!(buff.is_some(), "Venti C6 should have Hydro RES shred entry");
    let buff = buff.unwrap();
    assert!((buff.base_value - 0.20).abs() < EPS);
    assert_eq!(buff.target, BuffTarget::Team);
    assert_eq!(
        buff.activation,
        Some(Activation::Manual(ManualCondition::Toggle))
    );
}

/// Venti C6 should have Cryo RES-20% entry (Toggle)
#[test]
fn test_venti_c6_cryo_res_shred() {
    let buffs = find_talent_buffs("venti").expect("venti buffs not found");
    let buff = buffs.iter().find(|b| {
        b.stat == BuffableStat::ElementalResReduction(Element::Cryo)
            && b.source == TalentBuffSource::Constellation(6)
    });
    assert!(buff.is_some(), "Venti C6 should have Cryo RES shred entry");
    let buff = buff.unwrap();
    assert!((buff.base_value - 0.20).abs() < EPS);
    assert_eq!(buff.target, BuffTarget::Team);
    assert_eq!(
        buff.activation,
        Some(Activation::Manual(ManualCondition::Toggle))
    );
}

/// Venti C6 should have Electro RES-20% entry (Toggle)
#[test]
fn test_venti_c6_electro_res_shred() {
    let buffs = find_talent_buffs("venti").expect("venti buffs not found");
    let buff = buffs.iter().find(|b| {
        b.stat == BuffableStat::ElementalResReduction(Element::Electro)
            && b.source == TalentBuffSource::Constellation(6)
    });
    assert!(
        buff.is_some(),
        "Venti C6 should have Electro RES shred entry"
    );
    let buff = buff.unwrap();
    assert!((buff.base_value - 0.20).abs() < EPS);
    assert_eq!(buff.target, BuffTarget::Team);
    assert_eq!(
        buff.activation,
        Some(Activation::Manual(ManualCondition::Toggle))
    );
}

/// Venti should now have 8 buffs (C2 Anemo, C2 Phys, C4 Anemo, C6 Anemo, C6 Pyro, C6 Hydro, C6 Cryo, C6 Electro)
#[test]
fn test_venti_total_buff_count() {
    let buffs = find_talent_buffs("venti").expect("venti buffs not found");
    assert_eq!(
        buffs.len(),
        8,
        "Venti should have 8 buffs (C2x2 + C4 + C6 Anemo + C6 4 elements)"
    );
}
