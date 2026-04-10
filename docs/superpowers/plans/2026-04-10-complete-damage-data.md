# Complete Damage Data Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Complete audited damage-related character and artifact data using `docs/audit/`, excluding new correctness work for added-damage/proc effects.

**Architecture:** Add an exact-reaction buff stat while preserving broad reaction bonus stats for compatibility. Then repair audited data in bounded slices: core model, artifacts, character data, character talent buffs, and final audit closure.

**Tech Stack:** Rust 2024 workspace, `genshin-calc-core`, `genshin-calc-data`, `genshin-calc-wasm`, serde externally-tagged enums, Cargo tests.

---

## File Structure

- Modify: `crates/core/src/buff_types.rs`
  - Add `BuffableStat::ReactionDmgBonus(Reaction)` and serde roundtrip tests.
- Modify: `crates/core/src/team.rs`
  - Store reaction-specific bonuses in `DamageContext`.
  - Add helper methods for standard, transformative, and lunar reactions.
- Modify: `crates/data/src/team_builder.rs`
  - Treat `ReactionDmgBonus(_)` as a non-profile stat and a non-scaling source.
- Modify: `crates/wasm/ts/types.ts`
  - Add TypeScript shape for `ReactionDmgBonus`.
- Modify: `crates/data/src/artifacts.rs`
  - Fix audited artifact data and add missing damage-relevant sets.
- Modify: `crates/data/tests/data_integrity.rs`
  - Add artifact regression tests that inspect set definitions directly.
- Modify: `crates/core/tests/character_verification.rs` or `crates/data/tests/data_integrity.rs`
  - Add representative character-data regression tests where direct data inspection is simpler.
- Modify: `crates/data/src/characters/<element>/*.rs`
  - Repair audited character base stats, metadata, and talent scalings.
- Modify: `crates/data/src/talent_buffs/<element>.rs`
  - Repair audited character buff definitions.
- Modify: `crates/data/src/talent_buffs/mod.rs`
  - Add targeted tests for corrected talent buff definitions if existing module tests are the closest location.

## Task 1: Reaction-Specific Buff Model

**Files:**
- Modify: `crates/core/src/buff_types.rs`
- Modify: `crates/core/src/team.rs`
- Modify: `crates/data/src/team_builder.rs`
- Modify: `crates/wasm/ts/types.ts`

- [ ] **Step 1: Write failing tests for reaction-specific stat serialization and aggregation**

Add this test to `crates/core/src/buff_types.rs` inside the existing `#[cfg(test)] mod tests`:

```rust
#[test]
fn test_reaction_dmg_bonus_serde_roundtrip() {
    use crate::reaction::Reaction;
    use crate::types::Element;

    for stat in [
        BuffableStat::ReactionDmgBonus(Reaction::Bloom),
        BuffableStat::ReactionDmgBonus(Reaction::Aggravate),
        BuffableStat::ReactionDmgBonus(Reaction::Swirl(Element::Pyro)),
        BuffableStat::ReactionDmgBonus(Reaction::LunarBloom),
    ] {
        let json = serde_json::to_string(&stat).unwrap();
        let deser: BuffableStat = serde_json::from_str(&json).unwrap();
        assert_eq!(stat, deser);
    }
}
```

Add these tests to `crates/core/src/team.rs` inside the existing test module, or create a `#[cfg(test)] mod tests` at the bottom if none exists:

```rust
#[test]
fn reaction_dmg_bonus_exact_match_does_not_over_apply() {
    use crate::reaction::Reaction;
    use crate::types::Element;

    let ctx = DamageContext::from_buffs(&[
        ResolvedBuff {
            source: "Bloom Only".to_string(),
            stat: BuffableStat::ReactionDmgBonus(Reaction::Bloom),
            value: 0.40,
            target: BuffTarget::OnlySelf,
            origin: None,
        },
        ResolvedBuff {
            source: "Aggravate Only".to_string(),
            stat: BuffableStat::ReactionDmgBonus(Reaction::Aggravate),
            value: 0.20,
            target: BuffTarget::OnlySelf,
            origin: None,
        },
        ResolvedBuff {
            source: "Pyro Swirl Only".to_string(),
            stat: BuffableStat::ReactionDmgBonus(Reaction::Swirl(Element::Pyro)),
            value: 0.60,
            target: BuffTarget::OnlySelf,
            origin: None,
        },
    ]);

    assert!((ctx.reaction_bonus_for(Reaction::Bloom) - 0.40).abs() < 1e-9);
    assert!((ctx.reaction_bonus_for(Reaction::Overloaded) - 0.0).abs() < 1e-9);
    assert!((ctx.reaction_bonus_for(Reaction::Aggravate) - 0.20).abs() < 1e-9);
    assert!((ctx.reaction_bonus_for(Reaction::Spread) - 0.0).abs() < 1e-9);
    assert!((ctx.reaction_bonus_for(Reaction::Swirl(Element::Pyro)) - 0.60).abs() < 1e-9);
    assert!((ctx.reaction_bonus_for(Reaction::Swirl(Element::Hydro)) - 0.0).abs() < 1e-9);
}

#[test]
fn reaction_dmg_bonus_combines_broad_and_exact_values() {
    use crate::reaction::Reaction;

    let ctx = DamageContext::from_buffs(&[
        ResolvedBuff {
            source: "Broad Transformative".to_string(),
            stat: BuffableStat::TransformativeBonus,
            value: 0.10,
            target: BuffTarget::OnlySelf,
            origin: None,
        },
        ResolvedBuff {
            source: "Bloom Exact".to_string(),
            stat: BuffableStat::ReactionDmgBonus(Reaction::Bloom),
            value: 0.40,
            target: BuffTarget::OnlySelf,
            origin: None,
        },
    ]);

    assert!((ctx.reaction_bonus_for(Reaction::Bloom) - 0.50).abs() < 1e-9);
    assert!((ctx.reaction_bonus_for(Reaction::Overloaded) - 0.10).abs() < 1e-9);
}

#[test]
fn reaction_dmg_bonus_lunar_exact_match_does_not_over_apply() {
    use crate::reaction::Reaction;

    let ctx = DamageContext::from_buffs(&[ResolvedBuff {
        source: "Lunar Bloom Only".to_string(),
        stat: BuffableStat::ReactionDmgBonus(Reaction::LunarBloom),
        value: 0.15,
        target: BuffTarget::OnlySelf,
        origin: None,
    }]);

    assert!((ctx.reaction_bonus_for(Reaction::LunarBloom) - 0.15).abs() < 1e-9);
    assert!((ctx.reaction_bonus_for(Reaction::LunarElectroCharged) - 0.0).abs() < 1e-9);
}
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```sh
cargo test -p genshin-calc-core reaction_dmg_bonus -- --nocapture
```

Expected: compile failure because `BuffableStat::ReactionDmgBonus` and `DamageContext::reaction_bonus_for` do not exist.

- [ ] **Step 3: Implement the model extension**

In `crates/core/src/buff_types.rs`, import `Reaction` and add the enum variant:

```rust
use crate::reaction::Reaction;
use crate::types::Element;
use serde::{Deserialize, Serialize};
```

Add the variant after `AdditiveBonus`:

```rust
/// Exact elemental reaction DMG bonus.
ReactionDmgBonus(Reaction),
```

In `crates/core/src/team.rs`, import reaction types:

```rust
use crate::reaction::{Reaction, ReactionCategory};
```

Add this field to `DamageContext` after `additive_bonus`:

```rust
/// Exact reaction DMG bonuses that only apply to the matching reaction.
pub reaction_dmg_bonuses: Vec<(Reaction, f64)>,
```

Update `DamageContext::from_buffs` match:

```rust
BuffableStat::ReactionDmgBonus(reaction) => {
    ctx.reaction_dmg_bonuses.push((reaction, buff.value));
}
```

Add these methods on `impl DamageContext`:

```rust
/// Returns the total reaction bonus for a specific reaction, including
/// backward-compatible broad category bonuses and exact reaction bonuses.
pub fn reaction_bonus_for(&self, reaction: Reaction) -> f64 {
    self.broad_reaction_bonus_for(reaction)
        + self
            .reaction_dmg_bonuses
            .iter()
            .filter(|(r, _)| *r == reaction)
            .map(|(_, value)| *value)
            .sum::<f64>()
}

fn broad_reaction_bonus_for(&self, reaction: Reaction) -> f64 {
    match reaction.category() {
        ReactionCategory::Amplifying => self.amplifying_bonus,
        ReactionCategory::Catalyze => self.additive_bonus,
        ReactionCategory::Transformative => self.transformative_bonus,
        ReactionCategory::Lunar => self.transformative_bonus,
    }
}
```

In `crates/data/src/team_builder.rs`, add `BuffableStat::ReactionDmgBonus(_)` to the `read_stat_for_scaling` non-readable match arm:

```rust
| BuffableStat::ReactionDmgBonus(_)
```

In `crates/wasm/ts/types.ts`, add a helper type:

```ts
type ReactionDmgBonusStat = { ReactionDmgBonus: Reaction };
```

Then add it to `BuffableStat`:

```ts
  | ReactionDmgBonusStat
```

- [ ] **Step 4: Run tests to verify pass**

Run:

```sh
cargo test -p genshin-calc-core reaction_dmg_bonus -- --nocapture
```

Expected: all added tests pass.

- [ ] **Step 5: Commit**

```sh
git add crates/core/src/buff_types.rs crates/core/src/team.rs crates/data/src/team_builder.rs crates/wasm/ts/types.ts
git commit -m "feat: add reaction-specific damage bonuses"
```

## Task 2: Artifact Regression Tests

**Files:**
- Modify: `crates/data/tests/data_integrity.rs`

- [ ] **Step 1: Write failing artifact tests**

Add these tests to `crates/data/tests/data_integrity.rs`:

```rust
use genshin_calc_core::{BuffableStat, Element, Reaction};
```

If `BuffableStat` is already imported through another path, merge the import instead of duplicating it.

Add helper functions near the top of the file:

```rust
fn artifact_set(id: &str) -> &'static genshin_calc_data::types::ArtifactSet {
    ALL_ARTIFACT_SETS
        .iter()
        .copied()
        .find(|set| set.id == id)
        .unwrap_or_else(|| panic!("artifact set not found: {id}"))
}

fn conditional_values(
    set: &genshin_calc_data::types::ArtifactSet,
    stat: BuffableStat,
) -> Vec<f64> {
    set.four_piece
        .conditional_buffs
        .iter()
        .filter(|buff| buff.stat == stat)
        .map(|buff| buff.value)
        .collect()
}
```

Add tests:

```rust
#[test]
fn artifact_audit_nymphs_dream_stack_values_match_mirror() {
    let set = artifact_set("nymphs_dream");

    assert_eq!(
        conditional_values(set, BuffableStat::AtkPercent),
        vec![0.07, 0.16, 0.25]
    );
    assert_eq!(
        conditional_values(set, BuffableStat::ElementalDmgBonus(Element::Hydro)),
        vec![0.04, 0.09, 0.15]
    );
}

#[test]
fn artifact_audit_scroll_of_cinder_city_two_piece_has_no_damage_relevant_em_bonus() {
    let set = artifact_set("scroll_of_the_hero_of_cinder_city");

    assert!(
        set.two_piece
            .buffs
            .iter()
            .all(|buff| buff.stat != BuffableStat::ElementalMastery),
        "Scroll 2pc is energy-only in the mirror and must not grant EM"
    );
}

#[test]
fn artifact_audit_missing_damage_relevant_artifact_sets_are_registered() {
    for id in [
        "adventurer",
        "lucky_dog",
        "finale_of_the_deep_galleries",
        "long_nights_oath",
    ] {
        artifact_set(id);
    }
}

#[test]
fn artifact_audit_reaction_specific_buffs_are_not_generic() {
    let thundering_fury = artifact_set("thundering_fury");
    assert!(
        thundering_fury
            .four_piece
            .conditional_buffs
            .iter()
            .any(|buff| buff.stat == BuffableStat::ReactionDmgBonus(Reaction::Aggravate))
    );
    assert!(
        thundering_fury
            .four_piece
            .conditional_buffs
            .iter()
            .all(|buff| buff.stat != BuffableStat::AdditiveBonus)
    );

    let viridescent = artifact_set("viridescent_venerer");
    assert!(
        viridescent
            .four_piece
            .conditional_buffs
            .iter()
            .any(|buff| buff.stat == BuffableStat::ReactionDmgBonus(Reaction::Swirl(Element::Pyro)))
    );

    let paradise = artifact_set("flower_of_paradise_lost");
    assert!(
        paradise
            .four_piece
            .conditional_buffs
            .iter()
            .any(|buff| buff.stat == BuffableStat::ReactionDmgBonus(Reaction::LunarBloom))
    );
}
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```sh
cargo test -p genshin-calc-data artifact_audit -- --nocapture
```

Expected: failures for swapped Nymph values, Scroll EM, missing sets, and generic reaction stats.

- [ ] **Step 3: Commit failing tests**

```sh
git add crates/data/tests/data_integrity.rs
git commit -m "test: capture audited artifact discrepancies"
```

## Task 3: Artifact Data Fixes

**Files:**
- Modify: `crates/data/src/artifacts.rs`

- [ ] **Step 1: Fix Nymph's Dream and Scroll**

In `NYMPHS_DREAM`, set the three `AtkPercent` conditional buff values to:

```rust
0.07
0.16
0.25
```

Set the three `ElementalDmgBonus(Element::Hydro)` conditional buff values to:

```rust
0.04
0.09
0.15
```

In `SCROLL_OF_THE_HERO_OF_CINDER_CITY`, replace the two-piece `buffs` slice with:

```rust
buffs: &[],
```

Update the two-piece description so it states the 2pc is energy-only and not represented as a damage stat.

- [ ] **Step 2: Add missing damage-relevant artifact sets**

Add these constants near the mixed rarity artifact sets in `crates/data/src/artifacts.rs`.

Use `long_nights_oath` as the Rust ID to avoid an apostrophe in the identifier string.

```rust
pub const ADVENTURER: ArtifactSet = ArtifactSet {
    id: "adventurer",
    name: "冒険者",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("HP上限+1000"),
        buffs: &[StatBuff {
            stat: BuffableStat::HpFlat,
            value: 1000.0,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("宝箱を開けた後の回復効果。ダメージ計算対象外"),
        buffs: &[],
        conditional_buffs: &[],
    },
};

pub const LUCKY_DOG: ArtifactSet = ArtifactSet {
    id: "lucky_dog",
    name: "幸運",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("防御力+100"),
        buffs: &[StatBuff {
            stat: BuffableStat::DefFlat,
            value: 100.0,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("モラ獲得時の回復効果。ダメージ計算対象外"),
        buffs: &[],
        conditional_buffs: &[],
    },
};

pub const FINALE_OF_THE_DEEP_GALLERIES: ArtifactSet = ArtifactSet {
    id: "finale_of_the_deep_galleries",
    name: "深廊の終曲",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("氷元素ダメージ+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("元素エネルギーが0の時、通常攻撃と元素爆発のダメージ+60%"),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "finale_deep_galleries_normal_bonus",
                description: desc!("At 0 Elemental Energy, Normal Attack DMG +60%"),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.60,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "finale_deep_galleries_burst_bonus",
                description: desc!("At 0 Elemental Energy, Elemental Burst DMG +60%"),
                stat: BuffableStat::BurstDmgBonus,
                value: 0.60,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const LONG_NIGHTS_OATH: ArtifactSet = ArtifactSet {
    id: "long_nights_oath",
    name: "長き夜の誓い",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("落下攻撃ダメージ+25%"),
        buffs: &[StatBuff {
            stat: BuffableStat::PlungingAtkDmgBonus,
            value: 0.25,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("落下/重撃/元素スキル命中でスタックを獲得し、落下攻撃ダメージ+15%/層、最大5層"),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "long_nights_oath_plunging_stacks",
            description: desc!("Plunging Attack DMG +15% per stack, max 5"),
            stat: BuffableStat::PlungingAtkDmgBonus,
            value: 0.15,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Stacks(5)),
        }],
    },
};
```

Register all four constants in `ALL_ARTIFACT_SETS`.

- [ ] **Step 3: Replace audited generic artifact reaction bonuses**

For `THUNDERING_FURY`, replace broad reaction conditional buffs with exact reaction buffs:

```rust
ReactionDmgBonus(Reaction::Overloaded) => 0.40
ReactionDmgBonus(Reaction::ElectroCharged) => 0.40
ReactionDmgBonus(Reaction::Superconduct) => 0.40
ReactionDmgBonus(Reaction::Hyperbloom) => 0.40
ReactionDmgBonus(Reaction::Aggravate) => 0.20
ReactionDmgBonus(Reaction::LunarElectroCharged) => 0.20
```

For `VIRIDESCENT_VENERER`, replace `TransformativeBonus +0.60` with exact Swirl buffs:

```rust
ReactionDmgBonus(Reaction::Swirl(Element::Pyro)) => 0.60
ReactionDmgBonus(Reaction::Swirl(Element::Hydro)) => 0.60
ReactionDmgBonus(Reaction::Swirl(Element::Electro)) => 0.60
ReactionDmgBonus(Reaction::Swirl(Element::Cryo)) => 0.60
```

For `FLOWER_OF_PARADISE_LOST`, replace generic transformative buffs with exact reaction entries:

```rust
ReactionDmgBonus(Reaction::Bloom) => 0.40
ReactionDmgBonus(Reaction::Hyperbloom) => 0.40
ReactionDmgBonus(Reaction::Burgeon) => 0.40
ReactionDmgBonus(Reaction::LunarBloom) => 0.10
```

Add stack entries with `ManualCondition::Stacks(4)` and value `0.25` for Bloom, Hyperbloom, Burgeon, and LunarBloom.

Use distinct names for every conditional buff to satisfy the existing uniqueness test.

- [ ] **Step 4: Fix audited artifact descriptions/conditions**

Update the text and values for these audited discrepancies:

- `ARCHAIC_PETRA`: mention Lunar-Crystallize in the 4pc description.
- `FRAGMENT_OF_HARMONIC_WHIMSY`: condition text says Bond of Life changes, not HP changes.
- `MARTIAL_ARTIST`: condition says after Elemental Skill use, not Skill/Burst.
- `NIGHTTIME_WHISPERS_IN_THE_ECHOING_WOODS`: base Geo DMG remains `0.20`; enhanced condition grants an additional `0.30` so total reaches `0.50`.
- `OBSIDIAN_CODEX`: trigger text says on-field Nightsoul point consumption.
- `UNFINISHED_REVERIE`: condition text describes leaving combat and Burning enemy maintenance, while keeping max `DmgBonus` at `0.50`.

- [ ] **Step 5: Run artifact tests**

Run:

```sh
cargo test -p genshin-calc-data artifact_audit -- --nocapture
```

Expected: all tests pass.

- [ ] **Step 6: Commit**

```sh
git add crates/data/src/artifacts.rs crates/data/tests/data_integrity.rs
git commit -m "fix: align artifact damage buffs with audit"
```

## Task 4: Character Data Regression Tests

**Files:**
- Modify: `crates/data/tests/data_integrity.rs`

- [ ] **Step 1: Add direct character lookup helper**

Add this helper next to the artifact helper:

```rust
fn character(id: &str) -> &'static genshin_calc_data::types::CharacterData {
    genshin_calc_data::characters::all_characters()
        .copied()
        .find(|c| c.id == id)
        .unwrap_or_else(|| panic!("character not found: {id}"))
}
```

- [ ] **Step 2: Add failing tests for representative audited character problems**

Add tests:

```rust
#[test]
fn character_audit_metadata_matches_mirror() {
    let ifa = character("ifa");
    assert_eq!(ifa.weapon_type, genshin_calc_core::WeaponType::Catalyst);
    assert_eq!(ifa.element, genshin_calc_core::Element::Anemo);

    let ineffa = character("ineffa");
    assert_eq!(ineffa.weapon_type, genshin_calc_core::WeaponType::Polearm);

    let dahlia = character("dahlia");
    assert_eq!(dahlia.weapon_type, genshin_calc_core::WeaponType::Sword);
}

#[test]
fn character_audit_base_stats_are_not_scrambled() {
    let heizou = character("heizou");
    assert!(heizou.base_hp[1] < heizou.base_hp[13], "Heizou Lv20 HP must be below Lv90 HP");
    assert!(heizou.base_atk[1] < heizou.base_atk[13], "Heizou Lv20 ATK must be below Lv90 ATK");
    assert!(heizou.base_def[1] < heizou.base_def[13], "Heizou Lv20 DEF must be below Lv90 DEF");

    let kujou_sara = character("kujou_sara");
    assert!(
        kujou_sara.base_hp[1] < kujou_sara.base_hp[13],
        "Kujou Sara Lv20 HP must be below Lv90 HP"
    );

    let traveler_dendro = character("traveler_dendro");
    assert!(
        traveler_dendro.base_hp[1] < traveler_dendro.base_hp[13],
        "Traveler Dendro Lv20 HP must be below Lv90 HP"
    );
}

#[test]
fn character_audit_talent_structures_include_required_rows() {
    let aloy = character("aloy");
    assert_eq!(
        aloy.talents.normal_attack.hits.len(),
        4,
        "Aloy mirror has four normal attack rows"
    );

    let lisa = character("lisa");
    assert!(
        lisa.talents
            .elemental_skill
            .scalings
            .iter()
            .any(|s| s.name.contains("長押し") || s.name.to_ascii_lowercase().contains("hold")),
        "Lisa skill must include hold/stack scaling rows"
    );

    let kinich = character("kinich");
    assert!(
        kinich
            .talents
            .normal_attack
            .hits
            .iter()
            .any(|s| s.name.contains("空中") || s.name.to_ascii_lowercase().contains("mid-air")),
        "Kinich must include mid-air normal attack damage row"
    );
}
```

- [ ] **Step 3: Run tests to verify failure**

Run:

```sh
cargo test -p genshin-calc-data character_audit -- --nocapture
```

Expected: at least one failure from currently audited character data mismatches.

- [ ] **Step 4: Commit failing tests**

```sh
git add crates/data/tests/data_integrity.rs
git commit -m "test: capture audited character data regressions"
```

## Task 5: Character Data Fixes By Element

**Files:**
- Modify: `crates/data/src/characters/anemo/*.rs`
- Modify: `crates/data/src/characters/cryo/*.rs`
- Modify: `crates/data/src/characters/dendro/*.rs`
- Modify: `crates/data/src/characters/electro/*.rs`
- Modify: `crates/data/src/characters/geo/*.rs`
- Modify: `crates/data/src/characters/hydro/*.rs`
- Modify: `crates/data/src/characters/pyro/*.rs`

- [ ] **Step 1: Assign SubAgents by element**

Use subagent-driven execution with disjoint write scopes:

- Agent A owns `crates/data/src/characters/anemo/*.rs` and `docs/audit/anemo_audit.md`.
- Agent B owns `crates/data/src/characters/cryo/*.rs`, `crates/data/src/characters/dendro/*.rs`, `docs/audit/cryo_audit.md`, and `docs/audit/dendro_audit.md`.
- Agent C owns `crates/data/src/characters/electro/*.rs` and `docs/audit/electro_audit.md`.
- Agent D owns `crates/data/src/characters/geo/*.rs`, `crates/data/src/characters/hydro/*.rs`, `crates/data/src/characters/pyro/*.rs`, and the matching audit files.

Each agent must not edit files owned by another agent.

- [ ] **Step 2: Fix structural and metadata issues first**

Each agent repairs its audit items in this order:

1. Missing scaling rows.
2. Collapsed multi-hit rows.
3. Wrong weapon type, element, normal attack element, or ascension stat.
4. Scrambled base stat arrays.
5. Remaining numeric scaling drift.

Use the matching HoneyHunter mirror file named in the audit as the source of truth.

- [ ] **Step 3: Run focused character tests**

Run after each element slice:

```sh
cargo test -p genshin-calc-data character_audit -- --nocapture
```

Expected: failures reduce as slices are completed; after all slices, all three tests pass.

- [ ] **Step 4: Run broad data integrity tests**

Run:

```sh
cargo test -p genshin-calc-data --test data_integrity -- --nocapture
```

Expected: all data integrity tests pass.

- [ ] **Step 5: Commit**

```sh
git add crates/data/src/characters crates/data/tests/data_integrity.rs
git commit -m "fix: align audited character damage data"
```

## Task 6: Character Talent Buff Tests

**Files:**
- Modify: `crates/data/src/talent_buffs/mod.rs`

- [ ] **Step 1: Add targeted tests for audited buff classes**

Add tests in `crates/data/src/talent_buffs/mod.rs` under the existing test module:

```rust
#[test]
fn talent_buff_audit_reaction_specific_buffs_use_exact_reactions() {
    let nilou = find_talent_buffs("nilou").unwrap();
    assert!(
        nilou
            .iter()
            .any(|b| b.stat == BuffableStat::ReactionDmgBonus(Reaction::Bloom))
    );

    let fischl = find_talent_buffs("fischl").unwrap();
    assert!(
        fischl
            .iter()
            .any(|b| b.stat == BuffableStat::AtkPercent || b.stat == BuffableStat::ElementalMastery),
        "Fischl audited Hexerei damage buff should expose ATK or EM"
    );
}

#[test]
fn talent_buff_audit_targets_are_not_overbroad() {
    let sigewinne = find_talent_buffs("sigewinne").unwrap();
    assert!(
        sigewinne.iter().any(|b| {
            b.name.contains("Requires Appropriate Rest")
                && b.stat == BuffableStat::ElementalDmgBonus(Element::Hydro)
                && b.target == BuffTarget::OnlySelf
        })
    );

    let kaeya = find_talent_buffs("kaeya").unwrap();
    assert!(
        kaeya.iter().any(|b| {
            b.source == TalentBuffSource::Constellation(1)
                && b.target == BuffTarget::OnlySelf
        })
    );
}

#[test]
fn talent_buff_audit_values_match_priority_fixes() {
    let candace = find_talent_buffs("candace").unwrap();
    let burst = candace
        .iter()
        .find(|b| b.name.contains("Prayer of the Crimson Crown"))
        .expect("Candace burst normal attack buff should exist");
    let burst_scaling = burst
        .talent_scaling
        .expect("Candace burst buff should use a talent scaling table");
    assert_eq!(burst_scaling, &[0.20; 15]);

    let mona = find_talent_buffs("mona").unwrap();
    let omen = mona
        .iter()
        .find(|b| b.name.contains("Omen"))
        .expect("Mona Omen damage bonus should exist");
    let omen_scaling = omen
        .talent_scaling
        .expect("Mona Omen should use a talent scaling table");
    assert_eq!(
        omen_scaling,
        &[
            0.42, 0.44, 0.46, 0.48, 0.50, 0.52, 0.54, 0.56, 0.58, 0.60, 0.60, 0.60, 0.60,
            0.60, 0.60,
        ]
    );
}
```

Add imports to the test module if missing:

```rust
use genshin_calc_core::{BuffTarget, BuffableStat, Element, Reaction};
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```sh
cargo test -p genshin-calc-data talent_buff_audit -- --nocapture
```

Expected: failures from audited missing/wrong buff definitions.

- [ ] **Step 3: Commit failing tests**

```sh
git add crates/data/src/talent_buffs/mod.rs
git commit -m "test: capture audited talent buff discrepancies"
```

## Task 7: Character Talent Buff Fixes By Element

**Files:**
- Modify: `crates/data/src/talent_buffs/anemo.rs`
- Modify: `crates/data/src/talent_buffs/cryo.rs`
- Modify: `crates/data/src/talent_buffs/dendro.rs`
- Modify: `crates/data/src/talent_buffs/electro.rs`
- Modify: `crates/data/src/talent_buffs/geo.rs`
- Modify: `crates/data/src/talent_buffs/hydro.rs`
- Modify: `crates/data/src/talent_buffs/pyro.rs`
- Modify: `crates/data/src/talent_buffs/mod.rs`

- [ ] **Step 1: Assign SubAgents by buff file**

Use subagent-driven execution with disjoint write scopes:

- Agent A owns `crates/data/src/talent_buffs/anemo.rs` and `docs/audit/anemo_buffs_audit.md`.
- Agent B owns `crates/data/src/talent_buffs/cryo.rs` and `docs/audit/cryo_buffs_audit.md`.
- Agent C owns `crates/data/src/talent_buffs/dendro.rs`, `crates/data/src/talent_buffs/electro.rs`, and matching audit files.
- Agent D owns `crates/data/src/talent_buffs/geo.rs`, `crates/data/src/talent_buffs/hydro.rs`, `crates/data/src/talent_buffs/pyro.rs`, and matching audit files.

Each agent must use `ReactionDmgBonus` for exact reaction effects and must leave added-damage implementations intact.

- [ ] **Step 2: Fix audited bugs and high-priority missing buffs**

Within each file, repair in this order:

1. Wrong stat type.
2. Wrong target.
3. Wrong numeric value or talent scaling table.
4. Missing in-scope damage buff.
5. Misleading source/name/description where values are already correct.

When a condition cannot be represented exactly with existing `Activation`, use `Activation::Manual(ManualCondition::Toggle)` or `Stacks(n)` and encode the exact condition in the description.

- [ ] **Step 3: Run focused talent buff tests**

Run:

```sh
cargo test -p genshin-calc-data talent_buff_audit -- --nocapture
```

Expected: all targeted tests pass after all buff slices are complete.

- [ ] **Step 4: Run all talent buff module tests**

Run:

```sh
cargo test -p genshin-calc-data talent_buffs -- --nocapture
```

Expected: all talent buff tests pass.

- [ ] **Step 5: Commit**

```sh
git add crates/data/src/talent_buffs
git commit -m "fix: align audited character talent buffs"
```

## Task 8: Audit Closure Documentation

**Files:**
- Modify: `docs/audit/*.md`

- [ ] **Step 1: Mark fixed items without deleting original findings**

For each in-scope fixed finding, append a short status note under the finding:

```markdown
**Implementation status:** Fixed in this branch.
```

For findings excluded by scope, append:

```markdown
**Implementation status:** Out of scope for this pass because this is an added-damage/proc-damage or non-damage effect.
```

Do not remove the original audit evidence.

- [ ] **Step 2: Run audit closure scan**

Run:

```sh
rg -n "FAIL|ISSUE|MISSING|Discrepancy|discrepancy" docs/audit
```

Expected: remaining matches are historical audit headings or entries that now have an adjacent `Implementation status` note.

- [ ] **Step 3: Commit**

```sh
git add docs/audit
git commit -m "docs: mark damage audit findings resolved"
```

## Task 9: Final Verification

**Files:**
- No source edits unless verification exposes a concrete defect.

- [ ] **Step 1: Format**

Run:

```sh
cargo fmt
```

Expected: exits successfully.

- [ ] **Step 2: Run package tests**

Run:

```sh
cargo test -p genshin-calc-core
cargo test -p genshin-calc-data
cargo test -p genshin-calc-wasm
```

Expected: all tests pass.

- [ ] **Step 3: Run workspace tests**

Run:

```sh
cargo test --workspace
```

Expected: all tests pass.

- [ ] **Step 4: Check worktree**

Run:

```sh
git status --short
```

Expected: only intentionally untracked local files remain. If `docs/audit/` is still untracked, include it in the audit closure commit or explicitly explain why it remains untracked.

- [ ] **Step 5: Final commit if verification changed files**

If formatting or verification fixes changed files, commit them:

```sh
git add crates docs/audit
git commit -m "chore: finalize damage data verification"
```

Expected: no uncommitted implementation changes remain after the commit.
