//! Issue #140: Passive/constellation-gated direct lunar reaction scalings.
//!
//! Verifies that `CharacterData.passive_scalings` exposes the correct gate,
//! reaction, scaling stat, multiplier, and replacement targets for each
//! passive/constellation-driven direct lunar damage instance.

use genshin_calc_core::{Element, Reaction, ScalingStat};
use genshin_calc_data::find_character;
use genshin_calc_data::types::{CharacterData, PassiveScaling, ScalingActivationGate};

const EPSILON: f64 = 1e-9;

fn get<'a>(ch: &'a CharacterData, name: &str) -> &'a PassiveScaling {
    ch.passive_scalings
        .iter()
        .find(|p| p.name == name)
        .unwrap_or_else(|| panic!("passive scaling '{name}' not found for {}", ch.id))
}

// -- Ineffa A1 / C2 / C6 -----------------------------------------------------

#[test]
fn ineffa_a1_overclocking_circuit() {
    let ch = find_character("ineffa").unwrap();
    let p = get(ch, "オーバークロック追撃");
    assert_eq!(p.scaling_stat, ScalingStat::Atk);
    assert_eq!(p.damage_element, Some(Element::Electro));
    assert!((p.multiplier - 0.65).abs() < EPSILON);
    assert_eq!(p.reaction, Reaction::LunarElectroCharged);
    assert_eq!(p.gate, ScalingActivationGate::PassiveA1);
    assert!(p.replaces.is_empty());
}

#[test]
fn ineffa_c2_punishment_edict() {
    let ch = find_character("ineffa").unwrap();
    let p = get(ch, "懲戒訓示");
    assert_eq!(p.scaling_stat, ScalingStat::Atk);
    assert_eq!(p.damage_element, Some(Element::Electro));
    assert!((p.multiplier - 3.00).abs() < EPSILON);
    assert_eq!(p.reaction, Reaction::LunarElectroCharged);
    assert_eq!(p.gate, ScalingActivationGate::Constellation(2));
    assert!(p.replaces.is_empty());
}

#[test]
fn ineffa_c6_dawning_morn() {
    let ch = find_character("ineffa").unwrap();
    let p = get(ch, "貴方に捧げる暁");
    assert_eq!(p.scaling_stat, ScalingStat::Atk);
    assert_eq!(p.damage_element, Some(Element::Electro));
    assert!((p.multiplier - 1.35).abs() < EPSILON);
    assert_eq!(p.reaction, Reaction::LunarElectroCharged);
    assert_eq!(p.gate, ScalingActivationGate::Constellation(6));
    assert!(p.replaces.is_empty());
}

// -- Flins C2 ----------------------------------------------------------------

#[test]
fn flins_c2_devils_wall() {
    let ch = find_character("flins").unwrap();
    let p = get(ch, "邪悪の壁を超える者");
    assert_eq!(p.scaling_stat, ScalingStat::Atk);
    assert_eq!(p.damage_element, Some(Element::Electro));
    assert!((p.multiplier - 0.50).abs() < EPSILON);
    assert_eq!(p.reaction, Reaction::LunarElectroCharged);
    assert_eq!(p.gate, ScalingActivationGate::Constellation(2));
    assert!(p.replaces.is_empty());
}

// -- Nefer C6 (replacement + new) --------------------------------------------

#[test]
fn nefer_c6_phantasm2_replacement_em() {
    let ch = find_character("nefer").unwrap();
    let p = get(ch, "幻影演舞2段(Nefer) 置換");
    assert_eq!(p.scaling_stat, ScalingStat::Em);
    assert_eq!(p.damage_element, Some(Element::Dendro));
    assert!((p.multiplier - 0.85).abs() < EPSILON);
    assert_eq!(p.reaction, Reaction::LunarBloom);
    assert_eq!(p.gate, ScalingActivationGate::Constellation(6));

    // C6 must remove BOTH the ATK and EM scalings of Phantasm-Performance 2-Hit (Nefer).
    let names: Vec<&str> = p.replaces.to_vec();
    assert!(
        names.contains(&"幻影演舞2段(Nefer)・攻撃力"),
        "expected ATK scaling in replaces; got {names:?}"
    );
    assert!(
        names.contains(&"幻影演舞2段(Nefer)・元素熟知"),
        "expected EM scaling in replaces; got {names:?}"
    );
}

#[test]
fn nefer_c6_phantasm_extra_hit() {
    let ch = find_character("nefer").unwrap();
    let p = get(ch, "幻影演舞 終了時追加ダメージ");
    assert_eq!(p.scaling_stat, ScalingStat::Em);
    assert_eq!(p.damage_element, Some(Element::Dendro));
    assert!((p.multiplier - 1.20).abs() < EPSILON);
    assert_eq!(p.reaction, Reaction::LunarBloom);
    assert_eq!(p.gate, ScalingActivationGate::Constellation(6));
    assert!(p.replaces.is_empty());
}

#[test]
fn nefer_c6_replacement_targets_exist_as_existing_talent_scalings() {
    // The names listed in `replaces` must match real existing TalentScaling names
    // — otherwise the consumer has nothing to remove.
    let ch = find_character("nefer").unwrap();
    let p = get(ch, "幻影演舞2段(Nefer) 置換");
    let skill_names: Vec<&str> = ch
        .talents
        .elemental_skill
        .scalings
        .iter()
        .map(|s| s.name)
        .collect();
    for target in p.replaces {
        assert!(
            skill_names.contains(target),
            "replaces target '{target}' not found in nefer skill scalings: {skill_names:?}"
        );
    }
}

// -- Lauma C6 ----------------------------------------------------------------

#[test]
fn lauma_c6_sanctuary_additional_hit() {
    let ch = find_character("lauma").unwrap();
    let p = get(ch, "霜林の聖域 追撃");
    assert_eq!(p.scaling_stat, ScalingStat::Em);
    assert_eq!(p.damage_element, Some(Element::Dendro));
    assert!((p.multiplier - 1.85).abs() < EPSILON);
    assert_eq!(p.reaction, Reaction::LunarBloom);
    assert_eq!(p.gate, ScalingActivationGate::Constellation(6));
    assert!(p.replaces.is_empty());
}

#[test]
fn lauma_c6_pale_hymn_normal_attack() {
    let ch = find_character("lauma").unwrap();
    let p = get(ch, "蒼の讃歌・通常攻撃消費");
    assert_eq!(p.scaling_stat, ScalingStat::Em);
    assert_eq!(p.damage_element, Some(Element::Dendro));
    assert!((p.multiplier - 1.50).abs() < EPSILON);
    assert_eq!(p.reaction, Reaction::LunarBloom);
    assert_eq!(p.gate, ScalingActivationGate::Constellation(6));
    assert!(p.replaces.is_empty());
}

// -- Default empty for non-target characters ---------------------------------

#[test]
fn diluc_has_no_passive_scalings() {
    let ch = find_character("diluc").unwrap();
    assert!(
        ch.passive_scalings.is_empty(),
        "diluc has unexpected passive_scalings: {:?}",
        ch.passive_scalings
    );
}

#[test]
fn all_characters_passive_scalings_field_accessible() {
    // Smoke test: every character has the field (even if empty).
    for c in genshin_calc_data::characters::all_characters() {
        let _ = c.passive_scalings;
    }
}

// -- Replacement logic (active_passive_scalings + talent_scaling_is_replaced) -

#[test]
fn ineffa_active_scalings_filter_by_constellation_and_passive() {
    let ch = find_character("ineffa").unwrap();

    // Lv1 / C0 / no passive: nothing active.
    let none: Vec<&str> = ch
        .active_passive_scalings(0, false, false)
        .map(|p| p.name)
        .collect();
    assert!(none.is_empty(), "expected no active scalings, got {none:?}");

    // A1 unlocked, C0: only A1 passive active.
    let a1_only: Vec<&str> = ch
        .active_passive_scalings(0, true, false)
        .map(|p| p.name)
        .collect();
    assert_eq!(a1_only, vec!["オーバークロック追撃"]);

    // A1 + C2: A1 passive + C2 passive.
    let c2: Vec<&str> = ch
        .active_passive_scalings(2, true, false)
        .map(|p| p.name)
        .collect();
    assert_eq!(c2, vec!["オーバークロック追撃", "懲戒訓示"]);

    // A1 + C6: all three.
    let c6: Vec<&str> = ch
        .active_passive_scalings(6, true, false)
        .map(|p| p.name)
        .collect();
    assert_eq!(
        c6,
        vec!["オーバークロック追撃", "懲戒訓示", "貴方に捧げる暁"]
    );
}

#[test]
fn nefer_c6_replaces_phantasm2_scalings() {
    let ch = find_character("nefer").unwrap();

    // C0: no replacement — both ATK and EM scalings remain in standard pipeline.
    assert!(!ch.talent_scaling_is_replaced("幻影演舞2段(Nefer)・攻撃力", 0, false, false));
    assert!(!ch.talent_scaling_is_replaced("幻影演舞2段(Nefer)・元素熟知", 0, false, false));

    // C5: still no replacement (C6 gate not met).
    assert!(!ch.talent_scaling_is_replaced("幻影演舞2段(Nefer)・攻撃力", 5, true, true));
    assert!(!ch.talent_scaling_is_replaced("幻影演舞2段(Nefer)・元素熟知", 5, true, true));

    // C6: both Phantasm 2 (Nefer) ATK and EM scalings are replaced.
    assert!(ch.talent_scaling_is_replaced("幻影演舞2段(Nefer)・攻撃力", 6, true, true));
    assert!(ch.talent_scaling_is_replaced("幻影演舞2段(Nefer)・元素熟知", 6, true, true));

    // Other scalings (e.g. Phantasm 1) are NOT replaced even at C6.
    assert!(!ch.talent_scaling_is_replaced("幻影演舞1段(Nefer)・攻撃力", 6, true, true));
    assert!(!ch.talent_scaling_is_replaced("幻影演舞1段(Nefer)・元素熟知", 6, true, true));
    assert!(!ch.talent_scaling_is_replaced("幻影演舞2段(分身)", 6, true, true));
}

#[test]
fn lauma_active_scalings_only_at_c6() {
    let ch = find_character("lauma").unwrap();
    assert_eq!(ch.active_passive_scalings(5, true, true).count(), 0);
    assert_eq!(ch.active_passive_scalings(6, false, false).count(), 2);
}

// -- Replacement-target invariant for every passive scaling ------------------

#[test]
fn every_replacement_target_resolves_to_an_existing_scaling() {
    for c in genshin_calc_data::characters::all_characters() {
        let all_names: Vec<&str> = c
            .talents
            .normal_attack
            .hits
            .iter()
            .chain(c.talents.normal_attack.charged.iter())
            .chain(c.talents.normal_attack.plunging.iter())
            .chain(c.talents.elemental_skill.scalings.iter())
            .chain(c.talents.elemental_burst.scalings.iter())
            .map(|s| s.name)
            .collect();
        for p in c.passive_scalings {
            for target in p.replaces {
                assert!(
                    all_names.contains(target),
                    "{}: passive '{}' references missing scaling '{}'",
                    c.id,
                    p.name,
                    target
                );
            }
        }
    }
}
