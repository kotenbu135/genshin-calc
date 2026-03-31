# Artifact Set Effect API Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** WASM `build_stats` APIで聖遺物セット効果（条件付きバフ含む）を解決し、UI側のハードコードを不要にする。

**Architecture:** 既存の `TeamMemberBuilder` が聖遺物セット効果のバフ収集・解決を実装済み。`manual_activations` の型を `&'static str` → `String` に変更し、`CharacterBuild` → `TeamMemberBuilder` 変換関数を新設。`CharacterBuild` は `&'static` 参照を含み `Deserialize` 不可のため、WASM APIは既存の `build_stats_from_good` と同様に `(json, character_id)` + activations を受け取る方式にする。

**Tech Stack:** Rust, wasm-bindgen, serde, genshin-calc-core/data/good

**Design Spec:** `docs/superpowers/specs/2026-04-01-artifact-set-api-design.md`

---

## File Map

| File | Action | Responsibility |
|------|--------|---------------|
| `crates/data/src/team_builder.rs` | Modify | `manual_activations` 型を `String` に変更、`activate`/`activate_with_stacks`/`eval_manual` シグネチャ変更 |
| `crates/good/src/error.rs` | Modify | `GoodError::MissingWeapon` variant追加 |
| `crates/good/src/convert.rs` | Modify | `to_team_member_builder()` 変換関数追加 |
| `crates/good/src/lib.rs` | Modify | `to_team_member_builder` を re-export |
| `crates/wasm/src/lib.rs` | Modify | `build_stats` 関数追加（`WasmManualActivation` 型・`convert_activations` 関数含む） |

---

### Task 1: TeamMemberBuilder の manual_activations 型変更

**Files:**
- Modify: `crates/data/src/team_builder.rs:22` (`manual_activations` field)
- Modify: `crates/data/src/team_builder.rs:78-88` (`activate` / `activate_with_stacks`)
- Modify: `crates/data/src/team_builder.rs:577-580` (`eval_manual` signature)

- [ ] **Step 1: `manual_activations` フィールド型を変更**

`crates/data/src/team_builder.rs:22`:
```rust
// Before:
manual_activations: Vec<(&'static str, ManualActivation)>,
// After:
manual_activations: Vec<(String, ManualActivation)>,
```

- [ ] **Step 2: `activate` メソッドのシグネチャを変更**

`crates/data/src/team_builder.rs:78`:
```rust
// Before:
pub fn activate(mut self, name: &'static str) -> Self {
    self.manual_activations
        .push((name, ManualActivation::Active));
    self
}
// After:
pub fn activate(mut self, name: &str) -> Self {
    self.manual_activations
        .push((name.to_string(), ManualActivation::Active));
    self
}
```

- [ ] **Step 3: `activate_with_stacks` メソッドのシグネチャを変更**

`crates/data/src/team_builder.rs:85`:
```rust
// Before:
pub fn activate_with_stacks(mut self, name: &'static str, stacks: u8) -> Self {
    self.manual_activations
        .push((name, ManualActivation::Stacks(stacks)));
    self
}
// After:
pub fn activate_with_stacks(mut self, name: &str, stacks: u8) -> Self {
    self.manual_activations
        .push((name.to_string(), ManualActivation::Stacks(stacks)));
    self
}
```

- [ ] **Step 4: `eval_manual` のシグネチャを変更**

`crates/data/src/team_builder.rs:577`:
```rust
// Before:
fn eval_manual(
    cond: &ManualCondition,
    buff: &ConditionalBuff,
    activations: &[(&str, ManualActivation)],
    base_value: f64,
) -> Option<f64> {
// After:
fn eval_manual(
    cond: &ManualCondition,
    buff: &ConditionalBuff,
    activations: &[(String, ManualActivation)],
    base_value: f64,
) -> Option<f64> {
```

関数本体の変更は不要。`*name == buff.name` は `String == &str` の `PartialEq` で動作する。

- [ ] **Step 5: 既存テストが全てパスすることを確認**

Run: `cargo test -p genshin-calc-data`

Expected: 全テストPASS。既存呼び出しは `activate("literal")` で `&'static str` → `&str` に自動coerceされるため変更不要。

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/team_builder.rs
git commit -m "refactor: change manual_activations from &'static str to String

Enables dynamic activation names from WASM input (String) in addition
to static literal names used in existing tests."
```

---

### Task 2: GoodError::MissingWeapon variant 追加

**Files:**
- Modify: `crates/good/src/error.rs:4-13`

- [ ] **Step 1: `MissingWeapon` variant を追加**

`crates/good/src/error.rs` の `GoodError` enumに追加:
```rust
#[derive(Debug, thiserror::Error)]
pub enum GoodError {
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("invalid GOOD format: expected \"GOOD\", got \"{0}\"")]
    InvalidFormat(String),

    #[error("unsupported GOOD version: {0}")]
    UnsupportedVersion(u8),

    #[error("weapon is required for stat calculation")]
    MissingWeapon,
}
```

- [ ] **Step 2: ビルド確認**

Run: `cargo build -p genshin-calc-good`

Expected: BUILD SUCCESS

- [ ] **Step 3: Commit**

```bash
git add crates/good/src/error.rs
git commit -m "feat(good): add GoodError::MissingWeapon variant"
```

---

### Task 3: to_team_member_builder 変換関数 — テスト

**Files:**
- Modify: `crates/good/src/convert.rs` (テストを末尾に追加)

**注意:** `TeamMember` のバフフィールドは `buffs_provided`（`buffs` ではない）。
source文字列フォーマット: 無条件バフは `"{set.name} 2pc"` / `"{set.name} 4pc"`、条件付きバフは `"{cond_buff.name} ({set.name} 4pc)"`。

- [ ] **Step 1: テストを書く**

`crates/good/src/convert.rs` の末尾に追加:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use genshin_calc_data::buff::ManualActivation;

    fn make_build(
        character: &'static CharacterData,
        weapon: Option<&'static WeaponData>,
        artifact_set: Option<&'static ArtifactSet>,
    ) -> crate::CharacterBuild {
        crate::CharacterBuild {
            character,
            level: 90,
            ascension: 6,
            constellation: 0,
            talent_levels: [10, 8, 9],
            weapon: weapon.map(|w| crate::WeaponBuild {
                weapon: w,
                level: 90,
                refinement: 1,
            }),
            artifacts: crate::ArtifactsBuild {
                sets: vec![],
                four_piece_set: artifact_set,
                stats: StatProfile::default(),
            },
        }
    }

    #[test]
    fn test_basic_conversion() {
        let char = genshin_calc_data::find_character("diluc").unwrap();
        let weapon = genshin_calc_data::find_weapon("wolfs_gravestone").unwrap();
        let build = make_build(char, Some(weapon), None);
        let result = to_team_member_builder(&build, &[], &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_missing_weapon_error() {
        let char = genshin_calc_data::find_character("diluc").unwrap();
        let build = make_build(char, None, None);
        let result = to_team_member_builder(&build, &[], &[]);
        assert!(result.is_err());
        assert!(
            result.unwrap_err().to_string().contains("weapon"),
            "error should mention weapon"
        );
    }

    #[test]
    fn test_with_artifact_set_2pc_buff() {
        let char = genshin_calc_data::find_character("diluc").unwrap();
        let weapon = genshin_calc_data::find_weapon("wolfs_gravestone").unwrap();
        let cw = genshin_calc_data::find_artifact_set("crimson_witch").unwrap();
        let build = make_build(char, Some(weapon), Some(cw));
        let result = to_team_member_builder(&build, &[], &[]);
        assert!(result.is_ok());
        let member = result.unwrap().build().unwrap();
        // CW 2pc: ElementalDmgBonus(Pyro) +15% — source: "燃え盛る炎の魔女 2pc"
        let has_2pc = member
            .buffs_provided
            .iter()
            .any(|b| b.source.contains("2pc"));
        assert!(has_2pc, "should have 2pc static buff");
    }

    #[test]
    fn test_with_artifact_activation_stacks() {
        let char = genshin_calc_data::find_character("diluc").unwrap();
        let weapon = genshin_calc_data::find_weapon("wolfs_gravestone").unwrap();
        let cw = genshin_calc_data::find_artifact_set("crimson_witch").unwrap();
        let build = make_build(char, Some(weapon), Some(cw));

        // CW 4pc: cwof_pyro_stacks = ElementalDmgBonus(Pyro) +0.075/stack, max 3
        let activations = [("cwof_pyro_stacks", ManualActivation::Stacks(2))];
        let result = to_team_member_builder(&build, &[], &activations);
        assert!(result.is_ok());
        let member = result.unwrap().build().unwrap();
        // source format: "cwof_pyro_stacks (燃え盛る炎の魔女 4pc)"
        let stack_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("cwof_pyro_stacks"));
        assert!(stack_buff.is_some(), "should have CW 4pc stack buff");
        let buff = stack_buff.unwrap();
        // 2 stacks × 0.075 = 0.15
        assert!(
            (buff.value - 0.15).abs() < 1e-10,
            "2 stacks should give 0.15, got {}",
            buff.value
        );
    }
}
```

- [ ] **Step 2: テストを実行して失敗を確認**

Run: `cargo test -p genshin-calc-good -- convert::tests`

Expected: FAIL — `to_team_member_builder` が未定義

---

### Task 4: to_team_member_builder 変換関数 — 実装

**Files:**
- Modify: `crates/good/src/convert.rs` (関数追加)
- Modify: `crates/good/src/lib.rs` (re-export)

- [ ] **Step 1: import文を追加し変換関数を実装**

`crates/good/src/convert.rs` の既存import文の後、`build_imports` 関数の前に追加:
```rust
use genshin_calc_data::buff::ManualActivation;
use genshin_calc_data::team_builder::TeamMemberBuilder;

/// CharacterBuild と ManualActivation リストから TeamMemberBuilder を構築。
/// weapon が None の場合は Err を返す（ステータス計算には武器が必須）。
pub fn to_team_member_builder(
    build: &crate::CharacterBuild,
    weapon_activations: &[(&str, ManualActivation)],
    artifact_activations: &[(&str, ManualActivation)],
) -> Result<TeamMemberBuilder, crate::GoodError> {
    let weapon_build = build
        .weapon
        .as_ref()
        .ok_or(crate::GoodError::MissingWeapon)?;

    let mut builder = TeamMemberBuilder::new(build.character, weapon_build.weapon)
        .constellation(build.constellation)
        .talent_levels(build.talent_levels)
        .refinement(weapon_build.refinement);

    // 聖遺物ステータス（2pc無条件バフはimport時にstatsにマージ済み）
    builder = builder.artifact_stats(build.artifacts.stats.clone());

    // 4セット効果
    if let Some(set) = build.artifacts.four_piece_set {
        builder = builder.artifact_set(set);
    }

    // ManualActivation を登録
    for (name, activation) in weapon_activations.iter().chain(artifact_activations.iter()) {
        builder = match activation {
            ManualActivation::Active => builder.activate(name),
            ManualActivation::Stacks(n) => builder.activate_with_stacks(name, *n),
        };
    }

    Ok(builder)
}
```

- [ ] **Step 2: lib.rs に re-export を追加**

`crates/good/src/lib.rs` — `pub use build_stats::build_stat_profile;` の行の後に追加:
```rust
pub use convert::to_team_member_builder;
```

- [ ] **Step 3: テストを実行してパスを確認**

Run: `cargo test -p genshin-calc-good -- convert::tests`

Expected: 全4テストPASS

- [ ] **Step 4: Commit**

```bash
git add crates/good/src/convert.rs crates/good/src/lib.rs
git commit -m "feat(good): add to_team_member_builder conversion function

Bridges CharacterBuild (GOOD import result) to TeamMemberBuilder,
enabling weapon and artifact conditional buff activation via WASM."
```

---

### Task 5: WASM build_stats 関数 — テスト

**Files:**
- Modify: `crates/wasm/src/lib.rs` (テスト追加)

**注意:** `CharacterBuild` は `&'static` 参照を含み `Deserialize` 不可。WASMテストは `serde_wasm_bindgen` ではなく、既存テストパターンに合わせてネイティブRust関数を直接呼び出す形にする。WASM関数 `build_stats` は `(json, character_id, activations)` を受け取り内部で `import_good` → `to_team_member_builder` する。

- [ ] **Step 1: テストを書く**

`crates/wasm/src/lib.rs` の `mod tests` ブロック内に追加:
```rust
    #[test]
    fn test_build_stats_basic() {
        // to_team_member_builder を直接テスト（WASM境界なし）
        let char = genshin_calc_data::find_character("hu_tao").unwrap();
        let weapon = genshin_calc_data::find_weapon("staff_of_homa").unwrap();
        let build = genshin_calc_good::CharacterBuild {
            character: char,
            level: 90,
            ascension: 6,
            constellation: 0,
            talent_levels: [10, 10, 10],
            weapon: Some(genshin_calc_good::WeaponBuild {
                weapon,
                level: 90,
                refinement: 1,
            }),
            artifacts: genshin_calc_good::ArtifactsBuild {
                sets: vec![],
                four_piece_set: None,
                stats: genshin_calc_core::StatProfile::default(),
            },
        };
        let builder = genshin_calc_good::to_team_member_builder(&build, &[], &[]).unwrap();
        let member = builder.build().unwrap();
        let stats = genshin_calc_core::combine_stats(&member.stats).unwrap();
        assert!(stats.atk > 0.0, "ATK should be positive");
        assert!(stats.hp > 0.0, "HP should be positive");
    }

    #[test]
    fn test_build_stats_with_artifact_activation() {
        use genshin_calc_data::buff::ManualActivation;

        let char = genshin_calc_data::find_character("diluc").unwrap();
        let weapon = genshin_calc_data::find_weapon("wolfs_gravestone").unwrap();
        let cw = genshin_calc_data::find_artifact_set("crimson_witch").unwrap();
        let build = genshin_calc_good::CharacterBuild {
            character: char,
            level: 90,
            ascension: 6,
            constellation: 0,
            talent_levels: [10, 10, 10],
            weapon: Some(genshin_calc_good::WeaponBuild {
                weapon,
                level: 90,
                refinement: 1,
            }),
            artifacts: genshin_calc_good::ArtifactsBuild {
                sets: vec![cw],
                four_piece_set: Some(cw),
                stats: genshin_calc_core::StatProfile::default(),
            },
        };

        // Without activation
        let builder_no = genshin_calc_good::to_team_member_builder(&build, &[], &[]).unwrap();
        let member_no = builder_no.build().unwrap();
        let stats_no = genshin_calc_core::combine_stats(&member_no.stats).unwrap();

        // With 2 stacks of CW pyro buff
        let artifact_acts = [("cwof_pyro_stacks", ManualActivation::Stacks(2))];
        let builder_with =
            genshin_calc_good::to_team_member_builder(&build, &[], &artifact_acts).unwrap();
        let member_with = builder_with.build().unwrap();
        let stats_with = genshin_calc_core::combine_stats(&member_with.stats).unwrap();

        // With stacks, pyro_dmg_bonus should be higher by 0.15 (2 × 0.075)
        let diff = stats_with.pyro_dmg_bonus - stats_no.pyro_dmg_bonus;
        assert!(
            (diff - 0.15).abs() < 1e-10,
            "CW 4pc 2 stacks should add 0.15 pyro_dmg_bonus, got diff={}",
            diff
        );
    }
```

- [ ] **Step 2: テストを実行して失敗を確認**

Run: `cargo test -p genshin-calc-wasm -- test_build_stats`

Expected: FAIL — `build_stats` 関数やWASM型が未定義（テスト自体は `to_team_member_builder` を直接呼ぶのでコンパイルは通る可能性あり。その場合はPASSも許容）

---

### Task 6: WASM build_stats 関数 — 実装

**Files:**
- Modify: `crates/wasm/src/lib.rs`

**注意:** `CharacterBuild` は `Deserialize` 不可（`&'static` 参照含む）。既存の `build_stats_from_good` と同様に `json: &str` + `character_id: &str` を受け取り、内部で `import_good` → `to_team_member_builder` する。

- [ ] **Step 1: 型定義と変換関数を追加**

`crates/wasm/src/lib.rs` の `use` 文の後（`fn to_js` の前）に追加:
```rust
use serde::Deserialize;

#[derive(Clone, Deserialize)]
struct WasmManualActivation {
    name: String,
    active: bool,
    stacks: Option<u8>,
}

fn convert_activations(
    input: &[WasmManualActivation],
) -> Vec<(&str, genshin_calc_data::buff::ManualActivation)> {
    input
        .iter()
        .filter(|a| a.active)
        .map(|a| {
            let activation = match a.stacks {
                Some(n) => genshin_calc_data::buff::ManualActivation::Stacks(n),
                None => genshin_calc_data::buff::ManualActivation::Active,
            };
            (a.name.as_str(), activation)
        })
        .collect()
}
```

- [ ] **Step 2: `build_stats` WASM関数を追加**

`build_stats_from_good` 関数の後（L189付近）に追加:
```rust
/// Calculates final stats for a character build with conditional buff activations.
///
/// Unlike `build_stats_from_good`, this function resolves weapon and artifact
/// conditional buffs (toggles, stacks) via TeamMemberBuilder.
///
/// # Arguments
/// * `json` - GOOD format JSON string
/// * `character_id` - Character ID (e.g. "diluc")
/// * `weapon_activations` - JS array of WasmManualActivation for weapon buffs
/// * `artifact_activations` - JS array of WasmManualActivation for artifact set buffs
///
/// # Returns
/// Stats as a JS object, or null if character not found.
#[wasm_bindgen]
pub fn build_stats(
    json: &str,
    character_id: &str,
    weapon_activations: JsValue,
    artifact_activations: JsValue,
) -> Result<JsValue, JsError> {
    let import = genshin_calc_good::import_good(json)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let build = import
        .builds
        .iter()
        .find(|b| b.character.id == character_id);
    match build {
        Some(b) => {
            let w_acts: Vec<WasmManualActivation> = serde_wasm_bindgen::from_value(weapon_activations)
                .unwrap_or_default();
            let a_acts: Vec<WasmManualActivation> = serde_wasm_bindgen::from_value(artifact_activations)
                .unwrap_or_default();
            let w_converted = convert_activations(&w_acts);
            let a_converted = convert_activations(&a_acts);
            let builder = genshin_calc_good::to_team_member_builder(b, &w_converted, &a_converted)
                .map_err(|e| JsError::new(&e.to_string()))?;
            let member = builder
                .build()
                .map_err(|e| JsError::new(&e.to_string()))?;
            let stats = genshin_calc_core::combine_stats(&member.stats)
                .map_err(|e| JsError::new(&e.to_string()))?;
            to_js(&stats)
        }
        None => Ok(JsValue::NULL),
    }
}
```

- [ ] **Step 3: テストを実行してパスを確認**

Run: `cargo test -p genshin-calc-wasm -- test_build_stats`

Expected: 全テストPASS

- [ ] **Step 4: 全テスト確認**

Run: `cargo test`

Expected: 全crateのテストがPASS

- [ ] **Step 5: Clippy確認**

Run: `cargo clippy -- -D warnings`

Expected: 警告なし

- [ ] **Step 6: Commit**

```bash
git add crates/wasm/src/lib.rs
git commit -m "feat(wasm): add build_stats function with conditional buff activation

Accepts GOOD JSON + character_id + activation arrays.
Uses TeamMemberBuilder internally to resolve all buffs including
artifact 4-piece set effects (toggles, stacks)."
```

---

## 依存関係

- Task 1 → Task 3, 4（`activate(&str)` シグネチャが必要）
- Task 2 → Task 4（`GoodError::MissingWeapon` が必要）
- Task 3 → Task 4（テストファースト）
- Task 5 → Task 6（テストファースト）
- Task 4 → Task 5, 6（`to_team_member_builder` が必要）

実行順: Task 1 → Task 2 → Task 3 → Task 4 → Task 5 → Task 6
