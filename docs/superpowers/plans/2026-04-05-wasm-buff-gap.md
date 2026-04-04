# WASM Buff Gap Resolution Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Surface attack-type DMG bonuses, flat DMG, enemy debuffs, and reaction bonuses from team buffs through `TeamResolveResult` and the WASM API.

**Architecture:** Extend `TeamResolveResult` with two new structs (`DamageContext`, `EnemyDebuffs`) populated from a single pass over `applied_buffs`. Change WASM `resolve_team_stats` to return the full `TeamResolveResult`. Add WASM `apply_team_debuffs` helper function.

**Tech Stack:** Rust, wasm-bindgen, serde, serde_wasm_bindgen

**Spec:** `docs/superpowers/specs/2026-04-05-wasm-buff-gap-design.md`

---

## File Map

| Action | File | Responsibility |
|--------|------|----------------|
| Modify | `crates/core/src/team.rs` | Add `DamageContext` struct + `from_buffs`, extend `TeamResolveResult`, change `resolve_team_stats` return type |
| Modify | `crates/core/src/enemy.rs` | Add `EnemyDebuffs` struct + `collect_enemy_debuffs` + `apply_debuffs_to_enemy` |
| Modify | `crates/core/src/lib.rs` | Update public re-exports |
| Modify | `crates/wasm/src/lib.rs` | Change `resolve_team_stats` to return `TeamResolveResult`, add `apply_team_debuffs` |
| Modify | existing tests in `team.rs`, `enemy.rs`, `lib.rs`, `wasm/lib.rs` | Update for new return type |

---

### Task 1: Add `DamageContext` struct and `from_buffs`

**Files:**
- Modify: `crates/core/src/team.rs`

- [ ] **Step 1: Write the failing test for `DamageContext::from_buffs`**

Add to the `#[cfg(test)] mod tests` block in `crates/core/src/team.rs`:

```rust
#[test]
fn test_damage_context_from_buffs_empty() {
    let ctx = DamageContext::from_buffs(&[]);
    assert!((ctx.normal_atk_dmg_bonus - 0.0).abs() < EPSILON);
    assert!((ctx.skill_flat_dmg - 0.0).abs() < EPSILON);
    assert!((ctx.amplifying_bonus - 0.0).abs() < EPSILON);
}

#[test]
fn test_damage_context_from_buffs_mixed() {
    let buffs = vec![
        ResolvedBuff {
            source: "Yelan A4".into(),
            stat: BuffableStat::NormalAtkDmgBonus,
            value: 0.25,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "Shenhe E".into(),
            stat: BuffableStat::SkillFlatDmg,
            value: 3000.0,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "Shenhe E".into(),
            stat: BuffableStat::BurstFlatDmg,
            value: 3000.0,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "Bennett Burst".into(),
            stat: BuffableStat::AtkFlat,
            value: 1000.0,
            target: BuffTarget::Team,
        },
    ];
    let ctx = DamageContext::from_buffs(&buffs);
    assert!((ctx.normal_atk_dmg_bonus - 0.25).abs() < EPSILON);
    assert!((ctx.charged_atk_dmg_bonus - 0.0).abs() < EPSILON);
    assert!((ctx.skill_flat_dmg - 3000.0).abs() < EPSILON);
    assert!((ctx.burst_flat_dmg - 3000.0).abs() < EPSILON);
    assert!((ctx.normal_atk_flat_dmg - 0.0).abs() < EPSILON);
    // AtkFlat is unconditional — should NOT appear in DamageContext
    assert!((ctx.amplifying_bonus - 0.0).abs() < EPSILON);
}

#[test]
fn test_damage_context_reaction_bonuses() {
    let buffs = vec![
        ResolvedBuff {
            source: "4pc CW".into(),
            stat: BuffableStat::AmplifyingBonus,
            value: 0.15,
            target: BuffTarget::OnlySelf,
        },
        ResolvedBuff {
            source: "Sucrose C6".into(),
            stat: BuffableStat::AdditiveBonus,
            value: 0.20,
            target: BuffTarget::Team,
        },
    ];
    let ctx = DamageContext::from_buffs(&buffs);
    assert!((ctx.amplifying_bonus - 0.15).abs() < EPSILON);
    assert!((ctx.additive_bonus - 0.20).abs() < EPSILON);
    assert!((ctx.transformative_bonus - 0.0).abs() < EPSILON);
}

#[test]
fn test_damage_context_all_type_dmg_bonuses() {
    let buffs = vec![
        ResolvedBuff {
            source: "a".into(),
            stat: BuffableStat::NormalAtkDmgBonus,
            value: 0.10,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "b".into(),
            stat: BuffableStat::ChargedAtkDmgBonus,
            value: 0.20,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "c".into(),
            stat: BuffableStat::PlungingAtkDmgBonus,
            value: 0.30,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "d".into(),
            stat: BuffableStat::SkillDmgBonus,
            value: 0.40,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "e".into(),
            stat: BuffableStat::BurstDmgBonus,
            value: 0.50,
            target: BuffTarget::Team,
        },
    ];
    let ctx = DamageContext::from_buffs(&buffs);
    assert!((ctx.normal_atk_dmg_bonus - 0.10).abs() < EPSILON);
    assert!((ctx.charged_atk_dmg_bonus - 0.20).abs() < EPSILON);
    assert!((ctx.plunging_atk_dmg_bonus - 0.30).abs() < EPSILON);
    assert!((ctx.skill_dmg_bonus - 0.40).abs() < EPSILON);
    assert!((ctx.burst_dmg_bonus - 0.50).abs() < EPSILON);
}

#[test]
fn test_damage_context_all_type_flat_dmgs() {
    let buffs = vec![
        ResolvedBuff {
            source: "a".into(),
            stat: BuffableStat::NormalAtkFlatDmg,
            value: 100.0,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "b".into(),
            stat: BuffableStat::ChargedAtkFlatDmg,
            value: 200.0,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "c".into(),
            stat: BuffableStat::PlungingAtkFlatDmg,
            value: 300.0,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "d".into(),
            stat: BuffableStat::SkillFlatDmg,
            value: 400.0,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "e".into(),
            stat: BuffableStat::BurstFlatDmg,
            value: 500.0,
            target: BuffTarget::Team,
        },
    ];
    let ctx = DamageContext::from_buffs(&buffs);
    assert!((ctx.normal_atk_flat_dmg - 100.0).abs() < EPSILON);
    assert!((ctx.charged_atk_flat_dmg - 200.0).abs() < EPSILON);
    assert!((ctx.plunging_atk_flat_dmg - 300.0).abs() < EPSILON);
    assert!((ctx.skill_flat_dmg - 400.0).abs() < EPSILON);
    assert!((ctx.burst_flat_dmg - 500.0).abs() < EPSILON);
}

#[test]
fn test_damage_context_stacks_same_type() {
    let buffs = vec![
        ResolvedBuff {
            source: "Freedom-Sworn".into(),
            stat: BuffableStat::NormalAtkDmgBonus,
            value: 0.16,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "Yun Jin A4".into(),
            stat: BuffableStat::NormalAtkDmgBonus,
            value: 0.05,
            target: BuffTarget::Team,
        },
    ];
    let ctx = DamageContext::from_buffs(&buffs);
    assert!((ctx.normal_atk_dmg_bonus - 0.21).abs() < EPSILON);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core --lib team::tests::test_damage_context -- 2>&1 | head -30`
Expected: compilation error — `DamageContext` not defined

- [ ] **Step 3: Implement `DamageContext` struct and `from_buffs`**

Add before the `TeamResolveResult` struct in `crates/core/src/team.rs`:

```rust
/// Aggregated attack-type-specific DMG bonuses, flat DMG, and reaction bonuses
/// from team buffs. These cannot be applied to `StatProfile`/`Stats` because they
/// depend on `DamageType` or reaction context.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DamageContext {
    /// Normal attack DMG bonus from team buffs.
    pub normal_atk_dmg_bonus: f64,
    /// Charged attack DMG bonus from team buffs.
    pub charged_atk_dmg_bonus: f64,
    /// Plunging attack DMG bonus from team buffs.
    pub plunging_atk_dmg_bonus: f64,
    /// Elemental skill DMG bonus from team buffs.
    pub skill_dmg_bonus: f64,
    /// Elemental burst DMG bonus from team buffs.
    pub burst_dmg_bonus: f64,
    /// Flat damage added to normal attacks from team buffs.
    pub normal_atk_flat_dmg: f64,
    /// Flat damage added to charged attacks from team buffs.
    pub charged_atk_flat_dmg: f64,
    /// Flat damage added to plunging attacks from team buffs.
    pub plunging_atk_flat_dmg: f64,
    /// Flat damage added to elemental skill from team buffs.
    pub skill_flat_dmg: f64,
    /// Flat damage added to elemental burst from team buffs.
    pub burst_flat_dmg: f64,
    /// Amplifying reaction (vaporize/melt) DMG bonus from team buffs.
    pub amplifying_bonus: f64,
    /// Transformative reaction DMG bonus from team buffs.
    pub transformative_bonus: f64,
    /// Additive (catalyze) reaction DMG bonus from team buffs.
    pub additive_bonus: f64,
}

impl DamageContext {
    /// Aggregates conditional buffs from resolved buff list into a `DamageContext`.
    ///
    /// Only extracts attack-type DMG bonuses, flat DMG, and reaction bonuses.
    /// Unconditional stat buffs (ATK%, CRIT, etc.) are ignored — those are
    /// already applied via `apply_buffs_to_profile`.
    pub fn from_buffs(buffs: &[ResolvedBuff]) -> Self {
        let mut ctx = Self::default();
        for buff in buffs {
            match buff.stat {
                BuffableStat::NormalAtkDmgBonus => ctx.normal_atk_dmg_bonus += buff.value,
                BuffableStat::ChargedAtkDmgBonus => ctx.charged_atk_dmg_bonus += buff.value,
                BuffableStat::PlungingAtkDmgBonus => ctx.plunging_atk_dmg_bonus += buff.value,
                BuffableStat::SkillDmgBonus => ctx.skill_dmg_bonus += buff.value,
                BuffableStat::BurstDmgBonus => ctx.burst_dmg_bonus += buff.value,
                BuffableStat::NormalAtkFlatDmg => ctx.normal_atk_flat_dmg += buff.value,
                BuffableStat::ChargedAtkFlatDmg => ctx.charged_atk_flat_dmg += buff.value,
                BuffableStat::PlungingAtkFlatDmg => ctx.plunging_atk_flat_dmg += buff.value,
                BuffableStat::SkillFlatDmg => ctx.skill_flat_dmg += buff.value,
                BuffableStat::BurstFlatDmg => ctx.burst_flat_dmg += buff.value,
                BuffableStat::AmplifyingBonus => ctx.amplifying_bonus += buff.value,
                BuffableStat::TransformativeBonus => ctx.transformative_bonus += buff.value,
                BuffableStat::AdditiveBonus => ctx.additive_bonus += buff.value,
                _ => {}
            }
        }
        ctx
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-core --lib team::tests::test_damage_context`
Expected: all 6 tests PASS

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/team.rs
git commit -m "feat(core): add DamageContext struct with from_buffs aggregation"
```

---

### Task 2: Add `EnemyDebuffs` struct, `collect_enemy_debuffs`, and `apply_debuffs_to_enemy`

**Files:**
- Modify: `crates/core/src/enemy.rs`

- [ ] **Step 1: Write the failing tests**

Add to `#[cfg(test)] mod tests` in `crates/core/src/enemy.rs`:

```rust
#[test]
fn test_collect_enemy_debuffs_empty() {
    let debuffs = collect_enemy_debuffs(&[]);
    assert!((debuffs.pyro_res_reduction - 0.0).abs() < EPSILON);
    assert!((debuffs.def_reduction - 0.0).abs() < EPSILON);
}

#[test]
fn test_collect_enemy_debuffs_all_elements() {
    let buffs = vec![
        ResolvedBuff {
            source: "Citlali Q".into(),
            stat: BuffableStat::ElementalResReduction(Element::Pyro),
            value: 0.20,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "Citlali Q".into(),
            stat: BuffableStat::ElementalResReduction(Element::Cryo),
            value: 0.20,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "VV".into(),
            stat: BuffableStat::ElementalResReduction(Element::Hydro),
            value: 0.40,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "Superconduct".into(),
            stat: BuffableStat::PhysicalResReduction,
            value: 0.40,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "Lisa A4".into(),
            stat: BuffableStat::DefReduction,
            value: 0.15,
            target: BuffTarget::Team,
        },
        // This should be ignored (unconditional stat buff)
        ResolvedBuff {
            source: "Bennett".into(),
            stat: BuffableStat::AtkFlat,
            value: 1000.0,
            target: BuffTarget::Team,
        },
    ];
    let debuffs = collect_enemy_debuffs(&buffs);
    assert!((debuffs.pyro_res_reduction - 0.20).abs() < EPSILON);
    assert!((debuffs.cryo_res_reduction - 0.20).abs() < EPSILON);
    assert!((debuffs.hydro_res_reduction - 0.40).abs() < EPSILON);
    assert!((debuffs.electro_res_reduction - 0.0).abs() < EPSILON);
    assert!((debuffs.dendro_res_reduction - 0.0).abs() < EPSILON);
    assert!((debuffs.anemo_res_reduction - 0.0).abs() < EPSILON);
    assert!((debuffs.geo_res_reduction - 0.0).abs() < EPSILON);
    assert!((debuffs.physical_res_reduction - 0.40).abs() < EPSILON);
    assert!((debuffs.def_reduction - 0.15).abs() < EPSILON);
}

#[test]
fn test_collect_enemy_debuffs_stacks() {
    let buffs = vec![
        ResolvedBuff {
            source: "VV".into(),
            stat: BuffableStat::ElementalResReduction(Element::Pyro),
            value: 0.40,
            target: BuffTarget::Team,
        },
        ResolvedBuff {
            source: "Zhongli".into(),
            stat: BuffableStat::ElementalResReduction(Element::Pyro),
            value: 0.20,
            target: BuffTarget::Team,
        },
    ];
    let debuffs = collect_enemy_debuffs(&buffs);
    assert!((debuffs.pyro_res_reduction - 0.60).abs() < EPSILON);
}

#[test]
fn test_apply_debuffs_to_enemy_pyro() {
    let enemy = base_enemy(); // level: 90, resistance: 0.10, def_reduction: 0.0
    let debuffs = EnemyDebuffs {
        pyro_res_reduction: 0.40,
        ..Default::default()
    };
    let result = apply_debuffs_to_enemy(&enemy, &debuffs, Some(Element::Pyro));
    assert!((result.resistance - (-0.30)).abs() < EPSILON);
    assert!((result.def_reduction - 0.0).abs() < EPSILON);
}

#[test]
fn test_apply_debuffs_to_enemy_physical() {
    let enemy = base_enemy();
    let debuffs = EnemyDebuffs {
        physical_res_reduction: 0.40,
        ..Default::default()
    };
    let result = apply_debuffs_to_enemy(&enemy, &debuffs, None);
    assert!((result.resistance - (-0.30)).abs() < EPSILON);
}

#[test]
fn test_apply_debuffs_to_enemy_wrong_element_no_effect() {
    let enemy = base_enemy();
    let debuffs = EnemyDebuffs {
        pyro_res_reduction: 0.40,
        ..Default::default()
    };
    let result = apply_debuffs_to_enemy(&enemy, &debuffs, Some(Element::Cryo));
    assert!((result.resistance - 0.10).abs() < EPSILON);
}

#[test]
fn test_apply_debuffs_to_enemy_def_reduction_clamped() {
    let enemy = Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.80,
    };
    let debuffs = EnemyDebuffs {
        def_reduction: 0.50,
        ..Default::default()
    };
    let result = apply_debuffs_to_enemy(&enemy, &debuffs, Some(Element::Pyro));
    assert!((result.def_reduction - 1.0).abs() < EPSILON);
}

#[test]
fn test_apply_debuffs_to_enemy_combined() {
    let enemy = base_enemy();
    let debuffs = EnemyDebuffs {
        pyro_res_reduction: 0.40,
        cryo_res_reduction: 0.20,
        def_reduction: 0.15,
        ..Default::default()
    };
    let result = apply_debuffs_to_enemy(&enemy, &debuffs, Some(Element::Pyro));
    assert!((result.resistance - (-0.30)).abs() < EPSILON);
    assert!((result.def_reduction - 0.15).abs() < EPSILON);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core --lib enemy::tests::test_collect_enemy_debuffs -- 2>&1 | head -10`
Expected: compilation error — `EnemyDebuffs`, `collect_enemy_debuffs`, `apply_debuffs_to_enemy` not defined

- [ ] **Step 3: Implement `EnemyDebuffs`, `collect_enemy_debuffs`, and `apply_debuffs_to_enemy`**

Add after the `Enemy` struct in `crates/core/src/enemy.rs`:

```rust
/// Aggregated enemy debuffs from team buffs.
///
/// Each field represents the total resistance reduction or DEF reduction
/// collected from all team members' buffs. Use [`apply_debuffs_to_enemy`]
/// to apply these to a base [`Enemy`].
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnemyDebuffs {
    /// Pyro resistance reduction total.
    pub pyro_res_reduction: f64,
    /// Hydro resistance reduction total.
    pub hydro_res_reduction: f64,
    /// Electro resistance reduction total.
    pub electro_res_reduction: f64,
    /// Cryo resistance reduction total.
    pub cryo_res_reduction: f64,
    /// Dendro resistance reduction total.
    pub dendro_res_reduction: f64,
    /// Anemo resistance reduction total.
    pub anemo_res_reduction: f64,
    /// Geo resistance reduction total.
    pub geo_res_reduction: f64,
    /// Physical resistance reduction total.
    pub physical_res_reduction: f64,
    /// DEF reduction total.
    pub def_reduction: f64,
}

/// Collects enemy debuffs from resolved team buffs into an [`EnemyDebuffs`].
///
/// Extracts `ElementalResReduction`, `PhysicalResReduction`, and `DefReduction`
/// buffs. All other buff types are ignored.
pub fn collect_enemy_debuffs(buffs: &[ResolvedBuff]) -> EnemyDebuffs {
    let mut d = EnemyDebuffs::default();
    for buff in buffs {
        match buff.stat {
            BuffableStat::ElementalResReduction(Element::Pyro) => d.pyro_res_reduction += buff.value,
            BuffableStat::ElementalResReduction(Element::Hydro) => d.hydro_res_reduction += buff.value,
            BuffableStat::ElementalResReduction(Element::Electro) => d.electro_res_reduction += buff.value,
            BuffableStat::ElementalResReduction(Element::Cryo) => d.cryo_res_reduction += buff.value,
            BuffableStat::ElementalResReduction(Element::Dendro) => d.dendro_res_reduction += buff.value,
            BuffableStat::ElementalResReduction(Element::Anemo) => d.anemo_res_reduction += buff.value,
            BuffableStat::ElementalResReduction(Element::Geo) => d.geo_res_reduction += buff.value,
            BuffableStat::PhysicalResReduction => d.physical_res_reduction += buff.value,
            BuffableStat::DefReduction => d.def_reduction += buff.value,
            _ => {}
        }
    }
    d
}

/// Applies pre-collected enemy debuffs to a base enemy for a specific damage element.
///
/// Selects the matching resistance reduction by element (or physical if `None`),
/// subtracts from `enemy.resistance`, and adds `def_reduction` (clamped to 1.0).
/// Returns a new `Enemy` (immutable pattern).
pub fn apply_debuffs_to_enemy(
    enemy: &Enemy,
    debuffs: &EnemyDebuffs,
    element: Option<Element>,
) -> Enemy {
    let res_reduction = match element {
        Some(Element::Pyro) => debuffs.pyro_res_reduction,
        Some(Element::Hydro) => debuffs.hydro_res_reduction,
        Some(Element::Electro) => debuffs.electro_res_reduction,
        Some(Element::Cryo) => debuffs.cryo_res_reduction,
        Some(Element::Dendro) => debuffs.dendro_res_reduction,
        Some(Element::Anemo) => debuffs.anemo_res_reduction,
        Some(Element::Geo) => debuffs.geo_res_reduction,
        None => debuffs.physical_res_reduction,
    };
    Enemy {
        level: enemy.level,
        resistance: enemy.resistance - res_reduction,
        def_reduction: f64::min(1.0, enemy.def_reduction + debuffs.def_reduction),
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-core --lib enemy::tests`
Expected: all tests PASS (existing + new)

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/enemy.rs
git commit -m "feat(core): add EnemyDebuffs struct with collect and apply functions"
```

---

### Task 3: Extend `TeamResolveResult` and change `resolve_team_stats` return type

**Files:**
- Modify: `crates/core/src/team.rs`

- [ ] **Step 1: Write the failing test**

Add to `#[cfg(test)] mod tests` in `crates/core/src/team.rs`:

```rust
#[test]
fn test_resolve_team_stats_returns_full_result() {
    let mut support = make_member(Element::Cryo, 700.0);
    support.buffs_provided.push(ResolvedBuff {
        source: "Citlali Q".into(),
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
        value: 0.20,
        target: BuffTarget::Team,
    });
    support.buffs_provided.push(ResolvedBuff {
        source: "Freedom-Sworn".into(),
        stat: BuffableStat::NormalAtkDmgBonus,
        value: 0.16,
        target: BuffTarget::Team,
    });
    support.buffs_provided.push(ResolvedBuff {
        source: "Shenhe E".into(),
        stat: BuffableStat::NormalAtkFlatDmg,
        value: 2500.0,
        target: BuffTarget::Team,
    });
    let dps = make_member(Element::Pyro, 900.0);
    let team = vec![support, dps];

    let result = resolve_team_stats(&team, 1).unwrap();
    assert!(result.final_stats.atk > 0.0);
    assert!((result.damage_context.normal_atk_dmg_bonus - 0.16).abs() < EPSILON);
    assert!((result.damage_context.normal_atk_flat_dmg - 2500.0).abs() < EPSILON);
    assert!((result.enemy_debuffs.pyro_res_reduction - 0.20).abs() < EPSILON);
    assert!((result.enemy_debuffs.cryo_res_reduction - 0.0).abs() < EPSILON);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p genshin-calc-core --lib team::tests::test_resolve_team_stats_returns_full_result 2>&1 | head -20`
Expected: FAIL — `resolve_team_stats` returns `Stats`, not `TeamResolveResult`

- [ ] **Step 3: Add `use` for `EnemyDebuffs`, extend `TeamResolveResult`, change `resolve_team_stats`**

In `crates/core/src/team.rs`:

1. Add to the imports at the top:
```rust
use crate::enemy::{EnemyDebuffs, collect_enemy_debuffs};
```

2. Extend `TeamResolveResult`:
```rust
pub struct TeamResolveResult {
    pub base_stats: Stats,
    pub applied_buffs: Vec<ResolvedBuff>,
    pub resonances: Vec<ElementalResonance>,
    pub final_stats: Stats,
    /// Attack-type-specific DMG bonuses, flat DMG, and reaction bonuses.
    pub damage_context: DamageContext,
    /// Enemy resistance and DEF reduction from team debuffs.
    pub enemy_debuffs: EnemyDebuffs,
}
```

3. In `resolve_team_stats_detailed`, add after `let final_stats`:
```rust
let damage_context = DamageContext::from_buffs(&applied_buffs);
let enemy_debuffs = collect_enemy_debuffs(&applied_buffs);
```
And add `damage_context, enemy_debuffs,` to the `TeamResolveResult` construction.

4. Change `resolve_team_stats` return type:
```rust
pub fn resolve_team_stats(team: &[TeamMember], target_index: usize) -> Result<TeamResolveResult, CalcError> {
    resolve_team_stats_detailed(team, target_index)
}
```

- [ ] **Step 4: Fix existing tests and doc examples that call `resolve_team_stats`**

All tests that currently do `resolve_team_stats(&team, idx).unwrap()` expecting `Stats` need to change to `.unwrap().final_stats`. Update these in `crates/core/src/team.rs`:

- `test_single_member_no_buffs`: `let stats = resolve_team_stats(&team, 0).unwrap().final_stats;`
- `test_self_buff_applies_to_self`: both `let stats0 = ...` and `let stats1 = ...`
- `test_team_buff_applies_to_all`: both calls
- `test_team_exclude_self_buff`: both calls
- `test_pyro_resonance_with_4_members`: the call
- `test_no_resonance_with_3_members`: the call

Also update the inline doc example on `resolve_team_stats` function (around line 192-193):
```rust
/// let result = resolve_team_stats(&[member], 0).unwrap();
/// assert!(result.final_stats.atk > 0.0);
```

- [ ] **Step 5: Run all core tests**

Run: `cargo test -p genshin-calc-core`
Expected: PASS (some non-team tests may fail due to lib.rs doc examples — will fix in Task 4)

- [ ] **Step 6: Commit**

```bash
git add crates/core/src/team.rs
git commit -m "feat(core): extend TeamResolveResult with DamageContext and EnemyDebuffs

BREAKING: resolve_team_stats now returns TeamResolveResult instead of Stats"
```

---

### Task 4: Update public re-exports and fix doc examples in `lib.rs`

**Files:**
- Modify: `crates/core/src/lib.rs`

- [ ] **Step 1: Update re-exports in `crates/core/src/lib.rs`**

Add new types to the existing `pub use` lines:

```rust
pub use enemy::{Enemy, EnemyDebuffs, apply_debuffs_to_enemy, apply_enemy_debuffs, collect_enemy_debuffs, superconduct_debuff};
pub use team::{
    BuffTarget, DamageContext, ResolvedBuff, TeamMember, TeamResolveResult,
    apply_buffs_to_profile, resolve_team_stats, resolve_team_stats_detailed,
};
```

- [ ] **Step 2: Update doc examples that use `resolve_team_stats`**

In the module-level doc comment in `lib.rs` (around line 53-55), change:

```rust
//! let stats = resolve_team_stats(&[dps, support], 0).unwrap();
//! assert!(stats.atk > 900.0); // DPS gets Bennett's ATK buff
```

to:

```rust
//! let result = resolve_team_stats(&[dps, support], 0).unwrap();
//! assert!(result.final_stats.atk > 900.0); // DPS gets Bennett's ATK buff
```

- [ ] **Step 3: Update `test_public_api_usage_example` if it uses `resolve_team_stats`**

Check if it does — based on current code it does not, so no change needed.

- [ ] **Step 4: Run full core tests including doc tests**

Run: `cargo test -p genshin-calc-core`
Expected: all PASS

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/lib.rs
git commit -m "feat(core): export DamageContext, EnemyDebuffs, and new enemy functions"
```

---

### Task 5: Fix downstream crates (good, wasm) compilation

**Files:**
- Modify: `crates/good/src/lib.rs` (if it calls `resolve_team_stats`)
- Modify: `crates/wasm/src/lib.rs`

- [ ] **Step 1: Check which downstream files use `resolve_team_stats`**

Run: `grep -rn "resolve_team_stats" crates/good/ crates/wasm/`
Fix each call site to use `.final_stats` where only `Stats` was expected.

- [ ] **Step 2: Fix `crates/wasm/src/lib.rs` — `resolve_team_stats` function**

The WASM `resolve_team_stats` currently calls `genshin_calc_core::resolve_team_stats` and serializes the result. Since the core function now returns `TeamResolveResult`, the WASM function will now serialize the full result automatically — no code change needed for the function body, but the doc comment should be updated to reflect the new return shape.

Update doc comment:

```rust
/// Resolves team buffs and returns detailed result for the target member.
///
/// # Arguments
/// * `members` - Array of TeamMember objects
/// * `target_index` - Index of the DPS/target member (0-based)
///
/// # Returns
/// TeamResolveResult as a JS object with:
/// - `base_stats`: Stats before team buffs
/// - `applied_buffs`: All resolved buffs applied to target
/// - `resonances`: Active elemental resonances
/// - `final_stats`: Stats after all unconditional buffs
/// - `damage_context`: Attack-type DMG bonuses, flat DMG, reaction bonuses
/// - `enemy_debuffs`: Enemy resistance and DEF reduction from team debuffs
```

- [ ] **Step 3: Fix `build_stats` in wasm that uses `resolve_team_stats`**

Around line 250 in `crates/wasm/src/lib.rs`:
```rust
let stats = genshin_calc_core::resolve_team_stats(&[member], 0)
    .map_err(|e| JsError::new(&e.to_string()))?;
```
Change to:
```rust
let result = genshin_calc_core::resolve_team_stats(&[member], 0)
    .map_err(|e| JsError::new(&e.to_string()))?;
let stats = result.final_stats;
```

- [ ] **Step 4: Fix WASM tests that use `resolve_team_stats`**

Update `test_resolve_team_stats_via_core` (around line 452):
```rust
let result = resolve_team_stats(&[dps], 0).unwrap();
assert!(result.final_stats.atk > 0.0);
```

Update `test_build_stats_basic` (around line 493):
```rust
let result = genshin_calc_core::resolve_team_stats(&[member], 0).unwrap();
assert!(result.final_stats.atk > 0.0, "ATK should be positive");
assert!(result.final_stats.hp > 0.0, "HP should be positive");
```

Update `test_build_stats_with_artifact_activation` (around line 524-534): both `stats_no` and `stats_with` calls need `.final_stats`.

- [ ] **Step 5: Fix `crates/good/` if needed**

Check and fix any `resolve_team_stats` usage in good crate.

- [ ] **Step 6: Run full workspace build and tests**

Run: `cargo test`
Expected: all PASS

- [ ] **Step 7: Commit**

```bash
git add crates/wasm/src/lib.rs crates/good/
git commit -m "fix: update downstream crates for TeamResolveResult return type"
```

---

### Task 6: Add WASM `apply_team_debuffs` function

**Files:**
- Modify: `crates/wasm/src/lib.rs`

- [ ] **Step 1: Write the WASM test**

Add to `#[cfg(test)] mod tests` in `crates/wasm/src/lib.rs`:

```rust
#[test]
fn test_apply_debuffs_to_enemy_via_core() {
    use genshin_calc_core::*;

    let enemy = Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
    };
    let debuffs = EnemyDebuffs {
        pyro_res_reduction: 0.40,
        def_reduction: 0.15,
        ..Default::default()
    };
    let result = apply_debuffs_to_enemy(&enemy, &debuffs, Some(Element::Pyro));
    assert!((result.resistance - (-0.30)).abs() < 1e-6);
    assert!((result.def_reduction - 0.15).abs() < 1e-6);
}
```

- [ ] **Step 2: Run test to verify it passes** (requires Task 4 completed — `EnemyDebuffs` and `apply_debuffs_to_enemy` must be re-exported from `genshin_calc_core`)

Run: `cargo test -p genshin-calc-wasm --lib tests::test_apply_debuffs`
Expected: PASS

- [ ] **Step 3: Add `apply_team_debuffs` WASM function**

Add to `crates/wasm/src/lib.rs`:

```rust
/// Applies team enemy debuffs to a base enemy for a specific damage element.
///
/// # Arguments
/// * `enemy` - Base Enemy as a JS object
/// * `debuffs` - EnemyDebuffs from resolve_team_stats result
/// * `element` - Element string in PascalCase ("Pyro", "Hydro", etc.) or null for physical.
///   Uses the same format as DamageInput.element (serde PascalCase).
///
/// # Returns
/// New Enemy with debuffs applied.
#[wasm_bindgen]
pub fn apply_team_debuffs(
    enemy: JsValue,
    debuffs: JsValue,
    element: JsValue,
) -> Result<JsValue, JsError> {
    let enemy: genshin_calc_core::Enemy = serde_wasm_bindgen::from_value(enemy)
        .map_err(|e| JsError::new(&format!("Invalid enemy: {e}")))?;
    let debuffs: genshin_calc_core::EnemyDebuffs = serde_wasm_bindgen::from_value(debuffs)
        .map_err(|e| JsError::new(&format!("Invalid debuffs: {e}")))?;
    let elem: Option<genshin_calc_core::Element> = if element.is_null() || element.is_undefined() {
        None
    } else {
        let e: genshin_calc_core::Element = serde_wasm_bindgen::from_value(element)
            .map_err(|e| JsError::new(&format!("Invalid element: {e}")))?;
        Some(e)
    };
    let result = genshin_calc_core::apply_debuffs_to_enemy(&enemy, &debuffs, elem);
    to_js(&result)
}
```

- [ ] **Step 4: Run all WASM tests**

Run: `cargo test -p genshin-calc-wasm`
Expected: all PASS

- [ ] **Step 5: Commit**

```bash
git add crates/wasm/src/lib.rs
git commit -m "feat(wasm): add apply_team_debuffs function"
```

---

### Task 7: Full pipeline integration test

**Files:**
- Modify: `crates/core/src/enemy.rs` (add integration test)

- [ ] **Step 1: Write full pipeline golden test**

Add to `crates/core/src/enemy.rs` test module:

```rust
#[test]
fn test_full_pipeline_resolve_debuffs_damage() {
    // Scenario: DPS with support providing res shred + flat DMG + ATK buff
    use crate::damage::{DamageInput, calculate_damage};
    use crate::stats::Stats;
    use crate::team::{
        BuffTarget, DamageContext, ResolvedBuff, TeamMember, resolve_team_stats,
    };
    use crate::types::{DamageType, ScalingStat, WeaponType};
    use crate::stat_profile::StatProfile;

    let dps = TeamMember {
        element: Element::Pyro,
        weapon_type: WeaponType::Claymore,
        stats: StatProfile {
            base_atk: 900.0,
            crit_rate: 0.70,
            crit_dmg: 1.50,
            energy_recharge: 1.0,
            ..Default::default()
        },
        buffs_provided: vec![],
        is_moonsign: false,
    };

    let support = TeamMember {
        element: Element::Cryo,
        weapon_type: WeaponType::Catalyst,
        stats: StatProfile {
            base_atk: 600.0,
            energy_recharge: 1.0,
            ..Default::default()
        },
        buffs_provided: vec![
            ResolvedBuff {
                source: "Bennett Burst".into(),
                stat: BuffableStat::AtkFlat,
                value: 1000.0,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "VV Pyro".into(),
                stat: BuffableStat::ElementalResReduction(Element::Pyro),
                value: 0.40,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "Shenhe E".into(),
                stat: BuffableStat::NormalAtkFlatDmg,
                value: 2500.0,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "Freedom-Sworn".into(),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.16,
                target: BuffTarget::Team,
            },
        ],
        is_moonsign: false,
    };

    let team = vec![dps, support];
    let result = resolve_team_stats(&team, 0).unwrap();

    // Verify stats: base_atk 900 + Bennett 1000 = 1900
    assert!((result.final_stats.atk - 1900.0).abs() < EPSILON);

    // Verify damage_context
    assert!((result.damage_context.normal_atk_dmg_bonus - 0.16).abs() < EPSILON);
    assert!((result.damage_context.normal_atk_flat_dmg - 2500.0).abs() < EPSILON);

    // Verify enemy_debuffs
    assert!((result.enemy_debuffs.pyro_res_reduction - 0.40).abs() < EPSILON);

    // Build DamageInput using the resolved data
    let mut stats = result.final_stats.clone();
    stats.dmg_bonus += result.damage_context.normal_atk_dmg_bonus;

    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 1.76,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
        flat_dmg: result.damage_context.normal_atk_flat_dmg,
    };

    let base_enemy = Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
    };
    let debuffed_enemy = apply_debuffs_to_enemy(
        &base_enemy,
        &result.enemy_debuffs,
        Some(Element::Pyro),
    );

    // Verify debuffed enemy: 0.10 - 0.40 = -0.30
    assert!((debuffed_enemy.resistance - (-0.30)).abs() < EPSILON);

    // Calculate damage with and without the full pipeline
    let result_full = calculate_damage(&input, &debuffed_enemy).unwrap();

    // Compare with no-buff baseline
    let baseline_input = DamageInput {
        character_level: 90,
        stats: Stats {
            atk: 900.0,
            crit_rate: 0.70,
            crit_dmg: 1.50,
            ..Default::default()
        },
        talent_multiplier: 1.76,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
        flat_dmg: 0.0,
    };
    let result_baseline = calculate_damage(&baseline_input, &base_enemy).unwrap();

    // Full pipeline should produce significantly more damage
    assert!(result_full.average > result_baseline.average * 2.0);
    assert!(result_full.non_crit > 0.0);
    assert!(result_full.crit > result_full.non_crit);
}
```

- [ ] **Step 2: Run the integration test**

Run: `cargo test -p genshin-calc-core --lib enemy::tests::test_full_pipeline`
Expected: PASS

- [ ] **Step 3: Run full workspace tests and clippy**

Run: `cargo test && cargo clippy -- -D warnings`
Expected: all PASS, no warnings

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/enemy.rs
git commit -m "test(core): add full pipeline integration test for DamageContext + EnemyDebuffs"
```

---

### Task 8: Update version numbers

**Files:**
- Modify: `crates/core/Cargo.toml`
- Modify: `crates/wasm/Cargo.toml`

- [ ] **Step 1: Update core crate version to 0.3.0**

In `crates/core/Cargo.toml`, change version to `"0.3.0"`.

- [ ] **Step 2: Update wasm crate version to 0.3.0**

In `crates/wasm/Cargo.toml`, change version to `"0.3.0"`. Also update core dependency version if pinned.

- [ ] **Step 3: Run cargo build to verify**

Run: `cargo build`
Expected: success

- [ ] **Step 4: Commit**

```bash
git add crates/core/Cargo.toml crates/wasm/Cargo.toml
git commit -m "chore: bump core and wasm crate versions to 0.3.0"
```
