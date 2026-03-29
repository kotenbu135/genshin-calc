# P3 Complex Weapons ConditionalBuff Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add ConditionalBuff support for 4 complex weapons (Mistsplitter, Lithic Blade/Spear, Scarlet Sands secondary, Engulfing Lightning secondary) with supporting type extensions and bug fixes.

**Architecture:** Extend `AutoCondition` with `TeamRegionCount` for region-based team evaluation. Fix `eval_manual` to use refinement-resolved values and `Both` to pass auto-computed values to manual evaluation. Add weapon data for 5 new ConditionalBuffs across 4 weapons.

**Tech Stack:** Rust, genshin-calc-core, genshin-calc-data, cargo test

**Spec:** `docs/superpowers/specs/2026-03-29-complex-weapons-design.md`

---

### Task 1: Add `TeamRegionCount` to `AutoCondition`

**Files:**
- Modify: `crates/data/src/buff.rs:5` (import), `crates/data/src/buff.rs:30-55` (enum)

- [ ] **Step 1: Add `Region` import to buff.rs**

At line 5, add `Region` import:

```rust
use genshin_calc_core::{Element, WeaponType};
use crate::types::Region;
```

- [ ] **Step 2: Add `TeamRegionCount` variant to `AutoCondition`**

After `TeamDiffElementCount` (line 54), add:

```rust
    /// Buff scales with count of team members from a specific region.
    /// Returns effective_value * count. 0 members → None.
    TeamRegionCount { region: Region },
```

- [ ] **Step 3: Verify it compiles**

Run: `cargo build -p genshin-calc-data 2>&1 | head -20`
Expected: Compile errors in `team_builder.rs` (non-exhaustive match) — this is expected, fixed in Task 2.

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/buff.rs
git commit -m "feat: add AutoCondition::TeamRegionCount variant"
```

---

### Task 2: Add `team_regions` to `TeamMemberBuilder` and `eval_auto`

**Files:**
- Modify: `crates/data/src/team_builder.rs:1-25` (imports, struct fields)
- Modify: `crates/data/src/team_builder.rs:27-93` (builder impl, new setter)
- Modify: `crates/data/src/team_builder.rs:398-476` (eval_auto signature + new match arm)
- Modify: `crates/data/src/team_builder.rs:270-316` (resolve_conditionals closure — 3 call sites)

- [ ] **Step 1: Write failing test for `TeamRegionCount` eval_auto**

In `crates/data/src/team_builder.rs` tests section (after line ~1308, after `test_eval_auto_team_elements_only_empty_skips`), add:

```rust
    // --- eval_auto TeamRegionCount tests ---

    #[test]
    fn test_eval_auto_team_region_count_liyue_2() {
        let cond = AutoCondition::TeamRegionCount {
            region: crate::types::Region::Liyue,
        };
        let regions = vec![
            crate::types::Region::Liyue,
            crate::types::Region::Mondstadt,
            crate::types::Region::Liyue,
        ];
        let result = eval_auto(
            &cond,
            0.07,
            &StatProfile::default(),
            WeaponType::Claymore,
            Element::Pyro,
            &[],
            &regions,
            1,
        );
        assert!((result.unwrap() - 0.14).abs() < EPSILON); // 0.07 * 2
    }

    #[test]
    fn test_eval_auto_team_region_count_zero() {
        let cond = AutoCondition::TeamRegionCount {
            region: crate::types::Region::Liyue,
        };
        let regions = vec![crate::types::Region::Mondstadt];
        let result = eval_auto(
            &cond,
            0.07,
            &StatProfile::default(),
            WeaponType::Claymore,
            Element::Pyro,
            &[],
            &regions,
            1,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_team_region_count_empty() {
        let cond = AutoCondition::TeamRegionCount {
            region: crate::types::Region::Liyue,
        };
        let result = eval_auto(
            &cond,
            0.07,
            &StatProfile::default(),
            WeaponType::Claymore,
            Element::Pyro,
            &[],
            &[],
            1,
        );
        assert!(result.is_none());
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data team_region_count 2>&1 | tail -5`
Expected: FAIL (compile error — `eval_auto` signature mismatch)

- [ ] **Step 3: Add `team_regions` field and setter to `TeamMemberBuilder`**

In struct at line 23, add after `team_elements`:

```rust
    team_regions: Vec<crate::types::Region>,
```

In `new()` at line 40, add after `team_elements: Vec::new()`:

```rust
            team_regions: Vec::new(),
```

After `team_elements` setter (line 93), add:

```rust
    /// Set team region composition for Auto region-based conditions.
    pub fn team_regions(mut self, regions: Vec<crate::types::Region>) -> Self {
        self.team_regions = regions;
        self
    }
```

- [ ] **Step 4: Update `eval_auto` signature and add `TeamRegionCount` arm**

At line 398, update signature to add `team_regions: &[crate::types::Region]` after `team_elements`:

```rust
fn eval_auto(
    cond: &AutoCondition,
    multiplier: f64,
    profile: &StatProfile,
    weapon_type: WeaponType,
    element: Element,
    team_elements: &[Element],
    team_regions: &[crate::types::Region],
    refinement: u8,
) -> Option<f64> {
```

Add new match arm after `TeamDiffElementCount` (before closing `}`):

```rust
        AutoCondition::TeamRegionCount { region } => {
            if team_regions.is_empty() {
                return None;
            }
            let count = team_regions.iter().filter(|r| **r == region).count() as f64;
            if count > 0.0 {
                Some(multiplier * count)
            } else {
                None
            }
        }
```

- [ ] **Step 5: Update `resolve_conditionals` closure — both `eval_auto` call sites (Auto and Both branches)**

In the closure at lines 279-286 and 292-299, add `&self.team_regions` parameter after `&self.team_elements`:

Call site 1 (line 279-286, `Activation::Auto`):
```rust
                        Activation::Auto(auto) => eval_auto(
                            auto,
                            effective_value,
                            &profile,
                            char_data.weapon_type,
                            char_data.element,
                            &self.team_elements,
                            &self.team_regions,
                            refinement,
                        ),
```

Call site 2 (line 292-299, `Activation::Both`):
```rust
                        Activation::Both(auto, manual) => {
                            let auto_result = eval_auto(
                                auto,
                                effective_value,
                                &profile,
                                char_data.weapon_type,
                                char_data.element,
                                &self.team_elements,
                                &self.team_regions,
                                refinement,
                            );
```

- [ ] **Step 6: Fix ALL existing `eval_auto` calls in tests to add `&[]` for team_regions**

Every existing `eval_auto(...)` test call needs `&[]` added after the `team_elements` argument (before `refinement`). Search for all `eval_auto(` in the test module and add the parameter. There are 21 existing test calls.

Pattern: change `&[], 1)` → `&[], &[], 1)` (where the first `&[]` is team_elements and the second new `&[]` is team_regions).

- [ ] **Step 7: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data -- team_region_count 2>&1 | tail -10`
Expected: 3 tests PASS

Run: `cargo test -p genshin-calc-data 2>&1 | tail -5`
Expected: ALL tests pass

- [ ] **Step 8: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "feat: add team_regions to TeamMemberBuilder and eval_auto TeamRegionCount"
```

---

### Task 3: Fix `eval_manual` to use `base_value` parameter

**Files:**
- Modify: `crates/data/src/team_builder.rs:479-511` (eval_manual function)
- Modify: `crates/data/src/team_builder.rs:288-289` (Manual call site)
- Modify: `crates/data/src/team_builder.rs:1311-1393` (eval_manual tests)

- [ ] **Step 1: Write failing test for Toggle with base_value**

Add test after `test_eval_manual_stacks_with_active_treated_as_max` (line ~1393):

```rust
    #[test]
    fn test_eval_manual_toggle_uses_base_value() {
        let buff = make_test_buff("test_buff", 0.15, ManualCondition::Toggle);
        // base_value differs from buff.value — should return base_value
        let result = eval_manual(
            &ManualCondition::Toggle,
            &buff,
            &[("test_buff", ManualActivation::Active)],
            0.30, // base_value (e.g. R3 effective_value)
        );
        assert!((result.unwrap() - 0.30).abs() < EPSILON);
    }

    #[test]
    fn test_eval_manual_stacks_uses_base_value() {
        let buff = make_test_buff("test_buff", 0.05, ManualCondition::Stacks(3));
        let result = eval_manual(
            &ManualCondition::Stacks(3),
            &buff,
            &[("test_buff", ManualActivation::Stacks(2))],
            0.09, // base_value (e.g. R3 effective_value)
        );
        assert!((result.unwrap() - 0.18).abs() < EPSILON); // 0.09 * 2
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data eval_manual_toggle_uses 2>&1 | tail -5`
Expected: FAIL (compile error — wrong number of arguments)

- [ ] **Step 3: Add `base_value` parameter to `eval_manual`**

Change function signature at line 479:

```rust
fn eval_manual(
    cond: &ManualCondition,
    buff: &ConditionalBuff,
    activations: &[(&str, ManualActivation)],
    base_value: f64,
) -> Option<f64> {
```

Replace `buff.value` with `base_value` in Toggle (line 487) and linear Stacks (lines 498, 505):

```rust
    match cond {
        ManualCondition::Toggle => match activation {
            Some((_, ManualActivation::Active)) => Some(base_value),
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
                    None => Some(base_value * f64::from(effective)),
                }
            }
            Some((_, ManualActivation::Active)) => {
                let effective = *max;
                match buff.stack_values {
                    Some(values) => Some(values[(effective as usize).min(values.len()) - 1]),
                    None => Some(base_value * f64::from(effective)),
                }
            }
            _ => None,
        },
    }
```

- [ ] **Step 4: Update the `Activation::Manual` call site in resolve_conditionals (line 289)**

```rust
                        Activation::Manual(manual) => {
                            eval_manual(manual, cond_buff, &self.manual_activations, effective_value)
                        }
```

- [ ] **Step 5: Fix ALL existing `eval_manual` test calls to add `buff.value` as base_value**

For each existing eval_manual test, add `buff.value` as the 4th argument (preserves existing behavior since buff.value was what was used before):

- `test_eval_manual_toggle_active`: add `buff.value` → `eval_manual(&ManualCondition::Toggle, &buff, &[...], buff.value)`
- `test_eval_manual_toggle_not_present`: add `buff.value`
- `test_eval_manual_toggle_stacks_mismatch`: add `buff.value`
- `test_eval_manual_stacks_normal`: add `buff.value`
- `test_eval_manual_stacks_exceeds_max`: add `buff.value`
- `test_eval_manual_stacks_not_present`: add `buff.value`
- `test_eval_manual_stacks_with_active_treated_as_max`: add `buff.value`

- [ ] **Step 6: Run tests**

Run: `cargo test -p genshin-calc-data eval_manual 2>&1 | tail -10`
Expected: ALL eval_manual tests pass (including 2 new ones)

- [ ] **Step 7: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "fix: eval_manual uses base_value instead of buff.value for refinement support"
```

---

### Task 4: Fix `Both` semantics to pass `auto_val` to `eval_manual`

**Files:**
- Modify: `crates/data/src/team_builder.rs:291-304` (Both branch in resolve_conditionals)
- Modify: `crates/data/src/team_builder.rs:1395-1467` (Both tests)

- [ ] **Step 1: Write failing test for Both with StatScaling + Toggle returning auto_val**

Replace `test_both_auto_pass_manual_pass` (lines 1397-1426) with:

```rust
    #[test]
    fn test_both_auto_pass_manual_pass() {
        // Both should pass auto_val to eval_manual as base_value
        let auto = AutoCondition::StatScaling {
            stat: BuffableStat::HpPercent,
            offset: None,
            cap: None,
        };
        let manual = ManualCondition::Toggle;
        let profile = StatProfile {
            base_hp: 20000.0,
            hp_percent: 0.0,
            ..Default::default()
        };
        // eval_auto: 20000 * 0.01 = 200.0
        let auto_result = eval_auto(
            &auto,
            0.01,
            &profile,
            WeaponType::Polearm,
            Element::Pyro,
            &[],
            &[],
            1,
        );
        let auto_val = auto_result.unwrap();
        assert!((auto_val - 200.0).abs() < EPSILON);
        // Both passes auto_val (200.0) as base_value to eval_manual
        let buff = make_test_buff("test", 0.01, ManualCondition::Toggle);
        let result = auto_result
            .and_then(|av| eval_manual(&manual, &buff, &[("test", ManualActivation::Active)], av));
        assert!((result.unwrap() - 200.0).abs() < EPSILON);
    }
```

- [ ] **Step 2: Add test for Both + StatScaling + Stacks**

After the updated `test_both_auto_pass_manual_pass`, add:

```rust
    #[test]
    fn test_both_stat_scaling_with_stacks() {
        let auto = AutoCondition::StatScaling {
            stat: BuffableStat::ElementalMastery,
            offset: None,
            cap: None,
        };
        let manual = ManualCondition::Stacks(2);
        let profile = StatProfile {
            elemental_mastery: 200.0,
            ..Default::default()
        };
        // eval_auto: 200 * 0.28 = 56.0
        let auto_result = eval_auto(
            &auto,
            0.28,
            &profile,
            WeaponType::Polearm,
            Element::Pyro,
            &[],
            &[],
            1,
        );
        let auto_val = auto_result.unwrap();
        assert!((auto_val - 56.0).abs() < EPSILON);
        // Both passes auto_val (56.0) to eval_manual, stacks=2 → 56.0 * 2 = 112.0
        let buff = make_test_buff("test", 0.28, ManualCondition::Stacks(2));
        let result = auto_result
            .and_then(|av| eval_manual(&manual, &buff, &[("test", ManualActivation::Stacks(2))], av));
        assert!((result.unwrap() - 112.0).abs() < EPSILON);
    }
```

- [ ] **Step 3: Run tests to verify the first test passes (already manually calling with av)**

Run: `cargo test -p genshin-calc-data test_both 2>&1 | tail -10`
Expected: New tests PASS (they manually call eval_manual with av)

- [ ] **Step 4: Update the Both branch in resolve_conditionals (lines 291-304)**

**Note:** After Task 3, the Both branch already has `eval_manual(..., effective_value)`. Change `|_|` to `|av|` and `effective_value` to `av`:

Change from (post-Task 3 state):
```rust
                        Activation::Both(auto, manual) => {
                            ...
                            auto_result.and_then(|_| {
                                eval_manual(
                                    manual,
                                    cond_buff,
                                    &self.manual_activations,
                                    effective_value,
                                )
                            })
                        }
```

To:
```rust
                        Activation::Both(auto, manual) => {
                            let auto_result = eval_auto(
                                auto,
                                effective_value,
                                &profile,
                                char_data.weapon_type,
                                char_data.element,
                                &self.team_elements,
                                &self.team_regions,
                                refinement,
                            );
                            auto_result.and_then(|av| {
                                eval_manual(
                                    manual,
                                    cond_buff,
                                    &self.manual_activations,
                                    av,
                                )
                            })
                        }
```

- [ ] **Step 5: Fix remaining Both tests' eval_auto calls (add `&[]` for team_regions if not done in Task 2)**

Update `test_both_auto_fail_manual_pass` and `test_both_auto_pass_manual_fail`:
- Add `&[]` for team_regions in eval_auto calls
- Add `av` (or `buff.value` for non-auto-scaled cases) as base_value in eval_manual calls

For `test_both_auto_fail_manual_pass` (WeaponTypeRequired, auto fails → None):
```rust
        let auto_result = eval_auto(
            &auto, 0.35, &StatProfile::default(),
            WeaponType::Catalyst, Element::Pyro, &[], &[], 1,
        );
        assert!(auto_result.is_none());
        let buff = make_test_buff("test", 0.35, ManualCondition::Toggle);
        let result = auto_result
            .and_then(|av| eval_manual(&manual, &buff, &[("test", ManualActivation::Active)], av));
        assert!(result.is_none());
```

For `test_both_auto_pass_manual_fail` (WeaponTypeRequired passes, manual not activated):
```rust
        let auto_result = eval_auto(
            &auto, 0.35, &StatProfile::default(),
            WeaponType::Sword, Element::Pyro, &[], &[], 1,
        );
        assert!(auto_result.is_some());
        let buff = make_test_buff("test", 0.35, ManualCondition::Toggle);
        let result = auto_result.and_then(|av| {
            eval_manual(&manual, &buff, &[], av)
        });
        assert!(result.is_none());
```

- [ ] **Step 6: Run all tests**

Run: `cargo test -p genshin-calc-data 2>&1 | tail -5`
Expected: ALL tests pass

- [ ] **Step 7: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "fix: Both activation passes auto_val to eval_manual as base_value"
```

---

### Task 5: Add Mistsplitter Reforged ConditionalBuff

**Files:**
- Modify: `crates/data/src/weapons/sword.rs:302` (conditional_buffs)

- [ ] **Step 1: Write failing test**

In `crates/data/src/weapons/sword.rs` test module, add:

```rust
    #[test]
    fn mistsplitter_has_emblem_stacks() {
        let passive = MISTSPLITTER_REFORGED.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "mistsplitter_emblem");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        // Non-linear stack values
        let sv = buff.stack_values.unwrap();
        assert_eq!(sv.len(), 3);
        assert!((sv[0] - 0.08).abs() < 1e-6);
        assert!((sv[1] - 0.16).abs() < 1e-6);
        assert!((sv[2] - 0.28).abs() < 1e-6);
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data mistsplitter_has_emblem 2>&1 | tail -5`
Expected: FAIL (`cond_buffs.len()` is 0)

- [ ] **Step 3: Add ConditionalBuff to Mistsplitter**

In `crates/data/src/weapons/sword.rs`, replace `conditional_buffs: &[],` at line 302 with:

```rust
            conditional_buffs: &[ConditionalBuff {
                name: "mistsplitter_emblem",
                description: "霧切の巴紋: 1/2/3スタックで元素DMG+8%/16%/28% (R1)",
                stat: BuffableStat::DmgBonus,
                value: 0.08,
                refinement_values: None,
                stack_values: Some(&[0.08, 0.16, 0.28]),
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data mistsplitter_has_emblem 2>&1 | tail -5`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/sword.rs
git commit -m "feat: add Mistsplitter Reforged ConditionalBuff (non-linear Stacks(3))"
```

---

### Task 6: Add Lithic Blade and Lithic Spear ConditionalBuffs

**Files:**
- Modify: `crates/data/src/weapons/claymore.rs:495-500` (Lithic Blade)
- Modify: `crates/data/src/weapons/polearm.rs:544-549` (Lithic Spear)

- [ ] **Step 1: Write failing tests**

In `crates/data/src/weapons/claymore.rs` test module, add:

```rust
    #[test]
    fn lithic_blade_has_region_conditionals() {
        let passive = LITHIC_BLADE.passive.unwrap();
        assert_eq!(passive.name, "千岩の刃");
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        let atk = &cond_buffs[0];
        assert_eq!(atk.name, "lithic_blade_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert!((atk.value - 0.07).abs() < 1e-6);
        assert!(atk.refinement_values.is_some());
        assert!(matches!(
            atk.activation,
            Activation::Auto(AutoCondition::TeamRegionCount { region: crate::types::Region::Liyue })
        ));

        let cr = &cond_buffs[1];
        assert_eq!(cr.name, "lithic_blade_crit");
        assert_eq!(cr.stat, BuffableStat::CritRate);
        assert!((cr.value - 0.03).abs() < 1e-6);
        assert!(cr.refinement_values.is_some());
    }
```

In `crates/data/src/weapons/polearm.rs` test module, add:

```rust
    #[test]
    fn lithic_spear_has_region_conditionals() {
        let passive = LITHIC_SPEAR.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        let atk = &cond_buffs[0];
        assert_eq!(atk.name, "lithic_spear_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert!((atk.value - 0.07).abs() < 1e-6);
        assert!(matches!(
            atk.activation,
            Activation::Auto(AutoCondition::TeamRegionCount { region: crate::types::Region::Liyue })
        ));

        let cr = &cond_buffs[1];
        assert_eq!(cr.name, "lithic_spear_crit");
        assert_eq!(cr.stat, BuffableStat::CritRate);
        assert!((cr.value - 0.03).abs() < 1e-6);
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data lithic 2>&1 | tail -5`
Expected: FAIL

- [ ] **Step 3: Implement Lithic Blade ConditionalBuffs**

In `crates/data/src/weapons/claymore.rs`, update LITHIC_BLADE:

Change passive name from `"千岩の槍"` to `"千岩の刃"` (line 495).

Replace `conditional_buffs: &[],` (line 499) with:

```rust
            conditional_buffs: &[
                ConditionalBuff {
                    name: "lithic_blade_atk",
                    description: "璃月キャラ1人につきATK+7-11%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.07,
                    refinement_values: Some([0.07, 0.08, 0.09, 0.10, 0.11]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamRegionCount {
                        region: Region::Liyue,
                    }),
                },
                ConditionalBuff {
                    name: "lithic_blade_crit",
                    description: "璃月キャラ1人につきCR+3-7%",
                    stat: BuffableStat::CritRate,
                    value: 0.03,
                    refinement_values: Some([0.03, 0.04, 0.05, 0.06, 0.07]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamRegionCount {
                        region: Region::Liyue,
                    }),
                },
            ],
```

Add import at top of `claymore.rs` if not present: `use crate::types::Region;`

- [ ] **Step 4: Implement Lithic Spear ConditionalBuffs**

In `crates/data/src/weapons/polearm.rs`, update LITHIC_SPEAR similarly:

Replace `conditional_buffs: &[],` (line 548) with:

```rust
            conditional_buffs: &[
                ConditionalBuff {
                    name: "lithic_spear_atk",
                    description: "璃月キャラ1人につきATK+7-11%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.07,
                    refinement_values: Some([0.07, 0.08, 0.09, 0.10, 0.11]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamRegionCount {
                        region: Region::Liyue,
                    }),
                },
                ConditionalBuff {
                    name: "lithic_spear_crit",
                    description: "璃月キャラ1人につきCR+3-7%",
                    stat: BuffableStat::CritRate,
                    value: 0.03,
                    refinement_values: Some([0.03, 0.04, 0.05, 0.06, 0.07]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamRegionCount {
                        region: Region::Liyue,
                    }),
                },
            ],
```

Add import: `use crate::types::Region;`

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data lithic 2>&1 | tail -5`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/weapons/claymore.rs crates/data/src/weapons/polearm.rs
git commit -m "feat: add Lithic Blade/Spear ConditionalBuff (TeamRegionCount Liyue)"
```

---

### Task 7: Add Scarlet Sands secondary and Engulfing Lightning secondary

**Files:**
- Modify: `crates/data/src/weapons/polearm.rs:258-271` (Scarlet Sands)
- Modify: `crates/data/src/weapons/polearm.rs:78-91` (Engulfing Lightning)

- [ ] **Step 1: Write failing tests**

In `crates/data/src/weapons/polearm.rs` test module:

**Replace** the existing `staff_of_scarlet_sands_has_em_atk_conditional` test (line 923) with this updated version:

```rust
    #[test]
    fn staff_of_scarlet_sands_has_em_atk_conditional() {
        let passive = STAFF_OF_THE_SCARLET_SANDS.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2); // was 1, now 2

        // Primary: EM → ATK flat (Auto StatScaling)
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "scarlet_sands_em_atk");
        assert_eq!(buff.stat, BuffableStat::AtkFlat);
        assert!((buff.value - 0.52).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::ElementalMastery,
                offset: None,
                cap: None,
            })
        ));

        // Secondary: EM → ATK flat × stacks (Both StatScaling + Stacks)
        let buff2 = &cond_buffs[1];
        assert_eq!(buff2.name, "scarlet_sands_skill_stacks");
        assert_eq!(buff2.stat, BuffableStat::AtkFlat);
        assert!((buff2.value - 0.28).abs() < 1e-6);
        assert!(buff2.refinement_values.is_some());
        assert!(matches!(
            buff2.activation,
            Activation::Both(
                AutoCondition::StatScaling {
                    stat: BuffableStat::ElementalMastery,
                    offset: None,
                    cap: None,
                },
                ManualCondition::Stacks(2),
            )
        ));
    }
```

Add new test for Engulfing Lightning:

```rust
    #[test]
    fn engulfing_lightning_has_er_atk_and_burst_er() {
        let passive = ENGULFING_LIGHTNING.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2); // was 1, now 2

        // Primary: ER excess → ATK% (unchanged)
        assert_eq!(cond_buffs[0].name, "engulfing_er_atk");

        // Secondary: burst ER toggle
        let buff2 = &cond_buffs[1];
        assert_eq!(buff2.name, "engulfing_burst_er");
        assert_eq!(buff2.stat, BuffableStat::EnergyRecharge);
        assert!((buff2.value - 0.30).abs() < 1e-6);
        assert!(buff2.refinement_values.is_some());
        let rv = buff2.refinement_values.unwrap();
        assert!((rv[0] - 0.30).abs() < 1e-6);
        assert!((rv[4] - 0.50).abs() < 1e-6);
        assert!(matches!(
            buff2.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data scarlet_sands 2>&1 | tail -5`
Expected: FAIL (cond_buffs.len() is 1, expected 2)

- [ ] **Step 3: Add Scarlet Sands secondary ConditionalBuff**

In `crates/data/src/weapons/polearm.rs`, change the `conditional_buffs` array for STAFF_OF_THE_SCARLET_SANDS (line 258) from single-element to two-element:

```rust
            conditional_buffs: &[
                ConditionalBuff {
                    name: "scarlet_sands_em_atk",
                    description: "EM×52-104%分をATKフラットに加算",
                    stat: BuffableStat::AtkFlat,
                    value: 0.52,
                    refinement_values: Some([0.52, 0.65, 0.78, 0.91, 1.04]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::ElementalMastery,
                        offset: None,
                        cap: None,
                    }),
                },
                ConditionalBuff {
                    name: "scarlet_sands_skill_stacks",
                    description: "スキル命中後10秒間、EM×28-56%分ATKアップ。最大2スタック",
                    stat: BuffableStat::AtkFlat,
                    value: 0.28,
                    refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Both(
                        AutoCondition::StatScaling {
                            stat: BuffableStat::ElementalMastery,
                            offset: None,
                            cap: None,
                        },
                        ManualCondition::Stacks(2),
                    ),
                },
            ],
```

- [ ] **Step 4: Add Engulfing Lightning secondary ConditionalBuff**

In `crates/data/src/weapons/polearm.rs`, change ENGULFING_LIGHTNING's `conditional_buffs` (line 78) from single to two:

```rust
            conditional_buffs: &[
                ConditionalBuff {
                    name: "engulfing_er_atk",
                    description: "ER超過分の28-56%をATK%に変換 (cap: 80-120%)",
                    stat: BuffableStat::AtkPercent,
                    value: 0.28,
                    refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::EnergyRecharge,
                        offset: Some(1.0),
                        cap: Some([0.80, 0.90, 1.00, 1.10, 1.20]),
                    }),
                },
                ConditionalBuff {
                    name: "engulfing_burst_er",
                    description: "元素爆発後12秒間ER+30-50%",
                    stat: BuffableStat::EnergyRecharge,
                    value: 0.30,
                    refinement_values: Some([0.30, 0.35, 0.40, 0.45, 0.50]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
```

- [ ] **Step 5: Update existing `engulfing_lightning_has_er_atk_conditional` test**

Change `assert_eq!(cond_buffs.len(), 1)` to `assert_eq!(cond_buffs.len(), 2)` in the existing test (line 907).

- [ ] **Step 6: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data scarlet_sands engulfing 2>&1 | tail -10`
Expected: ALL PASS

- [ ] **Step 7: Commit**

```bash
git add crates/data/src/weapons/polearm.rs
git commit -m "feat: add Scarlet Sands secondary (Both EM×Stacks) and Engulfing burst ER toggle"
```

---

### Task 8: Integration tests

**Files:**
- Modify: `crates/data/src/team_builder.rs` (test module, add integration tests)

- [ ] **Step 1: Write Lithic Blade integration test**

```rust
    #[test]
    fn test_lithic_blade_with_liyue_team() {
        use crate::types::Region;
        let xiangling = find_character("xiangling").unwrap();
        let lithic = find_weapon("lithic_blade").unwrap();

        // Use a claymore Liyue character if available, else test with xiangling for logic
        // Key: verify TeamRegionCount produces correct buff values
        let beidou = find_character("beidou").unwrap();
        let member = TeamMemberBuilder::new(beidou, lithic)
            .team_regions(vec![Region::Liyue, Region::Mondstadt, Region::Liyue])
            .build()
            .unwrap();

        let atk_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("lithic_blade_atk"))
            .expect("Should have lithic_blade_atk buff");
        // R1: 0.07 * 2 Liyue members = 0.14
        assert!(
            (atk_buff.value - 0.14).abs() < EPSILON,
            "ATK% should be 0.14, got {}",
            atk_buff.value
        );

        let crit_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("lithic_blade_crit"))
            .expect("Should have lithic_blade_crit buff");
        // R1: 0.03 * 2 = 0.06
        assert!(
            (crit_buff.value - 0.06).abs() < EPSILON,
            "CR should be 0.06, got {}",
            crit_buff.value
        );
    }
```

- [ ] **Step 2: Write Scarlet Sands integration test**

```rust
    #[test]
    fn test_scarlet_sands_with_em_and_stacks() {
        let cyno = find_character("cyno").unwrap();
        let weapon = find_weapon("staff_of_the_scarlet_sands").unwrap();
        let em_stats = StatProfile {
            elemental_mastery: 100.0,
            ..Default::default()
        };
        let member = TeamMemberBuilder::new(cyno, weapon)
            .artifact_stats(em_stats)
            .activate_with_stacks("scarlet_sands_skill_stacks", 2)
            .build()
            .unwrap();

        // Primary: total_EM * 0.52 (Auto)
        let primary = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("scarlet_sands_em_atk"))
            .expect("Should have primary EM ATK buff");
        assert!(primary.value > 0.0);

        // Secondary: total_EM * 0.28 * 2 (Both)
        let secondary = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("scarlet_sands_skill_stacks"))
            .expect("Should have skill stacks buff");
        assert!(secondary.value > 0.0);
        // secondary should be approximately 2x the single-stack auto value
    }
```

- [ ] **Step 3: Write Engulfing Lightning integration test**

```rust
    #[test]
    fn test_engulfing_burst_er_toggle() {
        let raiden = find_character("raiden_shogun").unwrap();
        let weapon = find_weapon("engulfing_lightning").unwrap();
        let member = TeamMemberBuilder::new(raiden, weapon)
            .activate("engulfing_burst_er")
            .build()
            .unwrap();

        let er_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("engulfing_burst_er"))
            .expect("Should have burst ER buff");
        assert!(
            (er_buff.value - 0.30).abs() < EPSILON,
            "ER should be 0.30, got {}",
            er_buff.value
        );
    }
```

- [ ] **Step 4: Run all tests**

Run: `cargo test -p genshin-calc-data 2>&1 | tail -5`
Expected: ALL PASS

Run: `cargo test 2>&1 | tail -5`
Expected: ALL workspace tests PASS

- [ ] **Step 5: Run clippy**

Run: `cargo clippy -- -D warnings 2>&1 | tail -10`
Expected: No warnings

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "test: integration tests for Lithic, Scarlet Sands, Engulfing weapons"
```

---

### Task 9: Update documentation

**Files:**
- Modify: `CLAUDE.md`
- Modify: `docs/data-expansion-todo.md`

- [ ] **Step 1: Update CLAUDE.md**

In CLAUDE.md, find the line about Both semantics:
```
- `team_builder.rs`注意: `Both(auto, manual)`はauto値を破棄（ゲートのみ）。StatScaling結果を使いたい場合は`Auto(StatScaling)`を使用
```

Replace with:
```
- `team_builder.rs`注意: `Both(auto, manual)`はauto_valをeval_manualのbase_valueとして渡す。Toggle→auto_val返却、Stacks→auto_val×n
```

- [ ] **Step 2: Update data-expansion-todo.md**

In the "未対応" section, mark all 4 as done:

```markdown
- [x] 霧切の廻光 — 非線形Stacks(3)、stack_values R1絶対値
- [x] 千岩古剣/千岩長槍 — TeamRegionCount(Liyue)、Auto完全自動評価
- [x] 赤砂の杖 — 二次効果 Both(StatScaling(EM), Stacks(2))
- [x] 草薙の稲光 — 二次効果 Manual(Toggle) ER+30-50%
```

Update the summary stats (74 existing + 3 new weapons = 77; Scarlet Sands/Engulfing already counted):
```
| 武器パッシブ (ConditionalBuff) | 220 | 77 | 143 | 35% |
```

Update "残タスク" section to remove the 4 weapons entry.

Update "推奨実装順序" P3 line:
```
5. **P3 武器パッシブ** — 5星Batch 1-4 + 4星28本 + 複雑武器4本完了 (77/220)
```

- [ ] **Step 3: Commit**

```bash
git add CLAUDE.md docs/data-expansion-todo.md
git commit -m "docs: update CLAUDE.md Both semantics, mark P3 complex weapons complete"
```

- [ ] **Step 4: Delete spec and plan files**

```bash
rm docs/superpowers/specs/2026-03-29-complex-weapons-design.md
rm docs/superpowers/plans/2026-03-29-complex-weapons.md
git add -A docs/superpowers/
git commit -m "chore: remove completed spec and plan files"
```
