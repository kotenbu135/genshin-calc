# Constellation Conditional Buffs Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add ~62 missing constellation conditional buffs across 33 characters in 7 element files.

**Architecture:** Each element's `crates/data/src/talent_buffs/<element>.rs` is independent. New `TalentBuffDef` entries are appended to existing character `_BUFFS` arrays with `min_constellation` gating. Tests verify C0 exclusion and C6 inclusion.

**Tech Stack:** Rust, cargo test

**Parallelism:** All 7 tasks are independent and can run in parallel worktrees. Each task modifies only its own `<element>.rs` file. Tests are added to `mod.rs` — since this is a shared file, **merge must be sequential** (Pyro first, then Hydro rebased on Pyro, etc.). The `<element>.rs` changes never conflict.

---

## Reference: TalentBuffDef Template

Every new entry follows this pattern. Fields not shown use the values below unless specified otherwise in the task.

```rust
TalentBuffDef {
    name: "<character>_<constellation>_<short_name>",
    description: "<Cx>: <human-readable description>",
    stat: BuffableStat::<Variant>,
    base_value: <f64>,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,       // or Some(ScalingStat::Hp) etc
    target: BuffTarget::<Target>,
    source: TalentBuffSource::Constellation(<n>),
    min_constellation: <n>,
    cap: None,             // or Some(<f64>)
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
}
```

## Reference: Test Template

Each element task includes tests. Use this pattern:

```rust
#[test]
fn test_conditional_<character>_c<n>() {
    let c0 = get_talent_conditional_buffs("<character_id>", 0, &[10, 10, 10]);
    let c6 = get_talent_conditional_buffs("<character_id>", 6, &[10, 10, 10]);
    // C0 should NOT contain the constellation buff
    assert!(!c0.iter().any(|b| b.name == "<buff_name>"));
    // C6 should contain it
    let buff = c6.iter().find(|b| b.name == "<buff_name>").unwrap();
    assert_eq!(buff.stat, BuffableStat::<Expected>);
    let epsilon = 1e-9;
    assert!((buff.value - <expected_value>).abs() < epsilon);
}
```

---

## Task 1: Pyro Constellation Buffs

**Files:**
- Modify: `crates/data/src/talent_buffs/pyro.rs`
- Modify: `crates/data/src/talent_buffs/mod.rs` (tests)

**Characters:** chevreuse, durin, klee, xiangling, xinyan, yoimiya (6 chars, 13 entries)
**Merge order:** 1st (base)

- [ ] **Step 1: Read existing pyro.rs to find insertion points**

Read the file to identify each character's existing `_BUFFS` array and determine where to append new entries.

- [ ] **Step 2: Add Chevreuse C6 entries (2 entries)**

Append to `CHEVREUSE_BUFFS`:
```rust
// C6: After healing by Skill, party gains Pyro/Electro DMG +20% per stack (max 3)
TalentBuffDef {
    name: "chevreuse_c6_pyro_dmg",
    description: "C6: Party Pyro DMG Bonus +20% per stack (max 3)",
    stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
    base_value: 0.20,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
},
TalentBuffDef {
    name: "chevreuse_c6_electro_dmg",
    description: "C6: Party Electro DMG Bonus +20% per stack (max 3)",
    stat: BuffableStat::ElementalDmgBonus(Element::Electro),
    base_value: 0.20,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
},
```

- [ ] **Step 3: Add Durin C4, C6 entries (3 entries)**

Append to `DURIN_BUFFS`:
```rust
// C4: Burst DMG +40%
TalentBuffDef {
    name: "durin_c4_burst_dmg",
    description: "C4: Elemental Burst DMG +40%",
    stat: BuffableStat::BurstDmgBonus,
    base_value: 0.40,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
// C6: DEF Ignore 30%
TalentBuffDef {
    name: "durin_c6_def_ignore",
    description: "C6: Burst DMG ignores 30% of opponents' DEF",
    stat: BuffableStat::DefIgnore,
    base_value: 0.30,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
// C6: DEF Reduction 30% (Light form)
TalentBuffDef {
    name: "durin_c6_def_reduction",
    description: "C6: Light form decreases opponent DEF by 30%",
    stat: BuffableStat::DefReduction,
    base_value: 0.30,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 4: Add Klee C2, C6 entries (2 entries)**

Append to `KLEE_BUFFS`:
```rust
// C2: Mine DEF reduction 23%
TalentBuffDef {
    name: "klee_c2_def_reduction",
    description: "C2: Jumpy Dumpty mines reduce opponent DEF by 23%",
    stat: BuffableStat::DefReduction,
    base_value: 0.23,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
// C6: Party Pyro DMG +10%
TalentBuffDef {
    name: "klee_c6_pyro_dmg",
    description: "C6: Sparks 'n' Splash grants party Pyro DMG Bonus +10%",
    stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
    base_value: 0.10,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 5: Add Xiangling C1, C6 entries (2 entries)**

Append to `XIANGLING_BUFFS`:
```rust
// C1: Guoba Pyro RES shred -15%
TalentBuffDef {
    name: "xiangling_c1_pyro_res_shred",
    description: "C1: Opponents hit by Guoba have Pyro RES -15%",
    stat: BuffableStat::ElementalResReduction(Element::Pyro),
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(1),
    min_constellation: 1,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
// C6: Pyronado Pyro DMG +15%
TalentBuffDef {
    name: "xiangling_c6_pyro_dmg",
    description: "C6: During Pyronado, party gains Pyro DMG Bonus +15%",
    stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 6: Add Xinyan C2, C6 entries (2 entries)**

Append to `XINYAN_BUFFS`:
```rust
// C2: Burst Physical CRIT Rate +100% (approximation, applied generically like Sara C6)
TalentBuffDef {
    name: "xinyan_c2_burst_crit",
    description: "C2: Riff Revolution Physical DMG CRIT Rate +100% (approximation)",
    stat: BuffableStat::CritRate,
    base_value: 1.00,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
// C6: Charged ATK gains flat DMG = 50% DEF
TalentBuffDef {
    name: "xinyan_c6_charged_def_scaling",
    description: "C6: Charged ATK gains ATK bonus equal to 50% of DEF",
    stat: BuffableStat::ChargedAtkFlatDmg,
    base_value: 0.50,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: Some(ScalingStat::Def),
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 7: Add Yoimiya C1, C2 entries (2 entries)**

Append to `YOIMIYA_BUFFS`:
```rust
// C1: ATK +20% on Aurous Blaze defeat
TalentBuffDef {
    name: "yoimiya_c1_atk",
    description: "C1: ATK +20% for 20s when Aurous Blaze-affected opponent defeated",
    stat: BuffableStat::AtkPercent,
    base_value: 0.20,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(1),
    min_constellation: 1,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
// C2: Pyro DMG +25% on Pyro CRIT
TalentBuffDef {
    name: "yoimiya_c2_pyro_dmg",
    description: "C2: Pyro DMG Bonus +25% for 6s on Pyro CRIT hit",
    stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
    base_value: 0.25,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 8: Run cargo build to verify compilation**

Run: `cargo build -p genshin-calc-data 2>&1 | tail -5`
Expected: `Finished`

- [ ] **Step 9: Write tests in mod.rs**

Add to `#[cfg(test)]` in `mod.rs`:
```rust
#[test]
fn test_conditional_chevreuse_c6() {
    let c0 = get_talent_conditional_buffs("chevreuse", 0, &[10, 10, 10]);
    let c6 = get_talent_conditional_buffs("chevreuse", 6, &[10, 10, 10]);
    assert!(!c0.iter().any(|b| b.name == "chevreuse_c6_pyro_dmg"));
    assert!(c6.iter().any(|b| b.name == "chevreuse_c6_pyro_dmg"));
    assert!(c6.iter().any(|b| b.name == "chevreuse_c6_electro_dmg"));
}

#[test]
fn test_conditional_klee_c2_c6() {
    let c0 = get_talent_conditional_buffs("klee", 0, &[10, 10, 10]);
    let c2 = get_talent_conditional_buffs("klee", 2, &[10, 10, 10]);
    let c6 = get_talent_conditional_buffs("klee", 6, &[10, 10, 10]);
    assert!(!c0.iter().any(|b| b.name == "klee_c2_def_reduction"));
    assert!(c2.iter().any(|b| b.name == "klee_c2_def_reduction"));
    assert!(!c2.iter().any(|b| b.name == "klee_c6_pyro_dmg"));
    assert!(c6.iter().any(|b| b.name == "klee_c6_pyro_dmg"));
}

#[test]
fn test_conditional_xiangling_c1_c6() {
    let c0 = get_talent_conditional_buffs("xiangling", 0, &[10, 10, 10]);
    let c1 = get_talent_conditional_buffs("xiangling", 1, &[10, 10, 10]);
    let c6 = get_talent_conditional_buffs("xiangling", 6, &[10, 10, 10]);
    assert!(!c0.iter().any(|b| b.name == "xiangling_c1_pyro_res_shred"));
    assert!(c1.iter().any(|b| b.name == "xiangling_c1_pyro_res_shred"));
    let shred = c1.iter().find(|b| b.name == "xiangling_c1_pyro_res_shred").unwrap();
    assert_eq!(shred.stat, BuffableStat::ElementalResReduction(Element::Pyro));
    assert!((shred.value - 0.15).abs() < 1e-9);
    assert!(c6.iter().any(|b| b.name == "xiangling_c6_pyro_dmg"));
}

#[test]
fn test_conditional_xinyan_c2_c6() {
    let c0 = get_talent_conditional_buffs("xinyan", 0, &[10, 10, 10]);
    let c6 = get_talent_conditional_buffs("xinyan", 6, &[10, 10, 10]);
    assert!(!c0.iter().any(|b| b.name == "xinyan_c2_burst_crit"));
    assert!(c6.iter().any(|b| b.name == "xinyan_c2_burst_crit"));
    assert!(c6.iter().any(|b| b.name == "xinyan_c6_charged_def_scaling"));
    let def_scaling = c6.iter().find(|b| b.name == "xinyan_c6_charged_def_scaling").unwrap();
    assert_eq!(def_scaling.scales_on, Some(ScalingStat::Def));
}

#[test]
fn test_conditional_yoimiya_c1_c2() {
    let c0 = get_talent_conditional_buffs("yoimiya", 0, &[10, 10, 10]);
    let c2 = get_talent_conditional_buffs("yoimiya", 2, &[10, 10, 10]);
    assert!(!c0.iter().any(|b| b.name == "yoimiya_c1_atk"));
    assert!(c2.iter().any(|b| b.name == "yoimiya_c1_atk"));
    assert!(c2.iter().any(|b| b.name == "yoimiya_c2_pyro_dmg"));
}

#[test]
fn test_conditional_durin_c4_c6() {
    let c0 = get_talent_conditional_buffs("durin", 0, &[10, 10, 10]);
    let c6 = get_talent_conditional_buffs("durin", 6, &[10, 10, 10]);
    assert!(!c0.iter().any(|b| b.name == "durin_c4_burst_dmg"));
    assert!(c6.iter().any(|b| b.name == "durin_c4_burst_dmg"));
    assert!(c6.iter().any(|b| b.name == "durin_c6_def_ignore"));
    assert!(c6.iter().any(|b| b.name == "durin_c6_def_reduction"));
}
```

- [ ] **Step 10: Run tests**

Run: `cargo test -p genshin-calc-data -- test_conditional_chevreuse test_conditional_klee test_conditional_xiangling test_conditional_xinyan test_conditional_yoimiya test_conditional_durin 2>&1 | tail -10`
Expected: All tests PASS

- [ ] **Step 11: Run full test suite to check no regressions**

Run: `cargo test -p genshin-calc-data 2>&1 | tail -5`
Expected: All tests pass

- [ ] **Step 12: Commit**

```bash
git add crates/data/src/talent_buffs/pyro.rs crates/data/src/talent_buffs/mod.rs
git commit -m "feat: add constellation conditional buffs for Pyro characters (#51)

Add 13 new TalentBuffDef entries for 6 Pyro characters:
- Chevreuse C6: Pyro/Electro DMG Bonus
- Durin C4/C6: Burst DMG, DEF Ignore, DEF Reduction
- Klee C2/C6: DEF Reduction, Pyro DMG Bonus
- Xiangling C1/C6: Pyro RES Shred, Pyro DMG Bonus
- Xinyan C2/C6: CRIT Rate, Charged ATK Flat DMG
- Yoimiya C1/C2: ATK%, Pyro DMG Bonus"
```

---

## Task 2: Hydro Constellation Buffs

**Files:**
- Modify: `crates/data/src/talent_buffs/hydro.rs`
- Modify: `crates/data/src/talent_buffs/mod.rs` (tests)

**Characters:** aino, barbara, candace, columbina, mona, mualani, nilou, xingqiu, yelan (9 chars, 24 entries)
**Merge order:** 2nd (rebase on Pyro)

- [ ] **Step 1: Read existing hydro.rs to find insertion points**

- [ ] **Step 2: Add Aino C1, C6 entries (2 entries)**

```rust
TalentBuffDef {
    name: "aino_c1_em",
    description: "C1: After Skill/Burst, party EM +80",
    stat: BuffableStat::ElementalMastery,
    base_value: 80.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(1),
    min_constellation: 1,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "aino_c6_reaction_dmg",
    description: "C6: Electro-Charged/Bloom/Lunar-Charged reaction DMG +15%",
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 3: Add Barbara C2 entry (1 entry)**

```rust
TalentBuffDef {
    name: "barbara_c2_hydro_dmg",
    description: "C2: Active character gains Hydro DMG Bonus +15% during Skill",
    stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 4: Add Candace C2 entry (1 entry)**

```rust
TalentBuffDef {
    name: "candace_c2_hp",
    description: "C2: Max HP +20% for 15s when Skill hits",
    stat: BuffableStat::HpPercent,
    base_value: 0.20,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 5: Add Columbina C1, C2, C4, C6 entries (6 entries)**

```rust
TalentBuffDef {
    name: "columbina_c1_lunar_dmg",
    description: "C1: Party Lunar Reaction DMG +1.5%",
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.015,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(1),
    min_constellation: 1,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "columbina_c2_hp",
    description: "C2: Max HP +40% on Gravity Interference",
    stat: BuffableStat::HpPercent,
    base_value: 0.40,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "columbina_c2_lunar_dmg",
    description: "C2: Party Lunar Reaction DMG +7%",
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.07,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "columbina_c4_lunar_dmg",
    description: "C4: Party Lunar Reaction DMG +1.5%",
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.015,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "columbina_c6_crit_rate",
    description: "C6: Party CRIT Rate +80% for elemental DMG after Lunar reaction",
    stat: BuffableStat::CritRate,
    base_value: 0.80,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "columbina_c6_lunar_dmg",
    description: "C6: Party Lunar Reaction DMG +7%",
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.07,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 6: Add Mona C1, C2, C4, C6 entries (5 entries)**

```rust
TalentBuffDef {
    name: "mona_c1_reaction_dmg",
    description: "C1: Reaction DMG +15% vs Omen-affected opponents",
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(1),
    min_constellation: 1,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "mona_c2_em",
    description: "C2: Party EM +80 on Charged ATK hit",
    stat: BuffableStat::ElementalMastery,
    base_value: 80.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "mona_c4_crit_rate",
    description: "C4: CRIT Rate +15% vs Omen-affected opponents",
    stat: BuffableStat::CritRate,
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "mona_c4_crit_dmg",
    description: "C4: CRIT DMG +15% vs Omen-affected opponents (approximation)",
    stat: BuffableStat::CritDmg,
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "mona_c6_charged_dmg",
    description: "C6: Charged ATK DMG +180% (max in Illusory Torrent)",
    stat: BuffableStat::ChargedAtkDmgBonus,
    base_value: 1.80,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 7: Add Mualani C4 entry (1 entry)**

```rust
TalentBuffDef {
    name: "mualani_c4_burst_dmg",
    description: "C4: Boomsharka-laka DMG +75%",
    stat: BuffableStat::BurstDmgBonus,
    base_value: 0.75,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 8: Add Nilou C2, C4, C6 entries (5 entries)**

```rust
TalentBuffDef {
    name: "nilou_c2_hydro_res_shred",
    description: "C2: Opponents' Hydro RES -35% after Hydro DMG",
    stat: BuffableStat::ElementalResReduction(Element::Hydro),
    base_value: 0.35,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "nilou_c2_dendro_res_shred",
    description: "C2: Opponents' Dendro RES -35% after Bloom DMG",
    stat: BuffableStat::ElementalResReduction(Element::Dendro),
    base_value: 0.35,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "nilou_c4_burst_dmg",
    description: "C4: Dance of Abzendegi DMG +50% after 3rd dance step",
    stat: BuffableStat::BurstDmgBonus,
    base_value: 0.50,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "nilou_c6_crit_rate",
    description: "C6: CRIT Rate +0.6% per 1000 Max HP (max +30%)",
    stat: BuffableStat::CritRate,
    base_value: 0.006,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: Some(ScalingStat::Hp),
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: Some(0.30),
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "nilou_c6_crit_dmg",
    description: "C6: CRIT DMG +1.2% per 1000 Max HP (max +60%)",
    stat: BuffableStat::CritDmg,
    base_value: 0.012,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: Some(ScalingStat::Hp),
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: Some(0.60),
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 9: Add Xingqiu C2, C4 entries (2 entries)**

```rust
TalentBuffDef {
    name: "xingqiu_c2_hydro_res_shred",
    description: "C2: Opponents hit by sword rain have Hydro RES -15%",
    stat: BuffableStat::ElementalResReduction(Element::Hydro),
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "xingqiu_c4_skill_dmg",
    description: "C4: Skill DMG +50% during Burst",
    stat: BuffableStat::SkillDmgBonus,
    base_value: 0.50,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 10: Add Yelan C4 entry (1 entry)**

```rust
TalentBuffDef {
    name: "yelan_c4_hp",
    description: "C4: Party Max HP +10% per marked opponent (max 4)",
    stat: BuffableStat::HpPercent,
    base_value: 0.10,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Stacks(4))),
},
```

- [ ] **Step 11: Run cargo build**

Run: `cargo build -p genshin-calc-data 2>&1 | tail -5`

- [ ] **Step 12: Write tests**

Add tests following the pattern from Task 1. Test each character at C0 (buff absent) and max constellation (buff present). Verify stat type and value for at least one buff per character.

- [ ] **Step 13: Run tests and verify**

Run: `cargo test -p genshin-calc-data 2>&1 | tail -5`

- [ ] **Step 14: Commit**

```bash
git add crates/data/src/talent_buffs/hydro.rs crates/data/src/talent_buffs/mod.rs
git commit -m "feat: add constellation conditional buffs for Hydro characters (#51)

Add 17 new TalentBuffDef entries for 8 Hydro characters:
- Aino C1/C6, Barbara C2, Candace C2
- Columbina C1/C2/C4/C6, Mona C1/C2/C4/C6
- Mualani C4, Nilou C2/C4/C6
- Xingqiu C2/C4, Yelan C4"
```

---

## Task 3: Electro Constellation Buffs

**Files:**
- Modify: `crates/data/src/talent_buffs/electro.rs`
- Modify: `crates/data/src/talent_buffs/mod.rs` (tests)

**Characters:** beidou, flins, iansan, ineffa, yae_miko (5 chars, 8 entries)
**Merge order:** 3rd (rebase on Hydro)

- [ ] **Step 1: Read existing electro.rs to find insertion points**

- [ ] **Step 2: Add Beidou C4 entry (1 entry)**

```rust
TalentBuffDef {
    name: "beidou_c4_electro_dmg",
    description: "C4: Normal ATK Electro DMG Bonus +20% on hit",
    stat: BuffableStat::ElementalDmgBonus(Element::Electro),
    base_value: 0.20,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 3: Add Flins C2, C6 entries (3 entries)**

```rust
TalentBuffDef {
    name: "flins_c2_electro_res_shred",
    description: "C2: Opponents' Electro RES -25% during Ascendant Gleam Moonsign",
    stat: BuffableStat::ElementalResReduction(Element::Electro),
    base_value: 0.25,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "flins_c6_lunar_charged_self",
    description: "C6: Flins's Lunar-Charged DMG +35%",
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.35,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "flins_c6_lunar_charged_team",
    description: "C6: Party Lunar-Charged DMG +10% during Moonsign",
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.10,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 4: Add Iansan C2, C6 entries (2 entries)**

```rust
TalentBuffDef {
    name: "iansan_c2_atk",
    description: "C2: Off-field grants active character ATK +30%",
    stat: BuffableStat::AtkPercent,
    base_value: 0.30,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
TalentBuffDef {
    name: "iansan_c6_dmg_bonus",
    description: "C6: Active character DMG Bonus +25% on Nightsoul overflow",
    stat: BuffableStat::DmgBonus,
    base_value: 0.25,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 5: Add Ineffa C1 entry (1 entry)**

```rust
TalentBuffDef {
    name: "ineffa_c1_lunar_charged_dmg",
    description: "C1: Party Lunar-Charged DMG +2.5% per 100 ATK (max +50%)",
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.025,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: Some(ScalingStat::TotalAtk),
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(1),
    min_constellation: 1,
    cap: Some(0.50),
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 6: Add Yae Miko C6 entry (1 entry)**

```rust
TalentBuffDef {
    name: "yae_miko_c6_def_ignore",
    description: "C6: Sesshou Sakura attacks ignore 60% of opponents' DEF",
    stat: BuffableStat::DefIgnore,
    base_value: 0.60,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
},
```

- [ ] **Step 7: Build, write tests, run tests, commit**

Follow the same pattern as Task 1 Steps 8-12.

```bash
git commit -m "feat: add constellation conditional buffs for Electro characters (#51)

Add 8 new TalentBuffDef entries for 5 Electro characters:
- Beidou C4, Flins C2/C6, Iansan C2/C6
- Ineffa C1, Yae Miko C6"
```

---

## Task 4: Cryo Constellation Buffs

**Files:**
- Modify: `crates/data/src/talent_buffs/cryo.rs`
- Modify: `crates/data/src/talent_buffs/mod.rs` (tests)

**Characters:** escoffier, eula, ganyu, mika, qiqi, rosaria, shenhe (7 chars, 10 entries)
**Merge order:** 4th (rebase on Electro)

- [ ] **Step 1: Read existing cryo.rs to find insertion points**

- [ ] **Step 2: Add all entries**

Follow the spec table. Each entry uses the standard `TalentBuffDef` pattern. Key entries:

- Escoffier C1: `CritDmg 0.60, Team, Toggle` (Cryo CRIT DMG)
- Escoffier C2: `SkillFlatDmg, scales_on: TotalAtk, base_value: 2.40, Team, Toggle`
- Eula C1: `PhysicalDmgBonus 0.30, OnlySelf, Toggle`
- Eula C4: `BurstDmgBonus 0.25, OnlySelf, Toggle`
- Ganyu C1: `ElementalResReduction(Cryo) 0.15, Team, Toggle`
- Mika C6: `CritDmg 0.60, Team, Toggle` (Physical CRIT DMG)
- Qiqi C2: `NormalAtkDmgBonus 0.15` + `ChargedAtkDmgBonus 0.15, OnlySelf, Toggle`
- Rosaria C1: `NormalAtkDmgBonus 0.10, OnlySelf, Toggle`
- Shenhe C2: `CritDmg 0.15, Team, Toggle`

- [ ] **Step 3: Build, write tests, run tests, commit**

```bash
git commit -m "feat: add constellation conditional buffs for Cryo characters (#51)

Add 8 new TalentBuffDef entries for 6 Cryo characters:
- Escoffier C1/C2, Eula C1/C4, Ganyu C1
- Mika C6, Qiqi C2, Rosaria C1, Shenhe C2"
```

---

## Task 5: Dendro Constellation Buffs

**Files:**
- Modify: `crates/data/src/talent_buffs/dendro.rs`
- Modify: `crates/data/src/talent_buffs/mod.rs` (tests)

**Characters:** baizhu, collei, kirara, nahida, nefer, traveler_dendro, yaoyao (7 chars, 14 entries)
**Merge order:** 5th (rebase on Cryo)

- [ ] **Step 1: Read existing dendro.rs to find insertion points**

- [ ] **Step 2: Add all entries**

Key entries:
- Baizhu C4: `ElementalMastery 80.0, Team, Toggle`
- Baizhu C6: `SkillFlatDmg, scales_on: Hp, base_value: 0.08, OnlySelf, Toggle`
- Collei C4: `ElementalMastery 60.0, TeamExcludeSelf, Toggle`
- Kirara C6: `DmgBonus 0.12, Team, Toggle`
- Nahida C2: 3 entries (CritRate 0.20, CritDmg 1.00, DefReduction 0.30)
- Nahida C4: `ElementalMastery 100.0, OnlySelf, Stacks(4)`
- Nefer C2: `ElementalMastery 200.0, OnlySelf, Toggle`
- Nefer C4: `ElementalResReduction(Dendro) 0.20, Team, Toggle`
- Nefer C6: `TransformativeBonus 0.15, Team, Toggle`
- Traveler Dendro C6: `ElementalDmgBonus(Dendro) 0.12, Team, Toggle`
- Yaoyao C1: `ElementalDmgBonus(Dendro) 0.15, Team, Toggle`
- Yaoyao C4: `ElementalMastery, scales_on: Hp, base_value: 0.003, cap: Some(120.0), OnlySelf, Toggle`

- [ ] **Step 3: Build, write tests, run tests, commit**

```bash
git commit -m "feat: add constellation conditional buffs for Dendro characters (#51)

Add 13 new TalentBuffDef entries for 8 Dendro characters:
- Baizhu C4/C6, Collei C4, Kirara C6
- Nahida C2/C4, Nefer C2/C4/C6
- Traveler Dendro C6, Yaoyao C1/C4"
```

---

## Task 6: Anemo Constellation Buffs

**Files:**
- Modify: `crates/data/src/talent_buffs/anemo.rs`
- Modify: `crates/data/src/talent_buffs/mod.rs` (tests)

**Characters:** faruzan, jahoda, kazuha, venti, xianyun (5 chars, 11 entries)
**Merge order:** 6th (rebase on Dendro)

- [ ] **Step 1: Read existing anemo.rs to find insertion points**

- [ ] **Step 2: Add all entries**

Key entries:
- Faruzan C6: `CritDmg 0.40, Team, Toggle`
- Jahoda C6: `CritRate 0.05` + `CritDmg 0.40, Team, Toggle`
- Kazuha C2: `ElementalMastery 200.0, Team, Toggle`
- Kazuha C6: 3 entries (`NormalAtkDmgBonus`/`ChargedAtkDmgBonus`/`PlungingAtkDmgBonus`, each `scales_on: Em, base_value: 0.002, OnlySelf, Toggle`)
- Venti C4: `ElementalDmgBonus(Anemo) 0.25, OnlySelf, Toggle`
- Venti C6: `ElementalResReduction(Anemo) 0.20, Team, Toggle`
- Xianyun C2: `AtkPercent 0.20, OnlySelf, Toggle`
- Xianyun C6: `CritDmg 0.70, OnlySelf, Toggle`

- [ ] **Step 3: Build, write tests, run tests, commit**

```bash
git commit -m "feat: add constellation conditional buffs for Anemo characters (#51)

Add 10 new TalentBuffDef entries for 4 Anemo characters:
- Faruzan C6, Jahoda C6, Kazuha C2/C6
- Venti C4/C6, Xianyun C2/C6"
```

---

## Task 7: Geo Constellation Buffs

**Files:**
- Modify: `crates/data/src/talent_buffs/geo.rs`
- Modify: `crates/data/src/talent_buffs/mod.rs` (tests)

**Characters:** albedo, gorou, illuga, yun_jin (4 chars, 7 entries)
**Merge order:** 7th (rebase on Anemo)

- [ ] **Step 1: Read existing geo.rs to find insertion points**

- [ ] **Step 2: Add all entries**

Key entries:
- Albedo C1: `DefPercent 0.50, OnlySelf, Toggle`
- Albedo C4: `PlungingAtkDmgBonus 0.30, Team, Toggle`
- Albedo C6: `DmgBonus 0.17, Team, Toggle`
- Gorou C6: `CritDmg 0.40, Team, Toggle`
- Illuga C4: `DefFlat 200.0, Team, Toggle`
- Yun Jin C2: `NormalAtkDmgBonus 0.15, Team, Toggle`
- Yun Jin C4: `DefPercent 0.20, OnlySelf, Toggle`

- [ ] **Step 3: Build, write tests, run tests, commit**

```bash
git commit -m "feat: add constellation conditional buffs for Geo characters (#51)

Add 6 new TalentBuffDef entries for 4 Geo characters:
- Albedo C1/C4/C6, Gorou C6
- Illuga C4, Yun Jin C2/C4"
```

---

## Post-Implementation

### Sequential Merge (mod.rs conflict resolution)

Each task runs in its own worktree and commits to its own branch. Since all tasks add tests to the shared `mod.rs`, merge must be sequential:

1. Merge Pyro branch to main
2. Rebase Hydro branch on main, resolve mod.rs conflicts (append tests), merge
3. Repeat for Electro, Cryo, Dendro, Anemo, Geo

The `<element>.rs` files never conflict — only the test additions in `mod.rs` need resolution.

### Final Verification

- [ ] **Run full test suite:** `cargo test 2>&1 | tail -10`
- [ ] **Run clippy:** `cargo clippy -- -D warnings 2>&1 | tail -10`
- [ ] **Run fmt check:** `cargo fmt --check`
- [ ] **Delete spec and plan files** (per CLAUDE.md: implementation complete, code and tests are authoritative)
