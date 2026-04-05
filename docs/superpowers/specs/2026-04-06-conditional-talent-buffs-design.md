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
  - `Manual(Stacks(max))` → 該当名が `Stacks(n)` として存在すれば `value * n` で追加（天賦バフには `stack_values` なし、常に線形）
  - `Auto(condition)` → 既存の `eval_auto()` で自動評価
  - `Both(auto, manual)` → auto を eval し、base_value として manual に渡す

値の計算ロジック（talent_scaling / scales_on / cap）は activation 評価の前に実行し、既存のまま共通化。

**重要**: Stacks の評価は `eval_manual()` を流用せず、`collect_talent_buffs()` 内で直接実装する。理由: `eval_manual()` は `ConditionalBuff` 固有の `stack_values` フィールドに依存しており、`TalentBuffDef` とは型が異なるため。天賦バフの Stacks は常に `computed_value * n` の線形計算。

**重要**: `Both(auto, manual)` を使う天賦バフ（シュヴルーズ等）は `self.team_elements` へのアクセスが必要。`collect_talent_buffs()` は `&self` から既にアクセス可能なため問題なし。

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

**破壊的変更の扱い**: `wasm_bindgen` は位置引数のため、`talent_activations` を `artifact_activations` と `traveler_element` の間に挿入すると既存の呼び出し側が壊れる。これは **意図的な破壊的変更** とし、smart-paimon 側で対応する。semver は minor bump（v0.6.0）で対応。

`talent_activations` が `JsValue::UNDEFINED` の場合は空配列にフォールバックする（WASM内部での防御的処理）。ただし呼び出し側は明示的に `[]` を渡すことを期待する。

### 4. `available_conditionals` の拡張

**File:** `crates/data/src/team_builder.rs`

`TeamMemberBuilder::available_conditionals()` メソッドに天賦の条件付きバフを追加する。

**型の問題**: 現在の `AvailableConditional` は `buff: &'static ConditionalBuff` を持つが、`TalentBuffDef` は別の型。以下の方針で対応：

新しい `AvailableTalentConditional` 型を追加：

```rust
pub struct AvailableTalentConditional {
    pub source: &'static str,          // キャラクター名
    pub buff: &'static TalentBuffDef,  // 天賦バフ定義（activation 付き）
}
```

`available_conditionals()` の戻り値を変更せず、新しいメソッド `available_talent_conditionals()` を追加：

```rust
impl TeamMemberBuilder {
    pub fn available_talent_conditionals(&self) -> Vec<AvailableTalentConditional> {
        // find_talent_buffs(self.character.id) から activation: Some のものを抽出
    }
}
```

WASM 側で両方を統合した応答を返す。フロントエンドは武器/聖遺物と天賦の条件付きバフを同じ UI パターンで表示できる。

### 5. `evaluate_talent_buffs` の対応

**File:** `crates/good/src/evaluate_talent_buffs.rs`

現在の `evaluate_talent_buffs()` は全天賦バフを無条件に返す関数。この関数は `get_character_team_buffs` WASM 関数から呼ばれており、変更が必要。

**方針**: `evaluate_talent_buffs()` は「max-value policy」（全バフを最大値で返す）を維持する情報提供用APIとして位置づける。ただし、`activation` 情報を `ResolvedBuff` に何らかの形で含めるか、別途条件付きフラグを返すようにする。

具体的には：
- `evaluate_talent_buffs()` のシグネチャに `talent_activations: &[(String, ManualActivation)]` を追加
- `activation: None` のバフは従来通り無条件で返す
- `activation: Some(...)` のバフは `manual_activations` と照合して評価
- 既存の呼び出し元は空の activations を渡す（後方互換）

### 6. 影響するキャラクターデータ

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
| 申鶴 | Spring Spirit Summoning (x5) | `Manual(Stacks(5))` |
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
| シュヴルーズ | In Pursuit of Ending Evil | `Both(TeamElementsOnly([Pyro, Electro]), Toggle)` |
| ナヒーダ | Compassion Illuminated | `Manual(Toggle)` |
| ファルザン | Prayerful Wind's Benefit/Bale | `Manual(Toggle)` |
| スクロース | Catalyst/Mollis Favonius | `Manual(Toggle)` |
| ユーラ | Icetide Vortex | `Manual(Stacks(2))` |
| ジャン | Lands of Dandelion (C4) | `Manual(Toggle)` |

#### 変更なし（activation: None）

issue #38 に記載されていない既存の天賦バフは全て `activation: None` を設定。

#### 申鶴の補足

申鶴の Spring Spirit Summoning は氷翎（Icy Quill）のスタック消費型バフ。1回の発動で最大5スタック（A2天賦込み）。フロントエンドでスタック数を選択可能にするため `Manual(Stacks(5))` とする。Deific Embrace（爆発中の元素ダメージ+15%）は発動中かどうかの Toggle。

#### 閑雲の補足

Crane Form（落下攻撃DMG +75%）は Toggle で簡略化する。実際には3回の落下攻撃で消費されるが、ダメージ計算機の性質上「バフが乗っている1撃」を計算するため、ONならバフ適用・OFFなら非適用で十分。

### 7. 全 `TalentBuffDef` のマイグレーション

全てのキャラクターの `TalentBuffDef` 定義に `activation: None` を追加する必要がある（構造体フィールド追加のため）。

### 8. serde 互換性

`TalentBuffDef` は `Serialize` のみ。`Option<Activation>` は `Serialize` 済み。JSON出力に `activation` フィールドが追加されるが、`None` は `null` になるだけで後方互換。

既存のテストで `serde_json::to_value` による JSON 構造比較を行っているものは `activation` フィールドの追加に合わせて更新が必要。

### 9. テスト

- 既存テスト（`team_builder.rs` 内）: `activation: None` 追加でコンパイル修正のみ、動作変更なし
- 既存テスト（`evaluate_talent_buffs.rs` 内）: **動作変更あり**。`activation: Some(...)` に変更されるキャラのテストは `evaluate_talent_buffs()` に activation を渡す形に更新が必要:
  - `test_bennett_c0_burst_lv13` → `&[("Fantastic Voyage ATK Bonus".into(), ManualActivation::Active)]` を渡す
  - `test_bennett_c6_burst_lv13` → 上記 + Spirit of Pyro の activation を渡す
  - `test_furina_c0_burst_lv10` → `Stacks(300)` を渡す
  - `test_furina_c1_*` → 同上
  - `test_yelan_a4_returns_max` → Toggle active を渡す
  - `test_nahida_a1_*` → ナヒーダ A1 は `activation: None`（issue #38 では Compassion Illuminated のみ対象）なので変更不要
  - `test_sucrose_a4_em_sharing` → Toggle active を渡す
- 新規テスト:
  - マーヴィカ: activations 空 → buffs_provided にA1/A4なし、Toggle ON → あり
  - ベネット: activations 空 → Fantastic Voyage なし、Toggle ON → あり
  - フリーナ: Stacks(150) → DMG +37.5%、Stacks(0) or 未指定 → DMG +0%
  - 申鶴: Stacks(3) → 3スタック分のIcy Quill flat DMG
  - シュヴルーズ: `Both(TeamElementsOnly, Toggle)` — 炎/雷のみ編成 + Toggle ON → バフ適用、編成条件不成立 → 不適用
  - `available_talent_conditionals()` に天賦条件付きバフが含まれること
  - `evaluate_talent_buffs` に activations 渡して条件付きバフの有無を検証
  - AscensionPassive ソースの条件付きバフ（レベルによるゲーティングは既存の max-value policy で不要）

### 10. 関連 WASM 関数への影響

#### `get_character_team_buffs`

`evaluate_talent_buffs()` を直接呼ぶ。`evaluate_talent_buffs()` のシグネチャ変更に伴い、`&[]` を渡すように更新する。この関数は情報提供用の max-value policy API のままとし、外部シグネチャは変更しない（talent_activations パラメータを追加しない）。

#### `build_stats`

`to_team_member_builder()` → `.build()` を経由するため、`collect_talent_buffs()` の変更の影響を受ける。ただし `build_stats` は `manual_activations` に天賦の activation を含めないため、条件付き天賦バフは自動的にスキップされる（`activation: Some` のバフは manual_activations にマッチしないため追加されない）。これは意図的な動作：`build_stats` は武器・聖遺物の activation のみをサポートし、天賦 activation は `build_team_member` でのみサポートする。

#### `build_stats_from_good`

activation を一切受け取らない関数。条件付き天賦バフはスキップされる。変更不要。

### 11. `AvailableTalentConditional` の derive

```rust
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AvailableTalentConditional {
    pub source: &'static str,
    pub buff: &'static TalentBuffDef,
}
```

WASM 出力用に `Serialize` が必要。`&'static` 参照型のため `Deserialize` は不要（プロジェクト慣行通り）。

### 12. Naming Convention

天賦バフの `name` フィールドは既存通り人間が読める英語名（例: `"Fantastic Voyage ATK Bonus"`）をそのまま activation のルックアップキーとして使用する。武器/聖遺物の `ConditionalBuff.name` はスネークケースの機械可読名だが、天賦バフは既に人間可読名で統一されているため変更しない。フロントエンド側でのマッチングもこの名前ベースで行う。
