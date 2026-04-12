use genshin_calc_core::BuffableStat;
/// Issue #99: Add Columbina A1 CRIT Rate Stacks(3) buff and Burst Lunar Reaction DMG Bonus
/// Source: honeyhunter-mirror/md/characters/columbina_125.md
use genshin_calc_data::talent_buffs::{
    Activation, ManualCondition, TalentBuffSource, find_talent_buffs,
};

const EPS: f64 = 1e-6;

/// A1 "Lunacy's Lure": CRIT Rate +5% per stack, max 3 stacks
#[test]
fn test_columbina_a1_crit_rate_stacks3() {
    let buffs = find_talent_buffs("columbina").expect("columbina buffs not found");
    let buff = buffs
        .iter()
        .find(|b| {
            b.stat == BuffableStat::CritRate
                && b.source == TalentBuffSource::AscensionPassive(1)
                && b.min_constellation == 0
        })
        .expect("A1 CritRate Stacks(3) buff not found");
    assert!(
        (buff.base_value - 0.05).abs() < EPS,
        "A1 CR value: {}",
        buff.base_value
    );
    match &buff.activation {
        Some(Activation::Manual(ManualCondition::Stacks(n))) => {
            assert_eq!(*n, 3, "A1 should be Stacks(3), got Stacks({})", n);
        }
        other => panic!("Expected Stacks(3) activation for A1 CR, got {:?}", other),
    }
}

/// Burst "Lunar Reaction DMG Bonus": 13% at Lv1, scales with talent
#[test]
fn test_columbina_burst_lunar_dmg_bonus_lv1() {
    let buffs = find_talent_buffs("columbina").expect("columbina buffs not found");
    let buff = buffs
        .iter()
        .find(|b| {
            b.source == TalentBuffSource::ElementalBurst
                && b.stat == BuffableStat::TransformativeBonus
                && b.scales_with_talent
        })
        .expect("Burst Lunar Reaction DMG Bonus (talent-scaling) not found");
    let scaling = buff.talent_scaling.expect("talent_scaling should be Some");
    assert!(
        (scaling[0] - 0.13).abs() < EPS,
        "Burst Lunar DMG Lv1: {}",
        scaling[0]
    );
}

/// Burst scaling at Lv10: 40%, Lv15: 55%
#[test]
fn test_columbina_burst_lunar_dmg_bonus_scaling() {
    let buffs = find_talent_buffs("columbina").expect("columbina buffs not found");
    let buff = buffs
        .iter()
        .find(|b| {
            b.source == TalentBuffSource::ElementalBurst
                && b.stat == BuffableStat::TransformativeBonus
                && b.scales_with_talent
        })
        .expect("Burst Lunar Reaction DMG Bonus not found");
    let scaling = buff.talent_scaling.expect("talent_scaling should be Some");
    assert!(
        (scaling[9] - 0.40).abs() < EPS,
        "Burst Lunar DMG Lv10: {}",
        scaling[9]
    );
    assert!(
        (scaling[14] - 0.55).abs() < EPS,
        "Burst Lunar DMG Lv15: {}",
        scaling[14]
    );
}

/// Burst buff should be a Toggle activation
#[test]
fn test_columbina_burst_lunar_dmg_bonus_is_toggle() {
    let buffs = find_talent_buffs("columbina").expect("columbina buffs not found");
    let buff = buffs
        .iter()
        .find(|b| {
            b.source == TalentBuffSource::ElementalBurst
                && b.stat == BuffableStat::TransformativeBonus
                && b.scales_with_talent
        })
        .expect("Burst Lunar Reaction DMG Bonus not found");
    assert!(
        matches!(
            &buff.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        ),
        "Burst Lunar DMG Bonus should be Toggle, got {:?}",
        buff.activation
    );
}
