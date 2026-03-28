# P6: Enemy Debuff System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement `apply_enemy_debuffs` to consume resistance/DEF reduction buffs from `ResolvedBuff`, add superconduct helper, and add talent debuffs for Zhongli/Chevreuse/Lisa/Faruzan.

**Architecture:** Pure function `apply_enemy_debuffs(enemy, buffs, element) -> Enemy` in core crate filters `ResolvedBuff` by `BuffableStat` type (not `BuffTarget`). New `BuffableStat::DefReduction` variant for Lisa. Data crate adds 4 character talent debuff definitions using existing `TalentBuffDef` pattern.

**Tech Stack:** Rust, genshin-calc-core, genshin-calc-data

**Spec:** `docs/superpowers/specs/2026-03-29-enemy-debuff-system-design.md`

---

### Task 1: Add `BuffableStat::DefReduction` variant

**Files:**
- Modify: `crates/core/src/buff_types.rs:33` (add variant after `PhysicalResReduction`)

- [ ] **Step 1: Add the variant**

In `crates/core/src/buff_types.rs`, add `DefReduction` after `PhysicalResReduction`:

```rust
    // Enemy resistance reduction (consumed by damage calculation, not applied to StatProfile)
    ElementalResReduction(Element),
    PhysicalResReduction,
    DefReduction,
```

- [ ] **Step 2: Verify build**

Run: `cargo build -p genshin-calc-core`
Expected: PASS (new variant is additive, no exhaustive matches break)

- [ ] **Step 3: Run existing tests**

Run: `cargo test -p genshin-calc-core`
Expected: All 183 tests PASS

- [ ] **Step 4: Add serde roundtrip test for DefReduction**

Add to `crates/core/src/buff_types.rs` (create a `#[cfg(test)] mod tests` if not present):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_def_reduction_serde_roundtrip() {
        let stat = BuffableStat::DefReduction;
        let json = serde_json::to_string(&stat).unwrap();
        let deser: BuffableStat = serde_json::from_str(&json).unwrap();
        assert_eq!(stat, deser);
    }
}
```

Note: Add `serde_json` to dev-dependencies in `crates/core/Cargo.toml` if not already present.

- [ ] **Step 5: Run all tests**

Run: `cargo test -p genshin-calc-core && cargo test -p genshin-calc-data`
Expected: All PASS

- [ ] **Step 6: Commit**

```bash
git add crates/core/src/buff_types.rs
git commit -m "feat(core): add BuffableStat::DefReduction variant for enemy DEF debuffs"
```

---

### Task 2: Implement `apply_enemy_debuffs` with TDD

**Files:**
- Modify: `crates/core/src/enemy.rs` (add function + tests)

- [ ] **Step 1: Write failing tests**

Add to `crates/core/src/enemy.rs`:

```rust
use crate::buff_types::BuffableStat;
use crate::team::{BuffTarget, ResolvedBuff};
use crate::types::Element;

/// Applies enemy debuffs (resistance reduction, DEF reduction) from resolved buffs.
///
/// Filters buffs by `BuffableStat` type:
/// - `ElementalResReduction(e)`: reduces `resistance` when `element == Some(e)`
/// - `PhysicalResReduction`: reduces `resistance` when `element == None`
/// - `DefReduction`: adds to `def_reduction` (clamped to 1.0)
///
/// Other `BuffableStat` variants are ignored.
/// Returns a new `Enemy` (immutable pattern).
pub fn apply_enemy_debuffs(
    _enemy: &Enemy,
    _buffs: &[ResolvedBuff],
    _element: Option<Element>,
) -> Enemy {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    fn base_enemy() -> Enemy {
        Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        }
    }

    fn res_reduction_buff(element: Element, value: f64) -> ResolvedBuff {
        ResolvedBuff {
            source: "Test RES Shred".into(),
            stat: BuffableStat::ElementalResReduction(element),
            value,
            target: BuffTarget::Team,
        }
    }

    #[test]
    fn test_empty_buffs_no_change() {
        let enemy = base_enemy();
        let result = apply_enemy_debuffs(&enemy, &[], Some(Element::Pyro));
        assert!((result.resistance - 0.10).abs() < EPSILON);
        assert!((result.def_reduction - 0.0).abs() < EPSILON);
        assert_eq!(result.level, 90);
    }

    #[test]
    fn test_elemental_res_reduction_matching() {
        let enemy = base_enemy();
        let buffs = vec![res_reduction_buff(Element::Pyro, 0.40)];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        // 0.10 - 0.40 = -0.30
        assert!((result.resistance - (-0.30)).abs() < EPSILON);
    }

    #[test]
    fn test_elemental_res_reduction_non_matching_skipped() {
        let enemy = base_enemy();
        let buffs = vec![res_reduction_buff(Element::Pyro, 0.40)];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Cryo));
        // Pyro shred does not apply to Cryo attack
        assert!((result.resistance - 0.10).abs() < EPSILON);
    }

    #[test]
    fn test_physical_res_reduction() {
        let enemy = base_enemy();
        let buffs = vec![ResolvedBuff {
            source: "Superconduct".into(),
            stat: BuffableStat::PhysicalResReduction,
            value: 0.40,
            target: BuffTarget::Team,
        }];
        // element=None means physical attack
        let result = apply_enemy_debuffs(&enemy, &buffs, None);
        assert!((result.resistance - (-0.30)).abs() < EPSILON);
    }

    #[test]
    fn test_physical_res_reduction_ignored_for_elemental() {
        let enemy = base_enemy();
        let buffs = vec![ResolvedBuff {
            source: "Superconduct".into(),
            stat: BuffableStat::PhysicalResReduction,
            value: 0.40,
            target: BuffTarget::Team,
        }];
        // Physical shred does not apply to elemental attacks
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        assert!((result.resistance - 0.10).abs() < EPSILON);
    }

    #[test]
    fn test_multiple_debuffs_stack_additively() {
        let enemy = base_enemy();
        let buffs = vec![
            res_reduction_buff(Element::Pyro, 0.40),  // VV
            res_reduction_buff(Element::Pyro, 0.20),  // Zhongli
        ];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        // 0.10 - 0.40 - 0.20 = -0.50
        assert!((result.resistance - (-0.50)).abs() < EPSILON);
    }

    #[test]
    fn test_negative_resistance_no_floor() {
        let enemy = base_enemy();
        let buffs = vec![res_reduction_buff(Element::Pyro, 0.80)];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        // 0.10 - 0.80 = -0.70 (no floor)
        assert!((result.resistance - (-0.70)).abs() < EPSILON);
    }

    #[test]
    fn test_def_reduction() {
        let enemy = base_enemy();
        let buffs = vec![ResolvedBuff {
            source: "Lisa A4".into(),
            stat: BuffableStat::DefReduction,
            value: 0.15,
            target: BuffTarget::Team,
        }];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Electro));
        assert!((result.def_reduction - 0.15).abs() < EPSILON);
    }

    #[test]
    fn test_def_reduction_adds_to_existing() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.20,  // pre-existing
        };
        let buffs = vec![ResolvedBuff {
            source: "Lisa A4".into(),
            stat: BuffableStat::DefReduction,
            value: 0.15,
            target: BuffTarget::Team,
        }];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Electro));
        // 0.20 + 0.15 = 0.35
        assert!((result.def_reduction - 0.35).abs() < EPSILON);
    }

    #[test]
    fn test_def_reduction_clamped_at_1() {
        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.80,
        };
        let buffs = vec![ResolvedBuff {
            source: "Test".into(),
            stat: BuffableStat::DefReduction,
            value: 0.50,
            target: BuffTarget::Team,
        }];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Electro));
        // 0.80 + 0.50 = 1.30 → clamped to 1.0
        assert!((result.def_reduction - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_ally_buffs_ignored() {
        let enemy = base_enemy();
        let buffs = vec![
            ResolvedBuff {
                source: "Bennett".into(),
                stat: BuffableStat::AtkFlat,
                value: 1000.0,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "Noblesse".into(),
                stat: BuffableStat::AtkPercent,
                value: 0.20,
                target: BuffTarget::Team,
            },
        ];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        assert!((result.resistance - 0.10).abs() < EPSILON);
        assert!((result.def_reduction - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_physical_attack_zhongli_only_physical_shred() {
        let enemy = base_enemy();
        // Zhongli provides all 8 shred entries
        let buffs = vec![
            res_reduction_buff(Element::Pyro, 0.20),
            res_reduction_buff(Element::Hydro, 0.20),
            res_reduction_buff(Element::Electro, 0.20),
            res_reduction_buff(Element::Cryo, 0.20),
            res_reduction_buff(Element::Dendro, 0.20),
            res_reduction_buff(Element::Anemo, 0.20),
            res_reduction_buff(Element::Geo, 0.20),
            ResolvedBuff {
                source: "Zhongli Shield".into(),
                stat: BuffableStat::PhysicalResReduction,
                value: 0.20,
                target: BuffTarget::Team,
            },
        ];
        // Physical attack (element=None): only PhysicalResReduction applies
        let result = apply_enemy_debuffs(&enemy, &buffs, None);
        // 0.10 - 0.20 = -0.10 (only physical shred)
        assert!((result.resistance - (-0.10)).abs() < EPSILON);
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core -- enemy::tests`
Expected: FAIL with `todo!()` panic

- [ ] **Step 3: Implement `apply_enemy_debuffs`**

Replace the `todo!()` body in `apply_enemy_debuffs`:

```rust
pub fn apply_enemy_debuffs(
    enemy: &Enemy,
    buffs: &[ResolvedBuff],
    element: Option<Element>,
) -> Enemy {
    let mut res_reduction = 0.0;
    let mut def_reduction_add = 0.0;

    for buff in buffs {
        match buff.stat {
            BuffableStat::ElementalResReduction(e) => {
                if element == Some(e) {
                    res_reduction += buff.value;
                }
            }
            BuffableStat::PhysicalResReduction => {
                if element.is_none() {
                    res_reduction += buff.value;
                }
            }
            BuffableStat::DefReduction => {
                def_reduction_add += buff.value;
            }
            _ => {} // ally buffs — ignore
        }
    }

    Enemy {
        level: enemy.level,
        resistance: enemy.resistance - res_reduction,
        def_reduction: f64::min(1.0, enemy.def_reduction + def_reduction_add),
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-core -- enemy::tests`
Expected: All 12 tests PASS

- [ ] **Step 5: Run full core test suite**

Run: `cargo test -p genshin-calc-core`
Expected: All tests PASS

- [ ] **Step 6: Commit**

```bash
git add crates/core/src/enemy.rs
git commit -m "feat(core): implement apply_enemy_debuffs for resistance/DEF reduction"
```

---

### Task 3: Add `superconduct_debuff` helper + golden tests

**Files:**
- Modify: `crates/core/src/enemy.rs` (add helper + golden tests)

- [ ] **Step 1: Write failing tests**

Add to the `tests` module in `crates/core/src/enemy.rs`:

```rust
    #[test]
    fn test_superconduct_debuff_values() {
        let debuff = superconduct_debuff();
        assert_eq!(debuff.stat, BuffableStat::PhysicalResReduction);
        assert!((debuff.value - 0.40).abs() < EPSILON);
    }

    // Golden tests: hand-calculated resistance multiplier after shred
    #[test]
    fn test_golden_res_10_shred_20() {
        // 10% base - 20% shred = -10% → multiplier = 1 - (-0.10)/2 = 1.05
        let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
        let buffs = vec![res_reduction_buff(Element::Pyro, 0.20)];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        assert!((result.resistance - (-0.10)).abs() < EPSILON);
        let mult = crate::damage::resistance_multiplier(&result);
        assert!((mult - 1.05).abs() < EPSILON);
    }

    #[test]
    fn test_golden_res_10_shred_60() {
        // 10% base - 60% shred = -50% → multiplier = 1 - (-0.50)/2 = 1.25
        let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
        let buffs = vec![
            res_reduction_buff(Element::Pyro, 0.40),
            res_reduction_buff(Element::Pyro, 0.20),
        ];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        assert!((result.resistance - (-0.50)).abs() < EPSILON);
        let mult = crate::damage::resistance_multiplier(&result);
        assert!((mult - 1.25).abs() < EPSILON);
    }

    #[test]
    fn test_golden_res_70_shred_40() {
        // 70% base - 40% shred = 30% → multiplier = 1 - 0.30 = 0.70
        let enemy = Enemy { level: 90, resistance: 0.70, def_reduction: 0.0 };
        let buffs = vec![res_reduction_buff(Element::Pyro, 0.40)];
        let result = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));
        assert!((result.resistance - 0.30).abs() < EPSILON);
        let mult = crate::damage::resistance_multiplier(&result);
        assert!((mult - 0.70).abs() < EPSILON);
    }
```

- [ ] **Step 2: Add `superconduct_debuff` stub**

Add above the `#[cfg(test)]` block in `crates/core/src/enemy.rs`:

```rust
/// Creates a ResolvedBuff for Superconduct's physical resistance reduction (-40%).
///
/// Consumer adds this to the buff list when Superconduct reaction is active.
pub fn superconduct_debuff() -> ResolvedBuff {
    ResolvedBuff {
        source: "Superconduct".to_string(),
        stat: BuffableStat::PhysicalResReduction,
        value: 0.40,
        target: BuffTarget::Team,
    }
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-core -- enemy::tests`
Expected: All 16 tests PASS (12 previous + 4 new)

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/enemy.rs
git commit -m "feat(core): add superconduct_debuff helper and golden resistance tests"
```

---

### Task 4: Re-export new functions from `lib.rs`

**Files:**
- Modify: `crates/core/src/lib.rs:106` (update re-exports)

- [ ] **Step 1: Update re-exports**

In `crates/core/src/lib.rs`, change:

```rust
pub use enemy::Enemy;
```

to:

```rust
pub use enemy::{Enemy, apply_enemy_debuffs, superconduct_debuff};
```

- [ ] **Step 2: Verify build**

Run: `cargo build -p genshin-calc-core`
Expected: PASS

- [ ] **Step 3: Run all tests**

Run: `cargo test -p genshin-calc-core && cargo test -p genshin-calc-data`
Expected: All PASS

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/lib.rs
git commit -m "feat(core): re-export apply_enemy_debuffs and superconduct_debuff"
```

---

### Task 5: Add Zhongli talent debuffs

**Files:**
- Modify: `crates/data/src/talent_buffs.rs` (add Zhongli entries + update ALL_TALENT_BUFFS)

- [ ] **Step 1: Write failing test**

Add to the `tests` module in `crates/data/src/talent_buffs.rs`:

```rust
    #[test]
    fn test_find_zhongli_debuffs() {
        let buffs = find_talent_buffs("zhongli").unwrap();
        assert_eq!(buffs.len(), 8); // 7 elemental + 1 physical
        // All values are 0.20
        for b in buffs {
            assert!((b.base_value - 0.20).abs() < 1e-6);
            assert_eq!(b.target, BuffTarget::Team);
        }
        // Check specific elements
        assert!(buffs.iter().any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Pyro)));
        assert!(buffs.iter().any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Hydro)));
        assert!(buffs.iter().any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Electro)));
        assert!(buffs.iter().any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Cryo)));
        assert!(buffs.iter().any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Dendro)));
        assert!(buffs.iter().any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Anemo)));
        assert!(buffs.iter().any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Geo)));
        assert!(buffs.iter().any(|b| b.stat == BuffableStat::PhysicalResReduction));
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data -- tests::test_find_zhongli_debuffs`
Expected: FAIL (`find_talent_buffs("zhongli")` returns `None`)

- [ ] **Step 3: Add Zhongli buff data**

Add before `// ===== Jahoda =====` in `crates/data/src/talent_buffs.rs`:

```rust
// ===== Zhongli =====
// Jade Shield "Dominus Lapidis": All RES -20% for nearby enemies while shield is active
static ZHONGLI_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Jade Shield Pyro RES Shred",
        description: "Nearby enemies' Pyro RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Hydro RES Shred",
        description: "Nearby enemies' Hydro RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Hydro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Electro RES Shred",
        description: "Nearby enemies' Electro RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Electro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Cryo RES Shred",
        description: "Nearby enemies' Cryo RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Cryo),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Dendro RES Shred",
        description: "Nearby enemies' Dendro RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Dendro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Anemo RES Shred",
        description: "Nearby enemies' Anemo RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Anemo),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Geo RES Shred",
        description: "Nearby enemies' Geo RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Geo),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Physical RES Shred",
        description: "Nearby enemies' Physical RES -20%",
        stat: BuffableStat::PhysicalResReduction,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
];
```

Update `ALL_TALENT_BUFFS` — replace the comment:

```rust
    // Zhongli provides resistance shred (Enemy-side), not a stat buff — excluded here
```

with:

```rust
    ("zhongli", ZHONGLI_BUFFS),
```

- [ ] **Step 4: Update `test_find_nonexistent_character`**

Change the Zhongli assertion in the existing test:

```rust
    #[test]
    fn test_find_nonexistent_character() {
        assert!(find_talent_buffs("diluc").is_none());
        assert!(find_talent_buffs("unknown").is_none());
    }
```

- [ ] **Step 5: Run tests**

Run: `cargo test -p genshin-calc-data -- tests::test_find_zhongli`
Expected: PASS

Run: `cargo test -p genshin-calc-data`
Expected: All PASS

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/talent_buffs.rs
git commit -m "feat(data): add Zhongli jade shield resistance shred talent buffs"
```

---

### Task 6: Add Lisa, Chevreuse, Faruzan talent debuffs

**Files:**
- Modify: `crates/data/src/talent_buffs.rs` (add 3 characters)

- [ ] **Step 1: Write failing tests**

Add to `tests` module:

```rust
    #[test]
    fn test_find_lisa_debuffs() {
        let buffs = find_talent_buffs("lisa").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::DefReduction);
        assert!((buffs[0].base_value - 0.15).abs() < 1e-6);
    }

    #[test]
    fn test_find_chevreuse_debuffs() {
        let buffs = find_talent_buffs("chevreuse").unwrap();
        // Existing ATK+20% + 2 new res shreds = 3
        assert_eq!(buffs.len(), 3);
        let pyro_shred = buffs.iter().find(|b| b.stat == BuffableStat::ElementalResReduction(Element::Pyro));
        assert!(pyro_shred.is_some());
        assert!((pyro_shred.unwrap().base_value - 0.40).abs() < 1e-6);
        let electro_shred = buffs.iter().find(|b| b.stat == BuffableStat::ElementalResReduction(Element::Electro));
        assert!(electro_shred.is_some());
        assert!((electro_shred.unwrap().base_value - 0.40).abs() < 1e-6);
    }

    #[test]
    fn test_find_faruzan_res_shred() {
        let buffs = find_talent_buffs("faruzan").unwrap();
        // Existing Anemo DMG + new Anemo RES shred = 2
        assert_eq!(buffs.len(), 2);
        let shred = buffs.iter().find(|b| b.stat == BuffableStat::ElementalResReduction(Element::Anemo));
        assert!(shred.is_some());
        assert!((shred.unwrap().base_value - 0.30).abs() < 1e-6);
        assert!(!shred.unwrap().scales_with_talent);
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data -- tests::test_find_lisa_debuffs tests::test_find_chevreuse_debuffs tests::test_find_faruzan_res_shred`
Expected: FAIL

- [ ] **Step 3: Add Lisa talent buff data**

Add before `// ===== Zhongli =====`:

```rust
// ===== Lisa =====
// A4 "Static Electricity Field": Enemies hit by burst have DEF -15% for 10s
static LISA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Static Electricity Field",
    description: "Enemies hit by Lightning Rose have DEF -15% for 10s",
    stat: BuffableStat::DefReduction,
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];
```

Add `("lisa", LISA_BUFFS),` to `ALL_TALENT_BUFFS`.

- [ ] **Step 4: Update Chevreuse buff array**

Replace the existing `CHEVREUSE_BUFFS` static:

```rust
static CHEVREUSE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Vanguard's Coordinated Tactics",
        description: "After Overloaded, ATK+20% for party (Pyro+Electro teams only, approximation)",
        stat: BuffableStat::AtkPercent,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Overloaded Pyro RES Shred",
        description: "After Overloaded reaction, enemy Pyro RES -40% for 6s",
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Overloaded Electro RES Shred",
        description: "After Overloaded reaction, enemy Electro RES -40% for 6s",
        stat: BuffableStat::ElementalResReduction(Element::Electro),
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
];
```

- [ ] **Step 5: Update Faruzan buff array**

Replace the existing `FARUZAN_BUFFS` static:

```rust
static FARUZAN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Prayerful Wind's Benefit",
        description: "Anemo DMG Bonus based on burst talent level",
        stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&FARUZAN_BURST_ANEMO_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Perfidious Wind's Bale",
        description: "A4: Enemies hit by Pressurized Collapse have Anemo RES -30%",
        stat: BuffableStat::ElementalResReduction(Element::Anemo),
        base_value: 0.30,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
];
```

- [ ] **Step 6: Update existing `test_find_chevreuse_buffs` test**

The existing test asserts `buffs.len() == 1`. Update to expect 3:

```rust
    #[test]
    fn test_find_chevreuse_buffs() {
        let buffs = find_talent_buffs("chevreuse").unwrap();
        assert_eq!(buffs.len(), 3);
        assert_eq!(buffs[0].stat, BuffableStat::AtkPercent);
        assert!((buffs[0].base_value - 0.20).abs() < 1e-6);
    }
```

- [ ] **Step 7: Update existing `test_find_faruzan_buffs` test**

The existing test asserts `buffs.len() == 1`. Update to expect 2:

```rust
    #[test]
    fn test_find_faruzan_buffs() {
        let buffs = find_talent_buffs("faruzan").unwrap();
        assert_eq!(buffs.len(), 2);
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalDmgBonus(Element::Anemo)
        );
        assert!(buffs[0].scales_with_talent);
        assert!(buffs[0].talent_scaling.is_some());
        assert_eq!(buffs[0].source, TalentBuffSource::ElementalBurst);
    }
```

- [ ] **Step 8: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All PASS

- [ ] **Step 9: Commit**

```bash
git add crates/data/src/talent_buffs.rs
git commit -m "feat(data): add Lisa DEF shred, Chevreuse/Faruzan RES shred talent debuffs"
```

---

### Task 7: Integration test — debuffs applied to damage calculation

**Files:**
- Modify: `crates/core/src/enemy.rs` (add integration-style test in unit test module)

- [ ] **Step 1: Write integration test**

Add to `tests` module in `crates/core/src/enemy.rs`:

```rust
    #[test]
    fn test_integration_vv_zhongli_stacked_damage() {
        // Simulates: VV (-40%) + Zhongli (-20%) on 10% RES enemy
        // Expected: 0.10 - 0.40 - 0.20 = -0.50 → multiplier 1.25
        use crate::damage::{DamageInput, calculate_damage, resistance_multiplier};
        use crate::stats::Stats;
        use crate::types::{DamageType, ScalingStat};

        let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
        let buffs = vec![
            res_reduction_buff(Element::Pyro, 0.40),  // VV
            res_reduction_buff(Element::Pyro, 0.20),  // Zhongli
        ];
        let debuffed = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Pyro));

        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.0,
                crit_dmg: 0.0,
                dmg_bonus: 0.0,
                ..Default::default()
            },
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
        };

        // Calculate with and without debuffs
        let result_no_debuff = calculate_damage(&input, &enemy).unwrap();
        let result_debuffed = calculate_damage(&input, &debuffed).unwrap();

        // Debuffed damage should be higher
        assert!(result_debuffed.non_crit > result_no_debuff.non_crit);

        // Verify multiplier ratio: 1.25 / 0.90 = 1.3888...
        let ratio = result_debuffed.non_crit / result_no_debuff.non_crit;
        let expected_ratio = 1.25 / 0.90;
        assert!((ratio - expected_ratio).abs() < 0.001);
    }

    #[test]
    fn test_integration_superconduct_physical() {
        use crate::damage::{DamageInput, calculate_damage};
        use crate::stats::Stats;
        use crate::types::{DamageType, ScalingStat};

        let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
        let buffs = vec![superconduct_debuff()]; // -40% phys
        let debuffed = apply_enemy_debuffs(&enemy, &buffs, None); // physical

        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.0,
                crit_dmg: 0.0,
                dmg_bonus: 0.0,
                ..Default::default()
            },
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: None, // physical
            reaction: None,
            reaction_bonus: 0.0,
        };

        let result_no_debuff = calculate_damage(&input, &enemy).unwrap();
        let result_debuffed = calculate_damage(&input, &debuffed).unwrap();

        // 0.10 - 0.40 = -0.30 → mult 1.15 vs original 0.90
        let ratio = result_debuffed.non_crit / result_no_debuff.non_crit;
        let expected_ratio = 1.15 / 0.90;
        assert!((ratio - expected_ratio).abs() < 0.001);
    }

    #[test]
    fn test_integration_lisa_def_reduction() {
        use crate::damage::{DamageInput, calculate_damage};
        use crate::stats::Stats;
        use crate::types::{DamageType, ScalingStat};

        let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
        let buffs = vec![ResolvedBuff {
            source: "Lisa A4".into(),
            stat: BuffableStat::DefReduction,
            value: 0.15,
            target: BuffTarget::Team,
        }];
        let debuffed = apply_enemy_debuffs(&enemy, &buffs, Some(Element::Electro));

        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.0,
                crit_dmg: 0.0,
                dmg_bonus: 0.0,
                ..Default::default()
            },
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: Some(Element::Electro),
            reaction: None,
            reaction_bonus: 0.0,
        };

        let result_no_debuff = calculate_damage(&input, &enemy).unwrap();
        let result_debuffed = calculate_damage(&input, &debuffed).unwrap();

        // DEF reduction makes damage higher
        assert!(result_debuffed.non_crit > result_no_debuff.non_crit);

        // def_mult_debuffed = 190 / (190 + 190*(1-0.15)) = 190 / 351.5
        // def_mult_original = 190 / (190 + 190) = 0.5
        // resistance unchanged at 0.90 both cases
        let def_mult_debuffed = 190.0 / (190.0 + 190.0 * 0.85);
        let def_mult_original = 0.5;
        let expected_ratio = def_mult_debuffed / def_mult_original;
        let ratio = result_debuffed.non_crit / result_no_debuff.non_crit;
        assert!((ratio - expected_ratio).abs() < 0.001);
    }
```

- [ ] **Step 2: Run tests**

Run: `cargo test -p genshin-calc-core -- enemy::tests`
Expected: All PASS (16 unit + 3 integration = 19)

- [ ] **Step 3: Run full test suite**

Run: `cargo test -p genshin-calc-core && cargo test -p genshin-calc-data`
Expected: All PASS

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/enemy.rs
git commit -m "test(core): add integration tests for enemy debuff damage calculations"
```

---

### Task 8: Update docs and TODO

**Files:**
- Modify: `docs/data-expansion-todo.md` (mark P6 items complete)
- Modify: `CLAUDE.md` (update test counts and talent buff count)

- [ ] **Step 1: Update data-expansion-todo.md P6 section**

Mark all P6 items as done:

```markdown
- [x] Consumer側実装: ResolvedBuff の ElementalResReduction/PhysicalResReduction を Enemy.resistance に減算
- [x] 超電導 — 物理耐性-40%
- [x] Zhongli シールド — 全耐性-20%
- [x] Chevreuse — 炎/雷耐性-40%
- [x] Lisa A4 — DEF-15%
- [x] Faruzan — 風耐性シュレッド
```

- [ ] **Step 2: Update CLAUDE.md**

Update test counts (verify actual counts after all tests pass with `cargo test -p genshin-calc-core -- --list 2>/dev/null | grep -c ": test$"` and same for data crate).

Update talent buff count: `27キャラ・35定義` → new count (add Zhongli, Lisa = 29キャラ, 35+8+1 = 44定義, plus Chevreuse+2, Faruzan+1 = 47定義).

Update enemies line if needed.

- [ ] **Step 3: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

- [ ] **Step 4: Run fmt check**

Run: `cargo fmt --check`
Expected: No formatting issues

- [ ] **Step 5: Commit**

```bash
git add docs/data-expansion-todo.md CLAUDE.md
git commit -m "docs: mark P6 enemy debuff system complete, update test counts"
```
