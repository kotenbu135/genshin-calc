# P3 複雑な武器4本 ConditionalBuff設計

**Date:** 2026-03-29
**Scope:** 霧切の廻光、千岩古剣/千岩長槍、赤砂の杖(二次)、草薙の稲光(二次)
**Status:** Reviewed

---

## 概要

P3残タスクの複雑な武器4本（5種ConditionalBuff）を実装する。
千岩シリーズには新`AutoCondition::TeamRegionCount`が必要。
併せて`eval_manual`の精錬値バグ修正と`Both`セマンティクス修正を行う。

## 変更サマリ

| 変更 | ファイル | 影響度 |
|------|---------|--------|
| AutoCondition::TeamRegionCount追加 | buff.rs | 低 |
| buff.rsに`use crate::types::Region`追加 | buff.rs | 低 |
| eval_auto: team_regions引数追加 + TeamRegionCount評価 | team_builder.rs | 中 |
| eval_manual: base_value引数追加 | team_builder.rs | 中 |
| Both: auto_valをmanualに渡す | team_builder.rs | 中 |
| TeamMemberBuilder: team_regionsフィールド+setter追加 | team_builder.rs | 低 |
| 霧切の廻光 ConditionalBuff | weapons/sword.rs | 低 |
| 千岩古剣 ConditionalBuff | weapons/claymore.rs | 低 |
| 千岩長槍 ConditionalBuff | weapons/polearm.rs | 低 |
| 赤砂の杖 二次ConditionalBuff | weapons/polearm.rs | 低 |
| 草薙の稲光 二次ConditionalBuff | weapons/polearm.rs | 低 |
| CLAUDE.md Both注記更新 | CLAUDE.md | 低 |
| data-expansion-todo.md更新 | docs/ | 低 |

---

## 1. 型変更: AutoCondition::TeamRegionCount

### buff.rs

```rust
use crate::types::Region;  // NEW import

#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AutoCondition {
    // ... existing variants ...
    /// Buff scales with count of team members from a specific region.
    /// Returns effective_value * count. 0 members → None.
    TeamRegionCount { region: Region },
}
```

`Region`はdata crateの`types.rs`に既存（Mondstadt, Liyue, Inazuma, Sumeru, Fontaine, Natlan, Snezhnaya, Other）。
`Region`はSerialize/Deserialize両方をderiveしており、AutoConditionのSerialize互換性に問題なし。
将来的にmax_count制限が必要な武器が出た場合は`Option<u8>`フィールドを追加可能。現時点ではゲーム仕様上チーム4人が自然な上限。

---

## 2. eval_auto変更

### team_builder.rs — eval_auto シグネチャ

```rust
fn eval_auto(
    cond: &AutoCondition,
    multiplier: f64,
    profile: &StatProfile,
    weapon_type: WeaponType,
    element: Element,
    team_elements: &[Element],
    team_regions: &[Region],  // NEW
    refinement: u8,
) -> Option<f64>
```

### TeamRegionCount評価ロジック

```rust
AutoCondition::TeamRegionCount { region } => {
    let count = team_regions.iter().filter(|r| **r == region).count() as f64;
    if count > 0.0 {
        Some(multiplier * count)
    } else {
        None
    }
}
```

TeamElementCountと同パターン。`multiplier`はresolve_value済みの精錬値対応値。

### TeamMemberBuilder — team_regionsフィールド追加

既存の`team_elements`パターンに合わせてsetterメソッドを追加:

```rust
pub struct TeamMemberBuilder {
    // ... existing fields ...
    team_regions: Vec<Region>,  // NEW (line 23 pattern)
}

impl TeamMemberBuilder {
    pub fn new() -> Self {
        Self {
            // ... existing init ...
            team_regions: Vec::new(),  // NEW (line 40 pattern)
        }
    }

    /// Set team regions for region-based conditional evaluation.
    pub fn team_regions(mut self, regions: Vec<Region>) -> Self {
        self.team_regions = regions;  // line 90 pattern
        self
    }
}
```

### resolve_conditionalsクロージャのplumbing

`build()`内の`resolve_conditionals`クロージャはeval_autoを3箇所で呼び出す（武器、聖遺物2pc、聖遺物4pc）。
すべての呼び出しに`&self.team_regions`を追加:

```rust
let resolve_conditionals = |..., buffs: &mut Vec<ResolvedBuff>| {
    for cond_buff in conditional_buffs {
        let effective_value = resolve_value(...);
        let resolved_value = match &cond_buff.activation {
            Activation::Auto(auto) => eval_auto(
                auto, effective_value, &profile, char_data.weapon_type,
                char_data.element, &self.team_elements,
                &self.team_regions,  // NEW — 3箇所すべて
                refinement,
            ),
            // ...
        };
    }
};
```

---

## 3. eval_manual精錬値修正

### 現状の問題

eval_manualは`buff.value`（R1固定値）を使用。`effective_value`（精錬値調整済み）を使うべき。

### 修正

```rust
fn eval_manual(
    cond: &ManualCondition,
    buff: &ConditionalBuff,
    activations: &[(&str, ManualActivation)],
    base_value: f64,  // NEW: effective_value (Manual) or auto_val (Both)
) -> Option<f64> {
    let activation = activations.iter().find(|(name, _)| *name == buff.name);
    match cond {
        ManualCondition::Toggle => match activation {
            Some((_, ManualActivation::Active)) => Some(base_value),
            _ => None,
        },
        ManualCondition::Stacks(max) => match activation {
            Some((_, ManualActivation::Stacks(n))) => {
                let effective = (*n).min(*max);
                if effective == 0 { return None; }
                match buff.stack_values {
                    Some(values) => Some(values[(effective as usize).min(values.len()) - 1]),
                    None => Some(base_value * f64::from(effective)),
                }
            }
            Some((_, ManualActivation::Active)) => {
                let effective = *max;
                match buff.stack_values {
                    Some(values) => Some(values[(effective as usize).min(values.len()) - 1]),
                    None => Some(base_value * f64::from(effective)),
                }
            }
            _ => None,
        },
    }
}
```

変更点:
- Toggle: `buff.value` → `base_value`
- Linear Stacks: `buff.value * n` → `base_value * n`
- Non-linear stack_values: **変更なし**（絶対値を直接使用、R1のみ）

### 呼び出し側

```rust
Activation::Manual(manual) => {
    eval_manual(manual, cond_buff, &self.manual_activations, effective_value)
}
```

---

## 4. Bothセマンティクス修正

### 現状の問題

```rust
// 現在: auto値を破棄（ゲートのみ）
auto_result.and_then(|_| eval_manual(manual, cond_buff, &self.manual_activations))
```

Staff of Homa低HP（Both(StatScaling(HP), Toggle)）でHP×1.0%の計算値が破棄され、
buff.value=0.010が返る（正しくはHP×0.010のスケーリング値）。

### 修正

```rust
Activation::Both(auto, manual) => {
    let auto_val = eval_auto(
        auto, effective_value, &profile, char_data.weapon_type,
        char_data.element, &self.team_elements, &self.team_regions, refinement,
    );
    auto_val.and_then(|av| {
        eval_manual(manual, cond_buff, &self.manual_activations, av)
    })
}
```

auto_valをbase_valueとしてeval_manualに渡す。

### 影響（既存Both 2件）

| 武器 | stat | Before | After |
|------|------|--------|-------|
| Staff of Homa (低HP) | AtkFlat | 0.010 固定 | HP × 0.010 |
| Azurelight | NormalAtkDmgBonus | 0.0016 固定 | HP × 0.0016 (cap 0.40) |

両方とも**バグ修正**。ゲーム内の実際の挙動と一致するようになる。

### Both + Stacks（新パターン: 赤砂の杖二次効果）

`Both(StatScaling(EM), Stacks(2))`の場合:
- `auto_val = EM × 0.28` (1単位分の値)
- `eval_manual`に`base_value = auto_val`として渡される
- Linear stacks (stack_values: None): `base_value × n = EM × 0.28 × 2`

**セマンティクス**: base_valueはバフ1単位分の値。線形スタックはその倍数。
ゲーム仕様「各スタックでEM×28%ATKアップ、最大2スタック」と一致。

---

## 5. 武器データ

### 5a. 霧切の廻光 (Mistsplitter Reforged)

**ファイル:** `crates/data/src/weapons/sword.rs`

既存StatBuff（DmgBonus +12-24%）に加え、スタックConditionalBuffを追加:

```rust
conditional_buffs: &[ConditionalBuff {
    name: "mistsplitter_emblem",
    description: "霧切の巴紋: 1/2/3スタックで元素DMG+8%/16%/28% (R1)",
    stat: BuffableStat::DmgBonus,
    value: 0.08,
    refinement_values: None,
    stack_values: Some(&[0.08, 0.16, 0.28]),
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Stacks(3)),
}],
```

**ゲーム仕様:**
- 1スタック: 通常攻撃で元素ダメージを与えた時
- 2スタック: 元素爆発発動時
- 3スタック: 元素エネルギーが100%未満の時

非線形スタック（[8, 16, 28] はR1値）。stack_valuesは絶対値のためR1限定。
R2-R5ゲーム値: R2=[10,20,35], R3=[12,24,42], R4=[14,28,49], R5=[16,32,56]。
5星R2+は極めて稀なため許容。descriptionに"(R1)"を明記。

### 5b. 千岩古剣 (Lithic Blade)

**ファイル:** `crates/data/src/weapons/claymore.rs`

パッシブ名を"千岩の槍"から"千岩の刃"に修正（既存データの誤り）:

```rust
passive: Some(WeaponPassive {
    name: "千岩の刃",  // FIX: was "千岩の槍"
    effect: PassiveEffect {
        description: "Conditional: チーム内の璃月キャラ人数に応じてATK/CRIT Rateアップ",
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "lithic_blade_atk",
                description: "璃月キャラ1人につきATK+7-11%",
                stat: BuffableStat::AtkPercent,
                value: 0.07,
                refinement_values: Some([0.07, 0.08, 0.09, 0.10, 0.11]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamRegionCount {
                    region: Region::Liyue,
                }),
            },
            ConditionalBuff {
                name: "lithic_blade_crit",
                description: "璃月キャラ1人につきCR+3-7%",
                stat: BuffableStat::CritRate,
                value: 0.03,
                refinement_values: Some([0.03, 0.04, 0.05, 0.06, 0.07]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamRegionCount {
                    region: Region::Liyue,
                }),
            },
        ],
    },
}),
```

完全自動評価。effective_value × 璃月キャラ数。

### 5c. 千岩長槍 (Lithic Spear)

**ファイル:** `crates/data/src/weapons/polearm.rs`

千岩古剣と同じパッシブ・同じ値。name prefixを`lithic_spear_`に変更。

```rust
conditional_buffs: &[
    ConditionalBuff {
        name: "lithic_spear_atk",
        description: "璃月キャラ1人につきATK+7-11%",
        stat: BuffableStat::AtkPercent,
        value: 0.07,
        refinement_values: Some([0.07, 0.08, 0.09, 0.10, 0.11]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Auto(AutoCondition::TeamRegionCount {
            region: Region::Liyue,
        }),
    },
    ConditionalBuff {
        name: "lithic_spear_crit",
        description: "璃月キャラ1人につきCR+3-7%",
        stat: BuffableStat::CritRate,
        value: 0.03,
        refinement_values: Some([0.03, 0.04, 0.05, 0.06, 0.07]),
        stack_values: None,
        target: BuffTarget::OnlySelf,
        activation: Activation::Auto(AutoCondition::TeamRegionCount {
            region: Region::Liyue,
        }),
    },
],
```

### 5d. 赤砂の杖 — 二次効果

**ファイル:** `crates/data/src/weapons/polearm.rs`

既存`scarlet_sands_em_atk`（Auto StatScaling、EM×52-104%→ATKフラット）に加え、スキル命中スタックを追加:

```rust
ConditionalBuff {
    name: "scarlet_sands_skill_stacks",
    description: "スキル命中後10秒間、EM×28-56%分ATKアップ。最大2スタック",
    stat: BuffableStat::AtkFlat,
    value: 0.28,
    refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Both(
        AutoCondition::StatScaling {
            stat: BuffableStat::ElementalMastery,
            offset: None,
            cap: None,
        },
        ManualCondition::Stacks(2),
    ),
},
```

Both修正により: `eval_auto(EM × 0.28) → eval_manual(auto_val × stacks)`。
R1 EM200 2スタック: 200 × 0.28 × 2 = 112 ATK flat。
合計（base + secondary）: 200×0.52 + 200×0.28×2 = 104 + 112 = 216 ATK flat。

### 5e. 草薙の稲光 — 二次効果

**ファイル:** `crates/data/src/weapons/polearm.rs`

既存`engulfing_er_atk`（Auto StatScaling、ER超過×28-56%→ATK%、cap付き）に加え、爆発後ERアップを追加:

```rust
ConditionalBuff {
    name: "engulfing_burst_er",
    description: "元素爆発後12秒間ER+30-50%",
    stat: BuffableStat::EnergyRecharge,
    value: 0.30,
    refinement_values: Some([0.30, 0.35, 0.40, 0.45, 0.50]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
},
```

eval_manual修正によりeffective_valueを返すのでR1-R5すべて正確。

---

## 6. テスト計画

### ユニットテスト（team_builder.rs）

**eval_auto TeamRegionCount:**
- 璃月0人 → None
- 璃月1人 → Some(value × 1)
- 璃月3人 → Some(value × 3)
- team_regions空 → None
- 璃月以外のRegion → None

**eval_manual base_value修正:**
- Toggle: base_value=0.30 → returns 0.30（not buff.value）
- Linear Stacks: base_value=0.05, 3stacks → returns 0.15
- Non-linear stack_values: base_value無視、stack_values[n-1]を返却

**Both新セマンティクス:**
- StatScaling(HP) + Toggle: returns auto_val (HP × multiplier)
- StatScaling(EM) + Stacks(2): returns auto_val × 2

**既存Bothテスト更新（呼び出し規約 + 期待値）:**
- `test_both_auto_pass_manual_pass`: eval_manual呼び出しに`av`引数追加、期待値をauto_valに更新
- `test_both_auto_fail_manual_pass`: 変更なし（auto None → 全体None）
- `test_both_auto_pass_manual_fail`: eval_manual呼び出しに`av`引数追加、結果はNone（変更なし）

### 武器データテスト（各weapons/*.rs）

各武器に以下のテスト:
- ConditionalBuff数の検証
- name, stat, value, activation pattern のmatches!検証
- refinement_values存在確認
- stack_values検証（霧切のみ）

### 統合テスト（team_builder）

- 千岩古剣 + 璃月2人チーム: ATK% = 0.07×2=0.14, CR = 0.03×2=0.06
- 赤砂の杖 + EM200キャラ + 2スタック: ATK flat = 200×0.52 + 200×0.28×2 = 216
- 草薙の稲光 + Toggle ON: ER +0.30 がStatProfileに反映

### 既存テスト影響

- character_verification TOML: Homa/Azurelight使用ケースがあれば期待値更新
- data_integrity: Both + Stacks(max>0)チェック — 変更なし
- eval_manual既存テスト: 全呼び出しにbase_value引数追加

---

## 7. ドキュメント更新

- **CLAUDE.md**: Both注記を「auto_valをmanualのbase_valueとして渡す」に更新。TeamRegionCount追記。
- **data-expansion-todo.md**: 4本完了マーク。赤砂/草薙の説明を「二次効果追加」に更新（一次効果は実装済み）。P3残タスク→0本。残タスクセクション更新。

---

## 制約・トレードオフ

1. **霧切R2-R5非線形スタック未対応**: stack_valuesが絶対R1値のため。5星R2+は極めて稀で実用上問題なし。将来`refinement_stack_values`フィールド追加で対応可能。
2. **Both + 非線形stack_values**: 現時点で該当ケースなし。必要になった時に対応。
3. **Both修正による既存テスト更新**: Homa/Azurelight関連テスト2-3件の期待値・呼び出し規約変更が必要。バグ修正なので正当。
4. **eval_auto引数が8個**: 将来パラメータ構造体へのリファクタリング候補。本specの範囲外。
