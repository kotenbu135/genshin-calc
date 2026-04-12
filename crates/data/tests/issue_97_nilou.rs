/// Issue #97: Add Nilou Whirling Steps scaling entries
/// Source: honeyhunter-mirror/md/characters/nilou_070.md
use genshin_calc_core::{DamageType, ScalingStat};
use genshin_calc_data::find_character;

const EPS: f64 = 1e-4;

/// Existing: Skill initial DMG (Pirouette) at index 0: 3.34% HP
#[test]
fn test_nilou_skill_pirouette_lv1() {
    let c = find_character("nilou").expect("nilou not found");
    let s = c.talent_scaling(DamageType::Skill, 0).expect("skill hit 0");
    assert_eq!(s.scaling_stat, ScalingStat::Hp);
    assert!(
        (s.values[0] - 0.0334).abs() < EPS,
        "Pirouette Lv1: {}",
        s.values[0]
    );
}

/// Whirling Steps 1-Hit DMG at index 4: 3.26% HP Lv1
#[test]
fn test_nilou_whirling_step1_lv1() {
    let c = find_character("nilou").expect("nilou not found");
    let s = c
        .talent_scaling(DamageType::Skill, 4)
        .expect("skill hit 4 (Whirling Step 1)");
    assert_eq!(s.scaling_stat, ScalingStat::Hp);
    assert!(
        (s.values[0] - 0.0326).abs() < EPS,
        "Whirling Step 1 Lv1: {}",
        s.values[0]
    );
}

/// Whirling Steps 2-Hit DMG at index 5: 3.96% HP Lv1
#[test]
fn test_nilou_whirling_step2_lv1() {
    let c = find_character("nilou").expect("nilou not found");
    let s = c
        .talent_scaling(DamageType::Skill, 5)
        .expect("skill hit 5 (Whirling Step 2)");
    assert_eq!(s.scaling_stat, ScalingStat::Hp);
    assert!(
        (s.values[0] - 0.0396).abs() < EPS,
        "Whirling Step 2 Lv1: {}",
        s.values[0]
    );
}

/// Water Wheel DMG at index 6: 5.06% HP Lv1
#[test]
fn test_nilou_water_wheel_lv1() {
    let c = find_character("nilou").expect("nilou not found");
    let s = c
        .talent_scaling(DamageType::Skill, 6)
        .expect("skill hit 6 (Water Wheel)");
    assert_eq!(s.scaling_stat, ScalingStat::Hp);
    assert!(
        (s.values[0] - 0.0506).abs() < EPS,
        "Water Wheel Lv1: {}",
        s.values[0]
    );
}

/// Whirling Steps 1 at Lv10: 5.87% HP
#[test]
fn test_nilou_whirling_step1_lv10() {
    let c = find_character("nilou").expect("nilou not found");
    let s = c.talent_scaling(DamageType::Skill, 4).expect("skill hit 4");
    assert!(
        (s.values[9] - 0.0587).abs() < EPS,
        "Whirling Step 1 Lv10: {}",
        s.values[9]
    );
}

/// Water Wheel at Lv15: 12.02% HP
#[test]
fn test_nilou_water_wheel_lv15() {
    let c = find_character("nilou").expect("nilou not found");
    let s = c.talent_scaling(DamageType::Skill, 6).expect("skill hit 6");
    assert!(
        (s.values[14] - 0.1202).abs() < EPS,
        "Water Wheel Lv15: {}",
        s.values[14]
    );
}
