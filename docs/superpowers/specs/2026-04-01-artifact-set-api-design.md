# 聖遺物セット効果API 設計仕様

> Date: 2026-04-01
> Scope: `crates/data` (TeamMemberBuilder小修正), `crates/good` (CharacterBuild→TeamMemberBuilder変換), `crates/wasm` (build_stats API)
> Depends on: A. build_stats設計spec (2026-03-31) — `combine_stats()` 関数はそちらで定義

## 背景

現在WASMは `ArtifactsBuild` に `four_piece_set` のID/名前を返すが、4セット効果のバフ解決はUI側に委ねられている。UI側では `REACTION_BONUS_MAP` にハードコードしており対応範囲が限定的。

一方、`crates/data` の `TeamMemberBuilder` は既に聖遺物セット効果の完全なバフ収集・解決を実装済み：
- `collect_static_buffs()`: 2セット・4セットの無条件バフ収集
- `resolve_conditional_buffs()`: 2セット・4セットの条件付きバフ評価（`resolve_conditionals_for_source`経由）
- `available_conditionals()`: UI向けに利用可能な条件付きバフ一覧を返す

**問題**: WASM `build_stats` APIが未実装のため、この機能がWASM経由で利用できない。`CharacterBuild`（GOOD import結果）から `TeamMemberBuilder` への変換ブリッジも存在しない。

## 方針

既存の `TeamMemberBuilder` のセット効果処理をそのまま活用し、WASM `build_stats` APIで公開する。新しいバフ解決ロジックの追加は不要。

## 既存インフラ（実装済み）

| コンポーネント | 場所 | 状態 |
|--------------|------|------|
| `TeamMemberBuilder.artifact_set` フィールド | `team_builder.rs` | 実装済み |
| 2pc/4pc無条件バフ収集 | `collect_static_buffs()` | 実装済み |
| 2pc/4pc条件付きバフ評価 | `resolve_conditional_buffs()` | 実装済み |
| 利用可能な条件付きバフ一覧 | `available_conditionals()` | 実装済み |
| `ManualActivation` enum (`Active`, `Stacks(u8)`) | `team_builder.rs` | 実装済み |
| `find_artifact_set(id)` WASM API | `wasm/lib.rs` | 実装済み（メタデータ返却済み） |

## 新規実装

### 0. TeamMemberBuilder: manual_activations の型変更（前提変更）

現在 `manual_activations` は `Vec<(&'static str, ManualActivation)>` で、`activate()` / `activate_with_stacks()` は `&'static str` を要求する。WASM入力は `String` なので `&'static str` を満たせない。

**変更**: `manual_activations` の型を `Vec<(String, ManualActivation)>` に変更。

```rust
// team_builder.rs
pub struct TeamMemberBuilder {
    // ...
    manual_activations: Vec<(String, ManualActivation)>,  // &'static str → String
}

pub fn activate(mut self, name: &str) -> Self {          // &'static str → &str
    self.manual_activations.push((name.to_string(), ManualActivation::Active));
    self
}

pub fn activate_with_stacks(mut self, name: &str, stacks: u8) -> Self {  // &'static str → &str
    self.manual_activations.push((name.to_string(), ManualActivation::Stacks(stacks)));
    self
}
```

`eval_manual` 内の比較 `*n == cond_buff.name` は `String == &str` で動作するため、他の変更なし。既存の呼び出し元（`&'static str` を渡すコード）も `&str` に自動coerceされるため後方互換。

### 1. CharacterBuild → TeamMemberBuilder 変換関数

`crates/good/src/convert.rs` に変換関数を追加。`good` は `data` に依存しているので `good` crate内に配置。

```rust
/// CharacterBuild と ManualActivation リストから TeamMemberBuilder を構築。
/// weapon が None の場合は Err を返す（ステータス計算には武器が必須）。
pub fn to_team_member_builder(
    build: &CharacterBuild,
    weapon_activations: &[(&str, ManualActivation)],
    artifact_activations: &[(&str, ManualActivation)],
) -> Result<TeamMemberBuilder, GoodError> {
    let weapon_build = build.weapon.as_ref()
        .ok_or(GoodError::MissingWeapon)?;

    let mut builder = TeamMemberBuilder::new(build.character, weapon_build.weapon)
        .constellation(build.constellation)
        .talent_levels(build.talent_levels)
        .refinement(weapon_build.refinement);

    // 聖遺物ステータス（StatProfile）をマージ
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

**注意点:**
- `weapon` が `None` の場合は `GoodError::MissingWeapon` を返す（ステータス計算には武器基礎攻撃力が必須）
- `talent_levels` を設定（デフォルト `[1,1,1]` の回避）
- 2セット無条件バフは `build.artifacts.stats` に既にマージ済み（`import_good` 時に `apply_two_piece_buffs` で処理）

### 2. WASM build_stats API入力

JS/TS側の入力型：

```typescript
interface BuildStatsInput {
  build: CharacterBuild;
  weapon_activations?: WasmManualActivation[];
  artifact_activations?: WasmManualActivation[];
}

interface WasmManualActivation {
  name: string;       // ConditionalBuff.name に対応
  active: boolean;    // false の場合はフィルタされる
  stacks?: number;    // 指定時は Stacks(n)、未指定時は Active
}
```

WASM側のデシリアライズ：

```rust
#[derive(Deserialize)]
struct WasmManualActivation {
    name: String,
    active: bool,
    stacks: Option<u8>,
}

/// active: false をフィルタし、Rust側の (&str, ManualActivation) ペアに変換
fn convert_activations(input: &[WasmManualActivation]) -> Vec<(&str, ManualActivation)> {
    input.iter()
        .filter(|a| a.active)
        .map(|a| {
            let activation = match a.stacks {
                Some(n) => ManualActivation::Stacks(n),
                None => ManualActivation::Active,
            };
            (a.name.as_str(), activation)
        })
        .collect()
}
```

WASM関数（`combine_stats` はA設計specで定義）：

```rust
#[wasm_bindgen]
pub fn build_stats(input: JsValue) -> Result<JsValue, JsError> {
    let input: BuildStatsInput = serde_wasm_bindgen::from_value(input)?;
    let weapon_acts = convert_activations(&input.weapon_activations.unwrap_or_default());
    let artifact_acts = convert_activations(&input.artifact_activations.unwrap_or_default());
    let builder = to_team_member_builder(&input.build, &weapon_acts, &artifact_acts)?;
    let member = builder.build()?;
    let stats = combine_stats(&member.stats)?;
    to_js(&stats)
}
```

### 3. UI側フロー

1. `import_good()` → `CharacterBuild` 取得（`four_piece_set` 含む）
2. `find_artifact_set(id)` → 4セットの `conditional_buffs` をUI表示（Toggle/Stacksコントロール）
3. ユーザーが有効化を選択
4. `build_stats({ build, artifact_activations })` → 最終Stats取得

追加WASM API不要。`find_artifact_set` が既にセット効果メタデータを返している。

## 変更ファイル一覧

| ファイル | 変更内容 |
|---------|---------|
| `crates/data/src/team_builder.rs` | `manual_activations` 型を `Vec<(String, ManualActivation)>` に変更、`activate`/`activate_with_stacks` のシグネチャを `&str` に変更 |
| `crates/good/src/convert.rs` | `to_team_member_builder()` 変換関数を新設、`GoodError::MissingWeapon` variant追加 |
| `crates/wasm/src/lib.rs` | `build_stats` 関数追加（`BuildStatsInput` / `WasmManualActivation` / `convert_activations` 含む） |

## テスト戦略

### 既存テスト（変更不要）

`team_builder.rs` に聖遺物セット効果のテストが既に存在：
- 剣闘士4セット AutoCondition（武器種チェック）
- 魔女4セット Stacks
- 旧貴族4セット チームバフ
- 絶縁4セット StatScaling

既存テストは `activate("name")` を `&'static str` で呼び出しており、`&str` への変更で後方互換のためそのままコンパイル可能。

### 新規テスト

| テスト | 内容 | 対象ファイル |
|--------|------|-------------|
| to_team_member_builder変換 | CharacterBuild → TeamMemberBuilder が正しくフィールドをマップ（weapon, constellation, talent_levels, artifact_set） | `good/convert.rs` |
| weapon None エラー | weapon が None の場合に GoodError::MissingWeapon を返す | `good/convert.rs` |
| artifact_activations反映 | Toggle/Stacks指定がTeamMemberBuilderのmanual_activationsに反映 | `good/convert.rs` |
| convert_activations変換 | active: false フィルタ、stacks有無でActive/Stacks分岐 | `wasm/lib.rs` |
| WASM build_stats統合 | artifact_activations付きでbuild_stats呼出し → Statsに反映 | `wasm/lib.rs` |
| build_stats セット効果なし | four_piece_set = None → 4セットバフなし | `wasm/lib.rs` |

### 検証方法

- 既存のTeamMemberBuilderテストパターンに倣う
- ゲーム内の値と照合（魔女4セット反応ボーナス等）
