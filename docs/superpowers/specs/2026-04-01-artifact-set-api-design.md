# 聖遺物セット効果API 設計仕様

> Date: 2026-04-01
> Scope: `crates/data` (TeamMemberBuilder), `crates/wasm` (build_stats API)
> Depends on: A. build_stats設計spec (2026-03-31)

## 背景

現在WASMは `ArtifactsBuild` に `four_piece_set` のID/名前を返すが、4セット効果のバフ解決はUI側に委ねられている。UI側では `REACTION_BONUS_MAP` にハードコードしており対応範囲が限定的。

一方、`crates/data` には既に `ArtifactSet.four_piece: SetEffect` として全セット効果のデータ（`buffs` + `conditional_buffs`）が定義済み。武器パッシブと同じ `ConditionalBuff` / `Activation` 型を使用。

## 方針

武器パッシブと同じフロー（`TeamMemberBuilder` → `resolve_team_stats`）に聖遺物4セット効果を乗せる。新しい概念の導入なし。

## スコープ

### 対象

| 種類 | 例 | 処理方式 |
|------|-----|---------|
| 2セット無条件バフ | 剣闘士2: ATK+18% | 既存（import時マージ済み、変更なし） |
| 4セット無条件バフ | 剣闘士4: NormalAtkDmgBonus+35%（Auto: 武器種条件） | `collect_static_buffs` に追加 |
| 4セット条件付きバフ | 魔女4: 反応ボーナス+15%/スタック | `resolve_conditional_buffs` に追加 |

### 対象外（YAGNI）

- 2セット条件付きバフ: 現在のデータで全て `&[]`（空）。将来追加時は同じパターンでループ追加するだけ。

## 変更詳細

### 1. TeamMemberBuilder フィールド追加

```rust
pub struct TeamMemberBuilder {
    // 既存フィールド...
    pub four_piece_set: Option<&'static ArtifactSet>,  // 追加
}
```

### 2. collect_static_buffs() 拡張

4セットの無条件バフを収集:

```rust
fn collect_static_buffs(&self) -> Vec<ResolvedBuff> {
    let mut buffs = Vec::new();

    // 既存: 武器無条件バフ
    // ...

    // 追加: 4セット無条件バフ
    if let Some(set) = self.four_piece_set {
        for buff in set.four_piece.buffs {
            buffs.push(ResolvedBuff {
                source: BuffSource::ArtifactSet,
                stat: buff.stat,
                value: buff.value,
                target: BuffTarget::OnlySelf,
            });
        }
    }

    buffs
}
```

### 3. resolve_conditional_buffs() 拡張

4セットの条件付きバフを評価:

```rust
fn resolve_conditional_buffs(&self, profile: &StatProfile, buffs: &mut Vec<ResolvedBuff>) {
    // 既存: 武器条件付きバフ
    self.resolve_weapon_conditionals(profile, buffs);

    // 追加: 4セット条件付きバフ
    if let Some(set) = self.four_piece_set {
        for cond in set.four_piece.conditional_buffs {
            self.try_resolve_conditional(cond, None, profile, buffs);
            //                           refinement_level = None (聖遺物にrefinementなし)
        }
    }
}
```

### 4. BuffSource enum拡張

```rust
pub enum BuffSource {
    Weapon,
    Character,
    ArtifactSet,  // 追加
    Resonance,
}
```

### 5. WASM build_stats API入力変更

```typescript
interface BuildStatsInput {
  build: CharacterBuild;
  weapon_activations?: ManualActivation[];
  artifact_activations?: ManualActivation[];  // 追加
}

interface ManualActivation {
  name: string;
  active: boolean;
  stacks?: number;
}
```

WASM側:

```rust
#[wasm_bindgen]
pub fn build_stats(input: JsValue) -> Result<JsValue, JsError> {
    let input: BuildStatsInput = serde_wasm_bindgen::from_value(input)?;
    let builder = TeamMemberBuilder::from_character_build(
        &input.build,
        &input.weapon_activations,
        &input.artifact_activations,
    );
    let member = builder.build()?;
    let stats = combine_stats(&member.stats)?;
    to_js(&stats)
}
```

### 6. UI側フロー

1. `import_good()` → `CharacterBuild` 取得（`four_piece_set` のID含む）
2. `find_artifact_set(id)` → 4セットの `conditional_buffs` をUI表示（Toggle/Stacksコントロール）
3. ユーザーが有効化を選択
4. `build_stats({ build, artifact_activations })` → 最終Stats取得

追加WASM API不要。`find_artifact_set` が既にセット効果メタデータを返している。

## 変更ファイル一覧

| ファイル | 変更内容 |
|---------|---------|
| `crates/data/src/team_builder.rs` | `four_piece_set` フィールド追加、`collect_static_buffs` / `resolve_conditional_buffs` 拡張 |
| `crates/core/src/buff_types.rs` | `BuffSource::ArtifactSet` variant追加 |
| `crates/good/src/convert.rs` | `CharacterBuild` → `TeamMemberBuilder` 変換時に `four_piece_set` を渡す |
| `crates/wasm/src/lib.rs` | `build_stats` 入力に `artifact_activations` 追加 |

## テスト戦略

| テスト | 内容 | 対象ファイル |
|--------|------|-------------|
| 4セット無条件バフ（Auto条件） | 剣闘士4セット: 片手剣キャラ → NormalAtkDmgBonus+35%、法器キャラ → 適用なし | `team_builder.rs` |
| 4セット条件付きバフ（Toggle） | 血染め4セット: Toggle ON → ChargedAtkDmgBonus+50% | `team_builder.rs` |
| 4セット条件付きバフ（Stacks） | 魔女4セット: 2スタック → ElementalDmgBonus +100%（50%×2） | `team_builder.rs` |
| 4セット反応ボーナス | 魔女4セット: AmplifyingBonus+15%, TransformativeBonus+40% | `team_builder.rs` |
| WASM build_stats統合 | artifact_activations付きでbuild_stats呼出し → Statsに反映 | `wasm/lib.rs` |
| セット効果なし | four_piece_set = None → バフ追加なし | `team_builder.rs` |
