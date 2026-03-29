# P6: 敵側デバフシステム設計

**Date**: 2026-03-29
**Status**: Reviewed (v2, approved)
**Depends on**: P0 (ConditionalBuff — 完了), P2 (聖遺物4pc — 完了)
**Unblocks**: P1残り4キャラ (Zhongli, Chevreuse, Lisa, Faruzan)

> Nilou は耐性/DEFデバフではなく開花反応ボーナス条件のため、P6スコープ外。

## 概要

ResolvedBuff に含まれる耐性減少・DEF減少バフを Enemy に適用する仕組みを実装する。
core crate にヘルパー関数を追加し、data crate に天賦デバフ4キャラ + 超電導デバフを追加する。

## 設計方針

- **イミュータブル**: `apply_enemy_debuffs` は新しい `Enemy` を返す
- **純粋関数**: 副作用なし、入力→出力が一意
- **core/data責務分離**: core は適用ロジック、data はデバフ値定義
- **既存API最小変更**: `Enemy` 構造体・`BuffTarget` enum は変更しない。新関数と `BuffableStat` variant 追加のみ
- **BuffableStat による区別**: 敵側デバフは `BuffTarget::Enemy` ではなく `BuffableStat` の種類（`ElementalResReduction`, `PhysicalResReduction`, `DefReduction`）で判別する。既存の翠緑・深林ConditionalBuffは `BuffTarget::Team` で定義済みのため、`BuffTarget` に新variant は追加しない

## 1. Core: `apply_enemy_debuffs` 関数

### 場所: `crates/core/src/enemy.rs`

```rust
pub fn apply_enemy_debuffs(
    enemy: &Enemy,
    buffs: &[ResolvedBuff],
    element: Option<Element>,
) -> Enemy
```

### 処理ロジック

1. `buffs` をイテレートし、以下の `BuffableStat` を処理:
   - `ElementalResReduction(e)`: `element` が `Some(e)` と一致する場合、`resistance` から `value` を減算
   - `PhysicalResReduction`: `element` が `None` の場合、`resistance` から `value` を減算
   - `DefReduction`: `def_reduction` に `value` を加算
2. 上記以外の `BuffableStat` は無視（味方バフなのでスキップ）
3. 最終値の処理:
   - `resistance`: 下限なし（負の耐性は原神仕様で許容、`resistance_multiplier` が処理済み）
   - `def_reduction`: 全デバフ加算後の最終値を `f64::min(1.0, ...)` でクランプ（100%超過は原神では発生しない）
   - `def_reduction` は既存値に加算する（`enemy.def_reduction` + 全 `DefReduction` バフの合計）
4. 新しい `Enemy` を返す

## 2. Core: `BuffableStat::DefReduction` 追加

### 場所: `crates/core/src/buff_types.rs`

```rust
pub enum BuffableStat {
    // ... 既存variants ...
    ElementalResReduction(Element),
    PhysicalResReduction,
    DefReduction,  // 新規追加
}
```

Lisa A4 (DEF-15%) を表現するために必要。`apply_enemy_debuffs` で `Enemy.def_reduction` に加算される。

> 注: DEF reduction と DEF ignore は原神では別の仕組み。DEF reduction は防御乗算式の `(1 - def_reduction)` に作用し、DEF ignore (例: 雷電C2) は別途 `(1 - def_ignore)` で乗算される。現時点では DEF ignore は未実装・スコープ外。将来必要になった場合は `Enemy` に `def_ignore` フィールドを追加する。

## 3. Core: 超電導ヘルパー

### 場所: `crates/core/src/enemy.rs`

```rust
pub fn superconduct_debuff() -> ResolvedBuff {
    ResolvedBuff {
        source: "Superconduct".to_string(),
        stat: BuffableStat::PhysicalResReduction,
        value: 0.40,
        target: BuffTarget::Team,  // 既存パターンに合わせる
    }
}
```

超電導の物理耐性-40%デバフを生成する便利関数。Consumer側が反応発動時にbuffsリストに追加して使用。
`apply_enemy_debuffs` は `BuffTarget` ではなく `BuffableStat` で判別するため、`target` の値は動作に影響しない。

## 4. Data: 天賦デバフ4キャラ

### 場所: `crates/data/src/talent_buffs.rs`

| キャラ | ID | デバフ | BuffableStat | 値 | 条件 |
|--------|-----|--------|-------------|-----|------|
| Zhongli | `zhongli` | 玉璋シールド全耐性-20% | `ElementalResReduction(全7元素)` + `PhysicalResReduction` | 0.20 | シールド展開中 |
| Chevreuse | `chevreuse` | 過負荷後炎/雷耐性-40% | `ElementalResReduction(Pyro)` + `ElementalResReduction(Electro)` | 0.40 | 過負荷反応後 |
| Lisa | `lisa` | A4 DEF-15% | `DefReduction` | 0.15 | 元素爆発命中後 |
| Faruzan | `faruzan` | A4 風耐性-30% | `ElementalResReduction(Anemo)` | 0.30 | 元素爆発使用中 |

> **注**: 天賦デバフは既存の天賦バフと同様に「キャラがチームにいれば有効」として扱う（always-on）。条件付き発動の仕組み（シールド有無、反応トリガー等）は将来の拡張スコープ。

### Zhongli の展開

Zhongli の全耐性-20% は8つの `TalentBuffDef` エントリに展開:
- `ElementalResReduction(Element::Pyro)` = 0.20
- `ElementalResReduction(Element::Hydro)` = 0.20
- `ElementalResReduction(Element::Electro)` = 0.20
- `ElementalResReduction(Element::Cryo)` = 0.20
- `ElementalResReduction(Element::Dendro)` = 0.20
- `ElementalResReduction(Element::Anemo)` = 0.20
- `ElementalResReduction(Element::Geo)` = 0.20
- `PhysicalResReduction` = 0.20

全エントリに `target: BuffTarget::Team` を設定（既存パターン準拠）。

### Faruzan 風耐性シュレッド

FaruzanのA4「暴風の害」は固定値30%の風耐性減少（天賦レベル非依存）。
既存の風ダメバフ TalentBuffDef に加えて、新たに耐性シュレッド用エントリを追加。
`scales_with_talent: false`, `base_value: 0.30`。

### Chevreuse 追加デバフ

既存の ATK+20% (A1) に加えて、過負荷条件の耐性シュレッドを追加。
炎・雷の2エントリ。

## 5. Consumer側フロー

```rust
// 1. チームバフ解決
let result = resolve_team_stats_detailed(&team, target_index)?;

// 2. 敵データから Enemy 生成
let enemy = enemy_data.to_enemy(Some(Element::Pyro), 90, 0.0);

// 3. デバフ適用（超電導がある場合は追加）
let all_buffs: Vec<_> = result.applied_buffs.iter().cloned()
    .chain(has_superconduct.then(superconduct_debuff))
    .collect();
let enemy = apply_enemy_debuffs(&enemy, &all_buffs, Some(Element::Pyro));

// 4. ダメージ計算
let damage = calculate_damage(&input, &enemy)?;
```

## 6. テスト計画

### ユニットテスト (core)

- `apply_enemy_debuffs` 基本動作: 耐性減少適用
- `apply_enemy_debuffs` DEF減少適用
- `apply_enemy_debuffs` 元素不一致時スキップ（炎デバフ → 氷攻撃で無効）
- `apply_enemy_debuffs` 複数デバフ累積（VV + Zhongli で加算）
- `apply_enemy_debuffs` 負の耐性（下限なし確認）
- `apply_enemy_debuffs` DEF減少上限クランプ (1.0)
- `apply_enemy_debuffs` DEF減少は既存値に加算確認
- `apply_enemy_debuffs` 空のバフリスト → 変更なし
- `apply_enemy_debuffs` 味方バフ（AtkPercent等）混在時に無視確認
- `apply_enemy_debuffs` 物理攻撃時: Zhongli の PhysicalResReduction 適用、ElementalResReduction 無視
- `superconduct_debuff` 値検証

### ユニットテスト (data)

- Zhongli TalentBuffDef: 8エントリ存在、全て値0.20
- Chevreuse TalentBuffDef: 既存ATK + 2つの耐性シュレッド
- Lisa TalentBuffDef: DefReduction 0.15
- Faruzan TalentBuffDef: 既存風ダメ + 耐性シュレッド0.30（scales_with_talent: false）

### 統合テスト

- Zhongli + 翠緑4pc併用: 耐性累積減少 → ダメージ増加確認
- 超電導 + 物理攻撃: 物理耐性-40% → ダメージ検証
- Lisa DEF減少 + 既存DEF減少: 累積適用確認
- Zhongli + 物理攻撃: PhysicalResReduction のみ適用、ElementalResReduction 無視

### Goldenテスト

- 耐性10%の敵に-20%シュレッド → 耐性-10% → multiplier 1.05
- 耐性10%の敵に-60%シュレッド → 耐性-50% → multiplier 1.25
- 耐性70%の敵に-40%シュレッド → 耐性30% → multiplier 0.70

## 7. 影響範囲

### Core crate

| ファイル | 変更内容 |
|----------|---------|
| `buff_types.rs` | `BuffableStat::DefReduction` 追加 |
| `enemy.rs` | `apply_enemy_debuffs` 関数 + `superconduct_debuff` ヘルパー追加 |
| `lib.rs` | 新関数の re-export |

### Data crate

| ファイル | 変更内容 |
|----------|---------|
| `talent_buffs.rs` | Zhongli(8エントリ), Chevreuse(+2), Lisa(1), Faruzan(+1) のTalentBuffDef追加 |

### 変更しないもの

- `Enemy` 構造体 — 変更なし
- `BuffTarget` enum — `Enemy` variant は追加しない
- `resistance_multiplier` 関数 — 変更なし
- `calculate_damage` / `calculate_transformative` / `calculate_lunar` — 変更なし
- 既存テスト — 影響なし（新関数は追加のみ）

### 既存テストへの影響

- `BuffableStat` の serde roundtrip テスト: `DefReduction` 追加分
- `test_find_nonexistent_character` テスト: `zhongli` を別のキャラID に変更（Zhongli に天賦バフが追加されるため）
- `buff.rs` の re-export: `BuffableStat` enum 全体が再エクスポート済みのため変更不要
- 既存のダメージ計算テストには影響なし
