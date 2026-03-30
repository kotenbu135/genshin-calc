# Aino (Hydro/Claymore/★4) Character Data Completion Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Aino のキャラデータ実装を補完する — 天賦名のプレースホルダー修正、天賦バフ追加（A4 + C6）、月兆データ更新、TOML テストケース作成。

**Architecture:** データのみの変更。`crates/data` 内の既存定義を更新・追加。`crates/core` への変更なし。

**Tech Stack:** Rust const definitions, TOML test data, cargo test

**Source:** Honey Impact / Genshin Impact Wiki / KQM

---

## File Map

| Action | File | Responsibility |
|--------|------|---------------|
| Modify | `crates/data/src/characters/hydro.rs:2590-2733` | 天賦名プレースホルダー修正 + コンステパターン修正 |
| Modify | `crates/data/src/talent_buffs.rs` | A4 Burst DMGボーナス追加 + C6 反応DMGボーナス追加 |
| Modify | `crates/data/src/moonsign_chars.rs:70-76` | 月兆スタブ更新 + タレント強化追加 |
| Create | `crates/core/tests/data/characters/aino.toml` | TOML テストケース |

---

## 調査結果サマリ

### 天賦名（Honey Impact 確認済み）
- Normal Attack: **Bish-Bash-Bosh Repair**
- Elemental Skill: **Musecatcher**
- Elemental Burst: **Precision Hydronic Cooler**

### コンステパターン
- 現在: `C3SkillC5Burst` → 正: `C3BurstC5Skill`（C3=爆発+3, C5=スキル+3）

### 天賦バフ（要実装）
| Source | Buff | 値 | 対象 | 備考 |
|--------|------|----|------|------|
| C1 | EM +80 | 80.0 | Team | **実装済み** |
| A4 | Burst DMG Bonus = 50% × EM | 0.50 × EM | Self | `scales_on: Em`, `stat: BurstDmgBonus` |
| C6 | 反応DMG +15% (EC/Bloom/Lunar系) | 0.15 | Team | 爆発使用後15秒。Ascendant Gleamで+20%追加 |

### 月兆データ
Aino の A1 パッシブは「チーム月兆レベル+1」で、月光の祝福ボーナスの stat scaling ではない。MoonsignBenedictionDef の `enabled_reactions: &[]` は正しい可能性が高い。ただし C6 に月反応DMGバフがあるため、タレント強化として実装する。

---

## Task 1: 天賦名 + コンステパターン修正

**Files:**
- Modify: `crates/data/src/characters/hydro.rs:2590-2733`

- [ ] **Step 1: コメントヘッダ更新**

```rust
// =============================================================================
// Aino — 4★ Hydro Claymore (Nod-Krai / v6.0)
// Source: Honey Impact (gensh.honeyhunterworld.com) 2026-03-30
// Normal Attack: Bish-Bash-Bosh Repair
// Elemental Skill: Musecatcher
// Elemental Burst: Precision Hydronic Cooler
// =============================================================================
```

置換対象（L2590-2593）:
```
// Aino — 4★ Hydro Claymore (Nod-Krai / v6.0)
// Placeholder values — update from Genshin Impact Wiki when available
```

- [ ] **Step 2: 天賦名プレースホルダーを実名に置換**

L2718: `"Placeholder Normal Attack"` → `"Bish-Bash-Bosh Repair"`
L2724: `"Placeholder Skill"` → `"Musecatcher"`
L2728: `"Placeholder Burst"` → `"Precision Hydronic Cooler"`

- [ ] **Step 3: コンステパターン修正**

L2732: `ConstellationPattern::C3SkillC5Burst` → `ConstellationPattern::C3BurstC5Skill`

- [ ] **Step 4: ビルド確認**

Run: `cargo build -p genshin-calc-data`
Expected: コンパイル成功

- [ ] **Step 5: コミット**

```bash
git add crates/data/src/characters/hydro.rs
git commit -m "fix(data): update Aino talent names and constellation pattern"
```

---

## Task 2: A4 天賦バフ追加

**Files:**
- Modify: `crates/data/src/talent_buffs.rs`

- [ ] **Step 1: AINO_BUFFS に A4 バフ追加**

既存の `AINO_BUFFS` 配列（C1のみ）に A4 を追加する。

現在（L729-742）:
```rust
static AINO_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Aino C1 EM Share",
    ...
}];
```

変更後:
```rust
static AINO_BUFFS: &[TalentBuffDef] = &[
    // C1: EM+80 to Aino and active party member after skill/burst for 15s
    TalentBuffDef {
        name: "Aino C1 EM Share",
        description: "After Skill or Burst, Aino and active character gain EM+80 for 15s",
        stat: BuffableStat::ElementalMastery,
        base_value: 80.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
    },
    // A4: Structured Power Booster — Burst DMG increased by 50% of EM
    TalentBuffDef {
        name: "Aino A4 Burst DMG from EM",
        description: "Burst DMG increased by 50% of Elemental Mastery",
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.50,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
];
```

注意: `scales_on: Some(ScalingStat::Em)` により、`team_builder.rs` で `value = 0.50 * profile.elemental_mastery` と計算される。

- [ ] **Step 2: テスト確認**

Run: `cargo test -p genshin-calc-data -- aino`
Expected: 既存の C1 テスト通過 + A4 バフが `find_talent_buffs("aino")` で 2 件返却

- [ ] **Step 3: A4 用テスト追加**

既存テスト（L1002-1008付近）の隣に追加:

```rust
#[test]
fn test_aino_a4_burst_dmg_from_em() {
    let buffs = find_talent_buffs("aino").unwrap();
    let a4 = buffs.iter().find(|b| b.source == TalentBuffSource::AscensionPassive).unwrap();
    assert_eq!(a4.stat, BuffableStat::BurstDmgBonus);
    assert!((a4.base_value - 0.50).abs() < 1e-6);
    assert_eq!(a4.scales_on, Some(ScalingStat::Em));
    assert_eq!(a4.target, BuffTarget::OnlySelf);
}
```

- [ ] **Step 4: テスト実行**

Run: `cargo test -p genshin-calc-data -- aino`
Expected: PASS

- [ ] **Step 5: コミット**

```bash
git add crates/data/src/talent_buffs.rs
git commit -m "feat(data): add Aino A4 passive — Burst DMG scales on EM"
```

---

## Task 3: C6 天賦バフ追加

**Files:**
- Modify: `crates/data/src/talent_buffs.rs`

- [ ] **Step 1: C6 反応DMGバフを AINO_BUFFS に追加**

C6 "The Burden of Creative Genius": 爆発使用後15秒間、近くのアクティブキャラの反応DMG +15%。

```rust
    // C6: The Burden of Creative Genius — Reaction DMG +15% for 15s after Burst
    TalentBuffDef {
        name: "Aino C6 Reaction DMG Bonus",
        description: "After Burst, nearby active characters' reaction DMG +15% for 15s",
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
```

注意: C6 の Ascendant Gleam 追加ボーナス (+20%) は月兆タレント強化として Task 4 で実装する。

- [ ] **Step 2: テスト追加**

```rust
#[test]
fn test_aino_c6_reaction_dmg_bonus() {
    let buffs = find_talent_buffs("aino").unwrap();
    let c6 = buffs.iter().find(|b| b.source == TalentBuffSource::Constellation(6)).unwrap();
    assert_eq!(c6.stat, BuffableStat::TransformativeBonus);
    assert!((c6.base_value - 0.15).abs() < 1e-6);
    assert_eq!(c6.min_constellation, 6);
    assert_eq!(c6.target, BuffTarget::Team);
}
```

- [ ] **Step 3: テスト実行**

Run: `cargo test -p genshin-calc-data -- aino`
Expected: PASS (3 buffs total: C1, A4, C6)

- [ ] **Step 4: コミット**

```bash
git add crates/data/src/talent_buffs.rs
git commit -m "feat(data): add Aino C6 reaction DMG bonus"
```

---

## Task 4: 月兆データ更新

**Files:**
- Modify: `crates/data/src/moonsign_chars.rs`

- [ ] **Step 1: 月兆ベネディクション確認**

Aino の A1 パッシブ「Force Limit Analysis」は「パーティの月兆レベル+1」であり、`MoonsignBenedictionDef` の stat scaling ベネディクションではない。`enabled_reactions: &[]` のまま据え置き。

ただし、コメントで A1 効果を記載する:

```rust
    // Aino — A1 raises party Moonsign level by 1 (handled by MoonsignContext.level,
    // not via benediction stat scaling). No personal lunar reaction DMG scaling.
    MoonsignBenedictionDef {
        character_id: "aino",
        character_name: "Aino",
        enabled_reactions: &[],
        scaling_stat: None,
        rate: 0.0,
        max_bonus: 0.0,
    },
```

- [ ] **Step 2: C6 月兆タレント強化追加**

C6 の Ascendant Gleam 追加効果（反応DMG +20%追加）をタレント強化として定義:

```rust
pub const AINO_TALENT_ENHANCEMENTS: &[MoonsignTalentEnhancement] = &[MoonsignTalentEnhancement {
    character_name: "Aino",
    required_level: MoonsignLevel::AscendantGleam,
    description: "At Ascendant Gleam, reaction DMG bonus from C6 increases by +20% (total +35%)",
    effect: MoonsignTalentEffect::ReactionDmgBonus {
        reaction: Reaction::LunarElectroCharged,
        bonus: 0.20,
    },
}];
```

注意: C6 は Electro-Charged / Bloom / Lunar系の複数反応に適用されるが、`MoonsignTalentEffect::ReactionDmgBonus` は単一反応指定。Honey Impact の詳細データで最も重要な反応を選択する。複数反応が必要な場合は配列に複数エントリを追加する。

- [ ] **Step 3: `find_moonsign_talent_enhancements` に match arm 追加**

L150-154 の match 式に追加:

```rust
        "lauma" => LAUMA_TALENT_ENHANCEMENTS,
        "flins" => FLINS_TALENT_ENHANCEMENTS,
        "nefer" => NEFER_TALENT_ENHANCEMENTS,
        "aino" => AINO_TALENT_ENHANCEMENTS,
        _ => &[],
```

- [ ] **Step 4: テスト追加**

既存テスト `test_find_moonsign_benediction_aino_none` がベネディクション空を検証済みのため、追加不要。

```rust
#[test]
fn test_aino_talent_enhancements_at_ascendant_gleam() {
    let enhancements = find_moonsign_talent_enhancements("aino", MoonsignLevel::AscendantGleam);
    assert_eq!(enhancements.len(), 1);
    assert_eq!(enhancements[0].required_level, MoonsignLevel::AscendantGleam);
}

#[test]
fn test_aino_talent_enhancements_at_nascent_gleam() {
    let enhancements = find_moonsign_talent_enhancements("aino", MoonsignLevel::NascentGleam);
    assert!(enhancements.is_empty());
}
```

- [ ] **Step 5: テスト実行**

Run: `cargo test -p genshin-calc-data -- aino`
Expected: PASS

- [ ] **Step 6: コミット**

```bash
git add crates/data/src/moonsign_chars.rs
git commit -m "feat(data): add Aino moonsign talent enhancements (C6 Ascendant Gleam)"
```

---

## Task 5: TOML テストケース作成

**Files:**
- Create: `crates/core/tests/data/characters/aino.toml`

- [ ] **Step 1: TOML ファイル作成**

Skill (Musecatcher) Lv10 で基本ダメージテストを作成。

天賦倍率: `AINO_SKILL` Lv10 = 3.024 (ATK scaling, Hydro)

テスト条件:
- Lv90 Aino, ATK 1400, DEF 664, EM 200
- 50%/100% crit, 46.6% Hydro DMG Bonus
- Enemy Lv90, 10% Hydro resistance

手計算:
- Base DMG = 1400 × 3.024 = 4233.6
- DMG Bonus = 1 + 0.466 = 1.466
- After bonus = 4233.6 × 1.466 = 6206.4576
- DEF multiplier = (90+100) / ((90+100) + (90+100)) = 0.5
- After DEF = 6206.4576 × 0.5 = 3103.2288
- RES multiplier = 1 - 0.10 = 0.90
- Non-crit = 3103.2288 × 0.90 = 2792.90592
- Crit = 2792.90592 × (1 + 1.00) = 5585.81184
- Average = 2792.90592 × (1 + 0.50 × 1.00) = 4189.35888

```toml
# Source: Honey Impact — Aino Skill (Musecatcher) Lv10
# Hand-calculated: ATK 1400 × 3.024 × 1.466 × 0.5 × 0.90

[character]
name = "Aino"
element = "Hydro"

# Case 1: Skill (Musecatcher) Lv10 — no reactions
[[cases]]
type = "normal"
name = "Skill Lv10 vs Lv90 enemy"
character_level = 90
talent_multiplier = 3.024
scaling_stat = "Atk"
damage_type = "Skill"
element = "Hydro"

[cases.stats]
hp = 10768.0
atk = 1400.0
def = 664.0
elemental_mastery = 200.0
crit_rate = 0.50
crit_dmg = 1.00
energy_recharge = 1.0
dmg_bonus = 0.466

[cases.enemy]
level = 90
resistance = 0.10

[cases.expected]
non_crit = 2792.906
crit = 5585.812
average = 4189.359
```

注意: `expected` 値は手計算値。テスト実行で微小な浮動小数点差がある場合は許容誤差 `< 0.01` で判定される。

- [ ] **Step 2: テスト実行**

Run: `cargo test -p genshin-calc-core --test character_verification -- aino`
Expected: PASS（許容誤差内）

計算が合わない場合はテスト出力の actual 値と手計算を再照合する。

- [ ] **Step 3: コミット**

```bash
git add crates/core/tests/data/characters/aino.toml
git commit -m "test(data): add Aino TOML test case — Skill Lv10"
```

---

## Task 6: 全体検証

- [ ] **Step 1: フルビルド + テスト**

```bash
cargo build && cargo test && cargo clippy -- -D warnings && cargo fmt --check
```

Expected: 全パス

- [ ] **Step 2: 最終コミット（必要な場合）**

lint/format 修正があればここでコミット。

---

## 設計判断メモ

1. **C1 vs A2 の関係**: C1 は A2 と同一効果（EM+80）だが、C1 で解放される。現在の実装 `source: TalentBuffSource::Constellation(1)` はそのまま維持。A2 単体での TalentBuffDef は C1 と重複するため追加しない。

2. **C6 の反応対象**: C6 は EC/Bloom/Lunar-Charged/Lunar-Bloom/Lunar-Crystallize の5反応に適用される。`TransformativeBonus` で汎用的にモデル化。月兆強化の +20% 追加は `MoonsignTalentEnhancement` として分離。

3. **月兆ベネディクション**: Aino は月兆レベル+1のサポーター。個人の月反応DMG scaling はないため `enabled_reactions: &[]` を維持。

4. **C2 追加ヒット**: C2 は追加水球（25% ATK + 100% EM）のプロックダメージ。CLAUDE.md の武器 ConditionalBuff 分類ルールに従い「proc damage（追加ヒットのみ）」は SKIP。

5. **C4 エネルギー回復**: C4 はスキル命中時にエネルギー10回復（10秒に1回）。「エネルギー生成（Favonius系）」と同分類のユーティリティ効果のため SKIP。
