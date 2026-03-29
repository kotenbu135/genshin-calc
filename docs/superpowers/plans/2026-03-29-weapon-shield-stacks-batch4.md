# P3 Batch 4: Shield Series Weapon Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Stacks+Shield ConditionalBuff definitions to 4 Liyue Shield Series weapons.

**Architecture:** Data-only changes — no core crate modifications. Each weapon gets 2 identical `ManualCondition::Stacks(5)` ConditionalBuffs (normal + shield bonus). All 4 weapons share the same R1-R5 values.

**Tech Stack:** Rust, Cargo workspace (`crates/data`)

**Spec:** `docs/superpowers/specs/2026-03-29-weapon-shield-stacks-batch4-design.md`

---

## File Structure

| File | Action | Responsibility |
|------|--------|---------------|
| `crates/data/src/weapons/sword.rs` | Modify | Summit Shaper + test |
| `crates/data/src/weapons/claymore.rs` | Modify | The Unforged + test |
| `crates/data/src/weapons/polearm.rs` | Modify | Vortex Vanquisher + test |
| `crates/data/src/weapons/catalyst.rs` | Modify | Memory of Dust + test |

---

### Task 1: Summit Shaper (Sword)

**Files:**
- Modify: `crates/data/src/weapons/sword.rs:355-370` (weapon data)
- Modify: `crates/data/src/weapons/sword.rs` (test module)

- [ ] **Step 1: Write the failing test**

Add to the existing `mod tests` block in `crates/data/src/weapons/sword.rs`:

```rust
    #[test]
    fn summit_shaper_has_atk_stacks_and_shield_stacks() {
        let passive = SUMMIT_SHAPER.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "summit_shaper_atk_stacks");
        assert_eq!(cond_buffs[1].name, "summit_shaper_shield_atk_stacks");

        for buff in cond_buffs {
            assert_eq!(buff.stat, BuffableStat::AtkPercent);
            assert!((buff.value - 0.04).abs() < 1e-6);
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Stacks(5))
            ));
            assert!(buff.refinement_values.is_some());
        }
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data summit_shaper_has -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuffs**

Change:

```rust
            description: "Conditional: シールド強化+20-40%。攻撃命中でATK+4-8%、シールド時は2倍",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "攻撃命中でATK+4-8%スタック（最大5）、シールド時は2倍",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "summit_shaper_atk_stacks",
                    description: "攻撃命中でATK+4-8%（1スタック）、最大5スタック",
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
                ConditionalBuff {
                    name: "summit_shaper_shield_atk_stacks",
                    description: "シールド時にATKスタック効果2倍分（追加ATK+4-8%/スタック）",
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
            ],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data summit_shaper_has -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/sword.rs
git commit -m "feat: add Summit Shaper shield stacks ConditionalBuff (P3 Batch 4)"
```

---

### Task 2: The Unforged (Claymore)

**Files:**
- Modify: `crates/data/src/weapons/claymore.rs:184-199` (weapon data)
- Modify: `crates/data/src/weapons/claymore.rs` (test module)

- [ ] **Step 1: Write the failing test**

Add to `mod tests` in `crates/data/src/weapons/claymore.rs`:

```rust
    #[test]
    fn the_unforged_has_atk_stacks_and_shield_stacks() {
        let passive = THE_UNFORGED.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "the_unforged_atk_stacks");
        assert_eq!(cond_buffs[1].name, "the_unforged_shield_atk_stacks");

        for buff in cond_buffs {
            assert_eq!(buff.stat, BuffableStat::AtkPercent);
            assert!((buff.value - 0.04).abs() < 1e-6);
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Stacks(5))
            ));
            assert!(buff.refinement_values.is_some());
        }
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data the_unforged_has -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuffs**

Change:

```rust
            description: "Conditional: シールド強化+20-40%。命中時ATKアップ、シールド時さらにATKアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "攻撃命中でATK+4-8%スタック（最大5）、シールド時は2倍",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "the_unforged_atk_stacks",
                    description: "攻撃命中でATK+4-8%（1スタック）、最大5スタック",
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
                ConditionalBuff {
                    name: "the_unforged_shield_atk_stacks",
                    description: "シールド時にATKスタック効果2倍分（追加ATK+4-8%/スタック）",
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
            ],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data the_unforged_has -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/claymore.rs
git commit -m "feat: add The Unforged shield stacks ConditionalBuff (P3 Batch 4)"
```

---

### Task 3: Vortex Vanquisher (Polearm)

**Files:**
- Modify: `crates/data/src/weapons/polearm.rs:272-287` (weapon data)
- Modify: `crates/data/src/weapons/polearm.rs` (test module)

- [ ] **Step 1: Write the failing test**

Add to `mod tests` in `crates/data/src/weapons/polearm.rs`:

```rust
    #[test]
    fn vortex_vanquisher_has_atk_stacks_and_shield_stacks() {
        let passive = VORTEX_VANQUISHER.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "vortex_vanquisher_atk_stacks");
        assert_eq!(cond_buffs[1].name, "vortex_vanquisher_shield_atk_stacks");

        for buff in cond_buffs {
            assert_eq!(buff.stat, BuffableStat::AtkPercent);
            assert!((buff.value - 0.04).abs() < 1e-6);
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Stacks(5))
            ));
            assert!(buff.refinement_values.is_some());
        }
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data vortex_vanquisher_has -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuffs**

Change:

```rust
            description: "Conditional: シールド強化+20-40%。攻撃命中でATKアップ、シールド時は2倍",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "攻撃命中でATK+4-8%スタック（最大5）、シールド時は2倍",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "vortex_vanquisher_atk_stacks",
                    description: "攻撃命中でATK+4-8%（1スタック）、最大5スタック",
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
                ConditionalBuff {
                    name: "vortex_vanquisher_shield_atk_stacks",
                    description: "シールド時にATKスタック効果2倍分（追加ATK+4-8%/スタック）",
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
            ],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data vortex_vanquisher_has -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/polearm.rs
git commit -m "feat: add Vortex Vanquisher shield stacks ConditionalBuff (P3 Batch 4)"
```

---

### Task 4: Memory of Dust (Catalyst)

**Files:**
- Modify: `crates/data/src/weapons/catalyst.rs:157-175` (weapon data)
- Modify: `crates/data/src/weapons/catalyst.rs` (test module — may already exist from Batch 2/3)

- [ ] **Step 1: Write the failing test**

Add to `mod tests` in `crates/data/src/weapons/catalyst.rs`:

```rust
    #[test]
    fn memory_of_dust_has_atk_stacks_and_shield_stacks() {
        let passive = MEMORY_OF_DUST.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "memory_of_dust_atk_stacks");
        assert_eq!(cond_buffs[1].name, "memory_of_dust_shield_atk_stacks");

        for buff in cond_buffs {
            assert_eq!(buff.stat, BuffableStat::AtkPercent);
            assert!((buff.value - 0.04).abs() < 1e-6);
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Stacks(5))
            ));
            assert!(buff.refinement_values.is_some());
        }
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data memory_of_dust_has -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuffs**

Change:

```rust
            description: "Conditional: 命中時にATK%スタック、シールド時に効果2倍",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "攻撃命中でATK+4-8%スタック（最大5）、シールド時は2倍",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "memory_of_dust_atk_stacks",
                    description: "攻撃命中でATK+4-8%（1スタック）、最大5スタック",
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
                ConditionalBuff {
                    name: "memory_of_dust_shield_atk_stacks",
                    description: "シールド時にATKスタック効果2倍分（追加ATK+4-8%/スタック）",
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
            ],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data memory_of_dust_has -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/catalyst.rs
git commit -m "feat: add Memory of Dust shield stacks ConditionalBuff (P3 Batch 4)"
```

---

### Task 5: Full Test Suite & Verification

- [ ] **Step 1: Run full data test suite**

Run: `cargo test -p genshin-calc-data`
Expected: ALL PASS.

- [ ] **Step 2: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings.

- [ ] **Step 3: Run fmt check**

Run: `cargo fmt --check`
Expected: No formatting issues.

- [ ] **Step 4: Run full workspace test**

Run: `cargo test`
Expected: ALL PASS.

- [ ] **Step 5: Update data-expansion-todo.md**

Update weapon ConditionalBuff count to reflect 4 additional weapons.

- [ ] **Step 6: Commit**

```bash
git add docs/data-expansion-todo.md
git commit -m "docs: update weapon ConditionalBuff coverage (P3 Batch 4)"
```
