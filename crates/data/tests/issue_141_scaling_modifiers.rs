//! Issue #141: TalentScaling modifiers (additional flat / direct-lunar bonus / multiplier).
//!
//! Tests that `CharacterData.scaling_modifiers` exposes:
//! - Zibai A1: +60% DEF flat to Stride 2-Hit
//! - Zibai C1: +220% Lunar-Crystallize Reaction DMG to Stride 2-Hit
//! - Zibai C2: +550% DEF flat to Stride 2-Hit (further increases A1)
//! - Zibai C4: ×2.5 talent multiplier on LPS 4-Hit Additional
//! - Columbina C4: +12.5%/2.5%/12.5% Max HP flat to LunarCharged/Bloom/Crystallize
//!
//! Verifies regression: A1/C2 flat additions do NOT spill onto other Zibai skill hits.

use genshin_calc_core::{ScalingStat, Stats};
use genshin_calc_data::find_character;
use genshin_calc_data::types::{
    CharacterData, ScalingActivationGate, ScalingModifier, ScalingModifierKind,
};

const EPS: f64 = 1e-9;

fn get<'a>(ch: &'a CharacterData, name: &str) -> &'a ScalingModifier {
    ch.scaling_modifiers
        .iter()
        .find(|m| m.name == name)
        .unwrap_or_else(|| panic!("scaling_modifier '{name}' not found for {}", ch.id))
}

// =============================================================================
// Zibai A1: The Selenic Adeptus Descends — +60% DEF flat to Stride 2-Hit
// =============================================================================

#[test]
fn zibai_a1_selenic_descent_targets_stride_hit2_only() {
    let ch = find_character("zibai").unwrap();
    let m = get(ch, "月下に舞い降りる天女");
    assert_eq!(m.targets, &["霊駿突進2段ダメージ"]);
    assert_eq!(m.gate, ScalingActivationGate::PassiveA1);
    match m.kind {
        ScalingModifierKind::AdditionalFlat {
            scaling_stat,
            multiplier,
        } => {
            assert_eq!(scaling_stat, ScalingStat::Def);
            assert!((multiplier - 0.60).abs() < EPS);
        }
        other => panic!("expected AdditionalFlat, got {other:?}"),
    }
}

// =============================================================================
// Zibai C1: Phase Shift Radiance — +220% Lunar-Crystallize bonus on Stride 2-Hit
// =============================================================================

#[test]
fn zibai_c1_disquickly_emerge_adds_reaction_bonus() {
    let ch = find_character("zibai").unwrap();
    let m = get(ch, "疾く出でて粛然と帰す");
    assert_eq!(m.targets, &["霊駿突進2段ダメージ"]);
    assert_eq!(m.gate, ScalingActivationGate::Constellation(1));
    match m.kind {
        ScalingModifierKind::DirectLunarReactionBonus { bonus_ratio } => {
            assert!((bonus_ratio - 2.20).abs() < EPS);
        }
        other => panic!("expected DirectLunarReactionBonus, got {other:?}"),
    }
}

// =============================================================================
// Zibai C2: At Birth Are Souls Born — +550% DEF flat to Stride 2-Hit
// =============================================================================

#[test]
fn zibai_c2_souls_born_adds_def_flat_to_stride_hit2() {
    let ch = find_character("zibai").unwrap();
    let m = get(ch, "生は変じて屍と化する");
    assert_eq!(m.targets, &["霊駿突進2段ダメージ"]);
    assert_eq!(m.gate, ScalingActivationGate::Constellation(2));
    match m.kind {
        ScalingModifierKind::AdditionalFlat {
            scaling_stat,
            multiplier,
        } => {
            assert_eq!(scaling_stat, ScalingStat::Def);
            assert!((multiplier - 5.50).abs() < EPS);
        }
        other => panic!("expected AdditionalFlat, got {other:?}"),
    }
}

// =============================================================================
// Zibai C4: Spirit Passes Then Form Follows — ×2.5 multiplier on LPS 4-Hit Add
// =============================================================================

#[test]
fn zibai_c4_form_follows_multiplies_lps_hit4_additional() {
    let ch = find_character("zibai").unwrap();
    let m = get(ch, "魂向かひて身も従ずる");
    assert_eq!(m.targets, &["月相転移4段追加ダメージ"]);
    assert_eq!(m.gate, ScalingActivationGate::Constellation(4));
    match m.kind {
        ScalingModifierKind::DirectLunarMultiplier { factor } => {
            assert!((factor - 2.50).abs() < EPS);
        }
        other => panic!("expected DirectLunarMultiplier, got {other:?}"),
    }
}

// =============================================================================
// Columbina C4: Cloudveiled Ridges — HP flat to all 3 Gravity Interference scalings
// =============================================================================

#[test]
fn columbina_c4_lunar_charged_hp_flat() {
    let ch = find_character("columbina").unwrap();
    let m = get(ch, "花の嵐や雲と木と岩の陰・月感電");
    assert_eq!(m.targets, &["Gravity Interference: Lunar-Charged DMG"]);
    assert_eq!(m.gate, ScalingActivationGate::Constellation(4));
    match m.kind {
        ScalingModifierKind::AdditionalFlat {
            scaling_stat,
            multiplier,
        } => {
            assert_eq!(scaling_stat, ScalingStat::Hp);
            assert!((multiplier - 0.125).abs() < EPS);
        }
        other => panic!("expected AdditionalFlat, got {other:?}"),
    }
}

#[test]
fn columbina_c4_lunar_bloom_hp_flat() {
    let ch = find_character("columbina").unwrap();
    let m = get(ch, "花の嵐や雲と木と岩の陰・月開花");
    assert_eq!(m.targets, &["Gravity Interference: Lunar-Bloom DMG"]);
    assert_eq!(m.gate, ScalingActivationGate::Constellation(4));
    match m.kind {
        ScalingModifierKind::AdditionalFlat {
            scaling_stat,
            multiplier,
        } => {
            assert_eq!(scaling_stat, ScalingStat::Hp);
            assert!((multiplier - 0.025).abs() < EPS);
        }
        other => panic!("expected AdditionalFlat, got {other:?}"),
    }
}

#[test]
fn columbina_c4_lunar_crystallize_hp_flat() {
    let ch = find_character("columbina").unwrap();
    let m = get(ch, "花の嵐や雲と木と岩の陰・月結晶");
    assert_eq!(m.targets, &["Gravity Interference: Lunar-Crystallize DMG"]);
    assert_eq!(m.gate, ScalingActivationGate::Constellation(4));
    match m.kind {
        ScalingModifierKind::AdditionalFlat {
            scaling_stat,
            multiplier,
        } => {
            assert_eq!(scaling_stat, ScalingStat::Hp);
            assert!((multiplier - 0.125).abs() < EPS);
        }
        other => panic!("expected AdditionalFlat, got {other:?}"),
    }
}

// =============================================================================
// Gate filtering: active_scaling_modifiers respects constellation/passive level
// =============================================================================

#[test]
fn zibai_c0_no_constellation_modifiers_active() {
    let ch = find_character("zibai").unwrap();
    let active: Vec<&ScalingModifier> = ch.active_scaling_modifiers(0, true, true).collect();
    // C0 + A1/A4 unlocked → only A1 modifier active (no C1/C2/C4)
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].name, "月下に舞い降りる天女");
}

#[test]
fn zibai_c4_full_modifiers_active() {
    let ch = find_character("zibai").unwrap();
    let active: Vec<&ScalingModifier> = ch.active_scaling_modifiers(4, true, true).collect();
    // A1 + C1 + C2 + C4 = 4
    assert_eq!(active.len(), 4);
}

#[test]
fn zibai_a1_locked_filters_out() {
    let ch = find_character("zibai").unwrap();
    let active: Vec<&ScalingModifier> = ch.active_scaling_modifiers(0, false, false).collect();
    assert!(active.is_empty(), "A1 locked → no modifiers");
}

// =============================================================================
// Per-scaling lookup: scaling_modifiers_for() filters by target name
// =============================================================================

#[test]
fn zibai_stride_hit2_has_a1_c1_c2_modifiers_at_c4() {
    let ch = find_character("zibai").unwrap();
    let names: Vec<&str> = ch
        .scaling_modifiers_for("霊駿突進2段ダメージ", 4, true, true)
        .map(|m| m.name)
        .collect();
    assert!(names.contains(&"月下に舞い降りる天女"), "A1 missing");
    assert!(names.contains(&"疾く出でて粛然と帰す"), "C1 missing");
    assert!(names.contains(&"生は変じて屍と化する"), "C2 missing");
    // C4 targets a different scaling
    assert!(!names.contains(&"魂向かひて身も従ずる"));
}

#[test]
fn zibai_lps_hit4_additional_only_has_c4_modifier() {
    let ch = find_character("zibai").unwrap();
    let names: Vec<&str> = ch
        .scaling_modifiers_for("月相転移4段追加ダメージ", 4, true, true)
        .map(|m| m.name)
        .collect();
    assert_eq!(names, vec!["魂向かひて身も従ずる"]);
}

// Regression: A1/C2 flat damage MUST NOT spill onto LPS hits / charged / etc.
#[test]
fn zibai_a1_c2_flat_does_not_apply_to_other_skill_hits() {
    let ch = find_character("zibai").unwrap();
    let other_hits = [
        "月相転移1段ダメージ",
        "月相転移2段ダメージ",
        "月相転移3段ダメージ1",
        "月相転移3段ダメージ2",
        "月相転移4段ダメージ",
        "月相転移4段追加ダメージ",
        "月相転移重撃ダメージ1",
        "月相転移重撃ダメージ2",
        "霊駿突進1段ダメージ",
    ];
    for hit in other_hits {
        let mods: Vec<&ScalingModifier> = ch
            .scaling_modifiers_for(hit, 4, true, true)
            .filter(|m| matches!(m.kind, ScalingModifierKind::AdditionalFlat { .. }))
            .collect();
        assert!(
            mods.is_empty(),
            "AdditionalFlat must not apply to '{hit}' (found: {:?})",
            mods.iter().map(|m| m.name).collect::<Vec<_>>()
        );
    }
}

// =============================================================================
// Aggregated helpers (math): compute total additional flat / bonus / multiplier
// =============================================================================

#[test]
fn zibai_scaling_modifier_flat_dmg_a1_only() {
    let ch = find_character("zibai").unwrap();
    let stats = Stats {
        def: 1000.0,
        ..Stats::default()
    };
    // Stride 2-Hit at C0+A1: 1000 × 0.60 = 600
    let flat = ch.scaling_modifier_flat_dmg("霊駿突進2段ダメージ", &stats, 0, true, true);
    assert!((flat - 600.0).abs() < 1e-6);
}

#[test]
fn zibai_scaling_modifier_flat_dmg_a1_plus_c2() {
    let ch = find_character("zibai").unwrap();
    let stats = Stats {
        def: 1000.0,
        ..Stats::default()
    };
    // Stride 2-Hit at C2+A1: 1000 × (0.60 + 5.50) = 6100
    let flat = ch.scaling_modifier_flat_dmg("霊駿突進2段ダメージ", &stats, 2, true, true);
    assert!((flat - 6100.0).abs() < 1e-6);
}

#[test]
fn zibai_scaling_modifier_flat_dmg_zero_for_other_hits() {
    let ch = find_character("zibai").unwrap();
    let stats = Stats {
        def: 1000.0,
        ..Stats::default()
    };
    let flat = ch.scaling_modifier_flat_dmg("月相転移4段ダメージ", &stats, 4, true, true);
    assert!(flat.abs() < 1e-9, "expected 0, got {flat}");
}

#[test]
fn zibai_c1_direct_lunar_reaction_bonus_on_stride_hit2() {
    let ch = find_character("zibai").unwrap();
    let bonus =
        ch.scaling_modifier_direct_lunar_reaction_bonus("霊駿突進2段ダメージ", 1, true, true);
    assert!((bonus - 2.20).abs() < EPS);
}

#[test]
fn zibai_c0_no_direct_lunar_reaction_bonus() {
    let ch = find_character("zibai").unwrap();
    let bonus =
        ch.scaling_modifier_direct_lunar_reaction_bonus("霊駿突進2段ダメージ", 0, true, true);
    assert!(bonus.abs() < EPS);
}

#[test]
fn zibai_c4_direct_lunar_multiplier_on_lps_hit4_additional() {
    let ch = find_character("zibai").unwrap();
    let factor =
        ch.scaling_modifier_direct_lunar_multiplier("月相転移4段追加ダメージ", 4, true, true);
    assert!((factor - 2.50).abs() < EPS);
}

#[test]
fn zibai_c0_direct_lunar_multiplier_defaults_to_one() {
    let ch = find_character("zibai").unwrap();
    let factor =
        ch.scaling_modifier_direct_lunar_multiplier("月相転移4段追加ダメージ", 0, true, true);
    assert!((factor - 1.0).abs() < EPS, "expected 1.0, got {factor}");
}

#[test]
fn columbina_c4_flat_dmg_per_reaction() {
    let ch = find_character("columbina").unwrap();
    let stats = Stats {
        hp: 20000.0,
        ..Stats::default()
    };
    // C4 active: HP × 0.125 / 0.025 / 0.125
    let charged = ch.scaling_modifier_flat_dmg(
        "Gravity Interference: Lunar-Charged DMG",
        &stats,
        4,
        true,
        true,
    );
    assert!((charged - 2500.0).abs() < 1e-6);

    let bloom = ch.scaling_modifier_flat_dmg(
        "Gravity Interference: Lunar-Bloom DMG",
        &stats,
        4,
        true,
        true,
    );
    assert!((bloom - 500.0).abs() < 1e-6);

    let crystallize = ch.scaling_modifier_flat_dmg(
        "Gravity Interference: Lunar-Crystallize DMG",
        &stats,
        4,
        true,
        true,
    );
    assert!((crystallize - 2500.0).abs() < 1e-6);
}

#[test]
fn columbina_c3_no_c4_flat_dmg() {
    let ch = find_character("columbina").unwrap();
    let stats = Stats {
        hp: 20000.0,
        ..Stats::default()
    };
    let charged = ch.scaling_modifier_flat_dmg(
        "Gravity Interference: Lunar-Charged DMG",
        &stats,
        3,
        true,
        true,
    );
    assert!(charged.abs() < 1e-9);
}

// =============================================================================
// Bug fix verification: ZIBAI_BUFFS no longer contains the broken SkillFlatDmg
// =============================================================================

#[test]
fn zibai_skill_flat_dmg_buff_removed_from_talent_buffs() {
    use genshin_calc_core::BuffableStat;
    use genshin_calc_data::talent_buffs::find_talent_buffs;

    let buffs = find_talent_buffs("zibai").unwrap();
    let count = buffs
        .iter()
        .filter(|b| b.stat == BuffableStat::SkillFlatDmg)
        .count();
    assert_eq!(
        count, 0,
        "ZIBAI_BUFFS must not contain SkillFlatDmg \
         (it over-applies to all 10 skill hits — replaced by ScalingModifier)"
    );
}
