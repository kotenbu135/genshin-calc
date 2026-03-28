# P3 Batch 1: 5星武器 StatScaling ConditionalBuff 設計

**Date**: 2026-03-29
**Status**: Reviewed (v3)
**Depends on**: P0 (ConditionalBuff — 完了), P3 既存StatScaling実装 (磐岩結緑・護摩)
**Scope**: Core拡張4点 + 8本の5星武器にStatScaling ConditionalBuff追加

## 概要

既存の `AutoCondition::StatScaling` を拡張し、8本の5星武器のステータス依存パッシブを正確にモデリングする。
Core crateに4つの拡張、Data crateに8本の武器データ追加。

## Core拡張

### 拡張1: `StatScaling` に `offset` フィールド追加

草薙の稲光はER「超過分」（ER - 100%）にスケーリングする。現在のフレームワークは総ステータス値に対して計算するため、ベースラインを引く `offset` が必要。

```rust
StatScaling {
    stat: BuffableStat,
    offset: Option<f64>,              // 新規追加: スケーリング前にソース値から引く
    cap: Option<[f64; 5]>,            // 変更: 精錬別cap値（R1-R5）
}
```

計算式: `((stat_val - offset.unwrap_or(0.0)).max(0.0)) * multiplier`

- offset後の値は `.max(0.0)` でクランプ（負のバフ値を防止、ゲーム仕様準拠）
- 既存武器は `offset: None` で動作変更なし。草薙は `offset: Some(1.0)` でER超過分を計算

### 拡張2: `cap` を精錬対応に統一

旧: `cap: Option<f64>` — 単一cap値
新: `cap: Option<[f64; 5]>` — R1-R5のcap値配列

- 既存武器（Jade Cutter, Homa）は `cap: None` のまま、変更なし
- 草薙は `cap: Some([0.80, 0.90, 1.00, 1.10, 1.20])`
- `eval_auto` に `refinement: usize` パラメータを追加し、`cap[refinement]` でインデックス

> 旧 `cap: Option<f64>` は削除。`Option<[f64; 5]>` に一本化することで冗長性と曖昧さを排除。

### 拡張3: `DamageInput` に `flat_dmg` フィールド追加

萃光の裁葉・Hunter's Path・赤角石塵滅砕は「フラットダメージ加算」を行う。
これはShenhe/Yun Jinと同じ仕組みで、`ATK × 倍率` の後に加算される。

```rust
pub struct DamageInput {
    // ... 既存フィールド ...
    /// Flat damage added to base (e.g. Shenhe quill, weapon flat DMG scaling).
    #[serde(default)]
    pub flat_dmg: f64,  // 新規追加
}
```

計算式変更:
```rust
// Before:
let base = scaling_value * input.talent_multiplier + catalyze_flat;
// After:
let base = scaling_value * input.talent_multiplier + catalyze_flat + input.flat_dmg;
```

既存コードは `flat_dmg: 0.0` （`#[serde(default)]`）で動作変更なし。

> 注: 既存の `DamageInput` 構築箇所（テスト・docコメント・examples）に `flat_dmg: 0.0` を追加する必要がある。CLAUDE.mdに記載の通り「DamageInput変更時は全構築箇所を一括修正」。

**BuffableStat フラットダメージ variants (5種):**
- `NormalAtkFlatDmg`
- `ChargedAtkFlatDmg`
- `PlungingAtkFlatDmg`
- `SkillFlatDmg`
- `BurstFlatDmg`

Batch 1では3種のみ使用するが、DamageTypeとの1:1対応のため5種全て追加。

**Consumer側 flat_dmg 収集ヘルパー:**

```rust
pub fn collect_flat_dmg(buffs: &[ResolvedBuff], damage_type: DamageType) -> f64 {
    let target_stat = match damage_type {
        DamageType::Normal => BuffableStat::NormalAtkFlatDmg,
        DamageType::Charged => BuffableStat::ChargedAtkFlatDmg,
        DamageType::Plunging => BuffableStat::PlungingAtkFlatDmg,
        DamageType::Skill => BuffableStat::SkillFlatDmg,
        DamageType::Burst => BuffableStat::BurstFlatDmg,
    };
    buffs.iter()
        .filter(|b| b.stat == target_stat)
        .map(|b| b.value)
        .sum()
}
```

このヘルパーを `crates/core/src/damage.rs` に配置し、re-export。Consumer側は:
```rust
let flat_dmg = collect_flat_dmg(&result.applied_buffs, DamageType::Normal);
let input = DamageInput { ..., flat_dmg };
```

### 拡張4: `BuffableStat` に `DefPercentRaw` 追加

有楽御簾切りは「DEF増加分」（`def_percent` 値そのもの）にスケーリングする。
現在の `DefPercent` は `read_stat_for_scaling` で「総DEF」を返すため、生の `def_percent` 値を読むための別variant が必要。

```rust
pub enum BuffableStat {
    // ... 既存 ...
    DefPercentRaw,  // 新規: StatProfile.def_percent をそのまま返す
}
```

`read_stat_for_scaling` に分岐追加:
```rust
BuffableStat::DefPercentRaw => profile.def_percent,
```

## 8本の武器定義

### 1. Engulfing Lightning (草薙の稲光) — Polearm

```
入力: EnergyRecharge, offset: Some(1.0)  (ER超過分)
出力: AtkPercent
R1-R5値: [0.28, 0.35, 0.42, 0.49, 0.56]
Cap R1-R5: [0.80, 0.90, 1.00, 1.10, 1.20]
追加効果: 爆発後ER+30% → Batch 2 (Toggle)
```

### 2. Staff of the Scarlet Sands (赤砂の杖) — Polearm

```
入力: ElementalMastery
出力: AtkFlat
R1-R5値: [0.52, 0.65, 0.78, 0.91, 1.04]
Cap: None
追加効果: スキル命中ATKスタック → Batch 2 (Stacks)
```

### 3. Key of Khaj-Nisut — Sword

```
入力: HpPercent (→ 総HP)
出力: ElementalMastery (フラット)
R1-R5値: [0.0012, 0.0015, 0.0018, 0.0021, 0.0024]
Cap: None
追加効果: フルスタックチームEM → Batch 2 (Stacks+Team)
```

### 4. Light of Foliar Incision (萃光の裁葉) — Sword

2つの ConditionalBuff — Consumer側で `flat_dmg` に変換:

```
ConditionalBuff 1:
  入力: ElementalMastery
  出力: NormalAtkFlatDmg (新BuffableStat → Consumer側でflat_dmgへ)
  R1-R5値: [1.20, 1.50, 1.80, 2.10, 2.40]

ConditionalBuff 2:
  入力: ElementalMastery
  出力: SkillFlatDmg (新BuffableStat → Consumer側でflat_dmgへ)
  R1-R5値: 同上
```

> フラットダメージBuffableStatは `NormalAtkFlatDmg`, `ChargedAtkFlatDmg`, `SkillFlatDmg` の3種を追加。Consumer側がDamageTypeに応じて該当するフラットダメージを `DamageInput.flat_dmg` に設定。

### 5. Peak Patrol Song (頂の祈念歌) — Sword

```
入力: DefPercent (→ 総DEF)
出力: DmgBonus
R1-R5値: [0.08, 0.10, 0.12, 0.14, 0.16]
Cap: None
追加効果: チームDEF%/元素DMG → Batch 2 (Stacks+Team)
備考: DmgBonusは物理含む全ダメージに適用。元素限定はConsumer側制御。
```

### 6. Uraku Misugiri (有楽御簾切り) — Sword

```
入力: DefPercentRaw (→ def_percent生値)
出力: SkillDmgBonus
R1-R5値: [0.18, 0.225, 0.27, 0.315, 0.36]
Cap: None
```

### 7. Hunter's Path (猟人の道) — Bow

```
入力: ElementalMastery
出力: ChargedAtkFlatDmg (新BuffableStat → Consumer側でflat_dmgへ)
R1-R5値: [1.60, 2.00, 2.40, 2.80, 3.20]
Cap: None
```

### 8. Redhorn Stonethresher (赤角石塵滅砕) — Claymore

2つの ConditionalBuff:

```
ConditionalBuff 1:
  入力: DefPercent (→ 総DEF)
  出力: NormalAtkFlatDmg
  R1-R5値: [0.40, 0.50, 0.60, 0.70, 0.80]

ConditionalBuff 2:
  入力: DefPercent (→ 総DEF)
  出力: ChargedAtkFlatDmg
  R1-R5値: 同上
```

## テスト計画

### Core ユニットテスト

- `DamageInput.flat_dmg` 加算テスト: flat_dmg=100, base計算に加算確認
- `DamageInput.flat_dmg` ゼロ時: 既存動作と同一確認
- `StatScaling` offset テスト: ER=1.80, offset=1.0 → 0.80にスケーリング
- `StatScaling` cap_refinement_values テスト: 精錬別cap適用確認
- `DefPercentRaw` テスト: def_percent生値を返す確認
- serde roundtrip テスト: 新BuffableStat variants

### Data ユニットテスト

各武器のConditionalBuff存在確認 + 値検証（8テスト）

### 統合テスト

- Engulfing Lightning: ER 180% → ATK% = (1.80-1.0) × 0.28 = 0.224, cap 0.80
- Redhorn Stonethresher: DEF 2000 → NormalAtkFlatDmg = 2000 × 0.40 = 800
- Staff of Scarlet Sands: EM 200 → AtkFlat = 200 × 0.52 = 104

## 影響範囲

### Core crate

| ファイル | 変更内容 |
|----------|---------|
| `buff_types.rs` | `NormalAtkFlatDmg`, `ChargedAtkFlatDmg`, `PlungingAtkFlatDmg`, `SkillFlatDmg`, `BurstFlatDmg`, `DefPercentRaw` 追加 |
| `damage.rs` | `DamageInput.flat_dmg` 追加、base計算に加算、`collect_flat_dmg` ヘルパー追加 |
| `lib.rs` | 新型の re-export（必要に応じて） |

### Data crate

| ファイル | 変更内容 |
|----------|---------|
| `buff.rs` | `AutoCondition::StatScaling` に `offset`, `cap_refinement_values` 追加 |
| `team_builder.rs` | `read_stat_for_scaling` に `DefPercentRaw` 分岐追加、`eval_auto` に `refinement: usize` パラメータ追加、offset/cap 対応 |
| `weapons/sword.rs` | Key of Khaj-Nisut, Light of Foliar Incision, Peak Patrol Song, Uraku Misugiri |
| `weapons/polearm.rs` | Engulfing Lightning, Staff of Scarlet Sands |
| `weapons/bow.rs` | Hunter's Path |
| `weapons/claymore.rs` | Redhorn Stonethresher |

### 既存コードへの影響

- `DamageInput` 全構築箇所に `flat_dmg: 0.0` 追加が必要
- `StatScaling` 全構築箇所: `cap: Option<f64>` → `cap: Option<[f64; 5]>` に変更 + `offset: None` 追加
- 既存テストの動作は変更なし（デフォルト値で互換維持）

## 注意: ゲームデータ値の検証

R1-R5のスケーリング値・cap値はデータマイニング値に基づく。実装時にHoneyhunterWorld/Ambr.top等で検証すること。
