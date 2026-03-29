# P3: 4-Star Weapon ConditionalBuff Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add ConditionalBuff definitions to 27 four-star weapons across 4 weapon files (+ 1 test for existing Lion's Roar = 28 tests total).

**Architecture:** Data-only changes to existing weapon constants in `crates/data`. Each weapon's `conditional_buffs: &[]` is replaced with the appropriate ConditionalBuff array. No core crate changes.

**Tech Stack:** Rust, existing ConditionalBuff/Activation/BuffableStat types from `crates/data/src/buff.rs`

**Spec:** `docs/superpowers/specs/2026-03-29-4star-toggle-design.md`

---

## File Map

| File | Action | Weapons |
|------|--------|---------|
| `crates/data/src/weapons/sword.rs` | Modify 8 consts + add 9 tests (Lion's Roar: test only) | #1-9 |
| `crates/data/src/weapons/claymore.rs` | Modify 2 consts + add 2 tests | #10-11 |
| `crates/data/src/weapons/bow.rs` | Modify 8 consts + add 8 tests | #12-19 |
| `crates/data/src/weapons/catalyst.rs` | Modify 9 consts + add 9 tests | #20-28 |

## Reference: Existing Patterns

**Toggle (Lion's Roar — already implemented in sword.rs):**
```rust
conditional_buffs: &[ConditionalBuff {
    name: "lions_roar_dmg",
    description: "炎/雷元素の影響下の敵へのDMG+20-36%",
    stat: BuffableStat::DmgBonus,
    value: 0.20,
    refinement_values: Some([0.20, 0.24, 0.28, 0.32, 0.36]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

**Auto(StatScaling) (Hunter's Path — already implemented in bow.rs):**
```rust
conditional_buffs: &[ConditionalBuff {
    name: "hunters_path_em_ca_flat",
    description: "EM×160-320%分を重撃フラットダメージに加算",
    stat: BuffableStat::ChargedAtkFlatDmg,
    value: 1.60,
    refinement_values: Some([1.60, 2.00, 2.40, 2.80, 3.20]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Auto(AutoCondition::StatScaling {
        stat: BuffableStat::ElementalMastery,
        offset: None,
        cap: None,
    }),
}],
```

**Test pattern (existing in bow.rs):**
```rust
#[test]
fn hunters_path_has_em_ca_flat_conditional() {
    let passive = HUNTERS_PATH.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "hunters_path_em_ca_flat");
    assert_eq!(buff.stat, BuffableStat::ChargedAtkFlatDmg);
    assert!((buff.value - 1.60).abs() < 1e-6);
    assert!(matches!(
        buff.activation,
        Activation::Auto(AutoCondition::StatScaling {
            stat: BuffableStat::ElementalMastery,
            offset: None,
            cap: None,
        })
    ));
}
```

## TBV Value Resolution

9 weapons have TBV (To Be Verified) refinement values. Before implementing each TBV weapon, look up the exact values from the Genshin Impact Wiki (https://genshin-impact.fandom.com/wiki/) or Honey Hunter (https://ambr.top/en). Use the weapon's passive description page.

Standard 4-star refinement scaling patterns:
- Most common: `[base, base*1.25, base*1.5, base*1.75, base*2.0]`
- Some weapons use different scaling — always verify

---

### Task 1: Swords (9 weapons)

**Files:**
- Modify: `crates/data/src/weapons/sword.rs`

#### Subtask 1a: Cinnabar Spindle + Flute of Ezpitzal [Pattern C: Auto(StatScaling)]

- [ ] **Step 1: Write failing tests for Cinnabar Spindle and Flute of Ezpitzal**

Add to the `#[cfg(test)]` module at the bottom of `sword.rs`:

```rust
#[test]
fn cinnabar_spindle_has_def_scaling_skill_flat() {
    let passive = CINNABAR_SPINDLE.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "cinnabar_skill_def_flat");
    assert_eq!(buff.stat, BuffableStat::SkillFlatDmg);
    assert!((buff.value - 0.40).abs() < 1e-6);
    assert!(buff.refinement_values.is_some());
    let rv = buff.refinement_values.unwrap();
    assert!((rv[0] - 0.40).abs() < 1e-6);
    assert!((rv[4] - 0.80).abs() < 1e-6);
    assert!(matches!(
        buff.activation,
        Activation::Auto(AutoCondition::StatScaling {
            stat: BuffableStat::DefPercent,
            offset: None,
            cap: None,
        })
    ));
}

#[test]
fn flute_of_ezpitzal_has_def_scaling_flat() {
    let passive = FLUTE_OF_EZPITZAL.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "flute_ezpitzal_def_flat");
    // TBV: verify stat type and values from wiki
    assert!(matches!(
        buff.activation,
        Activation::Auto(AutoCondition::StatScaling {
            stat: BuffableStat::DefPercent,
            ..
        })
    ));
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data cinnabar_spindle_has_def_scaling flute_of_ezpitzal_has_def_scaling -- --nocapture`
Expected: FAIL (conditional_buffs is empty)

- [ ] **Step 3: Implement Cinnabar Spindle ConditionalBuff**

In `CINNABAR_SPINDLE`, replace `conditional_buffs: &[]` with:

```rust
conditional_buffs: &[ConditionalBuff {
    name: "cinnabar_skill_def_flat",
    description: "元素スキルのDMGにDEFの40-80%分を加算",
    stat: BuffableStat::SkillFlatDmg,
    value: 0.40,
    refinement_values: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Auto(AutoCondition::StatScaling {
        stat: BuffableStat::DefPercent,
        offset: None,
        cap: None,
    }),
}],
```

- [ ] **Step 4: Implement Flute of Ezpitzal ConditionalBuff**

Look up Flute of Ezpitzal's exact passive from wiki. Verify:
- Which attack types are buffed (Normal/Skill/all?)
- R1-R5 multiplier values

Then replace `conditional_buffs: &[]` with the appropriate ConditionalBuff using `Auto(StatScaling{DefPercent})`.

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data cinnabar_spindle_has_def_scaling flute_of_ezpitzal_has_def_scaling -- --nocapture`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/weapons/sword.rs
git commit -m "feat(data): add ConditionalBuff for Cinnabar Spindle and Flute of Ezpitzal"
```

#### Subtask 1b: Festering Desire [Pattern A — CritRate only]

- [ ] **Step 1: Write failing test**

```rust
#[test]
fn festering_desire_has_skill_cr_conditional() {
    let passive = FESTERING_DESIRE.passive.unwrap();
    // Existing StatBuff should still be there
    assert_eq!(passive.effect.buffs.len(), 1);
    assert_eq!(passive.effect.buffs[0].stat, BuffableStat::SkillDmgBonus);
    // New ConditionalBuff
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "festering_skill_cr");
    assert_eq!(buff.stat, BuffableStat::CritRate);
    assert!((buff.value - 0.06).abs() < 1e-6);
    assert!(buff.refinement_values.is_some());
    let rv = buff.refinement_values.unwrap();
    assert!((rv[0] - 0.06).abs() < 1e-6);
    assert!((rv[4] - 0.12).abs() < 1e-6);
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data festering_desire_has_skill_cr -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement**

In `FESTERING_DESIRE`, replace `conditional_buffs: &[]` with:

```rust
conditional_buffs: &[ConditionalBuff {
    name: "festering_skill_cr",
    description: "元素スキルのCRIT Rate+6-12%",
    stat: BuffableStat::CritRate,
    value: 0.06,
    refinement_values: Some([0.06, 0.075, 0.09, 0.105, 0.12]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

Keep the existing `buffs: &[StatBuff { stat: BuffableStat::SkillDmgBonus, ... }]` unchanged.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data festering_desire_has_skill_cr -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/sword.rs
git commit -m "feat(data): add CritRate ConditionalBuff for Festering Desire"
```

#### Subtask 1c: Remaining 6 Swords [Pattern A: Toggle]

Weapons: Lion's Roar (already implemented — test only), Prized Isshin Blade, Royal Longsword, Serenity's Call, Sturdy Bone, Toukabou Shigure.

- [ ] **Step 1: Look up TBV values**

Look up refinement values for:
- **Prized Isshin Blade**: ATK% after Skill hit — find R1-R5 values
- **Serenity's Call**: DMG bonus after Skill — find R1-R5 values
- **Sturdy Bone**: NA DMG after sprint — find R1-R5 values

Already known:
- **Lion's Roar**: DmgBonus: `[0.20, 0.24, 0.28, 0.32, 0.36]` (already implemented, test only)
- **Royal Longsword**: CritRate max 5 stacks: `[0.40, 0.50, 0.60, 0.70, 0.80]`
- **Toukabou Shigure**: DmgBonus: `[0.16, 0.20, 0.24, 0.28, 0.32]`

- [ ] **Step 2: Write tests for all 6 weapons (including Lion's Roar verification)**

```rust
#[test]
fn lions_roar_has_dmg_toggle() {
    let passive = LIONS_ROAR.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "lions_roar_dmg");
    assert_eq!(buff.stat, BuffableStat::DmgBonus);
    assert!((buff.value - 0.20).abs() < 1e-6);
    let rv = buff.refinement_values.unwrap();
    assert!((rv[0] - 0.20).abs() < 1e-6);
    assert!((rv[4] - 0.36).abs() < 1e-6);
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}
```

```rust
#[test]
fn prized_isshin_blade_has_atk_toggle() {
    let passive = PRIZED_ISSHIN_BLADE.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "prized_isshin_atk");
    assert_eq!(buff.stat, BuffableStat::AtkPercent);
    assert!(buff.refinement_values.is_some());
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}

#[test]
fn royal_longsword_has_cr_toggle() {
    let passive = ROYAL_LONGSWORD.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "royal_longsword_cr");
    assert_eq!(buff.stat, BuffableStat::CritRate);
    assert!((buff.value - 0.40).abs() < 1e-6);
    assert!(buff.refinement_values.is_some());
    let rv = buff.refinement_values.unwrap();
    assert!((rv[0] - 0.40).abs() < 1e-6);
    assert!((rv[4] - 0.80).abs() < 1e-6);
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}

#[test]
fn serenitys_call_has_dmg_toggle() {
    let passive = SERENITYS_CALL.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "serenitys_call_dmg");
    assert_eq!(buff.stat, BuffableStat::DmgBonus);
    assert!(buff.refinement_values.is_some());
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}

#[test]
fn sturdy_bone_has_na_dmg_toggle() {
    let passive = STURDY_BONE.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "sturdy_bone_na_dmg");
    assert_eq!(buff.stat, BuffableStat::NormalAtkDmgBonus);
    assert!(buff.refinement_values.is_some());
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}

#[test]
fn toukabou_shigure_has_dmg_toggle() {
    let passive = TOUKABOU_SHIGURE.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "toukabou_dmg");
    assert_eq!(buff.stat, BuffableStat::DmgBonus);
    assert!((buff.value - 0.16).abs() < 1e-6);
    assert!(buff.refinement_values.is_some());
    let rv = buff.refinement_values.unwrap();
    assert!((rv[0] - 0.16).abs() < 1e-6);
    assert!((rv[4] - 0.32).abs() < 1e-6);
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data prized_isshin_blade_has royal_longsword_has serenitys_call_has sturdy_bone_has toukabou_shigure_has -- --nocapture`
Expected: FAIL

- [ ] **Step 4: Implement all 5 Toggle ConditionalBuffs**

For each weapon, replace `conditional_buffs: &[]` with a `ConditionalBuff` using `Activation::Manual(ManualCondition::Toggle)`. Use the looked-up values for TBV weapons.

**Royal Longsword** (known values):
```rust
conditional_buffs: &[ConditionalBuff {
    name: "royal_longsword_cr",
    description: "ダメージ命中で会心でなければCRIT Rate+8%×5スタック（最大40-80%）",
    stat: BuffableStat::CritRate,
    value: 0.40,
    refinement_values: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

**Toukabou Shigure** (known values):
```rust
conditional_buffs: &[ConditionalBuff {
    name: "toukabou_dmg",
    description: "落下攻撃命中後にDMG+16-32%",
    stat: BuffableStat::DmgBonus,
    value: 0.16,
    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

Follow the same pattern for Prized Isshin Blade, Serenity's Call, Sturdy Bone with looked-up values.

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data prized_isshin_blade_has royal_longsword_has serenitys_call_has sturdy_bone_has toukabou_shigure_has -- --nocapture`
Expected: PASS

- [ ] **Step 6: Run all sword tests + clippy**

Run: `cargo test -p genshin-calc-data -- sword --nocapture && cargo clippy -p genshin-calc-data -- -D warnings`
Expected: All PASS, no warnings

- [ ] **Step 7: Commit**

```bash
git add crates/data/src/weapons/sword.rs
git commit -m "feat(data): add Toggle ConditionalBuff for 5 swords (Royal Longsword, Toukabou, etc.)"
```

---

### Task 2: Claymores (2 weapons)

**Files:**
- Modify: `crates/data/src/weapons/claymore.rs`

- [ ] **Step 1: Look up TBV values**

Look up refinement values for:
- **Earth Shaker**: BurstDmgBonus after Skill hit — find R1-R5 values
- **Master Key**: ElementalMastery after Skill — find R1-R5 values

- [ ] **Step 2: Write failing tests**

```rust
#[test]
fn earth_shaker_has_burst_dmg_toggle() {
    let passive = EARTH_SHAKER.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "earth_shaker_burst_dmg");
    assert_eq!(buff.stat, BuffableStat::BurstDmgBonus);
    assert!(buff.refinement_values.is_some());
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}

#[test]
fn master_key_has_em_toggle() {
    let passive = MASTER_KEY.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "master_key_em");
    assert_eq!(buff.stat, BuffableStat::ElementalMastery);
    assert!(buff.refinement_values.is_some());
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data earth_shaker_has master_key_has -- --nocapture`
Expected: FAIL

- [ ] **Step 4: Implement both ConditionalBuffs**

Replace `conditional_buffs: &[]` in both weapons with Toggle ConditionalBuff using looked-up values.

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data earth_shaker_has master_key_has -- --nocapture`
Expected: PASS

- [ ] **Step 6: Run clippy**

Run: `cargo clippy -p genshin-calc-data -- -D warnings`

- [ ] **Step 7: Commit**

```bash
git add crates/data/src/weapons/claymore.rs
git commit -m "feat(data): add Toggle ConditionalBuff for Earth Shaker and Master Key"
```

---

### Task 3: Bows (8 weapons)

**Files:**
- Modify: `crates/data/src/weapons/bow.rs`

#### Subtask 3a: Fading Twilight [Pattern D: Stacks(3)]

- [ ] **Step 1: Write failing test**

```rust
#[test]
fn fading_twilight_has_dmg_stacks() {
    let passive = FADING_TWILIGHT.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "fading_twilight_dmg");
    assert_eq!(buff.stat, BuffableStat::DmgBonus);
    assert!((buff.value - 0.06).abs() < 1e-6);
    assert!(buff.refinement_values.is_none());
    assert!(buff.stack_values.is_some());
    let sv = buff.stack_values.unwrap();
    assert_eq!(sv.len(), 3);
    assert!((sv[0] - 0.06).abs() < 1e-6);
    assert!((sv[1] - 0.10).abs() < 1e-6);
    assert!((sv[2] - 0.14).abs() < 1e-6);
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Stacks(3))
    ));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data fading_twilight_has -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement**

In `FADING_TWILIGHT`, replace `conditional_buffs: &[]` with:

```rust
conditional_buffs: &[ConditionalBuff {
    name: "fading_twilight_dmg",
    description: "夕暮(6%)/流明(10%)/朝暉(14%)の3状態を循環してDMGアップ",
    stat: BuffableStat::DmgBonus,
    value: 0.06,
    refinement_values: None,
    stack_values: Some(&[0.06, 0.10, 0.14]),
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Stacks(3)),
}],
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data fading_twilight_has -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/bow.rs
git commit -m "feat(data): add Stacks(3) ConditionalBuff for Fading Twilight"
```

#### Subtask 3b: Hamayumi [Pattern E: StatBuff + Toggle hybrid]

- [ ] **Step 1: Write failing test**

```rust
#[test]
fn hamayumi_has_statbuffs_and_energy_toggle() {
    let passive = HAMAYUMI.passive.unwrap();
    // Unconditional StatBuffs
    assert_eq!(passive.effect.buffs.len(), 2);
    assert_eq!(passive.effect.buffs[0].stat, BuffableStat::NormalAtkDmgBonus);
    assert!((passive.effect.buffs[0].value - 0.16).abs() < 1e-6);
    assert_eq!(passive.effect.buffs[1].stat, BuffableStat::ChargedAtkDmgBonus);
    assert!((passive.effect.buffs[1].value - 0.12).abs() < 1e-6);
    // Conditional Toggle buffs (energy full)
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 2);
    let na_buff = &cond_buffs[0];
    assert_eq!(na_buff.name, "hamayumi_full_energy_na");
    assert_eq!(na_buff.stat, BuffableStat::NormalAtkDmgBonus);
    assert!((na_buff.value - 0.16).abs() < 1e-6);
    assert!(matches!(
        na_buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
    let ca_buff = &cond_buffs[1];
    assert_eq!(ca_buff.name, "hamayumi_full_energy_ca");
    assert_eq!(ca_buff.stat, BuffableStat::ChargedAtkDmgBonus);
    assert!((ca_buff.value - 0.12).abs() < 1e-6);
    assert!(matches!(
        ca_buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data hamayumi_has_statbuffs -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement**

In `HAMAYUMI`, replace the entire `effect: PassiveEffect { ... }` with:

```rust
effect: PassiveEffect {
    description: "NA DMG+16-32%/CA DMG+12-24%。エネルギー満タンでさらにNA DMG+16-32%/CA DMG+12-24%",
    buffs: &[
        StatBuff {
            stat: BuffableStat::NormalAtkDmgBonus,
            value: 0.16,
            refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
        },
        StatBuff {
            stat: BuffableStat::ChargedAtkDmgBonus,
            value: 0.12,
            refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
        },
    ],
    conditional_buffs: &[
        ConditionalBuff {
            name: "hamayumi_full_energy_na",
            description: "エネルギー満タン時にNA DMG+16-32%",
            stat: BuffableStat::NormalAtkDmgBonus,
            value: 0.16,
            refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Toggle),
        },
        ConditionalBuff {
            name: "hamayumi_full_energy_ca",
            description: "エネルギー満タン時にCA DMG+12-24%",
            stat: BuffableStat::ChargedAtkDmgBonus,
            value: 0.12,
            refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Toggle),
        },
    ],
},
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p genshin-calc-data hamayumi_has_statbuffs -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/bow.rs
git commit -m "feat(data): add StatBuff + Toggle ConditionalBuff for Hamayumi"
```

#### Subtask 3c: Multi-buff bows [Pattern B]

Weapons: Mitternachts Waltz (2 buffs), Predator (2 buffs), Sequence of Solitude (2 buffs, TBV)

- [ ] **Step 1: Look up TBV values for Sequence of Solitude**

Look up Sequence of Solitude R1-R5 values for SkillDmgBonus and BurstDmgBonus.

- [ ] **Step 2: Write failing tests**

```rust
#[test]
fn mitternachts_waltz_has_cross_buff_toggle() {
    let passive = MITTERNACHTS_WALTZ.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 2);
    let skill_buff = &cond_buffs[0];
    assert_eq!(skill_buff.name, "mitternachts_skill_dmg");
    assert_eq!(skill_buff.stat, BuffableStat::SkillDmgBonus);
    assert!((skill_buff.value - 0.20).abs() < 1e-6);
    assert!(matches!(
        skill_buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
    let na_buff = &cond_buffs[1];
    assert_eq!(na_buff.name, "mitternachts_na_dmg");
    assert_eq!(na_buff.stat, BuffableStat::NormalAtkDmgBonus);
    assert!((na_buff.value - 0.20).abs() < 1e-6);
}

#[test]
fn predator_has_fixed_na_ca_toggle() {
    let passive = PREDATOR.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 2);
    let na_buff = &cond_buffs[0];
    assert_eq!(na_buff.name, "predator_na_dmg");
    assert_eq!(na_buff.stat, BuffableStat::NormalAtkDmgBonus);
    assert!((na_buff.value - 0.10).abs() < 1e-6);
    // All refinement values are 0.10 (PS exclusive, no refinement)
    let rv = na_buff.refinement_values.unwrap();
    for v in &rv {
        assert!((*v - 0.10).abs() < 1e-6);
    }
    let ca_buff = &cond_buffs[1];
    assert_eq!(ca_buff.name, "predator_ca_dmg");
    assert_eq!(ca_buff.stat, BuffableStat::ChargedAtkDmgBonus);
    assert!((ca_buff.value - 0.10).abs() < 1e-6);
}

#[test]
fn sequence_of_solitude_has_skill_burst_toggle() {
    let passive = SEQUENCE_OF_SOLITUDE.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 2);
    assert_eq!(cond_buffs[0].name, "sequence_skill_dmg");
    assert_eq!(cond_buffs[0].stat, BuffableStat::SkillDmgBonus);
    assert_eq!(cond_buffs[1].name, "sequence_burst_dmg");
    assert_eq!(cond_buffs[1].stat, BuffableStat::BurstDmgBonus);
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data mitternachts_waltz_has predator_has_fixed sequence_of_solitude_has -- --nocapture`
Expected: FAIL

- [ ] **Step 4: Implement all 3 weapons**

**Mitternachts Waltz:**
```rust
conditional_buffs: &[
    ConditionalBuff {
        name: "mitternachts_skill_dmg",
        description: "通常攻撃命中時にSkill DMG+20-40%",
        stat: BuffableStat::SkillDmgBonus,
        value: 0.20,
        refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
    ConditionalBuff {
        name: "mitternachts_na_dmg",
        description: "元素スキル命中時にNA DMG+20-40%",
        stat: BuffableStat::NormalAtkDmgBonus,
        value: 0.20,
        refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
],
```

**Predator** (fixed 10% at all refinements):
```rust
conditional_buffs: &[
    ConditionalBuff {
        name: "predator_na_dmg",
        description: "Cryo命中後にNA DMG+10%（精錬固定）",
        stat: BuffableStat::NormalAtkDmgBonus,
        value: 0.10,
        refinement_values: Some([0.10, 0.10, 0.10, 0.10, 0.10]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
    ConditionalBuff {
        name: "predator_ca_dmg",
        description: "Cryo命中後にCA DMG+10%（精錬固定）",
        stat: BuffableStat::ChargedAtkDmgBonus,
        value: 0.10,
        refinement_values: Some([0.10, 0.10, 0.10, 0.10, 0.10]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
],
```

**Sequence of Solitude:** use looked-up R1-R5 values.

- [ ] **Step 5: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data mitternachts_waltz_has predator_has_fixed sequence_of_solitude_has -- --nocapture`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/weapons/bow.rs
git commit -m "feat(data): add multi-buff Toggle ConditionalBuff for 3 bows"
```

#### Subtask 3d: Single-buff bows [Pattern A]

Weapons: Prototype Crescent, Snare Hook (TBV), Windblume Ode

- [ ] **Step 1: Look up TBV values for Snare Hook**

- [ ] **Step 2: Write failing tests**

```rust
#[test]
fn prototype_crescent_has_atk_toggle() {
    let passive = PROTOTYPE_CRESCENT.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "prototype_crescent_atk");
    assert_eq!(buff.stat, BuffableStat::AtkPercent);
    assert!((buff.value - 0.36).abs() < 1e-6);
    let rv = buff.refinement_values.unwrap();
    assert!((rv[4] - 0.72).abs() < 1e-6);
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}

#[test]
fn snare_hook_has_dmg_toggle() {
    let passive = SNARE_HOOK.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "snare_hook_dmg");
    assert_eq!(buff.stat, BuffableStat::DmgBonus);
    assert!(buff.refinement_values.is_some());
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}

#[test]
fn windblume_ode_has_atk_toggle() {
    let passive = WINDBLUME_ODE.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "windblume_atk");
    assert_eq!(buff.stat, BuffableStat::AtkPercent);
    assert!((buff.value - 0.16).abs() < 1e-6);
    let rv = buff.refinement_values.unwrap();
    assert!((rv[4] - 0.32).abs() < 1e-6);
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data prototype_crescent_has snare_hook_has windblume_ode_has -- --nocapture`
Expected: FAIL

- [ ] **Step 4: Implement all 3 weapons**

**Prototype Crescent:**
```rust
conditional_buffs: &[ConditionalBuff {
    name: "prototype_crescent_atk",
    description: "弱点命中時にATK+36-72%",
    stat: BuffableStat::AtkPercent,
    value: 0.36,
    refinement_values: Some([0.36, 0.45, 0.54, 0.63, 0.72]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

**Windblume Ode:**
```rust
conditional_buffs: &[ConditionalBuff {
    name: "windblume_atk",
    description: "元素スキル使用後にATK+16-32%",
    stat: BuffableStat::AtkPercent,
    value: 0.16,
    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

**Snare Hook:** use looked-up values.

- [ ] **Step 5: Run tests to verify they pass + clippy**

Run: `cargo test -p genshin-calc-data prototype_crescent_has snare_hook_has windblume_ode_has -- --nocapture && cargo clippy -p genshin-calc-data -- -D warnings`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/weapons/bow.rs
git commit -m "feat(data): add Toggle ConditionalBuff for Prototype Crescent, Snare Hook, Windblume Ode"
```

---

### Task 4: Catalysts (9 weapons)

**Files:**
- Modify: `crates/data/src/weapons/catalyst.rs`

#### Subtask 4a: Multi-buff catalysts [Pattern B]

Weapons: Dodoco Tales (2), Sacrificial Jade (2), Solar Pearl (3), The Widsith (3)

- [ ] **Step 1: Write failing tests**

```rust
#[test]
fn dodoco_tales_has_ca_dmg_and_atk_toggle() {
    let passive = DODOCO_TALES.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 2);
    let ca_buff = &cond_buffs[0];
    assert_eq!(ca_buff.name, "dodoco_ca_dmg");
    assert_eq!(ca_buff.stat, BuffableStat::ChargedAtkDmgBonus);
    assert!((ca_buff.value - 0.16).abs() < 1e-6);
    let atk_buff = &cond_buffs[1];
    assert_eq!(atk_buff.name, "dodoco_atk");
    assert_eq!(atk_buff.stat, BuffableStat::AtkPercent);
    assert!((atk_buff.value - 0.08).abs() < 1e-6);
}

#[test]
fn sacrificial_jade_has_hp_and_em_toggle() {
    let passive = SACRIFICIAL_JADE.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 2);
    let hp_buff = &cond_buffs[0];
    assert_eq!(hp_buff.name, "sacrificial_jade_hp");
    assert_eq!(hp_buff.stat, BuffableStat::HpPercent);
    assert!((hp_buff.value - 0.32).abs() < 1e-6);
    let rv = hp_buff.refinement_values.unwrap();
    assert!((rv[4] - 0.64).abs() < 1e-6);
    let em_buff = &cond_buffs[1];
    assert_eq!(em_buff.name, "sacrificial_jade_em");
    assert_eq!(em_buff.stat, BuffableStat::ElementalMastery);
    assert!((em_buff.value - 40.0).abs() < 1e-6);
    let rv2 = em_buff.refinement_values.unwrap();
    assert!((rv2[4] - 80.0).abs() < 1e-6);
}

#[test]
fn solar_pearl_has_skill_burst_na_toggle() {
    let passive = SOLAR_PEARL.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 3);
    assert_eq!(cond_buffs[0].name, "solar_pearl_skill_dmg");
    assert_eq!(cond_buffs[0].stat, BuffableStat::SkillDmgBonus);
    assert!((cond_buffs[0].value - 0.20).abs() < 1e-6);
    assert_eq!(cond_buffs[1].name, "solar_pearl_burst_dmg");
    assert_eq!(cond_buffs[1].stat, BuffableStat::BurstDmgBonus);
    assert!((cond_buffs[1].value - 0.20).abs() < 1e-6);
    assert_eq!(cond_buffs[2].name, "solar_pearl_na_dmg");
    assert_eq!(cond_buffs[2].stat, BuffableStat::NormalAtkDmgBonus);
    assert!((cond_buffs[2].value - 0.20).abs() < 1e-6);
}

#[test]
fn widsith_has_three_random_toggles() {
    let passive = THE_WIDSITH.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 3);
    let atk_buff = &cond_buffs[0];
    assert_eq!(atk_buff.name, "widsith_atk");
    assert_eq!(atk_buff.stat, BuffableStat::AtkPercent);
    assert!((atk_buff.value - 0.60).abs() < 1e-6);
    let rv = atk_buff.refinement_values.unwrap();
    assert!((rv[4] - 1.20).abs() < 1e-6);
    let dmg_buff = &cond_buffs[1];
    assert_eq!(dmg_buff.name, "widsith_dmg");
    assert_eq!(dmg_buff.stat, BuffableStat::DmgBonus);
    assert!((dmg_buff.value - 0.48).abs() < 1e-6);
    let em_buff = &cond_buffs[2];
    assert_eq!(em_buff.name, "widsith_em");
    assert_eq!(em_buff.stat, BuffableStat::ElementalMastery);
    assert!((em_buff.value - 240.0).abs() < 1e-6);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data dodoco_tales_has sacrificial_jade_has solar_pearl_has widsith_has -- --nocapture`
Expected: FAIL

- [ ] **Step 3: Implement all 4 weapons**

**Dodoco Tales:**
```rust
conditional_buffs: &[
    ConditionalBuff {
        name: "dodoco_ca_dmg",
        description: "通常攻撃命中時にCA DMG+16-32%",
        stat: BuffableStat::ChargedAtkDmgBonus,
        value: 0.16,
        refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
    ConditionalBuff {
        name: "dodoco_atk",
        description: "重撃命中時にATK+8-16%",
        stat: BuffableStat::AtkPercent,
        value: 0.08,
        refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
],
```

**Sacrificial Jade:**
```rust
conditional_buffs: &[
    ConditionalBuff {
        name: "sacrificial_jade_hp",
        description: "フィールドに出た時にHP+32-64%",
        stat: BuffableStat::HpPercent,
        value: 0.32,
        refinement_values: Some([0.32, 0.40, 0.48, 0.56, 0.64]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
    ConditionalBuff {
        name: "sacrificial_jade_em",
        description: "フィールドに出た時にEM+40-80",
        stat: BuffableStat::ElementalMastery,
        value: 40.0,
        refinement_values: Some([40.0, 50.0, 60.0, 70.0, 80.0]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
],
```

**Solar Pearl:**
```rust
conditional_buffs: &[
    ConditionalBuff {
        name: "solar_pearl_skill_dmg",
        description: "通常攻撃命中後にSkill DMG+20-40%",
        stat: BuffableStat::SkillDmgBonus,
        value: 0.20,
        refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
    ConditionalBuff {
        name: "solar_pearl_burst_dmg",
        description: "通常攻撃命中後にBurst DMG+20-40%",
        stat: BuffableStat::BurstDmgBonus,
        value: 0.20,
        refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
    ConditionalBuff {
        name: "solar_pearl_na_dmg",
        description: "元素スキル/爆発命中後にNA DMG+20-40%",
        stat: BuffableStat::NormalAtkDmgBonus,
        value: 0.20,
        refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
],
```

**The Widsith:**
```rust
conditional_buffs: &[
    ConditionalBuff {
        name: "widsith_atk",
        description: "登場時にランダム: ATK+60-120%",
        stat: BuffableStat::AtkPercent,
        value: 0.60,
        refinement_values: Some([0.60, 0.75, 0.90, 1.05, 1.20]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
    ConditionalBuff {
        name: "widsith_dmg",
        description: "登場時にランダム: 全元素DMG+48-96%",
        stat: BuffableStat::DmgBonus,
        value: 0.48,
        refinement_values: Some([0.48, 0.60, 0.72, 0.84, 0.96]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
    ConditionalBuff {
        name: "widsith_em",
        description: "登場時にランダム: EM+240-480",
        stat: BuffableStat::ElementalMastery,
        value: 240.0,
        refinement_values: Some([240.0, 300.0, 360.0, 420.0, 480.0]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
],
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data dodoco_tales_has sacrificial_jade_has solar_pearl_has widsith_has -- --nocapture`
Expected: PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/catalyst.rs
git commit -m "feat(data): add multi-buff Toggle ConditionalBuff for 4 catalysts"
```

#### Subtask 4b: Single-buff catalysts [Pattern A]

Weapons: Blackmarrow Lantern (TBV), Dawning Frost (TBV), Etherlight Spindlelute (TBV), Flowing Purity (TBV), Wine and Song

- [ ] **Step 1: Look up TBV values**

Look up refinement values for Blackmarrow Lantern, Dawning Frost, Etherlight Spindlelute, Flowing Purity.

- [ ] **Step 2: Write failing tests**

```rust
#[test]
fn blackmarrow_lantern_has_dmg_toggle() {
    let passive = BLACKMARROW_LANTERN.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "blackmarrow_dmg");
    assert_eq!(buff.stat, BuffableStat::DmgBonus);
    assert!(buff.refinement_values.is_some());
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}

#[test]
fn dawning_frost_has_dmg_toggle() {
    let passive = DAWNING_FROST.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "dawning_frost_dmg");
    assert_eq!(buff.stat, BuffableStat::DmgBonus);
    assert!(buff.refinement_values.is_some());
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}

#[test]
fn etherlight_spindlelute_has_dmg_toggle() {
    let passive = ETHERLIGHT_SPINDLELUTE.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "etherlight_dmg");
    assert_eq!(buff.stat, BuffableStat::DmgBonus);
    assert!(buff.refinement_values.is_some());
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}

#[test]
fn flowing_purity_has_dmg_toggle() {
    let passive = FLOWING_PURITY.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "flowing_purity_dmg");
    assert_eq!(buff.stat, BuffableStat::DmgBonus);
    assert!(buff.refinement_values.is_some());
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}

#[test]
fn wine_and_song_has_atk_toggle() {
    let passive = WINE_AND_SONG.passive.unwrap();
    let cond_buffs = passive.effect.conditional_buffs;
    assert_eq!(cond_buffs.len(), 1);
    let buff = &cond_buffs[0];
    assert_eq!(buff.name, "wine_and_song_atk");
    assert_eq!(buff.stat, BuffableStat::AtkPercent);
    assert!((buff.value - 0.20).abs() < 1e-6);
    let rv = buff.refinement_values.unwrap();
    assert!((rv[4] - 0.40).abs() < 1e-6);
    assert!(matches!(
        buff.activation,
        Activation::Manual(ManualCondition::Toggle)
    ));
}
```

- [ ] **Step 3: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data blackmarrow_lantern_has dawning_frost_has etherlight_spindlelute_has flowing_purity_has wine_and_song_has -- --nocapture`
Expected: FAIL

- [ ] **Step 4: Implement all 5 weapons**

**Wine and Song** (known values):
```rust
conditional_buffs: &[ConditionalBuff {
    name: "wine_and_song_atk",
    description: "ダッシュ後にATK+20-40%",
    stat: BuffableStat::AtkPercent,
    value: 0.20,
    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

Implement remaining 4 TBV weapons with looked-up values following the same pattern.

- [ ] **Step 5: Run tests to verify they pass + clippy**

Run: `cargo test -p genshin-calc-data blackmarrow_lantern_has dawning_frost_has etherlight_spindlelute_has flowing_purity_has wine_and_song_has -- --nocapture && cargo clippy -p genshin-calc-data -- -D warnings`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/weapons/catalyst.rs
git commit -m "feat(data): add Toggle ConditionalBuff for 5 catalysts"
```

---

### Task 5: Final Verification

- [ ] **Step 1: Run full test suite**

Run: `cargo test`
Expected: All tests pass (existing + 28 new tests, 27 new implementations + 1 Lion's Roar verification)

- [ ] **Step 2: Run clippy on entire workspace**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

- [ ] **Step 3: Run data integrity tests**

Run: `cargo test -p genshin-calc-data test_all_conditional_buff_names_unique test_all_stacks_max_positive -- --nocapture`
Expected: PASS — all new buff names are unique, no Stacks(0) violations

- [ ] **Step 4: Verify test count increased**

Run: `cargo test -p genshin-calc-data 2>&1 | tail -1`
Expected: test result should show ~228 tests (200 existing + 28 new tests)

- [ ] **Step 5: Commit if any remaining changes**

```bash
git add -A
git commit -m "feat(data): complete P3 4-star weapon ConditionalBuff (28 weapons)"
```
