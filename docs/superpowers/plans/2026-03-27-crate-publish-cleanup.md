# Crate公開前クリーンアップ 実装計画

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** genshin-calc-core と genshin-calc-data に doc comments、メタデータ、examples、READMEを追加して crates.io 公開可能にする

**Architecture:** core crate を先に完成 → data crate。各タスクはコミット単位。doc comments は英語、コード内コメントは日本語。

**Tech Stack:** Rust, cargo doc, cargo publish --dry-run

**Spec:** `docs/superpowers/specs/2026-03-27-crate-publish-cleanup-design.md`

---

## File Structure

### Core Crate (crates/core/)
- Modify: `Cargo.toml` — メタデータ追加
- Modify: `src/lib.rs` — `//!` crate doc 追加
- Modify: `src/types.rs` — `///` doc comments
- Modify: `src/stats.rs` — `///` doc comments
- Modify: `src/enemy.rs` — `///` doc comments
- Modify: `src/error.rs` — `///` doc comments
- Modify: `src/em.rs` — `///` doc comments
- Modify: `src/level_table.rs` — `///` doc comments
- Modify: `src/reaction.rs` — `///` doc comments
- Modify: `src/stat_profile.rs` — `///` doc comments
- Modify: `src/damage.rs` — `///` doc comments + `# Examples`
- Modify: `src/transformative.rs` — `///` doc comments + `# Examples`
- Modify: `src/lunar.rs` — `///` doc comments + `# Examples`
- Create: `examples/basic_damage.rs`
- Create: `examples/reactions.rs`
- Create: `README.md`

### Data Crate (crates/data/)
- Modify: `Cargo.toml` — メタデータ追加
- Modify: `src/lib.rs` — `//!` crate doc + 関数 `///`
- Modify: `src/types.rs` — `///` doc comments
- Modify: `src/buff.rs` — `///` doc comments
- Create: `examples/search.rs`
- Create: `README.md`

---

### Task 1: Core Cargo.toml メタデータ

**Files:**
- Modify: `crates/core/Cargo.toml`

- [ ] **Step 1: メタデータ追加**

`[package]` セクションに以下を追加（`license = "MIT"` の後に）:

```toml
authors = ["kotenbu"]
repository = "https://github.com/kotenbu/genshin-calc"
keywords = ["genshin-impact", "damage-calculator", "game", "wasm"]
categories = ["game-engines", "algorithms"]
rust-version = "1.85"
readme = "README.md"
```

- [ ] **Step 2: ビルド確認**

Run: `cargo build -p genshin-calc-core`
Expected: 成功

- [ ] **Step 3: コミット**

```bash
git add crates/core/Cargo.toml
git commit -m "chore(core): add crates.io metadata to Cargo.toml"
```

---

### Task 2: Core lib.rs crate-level doc

**Files:**
- Modify: `crates/core/src/lib.rs`

- [ ] **Step 1: crate doc 追加**

`lib.rs` の先頭（`pub mod damage;` の前）に追加:

```rust
//! # genshin-calc-core
//!
//! Damage and elemental reaction calculation engine for Genshin Impact.
//!
//! ## Calculation Pipelines
//!
//! | Pipeline | Function | Reactions |
//! |----------|----------|-----------|
//! | Standard | [`calculate_damage`] | Amplifying (vaporize/melt), Catalyze (spread/aggravate), or none |
//! | Transformative | [`calculate_transformative`] | Overloaded, superconduct, electro-charged, swirl, bloom, etc. |
//! | Lunar | [`calculate_lunar`] | Lunar electro-charged, lunar bloom, lunar crystallize |
//!
//! Standard damage passes through ATK/HP/DEF scaling, crit, defense, and resistance.
//! Transformative damage scales with character level and elemental mastery only (no crit, no defense).
//! Lunar damage scales like transformative but can crit.
//!
//! ## Example
//!
//! ```
//! use genshin_calc_core::*;
//!
//! let input = DamageInput {
//!     character_level: 90,
//!     stats: Stats {
//!         atk: 2000.0,
//!         crit_rate: 0.75,
//!         crit_dmg: 1.50,
//!         dmg_bonus: 0.466,
//!         ..Default::default()
//!     },
//!     talent_multiplier: 1.76,
//!     scaling_stat: ScalingStat::Atk,
//!     damage_type: DamageType::Skill,
//!     element: Some(Element::Pyro),
//!     reaction: None,
//!     reaction_bonus: 0.0,
//! };
//! let enemy = Enemy {
//!     level: 90,
//!     resistance: 0.10,
//!     def_reduction: 0.0,
//! };
//! let result = calculate_damage(&input, &enemy).unwrap();
//! assert!(result.average > 0.0);
//! ```

```

- [ ] **Step 2: doc test 実行**

Run: `cargo test --doc -p genshin-calc-core`
Expected: 1 test passed

- [ ] **Step 3: コミット**

```bash
git add crates/core/src/lib.rs
git commit -m "docs(core): add crate-level documentation with example"
```

---

### Task 3: Core 基本型 doc comments (types.rs, stats.rs, enemy.rs, error.rs)

**Files:**
- Modify: `crates/core/src/types.rs`
- Modify: `crates/core/src/stats.rs`
- Modify: `crates/core/src/enemy.rs`
- Modify: `crates/core/src/error.rs`

- [ ] **Step 1: types.rs に doc comments 追加**

```rust
/// Elements in Genshin Impact.
pub enum Element { ... }

/// Attack type classification.
pub enum DamageType { ... }

/// Stat used for damage scaling.
///
/// Most characters scale on ATK. Some scale on HP (e.g. Hu Tao)
/// or DEF (e.g. Albedo, Noelle).
pub enum ScalingStat { ... }
```

- [ ] **Step 2: stats.rs に doc comments 追加**

```rust
/// Final character stats used for damage calculation.
///
/// All percentage fields use decimal form (e.g. 75% crit rate = `0.75`).
pub struct Stats {
    /// Max HP.
    pub hp: f64,
    /// Total ATK (base + bonus).
    pub atk: f64,
    /// Total DEF (base + bonus).
    pub def: f64,
    /// Elemental mastery.
    pub elemental_mastery: f64,
    /// Crit rate in decimal form (0.0 to 1.0).
    pub crit_rate: f64,
    /// Crit DMG in decimal form (base 0.50 = 50%).
    pub crit_dmg: f64,
    /// Energy recharge in decimal form (base 1.0 = 100%).
    pub energy_recharge: f64,
    /// DMG bonus in decimal form (e.g. 0.466 for Pyro DMG goblet).
    pub dmg_bonus: f64,
}
```

- [ ] **Step 3: enemy.rs に doc comments 追加**

```rust
/// Enemy parameters for damage calculation.
pub struct Enemy {
    /// Enemy level (1-100).
    pub level: u32,
    /// Elemental resistance in decimal form (e.g. 0.10 = 10%).
    pub resistance: f64,
    /// DEF reduction from debuffs in decimal form (0.0 to 1.0).
    pub def_reduction: f64,
}
```

- [ ] **Step 4: error.rs に doc comments 追加**

```rust
/// Errors returned by calculation functions.
///
/// Each variant includes the invalid value for debugging.
pub enum CalcError { ... }
```

- [ ] **Step 5: テスト確認**

Run: `cargo test -p genshin-calc-core && cargo doc --no-deps -p genshin-calc-core`
Expected: 全テストパス、doc警告なし

- [ ] **Step 6: コミット**

```bash
git add crates/core/src/types.rs crates/core/src/stats.rs crates/core/src/enemy.rs crates/core/src/error.rs
git commit -m "docs(core): add doc comments to types, stats, enemy, error"
```

---

### Task 4: Core em.rs + level_table.rs doc comments

**Files:**
- Modify: `crates/core/src/em.rs`
- Modify: `crates/core/src/level_table.rs`

- [ ] **Step 1: em.rs に doc comments 追加**

```rust
/// Elemental mastery bonus for amplifying reactions (vaporize/melt).
///
/// Formula: `2.78 * EM / (EM + 1400)`
pub fn amplifying_em_bonus(em: f64) -> f64 { ... }

/// Elemental mastery bonus for catalyze reactions (spread/aggravate).
///
/// Formula: `5.0 * EM / (EM + 1200)`
pub fn catalyze_em_bonus(em: f64) -> f64 { ... }

/// Elemental mastery bonus for transformative reactions.
///
/// Formula: `16.0 * EM / (EM + 2000)`
pub fn transformative_em_bonus(em: f64) -> f64 { ... }

/// Elemental mastery bonus for lunar reactions.
///
/// Formula: `6.0 * EM / (EM + 2000)`
pub fn lunar_em_bonus(em: f64) -> f64 { ... }
```

- [ ] **Step 2: level_table.rs に doc comments 追加**

```rust
/// Returns the base reaction value for a given character level.
///
/// Used by transformative and lunar reaction calculations. Values are from
/// datamined level multiplier tables (Lv1-100).
///
/// Returns `None` if level is 0 or greater than 100.
pub fn reaction_base_value(level: u32) -> Option<f64> { ... }
```

- [ ] **Step 3: テスト確認**

Run: `cargo test -p genshin-calc-core && cargo doc --no-deps -p genshin-calc-core`
Expected: パス

- [ ] **Step 4: コミット**

```bash
git add crates/core/src/em.rs crates/core/src/level_table.rs
git commit -m "docs(core): add doc comments to em and level_table"
```

---

### Task 5: Core reaction.rs doc comments

**Files:**
- Modify: `crates/core/src/reaction.rs`

- [ ] **Step 1: 型に doc comments 追加**

```rust
/// Elemental reactions in Genshin Impact.
///
/// Reactions are grouped into four categories: amplifying, catalyze,
/// transformative, and lunar. Use [`Reaction::category`] to check which
/// category a reaction belongs to.
pub enum Reaction { ... }

/// Reaction category classification.
pub enum ReactionCategory { ... }
```

- [ ] **Step 2: 関数に doc comments 追加**

```rust
/// Determines the elemental reaction from a trigger and aura element.
///
/// Returns `None` if no reaction occurs (e.g. same element,
/// or Geo/Anemo as aura).
///
/// # Errors
/// This function does not return errors — it returns `Option`.
pub fn determine_reaction(trigger: Element, aura: Element) -> Option<Reaction> { ... }

/// Returns the amplifying reaction multiplier.
///
/// Vaporize (Hydro on Pyro) = 2.0, Vaporize (Pyro on Hydro) = 1.5,
/// Melt (Pyro on Cryo) = 2.0, Melt (Cryo on Pyro) = 1.5.
pub fn amplifying_multiplier(trigger: Element, aura: Element) -> Option<f64> { ... }

/// Returns the catalyze flat damage coefficient.
///
/// Aggravate = 1.15, Spread = 1.25.
pub fn catalyze_coefficient(reaction: Reaction) -> Option<f64> { ... }

/// Returns the transformative reaction damage multiplier.
pub fn transformative_multiplier(reaction: Reaction) -> Option<f64> { ... }

/// Returns the lunar reaction damage multiplier.
pub fn lunar_multiplier(reaction: Reaction) -> Option<f64> { ... }

/// Returns the damage element of a transformative reaction.
///
/// Returns `None` if the reaction is not transformative.
/// Returns `Some(None)` for reactions that deal physical damage (e.g. shattered).
pub fn transformative_element(reaction: Reaction) -> Option<Option<Element>> { ... }

/// Returns the damage element of a lunar reaction.
pub fn lunar_element(reaction: Reaction) -> Option<Option<Element>> { ... }
```

- [ ] **Step 3: テスト確認**

Run: `cargo test -p genshin-calc-core && cargo doc --no-deps -p genshin-calc-core`
Expected: パス

- [ ] **Step 4: コミット**

```bash
git add crates/core/src/reaction.rs
git commit -m "docs(core): add doc comments to reaction types and functions"
```

---

### Task 6: Core stat_profile.rs doc comments

**Files:**
- Modify: `crates/core/src/stat_profile.rs`

- [ ] **Step 1: doc comments 追加**

```rust
/// Input for stat calculation with base/percent/flat breakdown.
///
/// Represents character base stats, weapon bonuses, artifact substats, etc.
/// Use [`combine_stats`] to compute final [`Stats`](crate::Stats).
///
/// All percentage fields use decimal form (e.g. 46.6% ATK = `0.466`).
pub struct StatProfile {
    /// Base HP (character + weapon at current ascension).
    pub base_hp: f64,
    /// Base ATK (character + weapon at current ascension).
    pub base_atk: f64,
    /// Base DEF (character at current ascension).
    pub base_def: f64,
    /// HP% bonus (artifacts, buffs).
    pub hp_percent: f64,
    /// ATK% bonus (artifacts, buffs).
    pub atk_percent: f64,
    /// DEF% bonus (artifacts, buffs).
    pub def_percent: f64,
    /// Flat HP bonus (artifacts, buffs).
    pub hp_flat: f64,
    /// Flat ATK bonus (artifacts, feather).
    pub atk_flat: f64,
    /// Flat DEF bonus (artifacts, buffs).
    pub def_flat: f64,
    /// Elemental mastery.
    pub elemental_mastery: f64,
    /// Crit rate in decimal form.
    pub crit_rate: f64,
    /// Crit DMG in decimal form.
    pub crit_dmg: f64,
    /// Energy recharge in decimal form.
    pub energy_recharge: f64,
    /// DMG bonus in decimal form.
    pub dmg_bonus: f64,
}

/// Combines a [`StatProfile`] into final [`Stats`](crate::Stats).
///
/// Formula: `final = base * (1 + percent) + flat`
///
/// # Errors
///
/// Returns [`CalcError`](crate::CalcError) if any input value is out of valid range
/// (e.g. negative base values, percent bonus below -1.0).
pub fn combine_stats(profile: &StatProfile) -> Result<Stats, CalcError> { ... }
```

- [ ] **Step 2: テスト確認**

Run: `cargo test -p genshin-calc-core`
Expected: パス

- [ ] **Step 3: コミット**

```bash
git add crates/core/src/stat_profile.rs
git commit -m "docs(core): add doc comments to stat_profile"
```

---

### Task 7: Core damage.rs doc comments + Examples

**Files:**
- Modify: `crates/core/src/damage.rs`

- [ ] **Step 1: DamageInput/DamageResult に doc comments 追加**

```rust
/// Input parameters for standard damage calculation.
pub struct DamageInput {
    /// Character level (1-90).
    pub character_level: u32,
    /// Final character stats.
    pub stats: Stats,
    /// Talent multiplier in decimal form (e.g. 176% = `1.76`).
    pub talent_multiplier: f64,
    /// Stat used for scaling (ATK, HP, or DEF).
    pub scaling_stat: ScalingStat,
    /// Attack type (normal, charged, skill, burst, plunging).
    pub damage_type: DamageType,
    /// Damage element. `None` for physical damage.
    pub element: Option<Element>,
    /// Elemental reaction. `None` for no reaction.
    pub reaction: Option<Reaction>,
    /// Reaction DMG bonus from artifacts/buffs in decimal form.
    pub reaction_bonus: f64,
}

/// Result of standard damage calculation.
pub struct DamageResult {
    /// Damage without critical hit.
    pub non_crit: f64,
    /// Damage with critical hit.
    pub crit: f64,
    /// Average damage (weighted by crit rate).
    pub average: f64,
    /// Applied reaction, if any.
    pub reaction: Option<Reaction>,
}
```

- [ ] **Step 2: calculate_damage に doc + Example 追加**

```rust
/// Calculates standard damage for attacks and skills.
///
/// Supports amplifying reactions (vaporize/melt) and catalyze reactions
/// (spread/aggravate) via the `reaction` field. For transformative reactions
/// (overloaded, swirl, etc.) use [`calculate_transformative`](crate::calculate_transformative).
/// For lunar reactions use [`calculate_lunar`](crate::calculate_lunar).
///
/// ## Damage Formula
///
/// `base_dmg = stat * talent_multiplier * (1 + dmg_bonus)`
/// `final_dmg = base_dmg * defense_mult * resistance_mult * reaction_mult`
///
/// # Errors
///
/// Returns [`CalcError`] if any input parameter is out of valid range.
///
/// # Examples
///
/// ```
/// use genshin_calc_core::*;
///
/// let input = DamageInput {
///     character_level: 90,
///     stats: Stats {
///         atk: 2000.0,
///         crit_rate: 0.75,
///         crit_dmg: 1.50,
///         dmg_bonus: 0.466,
///         ..Default::default()
///     },
///     talent_multiplier: 1.76,
///     scaling_stat: ScalingStat::Atk,
///     damage_type: DamageType::Skill,
///     element: Some(Element::Pyro),
///     reaction: None,
///     reaction_bonus: 0.0,
/// };
/// let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
/// let result = calculate_damage(&input, &enemy).unwrap();
/// assert!(result.crit > result.non_crit);
/// ```
pub fn calculate_damage(...) { ... }
```

- [ ] **Step 3: doc test 実行**

Run: `cargo test --doc -p genshin-calc-core`
Expected: パス（lib.rs + damage.rs の2つ）

- [ ] **Step 4: コミット**

```bash
git add crates/core/src/damage.rs
git commit -m "docs(core): add doc comments and example to damage module"
```

---

### Task 8: Core transformative.rs + lunar.rs doc comments + Examples

**Files:**
- Modify: `crates/core/src/transformative.rs`
- Modify: `crates/core/src/lunar.rs`

- [ ] **Step 1: transformative.rs に doc comments + Example 追加**

```rust
/// Input for transformative reaction damage calculation.
pub struct TransformativeInput {
    /// Character level (1-100).
    pub character_level: u32,
    /// Elemental mastery.
    pub elemental_mastery: f64,
    /// Transformative reaction type.
    pub reaction: Reaction,
    /// Reaction DMG bonus in decimal form.
    pub reaction_bonus: f64,
}

/// Result of transformative reaction damage calculation.
pub struct TransformativeResult {
    /// Total reaction damage.
    pub damage: f64,
    /// Element of the reaction damage. `None` for physical (e.g. shattered).
    pub damage_element: Option<Element>,
}

/// Calculates transformative reaction damage.
///
/// Transformative reactions deal fixed damage based on character level and
/// elemental mastery. They ignore ATK, talent multipliers, crit, and defense.
///
/// # Errors
///
/// Returns [`CalcError`] if the reaction is not transformative or inputs are invalid.
///
/// # Examples
///
/// ```
/// use genshin_calc_core::*;
///
/// let input = TransformativeInput {
///     character_level: 90,
///     elemental_mastery: 200.0,
///     reaction: Reaction::Overloaded,
///     reaction_bonus: 0.0,
/// };
/// let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
/// let result = calculate_transformative(&input, &enemy).unwrap();
/// assert!(result.damage > 0.0);
/// ```
pub fn calculate_transformative(...) { ... }
```

- [ ] **Step 2: lunar.rs に doc comments + Example 追加**

```rust
/// Input for lunar reaction damage calculation.
pub struct LunarInput {
    /// Character level (1-100).
    pub character_level: u32,
    /// Elemental mastery.
    pub elemental_mastery: f64,
    /// Lunar reaction type.
    pub reaction: Reaction,
    /// Reaction DMG bonus in decimal form.
    pub reaction_bonus: f64,
    /// Crit rate in decimal form.
    pub crit_rate: f64,
    /// Crit DMG in decimal form.
    pub crit_dmg: f64,
}

/// Result of lunar reaction damage calculation.
pub struct LunarResult {
    /// Damage without critical hit.
    pub non_crit: f64,
    /// Damage with critical hit.
    pub crit: f64,
    /// Average damage (weighted by crit rate).
    pub average: f64,
    /// Element of the reaction damage.
    pub damage_element: Option<Element>,
}

/// Calculates lunar reaction damage.
///
/// Lunar reactions scale with character level and elemental mastery like
/// transformative reactions, but they can also crit. They ignore ATK,
/// talent multipliers, and defense.
///
/// # Errors
///
/// Returns [`CalcError`] if the reaction is not lunar or inputs are invalid.
///
/// # Examples
///
/// ```
/// use genshin_calc_core::*;
///
/// let input = LunarInput {
///     character_level: 90,
///     elemental_mastery: 300.0,
///     reaction: Reaction::LunarElectroCharged,
///     reaction_bonus: 0.0,
///     crit_rate: 0.60,
///     crit_dmg: 1.20,
/// };
/// let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
/// let result = calculate_lunar(&input, &enemy).unwrap();
/// assert!(result.crit > result.non_crit);
/// ```
pub fn calculate_lunar(...) { ... }
```

- [ ] **Step 3: doc test 実行**

Run: `cargo test --doc -p genshin-calc-core`
Expected: パス（lib.rs + damage.rs + transformative.rs + lunar.rs）

- [ ] **Step 4: コミット**

```bash
git add crates/core/src/transformative.rs crates/core/src/lunar.rs
git commit -m "docs(core): add doc comments and examples to transformative and lunar"
```

---

### Task 9: Core examples

**Files:**
- Create: `crates/core/examples/basic_damage.rs`
- Create: `crates/core/examples/reactions.rs`

- [ ] **Step 1: basic_damage.rs 作成**

```rust
//! Basic damage calculation examples.
//!
//! Run: `cargo run -p genshin-calc-core --example basic_damage`

use genshin_calc_core::*;

fn main() {
    let stats = Stats {
        hp: 20000.0,
        atk: 2000.0,
        def: 800.0,
        elemental_mastery: 100.0,
        crit_rate: 0.75,
        crit_dmg: 1.50,
        energy_recharge: 1.20,
        dmg_bonus: 0.466,
    };

    let enemy = Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
    };

    // Physical damage (no element)
    let physical = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: stats.clone(),
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: None,
            reaction: None,
            reaction_bonus: 0.0,
        },
        &enemy,
    )
    .unwrap();
    println!("Physical Normal:");
    println!("  non-crit: {:.1}", physical.non_crit);
    println!("  crit:     {:.1}", physical.crit);
    println!("  average:  {:.1}", physical.average);

    // Pyro skill damage
    let pyro = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: stats.clone(),
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
        },
        &enemy,
    )
    .unwrap();
    println!("\nPyro Skill:");
    println!("  non-crit: {:.1}", pyro.non_crit);
    println!("  crit:     {:.1}", pyro.crit);
    println!("  average:  {:.1}", pyro.average);

    // Vaporize (Pyro trigger = 1.5x)
    let vaporize = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats,
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: Some(Reaction::Vaporize),
            reaction_bonus: 0.0,
        },
        &enemy,
    )
    .unwrap();
    println!("\nVaporize (Pyro 1.5x):");
    println!("  non-crit: {:.1}", vaporize.non_crit);
    println!("  crit:     {:.1}", vaporize.crit);
    println!("  average:  {:.1}", vaporize.average);
}
```

- [ ] **Step 2: reactions.rs 作成**

```rust
//! Transformative and lunar reaction examples.
//!
//! Run: `cargo run -p genshin-calc-core --example reactions`

use genshin_calc_core::*;

fn main() {
    let enemy = Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
    };

    // Overloaded (transformative)
    let overloaded = calculate_transformative(
        &TransformativeInput {
            character_level: 90,
            elemental_mastery: 200.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
        },
        &enemy,
    )
    .unwrap();
    println!("Overloaded (EM 200):");
    println!("  damage:  {:.1}", overloaded.damage);
    println!("  element: {:?}", overloaded.damage_element);

    // Superconduct (transformative)
    let superconduct = calculate_transformative(
        &TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Superconduct,
            reaction_bonus: 0.0,
        },
        &enemy,
    )
    .unwrap();
    println!("\nSuperconduct (EM 0):");
    println!("  damage:  {:.1}", superconduct.damage);

    // Lunar Electro-Charged (lunar — can crit)
    let lunar_ec = calculate_lunar(
        &LunarInput {
            character_level: 90,
            elemental_mastery: 300.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 0.60,
            crit_dmg: 1.20,
        },
        &enemy,
    )
    .unwrap();
    println!("\nLunar Electro-Charged (EM 300, CR 60%, CD 120%):");
    println!("  non-crit: {:.1}", lunar_ec.non_crit);
    println!("  crit:     {:.1}", lunar_ec.crit);
    println!("  average:  {:.1}", lunar_ec.average);
}
```

- [ ] **Step 3: examples 実行確認**

Run: `cargo run -p genshin-calc-core --example basic_damage`
Expected: 数値出力

Run: `cargo run -p genshin-calc-core --example reactions`
Expected: 数値出力

- [ ] **Step 4: コミット**

```bash
git add crates/core/examples/
git commit -m "docs(core): add basic_damage and reactions examples"
```

---

### Task 10: Core README.md

**Files:**
- Create: `crates/core/README.md`

- [ ] **Step 1: README 作成**

```markdown
# genshin-calc-core

Damage and elemental reaction calculation engine for Genshin Impact.

## Installation

```toml
[dependencies]
genshin-calc-core = "0.1"
```

Minimum supported Rust version: **1.85**

## Features

Three calculation pipelines covering all damage types:

- **`calculate_damage`** — standard damage (normal/charged/skill/burst) with optional amplifying (vaporize/melt) or catalyze (spread/aggravate) reactions
- **`calculate_transformative`** — transformative reactions (overloaded, superconduct, electro-charged, swirl, bloom, etc.) — scales with level and EM, no crit
- **`calculate_lunar`** — lunar reactions (lunar electro-charged, lunar bloom, lunar crystallize) — like transformative but can crit

## Usage

```rust
use genshin_calc_core::*;

let input = DamageInput {
    character_level: 90,
    stats: Stats {
        atk: 2000.0,
        crit_rate: 0.75,
        crit_dmg: 1.50,
        dmg_bonus: 0.466,
        ..Default::default()
    },
    talent_multiplier: 1.76,
    scaling_stat: ScalingStat::Atk,
    damage_type: DamageType::Skill,
    element: Some(Element::Pyro),
    reaction: None,
    reaction_bonus: 0.0,
};
let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
let result = calculate_damage(&input, &enemy).unwrap();

println!("Average damage: {:.0}", result.average);
```

## Supported Reactions

| Category | Reactions |
|----------|-----------|
| Amplifying | Vaporize, Melt |
| Catalyze | Aggravate, Spread |
| Transformative | Overloaded, Superconduct, Electro-Charged, Shattered, Swirl, Bloom, Hyperbloom, Burgeon, Burning |
| Lunar | Lunar Electro-Charged, Lunar Bloom, Lunar Crystallize |

## License

MIT
```

- [ ] **Step 2: コミット**

```bash
git add crates/core/README.md
git commit -m "docs(core): add README"
```

---

### Task 11: Core 検証

- [ ] **Step 1: cargo doc**

Run: `cargo doc --no-deps -p genshin-calc-core 2>&1`
Expected: 警告なし

- [ ] **Step 2: cargo test (全テスト + doc tests)**

Run: `cargo test -p genshin-calc-core`
Expected: 127ユニットテスト + doc tests パス

- [ ] **Step 3: cargo test --doc 単体**

Run: `cargo test --doc -p genshin-calc-core`
Expected: doc tests パス（lib.rs, damage.rs, transformative.rs, lunar.rs）

- [ ] **Step 4: cargo clippy**

Run: `cargo clippy -p genshin-calc-core -- -D warnings`
Expected: 0 warnings

- [ ] **Step 5: cargo publish --dry-run**

Run: `cargo publish --dry-run -p genshin-calc-core 2>&1`
Expected: 成功（README.mdが含まれること確認）

---

### Task 12: Data Cargo.toml メタデータ

**Files:**
- Modify: `crates/data/Cargo.toml`

- [ ] **Step 1: メタデータ追加**

`[package]` セクションに追加:

```toml
authors = ["kotenbu"]
repository = "https://github.com/kotenbu/genshin-calc"
keywords = ["genshin-impact", "game-data", "damage-calculator", "wasm"]
categories = ["game-engines", "data-structures"]
rust-version = "1.85"
readme = "README.md"
```

- [ ] **Step 2: コミット**

```bash
git add crates/data/Cargo.toml
git commit -m "chore(data): add crates.io metadata to Cargo.toml"
```

---

### Task 13: Data lib.rs crate doc + 検索関数 doc comments

**Files:**
- Modify: `crates/data/src/lib.rs`

- [ ] **Step 1: crate doc + 関数 doc + GAME_VERSION doc 追加**

lib.rs 先頭に `//!` 追加。各関数・定数の前に `///` 追加:

```rust
//! # genshin-calc-data
//!
//! Genshin Impact v5.8 game data as Rust constants.
//!
//! Includes:
//! - 102 playable characters
//! - 230 weapons (all types)
//! - 52 artifact sets
//! - 15 enemies with resistance templates
//!
//! ## Example
//!
//! ```
//! use genshin_calc_data::*;
//!
//! let diluc = find_character("diluc").unwrap();
//! assert_eq!(diluc.name, "Diluc");
//!
//! let pyro_chars = characters_by_element(genshin_calc_core::Element::Pyro);
//! assert!(pyro_chars.len() > 10);
//! ```

/// Current game version for the included data.
pub const GAME_VERSION: &str = "5.8";

/// Finds a character by ID (lowercase, e.g. `"diluc"`, `"hu_tao"`).
///
/// # Examples
///
/// ```
/// use genshin_calc_data::find_character;
///
/// let ganyu = find_character("ganyu").unwrap();
/// assert_eq!(ganyu.name, "Ganyu");
/// assert!(find_character("nonexistent").is_none());
/// ```
pub fn find_character(...) { ... }

/// Finds a weapon by ID (lowercase, e.g. `"wolfs_gravestone"`).
///
/// # Examples
///
/// ```
/// use genshin_calc_data::find_weapon;
///
/// let weapon = find_weapon("wolfs_gravestone").unwrap();
/// assert_eq!(weapon.name, "Wolf's Gravestone");
/// ```
pub fn find_weapon(...) { ... }

/// Finds an artifact set by ID (lowercase, e.g. `"crimson_witch_of_flames"`).
pub fn find_artifact_set(...) { ... }

/// Finds an enemy by ID (lowercase, e.g. `"hilichurl"`).
pub fn find_enemy(...) { ... }

/// Returns all characters with the given element.
pub fn characters_by_element(...) { ... }

/// Returns all weapons of the given type.
pub fn weapons_by_type(...) { ... }
```

- [ ] **Step 2: doc test 実行**

Run: `cargo test --doc -p genshin-calc-data`
Expected: パス

- [ ] **Step 3: コミット**

```bash
git add crates/data/src/lib.rs
git commit -m "docs(data): add crate-level docs and search function doc comments"
```

---

### Task 14: Data types.rs doc comments

**Files:**
- Modify: `crates/data/src/types.rs`

- [ ] **Step 1: enum に doc comments 追加**

```rust
/// Weapon type classification.
pub enum WeaponType { ... }

/// Character or weapon rarity.
pub enum Rarity { ... }

/// Character region of origin.
pub enum Region { ... }

/// Stat gained from character ascension.
pub enum AscensionStat { ... }

/// Artifact set rarity availability.
pub enum ArtifactRarity { ... }

/// Weapon sub-stat type with values at each ascension breakpoint.
pub enum WeaponSubStat { ... }
```

- [ ] **Step 2: struct に doc comments 追加**

```rust
/// Talent scaling entry with multipliers at each talent level.
pub struct TalentScaling {
    /// Scaling name (e.g. "1-Hit DMG", "Charged Attack DMG").
    pub name: &'static str,
    /// Stat used for scaling.
    pub scaling_stat: ScalingStat,
    /// Element of the talent damage. `None` for physical.
    pub damage_element: Option<Element>,
    /// Multiplier values at talent levels 1-15.
    pub values: [f64; 15],
}

/// Talent data for an elemental skill or burst.
pub struct TalentData { ... }

/// Normal attack data including charged and plunging attacks.
pub struct NormalAttackData { ... }

/// Complete talent set for a character.
pub struct TalentSet { ... }

/// Character data including base stats, talents, and metadata.
pub struct CharacterData {
    /// Unique identifier (lowercase, e.g. `"hu_tao"`).
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Character element.
    pub element: Element,
    /// Weapon type.
    pub weapon_type: WeaponType,
    /// Rarity (4-star or 5-star).
    pub rarity: Rarity,
    /// Region of origin.
    pub region: Region,
    /// Base HP at ascension breakpoints [Lv1, Lv20, Lv80, Lv90].
    pub base_hp: [f64; 4],
    /// Base ATK at ascension breakpoints.
    pub base_atk: [f64; 4],
    /// Base DEF at ascension breakpoints.
    pub base_def: [f64; 4],
    /// Stat gained from ascension.
    pub ascension_stat: AscensionStat,
    /// Character talent set.
    pub talents: TalentSet,
}

/// Weapon data including base stats, passives, and metadata.
pub struct WeaponData { ... }

/// Weapon passive effect.
pub struct WeaponPassive { ... }

/// Artifact set effect for 2-piece or 4-piece bonus.
pub struct SetEffect { ... }

/// Artifact set data.
pub struct ArtifactSet { ... }

/// Elemental resistance template for an enemy type.
pub struct ResistanceTemplate { ... }

/// Enemy data with resistance template.
pub struct EnemyData { ... }
```

- [ ] **Step 3: impl メソッドに doc 追加**

```rust
impl ResistanceTemplate {
    /// Returns the resistance value for the given element, or physical if `None`.
    pub fn get(&self, element: Option<Element>) -> f64 { ... }
}

impl EnemyData {
    /// Converts to a [`genshin_calc_core::Enemy`] for use in damage calculations.
    pub fn to_enemy(&self, element: Option<Element>, level: u32, def_reduction: f64) -> ... { ... }
}
```

- [ ] **Step 4: テスト確認**

Run: `cargo test -p genshin-calc-data && cargo doc --no-deps -p genshin-calc-data`
Expected: パス

- [ ] **Step 5: コミット**

```bash
git add crates/data/src/types.rs
git commit -m "docs(data): add doc comments to all types"
```

---

### Task 15: Data buff.rs doc comments

**Files:**
- Modify: `crates/data/src/buff.rs`

- [ ] **Step 1: doc comments 追加**

```rust
/// Stat that can be buffed by weapons or artifacts.
pub enum BuffableStat { ... }

/// A stat buff with a value and optional refinement scaling.
pub struct StatBuff {
    /// Which stat is buffed.
    pub stat: BuffableStat,
    /// Buff value at refinement 1 (or fixed value if no refinement).
    pub value: f64,
    /// Values at refinements 1-5. `None` for fixed buffs.
    pub refinement_values: Option<[f64; 5]>,
}

/// Passive effect from a weapon or artifact set.
pub struct PassiveEffect {
    /// Human-readable description.
    pub description: &'static str,
    /// Stat buffs provided by this effect.
    pub buffs: &'static [StatBuff],
}
```

- [ ] **Step 2: コミット**

```bash
git add crates/data/src/buff.rs
git commit -m "docs(data): add doc comments to buff types"
```

---

### Task 16: Data examples

**Files:**
- Create: `crates/data/examples/search.rs`

- [ ] **Step 1: search.rs 作成**

```rust
//! Game data search examples.
//!
//! Run: `cargo run -p genshin-calc-data --example search`

use genshin_calc_core::Element;
use genshin_calc_data::*;
use genshin_calc_data::types::WeaponType;

fn main() {
    // Find a character by ID
    let diluc = find_character("diluc").unwrap();
    println!("Character: {} ({:?}, {:?})", diluc.name, diluc.element, diluc.weapon_type);
    println!("  Base ATK at Lv90: {:.0}", diluc.base_atk[3]);

    // Filter characters by element
    let pyro_chars = characters_by_element(Element::Pyro);
    println!("\nPyro characters ({}):", pyro_chars.len());
    for c in &pyro_chars {
        println!("  - {} ({:?})", c.name, c.rarity);
    }

    // Find a weapon by ID
    let wgs = find_weapon("wolfs_gravestone").unwrap();
    println!("\nWeapon: {} ({:?}, {:?})", wgs.name, wgs.weapon_type, wgs.rarity);

    // Filter weapons by type
    let swords = weapons_by_type(WeaponType::Sword);
    println!("\nSwords ({}):", swords.len());
    for w in swords.iter().take(5) {
        println!("  - {} ({:?})", w.name, w.rarity);
    }

    // Find an artifact set
    let cw = find_artifact_set("crimson_witch_of_flames").unwrap();
    println!("\nArtifact: {}", cw.name);
    println!("  2pc: {}", cw.two_piece.description);

    // Find an enemy and convert for calculation
    let enemy_data = find_enemy("hilichurl").unwrap();
    let enemy = enemy_data.to_enemy(Some(Element::Pyro), 90, 0.0);
    println!("\nEnemy: {} (Pyro resistance: {:.0}%)", enemy_data.name, enemy.resistance * 100.0);
}
```

- [ ] **Step 2: 実行確認**

Run: `cargo run -p genshin-calc-data --example search`
Expected: 数値出力

- [ ] **Step 3: コミット**

```bash
git add crates/data/examples/
git commit -m "docs(data): add search example"
```

---

### Task 17: Data README.md

**Files:**
- Create: `crates/data/README.md`

- [ ] **Step 1: README 作成**

```markdown
# genshin-calc-data

Genshin Impact v5.8 game data as Rust constants for use with
[genshin-calc-core](https://crates.io/crates/genshin-calc-core).

## Installation

```toml
[dependencies]
genshin-calc-data = "0.1"
```

Minimum supported Rust version: **1.85**

## Included Data

| Category | Count |
|----------|-------|
| Characters | 102 |
| Weapons | 230 |
| Artifact Sets | 52 |
| Enemies | 15 |

All data is current as of Genshin Impact **v5.8**.

## Usage

```rust
use genshin_calc_data::*;
use genshin_calc_core::Element;

// Look up a character
let hu_tao = find_character("hu_tao").unwrap();
println!("{} — {:?} {:?}", hu_tao.name, hu_tao.element, hu_tao.weapon_type);

// Find all Pyro characters
let pyro = characters_by_element(Element::Pyro);
println!("{} Pyro characters", pyro.len());

// Convert enemy data for damage calculation
let enemy_data = find_enemy("hilichurl").unwrap();
let enemy = enemy_data.to_enemy(Some(Element::Pyro), 90, 0.0);
```

## License

MIT
```

- [ ] **Step 2: コミット**

```bash
git add crates/data/README.md
git commit -m "docs(data): add README"
```

---

### Task 18: 最終検証

- [ ] **Step 1: cargo doc（両crate）**

Run: `cargo doc --no-deps 2>&1`
Expected: 警告なし

- [ ] **Step 2: cargo test（全テスト + doc tests）**

Run: `cargo test`
Expected: 既存127 + 統合153ケース + doc tests すべてパス

- [ ] **Step 3: cargo clippy**

Run: `cargo clippy -- -D warnings`
Expected: 0 warnings

- [ ] **Step 4: examples 実行**

Run: `cargo run -p genshin-calc-core --example basic_damage`
Run: `cargo run -p genshin-calc-core --example reactions`
Run: `cargo run -p genshin-calc-data --example search`
Expected: すべて正常出力

- [ ] **Step 5: cargo publish --dry-run（両crate）**

Run: `cargo publish --dry-run -p genshin-calc-core 2>&1`
Run: `cargo publish --dry-run -p genshin-calc-data 2>&1`
Expected: 両方成功

- [ ] **Step 6: Doc comment チェックリスト最終確認**

- [ ] 全doc comments は英語
- [ ] 1行説明は80文字以内
- [ ] Result 返す関数に # Errors
- [ ] 主要3関数に # Examples
- [ ] 検索関数に # Examples
