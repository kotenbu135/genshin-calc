# データ拡張残タスク一括完了 — 設計書

## 概要

genshin-calc-data crateのデータカバレッジを完了させる。残りの3領域（P2 2pc聖遺物、P3 武器ConditionalBuff、P1 Nilou）を武器種別バッチで一括実装する。

## スコープ

### Phase 1: P2 2pc残り7セット

artifacts.rsの`two_piece.buffs`が空の7セットにStatBuff追加。全て無条件バフ（元素ダメ+15%、物理ダメ+25%等）。テストは既存のserde roundtripテストで自動カバー。

### Phase 2: P3 武器ConditionalBuff ~130本

武器種別に5バッチで実装: sword → claymore → polearm → bow → catalyst。

各武器のpassive_descriptionから適切なActivationパターンを選択:
- `Manual(Toggle)` — 条件付きON/OFF（最多パターン）
- `Manual(Stacks(n))` — スタック蓄積
- `Auto(StatScaling{..})` — ステータス依存計算
- `Both(Auto, Manual)` — 自動+手動の組合せ

refinement_values: 全武器R1-R5を入力。

### Phase 3: P1 Nilou

`AutoCondition::TeamElementsOnly(&[Hydro, Dendro])` で水草限定を表現。Bloom反応ダメージボーナスをTalentBuffDefとして定義。

## スキップ基準

以下の武器はConditionalBuff実装をスキップ（`conditional_buffs: &[]`のまま）:

### 星レア度によるスキップ
- 1-2星武器（パッシブなし）: 10本
- 3星武器（ダメージ影響微小）: ~30本

### パッシブ内容によるスキップ（DescriptionOnly）
パッシブがダメージ計算に影響しない武器 (~15-20本):
- HP回復・ヒーリング効果のみ
- シールド強化のみ
- エネルギー回復・元素粒子生成のみ
- クールダウン短縮のみ
- 移動速度・スタミナ等のユーティリティ

**例外:** 上記に加えてダメージバフがある武器は実装する（ダメージバフ部分のみ）。

## 実装パターン

### Toggle（最多）

```rust
conditional_buffs: &[ConditionalBuff {
    name: "weapon_id_buff_name",
    description: "日本語の効果説明",
    stat: BuffableStat::AtkPercent,
    value: 0.20,
    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

### Stacks

```rust
conditional_buffs: &[ConditionalBuff {
    name: "weapon_id_stacks",
    description: "スタック効果の説明",
    stat: BuffableStat::AtkPercent,
    value: 0.08,
    refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
    stack_values: None, // linear: value * n
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Stacks(5)),
}],
```

### StatScaling

```rust
conditional_buffs: &[ConditionalBuff {
    name: "weapon_id_scaling",
    description: "ステータス依存バフの説明",
    stat: BuffableStat::AtkFlat,
    value: 0.012,
    refinement_values: Some([0.012, 0.015, 0.018, 0.021, 0.024]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Auto(AutoCondition::StatScaling {
        stat: BuffableStat::HpPercent,
        offset: None,
        cap: None,
    }),
}],
```

## バッチ構成

| バッチ | 対象 | 推定本数 | 備考 |
|--------|------|----------|------|
| 0 | P2 2pc残り7セット | 7 | ウォームアップ |
| 1 | sword.rs | ~25本 | 35本中3星+DescOnly除外 |
| 2 | claymore.rs | ~23本 | 同上 |
| 3 | polearm.rs | ~25本 | 同上 |
| 4 | bow.rs | ~27本 | 同上 |
| 5 | catalyst.rs | ~23本 | 同上 |
| 6 | Nilou TalentBuffDef | 1 | 特殊実装 |

## テスト戦略

- Phase 1: 既存serde roundtripテストで自動カバー
- Phase 2: 武器種ごとにConditionalBuff存在確認テスト + serde roundtrip
- Phase 3: Nilou専用の統合テスト（チーム構成バリデーション含む）
- 全Phase: `cargo test` + `cargo clippy -- -D warnings` でCI通過確認

## 命名規約

- ConditionalBuff.name: `{weapon_id}_{buff_description}` (snake_case)
  - 例: `skyward_blade_cr`, `amos_bow_na_ca_dmg`
- description: 日本語でゲーム内効果を簡潔に記述
- refinement_values: R1-R5の5要素配列、パーセンテージは小数形式（10.8% → 0.108）

## 成功基準

- P2: 52/52セットの2pc StatBuff実装（100%カバレッジ）
- P3: ダメージ影響のある4-5星武器の全ConditionalBuff実装
- P1: Nilou TalentBuffDef追加、水草限定条件テスト通過
- 全テスト通過、clippy警告なし
