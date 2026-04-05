# Conditional Talent Buffs Design

**Issue:** #38 — `build_team_member`: スキル/爆発依存のバフが `buffs_provided` に常時含まれている
**Date:** 2026-04-06
**Approach:** A — `TalentBuffDef` に `activation` フィールドを追加

## Problem

`build_team_member(good, character_id, [], [])` で activation を空にして呼び出しても、スキル/元素爆発/特定条件の発動が必要な天賦バフが `buffs_provided` に無条件で含まれ、`resolve_team_stats` で常時適用されてしまう。

武器・聖遺物では `buffs`（無条件）と `conditional_buffs`（条件付き）の分離が確立されているが、天賦バフ (`TalentBuffDef`) には条件付きの概念がなく、全て無条件として扱われている。

## Design

### 1. Data Model: `TalentBuffDef` に `activation` フィールド追加

**File:** `crates/data/src/talent_buffs/mod.rs`

```rust
pub struct TalentBuffDef {
    pub name: &'static str,
    pub description: &'static str,
    pub stat: BuffableStat,
    pub base_value: f64,
    pub scales_with_talent: bool,
    pub talent_scaling: Option<&'static [f64; 15]>,
    pub scales_on: Option<ScalingStat>,
    pub target: BuffTarget,
    pub source: TalentBuffSource,
    pub min_constellation: u8,
    pub cap: Option<f64>,
    pub activation: Option<Activation>,  // NEW: None = unconditional, Some = conditional
}
```

既存の `Activation` enum (`Auto`, `Manual`, `Both`) をそのまま再利用する。

### 2. Logic: `collect_talent_buffs()` の分岐

**File:** `crates/data/src/team_builder.rs`

`collect_talent_buffs()` で `activation` フィールドを評価：

- `activation: None` → 既存通り無条件で `buffs` に追加
- `activation: Some(activation)` → `manual_activations` と照合し、条件成立時のみ追加
  - `Manual(Toggle)` → 該当名が `manual_activations` に `Active` として存在すれば追加
  - `Manual(Stacks(max))` → 該当名が `Stacks(n)` として存在すれば `value * n` で追加
  - `Auto(condition)` → `eval_auto()` で自動評価
  - `Both(auto, manual)` → auto を eval し、base_value として manual に渡す

値の計算ロジック（talent_scaling / scales_on / cap）は activation 評価の前に実行し、既存のまま共通化。

### 3. WASM API: `build_team_member` にパラメータ追加

**File:** `crates/wasm/src/lib.rs`

```rust
#[wasm_bindgen]
pub fn build_team_member(
    json: &str,
    character_id: &str,
    weapon_activations: JsValue,
    artifact_activations: JsValue,
    talent_activations: JsValue,       // NEW
    traveler_element: Option<String>,
) -> Result<JsValue, JsError>
```

`talent_activations` は武器・聖遺物と同じ `[{name, active, stacks?}]` 形式。

`JsValue::UNDEFINED` の場合は空配列にフォールバック（後方互換）。

### 4. `get_available_conditionals` の拡張

**File:** `crates/wasm/src/lib.rs`

既存の `get_available_conditionals()` が返す条件付きバフリストに、天賦の条件付きバフ（`activation: Some` のもの）を追加。フロントエンドはこのAPIで activation UI を構築する。

天賦バフの `AvailableConditional` は source をキャラクター名にする。

### 5. 影響するキャラクターデータ

#### 影響大（activation 追加）

| キャラ | バフ名 | activation |
|--------|--------|-----------|
| マーヴィカ | Sunfrost Encomium ATK Bonus | `Manual(Toggle)` |
| マーヴィカ | Fire-Forged Heritage DMG Bonus | `Manual(Toggle)` |
| ベネット | Fantastic Voyage ATK Bonus | `Manual(Toggle)` |
| ベネット | Spirit of Pyro (C6) | `Manual(Toggle)` |
| フリーナ | Let the People Rejoice | `Manual(Stacks(300))` |
| フリーナ | Let the People Rejoice (DMG%) | `Manual(Stacks(300))` |
| 夜蘭 | Adapt With Ease | `Manual(Toggle)` |
| モナ | Stellaris Phantasm | `Manual(Toggle)` |
| 申鶴 | Spring Spirit Summoning (x5) | `Manual(Toggle)` |
| 申鶴 | Deific Embrace (x5) | `Manual(Toggle)` |
| 閑雲 | Stars Gather at Dusk | `Manual(Toggle)` |
| 閑雲 | Crane Form | `Manual(Toggle)` |
| 鍾離 | Jade Shield RES Shred (x8) | `Manual(Toggle)` |

#### 影響中（activation 追加）

| キャラ | バフ名 | activation |
|--------|--------|-----------|
| シロネン | Yohualliand RES Reduction | `Manual(Toggle)` |
| シトラリ | Itzpapa/Cold Moon RES Shred | `Manual(Toggle)` |
| シトラリ | Heart Devourer's Travail | `Manual(Toggle)` |
| シュヴルーズ | Overloaded RES Shred | `Manual(Toggle)` |
| シュヴルーズ | In Pursuit of Ending Evil | `Both(TeamElementsOnly, Toggle)` |
| ナヒーダ | Compassion Illuminated | `Manual(Toggle)` |
| ファルザン | Prayerful Wind's Benefit/Bale | `Manual(Toggle)` |
| スクロース | Catalyst/Mollis Favonius | `Manual(Toggle)` |
| ユーラ | Icetide Vortex | `Manual(Stacks(2))` |
| ジャン | Lands of Dandelion (C4) | `Manual(Toggle)` |

#### 変更なし（activation: None）

issue #38 に記載されていない既存の天賦バフは全て `activation: None` を設定。

### 6. 全 `TalentBuffDef` のマイグレーション

全てのキャラクターの `TalentBuffDef` 定義に `activation: None` を追加する必要がある（構造体フィールド追加のため）。

### 7. serde 互換性

`TalentBuffDef` は `Serialize` のみ。`Option<Activation>` は `Serialize` 済み。JSON出力に `activation` フィールドが追加されるが、`None` は `null` になるだけで後方互換。

### 8. テスト

- 既存テスト: `activation: None` 追加でコンパイル修正のみ、動作変更なし
- 新規テスト:
  - マーヴィカ: activations 空 → buffs_provided にA1/A4なし、Toggle ON → あり
  - ベネット: activations 空 → Fantastic Voyage なし、Toggle ON → あり
  - フリーナ: Stacks(150) → DMG +37.5%、Stacks(0) or 未指定 → DMG +0%
  - `get_available_conditionals` に天賦条件付きバフが含まれること
