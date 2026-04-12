/// Issues #100-105: Cryo/Hydro buff name swap, scaling value fixes, element fix, buff target/stat fixes
/// Source: honeyhunter-mirror/md/characters/ and talent_buffs/
use genshin_calc_core::{BuffTarget, BuffableStat, Element};
use genshin_calc_data::{
    find_character,
    talent_buffs::{TalentBuffSource, find_talent_buffs},
};

const EPS: f64 = 1e-4;

// ===== Issue #100: Neuvillette A1/A4 buff name swap =====

/// A1 buff should be named "Heir to the Ancient Sea's Authority"
#[test]
fn test_neuvillette_a1_buff_name() {
    let buffs = find_talent_buffs("neuvillette").expect("neuvillette buffs not found");
    let a1_buff = buffs
        .iter()
        .find(|b| b.source == TalentBuffSource::AscensionPassive(1))
        .expect("A1 buff not found");
    assert_eq!(
        a1_buff.name, "Heir to the Ancient Sea's Authority",
        "Neuvillette A1 buff name should be 'Heir to the Ancient Sea's Authority', got: {}",
        a1_buff.name
    );
}

/// A4 buff should be named "Discipline of the Supreme Arbitration"
#[test]
fn test_neuvillette_a4_buff_name() {
    let buffs = find_talent_buffs("neuvillette").expect("neuvillette buffs not found");
    let a4_buff = buffs
        .iter()
        .find(|b| b.source == TalentBuffSource::AscensionPassive(4))
        .expect("A4 buff not found");
    assert_eq!(
        a4_buff.name, "Discipline of the Supreme Arbitration",
        "Neuvillette A4 buff name should be 'Discipline of the Supreme Arbitration', got: {}",
        a4_buff.name
    );
}

// ===== Issue #101: Wriothesley Charged Attack Lv8 value error =====

/// Wriothesley Charged Attack Lv8 (index 7) should be 2.4474
#[test]
fn test_wriothesley_charged_attack_lv8() {
    let char_data = find_character("wriothesley").expect("wriothesley not found");
    let charged = char_data
        .talents
        .normal_attack
        .charged
        .first()
        .expect("Wriothesley charged attack scaling should exist");
    assert!(
        (charged.values[7] - 2.4474).abs() < EPS,
        "Wriothesley Charged Attack Lv8 should be 2.4474, got {}",
        charged.values[7]
    );
}

// ===== Issue #102: Layla Charged Attack 2 Lv14 + N1 Lv1 value errors =====

/// Layla Charged Attack 2 Lv14 (index 13) should be 1.3320
#[test]
fn test_layla_charged_attack_2_lv14() {
    let char_data = find_character("layla").expect("layla not found");
    let charged2 = char_data
        .talents
        .normal_attack
        .charged
        .get(1)
        .expect("Layla charged attack 2 scaling should exist");
    assert!(
        (charged2.values[13] - 1.3320).abs() < EPS,
        "Layla Charged Attack 2 Lv14 should be 1.3320, got {}",
        charged2.values[13]
    );
}

/// Layla N1 Lv1 (index 0) should be 0.5122
#[test]
fn test_layla_normal_attack_1_lv1() {
    let char_data = find_character("layla").expect("layla not found");
    let n1 = char_data
        .talents
        .normal_attack
        .hits
        .first()
        .expect("Layla normal attack 1 scaling should exist");
    assert!(
        (n1.values[0] - 0.5122).abs() < EPS,
        "Layla N1 Lv1 should be 0.5122, got {}",
        n1.values[0]
    );
}

// ===== Issue #103: Freminet Skill Lv4 Shattering Pressure element wrong =====

/// Freminet SKILL_LEVEL4 (Shattering Pressure Lv4) should have damage_element = None (Physical)
#[test]
fn test_freminet_skill_level4_is_physical() {
    let char_data = find_character("freminet").expect("freminet not found");
    let level4 = char_data
        .talents
        .elemental_skill
        .scalings
        .get(1)
        .expect("Freminet skill level4 scaling should exist");
    assert!(
        level4.damage_element.is_none(),
        "Freminet Shattering Pressure Lv4 should be Physical (damage_element = None), got {:?}",
        level4.damage_element
    );
}

// ===== Issue #104: Escoffier C1/C2 buff target/type wrong =====

/// Escoffier C1 should use BuffableStat::ElementalCritDmg(Element::Cryo)
#[test]
fn test_escoffier_c1_stat_is_cryo_crit_dmg() {
    let buffs = find_talent_buffs("escoffier").expect("escoffier buffs not found");
    let c1_buff = buffs
        .iter()
        .find(|b| b.source == TalentBuffSource::Constellation(1))
        .expect("Escoffier C1 buff not found");
    assert_eq!(
        c1_buff.stat,
        BuffableStat::ElementalCritDmg(Element::Cryo),
        "Escoffier C1 stat should be ElementalCritDmg(Cryo), got {:?}",
        c1_buff.stat
    );
}

/// Escoffier C2 should use BuffTarget::TeamExcludeSelf
#[test]
fn test_escoffier_c2_target_is_team_exclude_self() {
    let buffs = find_talent_buffs("escoffier").expect("escoffier buffs not found");
    let c2_buff = buffs
        .iter()
        .find(|b| b.source == TalentBuffSource::Constellation(2))
        .expect("Escoffier C2 buff not found");
    assert_eq!(
        c2_buff.target,
        BuffTarget::TeamExcludeSelf,
        "Escoffier C2 target should be TeamExcludeSelf, got {:?}",
        c2_buff.target
    );
}

// ===== Issue #105: Mika C6 PhysicalCritDmg + remove extra buff =====

/// Mika C6 CRIT DMG buff should use BuffableStat::PhysicalCritDmg
#[test]
fn test_mika_c6_crit_dmg_is_physical_crit_dmg() {
    let buffs = find_talent_buffs("mika").expect("mika buffs not found");
    let c6_crit = buffs
        .iter()
        .find(|b| {
            b.source == TalentBuffSource::Constellation(6)
                && b.stat == BuffableStat::PhysicalCritDmg
        })
        .expect("Mika C6 PhysicalCritDmg buff not found");
    assert!(
        (c6_crit.base_value - 0.60).abs() < EPS,
        "Mika C6 PhysicalCritDmg value should be 0.60, got {}",
        c6_crit.base_value
    );
}

/// Mika C6 should NOT have an extra PhysicalDmgBonus buff from Constellation(6)
#[test]
fn test_mika_c6_no_extra_physical_dmg_bonus() {
    let buffs = find_talent_buffs("mika").expect("mika buffs not found");
    let c6_phys_dmg_count = buffs
        .iter()
        .filter(|b| {
            b.source == TalentBuffSource::Constellation(6)
                && b.stat == BuffableStat::PhysicalDmgBonus
        })
        .count();
    assert_eq!(
        c6_phys_dmg_count, 0,
        "Mika C6 should have no PhysicalDmgBonus buff (HH does not list Physical DMG+10% for C6), found {}",
        c6_phys_dmg_count
    );
}
