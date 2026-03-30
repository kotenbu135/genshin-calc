# `build()` Method Extraction Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extract 228-line `TeamMemberBuilder::build()` into 7 private methods so `build()` becomes a ~10-line table of contents.

**Architecture:** Pure method extraction on `impl TeamMemberBuilder`. No new types, no new files, no behavioral changes. Existing free functions stay as-is.

**Tech Stack:** Rust

**Spec:** `docs/superpowers/specs/2026-03-31-build-refactoring-design.md`

---

### Task 1: Extract `validate`

**Files:**
- Modify: `crates/data/src/team_builder.rs:137-148`

- [ ] **Step 1: Add `validate` method**

Add after the `build()` method (after line 364), inside `impl TeamMemberBuilder`:

```rust
fn validate(&self) -> Result<(), CalcError> {
    if self.constellation > 6 {
        return Err(CalcError::InvalidConstellation(self.constellation));
    }
    for &level in &self.talent_levels {
        if level == 0 || level > 15 {
            return Err(CalcError::InvalidTalentLevel(level));
        }
    }
    if self.refinement == 0 || self.refinement > 5 {
        return Err(CalcError::InvalidRefinement(self.refinement));
    }
    Ok(())
}
```

- [ ] **Step 2: Replace inline validation in `build()`**

Replace lines 138-148 in `build()` with:

```rust
self.validate()?;
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All tests pass (no behavioral change)

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "refactor: extract validate() from build()"
```

---

### Task 2: Extract `build_base_profile`

**Files:**
- Modify: `crates/data/src/team_builder.rs`

- [ ] **Step 1: Add `build_base_profile` method**

Add inside `impl TeamMemberBuilder`:

```rust
fn build_base_profile(&self) -> StatProfile {
    let mut profile = StatProfile {
        base_hp: self.character.base_hp[3],
        base_atk: self.character.base_atk[3] + self.weapon.base_atk[3],
        base_def: self.character.base_def[3],
        ..Default::default()
    };
    apply_ascension_stat(&mut profile, &self.character.ascension_stat);
    if let Some(sub) = &self.weapon.sub_stat {
        apply_weapon_sub_stat(&mut profile, sub);
    }
    profile
}
```

- [ ] **Step 2: Replace inline code in `build()`**

Remove lines 150-167 (the `let char_data`, `let weapon`, base stats, ascension stat, weapon sub-stat blocks) and replace with:

```rust
let mut profile = self.build_base_profile();
```

Note: `let char_data = self.character;` and `let weapon = self.weapon;` are removed here. Replace ALL remaining `char_data` → `self.character` and `weapon` → `self.weapon` throughout the entire `build()` body, including:
- Line ~186: `&weapon.passive` → `&self.weapon.passive`
- Line ~189: `weapon.name` → `self.weapon.name`
- Line ~222: `char_data.id` → `self.character.id`
- Line ~242: `char_data.effective_talent_level(...)` → `self.character.effective_talent_level(...)`
- Line ~291: `char_data.weapon_type` → `self.character.weapon_type`
- Line ~292: `char_data.element` → `self.character.element`
- Line ~332: `&weapon.passive` → `&self.weapon.passive`
- Line ~335: `weapon.name` → `self.weapon.name`
- Line ~358: `char_data.element` → `self.character.element`
- Line ~359: `char_data.weapon_type` → `self.character.weapon_type`
- Line ~362: `char_data.id` → `self.character.id`

All of these will be progressively removed as Tasks 3-7 extract each block, but the code must compile after this task.

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "refactor: extract build_base_profile() from build()"
```

---

### Task 3: Extract `merge_artifact_stats`

**Files:**
- Modify: `crates/data/src/team_builder.rs`

- [ ] **Step 1: Add `merge_artifact_stats` method**

```rust
fn merge_artifact_stats(&self, profile: &mut StatProfile) {
    profile.hp_percent += self.artifact_stats.hp_percent;
    profile.atk_percent += self.artifact_stats.atk_percent;
    profile.def_percent += self.artifact_stats.def_percent;
    profile.hp_flat += self.artifact_stats.hp_flat;
    profile.atk_flat += self.artifact_stats.atk_flat;
    profile.def_flat += self.artifact_stats.def_flat;
    profile.elemental_mastery += self.artifact_stats.elemental_mastery;
    profile.crit_rate += self.artifact_stats.crit_rate;
    profile.crit_dmg += self.artifact_stats.crit_dmg;
    profile.energy_recharge += self.artifact_stats.energy_recharge;
    profile.dmg_bonus += self.artifact_stats.dmg_bonus;
}
```

- [ ] **Step 2: Replace inline code in `build()`**

Replace the 11-line artifact stats block with:

```rust
self.merge_artifact_stats(&mut profile);
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "refactor: extract merge_artifact_stats() from build()"
```

---

### Task 4: Extract `collect_static_buffs`

**Files:**
- Modify: `crates/data/src/team_builder.rs`

- [ ] **Step 1: Add `collect_static_buffs` method**

```rust
fn collect_static_buffs(&self) -> Vec<ResolvedBuff> {
    let mut buffs = Vec::new();

    // Weapon passive
    if let Some(passive) = &self.weapon.passive {
        for stat_buff in passive.effect.buffs {
            buffs.push(ResolvedBuff {
                source: format!("{} ({})", passive.name, self.weapon.name),
                stat: stat_buff.stat,
                value: resolve_value(
                    stat_buff.value,
                    stat_buff.refinement_values,
                    self.refinement,
                ),
                target: BuffTarget::OnlySelf,
            });
        }
    }

    // Artifact set effects
    if let Some(set) = self.artifact_set {
        for stat_buff in set.two_piece.buffs {
            buffs.push(ResolvedBuff {
                source: format!("{} 2pc", set.name),
                stat: stat_buff.stat,
                value: resolve_value(stat_buff.value, stat_buff.refinement_values, 1),
                target: BuffTarget::OnlySelf,
            });
        }
        for stat_buff in set.four_piece.buffs {
            buffs.push(ResolvedBuff {
                source: format!("{} 4pc", set.name),
                stat: stat_buff.stat,
                value: resolve_value(stat_buff.value, stat_buff.refinement_values, 1),
                target: BuffTarget::OnlySelf,
            });
        }
    }

    buffs
}
```

- [ ] **Step 2: Replace inline code in `build()`**

Replace the `let mut buffs = Vec::new();` + weapon passive + artifact set blocks with:

```rust
let mut buffs = self.collect_static_buffs();
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "refactor: extract collect_static_buffs() from build()"
```

---

### Task 5: Extract `collect_talent_buffs`

**Files:**
- Modify: `crates/data/src/team_builder.rs`

- [ ] **Step 1: Add `collect_talent_buffs` method**

```rust
fn collect_talent_buffs(&self, profile: &StatProfile, buffs: &mut Vec<ResolvedBuff>) {
    let Some(talent_defs) = find_talent_buffs(self.character.id) else {
        return;
    };
    for def in talent_defs {
        if self.constellation < def.min_constellation {
            continue;
        }
        let raw_value = if def.scales_with_talent {
            if let Some(scaling) = def.talent_scaling {
                let (talent_idx, damage_type) = match def.source {
                    crate::talent_buffs::TalentBuffSource::ElementalSkill => {
                        (1, DamageType::Skill)
                    }
                    crate::talent_buffs::TalentBuffSource::ElementalBurst => {
                        (2, DamageType::Burst)
                    }
                    _ => (0, DamageType::Normal),
                };
                let base_level = self.talent_levels[talent_idx];
                let level = self.character.effective_talent_level(
                    damage_type,
                    base_level,
                    self.constellation,
                );
                scaling[(level - 1) as usize]
            } else {
                def.base_value
            }
        } else {
            def.base_value
        };

        let value = if let Some(scaling_stat) = def.scales_on {
            let base = match scaling_stat {
                genshin_calc_core::ScalingStat::Atk => profile.base_atk,
                genshin_calc_core::ScalingStat::Hp => profile.base_hp,
                genshin_calc_core::ScalingStat::Def => profile.base_def,
                genshin_calc_core::ScalingStat::Em => profile.elemental_mastery,
            };
            base * raw_value
        } else {
            raw_value
        };

        buffs.push(ResolvedBuff {
            source: def.name.to_string(),
            stat: def.stat,
            value,
            target: def.target,
        });
    }
}
```

Note: Uses `let Some(...) = ... else { return; }` (let-else) instead of `if let Some(...)` to reduce nesting.

- [ ] **Step 2: Replace inline code in `build()`**

Replace the talent buffs block with:

```rust
self.collect_talent_buffs(&profile, &mut buffs);
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All tests pass

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "refactor: extract collect_talent_buffs() from build()"
```

---

### Task 6: Extract `resolve_conditional_buffs` and `resolve_conditionals_for_source`

**Files:**
- Modify: `crates/data/src/team_builder.rs`

- [ ] **Step 1: Add `resolve_conditionals_for_source` helper method**

```rust
fn resolve_conditionals_for_source(
    &self,
    conditional_buffs: &'static [ConditionalBuff],
    source_name: &str,
    refinement: u8,
    profile: &StatProfile,
    buffs: &mut Vec<ResolvedBuff>,
) {
    for cond_buff in conditional_buffs {
        let effective_value =
            resolve_value(cond_buff.value, cond_buff.refinement_values, refinement);
        let resolved_value = match &cond_buff.activation {
            Activation::Auto(auto) => eval_auto(
                auto,
                effective_value,
                profile,
                self.character.weapon_type,
                self.character.element,
                &self.team_elements,
                &self.team_regions,
                refinement,
            ),
            Activation::Manual(manual) => eval_manual(
                manual,
                cond_buff,
                &self.manual_activations,
                effective_value,
            ),
            Activation::Both(auto, manual) => {
                let auto_result = eval_auto(
                    auto,
                    effective_value,
                    profile,
                    self.character.weapon_type,
                    self.character.element,
                    &self.team_elements,
                    &self.team_regions,
                    refinement,
                );
                auto_result.and_then(|av| {
                    eval_manual(manual, cond_buff, &self.manual_activations, av)
                })
            }
        };

        if let Some(value) = resolved_value {
            buffs.push(ResolvedBuff {
                source: format!("{} ({})", cond_buff.name, source_name),
                stat: cond_buff.stat,
                value,
                target: cond_buff.target,
            });
        }
    }
}
```

- [ ] **Step 2: Add `resolve_conditional_buffs` method**

```rust
fn resolve_conditional_buffs(
    &self,
    profile: &StatProfile,
    buffs: &mut Vec<ResolvedBuff>,
) {
    if let Some(passive) = &self.weapon.passive {
        self.resolve_conditionals_for_source(
            passive.effect.conditional_buffs,
            self.weapon.name,
            self.refinement,
            profile,
            buffs,
        );
    }
    if let Some(set) = self.artifact_set {
        self.resolve_conditionals_for_source(
            set.two_piece.conditional_buffs,
            &format!("{} 2pc", set.name),
            1,
            profile,
            buffs,
        );
        self.resolve_conditionals_for_source(
            set.four_piece.conditional_buffs,
            &format!("{} 4pc", set.name),
            1,
            profile,
            buffs,
        );
    }
}
```

- [ ] **Step 3: Replace inline code in `build()`**

Replace the closure definition + 3 call sites with:

```rust
self.resolve_conditional_buffs(&profile, &mut buffs);
```

- [ ] **Step 4: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All tests pass

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "refactor: extract resolve_conditional_buffs() from build()"
```

---

### Task 7: Extract `into_team_member` and finalize `build()`

**Files:**
- Modify: `crates/data/src/team_builder.rs`

- [ ] **Step 1: Add `into_team_member` method**

```rust
fn into_team_member(self, stats: StatProfile, buffs: Vec<ResolvedBuff>) -> TeamMember {
    TeamMember {
        element: self.character.element,
        weapon_type: self.character.weapon_type,
        stats,
        buffs_provided: buffs,
        is_moonsign: is_moonsign_character(self.character.id),
    }
}
```

- [ ] **Step 2: Verify and finalize `build()` method**

Verify that `build()` now matches the following target form. If any inline code remains from previous tasks, replace the entire `build()` body with:

```rust
pub fn build(self) -> Result<TeamMember, CalcError> {
    self.validate()?;
    let mut profile = self.build_base_profile();
    self.merge_artifact_stats(&mut profile);
    let mut buffs = self.collect_static_buffs();
    self.collect_talent_buffs(&profile, &mut buffs);
    self.resolve_conditional_buffs(&profile, &mut buffs);
    Ok(self.into_team_member(profile, buffs))
}
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: All tests pass

- [ ] **Step 4: Run clippy and fmt**

Run: `cargo clippy -- -D warnings && cargo fmt --check`
Expected: No warnings, no format issues

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "refactor: extract into_team_member() and finalize build() as table of contents"
```

---

### Task 8: Final verification

- [ ] **Step 1: Run full test suite**

Run: `cargo test`
Expected: All tests pass across both crates

- [ ] **Step 2: Run clippy on workspace**

Run: `cargo clippy --workspace -- -D warnings`
Expected: No warnings

- [ ] **Step 3: Verify build() line count**

Run: `grep -n 'pub fn build' crates/data/src/team_builder.rs` and count lines to closing `}`.
Expected: ~10 lines

- [ ] **Step 4: Delete spec and plan files**

Per CLAUDE.md, implementation is complete so remove design docs:

```bash
rm docs/superpowers/specs/2026-03-31-build-refactoring-design.md
rm docs/superpowers/plans/2026-03-31-build-refactoring.md
git add -A docs/superpowers/
git commit -m "chore: remove completed build refactoring spec and plan"
```
