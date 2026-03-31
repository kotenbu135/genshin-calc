# build_stats Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `Stats` / `StatProfile` を元素別DMGボーナスに対応させ、`build_stats()` WASM関数を追加する

**Architecture:** `StatProfile` と `Stats` に8つの元素別DMGフィールドを追加。`total_dmg_bonus(element)` ヘルパーで計算時に適切な元素ボーナスを選択。`build_stat_profile()` を good crate に追加し、WASM の `build_stats()` から呼び出す。

**Tech Stack:** Rust, wasm-bindgen, serde

**Spec:** `docs/superpowers/specs/2026-03-31-build-stats-design.md`

---

### Task 1: `StatProfile` に元素別DMGフィールドを追加

**Files:**
- Modify: `crates/core/src/stat_profile.rs:13-42` (struct定義)
- Modify: `crates/core/src/stat_profile.rs:70-83` (`combine_stats`)
- Modify: `crates/core/src/stat_profile.rs:90-138` (`validate`)

- [ ] **Step 1: `StatProfile` struct に8フィールド追加**

`crates/core/src/stat_profile.rs:41` の `pub dmg_bonus: f64,` の後に追加:

```rust
    /// Pyro DMG bonus in decimal form.
    pub pyro_dmg_bonus: f64,
    /// Hydro DMG bonus in decimal form.
    pub hydro_dmg_bonus: f64,
    /// Electro DMG bonus in decimal form.
    pub electro_dmg_bonus: f64,
    /// Cryo DMG bonus in decimal form.
    pub cryo_dmg_bonus: f64,
    /// Dendro DMG bonus in decimal form.
    pub dendro_dmg_bonus: f64,
    /// Anemo DMG bonus in decimal form.
    pub anemo_dmg_bonus: f64,
    /// Geo DMG bonus in decimal form.
    pub geo_dmg_bonus: f64,
    /// Physical DMG bonus in decimal form.
    pub physical_dmg_bonus: f64,
```

- [ ] **Step 2: `validate()` に8フィールドの検証を追加**

`crates/core/src/stat_profile.rs:134` の `dmg_bonus` チェックの後に追加:

```rust
    if profile.pyro_dmg_bonus < -1.0 {
        return Err(CalcError::InvalidDmgBonus(profile.pyro_dmg_bonus));
    }
    if profile.hydro_dmg_bonus < -1.0 {
        return Err(CalcError::InvalidDmgBonus(profile.hydro_dmg_bonus));
    }
    if profile.electro_dmg_bonus < -1.0 {
        return Err(CalcError::InvalidDmgBonus(profile.electro_dmg_bonus));
    }
    if profile.cryo_dmg_bonus < -1.0 {
        return Err(CalcError::InvalidDmgBonus(profile.cryo_dmg_bonus));
    }
    if profile.dendro_dmg_bonus < -1.0 {
        return Err(CalcError::InvalidDmgBonus(profile.dendro_dmg_bonus));
    }
    if profile.anemo_dmg_bonus < -1.0 {
        return Err(CalcError::InvalidDmgBonus(profile.anemo_dmg_bonus));
    }
    if profile.geo_dmg_bonus < -1.0 {
        return Err(CalcError::InvalidDmgBonus(profile.geo_dmg_bonus));
    }
    if profile.physical_dmg_bonus < -1.0 {
        return Err(CalcError::InvalidDmgBonus(profile.physical_dmg_bonus));
    }
```

- [ ] **Step 3: `stat_profile.rs` 内のテストで `StatProfile` を直接構築している箇所に `..Default::default()` を追加して修正**

`crates/core/src/stat_profile.rs` 内の約17箇所の `StatProfile {` 構築で、新フィールドが不足するもの全てに `..Default::default()` を追加（既にある場合はそのまま）。

- [ ] **Step 4: ビルド確認**

Run: `cargo build -p genshin-calc-core 2>&1 | head -50`
Expected: コンパイルエラーが出る場合は `Stats` 構築箇所（次タスクで対応）

- [ ] **Step 5: コミット**

```bash
git add crates/core/src/stat_profile.rs
git commit -m "feat: add per-element DMG bonus fields to StatProfile"
```

---

### Task 2: `Stats` に元素別DMGフィールドと `total_dmg_bonus()` を追加

**Files:**
- Modify: `crates/core/src/stats.rs:7-24` (struct定義)
- Modify: `crates/core/src/stat_profile.rs:70-83` (`combine_stats` — 新フィールドのコピー追加)

- [ ] **Step 1: テストを先に書く — `total_dmg_bonus()` の単体テスト**

`crates/core/src/stats.rs` の末尾に追加:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Element;

    #[test]
    fn total_dmg_bonus_pyro() {
        let stats = Stats {
            dmg_bonus: 0.10,
            pyro_dmg_bonus: 0.466,
            ..Default::default()
        };
        let result = stats.total_dmg_bonus(Some(Element::Pyro));
        assert!((result - 0.566).abs() < 1e-10);
    }

    #[test]
    fn total_dmg_bonus_physical() {
        let stats = Stats {
            dmg_bonus: 0.10,
            physical_dmg_bonus: 0.583,
            ..Default::default()
        };
        let result = stats.total_dmg_bonus(None);
        assert!((result - 0.683).abs() < 1e-10);
    }

    #[test]
    fn total_dmg_bonus_no_element_bonus() {
        let stats = Stats {
            dmg_bonus: 0.15,
            pyro_dmg_bonus: 0.466,
            ..Default::default()
        };
        // Hydro damage should NOT include Pyro bonus
        let result = stats.total_dmg_bonus(Some(Element::Hydro));
        assert!((result - 0.15).abs() < 1e-10);
    }
}
```

- [ ] **Step 2: テスト実行 — 失敗を確認**

Run: `cargo test -p genshin-calc-core --lib stats::tests -- 2>&1 | head -20`
Expected: FAIL — `pyro_dmg_bonus` フィールドが存在しない

- [ ] **Step 3: `Stats` struct に8フィールド追加**

`crates/core/src/stats.rs:23` の `pub dmg_bonus: f64,` の後に追加:

```rust
    /// Pyro DMG bonus in decimal form.
    pub pyro_dmg_bonus: f64,
    /// Hydro DMG bonus in decimal form.
    pub hydro_dmg_bonus: f64,
    /// Electro DMG bonus in decimal form.
    pub electro_dmg_bonus: f64,
    /// Cryo DMG bonus in decimal form.
    pub cryo_dmg_bonus: f64,
    /// Dendro DMG bonus in decimal form.
    pub dendro_dmg_bonus: f64,
    /// Anemo DMG bonus in decimal form.
    pub anemo_dmg_bonus: f64,
    /// Geo DMG bonus in decimal form.
    pub geo_dmg_bonus: f64,
    /// Physical DMG bonus in decimal form.
    pub physical_dmg_bonus: f64,
```

- [ ] **Step 4: `total_dmg_bonus()` メソッド実装**

`crates/core/src/stats.rs` に `use crate::types::Element;` を追加し、impl ブロックを追加:

```rust
use crate::types::Element;

impl Stats {
    /// Returns the total DMG bonus for the given element.
    /// `None` means physical damage.
    pub fn total_dmg_bonus(&self, element: Option<Element>) -> f64 {
        self.dmg_bonus
            + match element {
                Some(Element::Pyro) => self.pyro_dmg_bonus,
                Some(Element::Hydro) => self.hydro_dmg_bonus,
                Some(Element::Electro) => self.electro_dmg_bonus,
                Some(Element::Cryo) => self.cryo_dmg_bonus,
                Some(Element::Dendro) => self.dendro_dmg_bonus,
                Some(Element::Anemo) => self.anemo_dmg_bonus,
                Some(Element::Geo) => self.geo_dmg_bonus,
                None => self.physical_dmg_bonus,
            }
    }
}
```

- [ ] **Step 5: `combine_stats()` に新フィールドのコピーを追加**

`crates/core/src/stat_profile.rs` の `combine_stats()` 内、`dmg_bonus: profile.dmg_bonus,` の後に追加:

```rust
        pyro_dmg_bonus: profile.pyro_dmg_bonus,
        hydro_dmg_bonus: profile.hydro_dmg_bonus,
        electro_dmg_bonus: profile.electro_dmg_bonus,
        cryo_dmg_bonus: profile.cryo_dmg_bonus,
        dendro_dmg_bonus: profile.dendro_dmg_bonus,
        anemo_dmg_bonus: profile.anemo_dmg_bonus,
        geo_dmg_bonus: profile.geo_dmg_bonus,
        physical_dmg_bonus: profile.physical_dmg_bonus,
```

- [ ] **Step 6: テスト実行 — パスを確認**

Run: `cargo test -p genshin-calc-core --lib stats::tests`
Expected: 3 tests PASS

- [ ] **Step 7: コミット**

```bash
git add crates/core/src/stats.rs crates/core/src/stat_profile.rs
git commit -m "feat: add per-element DMG bonus fields to Stats with total_dmg_bonus()"
```

---

### Task 3: 既存テストの修正（`Stats` / `StatProfile` 構築箇所）

**Files:**
- Modify: `crates/core/tests/test_types.rs:184-193` (Stats構築 2箇所)
- Modify: `crates/data/tests/core_integration.rs:12-21,51-60` (Stats構築 2箇所)
- Modify: `crates/core/src/team.rs` (テスト内StatProfile構築 4箇所)
- Modify: `crates/data/tests/team_integration.rs:21-30` (StatProfile構築 1箇所)

- [ ] **Step 1: `Stats {` 構築箇所に `..Default::default()` を追加**

以下のファイルの `Stats {` 直接構築箇所を修正（`..Default::default()` がない箇所のみ）:
- `crates/core/tests/test_types.rs` (2箇所: Line 184, 193付近)
- `crates/data/tests/core_integration.rs` (2箇所: Line 12, 51付近)

`cargo build 2>&1 | grep "missing field"` で他に漏れがないか確認。

- [ ] **Step 2: `StatProfile {` 構築箇所に同様の修正**

以下のファイルの `StatProfile {` 直接構築箇所を修正:
- `crates/core/src/team.rs` テストモジュール (4箇所)
- `crates/data/tests/team_integration.rs` (1箇所: Line 21付近)

`cargo build 2>&1 | grep "missing field"` で他に漏れがないか確認。

- [ ] **Step 3: 全テスト実行**

Run: `cargo test 2>&1 | tail -20`
Expected: ALL PASS（dmg_bonusの振る舞い変更は未適用なので既存テストの期待値は変わらない）

- [ ] **Step 4: コミット**

```bash
git add -A
git commit -m "fix: update test Stats/StatProfile constructions for new fields"
```

---

### Task 4: `damage.rs` で `total_dmg_bonus()` を使用

**Files:**
- Modify: `crates/core/src/damage.rs:180`

- [ ] **Step 1: テストを先に書く — 元素別DMGボーナスが正しく適用されるテスト**

`crates/core/src/damage.rs` のテストモジュールに追加:

```rust
#[test]
fn element_specific_dmg_bonus_applies() {
    let input = DamageInput {
        character_level: 90,
        stats: Stats {
            atk: 2000.0,
            crit_rate: 0.75,
            crit_dmg: 1.50,
            pyro_dmg_bonus: 0.466,
            ..Default::default()
        },
        talent_multiplier: 1.0,
        damage_type: DamageType::Skill,
        element: Some(Element::Pyro),
        ..Default::default()
    };
    let enemy = Enemy::default();
    let result = calculate_damage(&input, &enemy).unwrap();
    // non_crit = 2000 * (1 + 0.466) * def_mult * res_mult
    // Pyro bonus should be applied
    assert!(result.non_crit > 0.0);
    // Verify pyro bonus is used (not zero dmg_bonus)
    let input_no_bonus = DamageInput {
        stats: Stats {
            atk: 2000.0,
            crit_rate: 0.75,
            crit_dmg: 1.50,
            ..Default::default()
        },
        ..input.clone()
    };
    let result_no_bonus = calculate_damage(&input_no_bonus, &enemy).unwrap();
    assert!(result.non_crit > result_no_bonus.non_crit);
}

#[test]
fn wrong_element_dmg_bonus_does_not_apply() {
    let input = DamageInput {
        character_level: 90,
        stats: Stats {
            atk: 2000.0,
            crit_rate: 0.75,
            crit_dmg: 1.50,
            pyro_dmg_bonus: 0.466,
            ..Default::default()
        },
        talent_multiplier: 1.0,
        damage_type: DamageType::Skill,
        element: Some(Element::Hydro), // Hydro damage, NOT Pyro
        ..Default::default()
    };
    let enemy = Enemy::default();
    let result = calculate_damage(&input, &enemy).unwrap();
    // Pyro bonus should NOT apply to Hydro damage
    let input_no_bonus = DamageInput {
        stats: Stats {
            atk: 2000.0,
            crit_rate: 0.75,
            crit_dmg: 1.50,
            ..Default::default()
        },
        ..input.clone()
    };
    let result_no_bonus = calculate_damage(&input_no_bonus, &enemy).unwrap();
    assert!((result.non_crit - result_no_bonus.non_crit).abs() < 1e-6);
}
```

- [ ] **Step 2: テスト実行 — 失敗を確認**

Run: `cargo test -p genshin-calc-core element_specific_dmg_bonus`
Expected: `wrong_element_dmg_bonus_does_not_apply` は PASS（現状 pyro_dmg_bonus=0.466 は dmg_bonus=0.0 なので元素ボーナス無効）、`element_specific_dmg_bonus_applies` は FAIL（pyro_dmg_bonus が計算に使われていない）

- [ ] **Step 3: `damage.rs` の計算式を変更**

`crates/core/src/damage.rs:180` を変更:

```rust
// 変更前:
    * (1.0 + input.stats.dmg_bonus)

// 変更後:
    * (1.0 + input.stats.total_dmg_bonus(input.element))
```

- [ ] **Step 4: テスト実行 — パスを確認**

Run: `cargo test -p genshin-calc-core element_specific_dmg_bonus`
Expected: 2 tests PASS

- [ ] **Step 5: 既存テストの修正**

`damage.rs` 内の既存テストで `stats.dmg_bonus = 0.466` のように汎用dmg_bonusに元素ボーナスを入れているテストがある場合、テストの意図に応じて:
- 汎用バフのテスト → `dmg_bonus` のまま
- 元素杯テスト → 該当元素フィールドに移動

Run: `cargo test -p genshin-calc-core 2>&1 | tail -20`
Expected: ALL PASS

- [ ] **Step 6: コミット**

```bash
git add crates/core/src/damage.rs
git commit -m "feat: use total_dmg_bonus(element) in damage calculation"
```

---

### Task 5: `team_builder.rs` の振り分け変更

**Files:**
- Modify: `crates/data/src/team_builder.rs:405-418` (`apply_ascension_stat`)
- Modify: `crates/data/src/team_builder.rs:602-614` (`apply_weapon_sub_stat`)
- Modify: `crates/data/src/team_builder.rs:168-180` (`merge_artifact_stats`)

- [ ] **Step 1: `apply_ascension_stat` と `apply_weapon_sub_stat` を `pub` に変更**

`crates/data/src/team_builder.rs` の2つの関数のシグネチャを変更:

```rust
// 変更前:
fn apply_ascension_stat(profile: &mut StatProfile, stat: &AscensionStat) {
fn apply_weapon_sub_stat(profile: &mut StatProfile, sub: &WeaponSubStat) {

// 変更後:
pub fn apply_ascension_stat(profile: &mut StatProfile, stat: &AscensionStat) {
pub fn apply_weapon_sub_stat(profile: &mut StatProfile, sub: &WeaponSubStat) {
```

これらは Task 8 で `good` crate から呼び出すために `pub` が必要。`team_builder` モジュールが `pub` でない場合は `crates/data/src/lib.rs` で `pub mod team_builder;` としてre-exportする。

- [ ] **Step 2: `apply_ascension_stat()` を変更**

`crates/data/src/team_builder.rs:414-415` を変更:

```rust
// 変更前:
AscensionStat::ElementalDmgBonus(_, v) => profile.dmg_bonus += v,
AscensionStat::PhysicalDmgBonus(v) => profile.dmg_bonus += v,

// 変更後:
AscensionStat::ElementalDmgBonus(elem, v) => match elem {
    Element::Pyro => profile.pyro_dmg_bonus += v,
    Element::Hydro => profile.hydro_dmg_bonus += v,
    Element::Electro => profile.electro_dmg_bonus += v,
    Element::Cryo => profile.cryo_dmg_bonus += v,
    Element::Dendro => profile.dendro_dmg_bonus += v,
    Element::Anemo => profile.anemo_dmg_bonus += v,
    Element::Geo => profile.geo_dmg_bonus += v,
},
AscensionStat::PhysicalDmgBonus(v) => profile.physical_dmg_bonus += v,
```

注: `Element` は `genshin_calc_core::Element` — import が必要な場合は追加する。

- [ ] **Step 3: `apply_weapon_sub_stat()` を変更**

`crates/data/src/team_builder.rs:612` を変更:

```rust
// 変更前:
WeaponSubStat::PhysicalDmgBonus(v) => profile.dmg_bonus += v[3],

// 変更後:
WeaponSubStat::PhysicalDmgBonus(v) => profile.physical_dmg_bonus += v[3],
```

- [ ] **Step 4: `merge_artifact_stats()` に新フィールドを追加**

`crates/data/src/team_builder.rs:179` の `profile.dmg_bonus += ...` の後に追加:

```rust
    profile.pyro_dmg_bonus += self.artifact_stats.pyro_dmg_bonus;
    profile.hydro_dmg_bonus += self.artifact_stats.hydro_dmg_bonus;
    profile.electro_dmg_bonus += self.artifact_stats.electro_dmg_bonus;
    profile.cryo_dmg_bonus += self.artifact_stats.cryo_dmg_bonus;
    profile.dendro_dmg_bonus += self.artifact_stats.dendro_dmg_bonus;
    profile.anemo_dmg_bonus += self.artifact_stats.anemo_dmg_bonus;
    profile.geo_dmg_bonus += self.artifact_stats.geo_dmg_bonus;
    profile.physical_dmg_bonus += self.artifact_stats.physical_dmg_bonus;
```

- [ ] **Step 5: テスト実行**

Run: `cargo test -p genshin-calc-data 2>&1 | tail -20`
Expected: ALL PASS

- [ ] **Step 6: コミット**

```bash
git add crates/data/src/team_builder.rs crates/data/src/lib.rs
git commit -m "feat: route element/physical DMG to per-element StatProfile fields"
```

---

### Task 6: `team.rs` の `is_unconditional` / `apply_buffs_to_profile` 変更

**Files:**
- Modify: `crates/core/src/team.rs:63-78` (`is_unconditional`)
- Modify: `crates/core/src/team.rs:84-106` (`apply_buffs_to_profile`)

- [ ] **Step 1: テストを先に書く**

`crates/core/src/team.rs` のテストモジュールに追加:

```rust
#[test]
fn elemental_dmg_bonus_buff_applies_to_element_field() {
    let base = StatProfile::default();
    let buffs = vec![ResolvedBuff {
        source: "test".to_string(),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        value: 0.15,
        target: BuffTarget::Team,
    }];
    let result = apply_buffs_to_profile(&base, &buffs);
    assert!((result.pyro_dmg_bonus - 0.15).abs() < 1e-10);
    assert!((result.hydro_dmg_bonus - 0.0).abs() < 1e-10);
}

#[test]
fn physical_dmg_bonus_buff_applies_to_physical_field() {
    let base = StatProfile::default();
    let buffs = vec![ResolvedBuff {
        source: "test".to_string(),
        stat: BuffableStat::PhysicalDmgBonus,
        value: 0.25,
        target: BuffTarget::Team,
    }];
    let result = apply_buffs_to_profile(&base, &buffs);
    assert!((result.physical_dmg_bonus - 0.25).abs() < 1e-10);
    assert!((result.dmg_bonus - 0.0).abs() < 1e-10);
}
```

- [ ] **Step 2: テスト実行 — 失敗を確認**

Run: `cargo test -p genshin-calc-core elemental_dmg_bonus_buff`
Expected: FAIL — `ElementalDmgBonus` は `is_unconditional` に含まれないためスキップされる

- [ ] **Step 3: `is_unconditional()` に2バリアント追加**

`crates/core/src/team.rs:76` の `| BuffableStat::DmgBonus` の後に追加:

```rust
            | BuffableStat::ElementalDmgBonus(_)
            | BuffableStat::PhysicalDmgBonus
```

- [ ] **Step 4: `apply_buffs_to_profile()` に match arms 追加**

`crates/core/src/team.rs:101` の `BuffableStat::DmgBonus => p.dmg_bonus += buff.value,` の後に追加:

```rust
            BuffableStat::ElementalDmgBonus(elem) => match elem {
                Element::Pyro => p.pyro_dmg_bonus += buff.value,
                Element::Hydro => p.hydro_dmg_bonus += buff.value,
                Element::Electro => p.electro_dmg_bonus += buff.value,
                Element::Cryo => p.cryo_dmg_bonus += buff.value,
                Element::Dendro => p.dendro_dmg_bonus += buff.value,
                Element::Anemo => p.anemo_dmg_bonus += buff.value,
                Element::Geo => p.geo_dmg_bonus += buff.value,
            },
            BuffableStat::PhysicalDmgBonus => p.physical_dmg_bonus += buff.value,
```

- [ ] **Step 5: テスト実行 — パスを確認**

Run: `cargo test -p genshin-calc-core -- team::tests`
Expected: ALL PASS

- [ ] **Step 6: コミット**

```bash
git add crates/core/src/team.rs
git commit -m "feat: route ElementalDmgBonus/PhysicalDmgBonus in team buff resolution"
```

---

### Task 7: `good` crate の `stat_map.rs` / `convert.rs` 変更

**Files:**
- Modify: `crates/good/src/stat_map.rs:119-131` (`add_to_profile` — PhysicalDmgBonus, ElementalDmgBonus arms)
- Modify: `crates/good/src/convert.rs:221-226` (`apply_two_piece_buffs` — ElementalDmgBonus, PhysicalDmgBonus arms)

- [ ] **Step 1: `stat_map.rs` の `add_to_profile()` を変更**

`crates/good/src/stat_map.rs:119-131` を変更:

```rust
// 変更前:
StatField::PhysicalDmgBonus => {
    profile.dmg_bonus += value;
    true
}
StatField::ElementalDmgBonus(elem_str) => {
    let matches = character_element.is_some_and(|ce| element_str_matches(ce, elem_str));
    if matches {
        profile.dmg_bonus += value;
    }
    matches
}

// 変更後:
StatField::PhysicalDmgBonus => {
    profile.physical_dmg_bonus += value;
    true
}
StatField::ElementalDmgBonus(elem_str) => {
    match *elem_str {
        "pyro" => profile.pyro_dmg_bonus += value,
        "hydro" => profile.hydro_dmg_bonus += value,
        "electro" => profile.electro_dmg_bonus += value,
        "cryo" => profile.cryo_dmg_bonus += value,
        "dendro" => profile.dendro_dmg_bonus += value,
        "anemo" => profile.anemo_dmg_bonus += value,
        "geo" => profile.geo_dmg_bonus += value,
        _ => return false,
    }
    true
}
```

- [ ] **Step 1b: `element_str_matches()` 関数を削除**

`crates/good/src/stat_map.rs:133-143` の `element_str_matches()` は使われなくなるため削除。

- [ ] **Step 1c: `add_to_profile()` から `character_element` パラメータを削除**

`add_to_profile()` のシグネチャから `character_element: Option<genshin_calc_core::Element>` を削除し、呼び出し元（`convert.rs` 内）のコールサイトを更新。

- [ ] **Step 2: `convert.rs` の `apply_two_piece_buffs()` を変更**

`crates/good/src/convert.rs:221-226` を変更:

```rust
// 変更前:
BuffableStat::ElementalDmgBonus(elem) => {
    if elem == character.element {
        stats.dmg_bonus += buff.value;
    }
}
BuffableStat::PhysicalDmgBonus => stats.dmg_bonus += buff.value,

// 変更後:
BuffableStat::ElementalDmgBonus(elem) => match elem {
    Element::Pyro => stats.pyro_dmg_bonus += buff.value,
    Element::Hydro => stats.hydro_dmg_bonus += buff.value,
    Element::Electro => stats.electro_dmg_bonus += buff.value,
    Element::Cryo => stats.cryo_dmg_bonus += buff.value,
    Element::Dendro => stats.dendro_dmg_bonus += buff.value,
    Element::Anemo => stats.anemo_dmg_bonus += buff.value,
    Element::Geo => stats.geo_dmg_bonus += buff.value,
},
BuffableStat::PhysicalDmgBonus => stats.physical_dmg_bonus += buff.value,
```

注: `use genshin_calc_core::Element;` が必要な場合は追加。

- [ ] **Step 2b: `convert.rs` の `ElementMismatchGoblet` 警告生成ブロックを削除**

`crates/good/src/convert.rs` の `add_to_profile` の戻り値 `false` を使って `ElementMismatchGoblet` 警告を生成しているブロック（Lines 131-137付近）を削除する。`add_to_profile` が常に `true` を返すようになったため、このコードはdead codeになる。

- [ ] **Step 3: テスト実行**

Run: `cargo test 2>&1 | tail -20`
Expected: ALL PASS

- [ ] **Step 4: clippy確認**

Run: `cargo clippy -- -D warnings 2>&1 | head -30`
Expected: No warnings（dead code は Step 1b, 1c, 2b で削除済み）

- [ ] **Step 5: コミット**

```bash
git add crates/good/src/stat_map.rs crates/good/src/convert.rs
git commit -m "feat: route element/physical DMG to per-element fields in GOOD import"
```

---

### Task 8: `build_stat_profile()` を good crate に追加

**Files:**
- Create: `crates/good/src/build_stats.rs`
- Modify: `crates/good/src/lib.rs` (module追加 + re-export)

- [ ] **Step 1: テストを先に書く**

`crates/good/src/build_stats.rs` を作成:

```rust
use genshin_calc_core::StatProfile;
use crate::CharacterBuild;

/// Converts a CharacterBuild into a StatProfile.
///
/// Uses Lv90/A6 values for base stats, weapon stats, and ascension stats.
/// Merges artifact stats and applies base defaults (CR 5%, CD 50%, ER 100%).
pub fn build_stat_profile(build: &CharacterBuild) -> StatProfile {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_stats_from_character() {
        // Build a minimal CharacterBuild and verify base stats are set
        // This test requires a known character from the data crate
        // Use find_character to get a real CharacterData
        let character = genshin_calc_data::find_character("hu_tao")
            .expect("Hu Tao should exist");
        let build = CharacterBuild {
            character,
            level: 90,
            ascension: 6,
            constellation: 0,
            talent_levels: [10, 10, 10],
            weapon: None,
            artifacts: crate::ArtifactsBuild {
                sets: vec![],
                four_piece_set: None,
                stats: StatProfile::default(),
            },
        };
        let profile = build_stat_profile(&build);
        // Hu Tao Lv90 base HP = 15552.0 (index 3)
        assert!(profile.base_hp > 15000.0);
        // Base defaults
        assert!((profile.crit_rate - 0.05).abs() < 1e-10);
        assert!((profile.crit_dmg - 0.50).abs() < 1e-10);
        assert!((profile.energy_recharge - 1.0).abs() < 1e-10);
        // Hu Tao ascension stat: CritDmg(0.384)
        assert!(profile.crit_dmg > 0.50); // 0.50 base + 0.384 ascension
    }
}
```

- [ ] **Step 2: `lib.rs` にモジュール追加**

`crates/good/src/lib.rs` に追加:

```rust
mod build_stats;
pub use build_stats::build_stat_profile;
```

- [ ] **Step 3: テスト実行 — 失敗を確認**

Run: `cargo test -p genshin-calc-good build_stat_profile`
Expected: FAIL — `todo!()` でパニック

- [ ] **Step 4: `build_stat_profile()` を実装**

```rust
pub fn build_stat_profile(build: &CharacterBuild) -> StatProfile {
    use genshin_calc_data::team_builder::{apply_ascension_stat, apply_weapon_sub_stat};

    let mut profile = StatProfile {
        base_hp: build.character.base_hp[3],
        base_atk: build.character.base_atk[3],
        base_def: build.character.base_def[3],
        crit_rate: 0.05,
        crit_dmg: 0.50,
        energy_recharge: 1.0,
        ..Default::default()
    };

    // Weapon base ATK
    if let Some(ref wb) = build.weapon {
        profile.base_atk += wb.weapon.base_atk[3];
        // Weapon sub-stat
        if let Some(ref sub) = wb.weapon.sub_stat {
            apply_weapon_sub_stat(&mut profile, sub);
        }
    }

    // Ascension stat
    apply_ascension_stat(&mut profile, &build.character.ascension_stat);

    // Artifact stats merge
    profile.hp_percent += build.artifacts.stats.hp_percent;
    profile.atk_percent += build.artifacts.stats.atk_percent;
    profile.def_percent += build.artifacts.stats.def_percent;
    profile.hp_flat += build.artifacts.stats.hp_flat;
    profile.atk_flat += build.artifacts.stats.atk_flat;
    profile.def_flat += build.artifacts.stats.def_flat;
    profile.elemental_mastery += build.artifacts.stats.elemental_mastery;
    profile.crit_rate += build.artifacts.stats.crit_rate;
    profile.crit_dmg += build.artifacts.stats.crit_dmg;
    profile.energy_recharge += build.artifacts.stats.energy_recharge;
    profile.dmg_bonus += build.artifacts.stats.dmg_bonus;
    profile.pyro_dmg_bonus += build.artifacts.stats.pyro_dmg_bonus;
    profile.hydro_dmg_bonus += build.artifacts.stats.hydro_dmg_bonus;
    profile.electro_dmg_bonus += build.artifacts.stats.electro_dmg_bonus;
    profile.cryo_dmg_bonus += build.artifacts.stats.cryo_dmg_bonus;
    profile.dendro_dmg_bonus += build.artifacts.stats.dendro_dmg_bonus;
    profile.anemo_dmg_bonus += build.artifacts.stats.anemo_dmg_bonus;
    profile.geo_dmg_bonus += build.artifacts.stats.geo_dmg_bonus;
    profile.physical_dmg_bonus += build.artifacts.stats.physical_dmg_bonus;

    profile
}
```

注: `apply_weapon_sub_stat` と `apply_ascension_stat` は Task 5 Step 1 で `pub` に変更済み。

- [ ] **Step 5: テスト実行 — パスを確認**

Run: `cargo test -p genshin-calc-good build_stat_profile`
Expected: PASS

- [ ] **Step 6: コミット**

```bash
git add crates/good/src/build_stats.rs crates/good/src/lib.rs
git commit -m "feat: add build_stat_profile() to good crate"
```

---

### Task 9: WASM `build_stats_from_good()` 関数を追加

**Files:**
- Modify: `crates/wasm/src/lib.rs` (新関数追加 + docコメント更新)

**設計判断:** `CharacterBuild` は `&'static CharacterData` 参照を含むため、JSから直接デシリアライズできない。代わりに `build_stats_from_good(json, character_id)` を提供する。GOOD JSONを内部で再パースし、指定キャラのビルドを見つけてStatsを計算する。

- [ ] **Step 1: `build_stats_from_good()` 関数を追加**

`crates/wasm/src/lib.rs` の `import_good()` の後に追加:

```rust
/// Calculates final stats for a character from a GOOD JSON export.
///
/// # Arguments
/// * `json` - GOOD format JSON string (same as `import_good`)
/// * `character_id` - Character ID to find (e.g. "hu_tao")
///
/// # Returns
/// Stats with per-element DMG bonuses in separate fields.
/// Returns null if the character is not found in the GOOD data.
#[wasm_bindgen]
pub fn build_stats_from_good(json: &str, character_id: &str) -> Result<JsValue, JsError> {
    let import = genshin_calc_good::import_good(json)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let build = import
        .builds
        .iter()
        .find(|b| b.character.id == character_id);
    match build {
        Some(b) => {
            let profile = genshin_calc_good::build_stat_profile(b);
            let stats = genshin_calc_core::combine_stats(&profile)
                .map_err(|e| JsError::new(&e.to_string()))?;
            to_js(&stats)
        }
        None => Ok(JsValue::NULL),
    }
}
```

注: `import.builds` のフィールド名は実際の `GoodImport` struct のフィールドに合わせること。実装時に `GoodImport` の定義を確認。

- [ ] **Step 2: `resolve_team_stats` のdocコメント更新**

`crates/wasm/src/lib.rs` の `resolve_team_stats` docコメント（Line 151付近）を更新:

```rust
/// # Returns
/// Stats as a JS object with hp, atk, def, elemental_mastery, crit_rate,
/// crit_dmg, energy_recharge, dmg_bonus, pyro_dmg_bonus, hydro_dmg_bonus,
/// electro_dmg_bonus, cryo_dmg_bonus, dendro_dmg_bonus, anemo_dmg_bonus,
/// geo_dmg_bonus, physical_dmg_bonus.
```

- [ ] **Step 3: ビルド確認**

Run: `cargo build -p genshin-calc-wasm 2>&1 | tail -20`
Expected: PASS（WASM targetがなくてもRustとしてコンパイル可能なことを確認）

- [ ] **Step 4: コミット**

```bash
git add crates/wasm/src/lib.rs
git commit -m "feat(wasm): add build_stats() function"
```

---

### Task 10: 全体テスト・clippy・最終確認

**Files:** なし（検証のみ）

- [ ] **Step 1: 全テスト実行**

Run: `cargo test`
Expected: ALL PASS

- [ ] **Step 2: clippy 実行**

Run: `cargo clippy -- -D warnings`
Expected: No warnings

- [ ] **Step 3: フォーマット確認**

Run: `cargo fmt --check`
Expected: No formatting issues

- [ ] **Step 4: 最終コミット（必要な場合のみ）**

clippy/fmt で修正が必要な場合のみ:

```bash
git add -A
git commit -m "chore: fix clippy warnings and formatting"
```
