# genshin-calc v0.1.0 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust library that calculates single-character damage in Genshin Impact (base damage, crit, defense multiplier, resistance multiplier).

**Architecture:** Cargo workspace with `crates/core` (calculation engine, data-independent) and `crates/data` (skeleton only for v0.1.0). Pure functions, immutable data, Result-based error handling. All public types derive serde traits.

**Tech Stack:** Rust 1.94, serde 1 (derive), thiserror 2, serde_json 1 (dev)

**Spec:** `docs/superpowers/specs/2026-03-25-genshin-calc-design.md`

---

## File Structure

| File | Responsibility |
|------|---------------|
| `Cargo.toml` | Workspace root — defines members `crates/core`, `crates/data` |
| `CLAUDE.md` | Claude Code project configuration |
| `crates/core/Cargo.toml` | Core crate manifest — serde, thiserror deps |
| `crates/core/src/lib.rs` | Public API re-exports |
| `crates/core/src/types.rs` | `Element`, `DamageType` enums |
| `crates/core/src/stats.rs` | `Stats` struct |
| `crates/core/src/enemy.rs` | `Enemy` struct |
| `crates/core/src/error.rs` | `CalcError` enum |
| `crates/core/src/damage.rs` | `DamageInput`, `DamageResult`, `calculate_damage()`, internal calc functions |
| `crates/data/Cargo.toml` | Data crate manifest (empty skeleton) |
| `crates/data/src/lib.rs` | Empty placeholder |

---

### Task 1: Workspace scaffold + CLAUDE.md

**Files:**
- Create: `Cargo.toml`
- Create: `CLAUDE.md`
- Create: `crates/core/Cargo.toml`
- Create: `crates/core/src/lib.rs`
- Create: `crates/data/Cargo.toml`
- Create: `crates/data/src/lib.rs`

- [ ] **Step 1: Create workspace root `Cargo.toml`**

```toml
[workspace]
members = ["crates/core", "crates/data"]
resolver = "2"
```

- [ ] **Step 2: Create `crates/core/Cargo.toml`**

```toml
[package]
name = "genshin-calc-core"
version = "0.1.0"
edition = "2024"
description = "Genshin Impact damage calculation engine"
license = "MIT"

[dependencies]
serde = { version = "1", features = ["derive"] }
thiserror = "2"

[dev-dependencies]
serde_json = "1"
```

- [ ] **Step 3: Create `crates/core/src/lib.rs` (empty placeholder)**

```rust
// Modules will be added incrementally as files are created.
```

- [ ] **Step 4: Create `crates/data/Cargo.toml`**

```toml
[package]
name = "genshin-calc-data"
version = "0.1.0"
edition = "2024"
description = "Genshin Impact game data (future)"
license = "MIT"

[dependencies]
genshin-calc-core = { path = "../core" }
```

- [ ] **Step 5: Create `crates/data/src/lib.rs` (empty placeholder)**

```rust
// Game data will be added in future versions.
```

- [ ] **Step 6: Create `CLAUDE.md`**

```markdown
# genshin-calc

原神のダメージ・元素反応計算エンジン（Rust製ライブラリ）

## Project Structure
- Cargoワークスペース構成: `crates/core`（計算エンジン）, `crates/data`（ゲームデータ、将来）
- MIT License

## Development Commands
- `cargo build` — ビルド
- `cargo test` — 全テスト実行
- `cargo test -p genshin-calc-core` — coreのみテスト
- `cargo clippy -- -D warnings` — lint
- `cargo fmt --check` — フォーマット確認

## Conventions
- イミュータブル設計: 構造体は変更せず新規作成
- 純粋関数: 副作用なし、入力→出力が一意
- f64で数値計算（原神の内部仕様に合わせる）
- 浮動小数点テストは許容誤差で比較（assert_eq!禁止）
- WASM互換: stdの重い機能（ファイルI/O等）は使わない
- serde対応: 公開型にはSerialize/Deserializeをderive

## Architecture
- `core`はデータに依存しない純粋な計算ロジック
- ゲームデータ（キャラ、武器等）は`data` crateに分離
- 公開APIは最小限に保つ
```

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml CLAUDE.md crates/
git commit -m "chore: scaffold cargo workspace with core and data crates"
```

---

### Task 2: types.rs — Element + DamageType enums

**Files:**
- Create: `crates/core/src/types.rs`

- [ ] **Step 1: Create `crates/core/src/types.rs`**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    Pyro,
    Hydro,
    Electro,
    Cryo,
    Dendro,
    Anemo,
    Geo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    Normal,
    Charged,
    Plunging,
    Skill,
    Burst,
}
```

- [ ] **Step 2: Add module to lib.rs**

Update `crates/core/src/lib.rs`:

```rust
pub mod types;
```

- [ ] **Step 3: Verify it compiles**

Run: `cargo check -p genshin-calc-core`
Expected: Compiles successfully

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/types.rs crates/core/src/lib.rs
git commit -m "feat: add Element and DamageType enums"
```

---

### Task 3: stats.rs — Stats struct

**Files:**
- Create: `crates/core/src/stats.rs`
- Modify: `crates/core/src/lib.rs`

- [ ] **Step 1: Create `crates/core/src/stats.rs`**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Stats {
    pub hp: f64,
    pub atk: f64,
    pub def: f64,
    pub elemental_mastery: f64,
    pub crit_rate: f64,
    pub crit_dmg: f64,
    pub energy_recharge: f64,
    pub dmg_bonus: f64,
}
```

- [ ] **Step 2: Add module to lib.rs**

Append to `crates/core/src/lib.rs`:

```rust
pub mod stats;
```

- [ ] **Step 3: Verify it compiles**

Run: `cargo check -p genshin-calc-core`
Expected: Compiles successfully

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/stats.rs crates/core/src/lib.rs
git commit -m "feat: add Stats struct"
```

---

### Task 4: enemy.rs — Enemy struct

**Files:**
- Create: `crates/core/src/enemy.rs`
- Modify: `crates/core/src/lib.rs`

- [ ] **Step 1: Create `crates/core/src/enemy.rs`**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Enemy {
    pub level: u32,
    pub resistance: f64,
    pub def_reduction: f64,
}
```

- [ ] **Step 2: Add module to lib.rs**

Append to `crates/core/src/lib.rs`:

```rust
pub mod enemy;
```

- [ ] **Step 3: Verify it compiles**

Run: `cargo check -p genshin-calc-core`
Expected: Compiles successfully

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/enemy.rs crates/core/src/lib.rs
git commit -m "feat: add Enemy struct"
```

---

### Task 5: error.rs — CalcError enum

**Files:**
- Create: `crates/core/src/error.rs`

- [ ] **Step 1: Create `crates/core/src/error.rs`**

```rust
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
}
```

- [ ] **Step 2: Add module to lib.rs**

Append to `crates/core/src/lib.rs`:

```rust
pub mod error;
```

- [ ] **Step 3: Verify it compiles**

Run: `cargo check -p genshin-calc-core`
Expected: Compiles successfully

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/error.rs crates/core/src/lib.rs
git commit -m "feat: add CalcError error type"
```

---

### Task 6: damage.rs — DamageInput, DamageResult structs + validation

**Files:**
- Create: `crates/core/src/damage.rs`
- Modify: `crates/core/src/lib.rs`

- [ ] **Step 1: Add `damage` module to lib.rs**

Append to `crates/core/src/lib.rs`:

```rust
pub mod damage;
```

- [ ] **Step 2: Write failing test for input validation**

Create `crates/core/src/damage.rs` with structs, an empty `calculate_damage` that returns a dummy value, and tests:

```rust
use serde::{Deserialize, Serialize};

use crate::enemy::Enemy;
use crate::error::CalcError;
use crate::stats::Stats;
use crate::types::{DamageType, Element};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageInput {
    pub character_level: u32,
    pub stats: Stats,
    pub talent_multiplier: f64,
    pub damage_type: DamageType,
    pub element: Option<Element>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageResult {
    pub non_crit: f64,
    pub crit: f64,
    pub average: f64,
}

fn validate(input: &DamageInput, enemy: &Enemy) -> Result<(), CalcError> {
    if !(1..=90).contains(&input.character_level) {
        return Err(CalcError::InvalidCharacterLevel(input.character_level));
    }
    if !(1..=100).contains(&enemy.level) {
        return Err(CalcError::InvalidEnemyLevel(enemy.level));
    }
    if !(0.0..=1.0).contains(&input.stats.crit_rate) {
        return Err(CalcError::InvalidCritRate(input.stats.crit_rate));
    }
    if !(0.0..=1.0).contains(&enemy.def_reduction) {
        return Err(CalcError::InvalidDefReduction(enemy.def_reduction));
    }
    if input.talent_multiplier <= 0.0 {
        return Err(CalcError::InvalidTalentMultiplier(input.talent_multiplier));
    }
    Ok(())
}

pub fn calculate_damage(input: &DamageInput, enemy: &Enemy) -> Result<DamageResult, CalcError> {
    validate(input, enemy)?;
    todo!("calculation not yet implemented")
}

#[cfg(test)]
mod tests {
    use super::*;

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
        }
    }

    fn valid_enemy() -> Enemy {
        Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        }
    }

    #[test]
    fn test_invalid_character_level_zero() {
        let input = DamageInput { character_level: 0, ..valid_input() };
        let result = calculate_damage(&input, &valid_enemy());
        assert_eq!(result, Err(CalcError::InvalidCharacterLevel(0)));
    }

    #[test]
    fn test_invalid_character_level_too_high() {
        let input = DamageInput { character_level: 91, ..valid_input() };
        let result = calculate_damage(&input, &valid_enemy());
        assert_eq!(result, Err(CalcError::InvalidCharacterLevel(91)));
    }

    #[test]
    fn test_invalid_enemy_level_zero() {
        let enemy = Enemy { level: 0, ..valid_enemy() };
        let result = calculate_damage(&valid_input(), &enemy);
        assert_eq!(result, Err(CalcError::InvalidEnemyLevel(0)));
    }

    #[test]
    fn test_invalid_enemy_level_too_high() {
        let enemy = Enemy { level: 101, ..valid_enemy() };
        let result = calculate_damage(&valid_input(), &enemy);
        assert_eq!(result, Err(CalcError::InvalidEnemyLevel(101)));
    }

    #[test]
    fn test_invalid_crit_rate_negative() {
        let input = DamageInput {
            stats: Stats { crit_rate: -0.1, ..valid_input().stats },
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert_eq!(result, Err(CalcError::InvalidCritRate(-0.1)));
    }

    #[test]
    fn test_invalid_crit_rate_too_high() {
        let input = DamageInput {
            stats: Stats { crit_rate: 1.1, ..valid_input().stats },
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy());
        assert_eq!(result, Err(CalcError::InvalidCritRate(1.1)));
    }

    #[test]
    fn test_invalid_def_reduction_negative() {
        let enemy = Enemy { def_reduction: -0.1, ..valid_enemy() };
        let result = calculate_damage(&valid_input(), &enemy);
        assert_eq!(result, Err(CalcError::InvalidDefReduction(-0.1)));
    }

    #[test]
    fn test_invalid_def_reduction_too_high() {
        let enemy = Enemy { def_reduction: 1.5, ..valid_enemy() };
        let result = calculate_damage(&valid_input(), &enemy);
        assert_eq!(result, Err(CalcError::InvalidDefReduction(1.5)));
    }

    #[test]
    fn test_invalid_talent_multiplier_zero() {
        let input = DamageInput { talent_multiplier: 0.0, ..valid_input() };
        let result = calculate_damage(&input, &valid_enemy());
        assert_eq!(result, Err(CalcError::InvalidTalentMultiplier(0.0)));
    }

    #[test]
    fn test_invalid_talent_multiplier_negative() {
        let input = DamageInput { talent_multiplier: -1.0, ..valid_input() };
        let result = calculate_damage(&input, &valid_enemy());
        assert_eq!(result, Err(CalcError::InvalidTalentMultiplier(-1.0)));
    }
}
```

- [ ] **Step 3: Run validation tests**

Run: `cargo test -p genshin-calc-core`
Expected: All 10 validation tests PASS. Any test calling `calculate_damage` with valid input will panic (todo!).

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/damage.rs crates/core/src/lib.rs
git commit -m "feat: add DamageInput, DamageResult, validation with tests"
```

---

### Task 7: damage.rs — defense_multiplier implementation

**Files:**
- Modify: `crates/core/src/damage.rs`

- [ ] **Step 1: Write failing tests for defense_multiplier**

Add to the `tests` module in `damage.rs`:

```rust
    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_defense_multiplier_same_level() {
        // Lv90 vs Lv90, no def reduction → 0.5
        let enemy = Enemy { level: 90, resistance: 0.0, def_reduction: 0.0 };
        let result = defense_multiplier(90, &enemy);
        assert!((result - 0.5).abs() < EPSILON);
    }

    #[test]
    fn test_defense_multiplier_low_vs_high() {
        // Lv1 vs Lv90 → 101 / (101 + 190) = 101/291 ≈ 0.347079
        let enemy = Enemy { level: 90, resistance: 0.0, def_reduction: 0.0 };
        let result = defense_multiplier(1, &enemy);
        assert!((result - 101.0 / 291.0).abs() < EPSILON);
    }

    #[test]
    fn test_defense_multiplier_with_def_reduction() {
        // Lv90 vs Lv90, 30% def reduction
        // 190 / (190 + 190 * 0.7) = 190 / 323 ≈ 0.588235
        let enemy = Enemy { level: 90, resistance: 0.0, def_reduction: 0.3 };
        let result = defense_multiplier(90, &enemy);
        assert!((result - 190.0 / 323.0).abs() < EPSILON);
    }

    #[test]
    fn test_defense_multiplier_full_def_reduction() {
        // Lv90 vs Lv90, 100% def reduction
        // 190 / (190 + 190 * 0.0) = 190 / 190 = 1.0
        let enemy = Enemy { level: 90, resistance: 0.0, def_reduction: 1.0 };
        let result = defense_multiplier(90, &enemy);
        assert!((result - 1.0).abs() < EPSILON);
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core defense_multiplier`
Expected: FAIL — `defense_multiplier` function doesn't exist yet

- [ ] **Step 3: Implement defense_multiplier**

Add above `calculate_damage` in `damage.rs`:

```rust
fn defense_multiplier(char_level: u32, enemy: &Enemy) -> f64 {
    let char_part = f64::from(char_level) + 100.0;
    let enemy_part = (f64::from(enemy.level) + 100.0) * (1.0 - enemy.def_reduction);
    char_part / (char_part + enemy_part)
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-core defense_multiplier`
Expected: All 4 tests PASS

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/damage.rs
git commit -m "feat: implement defense_multiplier with tests"
```

---

### Task 8: damage.rs — resistance_multiplier implementation

**Files:**
- Modify: `crates/core/src/damage.rs`

- [ ] **Step 1: Write failing tests for resistance_multiplier**

Add to the `tests` module in `damage.rs`:

```rust
    #[test]
    fn test_resistance_negative() {
        // res = -0.2 → 1.0 - (-0.2)/2 = 1.1
        let enemy = Enemy { level: 90, resistance: -0.2, def_reduction: 0.0 };
        assert!((resistance_multiplier(&enemy) - 1.1).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_zero() {
        // res = 0.0 → 1.0 - 0.0 = 1.0
        let enemy = Enemy { level: 90, resistance: 0.0, def_reduction: 0.0 };
        assert!((resistance_multiplier(&enemy) - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_normal() {
        // res = 0.1 → 1.0 - 0.1 = 0.9
        let enemy = Enemy { level: 90, resistance: 0.1, def_reduction: 0.0 };
        assert!((resistance_multiplier(&enemy) - 0.9).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_boundary_below_75() {
        // res = 0.74 → 1.0 - 0.74 = 0.26
        let enemy = Enemy { level: 90, resistance: 0.74, def_reduction: 0.0 };
        assert!((resistance_multiplier(&enemy) - 0.26).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_at_75() {
        // res = 0.75 → 1/(4*0.75+1) = 1/4 = 0.25
        let enemy = Enemy { level: 90, resistance: 0.75, def_reduction: 0.0 };
        assert!((resistance_multiplier(&enemy) - 0.25).abs() < EPSILON);
    }

    #[test]
    fn test_resistance_high() {
        // res = 0.9 → 1/(4*0.9+1) = 1/4.6 ≈ 0.217391
        let enemy = Enemy { level: 90, resistance: 0.9, def_reduction: 0.0 };
        assert!((resistance_multiplier(&enemy) - 1.0 / 4.6).abs() < EPSILON);
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core resistance_multiplier`
Expected: FAIL — `resistance_multiplier` function doesn't exist yet

- [ ] **Step 3: Implement resistance_multiplier**

Add above `calculate_damage` in `damage.rs`:

```rust
fn resistance_multiplier(enemy: &Enemy) -> f64 {
    let res = enemy.resistance;
    if res < 0.0 {
        1.0 - res / 2.0
    } else if res < 0.75 {
        1.0 - res
    } else {
        1.0 / (4.0 * res + 1.0)
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-core resistance_multiplier`
Expected: All 6 tests PASS

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/damage.rs
git commit -m "feat: implement resistance_multiplier with tests"
```

---

### Task 9: damage.rs — base_damage + calculate_damage implementation

**Files:**
- Modify: `crates/core/src/damage.rs`

- [ ] **Step 1: Write failing tests for calculate_damage (golden tests)**

Add to the `tests` module in `damage.rs`:

```rust
    #[test]
    fn test_base_damage() {
        // ATK 2000 × 倍率1.76 × (1 + 0.466) = 5163.68
        let input = valid_input();
        let result = base_damage(&input);
        assert!((result - 2000.0 * 1.76 * 1.466).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_damage_golden() {
        // ATK 2000, 倍率1.76, bonus 0.466, Lv90 vs Lv90, res 0.1, no def reduction
        // base = 2000 * 1.76 * 1.466 = 5163.68
        // def  = 0.5
        // res  = 0.9
        // non_crit = 5163.68 * 0.5 * 0.9 = 2323.656
        // crit = 2323.656 * (1 + 1.0) = 4647.312
        // avg  = 2323.656 * 0.5 + 4647.312 * 0.5 = 3485.484
        let input = valid_input();
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();
        assert!((result.non_crit - 2323.656).abs() < 0.01);
        assert!((result.crit - 4647.312).abs() < 0.01);
        assert!((result.average - 3485.484).abs() < 0.01);
    }

    #[test]
    fn test_calculate_damage_no_crit() {
        // crit_rate = 0 → average == non_crit
        let input = DamageInput {
            stats: Stats { crit_rate: 0.0, ..valid_input().stats },
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy()).unwrap();
        assert!((result.average - result.non_crit).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_damage_guaranteed_crit() {
        // crit_rate = 1.0 → average == crit
        let input = DamageInput {
            stats: Stats { crit_rate: 1.0, ..valid_input().stats },
            ..valid_input()
        };
        let result = calculate_damage(&input, &valid_enemy()).unwrap();
        assert!((result.average - result.crit).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_damage_physical() {
        // element = None, same math
        let input = DamageInput { element: None, ..valid_input() };
        let result = calculate_damage(&input, &valid_enemy());
        assert!(result.is_ok());
    }

    #[test]
    fn test_calculate_damage_boundary_levels() {
        // Lv1 vs Lv1
        let input = DamageInput { character_level: 1, ..valid_input() };
        let enemy = Enemy { level: 1, resistance: 0.0, def_reduction: 0.0 };
        let result = calculate_damage(&input, &enemy);
        assert!(result.is_ok());
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-core test_calculate_damage`
Expected: FAIL — `calculate_damage` still has `todo!()` for valid inputs

- [ ] **Step 3: Implement base_damage and complete calculate_damage**

Add `base_damage` above `calculate_damage` and replace the `todo!()`:

```rust
fn base_damage(input: &DamageInput) -> f64 {
    input.stats.atk * input.talent_multiplier * (1.0 + input.stats.dmg_bonus)
}

pub fn calculate_damage(input: &DamageInput, enemy: &Enemy) -> Result<DamageResult, CalcError> {
    validate(input, enemy)?;

    let base = base_damage(input);
    let def_mult = defense_multiplier(input.character_level, enemy);
    let res_mult = resistance_multiplier(enemy);

    let non_crit = base * def_mult * res_mult;
    let crit = non_crit * (1.0 + input.stats.crit_dmg);
    let average = non_crit * (1.0 - input.stats.crit_rate)
        + crit * input.stats.crit_rate;

    Ok(DamageResult { non_crit, crit, average })
}
```

- [ ] **Step 4: Run all tests**

Run: `cargo test -p genshin-calc-core`
Expected: ALL tests PASS (validation + defense + resistance + golden + edge cases)

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/damage.rs
git commit -m "feat: implement calculate_damage with golden tests"
```

---

### Task 10: lib.rs — public API re-exports

**Files:**
- Modify: `crates/core/src/lib.rs`

- [ ] **Step 1: Add re-exports to lib.rs**

Add re-export lines to the existing `crates/core/src/lib.rs` (module declarations are already present from previous tasks):

```rust
pub use damage::{calculate_damage, DamageInput, DamageResult};
pub use enemy::Enemy;
pub use error::CalcError;
pub use stats::Stats;
pub use types::{DamageType, Element};
```

- [ ] **Step 2: Verify the public API works with the spec's usage example**

Add an integration-style test to the bottom of `lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_api_usage_example() {
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

        let input = DamageInput {
            character_level: 90,
            stats,
            talent_multiplier: 1.76,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
        };

        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };

        let result = calculate_damage(&input, &enemy).unwrap();
        assert!(result.non_crit > 0.0);
        assert!(result.crit > result.non_crit);
        assert!(result.average > result.non_crit);
        assert!(result.average < result.crit);
    }
}
```

- [ ] **Step 3: Run all tests**

Run: `cargo test -p genshin-calc-core`
Expected: ALL tests PASS

- [ ] **Step 4: Commit**

```bash
git add crates/core/src/lib.rs
git commit -m "feat: add public API re-exports with integration test"
```

---

### Task 11: Quality checks + serde round-trip test

**Files:**
- Modify: `crates/core/src/damage.rs` (add serde test)

- [ ] **Step 1: Add serde round-trip test**

Add to `tests` module in `damage.rs`:

```rust
    #[test]
    fn test_damage_result_serde_roundtrip() {
        let input = valid_input();
        let enemy = valid_enemy();
        let result = calculate_damage(&input, &enemy).unwrap();

        let json = serde_json::to_string(&result).unwrap();
        let deserialized: DamageResult = serde_json::from_str(&json).unwrap();
        assert_eq!(result, deserialized);
    }

    #[test]
    fn test_damage_input_serde_roundtrip() {
        let input = valid_input();
        let json = serde_json::to_string(&input).unwrap();
        let deserialized: DamageInput = serde_json::from_str(&json).unwrap();
        assert_eq!(input, deserialized);
    }
```

- [ ] **Step 2: Run full test suite**

Run: `cargo test`
Expected: ALL tests PASS (both core and data crates)

- [ ] **Step 3: Run clippy**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

- [ ] **Step 4: Run fmt check**

Run: `cargo fmt --check`
Expected: No formatting issues (run `cargo fmt` first if needed)

- [ ] **Step 5: Commit**

```bash
git add crates/core/src/damage.rs
git commit -m "test: add serde round-trip tests"
```

---

### Task 12: Final verification + README update

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Update README.md**

```markdown
# genshin-calc

原神（Genshin Impact）のダメージ計算エンジン。Rust製ライブラリ。

## Features

- 単体キャラのダメージ計算（基礎ダメージ、会心、防御補正、耐性補正）
- 入力バリデーション付き
- serde対応（JSON等でのシリアライズ/デシリアライズ）

## Usage

```rust
use genshin_calc_core::{calculate_damage, DamageInput, DamageType, Stats, Enemy, Element};

let stats = Stats {
    atk: 2000.0,
    crit_rate: 0.75,
    crit_dmg: 1.50,
    dmg_bonus: 0.466,
    ..Stats::default()
};

let input = DamageInput {
    character_level: 90,
    stats,
    talent_multiplier: 1.76,
    damage_type: DamageType::Skill,
    element: Some(Element::Pyro),
};

let enemy = Enemy {
    level: 90,
    resistance: 0.10,
    def_reduction: 0.0,
};

let result = calculate_damage(&input, &enemy).unwrap();
println!("Non-crit: {:.1}", result.non_crit);
println!("Crit: {:.1}", result.crit);
println!("Average: {:.1}", result.average);
```

## Development

```bash
cargo build          # ビルド
cargo test           # テスト実行
cargo clippy         # lint
cargo fmt --check    # フォーマット確認
```

## License

MIT
```

- [ ] **Step 2: Run full verification**

Run: `cargo test && cargo clippy -- -D warnings && cargo fmt --check`
Expected: ALL pass

- [ ] **Step 3: Commit**

```bash
git add README.md
git commit -m "docs: update README with usage example and project description"
```
