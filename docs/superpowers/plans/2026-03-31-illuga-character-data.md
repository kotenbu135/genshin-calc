# Illuga Character Data Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add Illuga (Geo/Polearm/★4) character data with EM+DEF dual scaling talents, A4 talent buffs, and TOML test case.

**Architecture:** New character data file following Kachina/Lauma patterns. Dual scaling on Skill/Burst uses `_EM`/`_DEF` suffix convention (Lauma precedent). Burst Lunar-Crystallize DMG increase modeled as TalentScaling (Lauma bloom increase pattern). A4 buffs + burst Geo DMG Bonus as TalentBuffDef entries.

**Tech Stack:** Rust (const data definitions), TOML (test cases)

**Spec:** `docs/superpowers/specs/2026-03-31-illuga-character-data-design.md`

---

### Task 1: Create character data file

**Files:**
- Create: `crates/data/src/characters/geo/illuga.rs`
- Reference: `crates/data/src/characters/geo/kachina.rs` (Geo/Polearm/★4 pattern)
- Reference: `crates/data/src/characters/dendro/lauma.rs` (dual scaling pattern)

- [ ] **Step 1: Create `illuga.rs` with all talent scalings and CharacterData**

```rust
use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// =============================================================================
// Illuga
// =============================================================================

// -- Normal Attack: Oathkeeper's Spear -- Physical --

const ILLUGA_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4737, 0.5122, 0.5508, 0.6058, 0.6444, 0.6885, 0.7490, 0.8096,
        0.8702, 0.9363, 1.0024, 1.0685, 1.1346, 1.2007, 1.2668,
    ],
};

const ILLUGA_NORMAL_2: TalentScaling = TalentScaling {
    name: "2段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.4853, 0.5248, 0.5642, 0.6207, 0.6602, 0.7053, 0.7674, 0.8294,
        0.8915, 0.9592, 1.0269, 1.0946, 1.1624, 1.2301, 1.2978,
    ],
};

const ILLUGA_NORMAL_3: TalentScaling = TalentScaling {
    name: "3段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.3143, 0.3399, 0.3655, 0.4020, 0.4276, 0.4569, 0.4971, 0.5373,
        0.5775, 0.6213, 0.6652, 0.7091, 0.7529, 0.7968, 0.8407,
    ],
};

const ILLUGA_NORMAL_4: TalentScaling = TalentScaling {
    name: "4段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.7628, 0.8249, 0.8870, 0.9757, 1.0377, 1.1087, 1.2063, 1.3038,
        1.4014, 1.5078, 1.6143, 1.7207, 1.8271, 1.9336, 2.0400,
    ],
};

// -- Charged Attack -- Physical --

const ILLUGA_CHARGED: TalentScaling = TalentScaling {
    name: "重撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.1103, 1.2006, 1.2910, 1.4201, 1.5105, 1.6137, 1.7558, 1.8978,
        2.0398, 2.1947, 2.3496, 2.5045, 2.6595, 2.8144, 2.9693,
    ],
};

// -- Plunging Attack -- Physical --

const ILLUGA_PLUNGE: TalentScaling = TalentScaling {
    name: "落下期間のダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928,
        1.1746, 1.2638, 1.3530, 1.4422, 1.5314, 1.6206, 1.7098,
    ],
};

const ILLUGA_PLUNGE_LOW: TalentScaling = TalentScaling {
    name: "低空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851,
        2.3486, 2.5270, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189,
    ],
};

const ILLUGA_PLUNGE_HIGH: TalentScaling = TalentScaling {
    name: "高空落下攻撃ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,
    values: [
        1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293,
        2.9336, 3.1564, 3.3792, 3.6020, 3.8248, 4.0476, 4.2704,
    ],
};

// -- Elemental Skill: Dawnbearing Songbird -- Geo (EM+DEF dual) --

const ILLUGA_SKILL_PRESS_EM: TalentScaling = TalentScaling {
    name: "Press DMG・元素熟知",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Geo),
    values: [
        4.8256, 5.1875, 5.5494, 6.0320, 6.3939, 6.7558, 7.2384, 7.7210,
        8.2035, 8.6861, 9.1686, 9.6512, 10.2544, 10.8576, 11.4608,
    ],
};

const ILLUGA_SKILL_PRESS_DEF: TalentScaling = TalentScaling {
    name: "Press DMG・防御力",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        2.4128, 2.5938, 2.7747, 3.0160, 3.1970, 3.3779, 3.6192, 3.8605,
        4.1018, 4.3430, 4.5843, 4.8256, 5.1272, 5.4288, 5.7304,
    ],
};

const ILLUGA_SKILL_HOLD_EM: TalentScaling = TalentScaling {
    name: "Hold DMG・元素熟知",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Geo),
    values: [
        6.0320, 6.4844, 6.9368, 7.5400, 7.9924, 8.4448, 9.0480, 9.6512,
        10.2544, 10.8576, 11.4608, 12.0640, 12.8180, 13.5720, 14.3260,
    ],
};

const ILLUGA_SKILL_HOLD_DEF: TalentScaling = TalentScaling {
    name: "Hold DMG・防御力",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        3.0160, 3.2422, 3.4684, 3.7700, 3.9962, 4.2224, 4.5240, 4.8256,
        5.1272, 5.4288, 5.7304, 6.0320, 6.4090, 6.7860, 7.1630,
    ],
};

// -- Elemental Burst: Shadowless Reflection -- Geo (EM+DEF dual) --

const ILLUGA_BURST_EM: TalentScaling = TalentScaling {
    name: "スキルダメージ・元素熟知",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Geo),
    values: [
        8.2720, 8.8924, 9.5128, 10.3400, 10.9604, 11.5808, 12.4080, 13.2352,
        14.0624, 14.8896, 15.7168, 16.5440, 17.5780, 18.6120, 19.6460,
    ],
};

const ILLUGA_BURST_DEF: TalentScaling = TalentScaling {
    name: "スキルダメージ・防御力",
    scaling_stat: ScalingStat::Def,
    damage_element: Some(Element::Geo),
    values: [
        4.1360, 4.4462, 4.7564, 5.1700, 5.4802, 5.7904, 6.2040, 6.6176,
        7.0312, 7.4448, 7.8584, 8.2720, 8.7890, 9.3060, 9.8230,
    ],
};

const ILLUGA_BURST_LUNAR_CRYSTALLIZE: TalentScaling = TalentScaling {
    name: "月結晶化ダメージ増加",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Geo),
    values: [
        2.2592, 2.4286, 2.5981, 2.8240, 2.9934, 3.1629, 3.3888, 3.6147,
        3.8406, 4.0666, 4.2925, 4.5184, 4.8008, 5.0832, 5.3656,
    ],
};

// -- Aggregation --

static ILLUGA_NA_HITS: &[TalentScaling] = &[
    ILLUGA_NORMAL_1, ILLUGA_NORMAL_2, ILLUGA_NORMAL_3, ILLUGA_NORMAL_4,
];
static ILLUGA_CHARGED_ATTACKS: &[TalentScaling] = &[ILLUGA_CHARGED];
static ILLUGA_PLUNGING: &[TalentScaling] = &[
    ILLUGA_PLUNGE, ILLUGA_PLUNGE_LOW, ILLUGA_PLUNGE_HIGH,
];
static ILLUGA_SKILL_SCALINGS: &[TalentScaling] = &[
    ILLUGA_SKILL_PRESS_EM, ILLUGA_SKILL_PRESS_DEF,
    ILLUGA_SKILL_HOLD_EM, ILLUGA_SKILL_HOLD_DEF,
];
static ILLUGA_BURST_SCALINGS: &[TalentScaling] = &[
    ILLUGA_BURST_EM, ILLUGA_BURST_DEF, ILLUGA_BURST_LUNAR_CRYSTALLIZE,
];

pub const ILLUGA: CharacterData = CharacterData {
    id: "illuga",
    name: "Illuga",
    element: Element::Geo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Natlan,
    base_hp: [1003.0, 2577.0, 10602.0, 11962.0],
    base_atk: [16.03, 41.17, 169.41, 191.16],
    base_def: [68.21, 175.24, 721.02, 813.57],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "Oathkeeper's Spear",
            hits: ILLUGA_NA_HITS,
            charged: ILLUGA_CHARGED_ATTACKS,
            plunging: ILLUGA_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "Dawnbearing Songbird",
            scalings: ILLUGA_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "Shadowless Reflection",
            scalings: ILLUGA_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
```

- [ ] **Step 2: Verify it compiles (will fail — not registered in mod.rs yet, but syntax check)**

Run: `cargo check -p genshin-calc-data 2>&1 | head -5`
Expected: success (unregistered file is silently ignored by Rust compiler)

---

### Task 2: Register in mod.rs

**Files:**
- Modify: `crates/data/src/characters/geo/mod.rs`

- [ ] **Step 1: Add mod declaration, pub use, and CHARACTERS entry**

Add `mod illuga;` after `mod gorou;` (alphabetical).
Add `pub use illuga::ILLUGA;` after `pub use gorou::GOROU;`.
Add `&ILLUGA` to `CHARACTERS` slice (alphabetical, after `&GOROU` and before `&ITTO`).

- [ ] **Step 2: Verify build succeeds**

Run: `cargo build -p genshin-calc-data`
Expected: success

- [ ] **Step 3: Run existing data integrity tests**

Run: `cargo test -p genshin-calc-data`
Expected: all pass (no ID duplicates, base stats positive, etc.)

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/characters/geo/illuga.rs crates/data/src/characters/geo/mod.rs
git commit -m "feat: add Illuga (Geo/Polearm/★4) character data"
```

---

### Task 3: Add TOML test case

**Files:**
- Create: `crates/core/tests/data/characters/illuga.toml`
- Reference: `crates/core/tests/data/characters/kachina.toml` (Geo/DEF scaling test pattern)

- [ ] **Step 1: Create TOML test file with two cases**

```toml
# Source: 4-star Geo Polearm, Normal Attack Hit 1 at Talent Lv10
# Honey Impact: 93.63% ATK

[character]
name = "Illuga"
element = "Geo"

# Case 1: Normal Attack Hit 1 — no reaction, physical
[[cases]]
type = "normal"
name = "Normal Attack Hit 1 Lv10 — no reaction"
character_level = 90
talent_multiplier = 0.9363
scaling_stat = "Atk"
damage_type = "Normal"
element = "Physical"

[cases.stats]
hp = 15000.0
atk = 2000.0
def = 800.0
elemental_mastery = 0.0
crit_rate = 0.50
crit_dmg = 1.00
energy_recharge = 1.0
dmg_bonus = 0.0

[cases.enemy]
level = 90
resistance = 0.10

# non_crit = 2000 * 0.9363 * 1.0 * 0.5 * 0.9 = 842.67
# crit = 842.67 * (1 + 1.00) = 1685.34
# average = 842.67 * (1 + 0.50 * 1.00) = 1264.005
[cases.expected]
non_crit = 842.670000
crit = 1685.340000
average = 1264.005000

# Case 2: Skill Press DMG EM part — Geo, EM scaling
# Honey Impact Lv10: 868.61% EM
[[cases]]
type = "normal"
name = "Skill Press EM Lv10 — no reaction"
character_level = 90
talent_multiplier = 8.6861
scaling_stat = "Em"
damage_type = "Skill"
element = "Geo"

[cases.stats]
hp = 15000.0
atk = 800.0
def = 1500.0
elemental_mastery = 500.0
crit_rate = 0.50
crit_dmg = 1.00
energy_recharge = 1.0
dmg_bonus = 0.466

[cases.enemy]
level = 90
resistance = 0.10

# non_crit = 500 * 8.6861 * 1.466 * 0.5 * 0.9 = 2865.110085
# crit = 2865.110085 * (1 + 1.00) = 5730.220170
# average = 2865.110085 * (1 + 0.50 * 1.00) = 4297.665128
[cases.expected]
non_crit = 2865.110085
crit = 5730.220170
average = 4297.665128
```

- [ ] **Step 2: Run character verification tests**

Run: `cargo test -p genshin-calc-core --test character_verification -- illuga`
Expected: 2 cases pass

- [ ] **Step 3: Commit**

```bash
git add crates/core/tests/data/characters/illuga.toml
git commit -m "test: add Illuga damage test cases"
```

---

### Task 4: Add talent buffs

**Files:**
- Modify: `crates/data/src/talent_buffs/geo.rs`
- Reference: existing buffs in same file (Albedo, Gorou, Yun Jin patterns)

- [ ] **Step 1: Add ILLUGA_BURST_GEO_DMG_SCALING and ILLUGA_BUFFS**

After Zhongli's buffs, before the registry, add:

```rust
// ===== Illuga =====
// A4 passive "Torchforger's Covenant": CRIT Rate +5%, CRIT DMG +10%, EM +50 (Moonsign)
// Burst "Shadowless Reflection": Geo DMG Bonus based on burst talent level
static ILLUGA_BURST_GEO_DMG_SCALING: [f64; 15] = [
    0.336, 0.3612, 0.3864, 0.42, 0.4452, 0.4704, 0.504, 0.5376, 0.5712,
    0.6048, 0.6384, 0.672, 0.714, 0.756, 0.798,
];

static ILLUGA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Torchforger's Covenant - CRIT Rate",
        description: "After Geo DMG hits opponent, party CRIT Rate +5% for 20s",
        stat: BuffableStat::CritRate,
        base_value: 0.05,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Torchforger's Covenant - CRIT DMG",
        description: "After Geo DMG hits opponent, party CRIT DMG +10% for 20s",
        stat: BuffableStat::CritDmg,
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Torchforger's Covenant - EM (Moonsign)",
        description: "With Moonsign active, party EM +50 for 20s (A4 Ascendant Gleam condition)",
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
        name: "Shadowless Reflection - Geo DMG",
        description: "During burst, Geo DMG Bonus based on talent level (flat%, not EM-scaled)",
        stat: BuffableStat::ElementalDmgBonus(Element::Geo),
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&ILLUGA_BURST_GEO_DMG_SCALING),
        scales_on: None,
        target: BuffTarget::Character("illuga"),
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
    },
];
```

- [ ] **Step 2: Register in GEO_TALENT_BUFFS**

Add `("illuga", ILLUGA_BUFFS),` to `GEO_TALENT_BUFFS` slice (alphabetical, after `("gorou", GOROU_BUFFS)`).

- [ ] **Step 3: Verify build and tests**

Run: `cargo test -p genshin-calc-data`
Expected: all pass (buff name uniqueness test included)

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/talent_buffs/geo.rs
git commit -m "feat: add Illuga talent buffs (A4 + burst Geo DMG)"
```

---

### Task 5: Final verification

- [ ] **Step 1: Full build**

Run: `cargo build`
Expected: success

- [ ] **Step 2: Full test suite**

Run: `cargo test`
Expected: all pass

- [ ] **Step 3: Clippy**

Run: `cargo clippy -- -D warnings`
Expected: no warnings

- [ ] **Step 4: Format check**

Run: `cargo fmt --check`
Expected: no changes needed

- [ ] **Step 5: Update TODO**

Mark Illuga as done in `docs/v6.0-6.3-data-update-todo.md` (checkbox or strikethrough).
