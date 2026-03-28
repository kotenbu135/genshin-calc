# P3 Batch 1: 5星武器 StatScaling ConditionalBuff Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend `AutoCondition::StatScaling` with offset/cap refinement support, add flat damage to the damage pipeline, and implement StatScaling ConditionalBuff for 8 five-star weapons.

**Architecture:** Four core extensions (BuffableStat variants, DamageInput.flat_dmg, StatScaling struct changes, DefPercentRaw) followed by 8 weapon data additions. All changes are backward-compatible via `Option` defaults.

**Tech Stack:** Rust, genshin-calc-core, genshin-calc-data

**Spec:** `docs/superpowers/specs/2026-03-29-weapon-statscaling-batch1-design.md`

---

### Task 1: Add BuffableStat variants (FlatDmg × 5 + DefPercentRaw)

**Files:**
- Modify: `crates/core/src/buff_types.rs`

- [ ] **Step 1: Add 6 new variants**

In `crates/core/src/buff_types.rs`, add after `DefReduction`:

```rust
    DefReduction,
    // Flat damage added to base (ATK*multiplier + flat_dmg), used by weapon passives
    NormalAtkFlatDmg,
    ChargedAtkFlatDmg,
    PlungingAtkFlatDmg,
    SkillFlatDmg,
    BurstFlatDmg,
    // Raw def_percent value (not total DEF), for weapons scaling on "DEF increase"
    DefPercentRaw,
```

- [ ] **Step 2: Add serde roundtrip tests**

Add to existing `mod tests` in `buff_types.rs`:

```rust
    #[test]
    fn test_flat_dmg_variants_serde_roundtrip() {
        for stat in [
            BuffableStat::NormalAtkFlatDmg,
            BuffableStat::ChargedAtkFlatDmg,
            BuffableStat::PlungingAtkFlatDmg,
            BuffableStat::SkillFlatDmg,
            BuffableStat::BurstFlatDmg,
            BuffableStat::DefPercentRaw,
        ] {
            let json = serde_json::to_string(&stat).unwrap();
            let deser: BuffableStat = serde_json::from_str(&json).unwrap();
            assert_eq!(stat, deser);
        }
    }
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-core && cargo test -p genshin-calc-data`
Expected: All PASS

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/buff_types.rs
git commit -m "feat(core): add FlatDmg and DefPercentRaw BuffableStat variants"
```

---

### Task 2: Add `DamageInput.flat_dmg` + `collect_flat_dmg` helper

**Files:**
- Modify: `crates/core/src/damage.rs` (add field + formula change + helper + update ~30 construction sites)
- Modify: `crates/core/src/lib.rs` (doc examples — 2 sites)
- Modify: `crates/core/src/enemy.rs` (integration tests — 3 sites)
- Modify: `crates/data/src/types.rs` (1 site)
- Modify: `crates/data/tests/core_integration.rs` (2 sites)
- Modify: `crates/data/tests/team_integration.rs` (1 site)

- [ ] **Step 1: Write failing test for flat_dmg**

Add to `damage.rs` test module:

```rust
    #[test]
    fn test_flat_dmg_added_to_base() {
        let mut input = valid_input();
        input.flat_dmg = 500.0;
        input.stats.crit_rate = 0.0;
        input.stats.crit_dmg = 0.0;
        input.stats.dmg_bonus = 0.0;
        input.talent_multiplier = 1.0;
        input.reaction = None;

        let enemy = Enemy { level: 90, resistance: 0.0, def_reduction: 0.0 };
        let result = calculate_damage(&input, &enemy).unwrap();

        // base = atk * 1.0 + flat_dmg = 2000 + 500 = 2500
        // non_crit = 2500 * 1.0 * 0.5 * 1.0 = 1250
        let expected = (2000.0 + 500.0) * 0.5; // def_mult=0.5 for lv90 vs lv90
        assert!((result.non_crit - expected).abs() < 1.0);
    }

    #[test]
    fn test_flat_dmg_zero_unchanged() {
        let mut input = valid_input();
        input.flat_dmg = 0.0;
        let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
        let result_zero = calculate_damage(&input, &enemy).unwrap();

        // Should be identical to the case without flat_dmg field
        let result_default = calculate_damage(&valid_input(), &enemy).unwrap();
        assert!((result_zero.non_crit - result_default.non_crit).abs() < 1e-6);
    }
```

- [ ] **Step 2: Add `flat_dmg` field to `DamageInput`**

In `crates/core/src/damage.rs`, add after `reaction_bonus`:

```rust
    /// Reaction DMG bonus from artifacts/buffs in decimal form.
    pub reaction_bonus: f64,
    /// Flat damage added to base (e.g. Shenhe quill, weapon flat DMG scaling).
    /// Added after ATK * talent_multiplier in the damage formula.
    #[serde(default)]
    pub flat_dmg: f64,
```

- [ ] **Step 3: Update `calculate_damage` formula**

Change line 170:
```rust
// Before:
let base = scaling_value * input.talent_multiplier + catalyze_flat;
// After:
let base = scaling_value * input.talent_multiplier + catalyze_flat + input.flat_dmg;
```

- [ ] **Step 4: Add `collect_flat_dmg` helper**

Add after `calculate_damage` function:

```rust
/// Collects flat damage bonus from resolved buffs for a specific damage type.
///
/// Matches `NormalAtkFlatDmg`, `ChargedAtkFlatDmg`, `PlungingAtkFlatDmg`,
/// `SkillFlatDmg`, or `BurstFlatDmg` against the damage type and sums values.
pub fn collect_flat_dmg(buffs: &[ResolvedBuff], damage_type: DamageType) -> f64 {
    use crate::buff_types::BuffableStat;
    let target_stat = match damage_type {
        DamageType::Normal => BuffableStat::NormalAtkFlatDmg,
        DamageType::Charged => BuffableStat::ChargedAtkFlatDmg,
        DamageType::Plunging => BuffableStat::PlungingAtkFlatDmg,
        DamageType::Skill => BuffableStat::SkillFlatDmg,
        DamageType::Burst => BuffableStat::BurstFlatDmg,
    };
    buffs
        .iter()
        .filter(|b| b.stat == target_stat)
        .map(|b| b.value)
        .sum()
}
```

Add import at top of `damage.rs`:
```rust
use crate::team::ResolvedBuff;
```

- [ ] **Step 5: Add `collect_flat_dmg` test**

```rust
    #[test]
    fn test_collect_flat_dmg() {
        use crate::team::{BuffTarget, ResolvedBuff};
        use crate::buff_types::BuffableStat;

        let buffs = vec![
            ResolvedBuff {
                source: "Weapon".into(),
                stat: BuffableStat::NormalAtkFlatDmg,
                value: 200.0,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "Weapon2".into(),
                stat: BuffableStat::NormalAtkFlatDmg,
                value: 100.0,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "Other".into(),
                stat: BuffableStat::SkillFlatDmg,
                value: 500.0,
                target: BuffTarget::Team,
            },
        ];
        assert!((collect_flat_dmg(&buffs, DamageType::Normal) - 300.0).abs() < 1e-6);
        assert!((collect_flat_dmg(&buffs, DamageType::Skill) - 500.0).abs() < 1e-6);
        assert!((collect_flat_dmg(&buffs, DamageType::Charged) - 0.0).abs() < 1e-6);
    }
```

- [ ] **Step 6: Update `valid_input()` helper and ALL `DamageInput` construction sites**

Add `flat_dmg: 0.0` to every `DamageInput { ... }` in the codebase. Use `grep -rn "DamageInput {" crates/ --include="*.rs"` to find ALL sites (~57 total across):
- `crates/core/src/damage.rs` (~43 sites including `valid_input()`, all test inputs, doc examples)
- `crates/core/src/lib.rs` (2 doc examples)
- `crates/core/src/enemy.rs` (3 integration tests)
- `crates/core/tests/character_verification.rs` (1 site — critical: drives 153 test cases)
- `crates/data/src/types.rs` (1 site)
- `crates/data/tests/core_integration.rs` (2 sites)
- `crates/data/tests/team_integration.rs` (1 site)

Also check for examples and docs:
- `crates/core/examples/` (if exists)
- Any `.md` files with DamageInput code blocks

Use `grep -rn "DamageInput {" crates/ --include="*.rs"` to find all sites.

- [ ] **Step 7: Update `lib.rs` re-exports**

Add `collect_flat_dmg` to damage re-exports:
```rust
pub use damage::{DamageInput, DamageResult, calculate_damage, collect_flat_dmg};
```

- [ ] **Step 8: Run tests**

Run: `cargo test -p genshin-calc-core && cargo test -p genshin-calc-data`
Expected: All PASS

- [ ] **Step 9: Commit**

```bash
git add crates/
git commit -m "feat(core): add DamageInput.flat_dmg field and collect_flat_dmg helper"
```

---

### Task 3: Modify `StatScaling` struct (offset + cap type change)

**Files:**
- Modify: `crates/data/src/buff.rs` (struct definition)
- Modify: `crates/data/src/team_builder.rs` (`eval_auto` + tests)
- Modify: `crates/data/src/weapons/sword.rs` (Jade Cutter — 1 site)
- Modify: `crates/data/src/weapons/polearm.rs` (Homa — 2 sites)
- Modify: `crates/data/src/artifacts.rs` (3 sites)

- [ ] **Step 1: Write failing tests for offset and cap changes**

Add to `team_builder.rs` test module:

```rust
    #[test]
    fn test_eval_auto_stat_scaling_with_offset() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::EnergyRecharge,
            offset: Some(1.0),
            cap: None,
        };
        let profile = StatProfile {
            energy_recharge: 1.80,
            ..Default::default()
        };
        // (1.80 - 1.0) * 0.28 = 0.224
        let result = eval_auto(&cond, 0.28, &profile, WeaponType::Polearm, Element::Electro, &[], 1);
        assert!((result.unwrap() - 0.224).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_stat_scaling_offset_clamps_negative() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::EnergyRecharge,
            offset: Some(1.0),
            cap: None,
        };
        let profile = StatProfile {
            energy_recharge: 0.80, // below offset
            ..Default::default()
        };
        // (0.80 - 1.0).max(0.0) * 0.28 = 0.0
        let result = eval_auto(&cond, 0.28, &profile, WeaponType::Polearm, Element::Electro, &[], 1);
        assert!((result.unwrap() - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_stat_scaling_refinement_cap() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::EnergyRecharge,
            offset: Some(1.0),
            cap: Some([0.80, 0.90, 1.00, 1.10, 1.20]),
        };
        let profile = StatProfile {
            energy_recharge: 5.00, // very high ER
            ..Default::default()
        };
        // (5.00 - 1.0) * 0.28 = 1.12 → capped at 0.80 (R1, refinement=0)
        let result = eval_auto(&cond, 0.28, &profile, WeaponType::Polearm, Element::Electro, &[], 1);
        assert!((result.unwrap() - 0.80).abs() < EPSILON);

        // R5 (refinement=5, 1-based): capped at 1.20
        let result_r5 = eval_auto(&cond, 0.56, &profile, WeaponType::Polearm, Element::Electro, &[], 5);
        // (5.00 - 1.0) * 0.56 = 2.24 → capped at 1.20
        assert!((result_r5.unwrap() - 1.20).abs() < EPSILON);
    }
```

- [ ] **Step 2: Update `AutoCondition::StatScaling` struct**

In `crates/data/src/buff.rs`, change:

```rust
    StatScaling {
        stat: BuffableStat,
        cap: Option<f64>,
    },
```

to:

```rust
    /// Buff value computed from a stat. Multiplier comes from ConditionalBuff.value.
    /// Final = ((stat_value - offset).max(0.0)) * multiplier, capped if set.
    /// The BuffableStat here indicates which "stat family" to read the total value from:
    /// - HpPercent → total HP (base_hp * (1 + hp_percent) + hp_flat)
    /// - AtkPercent → total ATK (base_atk * (1 + atk_percent) + atk_flat)
    /// - DefPercent → total DEF (base_def * (1 + def_percent) + def_flat)
    /// - DefPercentRaw → raw def_percent value (for "DEF increase" scaling)
    /// - ElementalMastery → elemental_mastery
    /// - EnergyRecharge → energy_recharge
    StatScaling {
        stat: BuffableStat,
        /// Subtracted from stat value before scaling. E.g. offset=1.0 for ER excess.
        offset: Option<f64>,
        /// Per-refinement cap values [R1..R5]. None = no cap.
        cap: Option<[f64; 5]>,
    },
```

- [ ] **Step 3: Update `eval_auto` signature and implementation**

Change `eval_auto` signature to add `refinement: usize`:

```rust
fn eval_auto(
    cond: &AutoCondition,
    multiplier: f64,
    profile: &StatProfile,
    weapon_type: WeaponType,
    element: Element,
    team_elements: &[Element],
    refinement: u8,  // NEW: 1-based refinement level (1=R1, 5=R5), matching resolve_value convention
) -> Option<f64> {
```

Update the `StatScaling` match arm:

```rust
        AutoCondition::StatScaling { stat, offset, cap } => {
            let stat_val = read_stat_for_scaling(stat, profile);
            let effective = (stat_val - offset.unwrap_or(0.0)).max(0.0);
            let raw = effective * multiplier;
            let idx = refinement.saturating_sub(1).min(4) as usize;
            Some(cap.map_or(raw, |caps| raw.min(caps[idx])))
        }
```

- [ ] **Step 4: Update all `eval_auto` call sites**

In `team_builder.rs`, the two call sites (lines ~280 and ~291) need the refinement parameter. The refinement index should come from the weapon's refinement level. Find the `refinement` variable in the build method scope and pass it:

```rust
// At the call sites, pass `refinement` (already available as a local variable in build())
Activation::Auto(auto) => eval_auto(
    auto, effective_value, &profile,
    char_data.weapon_type, char_data.element, &self.team_elements,
    refinement,  // ADD THIS
),
// ... and the Both arm similarly
```

Note: Check how `refinement` is computed in the builder. It should be `self.refinement.unwrap_or(0)` converted to a 0-based index.

- [ ] **Step 5: Update all existing `StatScaling` data sites**

Add `offset: None` to all existing `StatScaling` constructions. Change `cap: Some(val)` to `cap: Some([val; 5])` for artifact sites:

**Weapons (3 sites — all `cap: None`):**
- `sword.rs:248`: Add `offset: None,` before `cap: None,`
- `polearm.rs:184`: Add `offset: None,` before `cap: None,`
- `polearm.rs:198`: Add `offset: None,` before `cap: None,`

**Artifacts (3 sites):**
- `artifacts.rs:667`: Change to `offset: None, cap: Some([0.75; 5]),`
- `artifacts.rs:1802`: Change to `offset: None, cap: Some([0.80; 5]),`
- `artifacts.rs:1815`: Change to `offset: None, cap: Some([0.80; 5]),`

**Tests (4 sites in team_builder.rs):**
- Line ~1007: Change to `offset: None, cap: Some([0.75; 5]),` and add `, 0` to eval_auto call
- Line ~1022: Same
- Line ~1037: Change to `offset: None, cap: None,` and add `, 0` to eval_auto call
- Line ~1254: Same

Update ALL other `eval_auto` calls in tests to add the `, 0` refinement parameter.

- [ ] **Step 6: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All PASS

- [ ] **Step 7: Commit**

```bash
git add crates/data/src/buff.rs crates/data/src/team_builder.rs crates/data/src/weapons/ crates/data/src/artifacts.rs
git commit -m "feat(data): extend StatScaling with offset and per-refinement cap"
```

---

### Task 4: Add `DefPercentRaw` to `read_stat_for_scaling`

**Files:**
- Modify: `crates/data/src/team_builder.rs`

- [ ] **Step 1: Write failing test**

```rust
    #[test]
    fn test_eval_auto_stat_scaling_def_percent_raw() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::DefPercentRaw,
            offset: None,
            cap: None,
        };
        let profile = StatProfile {
            base_def: 800.0,
            def_percent: 0.50,
            ..Default::default()
        };
        // DefPercentRaw reads def_percent directly: 0.50 * 0.18 = 0.09
        let result = eval_auto(&cond, 0.18, &profile, WeaponType::Sword, Element::Geo, &[], 1);
        assert!((result.unwrap() - 0.09).abs() < EPSILON);
    }
```

- [ ] **Step 2: Add `DefPercentRaw` branch**

In `read_stat_for_scaling`:

```rust
fn read_stat_for_scaling(stat: &BuffableStat, profile: &StatProfile) -> f64 {
    match stat {
        BuffableStat::HpPercent => profile.base_hp * (1.0 + profile.hp_percent) + profile.hp_flat,
        BuffableStat::AtkPercent => profile.base_atk * (1.0 + profile.atk_percent) + profile.atk_flat,
        BuffableStat::DefPercent => profile.base_def * (1.0 + profile.def_percent) + profile.def_flat,
        BuffableStat::DefPercentRaw => profile.def_percent,  // NEW
        BuffableStat::ElementalMastery => profile.elemental_mastery,
        BuffableStat::EnergyRecharge => profile.energy_recharge,
        _ => 0.0,
    }
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All PASS

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "feat(data): add DefPercentRaw stat reading for Uraku Misugiri scaling"
```

---

### Task 5: Add Polearm weapons (Engulfing Lightning + Staff of Scarlet Sands)

**Files:**
- Modify: `crates/data/src/weapons/polearm.rs`

- [ ] **Step 1: Write failing tests**

Add to data crate tests (e.g., in `crates/data/tests/core_integration.rs` or a new test file):

```rust
#[test]
fn test_engulfing_lightning_conditional() {
    let weapon = genshin_calc_data::find_weapon("engulfing_lightning").unwrap();
    let passive = weapon.passive.unwrap();
    let cond = &passive.effect.conditional_buffs;
    assert!(!cond.is_empty());
    let scaling = &cond[0];
    assert_eq!(scaling.stat, BuffableStat::AtkPercent);
    assert!((scaling.value - 0.28).abs() < 1e-6);
}

#[test]
fn test_staff_of_scarlet_sands_conditional() {
    let weapon = genshin_calc_data::find_weapon("staff_of_the_scarlet_sands").unwrap();
    let passive = weapon.passive.unwrap();
    let cond = &passive.effect.conditional_buffs;
    assert!(!cond.is_empty());
    let scaling = &cond[0];
    assert_eq!(scaling.stat, BuffableStat::AtkFlat);
    assert!((scaling.value - 0.52).abs() < 1e-6);
}
```

- [ ] **Step 2: Add Engulfing Lightning ConditionalBuff**

In `polearm.rs`, find `ENGULFING_LIGHTNING` and replace `conditional_buffs: &[]` with:

```rust
            conditional_buffs: &[ConditionalBuff {
                name: "engulfing_er_atk",
                description: "ER超過分の28-56%をATK%に変換 (cap: 80-120%)",
                stat: BuffableStat::AtkPercent,
                value: 0.28,
                refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::EnergyRecharge,
                    offset: Some(1.0),
                    cap: Some([0.80, 0.90, 1.00, 1.10, 1.20]),
                }),
            }],
```

- [ ] **Step 3: Add Staff of Scarlet Sands ConditionalBuff**

In `polearm.rs`, find `STAFF_OF_THE_SCARLET_SANDS` and replace `conditional_buffs: &[]` with:

```rust
            conditional_buffs: &[ConditionalBuff {
                name: "scarlet_sands_em_atk",
                description: "EM×52-104%分をATKフラットに加算",
                stat: BuffableStat::AtkFlat,
                value: 0.52,
                refinement_values: Some([0.52, 0.65, 0.78, 0.91, 1.04]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::ElementalMastery,
                    offset: None,
                    cap: None,
                }),
            }],
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/polearm.rs crates/data/tests/
git commit -m "feat(data): add Engulfing Lightning and Staff of Scarlet Sands StatScaling"
```

---

### Task 6: Add Sword weapons (Key, Light of Foliar, Peak Patrol, Uraku)

**Files:**
- Modify: `crates/data/src/weapons/sword.rs`

- [ ] **Step 1: Write failing tests for all 4 swords**

- [ ] **Step 2: Add Key of Khaj-Nisut ConditionalBuff**

Replace `conditional_buffs: &[]` with:

```rust
            conditional_buffs: &[ConditionalBuff {
                name: "khaj_nisut_hp_em",
                description: "HP上限×0.12-0.24%分をEMに加算",
                stat: BuffableStat::ElementalMastery,
                value: 0.0012,
                refinement_values: Some([0.0012, 0.0015, 0.0018, 0.0021, 0.0024]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::HpPercent,
                    offset: None,
                    cap: None,
                }),
            }],
```

- [ ] **Step 3: Add Light of Foliar Incision ConditionalBuffs (2 entries)**

```rust
            conditional_buffs: &[
                ConditionalBuff {
                    name: "foliar_em_normal_flat",
                    description: "EM×120-240%分を通常攻撃フラットダメージに加算",
                    stat: BuffableStat::NormalAtkFlatDmg,
                    value: 1.20,
                    refinement_values: Some([1.20, 1.50, 1.80, 2.10, 2.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::ElementalMastery,
                        offset: None,
                        cap: None,
                    }),
                },
                ConditionalBuff {
                    name: "foliar_em_skill_flat",
                    description: "EM×120-240%分をスキルフラットダメージに加算",
                    stat: BuffableStat::SkillFlatDmg,
                    value: 1.20,
                    refinement_values: Some([1.20, 1.50, 1.80, 2.10, 2.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::ElementalMastery,
                        offset: None,
                        cap: None,
                    }),
                },
            ],
```

- [ ] **Step 4: Add Peak Patrol Song ConditionalBuff**

```rust
            conditional_buffs: &[ConditionalBuff {
                name: "peak_patrol_def_dmg",
                description: "DEF×8-16%分を元素DMGボーナスに加算",
                stat: BuffableStat::DmgBonus,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::DefPercent,
                    offset: None,
                    cap: None,
                }),
            }],
```

- [ ] **Step 5: Add Uraku Misugiri ConditionalBuff**

```rust
            conditional_buffs: &[ConditionalBuff {
                name: "uraku_def_skill",
                description: "DEF増加分×18-36%分をスキルDMGボーナスに加算",
                stat: BuffableStat::SkillDmgBonus,
                value: 0.18,
                refinement_values: Some([0.18, 0.225, 0.27, 0.315, 0.36]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::DefPercentRaw,
                    offset: None,
                    cap: None,
                }),
            }],
```

- [ ] **Step 6: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All PASS

- [ ] **Step 7: Commit**

```bash
git add crates/data/src/weapons/sword.rs crates/data/tests/
git commit -m "feat(data): add StatScaling for Key, Light of Foliar, Peak Patrol, Uraku"
```

---

### Task 7: Add Bow + Claymore weapons (Hunter's Path + Redhorn)

**Files:**
- Modify: `crates/data/src/weapons/bow.rs`
- Modify: `crates/data/src/weapons/claymore.rs`

- [ ] **Step 1: Write failing tests**

- [ ] **Step 2: Add Hunter's Path ConditionalBuff**

In `bow.rs`, find `HUNTERS_PATH` and replace `conditional_buffs: &[]`:

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

- [ ] **Step 3: Add Redhorn Stonethresher ConditionalBuffs (2 entries)**

In `claymore.rs`, find `REDHORN_STONETHRESHER` and replace `conditional_buffs: &[]`:

```rust
            conditional_buffs: &[
                ConditionalBuff {
                    name: "redhorn_def_normal_flat",
                    description: "DEF×40-80%分を通常攻撃フラットダメージに加算",
                    stat: BuffableStat::NormalAtkFlatDmg,
                    value: 0.40,
                    refinement_values: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::DefPercent,
                        offset: None,
                        cap: None,
                    }),
                },
                ConditionalBuff {
                    name: "redhorn_def_charged_flat",
                    description: "DEF×40-80%分を重撃フラットダメージに加算",
                    stat: BuffableStat::ChargedAtkFlatDmg,
                    value: 0.40,
                    refinement_values: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::DefPercent,
                        offset: None,
                        cap: None,
                    }),
                },
            ],
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/weapons/bow.rs crates/data/src/weapons/claymore.rs crates/data/tests/
git commit -m "feat(data): add Hunter's Path and Redhorn Stonethresher StatScaling"
```

---

### Task 8: Integration tests

**Files:**
- Modify: `crates/data/tests/core_integration.rs` or new test file

- [ ] **Step 1: Engulfing Lightning integration test**

```rust
#[test]
fn test_engulfing_lightning_er_scaling() {
    use genshin_calc_data::*;
    use genshin_calc_core::*;

    let char = find_character("raiden_shogun").unwrap();
    let weapon = find_weapon("engulfing_lightning").unwrap();
    let member = TeamMemberBuilder::new(char, weapon)
        .build()
        .unwrap();

    // Check that a resolved buff with AtkPercent StatScaling exists
    let has_atk_scaling = member.buffs_provided.iter().any(|b| {
        b.stat == BuffableStat::AtkPercent
    });
    assert!(has_atk_scaling, "Engulfing Lightning should provide AtkPercent scaling buff");
}
```

- [ ] **Step 2: Redhorn flat damage integration test**

```rust
#[test]
fn test_redhorn_def_flat_damage() {
    use genshin_calc_data::*;
    use genshin_calc_core::*;

    let char = find_character("itto").unwrap();
    let weapon = find_weapon("redhorn_stonethresher").unwrap();
    let member = TeamMemberBuilder::new(char, weapon)
        .build()
        .unwrap();

    // Check NormalAtkFlatDmg and ChargedAtkFlatDmg exist
    let has_normal_flat = member.buffs_provided.iter().any(|b| {
        b.stat == BuffableStat::NormalAtkFlatDmg
    });
    let has_charged_flat = member.buffs_provided.iter().any(|b| {
        b.stat == BuffableStat::ChargedAtkFlatDmg
    });
    assert!(has_normal_flat, "Redhorn should provide NormalAtkFlatDmg");
    assert!(has_charged_flat, "Redhorn should provide ChargedAtkFlatDmg");
}
```

- [ ] **Step 3: Run all tests**

Run: `cargo test && cargo clippy -- -D warnings && cargo fmt --check`
Expected: All PASS, no warnings, no formatting issues

- [ ] **Step 4: Commit**

```bash
git add crates/data/tests/
git commit -m "test(data): add integration tests for StatScaling weapon passives"
```

---

### Task 9: Update docs

**Files:**
- Modify: `docs/data-expansion-todo.md`
- Modify: `CLAUDE.md`

- [ ] **Step 1: Update data-expansion-todo.md**

Update the 現状サマリ table weapon ConditionalBuff count (12 → 22 including new 10 conditional entries for 8 weapons).

Update P3 section to note Batch 1 completion.

- [ ] **Step 2: Update CLAUDE.md**

Update test counts after running:
```bash
cargo test -p genshin-calc-core -- --list 2>/dev/null | grep -c ": test$"
cargo test -p genshin-calc-data -- --list 2>/dev/null | grep -c ": test$"
```

- [ ] **Step 3: Run clippy and fmt**

Run: `cargo clippy -- -D warnings && cargo fmt --check`
Expected: Clean

- [ ] **Step 4: Commit**

```bash
git add docs/data-expansion-todo.md CLAUDE.md
git commit -m "docs: update P3 Batch 1 completion status and test counts"
```
