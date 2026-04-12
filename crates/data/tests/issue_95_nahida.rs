/// Issue #95: Add Nahida Tri-Karma Purification dual ATK+EM scaling and fix C4 EM model
/// Source: honeyhunter-mirror/md/characters/nahida_073.md
use genshin_calc_core::{DamageType, ScalingStat};
use genshin_calc_data::{
    find_character,
    talent_buffs::{Activation, ManualCondition, find_talent_buffs},
};

const EPS: f64 = 1e-4;

/// Tri-Karma Purification ATK portion: 103.2% ATK at Lv1 (skill hit index 2)
#[test]
fn test_nahida_tri_karma_atk_scaling_lv1() {
    let c = find_character("nahida").expect("nahida not found");
    // Skill scalings: [0]=Press, [1]=Hold, [2]=Tri-Karma ATK, [3]=Tri-Karma EM
    let scaling = c
        .talent_scaling(DamageType::Skill, 2)
        .expect("skill hit 2 (Tri-Karma ATK)");
    assert_eq!(scaling.scaling_stat, ScalingStat::Atk);
    assert!(
        (scaling.values[0] - 1.0320).abs() < EPS,
        "Tri-Karma ATK Lv1: {}",
        scaling.values[0]
    );
}

/// Tri-Karma Purification EM portion: 206.4% EM at Lv1 (skill hit index 3)
#[test]
fn test_nahida_tri_karma_em_scaling_lv1() {
    let c = find_character("nahida").expect("nahida not found");
    let scaling = c
        .talent_scaling(DamageType::Skill, 3)
        .expect("skill hit 3 (Tri-Karma EM)");
    assert_eq!(scaling.scaling_stat, ScalingStat::Em);
    assert!(
        (scaling.values[0] - 2.0640).abs() < EPS,
        "Tri-Karma EM Lv1: {}",
        scaling.values[0]
    );
}

/// Tri-Karma at Lv10: 185.76% ATK + 371.52% EM
#[test]
fn test_nahida_tri_karma_scaling_lv10() {
    let c = find_character("nahida").expect("nahida not found");
    let atk = c
        .talent_scaling(DamageType::Skill, 2)
        .expect("Tri-Karma ATK");
    let em = c
        .talent_scaling(DamageType::Skill, 3)
        .expect("Tri-Karma EM");
    assert!(
        (atk.values[9] - 1.8576).abs() < EPS,
        "Tri-Karma ATK Lv10: {}",
        atk.values[9]
    );
    assert!(
        (em.values[9] - 3.7152).abs() < EPS,
        "Tri-Karma EM Lv10: {}",
        em.values[9]
    );
}

/// C4: 1 enemy = 100 EM, 2 enemies = 120 EM, 3 enemies = 140 EM, 4 enemies = 160 EM
/// Model: base 80 EM constant + Stacks(4) at 20 EM per stack
#[test]
fn test_nahida_c4_em_base_buff_is_80() {
    let buffs = find_talent_buffs("nahida").expect("nahida buffs not found");
    // There should be a constant-on C4 buff of 80 EM (no activation required)
    let base_buff = buffs
        .iter()
        .find(|b| {
            b.source == genshin_calc_data::talent_buffs::TalentBuffSource::Constellation(4)
                && b.activation.is_none()
        })
        .expect("C4 constant EM base buff (80 EM, no activation) not found");
    assert!(
        (base_buff.base_value - 80.0).abs() < EPS,
        "C4 base EM should be 80, got {}",
        base_buff.base_value
    );
}

#[test]
fn test_nahida_c4_em_stacks_buff_is_20_per_stack() {
    let buffs = find_talent_buffs("nahida").expect("nahida buffs not found");
    // There should be a Stacks(4) C4 buff of 20 EM per stack
    let stack_buff = buffs
        .iter()
        .find(|b| {
            b.source == genshin_calc_data::talent_buffs::TalentBuffSource::Constellation(4)
                && matches!(
                    &b.activation,
                    Some(Activation::Manual(ManualCondition::Stacks(4)))
                )
        })
        .expect("C4 stacked EM buff (20 EM × Stacks(4)) not found");
    assert!(
        (stack_buff.base_value - 20.0).abs() < EPS,
        "C4 per-stack EM should be 20, got {}",
        stack_buff.base_value
    );
}

/// Validate total EM at each stack count: 80+(20×n)
#[test]
fn test_nahida_c4_em_totals() {
    // 1 enemy: 80 + 20×1 = 100
    // 2 enemies: 80 + 20×2 = 120
    // 3 enemies: 80 + 20×3 = 140
    // 4 enemies: 80 + 20×4 = 160
    for (n, expected) in [(1u16, 100.0f64), (2, 120.0), (3, 140.0), (4, 160.0)] {
        let total = 80.0 + 20.0 * n as f64;
        assert!(
            (total - expected).abs() < EPS,
            "C4 EM at {} enemy/enemies: expected {}, got {}",
            n,
            expected,
            total
        );
    }
}
