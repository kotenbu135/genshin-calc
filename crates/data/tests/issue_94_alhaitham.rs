/// Issue #94: Fix Alhaitham EM dual-scaling and C2 stack count
/// Source: honeyhunter-mirror/md/characters/alhatham_078.md
use genshin_calc_core::{DamageType, ScalingStat};
use genshin_calc_data::{
    find_character,
    talent_buffs::{Activation, ManualCondition, find_talent_buffs},
};

const EPS: f64 = 1e-4;

/// Skill Rush Attack: 193.6% ATK + 154.88% EM at Lv1
#[test]
fn test_alhaitham_skill_thrust_atk_scaling_lv1() {
    let c = find_character("alhaitham").expect("alhaitham not found");
    let scaling = c.talent_scaling(DamageType::Skill, 0).expect("skill hit 0");
    assert_eq!(scaling.scaling_stat, ScalingStat::Atk);
    assert!(
        (scaling.values[0] - 1.9360).abs() < EPS,
        "Rush ATK Lv1: {}",
        scaling.values[0]
    );
}

/// 1-Mirror Projection ATK: 67.2% ATK at Lv1
#[test]
fn test_alhaitham_projection_atk_scaling_lv1() {
    let c = find_character("alhaitham").expect("alhaitham not found");
    let scaling = c
        .talent_scaling(DamageType::Skill, 1)
        .expect("skill hit 1 (1-mirror)");
    assert_eq!(scaling.scaling_stat, ScalingStat::Atk);
    assert!(
        (scaling.values[0] - 0.6720).abs() < EPS,
        "1-Mirror ATK Lv1: {}",
        scaling.values[0]
    );
}

/// 1-Mirror Projection EM: 134.4% EM at Lv1
#[test]
fn test_alhaitham_projection_em_scaling_lv1() {
    let c = find_character("alhaitham").expect("alhaitham not found");
    let em_scaling = c
        .talent_scaling(DamageType::Skill, 2)
        .expect("skill hit 2 (1-mirror EM)");
    assert_eq!(em_scaling.scaling_stat, ScalingStat::Em);
    assert!(
        (em_scaling.values[0] - 1.3440).abs() < EPS,
        "1-Mirror EM Lv1: {}",
        em_scaling.values[0]
    );
}

/// Burst Single-Instance: 121.6% ATK at Lv1
#[test]
fn test_alhaitham_burst_atk_scaling_lv1() {
    let c = find_character("alhaitham").expect("alhaitham not found");
    let scaling = c.talent_scaling(DamageType::Burst, 0).expect("burst hit 0");
    assert_eq!(scaling.scaling_stat, ScalingStat::Atk);
    assert!(
        (scaling.values[0] - 1.2160).abs() < EPS,
        "Burst ATK Lv1: {}",
        scaling.values[0]
    );
}

/// Burst Single-Instance EM: 97.28% EM at Lv1
#[test]
fn test_alhaitham_burst_em_scaling_lv1() {
    let c = find_character("alhaitham").expect("alhaitham not found");
    let em_scaling = c
        .talent_scaling(DamageType::Burst, 1)
        .expect("burst hit 1 (EM)");
    assert_eq!(em_scaling.scaling_stat, ScalingStat::Em);
    assert!(
        (em_scaling.values[0] - 0.9728).abs() < EPS,
        "Burst EM Lv1: {}",
        em_scaling.values[0]
    );
}

/// Skill Rush EM at Lv10: 278.78% EM
#[test]
fn test_alhaitham_skill_thrust_em_scaling_lv10() {
    let c = find_character("alhaitham").expect("alhaitham not found");
    let em_scaling = c
        .talent_scaling(DamageType::Skill, 3)
        .expect("skill hit 3 (rush EM)");
    assert_eq!(em_scaling.scaling_stat, ScalingStat::Em);
    assert!(
        (em_scaling.values[9] - 2.7878).abs() < EPS,
        "Rush EM Lv10: {}",
        em_scaling.values[9]
    );
}

/// C2: max 4 stacks of 50 EM (not 3)
#[test]
fn test_alhaitham_c2_em_buff_is_4_stacks() {
    let buffs = find_talent_buffs("alhaitham").expect("alhaitham buffs not found");
    let buff = buffs
        .iter()
        .find(|b| b.name.contains("Chisel-Light Mirror EM Bonus"))
        .expect("C2 EM buff not found");
    assert_eq!(buff.base_value, 50.0);
    match &buff.activation {
        Some(Activation::Manual(ManualCondition::Stacks(n))) => {
            assert_eq!(*n, 4, "C2 should be Stacks(4), got Stacks({})", n);
        }
        other => panic!("Expected Stacks(4) activation, got {:?}", other),
    }
}
