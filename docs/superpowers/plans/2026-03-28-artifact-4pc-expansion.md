# P2: 聖遺物4pc効果 全セット実装 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 全52聖遺物セットの4pc効果を ConditionalBuff システムで実装する

**Architecture:** core crateにBuffableStat 5 variant追加、data crateのConditionalBuff構造体にstack_values/targetフィールド追加、artifacts.rsに35セットのConditionalBuff定義を追加。TDD: テスト先行→実装→コミットの繰り返し。

**Tech Stack:** Rust, serde, cargo test

**Spec:** `docs/superpowers/specs/2026-03-28-artifact-4pc-expansion-design.md`

---

## File Map

| File | Action | Responsibility |
|---|---|---|
| `crates/core/src/buff_types.rs` | Modify | BuffableStat +5 variants |
| `crates/core/src/team.rs` | Modify | is_unconditional() update |
| `crates/data/src/buff.rs` | Modify | ConditionalBuff +2 fields, AutoCondition +2 variants |
| `crates/data/src/team_builder.rs` | Modify | eval_manual signature, stack_values, resolve_conditionals target |
| `crates/data/src/artifacts.rs` | Modify | 35 artifact sets 4pc effects |

---

## Task 1: BuffableStat — 新 variant 追加

**Files:**
- Modify: `crates/core/src/buff_types.rs`
- Modify: `crates/core/src/team.rs:63-78` (is_unconditional)

- [ ] **Step 1: Write failing test — serde roundtrip for new variants**

Add to bottom of `crates/core/src/buff_types.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amplifying_bonus_serde_roundtrip() {
        let stat = BuffableStat::AmplifyingBonus;
        let json = serde_json::to_string(&stat).unwrap();
        let back: BuffableStat = serde_json::from_str(&json).unwrap();
        assert_eq!(stat, back);
    }

    #[test]
    fn test_transformative_bonus_serde_roundtrip() {
        let stat = BuffableStat::TransformativeBonus;
        let json = serde_json::to_string(&stat).unwrap();
        let back: BuffableStat = serde_json::from_str(&json).unwrap();
        assert_eq!(stat, back);
    }

    #[test]
    fn test_additive_bonus_serde_roundtrip() {
        let stat = BuffableStat::AdditiveBonus;
        let json = serde_json::to_string(&stat).unwrap();
        let back: BuffableStat = serde_json::from_str(&json).unwrap();
        assert_eq!(stat, back);
    }

    #[test]
    fn test_elemental_res_reduction_serde_roundtrip() {
        let stat = BuffableStat::ElementalResReduction(crate::types::Element::Pyro);
        let json = serde_json::to_string(&stat).unwrap();
        let back: BuffableStat = serde_json::from_str(&json).unwrap();
        assert_eq!(stat, back);
    }

    #[test]
    fn test_physical_res_reduction_serde_roundtrip() {
        let stat = BuffableStat::PhysicalResReduction;
        let json = serde_json::to_string(&stat).unwrap();
        let back: BuffableStat = serde_json::from_str(&json).unwrap();
        assert_eq!(stat, back);
    }
}
```

- [ ] **Step 2: Run tests — verify they fail**

Run: `cargo test -p genshin-calc-core buff_types::tests -v`
Expected: FAIL (variants don't exist)

- [ ] **Step 3: Add 5 new variants to BuffableStat**

In `crates/core/src/buff_types.rs`, add after `ShieldStrength`:

```rust
    // Reaction bonuses (not applied to StatProfile; kept as ResolvedBuff for consumer)
    AmplifyingBonus,
    TransformativeBonus,
    AdditiveBonus,
    // Enemy resistance reduction (not applied to StatProfile; kept as ResolvedBuff for consumer)
    ElementalResReduction(Element),
    PhysicalResReduction,
```

- [ ] **Step 4: Run tests — verify they pass**

Run: `cargo test -p genshin-calc-core buff_types::tests -v`
Expected: PASS

- [ ] **Step 5: Write failing test — is_unconditional excludes new variants**

Add to `crates/core/src/team.rs` tests module:

```rust
    #[test]
    fn test_new_stats_are_conditional() {
        assert!(!is_unconditional(&BuffableStat::AmplifyingBonus));
        assert!(!is_unconditional(&BuffableStat::TransformativeBonus));
        assert!(!is_unconditional(&BuffableStat::AdditiveBonus));
        assert!(!is_unconditional(&BuffableStat::ElementalResReduction(Element::Pyro)));
        assert!(!is_unconditional(&BuffableStat::PhysicalResReduction));
    }
```

Note: `is_unconditional` is `fn` (not `pub fn`), so the test module within `team.rs` can access it.

- [ ] **Step 6: Run test — verify it passes (already excluded by wildcard match)**

Run: `cargo test -p genshin-calc-core team::tests::test_new_stats_are_conditional -v`
Expected: PASS — the `_ => {}` arm in `apply_buffs_to_profile` and the exhaustive match in `is_unconditional` already exclude unknown variants. Verify this is the case; if `is_unconditional` uses an exhaustive match that needs updating, add the new variants.

- [ ] **Step 7: Run full core test suite**

Run: `cargo test -p genshin-calc-core`
Expected: All tests pass

- [ ] **Step 8: Commit**

```bash
git add crates/core/src/buff_types.rs crates/core/src/team.rs
git commit -m "feat: add 5 new BuffableStat variants for reaction bonuses and resistance reduction"
```

---

## Task 2: ConditionalBuff — stack_values + target フィールド追加

**Files:**
- Modify: `crates/data/src/buff.rs:74-89` (ConditionalBuff struct)
- Modify: `crates/data/src/artifacts.rs` (existing 3 consts migration)

- [ ] **Step 1: Add stack_values and target to ConditionalBuff**

In `crates/data/src/buff.rs`, add after `refinement_values` field:

```rust
    /// Per-stack cumulative values for non-linear stacks. None = linear (value * stacks).
    pub stack_values: Option<&'static [f64]>,
    /// Who receives this buff.
    pub target: BuffTarget,
```

Add import at top of `crates/data/src/buff.rs`:

```rust
use genshin_calc_core::BuffTarget;
```

- [ ] **Step 2: Fix compilation — migrate existing ConditionalBuff consts**

In `crates/data/src/artifacts.rs`, add `stack_values: None, target: BuffTarget::OnlySelf,` to the 3 existing ConditionalBuff definitions:

1. CRIMSON_WITCH `cwof_pyro_stacks` (line ~28)
2. GLADIATORS_FINALE `gladiator_normal_bonus` (line ~54)
3. EMBLEM_OF_SEVERED_FATE (search for `emblem_burst_scaling`)

Also add `BuffTarget` to imports at top of artifacts.rs:

```rust
use genshin_calc_core::{BuffTarget, Element, WeaponType};
```

Search all weapon files for ConditionalBuff usages and migrate them too:

Run: `grep -rn "ConditionalBuff {" crates/data/src/weapons/`

Add `stack_values: None, target: BuffTarget::OnlySelf,` to each.

- [ ] **Step 3: Run full test suite — verify everything compiles and passes**

Run: `cargo test`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/buff.rs crates/data/src/artifacts.rs crates/data/src/weapons/
git commit -m "feat: add stack_values and target fields to ConditionalBuff"
```

---

## Task 3: AutoCondition — TeamSameElementCount / TeamDiffElementCount

**Files:**
- Modify: `crates/data/src/buff.rs:30-51` (AutoCondition enum)
- Modify: `crates/data/src/team_builder.rs:375-427` (eval_auto)

- [ ] **Step 1: Write failing test for TeamSameElementCount**

Add to `crates/data/src/team_builder.rs` tests module:

```rust
    #[test]
    fn test_eval_auto_team_same_element_count_pass() {
        let profile = StatProfile::default();
        // Character is Pyro, team has 2 Pyro members
        let team = vec![Element::Pyro, Element::Pyro, Element::Hydro, Element::Dendro];
        let cond = AutoCondition::TeamSameElementCount { min_count: 1 };
        let result = eval_auto(&cond, 0.14, &profile, WeaponType::Sword, Element::Pyro, &team);
        assert!(result.is_some());
        assert!((result.unwrap() - 0.14).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_team_same_element_count_fail() {
        let profile = StatProfile::default();
        // Character is Pyro, team has 0 other Pyro members (only self)
        let team = vec![Element::Pyro, Element::Hydro, Element::Dendro, Element::Cryo];
        let cond = AutoCondition::TeamSameElementCount { min_count: 2 };
        let result = eval_auto(&cond, 0.14, &profile, WeaponType::Sword, Element::Pyro, &team);
        // Only 1 Pyro (self counts), need 2 → depends on whether self is included
        // Gilded Dreams counts teammates excluding self, so team_elements should exclude self
        // With team_elements = [Hydro, Dendro, Cryo], same-element count = 0
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_team_diff_element_count_pass() {
        let profile = StatProfile::default();
        // Character is Pyro, team has 3 non-Pyro members
        let team = vec![Element::Pyro, Element::Hydro, Element::Dendro, Element::Cryo];
        let cond = AutoCondition::TeamDiffElementCount { min_count: 2 };
        let result = eval_auto(&cond, 50.0, &profile, WeaponType::Sword, Element::Pyro, &team);
        assert!(result.is_some());
        assert!((result.unwrap() - 50.0).abs() < EPSILON);
    }
```

- [ ] **Step 2: Run tests — verify they fail**

Run: `cargo test -p genshin-calc-data team_builder::tests::test_eval_auto_team_same -v`
Expected: FAIL (variants don't exist)

- [ ] **Step 3: Add AutoCondition variants and eval_auto branches**

In `crates/data/src/buff.rs` AutoCondition enum, add:

```rust
    /// Requires N+ teammates of the SAME element as the character (excludes self).
    TeamSameElementCount { min_count: u8 },
    /// Requires N+ teammates of DIFFERENT elements than the character (excludes self).
    TeamDiffElementCount { min_count: u8 },
```

In `crates/data/src/team_builder.rs` eval_auto function, add match arms:

```rust
        AutoCondition::TeamSameElementCount { min_count } => {
            if team_elements.is_empty() {
                return None;
            }
            // team_elements includes self; count same-element teammates excluding self
            let same_count = team_elements.iter().filter(|e| **e == element).count().saturating_sub(1) as u8;
            if same_count >= *min_count {
                Some(multiplier)
            } else {
                None
            }
        }
        AutoCondition::TeamDiffElementCount { min_count } => {
            if team_elements.is_empty() {
                return None;
            }
            let diff_count = team_elements.iter().filter(|e| **e != element).count() as u8;
            if diff_count >= *min_count {
                Some(multiplier)
            } else {
                None
            }
        }
```

- [ ] **Step 4: Run tests — verify they pass**

Run: `cargo test -p genshin-calc-data team_builder::tests::test_eval_auto_team -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/buff.rs crates/data/src/team_builder.rs
git commit -m "feat: add TeamSameElementCount/TeamDiffElementCount AutoCondition variants"
```

---

## Task 4: eval_manual — シグネチャ変更 + stack_values 対応

**Files:**
- Modify: `crates/data/src/team_builder.rs:431-454` (eval_manual)
- Modify: `crates/data/src/team_builder.rs:256-304` (resolve_conditionals)

- [ ] **Step 1: Write failing test for stack_values (non-linear)**

Add to `crates/data/src/team_builder.rs` tests module:

```rust
    #[test]
    fn test_eval_manual_stack_values_nonlinear() {
        let buff = ConditionalBuff {
            name: "test_nonlinear",
            description: "test",
            stat: BuffableStat::AtkPercent,
            value: 0.0,
            refinement_values: None,
            stack_values: Some(&[0.04, 0.09, 0.15]),
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Stacks(3)),
        };
        let activations = vec![("test_nonlinear", ManualActivation::Stacks(2))];
        let result = eval_manual(&ManualCondition::Stacks(3), &buff, &activations);
        assert!(result.is_some());
        assert!((result.unwrap() - 0.09).abs() < EPSILON);
    }

    #[test]
    fn test_eval_manual_stack_values_zero_stacks() {
        let buff = ConditionalBuff {
            name: "test_zero",
            description: "test",
            stat: BuffableStat::AtkPercent,
            value: 0.0,
            refinement_values: None,
            stack_values: Some(&[0.04, 0.09, 0.15]),
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Stacks(3)),
        };
        let activations = vec![("test_zero", ManualActivation::Stacks(0))];
        let result = eval_manual(&ManualCondition::Stacks(3), &buff, &activations);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_manual_stack_values_none_uses_linear() {
        let buff = ConditionalBuff {
            name: "test_linear",
            description: "test",
            stat: BuffableStat::CritRate,
            value: 0.12,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Stacks(3)),
        };
        let activations = vec![("test_linear", ManualActivation::Stacks(2))];
        let result = eval_manual(&ManualCondition::Stacks(3), &buff, &activations);
        assert!(result.is_some());
        assert!((result.unwrap() - 0.24).abs() < EPSILON);
    }
```

- [ ] **Step 2: Run tests — verify they fail**

Run: `cargo test -p genshin-calc-data test_eval_manual_stack_values -v`
Expected: FAIL (signature mismatch)

- [ ] **Step 3: Change eval_manual signature and implement stack_values**

Replace the `eval_manual` function in `crates/data/src/team_builder.rs`:

```rust
fn eval_manual(
    cond: &ManualCondition,
    buff: &ConditionalBuff,
    activations: &[(&str, ManualActivation)],
) -> Option<f64> {
    let activation = activations.iter().find(|(name, _)| *name == buff.name);
    match cond {
        ManualCondition::Toggle => match activation {
            Some((_, ManualActivation::Active)) => Some(buff.value),
            _ => None,
        },
        ManualCondition::Stacks(max) => match activation {
            Some((_, ManualActivation::Stacks(n))) => {
                let effective = (*n).min(*max);
                if effective == 0 {
                    return None;
                }
                match buff.stack_values {
                    Some(values) => Some(values[(effective as usize).min(values.len()) - 1]),
                    None => Some(buff.value * f64::from(effective)),
                }
            }
            Some((_, ManualActivation::Active)) => {
                let effective = *max;
                match buff.stack_values {
                    Some(values) => Some(values[(effective as usize).min(values.len()) - 1]),
                    None => Some(buff.value * f64::from(effective)),
                }
            }
            _ => None,
        },
    }
}
```

- [ ] **Step 4: Update all call sites of eval_manual**

In `resolve_conditionals` closure (~line 271 and ~line 286), change:

```rust
// Before:
eval_manual(manual, cond_buff.name, cond_buff.value, &self.manual_activations)

// After:
eval_manual(manual, cond_buff, &self.manual_activations)
```

Both the `Activation::Manual` and `Activation::Both` arms need updating.

- [ ] **Step 5: Update resolve_conditionals to use buff.target**

In the `resolve_conditionals` closure, remove `target: BuffTarget` parameter and use `cond_buff.target`:

```rust
let resolve_conditionals =
    |conditional_buffs: &'static [ConditionalBuff],
     source_name: &str,
     buffs: &mut Vec<ResolvedBuff>| {
        for cond_buff in conditional_buffs {
            // ... eval logic unchanged ...
            if let Some(value) = resolved_value {
                buffs.push(ResolvedBuff {
                    source: format!("{} ({})", cond_buff.name, source_name),
                    stat: cond_buff.stat,
                    value,
                    target: cond_buff.target,
                });
            }
        }
    };
```

Update all 3 call sites to remove the `BuffTarget::OnlySelf` argument:

```rust
resolve_conditionals(passive.effect.conditional_buffs, weapon.name, &mut buffs);
resolve_conditionals(set.two_piece.conditional_buffs, &format!("{} 2pc", set.name), &mut buffs);
resolve_conditionals(set.four_piece.conditional_buffs, &format!("{} 4pc", set.name), &mut buffs);
```

- [ ] **Step 6: Run tests — verify they pass**

Run: `cargo test -p genshin-calc-data`
Expected: All tests pass (both new and existing)

- [ ] **Step 7: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "feat: update eval_manual for stack_values and resolve_conditionals for buff.target"
```

---

## Task 5: 聖遺物4pc — Simple Toggle セット (batch 1: 14セット)

**Files:**
- Modify: `crates/data/src/artifacts.rs`

These sets each have 1-3 ConditionalBuff entries with `Activation::Manual(ManualCondition::Toggle)` and `target: BuffTarget::OnlySelf`.

- [ ] **Step 1: Write data integrity test**

Add a new test file or add to existing tests in `crates/data/src/artifacts.rs`:

```rust
#[test]
fn test_simple_toggle_sets_have_conditional_buffs() {
    let sets = [
        ("shimenawas_reminiscence", 3),  // Normal+Charged+Plunging
        ("heart_of_depth", 2),            // Normal+Charged
        ("retracing_bolide", 2),          // Normal+Charged
        ("desert_pavilion_chronicle", 3), // Normal+Charged+Plunging
        ("unfinished_reverie", 1),        // DmgBonus
        ("nighttime_whispers_in_the_echoing_woods", 1), // GeoDmg
        ("obsidian_codex", 2),            // DmgBonus + CritRate
        ("brave_heart", 1),               // DmgBonus
        ("berserker", 1),                 // CritRate
        ("martial_artist", 2),            // Normal+Charged
        ("thundersoother", 1),            // DmgBonus
        ("lavawalker", 1),                // DmgBonus
        ("bloodstained_chivalry", 1),     // ChargedAtkDmgBonus
        ("resolution_of_sojourner", 1),   // CritRate
    ];
    for (id, expected_count) in sets {
        let set = crate::find_artifact_set(id).unwrap_or_else(|| panic!("Set {} not found", id));
        assert_eq!(
            set.four_piece.conditional_buffs.len(),
            expected_count,
            "Set {} expected {} conditional_buffs, got {}",
            id,
            expected_count,
            set.four_piece.conditional_buffs.len()
        );
    }
}
```

- [ ] **Step 2: Run test — verify it fails**

Run: `cargo test -p genshin-calc-data test_simple_toggle_sets -v`
Expected: FAIL (conditional_buffs are empty)

- [ ] **Step 3: Implement 14 simple Toggle sets**

For each set in `crates/data/src/artifacts.rs`, replace `conditional_buffs: &[]` in `four_piece` with the appropriate ConditionalBuff entries. Example pattern:

```rust
// しめ縄の記憶
four_piece: SetEffect {
    description: "...",
    buffs: &[],
    conditional_buffs: &[
        ConditionalBuff {
            name: "shimenawa_normal",
            description: "Normal ATK DMG +50% after using Skill",
            stat: BuffableStat::NormalAtkDmgBonus,
            value: 0.50,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Toggle),
        },
        ConditionalBuff {
            name: "shimenawa_charged",
            description: "Charged ATK DMG +50% after using Skill",
            stat: BuffableStat::ChargedAtkDmgBonus,
            value: 0.50,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Toggle),
        },
        ConditionalBuff {
            name: "shimenawa_plunging",
            description: "Plunging ATK DMG +50% after using Skill",
            stat: BuffableStat::PlungingAtkDmgBonus,
            value: 0.50,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Toggle),
        },
    ],
},
```

Apply same pattern to all 14 sets per spec values:
- `heart_of_depth`: Normal+Charged +0.30
- `retracing_bolide`: Normal+Charged +0.40
- `desert_pavilion_chronicle`: Normal+Charged+Plunging +0.40
- `unfinished_reverie`: DmgBonus +0.50
- `nighttime_whispers`: ElementalDmgBonus(Geo) +0.20
- `obsidian_codex`: DmgBonus +0.25 ("obsidian_dmg"), CritRate +0.40 ("obsidian_crit")
- `brave_heart`: DmgBonus +0.30
- `berserker`: CritRate +0.24
- `martial_artist`: Normal+Charged +0.25
- `thundersoother`: DmgBonus +0.35
- `lavawalker`: DmgBonus +0.35
- `bloodstained_chivalry`: ChargedAtkDmgBonus +0.50
- `resolution_of_sojourner`: CritRate +0.30

- [ ] **Step 4: Run test — verify it passes**

Run: `cargo test -p genshin-calc-data test_simple_toggle_sets -v`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/artifacts.rs
git commit -m "feat: add 4pc ConditionalBuff for 14 simple Toggle artifact sets"
```

---

## Task 6: 聖遺物4pc — Team Toggle セット (3セット)

**Files:**
- Modify: `crates/data/src/artifacts.rs`

- [ ] **Step 1: Write test for Team target sets**

```rust
#[test]
fn test_team_toggle_sets() {
    let noblesse = crate::find_artifact_set("noblesse_oblige").unwrap();
    let nb = &noblesse.four_piece.conditional_buffs;
    assert_eq!(nb.len(), 1);
    assert_eq!(nb[0].stat, BuffableStat::AtkPercent);
    assert!((nb[0].value - 0.20).abs() < 0.001);
    assert_eq!(nb[0].target, BuffTarget::Team);

    let tenacity = crate::find_artifact_set("tenacity_of_the_millelith").unwrap();
    let tb = &tenacity.four_piece.conditional_buffs;
    assert_eq!(tb.len(), 2);
    assert_eq!(tb[0].target, BuffTarget::Team);
    assert_eq!(tb[1].target, BuffTarget::Team);

    let instructor = crate::find_artifact_set("instructor").unwrap();
    let ib = &instructor.four_piece.conditional_buffs;
    assert_eq!(ib.len(), 1);
    assert_eq!(ib[0].stat, BuffableStat::ElementalMastery);
    assert!((ib[0].value - 120.0).abs() < 0.001);
    assert_eq!(ib[0].target, BuffTarget::Team);
}
```

- [ ] **Step 2: Run test — verify it fails**

Run: `cargo test -p genshin-calc-data test_team_toggle_sets -v`

- [ ] **Step 3: Implement 3 Team Toggle sets**

- `noblesse_oblige`: AtkPercent +0.20, target: Team, Toggle
- `tenacity_of_the_millelith`: [AtkPercent +0.20, ShieldStrength +0.30], target: Team, Toggle
- `instructor`: EM +120.0, target: Team, Toggle

- [ ] **Step 4: Run test — verify it passes**

Run: `cargo test -p genshin-calc-data test_team_toggle_sets -v`

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/artifacts.rs
git commit -m "feat: add 4pc ConditionalBuff for team-target Toggle sets (Noblesse, Tenacity, Instructor)"
```

---

## Task 7: 聖遺物4pc — Reaction Toggle + CW 追加 (2セット)

**Files:**
- Modify: `crates/data/src/artifacts.rs`

- [ ] **Step 1: Write test**

```rust
#[test]
fn test_reaction_toggle_sets() {
    // Crimson Witch should now have stacks + 2 reaction toggles
    let cw = crate::find_artifact_set("crimson_witch").unwrap();
    let cb = &cw.four_piece.conditional_buffs;
    assert!(cb.len() >= 3); // stacks + amplifying + transformative
    let amp = cb.iter().find(|b| b.name == "cw_amplifying").unwrap();
    assert_eq!(amp.stat, BuffableStat::AmplifyingBonus);
    assert!((amp.value - 0.15).abs() < 0.001);
    let trans = cb.iter().find(|b| b.name == "cw_transformative").unwrap();
    assert_eq!(trans.stat, BuffableStat::TransformativeBonus);
    assert!((trans.value - 0.40).abs() < 0.001);

    // Thundering Fury
    let tf = crate::find_artifact_set("thundering_fury").unwrap();
    let tb = &tf.four_piece.conditional_buffs;
    assert!(tb.len() >= 2);
    let tf_trans = tb.iter().find(|b| b.name == "tf_transformative").unwrap();
    assert_eq!(tf_trans.stat, BuffableStat::TransformativeBonus);
    assert!((tf_trans.value - 0.40).abs() < 0.001);
    let tf_add = tb.iter().find(|b| b.name == "tf_additive").unwrap();
    assert_eq!(tf_add.stat, BuffableStat::AdditiveBonus);
    assert!((tf_add.value - 0.20).abs() < 0.001);
}
```

- [ ] **Step 2: Run test — verify it fails**

- [ ] **Step 3: Implement**

Add to CW existing conditional_buffs array (after cwof_pyro_stacks):
- `cw_amplifying`: AmplifyingBonus +0.15, Toggle
- `cw_transformative`: TransformativeBonus +0.40, Toggle

Add to TF four_piece.conditional_buffs:
- `tf_transformative`: TransformativeBonus +0.40, Toggle
- `tf_additive`: AdditiveBonus +0.20, Toggle

- [ ] **Step 4: Run test — verify it passes**

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/artifacts.rs
git commit -m "feat: add reaction bonus ConditionalBuffs for Crimson Witch and Thundering Fury"
```

---

## Task 8: 聖遺物4pc — Auto 条件セット (2セット)

**Files:**
- Modify: `crates/data/src/artifacts.rs`

- [ ] **Step 1: Write test**

```rust
#[test]
fn test_auto_condition_sets() {
    let wt = crate::find_artifact_set("wanderers_troupe").unwrap();
    let cb = &wt.four_piece.conditional_buffs;
    assert_eq!(cb.len(), 1);
    assert_eq!(cb[0].stat, BuffableStat::ChargedAtkDmgBonus);
    assert!((cb[0].value - 0.35).abs() < 0.001);

    let cs = crate::find_artifact_set("chronicled_sands_and_water").unwrap();
    let cb = &cs.four_piece.conditional_buffs;
    assert!(cb.len() >= 2); // SkillDmgBonus + BurstDmgBonus
}
```

- [ ] **Step 2: Run test — verify it fails**

- [ ] **Step 3: Implement**

- `wanderers_troupe`: ChargedAtkDmgBonus +0.35, Auto(WeaponTypeRequired(&[Catalyst, Bow]))
- `chronicled_sands_and_water`: SkillDmgBonus + BurstDmgBonus, Auto(StatScaling{stat: EnergyRecharge, cap: ...})
  - Need to determine the cap value from game data. The 4pc description says "ER based Skill/Burst bonus" — check the actual formula and cap.

- [ ] **Step 4: Run test — verify it passes**

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/artifacts.rs
git commit -m "feat: add 4pc ConditionalBuff for Auto condition sets (Wanderer's Troupe, Chronicled Sands)"
```

---

## Task 9: 聖遺物4pc — Linear Stacks セット (4セット)

**Files:**
- Modify: `crates/data/src/artifacts.rs`

- [ ] **Step 1: Write test**

```rust
#[test]
fn test_linear_stacks_sets() {
    let bs = crate::find_artifact_set("blizzard_strayer").unwrap();
    let cb = &bs.four_piece.conditional_buffs;
    assert_eq!(cb.len(), 1);
    assert_eq!(cb[0].stat, BuffableStat::CritRate);
    assert!((cb[0].value - 0.20).abs() < 0.001);
    assert!(cb[0].stack_values.is_none());

    let husk = crate::find_artifact_set("husk_of_opulent_dreams").unwrap();
    let cb = &husk.four_piece.conditional_buffs;
    assert_eq!(cb.len(), 2); // DefPercent + GeoDmg

    let mh = crate::find_artifact_set("marechaussee_hunter").unwrap();
    let cb = &mh.four_piece.conditional_buffs;
    assert_eq!(cb.len(), 1);
    assert!((cb[0].value - 0.12).abs() < 0.001);

    let fhw = crate::find_artifact_set("fragment_of_harmonic_whimsy").unwrap();
    let cb = &fhw.four_piece.conditional_buffs;
    assert_eq!(cb.len(), 1);
    assert!((cb[0].value - 0.18).abs() < 0.001);
}
```

- [ ] **Step 2: Run test — verify it fails**

- [ ] **Step 3: Implement 4 linear Stacks sets**

- `blizzard_strayer`: CritRate +0.20/stack, Stacks(2)
- `husk_of_opulent_dreams`: [DefPercent +0.06/stack, GeoDmg +0.06/stack], Stacks(4)
- `marechaussee_hunter`: CritRate +0.12/stack, Stacks(3)
- `fragment_of_harmonic_whimsy`: DmgBonus +0.18/stack, Stacks(3)

- [ ] **Step 4: Run test — verify it passes**

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/artifacts.rs
git commit -m "feat: add 4pc ConditionalBuff for linear Stacks sets (Blizzard, Husk, Marechaussee, Fragment)"
```

---

## Task 10: 聖遺物4pc — Non-linear Stacks + Stacks+Toggle 複合 (2セット)

**Files:**
- Modify: `crates/data/src/artifacts.rs`

- [ ] **Step 1: Write test**

```rust
#[test]
fn test_nonlinear_stacks_sets() {
    // Nymph's Dream
    let nd = crate::find_artifact_set("nymphs_dream").unwrap();
    let cb = &nd.four_piece.conditional_buffs;
    assert_eq!(cb.len(), 2); // AtkPercent + HydroDmg
    let atk = cb.iter().find(|b| b.stat == BuffableStat::AtkPercent).unwrap();
    assert_eq!(atk.stack_values, Some([0.04, 0.09, 0.15].as_slice()));
    let hydro = cb.iter().find(|b| b.stat == BuffableStat::ElementalDmgBonus(Element::Hydro)).unwrap();
    assert_eq!(hydro.stack_values, Some([0.07, 0.16, 0.25].as_slice()));
}

#[test]
fn test_pale_flame_stacks_plus_toggle() {
    let pf = crate::find_artifact_set("pale_flame").unwrap();
    let cb = &pf.four_piece.conditional_buffs;
    assert_eq!(cb.len(), 2); // AtkPercent stacks + PhysDMG toggle
    let atk = cb.iter().find(|b| b.stat == BuffableStat::AtkPercent).unwrap();
    assert!((atk.value - 0.09).abs() < 0.001);
    assert!(atk.stack_values.is_none()); // linear
    let phys = cb.iter().find(|b| b.stat == BuffableStat::PhysicalDmgBonus).unwrap();
    assert!((phys.value - 0.25).abs() < 0.001);
}
```

- [ ] **Step 2: Run test — verify it fails**

- [ ] **Step 3: Implement**

- `nymphs_dream`: AtkPercent Stacks(3) stack_values=[0.04,0.09,0.15], HydroDmg Stacks(3) stack_values=[0.07,0.16,0.25]
- `pale_flame`: AtkPercent Stacks(2) +0.09 (linear), PhysicalDmgBonus Toggle +0.25

- [ ] **Step 4: Run test — verify it passes**

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/artifacts.rs
git commit -m "feat: add 4pc ConditionalBuff for Nymph's Dream (non-linear) and Pale Flame (stacks+toggle)"
```

---

## Task 11: 聖遺物4pc — Base + Toggle/Stacks セット (5セット)

**Files:**
- Modify: `crates/data/src/artifacts.rs`

- [ ] **Step 1: Write test**

```rust
#[test]
fn test_base_plus_conditional_sets() {
    // Golden Troupe: base StatBuff + Toggle
    let gt = crate::find_artifact_set("golden_troupe").unwrap();
    assert_eq!(gt.four_piece.buffs.len(), 1);
    assert_eq!(gt.four_piece.buffs[0].stat, BuffableStat::SkillDmgBonus);
    assert!((gt.four_piece.buffs[0].value - 0.25).abs() < 0.001);
    assert_eq!(gt.four_piece.conditional_buffs.len(), 1);
    assert_eq!(gt.four_piece.conditional_buffs[0].stat, BuffableStat::SkillDmgBonus);

    // Vourukasha: base StatBuff + Stacks
    let vg = crate::find_artifact_set("vourukashas_glow").unwrap();
    assert_eq!(vg.four_piece.buffs.len(), 2); // SkillDmgBonus + BurstDmgBonus
    assert_eq!(vg.four_piece.conditional_buffs.len(), 2); // Stacks for each

    // Vermillion: Toggle base + Stacks
    let vh = crate::find_artifact_set("vermillion_hereafter").unwrap();
    assert!(vh.four_piece.conditional_buffs.len() >= 2); // Toggle + Stacks

    // Flower of Paradise Lost: Toggle base + Stacks
    let fopl = crate::find_artifact_set("flower_of_paradise_lost").unwrap();
    assert!(fopl.four_piece.conditional_buffs.len() >= 2);
    let base = fopl.four_piece.conditional_buffs.iter().find(|b| b.name == "fopl_bloom_base").unwrap();
    assert_eq!(base.stat, BuffableStat::TransformativeBonus);
    assert!((base.value - 0.40).abs() < 0.001);
}
```

- [ ] **Step 2: Run test — verify it fails**

- [ ] **Step 3: Implement 5 sets**

- `golden_troupe`: buffs=[SkillDmgBonus +0.25], conditional_buffs=[SkillDmgBonus +0.25 Toggle "golden_troupe_offfield"]
- `vourukashas_glow`: buffs=[SkillDmgBonus +0.10, BurstDmgBonus +0.10], conditional_buffs=[SkillDmgBonus +0.08 Stacks(5), BurstDmgBonus +0.08 Stacks(5)]
- `vermillion_hereafter`: conditional_buffs=[AtkPercent +0.08 Toggle "vermillion_base", AtkPercent +0.10 Stacks(4) "vermillion_stacks"]
- `flower_of_paradise_lost`: conditional_buffs=[TransformativeBonus +0.40 Toggle "fopl_bloom_base", TransformativeBonus +0.10 Stacks(4) "fopl_bloom_stacks"]

- [ ] **Step 4: Run test — verify it passes**

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/artifacts.rs
git commit -m "feat: add 4pc effects for base+conditional sets (Golden Troupe, Vourukasha, Vermillion, FoPL)"
```

---

## Task 12: 聖遺物4pc — Team 条件 + 耐性減少 (4セット)

**Files:**
- Modify: `crates/data/src/artifacts.rs`

- [ ] **Step 1: Write test**

```rust
#[test]
fn test_team_condition_sets() {
    // Gilded Dreams: 6 ConditionalBuffs
    let gd = crate::find_artifact_set("gilded_dreams").unwrap();
    assert_eq!(gd.four_piece.conditional_buffs.len(), 6);

    // Scroll
    let scroll = crate::find_artifact_set("scroll_of_the_hero_of_cinder_city").unwrap();
    assert!(scroll.four_piece.conditional_buffs.len() >= 8); // EM + 7 element DMG
}

#[test]
fn test_resistance_reduction_sets() {
    // Viridescent Venerer
    let vv = crate::find_artifact_set("viridescent_venerer").unwrap();
    let cb = &vv.four_piece.conditional_buffs;
    assert!(cb.len() >= 5); // swirl bonus + 4 element res shred
    let swirl = cb.iter().find(|b| b.name == "vv_swirl_bonus").unwrap();
    assert_eq!(swirl.stat, BuffableStat::TransformativeBonus);
    assert!((swirl.value - 0.60).abs() < 0.001);
    let pyro_shred = cb.iter().find(|b| b.name == "vv_res_shred_pyro").unwrap();
    assert_eq!(pyro_shred.stat, BuffableStat::ElementalResReduction(Element::Pyro));
    assert!((pyro_shred.value - 0.40).abs() < 0.001);
    assert_eq!(pyro_shred.target, BuffTarget::Team);

    // Deepwood Memories
    let dm = crate::find_artifact_set("deepwood_memories").unwrap();
    let cb = &dm.four_piece.conditional_buffs;
    assert_eq!(cb.len(), 1);
    assert_eq!(cb[0].stat, BuffableStat::ElementalResReduction(Element::Dendro));
    assert!((cb[0].value - 0.30).abs() < 0.001);
    assert_eq!(cb[0].target, BuffTarget::Team);
}
```

- [ ] **Step 2: Run test — verify it fails**

- [ ] **Step 3: Implement 4 sets**

- `gilded_dreams`: 6 ConditionalBuffs with Auto(TeamSameElementCount/TeamDiffElementCount)
- `scroll_of_the_hero_of_cinder_city`: EM +40 Toggle(Team) + 7× ElementalDmgBonus Toggle(Team)
- `viridescent_venerer`: TransformativeBonus +0.60 Toggle + 4× ElementalResReduction Toggle(Team)
- `deepwood_memories`: ElementalResReduction(Dendro) -0.30 Toggle(Team)

- [ ] **Step 4: Run test — verify it passes**

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/artifacts.rs
git commit -m "feat: add 4pc effects for team condition and resistance reduction sets (Gilded, Scroll, VV, Deepwood)"
```

---

## Task 13: データ整合性テスト + 全体検証

**Files:**
- Modify: `crates/data/src/artifacts.rs` (test section)

- [ ] **Step 1: Write comprehensive data integrity test**

```rust
#[test]
fn test_all_artifact_sets_4pc_coverage() {
    use crate::artifacts::ALL_ARTIFACT_SETS;

    // Description-only sets (no conditional_buffs expected)
    let desc_only = [
        "ocean_hued_clam", "maiden_beloved", "archaic_petra",
        "song_of_days_past", "glory_of_the_ancient_sea", "echoes_of_an_offering",
        "exile", "gambler", "scholar", "tiny_miracle", "defenders_will",
    ];

    // No 4pc sets
    let no_4pc = [
        "prayers_for_illumination", "prayers_for_destiny",
        "prayers_for_wisdom", "prayers_to_springtime",
    ];

    for set in ALL_ARTIFACT_SETS {
        if no_4pc.contains(&set.id) {
            continue;
        }
        if desc_only.contains(&set.id) {
            assert!(
                set.four_piece.conditional_buffs.is_empty()
                    && set.four_piece.buffs.is_empty(),
                "Description-only set {} should have no buffs",
                set.id
            );
            continue;
        }
        // All other sets must have EITHER buffs or conditional_buffs in four_piece
        assert!(
            !set.four_piece.conditional_buffs.is_empty() || !set.four_piece.buffs.is_empty(),
            "Set {} has empty 4pc effects but is not in desc_only or no_4pc list",
            set.id
        );
    }
}
```

- [ ] **Step 2: Run test — verify it passes**

Run: `cargo test -p genshin-calc-data test_all_artifact_sets_4pc_coverage -v`
Expected: PASS

- [ ] **Step 3: Run full test suite**

Run: `cargo test`
Expected: All tests pass

- [ ] **Step 4: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/artifacts.rs
git commit -m "test: add comprehensive 4pc coverage integrity test for all 52 artifact sets"
```

---

## Task 14: TODO ドキュメント更新

**Files:**
- Modify: `docs/data-expansion-todo.md`

- [ ] **Step 1: Update P2 checkboxes**

Mark all P2 items as complete. Update the summary table artifact coverage.

- [ ] **Step 2: Commit**

```bash
git add docs/data-expansion-todo.md
git commit -m "docs: mark P2 artifact 4pc effects as complete in data expansion TODO"
```
