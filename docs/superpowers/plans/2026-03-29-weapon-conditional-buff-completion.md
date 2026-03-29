# 武器ConditionalBuff一括完了 実装計画

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** ダメージ影響のある4-5星武器の全ConditionalBuff実装を完了する（~130本）

**Architecture:** 既存のConditionalBuffシステム（buff.rs）を活用し、各武器のpassive_descriptionからActivationパターン（Toggle/Stacks/StatScaling/Both/TeamComp）を選択して実装する。武器種別に5バッチで作業し、各バッチは独立してテスト・コミット可能。

**Tech Stack:** Rust, genshin-calc-data crate, ConditionalBuff/Activation/ManualCondition/AutoCondition types

**Spec:** `docs/superpowers/specs/2026-03-29-data-expansion-completion-design.md`

---

## ファイル構成

| ファイル | 操作 | 責務 |
|----------|------|------|
| `crates/data/src/weapons/sword.rs` | Modify | 片手剣のConditionalBuff追加 |
| `crates/data/src/weapons/claymore.rs` | Modify | 両手剣のConditionalBuff追加 |
| `crates/data/src/weapons/polearm.rs` | Modify | 長柄武器のConditionalBuff追加 |
| `crates/data/src/weapons/bow.rs` | Modify | 弓のConditionalBuff追加 |
| `crates/data/src/weapons/catalyst.rs` | Modify | 法器のConditionalBuff追加 |
| `crates/data/src/lib.rs` | Read only | find_weapon等の検索API確認 |
| `crates/data/src/buff.rs` | Read only | ConditionalBuff型定義の参照 |
| `docs/data-expansion-todo.md` | Modify | カバレッジ更新 |

## 共通ルール（全タスク共通）

### スキップ基準
以下の武器は`conditional_buffs: &[]`のまま据え置く:
- **1-3星武器**: ダメージ影響微小
- **DescriptionOnly**: HP回復、シールド、エネルギー生成、CD短縮、移動速度のみ
- **例外**: 上記に加えてダメージバフがある場合はダメージ部分のみ実装

### プレースホルダー武器の扱い

passive_descriptionが`"Conditional: 条件付きバフ効果"`等の汎用テキストの武器は、実際のパッシブ効果が未入力。対処:
1. Genshin Wikiで実際のパッシブを確認
2. ダメージ関連の効果があればpassive_descriptionを正しい内容に更新し、ConditionalBuffを実装
3. 確認できない場合は`conditional_buffs: &[]`のまま据え置き

### パターン選択ガイド

passive_descriptionの文言からActivationパターンを判定:

| キーワード | パターン | 例 |
|-----------|---------|-----|
| 「〜時」「〜後」「〜中」 | `Manual(Toggle)` | 「元素スキル使用後ATK+20%」 |
| 「最大N重」「N層」 | `Manual(Stacks(N))` | 「命中時DMG+6%、最大5重」 |
| 「〜の○%」 | `Auto(StatScaling{..})` | 「HP上限の1.2%分ATKアップ」 |
| 「チーム」「味方」 | `BuffTarget::Team` | 「チーム全員のATK+20%」 |
| 非線形スタック | `stack_values: Some(&[..])` | 1/2/3重で8%/16%/28% |

### R1-R5値の導出

passive_descriptionに「ATK+20-40%」のような範囲表記がある場合:
- R1=20%, R5=40% → step = (40-20)/(5-1) = 5%
- R1=0.20, R2=0.25, R3=0.30, R4=0.35, R5=0.40

passive_descriptionにR1値しかない場合:
- Genshin Wiki (https://genshin-impact.fandom.com/) でR1-R5を確認
- 一般的パターン: R1×1.0, R1×1.25, R1×1.5, R1×1.75, R1×2.0

### 既存buffsとの共存

`buffs: &[StatBuff{..}]`が既に入っている武器は**buffsを一切変更しない**。`conditional_buffs: &[]`のみを置き換える。

### 命名規約

- name: `{weapon_id}_{buff_stat}` (snake_case) — 例: `skyward_blade_cr`, `freedom_sworn_team_atk`
- description: 日本語でゲーム内効果を簡潔に記述
- 数値単位: パーセンテージは小数形式（10.8% → 0.108）

---

## Task 1: sword.rs — 片手剣ConditionalBuff

**Files:**
- Modify: `crates/data/src/weapons/sword.rs`
- Read: `crates/data/src/buff.rs` (型参照)

### 対象武器分類

**5星 — 実装対象 (5本):**

| 武器 | パターン | バフ対象 | 複数バフ | 概要 |
|------|---------|---------|---------|------|
| ABSOLUTION | Stacks | OnlySelf | No | 罪禍スタックでCRIT DMG追加 |
| FREEDOM_SWORN | Toggle | Team | Yes | チームATK+20% + NA/CA/Plunge DMG各+16%（4エントリ） |
| HARAN_GEPPAKU_FUTSU | Toggle | OnlySelf | No | 味方スキル使用でNA DMG+20-40% |
| SKYWARD_BLADE | Toggle | OnlySelf | Yes | 爆発後NA DMG + CA DMG（CRは既存buffsに実装済み） |
| SPLENDOR_OF_TRANQUIL_WATERS | Toggle | OnlySelf | Yes | HP変動でSkill DMG+, NA命中でHP変動DMG+ |

**5星 — スキップ (1本):** AQUILA_FAVONIA（ATK+20-40%は既存StatBuff。追加効果はproc系のみ）

**4星 — 実装対象 (~14本):**

| 武器 | パターン | 概要 |
|------|---------|------|
| BLACKCLIFF_LONGSWORD | Stacks(3) | 敵撃破でATK+12-24%、最大3重 |
| CALAMITY_OF_ESHU | Toggle | 反応後Skill DMG + CR |
| FESTERING_DESIRE | Toggle | Skill CR+6-12%（既存buffsにSkillDMGあり） |
| FINALE_OF_THE_DEEP | Toggle | HP変動でNA/CA/Plunge DMG+12-24% |
| FLEUVE_CENDRE_FERRYMAN | Toggle | スキル後Skill CR+8-16% + ER+16-32% |
| KAGOTSURUBE_ISSHIN | Toggle | NA/CA/Plunge命中でATK+15% |
| MOONWEAVERS_DAWN | Toggle | スキル/爆発命中でDMG+ |
| PROTOTYPE_RANCOUR | Stacks(4) | NA/CA命中でATK+4-8%/DEF+4-8% |
| THE_ALLEY_FLASH | — | DMG+12-24%は既存StatBuff。被ダメ解除は状態変化→現状維持 |
| THE_DOCKHANDS_ASSISTANT | Stacks(3) | 反応後EM+40-80、最大3重 |
| THE_FLUTE | DescriptionOnly | 和音によるATK100%ダメ→proc系、スキップ検討 |
| XIPHOS_MOONLIGHT | StatScaling | EM×0.036-0.072%のER（自身+チーム30%） |
| THE_BLACK_SWORD | — | 既存buffsにNA/CA DMGあり、追加は回復のみ→スキップ |
| SAPWOOD_BLADE | DescriptionOnly | 草反応でEM付与→スキップ |

**4星 — スキップ (5本):** AMENOMA_KAGEUCHI, FAVONIUS_SWORD, SACRIFICIAL_SWORD, SWORD_OF_DESCENSION, SWORD_OF_NARZISSENKREUZ

**3星 — 全スキップ (6本):** COOL_STEEL, DARK_IRON_SWORD, FILLET_BLADE, HARBINGER_OF_DAWN, SKYRIDER_SWORD, TRAVELERS_HANDY_SWORD

### ステップ

- [ ] **Step 1: 対象武器のpassive_descriptionを読み、R1-R5値を確認**

各武器のpassive_descriptionフィールドからR1値を読み取り、R1-R5の5値を計算する。Wikiで正確な値を確認。

- [ ] **Step 2: 5星武器6本のConditionalBuff実装**

各武器の`conditional_buffs: &[]`を適切なConditionalBuff配列に置き換える。既存パターン（ATHAME_ARTIS, LIGHTBEARING_MOONSHARD等）を参考に。

FREEDOM_SWORN例（4バフ — NA/CA/Plungeは別エントリ）:
```rust
conditional_buffs: &[
    ConditionalBuff {
        name: "freedom_sworn_team_atk",
        description: "千年の大楽章:抗争の歌発動時、チームATK+20%",
        stat: BuffableStat::AtkPercent,
        value: 0.20,
        refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
        stack_values: None,
        target: BuffTarget::Team,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
    ConditionalBuff {
        name: "freedom_sworn_team_na_dmg",
        description: "千年の大楽章:抗争の歌発動時、通常攻撃DMG+16%",
        stat: BuffableStat::NormalAtkDmgBonus,
        value: 0.16,
        refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
        stack_values: None,
        target: BuffTarget::Team,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
    ConditionalBuff {
        name: "freedom_sworn_team_ca_dmg",
        description: "千年の大楽章:抗争の歌発動時、重撃DMG+16%",
        stat: BuffableStat::ChargedAtkDmgBonus,
        value: 0.16,
        refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
        stack_values: None,
        target: BuffTarget::Team,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
    ConditionalBuff {
        name: "freedom_sworn_team_plunge_dmg",
        description: "千年の大楽章:抗争の歌発動時、落下攻撃DMG+16%",
        stat: BuffableStat::PlungingAtkDmgBonus,
        value: 0.16,
        refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
        stack_values: None,
        target: BuffTarget::Team,
        activation: Activation::Manual(ManualCondition::Toggle),
    },
],
```

**重要: NA/CA/Plungeは常に別々のConditionalBuffエントリにする。** 既存実装（ATHAME_ARTIS等）と同じパターン。`BuffableStat::NormalAtkDmgBonus`, `ChargedAtkDmgBonus`, `PlungingAtkDmgBonus`をそれぞれ使用。

- [ ] **Step 3: 4星武器~14本のConditionalBuff実装**

Toggle/Stacks/StatScaling各パターンに分けて実装。

- [ ] **Step 4: cargo build でコンパイル確認**

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
git add crates/data/src/weapons/sword.rs
git commit -m "feat(data): add ConditionalBuff for sword weapons (~20 weapons)"
```

---

## Task 2: claymore.rs — 両手剣ConditionalBuff

**Files:**
- Modify: `crates/data/src/weapons/claymore.rs`

### 対象武器分類

**5星 — 実装対象 (4本):**

| 武器 | パターン | 複数バフ | 概要 |
|------|---------|---------|------|
| A_THOUSAND_BLAZING_SUNS | — | No | 既存buffsにCritDmg+ATKあり。追加条件付き効果を確認 |
| SONG_OF_BROKEN_PINES | Toggle/Team | Yes | 4マーク達成時チームATK速度+12%+ATK+20% |
| SKYWARD_PRIDE | — | No | 既存buffsにDMG+8-16%あり。追加は真空刃→proc系スキップ検討 |
| VERDICT | Stacks | No | シールドで追加Skill DMG+18-36% |

**4星 — 実装対象 (~15本):**

| 武器 | パターン | 概要 |
|------|---------|------|
| AKUOUMARU | StatScaling | チームER合計でBurst DMG+ |
| BLACKCLIFF_SLASHER | Stacks(3) | 敵撃破でATK+12-24% |
| FOREST_REGALIA | Toggle | 草反応でEM+60-120 |
| MAILED_FLOWER | Toggle | スキル命中でATK+12-24% + EM+48-96 |
| MAKHAIRA_AQUAMARINE | StatScaling | EM×0.24-0.48のATK（チーム30%） |
| PORTABLE_POWER_SAW | Stacks | スタックでEM+ |
| PROTOTYPE_ARCHAIC | DescriptionOnly | proc系追加ダメ→スキップ |
| RAINSLASHER | Toggle | 水/雷影響敵にDMG+20-36% |
| ROYAL_GREATSWORD | Stacks(5) | CR+8-16%、最大5重 |
| SNOW_TOMBED_STARSILVER | DescriptionOnly | proc系→スキップ |
| TALKING_STICK | Toggle | 反応でElem DMG+16-32% or ATK+16-32% |
| THE_BELL | Toggle | シールド中DMG+12-24% |
| TIDAL_SHADOW | Toggle | 回復時ATK+24-48% |
| ULTIMATE_OVERLORDS_MEGA_MAGIC_SWORD | TeamComp | チーム元素種類でATK/Elem DMG+ |

**4星 — スキップ:** FAVONIUS_GREATSWORD, SACRIFICIAL_GREATSWORD, KATSURAGIKIRI_NAGAMASA(既存buffsにSkillDMGあり+エネルギーのみ)
**3星 — 全スキップ (5本)**

### ステップ

- [ ] **Step 1: 対象武器のpassive_descriptionを読み、R1-R5値を確認**
- [ ] **Step 2: 5星武器のConditionalBuff実装**
- [ ] **Step 3: 4星武器のConditionalBuff実装**
- [ ] **Step 4: cargo build コンパイル確認**

Run: `cargo build -p genshin-calc-data`

- [ ] **Step 5: cargo test 実行**

Run: `cargo test -p genshin-calc-data`

- [ ] **Step 6: cargo clippy 実行**

Run: `cargo clippy -- -D warnings`

- [ ] **Step 7: コミット**

```bash
git add crates/data/src/weapons/claymore.rs
git commit -m "feat(data): add ConditionalBuff for claymore weapons (~19 weapons)"
```

---

## Task 3: polearm.rs — 長柄武器ConditionalBuff

**Files:**
- Modify: `crates/data/src/weapons/polearm.rs`

### 対象武器分類

**5星 — 実装対象 (~4本):**

| 武器 | パターン | 複数バフ | 概要 |
|------|---------|---------|------|
| CALAMITY_QUELLER | Stacks | Yes | スキル使用後ATK+3.2-6.4%最大6重、出場外2倍 |
| LUMIDOUCE_ELEGY | Toggle | Yes | 反応でATK+, HP+（既存buffsと確認） |
| SKYWARD_SPINE | Toggle | No | CR+8-16%あり、追加はNA速度+proc→proc部分スキップ |
| CRIMSON_MOONS_SEMBLANCE | Toggle | No | パッシブ確認要 |

**4星 — 実装対象 (~10本):**

| 武器 | パターン | 概要 |
|------|---------|------|
| BALLAD_OF_THE_FJORDS | TeamComp | チーム元素3種でEM+120 |
| CRESCENT_PIKE | DescriptionOnly | 粒子取得で追加ダメ→proc系スキップ |
| MISSIVE_WINDSPEAR | Toggle | 反応後ATK+12-24% + EM+48-96 |
| PROTOTYPE_STARGLITTER | Stacks(2) | スキル使用後NA/CA DMG+8-16% |
| ROYAL_SPEAR | Stacks(5) | CR+8-16%、最大5重 |
| WAVEBREAKERS_FIN | StatScaling | チームER合計でBurst DMG+ |
| KITAIN_CROSS_SPEAR | Toggle | Skill DMG+6-12%（buffsに既存なら確認） |
| THE_CATCH | — | 既存buffsにBurst DMG+CR→確認 |
| DRAGONSPINE_SPEAR | DescriptionOnly | proc系→スキップ |

**4星 — スキップ:** FAVONIUS_LANCE, RIGHTFUL_REWARD, その他プレースホルダー武器
**3星 — 全スキップ:** BLACK_TASSEL, HALBERD, WHITE_TASSEL

### ステップ

- [ ] **Step 1: 対象武器のpassive_descriptionを読み、R1-R5値を確認**

polearm.rsには汎用プレースホルダー（`"Conditional: 条件付きバフ効果"`）の武器が多数存在する可能性あり（BLOODSOAKED_RUINS, CRIMSON_MOONS_SEMBLANCE, FRACTURED_HALO, SYMPHONIST_OF_SCENTS, DIALOGUES_OF_THE_DESERT_SAGES, FOOTPRINT_OF_THE_RAINBOW, MOUNTAIN_BRACING_BOLT, PROSPECTORS_DRILL, PROSPECTORS_SHOVEL, SACRIFICERS_STAFF, TAMAYURATEI_NO_OHANASHI等）。共通ルール「プレースホルダー武器の扱い」に従いWiki確認→実装 or 据え置き。

- [ ] **Step 2: 5星武器のConditionalBuff実装**
- [ ] **Step 3: 4星武器のConditionalBuff実装**
- [ ] **Step 4: cargo build コンパイル確認**

Run: `cargo build -p genshin-calc-data`

- [ ] **Step 5: cargo test 実行**

Run: `cargo test -p genshin-calc-data`

- [ ] **Step 6: cargo clippy 実行**

Run: `cargo clippy -- -D warnings`

- [ ] **Step 7: コミット**

```bash
git add crates/data/src/weapons/polearm.rs
git commit -m "feat(data): add ConditionalBuff for polearm weapons (~14 weapons)"
```

---

## Task 4: bow.rs — 弓ConditionalBuff

**Files:**
- Modify: `crates/data/src/weapons/bow.rs`

### 対象武器分類

**5星 — 実装対象 (4本):**

| 武器 | パターン | 複数バフ | 概要 |
|------|---------|---------|------|
| AMOS_BOW | Stacks(5) | No | 矢の飛行時間でNA/CA DMG+8-16%、最大5重 |
| POLAR_STAR | Stacks(4) | No | 白極スタックでATK+10-20%/20-40%/30-60%/48-96%（非線形） |
| SKYWARD_HARP | Toggle | No | 命中で追加ダメ（proc系→スキップ検討。CR部分は既存buffsか確認） |
| THUNDERING_PULSE | Stacks(3) | No | 雷の巴紋スタックでNA DMG+12-24%/24-48%/40-80%（非線形） |

**4星 — 実装対象 (~16本):**

| 武器 | パターン | 概要 |
|------|---------|------|
| ALLEY_HUNTER | Stacks(10) | 出場外1秒ごとにDMG+2-4%、最大10重 |
| BLACKCLIFF_WARBOW | Stacks(3) | 敵撃破でATK+12-24% |
| CHAIN_BREAKER | TeamComp | チーム元素種でEM/ATK+ |
| CLOUDFORGED | Toggle/Team | エネルギー消費でチームEM+40-80 |
| COMPOUND_BOW | Stacks(4) | NA/CA命中でATK+4-8%/攻撃速度+ |
| FLOWER_WREATHED_FEATHERS | Toggle | 反応後DMG+ |
| IBIS_PIERCER | Stacks(2) | CA命中でEM+20-40 |
| KINGS_SQUIRE | Toggle | スキル/爆発使用後EM+60-120 |
| MOUUNS_MOON | StatScaling | チームER合計でBurst DMG+ |
| RANGE_GAUGE | Stacks | CA命中でATK+ |
| ROYAL_BOW | Stacks(5) | CR+8-16%、最大5重 |
| RUST | — | 既存buffsにNA DMG+40-80%あり、CA-10%はペナルティ→スキップ |
| SCION_OF_THE_BLAZING_SUN | Toggle | CA DMG+ |
| SONG_OF_STILLNESS | Toggle | 回復後DMG+ |
| RAINBOW_SERPENTS_RAIN_BOW | TeamComp | チーム元素でDMG+ |

**4星 — スキップ:** FAVONIUS_WARBOW, SACRIFICIAL_BOW, THE_VIRIDESCENT_HUNT, THE_STRINGLESS(既存buffsにSkill/Burst DMGあり)
**3星 — 全スキップ (5本)**

### ステップ

- [ ] **Step 1: 対象武器のpassive_descriptionを読み、R1-R5値を確認**

POLAR_STARとTHUNDERING_PULSEは非線形Stacks → stack_values必須。Wiki確認。

- [ ] **Step 2: 5星武器のConditionalBuff実装**
- [ ] **Step 3: 4星武器のConditionalBuff実装**
- [ ] **Step 4: cargo build コンパイル確認**

Run: `cargo build -p genshin-calc-data`

- [ ] **Step 5: cargo test 実行**

Run: `cargo test -p genshin-calc-data`

- [ ] **Step 6: cargo clippy 実行**

Run: `cargo clippy -- -D warnings`

- [ ] **Step 7: コミット**

```bash
git add crates/data/src/weapons/bow.rs
git commit -m "feat(data): add ConditionalBuff for bow weapons (~20 weapons)"
```

---

## Task 5: catalyst.rs — 法器ConditionalBuff

**Files:**
- Modify: `crates/data/src/weapons/catalyst.rs`

### 対象武器分類

**5星 — 実装対象 (~4本):**

| 武器 | パターン | 複数バフ | 概要 |
|------|---------|---------|------|
| CASHFLOW_SUPERVISION | Toggle | No | NA/CA DMG+（既存buffsと共存確認） |
| CRANES_ECHOING_CALL | Toggle | No | Plunge DMG+ |
| SKYWARD_ATLAS | Toggle | No | Elem DMG+（既存buffsと共存確認、proc部分スキップ） |
| SURFS_UP | Stacks | No | NA DMG+スタック |
| TOME_OF_THE_ETERNAL_FLOW | Stacks | No | CA DMG+スタック |

**4星 — 実装対象 (~13本):**

| 武器 | パターン | 概要 |
|------|---------|------|
| BALLAD_OF_THE_BOUNDLESS_BLUE | — | 既存buffsにNA/CA DMGあり→確認 |
| BLACKCLIFF_AGATE | Stacks(3) | 敵撃破でATK+12-24% |
| ASH_GRAVEN_DRINKING_HORN | StatScaling | HP依存DMG+ |
| FRUIT_OF_FULFILLMENT | Stacks | 反応でEM+24-40、ATK-5% |
| HAKUSHIN_RING | Toggle/Team | 反応でチームElem DMG+10-20% |
| RING_OF_YAXCHE | StatScaling | HP依存NA DMG+ |
| ROYAL_GRIMOIRE | Stacks(5) | CR+8-16%、最大5重 |
| WANDERING_EVENSTAR | StatScaling/Team | EM×0.24-0.48のATK（チーム30%） |
| WAVERIDING_WHIRL | Stacks | 反応でEM+ |
| FROSTBEARER | DescriptionOnly | proc系→スキップ |
| EYE_OF_PERCEPTION | DescriptionOnly | proc系→スキップ |
| THRILLING_TALES_OF_DRAGON_SLAYERS | Toggle/Team | キャラ交代でチームATK+24-48%（3星だが人気高→実装検討） |
| MAGIC_GUIDE | Toggle | 水/雷影響敵にDMG+12-24%（3星だが頻用→実装検討） |

**4星 — スキップ:** FAVONIUS_CODEX, SACRIFICIAL_FRAGMENTS, PROTOTYPE_AMBER, OATHSWORN_EYE, EVERLASTING_MOONGLOW
**3星 — 原則スキップ:** EMERALD_ORB, OTHERWORLDLY_STORY, TWIN_NEPHRITE
**3星例外:** THRILLING_TALES_OF_DRAGON_SLAYERS（メタ頻出武器、チームATKバフ。実装推奨）

### ステップ

- [ ] **Step 1: 対象武器のpassive_descriptionを読み、R1-R5値を確認**
- [ ] **Step 2: 5星武器のConditionalBuff実装**
- [ ] **Step 3: 4星武器のConditionalBuff実装**
- [ ] **Step 4: THRILLING_TALES (3星例外) の実装**

メタ頻出の龍殺しの英傑譚はToggle+Team ATK+24-48%として実装:
```rust
conditional_buffs: &[ConditionalBuff {
    name: "ttds_team_atk",
    description: "キャラ交代時、次の出場キャラATK+24-48%",
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

- [ ] **Step 6: cargo test 実行**

Run: `cargo test -p genshin-calc-data`

- [ ] **Step 7: cargo clippy 実行**

Run: `cargo clippy -- -D warnings`

- [ ] **Step 8: コミット**

```bash
git add crates/data/src/weapons/catalyst.rs
git commit -m "feat(data): add ConditionalBuff for catalyst weapons (~18 weapons)"
```

---

## Task 6: data-expansion-todo.md更新 + 最終検証

**Files:**
- Modify: `docs/data-expansion-todo.md`

### ステップ

- [ ] **Step 1: 全テスト実行**

Run: `cargo test`
Expected: 全テスト通過

- [ ] **Step 2: clippy実行**

Run: `cargo clippy -- -D warnings`
Expected: 警告なし

- [ ] **Step 3: 実装済みConditionalBuff数を集計**

```bash
grep -cP 'conditional_buffs: &\[(?![\],])' crates/data/src/weapons/*.rs
```

- [ ] **Step 4: data-expansion-todo.mdのP3セクション更新**

カバレッジ数値を更新、完了済みバッチにチェックマーク追加。

- [ ] **Step 5: コミット**

```bash
git add docs/data-expansion-todo.md
git commit -m "docs: update data-expansion-todo with P3 weapon completion status"
```

- [ ] **Step 6: docs/superpowers/specs/ と docs/superpowers/plans/ の該当ファイル削除**

CLAUDE.md指示: 実装完了後に設計書・計画書を削除。

```bash
rm docs/superpowers/specs/2026-03-29-data-expansion-completion-design.md
rm docs/superpowers/plans/2026-03-29-weapon-conditional-buff-completion.md
git add -A docs/superpowers/
git commit -m "chore: remove completed spec and plan docs"
```
