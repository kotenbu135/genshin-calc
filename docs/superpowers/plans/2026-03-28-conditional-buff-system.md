# Conditional Buff System (P0) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enable conditional weapon/artifact effects (HP thresholds, weapon type restrictions, stat scaling, stacks) in the data crate, unlocking P2-P6 data expansion.

**Architecture:** All new types (`ConditionalBuff`, `Activation`, `AutoCondition`, `ManualCondition`) live in the data crate. Core crate has zero changes. The `TeamMemberBuilder.build()` evaluates Auto conditions from character/team data and Manual conditions from user-provided activation config. Output is the existing `ResolvedBuff` type.

**Tech Stack:** Rust, serde (Serialize only for &'static types), cargo test

**Spec:** `docs/superpowers/specs/2026-03-28-conditional-buff-system-design.md`

---

## File Structure

| File | Action | Responsibility |
|------|--------|---------------|
| `crates/data/src/buff.rs` | Modify | Add `ConditionalBuff`, `Activation`, `AutoCondition`, `ManualCondition`, `ManualActivation`, `AvailableConditional`; extend `PassiveEffect` |
| `crates/data/src/types.rs` | Modify | Extend `SetEffect` with `conditional_buffs` field |
| `crates/data/src/team_builder.rs` | Modify | Add `activate()`, `activate_with_stacks()`, `team_elements()`, `available_conditionals()`; add eval logic in `build()` |
| `crates/data/src/weapons/*.rs` | Modify | Add `conditional_buffs: &[]` to all 220 `PassiveEffect` blocks |
| `crates/data/src/artifacts.rs` | Modify | Add `conditional_buffs: &[]` to all 104 `SetEffect` blocks |
| `crates/data/src/lib.rs` | Modify | Re-export new public types |

Note: 230 weapons total, but 10 have `passive: None` → 220 `PassiveEffect` blocks to modify.

---

## Task 1: New Types in buff.rs (compile-safe)

Add all new types first WITHOUT extending `PassiveEffect`/`SetEffect`. This keeps the project compiling throughout.

**Files:**
- Modify: `crates/data/src/buff.rs`

- [ ] **Step 1: Add all new types**

Add after `PassiveEffect` struct in `buff.rs`:

```rust
use genshin_calc_core::{Element, WeaponType};

/// Condition the builder can evaluate automatically from character/team data.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AutoCondition {
    /// Buff only applies to specific weapon types (e.g. Gladiator 4pc).
    WeaponTypeRequired(&'static [WeaponType]),
    /// Buff only applies to characters of specific elements.
    ElementRequired(&'static [Element]),
    /// Buff value computed from a stat. Multiplier comes from ConditionalBuff.value.
    /// Final = stat_value * multiplier, capped at `cap` if set.
    /// The BuffableStat here indicates which "stat family" to read the total value from:
    /// - HpPercent → total HP (base_hp * (1 + hp_percent) + hp_flat)
    /// - AtkPercent → total ATK (base_atk * (1 + atk_percent) + atk_flat)
    /// - DefPercent → total DEF (base_def * (1 + def_percent) + def_flat)
    /// - ElementalMastery → elemental_mastery
    /// - EnergyRecharge → energy_recharge
    StatScaling {
        stat: BuffableStat,
        cap: Option<f64>,
    },
    /// Requires N+ team members of a specific element (e.g. Gorou).
    TeamElementCount {
        element: Element,
        min_count: u8,
    },
    /// Team must consist only of specified elements (e.g. Nilou: Hydro+Dendro).
    TeamElementsOnly(&'static [Element]),
}

/// Condition requiring user input (game state the builder cannot determine).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManualCondition {
    /// Simple on/off toggle (e.g. "HP below 50%").
    Toggle,
    /// Stackable buff with max stack count (e.g. CW 4pc max 3).
    Stacks(u8),
}

/// How a conditional buff is activated.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Activation {
    /// Evaluated automatically by the builder.
    Auto(AutoCondition),
    /// Requires user input.
    Manual(ManualCondition),
    /// Both conditions must be satisfied (Auto first, short-circuits).
    Both(AutoCondition, ManualCondition),
}

/// A stat buff that requires a condition to be active.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConditionalBuff {
    /// Machine-readable identifier (e.g. "homa_hp_bonus").
    pub name: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Which stat is buffed.
    pub stat: BuffableStat,
    /// Buff value (or multiplier for StatScaling).
    pub value: f64,
    /// Values at refinements 1-5. None for non-weapon / fixed buffs.
    /// TODO(P4): Builder will use refinement_values[r] when refinement level is available.
    pub refinement_values: Option<[f64; 5]>,
    /// Activation condition.
    pub activation: Activation,
}

/// User-provided activation state for a manual condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ManualActivation {
    /// Toggle ON.
    Active,
    /// Stackable buff with specified stack count.
    Stacks(u8),
}

/// A conditional buff with source context, for UI display.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AvailableConditional {
    /// Source name (e.g. "Staff of Homa", "Crimson Witch 4pc").
    pub source: &'static str,
    /// The conditional buff definition.
    pub buff: &'static ConditionalBuff,
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cargo build -p genshin-calc-data`

Expected: SUCCESS — no existing code is affected.

- [ ] **Step 3: Commit**

```bash
git add crates/data/src/buff.rs
git commit -m "feat(data): add conditional buff type definitions"
```

---

## Task 2: Extend PassiveEffect + SetEffect + Migrate All Data

Extend the structs and immediately fix all 324 usages in the same commit so the project never enters a broken state.

**Files:**
- Modify: `crates/data/src/buff.rs:17-23` (PassiveEffect)
- Modify: `crates/data/src/types.rs:276-281` (SetEffect)
- Modify: `crates/data/src/weapons/sword.rs` (51 PassiveEffect)
- Modify: `crates/data/src/weapons/claymore.rs` (40 PassiveEffect)
- Modify: `crates/data/src/weapons/polearm.rs` (37 PassiveEffect)
- Modify: `crates/data/src/weapons/bow.rs` (44 PassiveEffect)
- Modify: `crates/data/src/weapons/catalyst.rs` (48 PassiveEffect)
- Modify: `crates/data/src/artifacts.rs` (104 SetEffect)

- [ ] **Step 1: Extend PassiveEffect in buff.rs**

Change:
```rust
pub struct PassiveEffect {
    pub description: &'static str,
    pub buffs: &'static [StatBuff],
}
```

To:
```rust
pub struct PassiveEffect {
    pub description: &'static str,
    pub buffs: &'static [StatBuff],
    /// Conditional buffs that require activation.
    pub conditional_buffs: &'static [ConditionalBuff],
}
```

- [ ] **Step 2: Extend SetEffect in types.rs**

Change `crates/data/src/types.rs:276-281`:
```rust
pub struct SetEffect {
    pub description: &'static str,
    pub buffs: &'static [crate::buff::StatBuff],
    /// Conditional buffs that require activation.
    pub conditional_buffs: &'static [crate::buff::ConditionalBuff],
}
```

- [ ] **Step 3: Add `conditional_buffs: &[]` to all weapon PassiveEffect blocks**

For each of the 5 weapon files, add `conditional_buffs: &[],` to every `PassiveEffect { ... }` block. The pattern is consistent:

```rust
// Before
effect: PassiveEffect {
    description: "...",
    buffs: &[StatBuff { ... }],
},

// After
effect: PassiveEffect {
    description: "...",
    buffs: &[StatBuff { ... }],
    conditional_buffs: &[],
},
```

Process each file one at a time. After each file, run `cargo build -p genshin-calc-data` to verify.

Files in order: `sword.rs` (51), `claymore.rs` (40), `polearm.rs` (37), `bow.rs` (44), `catalyst.rs` (48).

- [ ] **Step 4: Add `conditional_buffs: &[]` to all artifact SetEffect blocks**

Same pattern for `artifacts.rs` — 52 sets × 2 (two_piece + four_piece) = 104 instances:

```rust
// Before
two_piece: SetEffect {
    description: "...",
    buffs: &[...],
},

// After
two_piece: SetEffect {
    description: "...",
    buffs: &[...],
    conditional_buffs: &[],
},
```

- [ ] **Step 5: Verify full build**

Run: `cargo build -p genshin-calc-data`

Expected: SUCCESS.

- [ ] **Step 6: Run all tests**

Run: `cargo test`

Expected: All existing tests pass. Zero behavior change.

- [ ] **Step 7: Commit**

```bash
git add crates/data/src/buff.rs crates/data/src/types.rs crates/data/src/weapons/ crates/data/src/artifacts.rs
git commit -m "feat(data): extend PassiveEffect/SetEffect with conditional_buffs field"
```

---

## Task 3: Builder Eval Functions (TDD)

**Files:**
- Modify: `crates/data/src/team_builder.rs`

- [ ] **Step 1: Write eval_auto tests**

Add to `crates/data/src/team_builder.rs`:

```rust
#[cfg(test)]
mod conditional_tests {
    use super::*;
    use crate::buff::*;
    use genshin_calc_core::{BuffableStat, Element, WeaponType};

    const EPSILON: f64 = 1e-6;

    // --- eval_auto tests ---

    #[test]
    fn test_eval_auto_weapon_type_match() {
        let cond = AutoCondition::WeaponTypeRequired(&[WeaponType::Sword, WeaponType::Claymore]);
        let result = eval_auto(&cond, 0.35, &StatProfile::default(), WeaponType::Sword, Element::Pyro, &[]);
        assert!((result.unwrap() - 0.35).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_weapon_type_mismatch() {
        let cond = AutoCondition::WeaponTypeRequired(&[WeaponType::Sword, WeaponType::Claymore]);
        let result = eval_auto(&cond, 0.35, &StatProfile::default(), WeaponType::Catalyst, Element::Pyro, &[]);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_element_match() {
        let cond = AutoCondition::ElementRequired(&[Element::Pyro, Element::Hydro]);
        let result = eval_auto(&cond, 0.20, &StatProfile::default(), WeaponType::Sword, Element::Pyro, &[]);
        assert!((result.unwrap() - 0.20).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_element_mismatch() {
        let cond = AutoCondition::ElementRequired(&[Element::Pyro]);
        let result = eval_auto(&cond, 0.20, &StatProfile::default(), WeaponType::Sword, Element::Cryo, &[]);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_stat_scaling_normal() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::EnergyRecharge,
            cap: Some(0.75),
        };
        let profile = StatProfile { energy_recharge: 1.80, ..Default::default() };
        // 1.80 * 0.25 = 0.45 (below cap)
        let result = eval_auto(&cond, 0.25, &profile, WeaponType::Sword, Element::Pyro, &[]);
        assert!((result.unwrap() - 0.45).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_stat_scaling_capped() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::EnergyRecharge,
            cap: Some(0.75),
        };
        let profile = StatProfile { energy_recharge: 4.00, ..Default::default() };
        // 4.00 * 0.25 = 1.00 → capped at 0.75
        let result = eval_auto(&cond, 0.25, &profile, WeaponType::Sword, Element::Pyro, &[]);
        assert!((result.unwrap() - 0.75).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_stat_scaling_hp() {
        let cond = AutoCondition::StatScaling { stat: BuffableStat::HpPercent, cap: None };
        let profile = StatProfile {
            base_hp: 15000.0,
            hp_percent: 0.466,
            hp_flat: 4780.0,
            ..Default::default()
        };
        // total_hp = 15000 * 1.466 + 4780 = 26770.0; 26770.0 * 0.008 = 214.16
        let result = eval_auto(&cond, 0.008, &profile, WeaponType::Polearm, Element::Pyro, &[]);
        assert!((result.unwrap() - 214.16).abs() < 0.01);
    }

    #[test]
    fn test_eval_auto_team_element_count_pass() {
        let cond = AutoCondition::TeamElementCount { element: Element::Geo, min_count: 3 };
        let team = vec![Element::Geo, Element::Geo, Element::Geo, Element::Pyro];
        let result = eval_auto(&cond, 0.25, &StatProfile::default(), WeaponType::Bow, Element::Geo, &team);
        assert!((result.unwrap() - 0.25).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_team_element_count_fail() {
        let cond = AutoCondition::TeamElementCount { element: Element::Geo, min_count: 3 };
        let team = vec![Element::Geo, Element::Geo, Element::Pyro, Element::Hydro];
        let result = eval_auto(&cond, 0.25, &StatProfile::default(), WeaponType::Bow, Element::Geo, &team);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_team_element_count_empty_skips() {
        let cond = AutoCondition::TeamElementCount { element: Element::Geo, min_count: 3 };
        let result = eval_auto(&cond, 0.25, &StatProfile::default(), WeaponType::Bow, Element::Geo, &[]);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_team_elements_only_pass() {
        let cond = AutoCondition::TeamElementsOnly(&[Element::Hydro, Element::Dendro]);
        let team = vec![Element::Hydro, Element::Dendro, Element::Hydro, Element::Dendro];
        let result = eval_auto(&cond, 0.10, &StatProfile::default(), WeaponType::Sword, Element::Hydro, &team);
        assert!((result.unwrap() - 0.10).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_team_elements_only_fail() {
        let cond = AutoCondition::TeamElementsOnly(&[Element::Hydro, Element::Dendro]);
        let team = vec![Element::Hydro, Element::Dendro, Element::Pyro, Element::Dendro];
        let result = eval_auto(&cond, 0.10, &StatProfile::default(), WeaponType::Sword, Element::Hydro, &team);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_team_elements_only_empty_skips() {
        let cond = AutoCondition::TeamElementsOnly(&[Element::Hydro, Element::Dendro]);
        let result = eval_auto(&cond, 0.10, &StatProfile::default(), WeaponType::Sword, Element::Hydro, &[]);
        assert!(result.is_none());
    }
}
```

- [ ] **Step 2: Run tests — should fail**

Run: `cargo test -p genshin-calc-data conditional_tests`

Expected: FAIL — `eval_auto` not found.

- [ ] **Step 3: Implement eval_auto**

Add as private functions in `team_builder.rs`:

```rust
use crate::buff::{Activation, AutoCondition, ManualCondition, ManualActivation, ConditionalBuff, AvailableConditional};

/// Reads the relevant stat value from a profile for StatScaling.
/// BuffableStat here indicates the "stat family" — see AutoCondition::StatScaling doc.
fn read_stat_for_scaling(stat: &BuffableStat, profile: &StatProfile) -> f64 {
    match stat {
        BuffableStat::HpPercent => profile.base_hp * (1.0 + profile.hp_percent) + profile.hp_flat,
        BuffableStat::AtkPercent => profile.base_atk * (1.0 + profile.atk_percent) + profile.atk_flat,
        BuffableStat::DefPercent => profile.base_def * (1.0 + profile.def_percent) + profile.def_flat,
        BuffableStat::ElementalMastery => profile.elemental_mastery,
        BuffableStat::EnergyRecharge => profile.energy_recharge,
        _ => 0.0,
    }
}

/// Evaluates an Auto condition. Returns Some(value) if condition is met.
fn eval_auto(
    cond: &AutoCondition,
    multiplier: f64,
    profile: &StatProfile,
    weapon_type: WeaponType,
    element: Element,
    team_elements: &[Element],
) -> Option<f64> {
    match cond {
        AutoCondition::WeaponTypeRequired(types) => {
            if types.contains(&weapon_type) { Some(multiplier) } else { None }
        }
        AutoCondition::ElementRequired(elements) => {
            if elements.contains(&element) { Some(multiplier) } else { None }
        }
        AutoCondition::StatScaling { stat, cap } => {
            let stat_val = read_stat_for_scaling(stat, profile);
            let raw = stat_val * multiplier;
            Some(cap.map_or(raw, |c| raw.min(c)))
        }
        AutoCondition::TeamElementCount { element: elem, min_count } => {
            if team_elements.is_empty() { return None; }
            let count = team_elements.iter().filter(|e| *e == elem).count() as u8;
            if count >= *min_count { Some(multiplier) } else { None }
        }
        AutoCondition::TeamElementsOnly(allowed) => {
            if team_elements.is_empty() { return None; }
            if team_elements.iter().all(|e| allowed.contains(e)) {
                Some(multiplier)
            } else {
                None
            }
        }
    }
}
```

- [ ] **Step 4: Run eval_auto tests — should pass**

Run: `cargo test -p genshin-calc-data conditional_tests`

Expected: All eval_auto tests PASS.

- [ ] **Step 5: Write eval_manual tests**

Add to `conditional_tests` module:

```rust
    // --- eval_manual tests ---

    #[test]
    fn test_eval_manual_toggle_active() {
        let result = eval_manual(&ManualCondition::Toggle, "test_buff", 0.15, &[("test_buff", ManualActivation::Active)]);
        assert!((result.unwrap() - 0.15).abs() < EPSILON);
    }

    #[test]
    fn test_eval_manual_toggle_not_present() {
        let result = eval_manual(&ManualCondition::Toggle, "test_buff", 0.15, &[]);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_manual_toggle_stacks_mismatch() {
        let result = eval_manual(&ManualCondition::Toggle, "test_buff", 0.15, &[("test_buff", ManualActivation::Stacks(2))]);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_manual_stacks_normal() {
        let result = eval_manual(&ManualCondition::Stacks(3), "test_buff", 0.075, &[("test_buff", ManualActivation::Stacks(2))]);
        assert!((result.unwrap() - 0.15).abs() < EPSILON); // 0.075 * 2
    }

    #[test]
    fn test_eval_manual_stacks_exceeds_max() {
        let result = eval_manual(&ManualCondition::Stacks(3), "test_buff", 0.075, &[("test_buff", ManualActivation::Stacks(5))]);
        assert!((result.unwrap() - 0.225).abs() < EPSILON); // 0.075 * 3 (capped)
    }

    #[test]
    fn test_eval_manual_stacks_not_present() {
        let result = eval_manual(&ManualCondition::Stacks(3), "test_buff", 0.075, &[]);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_manual_stacks_with_active_treated_as_max() {
        let result = eval_manual(&ManualCondition::Stacks(3), "test_buff", 0.075, &[("test_buff", ManualActivation::Active)]);
        assert!((result.unwrap() - 0.225).abs() < EPSILON); // 0.075 * 3 (max)
    }
```

- [ ] **Step 6: Run tests — should fail**

Run: `cargo test -p genshin-calc-data conditional_tests`

Expected: FAIL — `eval_manual` not found.

- [ ] **Step 7: Implement eval_manual**

```rust
/// Evaluates a Manual condition. Returns Some(value) if user activated it.
fn eval_manual(
    cond: &ManualCondition,
    buff_name: &str,
    value: f64,
    activations: &[(&str, ManualActivation)],
) -> Option<f64> {
    let activation = activations.iter().find(|(name, _)| *name == buff_name);
    match cond {
        ManualCondition::Toggle => match activation {
            Some((_, ManualActivation::Active)) => Some(value),
            _ => None, // Stacks mismatch or not present
        },
        ManualCondition::Stacks(max) => match activation {
            Some((_, ManualActivation::Stacks(n))) => {
                let effective = (*n).min(*max);
                Some(value * f64::from(effective))
            }
            Some((_, ManualActivation::Active)) => {
                Some(value * f64::from(*max)) // Active on Stacks → max stacks
            }
            _ => None,
        },
    }
}
```

- [ ] **Step 8: Run eval_manual tests — should pass**

Run: `cargo test -p genshin-calc-data conditional_tests`

Expected: All eval_manual tests PASS.

- [ ] **Step 9: Write Activation::Both tests**

Add to `conditional_tests` module:

```rust
    // --- Activation::Both tests (unit level) ---

    #[test]
    fn test_both_auto_pass_manual_pass() {
        // Simulates Homa low HP: StatScaling(HP) + Toggle
        let auto = AutoCondition::StatScaling { stat: BuffableStat::HpPercent, cap: None };
        let manual = ManualCondition::Toggle;
        let profile = StatProfile {
            base_hp: 20000.0,
            hp_percent: 0.0,
            ..Default::default()
        };
        // eval_auto: 20000 * 0.01 = 200.0
        let auto_result = eval_auto(&auto, 0.01, &profile, WeaponType::Polearm, Element::Pyro, &[]);
        assert!(auto_result.is_some());
        // eval_manual with auto_value as input
        let result = auto_result.and_then(|auto_value| {
            eval_manual(&manual, "test", auto_value, &[("test", ManualActivation::Active)])
        });
        assert!((result.unwrap() - 200.0).abs() < EPSILON);
    }

    #[test]
    fn test_both_auto_fail_manual_pass() {
        // Auto condition (weapon type) fails → manual never evaluated
        let auto = AutoCondition::WeaponTypeRequired(&[WeaponType::Sword]);
        let manual = ManualCondition::Toggle;
        let auto_result = eval_auto(&auto, 0.35, &StatProfile::default(), WeaponType::Catalyst, Element::Pyro, &[]);
        assert!(auto_result.is_none());
        let result = auto_result.and_then(|v| {
            eval_manual(&manual, "test", v, &[("test", ManualActivation::Active)])
        });
        assert!(result.is_none());
    }

    #[test]
    fn test_both_auto_pass_manual_fail() {
        // Auto passes but manual not activated
        let auto = AutoCondition::WeaponTypeRequired(&[WeaponType::Sword]);
        let manual = ManualCondition::Toggle;
        let auto_result = eval_auto(&auto, 0.35, &StatProfile::default(), WeaponType::Sword, Element::Pyro, &[]);
        assert!(auto_result.is_some());
        let result = auto_result.and_then(|v| {
            eval_manual(&manual, "test", v, &[]) // not activated
        });
        assert!(result.is_none());
    }
```

- [ ] **Step 10: Run all conditional tests — should pass**

Run: `cargo test -p genshin-calc-data conditional_tests`

Expected: All PASS.

- [ ] **Step 11: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "feat(data): add eval_auto and eval_manual for conditional buffs"
```

---

## Task 4: Builder API Methods

**Files:**
- Modify: `crates/data/src/team_builder.rs`

- [ ] **Step 1: Write builder API tests**

Add to `conditional_tests`:

```rust
    // --- Builder API tests ---

    #[test]
    fn test_builder_activate_noop_for_unknown_name() {
        let char = crate::find_character("bennett").unwrap();
        let weapon = crate::find_weapon("aquila_favonia").unwrap();
        let _member = TeamMemberBuilder::new(char, weapon)
            .activate("nonexistent_buff")
            .build()
            .unwrap();
    }

    #[test]
    fn test_builder_activate_with_stacks_noop_for_unknown_name() {
        let char = crate::find_character("bennett").unwrap();
        let weapon = crate::find_weapon("aquila_favonia").unwrap();
        let _member = TeamMemberBuilder::new(char, weapon)
            .activate_with_stacks("nonexistent_buff", 2)
            .build()
            .unwrap();
    }

    #[test]
    fn test_builder_team_elements() {
        let char = crate::find_character("bennett").unwrap();
        let weapon = crate::find_weapon("aquila_favonia").unwrap();
        let _member = TeamMemberBuilder::new(char, weapon)
            .team_elements(vec![Element::Pyro, Element::Hydro, Element::Cryo, Element::Dendro])
            .build()
            .unwrap();
    }

    #[test]
    fn test_builder_available_conditionals_empty() {
        let char = crate::find_character("bennett").unwrap();
        let weapon = crate::find_weapon("aquila_favonia").unwrap();
        let builder = TeamMemberBuilder::new(char, weapon);
        assert!(builder.available_conditionals().is_empty());
    }
```

- [ ] **Step 2: Run tests — should fail**

Run: `cargo test -p genshin-calc-data conditional_tests`

Expected: FAIL — methods not found.

- [ ] **Step 3: Add new fields and methods**

Add fields to struct:
```rust
pub struct TeamMemberBuilder {
    character: &'static CharacterData,
    weapon: &'static WeaponData,
    artifact_set: Option<&'static ArtifactSet>,
    artifact_stats: StatProfile,
    constellation: u8,
    talent_levels: [u8; 3],
    manual_activations: Vec<(&'static str, ManualActivation)>,
    team_elements: Vec<Element>,
}
```

Update `new()`:
```rust
manual_activations: Vec::new(),
team_elements: Vec::new(),
```

Add methods:
```rust
/// Activate a manual conditional buff by name.
pub fn activate(mut self, name: &'static str) -> Self {
    self.manual_activations.push((name, ManualActivation::Active));
    self
}

/// Activate a stackable conditional buff with stack count.
pub fn activate_with_stacks(mut self, name: &'static str, stacks: u8) -> Self {
    self.manual_activations.push((name, ManualActivation::Stacks(stacks)));
    self
}

/// Set team element composition for Auto team-based conditions.
pub fn team_elements(mut self, elements: Vec<Element>) -> Self {
    self.team_elements = elements;
    self
}

/// Returns available conditional buffs with source context.
pub fn available_conditionals(&self) -> Vec<AvailableConditional> {
    let mut result = Vec::new();
    if let Some(passive) = &self.weapon.passive {
        for buff in passive.effect.conditional_buffs {
            result.push(AvailableConditional { source: self.weapon.name, buff });
        }
    }
    if let Some(set) = self.artifact_set {
        for buff in set.two_piece.conditional_buffs {
            result.push(AvailableConditional { source: set.name, buff });
        }
        for buff in set.four_piece.conditional_buffs {
            result.push(AvailableConditional { source: set.name, buff });
        }
    }
    result
}
```

- [ ] **Step 4: Run tests — should pass**

Run: `cargo test -p genshin-calc-data conditional_tests`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "feat(data): add conditional buff activation methods to TeamMemberBuilder"
```

---

## Task 5: Builder Conditional Resolution in build()

**Files:**
- Modify: `crates/data/src/team_builder.rs`

- [ ] **Step 1: Add conditional resolution logic in build()**

After the talent buffs section (around line 193, before `Ok(TeamMember { ... })`), add:

```rust
        // 9. Resolve conditional buffs
        // TODO(P4): use refinement_values[r] when refinement level is available
        let resolve_conditionals = |conditional_buffs: &'static [ConditionalBuff],
                                     source_name: &str,
                                     target: BuffTarget,
                                     buffs: &mut Vec<ResolvedBuff>| {
            for cond_buff in conditional_buffs {
                let resolved_value = match &cond_buff.activation {
                    Activation::Auto(auto) => {
                        eval_auto(auto, cond_buff.value, &profile, char_data.weapon_type, char_data.element, &self.team_elements)
                    }
                    Activation::Manual(manual) => {
                        eval_manual(manual, cond_buff.name, cond_buff.value, &self.manual_activations)
                    }
                    Activation::Both(auto, manual) => {
                        eval_auto(auto, cond_buff.value, &profile, char_data.weapon_type, char_data.element, &self.team_elements)
                            .and_then(|auto_value| {
                                eval_manual(manual, cond_buff.name, auto_value, &self.manual_activations)
                            })
                    }
                };

                if let Some(value) = resolved_value {
                    buffs.push(ResolvedBuff {
                        source: format!("{} ({})", cond_buff.name, source_name),
                        stat: cond_buff.stat,
                        value,
                        target,
                    });
                }
            }
        };

        // Weapon conditional buffs
        if let Some(passive) = &weapon.passive {
            resolve_conditionals(passive.effect.conditional_buffs, weapon.name, BuffTarget::OnlySelf, &mut buffs);
        }

        // Artifact conditional buffs
        if let Some(set) = self.artifact_set {
            resolve_conditionals(set.two_piece.conditional_buffs, &format!("{} 2pc", set.name), BuffTarget::OnlySelf, &mut buffs);
            resolve_conditionals(set.four_piece.conditional_buffs, &format!("{} 4pc", set.name), BuffTarget::OnlySelf, &mut buffs);
        }
```

- [ ] **Step 2: Verify all existing tests still pass**

Run: `cargo test`

Expected: All PASS — all conditional_buffs are `&[]` so no new behavior.

- [ ] **Step 3: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "feat(data): integrate conditional buff resolution into build()"
```

---

## Task 6: Example Conditional Data + Integration Tests

**Files:**
- Modify: `crates/data/src/artifacts.rs`
- Modify: `crates/data/src/team_builder.rs`

- [ ] **Step 1: Write Gladiator 4pc integration tests**

```rust
    #[test]
    fn test_gladiator_4pc_sword_gets_normal_bonus() {
        let char = crate::find_character("bennett").unwrap(); // Sword
        let weapon = crate::find_weapon("aquila_favonia").unwrap();
        let glad = crate::find_artifact_set("gladiators_finale").unwrap();
        let member = TeamMemberBuilder::new(char, weapon)
            .artifact_set(glad)
            .build()
            .unwrap();
        let buff = member.buffs_provided.iter().find(|b| b.stat == BuffableStat::NormalAtkDmgBonus);
        assert!(buff.is_some());
        assert!((buff.unwrap().value - 0.35).abs() < EPSILON);
    }

    #[test]
    fn test_gladiator_4pc_catalyst_no_bonus() {
        let char = crate::find_character("nahida").unwrap(); // Catalyst
        let weapon = crate::find_weapon("a_thousand_floating_dreams").unwrap();
        let glad = crate::find_artifact_set("gladiators_finale").unwrap();
        let member = TeamMemberBuilder::new(char, weapon)
            .artifact_set(glad)
            .build()
            .unwrap();
        let buff = member.buffs_provided.iter().find(|b| b.stat == BuffableStat::NormalAtkDmgBonus);
        assert!(buff.is_none());
    }
```

- [ ] **Step 2: Run — should fail (no conditional data)**

Run: `cargo test -p genshin-calc-data test_gladiator_4pc`

Expected: FAIL.

- [ ] **Step 3: Add Gladiator 4pc conditional data**

In `crates/data/src/artifacts.rs`, add import:
```rust
use crate::buff::{Activation, AutoCondition, ConditionalBuff, ManualCondition};
use genshin_calc_core::WeaponType;
```

Change Gladiator's `four_piece`:
```rust
    four_piece: SetEffect {
        description: "該当キャラクターが片手剣、両手剣、長柄武器キャラの場合、通常攻撃ダメージ+35%",
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "gladiator_normal_bonus",
            description: "Normal Attack DMG +35% for sword/claymore/polearm",
            stat: BuffableStat::NormalAtkDmgBonus,
            value: 0.35,
            refinement_values: None,
            activation: Activation::Auto(AutoCondition::WeaponTypeRequired(&[
                WeaponType::Sword,
                WeaponType::Claymore,
                WeaponType::Polearm,
            ])),
        }],
    },
```

- [ ] **Step 4: Run Gladiator tests — should pass**

Run: `cargo test -p genshin-calc-data test_gladiator_4pc`

Expected: PASS.

- [ ] **Step 5: Write and implement Emblem 4pc**

Test:
```rust
    #[test]
    fn test_emblem_4pc_burst_bonus_from_er() {
        let char = crate::find_character("raiden_shogun").unwrap();
        let weapon = crate::find_weapon("engulfing_lightning").unwrap();
        let emblem = crate::find_artifact_set("emblem_of_severed_fate").unwrap();
        let member = TeamMemberBuilder::new(char, weapon)
            .artifact_stats(StatProfile { energy_recharge: 0.518, ..Default::default() })
            .artifact_set(emblem)
            .build()
            .unwrap();
        let buff = member.buffs_provided.iter()
            .find(|b| b.stat == BuffableStat::BurstDmgBonus && b.source.contains("emblem"));
        assert!(buff.is_some());
        let val = buff.unwrap().value;
        assert!(val > 0.0 && val <= 0.75);
    }
```

Data for Emblem 4pc in `artifacts.rs`:
```rust
    four_piece: SetEffect {
        description: "元素チャージ効率の25%を基準に、元素爆発のダメージがアップ。最大75%まで",
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "emblem_burst_bonus",
            description: "Burst DMG +25% of Energy Recharge, max 75%",
            stat: BuffableStat::BurstDmgBonus,
            value: 0.25,
            refinement_values: None,
            activation: Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::EnergyRecharge,
                cap: Some(0.75),
            }),
        }],
    },
```

- [ ] **Step 6: Write and implement CW 4pc pyro stacks**

Tests:
```rust
    #[test]
    fn test_cw_4pc_pyro_stacks_activated() {
        let char = crate::find_character("hu_tao").unwrap();
        let weapon = crate::find_weapon("staff_of_homa").unwrap();
        let cw = crate::find_artifact_set("crimson_witch").unwrap();
        let member = TeamMemberBuilder::new(char, weapon)
            .artifact_set(cw)
            .activate_with_stacks("cwof_pyro_stacks", 1)
            .build()
            .unwrap();
        let buff = member.buffs_provided.iter().find(|b| b.source.contains("cwof_pyro_stacks"));
        assert!(buff.is_some());
        assert!((buff.unwrap().value - 0.075).abs() < EPSILON); // 1 stack
    }

    #[test]
    fn test_cw_4pc_no_activation_no_buff() {
        let char = crate::find_character("hu_tao").unwrap();
        let weapon = crate::find_weapon("staff_of_homa").unwrap();
        let cw = crate::find_artifact_set("crimson_witch").unwrap();
        let member = TeamMemberBuilder::new(char, weapon)
            .artifact_set(cw)
            .build()
            .unwrap();
        let buff = member.buffs_provided.iter().find(|b| b.source.contains("cwof_pyro_stacks"));
        assert!(buff.is_none());
    }
```

Data for CW 4pc in `artifacts.rs`:
```rust
    four_piece: SetEffect {
        description: "過負荷、燃焼、烈開花反応ダメージ+40%。蒸発、溶解反応倍率+15%。元素スキル使用後2セット効果+50%、最大3スタック",
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "cwof_pyro_stacks",
            description: "Using Skill increases Pyro DMG by 7.5%, max 3 stacks",
            stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
            value: 0.075,
            refinement_values: None,
            activation: Activation::Manual(ManualCondition::Stacks(3)),
        }],
    },
```

- [ ] **Step 7: Write available_conditionals non-empty test**

```rust
    #[test]
    fn test_available_conditionals_with_gladiator() {
        let char = crate::find_character("bennett").unwrap();
        let weapon = crate::find_weapon("aquila_favonia").unwrap();
        let glad = crate::find_artifact_set("gladiators_finale").unwrap();
        let builder = TeamMemberBuilder::new(char, weapon).artifact_set(glad);
        let conditionals = builder.available_conditionals();
        assert_eq!(conditionals.len(), 1);
        assert_eq!(conditionals[0].buff.name, "gladiator_normal_bonus");
    }
```

- [ ] **Step 8: Run all tests**

Run: `cargo test`

Expected: All PASS.

- [ ] **Step 9: Commit**

```bash
git add crates/data/src/artifacts.rs crates/data/src/team_builder.rs
git commit -m "feat(data): add first conditional buffs (Gladiator, Emblem, CW 4pc)"
```

---

## Task 7: Re-exports and Data Integrity Tests

**Files:**
- Modify: `crates/data/src/lib.rs`
- Modify: `crates/data/src/team_builder.rs`

- [ ] **Step 1: Add re-exports in lib.rs**

```rust
pub use buff::{
    Activation, AutoCondition, AvailableConditional, ConditionalBuff, ManualActivation,
    ManualCondition,
};
```

- [ ] **Step 2: Write data integrity tests**

```rust
    #[test]
    fn test_all_conditional_buff_names_unique() {
        let mut names: Vec<&str> = Vec::new();
        // Check artifacts
        for set in crate::artifacts::ALL_ARTIFACT_SETS {
            for buff in set.two_piece.conditional_buffs {
                assert!(!names.contains(&buff.name), "Duplicate: {}", buff.name);
                names.push(buff.name);
            }
            for buff in set.four_piece.conditional_buffs {
                assert!(!names.contains(&buff.name), "Duplicate: {}", buff.name);
                names.push(buff.name);
            }
        }
        // Check weapons
        for weapon in crate::weapons::ALL_WEAPONS {
            if let Some(passive) = &weapon.passive {
                for buff in passive.effect.conditional_buffs {
                    assert!(!names.contains(&buff.name), "Duplicate: {}", buff.name);
                    names.push(buff.name);
                }
            }
        }
    }

    #[test]
    fn test_all_stacks_max_positive() {
        let check_buffs = |buffs: &[ConditionalBuff]| {
            for buff in buffs {
                match &buff.activation {
                    Activation::Manual(ManualCondition::Stacks(max)) => {
                        assert!(*max > 0, "Stacks max must be > 0 for {}", buff.name);
                    }
                    Activation::Both(_, ManualCondition::Stacks(max)) => {
                        assert!(*max > 0, "Stacks max must be > 0 for {}", buff.name);
                    }
                    _ => {}
                }
            }
        };
        for set in crate::artifacts::ALL_ARTIFACT_SETS {
            check_buffs(set.two_piece.conditional_buffs);
            check_buffs(set.four_piece.conditional_buffs);
        }
        for weapon in crate::weapons::ALL_WEAPONS {
            if let Some(passive) = &weapon.passive {
                check_buffs(passive.effect.conditional_buffs);
            }
        }
    }

    #[test]
    fn test_existing_bennett_burst_unchanged() {
        let bennett = crate::find_character("bennett").unwrap();
        let weapon = crate::find_weapon("aquila_favonia").unwrap();
        let member = TeamMemberBuilder::new(bennett, weapon)
            .talent_levels([1, 1, 13])
            .build()
            .unwrap();
        let burst_buff = member.buffs_provided.iter()
            .find(|b| b.source.contains("Fantastic Voyage")).unwrap();
        let expected = member.stats.base_atk * 1.19;
        assert!((burst_buff.value - expected).abs() < 1e-4);
    }
```

- [ ] **Step 3: Run all tests**

Run: `cargo test`

Expected: All PASS.

- [ ] **Step 4: Run clippy and fmt**

Run: `cargo clippy -- -D warnings && cargo fmt --check`

Expected: No issues.

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/lib.rs crates/data/src/team_builder.rs
git commit -m "feat(data): add re-exports and data integrity tests for conditional buffs"
```

---

## Task 8: Final Verification

- [ ] **Step 1: Run full test suite**

Run: `cargo test 2>&1 | tail -5`

Expected: All tests pass (original + ~30 new).

- [ ] **Step 2: Run character verification tests**

Run: `cargo test --test character_verification`

Expected: All 153 cases PASS (zero regression).

- [ ] **Step 3: Run clippy and fmt**

Run: `cargo clippy -- -D warnings && cargo fmt --check`

Expected: Clean.
