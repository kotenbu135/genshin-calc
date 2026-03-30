# Flins (Electro/Polearm/★5) Character Data Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Flins's complete game data (character stats, talent scalings, talent buffs, moonsign check, tests) to genshin-calc.

**Architecture:** Data-only addition following existing Electro/Polearm pattern (Cyno reference). One new CharacterData const in electro.rs, registration in ALL_CHARACTERS, talent buffs in talent_buffs.rs, moonsign if applicable, TOML test case. No Region change needed (NodKrai already exists).

**Tech Stack:** Rust, serde, cargo test, genshin-db-api (data source)

---

## File Map

| Action | File | Responsibility |
|--------|------|---------------|
| Modify | `crates/data/src/characters/electro.rs` (append) | Flins TalentScaling consts + CharacterData |
| Modify | `crates/data/src/characters/mod.rs:45-63` | Add `&electro::FLINS` to ALL_CHARACTERS Electro section |
| Modify | `crates/data/src/talent_buffs.rs` | Add FLINS_BUFFS + register in ALL_TALENT_BUFFS (if applicable) |
| Verify | `crates/data/src/moonsign_chars.rs` | Add moonsign data if Flins has moon-related passives |
| Create | `crates/core/tests/data/characters/flins.toml` | Data-driven test cases |

---

### Task 1: Fetch Flins data from genshin-db-api

**Files:** None (research only)

- [ ] **Step 1: Fetch character base stats**

Query genshin-db-api for Flins's stats. Extract:
- `base_hp`, `base_atk`, `base_def` at Lv1, Lv20, Lv80, Lv90
- `ascension_stat` (type + value as decimal)
- `constellation_pattern` (check which constellation boosts Skill vs Burst)

- [ ] **Step 2: Fetch talent scalings**

Extract all talent scalings as `[f64; 15]` arrays (Lv1-15):
- Normal Attack: each hit's multiplier (Physical — `damage_element: None`)
- Charged attack, plunging (3 variants) — also Physical
- Elemental Skill: each damage instance (`damage_element: Some(Element::Electro)`)
- Elemental Burst: each damage instance (`damage_element: Some(Element::Electro)`)

**IMPORTANT — Infusion check:** If Flins has a burst/skill that infuses normal attacks with Electro (like Cyno), extract the burst-infused normal attack scalings separately. These go under `elemental_burst.scalings` with `damage_element: Some(Element::Electro)`, following the Cyno pattern in electro.rs (search for `CYNO_BURST_NORMAL_1`).

All values as decimals (e.g. 10.8% → 0.108).

- [ ] **Step 3: Fetch passive talents and constellations**

Extract A1/A4 passives and C1-C6 descriptions. Identify stat buffs:
- IMPLEMENT: ATK/DMG/CRIT/EM stat buffs → TalentBuffDef
- SKIP: proc damage, healing, energy generation, utility

Also check: does Flins have moon-related passives (Moonsign benediction, lunar reactions)?

---

### Task 2: Implement Flins CharacterData in electro.rs

**Files:**
- Modify: `crates/data/src/characters/electro.rs` (append at end of file)

- [ ] **Step 1: Define TalentScaling consts for Normal Attack**

Append to `electro.rs`. Polearm normals are Physical:

```rust
// =============================================================================
// Flins — 5★ Electro Polearm (Nod-Krai)
// Source: genshin-db-api (genshin-db-api.vercel.app)
// Normal Attack: <name from API>
// Elemental Skill: <name from API>
// Elemental Burst: <name from API>
// =============================================================================

const FLINS_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None, // Physical — Polearm
    values: [/* 15 values from API */],
};
// ... repeat for each normal hit, charged, plunging
```

- [ ] **Step 2: Define TalentScaling consts for Skill and Burst**

Skill and Burst hits are Electro:

```rust
const FLINS_SKILL_DMG: TalentScaling = TalentScaling {
    name: "<skill hit name>",
    scaling_stat: ScalingStat::Atk, // or Em if EM-scaling
    damage_element: Some(Element::Electro),
    values: [/* 15 values from API */],
};
```

If Flins has burst-infused normal attacks (Cyno pattern), add them as burst scalings:

```rust
const FLINS_BURST_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ(元素爆発)",
    scaling_stat: ScalingStat::Atk,
    damage_element: Some(Element::Electro), // Infused
    values: [/* 15 values from API */],
};
```

- [ ] **Step 3: Define CharacterData const**

Note: electro.rs ではインラインスライスパターンを使用（Cyno準拠）。dendro.rs の `static` 中間変数パターンは使わない。

```rust
pub const FLINS: CharacterData = CharacterData {
    id: "flins",
    name: "Flins",
    element: Element::Electro,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    region: Region::NodKrai,
    base_hp: [/* Lv1, Lv20, Lv80, Lv90 */],
    base_atk: [/* Lv1, Lv20, Lv80, Lv90 */],
    base_def: [/* Lv1, Lv20, Lv80, Lv90 */],
    ascension_stat: AscensionStat::/* from API */,
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "<通常攻撃名 from API>",
            hits: &[FLINS_NORMAL_1, FLINS_NORMAL_2, /* ... */],
            charged: &[FLINS_CHARGED],
            plunging: &[FLINS_PLUNGE, FLINS_PLUNGE_LOW, FLINS_PLUNGE_HIGH],
        },
        elemental_skill: TalentData {
            name: "<スキル名 from API>",
            scalings: &[FLINS_SKILL_DMG, /* ... */],
        },
        elemental_burst: TalentData {
            name: "<爆発名 from API>",
            scalings: &[/* burst hits + infused normals if applicable */],
        },
    },
    constellation_pattern: ConstellationPattern::/* from API */,
};
```

- [ ] **Step 4: Build to verify compilation**

Run: `cargo build -p genshin-calc-data 2>&1 | head -20`
Expected: success

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/characters/electro.rs
git commit -m "feat: add Flins (Electro/Polearm/5★) character data"
```

---

### Task 3: Register Flins in ALL_CHARACTERS

**Files:**
- Modify: `crates/data/src/characters/mod.rs:45-63`

- [ ] **Step 1: Add Flins to Electro section**

Insert `&electro::FLINS` in alphabetical order (after FISCHL, before IANSAN). Update the Electro count comment:

```rust
    // Electro (18)
    &electro::BEIDOU,
    &electro::CLORINDE,
    &electro::CYNO,
    &electro::DORI,
    &electro::FISCHL,
    &electro::FLINS,
    &electro::IANSAN,
    // ... rest
```

- [ ] **Step 2: Build and run find_character test**

Run: `cargo test -p genshin-calc-data test_find 2>&1 | tail -10`
Expected: all find tests pass

- [ ] **Step 3: Commit**

```bash
git add crates/data/src/characters/mod.rs
git commit -m "feat: register Flins in ALL_CHARACTERS"
```

---

### Task 4: Add Flins talent buffs (if applicable)

**Files:**
- Modify: `crates/data/src/talent_buffs.rs`

- [ ] **Step 1: Define FLINS_BUFFS static (if stat buffs found in Task 1)**

Based on Task 1 research. Example pattern:

```rust
static FLINS_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "<buff name>",
        description: "<description>",
        stat: BuffableStat::/* stat type */,
        base_value: /* value as decimal */,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team, // or OnlySelf / TeamExcludeSelf
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
];
```

If no stat buffs found (all proc/heal/utility), skip this task and note in commit.

- [ ] **Step 2: Register in ALL_TALENT_BUFFS**

Append to the `ALL_TALENT_BUFFS` array (末尾に追加 — 既存は挿入順):

```rust
    ("flins", FLINS_BUFFS),
```

- [ ] **Step 3: Run talent buff tests**

Run: `cargo test -p genshin-calc-data test_talent 2>&1 | tail -10`
Expected: all pass

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/talent_buffs.rs
git commit -m "feat: add Flins talent buffs"
```

---

### Task 5: Add moonsign data (if applicable)

**Files:**
- Modify: `crates/data/src/moonsign_chars.rs` (if Flins has moon-related passives)

- [ ] **Step 1: Check API data from Task 1**

If Flins has a Moonsign benediction passive or talent enhancements, add them following the existing pattern (search for "lauma" in the file for reference).

If no moon-related passives, skip this task.

- [ ] **Step 2: Run moonsign tests**

Run: `cargo test -p genshin-calc-data test_find_moonsign 2>&1 | tail -10`
Expected: all pass

- [ ] **Step 3: Commit (if changes made)**

```bash
git add crates/data/src/moonsign_chars.rs
git commit -m "feat: add Flins moonsign data"
```

---

### Task 6: Create TOML test case

**Files:**
- Create: `crates/core/tests/data/characters/flins.toml`

- [ ] **Step 1: Write Physical normal damage test case**

Use Lv90 character, known talent multiplier from Task 1 data:

```toml
# Source: Electro Polearm DPS

[character]
name = "Flins"
element = "Electro"

# Case 1: Normal Attack Hit 1 — Physical, no reaction
[[cases]]
type = "normal"
name = "Normal Attack 1 — Physical, no reaction"
character_level = 90
talent_multiplier = # Lv10 value from FLINS_NORMAL_1
scaling_stat = "Atk"
damage_type = "Normal"
element = "Physical"

[cases.stats]
hp = 20000.0
atk = 2000.0
def = 700.0
elemental_mastery = 100.0
crit_rate = 0.50
crit_dmg = 1.00
energy_recharge = 1.0
dmg_bonus = 0.0

[cases.enemy]
level = 90
resistance = 0.10

[cases.expected]
non_crit = # hand-calculated
crit = # non_crit * (1 + crit_dmg)
average = # non_crit * (1 + crit_rate * crit_dmg)
```

Hand-calculation:
- `base_dmg = atk * talent_multiplier`
- `dmg_bonus_mult = 1 + dmg_bonus = 1.0`
- `defense_mult = 190 / 380 = 0.5`
- `resistance_mult = 1 - 0.10 = 0.90`
- `non_crit = base_dmg * dmg_bonus_mult * defense_mult * resistance_mult`

- [ ] **Step 2: Add Electro Skill damage test case**

```toml
# Case 2: Skill — Electro, no reaction
[[cases]]
type = "normal"
name = "Skill DMG — Electro, no reaction"
character_level = 90
talent_multiplier = # Lv10 value from FLINS_SKILL_DMG
scaling_stat = "Atk"
damage_type = "Skill"
element = "Electro"

[cases.stats]
hp = 20000.0
atk = 2000.0
def = 700.0
elemental_mastery = 100.0
crit_rate = 0.50
crit_dmg = 1.00
energy_recharge = 1.0
dmg_bonus = 0.466

[cases.enemy]
level = 90
resistance = 0.10

[cases.expected]
non_crit = # hand-calculated (with dmg_bonus = 0.466)
crit = # non_crit * 2.0
average = # non_crit * 1.5
```

- [ ] **Step 3: Run character verification test**

Run: `cargo test --test character_verification 2>&1 | tail -20`
Expected: all tests pass including Flins cases

- [ ] **Step 4: Commit**

```bash
git add crates/core/tests/data/characters/flins.toml
git commit -m "test: add Flins character verification TOML test cases"
```

---

### Task 7: Full verification

- [ ] **Step 1: Run full test suite**

Run: `cargo test 2>&1 | tail -10`
Expected: all tests pass

- [ ] **Step 2: Run clippy**

Run: `cargo clippy -- -D warnings 2>&1 | tail -10`
Expected: no warnings

- [ ] **Step 3: Run fmt check**

Run: `cargo fmt --check 2>&1`
Expected: no formatting issues

- [ ] **Step 4: Final commit (if any formatting fixes needed)**

```bash
cargo fmt
git add -A
git commit -m "style: format Flins implementation"
```
