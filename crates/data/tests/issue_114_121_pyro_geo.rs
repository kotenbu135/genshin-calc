use genshin_calc_core::{BuffTarget, BuffableStat, Element, ScalingStat};
/// Tests for issues #114-#121: Amber, Bennett, Chevreuse, Albedo, Gorou, Zhongli, Xilonen, Kachina talent buff additions.
use genshin_calc_data::talent_buffs::{TalentBuffSource, find_talent_buffs};

const EPS: f64 = 1e-6;

fn close(a: f64, b: f64) -> bool {
    (a - b).abs() < EPS
}

// ===== Issue #114: Amber A1/A4 =====

#[test]
fn amber_a1_crit_rate_burst_second_wave() {
    let buffs = find_talent_buffs("amber").unwrap();
    let a1 = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::CritRate && b.source == TalentBuffSource::AscensionPassive(1)
        })
        .expect("Amber A1 CritRate buff missing");
    assert!(
        close(a1.base_value, 0.10),
        "A1 CritRate should be 0.10, got {}",
        a1.base_value
    );
    assert_eq!(a1.target, BuffTarget::OnlySelf);
}

#[test]
fn amber_a4_atk_percent_on_weak_point_hit() {
    let buffs = find_talent_buffs("amber").unwrap();
    let a4 = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::AtkPercent && b.source == TalentBuffSource::AscensionPassive(4)
        })
        .expect("Amber A4 AtkPercent buff missing");
    assert!(
        close(a4.base_value, 0.15),
        "A4 ATK% should be 0.15, got {}",
        a4.base_value
    );
    assert_eq!(a4.target, BuffTarget::OnlySelf);
}

#[test]
fn amber_buff_count_includes_a1_a4_and_c6() {
    let buffs = find_talent_buffs("amber").unwrap();
    // Should now have: C6 Wildfire + A1 CritRate + A4 AtkPercent = 3
    assert_eq!(buffs.len(), 3, "amber should have 3 buffs (C6 + A1 + A4)");
}

// ===== Issue #115: Bennett C2 ER+30% =====

#[test]
fn bennett_c2_energy_recharge_bonus() {
    let buffs = find_talent_buffs("bennett").unwrap();
    let c2 = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::EnergyRecharge && b.source == TalentBuffSource::Constellation(2)
        })
        .expect("Bennett C2 EnergyRecharge buff missing");
    assert!(
        close(c2.base_value, 0.30),
        "C2 ER should be 0.30, got {}",
        c2.base_value
    );
    assert_eq!(c2.min_constellation, 2);
    assert_eq!(c2.target, BuffTarget::OnlySelf);
}

#[test]
fn bennett_buff_count_with_c2() {
    let buffs = find_talent_buffs("bennett").unwrap();
    // Should now have: Fantastic Voyage ATK + C1 Grand Expectation + C6 Spirit of Pyro + C2 ER = 4
    assert_eq!(buffs.len(), 4, "bennett should have 4 buffs");
}

// ===== Issue #116: Chevreuse A4 ATK+20% for Pyro/Electro =====

#[test]
fn chevreuse_a4_atk_percent_pyro_electro() {
    let buffs = find_talent_buffs("chevreuse").unwrap();
    // A4 "Vertical Force Coordination" - the ATK+20% based on HP for Pyro/Electro chars
    let a4 = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::AtkPercent && b.source == TalentBuffSource::AscensionPassive(4)
        })
        .expect("Chevreuse A4 AtkPercent buff missing");
    assert!(
        close(a4.base_value, 0.20),
        "A4 ATK% should be 0.20, got {}",
        a4.base_value
    );
    assert_eq!(a4.target, BuffTarget::Team);
}

#[test]
fn chevreuse_no_a1_atk_buff_duplicate() {
    let buffs = find_talent_buffs("chevreuse").unwrap();
    // A1 should NOT have an AtkPercent buff anymore (was misattributed)
    let a1_atk = buffs.iter().find(|b| {
        b.stat == BuffableStat::AtkPercent && b.source == TalentBuffSource::AscensionPassive(1)
    });
    assert!(
        a1_atk.is_none(),
        "Chevreuse should not have AtkPercent under A1 (it's A4)"
    );
}

// ===== Issue #117: Albedo A1 and C2 =====

#[test]
fn albedo_a1_skill_dmg_bonus() {
    let buffs = find_talent_buffs("albedo").unwrap();
    let a1 = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::SkillDmgBonus
                && b.source == TalentBuffSource::AscensionPassive(1)
        })
        .expect("Albedo A1 SkillDmgBonus buff missing");
    assert!(
        close(a1.base_value, 0.25),
        "A1 SkillDmgBonus should be 0.25, got {}",
        a1.base_value
    );
    assert_eq!(a1.target, BuffTarget::OnlySelf);
}

#[test]
fn albedo_c2_burst_flat_dmg_def_scaling() {
    let buffs = find_talent_buffs("albedo").unwrap();
    let c2 = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::BurstFlatDmg && b.source == TalentBuffSource::Constellation(2)
        })
        .expect("Albedo C2 BurstFlatDmg buff missing");
    assert!(
        close(c2.base_value, 0.30),
        "C2 BurstFlatDmg ratio should be 0.30, got {}",
        c2.base_value
    );
    assert_eq!(c2.scales_on, Some(ScalingStat::Def));
    assert_eq!(c2.min_constellation, 2);
    assert_eq!(c2.target, BuffTarget::OnlySelf);
}

#[test]
fn albedo_buff_count_with_a1_c2() {
    let buffs = find_talent_buffs("albedo").unwrap();
    // Was 4, now +2 (A1 SkillDmgBonus + C2 BurstFlatDmg) = 6
    assert_eq!(buffs.len(), 6, "albedo should have 6 buffs");
}

// ===== Issue #118: Gorou A1 =====

#[test]
fn gorou_a1_skill_flat_dmg_def_scaling() {
    let buffs = find_talent_buffs("gorou").unwrap();
    let a1_skill = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::SkillFlatDmg
                && b.source == TalentBuffSource::AscensionPassive(1)
        })
        .expect("Gorou A1 SkillFlatDmg buff missing");
    assert!(
        close(a1_skill.base_value, 1.56),
        "A1 SkillFlatDmg ratio should be 1.56, got {}",
        a1_skill.base_value
    );
    assert_eq!(a1_skill.scales_on, Some(ScalingStat::Def));
    assert_eq!(a1_skill.target, BuffTarget::OnlySelf);
}

#[test]
fn gorou_a1_burst_flat_dmg_def_scaling() {
    let buffs = find_talent_buffs("gorou").unwrap();
    let a1_burst = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::BurstFlatDmg
                && b.source == TalentBuffSource::AscensionPassive(1)
        })
        .expect("Gorou A1 BurstFlatDmg buff missing");
    assert!(
        close(a1_burst.base_value, 0.156),
        "A1 BurstFlatDmg ratio should be 0.156, got {}",
        a1_burst.base_value
    );
    assert_eq!(a1_burst.scales_on, Some(ScalingStat::Def));
    assert_eq!(a1_burst.target, BuffTarget::OnlySelf);
}

#[test]
fn gorou_buff_count_with_a1() {
    let buffs = find_talent_buffs("gorou").unwrap();
    // Was 4, +2 (A1 skill + burst flat DMG) = 6
    assert_eq!(buffs.len(), 6, "gorou should have 6 buffs");
}

// ===== Issue #119: Zhongli A4 =====

#[test]
fn zhongli_a4_na_flat_dmg_hp_scaling() {
    let buffs = find_talent_buffs("zhongli").unwrap();
    let na = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::NormalAtkFlatDmg
                && b.source == TalentBuffSource::AscensionPassive(4)
        })
        .expect("Zhongli A4 NormalAtkFlatDmg buff missing");
    assert!(
        close(na.base_value, 0.0139),
        "A4 NA flat DMG ratio should be 0.0139, got {}",
        na.base_value
    );
    assert_eq!(na.target, BuffTarget::OnlySelf);
}

#[test]
fn zhongli_a4_ca_flat_dmg_hp_scaling() {
    let buffs = find_talent_buffs("zhongli").unwrap();
    let ca = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::ChargedAtkFlatDmg
                && b.source == TalentBuffSource::AscensionPassive(4)
        })
        .expect("Zhongli A4 ChargedAtkFlatDmg buff missing");
    assert!(
        close(ca.base_value, 0.0139),
        "A4 CA flat DMG ratio should be 0.0139, got {}",
        ca.base_value
    );
}

#[test]
fn zhongli_a4_plunging_flat_dmg_hp_scaling() {
    let buffs = find_talent_buffs("zhongli").unwrap();
    let plunge = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::PlungingAtkFlatDmg
                && b.source == TalentBuffSource::AscensionPassive(4)
        })
        .expect("Zhongli A4 PlungingAtkFlatDmg buff missing");
    assert!(
        close(plunge.base_value, 0.0139),
        "A4 Plunge flat DMG ratio should be 0.0139, got {}",
        plunge.base_value
    );
}

#[test]
fn zhongli_a4_skill_flat_dmg_hp_scaling() {
    let buffs = find_talent_buffs("zhongli").unwrap();
    let skill = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::SkillFlatDmg
                && b.source == TalentBuffSource::AscensionPassive(4)
        })
        .expect("Zhongli A4 SkillFlatDmg buff missing");
    assert!(
        close(skill.base_value, 0.019),
        "A4 Skill flat DMG ratio should be 0.019, got {}",
        skill.base_value
    );
    assert_eq!(skill.target, BuffTarget::OnlySelf);
}

#[test]
fn zhongli_a4_burst_flat_dmg_hp_scaling() {
    let buffs = find_talent_buffs("zhongli").unwrap();
    let burst = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::BurstFlatDmg
                && b.source == TalentBuffSource::AscensionPassive(4)
        })
        .expect("Zhongli A4 BurstFlatDmg buff missing");
    assert!(
        close(burst.base_value, 0.33),
        "A4 Burst flat DMG ratio should be 0.33, got {}",
        burst.base_value
    );
    assert_eq!(burst.target, BuffTarget::OnlySelf);
}

// ===== Issue #120: Xilonen A1 =====

#[test]
fn xilonen_a1_normal_atk_dmg_bonus() {
    let buffs = find_talent_buffs("xilonen").unwrap();
    let a1_na = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::NormalAtkDmgBonus
                && b.source == TalentBuffSource::AscensionPassive(1)
        })
        .expect("Xilonen A1 NormalAtkDmgBonus buff missing");
    assert!(
        close(a1_na.base_value, 0.30),
        "A1 NA DMG Bonus should be 0.30, got {}",
        a1_na.base_value
    );
    assert_eq!(a1_na.target, BuffTarget::OnlySelf);
}

#[test]
fn xilonen_a1_plunging_atk_dmg_bonus() {
    let buffs = find_talent_buffs("xilonen").unwrap();
    let a1_plunge = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::PlungingAtkDmgBonus
                && b.source == TalentBuffSource::AscensionPassive(1)
        })
        .expect("Xilonen A1 PlungingAtkDmgBonus buff missing");
    assert!(
        close(a1_plunge.base_value, 0.30),
        "A1 Plunge DMG Bonus should be 0.30, got {}",
        a1_plunge.base_value
    );
    assert_eq!(a1_plunge.target, BuffTarget::OnlySelf);
}

// ===== Issue #121: Kachina A4 and A1 name fix =====

#[test]
fn kachina_a4_skill_flat_dmg_def_scaling() {
    let buffs = find_talent_buffs("kachina").unwrap();
    let a4 = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::SkillFlatDmg
                && b.source == TalentBuffSource::AscensionPassive(4)
        })
        .expect("Kachina A4 SkillFlatDmg buff missing");
    assert!(
        close(a4.base_value, 0.20),
        "A4 SkillFlatDmg ratio should be 0.20, got {}",
        a4.base_value
    );
    assert_eq!(a4.scales_on, Some(ScalingStat::Def));
    assert_eq!(a4.target, BuffTarget::OnlySelf);
}

#[test]
fn kachina_a1_buff_named_mountain_echoes() {
    let buffs = find_talent_buffs("kachina").unwrap();
    // A1 should be named "Mountain Echoes" not "Flowy Mountain"
    let a1 = buffs
        .iter()
        .find(|b| b.source == TalentBuffSource::AscensionPassive(1))
        .expect("Kachina A1 buff missing");
    assert!(
        !a1.name.contains("Flowy Mountain"),
        "Kachina A1 buff name should not be 'Flowy Mountain', got '{}'",
        a1.name
    );
    assert!(
        a1.name.contains("Mountain Echoes"),
        "Kachina A1 buff name should contain 'Mountain Echoes', got '{}'",
        a1.name
    );
}

#[test]
fn kachina_buff_count_with_a4() {
    let buffs = find_talent_buffs("kachina").unwrap();
    // Was 2 (A1 Geo DMG, C4 DEF), now 3 (A1 + A4 skill flat DMG + C4)
    assert_eq!(buffs.len(), 3, "kachina should have 3 buffs");
}
