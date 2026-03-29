# Crate公開前クリーンアップ設計

## 概要

genshin-calc-core と genshin-calc-data を crates.io 公開可能な状態にするためのドキュメント・メタデータ整備。

## 背景

- コード品質は十分（clippy 0警告、167テスト + 153データ駆動ケース）
- doc comments が0件、Cargo.tomlメタデータ不足、examples/README未作成

## スコープ

### IN
- Cargo.toml メタデータ補完（両crate）
- doc comments 追加（全公開型・関数）
- crate-level docs（`//!`）
- examples/ 作成（core 2ファイル、data 1ファイル）
- crate別 README.md 作成

### OUT
- コードリファクタリング（damage.rs分割等）
- データファイル分割
- CHANGELOG（公開時に別途作成）
- crates.io への実際の publish

## 言語方針

- doc comments / README / examples: 英語
- コード内コメント（内部実装メモ）: 日本語

## 作業順序

core crate を先に完成させてから data crate へ（依存関係の順序）。

---

## 1. Cargo.toml メタデータ

### core (crates/core/Cargo.toml)

追加フィールド:
```toml
authors = ["kotenbu"]
repository = "https://github.com/kotenbu/genshin-calc"
keywords = ["genshin-impact", "damage-calculator", "game", "wasm"]
categories = ["game-engines", "algorithms"]
rust-version = "1.85"
readme = "README.md"
```

### data (crates/data/Cargo.toml)

追加フィールド:
```toml
authors = ["kotenbu"]
repository = "https://github.com/kotenbu/genshin-calc"
keywords = ["genshin-impact", "game-data", "damage-calculator", "wasm"]
categories = ["game-engines", "data-structures"]
rust-version = "1.85"
readme = "README.md"
```

---

## 2. Core Crate ドキュメント

### 2.1 lib.rs crate-level doc

```rust
//! # genshin-calc-core
//!
//! Damage and elemental reaction calculation engine for Genshin Impact.
//!
//! Three calculation pipelines:
//! - [`calculate_damage`] — normal/skill/burst damage with amplifying/catalyze reactions
//! - [`calculate_transformative`] — transformative reactions (overloaded, superconduct, swirl, etc.)
//! - [`calculate_lunar`] — lunar reactions (lunar electro-charged, lunar bloom, lunar crystallize)
//!
//! ## Example
//! ```
//! use genshin_calc_core::*;
//!
//! let input = DamageInput {
//!     character_level: 90,
//!     stats: Stats { atk: 2000.0, crit_rate: 0.75, crit_dmg: 1.50, ..Default::default() },
//!     talent_multiplier: 1.76,
//!     scaling_stat: ScalingStat::Atk,
//!     damage_type: DamageType::Skill,
//!     element: Some(Element::Pyro),
//!     reaction: None,
//!     reaction_bonus: 0.0,
//! };
//! let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
//! let result = calculate_damage(&input, &enemy).unwrap();
//! assert!(result.average > 0.0);
//! ```
```

### 2.2 公開型の `///` doc comments

対象ファイルと方針:

| ファイル | 対象 | 方針 |
|---------|------|------|
| stats.rs | Stats | 1行説明 + 全8フィールドに `///` |
| enemy.rs | Enemy | 1行説明 + 全3フィールドに `///` |
| types.rs | Element, DamageType, ScalingStat | 1行説明、バリアントは自明なら省略 |
| error.rs | CalcError | 1行説明、バリアントは `#[error]` メッセージで十分 |
| reaction.rs | Reaction, ReactionCategory, determine_reaction | 型1行 + 関数に説明・Errors |
| em.rs | 4つのem_bonus関数 | 各1行説明 + 数式の簡潔な説明 |
| level_table.rs | reaction_base_value | 1行説明 |
| stat_profile.rs | StatProfile, combine_stats | 型1行 + フィールドdoc + 関数にErrors |
| damage.rs | DamageInput, DamageResult, calculate_damage | 型1行 + フィールドdoc + 関数にExamples/Errors |
| transformative.rs | TransformativeInput/Result, calculate_transformative | 同上 |
| lunar.rs | LunarInput/Result, calculate_lunar | 同上 |

### 2.3 doc comments のスタイル

```rust
/// Final character stats used for damage calculation.
pub struct Stats {
    /// Max HP.
    pub hp: f64,
}

/// Calculates damage for normal attacks, skills, and bursts.
///
/// Supports amplifying reactions (vaporize/melt) and catalyze reactions
/// (spread/aggravate) via the `reaction` field.
///
/// # Errors
/// Returns [`CalcError`] if any input parameter is out of valid range.
///
/// # Examples
/// ```
/// use genshin_calc_core::*;
/// // ...
/// ```
pub fn calculate_damage(...) -> Result<DamageResult, CalcError> {
```

---

## 3. Core Examples

### examples/basic_damage.rs

Dilucのスキルダメージ計算:
- 物理ダメージ（element: None）
- 炎ダメージ（element: Some(Pyro)）
- 蒸発反応（reaction: Some(Vaporize)）

3パターンの結果を `println!` で出力。

### examples/reactions.rs

- 過負荷（Overloaded）: calculate_transformative
- 月感電（LunarElectroCharged）: calculate_lunar

各結果を `println!` で出力。

---

## 4. Core README.md

構成:
1. タイトル + 1行説明
2. Installation（Cargo.toml依存記述 + MSRV: 1.85）
3. Features（3パイプライン概要）
4. Usage（コード例）
5. Supported Reactions（一覧）
6. License

---

## 5. Data Crate ドキュメント

### 5.1 lib.rs crate-level doc

`//!` で:
- 1行説明（v5.8 game data as Rust constants）
- 収録データ数（102 characters, 230 weapons, 52 artifact sets, 15 enemies）
- 検索API紹介
- 使用例（find_character + characters_by_element）

### 5.2 公開型の `///` doc comments

| ファイル | 対象 | 方針 |
|---------|------|------|
| types.rs | CharacterData, WeaponData, ArtifactSet, EnemyData 等14型 | 1行説明 + フィールドdoc |
| buff.rs | BuffableStat, StatBuff, PassiveEffect | 1行説明 |
| lib.rs | 6検索関数 + GAME_VERSION | 説明 + find_character/find_weapon にExamples |

### 5.3 examples/search.rs

- find_character でキャラ検索
- characters_by_element で元素フィルタ
- find_weapon で武器検索
- weapons_by_type で武器種フィルタ

### 5.4 README.md

構成:
1. タイトル + 1行説明
2. Installation（Cargo.toml依存記述 + MSRV: 1.85）
3. Included Data（数量一覧）
4. Usage（コード例）
5. License

---

## 6. 検証基準

- `cargo doc --no-deps` 警告なし
- `cargo test --doc` 全doc-testsパス（doc examples は原則実行可能、複雑な初期化が必要な場合のみ `no_run` 許容）
- `cargo clippy -- -D warnings` 0警告維持
- `cargo test` 既存テスト全パス維持
- `cargo run --example <name>` 全examplesが単独実行可能（core examplesはdata crateに依存しない）
- `cargo publish --dry-run -p genshin-calc-core` 成功
- `cargo publish --dry-run -p genshin-calc-data` 成功

### Doc comment チェックリスト

- [ ] 全doc comments は英語（コード内コメントは日本語OK）
- [ ] 1行説明は80文字以内
- [ ] `Result` を返す関数に `# Errors` セクション
- [ ] 主要3関数（calculate_damage/transformative/lunar）に `# Examples`
- [ ] 検索関数（find_character/find_weapon）に `# Examples`
