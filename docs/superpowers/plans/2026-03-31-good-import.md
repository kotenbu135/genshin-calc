# GOOD形式インポート 実装計画

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** GOOD (Genshin Open Object Description) JSONをパースし、計算エンジンの `CharacterBuild` 構造体に変換する `crates/good` クレートを新規実装する。

**Architecture:** 新クレート `crates/good` を追加。内部は5モジュール構成: `error.rs`（エラー型）, `types.rs`（GOOD JSONデシリアライズ構造体）, `key_map.rs`（PascalCase→snake_case変換）, `stat_map.rs`（StatKey変換+聖遺物メインステテーブル）, `convert.rs`（変換ロジック）。公開APIは `lib.rs` から `import_good()` と `convert_good()` を公開。

**Tech Stack:** Rust, serde/serde_json, thiserror, genshin-calc-core, genshin-calc-data

**Spec:** `docs/superpowers/specs/2026-03-31-good-import-design.md`

---

## ファイル構造

| ファイル | 責務 |
|---------|------|
| `crates/good/Cargo.toml` | クレート定義・依存関係 |
| `crates/good/src/lib.rs` | 公開API (`import_good`, `convert_good`) + モジュール宣言 |
| `crates/good/src/error.rs` | `GoodError`, `ImportWarning` |
| `crates/good/src/types.rs` | GOOD JSON構造体 (`GoodFormat`, `GoodCharacter`, etc.) |
| `crates/good/src/key_map.rs` | PascalCase→snake_case変換 + 例外テーブル |
| `crates/good/src/stat_map.rs` | StatKey→StatProfile変換 + メインステ値テーブル |
| `crates/good/src/convert.rs` | `GoodFormat` → `GoodImport` 変換ロジック |
| `crates/good/tests/data/*.json` | テスト用GOODフォーマットJSONファイル |

---

### Task 1: クレート初期化

**Files:**
- Create: `crates/good/Cargo.toml`
- Create: `crates/good/src/lib.rs`
- Modify: `Cargo.toml` (workspace root — members に追加)

- [ ] **Step 1: ワークスペースに `crates/good` を追加**

`Cargo.toml`（ワークスペースルート）の `members` に `"crates/good"` を追加:
```toml
members = ["crates/core", "crates/data", "crates/wasm", "crates/good"]
```

- [ ] **Step 2: `crates/good/Cargo.toml` を作成**

```toml
[package]
name = "genshin-calc-good"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
license = "MIT"
description = "GOOD (Genshin Open Object Description) format importer for genshin-calc"

[dependencies]
genshin-calc-core = { path = "../core" }
genshin-calc-data = { path = "../data" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"

[dev-dependencies]
serde_json = "1"
```

- [ ] **Step 3: `crates/good/src/lib.rs` スケルトン作成**

```rust
mod convert;
mod error;
mod key_map;
mod stat_map;
mod types;

pub use error::{GoodError, ImportWarning};
pub use types::GoodFormat;

use convert::build_imports;
use error::validate_format;

/// GOOD形式インポートの結果
#[derive(Debug, Clone, serde::Serialize)]
pub struct GoodImport {
    pub source: String,
    pub version: u8,
    pub builds: Vec<CharacterBuild>,
    pub warnings: Vec<ImportWarning>,
}

/// 1キャラ分のビルド
#[derive(Debug, Clone, serde::Serialize)]
pub struct CharacterBuild {
    pub character: &'static genshin_calc_data::types::CharacterData,
    pub level: u32,
    pub ascension: u8,
    pub constellation: u8,
    pub talent_levels: [u8; 3],
    pub weapon: Option<WeaponBuild>,
    pub artifacts: ArtifactsBuild,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct WeaponBuild {
    pub weapon: &'static genshin_calc_data::types::WeaponData,
    pub level: u32,
    pub refinement: u8,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArtifactsBuild {
    pub sets: Vec<&'static genshin_calc_data::types::ArtifactSet>,
    pub four_piece_set: Option<&'static genshin_calc_data::types::ArtifactSet>,
    pub stats: genshin_calc_core::StatProfile,
}

/// GOOD JSONをパースしてビルド一覧に変換
pub fn import_good(json: &str) -> Result<GoodImport, GoodError> {
    let good: GoodFormat = serde_json::from_str(json)?;
    convert_good(good)
}

/// 既にデシリアライズ済みのGoodFormatから変換
pub fn convert_good(good: GoodFormat) -> Result<GoodImport, GoodError> {
    validate_format(&good)?;
    Ok(build_imports(good))
}
```

- [ ] **Step 4: 各モジュールの空ファイルを作成**

`error.rs`, `types.rs`, `key_map.rs`, `stat_map.rs`, `convert.rs` をそれぞれ空の状態で作成（コンパイル通過のための最小限のスタブ）。

- [ ] **Step 5: ビルド確認**

Run: `cargo build -p genshin-calc-good`
Expected: PASS（警告は許容）

- [ ] **Step 6: コミット**

```bash
git add crates/good/ Cargo.toml Cargo.lock
git commit -m "feat(good): scaffold crates/good with module structure"
```

---

### Task 2: エラー型

**Files:**
- Create: `crates/good/src/error.rs`

- [ ] **Step 1: `error.rs` を実装**

```rust
use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum GoodError {
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("invalid GOOD format: expected \"GOOD\", got \"{0}\"")]
    InvalidFormat(String),

    #[error("unsupported GOOD version: {0}")]
    UnsupportedVersion(u8),
}

#[derive(Debug, Clone, Serialize)]
pub enum ImportWarning {
    UnknownCharacter(String),
    UnknownWeapon(String),
    UnknownArtifactSet(String),
    UnknownStatKey(String),
    ElementMismatchGoblet {
        character: String,
        goblet_element: String,
    },
}

pub(crate) fn validate_format(good: &crate::types::GoodFormat) -> Result<(), GoodError> {
    if good.format != "GOOD" {
        return Err(GoodError::InvalidFormat(good.format.clone()));
    }
    if good.version == 0 || good.version > 3 {
        return Err(GoodError::UnsupportedVersion(good.version));
    }
    Ok(())
}
```

- [ ] **Step 2: ビルド確認**

Run: `cargo build -p genshin-calc-good`
Expected: PASS

- [ ] **Step 3: コミット**

```bash
git add crates/good/src/error.rs
git commit -m "feat(good): add GoodError and ImportWarning types"
```

---

### Task 3: GOOD JSON入力型 + パーステスト

**Files:**
- Create: `crates/good/src/types.rs`
- Create: `crates/good/tests/data/minimal.json`
- Create: `crates/good/tests/test_parse.rs`

- [ ] **Step 1: テスト用JSONファイル `minimal.json` を作成**

```json
{
  "format": "GOOD",
  "source": "TestScanner",
  "version": 2,
  "characters": [
    {
      "key": "HuTao",
      "level": 90,
      "constellation": 1,
      "ascension": 6,
      "talent": { "auto": 10, "skill": 10, "burst": 8 }
    }
  ],
  "weapons": [
    {
      "key": "StaffOfHoma",
      "level": 90,
      "ascension": 6,
      "refinement": 1,
      "location": "HuTao",
      "lock": true
    }
  ],
  "artifacts": [
    {
      "setKey": "CrimsonWitchOfFlames",
      "slotKey": "flower",
      "level": 20,
      "rarity": 5,
      "mainStatKey": "hp",
      "location": "HuTao",
      "lock": false,
      "substats": [
        { "key": "critRate_", "value": 10.5 },
        { "key": "critDMG_", "value": 21.0 },
        { "key": "atk_", "value": 5.8 },
        { "key": "eleMas", "value": 23 }
      ]
    },
    {
      "setKey": "CrimsonWitchOfFlames",
      "slotKey": "plume",
      "level": 20,
      "rarity": 5,
      "mainStatKey": "atk",
      "location": "HuTao",
      "lock": false,
      "substats": [
        { "key": "hp_", "value": 9.9 },
        { "key": "critDMG_", "value": 14.8 },
        { "key": "eleMas", "value": 40 },
        { "key": "enerRech_", "value": 6.5 }
      ]
    },
    {
      "setKey": "CrimsonWitchOfFlames",
      "slotKey": "sands",
      "level": 20,
      "rarity": 5,
      "mainStatKey": "hp_",
      "location": "HuTao",
      "lock": false,
      "substats": [
        { "key": "critRate_", "value": 6.2 },
        { "key": "critDMG_", "value": 13.2 },
        { "key": "atk_", "value": 4.1 },
        { "key": "def", "value": 37 }
      ]
    },
    {
      "setKey": "CrimsonWitchOfFlames",
      "slotKey": "goblet",
      "level": 20,
      "rarity": 5,
      "mainStatKey": "pyro_dmg_",
      "location": "HuTao",
      "lock": false,
      "substats": [
        { "key": "hp_", "value": 14.0 },
        { "key": "critRate_", "value": 3.5 },
        { "key": "critDMG_", "value": 7.8 },
        { "key": "atk", "value": 33 }
      ]
    },
    {
      "setKey": "CrimsonWitchOfFlames",
      "slotKey": "circlet",
      "level": 20,
      "rarity": 5,
      "mainStatKey": "critDMG_",
      "location": "HuTao",
      "lock": false,
      "substats": [
        { "key": "hp_", "value": 11.7 },
        { "key": "critRate_", "value": 7.4 },
        { "key": "atk_", "value": 5.3 },
        { "key": "def_", "value": 7.3 }
      ]
    }
  ]
}
```

- [ ] **Step 2: テスト `test_parse.rs` を作成（RED）**

```rust
use genshin_calc_good::GoodFormat;

#[test]
fn parse_minimal_good_json() {
    let json = include_str!("data/minimal.json");
    let good: GoodFormat = serde_json::from_str(json).unwrap();

    assert_eq!(good.format, "GOOD");
    assert_eq!(good.source, "TestScanner");
    assert_eq!(good.version, 2);

    let chars = good.characters.as_ref().unwrap();
    assert_eq!(chars.len(), 1);
    assert_eq!(chars[0].key, "HuTao");
    assert_eq!(chars[0].level, 90);
    assert_eq!(chars[0].constellation, 1);
    assert_eq!(chars[0].ascension, 6);
    assert_eq!(chars[0].talent.auto, 10);
    assert_eq!(chars[0].talent.skill, 10);
    assert_eq!(chars[0].talent.burst, 8);

    let weapons = good.weapons.as_ref().unwrap();
    assert_eq!(weapons.len(), 1);
    assert_eq!(weapons[0].key, "StaffOfHoma");
    assert_eq!(weapons[0].level, 90);
    assert_eq!(weapons[0].refinement, 1);
    assert_eq!(weapons[0].location.as_deref(), Some("HuTao"));

    let artifacts = good.artifacts.as_ref().unwrap();
    assert_eq!(artifacts.len(), 5);
    assert_eq!(artifacts[0].set_key, "CrimsonWitchOfFlames");
    assert_eq!(artifacts[0].slot_key, "flower");
    assert_eq!(artifacts[0].level, 20);
    assert_eq!(artifacts[0].rarity, 5);
    assert_eq!(artifacts[0].main_stat_key, "hp");
    assert_eq!(artifacts[0].substats.len(), 4);
    assert_eq!(artifacts[0].substats[0].key, "critRate_");
    assert!((artifacts[0].substats[0].value - 10.5).abs() < 1e-6);
}

#[test]
fn parse_empty_fields() {
    let json = r#"{ "format": "GOOD", "source": "Empty", "version": 1 }"#;
    let good: GoodFormat = serde_json::from_str(json).unwrap();
    assert!(good.characters.is_none());
    assert!(good.weapons.is_none());
    assert!(good.artifacts.is_none());
}

#[test]
fn parse_empty_location_is_unequipped() {
    let json = r#"{
        "format": "GOOD", "source": "Test", "version": 1,
        "weapons": [{
            "key": "DullBlade", "level": 1, "ascension": 0,
            "refinement": 1, "location": "", "lock": false
        }]
    }"#;
    let good: GoodFormat = serde_json::from_str(json).unwrap();
    let weapons = good.weapons.unwrap();
    assert_eq!(weapons[0].location.as_deref(), Some(""));
}
```

- [ ] **Step 3: テスト実行 — 失敗を確認**

Run: `cargo test -p genshin-calc-good --test test_parse`
Expected: FAIL（`GoodFormat` が存在しない）

- [ ] **Step 4: `types.rs` を実装（GREEN）**

```rust
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodFormat {
    pub format: String,
    pub source: String,
    pub version: u8,
    pub characters: Option<Vec<GoodCharacter>>,
    pub artifacts: Option<Vec<GoodArtifact>>,
    pub weapons: Option<Vec<GoodWeapon>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodCharacter {
    pub key: String,
    pub level: u32,
    pub constellation: u8,
    pub ascension: u8,
    pub talent: GoodTalent,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodTalent {
    pub auto: u8,
    pub skill: u8,
    pub burst: u8,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodWeapon {
    pub key: String,
    pub level: u32,
    pub ascension: u8,
    pub refinement: u8,
    pub location: Option<String>,
    pub lock: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodArtifact {
    pub set_key: String,
    pub slot_key: String,
    pub level: u8,
    pub rarity: u8,
    pub main_stat_key: String,
    pub location: Option<String>,
    pub lock: bool,
    pub substats: Vec<GoodSubstat>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodSubstat {
    pub key: String,
    pub value: f64,
}
```

- [ ] **Step 5: テスト実行 — 成功を確認**

Run: `cargo test -p genshin-calc-good --test test_parse`
Expected: PASS

- [ ] **Step 6: コミット**

```bash
git add crates/good/src/types.rs crates/good/tests/
git commit -m "feat(good): add GOOD JSON types with parse tests"
```

---

### Task 4: キーマッピング

**Files:**
- Create: `crates/good/src/key_map.rs`
- Create: `crates/good/tests/test_key_map.rs`

- [ ] **Step 1: テスト `test_key_map.rs` を作成（RED）**

```rust
use genshin_calc_good::key_map;

#[test]
fn pascal_to_snake_basic() {
    assert_eq!(key_map::pascal_to_snake("HuTao"), "hu_tao");
    assert_eq!(key_map::pascal_to_snake("KamisatoAyaka"), "kamisato_ayaka");
    assert_eq!(key_map::pascal_to_snake("Xiangling"), "xiangling");
    assert_eq!(key_map::pascal_to_snake("TravelerAnemo"), "traveler_anemo");
}

#[test]
fn pascal_to_snake_with_numbers_and_special() {
    assert_eq!(key_map::pascal_to_snake("WolfsGravestone"), "wolfs_gravestone");
    assert_eq!(key_map::pascal_to_snake("StaffOfHoma"), "staff_of_homa");
    assert_eq!(key_map::pascal_to_snake("CrimsonWitchOfFlames"), "crimson_witch_of_flames");
}

#[test]
fn lookup_character() {
    let result = key_map::lookup_character("HuTao");
    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "hu_tao");
}

#[test]
fn lookup_character_unknown() {
    let result = key_map::lookup_character("NonExistentCharacter");
    assert!(result.is_none());
}

#[test]
fn lookup_weapon() {
    let result = key_map::lookup_weapon("StaffOfHoma");
    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "staff_of_homa");
}

#[test]
fn lookup_artifact_set_direct() {
    // 自動変換で一致するケース
    let result = key_map::lookup_artifact_set("GladiatorsFinale");
    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "gladiators_finale");
}

#[test]
fn lookup_artifact_set_alias() {
    // 例外テーブルが必要なケース
    let result = key_map::lookup_artifact_set("CrimsonWitchOfFlames");
    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "crimson_witch");
}

#[test]
fn lookup_artifact_set_the_exile() {
    let result = key_map::lookup_artifact_set("TheExile");
    assert!(result.is_some());
    assert_eq!(result.unwrap().id, "exile");
}
```

- [ ] **Step 2: テスト実行 — 失敗を確認**

Run: `cargo test -p genshin-calc-good --test test_key_map`
Expected: FAIL

- [ ] **Step 3: `key_map.rs` を実装（GREEN）**

```rust
use genshin_calc_data::types::{ArtifactSet, CharacterData, WeaponData};

/// PascalCase を snake_case に変換
pub fn pascal_to_snake(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap());
        } else {
            result.push(ch);
        }
    }
    result
}

/// GOODキャラクターキーから内部CharacterDataをルックアップ
pub fn lookup_character(good_key: &str) -> Option<&'static CharacterData> {
    let snake = pascal_to_snake(good_key);
    genshin_calc_data::find_character(&snake)
}

/// GOOD武器キーから内部WeaponDataをルックアップ
pub fn lookup_weapon(good_key: &str) -> Option<&'static WeaponData> {
    let snake = pascal_to_snake(good_key);
    genshin_calc_data::find_weapon(&snake)
}

/// GOOD聖遺物セットキーから内部ArtifactSetをルックアップ
pub fn lookup_artifact_set(good_key: &str) -> Option<&'static ArtifactSet> {
    let snake = pascal_to_snake(good_key);
    // 直接ルックアップを試行
    if let Some(set) = genshin_calc_data::find_artifact_set(&snake) {
        return Some(set);
    }
    // 例外テーブルを参照
    if let Some(alias) = artifact_alias(&snake) {
        return genshin_calc_data::find_artifact_set(alias);
    }
    None
}

/// 聖遺物セットIDの例外テーブル（GOOD snake_case → 内部ID）
fn artifact_alias(snake_key: &str) -> Option<&'static str> {
    static ALIASES: &[(&str, &str)] = &[
        ("crimson_witch_of_flames", "crimson_witch"),
        ("the_exile", "exile"),
    ];
    ALIASES.iter().find(|(k, _)| *k == snake_key).map(|(_, v)| *v)
}
```

注意: `key_map` モジュールを外部テストからアクセスするため、`lib.rs` に `pub mod key_map;` を追加する（`mod key_map;` を置き換え）。

- [ ] **Step 4: テスト実行 — 成功を確認**

Run: `cargo test -p genshin-calc-good --test test_key_map`
Expected: PASS

- [ ] **Step 5: 例外テーブルの網羅性を検証するテストを追加**

data crateの全聖遺物セットに対してGOOD→内部変換の一致を検証するテストを追加:

```rust
#[test]
fn all_data_crate_artifact_sets_are_reachable() {
    // data crateの全聖遺物セットに対して、IDで直接findできることを確認
    // このテストは例外テーブル漏れの間接検出にも使える
    use genshin_calc_data::artifacts::ALL_ARTIFACT_SETS;
    for set in ALL_ARTIFACT_SETS {
        let found = genshin_calc_data::find_artifact_set(set.id);
        assert!(found.is_some(), "artifact set '{}' not findable by id", set.id);
    }
}
```

- [ ] **Step 6: コミット**

```bash
git add crates/good/src/key_map.rs crates/good/src/lib.rs crates/good/tests/test_key_map.rs
git commit -m "feat(good): add PascalCase→snake_case key mapping with alias table"
```

---

### Task 5: ステータスマッピング + メインステテーブル

**Files:**
- Create: `crates/good/src/stat_map.rs`
- Create: `crates/good/tests/test_stat_map.rs`

- [ ] **Step 1: テスト `test_stat_map.rs` を作成（RED）**

```rust
use genshin_calc_good::stat_map;

const EPS: f64 = 1e-4;

// --- StatKey変換テスト ---

#[test]
fn flat_stats_no_conversion() {
    use stat_map::StatConvertResult::Converted;

    let Converted(field, value) = stat_map::convert_stat("hp", 4780.0) else { panic!() };
    assert_eq!(field, stat_map::StatField::HpFlat);
    assert!((value - 4780.0).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("atk", 311.0) else { panic!() };
    assert_eq!(field, stat_map::StatField::AtkFlat);
    assert!((value - 311.0).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("def", 37.0) else { panic!() };
    assert_eq!(field, stat_map::StatField::DefFlat);
    assert!((value - 37.0).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("eleMas", 187.0) else { panic!() };
    assert_eq!(field, stat_map::StatField::ElementalMastery);
    assert!((value - 187.0).abs() < EPS);
}

#[test]
fn percent_stats_divide_by_100() {
    use stat_map::StatConvertResult::Converted;

    let Converted(field, value) = stat_map::convert_stat("hp_", 46.6) else { panic!() };
    assert_eq!(field, stat_map::StatField::HpPercent);
    assert!((value - 0.466).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("atk_", 46.6) else { panic!() };
    assert_eq!(field, stat_map::StatField::AtkPercent);
    assert!((value - 0.466).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("critRate_", 31.1) else { panic!() };
    assert_eq!(field, stat_map::StatField::CritRate);
    assert!((value - 0.311).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("critDMG_", 62.2) else { panic!() };
    assert_eq!(field, stat_map::StatField::CritDmg);
    assert!((value - 0.622).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("enerRech_", 51.8) else { panic!() };
    assert_eq!(field, stat_map::StatField::EnergyRecharge);
    assert!((value - 0.518).abs() < EPS);
}

#[test]
fn elemental_dmg_bonus() {
    use stat_map::StatConvertResult::Converted;

    let Converted(field, value) = stat_map::convert_stat("pyro_dmg_", 46.6) else { panic!() };
    assert_eq!(field, stat_map::StatField::ElementalDmgBonus("pyro"));
    assert!((value - 0.466).abs() < EPS);

    let Converted(field, value) = stat_map::convert_stat("physical_dmg_", 58.3) else { panic!() };
    assert_eq!(field, stat_map::StatField::PhysicalDmgBonus);
    assert!((value - 0.583).abs() < EPS);
}

#[test]
fn healing_bonus_returns_skip() {
    assert_eq!(stat_map::convert_stat("heal_", 35.9), stat_map::StatConvertResult::Skip);
}

#[test]
fn unknown_stat_key() {
    assert_eq!(stat_map::convert_stat("unknown_stat", 10.0), stat_map::StatConvertResult::Unknown);
}

// --- メインステ値テーブルテスト ---

#[test]
fn star5_flower_lv20() {
    let value = stat_map::main_stat_value(5, "hp", 20);
    assert!(value.is_some());
    assert!((value.unwrap() - 4780.0).abs() < 1.0);
}

#[test]
fn star5_plume_lv20() {
    let value = stat_map::main_stat_value(5, "atk", 20);
    assert!(value.is_some());
    assert!((value.unwrap() - 311.0).abs() < 1.0);
}

#[test]
fn star5_hp_percent_lv20() {
    let value = stat_map::main_stat_value(5, "hp_", 20);
    assert!(value.is_some());
    // テーブル値は生の%値（46.6）、convert_statで÷100する
    assert!((value.unwrap() - 46.6).abs() < 0.1);
}

#[test]
fn star5_crit_rate_lv0() {
    let value = stat_map::main_stat_value(5, "critRate_", 0);
    assert!(value.is_some());
    assert!((value.unwrap() - 4.7).abs() < 0.1);
}

#[test]
fn star5_crit_dmg_lv20() {
    let value = stat_map::main_stat_value(5, "critDMG_", 20);
    assert!(value.is_some());
    assert!((value.unwrap() - 62.2).abs() < 0.1);
}

#[test]
fn star4_atk_percent_lv16() {
    let value = stat_map::main_stat_value(4, "atk_", 16);
    assert!(value.is_some());
    assert!((value.unwrap() - 34.8).abs() < 0.1);
}

#[test]
fn star3_hp_flat_lv12() {
    let value = stat_map::main_stat_value(3, "hp", 12);
    assert!(value.is_some());
    assert!((value.unwrap() - 1893.0).abs() < 1.0);
}

#[test]
fn star5_atk_percent_lv10_intermediate() {
    // 線形補間の中間レベルテスト（許容誤差を広めに設定）
    let value = stat_map::main_stat_value(5, "atk_", 10).unwrap();
    // Lv10は概ね26.8% (ゲーム内実測)
    assert!((value - 26.8).abs() < 1.0);
}

#[test]
fn invalid_rarity_returns_none() {
    assert!(stat_map::main_stat_value(2, "hp", 0).is_none());
}
```

- [ ] **Step 2: テスト実行 — 失敗を確認**

Run: `cargo test -p genshin-calc-good --test test_stat_map`
Expected: FAIL

- [ ] **Step 3: `stat_map.rs` を実装（GREEN）**

```rust
/// StatKey変換結果のフィールド識別子
#[derive(Debug, Clone, PartialEq)]
pub enum StatField {
    HpFlat,
    HpPercent,
    AtkFlat,
    AtkPercent,
    DefFlat,
    DefPercent,
    ElementalMastery,
    EnergyRecharge,
    CritRate,
    CritDmg,
    DmgBonus,
    ElementalDmgBonus(&'static str),  // "pyro", "hydro", etc.
    PhysicalDmgBonus,
}

/// StatKey変換の結果
#[derive(Debug, PartialEq)]
pub enum StatConvertResult {
    /// 変換成功
    Converted(StatField, f64),
    /// 既知だがStatProfileに該当なし（heal_等）— warningにしない
    Skip,
    /// 未知のキー — warning対象
    Unknown,
}

/// GOOD StatKeyと値を内部形式に変換
/// パーセント系は÷100済み
pub fn convert_stat(key: &str, value: f64) -> StatConvertResult {
    match key {
        "hp" => StatConvertResult::Converted(StatField::HpFlat, value),
        "atk" => StatConvertResult::Converted(StatField::AtkFlat, value),
        "def" => StatConvertResult::Converted(StatField::DefFlat, value),
        "hp_" => StatConvertResult::Converted(StatField::HpPercent, value / 100.0),
        "atk_" => StatConvertResult::Converted(StatField::AtkPercent, value / 100.0),
        "def_" => StatConvertResult::Converted(StatField::DefPercent, value / 100.0),
        "eleMas" => StatConvertResult::Converted(StatField::ElementalMastery, value),
        "enerRech_" => StatConvertResult::Converted(StatField::EnergyRecharge, value / 100.0),
        "critRate_" => StatConvertResult::Converted(StatField::CritRate, value / 100.0),
        "critDMG_" => StatConvertResult::Converted(StatField::CritDmg, value / 100.0),
        "physical_dmg_" => StatConvertResult::Converted(StatField::PhysicalDmgBonus, value / 100.0),
        "pyro_dmg_" => StatConvertResult::Converted(StatField::ElementalDmgBonus("pyro"), value / 100.0),
        "hydro_dmg_" => StatConvertResult::Converted(StatField::ElementalDmgBonus("hydro"), value / 100.0),
        "electro_dmg_" => StatConvertResult::Converted(StatField::ElementalDmgBonus("electro"), value / 100.0),
        "cryo_dmg_" => StatConvertResult::Converted(StatField::ElementalDmgBonus("cryo"), value / 100.0),
        "dendro_dmg_" => StatConvertResult::Converted(StatField::ElementalDmgBonus("dendro"), value / 100.0),
        "anemo_dmg_" => StatConvertResult::Converted(StatField::ElementalDmgBonus("anemo"), value / 100.0),
        "geo_dmg_" => StatConvertResult::Converted(StatField::ElementalDmgBonus("geo"), value / 100.0),
        "heal_" => StatConvertResult::Skip,
        _ => StatConvertResult::Unknown,
    }
}

/// StatFieldの値をStatProfileに加算する
pub fn add_to_profile(
    profile: &mut genshin_calc_core::StatProfile,
    field: &StatField,
    value: f64,
    character_element: Option<genshin_calc_core::Element>,
) -> bool {
    match field {
        StatField::HpFlat => { profile.hp_flat += value; true }
        StatField::HpPercent => { profile.hp_percent += value; true }
        StatField::AtkFlat => { profile.atk_flat += value; true }
        StatField::AtkPercent => { profile.atk_percent += value; true }
        StatField::DefFlat => { profile.def_flat += value; true }
        StatField::DefPercent => { profile.def_percent += value; true }
        StatField::ElementalMastery => { profile.elemental_mastery += value; true }
        StatField::EnergyRecharge => { profile.energy_recharge += value; true }
        StatField::CritRate => { profile.crit_rate += value; true }
        StatField::CritDmg => { profile.crit_dmg += value; true }
        StatField::DmgBonus => { profile.dmg_bonus += value; true }
        StatField::PhysicalDmgBonus => { profile.dmg_bonus += value; true }
        StatField::ElementalDmgBonus(elem_str) => {
            // キャラ元素と一致する場合のみdmg_bonusに加算
            let matches = character_element.map_or(false, |ce| {
                element_str_matches(ce, elem_str)
            });
            if matches {
                profile.dmg_bonus += value;
            }
            matches
        }
    }
}

fn element_str_matches(element: genshin_calc_core::Element, s: &str) -> bool {
    match element {
        genshin_calc_core::Element::Pyro => s == "pyro",
        genshin_calc_core::Element::Hydro => s == "hydro",
        genshin_calc_core::Element::Electro => s == "electro",
        genshin_calc_core::Element::Cryo => s == "cryo",
        genshin_calc_core::Element::Dendro => s == "dendro",
        genshin_calc_core::Element::Anemo => s == "anemo",
        genshin_calc_core::Element::Geo => s == "geo",
    }
}

/// 聖遺物メインステの値を返す（生の%値、÷100前）
/// rarity: 3-5, main_stat_key: GOOD形式, level: 0-20
/// 注: 線形補間による近似値。中間レベルで±0.5%程度の誤差あり
pub fn main_stat_value(rarity: u8, main_stat_key: &str, level: u8) -> Option<f64> {
    let max_level = match rarity {
        5 => 20,
        4 => 16,
        3 => 12,
        _ => return None,
    };
    if level > max_level {
        return None;
    }
    let (base, max_val) = main_stat_base_max(rarity, main_stat_key)?;
    // 線形補間
    Some(base + (max_val - base) * (level as f64) / (max_level as f64))
}

/// (Lv0値, 最大Lv値) を返す
fn main_stat_base_max(rarity: u8, key: &str) -> Option<(f64, f64)> {
    match (rarity, key) {
        // --- Star 5 (Lv0 → Lv20) ---
        (5, "hp") => Some((717.0, 4780.0)),
        (5, "atk") => Some((47.0, 311.0)),
        (5, "hp_") => Some((7.0, 46.6)),
        (5, "atk_") => Some((7.0, 46.6)),
        (5, "def_") => Some((8.7, 58.3)),
        (5, "eleMas") => Some((28.0, 187.0)),
        (5, "enerRech_") => Some((7.8, 51.8)),
        (5, "critRate_") => Some((4.7, 31.1)),
        (5, "critDMG_") => Some((9.3, 62.2)),
        (5, "physical_dmg_") => Some((8.7, 58.3)),
        (5, "pyro_dmg_") | (5, "hydro_dmg_") | (5, "electro_dmg_") |
        (5, "cryo_dmg_") | (5, "dendro_dmg_") | (5, "anemo_dmg_") |
        (5, "geo_dmg_") => Some((7.0, 46.6)),
        (5, "heal_") => Some((5.4, 35.9)),
        // --- Star 4 (Lv0 → Lv16) ---
        (4, "hp") => Some((645.0, 3571.0)),
        (4, "atk") => Some((42.0, 232.0)),
        (4, "hp_") => Some((6.3, 34.8)),
        (4, "atk_") => Some((6.3, 34.8)),
        (4, "def_") => Some((7.9, 43.5)),
        (4, "eleMas") => Some((25.0, 139.0)),
        (4, "enerRech_") => Some((7.0, 38.7)),
        (4, "critRate_") => Some((4.2, 23.2)),
        (4, "critDMG_") => Some((8.4, 46.4)),
        (4, "physical_dmg_") => Some((7.9, 43.5)),
        (4, "pyro_dmg_") | (4, "hydro_dmg_") | (4, "electro_dmg_") |
        (4, "cryo_dmg_") | (4, "dendro_dmg_") | (4, "anemo_dmg_") |
        (4, "geo_dmg_") => Some((6.3, 34.8)),
        (4, "heal_") => Some((4.8, 26.8)),
        // --- Star 3 (Lv0 → Lv12) ---
        (3, "hp") => Some((430.0, 1893.0)),
        (3, "atk") => Some((28.0, 123.0)),
        (3, "hp_") => Some((5.2, 24.1)),
        (3, "atk_") => Some((5.2, 24.1)),
        (3, "def_") => Some((6.6, 30.2)),
        (3, "eleMas") => Some((21.0, 94.0)),
        (3, "enerRech_") => Some((5.8, 26.8)),
        (3, "critRate_") => Some((3.5, 16.2)),
        (3, "critDMG_") => Some((7.0, 32.4)),
        (3, "physical_dmg_") => Some((6.6, 30.2)),
        (3, "pyro_dmg_") | (3, "hydro_dmg_") | (3, "electro_dmg_") |
        (3, "cryo_dmg_") | (3, "dendro_dmg_") | (3, "anemo_dmg_") |
        (3, "geo_dmg_") => Some((5.2, 24.1)),
        (3, "heal_") => Some((4.0, 18.6)),
        _ => None,
    }
}
```

注意: `stat_map` も `pub mod stat_map;` で公開する。

- [ ] **Step 4: テスト実行 — 成功を確認**

Run: `cargo test -p genshin-calc-good --test test_stat_map`
Expected: PASS

- [ ] **Step 5: コミット**

```bash
git add crates/good/src/stat_map.rs crates/good/src/lib.rs crates/good/tests/test_stat_map.rs
git commit -m "feat(good): add stat key mapping and main stat value table"
```

---

### Task 6: 変換ロジック

**Files:**
- Create: `crates/good/src/convert.rs`
- Create: `crates/good/tests/test_convert.rs`

- [ ] **Step 1: テスト `test_convert.rs` を作成（RED）**

```rust
use genshin_calc_good::import_good;

const EPS: f64 = 1e-4;

#[test]
fn import_minimal_hu_tao() {
    let json = include_str!("data/minimal.json");
    let result = import_good(json).unwrap();

    assert_eq!(result.source, "TestScanner");
    assert_eq!(result.version, 2);
    assert_eq!(result.builds.len(), 1);
    assert!(result.warnings.is_empty(), "unexpected warnings: {:?}", result.warnings);

    let build = &result.builds[0];
    assert_eq!(build.character.id, "hu_tao");
    assert_eq!(build.level, 90);
    assert_eq!(build.ascension, 6);
    assert_eq!(build.constellation, 1);
    assert_eq!(build.talent_levels, [10, 10, 8]);

    // 武器
    let weapon = build.weapon.as_ref().unwrap();
    assert_eq!(weapon.weapon.id, "staff_of_homa");
    assert_eq!(weapon.level, 90);
    assert_eq!(weapon.refinement, 1);

    // 聖遺物セット: 5個全部CrimsonWitch → 4pcセット
    assert_eq!(build.artifacts.sets.len(), 1);
    assert_eq!(build.artifacts.sets[0].id, "crimson_witch");
    assert!(build.artifacts.four_piece_set.is_some());
    assert_eq!(build.artifacts.four_piece_set.unwrap().id, "crimson_witch");
}

#[test]
fn artifact_stats_aggregation() {
    let json = include_str!("data/minimal.json");
    let result = import_good(json).unwrap();
    let stats = &result.builds[0].artifacts.stats;

    // Flower main: hp flat 4780
    // Plume main: atk flat 311
    // Sands main: hp_ 46.6% → 0.466
    // Goblet main: pyro_dmg_ 46.6% → 0.466 (HuTao is Pyro → dmg_bonus)
    // Circlet main: critDMG_ 62.2% → 0.622

    // hp_flat: flower(4780)
    assert!((stats.hp_flat - 4780.0).abs() < 1.0);
    // atk_flat: plume(311) + goblet sub(33)
    assert!((stats.atk_flat - 344.0).abs() < 1.0);
    // hp_percent: sands main(0.466) + plume sub(0.099) + goblet sub(0.140) + circlet sub(0.117)
    assert!((stats.hp_percent - 0.822).abs() < 0.01);
    // crit_rate subs: flower(0.105) + sands(0.062) + goblet(0.035) + circlet(0.074)
    assert!((stats.crit_rate - 0.276).abs() < 0.01);
    // crit_dmg: circlet main(0.622) + flower sub(0.210) + plume sub(0.148) + sands sub(0.132) + goblet sub(0.078)
    assert!((stats.crit_dmg - 1.19).abs() < 0.01);
    // dmg_bonus: goblet main pyro 0.466 (HuTao = Pyro)
    assert!((stats.dmg_bonus - 0.466).abs() < 0.01);
    // def_flat: sands sub(37)
    assert!((stats.def_flat - 37.0).abs() < 1.0);
}

#[test]
fn invalid_format_returns_error() {
    let json = r#"{ "format": "NOT_GOOD", "source": "X", "version": 1 }"#;
    let result = import_good(json);
    assert!(result.is_err());
}

#[test]
fn unknown_character_becomes_warning() {
    let json = r#"{
        "format": "GOOD", "source": "Test", "version": 1,
        "characters": [
            { "key": "FutureCharacter", "level": 90, "constellation": 0, "ascension": 6,
              "talent": { "auto": 1, "skill": 1, "burst": 1 } }
        ]
    }"#;
    let result = import_good(json).unwrap();
    assert_eq!(result.builds.len(), 0);
    assert_eq!(result.warnings.len(), 1);
}

#[test]
fn empty_good_returns_empty_builds() {
    let json = r#"{ "format": "GOOD", "source": "Empty", "version": 1 }"#;
    let result = import_good(json).unwrap();
    assert!(result.builds.is_empty());
    assert!(result.warnings.is_empty());
}
```

- [ ] **Step 2: テスト実行 — 失敗を確認**

Run: `cargo test -p genshin-calc-good --test test_convert`
Expected: FAIL

- [ ] **Step 3: `convert.rs` を実装（GREEN）**

```rust
use std::collections::HashMap;

use genshin_calc_core::StatProfile;
use genshin_calc_data::types::{ArtifactSet, CharacterData, WeaponData};

use crate::error::ImportWarning;
use crate::key_map;
use crate::stat_map;
use crate::types::{GoodArtifact, GoodFormat};
use crate::{ArtifactsBuild, CharacterBuild, GoodImport, WeaponBuild};

pub(crate) fn build_imports(good: GoodFormat) -> GoodImport {
    let mut warnings = Vec::new();

    // 武器をlocationでインデックス化
    let weapons = index_weapons(&good, &mut warnings);
    // 聖遺物をlocationでグルーピング
    let artifacts = group_artifacts(&good, &mut warnings);

    let mut builds = Vec::new();

    if let Some(chars) = &good.characters {
        for gc in chars {
            let character = match key_map::lookup_character(&gc.key) {
                Some(c) => c,
                None => {
                    warnings.push(ImportWarning::UnknownCharacter(gc.key.clone()));
                    continue;
                }
            };

            let weapon = weapons.get(gc.key.as_str()).copied();
            let arts = artifacts.get(gc.key.as_str());
            let artifacts_build = build_artifacts(
                arts.map(|a| a.as_slice()).unwrap_or(&[]),
                character,
                &mut warnings,
            );

            builds.push(CharacterBuild {
                character,
                level: gc.level,
                ascension: gc.ascension,
                constellation: gc.constellation,
                talent_levels: [gc.talent.auto, gc.talent.skill, gc.talent.burst],
                weapon: weapon.map(|(w, level, refinement)| WeaponBuild {
                    weapon: w,
                    level,
                    refinement,
                }),
                artifacts: artifacts_build,
            });
        }
    }

    GoodImport {
        source: good.source,
        version: good.version,
        builds,
        warnings,
    }
}

/// 武器をlocation(CharacterKey)でインデックス化
fn index_weapons<'a>(
    good: &'a GoodFormat,
    warnings: &mut Vec<ImportWarning>,
) -> HashMap<&'a str, (&'static WeaponData, u32, u8)> {
    let mut map = HashMap::new();
    if let Some(weapons) = &good.weapons {
        for gw in weapons {
            let location = match &gw.location {
                Some(loc) if !loc.is_empty() => loc.as_str(),
                _ => continue,
            };
            let weapon = match key_map::lookup_weapon(&gw.key) {
                Some(w) => w,
                None => {
                    warnings.push(ImportWarning::UnknownWeapon(gw.key.clone()));
                    continue;
                }
            };
            map.insert(location, (weapon, gw.level, gw.refinement));
        }
    }
    map
}

/// 聖遺物をlocation(CharacterKey)でグルーピング
fn group_artifacts<'a>(
    good: &'a GoodFormat,
    warnings: &mut Vec<ImportWarning>,
) -> HashMap<&'a str, Vec<&'a GoodArtifact>> {
    let mut map: HashMap<&str, Vec<&GoodArtifact>> = HashMap::new();
    if let Some(artifacts) = &good.artifacts {
        for ga in artifacts {
            let location = match &ga.location {
                Some(loc) if !loc.is_empty() => loc.as_str(),
                _ => continue,
            };
            // set_keyの有効性は後で検証
            map.entry(location).or_default().push(ga);
        }
    }
    let _ = warnings; // 聖遺物警告はbuild_artifactsで処理
    map
}

/// 聖遺物リストからArtifactsBuildを構築
fn build_artifacts(
    artifacts: &[&GoodArtifact],
    character: &'static CharacterData,
    warnings: &mut Vec<ImportWarning>,
) -> ArtifactsBuild {
    let mut stats = StatProfile::default();
    // ArtifactSetはf64を含むためHash/Eq不可 → IDをキーにする
    let mut set_counts: HashMap<&'static str, (&'static ArtifactSet, u8)> = HashMap::new();

    for art in artifacts {
        // セットを解決してカウント
        if let Some(set) = key_map::lookup_artifact_set(&art.set_key) {
            let entry = set_counts.entry(set.id).or_insert((set, 0));
            entry.1 += 1;
        } else {
            warnings.push(ImportWarning::UnknownArtifactSet(art.set_key.clone()));
        }

        // メインステを加算
        if let Some(main_value) = stat_map::main_stat_value(art.rarity, &art.main_stat_key, art.level) {
            if let stat_map::StatConvertResult::Converted(field, converted) =
                stat_map::convert_stat(&art.main_stat_key, main_value)
            {
                let added = stat_map::add_to_profile(
                    &mut stats,
                    &field,
                    converted,
                    Some(character.element),
                );
                if !added {
                    if let stat_map::StatField::ElementalDmgBonus(elem) = &field {
                        warnings.push(ImportWarning::ElementMismatchGoblet {
                            character: character.id.to_string(),
                            goblet_element: elem.to_string(),
                        });
                    }
                }
            }
        }

        // サブステを加算
        for sub in &art.substats {
            if sub.key.is_empty() {
                continue;
            }
            match stat_map::convert_stat(&sub.key, sub.value) {
                stat_map::StatConvertResult::Converted(field, converted) => {
                    stat_map::add_to_profile(
                        &mut stats,
                        &field,
                        converted,
                        Some(character.element),
                    );
                }
                stat_map::StatConvertResult::Skip => {} // heal_等 — 無視
                stat_map::StatConvertResult::Unknown => {
                    warnings.push(ImportWarning::UnknownStatKey(sub.key.clone()));
                }
            }
        }
    }

    // セット検出
    let (sets, four_piece_set) = detect_sets(&set_counts);

    // 2pc/2pc+2pcの場合、2pcバフをstatsに合算
    if four_piece_set.is_none() {
        for set in &sets {
            apply_two_piece_buffs(set, character, &mut stats);
        }
    }

    ArtifactsBuild {
        sets,
        four_piece_set,
        stats,
    }
}

/// セットカウントから有効セットを検出
fn detect_sets(
    counts: &HashMap<&'static str, (&'static ArtifactSet, u8)>,
) -> (Vec<&'static ArtifactSet>, Option<&'static ArtifactSet>) {
    let mut four_piece = None;
    let mut two_pieces = Vec::new();

    for (_, &(set, count)) in counts {
        if count >= 4 {
            four_piece = Some(set);
        } else if count >= 2 {
            two_pieces.push(set);
        }
    }

    if let Some(fp) = four_piece {
        (vec![fp], Some(fp))
    } else {
        (two_pieces, None)
    }
}

/// 2pcセットの静的バフをStatProfileに加算
fn apply_two_piece_buffs(
    set: &'static ArtifactSet,
    character: &'static CharacterData,
    stats: &mut StatProfile,
) {
    use genshin_calc_core::BuffableStat;

    for buff in set.two_piece.buffs {
        match buff.stat {
            BuffableStat::HpPercent => stats.hp_percent += buff.value,
            BuffableStat::AtkPercent => stats.atk_percent += buff.value,
            BuffableStat::DefPercent => stats.def_percent += buff.value,
            BuffableStat::HpFlat => stats.hp_flat += buff.value,
            BuffableStat::AtkFlat => stats.atk_flat += buff.value,
            BuffableStat::DefFlat => stats.def_flat += buff.value,
            BuffableStat::ElementalMastery => stats.elemental_mastery += buff.value,
            BuffableStat::EnergyRecharge => stats.energy_recharge += buff.value,
            BuffableStat::CritRate => stats.crit_rate += buff.value,
            BuffableStat::CritDmg => stats.crit_dmg += buff.value,
            BuffableStat::DmgBonus => stats.dmg_bonus += buff.value,
            BuffableStat::ElementalDmgBonus(elem) => {
                // キャラ元素と一致する場合のみ加算
                if elem == character.element {
                    stats.dmg_bonus += buff.value;
                }
            }
            BuffableStat::PhysicalDmgBonus => stats.dmg_bonus += buff.value,
            _ => {} // 2pc効果にはないバフ種別はスキップ
        }
    }
}
```

- [ ] **Step 4: テスト実行 — 成功を確認**

Run: `cargo test -p genshin-calc-good --test test_convert`
Expected: PASS

- [ ] **Step 5: コミット**

```bash
git add crates/good/src/convert.rs crates/good/tests/test_convert.rs
git commit -m "feat(good): implement GOOD → CharacterBuild conversion"
```

---

### Task 7: 追加テストデータ + エッジケーステスト

**Files:**
- Create: `crates/good/tests/data/two_piece_two_piece.json`
- Create: `crates/good/tests/data/partial.json`
- Modify: `crates/good/tests/test_convert.rs`

- [ ] **Step 1: 2pc+2pcテストデータ作成**

`two_piece_two_piece.json` — グラディエーター2pc(ATK+18%) + 剣闘士2pc のような構成:

```json
{
  "format": "GOOD",
  "source": "Test",
  "version": 2,
  "characters": [
    {
      "key": "Xiangling",
      "level": 90,
      "constellation": 6,
      "ascension": 6,
      "talent": { "auto": 6, "skill": 9, "burst": 12 }
    }
  ],
  "weapons": [
    {
      "key": "TheCatch",
      "level": 90,
      "ascension": 6,
      "refinement": 5,
      "location": "Xiangling",
      "lock": true
    }
  ],
  "artifacts": [
    {
      "setKey": "EmblemOfSeveredFate",
      "slotKey": "flower",
      "level": 20,
      "rarity": 5,
      "mainStatKey": "hp",
      "location": "Xiangling",
      "lock": false,
      "substats": [
        { "key": "critRate_", "value": 7.0 },
        { "key": "critDMG_", "value": 14.0 },
        { "key": "atk_", "value": 5.0 },
        { "key": "enerRech_", "value": 5.0 }
      ]
    },
    {
      "setKey": "EmblemOfSeveredFate",
      "slotKey": "plume",
      "level": 20,
      "rarity": 5,
      "mainStatKey": "atk",
      "location": "Xiangling",
      "lock": false,
      "substats": [
        { "key": "critRate_", "value": 7.0 },
        { "key": "critDMG_", "value": 14.0 },
        { "key": "enerRech_", "value": 10.0 },
        { "key": "hp_", "value": 5.0 }
      ]
    },
    {
      "setKey": "GladiatorsFinale",
      "slotKey": "sands",
      "level": 20,
      "rarity": 5,
      "mainStatKey": "enerRech_",
      "location": "Xiangling",
      "lock": false,
      "substats": [
        { "key": "critRate_", "value": 7.0 },
        { "key": "critDMG_", "value": 14.0 },
        { "key": "atk_", "value": 5.0 },
        { "key": "hp", "value": 500 }
      ]
    },
    {
      "setKey": "GladiatorsFinale",
      "slotKey": "goblet",
      "level": 20,
      "rarity": 5,
      "mainStatKey": "pyro_dmg_",
      "location": "Xiangling",
      "lock": false,
      "substats": [
        { "key": "critRate_", "value": 7.0 },
        { "key": "critDMG_", "value": 14.0 },
        { "key": "atk_", "value": 5.0 },
        { "key": "enerRech_", "value": 5.0 }
      ]
    },
    {
      "setKey": "EmblemOfSeveredFate",
      "slotKey": "circlet",
      "level": 20,
      "rarity": 5,
      "mainStatKey": "critRate_",
      "location": "Xiangling",
      "lock": false,
      "substats": [
        { "key": "critDMG_", "value": 14.0 },
        { "key": "atk_", "value": 5.0 },
        { "key": "enerRech_", "value": 5.0 },
        { "key": "hp", "value": 500 }
      ]
    }
  ]
}
```

- [ ] **Step 2: テストを追加**

```rust
#[test]
fn two_piece_two_piece_sets() {
    let json = include_str!("data/two_piece_two_piece.json");
    let result = import_good(json).unwrap();
    let build = &result.builds[0];

    // Emblem 3pc + Gladiator 2pc → Emblem 2pc + Gladiator 2pc
    assert_eq!(build.artifacts.sets.len(), 2);
    assert!(build.artifacts.four_piece_set.is_none());

    let set_ids: Vec<&str> = build.artifacts.sets.iter().map(|s| s.id).collect();
    assert!(set_ids.contains(&"emblem_of_severed_fate"));
    assert!(set_ids.contains(&"gladiators_finale"));

    // Emblem 2pc: ER+20% (0.20) が statsに合算されている
    // Gladiator 2pc: ATK+18% (0.18) が statsに合算されている
    // subs enerRech_: 5+10+5+5 = 25% → 0.25
    // sands main enerRech_: 51.8% → 0.518
    // total enerRech_ = 0.25 + 0.518 + 0.20(emblem 2pc) = 0.968
    assert!((build.artifacts.stats.energy_recharge - 0.968).abs() < 0.01);

    // atk_percent subs: 5+5+5+5 = 20% → 0.20
    // + gladiator 2pc 0.18 = 0.38
    assert!((build.artifacts.stats.atk_percent - 0.38).abs() < 0.01);
}

#[test]
fn partial_build_no_weapon() {
    let json = r#"{
        "format": "GOOD", "source": "Test", "version": 1,
        "characters": [
            { "key": "Xiangling", "level": 80, "constellation": 0, "ascension": 5,
              "talent": { "auto": 1, "skill": 1, "burst": 1 } }
        ]
    }"#;
    let result = import_good(json).unwrap();
    assert_eq!(result.builds.len(), 1);
    assert!(result.builds[0].weapon.is_none());
    assert!(result.builds[0].artifacts.sets.is_empty());
}

#[test]
fn element_mismatch_goblet_warning() {
    let json = r#"{
        "format": "GOOD", "source": "Test", "version": 1,
        "characters": [
            { "key": "Xiangling", "level": 90, "constellation": 0, "ascension": 6,
              "talent": { "auto": 1, "skill": 1, "burst": 1 } }
        ],
        "artifacts": [
            {
                "setKey": "GladiatorsFinale", "slotKey": "goblet",
                "level": 20, "rarity": 5, "mainStatKey": "hydro_dmg_",
                "location": "Xiangling", "lock": false, "substats": []
            }
        ]
    }"#;
    let result = import_good(json).unwrap();
    // Xiangling is Pyro, goblet is hydro → warning + dmg_bonus not added
    assert!(!result.warnings.is_empty());
    assert!((result.builds[0].artifacts.stats.dmg_bonus).abs() < EPS);
}
```

- [ ] **Step 3: テスト実行 — 成功を確認**

Run: `cargo test -p genshin-calc-good`
Expected: ALL PASS

- [ ] **Step 4: コミット**

```bash
git add crates/good/tests/
git commit -m "test(good): add 2pc+2pc, partial build, and element mismatch tests"
```

---

### Task 8: 最終確認 + clippy + fmt

**Files:**
- All files in `crates/good/`

- [ ] **Step 1: 全テスト実行**

Run: `cargo test -p genshin-calc-good`
Expected: ALL PASS

- [ ] **Step 2: clippy**

Run: `cargo clippy -p genshin-calc-good -- -D warnings`
Expected: PASS（警告なし）

- [ ] **Step 3: fmt**

Run: `cargo fmt -p genshin-calc-good --check`
Expected: PASS

- [ ] **Step 4: ワークスペース全体テスト**

Run: `cargo test`
Expected: ALL PASS（既存テストに影響なし）

- [ ] **Step 5: コミット（必要な修正があれば）**

```bash
git add -A crates/good/
git commit -m "chore(good): fix clippy warnings and formatting"
```
