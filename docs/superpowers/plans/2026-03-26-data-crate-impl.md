# Data Crate Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `genshin-calc-data` crateにv5.8の全ゲームデータ（102キャラ、全武器、聖遺物セット、敵）をRust定数として実装する

**Architecture:** `types.rs`/`buff.rs`に型定義、`characters/`/`weapons/`に元素別・武器種別のデータファイル、`lib.rs`に検索API。全データは`const`/`static`で埋め込み、WASM互換。`core`クレートの`Element`/`ScalingStat`/`DamageType`を再利用。

**Tech Stack:** Rust, serde (Serialize/Deserialize), genshin-calc-core

**Spec:** [`docs/superpowers/specs/2026-03-26-data-crate-design.md`](../specs/2026-03-26-data-crate-design.md)

---

## File Structure

```
crates/data/
├── Cargo.toml                    # 依存: genshin-calc-core, serde
├── src/
│   ├── lib.rs                    # モジュールre-export + find_*/filter API + ALL_*定数
│   ├── types.rs                  # CharacterData, TalentSet, WeaponData等の構造体 + enums
│   ├── buff.rs                   # BuffableStat, StatBuff, PassiveEffect
│   ├── characters/
│   │   ├── mod.rs                # pub mod per element + ALL_CHARACTERS
│   │   ├── pyro.rs               # 炎キャラ16体
│   │   ├── hydro.rs              # 水キャラ14体
│   │   ├── electro.rs            # 雷キャラ17体
│   │   ├── cryo.rs               # 氷キャラ18体
│   │   ├── dendro.rs             # 草キャラ11体
│   │   ├── anemo.rs              # 風キャラ15体
│   │   └── geo.rs                # 岩キャラ11体（旅人は各元素ファイルに配置）
│   ├── weapons/
│   │   ├── mod.rs                # pub mod per weapon type + ALL_WEAPONS
│   │   ├── sword.rs              # 片手剣
│   │   ├── claymore.rs           # 両手剣
│   │   ├── polearm.rs            # 長柄武器
│   │   ├── bow.rs                # 弓
│   │   └── catalyst.rs           # 法器
│   ├── artifacts.rs              # 聖遺物セット効果 + ALL_ARTIFACT_SETS
│   └── enemies.rs                # 耐性テンプレート + 敵データ + to_enemy + ALL_ENEMIES
└── tests/
    ├── search_api.rs             # find_*/filter API テスト
    ├── data_integrity.rs         # 全データの整合性バリデーション
    └── core_integration.rs       # data → core DamageInput 構築テスト
```

---

## Phase 1: 型定義 + 基盤

### Task 1: Cargo.toml + Core Enums

**Files:**
- Modify: `crates/data/Cargo.toml`
- Create: `crates/data/src/types.rs`
- Modify: `crates/data/src/lib.rs`

- [ ] **Step 1: Cargo.tomlにserde依存を追加**

```toml
[package]
name = "genshin-calc-data"
version = "0.1.0"
edition = "2024"
description = "Genshin Impact game data for genshin-calc"
license = "MIT"

[dependencies]
genshin-calc-core = { path = "../core" }
serde = { version = "1", features = ["derive"] }

[dev-dependencies]
glob = "0.3"
serde_json = "1"
```

- [ ] **Step 2: types.rsにcore enums（WeaponType, Rarity, Region）を実装**

```rust
use genshin_calc_core::Element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Claymore,
    Polearm,
    Bow,
    Catalyst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rarity {
    Star1,
    Star2,
    Star3,
    Star4,
    Star5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Region {
    Mondstadt,
    Liyue,
    Inazuma,
    Sumeru,
    Fontaine,
    Natlan,
    Snezhnaya,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AscensionStat {
    Hp(f64),
    Atk(f64),
    Def(f64),
    CritRate(f64),
    CritDmg(f64),
    ElementalMastery(f64),
    EnergyRecharge(f64),
    ElementalDmgBonus(Element, f64),
    PhysicalDmgBonus(f64),
    HealingBonus(f64),
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ArtifactRarity {
    Star4,
    Star5,
    Star4And5,
}
```

- [ ] **Step 3: lib.rsにpub mod typesを追加**

```rust
pub mod types;
```

- [ ] **Step 4: ビルド確認**

Run: `cargo build -p genshin-calc-data`
Expected: 成功（warnings OK）

- [ ] **Step 5: コミット**

```
feat(data): add core enum types (WeaponType, Rarity, Region, AscensionStat)
```

---

### Task 2: Character & Talent 構造体

**Files:**
- Modify: `crates/data/src/types.rs`

- [ ] **Step 1: TalentScaling, TalentData, NormalAttackData, TalentSet を追加**

```rust
use genshin_calc_core::{Element, ScalingStat};

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentScaling {
    pub name: &'static str,
    pub scaling_stat: ScalingStat,
    pub damage_element: Option<Element>,
    pub values: [f64; 15],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentData {
    pub name: &'static str,
    pub scalings: &'static [TalentScaling],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NormalAttackData {
    pub name: &'static str,
    pub hits: &'static [TalentScaling],
    pub charged: &'static [TalentScaling],
    pub plunging: &'static [TalentScaling],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentSet {
    pub normal_attack: NormalAttackData,
    pub elemental_skill: TalentData,
    pub elemental_burst: TalentData,
}
```

- [ ] **Step 2: CharacterData を追加**

```rust
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CharacterData {
    pub id: &'static str,
    pub name: &'static str,
    pub element: Element,
    pub weapon_type: WeaponType,
    pub rarity: Rarity,
    pub region: Region,
    pub base_hp: [f64; 4],
    pub base_atk: [f64; 4],
    pub base_def: [f64; 4],
    pub ascension_stat: AscensionStat,
    pub talents: TalentSet,
}
```

- [ ] **Step 3: ビルド確認**

Run: `cargo build -p genshin-calc-data`
Expected: 成功

- [ ] **Step 4: コミット**

```
feat(data): add CharacterData and talent type definitions
```

---

### Task 3: Weapon & Artifact 構造体

**Files:**
- Modify: `crates/data/src/types.rs`

- [ ] **Step 1: WeaponSubStat, WeaponPassive, WeaponData を追加**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WeaponSubStat {
    HpPercent([f64; 4]),
    AtkPercent([f64; 4]),
    DefPercent([f64; 4]),
    CritRate([f64; 4]),
    CritDmg([f64; 4]),
    ElementalMastery([f64; 4]),
    EnergyRecharge([f64; 4]),
    PhysicalDmgBonus([f64; 4]),
}

// WeaponPassive depends on buff.rs — forward declaration comment
// Will be completed in Task 5 after buff.rs exists
```

Note: `WeaponData`と`WeaponPassive`は`buff.rs`の`PassiveEffect`に依存する。Task 5でbuff.rs作成後に完成させる。

- [ ] **Step 2: ResistanceTemplate, EnemyData を追加**

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ResistanceTemplate {
    pub name: &'static str,
    pub pyro: f64,
    pub hydro: f64,
    pub electro: f64,
    pub cryo: f64,
    pub dendro: f64,
    pub anemo: f64,
    pub geo: f64,
    pub physical: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EnemyData {
    pub id: &'static str,
    pub name: &'static str,
    pub resistance: &'static ResistanceTemplate,
}

// SetEffect depends on buff.rs — completed in Task 5
```

- [ ] **Step 3: ビルド確認**

Run: `cargo build -p genshin-calc-data`
Expected: 成功

- [ ] **Step 4: コミット**

```
feat(data): add WeaponSubStat, ResistanceTemplate, EnemyData types
```

---

### Task 4: Buff型定義

**Files:**
- Create: `crates/data/src/buff.rs`
- Modify: `crates/data/src/lib.rs`

- [ ] **Step 1: buff.rsにBuffableStat, StatBuff, PassiveEffect を実装**

```rust
use genshin_calc_core::Element;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuffableStat {
    HpPercent,
    AtkPercent,
    DefPercent,
    HpFlat,
    AtkFlat,
    DefFlat,
    CritRate,
    CritDmg,
    ElementalMastery,
    EnergyRecharge,
    DmgBonus,
    ElementalDmgBonus(Element),
    PhysicalDmgBonus,
    NormalAtkDmgBonus,
    ChargedAtkDmgBonus,
    PlungingAtkDmgBonus,
    SkillDmgBonus,
    BurstDmgBonus,
    HealingBonus,
    ShieldStrength,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StatBuff {
    pub stat: BuffableStat,
    pub value: f64,
    pub refinement_values: Option<[f64; 5]>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PassiveEffect {
    pub description: &'static str,
    pub buffs: &'static [StatBuff],
}
```

- [ ] **Step 2: lib.rsにpub mod buffを追加**

- [ ] **Step 3: ビルド確認**

Run: `cargo build -p genshin-calc-data`

- [ ] **Step 4: コミット**

```
feat(data): add buff types (BuffableStat, StatBuff, PassiveEffect)
```

---

### Task 5: Weapon/Artifact構造体の完成

**Files:**
- Modify: `crates/data/src/types.rs`

buff.rsが存在するので、WeaponPassive, WeaponData, SetEffect, ArtifactSet を完成させる。

- [ ] **Step 1: WeaponPassive, WeaponData を追加**

```rust
use crate::buff::PassiveEffect;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeaponPassive {
    pub name: &'static str,
    pub effect: PassiveEffect,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeaponData {
    pub id: &'static str,
    pub name: &'static str,
    pub weapon_type: WeaponType,
    pub rarity: Rarity,
    pub base_atk: [f64; 4],
    pub sub_stat: Option<WeaponSubStat>,
    pub passive: Option<WeaponPassive>,
}
```

- [ ] **Step 2: SetEffect, ArtifactSet を追加**

```rust
use crate::buff::StatBuff;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetEffect {
    pub description: &'static str,
    pub buffs: &'static [StatBuff],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ArtifactSet {
    pub id: &'static str,
    pub name: &'static str,
    pub rarity: ArtifactRarity,
    pub two_piece: SetEffect,
    pub four_piece: SetEffect,
}
```

- [ ] **Step 3: ResistanceTemplate implメソッド + EnemyData.to_enemy を追加**

```rust
use genshin_calc_core::Enemy;

impl ResistanceTemplate {
    pub fn get(&self, element: Option<Element>) -> f64 {
        match element {
            Some(Element::Pyro) => self.pyro,
            Some(Element::Hydro) => self.hydro,
            Some(Element::Electro) => self.electro,
            Some(Element::Cryo) => self.cryo,
            Some(Element::Dendro) => self.dendro,
            Some(Element::Anemo) => self.anemo,
            Some(Element::Geo) => self.geo,
            None => self.physical,
        }
    }
}

impl EnemyData {
    pub fn to_enemy(&self, element: Option<Element>, level: u32, def_reduction: f64) -> Enemy {
        Enemy {
            level,
            resistance: self.resistance.get(element),
            def_reduction,
        }
    }
}
```

- [ ] **Step 4: ビルド確認**

Run: `cargo build -p genshin-calc-data`
Expected: 成功

- [ ] **Step 5: コミット**

```
feat(data): complete WeaponData, ArtifactSet, enemy conversion types
```

---

## Phase 2: 最初の垂直スライス（Diluc + テスト）

### Task 6: 敵データ

**Files:**
- Create: `crates/data/src/enemies.rs`
- Modify: `crates/data/src/lib.rs`

- [ ] **Step 1: 耐性テンプレート定数を定義**

主要パターン（10-15種）。データソース: Genshin Wiki Resistance表。

```rust
use crate::types::{EnemyData, ResistanceTemplate};

pub const ALL_10: ResistanceTemplate = ResistanceTemplate {
    name: "標準 (全耐性10%)",
    pyro: 0.10, hydro: 0.10, electro: 0.10, cryo: 0.10,
    dendro: 0.10, anemo: 0.10, geo: 0.10, physical: 0.10,
};

pub const PHYS_50_ELEM_10: ResistanceTemplate = ResistanceTemplate {
    name: "物理50% 元素10%",
    pyro: 0.10, hydro: 0.10, electro: 0.10, cryo: 0.10,
    dendro: 0.10, anemo: 0.10, geo: 0.10, physical: 0.50,
};

// ... 残りのテンプレート (pyro_immunity, cryo_70_etc, ...)
```

- [ ] **Step 2: 代表的な敵データ定数を定義**

```rust
pub const HILICHURL: EnemyData = EnemyData {
    id: "hilichurl",
    name: "ヒルチャール",
    resistance: &ALL_10,
};

// ... 主要な敵 (アビスメイジ、マトリックス等)
```

- [ ] **Step 3: ALL_ENEMIES 定数を定義**

```rust
pub const ALL_ENEMIES: &[&EnemyData] = &[
    &HILICHURL,
    // ...
];
```

- [ ] **Step 4: lib.rsにpub mod enemiesを追加 + ビルド確認**

- [ ] **Step 5: コミット**

```
feat(data): add enemy resistance templates and enemy data
```

---

### Task 7: 最初のキャラクター（Diluc）

**Files:**
- Create: `crates/data/src/characters/mod.rs`
- Create: `crates/data/src/characters/pyro.rs`
- Modify: `crates/data/src/lib.rs`

データソース: Genshin Wiki — Diluc/Talents, Diluc/Base Stats

- [ ] **Step 1: characters/pyro.rsにDilucのタレントscaling定数を定義**

```rust
use crate::types::*;
use genshin_calc_core::{Element, ScalingStat};

// 通常攻撃: 西風剣術
const DILUC_NORMAL_1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None, // 物理
    values: [0.8974, 0.9704, 1.0434, 1.1477, 1.2207, 1.3042, 1.4191, 1.5340, 1.6489, 1.7739, 1.8989, 2.0239, 2.1489, 2.2739, 2.3989],
};
// ... 残りのhits, charged, plunging, skill, burst
```

- [ ] **Step 2: Dilucの CharacterData 定数を定義**

```rust
pub const DILUC: CharacterData = CharacterData {
    id: "diluc",
    name: "Diluc",
    element: Element::Pyro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [1011.0, 10122.0, 11243.0, 12068.0],
    base_atk: [26.0, 260.0, 289.0, 311.0],
    base_def: [61.0, 612.0, 680.0, 729.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet {
        normal_attack: NormalAttackData { /* ... */ },
        elemental_skill: TalentData { /* ... */ },
        elemental_burst: TalentData { /* ... */ },
    },
};
```

- [ ] **Step 3: characters/mod.rsにpub mod pyro + 元素別定数スライスを定義**

```rust
pub mod pyro;

use crate::types::CharacterData;

pub const ALL_CHARACTERS: &[&CharacterData] = &[
    &pyro::DILUC,
    // Phase 3で残り追加
];
```

- [ ] **Step 4: lib.rsにpub mod characters を追加 + ビルド確認**

Run: `cargo build -p genshin-calc-data`

- [ ] **Step 5: コミット**

```
feat(data): add Diluc character data with full talent scalings
```

---

### Task 8: 最初の武器（Wolf's Gravestone）

**Files:**
- Create: `crates/data/src/weapons/mod.rs`
- Create: `crates/data/src/weapons/claymore.rs`
- Modify: `crates/data/src/lib.rs`

- [ ] **Step 1: weapons/claymore.rsにWolf's Gravestoneを定義**

スペックのデータ例をそのまま使用（spec §Data Examples参照）。

- [ ] **Step 2: weapons/mod.rsにpub mod claymore + ALL_WEAPONS を定義**

- [ ] **Step 3: lib.rsにpub mod weapons を追加 + ビルド確認**

- [ ] **Step 4: コミット**

```
feat(data): add Wolf's Gravestone weapon data
```

---

### Task 9: 聖遺物セット（Crimson Witch）

**Files:**
- Create: `crates/data/src/artifacts.rs`
- Modify: `crates/data/src/lib.rs`

- [ ] **Step 1: artifacts.rsにCrimson Witchを定義**

スペックのデータ例をそのまま使用。

- [ ] **Step 2: ALL_ARTIFACT_SETS 定数を定義**

- [ ] **Step 3: lib.rsにpub mod artifacts + GAME_VERSION を追加 + ビルド確認**

- [ ] **Step 4: コミット**

```
feat(data): add Crimson Witch artifact set data
```

---

### Task 10: 検索API

**Files:**
- Modify: `crates/data/src/lib.rs`

- [ ] **Step 1: find_* 関数を実装**

```rust
pub fn find_character(id: &str) -> Option<&'static CharacterData> {
    characters::ALL_CHARACTERS.iter().find(|c| c.id == id).copied()
}

pub fn find_weapon(id: &str) -> Option<&'static WeaponData> {
    weapons::ALL_WEAPONS.iter().find(|w| w.id == id).copied()
}

pub fn find_artifact_set(id: &str) -> Option<&'static ArtifactSet> {
    artifacts::ALL_ARTIFACT_SETS.iter().find(|a| a.id == id).copied()
}

pub fn find_enemy(id: &str) -> Option<&'static EnemyData> {
    enemies::ALL_ENEMIES.iter().find(|e| e.id == id).copied()
}
```

- [ ] **Step 2: filter関数を実装**

```rust
pub fn characters_by_element(element: Element) -> Vec<&'static CharacterData> {
    characters::ALL_CHARACTERS.iter().filter(|c| c.element == element).copied().collect()
}

pub fn weapons_by_type(weapon_type: WeaponType) -> Vec<&'static WeaponData> {
    weapons::ALL_WEAPONS.iter().filter(|w| w.weapon_type == weapon_type).copied().collect()
}
```

- [ ] **Step 3: ビルド確認**

Run: `cargo build -p genshin-calc-data`

- [ ] **Step 4: コミット**

```
feat(data): add search and filter API functions
```

---

### Task 11: ユニットテスト（型 + 検索API + enemy変換）

**Files:**
- Create: `crates/data/tests/search_api.rs`

- [ ] **Step 1: search_api.rsに検索テストを記述**

```rust
use genshin_calc_core::Element;
use genshin_calc_data::*;
use genshin_calc_data::types::*;

#[test]
fn find_character_by_id() {
    let diluc = find_character("diluc").expect("Diluc should exist");
    assert_eq!(diluc.name, "Diluc");
    assert_eq!(diluc.element, Element::Pyro);
    assert_eq!(diluc.weapon_type, WeaponType::Claymore);
}

#[test]
fn find_character_not_found() {
    assert!(find_character("nonexistent").is_none());
}

#[test]
fn find_weapon_by_id() {
    let wgs = find_weapon("wolfs_gravestone").expect("WGS should exist");
    assert_eq!(wgs.weapon_type, WeaponType::Claymore);
    assert_eq!(wgs.rarity, Rarity::Star5);
}

#[test]
fn find_artifact_set_by_id() {
    let cw = find_artifact_set("crimson_witch").expect("CW should exist");
    assert!(!cw.two_piece.buffs.is_empty());
}

#[test]
fn find_enemy_by_id() {
    let hilichurl = find_enemy("hilichurl").expect("Hilichurl should exist");
    assert_eq!(hilichurl.name, "ヒルチャール");
}

#[test]
fn filter_characters_by_element() {
    let pyro_chars = characters_by_element(Element::Pyro);
    assert!(pyro_chars.iter().all(|c| c.element == Element::Pyro));
    assert!(!pyro_chars.is_empty());
}

#[test]
fn filter_weapons_by_type() {
    let claymores = weapons_by_type(WeaponType::Claymore);
    assert!(claymores.iter().all(|w| w.weapon_type == WeaponType::Claymore));
}
```

- [ ] **Step 2: テスト実行**

Run: `cargo test -p genshin-calc-data`
Expected: 全テストPASS

- [ ] **Step 3: enemy変換テストを追加（同ファイルまたはsrc/types.rs内テストモジュール）**

```rust
#[test]
fn enemy_to_core_conversion() {
    let hilichurl = find_enemy("hilichurl").unwrap();
    let enemy = hilichurl.to_enemy(Some(Element::Pyro), 90, 0.0);
    assert_eq!(enemy.level, 90);
    assert!((enemy.resistance - 0.10).abs() < f64::EPSILON);
    assert!((enemy.def_reduction - 0.0).abs() < f64::EPSILON);
}

#[test]
fn resistance_template_physical() {
    let hilichurl = find_enemy("hilichurl").unwrap();
    let enemy = hilichurl.to_enemy(None, 85, 0.12);
    assert!((enemy.resistance - 0.10).abs() < f64::EPSILON);
    assert!((enemy.def_reduction - 0.12).abs() < f64::EPSILON);
}
```

- [ ] **Step 4: コミット**

```
test(data): add search API and enemy conversion tests
```

---

### Task 11.5: Serde Serialization / Roundtripテスト

**Files:**
- Create: `crates/data/tests/serialization.rs`

- [ ] **Step 1: Deserialize可能な小型enum/StatBuffのroundtripテスト**

```rust
use genshin_calc_core::Element;
use genshin_calc_data::buff::*;
use genshin_calc_data::types::*;

#[test]
fn weapon_type_roundtrip() {
    let original = WeaponType::Claymore;
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: WeaponType = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn rarity_roundtrip() {
    let original = Rarity::Star5;
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Rarity = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn region_roundtrip() {
    let original = Region::Mondstadt;
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: Region = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn ascension_stat_roundtrip() {
    let original = AscensionStat::CritRate(0.192);
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: AscensionStat = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn stat_buff_roundtrip() {
    let original = StatBuff {
        stat: BuffableStat::AtkPercent,
        value: 0.20,
        refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
    };
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: StatBuff = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}

#[test]
fn weapon_sub_stat_roundtrip() {
    let original = WeaponSubStat::AtkPercent([0.108, 0.406, 0.444, 0.496]);
    let json = serde_json::to_string(&original).unwrap();
    let deserialized: WeaponSubStat = serde_json::from_str(&json).unwrap();
    assert_eq!(original, deserialized);
}
```

- [ ] **Step 2: &'static参照型のSerialize-onlyテスト（Deserializeしない）**

```rust
use genshin_calc_data::*;

#[test]
fn character_data_serializes() {
    let diluc = find_character("diluc").unwrap();
    let json = serde_json::to_string(diluc).unwrap();
    assert!(json.contains("\"id\":\"diluc\""));
    assert!(json.contains("\"name\":\"Diluc\""));
}

#[test]
fn weapon_data_serializes() {
    let wgs = find_weapon("wolfs_gravestone").unwrap();
    let json = serde_json::to_string(wgs).unwrap();
    assert!(json.contains("\"id\":\"wolfs_gravestone\""));
}
```

- [ ] **Step 3: テスト実行**

Run: `cargo test -p genshin-calc-data -- serialization`
Expected: 全PASS

- [ ] **Step 4: コミット**

```
test(data): add serde serialization and roundtrip tests
```

---

### Task 12: データ整合性テスト

**Files:**
- Create: `crates/data/tests/data_integrity.rs`

- [ ] **Step 1: 全キャラの基礎ステータス妥当性チェック**

```rust
use genshin_calc_data::characters::ALL_CHARACTERS;

#[test]
fn all_characters_have_positive_base_stats() {
    for c in ALL_CHARACTERS {
        for &hp in &c.base_hp {
            assert!(hp > 0.0, "{}: base_hp has non-positive value {}", c.id, hp);
        }
        for &atk in &c.base_atk {
            assert!(atk > 0.0, "{}: base_atk has non-positive value {}", c.id, atk);
        }
        for &def in &c.base_def {
            assert!(def > 0.0, "{}: base_def has non-positive value {}", c.id, def);
        }
    }
}

#[test]
fn all_characters_base_stats_ascending() {
    for c in ALL_CHARACTERS {
        // Lv1 < Lv80 < Lv80+ < Lv90
        assert!(c.base_hp[0] < c.base_hp[1], "{}: HP not ascending", c.id);
        assert!(c.base_hp[1] <= c.base_hp[2], "{}: HP Lv80 > Lv80+", c.id);
        assert!(c.base_hp[2] < c.base_hp[3], "{}: HP Lv80+ > Lv90", c.id);
        // 同様にATK, DEF
    }
}

#[test]
fn all_characters_have_unique_ids() {
    let mut ids: Vec<&str> = ALL_CHARACTERS.iter().map(|c| c.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), ALL_CHARACTERS.len(), "Duplicate character IDs found");
}

#[test]
fn all_talent_values_non_negative() {
    for c in ALL_CHARACTERS {
        for scaling in c.talents.normal_attack.hits {
            for &v in &scaling.values {
                assert!(v >= 0.0, "{}: talent {} has negative value", c.id, scaling.name);
            }
        }
        // charged, plunging, skill, burst も同様
    }
}

#[test]
fn talent_values_generally_ascending() {
    // Lv1 <= Lv15 であることを確認（倍率は上がる一方）
    for c in ALL_CHARACTERS {
        for scaling in c.talents.normal_attack.hits {
            assert!(
                scaling.values[0] <= scaling.values[14],
                "{}: talent {} Lv1 ({}) > Lv15 ({})",
                c.id, scaling.name, scaling.values[0], scaling.values[14]
            );
        }
    }
}
```

- [ ] **Step 2: 武器・聖遺物・敵の整合性チェック**

```rust
use genshin_calc_data::weapons::ALL_WEAPONS;
use genshin_calc_data::artifacts::ALL_ARTIFACT_SETS;
use genshin_calc_data::enemies::ALL_ENEMIES;

#[test]
fn all_weapons_have_positive_base_atk() {
    for w in ALL_WEAPONS {
        for &atk in &w.base_atk {
            assert!(atk > 0.0, "{}: base_atk non-positive {}", w.id, atk);
        }
    }
}

#[test]
fn all_weapons_have_unique_ids() {
    let mut ids: Vec<&str> = ALL_WEAPONS.iter().map(|w| w.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), ALL_WEAPONS.len());
}

#[test]
fn all_artifact_sets_have_unique_ids() {
    let mut ids: Vec<&str> = ALL_ARTIFACT_SETS.iter().map(|a| a.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), ALL_ARTIFACT_SETS.len());
}

#[test]
fn all_enemies_have_unique_ids() {
    let mut ids: Vec<&str> = ALL_ENEMIES.iter().map(|e| e.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), ALL_ENEMIES.len());
}
```

- [ ] **Step 3: テスト実行**

Run: `cargo test -p genshin-calc-data`
Expected: 全PASS

- [ ] **Step 4: コミット**

```
test(data): add data integrity validation tests
```

---

### Task 13: Core統合テスト

**Files:**
- Create: `crates/data/tests/core_integration.rs`

- [ ] **Step 1: data crateのキャラデータからDamageInputを構築してcore計算を実行**

```rust
use genshin_calc_core::*;
use genshin_calc_data::*;

const EPSILON: f64 = 1.0; // ゲーム検証の許容誤差

#[test]
fn diluc_normal_attack_with_data_crate() {
    let diluc = find_character("diluc").unwrap();
    let talent_lv = 9; // 0-indexed (Lv10)

    // タレント倍率をdata crateから取得
    let hit1 = &diluc.talents.normal_attack.hits[0];
    let multiplier = hit1.values[talent_lv];

    // Statsはユーザー側で構築（data crateは基礎値のみ提供）
    let stats = Stats {
        hp: 20000.0,
        atk: 2000.0,
        def: 800.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };

    let hilichurl = find_enemy("hilichurl").unwrap();
    let enemy = hilichurl.to_enemy(hit1.damage_element, 90, 0.0);

    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: multiplier,
        scaling_stat: hit1.scaling_stat,
        damage_type: DamageType::Normal,
        element: hit1.damage_element,
        reaction: None,
        reaction_bonus: 0.0,
    };

    let result = calculate_damage(&input, &enemy).unwrap();
    assert!(result.non_crit > 0.0);
    assert!(result.crit > result.non_crit);
    assert!(result.average > result.non_crit);
}
```

- [ ] **Step 2: テスト実行**

Run: `cargo test -p genshin-calc-data`
Expected: PASS

- [ ] **Step 3: コミット**

```
test(data): add core integration test with Diluc data
```

---

## Phase 3: 全キャラクターデータ

各タスクは1元素 = 1ファイル。データソース: Genshin Wiki各キャラページのBase Stats + Talent Scaling表。

**各タスクの共通ステップ:**
1. Wiki/データマイニングソースから基礎ステータスとタレント倍率を収集
2. `characters/<element>.rs`に全キャラの`const`定義を記述
3. `characters/mod.rs`の`ALL_CHARACTERS`に追加
4. `cargo build -p genshin-calc-data` でコンパイル確認
5. `cargo test -p genshin-calc-data` でデータ整合性テストPASS
6. コミット

### Task 14: 炎キャラ全員（16体）

**Files:**
- Modify: `crates/data/src/characters/pyro.rs`
- Modify: `crates/data/src/characters/mod.rs`

キャラ: Amber, Bennett, Chevreuse, Diluc (Task 7で済), Gaming, Hu Tao, Klee, Lyney, Mavuika, Thoma, Xiangling, Xinyan, Yanfei, Yoimiya

- [ ] **Step 1-14: 各キャラのCharacterData定数を定義（Diluc以外13体）**
- [ ] **Step 15: ALL_CHARACTERSに追加 + ビルド + テスト**
- [ ] **Step 16: コミット**

```
feat(data): add all Pyro character data (16 characters)
```

---

### Task 15: 水キャラ全員（14体）

**Files:**
- Create: `crates/data/src/characters/hydro.rs`
- Modify: `crates/data/src/characters/mod.rs`

キャラ: Ayato, Barbara, Candace, Furina, Mualani, Neuvillette, Nilou, Sigewinne, Tartaglia, Xingqiu, Yelan

- [ ] **Step 1-11: 各キャラのCharacterData定数を定義**
- [ ] **Step 12: ALL_CHARACTERSに追加 + ビルド + テスト**
- [ ] **Step 13: コミット**

```
feat(data): add all Hydro character data (14 characters)
```

---

### Task 16: 雷キャラ全員（17体）

**Files:**
- Create: `crates/data/src/characters/electro.rs`
- Modify: `crates/data/src/characters/mod.rs`

キャラ: Beidou, Clorinde, Cyno, Dori, Fischl, Keqing, Kuki Shinobu, Lisa, Ororon, Raiden Shogun, Razor, Sara, Sethos, Yae Miko

- [ ] **Steps: 各キャラ定義 + 追加 + テスト**
- [ ] **コミット**

```
feat(data): add all Electro character data (17 characters)
```

---

### Task 17: 氷キャラ全員（18体）

**Files:**
- Create: `crates/data/src/characters/cryo.rs`
- Modify: `crates/data/src/characters/mod.rs`

キャラ: Ayaka, Aloy, Charlotte, Chongyun, Citlali, Diona, Eula, Escoffier, Freminet, Ganyu, Kaeya, Layla, Mika, Qiqi, Rosaria, Shenhe, Skirk, Wriothesley（正確なリストはテストTOMLと照合）

- [ ] **Steps: 各キャラ定義 + 追加 + テスト**
- [ ] **コミット**

```
feat(data): add all Cryo character data (18 characters)
```

---

### Task 18: 草キャラ全員（11体）

**Files:**
- Create: `crates/data/src/characters/dendro.rs`
- Modify: `crates/data/src/characters/mod.rs`

キャラ: Alhaitham, Baizhu, Collei, Emilie, Kaveh, Kinich, Kirara, Nahida, Tighnari, Yaoyao（正確なリストはテストTOMLと照合）

- [ ] **Steps: 各キャラ定義 + 追加 + テスト**
- [ ] **コミット**

```
feat(data): add all Dendro character data (11 characters)
```

---

### Task 19: 風キャラ全員（15体）

**Files:**
- Create: `crates/data/src/characters/anemo.rs`
- Modify: `crates/data/src/characters/mod.rs`

キャラ: Chasca, Faruzan, Heizou, Jean, Kazuha, Lynette, Sayu, Sucrose, Traveler (Anemo), Venti, Wanderer, Xianyun, Xiao

- [ ] **Steps: 各キャラ定義 + 追加 + テスト**
- [ ] **コミット**

```
feat(data): add all Anemo character data (15 characters)
```

---

### Task 20: 岩キャラ全員（11体）+ 旅人（残り元素）

**Files:**
- Create: `crates/data/src/characters/geo.rs`
- Modify: `crates/data/src/characters/mod.rs`

キャラ: Albedo, Chiori, Gorou, Itto, Navia, Ningguang, Noelle, Traveler (Geo), Yun Jin, Zhongli, (+Traveler variants in other element files)

- [ ] **Steps: 各キャラ定義 + 旅人バリアント + 追加 + テスト**
- [ ] **コミット**

```
feat(data): add all Geo character data (11 characters) + Traveler variants
```

---

### Task 21: データ件数カウントアサーション

**Files:**
- Modify: `crates/data/tests/data_integrity.rs`

- [ ] **Step 1: 全カテゴリの件数テスト追加**

```rust
use genshin_calc_data::characters::ALL_CHARACTERS;
use genshin_calc_data::weapons::ALL_WEAPONS;
use genshin_calc_data::artifacts::ALL_ARTIFACT_SETS;
use genshin_calc_data::enemies::ALL_ENEMIES;

#[test]
fn character_count() {
    assert_eq!(ALL_CHARACTERS.len(), 102, "Expected 102 characters for v5.8");
}

#[test]
fn weapon_count() {
    // v5.8全武器数（実装時にWikiで確認して正確な値に更新）
    assert!(ALL_WEAPONS.len() > 100, "Expected 100+ weapons for v5.8");
}

#[test]
fn artifact_set_count() {
    // v5.8全聖遺物セット数（実装時に確認して更新）
    assert!(ALL_ARTIFACT_SETS.len() > 30, "Expected 30+ artifact sets for v5.8");
}

#[test]
fn enemy_count() {
    assert!(!ALL_ENEMIES.is_empty(), "Enemy data should not be empty");
}
```

- [ ] **Step 2: テスト実行 — 全PASS確認**

Run: `cargo test -p genshin-calc-data`

- [ ] **Step 3: コミット**

```
test(data): add data count assertions (characters, weapons, artifacts, enemies)
```

---

## Phase 4: 全武器データ

各タスクは1武器種 = 1ファイル。データソース: Genshin Wiki Weapon一覧。

### Task 22: 片手剣

**Files:**
- Create: `crates/data/src/weapons/sword.rs`
- Modify: `crates/data/src/weapons/mod.rs`

- [ ] **Steps: 全片手剣の定数定義 + ALL_WEAPONSに追加 + テスト**
- [ ] **コミット**

```
feat(data): add all Sword weapon data
```

---

### Task 23: 両手剣（残り）

**Files:**
- Modify: `crates/data/src/weapons/claymore.rs`
- Modify: `crates/data/src/weapons/mod.rs`

- [ ] **Steps: WGS以外の全両手剣定数定義 + テスト**
- [ ] **コミット**

```
feat(data): add all Claymore weapon data
```

---

### Task 24: 長柄武器

**Files:**
- Create: `crates/data/src/weapons/polearm.rs`
- Modify: `crates/data/src/weapons/mod.rs`

- [ ] **Steps: 全長柄武器の定数定義 + テスト**
- [ ] **コミット**

```
feat(data): add all Polearm weapon data
```

---

### Task 25: 弓

**Files:**
- Create: `crates/data/src/weapons/bow.rs`
- Modify: `crates/data/src/weapons/mod.rs`

- [ ] **Steps: 全弓の定数定義 + テスト**
- [ ] **コミット**

```
feat(data): add all Bow weapon data
```

---

### Task 26: 法器

**Files:**
- Create: `crates/data/src/weapons/catalyst.rs`
- Modify: `crates/data/src/weapons/mod.rs`

- [ ] **Steps: 全法器の定数定義 + テスト**
- [ ] **コミット**

```
feat(data): add all Catalyst weapon data
```

---

## Phase 5: 聖遺物 + 敵データ完成

### Task 27: 全聖遺物セット

**Files:**
- Modify: `crates/data/src/artifacts.rs`

- [ ] **Steps: v5.8全聖遺物セットの定数定義 + ALL_ARTIFACT_SETSに追加 + テスト**
- [ ] **コミット**

```
feat(data): add all artifact set data
```

---

### Task 28: 敵データ充実

**Files:**
- Modify: `crates/data/src/enemies.rs`

- [ ] **Steps: 主要ボス・精鋭敵のEnemyData追加 + テスト**
- [ ] **コミット**

```
feat(data): expand enemy data with bosses and elite enemies
```

---

## Phase 6: 最終検証

### Task 29: 既存character_verificationとのクロスチェック

**Files:**
- Create: `crates/data/tests/cross_check.rs`

- [ ] **Step 1: テストTOMLからキャラ名を抽出し、data crateに全キャラ存在するか確認**

```rust
use std::fs;

#[test]
fn all_test_characters_exist_in_data_crate() {
    let pattern = concat!(env!("CARGO_MANIFEST_DIR"), "/../core/tests/data/characters/*.toml");
    let paths: Vec<_> = glob::glob(pattern)
        .unwrap()
        .filter_map(Result::ok)
        .collect();

    for path in &paths {
        let stem = path.file_stem().unwrap().to_str().unwrap();
        // TOML filename = character id (convention)
        let found = genshin_calc_data::find_character(stem);
        assert!(found.is_some(), "Character '{}' in test data but not in data crate", stem);
    }
}
```

- [ ] **Step 2: テスト実行**

Run: `cargo test -p genshin-calc-data -- cross_check`

- [ ] **Step 3: コミット**

```
test(data): add cross-check with existing character verification data
```

---

### Task 30: Clippy + fmt + 全テスト

- [ ] **Step 1: フォーマット確認**

Run: `cargo fmt --check -p genshin-calc-data`

- [ ] **Step 2: Clippy**

Run: `cargo clippy -p genshin-calc-data -- -D warnings`

- [ ] **Step 3: 全テスト**

Run: `cargo test --workspace`
Expected: 全PASS（data + coreの既存テスト128件 + data新規テスト）

- [ ] **Step 4: CLAUDE.mdのdata crate情報を更新**

- [ ] **Step 5: コミット**

```
chore(data): pass clippy, fmt, update CLAUDE.md
```

---

## 注意事項

### データ収集について
- Phase 3-5のデータ入力は[Genshin Wiki](https://genshin-impact.fandom.com/)のTalent Scaling表を参照
- タレント倍率はパーセント表記を小数形式に変換: `89.74%` → `0.8974`
- 基礎ステータスはLv1/Lv80(突破前)/Lv80(突破後)/Lv90 の4点
- 1キャラあたりのデータ入力量が大きい（~150-200 f64値）ため、元素単位でバッチ処理

### const制約
- `&'static [T]`スライスは`const`ブロック内で`&[...]`リテラルとして記述可能（Rust 2024 edition）
- `const`内で`String`は使えないので`&'static str`を使用（スペック通り）

### 旅人の扱い
- 元素ごとに別の`CharacterData`定数: `TRAVELER_ANEMO`, `TRAVELER_GEO`, etc.
- 各バリアントはそれぞれの元素ファイルに配置
- base_hp/atk/defは全元素で同一値（コピー、シンプルさ優先）
