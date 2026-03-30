# WASM Bindings Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** genshin-calc の計算エンジン＋ゲームデータをブラウザから使えるWASMパッケージとして提供する

**Architecture:** 新クレート `crates/wasm` に wasm-bindgen ラッパーを実装。core/data の関数を serde-wasm-bindgen 経由で JsValue in/out パターンで公開する。文字列→enum変換はWASM層で手動実装。

**Tech Stack:** wasm-bindgen, serde-wasm-bindgen, console_error_panic_hook, wasm-pack

---

## File Structure

```
crates/wasm/                    # 新規クレート
├── Cargo.toml                  # cdylib + rlib, 依存: core/data/wasm-bindgen/serde-wasm-bindgen
├── .gitignore                  # pkg/ を除外
├── src/
│   ├── lib.rs                  # #[wasm_bindgen] 公開API + init + game_version
│   └── convert.rs              # 文字列→enum変換ヘルパー (parse_element, parse_weapon_type)
└── ts/
    └── types.ts                # 手書きTypeScript型定義（参照用）
Cargo.toml (root)               # workspace members に "crates/wasm" 追加
```

---

### Task 1: クレート骨格 + workspace 設定

**Files:**
- Create: `crates/wasm/Cargo.toml`
- Create: `crates/wasm/.gitignore`
- Create: `crates/wasm/src/lib.rs`
- Modify: `Cargo.toml` (root)

- [ ] **Step 1: ルート `Cargo.toml` に wasm クレートを追加**

`Cargo.toml` (root) を以下に変更:

```toml
[workspace]
members = ["crates/core", "crates/data", "crates/wasm"]
resolver = "2"
```

- [ ] **Step 2: `crates/wasm/Cargo.toml` を作成**

```toml
[package]
name = "genshin-calc-wasm"
version = "0.1.0"
edition = "2024"
description = "WASM bindings for genshin-calc damage calculator"
license = "MIT"
authors = ["kotenbu"]
repository = "https://github.com/kotenbu/genshin-calc"
rust-version = "1.85"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
genshin-calc-core = { version = "0.1.0", path = "../core" }
genshin-calc-data = { version = "0.1.0", path = "../data" }
wasm-bindgen = "0.2"
serde-wasm-bindgen = "0.6"
serde = { version = "1", features = ["derive"] }
console_error_panic_hook = "0.1"

[dev-dependencies]
serde_json = "1"
```

- [ ] **Step 3: `crates/wasm/.gitignore` を作成**

```
pkg/
```

- [ ] **Step 4: 最小限の `crates/wasm/src/lib.rs` を作成**

```rust
use wasm_bindgen::prelude::*;

/// Initialize panic hook for better error messages in browser console.
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Returns the current game data version.
#[wasm_bindgen]
pub fn game_version() -> String {
    genshin_calc_data::GAME_VERSION.to_string()
}
```

- [ ] **Step 5: ビルド確認**

Run: `cargo build -p genshin-calc-wasm`
Expected: 成功（warnings は OK）

- [ ] **Step 6: コミット**

```bash
git add Cargo.toml crates/wasm/
git commit -m "feat(wasm): add crate skeleton with init and game_version"
```

---

### Task 2: 文字列→enum変換モジュール

**Files:**
- Create: `crates/wasm/src/convert.rs`
- Modify: `crates/wasm/src/lib.rs` (mod宣言追加)

- [ ] **Step 1: テスト付きで `convert.rs` を作成**

```rust
use genshin_calc_core::{Element, WeaponType};

/// Parses a lowercase element string to Element enum.
///
/// Accepts: "pyro", "hydro", "electro", "cryo", "anemo", "geo", "dendro"
pub fn parse_element(s: &str) -> Result<Element, String> {
    match s {
        "pyro" => Ok(Element::Pyro),
        "hydro" => Ok(Element::Hydro),
        "electro" => Ok(Element::Electro),
        "cryo" => Ok(Element::Cryo),
        "anemo" => Ok(Element::Anemo),
        "geo" => Ok(Element::Geo),
        "dendro" => Ok(Element::Dendro),
        _ => Err(format!("Unknown element: '{s}'. Expected one of: pyro, hydro, electro, cryo, anemo, geo, dendro")),
    }
}

/// Parses a lowercase weapon type string to WeaponType enum.
///
/// Accepts: "sword", "claymore", "polearm", "bow", "catalyst"
pub fn parse_weapon_type(s: &str) -> Result<WeaponType, String> {
    match s {
        "sword" => Ok(WeaponType::Sword),
        "claymore" => Ok(WeaponType::Claymore),
        "polearm" => Ok(WeaponType::Polearm),
        "bow" => Ok(WeaponType::Bow),
        "catalyst" => Ok(WeaponType::Catalyst),
        _ => Err(format!("Unknown weapon type: '{s}'. Expected one of: sword, claymore, polearm, bow, catalyst")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_element_all_valid() {
        assert_eq!(parse_element("pyro").unwrap(), Element::Pyro);
        assert_eq!(parse_element("hydro").unwrap(), Element::Hydro);
        assert_eq!(parse_element("electro").unwrap(), Element::Electro);
        assert_eq!(parse_element("cryo").unwrap(), Element::Cryo);
        assert_eq!(parse_element("anemo").unwrap(), Element::Anemo);
        assert_eq!(parse_element("geo").unwrap(), Element::Geo);
        assert_eq!(parse_element("dendro").unwrap(), Element::Dendro);
    }

    #[test]
    fn test_parse_element_invalid() {
        assert!(parse_element("fire").is_err());
        assert!(parse_element("Pyro").is_err()); // PascalCase は不正
        assert!(parse_element("").is_err());
    }

    #[test]
    fn test_parse_weapon_type_all_valid() {
        assert_eq!(parse_weapon_type("sword").unwrap(), WeaponType::Sword);
        assert_eq!(parse_weapon_type("claymore").unwrap(), WeaponType::Claymore);
        assert_eq!(parse_weapon_type("polearm").unwrap(), WeaponType::Polearm);
        assert_eq!(parse_weapon_type("bow").unwrap(), WeaponType::Bow);
        assert_eq!(parse_weapon_type("catalyst").unwrap(), WeaponType::Catalyst);
    }

    #[test]
    fn test_parse_weapon_type_invalid() {
        assert!(parse_weapon_type("greatsword").is_err());
        assert!(parse_weapon_type("Sword").is_err());
        assert!(parse_weapon_type("").is_err());
    }
}
```

- [ ] **Step 2: `lib.rs` に mod 宣言を追加**

`lib.rs` の先頭（`use wasm_bindgen::prelude::*;` の前）に追加:

```rust
mod convert;
```

- [ ] **Step 3: テスト実行**

Run: `cargo test -p genshin-calc-wasm`
Expected: 全テスト PASS

- [ ] **Step 4: コミット**

```bash
git add crates/wasm/src/
git commit -m "feat(wasm): add string-to-enum conversion helpers with tests"
```

---

### Task 3: データ検索関数

**Files:**
- Modify: `crates/wasm/src/lib.rs`

- [ ] **Step 1: データ検索のテストを追加**

`lib.rs` の末尾に以下を追加:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert;

    #[test]
    fn test_game_version_returns_string() {
        let v = game_version();
        assert!(!v.is_empty());
    }

    #[test]
    fn test_find_character_serde_roundtrip() {
        let diluc = genshin_calc_data::find_character("diluc").unwrap();
        let json = serde_json::to_value(diluc).unwrap();
        assert_eq!(json["name"], "Diluc");
        assert_eq!(json["element"], "Pyro");
    }

    #[test]
    fn test_find_character_not_found() {
        assert!(genshin_calc_data::find_character("nonexistent").is_none());
    }

    #[test]
    fn test_find_weapon_serde_roundtrip() {
        let weapon = genshin_calc_data::find_weapon("wolfs_gravestone").unwrap();
        let json = serde_json::to_value(weapon).unwrap();
        assert_eq!(json["id"], "wolfs_gravestone");
    }

    #[test]
    fn test_find_artifact_set_serde_roundtrip() {
        let set = genshin_calc_data::find_artifact_set("crimson_witch").unwrap();
        let json = serde_json::to_value(set).unwrap();
        assert_eq!(json["id"], "crimson_witch");
    }

    #[test]
    fn test_find_enemy_serde_roundtrip() {
        let enemy = genshin_calc_data::find_enemy("hilichurl").unwrap();
        let json = serde_json::to_value(enemy).unwrap();
        assert_eq!(json["id"], "hilichurl");
    }

    #[test]
    fn test_characters_by_element_pyro() {
        let element = convert::parse_element("pyro").unwrap();
        let chars = genshin_calc_data::characters_by_element(element);
        assert!(chars.len() > 10);
    }

    #[test]
    fn test_weapons_by_type_sword() {
        let wt = convert::parse_weapon_type("sword").unwrap();
        let weapons = genshin_calc_data::weapons_by_type(wt);
        assert!(!weapons.is_empty());
    }
}
```

- [ ] **Step 2: テスト実行（RED確認は不要 — serde_json roundtripの正常系確認）**

Run: `cargo test -p genshin-calc-wasm`
Expected: 全テスト PASS（ラッパー関数はまだないがロジック確認のため先にテスト）

- [ ] **Step 3: データ検索の `#[wasm_bindgen]` 関数を実装**

`lib.rs` の `game_version` 関数の後に追加:

```rust
/// Finds a character by ID (lowercase, e.g. "diluc", "hu_tao").
/// Returns the character data as a JS object, or null if not found.
#[wasm_bindgen]
pub fn find_character(id: &str) -> JsValue {
    match genshin_calc_data::find_character(id) {
        Some(c) => serde_wasm_bindgen::to_value(c).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Finds a weapon by ID (lowercase, e.g. "wolfs_gravestone").
/// Returns the weapon data as a JS object, or null if not found.
#[wasm_bindgen]
pub fn find_weapon(id: &str) -> JsValue {
    match genshin_calc_data::find_weapon(id) {
        Some(w) => serde_wasm_bindgen::to_value(w).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Finds an artifact set by ID (lowercase, e.g. "crimson_witch").
/// Returns the artifact set data as a JS object, or null if not found.
#[wasm_bindgen]
pub fn find_artifact_set(id: &str) -> JsValue {
    match genshin_calc_data::find_artifact_set(id) {
        Some(a) => serde_wasm_bindgen::to_value(a).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Finds an enemy by ID (lowercase, e.g. "hilichurl").
/// Returns the enemy data as a JS object, or null if not found.
#[wasm_bindgen]
pub fn find_enemy(id: &str) -> JsValue {
    match genshin_calc_data::find_enemy(id) {
        Some(e) => serde_wasm_bindgen::to_value(e).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Returns all characters with the given element.
/// Element is a lowercase string: "pyro", "hydro", "electro", "cryo", "anemo", "geo", "dendro".
#[wasm_bindgen]
pub fn characters_by_element(element: &str) -> Result<JsValue, JsError> {
    let elem = convert::parse_element(element).map_err(|e| JsError::new(&e))?;
    let chars = genshin_calc_data::characters_by_element(elem);
    serde_wasm_bindgen::to_value(&chars).map_err(|e| JsError::new(&e.to_string()))
}

/// Returns all weapons of the given type.
/// Weapon type is a lowercase string: "sword", "claymore", "polearm", "bow", "catalyst".
#[wasm_bindgen]
pub fn weapons_by_type(weapon_type: &str) -> Result<JsValue, JsError> {
    let wt = convert::parse_weapon_type(weapon_type).map_err(|e| JsError::new(&e))?;
    let weapons = genshin_calc_data::weapons_by_type(wt);
    serde_wasm_bindgen::to_value(&weapons).map_err(|e| JsError::new(&e.to_string()))
}
```

注: `genshin_calc_data::types::WeaponType` は `genshin_calc_core::WeaponType` の re-export（`pub use genshin_calc_core::WeaponType;`）なので、`convert::parse_weapon_type` の返り値をそのまま `genshin_calc_data::weapons_by_type` に渡せる。

- [ ] **Step 4: ビルド＋テスト**

Run: `cargo test -p genshin-calc-wasm && cargo build -p genshin-calc-wasm`
Expected: 全テスト PASS、ビルド成功

- [ ] **Step 5: コミット**

```bash
git add crates/wasm/src/lib.rs
git commit -m "feat(wasm): add data lookup functions (find_character/weapon/artifact/enemy)"
```

---

### Task 4: 計算関数

**Files:**
- Modify: `crates/wasm/src/lib.rs`

- [ ] **Step 1: 計算関数のテストを追加**

`lib.rs` の `tests` モジュール内に追加:

```rust
    #[test]
    fn test_damage_input_serde_roundtrip() {
        use genshin_calc_core::*;
        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                hp: 20000.0, atk: 2000.0, def: 800.0,
                elemental_mastery: 100.0, crit_rate: 0.75,
                crit_dmg: 1.50, energy_recharge: 1.20, dmg_bonus: 0.466,
            },
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let json = serde_json::to_string(&input).unwrap();
        let back: DamageInput = serde_json::from_str(&json).unwrap();
        assert_eq!(input.character_level, back.character_level);
        assert!((input.talent_multiplier - back.talent_multiplier).abs() < 1e-10);
    }

    #[test]
    fn test_damage_calculation_via_core() {
        use genshin_calc_core::*;
        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                hp: 20000.0, atk: 2000.0, def: 800.0,
                elemental_mastery: 100.0, crit_rate: 0.75,
                crit_dmg: 1.50, energy_recharge: 1.20, dmg_bonus: 0.466,
            },
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
        let result = calculate_damage(&input, &enemy).unwrap();
        assert!(result.average > 0.0);
        assert!(result.crit > result.non_crit);
    }

    #[test]
    fn test_transformative_calculation_via_core() {
        use genshin_calc_core::*;
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 200.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
        };
        let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
        let result = calculate_transformative(&input, &enemy).unwrap();
        assert!(result.damage > 0.0);
    }

    #[test]
    fn test_lunar_calculation_via_core() {
        use genshin_calc_core::*;
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 200.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 0.5,
            crit_dmg: 1.0,
            base_dmg_bonus: 0.0,
        };
        let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
        let result = calculate_lunar(&input, &enemy).unwrap();
        assert!(result.average > 0.0);
    }

    #[test]
    fn test_resolve_team_stats_via_core() {
        use genshin_calc_core::*;
        let dps = TeamMember {
            element: Element::Pyro,
            weapon_type: WeaponType::Claymore,
            stats: StatProfile {
                base_atk: 900.0,
                crit_rate: 0.60,
                crit_dmg: 1.50,
                energy_recharge: 1.0,
                ..Default::default()
            },
            buffs_provided: vec![],
            is_moonsign: false,
        };
        let result = resolve_team_stats(&[dps], 0).unwrap();
        assert!(result.atk > 0.0);
    }

    #[test]
    fn test_reaction_swirl_serde() {
        use genshin_calc_core::Reaction;
        let swirl = Reaction::Swirl(genshin_calc_core::Element::Pyro);
        let json = serde_json::to_string(&swirl).unwrap();
        assert_eq!(json, r#"{"Swirl":"Pyro"}"#);
        let back: Reaction = serde_json::from_str(&json).unwrap();
        assert_eq!(swirl, back);
    }
```

- [ ] **Step 2: テスト実行**

Run: `cargo test -p genshin-calc-wasm`
Expected: 全テスト PASS

- [ ] **Step 3: 計算関数の `#[wasm_bindgen]` ラッパーを実装**

`lib.rs` のデータ検索関数の後に追加:

```rust
/// Calculates standard damage (ATK/HP/DEF scaling with crit, defense, resistance).
///
/// # Arguments
/// * `input` - DamageInput as a JS object (PascalCase enum variants, e.g. element: "Pyro")
/// * `enemy` - Enemy as a JS object
///
/// # Returns
/// DamageResult as a JS object with non_crit, crit, average, reaction fields.
///
/// # Errors
/// Throws JsError on invalid input or calculation error.
#[wasm_bindgen]
pub fn calculate_damage(input: JsValue, enemy: JsValue) -> Result<JsValue, JsError> {
    let input: genshin_calc_core::DamageInput =
        serde_wasm_bindgen::from_value(input).map_err(|e| JsError::new(&format!("Invalid input: {e}")))?;
    let enemy: genshin_calc_core::Enemy =
        serde_wasm_bindgen::from_value(enemy).map_err(|e| JsError::new(&format!("Invalid enemy: {e}")))?;
    let result = genshin_calc_core::calculate_damage(&input, &enemy)
        .map_err(|e| JsError::new(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
}

/// Calculates transformative reaction damage (overloaded, superconduct, swirl, etc.).
///
/// # Arguments
/// * `input` - TransformativeInput as a JS object
/// * `enemy` - Enemy as a JS object
///
/// # Returns
/// TransformativeResult as a JS object with damage and damage_element fields.
#[wasm_bindgen]
pub fn calculate_transformative(input: JsValue, enemy: JsValue) -> Result<JsValue, JsError> {
    let input: genshin_calc_core::TransformativeInput =
        serde_wasm_bindgen::from_value(input).map_err(|e| JsError::new(&format!("Invalid input: {e}")))?;
    let enemy: genshin_calc_core::Enemy =
        serde_wasm_bindgen::from_value(enemy).map_err(|e| JsError::new(&format!("Invalid enemy: {e}")))?;
    let result = genshin_calc_core::calculate_transformative(&input, &enemy)
        .map_err(|e| JsError::new(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
}

/// Calculates lunar reaction damage (Nod-Krai exclusive crittable reactions).
///
/// # Arguments
/// * `input` - LunarInput as a JS object
/// * `enemy` - Enemy as a JS object
///
/// # Returns
/// LunarResult as a JS object with non_crit, crit, average, damage_element fields.
#[wasm_bindgen]
pub fn calculate_lunar(input: JsValue, enemy: JsValue) -> Result<JsValue, JsError> {
    let input: genshin_calc_core::LunarInput =
        serde_wasm_bindgen::from_value(input).map_err(|e| JsError::new(&format!("Invalid input: {e}")))?;
    let enemy: genshin_calc_core::Enemy =
        serde_wasm_bindgen::from_value(enemy).map_err(|e| JsError::new(&format!("Invalid enemy: {e}")))?;
    let result = genshin_calc_core::calculate_lunar(&input, &enemy)
        .map_err(|e| JsError::new(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
}

/// Resolves team buffs and returns final stats for the target member.
///
/// # Arguments
/// * `members` - Array of TeamMember objects
/// * `target_index` - Index of the DPS/target member (0-based)
///
/// # Returns
/// Stats as a JS object with hp, atk, def, elemental_mastery, crit_rate, crit_dmg, energy_recharge, dmg_bonus.
#[wasm_bindgen]
pub fn resolve_team_stats(members: JsValue, target_index: u32) -> Result<JsValue, JsError> {
    let members: Vec<genshin_calc_core::TeamMember> =
        serde_wasm_bindgen::from_value(members).map_err(|e| JsError::new(&format!("Invalid members: {e}")))?;
    let result = genshin_calc_core::resolve_team_stats(&members, target_index as usize)
        .map_err(|e| JsError::new(&e.to_string()))?;
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsError::new(&e.to_string()))
}
```

- [ ] **Step 4: ビルド＋テスト**

Run: `cargo test -p genshin-calc-wasm && cargo build -p genshin-calc-wasm`
Expected: 全テスト PASS、ビルド成功

- [ ] **Step 5: コミット**

```bash
git add crates/wasm/src/lib.rs
git commit -m "feat(wasm): add calculation functions (damage/transformative/lunar/team)"
```

---

### Task 5: TypeScript型定義

**Files:**
- Create: `crates/wasm/ts/types.ts`

- [ ] **Step 1: `crates/wasm/ts/types.ts` を作成**

スペックの「TypeScript型定義」セクションの全内容をそのままコピー:

```typescript
// genshin-calc-wasm TypeScript type definitions
// These types describe the JSON shapes used by the WASM API.
// Import or reference these when calling WASM functions from TypeScript.

// === Core Types ===

export interface Stats {
  hp: number;
  atk: number;
  def: number;
  elemental_mastery: number;
  crit_rate: number;
  crit_dmg: number;
  energy_recharge: number;
  dmg_bonus: number;
}

export type Element = "Pyro" | "Hydro" | "Electro" | "Cryo" | "Anemo" | "Geo" | "Dendro";
export type ScalingStat = "Atk" | "Hp" | "Def" | "Em";
export type DamageType = "Normal" | "Charged" | "Plunging" | "Skill" | "Burst";
export type WeaponType = "Sword" | "Claymore" | "Polearm" | "Bow" | "Catalyst";

// Reaction: simple variants are strings, Swirl is an object with the swirled element
export type Reaction =
  | "Vaporize" | "Melt"
  | "Aggravate" | "Spread"
  | "Overloaded" | "Superconduct" | "ElectroCharged" | "Shattered"
  | { Swirl: Element }
  | "Bloom" | "Hyperbloom" | "Burgeon" | "Burning"
  | "LunarElectroCharged" | "LunarBloom" | "LunarCrystallize" | "LunarCrystallizeSecondary";

export type BuffTarget = "OnlySelf" | "Team" | "TeamExcludeSelf";

export type BuffableStat =
  | "HpPercent" | "AtkPercent" | "DefPercent"
  | "HpFlat" | "AtkFlat" | "DefFlat"
  | "CritRate" | "CritDmg"
  | "ElementalMastery" | "EnergyRecharge"
  | "DmgBonus"
  | "NormalDmgBonus" | "ChargedDmgBonus" | "PlungingDmgBonus"
  | "SkillDmgBonus" | "BurstDmgBonus"
  | "PyroDmgBonus" | "HydroDmgBonus" | "ElectroDmgBonus"
  | "CryoDmgBonus" | "AnemoDmgBonus" | "GeoDmgBonus" | "DendroDmgBonus"
  | "PhysicalDmgBonus"
  | "HealingBonus" | "ShieldStrength"
  | "DefReduction" | "ResistanceReduction"
  | "DefIgnore"
  | "AmplifyingBonus" | "TransformativeBonus" | "CatalyzeBonus"
  | "BaseAtkPercent"
  | "AllElementalDmgBonus"
  | "FlatDmg";

// === Input/Output Types ===

export interface Enemy {
  level: number;
  resistance: number;
  def_reduction: number;
}

export interface DamageInput {
  character_level: number;
  stats: Stats;
  talent_multiplier: number;
  scaling_stat: ScalingStat;
  damage_type: DamageType;
  element: Element | null;
  reaction: Reaction | null;
  reaction_bonus: number;
  flat_dmg: number;
}

export interface DamageResult {
  non_crit: number;
  crit: number;
  average: number;
  reaction: Reaction | null;
}

export interface TransformativeInput {
  character_level: number;
  elemental_mastery: number;
  reaction: Reaction;
  reaction_bonus: number;
}

export interface TransformativeResult {
  damage: number;
  damage_element: Element | null;
}

export interface LunarInput {
  character_level: number;
  elemental_mastery: number;
  reaction: Reaction;
  reaction_bonus: number;
  crit_rate: number;
  crit_dmg: number;
  base_dmg_bonus: number;
}

export interface LunarResult {
  non_crit: number;
  crit: number;
  average: number;
  damage_element: Element | null;
}

// === Team Types ===

export interface StatProfile {
  base_hp: number;
  base_atk: number;
  base_def: number;
  hp_percent: number;
  atk_percent: number;
  def_percent: number;
  hp_flat: number;
  atk_flat: number;
  def_flat: number;
  elemental_mastery: number;
  crit_rate: number;
  crit_dmg: number;
  energy_recharge: number;
  dmg_bonus: number;
}

export interface ResolvedBuff {
  source: string;
  stat: BuffableStat;
  value: number;
  target: BuffTarget;
}

export interface TeamMember {
  element: Element;
  weapon_type: WeaponType;
  stats: StatProfile;
  buffs_provided: ResolvedBuff[];
  is_moonsign: boolean;
}
```

- [ ] **Step 2: コミット**

```bash
git add crates/wasm/ts/
git commit -m "docs(wasm): add TypeScript type definitions"
```

---

### Task 6: wasm-pack ビルド検証

**Files:**
- なし（ビルド検証のみ）

- [ ] **Step 1: wasm-pack がインストールされているか確認**

Run: `wasm-pack --version`
Expected: バージョンが表示される。なければ `cargo install wasm-pack` を実行。

- [ ] **Step 2: wasm-pack ビルド**

Run: `wasm-pack build crates/wasm --target web`
Expected: `pkg/` に以下が生成される:
- `genshin_calc_wasm_bg.wasm`
- `genshin_calc_wasm.js`
- `genshin_calc_wasm.d.ts`
- `package.json`

- [ ] **Step 3: 生成された `.d.ts` に公開関数が含まれているか確認**

Run: `grep -E "^export function" crates/wasm/pkg/genshin_calc_wasm.d.ts`
Expected: 以下の関数が全て含まれる:
- `game_version`
- `find_character`
- `find_weapon`
- `find_artifact_set`
- `find_enemy`
- `characters_by_element`
- `weapons_by_type`
- `calculate_damage`
- `calculate_transformative`
- `calculate_lunar`
- `resolve_team_stats`

- [ ] **Step 4: 全体テスト + clippy**

Run: `cargo test && cargo clippy -- -D warnings`
Expected: 全テスト PASS、clippy警告なし

- [ ] **Step 5: pkg/ を clean up（.gitignore対象なので不要だがワークツリー上のゴミを除去）**

Run: `rm -rf crates/wasm/pkg`

- [ ] **Step 6: clippy修正があればコミット**

clippy警告の修正があった場合のみ:
```bash
git add crates/wasm/
git commit -m "fix(wasm): resolve clippy warnings"
```
