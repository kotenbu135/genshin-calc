# データ拡張残タスク一括完了 — 設計書

## 概要

genshin-calc-data crateのデータカバレッジを完了させる。残りの3領域（P2 2pc聖遺物、P3 武器ConditionalBuff、P1 Nilou）を武器種別バッチで一括実装する。

## スコープ

### Phase 1: P2 2pc残り7セット → スキップ（意図的に空）

残り7セットの2pc効果はいずれもBuffableStatで表現不可能な効果:
- 耐性+: Thundersoother（雷耐性+40%）、Lavawalker（炎耐性+40%）、Tiny Miracle（全元素耐性+20%）
- 元素影響時間-: Prayers系4種（炎/水/雷/氷の影響時間-40%）

これらはダメージ計算に影響しないユーティリティ効果であり、`buffs: &[]`のまま据え置く。既存45/52の実装で実質100%カバレッジ。

### Phase 2: P3 武器ConditionalBuff ~130本

武器種別に5バッチで実装: sword → claymore → polearm → bow → catalyst。

各武器のpassive_descriptionから適切なActivationパターンを選択:
- `Manual(Toggle)` — 条件付きON/OFF（最多パターン）
- `Manual(Stacks(n))` — スタック蓄積
- `Auto(StatScaling{..})` — ステータス依存計算
- `Both(Auto, Manual)` — 自動+手動の組合せ

refinement_values: 全武器R1-R5を入力。

### Phase 3: P1 Nilou（別途ミニスペック）

Nilouの開花反応ボーナスは通常のステータスバフと異なり、反応ダメージ自体の変更を伴う特殊実装。TalentBuffDefに`activation`フィールドがない現状では、Phase 2完了後に別途ミニスペックを作成して設計を詰める。

検討事項:
- TalentBuffDefに`auto_condition: Option<AutoCondition>`を追加するか、別アプローチか
- 対象パッシブ: A1（金盞花の宮、Bloom→BountifulBloom変換）とA2（夢蓮の舞、HP30000超過分×0.9%のBloomダメボーナス）
- BuffableStat::TransformativeBonusでBloomダメージボーナスを表現可能か
- HP依存スケーリング（A2）のStatScaling連携

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

### Stacks（線形）

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

### Stacks（非線形）

スタック値が単純な倍数でない場合は`stack_values`を使用:

```rust
conditional_buffs: &[ConditionalBuff {
    name: "weapon_id_stacks",
    description: "非線形スタック効果の説明",
    stat: BuffableStat::DmgBonus,
    value: 0.08,
    refinement_values: None,
    stack_values: Some(&[0.08, 0.16, 0.28]), // 3スタック: 8%/16%/28%
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Stacks(3)),
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

### 複数ConditionalBuff（1武器に2+バフ）

多くの武器は複数の条件付き効果を持つ。既存例: ATHAME_ARTIS（CR+SkillDMG）、LIGHTBEARING_MOONSHARD（ATK+CR）。

```rust
conditional_buffs: &[
    ConditionalBuff {
        name: "weapon_id_atk",
        description: "効果1の説明",
        stat: BuffableStat::AtkPercent,
        value: 0.20,
        refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
    ConditionalBuff {
        name: "weapon_id_dmg",
        description: "効果2の説明",
        stat: BuffableStat::DmgBonus,
        value: 0.16,
        refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
],
```

## BuffTarget選択ガイド

武器パッシブのバフ対象に応じて適切なBuffTargetを選択:

- `BuffTarget::OnlySelf` — 装備者のみ（大半の武器パッシブ）
- `BuffTarget::Team` — チーム全員（例: Elegy for the End のATK+20%/EM+100）
- `BuffTarget::TeamExcludeSelf` — 装備者以外のチームメンバー（例: A Thousand Floating Dreams のEM+40）

## 既存buffsとの共存

武器によっては無条件`buffs`（StatBuff）が既に実装済みで、条件付き`conditional_buffs`のみ追加が必要なケースがある（例: MISTSPLITTER_REFORGED、ABSOLUTION）。

**ルール: 既存の`buffs`は一切変更しない。`conditional_buffs: &[]`のみを実装で置き換える。**

## バッチ構成

| バッチ | 対象 | 推定本数 | 備考 |
|--------|------|----------|------|
| ~~0~~ | ~~P2 2pc残り7セット~~ | 0 | 意図的空、スキップ |
| 1 | sword.rs (~1700行) | ~25本 | 35本中3星+DescOnly除外 |
| 2 | claymore.rs (~1200行) | ~23本 | 同上 |
| 3 | polearm.rs (~1100行) | ~25本 | 同上 |
| 4 | bow.rs (~1500行) | ~27本 | 同上 |
| 5 | catalyst.rs (~1700行) | ~23本 | 同上 |
| 6 | Nilou（別途ミニスペック） | 1 | Phase 2完了後に設計 |

注: 武器ファイルはデータ定義であり、コーディングスタイルの800行制限はロジックファイル向け。データファイルの行数増加は許容。

## テスト戦略

- Phase 1: 既存serde roundtripテストで自動カバー
- Phase 2: 武器種ごとにConditionalBuff存在確認のバッチテスト（全4-5星ダメージ武器が非空conditional_buffsを持つことを検証）+ serde roundtrip
- Phase 3: Nilou専用の統合テスト（チーム構成バリデーション含む）— ミニスペックで詳細化
- 全Phase: `cargo test` + `cargo clippy -- -D warnings` でCI通過確認

## 命名規約

- ConditionalBuff.name: `{weapon_id}_{buff_description}` (snake_case)
  - 例: `skyward_blade_cr`, `amos_bow_na_ca_dmg`, `freedom_sworn_team_atk`
- description: 日本語でゲーム内効果を簡潔に記述
- refinement_values: R1-R5の5要素配列、パーセンテージは小数形式（10.8% → 0.108）

## 成功基準

- P2: 45/52セットの2pc StatBuff実装維持（残り7セットは表現不可能な効果、意図的空）
- P3: ダメージ影響のある4-5星武器の全ConditionalBuff実装
- P1: Nilouミニスペック作成・レビュー完了
- 全テスト通過、clippy警告なし
