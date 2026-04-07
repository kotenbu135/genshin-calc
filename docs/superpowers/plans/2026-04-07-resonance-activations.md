# Resonance Activations Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `resonance_activations` parameter to `resolve_team_stats` so frontends can toggle conditional resonance buffs (Cryo CritRate, Geo DmgBonus, Dendro extra EM).

**Architecture:** Add `resonance_conditional_buffs()` to `resonance.rs`, thread `&[(ElementalResonance, bool)]` through `resolve_team_stats` → `collect_buffs`, apply matching conditional buffs as `ResolvedBuff`. All existing call sites pass `&[]` for backward compatibility.

**Tech Stack:** Rust, cargo test

**Spec:** `docs/superpowers/specs/2026-04-07-resonance-activations-design.md`
**Issue:** #52

---

### Task 1: Add `resonance_conditional_buffs()` to resonance.rs

**Files:**
- Modify: `crates/core/src/resonance.rs:66-83`

- [ ] **Step 1: Write failing tests for conditional buffs**

Add tests in the existing `#[cfg(test)]` module at the bottom of `resonance.rs`:

```rust
#[test]
fn test_shattering_ice_conditional_buffs() {
    let buffs = resonance_conditional_buffs(ElementalResonance::ShatteringIce);
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0], (BuffableStat::CritRate, 0.15));
}

#[test]
fn test_enduring_rock_conditional_buffs() {
    let buffs = resonance_conditional_buffs(ElementalResonance::EnduringRock);
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0], (BuffableStat::DmgBonus, 0.15));
}

#[test]
fn test_sprawling_greenery_conditional_buffs() {
    let buffs = resonance_conditional_buffs(ElementalResonance::SprawlingGreenery);
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0], (BuffableStat::ElementalMastery, 30.0));
}

#[test]
fn test_unconditional_resonances_have_no_conditional_buffs() {
    assert!(resonance_conditional_buffs(ElementalResonance::FerventFlames).is_empty());
    assert!(resonance_conditional_buffs(ElementalResonance::SoothingWater).is_empty());
    assert!(resonance_conditional_buffs(ElementalResonance::HighVoltage).is_empty());
    assert!(resonance_conditional_buffs(ElementalResonance::ImpetuousWinds).is_empty());
    assert!(resonance_conditional_buffs(ElementalResonance::ProtectiveCanopy).is_empty());
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core resonance_conditional -- --nocapture`
Expected: compilation error — `resonance_conditional_buffs` not found.

- [ ] **Step 3: Implement `resonance_conditional_buffs`**

Add after `resonance_buffs()` (after line 83 in `crates/core/src/resonance.rs`):

```rust
/// Returns conditional stat buffs for a resonance.
///
/// These require user activation (e.g. enemy affected by Cryo, shielded, post-reaction).
/// Returns empty for resonances with no conditional effect.
pub fn resonance_conditional_buffs(resonance: ElementalResonance) -> Vec<(BuffableStat, f64)> {
    match resonance {
        ElementalResonance::ShatteringIce => {
            vec![(BuffableStat::CritRate, 0.15)]
        }
        ElementalResonance::EnduringRock => {
            vec![(BuffableStat::DmgBonus, 0.15)]
        }
        ElementalResonance::SprawlingGreenery => {
            vec![(BuffableStat::ElementalMastery, 30.0)]
        }
        _ => vec![],
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-core resonance_conditional -- --nocapture`
Expected: all 4 tests PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/resonance.rs
git commit -m "feat: add resonance_conditional_buffs() for conditional resonance effects (#52)"
```

---

### Task 2: Thread `resonance_activations` through `resolve_team_stats`

**Files:**
- Modify: `crates/core/src/team.rs:203-236` (collect_buffs)
- Modify: `crates/core/src/team.rs:317-322` (resolve_team_stats)
- Modify: `crates/core/src/team.rs:332-359` (resolve_team_stats_detailed)

- [ ] **Step 1: Write failing test for conditional resonance activation**

Add in the `#[cfg(test)]` module of `crates/core/src/team.rs`:

```rust
#[test]
fn test_cryo_resonance_activation_applies_crit_rate() {
    let team = vec![
        make_member(Element::Cryo, 800.0),
        make_member(Element::Cryo, 600.0),
        make_member(Element::Pyro, 700.0),
        make_member(Element::Hydro, 500.0),
    ];
    let activations = [(ElementalResonance::ShatteringIce, true)];
    let result = resolve_team_stats(&team, 0, &activations).unwrap();
    let has_crit = result
        .applied_buffs
        .iter()
        .any(|b| b.stat == BuffableStat::CritRate && (b.value - 0.15).abs() < EPSILON);
    assert!(has_crit, "ShatteringIce conditional CritRate +0.15 should be in applied_buffs");
}

#[test]
fn test_resonance_activation_false_does_not_apply() {
    let team = vec![
        make_member(Element::Cryo, 800.0),
        make_member(Element::Cryo, 600.0),
        make_member(Element::Pyro, 700.0),
        make_member(Element::Hydro, 500.0),
    ];
    let activations = [(ElementalResonance::ShatteringIce, false)];
    let result = resolve_team_stats(&team, 0, &activations).unwrap();
    let has_crit = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("ShatteringIce") && b.stat == BuffableStat::CritRate);
    assert!(!has_crit, "ShatteringIce with active=false should not appear");
}

#[test]
fn test_empty_activations_backward_compatible() {
    let team = vec![
        make_member(Element::Cryo, 800.0),
        make_member(Element::Cryo, 600.0),
        make_member(Element::Pyro, 700.0),
        make_member(Element::Hydro, 500.0),
    ];
    let result = resolve_team_stats(&team, 0, &[]).unwrap();
    let has_crit = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("ShatteringIce"));
    assert!(!has_crit, "Empty activations should not apply conditional buffs");
}

#[test]
fn test_activation_ignored_when_resonance_not_detected() {
    // 3-member team: no resonance possible
    let team = vec![
        make_member(Element::Cryo, 800.0),
        make_member(Element::Cryo, 600.0),
        make_member(Element::Pyro, 700.0),
    ];
    let activations = [(ElementalResonance::ShatteringIce, true)];
    let result = resolve_team_stats(&team, 0, &activations).unwrap();
    assert!(result.resonances.is_empty());
    let has_crit = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("ShatteringIce"));
    assert!(!has_crit, "Activation without detected resonance should be ignored");
}

#[test]
fn test_double_resonance_partial_activation() {
    // Cryo x2 + Geo x2: activate only Cryo
    let team = vec![
        make_member(Element::Cryo, 800.0),
        make_member(Element::Cryo, 600.0),
        make_member(Element::Geo, 700.0),
        make_member(Element::Geo, 500.0),
    ];
    let activations = [(ElementalResonance::ShatteringIce, true)];
    let result = resolve_team_stats(&team, 0, &activations).unwrap();
    let has_crit = result
        .applied_buffs
        .iter()
        .any(|b| b.stat == BuffableStat::CritRate && (b.value - 0.15).abs() < EPSILON);
    assert!(has_crit, "ShatteringIce should be active");
    let has_dmg = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("EnduringRock"));
    assert!(!has_dmg, "EnduringRock should NOT be active (not in activations)");
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core test_cryo_resonance_activation -- --nocapture`
Expected: compilation error — `resolve_team_stats` signature mismatch.

- [ ] **Step 3: Change signatures and implement**

In `crates/core/src/team.rs`:

**`collect_buffs`** (line 203): add `resonance_activations` parameter and conditional buff logic:

```rust
fn collect_buffs(
    team: &[TeamMember],
    target_index: usize,
    resonance_activations: &[(ElementalResonance, bool)],
) -> Vec<ResolvedBuff> {
    let mut buffs = Vec::new();

    for (i, member) in team.iter().enumerate() {
        for buff in &member.buffs_provided {
            let applies = match buff.target {
                BuffTarget::OnlySelf => i == target_index,
                BuffTarget::Team => true,
                BuffTarget::TeamExcludeSelf => i != target_index,
            };
            if applies {
                buffs.push(buff.clone());
            }
        }
    }

    // Elemental resonance buffs
    let elements: Vec<Element> = team.iter().map(|m| m.element).collect();
    let resonances = determine_resonances(&elements);
    for resonance in &resonances {
        // Unconditional buffs (always applied)
        for (stat, value) in resonance_buffs(*resonance) {
            buffs.push(ResolvedBuff {
                source: format!("{:?}", resonance),
                stat,
                value,
                target: BuffTarget::Team,
                origin: None,
            });
        }
        // Conditional buffs (only when activated)
        let is_active = resonance_activations
            .iter()
            .any(|(r, active)| r == resonance && *active);
        if is_active {
            for (stat, value) in resonance_conditional_buffs(*resonance) {
                buffs.push(ResolvedBuff {
                    source: format!("{:?}(conditional)", resonance),
                    stat,
                    value,
                    target: BuffTarget::Team,
                    origin: None,
                });
            }
        }
    }

    dedup_by_origin(&mut buffs);
    buffs
}
```

**`resolve_team_stats`** (line 317):

```rust
pub fn resolve_team_stats(
    team: &[TeamMember],
    target_index: usize,
    resonance_activations: &[(ElementalResonance, bool)],
) -> Result<TeamResolveResult, CalcError> {
    resolve_team_stats_detailed(team, target_index, resonance_activations)
}
```

**`resolve_team_stats_detailed`** (line 332):

```rust
pub fn resolve_team_stats_detailed(
    team: &[TeamMember],
    target_index: usize,
    resonance_activations: &[(ElementalResonance, bool)],
) -> Result<TeamResolveResult, CalcError> {
    validate_team(team, target_index)?;

    let base_profile = &team[target_index].stats;
    let base_stats = combine_stats(base_profile)?;

    let applied_buffs = collect_buffs(team, target_index, resonance_activations);
    let buffed_profile = apply_buffs_to_profile(base_profile, &applied_buffs);
    let final_stats = combine_stats(&buffed_profile)?;

    let elements: Vec<Element> = team.iter().map(|m| m.element).collect();
    let resonances = determine_resonances(&elements);

    let damage_context = DamageContext::from_buffs(&applied_buffs);
    let enemy_debuffs = collect_enemy_debuffs(&applied_buffs);

    Ok(TeamResolveResult {
        base_stats,
        applied_buffs,
        resonances,
        final_stats,
        damage_context,
        enemy_debuffs,
    })
}
```

Add import at top of team.rs: `use crate::resonance::resonance_conditional_buffs;`

Update the doctest in `resolve_team_stats` doc comment (line 314):
```rust
/// let result = resolve_team_stats(&[member], 0, &[]).unwrap();
```

- [ ] **Step 4: Fix all existing call sites in `crates/core/`**

All `resolve_team_stats(&..., N)` → `resolve_team_stats(&..., N, &[])` in:
- `crates/core/src/team.rs` tests (~15 calls)
- `crates/core/src/enemy.rs` test (line 796)
- `crates/core/src/lib.rs` doctest (line 56)

All `resolve_team_stats_detailed(&..., N)` → `resolve_team_stats_detailed(&..., N, &[])` in:
- `crates/core/src/team.rs` test (line 516)

- [ ] **Step 5: Run core tests**

Run: `cargo test -p genshin-calc-core`
Expected: all tests PASS including new ones.

- [ ] **Step 6: Commit**

```bash
git add crates/core/src/team.rs crates/core/src/lib.rs crates/core/src/enemy.rs crates/core/src/resonance.rs
git commit -m "feat: add resonance_activations parameter to resolve_team_stats (#52)"
```

---

### Task 3: Fix external call sites (data tests, wasm, good)

**Files:**
- Modify: `crates/data/tests/team_integration.rs` (2 calls)
- Modify: `crates/data/tests/meta_team_edge_cases.rs` (~14 calls)
- Modify: `crates/data/tests/meta_team_verification.rs` (~15 calls)
- Modify: `crates/wasm/src/lib.rs` (~10 calls)
- Modify: `crates/good/tests/evaluate_talent_buffs_integration.rs` (1 call)

- [ ] **Step 1: Update all `resolve_team_stats` calls with `&[]`**

Mechanical find-and-replace in each file:
- `resolve_team_stats(&team, N)` → `resolve_team_stats(&team, N, &[])`
- `resolve_team_stats(&[member], 0)` → `resolve_team_stats(&[member], 0, &[])`
- `resolve_team_stats_detailed(&team, N)` → `resolve_team_stats_detailed(&team, N, &[])`

For `crates/wasm/src/lib.rs`:
- Line 191: `genshin_calc_core::resolve_team_stats(&members, target_index as usize)` → add `, &[]`
- Line 269: same pattern
- All test calls: same pattern

- [ ] **Step 2: Run full test suite**

Run: `cargo test`
Expected: all tests PASS.

- [ ] **Step 3: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: no warnings.

- [ ] **Step 4: Commit**

```bash
git add crates/data/tests/ crates/wasm/src/lib.rs crates/good/tests/
git commit -m "chore: update all resolve_team_stats call sites with empty activations (#52)"
```

---

### Task 4: Add integration test with real character data

**Files:**
- Modify: `crates/data/tests/meta_team_verification.rs` or new test file

- [ ] **Step 1: Write integration test using real Cryo resonance team**

Add a test using real character data (e.g., 2 Cryo + 2 others) that activates ShatteringIce and verifies CritRate in final_stats:

```rust
#[test]
fn test_cryo_resonance_conditional_activation() {
    // Build a 4-member team with 2 Cryo characters
    // Activate ShatteringIce
    // Assert CritRate increased by 0.15 in final_stats vs without activation
    let team = /* build team with 2 Cryo members */;
    let result_off = resolve_team_stats_detailed(&team, 0, &[]).unwrap();
    let activations = [(ElementalResonance::ShatteringIce, true)];
    let result_on = resolve_team_stats_detailed(&team, 0, &activations).unwrap();
    assert!(
        (result_on.final_stats.crit_rate - result_off.final_stats.crit_rate - 0.15).abs() < EPSILON,
        "Activating ShatteringIce should add 0.15 CritRate"
    );
}
```

Similarly for EnduringRock (Geo) and SprawlingGreenery (Dendro extra EM). Use whichever characters are already available in the test helpers.

- [ ] **Step 2: Run tests**

Run: `cargo test -p genshin-calc-data resonance_conditional -- --nocapture`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add crates/data/tests/
git commit -m "test: add integration tests for conditional resonance activations (#52)"
```

---

### Task 5: Export from `crates/core/src/lib.rs` and update docs

**Files:**
- Modify: `crates/core/src/lib.rs:146` (ensure `resonance_conditional_buffs` is re-exported)

- [ ] **Step 1: Verify `resonance_conditional_buffs` is accessible from outside the crate**

Check if `resonance_conditional_buffs` is already exported via `pub use`. If not, add to re-exports in `crates/core/src/lib.rs`.

- [ ] **Step 2: Run full test suite + clippy + fmt**

```bash
cargo test && cargo clippy -- -D warnings && cargo fmt --check
```
Expected: all pass.

- [ ] **Step 3: Commit**

```bash
git add crates/core/src/lib.rs
git commit -m "chore: export resonance_conditional_buffs from core crate (#52)"
```
