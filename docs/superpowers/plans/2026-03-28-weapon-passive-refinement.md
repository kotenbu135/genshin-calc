# Weapon Passive + Refinement Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add refinement level support to TeamMemberBuilder and expand weapon passive data for ~60-70 weapons (all 5-star + popular 4-star) with StatBuff, ConditionalBuff, and R1-R5 values.

**Architecture:** Two-stage PR approach. PR1 adds a `refinement` field to TeamMemberBuilder with a `resolve_value()` helper applied at all 6 buff resolution sites. PR2 fills weapon passive data using both StatBuff and ConditionalBuff (first real usage of the P0 conditional system for weapons).

**Tech Stack:** Rust, Cargo workspace (crates/core + crates/data), serde, thiserror

**Spec:** `docs/superpowers/specs/2026-03-28-weapon-passive-refinement-design.md`

---

## PR1: TeamMemberBuilder Refinement Integration

### Task 1: Add `CalcError::InvalidRefinement` variant

**Files:**
- Modify: `crates/core/src/error.rs:83` (add after `InvalidTalentLevel`)

- [ ] **Step 1: Write the failing test**

In `crates/core/src/error.rs`, there are no separate test files for CalcError — it uses `thiserror` derive. Instead, verify the variant exists by writing a test in the data crate (Task 2). For now, add the variant.

- [ ] **Step 2: Add the variant**

Add after line 82 (`InvalidTalentLevel(u8)`):

```rust
    #[error("refinement must be 1..=5, got {0}")]
    InvalidRefinement(u8),
```

- [ ] **Step 3: Verify build**

Run: `cargo build -p genshin-calc-core`
Expected: SUCCESS (new variant, no consumers yet)

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/error.rs
git commit -m "feat(core): add CalcError::InvalidRefinement variant"
```

---

### Task 2: Add `refinement` field and `resolve_value()` to TeamMemberBuilder

**Files:**
- Modify: `crates/data/src/team_builder.rs:15-40` (struct + new field + constructor)
- Modify: `crates/data/src/team_builder.rs:115-339` (build method)

- [ ] **Step 1: Write failing tests for refinement validation**

Add to `crates/data/src/team_builder.rs` inside `mod tests` (after the existing `test_invalid_talent_level` test at line ~547):

```rust
    #[test]
    fn test_invalid_refinement_zero() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let result = TeamMemberBuilder::new(bennett, weapon)
            .refinement(0)
            .build();
        assert!(matches!(result, Err(CalcError::InvalidRefinement(0))));
    }

    #[test]
    fn test_invalid_refinement_six() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let result = TeamMemberBuilder::new(bennett, weapon)
            .refinement(6)
            .build();
        assert!(matches!(result, Err(CalcError::InvalidRefinement(6))));
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data test_invalid_refinement`
Expected: FAIL — `refinement` method does not exist

- [ ] **Step 3: Add `refinement` field, method, and validation**

In `TeamMemberBuilder` struct (line 15-24), add field:

```rust
pub struct TeamMemberBuilder {
    character: &'static CharacterData,
    weapon: &'static WeaponData,
    artifact_set: Option<&'static ArtifactSet>,
    artifact_stats: StatProfile,
    constellation: u8,
    refinement: u8,
    talent_levels: [u8; 3],
    manual_activations: Vec<(&'static str, ManualActivation)>,
    team_elements: Vec<Element>,
}
```

In `new()` (line 30-41), add default:

```rust
    pub fn new(character: &'static CharacterData, weapon: &'static WeaponData) -> Self {
        Self {
            character,
            weapon,
            artifact_set: None,
            artifact_stats: StatProfile::default(),
            constellation: 0,
            refinement: 1,
            talent_levels: [1, 1, 1],
            manual_activations: Vec::new(),
            team_elements: Vec::new(),
        }
    }
```

Add builder method after `constellation()` (after line 58):

```rust
    /// Sets the weapon refinement level (1-5).
    pub fn refinement(mut self, r: u8) -> Self {
        self.refinement = r;
        self
    }
```

In `build()` method, add validation after the talent level check (after line 129):

```rust
        if self.refinement == 0 || self.refinement > 5 {
            return Err(CalcError::InvalidRefinement(self.refinement));
        }
```

- [ ] **Step 4: Update `build()` doc comment**

Add `InvalidRefinement` to the `# Errors` doc comment on `build()`:

```rust
    /// # Errors
    ///
    /// Returns [`CalcError::InvalidConstellation`] if constellation > 6.
    /// Returns [`CalcError::InvalidTalentLevel`] if any talent level is 0 or > 15.
    /// Returns [`CalcError::InvalidRefinement`] if refinement is 0 or > 5.
```

- [ ] **Step 5: Run validation tests**

Run: `cargo test -p genshin-calc-data test_invalid_refinement`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "feat(data): add refinement field to TeamMemberBuilder with validation"
```

---

### Task 3: Implement `resolve_value()` and apply to all 6 buff resolution sites

**Files:**
- Modify: `crates/data/src/team_builder.rs:163-330` (buff resolution in build())

- [ ] **Step 1: Write failing tests for refinement-aware buff resolution**

Add to `mod tests` in `crates/data/src/team_builder.rs`:

```rust
    #[test]
    fn test_refinement_default_uses_r1() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        // Default refinement = 1 → ATK+20%
        let member = TeamMemberBuilder::new(bennett, weapon).build().unwrap();
        let weapon_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Aquila Favonia"))
            .unwrap();
        assert!((weapon_buff.value - 0.20).abs() < EPSILON);
    }

    #[test]
    fn test_refinement_r5_uses_r5_value() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        // R5 → ATK+40%
        let member = TeamMemberBuilder::new(bennett, weapon)
            .refinement(5)
            .build()
            .unwrap();
        let weapon_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Aquila Favonia"))
            .unwrap();
        assert!((weapon_buff.value - 0.40).abs() < EPSILON);
    }

    #[test]
    fn test_refinement_r3_uses_r3_value() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        // R3 → ATK+30%
        let member = TeamMemberBuilder::new(bennett, weapon)
            .refinement(3)
            .build()
            .unwrap();
        let weapon_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Aquila Favonia"))
            .unwrap();
        assert!((weapon_buff.value - 0.30).abs() < EPSILON);
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data test_refinement_`
Expected: `test_refinement_r5_uses_r5_value` and `test_refinement_r3_uses_r3_value` FAIL — `refinement()` method exists (Task 2), but `build()` still uses `stat_buff.value` directly instead of `refinement_values[r-1]`, so R3/R5 return the same value as R1

- [ ] **Step 3: Add `resolve_value()` helper and apply to all 6 sites**

Add helper function before `apply_ascension_stat` (around line 341):

```rust
/// Resolves a buff value using refinement level.
/// If `refinement_values` is Some, uses values[refinement-1].
/// Otherwise falls back to the base `value`.
///
/// # Panics
/// Panics if `refinement` is 0 or > 5. Caller must validate beforehand.
fn resolve_value(value: f64, refinement_values: Option<[f64; 5]>, refinement: u8) -> f64 {
    debug_assert!((1..=5).contains(&refinement), "refinement must be 1..=5");
    match refinement_values {
        Some(values) => values[(refinement - 1) as usize],
        None => value,
    }
}
```

Apply to **Site 1** — Weapon passive StatBuff (line 168-175):

```rust
        // Weapon passive
        if let Some(passive) = &weapon.passive {
            for stat_buff in passive.effect.buffs {
                buffs.push(ResolvedBuff {
                    source: format!("{} ({})", passive.name, weapon.name),
                    stat: stat_buff.stat,
                    value: resolve_value(stat_buff.value, stat_buff.refinement_values, self.refinement),
                    target: BuffTarget::OnlySelf,
                });
            }
        }
```

Apply to **Site 2 & 3** — Artifact set StatBuff (line 180-195):

```rust
        // Artifact set effects
        if let Some(set) = self.artifact_set {
            for stat_buff in set.two_piece.buffs {
                buffs.push(ResolvedBuff {
                    source: format!("{} 2pc", set.name),
                    stat: stat_buff.stat,
                    value: resolve_value(stat_buff.value, stat_buff.refinement_values, self.refinement),
                    target: BuffTarget::OnlySelf,
                });
            }
            for stat_buff in set.four_piece.buffs {
                buffs.push(ResolvedBuff {
                    source: format!("{} 4pc", set.name),
                    stat: stat_buff.stat,
                    value: resolve_value(stat_buff.value, stat_buff.refinement_values, self.refinement),
                    target: BuffTarget::OnlySelf,
                });
            }
        }
```

Apply to **Sites 4, 5, 6** — ConditionalBuff resolution. Update the `resolve_conditionals` closure (line 256-304). Replace `cond_buff.value` with resolved value **before** passing to eval functions:

```rust
        // 9. Resolve conditional buffs
        let refinement = self.refinement;
        let resolve_conditionals =
            |conditional_buffs: &'static [ConditionalBuff],
             source_name: &str,
             target: BuffTarget,
             buffs: &mut Vec<ResolvedBuff>| {
                for cond_buff in conditional_buffs {
                    let base_value = resolve_value(cond_buff.value, cond_buff.refinement_values, refinement);
                    let resolved_value = match &cond_buff.activation {
                        Activation::Auto(auto) => eval_auto(
                            auto,
                            base_value,
                            &profile,
                            char_data.weapon_type,
                            char_data.element,
                            &self.team_elements,
                        ),
                        Activation::Manual(manual) => eval_manual(
                            manual,
                            cond_buff.name,
                            base_value,
                            &self.manual_activations,
                        ),
                        Activation::Both(auto, manual) => eval_auto(
                            auto,
                            base_value,
                            &profile,
                            char_data.weapon_type,
                            char_data.element,
                            &self.team_elements,
                        )
                        .and_then(|auto_value| {
                            eval_manual(
                                manual,
                                cond_buff.name,
                                auto_value,
                                &self.manual_activations,
                            )
                        }),
                    };

                    if let Some(value) = resolved_value {
                        buffs.push(ResolvedBuff {
                            source: format!("{} ({})", cond_buff.name, source_name),
                            stat: cond_buff.stat,
                            value,
                            target,
                        });
                    }
                }
            };
```

Remove `TODO(P4)` comments in both files:
- `crates/data/src/team_builder.rs` (search for `TODO(P4)`)
- `crates/data/src/buff.rs` (line 85, `ConditionalBuff.refinement_values` doc comment)

- [ ] **Step 4: Run refinement tests**

Run: `cargo test -p genshin-calc-data test_refinement_`
Expected: ALL PASS

- [ ] **Step 5: Run full test suite for regression**

Run: `cargo test`
Expected: ALL PASS (existing tests unaffected — default refinement=1 uses same value as before)

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "feat(data): implement resolve_value() for refinement-aware buff resolution

Apply refinement value resolution at all 6 buff sites:
weapon StatBuff, weapon ConditionalBuff, artifact 2pc/4pc StatBuff,
artifact 2pc/4pc ConditionalBuff."
```

---

### Task 4: Add R1 invariant regression test for existing weapons

**Files:**
- Modify: `crates/data/src/team_builder.rs` (add test in `mod tests`)

- [ ] **Step 1: Write the data integrity test**

Add to `mod tests`:

```rust
    #[test]
    fn test_all_weapon_refinement_values_r1_matches_value() {
        for weapon in crate::weapons::ALL_WEAPONS {
            if let Some(passive) = &weapon.passive {
                for (i, stat_buff) in passive.effect.buffs.iter().enumerate() {
                    if let Some(ref_values) = stat_buff.refinement_values {
                        assert!(
                            (ref_values[0] - stat_buff.value).abs() < 1e-10,
                            "Weapon '{}' StatBuff[{}]: refinement_values[0]={} != value={}",
                            weapon.name, i, ref_values[0], stat_buff.value
                        );
                    }
                }
                for (i, cond_buff) in passive.effect.conditional_buffs.iter().enumerate() {
                    if let Some(ref_values) = cond_buff.refinement_values {
                        assert!(
                            (ref_values[0] - cond_buff.value).abs() < 1e-10,
                            "Weapon '{}' ConditionalBuff[{}] '{}': refinement_values[0]={} != value={}",
                            weapon.name, i, cond_buff.name, ref_values[0], cond_buff.value
                        );
                    }
                }
            }
        }
    }
```

- [ ] **Step 2: Run the test**

Run: `cargo test -p genshin-calc-data test_all_weapon_refinement_values_r1_matches_value`
Expected: PASS (existing 37 weapons should already satisfy this invariant)

- [ ] **Step 3: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "test(data): add R1 invariant regression test for weapon refinement_values"
```

---

### Task 5: Run clippy + fmt + full test suite

**Files:** None (verification only)

- [ ] **Step 1: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

- [ ] **Step 2: Run fmt check**

Run: `cargo fmt --check`
Expected: No formatting issues (run `cargo fmt` if needed)

- [ ] **Step 3: Run full test suite**

Run: `cargo test`
Expected: ALL PASS

- [ ] **Step 4: Fix any issues and commit if needed**

---

## PR2: Weapon Passive Data Expansion

PR2 is a data-entry task. Each task below covers one weapon type file. The pattern for each weapon:

1. Look up passive effect on Genshin wiki
2. Determine if it's StatBuff (unconditional) or ConditionalBuff (conditional)
3. Add R1-R5 values with `refinement_values: Some([r1, r2, r3, r4, r5])`
4. Set `value` = R1 value
5. For non-damage effects (CD reduction, healing, shields), keep `buffs: &[]` and describe in `description`

### Task 6: Expand sword passives

**Files:**
- Modify: `crates/data/src/weapons/sword.rs`

- [ ] **Step 1: Identify target swords**

Star 5 (unimplemented passives): Primordial Jade Cutter (`primordial_jade_cutter`), Mistsplitter Reforged (`mistsplitter_reforged`), Haran Geppaku Futsu (`haran_geppaku_futsu`), Light of Foliar Incision (`light_of_foliar_incision`), Splendor of Tranquil Waters (`splendor_of_tranquil_waters`), Uraku Misugiri (`uraku_misugiri`), Peak Patrol Song (`peak_patrol_song`), etc.

Popular Star 4: The Black Sword (`the_black_sword`), Lion's Roar (`lions_roar`), Iron Sting (`iron_sting`), Festering Desire (`festering_desire`), etc.

- [ ] **Step 2: Add passives for each sword**

For each weapon, fill in `buffs` and/or `conditional_buffs` with correct values. Example for Primordial Jade Cutter:

```rust
passive: Some(WeaponPassive {
    name: "HP+20-40%。HPを基にATK上昇",
    effect: PassiveEffect {
        description: "HP+20-40%。さらにHP上限の1.2-2.4%分、ATKが上がる",
        buffs: &[StatBuff {
            stat: BuffableStat::HpPercent,
            value: 0.20,
            refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
        }],
        conditional_buffs: &[ConditionalBuff {
            name: "jade_cutter_hp_atk",
            description: "HP上限の1.2%分、ATKが上がる",
            stat: BuffableStat::AtkFlat,
            value: 0.012,
            refinement_values: Some([0.012, 0.015, 0.018, 0.021, 0.024]),
            activation: Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::HpPercent,
                cap: None,
            }),
        }],
    },
}),
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: PASS (data integrity tests validate R1 invariant and non-decreasing)

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/weapons/sword.rs
git commit -m "feat(data): expand sword weapon passives with refinement values"
```

---

### Task 7: Expand claymore passives

**Files:**
- Modify: `crates/data/src/weapons/claymore.rs`

- [ ] **Step 1: Identify target claymores**

Star 5: Wolf's Gravestone (`wolfs_gravestone`), Song of Broken Pines (`song_of_broken_pines`), Redhorn Stonethresher (`redhorn_stonethresher`), Beacon of the Reed Sea (`beacon_of_the_reed_sea`), etc.

Popular Star 4: Serpent Spine (`serpent_spine`), Rainslasher (`rainslasher`), Whiteblind (`whiteblind`), Blackcliff Slasher (`blackcliff_slasher`), etc.

- [ ] **Step 2: Add passives for each claymore**

Follow same pattern as Task 6. Example for Wolf's Gravestone:

```rust
buffs: &[StatBuff {
    stat: BuffableStat::AtkPercent,
    value: 0.20,
    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
}],
conditional_buffs: &[ConditionalBuff {
    name: "wolfs_gravestone_team_atk",
    description: "HP30%以下の敵に命中時、チーム全員ATK+40-80%",
    stat: BuffableStat::AtkPercent,
    value: 0.40,
    refinement_values: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/weapons/claymore.rs
git commit -m "feat(data): expand claymore weapon passives with refinement values"
```

---

### Task 8: Expand polearm passives

**Files:**
- Modify: `crates/data/src/weapons/polearm.rs`

- [ ] **Step 1: Identify target polearms**

Star 5: Staff of Homa (`staff_of_homa`), Engulfing Lightning (`engulfing_lightning`), Staff of the Scarlet Sands (`staff_of_the_scarlet_sands`), Primordial Jade Winged-Spear (`primordial_jade_winged_spear`), etc.

Popular Star 4: The Catch (`the_catch`), Deathmatch (`deathmatch`), Dragon's Bane (`dragons_bane`), Blackcliff Pole (`blackcliff_pole`), Lithic Spear (`lithic_spear`), etc.

- [ ] **Step 2: Add passives for each polearm**

Example for Staff of Homa:

```rust
buffs: &[StatBuff {
    stat: BuffableStat::HpPercent,
    value: 0.20,
    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
}],
conditional_buffs: &[
    ConditionalBuff {
        name: "homa_hp_atk",
        description: "HP上限の0.8%分、ATKが上がる",
        stat: BuffableStat::AtkFlat,
        value: 0.008,
        refinement_values: Some([0.008, 0.010, 0.012, 0.014, 0.016]),
        activation: Activation::Auto(AutoCondition::StatScaling {
            stat: BuffableStat::HpPercent,
            cap: None,
        }),
    },
    ConditionalBuff {
        name: "homa_low_hp_atk",
        description: "HP50%以下でさらにHP上限の1.0%分ATK上昇",
        stat: BuffableStat::AtkFlat,
        value: 0.010,
        refinement_values: Some([0.010, 0.012, 0.014, 0.016, 0.018]),
        activation: Activation::Both(
            AutoCondition::StatScaling {
                stat: BuffableStat::HpPercent,
                cap: None,
            },
            ManualCondition::Toggle,
        ),
    },
],
```

Note: Homa's two ConditionalBuffs are **additive**. `homa_hp_atk` (Auto, always on) gives base HP→ATK conversion. `homa_low_hp_atk` (Both, Toggle required) gives the additional HP→ATK conversion when HP < 50%. Both independently compute `total_hp * multiplier` and both contribute to final AtkFlat.

Example for The Catch:

```rust
buffs: &[
    StatBuff {
        stat: BuffableStat::BurstDmgBonus,
        value: 0.16,
        refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
    },
    // Note: The Catch's crit rate buff is burst-only in game, but BuffableStat
    // has no BurstCritRate variant. Using CritRate as approximation (existing pattern).
    StatBuff {
        stat: BuffableStat::CritRate,
        value: 0.06,
        refinement_values: Some([0.06, 0.075, 0.09, 0.105, 0.12]),
    },
],
conditional_buffs: &[],
```

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/weapons/polearm.rs
git commit -m "feat(data): expand polearm weapon passives with refinement values"
```

---

### Task 9: Expand bow passives

**Files:**
- Modify: `crates/data/src/weapons/bow.rs`

- [ ] **Step 1: Identify target bows**

Star 5: Thundering Pulse (`thundering_pulse`), Aqua Simulacra (`aqua_simulacra`), Polar Star (`polar_star`), Elegy for the End (`elegy_for_the_end`), Hunter's Path (`hunters_path`), etc.

Popular Star 4: Stringless (`the_stringless`), Rust (`rust`), Mouun's Moon (`mouuns_moon`), Blackcliff Warbow (`blackcliff_warbow`), etc.

- [ ] **Step 2: Add passives for each bow**

Follow same pattern.

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/weapons/bow.rs
git commit -m "feat(data): expand bow weapon passives with refinement values"
```

---

### Task 10: Expand catalyst passives

**Files:**
- Modify: `crates/data/src/weapons/catalyst.rs`

- [ ] **Step 1: Identify target catalysts**

Star 5: Kagura's Verity (`kaguras_verity`), Lost Prayer to the Sacred Winds (`lost_prayer_to_the_sacred_winds`), A Thousand Floating Dreams (`a_thousand_floating_dreams`), Tulaytullah's Remembrance (`tulaytullahs_remembrance`), etc.

Popular Star 4: Widsith (`the_widsith`), Solar Pearl (`solar_pearl`), Mappa Mare (`mappa_mare`), Dodoco Tales (`dodoco_tales`), Blackcliff Agate (`blackcliff_agate`), etc.

- [ ] **Step 2: Add passives for each catalyst**

Follow same pattern.

- [ ] **Step 3: Run tests**

Run: `cargo test -p genshin-calc-data`
Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/weapons/catalyst.rs
git commit -m "feat(data): expand catalyst weapon passives with refinement values"
```

---

### Task 11: Add data integrity tests for PR2

**Files:**
- Modify: `crates/data/src/team_builder.rs` (add tests)

- [ ] **Step 1: Write non-decreasing refinement values test**

Add to `mod tests`:

```rust
    #[test]
    fn test_all_weapon_refinement_values_non_decreasing() {
        for weapon in crate::weapons::ALL_WEAPONS {
            if let Some(passive) = &weapon.passive {
                for (i, stat_buff) in passive.effect.buffs.iter().enumerate() {
                    if let Some(rv) = stat_buff.refinement_values {
                        for w in rv.windows(2) {
                            assert!(
                                w[0] <= w[1],
                                "Weapon '{}' StatBuff[{}]: refinement values not non-decreasing: {:?}",
                                weapon.name, i, rv
                            );
                        }
                    }
                }
                for (i, cond_buff) in passive.effect.conditional_buffs.iter().enumerate() {
                    if let Some(rv) = cond_buff.refinement_values {
                        for w in rv.windows(2) {
                            assert!(
                                w[0] <= w[1],
                                "Weapon '{}' ConditionalBuff[{}] '{}': refinement values not non-decreasing: {:?}",
                                weapon.name, i, cond_buff.name, rv
                            );
                        }
                    }
                }
            }
        }
    }
```

- [ ] **Step 2: Write integration test for a ConditionalBuff weapon**

Add to `mod conditional_tests`:

```rust
    #[test]
    fn test_weapon_conditional_buff_with_refinement() {
        // Requires Task 6 to be completed first (Jade Cutter ConditionalBuff data).
        // Tests that both StatBuff (HP%) and ConditionalBuff (HP→ATK) use refinement values.
        let keqing = find_character("keqing").unwrap();
        let weapon = find_weapon("primordial_jade_cutter").unwrap();

        // R1
        let r1 = TeamMemberBuilder::new(keqing, weapon)
            .refinement(1)
            .build()
            .unwrap();

        // R5
        let r5 = TeamMemberBuilder::new(keqing, weapon)
            .refinement(5)
            .build()
            .unwrap();

        // HP% StatBuff should differ between R1 and R5
        let r1_hp = r1.buffs_provided.iter()
            .find(|b| b.stat == BuffableStat::HpPercent && b.source.contains("Jade Cutter"))
            .unwrap();
        let r5_hp = r5.buffs_provided.iter()
            .find(|b| b.stat == BuffableStat::HpPercent && b.source.contains("Jade Cutter"))
            .unwrap();
        assert!((r1_hp.value - 0.20).abs() < EPSILON);
        assert!((r5_hp.value - 0.40).abs() < EPSILON);
    }
```

- [ ] **Step 3: Run all tests**

Run: `cargo test`
Expected: ALL PASS

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "test(data): add data integrity tests for weapon refinement values"
```

---

### Task 12: Final verification

**Files:** None (verification only)

- [ ] **Step 1: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

- [ ] **Step 2: Run fmt check**

Run: `cargo fmt --check`
Expected: Clean

- [ ] **Step 3: Run full test suite**

Run: `cargo test`
Expected: ALL PASS

- [ ] **Step 4: Update data-expansion-todo.md**

Mark P3 and P4 items as done (at least partially). Update the coverage numbers in the summary table.

- [ ] **Step 5: Commit**

```bash
git add docs/data-expansion-todo.md
git commit -m "docs: update data expansion TODO with P3/P4 progress"
```

---

## Notes for Implementer

### Data Sources
- Genshin Impact Wiki (fandom) for weapon passive descriptions and R1-R5 values
- All percentages in decimal form: 10.8% → 0.108

### BuffableStat Reference
Common stats used in weapon passives:
- `AtkPercent`, `AtkFlat`, `HpPercent`, `DefPercent`
- `CritRate`, `CritDmg`
- `DmgBonus`, `ElementalDmgBonus(Element::X)`
- `NormalAtkDmgBonus`, `ChargedAtkDmgBonus`, `SkillDmgBonus`, `BurstDmgBonus`
- `ElementalMastery`, `EnergyRecharge`

### Non-implementable Effects (keep `buffs: &[]`)
- Favonius series: particle generation on crit
- Sacrificial series: skill CD reset on hit
- Healing, shields, movement speed, stamina effects
- Proc damage (e.g. Aquila Favonia's ATK% damage proc — the ATK% buff IS implemented, the proc is not)

### ConditionalBuff Activation Quick Reference
| Game Mechanic | Activation Type |
|---------------|----------------|
| Always-on stat conversion (Jade Cutter HP→ATK) | `Auto(StatScaling)` |
| Weapon type restriction | `Auto(WeaponTypeRequired)` |
| Element restriction | `Auto(ElementRequired)` |
| HP threshold, on-hit proc, after skill | `Manual(Toggle)` |
| Stackable effect | `Manual(Stacks(max))` |
| Stat conversion + toggle (Homa low HP) | `Both(StatScaling, Toggle)` |
