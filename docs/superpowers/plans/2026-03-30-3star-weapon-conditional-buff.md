# 3星武器 + SAPWOOD_BLADE ConditionalBuff 実装計画

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 既存プランで「3星=全スキップ」により対象外となった15本のダメージ影響武器にConditionalBuffを実装する

**Architecture:** 既存ConditionalBuffシステム（buff.rs）をそのまま活用。新しい型追加なし。武器種別に4ファイルを修正し、各武器の`conditional_buffs: &[]`を適切なConditionalBuff配列に置き換える。passive_descriptionの`Conditional:`プレフィックスも同時に除去。

**Tech Stack:** Rust, genshin-calc-data crate, ConditionalBuff/Activation/ManualCondition types

**Spec:** `docs/superpowers/specs/2026-03-30-3star-weapon-conditional-buff-design.md`

---

## ファイル構成

| ファイル | 操作 | 責務 |
|----------|------|------|
| `crates/data/src/weapons/sword.rs` | Modify | 片手剣5本のConditionalBuff追加 |
| `crates/data/src/weapons/claymore.rs` | Modify | 両手剣3本のConditionalBuff追加 |
| `crates/data/src/weapons/bow.rs` | Modify | 弓3本のConditionalBuff追加 |
| `crates/data/src/weapons/catalyst.rs` | Modify | 法器4本のConditionalBuff追加 |
| `crates/data/src/buff.rs` | Read only | ConditionalBuff型定義の参照 |

## 共通ルール

### 命名規約
- name: `{weapon_id}_{buff_stat}` (snake_case)
- description: 日本語でゲーム内効果を簡潔に記述（`Conditional:`プレフィックスなし）
- 数値: パーセンテージは小数形式（14% → 0.14）

### passive_description更新
各武器のdescriptionから`"Conditional: "`プレフィックスを除去し、R1-R5の範囲表記に更新する。

### 既存buffsとの共存
`buffs: &[StatBuff{..}]`が既にある武器はbuffsを一切変更しない。`conditional_buffs: &[]`のみ置き換える。

---

## Task 1: sword.rs — 片手剣5本

**Files:**
- Modify: `crates/data/src/weapons/sword.rs`
- Read: `crates/data/src/buff.rs` (型参照)

### 対象武器

| 武器 | パターン | target | バフ数 |
|------|---------|--------|--------|
| HARBINGER_OF_DAWN | Toggle | OnlySelf | 1 |
| COOL_STEEL | Toggle | OnlySelf | 1 |
| DARK_IRON_SWORD | Toggle | OnlySelf | 1 |
| SKYRIDER_SWORD | Toggle | OnlySelf | 2 (NA+CA) |
| SAPWOOD_BLADE | Toggle | Team | 1 |

### ステップ

- [ ] **Step 1: HARBINGER_OF_DAWN のConditionalBuff実装**

`crates/data/src/weapons/sword.rs` のHARBINGER_OF_DAWN定義を修正:

descriptionを更新:
```
description: "HP90%以上でCRIT Rate+14-28%",
```

conditional_buffsを置き換え:
```rust
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

- [ ] **Step 2: COOL_STEEL のConditionalBuff実装**

descriptionを更新:
```
description: "水/氷の影響を受けた敵にDMG+12-24%",
```

conditional_buffsを置き換え:
```rust
conditional_buffs: &[ConditionalBuff {
    name: "cool_steel_dmg",
    description: "水/氷の影響を受けた敵にDMG+12-24%",
    stat: BuffableStat::DmgBonus,
    value: 0.12,
    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

- [ ] **Step 3: DARK_IRON_SWORD のConditionalBuff実装**

descriptionを更新:
```
description: "雷元素反応時にATK+20-40%、12秒",
```

conditional_buffsを置き換え:
```rust
conditional_buffs: &[ConditionalBuff {
    name: "dark_iron_sword_atk",
    description: "雷元素反応時にATK+20-40%",
    stat: BuffableStat::AtkPercent,
    value: 0.20,
    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

- [ ] **Step 4: SKYRIDER_SWORD のConditionalBuff実装（2エントリ）**

descriptionを更新:
```
description: "元素爆発後にNA/CA DMG+12-24%、15秒",
```

conditional_buffsを置き換え（NA/CAは別エントリ）:
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

- [ ] **Step 5: SAPWOOD_BLADE のConditionalBuff実装**

descriptionを更新:
```
description: "草元素反応時にチームEM+60-120",
```

conditional_buffsを置き換え:
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

- [ ] **Step 6: cargo build コンパイル確認**

Run: `cargo build -p genshin-calc-data`
Expected: コンパイル成功

- [ ] **Step 7: cargo test 実行**

Run: `cargo test -p genshin-calc-data`
Expected: 全テスト通過（serde roundtrip含む）

- [ ] **Step 8: cargo clippy 実行**

Run: `cargo clippy -- -D warnings`
Expected: 警告なし

- [ ] **Step 9: コミット**

```bash
git add crates/data/src/weapons/sword.rs
git commit -m "feat(data): add ConditionalBuff for 3-star swords + Sapwood Blade (5 weapons)"
```

---

## Task 2: claymore.rs — 両手剣3本

**Files:**
- Modify: `crates/data/src/weapons/claymore.rs`

### 対象武器

| 武器 | パターン | target | バフ数 |
|------|---------|--------|--------|
| BLOODTAINTED_GREATSWORD | Toggle | OnlySelf | 1 |
| FERROUS_SHADOW | Toggle | OnlySelf | 1 |
| SKYRIDER_GREATSWORD | Stacks(4) | OnlySelf | 1 |

### ステップ

- [ ] **Step 1: BLOODTAINTED_GREATSWORD のConditionalBuff実装**

descriptionを更新:
```
description: "炎/雷元素影響下の敵へのDMG+12-24%",
```

conditional_buffsを置き換え:
```rust
conditional_buffs: &[ConditionalBuff {
    name: "bloodtainted_greatsword_dmg",
    description: "炎/雷元素影響下の敵へのDMG+12-24%",
    stat: BuffableStat::DmgBonus,
    value: 0.12,
    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

- [ ] **Step 2: FERROUS_SHADOW のConditionalBuff実装**

descriptionを更新:
```
description: "HP70%以下で重撃DMG+30-50%",
```

conditional_buffsを置き換え:
```rust
conditional_buffs: &[ConditionalBuff {
    name: "ferrous_shadow_ca_dmg",
    description: "HP70%以下で重撃DMG+30-50%",
    stat: BuffableStat::ChargedAtkDmgBonus,
    value: 0.30,
    refinement_values: Some([0.30, 0.35, 0.40, 0.45, 0.50]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

- [ ] **Step 3: SKYRIDER_GREATSWORD のConditionalBuff実装**

descriptionを更新:
```
description: "通常/重撃命中でATK+6-10%、最大4重",
```

conditional_buffsを置き換え:
```rust
conditional_buffs: &[ConditionalBuff {
    name: "skyrider_greatsword_atk",
    description: "通常/重撃命中でATK+6-10%、最大4重",
    stat: BuffableStat::AtkPercent,
    value: 0.06,
    refinement_values: Some([0.06, 0.07, 0.08, 0.09, 0.10]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Stacks(4)),
}],
```

- [ ] **Step 4: cargo build コンパイル確認**

Run: `cargo build -p genshin-calc-data`
Expected: コンパイル成功

- [ ] **Step 5: cargo test 実行**

Run: `cargo test -p genshin-calc-data`
Expected: 全テスト通過

- [ ] **Step 6: cargo clippy 実行**

Run: `cargo clippy -- -D warnings`
Expected: 警告なし

- [ ] **Step 7: コミット**

```bash
git add crates/data/src/weapons/claymore.rs
git commit -m "feat(data): add ConditionalBuff for 3-star claymores (3 weapons)"
```

---

## Task 3: bow.rs — 弓3本

**Files:**
- Modify: `crates/data/src/weapons/bow.rs`

### 対象武器

| 武器 | パターン | target | バフ数 |
|------|---------|--------|--------|
| RAVEN_BOW | Toggle | OnlySelf | 1 |
| SHARPSHOOTERS_OATH | Toggle | OnlySelf | 1 |
| SLINGSHOT | Toggle | OnlySelf | 1 |

### ステップ

- [ ] **Step 1: RAVEN_BOW のConditionalBuff実装**

descriptionを更新:
```
description: "水/炎影響を受けた敵へのDMG+12-24%",
```

conditional_buffsを置き換え:
```rust
conditional_buffs: &[ConditionalBuff {
    name: "raven_bow_dmg",
    description: "水/炎影響を受けた敵へのDMG+12-24%",
    stat: BuffableStat::DmgBonus,
    value: 0.12,
    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

- [ ] **Step 2: SHARPSHOOTERS_OATH のConditionalBuff実装**

descriptionを更新:
```
description: "弱点命中時にDMG+24-48%",
```

conditional_buffsを置き換え:
```rust
conditional_buffs: &[ConditionalBuff {
    name: "sharpshooters_oath_dmg",
    description: "弱点命中時にDMG+24-48%",
    stat: BuffableStat::DmgBonus,
    value: 0.24,
    refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

- [ ] **Step 3: SLINGSHOT のConditionalBuff実装**

descriptionを更新:
```
description: "通常攻撃の矢が0.3秒以内に命中でDMG+36-60%",
```

conditional_buffsを置き換え:
```rust
conditional_buffs: &[ConditionalBuff {
    name: "slingshot_dmg",
    description: "通常攻撃の矢が0.3秒以内に命中でDMG+36-60%",
    stat: BuffableStat::DmgBonus,
    value: 0.36,
    refinement_values: Some([0.36, 0.42, 0.48, 0.54, 0.60]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

- [ ] **Step 4: cargo build コンパイル確認**

Run: `cargo build -p genshin-calc-data`
Expected: コンパイル成功

- [ ] **Step 5: cargo test 実行**

Run: `cargo test -p genshin-calc-data`
Expected: 全テスト通過

- [ ] **Step 6: cargo clippy 実行**

Run: `cargo clippy -- -D warnings`
Expected: 警告なし

- [ ] **Step 7: コミット**

```bash
git add crates/data/src/weapons/bow.rs
git commit -m "feat(data): add ConditionalBuff for 3-star bows (3 weapons)"
```

---

## Task 4: catalyst.rs — 法器4本

**Files:**
- Modify: `crates/data/src/weapons/catalyst.rs`

### 対象武器

| 武器 | パターン | target | バフ数 |
|------|---------|--------|--------|
| EMERALD_ORB | Toggle | OnlySelf | 1 |
| TWIN_NEPHRITE | Toggle | OnlySelf | 1 |
| MAGIC_GUIDE | Toggle | OnlySelf | 1 |
| THRILLING_TALES_OF_DRAGON_SLAYERS | Toggle | TeamExcludeSelf | 1 |

### ステップ

- [ ] **Step 1: EMERALD_ORB のConditionalBuff実装**

descriptionを更新:
```
description: "水元素反応時にATK+20-40%",
```

conditional_buffsを置き換え:
```rust
conditional_buffs: &[ConditionalBuff {
    name: "emerald_orb_atk",
    description: "水元素反応時にATK+20-40%",
    stat: BuffableStat::AtkPercent,
    value: 0.20,
    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

- [ ] **Step 2: TWIN_NEPHRITE のConditionalBuff実装**

descriptionを更新:
```
description: "敵撃破時にATK+12-24%",
```

conditional_buffsを置き換え:
```rust
conditional_buffs: &[ConditionalBuff {
    name: "twin_nephrite_atk",
    description: "敵撃破時にATK+12-24%",
    stat: BuffableStat::AtkPercent,
    value: 0.12,
    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

- [ ] **Step 3: MAGIC_GUIDE のConditionalBuff実装**

descriptionを更新:
```
description: "水/雷の影響を受けた敵にDMG+12-24%",
```

conditional_buffsを置き換え:
```rust
conditional_buffs: &[ConditionalBuff {
    name: "magic_guide_dmg",
    description: "水/雷の影響を受けた敵にDMG+12-24%",
    stat: BuffableStat::DmgBonus,
    value: 0.12,
    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}],
```

- [ ] **Step 4: THRILLING_TALES_OF_DRAGON_SLAYERS のConditionalBuff実装**

descriptionを更新:
```
description: "キャラ交代時に次のキャラATK+24-48%",
```

conditional_buffsを置き換え:
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

- [ ] **Step 5: cargo build コンパイル確認**

Run: `cargo build -p genshin-calc-data`
Expected: コンパイル成功

- [ ] **Step 6: cargo test 実行**

Run: `cargo test -p genshin-calc-data`
Expected: 全テスト通過

- [ ] **Step 7: cargo clippy 実行**

Run: `cargo clippy -- -D warnings`
Expected: 警告なし

- [ ] **Step 8: コミット**

```bash
git add crates/data/src/weapons/catalyst.rs
git commit -m "feat(data): add ConditionalBuff for 3-star catalysts + TTDS (4 weapons)"
```

---

## Task 5: data-expansion-todo更新 + 最終検証 + クリーンアップ

**Files:**
- Modify: `docs/data-expansion-todo.md`
- Delete: `docs/superpowers/specs/2026-03-30-3star-weapon-conditional-buff-design.md`
- Delete: `docs/superpowers/plans/2026-03-30-3star-weapon-conditional-buff.md`

### ステップ

- [ ] **Step 1: 全テスト実行**

Run: `cargo test`
Expected: 全テスト通過

- [ ] **Step 2: clippy実行**

Run: `cargo clippy -- -D warnings`
Expected: 警告なし

- [ ] **Step 3: data-expansion-todo.md更新**

「武器パッシブ (ConditionalBuff)」のカウントを全箇所更新（77→92）。`grep -n '77' docs/data-expansion-todo.md`で対象行を特定:
- 現状サマリ表の実装済み列
- ボトルネックセクションの本文
- P3セクションの進捗表記
- 優先度マトリクスの表記

P3セクションに3星武器15本の完了記録を追加。

- [ ] **Step 4: コミット**

```bash
git add docs/data-expansion-todo.md
git commit -m "docs: update data-expansion-todo with 3-star weapon completion"
```

- [ ] **Step 5: spec/planファイル削除**

CLAUDE.md指示: 実装完了後に設計書・計画書を削除。

```bash
rm docs/superpowers/specs/2026-03-30-3star-weapon-conditional-buff-design.md
rm docs/superpowers/plans/2026-03-30-3star-weapon-conditional-buff.md
git add -A docs/superpowers/
git commit -m "chore: remove completed 3-star weapon spec and plan"
```
