# P3 Batch 2: 5-Star Weapon Stacks Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `ManualCondition::Stacks` ConditionalBuff definitions to 9 five-star weapons.

**Architecture:** Data-only changes — no core crate modifications. Each weapon gets 1-2 ConditionalBuffs using existing `ManualCondition::Stacks(N)` and optional `ManualCondition::Toggle`. Tests verify buff existence, stat type, value, and activation type.

**Tech Stack:** Rust, Cargo workspace (`crates/data`)

**Spec:** `docs/superpowers/specs/2026-03-29-weapon-stacks-batch2-design.md`

---

## File Structure

| File | Action | Responsibility |
|------|--------|---------------|
| `crates/data/src/weapons/polearm.rs` | Modify | PJWS ConditionalBuffs + test |
| `crates/data/src/weapons/claymore.rs` | Modify | Fang of Mountain King ConditionalBuff + test |
| `crates/data/src/weapons/bow.rs` | Modify | Astral Vulture, Silvershower, Daybreak ConditionalBuffs + tests |
| `crates/data/src/weapons/catalyst.rs` | Modify | Kagura, Tulaytullah, Nocturne, Vivid Notions ConditionalBuffs + tests (create test module) |

---

### Task 1: Primordial Jade Winged-Spear (Polearm, Pattern B)

**Files:**
- Modify: `crates/data/src/weapons/polearm.rs:134-149` (PJWS weapon data)
- Modify: `crates/data/src/weapons/polearm.rs:856-898` (test module)

- [ ] **Step 1: Write the failing test**

Add to the existing `mod tests` block in `crates/data/src/weapons/polearm.rs` (after line 897, before the closing `}`):

```rust
    #[test]
    fn pjws_has_atk_stacks_and_full_stack_dmg() {
        let passive = PRIMORDIAL_JADE_WINGED_SPEAR.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        // Stacks buff: ATK% per stack, max 6
        let stacks_buff = &cond_buffs[0];
        assert_eq!(stacks_buff.name, "pjws_atk_stacks");
        assert_eq!(stacks_buff.stat, BuffableStat::AtkPercent);
        assert!((stacks_buff.value - 0.032).abs() < 1e-6);
        assert!(matches!(
            stacks_buff.activation,
            Activation::Manual(ManualCondition::Stacks(6))
        ));
        assert!(stacks_buff.refinement_values.is_some());

        // Full-stack bonus: DMG%
        let full_buff = &cond_buffs[1];
        assert_eq!(full_buff.name, "pjws_full_stack_dmg");
        assert_eq!(full_buff.stat, BuffableStat::DmgBonus);
        assert!((full_buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            full_buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data pjws_has_atk_stacks -- --nocapture`
Expected: FAIL (conditional_buffs is empty, `cond_buffs.len()` is 0)

- [ ] **Step 3: Implement the ConditionalBuffs**

In `crates/data/src/weapons/polearm.rs`, replace the PJWS passive section. Change:

```rust
            description: "Conditional: 命中時にATK+3.2-6%、6スタックまで。フルスタックでDMG+12-24%",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "命中時にATK+3.2-6%、6スタックまで。フルスタックでDMG+12-24%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "pjws_atk_stacks",
                    description: "命中時にATK+3.2-6%（1スタック）、最大6スタック",
                    stat: BuffableStat::AtkPercent,
                    value: 0.032,
                    refinement_values: Some([0.032, 0.039, 0.046, 0.053, 0.060]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(6)),
                },
                ConditionalBuff {
                    name: "pjws_full_stack_dmg",
                    description: "フルスタック時にDMG+12-24%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
```

Note: Add `ManualCondition` to the imports at top of file if not already present. The existing imports line is:
```rust
use crate::buff::{
    Activation, AutoCondition, BuffTarget, BuffableStat, ConditionalBuff, ManualCondition,
    PassiveEffect, StatBuff,
};
```
If `ManualCondition` is already imported, no change needed. Verify by reading line 1-6.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data pjws_has_atk_stacks -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/polearm.rs
git commit -m "feat: add PJWS stacks ConditionalBuff (P3 Batch 2)"
```

---

### Task 2: Fang of the Mountain King (Claymore, Pattern A)

**Files:**
- Modify: `crates/data/src/weapons/claymore.rs:56-71` (weapon data)
- Modify: `crates/data/src/weapons/claymore.rs:886-913` (test module)

- [ ] **Step 1: Write the failing test**

Add to the existing `mod tests` block in `crates/data/src/weapons/claymore.rs`:

```rust
    #[test]
    fn fang_of_mountain_king_has_dmg_stacks() {
        let passive = FANG_OF_THE_MOUNTAIN_KING.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "fang_mountain_king_elemental_dmg_stacks");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.10).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        assert!(buff.refinement_values.is_some());
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data fang_of_mountain_king -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuff**

In `crates/data/src/weapons/claymore.rs`, replace the Fang passive section. Change:

```rust
            description: "Conditional: 元素スキル命中でスタック獲得、スタック数に応じて元素ダメージアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "元素スキル命中でDMG+10-20%スタック（最大3スタック）。DmgBonus近似値（実際は全元素DMG、物理除外）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "fang_mountain_king_elemental_dmg_stacks",
                description: "元素スキル命中でDMG+10-20%（1スタック）、最大3スタック（DmgBonus近似値、物理除外）",
                stat: BuffableStat::DmgBonus,
                value: 0.10,
                refinement_values: Some([0.10, 0.125, 0.15, 0.175, 0.20]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
```

Note: Ensure `ManualCondition` is in the imports. Existing imports at top of claymore.rs likely include it (used by serpent_spine and whiteblind). Verify.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data fang_of_mountain_king -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/claymore.rs
git commit -m "feat: add Fang of Mountain King stacks ConditionalBuff (P3 Batch 2)"
```

---

### Task 3: Astral Vulture's Crimson Plumage (Bow, Pattern B)

**Files:**
- Modify: `crates/data/src/weapons/bow.rs:69-84` (weapon data)
- Modify: `crates/data/src/weapons/bow.rs:928-951` (test module)

- [ ] **Step 1: Write the failing test**

Add to the existing `mod tests` block in `crates/data/src/weapons/bow.rs`:

```rust
    #[test]
    fn astral_vulture_has_atk_stacks_and_full_stack_dmg() {
        let passive = ASTRAL_VULTURES_CRIMSON_PLUMAGE.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        let stacks_buff = &cond_buffs[0];
        assert_eq!(stacks_buff.name, "astral_vulture_atk_stacks");
        assert_eq!(stacks_buff.stat, BuffableStat::AtkPercent);
        assert!((stacks_buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            stacks_buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));

        let full_buff = &cond_buffs[1];
        assert_eq!(full_buff.name, "astral_vulture_full_stack_dmg");
        assert_eq!(full_buff.stat, BuffableStat::DmgBonus);
        assert!((full_buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            full_buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data astral_vulture -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuffs**

In `crates/data/src/weapons/bow.rs`, replace the Astral Vulture passive section. Change:

```rust
            description: "Conditional: 夜魂のスタック蓄積でATKとDMGアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "夜魂スタック蓄積でATK+12-24%（最大3スタック）、フルスタックでDMG+12-24%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "astral_vulture_atk_stacks",
                    description: "夜魂スタックでATK+12-24%（1スタック）、最大3スタック",
                    stat: BuffableStat::AtkPercent,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(3)),
                },
                ConditionalBuff {
                    name: "astral_vulture_full_stack_dmg",
                    description: "フルスタック時にDMG+12-24%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
```

Note: Add `ManualCondition` to bow.rs imports if not already present. Current imports are:
```rust
use crate::buff::{
    Activation, AutoCondition, BuffTarget, BuffableStat, ConditionalBuff, ManualCondition,
    PassiveEffect, StatBuff,
};
```
If `ManualCondition` is missing, add it.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data astral_vulture -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/bow.rs
git commit -m "feat: add Astral Vulture stacks ConditionalBuff (P3 Batch 2)"
```

---

### Task 4: Silvershower Heartstrings (Bow, Pattern B)

**Files:**
- Modify: `crates/data/src/weapons/bow.rs:165-180` (weapon data)
- Modify: `crates/data/src/weapons/bow.rs` (test module)

- [ ] **Step 1: Write the failing test**

Add to the existing `mod tests` block in `crates/data/src/weapons/bow.rs`:

```rust
    #[test]
    fn silvershower_has_hp_stacks_and_hydro_dmg() {
        let passive = SILVERSHOWER_HEARTSTRINGS.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        let stacks_buff = &cond_buffs[0];
        assert_eq!(stacks_buff.name, "silvershower_hp_stacks");
        assert_eq!(stacks_buff.stat, BuffableStat::HpPercent);
        assert!((stacks_buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            stacks_buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));

        let full_buff = &cond_buffs[1];
        assert_eq!(full_buff.name, "silvershower_full_stack_hydro_dmg");
        assert_eq!(
            full_buff.stat,
            BuffableStat::ElementalDmgBonus(genshin_calc_core::Element::Hydro)
        );
        assert!((full_buff.value - 0.28).abs() < 1e-6);
        assert!(matches!(
            full_buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data silvershower_has_hp -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuffs**

In `crates/data/src/weapons/bow.rs`, replace the Silvershower passive section. Change:

```rust
            description: "Conditional: 元素スキル使用でHP上限アップ。3スタックでHydro DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "元素スキル使用でHP+12-24%スタック（最大3）、3スタックでHydro DMG+28-56%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "silvershower_hp_stacks",
                    description: "元素スキル使用でHP+12-24%（1スタック）、最大3スタック",
                    stat: BuffableStat::HpPercent,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(3)),
                },
                ConditionalBuff {
                    name: "silvershower_full_stack_hydro_dmg",
                    description: "3スタック時にHydro DMG+28-56%",
                    stat: BuffableStat::ElementalDmgBonus(genshin_calc_core::Element::Hydro),
                    value: 0.28,
                    refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
```

Note: `Element` is NOT imported in `bow.rs`. The implementation uses the fully qualified path `genshin_calc_core::Element::Hydro`. Alternatively, add `use genshin_calc_core::Element;` at the top of the file.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data silvershower_has_hp -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/bow.rs
git commit -m "feat: add Silvershower Heartstrings stacks ConditionalBuff (P3 Batch 2)"
```

---

### Task 5: The Daybreak Chronicles (Bow, Pattern A)

**Files:**
- Modify: `crates/data/src/weapons/bow.rs:203-218` (weapon data)
- Modify: `crates/data/src/weapons/bow.rs` (test module)

- [ ] **Step 1: Write the failing test**

Add to the existing `mod tests` block in `crates/data/src/weapons/bow.rs`:

```rust
    #[test]
    fn daybreak_chronicles_has_dmg_stacks() {
        let passive = THE_DAYBREAK_CHRONICLES.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "the_daybreak_chronicles_dmg_stacks");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        assert!(buff.refinement_values.is_some());
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data daybreak_chronicles -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuff**

In `crates/data/src/weapons/bow.rs`, replace the Daybreak Chronicles passive section. Change:

```rust
            description: "Conditional: 物語のスタック蓄積でDMGアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "物語のスタック蓄積でDMG+8-16%（最大3スタック）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "the_daybreak_chronicles_dmg_stacks",
                description: "物語のスタックでDMG+8-16%（1スタック）、最大3スタック",
                stat: BuffableStat::DmgBonus,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data daybreak_chronicles -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/bow.rs
git commit -m "feat: add Daybreak Chronicles stacks ConditionalBuff (P3 Batch 2)"
```

---

### Task 6: Kagura's Verity (Catalyst, Pattern B)

**Files:**
- Modify: `crates/data/src/weapons/catalyst.rs:114-129` (weapon data)
- Modify: `crates/data/src/weapons/catalyst.rs` (create test module at end of file)

**Important:** `catalyst.rs` currently has NO test module. You must create one.

- [ ] **Step 1: Write the failing test**

Add a new test module at the very end of `crates/data/src/weapons/catalyst.rs` (after the `ALL_CATALYSTS` array, after the closing `];`):

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kagura_has_skill_dmg_stacks_and_full_stack_bonus() {
        let passive = KAGURAS_VERITY.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        let stacks_buff = &cond_buffs[0];
        assert_eq!(stacks_buff.name, "kagura_skill_dmg_stacks");
        assert_eq!(stacks_buff.stat, BuffableStat::SkillDmgBonus);
        assert!((stacks_buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            stacks_buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));

        let full_buff = &cond_buffs[1];
        assert_eq!(full_buff.name, "kagura_full_stack_elemental_dmg");
        assert_eq!(full_buff.stat, BuffableStat::DmgBonus);
        assert!((full_buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            full_buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data kagura_has_skill -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuffs**

In `crates/data/src/weapons/catalyst.rs`, replace the Kagura passive section. Change:

```rust
            description: "Conditional: 元素スキル使用でスキルDMGスタック、3スタックで元素DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "元素スキル使用でスキルDMG+12-24%スタック（最大3）、3スタックで元素DMG+12-24%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "kagura_skill_dmg_stacks",
                    description: "元素スキル使用でスキルDMG+12-24%（1スタック）、最大3スタック",
                    stat: BuffableStat::SkillDmgBonus,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(3)),
                },
                ConditionalBuff {
                    name: "kagura_full_stack_elemental_dmg",
                    description: "3スタック時に元素DMG+12-24%（DmgBonus近似値、物理除外）",
                    stat: BuffableStat::DmgBonus,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
```

Note: Verify `ManualCondition` is in the imports at top of catalyst.rs. It's already imported for Lost Prayer and Mappa Mare usage, so it should be there.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data kagura_has_skill -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/catalyst.rs
git commit -m "feat: add Kagura's Verity stacks ConditionalBuff (P3 Batch 2)"
```

---

### Task 7: Tulaytullah's Remembrance (Catalyst, Pattern A)

**Files:**
- Modify: `crates/data/src/weapons/catalyst.rs:322-337` (weapon data)
- Modify: `crates/data/src/weapons/catalyst.rs` (test module, created in Task 6)

- [ ] **Step 1: Write the failing test**

Add to the `mod tests` block in `crates/data/src/weapons/catalyst.rs`:

```rust
    #[test]
    fn tulaytullah_has_na_dmg_stacks() {
        let passive = TULAYTULLAHS_REMEMBRANCE.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "tulaytullah_na_dmg_stacks");
        assert_eq!(buff.stat, BuffableStat::NormalAtkDmgBonus);
        assert!((buff.value - 0.048).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(10))
        ));
        assert!(buff.refinement_values.is_some());
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data tulaytullah_has_na -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuff**

In `crates/data/src/weapons/catalyst.rs`, replace the Tulaytullah passive section. Change:

```rust
            description: "Conditional: 通常攻撃速度アップ、NA DMGスタック",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "通常攻撃DMG+4.8-9.6%スタック（最大10スタック、1秒毎）。攻撃速度バフは非対応",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "tulaytullah_na_dmg_stacks",
                description: "通常攻撃DMG+4.8-9.6%（1スタック）、最大10スタック",
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.048,
                refinement_values: Some([0.048, 0.06, 0.072, 0.084, 0.096]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(10)),
            }],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data tulaytullah_has_na -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/catalyst.rs
git commit -m "feat: add Tulaytullah's Remembrance stacks ConditionalBuff (P3 Batch 2)"
```

---

### Task 8: Nocturne's Curtain Call (Catalyst, Pattern A)

**Files:**
- Modify: `crates/data/src/weapons/catalyst.rs:191-206` (weapon data)
- Modify: `crates/data/src/weapons/catalyst.rs` (test module)

- [ ] **Step 1: Write the failing test**

Add to the `mod tests` block in `crates/data/src/weapons/catalyst.rs`:

```rust
    #[test]
    fn nocturne_has_dmg_stacks() {
        let passive = NOCTURNES_CURTAIN_CALL.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "nocturne_dmg_stacks");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(5))
        ));
        assert!(buff.refinement_values.is_some());
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data nocturne_has_dmg -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuff**

In `crates/data/src/weapons/catalyst.rs`, replace the Nocturne passive section. Change:

```rust
            description: "Conditional: 元素スキル/爆発命中でスタック獲得、DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "元素スキル/爆発命中でDMG+8-16%スタック（最大5スタック）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "nocturne_dmg_stacks",
                description: "元素スキル/爆発命中でDMG+8-16%（1スタック）、最大5スタック",
                stat: BuffableStat::DmgBonus,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(5)),
            }],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data nocturne_has_dmg -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/catalyst.rs
git commit -m "feat: add Nocturne's Curtain Call stacks ConditionalBuff (P3 Batch 2)"
```

---

### Task 9: Vivid Notions (Catalyst, Pattern A)

**Files:**
- Modify: `crates/data/src/weapons/catalyst.rs:339-354` (weapon data)
- Modify: `crates/data/src/weapons/catalyst.rs` (test module)

- [ ] **Step 1: Write the failing test**

Add to the `mod tests` block in `crates/data/src/weapons/catalyst.rs`:

```rust
    #[test]
    fn vivid_notions_has_dmg_stacks() {
        let passive = VIVID_NOTIONS.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "vivid_notions_dmg_stacks");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        assert!(buff.refinement_values.is_some());
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data vivid_notions_has -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuff**

In `crates/data/src/weapons/catalyst.rs`, replace the Vivid Notions passive section. Change:

```rust
            description: "Conditional: 夜魂バースト時にDMGアップスタック",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "夜魂バースト時にDMG+8-16%スタック（最大3スタック）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "vivid_notions_dmg_stacks",
                description: "夜魂バースト時にDMG+8-16%（1スタック）、最大3スタック",
                stat: BuffableStat::DmgBonus,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data vivid_notions_has -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/catalyst.rs
git commit -m "feat: add Vivid Notions stacks ConditionalBuff (P3 Batch 2)"
```

---

### Task 10: Full Test Suite & Data Integrity

**Files:**
- No new files. Runs existing tests.

- [ ] **Step 1: Run full data test suite**

Run: `cargo test -p genshin-calc-data`
Expected: ALL PASS. The existing `data_integrity` tests (unique names, stacks max > 0, refinement_values consistency) automatically validate all new ConditionalBuffs.

- [ ] **Step 2: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings.

- [ ] **Step 3: Run fmt check**

Run: `cargo fmt --check`
Expected: No formatting issues.

- [ ] **Step 4: Run full workspace test**

Run: `cargo test`
Expected: ALL PASS. Core crate tests unaffected (no core changes).

- [ ] **Step 5: Update data-expansion-todo.md**

In `docs/data-expansion-todo.md`, update the ConditionalBuff count in the 現状サマリ table:

Change the line:
```
| 武器パッシブ (ConditionalBuff) | 220 | 20 | 200 | 9% |
```
To (15 existing weapons + 9 new weapons = 24):
```
| 武器パッシブ (ConditionalBuff) | 220 | 24 | 196 | 11% |
```

Note: The "20" was stale — actual current count is 15 weapons with ConditionalBuff. After adding 9 new weapons = 24 weapons.

Also update the prose at line 20 from "12本にConditionalBuff実装済み" to "24本にConditionalBuff実装済み".

- [ ] **Step 6: Commit**

```bash
git add docs/data-expansion-todo.md
git commit -m "docs: update weapon ConditionalBuff coverage to 15% (P3 Batch 2)"
```
