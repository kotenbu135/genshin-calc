# 3星武器 + SAPWOOD_BLADE ConditionalBuff実装 — 設計書

## 概要

既存の武器ConditionalBuff一括完了プラン（2026-03-29）で「3星 = 全スキップ」ルールにより対象外となった武器のうち、ダメージ計算に影響するパッシブを持つ武器にConditionalBuffを実装する。加えて、4星SAPWOOD_BLADEの草反応EMバフも実装する。

## スコープ

### 対象: 15本

| # | 武器 | 星 | 種 | パッシブ概要 | パターン |
|---|------|----|----|-------------|---------|
| 1 | HARBINGER_OF_DAWN | 3 | 剣 | HP90%以上でCR+14-28% | Toggle |
| 2 | COOL_STEEL | 3 | 剣 | 水/氷影響敵にDMG+12-24% | Toggle |
| 3 | DARK_IRON_SWORD | 3 | 剣 | 雷反応でATK+20-40% | Toggle |
| 4 | SKYRIDER_SWORD | 3 | 剣 | 爆発後NA DMG+12-24%, CA DMG+12-24% | Toggle (2エントリ) |
| 5 | SAPWOOD_BLADE | 4 | 剣 | 草反応でチームEM+60-120 | Toggle + Team |
| 6 | BLOODTAINTED_GREATSWORD | 3 | 両手剣 | 炎/雷影響敵にDMG+12-24% | Toggle |
| 7 | FERROUS_SHADOW | 3 | 両手剣 | HP70%以下でCA DMG+30-50% | Toggle |
| 8 | SKYRIDER_GREATSWORD | 3 | 両手剣 | NA/CA命中でATK+6-10%、4スタック | Stacks(4) |
| 9 | RAVEN_BOW | 3 | 弓 | 水/炎影響敵にDMG+12-24% | Toggle |
| 10 | SHARPSHOOTERS_OATH | 3 | 弓 | 弱点命中DMG+24-48% | Toggle |
| 11 | SLINGSHOT | 3 | 弓 | 近距離命中DMG+36-60% | Toggle |
| 12 | EMERALD_ORB | 3 | 法器 | 水反応でATK+20-40% | Toggle |
| 13 | TWIN_NEPHRITE | 3 | 法器 | 敵撃破ATK+12-24% | Toggle |
| 14 | MAGIC_GUIDE | 3 | 法器 | 水/雷影響敵にDMG+12-24% | Toggle |
| 15 | THRILLING_TALES_OF_DRAGON_SLAYERS | 3 | 法器 | チームATK+24-48% | Toggle + TeamExcludeSelf |

### 対象外（同星で除外）

- **proc系**: FILLET_BLADE, DEBATE_CLUB, HALBERD, MESSENGER — 追加ダメージインスタンス。ConditionalBuff（ステータスバフ）では表現不可
- **回復のみ**: TRAVELERS_HANDY_SWORD, RECURVE_BOW, OTHERWORLDLY_STORY, WHITE_IRON_GREATSWORD
- **BLACK_TASSEL**: スライム限定DMG+40-80% — 敵種別条件は汎用性が低い
- **長柄3星**: BLACK_TASSEL(上記), HALBERD(proc), WHITE_TASSEL(既存buffsにNA DMGあり)

## R1-R5値

3星武器の精錬パターン: 多くはR1値 × [1.0, 1.25, 1.5, 1.75, 2.0]。例外あり（FERROUS_SHADOW, SLINGSHOT, SKYRIDER_GREATSWORDは線形ステップ）。下表の実値を使用。

| 武器 | BuffableStat | R1 | R2 | R3 | R4 | R5 |
|------|-------------|----:|----:|----:|----:|----:|
| HARBINGER_OF_DAWN | CritRate | 0.14 | 0.175 | 0.21 | 0.245 | 0.28 |
| COOL_STEEL | DmgBonus | 0.12 | 0.15 | 0.18 | 0.21 | 0.24 |
| DARK_IRON_SWORD | AtkPercent | 0.20 | 0.25 | 0.30 | 0.35 | 0.40 |
| SKYRIDER_SWORD (NA) | NormalAtkDmgBonus | 0.12 | 0.15 | 0.18 | 0.21 | 0.24 |
| SKYRIDER_SWORD (CA) | ChargedAtkDmgBonus | 0.12 | 0.15 | 0.18 | 0.21 | 0.24 |
| SAPWOOD_BLADE | ElementalMastery | 60.0 | 75.0 | 90.0 | 105.0 | 120.0 |
| BLOODTAINTED_GREATSWORD | DmgBonus | 0.12 | 0.15 | 0.18 | 0.21 | 0.24 |
| FERROUS_SHADOW | ChargedAtkDmgBonus | 0.30 | 0.35 | 0.40 | 0.45 | 0.50 |
| SKYRIDER_GREATSWORD | AtkPercent | 0.06 | 0.07 | 0.08 | 0.09 | 0.10 |
| RAVEN_BOW | DmgBonus | 0.12 | 0.15 | 0.18 | 0.21 | 0.24 |
| SHARPSHOOTERS_OATH | DmgBonus | 0.24 | 0.30 | 0.36 | 0.42 | 0.48 |
| SLINGSHOT | DmgBonus | 0.36 | 0.42 | 0.48 | 0.54 | 0.60 |
| EMERALD_ORB | AtkPercent | 0.20 | 0.25 | 0.30 | 0.35 | 0.40 |
| TWIN_NEPHRITE | AtkPercent | 0.12 | 0.15 | 0.18 | 0.21 | 0.24 |
| MAGIC_GUIDE | DmgBonus | 0.12 | 0.15 | 0.18 | 0.21 | 0.24 |
| THRILLING_TALES | AtkPercent | 0.24 | 0.30 | 0.36 | 0.42 | 0.48 |

## 実装パターン

全武器が既存のActivationパターンで表現可能。新しい型やフィールドの追加は不要。

### Toggle（12本）

大半の武器。条件成立時にON/OFFで表現。

```rust
// HARBINGER_OF_DAWN例
conditional_buffs: &[ConditionalBuff {
    name: "harbinger_of_dawn_cr",
    description: "HP90%以上でCRIT Rate+14-28%",
    stat: BuffableStat::CritRate,
    value: 0.14,
    refinement_values: Some([0.14, 0.175, 0.21, 0.245, 0.28]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

### Toggle + 複数エントリ（1本）

SKYRIDER_SWORD: NA/CAは別エントリ（既存パターン準拠）。

```rust
conditional_buffs: &[
    ConditionalBuff {
        name: "skyrider_sword_na_dmg",
        description: "元素爆発後にNA DMG+12-24%",
        stat: BuffableStat::NormalAtkDmgBonus,
        value: 0.12,
        refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
    ConditionalBuff {
        name: "skyrider_sword_ca_dmg",
        description: "元素爆発後にCA DMG+12-24%",
        stat: BuffableStat::ChargedAtkDmgBonus,
        value: 0.12,
        refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
],
```

### Toggle + Team（1本）

SAPWOOD_BLADE: チーム全体にEMバフ。

```rust
conditional_buffs: &[ConditionalBuff {
    name: "sapwood_blade_team_em",
    description: "草元素反応時にチームEM+60-120",
    stat: BuffableStat::ElementalMastery,
    value: 60.0,
    refinement_values: Some([60.0, 75.0, 90.0, 105.0, 120.0]),
    stack_values: None,
    target: BuffTarget::Team,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

### Toggle + TeamExcludeSelf（1本）

THRILLING_TALES: 交代先のキャラにATKバフ（装備者除外）。

```rust
conditional_buffs: &[ConditionalBuff {
    name: "ttds_team_atk",
    description: "キャラ交代時に次のキャラATK+24-48%",
    stat: BuffableStat::AtkPercent,
    value: 0.24,
    refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
    stack_values: None,
    target: BuffTarget::TeamExcludeSelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

### Stacks（1本）

SKYRIDER_GREATSWORD: 線形スタック（value × n）。

```rust
conditional_buffs: &[ConditionalBuff {
    name: "skyrider_greatsword_atk",
    description: "NA/CA命中でATK+6-10%、最大4重",
    stat: BuffableStat::AtkPercent,
    value: 0.06,
    refinement_values: Some([0.06, 0.07, 0.08, 0.09, 0.10]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Stacks(4)),
}],
```

## passive_description更新

各武器のdescriptionフィールドから`"Conditional: "`プレフィックスを除去し、実効果を記述する。

## 既存プランとの関係

- メインプラン（2026-03-29）のTask 5 Step 4にTHRILLING_TALESとMAGIC_GUIDEが含まれている
- メインプランのTask 1でSAPWOOD_BLADEが「DescriptionOnly → スキップ」とされているが、EMは反応ダメージに影響するため本プランで override して実装する
- 本プランで先に実装するため、メインプラン実行時はこの3本（THRILLING_TALES, MAGIC_GUIDE, SAPWOOD_BLADE）をスキップする
- メインプランのスキップ基準「3星 = 全スキップ」は本プラン完了後に「3星 = ダメージ影響なしのみスキップ」に実質更新される

## テスト戦略

- 既存serde roundtripテストで自動カバー
- `cargo test -p genshin-calc-data` で全テスト通過
- `cargo clippy -- -D warnings` で警告なし

## 成功基準

- 15本の武器にConditionalBuff実装完了
- passive_descriptionから`Conditional:`プレフィックス除去
- 全テスト通過、clippy警告なし
