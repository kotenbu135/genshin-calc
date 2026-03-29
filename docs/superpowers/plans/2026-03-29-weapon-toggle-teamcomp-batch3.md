# P3 Batch 3: 5-Star Weapon Toggle/TeamComp Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Toggle/TeamComp/Both ConditionalBuff definitions to 13 five-star weapons.

**Architecture:** Data-only changes — no core crate modifications. Weapons use existing `ManualCondition::Toggle`, `AutoCondition::TeamSameElementCount/TeamDiffElementCount`, and `Activation::Both`. Tests verify buff existence, stat, value, and activation type.

**Tech Stack:** Rust, Cargo workspace (`crates/data`)

**Spec:** `docs/superpowers/specs/2026-03-29-weapon-toggle-teamcomp-batch3-design.md`

---

## File Structure

| File | Action | Responsibility |
|------|--------|---------------|
| `crates/data/src/weapons/sword.rs` | Modify | Athame Artis, Azurelight, Lightbearing Moonshard + tests |
| `crates/data/src/weapons/claymore.rs` | Modify | Beacon of Reed Sea, Gest of Mighty Wolf + tests |
| `crates/data/src/weapons/bow.rs` | Modify | Elegy for the End, The First Great Magic + tests |
| `crates/data/src/weapons/catalyst.rs` | Modify | A Thousand Floating Dreams, Jadefall, Nightweaver, Reliquary, Starcaller, Sunny + tests (add `AutoCondition` to imports) |

---

### Task 1: Athame Artis (Sword, Toggle — 2 buffs)

**Files:**
- Modify: `crates/data/src/weapons/sword.rs:55-70` (weapon data)
- Modify: `crates/data/src/weapons/sword.rs` (test module at end of file)

- [ ] **Step 1: Write the failing test**

Add to the existing `mod tests` block in `crates/data/src/weapons/sword.rs`:

```rust
    #[test]
    fn athame_artis_has_skill_cr_and_skill_dmg() {
        let passive = ATHAME_ARTIS.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        let cr_buff = &cond_buffs[0];
        assert_eq!(cr_buff.name, "athame_artis_skill_cr");
        assert_eq!(cr_buff.stat, BuffableStat::CritRate);
        assert!((cr_buff.value - 0.10).abs() < 1e-6);
        assert!(matches!(cr_buff.activation, Activation::Manual(ManualCondition::Toggle)));

        let dmg_buff = &cond_buffs[1];
        assert_eq!(dmg_buff.name, "athame_artis_skill_dmg");
        assert_eq!(dmg_buff.stat, BuffableStat::SkillDmgBonus);
        assert!((dmg_buff.value - 0.08).abs() < 1e-6);
        assert!(matches!(dmg_buff.activation, Activation::Manual(ManualCondition::Toggle)));
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data athame_artis_has -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuffs**

In `crates/data/src/weapons/sword.rs`, replace the Athame Artis passive. Change:

```rust
            description: "Conditional: 元素スキルの会心率と元素ダメージがアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "元素スキル使用後にスキルCR+10-20%/スキルDMG+8-16%（CritRate近似値）",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "athame_artis_skill_cr",
                    description: "元素スキル使用後にCR+10-20%（スキルのみ。CritRate近似値）",
                    stat: BuffableStat::CritRate,
                    value: 0.10,
                    refinement_values: Some([0.10, 0.125, 0.15, 0.175, 0.20]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "athame_artis_skill_dmg",
                    description: "元素スキル使用後にスキルDMG+8-16%",
                    stat: BuffableStat::SkillDmgBonus,
                    value: 0.08,
                    refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data athame_artis_has -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/sword.rs
git commit -m "feat: add Athame Artis toggle ConditionalBuff (P3 Batch 3)"
```

---

### Task 2: Azurelight (Sword, Both — 1 buff)

**Files:**
- Modify: `crates/data/src/weapons/sword.rs:72-88` (weapon data)
- Modify: `crates/data/src/weapons/sword.rs` (test module)

- [ ] **Step 1: Write the failing test**

```rust
    #[test]
    fn azurelight_has_hp_na_dmg_both() {
        let passive = AZURELIGHT.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "azurelight_hp_na_dmg");
        assert_eq!(buff.stat, BuffableStat::NormalAtkDmgBonus);
        assert!((buff.value - 0.0016).abs() < 1e-7);
        assert!(matches!(
            buff.activation,
            Activation::Both(
                AutoCondition::StatScaling { stat: BuffableStat::HpPercent, .. },
                ManualCondition::Toggle
            )
        ));
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data azurelight_has -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuff**

Change:

```rust
            description: "Conditional: HP上限を基にした通常攻撃ダメージアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "HP上限×0.16-0.32%分をNA DMGボーナスに加算（条件付き、上限40-80%）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "azurelight_hp_na_dmg",
                description: "HP上限×0.16-0.32%分をNA DMGに加算",
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.0016,
                refinement_values: Some([0.0016, 0.002, 0.0024, 0.0028, 0.0032]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Both(
                    AutoCondition::StatScaling {
                        stat: BuffableStat::HpPercent,
                        offset: None,
                        cap: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
                    },
                    ManualCondition::Toggle,
                ),
            }],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data azurelight_has -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/sword.rs
git commit -m "feat: add Azurelight Both(StatScaling,Toggle) ConditionalBuff (P3 Batch 3)"
```

---

### Task 3: Lightbearing Moonshard (Sword, Toggle — 2 buffs)

**Files:**
- Modify: `crates/data/src/weapons/sword.rs` (weapon data + test module)

- [ ] **Step 1: Write the failing test**

```rust
    #[test]
    fn lightbearing_moonshard_has_atk_and_dmg_toggle() {
        let passive = LIGHTBEARING_MOONSHARD.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "lightbearing_atk");
        assert_eq!(cond_buffs[0].stat, BuffableStat::AtkPercent);
        assert!((cond_buffs[0].value - 0.24).abs() < 1e-6);

        assert_eq!(cond_buffs[1].name, "lightbearing_dmg");
        assert_eq!(cond_buffs[1].stat, BuffableStat::DmgBonus);
        assert!((cond_buffs[1].value - 0.20).abs() < 1e-6);

        for buff in cond_buffs {
            assert!(matches!(buff.activation, Activation::Manual(ManualCondition::Toggle)));
        }
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data lightbearing_moonshard -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuffs**

Change:

```rust
            description: "Conditional: 月光の力で攻撃力とダメージがアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "月光の力でATK+24-48%/DMG+20-40%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "lightbearing_atk",
                    description: "月光発動時にATK+24-48%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.24,
                    refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "lightbearing_dmg",
                    description: "月光発動時にDMG+20-40%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data lightbearing_moonshard -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/sword.rs
git commit -m "feat: add Lightbearing Moonshard toggle ConditionalBuff (P3 Batch 3)"
```

---

### Task 4: Beacon of the Reed Sea (Claymore, Multi-Toggle — 3 buffs)

**Files:**
- Modify: `crates/data/src/weapons/claymore.rs` (weapon data + test module)

- [ ] **Step 1: Write the failing test**

Add to `mod tests` in `crates/data/src/weapons/claymore.rs`:

```rust
    #[test]
    fn beacon_reed_sea_has_three_toggles() {
        let passive = BEACON_OF_THE_REED_SEA.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 3);

        assert_eq!(cond_buffs[0].name, "beacon_skill_atk");
        assert_eq!(cond_buffs[0].stat, BuffableStat::AtkPercent);
        assert!((cond_buffs[0].value - 0.20).abs() < 1e-6);

        assert_eq!(cond_buffs[1].name, "beacon_dmg_taken_atk");
        assert_eq!(cond_buffs[1].stat, BuffableStat::AtkPercent);
        assert!((cond_buffs[1].value - 0.20).abs() < 1e-6);

        assert_eq!(cond_buffs[2].name, "beacon_shield_hp");
        assert_eq!(cond_buffs[2].stat, BuffableStat::HpPercent);
        assert!((cond_buffs[2].value - 0.32).abs() < 1e-6);

        for buff in cond_buffs {
            assert!(matches!(buff.activation, Activation::Manual(ManualCondition::Toggle)));
        }
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data beacon_reed_sea -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuffs**

Change:

```rust
            description: "Conditional: スキル命中後ATKアップ、ダメージ受けるとATKアップ、シールド時HP%アップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "スキル命中後ATK+20-40%、被弾後ATK+20-40%、シールド時HP+32-64%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "beacon_skill_atk",
                    description: "元素スキル命中後にATK+20-40%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "beacon_dmg_taken_atk",
                    description: "ダメージを受けた後にATK+20-40%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "beacon_shield_hp",
                    description: "シールド保護時にHP+32-64%",
                    stat: BuffableStat::HpPercent,
                    value: 0.32,
                    refinement_values: Some([0.32, 0.40, 0.48, 0.56, 0.64]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data beacon_reed_sea -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/claymore.rs
git commit -m "feat: add Beacon of Reed Sea multi-toggle ConditionalBuff (P3 Batch 3)"
```

---

### Task 5: Gest of the Mighty Wolf (Claymore, Toggle — 2 buffs)

**Files:**
- Modify: `crates/data/src/weapons/claymore.rs` (weapon data + test module)

- [ ] **Step 1: Write the failing test**

```rust
    #[test]
    fn gest_mighty_wolf_has_dmg_and_atk_toggle() {
        let passive = GEST_OF_THE_MIGHTY_WOLF.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "gest_wolf_dmg");
        assert_eq!(cond_buffs[0].stat, BuffableStat::DmgBonus);
        assert!((cond_buffs[0].value - 0.16).abs() < 1e-6);

        assert_eq!(cond_buffs[1].name, "gest_wolf_atk");
        assert_eq!(cond_buffs[1].stat, BuffableStat::AtkPercent);
        assert!((cond_buffs[1].value - 0.16).abs() < 1e-6);

        for buff in cond_buffs {
            assert!(matches!(buff.activation, Activation::Manual(ManualCondition::Toggle)));
        }
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data gest_mighty_wolf -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuffs**

Change:

```rust
            description: "Conditional: 狼の力でダメージとバフがアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "狼の力でDMG+16-32%/ATK+16-32%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "gest_wolf_dmg",
                    description: "狼の力発動時にDMG+16-32%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "gest_wolf_atk",
                    description: "狼の力発動時にATK+16-32%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data gest_mighty_wolf -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/claymore.rs
git commit -m "feat: add Gest of Mighty Wolf toggle ConditionalBuff (P3 Batch 3)"
```

---

### Task 6: Elegy for the End (Bow, TeamBuff — StatBuff + 2 ConditionalBuffs)

**Files:**
- Modify: `crates/data/src/weapons/bow.rs:86-101` (weapon data)
- Modify: `crates/data/src/weapons/bow.rs` (test module)

- [ ] **Step 1: Write the failing test**

```rust
    #[test]
    fn elegy_has_em_statbuff_and_team_conditionals() {
        let passive = ELEGY_FOR_THE_END.passive.unwrap();

        // Always-on EM StatBuff
        assert_eq!(passive.effect.buffs.len(), 1);
        assert_eq!(passive.effect.buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((passive.effect.buffs[0].value - 60.0).abs() < 1e-6);

        // Team conditional buffs
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "elegy_team_em");
        assert_eq!(cond_buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((cond_buffs[0].value - 100.0).abs() < 1e-6);
        assert_eq!(cond_buffs[0].target, BuffTarget::Team);

        assert_eq!(cond_buffs[1].name, "elegy_team_atk");
        assert_eq!(cond_buffs[1].stat, BuffableStat::AtkPercent);
        assert!((cond_buffs[1].value - 0.20).abs() < 1e-6);
        assert_eq!(cond_buffs[1].target, BuffTarget::Team);

        for buff in cond_buffs {
            assert!(matches!(buff.activation, Activation::Manual(ManualCondition::Toggle)));
        }
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data elegy_has_em -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement StatBuff + ConditionalBuffs**

Change:

```rust
            description: "Conditional: EM+60。追憶の印蓄積でチーム全員にEM+100/ATK+20%",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "EM+60-120。追憶の印蓄積でチーム全員にEM+100-200/ATK+20-40%",
            buffs: &[StatBuff {
                stat: BuffableStat::ElementalMastery,
                value: 60.0,
                refinement_values: Some([60.0, 75.0, 90.0, 105.0, 120.0]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "elegy_team_em",
                    description: "追憶の印フルスタック時にチーム全員EM+100-200",
                    stat: BuffableStat::ElementalMastery,
                    value: 100.0,
                    refinement_values: Some([100.0, 125.0, 150.0, 175.0, 200.0]),
                    stack_values: None,
                    target: BuffTarget::Team,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "elegy_team_atk",
                    description: "追憶の印フルスタック時にチーム全員ATK+20-40%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::Team,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data elegy_has_em -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/bow.rs
git commit -m "feat: add Elegy for the End team buff ConditionalBuff (P3 Batch 3)"
```

---

### Task 7: The First Great Magic (Bow, TeamComp — 3 buffs)

**Files:**
- Modify: `crates/data/src/weapons/bow.rs:220-239` (weapon data)
- Modify: `crates/data/src/weapons/bow.rs` (test module)

- [ ] **Step 1: Write the failing test**

```rust
    #[test]
    fn first_great_magic_has_team_diff_atk_conditionals() {
        let passive = THE_FIRST_GREAT_MAGIC.passive.unwrap();

        // CA DMG StatBuff should still exist
        assert_eq!(passive.effect.buffs.len(), 1);
        assert_eq!(passive.effect.buffs[0].stat, BuffableStat::ChargedAtkDmgBonus);

        // 3 overlapping TeamDiffElementCount buffs
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 3);

        let expected_min_counts = [1u8, 2, 3];
        for (i, buff) in cond_buffs.iter().enumerate() {
            let expected_name = format!("first_great_magic_atk_{}", i + 1);
            assert_eq!(buff.name, expected_name.as_str());
            assert_eq!(buff.stat, BuffableStat::AtkPercent);
            assert!((buff.value - 0.16).abs() < 1e-6);
            assert!(matches!(
                buff.activation,
                Activation::Auto(AutoCondition::TeamDiffElementCount { min_count })
                if min_count == expected_min_counts[i]
            ));
        }
    }
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data first_great_magic_has -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement the ConditionalBuffs**

Change:

```rust
            conditional_buffs: &[],
```

(In THE_FIRST_GREAT_MAGIC, keeping existing `buffs` array with ChargedAtkDmgBonus intact)

To:

```rust
            conditional_buffs: &[
                ConditionalBuff {
                    name: "first_great_magic_atk_1",
                    description: "1+異元素チームメンバーでATK+16-32%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount { min_count: 1 }),
                },
                ConditionalBuff {
                    name: "first_great_magic_atk_2",
                    description: "2+異元素チームメンバーでATK+16-32%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount { min_count: 2 }),
                },
                ConditionalBuff {
                    name: "first_great_magic_atk_3",
                    description: "3+異元素チームメンバーでATK+16-32%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount { min_count: 3 }),
                },
            ],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data first_great_magic_has -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/bow.rs
git commit -m "feat: add The First Great Magic TeamComp ConditionalBuff (P3 Batch 3)"
```

---

### Task 8: A Thousand Floating Dreams (Catalyst, TeamComp — 7 buffs)

**Files:**
- Modify: `crates/data/src/weapons/catalyst.rs:1-3` (add `AutoCondition` to imports)
- Modify: `crates/data/src/weapons/catalyst.rs` (weapon data)
- Modify: `crates/data/src/weapons/catalyst.rs` (test module — create if not exists from Batch 2)

**Important:** `catalyst.rs` may already have a test module from Batch 2 implementation. If not, create one. Also, add `AutoCondition` to the imports at line 2 — currently missing.

- [ ] **Step 1: Add `AutoCondition` to catalyst.rs imports**

Change line 1-3:

```rust
use crate::buff::{
    Activation, BuffTarget, BuffableStat, ConditionalBuff, ManualCondition, PassiveEffect, StatBuff,
};
```

To:

```rust
use crate::buff::{
    Activation, AutoCondition, BuffTarget, BuffableStat, ConditionalBuff, ManualCondition,
    PassiveEffect, StatBuff,
};
```

- [ ] **Step 2: Write the failing test**

```rust
    #[test]
    fn thousand_floating_dreams_has_team_comp_and_team_buff() {
        let passive = A_THOUSAND_FLOATING_DREAMS.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 7);

        // Same-element EM (3 overlapping)
        for i in 0..3 {
            let name = format!("thousand_dreams_same{}_em", i + 1);
            let expected_min = (i + 1) as u8;
            assert_eq!(cond_buffs[i].name, name.as_str());
            assert_eq!(cond_buffs[i].stat, BuffableStat::ElementalMastery);
            assert!((cond_buffs[i].value - 32.0).abs() < 1e-6);
            assert!(matches!(
                cond_buffs[i].activation,
                Activation::Auto(AutoCondition::TeamSameElementCount { min_count })
                if min_count == expected_min
            ));
        }

        // Different-element DMG% (3 overlapping)
        for i in 3..6 {
            let name = format!("thousand_dreams_diff{}_dmg", i - 2);
            let expected_min = (i - 2) as u8;
            assert_eq!(cond_buffs[i].name, name.as_str());
            assert_eq!(cond_buffs[i].stat, BuffableStat::DmgBonus);
            assert!((cond_buffs[i].value - 0.10).abs() < 1e-6);
            assert!(matches!(
                cond_buffs[i].activation,
                Activation::Auto(AutoCondition::TeamDiffElementCount { min_count })
                if min_count == expected_min
            ));
        }

        // Team EM buff
        assert_eq!(cond_buffs[6].name, "thousand_dreams_team_em");
        assert_eq!(cond_buffs[6].stat, BuffableStat::ElementalMastery);
        assert!((cond_buffs[6].value - 40.0).abs() < 1e-6);
        assert_eq!(cond_buffs[6].target, BuffTarget::TeamExcludeSelf);
    }
```

- [ ] **Step 3: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data thousand_floating_dreams -- --nocapture`
Expected: FAIL

- [ ] **Step 4: Implement the ConditionalBuffs**

Find `A_THOUSAND_FLOATING_DREAMS` in catalyst.rs and change:

```rust
            description: "Conditional: チームの元素タイプに応じてEM/DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "同元素1人毎にEM+32-64、異元素1人毎にDMG+10-26%。チームにEM+40-56",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "thousand_dreams_same1_em",
                    description: "1+同元素チームメンバーでEM+32-64",
                    stat: BuffableStat::ElementalMastery,
                    value: 32.0,
                    refinement_values: Some([32.0, 40.0, 48.0, 56.0, 64.0]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamSameElementCount { min_count: 1 }),
                },
                ConditionalBuff {
                    name: "thousand_dreams_same2_em",
                    description: "2+同元素チームメンバーでEM+32-64",
                    stat: BuffableStat::ElementalMastery,
                    value: 32.0,
                    refinement_values: Some([32.0, 40.0, 48.0, 56.0, 64.0]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamSameElementCount { min_count: 2 }),
                },
                ConditionalBuff {
                    name: "thousand_dreams_same3_em",
                    description: "3+同元素チームメンバーでEM+32-64",
                    stat: BuffableStat::ElementalMastery,
                    value: 32.0,
                    refinement_values: Some([32.0, 40.0, 48.0, 56.0, 64.0]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamSameElementCount { min_count: 3 }),
                },
                ConditionalBuff {
                    name: "thousand_dreams_diff1_dmg",
                    description: "1+異元素チームメンバーでDMG+10-26%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.10,
                    refinement_values: Some([0.10, 0.14, 0.18, 0.22, 0.26]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount { min_count: 1 }),
                },
                ConditionalBuff {
                    name: "thousand_dreams_diff2_dmg",
                    description: "2+異元素チームメンバーでDMG+10-26%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.10,
                    refinement_values: Some([0.10, 0.14, 0.18, 0.22, 0.26]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount { min_count: 2 }),
                },
                ConditionalBuff {
                    name: "thousand_dreams_diff3_dmg",
                    description: "3+異元素チームメンバーでDMG+10-26%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.10,
                    refinement_values: Some([0.10, 0.14, 0.18, 0.22, 0.26]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount { min_count: 3 }),
                },
                ConditionalBuff {
                    name: "thousand_dreams_team_em",
                    description: "チームメンバーにEM+40-56",
                    stat: BuffableStat::ElementalMastery,
                    value: 40.0,
                    refinement_values: Some([40.0, 44.0, 48.0, 52.0, 56.0]),
                    stack_values: None,
                    target: BuffTarget::TeamExcludeSelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data thousand_floating_dreams -- --nocapture`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/weapons/catalyst.rs
git commit -m "feat: add A Thousand Floating Dreams TeamComp ConditionalBuff (P3 Batch 3)"
```

---

### Task 9: Simple Toggle Catalysts — Jadefall, Nightweaver, Reliquary (3 weapons)

**Files:**
- Modify: `crates/data/src/weapons/catalyst.rs` (3 weapon data entries + tests)

- [ ] **Step 1: Write the failing tests**

```rust
    #[test]
    fn jadefall_has_em_toggle() {
        let passive = JADEFALLS_SPLENDOR.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        assert_eq!(cond_buffs[0].name, "jadefall_em");
        assert_eq!(cond_buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((cond_buffs[0].value - 32.0).abs() < 1e-6);
        assert!(matches!(cond_buffs[0].activation, Activation::Manual(ManualCondition::Toggle)));
    }

    #[test]
    fn nightweaver_has_na_ca_dmg_toggle() {
        let passive = NIGHTWEAVERS_LOOKING_GLASS.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);
        assert_eq!(cond_buffs[0].name, "nightweaver_na_dmg");
        assert_eq!(cond_buffs[0].stat, BuffableStat::NormalAtkDmgBonus);
        assert!((cond_buffs[0].value - 0.16).abs() < 1e-6);
        assert_eq!(cond_buffs[1].name, "nightweaver_ca_dmg");
        assert_eq!(cond_buffs[1].stat, BuffableStat::ChargedAtkDmgBonus);
        assert!((cond_buffs[1].value - 0.16).abs() < 1e-6);
        for buff in cond_buffs {
            assert!(matches!(buff.activation, Activation::Manual(ManualCondition::Toggle)));
        }
    }

    #[test]
    fn reliquary_truth_has_dmg_toggle() {
        let passive = RELIQUARY_OF_TRUTH.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        assert_eq!(cond_buffs[0].name, "reliquary_truth_dmg");
        assert_eq!(cond_buffs[0].stat, BuffableStat::DmgBonus);
        assert!((cond_buffs[0].value - 0.12).abs() < 1e-6);
        assert!(matches!(cond_buffs[0].activation, Activation::Manual(ManualCondition::Toggle)));
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data jadefall_has_em -- --nocapture && cargo test -p genshin-calc-data nightweaver_has -- --nocapture && cargo test -p genshin-calc-data reliquary_truth -- --nocapture`
Expected: ALL FAIL

- [ ] **Step 3: Implement Jadefall's Splendor**

Change:

```rust
            description: "Conditional: 元素エネルギー消費後にEM獲得",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "元素エネルギー消費後にEM+32-64",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "jadefall_em",
                description: "元素エネルギー消費後にEM+32-64",
                stat: BuffableStat::ElementalMastery,
                value: 32.0,
                refinement_values: Some([32.0, 40.0, 48.0, 56.0, 64.0]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
```

- [ ] **Step 4: Implement Nightweaver's Looking Glass**

Change:

```rust
            description: "Conditional: 夜魂を消耗して通常/重撃ダメージアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "夜魂消費でNA DMG+16-32%/CA DMG+16-32%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "nightweaver_na_dmg",
                    description: "夜魂消費でNA DMG+16-32%",
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "nightweaver_ca_dmg",
                    description: "夜魂消費でCA DMG+16-32%",
                    stat: BuffableStat::ChargedAtkDmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
```

- [ ] **Step 5: Implement Reliquary of Truth**

Change:

```rust
            description: "Conditional: 元素反応でDMGアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "元素反応後にDMG+12-36%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "reliquary_truth_dmg",
                description: "元素反応後にDMG+12-36%",
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.18, 0.24, 0.30, 0.36]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
```

- [ ] **Step 6: Run all 3 tests to verify they pass**

Run: `cargo test -p genshin-calc-data jadefall_has_em nightweaver_has reliquary_truth -- --nocapture`
Expected: ALL PASS

- [ ] **Step 7: Commit**

```bash
git add crates/data/src/weapons/catalyst.rs
git commit -m "feat: add Jadefall, Nightweaver, Reliquary toggle ConditionalBuff (P3 Batch 3)"
```

---

### Task 10: Simple Toggle Catalysts — Starcaller, Sunny Morning (2 weapons)

**Files:**
- Modify: `crates/data/src/weapons/catalyst.rs` (2 weapon data entries + tests)

- [ ] **Step 1: Write the failing tests**

```rust
    #[test]
    fn starcaller_has_dmg_toggle() {
        let passive = STARCALLERS_WATCH.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        assert_eq!(cond_buffs[0].name, "starcaller_dmg");
        assert_eq!(cond_buffs[0].stat, BuffableStat::DmgBonus);
        assert!((cond_buffs[0].value - 0.20).abs() < 1e-6);
        assert!(matches!(cond_buffs[0].activation, Activation::Manual(ManualCondition::Toggle)));
    }

    #[test]
    fn sunny_morning_has_atk_and_dmg_toggle() {
        let passive = SUNNY_MORNING_SLEEP_IN.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);
        assert_eq!(cond_buffs[0].name, "sunny_morning_atk");
        assert_eq!(cond_buffs[0].stat, BuffableStat::AtkPercent);
        assert!((cond_buffs[0].value - 0.14).abs() < 1e-6);
        assert_eq!(cond_buffs[1].name, "sunny_morning_dmg");
        assert_eq!(cond_buffs[1].stat, BuffableStat::DmgBonus);
        assert!((cond_buffs[1].value - 0.18).abs() < 1e-6);
        for buff in cond_buffs {
            assert!(matches!(buff.activation, Activation::Manual(ManualCondition::Toggle)));
        }
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data starcaller_has -- --nocapture && cargo test -p genshin-calc-data sunny_morning -- --nocapture`
Expected: ALL FAIL

- [ ] **Step 3: Implement Starcaller's Watch**

Change:

```rust
            description: "Conditional: チームの元素反応でDMGアップ",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "チームの元素反応でDMG+20-40%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "starcaller_dmg",
                description: "チームが元素反応を起こした後にDMG+20-40%",
                stat: BuffableStat::DmgBonus,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
```

- [ ] **Step 4: Implement Sunny Morning Sleep-In**

Change:

```rust
            description: "Conditional: 元素反応時にバフ獲得",
            buffs: &[],
            conditional_buffs: &[],
```

To:

```rust
            description: "元素反応時にATK+14-28%/DMG+18-36%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "sunny_morning_atk",
                    description: "元素反応時にATK+14-28%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.14,
                    refinement_values: Some([0.14, 0.175, 0.21, 0.245, 0.28]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "sunny_morning_dmg",
                    description: "元素反応時にDMG+18-36%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.18,
                    refinement_values: Some([0.18, 0.225, 0.27, 0.315, 0.36]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
```

- [ ] **Step 5: Run all tests to verify they pass**

Run: `cargo test -p genshin-calc-data starcaller_has sunny_morning -- --nocapture`
Expected: ALL PASS

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/weapons/catalyst.rs
git commit -m "feat: add Starcaller, Sunny Morning toggle ConditionalBuff (P3 Batch 3)"
```

---

### Task 11: Full Test Suite & Data Integrity

**Files:**
- No new files. Runs existing tests.

- [ ] **Step 1: Run full data test suite**

Run: `cargo test -p genshin-calc-data`
Expected: ALL PASS. Existing `data_integrity` tests validate unique names, stacks consistency, etc.

- [ ] **Step 2: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings.

- [ ] **Step 3: Run fmt check**

Run: `cargo fmt --check`
Expected: No formatting issues.

- [ ] **Step 4: Run full workspace test**

Run: `cargo test`
Expected: ALL PASS. Core crate tests unaffected.

- [ ] **Step 5: Update data-expansion-todo.md**

In `docs/data-expansion-todo.md`, update the ConditionalBuff weapon count. Current value is stale — check with:
```bash
grep -c "conditional_buffs: &\[ConditionalBuff" crates/data/src/weapons/*.rs | tail -1
```

Update the line:
```
| 武器パッシブ (ConditionalBuff) | 220 | <old> | <old> | <old>% |
```
To reflect new total (previous + 13 weapons added in this batch).

- [ ] **Step 6: Commit**

```bash
git add docs/data-expansion-todo.md
git commit -m "docs: update weapon ConditionalBuff coverage (P3 Batch 3)"
```
