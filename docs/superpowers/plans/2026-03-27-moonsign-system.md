# Moonsign System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the Moonsign (月兆) system — lunar reaction BaseDMGBonus from Nod-Krai characters, non-moonsign team buffs, talent enhancements, and contribution combination.

**Architecture:** New `core/moonsign.rs` module for pure Moonsign logic (level, buffs, combination). New `data/moonsign_chars.rs` for 9 character definitions. Extend `LunarInput` with `base_dmg_bonus` and `TeamMember` with `is_moonsign`. Follows existing immutable/pure-function patterns.

**Tech Stack:** Rust, serde, thiserror

**Spec:** `docs/superpowers/specs/2026-03-27-moonsign-system-design.md`

---

### Task 1: Add `ScalingStat::Em` variant

**Files:**
- Modify: `crates/core/src/types.rs:29-35`
- Modify: `crates/data/src/team_builder.rs:164-169`

- [ ] **Step 1: Add `Em` to `ScalingStat` enum**

```rust
// crates/core/src/types.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ScalingStat {
    #[default]
    Atk,
    Hp,
    Def,
    Em,
}
```

- [ ] **Step 2: Handle `Em` in `team_builder.rs` match**

In `crates/data/src/team_builder.rs:164-169`, add `Em` arm:

```rust
let base = match scaling_stat {
    genshin_calc_core::ScalingStat::Atk => profile.base_atk,
    genshin_calc_core::ScalingStat::Hp => profile.base_hp,
    genshin_calc_core::ScalingStat::Def => profile.base_def,
    genshin_calc_core::ScalingStat::Em => profile.elemental_mastery,
};
```

- [ ] **Step 3: Build and test**

Run: `cargo test`
Expected: All 166 existing tests pass. No existing code uses `ScalingStat::Em` yet.

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/types.rs crates/data/src/team_builder.rs
git commit -m "feat(core): add ScalingStat::Em variant for EM-based scaling"
```

---

### Task 2: Add `base_dmg_bonus` to `LunarInput` and update formula

**Files:**
- Modify: `crates/core/src/lunar.rs:13-26` (struct), `lunar.rs:98` (formula)
- Modify: `crates/core/src/lib.rs:75-83` (doc example)

- [ ] **Step 1: Write test for base_dmg_bonus effect**

Add to `crates/core/src/lunar.rs` tests module:

```rust
#[test]
fn test_lunar_base_dmg_bonus_applied() {
    let base = LunarInput {
        character_level: 90,
        elemental_mastery: 0.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: 0.0,
        crit_rate: 0.0,
        crit_dmg: 0.0,
        base_dmg_bonus: 0.0,
    };
    let with_bonus = LunarInput {
        base_dmg_bonus: 0.21,
        ..base.clone()
    };
    let r1 = calculate_lunar(&base, &default_enemy()).unwrap();
    let r2 = calculate_lunar(&with_bonus, &default_enemy()).unwrap();
    // (1 + 0.21) / (1 + 0.0) = 1.21
    assert!((r2.non_crit / r1.non_crit - 1.21).abs() < EPSILON);
}

#[test]
fn test_golden_lunar_ec_with_base_dmg_bonus() {
    // Lv90, EM 500, Lunar EC (1.8), base_dmg_bonus 0.14
    // em_bonus = 6 * 500 / (500 + 2000) = 1.2
    // non_crit = 1446.8535 * 1.8 * (1 + 0.14) * (1 + 1.2) * 0.9
    //          = 2604.336 * 1.14 * 2.2 * 0.9 = 5878.508
    let input = LunarInput {
        character_level: 90,
        elemental_mastery: 500.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: 0.0,
        crit_rate: 0.6,
        crit_dmg: 1.2,
        base_dmg_bonus: 0.14,
    };
    let result = calculate_lunar(&input, &default_enemy()).unwrap();
    assert!((result.non_crit - 5878.508).abs() < 0.1);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core lunar`
Expected: FAIL — `base_dmg_bonus` field does not exist yet.

- [ ] **Step 3: Add `base_dmg_bonus` field to `LunarInput`**

```rust
// crates/core/src/lunar.rs — add after crit_dmg field
    /// Base DMG Bonus from Moonsign Benediction passives (decimal form, default 0.0).
    pub base_dmg_bonus: f64,
```

- [ ] **Step 4: Update the calculation formula**

```rust
// crates/core/src/lunar.rs line 98 — replace
let non_crit = level_base * reaction_mult
    * (1.0 + input.base_dmg_bonus)
    * (1.0 + em_bonus + input.reaction_bonus)
    * res_mult;
```

- [ ] **Step 5: Add `base_dmg_bonus: 0.0` to ALL existing `LunarInput` constructions**

Update every test in `lunar.rs` and the doc example in `lib.rs:78-86` to include `base_dmg_bonus: 0.0`. Search for all `LunarInput {` occurrences:

Files to update:
- `crates/core/src/lunar.rs` — all test functions (~8 tests)
- `crates/core/src/lib.rs` — doc example

- [ ] **Step 6: Run all tests**

Run: `cargo test`
Expected: All tests pass, including the 2 new golden tests.

- [ ] **Step 7: Commit**

```bash
git add crates/core/src/lunar.rs crates/core/src/lib.rs
git commit -m "feat(core): add base_dmg_bonus to LunarInput for Moonsign BaseDMGBonus"
```

---

### Task 3: Create `core/moonsign.rs` — types and MoonsignLevel

**Files:**
- Create: `crates/core/src/moonsign.rs`
- Modify: `crates/core/src/lib.rs:91` (module declaration + re-exports)

- [ ] **Step 1: Write tests for MoonsignLevel determination**

Create `crates/core/src/moonsign.rs` with tests module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moonsign_level_none() {
        assert_eq!(determine_moonsign_level(0), MoonsignLevel::None);
    }

    #[test]
    fn test_moonsign_level_nascent() {
        assert_eq!(determine_moonsign_level(1), MoonsignLevel::NascentGleam);
    }

    #[test]
    fn test_moonsign_level_ascendant() {
        assert_eq!(determine_moonsign_level(2), MoonsignLevel::AscendantGleam);
        assert_eq!(determine_moonsign_level(4), MoonsignLevel::AscendantGleam);
    }

    #[test]
    fn test_moonsign_context_lookup_found() {
        let ctx = MoonsignContext {
            level: MoonsignLevel::NascentGleam,
            base_dmg_bonus_by_reaction: vec![
                (Reaction::LunarElectroCharged, 0.14),
            ],
            non_moonsign_lunar_bonus: 0.0,
            talent_enhancements: vec![],
        };
        assert!((ctx.base_dmg_bonus_for(Reaction::LunarElectroCharged) - 0.14).abs() < 1e-6);
    }

    #[test]
    fn test_moonsign_context_lookup_not_found() {
        let ctx = MoonsignContext {
            level: MoonsignLevel::None,
            base_dmg_bonus_by_reaction: vec![],
            non_moonsign_lunar_bonus: 0.0,
            talent_enhancements: vec![],
        };
        assert!((ctx.base_dmg_bonus_for(Reaction::LunarBloom) - 0.0).abs() < 1e-6);
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core moonsign`
Expected: FAIL — module and types don't exist yet.

- [ ] **Step 3: Implement types and `determine_moonsign_level`**

Write the full module with:
- `MoonsignLevel` enum (None, NascentGleam, AscendantGleam) with Serialize/Deserialize
- `MoonsignBenediction` struct (base_dmg_bonus, enabled_reactions)
- `MoonsignTalentEnhancement` struct (character_name, required_level, description, effect)
- `MoonsignTalentEffect` enum (GrantReactionCrit, StatBuff)
- `MoonsignContext` struct with `base_dmg_bonus_for()` helper
- `determine_moonsign_level()` function

- [ ] **Step 4: Register module in `lib.rs`**

Add to `crates/core/src/lib.rs`:
```rust
pub mod moonsign;
```

Add re-exports:
```rust
pub use moonsign::{
    MoonsignBenediction, MoonsignContext, MoonsignLevel, MoonsignTalentEffect,
    MoonsignTalentEnhancement, determine_moonsign_level,
};
```

- [ ] **Step 5: Run tests**

Run: `cargo test -p genshin-calc-core moonsign`
Expected: All 5 new tests pass.

- [ ] **Step 6: Commit**

```bash
git add crates/core/src/moonsign.rs crates/core/src/lib.rs
git commit -m "feat(core): add moonsign module with types and level determination"
```

---

### Task 4: Non-Moonsign buff calculation

**Files:**
- Modify: `crates/core/src/moonsign.rs`

- [ ] **Step 1: Write tests for non-moonsign buff**

```rust
#[test]
fn test_non_moonsign_scaling_pyro() {
    let buff = non_moonsign_scaling(Element::Pyro);
    assert_eq!(buff.scaling_stat, ScalingStat::Atk);
    assert!((buff.max_bonus - 0.36).abs() < 1e-6);
}

#[test]
fn test_non_moonsign_scaling_hydro() {
    let buff = non_moonsign_scaling(Element::Hydro);
    assert_eq!(buff.scaling_stat, ScalingStat::Hp);
}

#[test]
fn test_non_moonsign_scaling_anemo() {
    let buff = non_moonsign_scaling(Element::Anemo);
    assert_eq!(buff.scaling_stat, ScalingStat::Em);
}

#[test]
fn test_calculate_non_moonsign_bonus_atk_capped() {
    let buff = non_moonsign_scaling(Element::Pyro);
    // 4000 ATK * 0.00009 = 0.36 (at cap)
    assert!((calculate_non_moonsign_bonus(&buff, 4000.0) - 0.36).abs() < 1e-6);
    // 5000 ATK * 0.00009 = 0.45 → capped at 0.36
    assert!((calculate_non_moonsign_bonus(&buff, 5000.0) - 0.36).abs() < 1e-6);
}

#[test]
fn test_calculate_non_moonsign_bonus_below_cap() {
    let buff = non_moonsign_scaling(Element::Pyro);
    // 2000 ATK * 0.00009 = 0.18
    assert!((calculate_non_moonsign_bonus(&buff, 2000.0) - 0.18).abs() < 1e-6);
}

#[test]
fn test_select_non_moonsign_buff_picks_max() {
    // Pyro with 2000 ATK → 0.18, Dendro with 800 EM → 0.18
    // Pyro with 3000 ATK → 0.27 (higher)
    let members = vec![
        (Element::Pyro, 3000.0),
        (Element::Dendro, 800.0),
    ];
    let result = select_non_moonsign_buff(&members);
    assert!((result - 0.27).abs() < 1e-6);
}

#[test]
fn test_select_non_moonsign_buff_empty() {
    assert!((select_non_moonsign_buff(&[]) - 0.0).abs() < 1e-6);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core moonsign`
Expected: FAIL — functions don't exist.

- [ ] **Step 3: Implement**

Add to `crates/core/src/moonsign.rs`:
- `NonMoonsignLunarBuff` struct
- `non_moonsign_scaling(element)` — mapping table for all 7 elements
- `calculate_non_moonsign_bonus(buff, stat_value)` — `(rate * stat_value).min(max_bonus)`
- `select_non_moonsign_buff(members)` — fold max

- [ ] **Step 4: Run tests**

Run: `cargo test -p genshin-calc-core moonsign`
Expected: All tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/moonsign.rs
git commit -m "feat(core): add non-moonsign lunar buff calculation"
```

---

### Task 5: Contribution combination — `calculate_lunar_team`

**Files:**
- Modify: `crates/core/src/moonsign.rs`

- [ ] **Step 1: Write tests**

```rust
#[test]
fn test_lunar_team_single_contribution() {
    let input = LunarInput {
        character_level: 90,
        elemental_mastery: 0.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: 0.0,
        crit_rate: 0.5,
        crit_dmg: 1.0,
        base_dmg_bonus: 0.0,
    };
    let enemy = Enemy { level: 90, resistance: 0.1, def_reduction: 0.0 };
    let contributions = vec![LunarContribution { input: input.clone() }];
    let team_result = calculate_lunar_team(&contributions, &enemy).unwrap();
    let solo_result = calculate_lunar(&input, &enemy).unwrap();
    assert!((team_result.average - solo_result.average).abs() < 1e-6);
}

#[test]
fn test_lunar_team_two_contributions() {
    let enemy = Enemy { level: 90, resistance: 0.1, def_reduction: 0.0 };
    let input1 = LunarInput {
        character_level: 90,
        elemental_mastery: 500.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: 0.0,
        crit_rate: 0.6,
        crit_dmg: 1.2,
        base_dmg_bonus: 0.0,
    };
    let input2 = LunarInput {
        character_level: 90,
        elemental_mastery: 100.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: 0.0,
        crit_rate: 0.3,
        crit_dmg: 0.6,
        base_dmg_bonus: 0.0,
    };
    let r1 = calculate_lunar(&input1, &enemy).unwrap();
    let r2 = calculate_lunar(&input2, &enemy).unwrap();
    // r1.average > r2.average, so r1 is rank 1
    let expected_avg = r1.average * 1.0 + r2.average * 0.5;
    let contributions = vec![
        LunarContribution { input: input1 },
        LunarContribution { input: input2 },
    ];
    let result = calculate_lunar_team(&contributions, &enemy).unwrap();
    assert!((result.average - expected_avg).abs() < 0.01);
}

#[test]
fn test_lunar_team_two_contributions_reverse_order() {
    // Same inputs as above but in reverse order — verify sort handles it
    let enemy = Enemy { level: 90, resistance: 0.1, def_reduction: 0.0 };
    let input1 = LunarInput {
        character_level: 90,
        elemental_mastery: 500.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: 0.0,
        crit_rate: 0.6,
        crit_dmg: 1.2,
        base_dmg_bonus: 0.0,
    };
    let input2 = LunarInput {
        character_level: 90,
        elemental_mastery: 100.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: 0.0,
        crit_rate: 0.3,
        crit_dmg: 0.6,
        base_dmg_bonus: 0.0,
    };
    let r1 = calculate_lunar(&input1, &enemy).unwrap();
    let r2 = calculate_lunar(&input2, &enemy).unwrap();
    let expected_avg = r1.average * 1.0 + r2.average * 0.5;
    // Give contributions in REVERSE order (weaker first)
    let contributions = vec![
        LunarContribution { input: input2 },
        LunarContribution { input: input1 },
    ];
    let result = calculate_lunar_team(&contributions, &enemy).unwrap();
    // Should produce same result regardless of input order
    assert!((result.average - expected_avg).abs() < 0.01);
}

#[test]
fn test_lunar_team_empty_error() {
    let enemy = Enemy { level: 90, resistance: 0.1, def_reduction: 0.0 };
    let result = calculate_lunar_team(&[], &enemy);
    assert!(matches!(result, Err(CalcError::InvalidTeamSize(0))));
}

#[test]
fn test_lunar_team_five_error() {
    let enemy = Enemy { level: 90, resistance: 0.1, def_reduction: 0.0 };
    let input = LunarInput {
        character_level: 90,
        elemental_mastery: 0.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: 0.0,
        crit_rate: 0.0,
        crit_dmg: 0.0,
        base_dmg_bonus: 0.0,
    };
    let contributions: Vec<LunarContribution> = (0..5)
        .map(|_| LunarContribution { input: input.clone() })
        .collect();
    let result = calculate_lunar_team(&contributions, &enemy);
    assert!(matches!(result, Err(CalcError::InvalidTeamSize(5))));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core moonsign`
Expected: FAIL — `LunarContribution` and `calculate_lunar_team` don't exist.

- [ ] **Step 3: Implement**

Add to `crates/core/src/moonsign.rs`:
- `CONTRIBUTION_WEIGHTS: [f64; 4]` constant
- `LunarContribution` struct
- `calculate_lunar_team()` function with validation, per-character calculation, sort, weighted sum

- [ ] **Step 4: Run tests**

Run: `cargo test -p genshin-calc-core moonsign`
Expected: All tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/moonsign.rs
git commit -m "feat(core): add calculate_lunar_team for contribution combination"
```

---

### Task 6: Talent enhancements — `apply_moonsign_enhancements`

**Files:**
- Modify: `crates/core/src/moonsign.rs`

- [ ] **Step 1: Write tests**

```rust
#[test]
fn test_apply_moonsign_enhancement_matching_reaction() {
    let input = LunarInput {
        character_level: 90,
        elemental_mastery: 0.0,
        reaction: Reaction::LunarBloom,
        reaction_bonus: 0.0,
        crit_rate: 0.0,
        crit_dmg: 0.0,
        base_dmg_bonus: 0.0,
    };
    let enhancements = vec![MoonsignTalentEnhancement {
        character_name: "Lauma",
        required_level: MoonsignLevel::NascentGleam,
        description: "Bloom gains crit",
        effect: MoonsignTalentEffect::GrantReactionCrit {
            reaction: Reaction::LunarBloom,
            crit_rate: 0.15,
            crit_dmg: 1.0,
        },
    }];
    let result = apply_moonsign_enhancements(&input, &enhancements);
    assert!((result.crit_rate - 0.15).abs() < 1e-6);
    assert!((result.crit_dmg - 1.0).abs() < 1e-6);
}

#[test]
fn test_apply_moonsign_enhancement_non_matching_reaction() {
    let input = LunarInput {
        character_level: 90,
        elemental_mastery: 0.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: 0.0,
        crit_rate: 0.5,
        crit_dmg: 1.0,
        base_dmg_bonus: 0.0,
    };
    let enhancements = vec![MoonsignTalentEnhancement {
        character_name: "Lauma",
        required_level: MoonsignLevel::NascentGleam,
        description: "Bloom gains crit",
        effect: MoonsignTalentEffect::GrantReactionCrit {
            reaction: Reaction::LunarBloom,
            crit_rate: 0.15,
            crit_dmg: 1.0,
        },
    }];
    let result = apply_moonsign_enhancements(&input, &enhancements);
    // No change — reaction doesn't match
    assert!((result.crit_rate - 0.5).abs() < 1e-6);
    assert!((result.crit_dmg - 1.0).abs() < 1e-6);
}

#[test]
fn test_apply_moonsign_enhancement_crit_rate_capped() {
    let input = LunarInput {
        character_level: 90,
        elemental_mastery: 0.0,
        reaction: Reaction::LunarBloom,
        reaction_bonus: 0.0,
        crit_rate: 0.95,
        crit_dmg: 0.5,
        base_dmg_bonus: 0.0,
    };
    let enhancements = vec![MoonsignTalentEnhancement {
        character_name: "Lauma",
        required_level: MoonsignLevel::NascentGleam,
        description: "Bloom gains crit",
        effect: MoonsignTalentEffect::GrantReactionCrit {
            reaction: Reaction::LunarBloom,
            crit_rate: 0.15,
            crit_dmg: 1.0,
        },
    }];
    let result = apply_moonsign_enhancements(&input, &enhancements);
    // 0.95 + 0.15 = 1.10 → capped at 1.0
    assert!((result.crit_rate - 1.0).abs() < 1e-6);
    assert!((result.crit_dmg - 1.5).abs() < 1e-6);
}

#[test]
fn test_apply_moonsign_enhancement_empty() {
    let input = LunarInput {
        character_level: 90,
        elemental_mastery: 0.0,
        reaction: Reaction::LunarBloom,
        reaction_bonus: 0.0,
        crit_rate: 0.5,
        crit_dmg: 1.0,
        base_dmg_bonus: 0.0,
    };
    let result = apply_moonsign_enhancements(&input, &[]);
    assert!((result.crit_rate - 0.5).abs() < 1e-6);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core moonsign`
Expected: FAIL — `apply_moonsign_enhancements` doesn't exist.

- [ ] **Step 3: Implement `apply_moonsign_enhancements`**

```rust
pub fn apply_moonsign_enhancements(
    input: &LunarInput,
    enhancements: &[MoonsignTalentEnhancement],
) -> LunarInput {
    let mut result = input.clone();
    for enh in enhancements {
        if let MoonsignTalentEffect::GrantReactionCrit {
            reaction,
            crit_rate,
            crit_dmg,
        } = &enh.effect
        {
            if *reaction == input.reaction {
                result.crit_rate = (result.crit_rate + crit_rate).min(1.0);
                result.crit_dmg += crit_dmg;
            }
        }
    }
    result
}
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p genshin-calc-core moonsign`
Expected: All tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/moonsign.rs
git commit -m "feat(core): add apply_moonsign_enhancements for talent crit grants"
```

---

### Task 7: `resolve_moonsign_context`

**Files:**
- Modify: `crates/core/src/moonsign.rs`

- [ ] **Step 1: Write tests**

```rust
#[test]
fn test_resolve_context_no_moonsign() {
    let ctx = resolve_moonsign_context(0, &[], 0.0, vec![]);
    assert_eq!(ctx.level, MoonsignLevel::None);
    assert!(ctx.base_dmg_bonus_by_reaction.is_empty());
    assert!((ctx.non_moonsign_lunar_bonus - 0.0).abs() < 1e-6);
}

#[test]
fn test_resolve_context_single_benediction() {
    let benedictions = vec![MoonsignBenediction {
        base_dmg_bonus: 0.14,
        enabled_reactions: vec![Reaction::LunarElectroCharged],
    }];
    let ctx = resolve_moonsign_context(1, &benedictions, 0.0, vec![]);
    assert_eq!(ctx.level, MoonsignLevel::NascentGleam);
    assert!((ctx.base_dmg_bonus_for(Reaction::LunarElectroCharged) - 0.14).abs() < 1e-6);
    // Non-moonsign bonus is 0 because we're at NascentGleam, not AscendantGleam
    // (but the function takes pre-calculated bonus; caller handles the level check)
}

#[test]
fn test_resolve_context_stacking_same_reaction() {
    // Ineffa (0.14) + Columbina (0.07) → LunarEC = 0.21
    let benedictions = vec![
        MoonsignBenediction {
            base_dmg_bonus: 0.14,
            enabled_reactions: vec![Reaction::LunarElectroCharged],
        },
        MoonsignBenediction {
            base_dmg_bonus: 0.07,
            enabled_reactions: vec![
                Reaction::LunarElectroCharged,
                Reaction::LunarBloom,
                Reaction::LunarCrystallize,
            ],
        },
    ];
    let ctx = resolve_moonsign_context(2, &benedictions, 0.30, vec![]);
    assert_eq!(ctx.level, MoonsignLevel::AscendantGleam);
    assert!((ctx.base_dmg_bonus_for(Reaction::LunarElectroCharged) - 0.21).abs() < 1e-6);
    assert!((ctx.base_dmg_bonus_for(Reaction::LunarBloom) - 0.07).abs() < 1e-6);
    assert!((ctx.non_moonsign_lunar_bonus - 0.30).abs() < 1e-6);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core moonsign`
Expected: FAIL — `resolve_moonsign_context` doesn't exist.

- [ ] **Step 3: Implement**

```rust
pub fn resolve_moonsign_context(
    moonsign_count: usize,
    benedictions: &[MoonsignBenediction],
    non_moonsign_bonus: f64,
    enhancements: Vec<MoonsignTalentEnhancement>,
) -> MoonsignContext {
    let level = determine_moonsign_level(moonsign_count);

    // Accumulate base_dmg_bonus per reaction type
    let mut by_reaction: Vec<(Reaction, f64)> = Vec::new();
    for bene in benedictions {
        for &reaction in &bene.enabled_reactions {
            if let Some(entry) = by_reaction.iter_mut().find(|(r, _)| *r == reaction) {
                entry.1 += bene.base_dmg_bonus;
            } else {
                by_reaction.push((reaction, bene.base_dmg_bonus));
            }
        }
    }

    MoonsignContext {
        level,
        base_dmg_bonus_by_reaction: by_reaction,
        non_moonsign_lunar_bonus: non_moonsign_bonus,
        talent_enhancements: enhancements,
    }
}
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p genshin-calc-core moonsign`
Expected: All tests pass.

- [ ] **Step 5: Update re-exports in `lib.rs`**

Add `resolve_moonsign_context`, `NonMoonsignLunarBuff`, `calculate_non_moonsign_bonus`, `select_non_moonsign_buff`, `LunarContribution`, `calculate_lunar_team`, `apply_moonsign_enhancements` to the public API.

- [ ] **Step 6: Run full test suite**

Run: `cargo test`
Expected: All tests pass.

- [ ] **Step 7: Commit**

```bash
git add crates/core/src/moonsign.rs crates/core/src/lib.rs
git commit -m "feat(core): add resolve_moonsign_context and complete moonsign public API"
```

---

### Task 8: Add `is_moonsign` to `TeamMember`

**Files:**
- Modify: `crates/core/src/team.rs:36-45` (TeamMember)
- Modify: `crates/core/src/lib.rs` (doc example)

`MoonsignContext` is NOT added to `TeamResolveResult`. The full context (with benediction bonuses, non-moonsign buff, talent enhancements) must be constructed by the data crate caller using `resolve_moonsign_context()`. Core only provides `is_moonsign` on TeamMember so callers can count moonsign members.

- [ ] **Step 1: Add `is_moonsign: bool` to `TeamMember`**

```rust
pub struct TeamMember {
    pub element: Element,
    pub weapon_type: WeaponType,
    pub stats: StatProfile,
    pub buffs_provided: Vec<ResolvedBuff>,
    /// Whether this character is a Moonsign (月兆) character from Nod-Krai.
    pub is_moonsign: bool,
}
```

- [ ] **Step 2: Update ALL existing `TeamMember` constructions to include `is_moonsign: false`**

Search for all `TeamMember {` in:
- `crates/core/src/team.rs` — `make_member()` helper (line 207) and any direct constructions
- `crates/core/src/lib.rs` — doc example in `//!` comments (lines 24-51)
- `crates/data/src/team_builder.rs:184` — `build()` return
- `crates/data/tests/team_integration.rs` — integration tests with direct struct construction

- [ ] **Step 3: Run all tests**

Run: `cargo test`
Expected: All tests pass.

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/team.rs crates/core/src/lib.rs crates/data/src/team_builder.rs crates/data/tests/team_integration.rs
git commit -m "feat(core): add is_moonsign flag to TeamMember"
```

---

### Task 9: Create `data/moonsign_chars.rs` — character data

**Files:**
- Create: `crates/data/src/moonsign_chars.rs`
- Modify: `crates/data/src/lib.rs` (module + re-exports)

- [ ] **Step 1: Write tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use genshin_calc_core::Reaction;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_is_moonsign_known_characters() {
        assert!(is_moonsign_character("ineffa"));
        assert!(is_moonsign_character("flins"));
        assert!(is_moonsign_character("lauma"));
        assert!(is_moonsign_character("columbina"));
        assert!(is_moonsign_character("aino"));
        assert!(!is_moonsign_character("bennett"));
        assert!(!is_moonsign_character("diluc"));
    }

    #[test]
    fn test_find_moonsign_benediction_ineffa() {
        let def = find_moonsign_benediction("ineffa").unwrap();
        assert_eq!(def.enabled_reactions, &[Reaction::LunarElectroCharged]);
        assert!((def.max_bonus - 0.14).abs() < EPSILON);
    }

    #[test]
    fn test_find_moonsign_benediction_columbina_all_reactions() {
        let def = find_moonsign_benediction("columbina").unwrap();
        // Columbina enables all 3 base lunar reaction types:
        // LunarElectroCharged, LunarBloom, LunarCrystallize
        // (LunarCrystallizeSecondary is a sub-type, not separately enabled)
        assert_eq!(def.enabled_reactions.len(), 3);
        assert!(def.enabled_reactions.contains(&Reaction::LunarElectroCharged));
        assert!(def.enabled_reactions.contains(&Reaction::LunarBloom));
        assert!(def.enabled_reactions.contains(&Reaction::LunarCrystallize));
        assert!((def.max_bonus - 0.07).abs() < EPSILON);
    }

    #[test]
    fn test_find_moonsign_benediction_aino_none() {
        let def = find_moonsign_benediction("aino").unwrap();
        assert!(def.enabled_reactions.is_empty());
        assert!((def.max_bonus - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_benediction_bonus_ineffa() {
        let def = find_moonsign_benediction("ineffa").unwrap();
        // 2000 ATK * 0.00007 = 0.14 (at cap)
        assert!((calculate_benediction_bonus(def, 2000.0) - 0.14).abs() < EPSILON);
        // 1000 ATK * 0.00007 = 0.07
        assert!((calculate_benediction_bonus(def, 1000.0) - 0.07).abs() < EPSILON);
        // 3000 ATK → capped at 0.14
        assert!((calculate_benediction_bonus(def, 3000.0) - 0.14).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_benediction_bonus_lauma_em() {
        let def = find_moonsign_benediction("lauma").unwrap();
        // 800 EM * 0.000175 = 0.14 (at cap)
        assert!((calculate_benediction_bonus(def, 800.0) - 0.14).abs() < EPSILON);
        // 400 EM * 0.000175 = 0.07
        assert!((calculate_benediction_bonus(def, 400.0) - 0.07).abs() < EPSILON);
    }

    #[test]
    fn test_find_moonsign_talent_enhancements_lauma() {
        let enhancements = find_moonsign_talent_enhancements("lauma");
        assert_eq!(enhancements.len(), 1);
        assert_eq!(enhancements[0].required_level, MoonsignLevel::NascentGleam);
    }

    #[test]
    fn test_all_moonsign_characters_count() {
        assert_eq!(ALL_MOONSIGN_BENEDICTIONS.len(), 9);
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data moonsign`
Expected: FAIL — module doesn't exist.

- [ ] **Step 3: Implement the module**

Create `crates/data/src/moonsign_chars.rs` with:
- `ALL_MOONSIGN_BENEDICTIONS: &[MoonsignBenedictionDef]` — 9 entries
- `LAUMA_TALENT_ENHANCEMENTS` — Lauma crit grant
- `is_moonsign_character(id)` — lookup
- `find_moonsign_benediction(id)` — lookup
- `calculate_benediction_bonus(def, stat_value)` — `(rate * stat_value).min(max_bonus)`
- `find_moonsign_talent_enhancements(id)` — returns Vec

- [ ] **Step 4: Register module in `lib.rs`**

```rust
pub mod moonsign_chars;
pub use moonsign_chars::{
    is_moonsign_character, find_moonsign_benediction, calculate_benediction_bonus,
    find_moonsign_talent_enhancements, MoonsignBenedictionDef, ALL_MOONSIGN_BENEDICTIONS,
};
```

- [ ] **Step 5: Run tests**

Run: `cargo test -p genshin-calc-data moonsign`
Expected: All tests pass.

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/moonsign_chars.rs crates/data/src/lib.rs
git commit -m "feat(data): add moonsign character data for 9 Nod-Krai characters"
```

---

### Task 10: Update `TeamMemberBuilder` for `is_moonsign` auto-detection

**Files:**
- Modify: `crates/data/src/team_builder.rs`

- [ ] **Step 1: Write test**

```rust
#[test]
fn test_moonsign_character_auto_detected() {
    // This test requires moonsign characters to be in the character database.
    // If they are not yet added, use is_moonsign_character directly.
    // For now, test that non-moonsign characters get is_moonsign = false.
    let bennett = find_character("bennett").unwrap();
    let weapon = find_weapon("aquila_favonia").unwrap();
    let member = TeamMemberBuilder::new(bennett, weapon).build().unwrap();
    assert!(!member.is_moonsign);
}
```

- [ ] **Step 2: Run test to verify it fails or passes**

Run: `cargo test -p genshin-calc-data team_builder`
Expected: Depends on Task 8 completion. `is_moonsign` field exists but needs to be set.

- [ ] **Step 3: Update `build()` to set `is_moonsign`**

In `crates/data/src/team_builder.rs`, add import and update the `build()` return:

```rust
use crate::moonsign_chars::is_moonsign_character;

// In build(), replace the Ok(TeamMember { ... }) block:
Ok(TeamMember {
    element: char_data.element,
    weapon_type: char_data.weapon_type,
    stats: profile,
    buffs_provided: buffs,
    is_moonsign: is_moonsign_character(char_data.id),
})
```

- [ ] **Step 4: Run tests**

Run: `cargo test`
Expected: All tests pass.

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "feat(data): auto-detect is_moonsign in TeamMemberBuilder"
```

---

### Task 11: Integration tests — full Moonsign pipeline

**Files:**
- Create: `crates/core/tests/moonsign_integration.rs`

- [ ] **Step 1: Write integration test**

```rust
//! Integration tests for the complete Moonsign pipeline.

use genshin_calc_core::*;

const EPSILON: f64 = 0.01;

/// Test: Ineffa solo → LunarElectroCharged with BaseDMGBonus
#[test]
fn test_ineffa_solo_lunar_ec() {
    // Ineffa: ATK 2000 → BaseDMGBonus = 2000 * 0.00007 = 0.14
    let benediction = MoonsignBenediction {
        base_dmg_bonus: 0.14,
        enabled_reactions: vec![Reaction::LunarElectroCharged],
    };
    let ctx = resolve_moonsign_context(1, &[benediction], 0.0, vec![]);

    let input = LunarInput {
        character_level: 90,
        elemental_mastery: 300.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: 0.0,
        crit_rate: 0.6,
        crit_dmg: 1.2,
        base_dmg_bonus: ctx.base_dmg_bonus_for(Reaction::LunarElectroCharged),
    };

    let enemy = Enemy { level: 90, resistance: 0.1, def_reduction: 0.0 };
    let result = calculate_lunar(&input, &enemy).unwrap();

    // Verify BaseDMGBonus is applied
    // em_bonus = 6 * 300 / (300 + 2000) = 0.7826...
    // non_crit = 1446.8535 * 1.8 * (1 + 0.14) * (1 + 0.7826) * 0.9
    let level_base = 1446.8535;
    let em_bonus = 6.0 * 300.0 / (300.0 + 2000.0);
    let expected = level_base * 1.8 * 1.14 * (1.0 + em_bonus) * 0.9;
    assert!((result.non_crit - expected).abs() < EPSILON);
}

/// Test: Lauma NascentGleam → LunarBloom with crit grant
#[test]
fn test_lauma_nascent_gleam_bloom_crit() {
    let enhancements = vec![MoonsignTalentEnhancement {
        character_name: "Lauma",
        required_level: MoonsignLevel::NascentGleam,
        description: "Bloom gains crit",
        effect: MoonsignTalentEffect::GrantReactionCrit {
            reaction: Reaction::LunarBloom,
            crit_rate: 0.15,
            crit_dmg: 1.0,
        },
    }];
    let benediction = MoonsignBenediction {
        base_dmg_bonus: 0.14,
        enabled_reactions: vec![Reaction::LunarBloom],
    };
    let ctx = resolve_moonsign_context(1, &[benediction], 0.0, enhancements);

    let base_input = LunarInput {
        character_level: 90,
        elemental_mastery: 800.0,
        reaction: Reaction::LunarBloom,
        reaction_bonus: 0.0,
        crit_rate: 0.0,
        crit_dmg: 0.0,
        base_dmg_bonus: ctx.base_dmg_bonus_for(Reaction::LunarBloom),
    };

    // Apply Lauma's crit grant
    let input = apply_moonsign_enhancements(&base_input, &ctx.talent_enhancements);
    assert!((input.crit_rate - 0.15).abs() < 1e-6);
    assert!((input.crit_dmg - 1.0).abs() < 1e-6);

    let enemy = Enemy { level: 90, resistance: 0.1, def_reduction: 0.0 };
    let result = calculate_lunar(&input, &enemy).unwrap();
    // Crit should be higher than non_crit
    assert!(result.crit > result.non_crit);
}

/// Test: 2-member team (Ineffa + Columbina) with non-moonsign buff
#[test]
fn test_two_moonsign_with_non_moonsign_buff() {
    let benedictions = vec![
        MoonsignBenediction {
            base_dmg_bonus: 0.14,
            enabled_reactions: vec![Reaction::LunarElectroCharged],
        },
        MoonsignBenediction {
            base_dmg_bonus: 0.07,
            enabled_reactions: vec![
                Reaction::LunarElectroCharged,
                Reaction::LunarBloom,
                Reaction::LunarCrystallize,
            ],
        },
    ];

    // Non-moonsign: Pyro character with 2000 ATK → 0.18
    let non_moonsign_bonus = select_non_moonsign_buff(&[(Element::Pyro, 2000.0)]);
    assert!((non_moonsign_bonus - 0.18).abs() < 1e-6);

    let ctx = resolve_moonsign_context(2, &benedictions, non_moonsign_bonus, vec![]);
    assert_eq!(ctx.level, MoonsignLevel::AscendantGleam);
    assert!((ctx.base_dmg_bonus_for(Reaction::LunarElectroCharged) - 0.21).abs() < 1e-6);

    let input = LunarInput {
        character_level: 90,
        elemental_mastery: 200.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: ctx.non_moonsign_lunar_bonus,
        crit_rate: 0.5,
        crit_dmg: 1.0,
        base_dmg_bonus: ctx.base_dmg_bonus_for(Reaction::LunarElectroCharged),
    };

    let enemy = Enemy { level: 90, resistance: 0.1, def_reduction: 0.0 };
    let result = calculate_lunar(&input, &enemy).unwrap();
    assert!(result.average > 0.0);
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test -p genshin-calc-core --test moonsign_integration`
Expected: All 3 integration tests pass.

- [ ] **Step 3: Run full test suite + clippy**

Run: `cargo test && cargo clippy -- -D warnings`
Expected: All tests pass, no clippy warnings.

- [ ] **Step 4: Commit**

```bash
git add crates/core/tests/moonsign_integration.rs
git commit -m "test(core): add moonsign integration tests for full pipeline"
```

---

### Task 12: Update CLAUDE.md

**Files:**
- Modify: `CLAUDE.md`

- [ ] **Step 1: Update Architecture section**

Add to the core crate description:
```
- `moonsign.rs`: 月兆システム（MoonsignLevel, MoonsignContext, 非月兆バフ, 貢献度合算, タレント強化適用）
```

Add to the data crate description:
```
- `moonsign_chars.rs`: 月兆キャラデータ（9キャラの月光の祝福パッシブ + Laumaタレント強化）
```

Update the Testing section with new test counts.

- [ ] **Step 2: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md with moonsign system architecture"
```
