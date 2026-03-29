# P1: Talent Buff Expansion Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Expand TalentBuffDef coverage from 9 to 27 characters (25 new buff entries) using existing data structures, with no core crate changes.

**Architecture:** All new entries follow existing TalentBuffDef patterns (fixed value, talent scaling, builder pre-computation). Two new CharacterData entries (Jahoda, Aino) added to data crate. Sara ID bug fixed as pre-requisite.

**Tech Stack:** Rust, genshin-calc-core, genshin-calc-data

**Spec:** `docs/superpowers/specs/2026-03-28-talent-buff-expansion-design.md`

---

## File Structure

| File | Responsibility | Change |
|------|---------------|--------|
| `crates/data/src/talent_buffs.rs` | Talent buff definitions | Fix Sara ID + 25 new entries + scaling arrays |
| `crates/data/src/characters/anemo.rs` | Anemo character data | +Jahoda CharacterData |
| `crates/data/src/characters/hydro.rs` | Hydro character data | +Aino CharacterData |
| `crates/data/src/characters/mod.rs` | Character registry | +Jahoda, Aino in ALL_CHARACTERS |
| `crates/data/src/team_builder.rs` | Builder + tests | +integration tests only |
| `docs/data-expansion-todo.md` | Progress tracking | P1 checklist update |

---

### Task 1: Fix Sara ID Bug

**Files:**
- Modify: `crates/data/src/talent_buffs.rs:215`

- [ ] **Step 1: Write test exposing the bug**

Add to `crates/data/src/talent_buffs.rs` tests module:

```rust
#[test]
fn test_find_sara_buffs_by_character_id() {
    // Sara's CharacterData uses id "kujou_sara", talent_buffs must match
    let buffs = find_talent_buffs("kujou_sara").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-data test_find_sara_buffs_by_character_id`
Expected: FAIL — `find_talent_buffs("kujou_sara")` returns `None`

- [ ] **Step 3: Fix the ID**

In `crates/data/src/talent_buffs.rs` line 215, change:

```rust
// Before
("sara", SARA_BUFFS),
// After
("kujou_sara", SARA_BUFFS),
```

- [ ] **Step 4: Update existing test that uses old ID**

The test `test_find_nonexistent_character` at line 251 asserts `find_talent_buffs("zhongli").is_none()`. No change needed there. But verify no test uses `find_talent_buffs("sara")` — if so, update to `"kujou_sara"`.

- [ ] **Step 5: Run tests to verify fix**

Run: `cargo test -p genshin-calc-data`
Expected: All pass including new test

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/talent_buffs.rs
git commit -m "fix: correct Sara talent buff ID to match CharacterData"
```

---

### Task 2: Fixed-Value Ascension Passive Buffs (7 entries / 7 characters)

**Files:**
- Modify: `crates/data/src/talent_buffs.rs`

Characters: Sucrose A1, Ganyu A4, Albedo A4, Ningguang A4, Dendro Traveler A4, Yoimiya A4, Chevreuse A1
(Gorou is fully handled in Task 5)

- [ ] **Step 0: Add Element import**

In `crates/data/src/talent_buffs.rs` line 1, update the import:

```rust
// Before
use genshin_calc_core::{BuffTarget, BuffableStat, ScalingStat};
// After
use genshin_calc_core::{BuffTarget, BuffableStat, Element, ScalingStat};
```

- [ ] **Step 1: Write tests for all 7 entries**

Add to `crates/data/src/talent_buffs.rs` tests module:

```rust
#[test]
fn test_find_sucrose_buffs() {
    let buffs = find_talent_buffs("sucrose").unwrap();
    // A1 (EM+50) + A4 (EM builder) = at least 1 fixed entry
    assert!(buffs.iter().any(|b| b.stat == BuffableStat::ElementalMastery && (b.base_value - 50.0).abs() < 1e-6));
}

#[test]
fn test_find_ganyu_buffs() {
    let buffs = find_talent_buffs("ganyu").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::ElementalDmgBonus(Element::Cryo));
    assert!((buffs[0].base_value - 0.20).abs() < 1e-6);
    assert_eq!(buffs[0].target, BuffTarget::Team);
}

#[test]
fn test_find_albedo_buffs() {
    let buffs = find_talent_buffs("albedo").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
    assert!((buffs[0].base_value - 125.0).abs() < 1e-6);
}

#[test]
fn test_find_ningguang_buffs() {
    let buffs = find_talent_buffs("ningguang").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::ElementalDmgBonus(Element::Geo));
    assert!((buffs[0].base_value - 0.12).abs() < 1e-6);
}

#[test]
fn test_find_traveler_dendro_buffs() {
    let buffs = find_talent_buffs("traveler_dendro").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
    assert!((buffs[0].base_value - 60.0).abs() < 1e-6);
}

#[test]
fn test_find_yoimiya_buffs() {
    let buffs = find_talent_buffs("yoimiya").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::AtkPercent);
    assert!((buffs[0].base_value - 0.20).abs() < 1e-6);
    assert_eq!(buffs[0].target, BuffTarget::TeamExcludeSelf);
}

#[test]
fn test_find_chevreuse_buffs() {
    let buffs = find_talent_buffs("chevreuse").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::AtkPercent);
    assert!((buffs[0].base_value - 0.20).abs() < 1e-6);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data test_find_sucrose test_find_ganyu test_find_albedo test_find_ningguang test_find_traveler_dendro test_find_yoimiya test_find_chevreuse`
Expected: All FAIL — `find_talent_buffs` returns `None`

- [ ] **Step 3: Implement all 8 buff definitions**

Add to `crates/data/src/talent_buffs.rs` before `ALL_TALENT_BUFFS`:

```rust
// ===== Sucrose =====
// A1 passive "Catalyst Conversion": Swirl triggers EM+50 for team 8s
static SUCROSE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Catalyst Conversion",
        description: "After triggering Swirl, grants EM+50 to party members with matching element",
        stat: BuffableStat::ElementalMastery,
        base_value: 50.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    // A4 added in Task 6 (builder pattern)
];

// ===== Ganyu =====
// A4 passive "Harmony between Heaven and Earth": Cryo DMG+20% in burst field
static GANYU_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Harmony between Heaven and Earth",
    description: "Cryo DMG Bonus +20% for party members in burst field",
    stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
    base_value: 0.20,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Albedo =====
// A4 passive "Homuncular Nature": EM+125 for team after burst
static ALBEDO_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Homuncular Nature",
    description: "After burst, grants EM+125 to nearby party members for 10s",
    stat: BuffableStat::ElementalMastery,
    base_value: 125.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Ningguang =====
// A4 passive "Strategic Reserve": Geo DMG+12% when passing through Jade Screen
static NINGGUANG_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Strategic Reserve",
    description: "Characters passing through Jade Screen gain Geo DMG Bonus +12%",
    stat: BuffableStat::ElementalDmgBonus(Element::Geo),
    base_value: 0.12,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Dendro Traveler =====
// A4 passive "Verdant Luxury": EM+60 in Lea Lotus Lamp field
static TRAVELER_DENDRO_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Verdant Luxury",
    description: "Characters within Lea Lotus Lamp field gain EM+60",
    stat: BuffableStat::ElementalMastery,
    base_value: 60.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Yoimiya =====
// A4 passive "Summer Night's Dawn": ATK+10-20% to party after burst (max 2 stacks)
static YOIMIYA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Summer Night's Dawn",
    description: "After burst, party members (excluding Yoimiya) gain ATK+20% (max assumption)",
    stat: BuffableStat::AtkPercent,
    base_value: 0.20,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::TeamExcludeSelf,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Chevreuse =====
// A1 passive "Vanguard's Coordinated Tactics": ATK+20% after Overloaded (Pyro+Electro team)
static CHEVREUSE_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
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
}];
```

Also add Gorou's Geo DMG entry (the DefFlat scaling entry is in Task 5):

```rust
// ===== Gorou ===== (Geo DMG part; DefFlat scaling in separate entry)
// Skill "Inuzaka All-Round Defense" 3-Geo bonus: Geo DMG+15%
```

Note: Gorou will have both a scaling DefFlat entry (Task 5) and a fixed Geo DMG entry. Define the full GOROU_BUFFS array in Task 5 when both are ready.

Register in `ALL_TALENT_BUFFS` (Gorou deferred to Task 5):

```rust
("sucrose", SUCROSE_BUFFS),
("ganyu", GANYU_BUFFS),
("albedo", ALBEDO_BUFFS),
("ningguang", NINGGUANG_BUFFS),
("traveler_dendro", TRAVELER_DENDRO_BUFFS),
("yoimiya", YOIMIYA_BUFFS),
("chevreuse", CHEVREUSE_BUFFS),
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data`
Expected: All pass

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/talent_buffs.rs
git commit -m "feat: add fixed-value talent buffs for 7 characters"
```

---

### Task 3: Constellation-Gated Buffs (4 entries / 4 characters)

**Files:**
- Modify: `crates/data/src/talent_buffs.rs`

Characters: Diona C6, Amber C6, Barbara C2, Sara C6

- [ ] **Step 1: Write tests**

```rust
#[test]
fn test_find_diona_buffs() {
    let buffs = find_talent_buffs("diona").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
    assert!((buffs[0].base_value - 200.0).abs() < 1e-6);
    assert_eq!(buffs[0].min_constellation, 6);
}

#[test]
fn test_find_amber_buffs() {
    let buffs = find_talent_buffs("amber").unwrap();
    assert_eq!(buffs[0].stat, BuffableStat::AtkPercent);
    assert_eq!(buffs[0].min_constellation, 6);
}

#[test]
fn test_find_barbara_buffs() {
    let buffs = find_talent_buffs("barbara").unwrap();
    assert_eq!(buffs[0].stat, BuffableStat::ElementalDmgBonus(Element::Hydro));
    assert_eq!(buffs[0].min_constellation, 2);
}

#[test]
fn test_find_sara_c6_buff() {
    let buffs = find_talent_buffs("kujou_sara").unwrap();
    // Original skill buff + C6 CritDmg
    let c6 = buffs.iter().find(|b| b.stat == BuffableStat::CritDmg).unwrap();
    assert!((c6.base_value - 0.60).abs() < 1e-6);
    assert_eq!(c6.min_constellation, 6);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data test_find_diona test_find_amber test_find_barbara test_find_sara_c6`
Expected: FAIL

- [ ] **Step 3: Implement buff definitions**

```rust
// ===== Diona =====
// C6 "Cat's Tail Closing Time": EM+200 in burst field
static DIONA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Cat's Tail Closing Time",
    description: "Characters within burst field gain EM+200",
    stat: BuffableStat::ElementalMastery,
    base_value: 200.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
}];

// ===== Amber =====
// C6 "Wildfire": ATK+15% for party during burst
static AMBER_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Wildfire",
    description: "During burst, party members gain ATK+15%",
    stat: BuffableStat::AtkPercent,
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
}];

// ===== Barbara =====
// C2 "Vitality Burst": Hydro DMG+15% during skill
static BARBARA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Vitality Burst",
    description: "During skill, active character gains Hydro DMG Bonus +15%",
    stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
}];
```

Add Sara C6 to existing `SARA_BUFFS` array:

```rust
static SARA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Tengu Juurai ATK Bonus",
        // ... existing entry unchanged ...
    },
    TalentBuffDef {
        name: "Sin of Pride",
        description: "Electro CRIT DMG +60% (approximated as generic CritDmg; Electro-only in game)",
        stat: BuffableStat::CritDmg,
        base_value: 0.60,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
];
```

Register new characters in `ALL_TALENT_BUFFS`:

```rust
("diona", DIONA_BUFFS),
("amber", AMBER_BUFFS),
("barbara", BARBARA_BUFFS),
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All pass

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/talent_buffs.rs
git commit -m "feat: add constellation-gated talent buffs (Diona C6, Amber C6, Barbara C2, Sara C6)"
```

---

### Task 4: Multi-Entry Buffs (5 entries / 2 characters)

**Files:**
- Modify: `crates/data/src/talent_buffs.rs`

Characters: Shenhe A1 press (2 entries), Thoma C6 (3 entries)

- [ ] **Step 1: Write tests**

```rust
#[test]
fn test_shenhe_a1_press_buffs() {
    let buffs = find_talent_buffs("shenhe").unwrap();
    // Existing: Spring Spirit Summoning (skill scaling)
    // New: Deific Embrace press = SkillDmgBonus + BurstDmgBonus
    let skill_dmg = buffs.iter().find(|b| b.stat == BuffableStat::SkillDmgBonus);
    let burst_dmg = buffs.iter().find(|b| b.stat == BuffableStat::BurstDmgBonus);
    assert!(skill_dmg.is_some(), "Should have SkillDmgBonus from A1 press");
    assert!(burst_dmg.is_some(), "Should have BurstDmgBonus from A1 press");
    assert!((skill_dmg.unwrap().base_value - 0.15).abs() < 1e-6);
    assert!((burst_dmg.unwrap().base_value - 0.15).abs() < 1e-6);
}

#[test]
fn test_thoma_c6_buffs() {
    let buffs = find_talent_buffs("thoma").unwrap();
    assert_eq!(buffs.len(), 3);
    assert!(buffs.iter().any(|b| b.stat == BuffableStat::NormalAtkDmgBonus));
    assert!(buffs.iter().any(|b| b.stat == BuffableStat::ChargedAtkDmgBonus));
    assert!(buffs.iter().any(|b| b.stat == BuffableStat::PlungingAtkDmgBonus));
    for b in buffs {
        assert!((b.base_value - 0.15).abs() < 1e-6);
        assert_eq!(b.min_constellation, 6);
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data test_shenhe_a1 test_thoma_c6`
Expected: FAIL

- [ ] **Step 3: Implement**

Expand existing `SHENHE_BUFFS` to include A1 entries:

```rust
static SHENHE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Spring Spirit Summoning Quill DMG",
        // ... existing entry unchanged ...
    },
    TalentBuffDef {
        name: "Deific Embrace Press - Skill DMG",
        description: "After press E, party Skill DMG +15% for 10s",
        stat: BuffableStat::SkillDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Deific Embrace Press - Burst DMG",
        description: "After press E, party Burst DMG +15% for 10s",
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
];
```

Add Thoma:

```rust
// ===== Thoma =====
// C6 "Burning Heart": Normal/Charged/Plunging +15% after burst
static THOMA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Burning Heart - Normal ATK",
        description: "After burst, party Normal ATK DMG +15%",
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
    TalentBuffDef {
        name: "Burning Heart - Charged ATK",
        description: "After burst, party Charged ATK DMG +15%",
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
    TalentBuffDef {
        name: "Burning Heart - Plunging ATK",
        description: "After burst, party Plunging ATK DMG +15%",
        stat: BuffableStat::PlungingAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
];
```

Register: `("thoma", THOMA_BUFFS),`

- [ ] **Step 4: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All pass

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/talent_buffs.rs
git commit -m "feat: add Shenhe A1 press and Thoma C6 multi-entry talent buffs"
```

---

### Task 5: Talent-Scaling Buffs (4 entries / 3 characters)

**Files:**
- Modify: `crates/data/src/talent_buffs.rs`

Characters: Faruzan (Anemo DMG, burst scaling), Candace (NormalAtkDmgBonus, burst scaling), Gorou (DefFlat skill scaling + Geo DMG fixed)

**Important:** The `[f64; 15]` scaling arrays require values from the Genshin Impact Wiki. The implementor must look up each character's talent page and extract Lv1-Lv15 values. Placeholder structure is provided below.

- [ ] **Step 1: Write tests**

```rust
#[test]
fn test_find_faruzan_buffs() {
    let buffs = find_talent_buffs("faruzan").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::ElementalDmgBonus(Element::Anemo));
    assert!(buffs[0].scales_with_talent);
    assert!(buffs[0].talent_scaling.is_some());
    assert_eq!(buffs[0].source, TalentBuffSource::ElementalBurst);
}

#[test]
fn test_faruzan_burst_scaling_lv13() {
    let buffs = find_talent_buffs("faruzan").unwrap();
    let scaling = buffs[0].talent_scaling.unwrap();
    // Verify Lv13 value against Wiki (implementor fills exact value)
    assert!(scaling[12] > 0.0, "Lv13 scaling should be positive");
}

#[test]
fn test_find_candace_buffs() {
    let buffs = find_talent_buffs("candace").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::NormalAtkDmgBonus);
    assert!(buffs[0].scales_with_talent);
    assert_eq!(buffs[0].source, TalentBuffSource::ElementalBurst);
}

#[test]
fn test_find_gorou_buffs() {
    let buffs = find_talent_buffs("gorou").unwrap();
    assert_eq!(buffs.len(), 2);
    // DefFlat scaling entry
    let def_buff = buffs.iter().find(|b| b.stat == BuffableStat::DefFlat).unwrap();
    assert!(def_buff.scales_with_talent);
    assert_eq!(def_buff.source, TalentBuffSource::ElementalSkill);
    // Geo DMG fixed entry
    let geo_buff = buffs.iter().find(|b| b.stat == BuffableStat::ElementalDmgBonus(Element::Geo)).unwrap();
    assert!((geo_buff.base_value - 0.15).abs() < 1e-6);
    assert!(!geo_buff.scales_with_talent);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data test_find_faruzan test_faruzan_burst test_find_candace test_find_gorou`
Expected: FAIL

- [ ] **Step 3: Look up Wiki scaling values**

For each character, look up the talent scaling page on the Genshin Impact Wiki:
- Faruzan Burst "The Wind's Secret Ways" → Prayerful Wind's Benefit Anemo DMG% per level
- Candace Burst "Sacred Rite: Heron's Sanctum" → Normal ATK DMG bonus per level
- Gorou Skill "Inuzaka All-Round Defense" → DEF increase per level

Record the Lv1-Lv15 values as decimal fractions (e.g., 18% → 0.18).

- [ ] **Step 4: Implement scaling arrays and buff definitions**

```rust
// ===== Faruzan =====
// Burst "The Wind's Secret Ways": Anemo DMG bonus (Lv1-15)
// Values from Wiki — implementor must verify
static FARUZAN_BURST_ANEMO_SCALING: [f64; 15] = [
    // Lv1, Lv2, ..., Lv15 — fill from Wiki
    0.182, 0.196, 0.209, 0.228, 0.241, 0.255, 0.273, 0.291, 0.310, 0.328,
    0.346, 0.364, 0.387, 0.410, 0.432,
];

static FARUZAN_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
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
}];

// ===== Candace =====
// Burst "Sacred Rite: Heron's Sanctum": Normal ATK DMG bonus (Lv1-15)
static CANDACE_BURST_NORMAL_SCALING: [f64; 15] = [
    // Fill from Wiki
    0.20, 0.215, 0.23, 0.25, 0.265, 0.28, 0.30, 0.32, 0.34, 0.36,
    0.38, 0.40, 0.425, 0.45, 0.475,
];

static CANDACE_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Sacred Rite: Heron's Sanctum",
    description: "Normal ATK DMG Bonus based on burst talent level",
    stat: BuffableStat::NormalAtkDmgBonus,
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&CANDACE_BURST_NORMAL_SCALING),
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalBurst,
    min_constellation: 0,
}];

// ===== Gorou =====
// Skill "Inuzaka All-Round Defense": DEF flat (Lv1-15) + Geo DMG+15% (3 Geo)
static GOROU_SKILL_DEF_SCALING: [f64; 15] = [
    // Fill from Wiki — these are flat DEF values, not percentages
    206.16, 221.62, 237.08, 257.70, 273.16, 288.62, 309.24, 329.86, 350.48,
    371.10, 391.72, 412.34, 438.11, 463.88, 489.65,
];

static GOROU_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Inuzaka All-Round Defense",
        description: "DEF increase based on skill talent level",
        stat: BuffableStat::DefFlat,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&GOROU_SKILL_DEF_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Inuzaka All-Round Defense - Geo DMG",
        description: "With 3 Geo members, Geo DMG Bonus +15% (approximation: always registered)",
        stat: BuffableStat::ElementalDmgBonus(Element::Geo),
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
];
```

Register in `ALL_TALENT_BUFFS`:

```rust
("faruzan", FARUZAN_BUFFS),
("candace", CANDACE_BUFFS),
("gorou", GOROU_BUFFS),
```

- [ ] **Step 5: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All pass

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/talent_buffs.rs
git commit -m "feat: add talent-scaling buffs for Faruzan, Candace, Gorou"
```

---

### Task 6: Builder-Pattern Buffs (4 entries / 3 characters)

**Files:**
- Modify: `crates/data/src/talent_buffs.rs`

Characters: Sucrose A4, Yelan A4, Ineffa A4

Note: Sucrose already has A1 from Task 2. Expand `SUCROSE_BUFFS` to include A4.

- [ ] **Step 1: Write tests**

```rust
#[test]
fn test_sucrose_a4_builder_pattern() {
    let buffs = find_talent_buffs("sucrose").unwrap();
    assert_eq!(buffs.len(), 2); // A1 + A4
    let a4 = buffs.iter().find(|b| b.name == "Mollis Favonius").unwrap();
    assert_eq!(a4.stat, BuffableStat::ElementalMastery);
    assert!((a4.base_value - 0.0).abs() < 1e-6); // builder sets value
    assert_eq!(a4.target, BuffTarget::TeamExcludeSelf);
}

#[test]
fn test_find_yelan_buffs() {
    let buffs = find_talent_buffs("yelan").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::DmgBonus);
    assert!((buffs[0].base_value - 0.0).abs() < 1e-6); // builder sets value
}

#[test]
fn test_find_ineffa_talent_buffs() {
    let buffs = find_talent_buffs("ineffa").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
    assert!((buffs[0].base_value - 0.0).abs() < 1e-6); // builder sets value
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data test_sucrose_a4 test_find_yelan test_find_ineffa_talent`
Expected: FAIL

- [ ] **Step 3: Implement**

Update `SUCROSE_BUFFS` (add A4 entry to existing array from Task 2):

```rust
static SUCROSE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Catalyst Conversion",
        description: "After triggering Swirl, grants EM+50 to party members with matching element",
        stat: BuffableStat::ElementalMastery,
        base_value: 50.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Mollis Favonius",
        description: "Shares 20% of Sucrose's EM to party (builder computes EM * 0.20)",
        stat: BuffableStat::ElementalMastery,
        base_value: 0.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::TeamExcludeSelf,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
];
```

Add Yelan and Ineffa:

```rust
// ===== Yelan =====
// A4 passive "Adapt With Ease": DMG bonus ramp 1-50% (builder sets desired value)
static YELAN_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Adapt With Ease",
    description: "DMG Bonus ramps 1-50% over time (builder sets value, max 0.50)",
    stat: BuffableStat::DmgBonus,
    base_value: 0.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Ineffa =====
// A4 passive: EM share based on ATK (builder computes ATK * 0.06)
static INEFFA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Ineffa A4 EM Share",
    description: "Grants EM equal to 6% of Ineffa's ATK (builder computes ATK * 0.06)",
    stat: BuffableStat::ElementalMastery,
    base_value: 0.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];
```

Register:

```rust
("yelan", YELAN_BUFFS),
("ineffa", INEFFA_BUFFS),
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All pass

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/talent_buffs.rs
git commit -m "feat: add builder-pattern talent buffs for Sucrose A4, Yelan, Ineffa"
```

---

### Task 7: Jahoda CharacterData + Talent Buff

**Files:**
- Modify: `crates/data/src/characters/anemo.rs`
- Modify: `crates/data/src/characters/mod.rs`
- Modify: `crates/data/src/talent_buffs.rs`

**Important:** CharacterData requires full talent scaling data (normal attack hits, skill, burst) sourced from Wiki. This is a substantial task (~150-300 lines per character). See `crates/data/src/characters/cryo.rs` Diona (line 979) for a reference 4-star implementation. The implementor must look up all talent scaling values on the Genshin Impact Wiki.

- [ ] **Step 1: Write tests**

Add to `crates/data/src/talent_buffs.rs` tests:

```rust
#[test]
fn test_find_jahoda_buffs() {
    let buffs = find_talent_buffs("jahoda").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
    assert!((buffs[0].base_value - 100.0).abs() < 1e-6);
}
```

Add character lookup test in appropriate test file or `lib.rs` tests:

```rust
#[test]
fn test_find_jahoda_character() {
    let jahoda = crate::find_character("jahoda").unwrap();
    assert_eq!(jahoda.element, Element::Anemo);
    assert_eq!(jahoda.weapon_type, WeaponType::Bow);
    assert!(jahoda.base_hp[3] > 0.0); // Lv90 HP
    assert!(jahoda.base_atk[3] > 0.0);
    assert!(jahoda.base_def[3] > 0.0);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data test_find_jahoda`
Expected: FAIL

- [ ] **Step 3: Look up Jahoda's full data from Wiki**

Search the Genshin Impact Wiki for Jahoda's:
- Base HP/ATK/DEF at [Lv1, Lv20, Lv80, Lv90] (4 breakpoints)
- Ascension stat (e.g., `AscensionStat::Atk(0.24)`)
- Constellation pattern (`C3SkillC5Burst` or `C3BurstC5Skill`)
- Region (likely `Region::Snezhnaya` for Nod-Krai; add new variant if needed)
- **All talent scaling values** (Lv1-15 for each hit):
  - Normal attack: each hit's ATK% scaling
  - Charged attack: aimed shot, fully-charged shot
  - Plunging: plunge/low/high
  - Elemental Skill: all scaling entries
  - Elemental Burst: all scaling entries

Define each scaling as a `const TalentScaling` following the Diona pattern.

- [ ] **Step 4: Implement CharacterData**

Add `const` talent scaling entries, then the CharacterData to `crates/data/src/characters/anemo.rs`:

```rust
// =============================================================================
// Jahoda — 4-star Anemo Bow (v6.2, Nod-Krai)
// =============================================================================

// -- Normal Attack: [name from Wiki] -- Physical --
const JAHODA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [/* Lv1-15 from Wiki */],
};
// ... more normal hits, charged, plunging, skill, burst ...

pub const JAHODA: CharacterData = CharacterData {
    id: "jahoda",
    name: "Jahoda",
    element: Element::Anemo,
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    region: Region::Snezhnaya, // or new variant if needed
    base_hp: [/* Lv1, Lv20, Lv80, Lv90 from Wiki */],
    base_atk: [/* ... */],
    base_def: [/* ... */],
    ascension_stat: AscensionStat::/* from Wiki */,
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "/* Wiki name */",
            hits: &[JAHODA_NORMAL_1, /* ... */],
            charged: &[/* aimed, fully charged */],
            plunging: &[/* plunge, low, high */],
        },
        elemental_skill: TalentData {
            name: "/* Wiki name */",
            scalings: &[/* skill scaling entries */],
        },
        elemental_burst: TalentData {
            name: "/* Wiki name */",
            scalings: &[/* burst scaling entries */],
        },
    },
    constellation_pattern: ConstellationPattern::/* from Wiki */,
};
```

Register in `crates/data/src/characters/mod.rs` ALL_CHARACTERS: `&anemo::JAHODA,`

Add talent buff in `crates/data/src/talent_buffs.rs`:

```rust
// ===== Jahoda =====
// A4 passive: EM+100 when Burst robots heal characters with HP>70%
static JAHODA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Jahoda A4 EM Buff",
    description: "When burst heal target has HP>70%, EM+100 for 6s (assumes active)",
    stat: BuffableStat::ElementalMastery,
    base_value: 100.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];
```

Register: `("jahoda", JAHODA_BUFFS),`

- [ ] **Step 5: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All pass

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/characters/anemo.rs crates/data/src/characters/mod.rs crates/data/src/talent_buffs.rs
git commit -m "feat: add Jahoda CharacterData and A4 talent buff"
```

---

### Task 8: Aino CharacterData + Talent Buff

**Files:**
- Modify: `crates/data/src/characters/hydro.rs`
- Modify: `crates/data/src/characters/mod.rs`
- Modify: `crates/data/src/talent_buffs.rs`

Same approach as Task 7. Full CharacterData with all talent scalings required (~150-300 lines). See Diona in `cryo.rs` for reference.

- [ ] **Step 1: Write tests**

```rust
#[test]
fn test_find_aino_buffs() {
    let buffs = find_talent_buffs("aino").unwrap();
    assert_eq!(buffs.len(), 1);
    assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
    assert!((buffs[0].base_value - 80.0).abs() < 1e-6);
    assert_eq!(buffs[0].min_constellation, 1);
}

#[test]
fn test_find_aino_character() {
    let aino = crate::find_character("aino").unwrap();
    assert_eq!(aino.element, Element::Hydro);
    assert_eq!(aino.weapon_type, WeaponType::Claymore);
    assert!(aino.base_hp[3] > 0.0);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data test_find_aino`
Expected: FAIL

- [ ] **Step 3: Look up Aino's full data from Wiki and implement**

Same as Task 7 Step 3: look up all base stats, ascension stat, constellation pattern, region, and **all talent scaling values** (Lv1-15 for every hit/skill/burst entry).

Add const talent scaling entries + CharacterData to `crates/data/src/characters/hydro.rs` following the Diona pattern:

```rust
// =============================================================================
// Aino — 4-star Hydro Claymore (v6.0, Nod-Krai)
// =============================================================================

const AINO_NORMAL_1: TalentScaling = TalentScaling { /* Wiki values */ };
// ... all talent entries ...

pub const AINO: CharacterData = CharacterData {
    id: "aino",
    name: "Aino",
    element: Element::Hydro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    region: Region::Snezhnaya,
    base_hp: [/* Wiki */],
    base_atk: [/* Wiki */],
    base_def: [/* Wiki */],
    ascension_stat: AscensionStat::/* Wiki */,
    talents: TalentSet { /* full talent data */ },
    constellation_pattern: ConstellationPattern::/* Wiki */,
};
```

Register in `mod.rs` ALL_CHARACTERS: `&hydro::AINO,`

Add talent buff:

```rust
// ===== Aino =====
// C1: EM+80 to Aino and active party member after skill/burst
static AINO_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Aino C1 EM Share",
    description: "After Skill or Burst, Aino and active character gain EM+80 for 15s",
    stat: BuffableStat::ElementalMastery,
    base_value: 80.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(1),
    min_constellation: 1,
}];
```

Register: `("aino", AINO_BUFFS),`

- [ ] **Step 4: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All pass

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/characters/hydro.rs crates/data/src/characters/mod.rs crates/data/src/talent_buffs.rs
git commit -m "feat: add Aino CharacterData and C1 talent buff"
```

---

### Task 9: Integration Tests in team_builder.rs

**Files:**
- Modify: `crates/data/src/team_builder.rs` (tests only)

- [ ] **Step 1: Write integration tests**

Add to `crates/data/src/team_builder.rs` tests module:

```rust
#[test]
fn test_faruzan_burst_buff_at_lv13() {
    let faruzan = find_character("faruzan").unwrap();
    let weapon = find_weapon("favonius_warbow").unwrap();
    let member = TeamMemberBuilder::new(faruzan, weapon)
        .talent_levels([1, 1, 13])
        .build()
        .unwrap();

    let buff = member
        .buffs_provided
        .iter()
        .find(|b| b.source.contains("Prayerful Wind"))
        .expect("Should have Faruzan burst buff");
    assert_eq!(buff.stat, genshin_calc_core::BuffableStat::ElementalDmgBonus(
        genshin_calc_core::Element::Anemo
    ));
    assert!(buff.value > 0.0, "Buff value should be positive");
}

#[test]
fn test_diona_c6_constellation_gate() {
    let diona = find_character("diona").unwrap();
    let weapon = find_weapon("favonius_warbow").unwrap();

    // C0: no EM buff
    let c0 = TeamMemberBuilder::new(diona, weapon)
        .constellation(0)
        .build()
        .unwrap();
    assert!(
        !c0.buffs_provided.iter().any(|b| b.source.contains("Cat's Tail")),
        "C0 Diona should not have C6 buff"
    );

    // C6: EM+200 buff present
    let c6 = TeamMemberBuilder::new(diona, weapon)
        .constellation(6)
        .build()
        .unwrap();
    let buff = c6
        .buffs_provided
        .iter()
        .find(|b| b.source.contains("Cat's Tail"))
        .expect("C6 Diona should have EM buff");
    assert!((buff.value - 200.0).abs() < EPSILON);
}

#[test]
fn test_shenhe_a1_press_in_buffs_provided() {
    let shenhe = find_character("shenhe").unwrap();
    let weapon = find_weapon("calamity_queller").unwrap();
    let member = TeamMemberBuilder::new(shenhe, weapon)
        .build()
        .unwrap();

    assert!(
        member.buffs_provided.iter().any(|b| b.stat == genshin_calc_core::BuffableStat::SkillDmgBonus),
        "Should have SkillDmgBonus from A1"
    );
    assert!(
        member.buffs_provided.iter().any(|b| b.stat == genshin_calc_core::BuffableStat::BurstDmgBonus),
        "Should have BurstDmgBonus from A1"
    );
}

#[test]
fn test_sara_c6_crit_dmg_gate() {
    let sara = find_character("kujou_sara").unwrap();
    let weapon = find_weapon("favonius_warbow").unwrap();

    // C0: only ATK buff, no CritDmg
    let c0 = TeamMemberBuilder::new(sara, weapon)
        .constellation(0)
        .build()
        .unwrap();
    assert!(
        !c0.buffs_provided.iter().any(|b| b.stat == genshin_calc_core::BuffableStat::CritDmg),
        "C0 Sara should not have C6 CritDmg buff"
    );

    // C6: CritDmg present
    let c6 = TeamMemberBuilder::new(sara, weapon)
        .constellation(6)
        .build()
        .unwrap();
    let buff = c6
        .buffs_provided
        .iter()
        .find(|b| b.stat == genshin_calc_core::BuffableStat::CritDmg)
        .expect("C6 Sara should have CritDmg buff");
    assert!((buff.value - 0.60).abs() < EPSILON);
}
```

Also add Jahoda/Aino integration tests:

```rust
#[test]
fn test_jahoda_builds_with_talent_buff() {
    let jahoda = find_character("jahoda").unwrap();
    // Use any bow — pick an existing one
    let weapon = find_weapon("favonius_warbow").unwrap();
    let member = TeamMemberBuilder::new(jahoda, weapon).build().unwrap();
    assert!(
        member.buffs_provided.iter().any(|b| b.source.contains("Jahoda A4")),
        "Should have Jahoda A4 EM buff"
    );
}

#[test]
fn test_aino_c1_builds_with_talent_buff() {
    let aino = find_character("aino").unwrap();
    // Use any claymore
    let weapon = find_weapon("favonius_greatsword").unwrap();

    // C0: no C1 buff
    let c0 = TeamMemberBuilder::new(aino, weapon)
        .constellation(0)
        .build()
        .unwrap();
    assert!(
        !c0.buffs_provided.iter().any(|b| b.source.contains("Aino C1")),
        "C0 Aino should not have C1 buff"
    );

    // C1: EM+80 present
    let c1 = TeamMemberBuilder::new(aino, weapon)
        .constellation(1)
        .build()
        .unwrap();
    let buff = c1
        .buffs_provided
        .iter()
        .find(|b| b.source.contains("Aino C1"))
        .expect("C1 Aino should have EM buff");
    assert!((buff.value - 80.0).abs() < EPSILON);
}
```

- [ ] **Step 2: Run integration tests**

Run: `cargo test -p genshin-calc-data`
Expected: All pass

- [ ] **Step 3: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "test: add integration tests for new talent buffs"
```

---

### Task 10: Final Verification + TODO Update

**Files:**
- Modify: `docs/data-expansion-todo.md`

- [ ] **Step 1: Run full test suite**

Run: `cargo test`
Expected: All tests pass (core + data)

- [ ] **Step 2: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

- [ ] **Step 3: Verify talent buff count**

Run: `cargo test -p genshin-calc-data test_all_talent_buffs_have_unique_ids -- --nocapture`
Expected: Pass. ALL_TALENT_BUFFS should now have 27 entries (9 original + 18 new).

- [ ] **Step 4: Update TODO**

In `docs/data-expansion-todo.md`, update P1 section checklist items to `[x]` for all completed characters. Update the summary table:

```markdown
| 天賦バフ (TalentBuffDef) | ~30対象 | 27 | ~3 | ~90% |
```

- [ ] **Step 5: Commit**

```bash
git add docs/data-expansion-todo.md
git commit -m "docs: update data expansion TODO with P1 talent buff completion"
```
