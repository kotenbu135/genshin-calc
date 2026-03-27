# genshin-calc 設計ドキュメント

## 概要

原神のダメージ・元素反応計算エンジン。Rust製の汎用ライブラリ（crate）として開発し、WASM対応も見据える。

## スコープ

### v0.1.0（初期リリース）
- 単体キャラのダメージ計算（基礎ダメージ、会心、防御補正、耐性補正）
- ATKスケーリングのみ対応。HP/DEFスケーリングおよびフラット加算バフ（申鶴・雲菫等）は将来バージョンで対応

### 将来のロードマップ
1. 元素反応（蒸発・溶解・超電導・開花系など）
2. キャラクター・武器・聖遺物のステータス管理（HP/DEFスケーリング含む）
3. チーム編成・ローテーションシミュレーション

## 設計方針

- **トレイトベース設計**: 計算の各要素をトレイトで抽象化し、拡張性を確保
- **イミュータブル**: 構造体は変更せず新規作成
- **純粋関数**: 副作用なし、入力から出力が一意に決まる
- **WASM互換**: std依存を最小限に、f64ベース、serde対応
- **std使用**: `core` crateは `std` を使用するが、ファイルI/O・ネットワーク等の重い機能は使わない（`no_std`ではない）

## プロジェクト構成

```
genshin-calc/
├── Cargo.toml              # ワークスペースルート
├── CLAUDE.md
├── crates/
│   ├── core/               # 計算エンジン（データ非依存）
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs      # 公開API再エクスポート
│   │       ├── types.rs    # Element, DamageType
│   │       ├── stats.rs    # Stats構造体
│   │       ├── enemy.rs    # Enemy構造体
│   │       ├── error.rs    # CalcError エラー型
│   │       └── damage.rs   # ダメージ計算ロジック
│   └── data/               # ゲームデータ同梱（将来）
│       ├── Cargo.toml
│       └── src/
│           └── lib.rs
```

- `core`: データに一切依存しない純粋な計算ロジック
- `data`: v0.1.0ではスケルトンのみ。将来、手動管理のキャラ・武器データを同梱

## コアデータモデル

### 共通derive方針

全ての公開型に以下をderiveする:
- `Debug`, `Clone`, `PartialEq` — 全型共通
- `Copy` — `Element`, `DamageType` など小さいenum
- `Default` — `Stats`, `Enemy`
- `Serialize`, `Deserialize` — 全公開型（`serde` feature flag経由）

### types.rs

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    Pyro, Hydro, Electro, Cryo, Dendro, Anemo, Geo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    Normal, Charged, Plunging, Skill, Burst,
}
```

### stats.rs

```rust
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Stats {
    pub hp: f64,               // v0.1.0では未使用（将来のHP/DEFスケーリング用）
    pub atk: f64,              // ★ v0.1.0で使用
    pub def: f64,              // v0.1.0では未使用
    pub elemental_mastery: f64, // v0.1.0では未使用（将来の元素反応用）
    pub crit_rate: f64,        // ★ v0.1.0で使用, 0.0〜1.0
    pub crit_dmg: f64,         // ★ v0.1.0で使用, 例: 0.5 = 50%
    pub energy_recharge: f64,  // v0.1.0では未使用
    pub dmg_bonus: f64,        // ★ v0.1.0で使用, 元素/物理ダメージバフ合計
}
```

### enemy.rs

```rust
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Enemy {
    pub level: u32,
    pub resistance: f64,      // 小数表現（0.1 = 10%、-0.2 = -20%）
    pub def_reduction: f64,   // 小数表現（0.0〜1.0）
}
```

### error.rs

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

### damage.rs

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageInput {
    pub character_level: u32,
    pub stats: Stats,
    pub talent_multiplier: f64,  // 天賦倍率（例: 1.76 = 176%）
    pub damage_type: DamageType, // v0.1.0では未使用（将来のダメージタイプ別補正用）
    pub element: Option<Element>, // v0.1.0では未使用（将来の元素反応用）。Noneなら物理
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DamageResult {
    pub non_crit: f64,
    pub crit: f64,
    pub average: f64,
}
```

## 入力バリデーション

`calculate_damage` は入力を検証し、不正な場合は `Err(CalcError)` を返す。

| フィールド | 有効範囲 | エラー |
|-----------|---------|--------|
| `character_level` | 1..=90 | `InvalidCharacterLevel` |
| `enemy.level` | 1..=100 | `InvalidEnemyLevel` |
| `stats.crit_rate` | 0.0..=1.0 | `InvalidCritRate` |
| `enemy.def_reduction` | 0.0..=1.0 | `InvalidDefReduction` |
| `talent_multiplier` | > 0.0 | `InvalidTalentMultiplier` |

その他のフィールド（`atk`, `dmg_bonus`, `resistance`等）はバリデーションしない。負のATKやバフはゲーム上起きないが、計算としては問題なく動作するため。

## 計算ロジック

### ダメージ計算式

```
最終ダメージ = 基礎ダメージ × 防御補正 × 耐性補正
```

#### 基礎ダメージ

```
基礎ダメージ = ATK × 天賦倍率 × (1 + ダメージバフ)
```

#### 防御補正

```
防御補正 = (キャラLv + 100) / ((キャラLv + 100) + (敵Lv + 100) × (1 - 防御ダウン))
```

#### 耐性補正（3段階分岐）

全て小数表現（0.1 = 10%）で計算する。

| 条件 | 計算式 |
|------|--------|
| res < 0.0 | 1.0 - res / 2.0 |
| 0.0 <= res < 0.75 | 1.0 - res |
| res >= 0.75 | 1.0 / (4.0 * res + 1.0) |

#### 会心

```
会心ダメージ = 非会心ダメージ × (1 + 会心ダメージ率)
期待値 = 非会心 × (1 - 会心率) + 会心 × 会心率
```

### 公開API

```rust
pub fn calculate_damage(input: &DamageInput, enemy: &Enemy) -> Result<DamageResult, CalcError>
```

v0.1.0ではこの1関数がエントリポイント。内部関数（`base_damage`, `defense_multiplier`, `resistance_multiplier`）は非公開。

## 利用イメージ

```rust
use genshin_calc_core::{calculate_damage, DamageInput, DamageType, Stats, Enemy, Element};

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
// result.non_crit, result.crit, result.average
```

## テスト戦略

### テスト種別

| 種別 | 対象 | 例 |
|------|------|-----|
| 単体テスト | 内部関数 | 防御補正、耐性補正の各分岐 |
| ゴールデンテスト | 手計算照合 | 既知の入力→出力を検証 |
| 境界値テスト | エッジケース | 耐性0.0/0.75境界、Lv1、防御ダウン0.99 |
| バリデーションテスト | 入力検証 | 不正入力→CalcErrorを返すことを検証 |

### 方針
- 浮動小数点の比較は統一定数 `const EPSILON: f64 = 1e-6` を使用
- ゴールデンテスト（手計算照合）のみ `0.01` の許容誤差を許可
- `Stats::default()` でテストを簡潔に
- 80%+カバレッジ目標

## WASM対応方針

v0.1.0ではWASMビルドはしないが、以下の制約を守る：

- `core` crateは `std` を使用するが、ファイルI/O・ネットワーク等は使わない
- `f64`使用（WASMのnumber型と直接対応）
- 外部crateへの依存は慎重に（WASM非対応crateを避ける）
- `serde`の`Serialize`/`Deserialize`をderive

将来 `crates/wasm/` を追加し、`core`の型を`#[wasm_bindgen]`でラップする薄いレイヤーとする。

## 依存crate（v0.1.0）

| crate | バージョン | 用途 |
|-------|-----------|------|
| `serde` | 1, features = ["derive"] | シリアライズ/デシリアライズ |
| `serde_json` | 1 | JSONサポート（dev-dependencies） |
| `thiserror` | 2 | エラー型derive |

## コード品質

- `cargo fmt --check` — フォーマット確認
- `cargo clippy -- -D warnings` — lint（警告をエラー扱い）
- コミット前にローカルで上記を通すこと

## 公開・運用

- GitHub公開（crates.ioは後回し）
- MIT License（kotenbu名義）
- ゲームデータは手動管理
