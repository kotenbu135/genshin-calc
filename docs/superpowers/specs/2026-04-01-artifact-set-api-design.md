# 聖遺物セット効果API 設計仕様

> Date: 2026-04-01
> Scope: `crates/good` (CharacterBuild→TeamMemberBuilder変換), `crates/wasm` (build_stats API)
> Depends on: A. build_stats設計spec (2026-03-31)

## 背景

現在WASMは `ArtifactsBuild` に `four_piece_set` のID/名前を返すが、4セット効果のバフ解決はUI側に委ねられている。UI側では `REACTION_BONUS_MAP` にハードコードしており対応範囲が限定的。

一方、`crates/data` の `TeamMemberBuilder` は既に聖遺物セット効果の完全なバフ収集・解決を実装済み：
- `collect_static_buffs()`: 2セット・4セットの無条件バフ収集
- `resolve_conditional_buffs()`: 2セット・4セットの条件付きバフ評価（`resolve_conditionals_for_source`経由）
- `available_conditionals()`: UI向けに利用可能な条件付きバフ一覧を返す

**問題**: WASM `build_stats` APIが未実装のため、この機能がWASM経由で利用できない。`CharacterBuild`（GOOD import結果）から `TeamMemberBuilder` への変換ブリッジも存在しない。

## 方針

既存の `TeamMemberBuilder` のセット効果処理をそのまま活用し、WASM `build_stats` APIで公開する。新しいバフ解決ロジックの追加は不要。

## 既存インフラ（実装済み・変更不要）

| コンポーネント | 場所 | 状態 |
|--------------|------|------|
| `TeamMemberBuilder.artifact_set` フィールド | `team_builder.rs:18` | 実装済み |
| 2pc/4pc無条件バフ収集 | `collect_static_buffs()` L337-374 | 実装済み |
| 2pc/4pc条件付きバフ評価 | `resolve_conditional_buffs()` L248-276 | 実装済み |
| 利用可能な条件付きバフ一覧 | `available_conditionals()` L114-128 | 実装済み |
| `ManualActivation` enum (`Active`, `Stacks(u8)`) | `team_builder.rs` | 実装済み |
| `find_artifact_set(id)` WASM API | `wasm/lib.rs` | 実装済み（メタデータ返却済み） |

## 新規実装

### 1. CharacterBuild → TeamMemberBuilder 変換関数

`crates/good/src/convert.rs`（または新モジュール）に変換関数を追加。

`CharacterBuild` は `crates/good` に、`TeamMemberBuilder` は `crates/data` にある。`good` は `data` に依存しているので、変換関数は `good` crate内に配置する。

```rust
/// CharacterBuild と ManualActivation リストから TeamMemberBuilder を構築
pub fn to_team_member_builder(
    build: &CharacterBuild,
    weapon_activations: &[(&str, ManualActivation)],
    artifact_activations: &[(&str, ManualActivation)],
) -> TeamMemberBuilder {
    let mut builder = TeamMemberBuilder::new(build.character, build.weapon.weapon)
        .constellation(build.constellation)
        .refinement(build.weapon.refinement);

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
            ManualActivation::Stacks(n) => builder.activate_stacks(name, *n),
        };
    }

    builder
}
```

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

impl WasmManualActivation {
    /// active: false をフィルタし、Rust側の ManualActivation に変換
    fn to_activations(input: &[WasmManualActivation]) -> Vec<(&str, ManualActivation)> {
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
}
```

WASM関数：

```rust
#[wasm_bindgen]
pub fn build_stats(input: JsValue) -> Result<JsValue, JsError> {
    let input: BuildStatsInput = serde_wasm_bindgen::from_value(input)?;
    let weapon_acts = WasmManualActivation::to_activations(&input.weapon_activations);
    let artifact_acts = WasmManualActivation::to_activations(&input.artifact_activations);
    let builder = to_team_member_builder(&input.build, &weapon_acts, &artifact_acts);
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
| `crates/good/src/convert.rs` | `to_team_member_builder()` 変換関数を新設 |
| `crates/wasm/src/lib.rs` | `build_stats` 関数追加（`BuildStatsInput` / `WasmManualActivation` 型含む） |

## テスト戦略

### 既存テスト（変更不要）

`team_builder.rs` に聖遺物セット効果のテストが既に存在：
- 剣闘士4セット AutoCondition（武器種チェック）
- 魔女4セット Stacks
- 旧貴族4セット チームバフ
- 絶縁4セット StatScaling

### 新規テスト

| テスト | 内容 | 対象ファイル |
|--------|------|-------------|
| to_team_member_builder変換 | CharacterBuild → TeamMemberBuilder が正しくフィールドをマップ | `good/convert.rs` |
| artifact_activations反映 | Toggle/Stacks指定がTeamMemberBuilderのmanual_activationsに反映 | `good/convert.rs` |
| WasmManualActivation変換 | active: false フィルタ、stacks有無でActive/Stacks分岐 | `wasm/lib.rs` |
| WASM build_stats統合 | artifact_activations付きでbuild_stats呼出し → Statsに反映 | `wasm/lib.rs` |
| build_stats セット効果なし | four_piece_set = None → 4セットバフなし | `wasm/lib.rs` |

### 検証方法

- 既存のTeamMemberBuilderテストパターンに倣う
- ゲーム内の値と照合（魔女4セット反応ボーナス等）
