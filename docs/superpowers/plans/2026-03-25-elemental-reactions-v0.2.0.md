# Elemental Reactions v0.2.0 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add all elemental reaction calculations (amplifying, catalyze, transformative, lunar) to the genshin-calc Rust library.

**Architecture:** New modules (reaction.rs, em.rs, level_table.rs, transformative.rs, lunar.rs) added to `crates/core`. Existing `damage.rs` extended for amplifying/catalyze. Three public calculation functions: `calculate_damage` (extended), `calculate_transformative` (new), `calculate_lunar` (new).

**Tech Stack:** Rust 1.94, serde 1, thiserror 2

**Spec:** `docs/superpowers/specs/2026-03-25-elemental-reactions-design.md`

---

## File Structure

| File | Responsibility | Action |
|------|---------------|--------|
| `crates/core/src/reaction.rs` | Reaction enum, ReactionCategory, determine_reaction, multiplier functions | Create |
| `crates/core/src/em.rs` | 4 EM bonus functions | Create |
| `crates/core/src/level_table.rs` | Level 1-100 base value table | Create |
| `crates/core/src/error.rs` | New CalcError variants | Modify |
| `crates/core/src/damage.rs` | DamageInput/Result extension, amplifying/catalyze logic | Modify |
| `crates/core/src/transformative.rs` | TransformativeInput/Result, calculate_transformative | Create |
| `crates/core/src/lunar.rs` | LunarInput/Result, calculate_lunar | Create |
| `crates/core/src/lib.rs` | New module declarations and re-exports | Modify |

---

### Task 1: reaction.rs — Reaction enum, category, and multiplier lookups

**Files:**
- Create: `crates/core/src/reaction.rs`
- Modify: `crates/core/src/lib.rs` (add `pub mod reaction;`)

- [ ] **Step 1: Create `crates/core/src/reaction.rs` with Reaction enum and category**

```rust
use serde::{Deserialize, Serialize};

use crate::types::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Reaction {
    // Amplifying
    Vaporize,
    Melt,
    // Catalyze
    Aggravate,
    Spread,
    // Transformative
    Overloaded,
    Superconduct,
    ElectroCharged,
    Shattered,
    Swirl(Element),
    Bloom,
    Hyperbloom,
    Burgeon,
    Burning,
    // Lunar
    LunarElectroCharged,
    LunarBloom,
    LunarCrystallize,
    LunarCrystallizeSecondary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReactionCategory {
    Amplifying,
    Catalyze,
    Transformative,
    Lunar,
}

impl Reaction {
    pub fn category(&self) -> ReactionCategory {
        match self {
            Reaction::Vaporize | Reaction::Melt => ReactionCategory::Amplifying,
            Reaction::Aggravate | Reaction::Spread => ReactionCategory::Catalyze,
            Reaction::Overloaded
            | Reaction::Superconduct
            | Reaction::ElectroCharged
            | Reaction::Shattered
            | Reaction::Swirl(_)
            | Reaction::Bloom
            | Reaction::Hyperbloom
            | Reaction::Burgeon
            | Reaction::Burning => ReactionCategory::Transformative,
            Reaction::LunarElectroCharged
            | Reaction::LunarBloom
            | Reaction::LunarCrystallize
            | Reaction::LunarCrystallizeSecondary => ReactionCategory::Lunar,
        }
    }
}

pub fn determine_reaction(trigger: Element, aura: Element) -> Option<Reaction> {
    match (trigger, aura) {
        // Vaporize
        (Element::Pyro, Element::Hydro) => Some(Reaction::Vaporize),
        (Element::Hydro, Element::Pyro) => Some(Reaction::Vaporize),
        // Melt
        (Element::Pyro, Element::Cryo) => Some(Reaction::Melt),
        (Element::Cryo, Element::Pyro) => Some(Reaction::Melt),
        // Overloaded
        (Element::Pyro, Element::Electro) | (Element::Electro, Element::Pyro) => {
            Some(Reaction::Overloaded)
        }
        // Superconduct
        (Element::Cryo, Element::Electro) | (Element::Electro, Element::Cryo) => {
            Some(Reaction::Superconduct)
        }
        // Electro-Charged
        (Element::Hydro, Element::Electro) | (Element::Electro, Element::Hydro) => {
            Some(Reaction::ElectroCharged)
        }
        // Swirl
        (Element::Anemo, aura @ (Element::Pyro | Element::Hydro | Element::Electro | Element::Cryo)) => {
            Some(Reaction::Swirl(aura))
        }
        // Bloom
        (Element::Hydro, Element::Dendro) | (Element::Dendro, Element::Hydro) => {
            Some(Reaction::Bloom)
        }
        // Burning
        (Element::Pyro, Element::Dendro) | (Element::Dendro, Element::Pyro) => {
            Some(Reaction::Burning)
        }
        // Shattered is triggered by heavy attacks on frozen enemies, not by element combination
        // Hyperbloom/Burgeon are triggered on Bloom cores, not by direct element combination
        // Catalyze (Aggravate/Spread) requires Quicken state, not representable here
        // Lunar reactions have special triggers not representable here
        _ => None,
    }
}

pub fn amplifying_multiplier(trigger: Element, aura: Element) -> Option<f64> {
    match (trigger, aura) {
        (Element::Pyro, Element::Hydro) => Some(1.5),   // Reverse Vaporize
        (Element::Hydro, Element::Pyro) => Some(2.0),   // Forward Vaporize
        (Element::Pyro, Element::Cryo) => Some(2.0),    // Forward Melt
        (Element::Cryo, Element::Pyro) => Some(1.5),    // Reverse Melt
        _ => None,
    }
}

pub fn catalyze_coefficient(reaction: Reaction) -> Option<f64> {
    match reaction {
        Reaction::Aggravate => Some(1.15),
        Reaction::Spread => Some(1.25),
        _ => None,
    }
}

pub fn transformative_multiplier(reaction: Reaction) -> Option<f64> {
    match reaction {
        Reaction::Overloaded => Some(2.75),
        Reaction::Superconduct => Some(1.5),
        Reaction::ElectroCharged => Some(2.0),
        Reaction::Shattered => Some(3.0),
        Reaction::Swirl(_) => Some(0.6),
        Reaction::Bloom => Some(2.0),
        Reaction::Hyperbloom => Some(3.0),
        Reaction::Burgeon => Some(3.0),
        Reaction::Burning => Some(0.25),
        _ => None,
    }
}

pub fn lunar_multiplier(reaction: Reaction) -> Option<f64> {
    match reaction {
        Reaction::LunarElectroCharged => Some(1.8),
        Reaction::LunarBloom => Some(1.0),
        Reaction::LunarCrystallize => Some(0.96),
        Reaction::LunarCrystallizeSecondary => Some(1.6),
        _ => None,
    }
}

pub fn transformative_element(reaction: Reaction) -> Option<Option<Element>> {
    match reaction {
        Reaction::Overloaded => Some(Some(Element::Pyro)),
        Reaction::Superconduct => Some(Some(Element::Cryo)),
        Reaction::ElectroCharged => Some(Some(Element::Electro)),
        Reaction::Shattered => Some(None), // Physical
        Reaction::Swirl(elem) => Some(Some(elem)),
        Reaction::Bloom | Reaction::Hyperbloom | Reaction::Burgeon => Some(Some(Element::Dendro)),
        Reaction::Burning => Some(Some(Element::Pyro)),
        _ => None,
    }
}

pub fn lunar_element(reaction: Reaction) -> Option<Option<Element>> {
    match reaction {
        Reaction::LunarElectroCharged => Some(Some(Element::Electro)),
        Reaction::LunarBloom => Some(Some(Element::Dendro)),
        Reaction::LunarCrystallize | Reaction::LunarCrystallizeSecondary => Some(Some(Element::Geo)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_amplifying() {
        assert_eq!(Reaction::Vaporize.category(), ReactionCategory::Amplifying);
        assert_eq!(Reaction::Melt.category(), ReactionCategory::Amplifying);
    }

    #[test]
    fn test_category_catalyze() {
        assert_eq!(Reaction::Aggravate.category(), ReactionCategory::Catalyze);
        assert_eq!(Reaction::Spread.category(), ReactionCategory::Catalyze);
    }

    #[test]
    fn test_category_transformative() {
        assert_eq!(Reaction::Overloaded.category(), ReactionCategory::Transformative);
        assert_eq!(Reaction::Swirl(Element::Pyro).category(), ReactionCategory::Transformative);
    }

    #[test]
    fn test_category_lunar() {
        assert_eq!(Reaction::LunarElectroCharged.category(), ReactionCategory::Lunar);
        assert_eq!(Reaction::LunarBloom.category(), ReactionCategory::Lunar);
    }

    #[test]
    fn test_determine_vaporize() {
        assert_eq!(determine_reaction(Element::Pyro, Element::Hydro), Some(Reaction::Vaporize));
        assert_eq!(determine_reaction(Element::Hydro, Element::Pyro), Some(Reaction::Vaporize));
    }

    #[test]
    fn test_determine_melt() {
        assert_eq!(determine_reaction(Element::Pyro, Element::Cryo), Some(Reaction::Melt));
        assert_eq!(determine_reaction(Element::Cryo, Element::Pyro), Some(Reaction::Melt));
    }

    #[test]
    fn test_determine_overloaded() {
        assert_eq!(determine_reaction(Element::Pyro, Element::Electro), Some(Reaction::Overloaded));
        assert_eq!(determine_reaction(Element::Electro, Element::Pyro), Some(Reaction::Overloaded));
    }

    #[test]
    fn test_determine_superconduct() {
        assert_eq!(determine_reaction(Element::Cryo, Element::Electro), Some(Reaction::Superconduct));
    }

    #[test]
    fn test_determine_electro_charged() {
        assert_eq!(determine_reaction(Element::Hydro, Element::Electro), Some(Reaction::ElectroCharged));
    }

    #[test]
    fn test_determine_swirl() {
        assert_eq!(determine_reaction(Element::Anemo, Element::Pyro), Some(Reaction::Swirl(Element::Pyro)));
        assert_eq!(determine_reaction(Element::Anemo, Element::Hydro), Some(Reaction::Swirl(Element::Hydro)));
        assert_eq!(determine_reaction(Element::Anemo, Element::Electro), Some(Reaction::Swirl(Element::Electro)));
        assert_eq!(determine_reaction(Element::Anemo, Element::Cryo), Some(Reaction::Swirl(Element::Cryo)));
    }

    #[test]
    fn test_determine_swirl_invalid_elements() {
        assert_eq!(determine_reaction(Element::Anemo, Element::Dendro), None);
        assert_eq!(determine_reaction(Element::Anemo, Element::Geo), None);
        assert_eq!(determine_reaction(Element::Anemo, Element::Anemo), None);
    }

    #[test]
    fn test_determine_bloom() {
        assert_eq!(determine_reaction(Element::Hydro, Element::Dendro), Some(Reaction::Bloom));
        assert_eq!(determine_reaction(Element::Dendro, Element::Hydro), Some(Reaction::Bloom));
    }

    #[test]
    fn test_determine_burning() {
        assert_eq!(determine_reaction(Element::Pyro, Element::Dendro), Some(Reaction::Burning));
    }

    #[test]
    fn test_determine_no_reaction() {
        assert_eq!(determine_reaction(Element::Geo, Element::Geo), None);
        assert_eq!(determine_reaction(Element::Hydro, Element::Hydro), None);
    }

    #[test]
    fn test_amplifying_multiplier_vaporize() {
        assert_eq!(amplifying_multiplier(Element::Pyro, Element::Hydro), Some(1.5));
        assert_eq!(amplifying_multiplier(Element::Hydro, Element::Pyro), Some(2.0));
    }

    #[test]
    fn test_amplifying_multiplier_melt() {
        assert_eq!(amplifying_multiplier(Element::Pyro, Element::Cryo), Some(2.0));
        assert_eq!(amplifying_multiplier(Element::Cryo, Element::Pyro), Some(1.5));
    }

    #[test]
    fn test_amplifying_multiplier_non_amplifying() {
        assert_eq!(amplifying_multiplier(Element::Pyro, Element::Electro), None);
    }

    #[test]
    fn test_catalyze_coefficient_values() {
        assert_eq!(catalyze_coefficient(Reaction::Aggravate), Some(1.15));
        assert_eq!(catalyze_coefficient(Reaction::Spread), Some(1.25));
        assert_eq!(catalyze_coefficient(Reaction::Vaporize), None);
    }

    #[test]
    fn test_transformative_multiplier_values() {
        assert_eq!(transformative_multiplier(Reaction::Overloaded), Some(2.75));
        assert_eq!(transformative_multiplier(Reaction::Superconduct), Some(1.5));
        assert_eq!(transformative_multiplier(Reaction::ElectroCharged), Some(2.0));
        assert_eq!(transformative_multiplier(Reaction::Shattered), Some(3.0));
        assert_eq!(transformative_multiplier(Reaction::Swirl(Element::Pyro)), Some(0.6));
        assert_eq!(transformative_multiplier(Reaction::Bloom), Some(2.0));
        assert_eq!(transformative_multiplier(Reaction::Hyperbloom), Some(3.0));
        assert_eq!(transformative_multiplier(Reaction::Burgeon), Some(3.0));
        assert_eq!(transformative_multiplier(Reaction::Burning), Some(0.25));
    }

    #[test]
    fn test_lunar_multiplier_values() {
        assert_eq!(lunar_multiplier(Reaction::LunarElectroCharged), Some(1.8));
        assert_eq!(lunar_multiplier(Reaction::LunarBloom), Some(1.0));
        assert_eq!(lunar_multiplier(Reaction::LunarCrystallize), Some(0.96));
        assert_eq!(lunar_multiplier(Reaction::LunarCrystallizeSecondary), Some(1.6));
    }

    #[test]
    fn test_transformative_element_values() {
        assert_eq!(transformative_element(Reaction::Overloaded), Some(Some(Element::Pyro)));
        assert_eq!(transformative_element(Reaction::Superconduct), Some(Some(Element::Cryo)));
        assert_eq!(transformative_element(Reaction::Shattered), Some(None));
        assert_eq!(transformative_element(Reaction::Swirl(Element::Hydro)), Some(Some(Element::Hydro)));
        assert_eq!(transformative_element(Reaction::Bloom), Some(Some(Element::Dendro)));
        assert_eq!(transformative_element(Reaction::Vaporize), None);
    }
}
```

- [ ] **Step 2: Add `pub mod reaction;` to lib.rs**

- [ ] **Step 3: Verify it compiles and tests pass**

Run: `cargo test -p genshin-calc-core`
Expected: All new reaction tests pass, existing 33 tests still pass

- [ ] **Step 4: Run `cargo fmt`**

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/reaction.rs crates/core/src/lib.rs
git commit -m "feat: add Reaction enum, determine_reaction, and multiplier lookups"
```

---

### Task 2: em.rs — Elemental Mastery bonus functions

**Files:**
- Create: `crates/core/src/em.rs`
- Modify: `crates/core/src/lib.rs` (add `pub mod em;`)

- [ ] **Step 1: Create `crates/core/src/em.rs`**

```rust
pub fn amplifying_em_bonus(em: f64) -> f64 {
    2.78 * em / (em + 1400.0)
}

pub fn catalyze_em_bonus(em: f64) -> f64 {
    5.0 * em / (em + 1200.0)
}

pub fn transformative_em_bonus(em: f64) -> f64 {
    16.0 * em / (em + 2000.0)
}

pub fn lunar_em_bonus(em: f64) -> f64 {
    6.0 * em / (em + 2000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_amplifying_em_zero() {
        assert!((amplifying_em_bonus(0.0) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_amplifying_em_200() {
        // 2.78 * 200 / (200 + 1400) = 556 / 1600 = 0.3475
        assert!((amplifying_em_bonus(200.0) - 0.3475).abs() < EPSILON);
    }

    #[test]
    fn test_amplifying_em_1000() {
        // 2.78 * 1000 / (1000 + 1400) = 2780 / 2400 = 1.158333...
        assert!((amplifying_em_bonus(1000.0) - 2780.0 / 2400.0).abs() < EPSILON);
    }

    #[test]
    fn test_catalyze_em_zero() {
        assert!((catalyze_em_bonus(0.0) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_catalyze_em_200() {
        // 5.0 * 200 / (200 + 1200) = 1000 / 1400 = 0.714285...
        assert!((catalyze_em_bonus(200.0) - 1000.0 / 1400.0).abs() < EPSILON);
    }

    #[test]
    fn test_transformative_em_zero() {
        assert!((transformative_em_bonus(0.0) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_transformative_em_800() {
        // 16.0 * 800 / (800 + 2000) = 12800 / 2800 = 4.571428...
        assert!((transformative_em_bonus(800.0) - 12800.0 / 2800.0).abs() < EPSILON);
    }

    #[test]
    fn test_lunar_em_zero() {
        assert!((lunar_em_bonus(0.0) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_lunar_em_300() {
        // 6.0 * 300 / (300 + 2000) = 1800 / 2300 = 0.782608...
        assert!((lunar_em_bonus(300.0) - 1800.0 / 2300.0).abs() < EPSILON);
    }
}
```

- [ ] **Step 2: Add `pub mod em;` to lib.rs**

- [ ] **Step 3: Verify tests pass**

Run: `cargo test -p genshin-calc-core`

- [ ] **Step 4: Run `cargo fmt` and commit**

```bash
git add crates/core/src/em.rs crates/core/src/lib.rs
git commit -m "feat: add elemental mastery bonus functions"
```

---

### Task 3: level_table.rs — Level base value table

**Files:**
- Create: `crates/core/src/level_table.rs`
- Modify: `crates/core/src/lib.rs` (add `pub mod level_table;`)

- [ ] **Step 1: Create `crates/core/src/level_table.rs`**

```rust
const LEVEL_TABLE: [f64; 100] = [
    17.165606,   18.535048,   19.904854,   21.274902,   22.6454,
    24.649612,   26.640642,   28.868587,   31.36768,    34.143345,
    37.201,      40.66,       44.446667,   48.56352,    53.74848,
    59.081898,   64.420044,   69.72446,    75.12314,    80.58478,
    86.11203,    91.70374,    97.24463,    102.812645,  108.40956,
    113.20169,   118.102905,  122.97932,   129.72733,   136.29291,
    142.67085,   149.02902,   155.41699,   161.8255,    169.10631,
    176.51808,   184.07274,   191.70952,   199.55692,   207.38205,
    215.3989,    224.16566,   233.50217,   243.35057,   256.06308,
    268.5435,    281.52606,   295.01364,   309.0672,    323.6016,
    336.75754,   350.5303,    364.4827,    378.61917,   398.6004,
    416.39825,   434.387,     452.95105,   472.60623,   492.8849,
    513.56854,   539.1032,    565.51056,   592.53876,   624.4434,
    651.47015,   679.4968,    707.79407,   736.67145,   765.64026,
    794.7734,    824.67737,   851.1578,    877.74207,   914.2291,
    946.74677,   979.4114,    1011.223,    1044.7917,   1077.4437,
    1109.9976,   1142.9766,   1176.3695,   1210.1844,   1253.8357,
    1288.9528,   1325.4841,   1363.4569,   1405.0974,   1446.8535,
    1462.788,    1475.6956,   1497.9644,   1516.9423,   1561.468,
    1593.5062,   1621.0258,   1643.8679,   1662.1382,   1674.8092,
];

pub fn reaction_base_value(level: u32) -> Option<f64> {
    if level == 0 || level > 100 {
        return None;
    }
    Some(LEVEL_TABLE[(level - 1) as usize])
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_level_1() {
        assert!((reaction_base_value(1).unwrap() - 17.165606).abs() < EPSILON);
    }

    #[test]
    fn test_level_90() {
        assert!((reaction_base_value(90).unwrap() - 1446.8535).abs() < EPSILON);
    }

    #[test]
    fn test_level_100() {
        assert!((reaction_base_value(100).unwrap() - 1674.8092).abs() < EPSILON);
    }

    #[test]
    fn test_level_0_invalid() {
        assert_eq!(reaction_base_value(0), None);
    }

    #[test]
    fn test_level_101_invalid() {
        assert_eq!(reaction_base_value(101), None);
    }
}
```

- [ ] **Step 2: Add `pub mod level_table;` to lib.rs**

- [ ] **Step 3: Verify tests pass**

- [ ] **Step 4: Run `cargo fmt` and commit**

```bash
git add crates/core/src/level_table.rs crates/core/src/lib.rs
git commit -m "feat: add level-based reaction base value table (Lv1-100)"
```

---

### Task 4: error.rs — Add new CalcError variants

**Files:**
- Modify: `crates/core/src/error.rs`

- [ ] **Step 1: Update error.rs with new variants**

Replace contents of `crates/core/src/error.rs`:

```rust
use crate::reaction::Reaction;
use crate::types::Element;

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CalcError {
    #[error("character level must be 1..=90, got {0}")]
    InvalidCharacterLevel(u32),

    #[error("enemy level must be 1..=100, got {0}")]
    InvalidEnemyLevel(u32),

    #[error("crit_rate must be 0.0..=1.0, got {0}")]
    InvalidCritRate(f64),

    #[error("def_reduction must be 0.0..=1.0, got {0}")]
    InvalidDefReduction(f64),

    #[error("talent_multiplier must be > 0.0, got {0}")]
    InvalidTalentMultiplier(f64),

    #[error("amplifying reaction requires an element, but element is None (physical)")]
    AmplifyingRequiresElement,

    #[error("elemental_mastery must be >= 0.0, got {0}")]
    InvalidElementalMastery(f64),

    #[error("reaction_bonus must be >= 0.0, got {0}")]
    InvalidReactionBonus(f64),

    #[error("reaction {0:?} is not amplifying or catalyze; use calculate_transformative")]
    UseTransformativeFunction(Reaction),

    #[error("reaction {0:?} is not amplifying or catalyze; use calculate_lunar")]
    UseLunarFunction(Reaction),

    #[error("reaction {0:?} is not a transformative reaction")]
    NotTransformative(Reaction),

    #[error("reaction {0:?} is not a lunar reaction")]
    NotLunar(Reaction),

    #[error("swirl element must be Pyro, Hydro, Electro, or Cryo, got {0:?}")]
    InvalidSwirlElement(Element),

    #[error("invalid amplifying combination: {0:?} with {1:?} trigger")]
    InvalidAmplifyingCombination(Reaction, Element),

    #[error("character level must be 1..=100 for reaction calculations, got {0}")]
    InvalidReactionLevel(u32),
}
```

- [ ] **Step 2: Verify it compiles**

Run: `cargo check -p genshin-calc-core`

Note: There may be a circular dependency issue since error.rs imports from reaction.rs and reaction.rs might need error.rs. In this design, reaction.rs does NOT import error.rs (it returns Option, not Result), so error.rs importing reaction.rs is fine.

- [ ] **Step 3: Run `cargo fmt` and commit**

```bash
git add crates/core/src/error.rs
git commit -m "feat: add new CalcError variants for reaction validation"
```

---

### Task 5: damage.rs — Breaking changes + amplifying/catalyze reactions

**Files:**
- Modify: `crates/core/src/damage.rs`

This is the most complex task. It modifies DamageInput, DamageResult, and the calculate_damage logic.

- [ ] **Step 1: Update DamageInput and DamageResult structs**

Add new fields and imports. Update `DamageInput`:

```rust
use crate::reaction::{
    amplifying_multiplier, catalyze_coefficient, Reaction, ReactionCategory,
};
use crate::em::{amplifying_em_bonus, catalyze_em_bonus};
use crate::level_table::reaction_base_value;
```

Add to `DamageInput`:
```rust
    pub reaction: Option<Reaction>,
    pub reaction_bonus: f64,
```

Add to `DamageResult`:
```rust
    pub reaction: Option<Reaction>,
```

- [ ] **Step 2: Update all existing tests to include new fields**

Every `DamageInput` construction needs `reaction: None, reaction_bonus: 0.0`.
Every `DamageResult` assertion needs to account for `reaction: None`.

For the `valid_input()` helper:
```rust
    fn valid_input() -> DamageInput {
        DamageInput {
            character_level: 90,
            stats: Stats {
                atk: 2000.0,
                crit_rate: 0.5,
                crit_dmg: 1.0,
                dmg_bonus: 0.466,
                ..Stats::default()
            },
            talent_multiplier: 1.76,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
        }
    }
```

- [ ] **Step 3: Run existing tests — they should all pass**

Run: `cargo test -p genshin-calc-core`
Expected: All 33+ existing tests pass (backward compatibility confirmed)

- [ ] **Step 4: Implement reaction logic in calculate_damage**

Also make `resistance_multiplier` pub(crate) (needed by transformative.rs and lunar.rs later):

```rust
pub(crate) fn resistance_multiplier(enemy: &Enemy) -> f64 {
```

Remove the old `base_damage` helper function (its logic is now inlined in `calculate_damage` to support catalyze flat additive).

Add `reaction_bonus` validation to the `validate` function:

```rust
    if input.reaction.is_some() && input.reaction_bonus < 0.0 {
        return Err(CalcError::InvalidReactionBonus(input.reaction_bonus));
    }
```

Update `calculate_damage`:

```rust
pub fn calculate_damage(input: &DamageInput, enemy: &Enemy) -> Result<DamageResult, CalcError> {
    validate(input, enemy)?;

    let mut catalyze_flat = 0.0;
    let mut amplify_mult = 1.0;
    let mut reaction_result = None;

    if let Some(reaction) = input.reaction {
        match reaction.category() {
            ReactionCategory::Amplifying => {
                let trigger = input.element.ok_or(CalcError::AmplifyingRequiresElement)?;
                let base_mult = match (reaction, trigger) {
                    (Reaction::Vaporize, Element::Pyro) => 1.5,
                    (Reaction::Vaporize, Element::Hydro) => 2.0,
                    (Reaction::Melt, Element::Pyro) => 2.0,
                    (Reaction::Melt, Element::Cryo) => 1.5,
                    _ => return Err(CalcError::InvalidAmplifyingCombination(reaction, trigger)),
                };
                let em_bonus = amplifying_em_bonus(input.stats.elemental_mastery);
                amplify_mult = base_mult * (1.0 + em_bonus + input.reaction_bonus);
                reaction_result = Some(reaction);
            }
            ReactionCategory::Catalyze => {
                let coeff = catalyze_coefficient(reaction).unwrap();
                let em_bonus = catalyze_em_bonus(input.stats.elemental_mastery);
                let level_base = reaction_base_value(input.character_level).unwrap();
                catalyze_flat = coeff * level_base * (1.0 + em_bonus + input.reaction_bonus);
                reaction_result = Some(reaction);
            }
            ReactionCategory::Transformative => {
                return Err(CalcError::UseTransformativeFunction(reaction));
            }
            ReactionCategory::Lunar => {
                return Err(CalcError::UseLunarFunction(reaction));
            }
        }
    }

    let base = input.stats.atk * input.talent_multiplier + catalyze_flat;
    let non_crit = base * (1.0 + input.stats.dmg_bonus)
        * defense_multiplier(input.character_level, enemy)
        * resistance_multiplier(enemy)
        * amplify_mult;
    let crit = non_crit * (1.0 + input.stats.crit_dmg);
    let average = non_crit * (1.0 - input.stats.crit_rate) + crit * input.stats.crit_rate;

    Ok(DamageResult {
        non_crit,
        crit,
        average,
        reaction: reaction_result,
    })
}
```

- [ ] **Step 5: Add amplifying reaction tests**

```rust
    #[test]
    fn test_vaporize_pyro_on_hydro() {
        // Pyro trigger = 1.5x, EM=200 → em_bonus = 0.3475, reaction_bonus = 0.15
        // amplify = 1.5 * (1 + 0.3475 + 0.15) = 1.5 * 1.4975 = 2.24625
        let input = DamageInput {
            stats: Stats {
                atk: 1800.0, crit_rate: 0.6, crit_dmg: 1.2,
                dmg_bonus: 0.466, elemental_mastery: 200.0, ..Stats::default()
            },
            talent_multiplier: 1.5104,
            element: Some(Element::Pyro),
            reaction: Some(Reaction::Vaporize),
            reaction_bonus: 0.15,
            ..valid_input()
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        assert_eq!(result.reaction, Some(Reaction::Vaporize));
        // Verify amplified damage > non-amplified
        let no_reaction = calculate_damage(&DamageInput { reaction: None, reaction_bonus: 0.0, ..input.clone() }, &enemy).unwrap();
        assert!(result.non_crit > no_reaction.non_crit * 1.4);
    }

    #[test]
    fn test_vaporize_hydro_on_pyro() {
        // Hydro trigger = 2.0x
        let input = DamageInput {
            element: Some(Element::Hydro),
            reaction: Some(Reaction::Vaporize),
            reaction_bonus: 0.0,
            ..valid_input()
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        let no_reaction = calculate_damage(&DamageInput { reaction: None, ..input.clone() }, &enemy).unwrap();
        assert!((result.non_crit / no_reaction.non_crit - 2.0).abs() < 0.01);
    }

    #[test]
    fn test_melt_pyro_on_cryo() {
        // Pyro trigger Melt = 2.0x
        let input = DamageInput {
            element: Some(Element::Pyro),
            reaction: Some(Reaction::Melt),
            reaction_bonus: 0.0,
            ..valid_input()
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        let no_reaction = calculate_damage(&DamageInput { reaction: None, ..input.clone() }, &enemy).unwrap();
        assert!((result.non_crit / no_reaction.non_crit - 2.0).abs() < 0.01);
    }
```

- [ ] **Step 6: Add catalyze reaction tests**

```rust
    #[test]
    fn test_aggravate_adds_flat_damage() {
        let input = DamageInput {
            element: Some(Element::Electro),
            reaction: Some(Reaction::Aggravate),
            reaction_bonus: 0.0,
            ..valid_input()
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        let no_reaction = calculate_damage(&DamageInput { reaction: None, ..input.clone() }, &enemy).unwrap();
        // Aggravate should add flat damage, making result higher
        assert!(result.non_crit > no_reaction.non_crit);
        assert_eq!(result.reaction, Some(Reaction::Aggravate));
    }

    #[test]
    fn test_spread_adds_flat_damage() {
        let input = DamageInput {
            element: Some(Element::Dendro),
            reaction: Some(Reaction::Spread),
            reaction_bonus: 0.0,
            ..valid_input()
        };
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        let no_reaction = calculate_damage(&DamageInput { reaction: None, ..input.clone() }, &enemy).unwrap();
        assert!(result.non_crit > no_reaction.non_crit);
    }
```

- [ ] **Step 7: Add error tests for wrong reaction category**

```rust
    #[test]
    fn test_transformative_in_calculate_damage_errors() {
        let input = DamageInput {
            reaction: Some(Reaction::Overloaded),
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert!(matches!(result, Err(CalcError::UseTransformativeFunction(_))));
    }

    #[test]
    fn test_lunar_in_calculate_damage_errors() {
        let input = DamageInput {
            reaction: Some(Reaction::LunarElectroCharged),
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert!(matches!(result, Err(CalcError::UseLunarFunction(_))));
    }

    #[test]
    fn test_amplifying_without_element_errors() {
        let input = DamageInput {
            element: None,
            reaction: Some(Reaction::Vaporize),
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert!(matches!(result, Err(CalcError::AmplifyingRequiresElement)));
    }
```

- [ ] **Step 8: Update lib.rs integration test**

Add `reaction: None, reaction_bonus: 0.0` to the test in lib.rs.

- [ ] **Step 9: Run ALL tests**

Run: `cargo test -p genshin-calc-core`
Expected: All tests pass including backward compatibility

- [ ] **Step 10: Run `cargo fmt` and `cargo clippy -- -D warnings`**

- [ ] **Step 11: Commit**

```bash
git add crates/core/src/damage.rs crates/core/src/lib.rs
git commit -m "feat: extend calculate_damage with amplifying and catalyze reactions"
```

---

### Task 6: transformative.rs — Fixed reaction damage calculation

**Files:**
- Create: `crates/core/src/transformative.rs`
- Modify: `crates/core/src/lib.rs` (add `pub mod transformative;`)

- [ ] **Step 1: Create `crates/core/src/transformative.rs`**

```rust
use serde::{Deserialize, Serialize};

use crate::damage::resistance_multiplier;
use crate::em::transformative_em_bonus;
use crate::enemy::Enemy;
use crate::error::CalcError;
use crate::level_table::reaction_base_value;
use crate::reaction::{
    transformative_element, transformative_multiplier, Reaction, ReactionCategory,
};
use crate::types::Element;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformativeInput {
    pub character_level: u32,
    pub elemental_mastery: f64,
    pub reaction: Reaction,
    pub reaction_bonus: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformativeResult {
    pub damage: f64,
    pub damage_element: Option<Element>,
}

fn validate(input: &TransformativeInput, enemy: &Enemy) -> Result<(), CalcError> {
    if !(1..=100).contains(&input.character_level) {
        return Err(CalcError::InvalidReactionLevel(input.character_level));
    }
    if !(1..=100).contains(&enemy.level) {
        return Err(CalcError::InvalidEnemyLevel(enemy.level));
    }
    if input.elemental_mastery < 0.0 {
        return Err(CalcError::InvalidElementalMastery(input.elemental_mastery));
    }
    if input.reaction_bonus < 0.0 {
        return Err(CalcError::InvalidReactionBonus(input.reaction_bonus));
    }
    if input.reaction.category() != ReactionCategory::Transformative {
        return Err(CalcError::NotTransformative(input.reaction));
    }
    if let Reaction::Swirl(elem) = input.reaction {
        match elem {
            Element::Pyro | Element::Hydro | Element::Electro | Element::Cryo => {}
            _ => return Err(CalcError::InvalidSwirlElement(elem)),
        }
    }
    Ok(())
}

pub fn calculate_transformative(
    input: &TransformativeInput,
    enemy: &Enemy,
) -> Result<TransformativeResult, CalcError> {
    validate(input, enemy)?;

    let level_base = reaction_base_value(input.character_level).unwrap();
    let reaction_mult = transformative_multiplier(input.reaction).unwrap();
    let em_bonus = transformative_em_bonus(input.elemental_mastery);
    let res_mult = resistance_multiplier(enemy);
    let damage_elem = transformative_element(input.reaction).unwrap();

    let damage = level_base * reaction_mult * (1.0 + em_bonus + input.reaction_bonus) * res_mult;

    Ok(TransformativeResult {
        damage,
        damage_element: damage_elem,
    })
}
```

Note: `resistance_multiplier` was already made `pub(crate)` in Task 5.

- [ ] **Step 2: Add tests**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    fn default_enemy() -> Enemy {
        Enemy { level: 90, resistance: 0.1, def_reduction: 0.0 }
    }

    #[test]
    fn test_overloaded_lv90_no_em() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
        };
        // 1446.8535 * 2.75 * 1.0 * 0.9 = 3577.9122...
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 1446.8535 * 2.75 * 0.9).abs() < 0.01);
        assert_eq!(result.damage_element, Some(Element::Pyro));
    }

    #[test]
    fn test_overloaded_lv90_em800() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 800.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
        };
        let em_bonus = 16.0 * 800.0 / (800.0 + 2000.0);
        let expected = 1446.8535 * 2.75 * (1.0 + em_bonus) * 0.9;
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - expected).abs() < 0.01);
    }

    #[test]
    fn test_superconduct() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Superconduct,
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 1446.8535 * 1.5 * 0.9).abs() < 0.01);
        assert_eq!(result.damage_element, Some(Element::Cryo));
    }

    #[test]
    fn test_swirl_pyro() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Swirl(Element::Pyro),
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 1446.8535 * 0.6 * 0.9).abs() < 0.01);
        assert_eq!(result.damage_element, Some(Element::Pyro));
    }

    #[test]
    fn test_swirl_invalid_element() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Swirl(Element::Dendro),
            reaction_bonus: 0.0,
        };
        assert!(matches!(
            calculate_transformative(&input, &default_enemy()),
            Err(CalcError::InvalidSwirlElement(Element::Dendro))
        ));
    }

    #[test]
    fn test_shattered_physical() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Shattered,
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert_eq!(result.damage_element, None);
    }

    #[test]
    fn test_bloom() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Bloom,
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy()).unwrap();
        assert!((result.damage - 1446.8535 * 2.0 * 0.9).abs() < 0.01);
        assert_eq!(result.damage_element, Some(Element::Dendro));
    }

    #[test]
    fn test_not_transformative_error() {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Vaporize,
            reaction_bonus: 0.0,
        };
        assert!(matches!(
            calculate_transformative(&input, &default_enemy()),
            Err(CalcError::NotTransformative(_))
        ));
    }

    #[test]
    fn test_level_100_valid() {
        let input = TransformativeInput {
            character_level: 100,
            elemental_mastery: 0.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
        };
        let result = calculate_transformative(&input, &default_enemy());
        assert!(result.is_ok());
    }

    #[test]
    fn test_reaction_bonus_applied() {
        let base = TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
        };
        let with_bonus = TransformativeInput {
            reaction_bonus: 0.4,
            ..base.clone()
        };
        let r1 = calculate_transformative(&base, &default_enemy()).unwrap();
        let r2 = calculate_transformative(&with_bonus, &default_enemy()).unwrap();
        assert!((r2.damage / r1.damage - 1.4).abs() < EPSILON);
    }
}
```

- [ ] **Step 3: Add `pub mod transformative;` to lib.rs**

- [ ] **Step 4: Run all tests**

Run: `cargo test -p genshin-calc-core`

- [ ] **Step 5: Run `cargo fmt` and `cargo clippy -- -D warnings`**

- [ ] **Step 6: Commit**

```bash
git add crates/core/src/transformative.rs crates/core/src/lib.rs
git commit -m "feat: add calculate_transformative for fixed reaction damage"
```

---

### Task 7: lunar.rs — Lunar reaction damage calculation

**Files:**
- Create: `crates/core/src/lunar.rs`
- Modify: `crates/core/src/lib.rs` (add `pub mod lunar;`)

- [ ] **Step 1: Create `crates/core/src/lunar.rs`**

```rust
use serde::{Deserialize, Serialize};

use crate::damage::resistance_multiplier;
use crate::em::lunar_em_bonus;
use crate::enemy::Enemy;
use crate::error::CalcError;
use crate::level_table::reaction_base_value;
use crate::reaction::{lunar_element, lunar_multiplier, Reaction, ReactionCategory};
use crate::types::Element;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LunarInput {
    pub character_level: u32,
    pub elemental_mastery: f64,
    pub reaction: Reaction,
    pub reaction_bonus: f64,
    pub crit_rate: f64,
    pub crit_dmg: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LunarResult {
    pub non_crit: f64,
    pub crit: f64,
    pub average: f64,
    pub damage_element: Option<Element>,
}

fn validate(input: &LunarInput, enemy: &Enemy) -> Result<(), CalcError> {
    if !(1..=100).contains(&input.character_level) {
        return Err(CalcError::InvalidReactionLevel(input.character_level));
    }
    if !(1..=100).contains(&enemy.level) {
        return Err(CalcError::InvalidEnemyLevel(enemy.level));
    }
    if input.elemental_mastery < 0.0 {
        return Err(CalcError::InvalidElementalMastery(input.elemental_mastery));
    }
    if input.reaction_bonus < 0.0 {
        return Err(CalcError::InvalidReactionBonus(input.reaction_bonus));
    }
    if !(0.0..=1.0).contains(&input.crit_rate) {
        return Err(CalcError::InvalidCritRate(input.crit_rate));
    }
    if input.reaction.category() != ReactionCategory::Lunar {
        return Err(CalcError::NotLunar(input.reaction));
    }
    Ok(())
}

pub fn calculate_lunar(
    input: &LunarInput,
    enemy: &Enemy,
) -> Result<LunarResult, CalcError> {
    validate(input, enemy)?;

    let level_base = reaction_base_value(input.character_level).unwrap();
    let reaction_mult = lunar_multiplier(input.reaction).unwrap();
    let em_bonus = lunar_em_bonus(input.elemental_mastery);
    let res_mult = resistance_multiplier(enemy);

    let non_crit = level_base * reaction_mult * (1.0 + em_bonus + input.reaction_bonus) * res_mult;
    let crit = non_crit * (1.0 + input.crit_dmg);
    let average = non_crit * (1.0 - input.crit_rate) + crit * input.crit_rate;

    let damage_element = lunar_element(input.reaction).unwrap();

    Ok(LunarResult {
        non_crit,
        crit,
        average,
        damage_element,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    fn default_enemy() -> Enemy {
        Enemy { level: 90, resistance: 0.1, def_reduction: 0.0 }
    }

    #[test]
    fn test_lunar_electro_charged_no_em() {
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 0.5,
            crit_dmg: 1.0,
        };
        // 1446.8535 * 1.8 * 1.0 * 0.9 = 2343.9...
        let expected_non_crit = 1446.8535 * 1.8 * 0.9;
        let result = calculate_lunar(&input, &default_enemy()).unwrap();
        assert!((result.non_crit - expected_non_crit).abs() < 0.01);
        assert!((result.crit - expected_non_crit * 2.0).abs() < 0.01);
        assert_eq!(result.damage_element, Some(Element::Electro));
    }

    #[test]
    fn test_lunar_bloom() {
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarBloom,
            reaction_bonus: 0.0,
            crit_rate: 0.0,
            crit_dmg: 0.0,
        };
        let expected = 1446.8535 * 1.0 * 0.9;
        let result = calculate_lunar(&input, &default_enemy()).unwrap();
        assert!((result.non_crit - expected).abs() < 0.01);
        assert!((result.average - result.non_crit).abs() < EPSILON);
    }

    #[test]
    fn test_lunar_crystallize() {
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarCrystallize,
            reaction_bonus: 0.0,
            crit_rate: 0.5,
            crit_dmg: 1.0,
        };
        let expected = 1446.8535 * 0.96 * 0.9;
        let result = calculate_lunar(&input, &default_enemy()).unwrap();
        assert!((result.non_crit - expected).abs() < 0.01);
    }

    #[test]
    fn test_lunar_crit_applied() {
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 1.0,
            crit_dmg: 1.0,
        };
        let result = calculate_lunar(&input, &default_enemy()).unwrap();
        assert!((result.average - result.crit).abs() < EPSILON);
    }

    #[test]
    fn test_lunar_not_lunar_error() {
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
            crit_rate: 0.5,
            crit_dmg: 1.0,
        };
        assert!(matches!(
            calculate_lunar(&input, &default_enemy()),
            Err(CalcError::NotLunar(_))
        ));
    }

    #[test]
    fn test_lunar_em_applied() {
        let base = LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 0.0,
            crit_dmg: 0.0,
        };
        let with_em = LunarInput {
            elemental_mastery: 300.0,
            ..base.clone()
        };
        let r1 = calculate_lunar(&base, &default_enemy()).unwrap();
        let r2 = calculate_lunar(&with_em, &default_enemy()).unwrap();
        let em_bonus = 6.0 * 300.0 / (300.0 + 2000.0);
        assert!((r2.non_crit / r1.non_crit - (1.0 + em_bonus)).abs() < EPSILON);
    }
}
```

- [ ] **Step 2: Add `pub mod lunar;` to lib.rs**

- [ ] **Step 3: Run all tests**

- [ ] **Step 4: Run `cargo fmt` and `cargo clippy -- -D warnings`**

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/lunar.rs crates/core/src/lib.rs
git commit -m "feat: add calculate_lunar for lunar reaction damage"
```

---

### Task 8: lib.rs — Public API re-exports + final quality checks

**Files:**
- Modify: `crates/core/src/lib.rs`

- [ ] **Step 1: Add all re-exports to lib.rs**

```rust
pub use reaction::{determine_reaction, Reaction, ReactionCategory};
pub use transformative::{calculate_transformative, TransformativeInput, TransformativeResult};
pub use lunar::{calculate_lunar, LunarInput, LunarResult};
pub use em::{amplifying_em_bonus, catalyze_em_bonus, transformative_em_bonus, lunar_em_bonus};
pub use level_table::reaction_base_value;
```

- [ ] **Step 2: Run full test suite**

Run: `cargo test`
Expected: All tests pass

- [ ] **Step 3: Run `cargo clippy -- -D warnings`**

- [ ] **Step 4: Run `cargo fmt --check`**

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/lib.rs
git commit -m "feat: add public API re-exports for reaction modules"
```

---

### Task 9: Final verification + CLAUDE.md update

**Files:**
- Modify: `CLAUDE.md`

- [ ] **Step 1: Update CLAUDE.md**

Add to the Architecture section:

```markdown
- `reaction.rs`: 元素反応タイプ判定、倍率テーブル
- `em.rs`: 元素熟知ボーナス計算（4種の式）
- `level_table.rs`: レベル基礎値テーブル（Lv1-100、データマイニング値）
- `transformative.rs`: 固定反応ダメージ計算
- `lunar.rs`: 月反応ダメージ計算
```

- [ ] **Step 2: Run full verification**

Run: `cargo test && cargo clippy -- -D warnings && cargo fmt --check`

- [ ] **Step 3: Commit**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md with v0.2.0 reaction modules"
```
