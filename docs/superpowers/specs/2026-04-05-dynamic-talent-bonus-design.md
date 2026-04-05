# Dynamic Talent Bonus Design

Issue: #36 — 動的天賦倍率ボーナス（闘志・願力等）を calculate_damage に反映する

## Problem

`find_character()` が返す `TalentScaling.values` は基本倍率のみで、スタック/ゲージ消費に応じた天賦倍率加算が反映されていない。

| キャラ | メカニクス | 基本倍率 (Lv10) | MAX時の本来の倍率 |
|--------|-----------|-----------------|-------------------|
| マーヴィカ | 闘志(0-200) × 2.9%/pt | 800.64% | 1,380.64% (+580%) |
| 雷電将軍 | 願力(0-60) × 7.0%/stack 初撃 | 721.44% | 1,141.44% (+420%) |
| 雷電将軍 | 願力(0-60) × 1.31%/stack 通常/重撃 | 79.82% (1段) | 158.42% (+78.6%) |

## Approach: Data Extension Only (Approach A)

`TalentScaling` に `dynamic_bonus` フィールドを追加。`DamageInput` / `calculate_damage` は変更しない。フロントエンドがボーナス情報を読み取り、調整済み倍率を `DamageInput.talent_multiplier` に渡す。

### Why This Approach

- core の計算ロジックは純粋なまま（ゲーム固有概念を持ち込まない）
- `DamageInput` 変更は全構築箇所に影響するため回避
- 各 `TalentScaling` ごとにボーナスを定義でき、雷電のように初撃と通常で異なる per_stack 値を持つケースに対応

## Data Model

### New Types (crates/data/src/types.rs)

```rust
/// 動的天賦ボーナス（スタック/ゲージ消費で天賦倍率に加算される効果）
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DynamicTalentBonus {
    /// ボーナス名（"闘志", "諸願百目の輪"等）
    pub name: &'static str,
    /// 最大スタック/ゲージ数
    pub max_stacks: u16,
    /// 天賦レベル別のスタック1あたりの加算倍率（小数形式）
    /// per_stack[talent_level - 1] × stacks を基本倍率に加算
    pub per_stack: [f64; 15],
}
```

### TalentScaling Extension

```rust
pub struct TalentScaling {
    pub name: &'static str,
    pub scaling_stat: ScalingStat,
    pub damage_element: Option<Element>,
    pub values: [f64; 15],
    /// 動的天賦ボーナス。Noneなら静的倍率のみ。
    pub dynamic_bonus: Option<&'static DynamicTalentBonus>,
}
```

- `Option<&'static DynamicTalentBonus>`: 既存キャラは全て `None`
- `&'static` 参照型なので `Serialize` のみ（Deserialize なし、conventions 通り）

## Character Data

### Mavuika (crates/data/src/characters/pyro/mavuika.rs)

```rust
static MAVUIKA_BURST_FIGHTING_SPIRIT: DynamicTalentBonus = DynamicTalentBonus {
    name: "闘志",
    max_stacks: 200,
    per_stack: [/* Lv1-15: Honey Impact から取得。Lv10 = 0.029 */],
};

const MAVUIKA_BURST: TalentScaling = TalentScaling {
    // ...existing fields...
    dynamic_bonus: Some(&MAVUIKA_BURST_FIGHTING_SPIRIT),
};
```

### Raiden Shogun (crates/data/src/characters/electro/raiden_shogun.rs)

2 つの `DynamicTalentBonus` を定義:

1. **RAIDEN_RESOLVE_MUSOU**: 初撃（夢想の一太刀）用。Lv10 = 0.070/stack
2. **RAIDEN_RESOLVE_NORMAL**: 一心通常/重撃用。Lv10 = 0.0131/stack

```rust
// 初撃
RAIDEN_BURST_MUSOU.dynamic_bonus = Some(&RAIDEN_RESOLVE_MUSOU);
// 通常/重撃 (N1-N5, Charged1, Charged2)
RAIDEN_BURST_N1.dynamic_bonus = Some(&RAIDEN_RESOLVE_NORMAL);
// ... N2-N5, Charged も同様
```

### Existing Characters

全既存 `TalentScaling` に `dynamic_bonus: None` を追加。

## Frontend Usage

`find_character()` の JSON レスポンスに `dynamic_bonus` が含まれる:

```json
{
  "name": "スキルダメージ",
  "values": [4.448, ...],
  "dynamic_bonus": {
    "name": "闘志",
    "max_stacks": 200,
    "per_stack": [0.016, ...]
  }
}
```

フロントの計算:

```
adjusted = values[talentLevel - 1] + stacks * per_stack[talentLevel - 1]
```

`dynamic_bonus` が `null` なら従来通り `values[talentLevel - 1]` のみ。

## Testing

1. **単体テスト**: `DynamicTalentBonus` のシリアライズ、`None` との互換性
2. **データ駆動テスト (TOML)**: stacks パラメータ追加
   - マーヴィカ: 闘志200, Lv10 → `8.0064 + 200 × 0.029 = 13.8064`
   - 雷電初撃: 願力60, Lv10 → `7.2144 + 60 × 0.070 = 11.4144`
3. **既存テスト**: `dynamic_bonus: None` 追加で全テストパス確認

## Scope

### In Scope

- `DynamicTalentBonus` 型定義
- `TalentScaling.dynamic_bonus` フィールド追加
- マーヴィカ・雷電将軍のデータ定義
- 全既存 `TalentScaling` に `None` 追加
- テスト

### Out of Scope

- `DamageInput` / `calculate_damage` の変更
- WASM API の追加関数
- `ConditionalBuff` システムの変更
- 要確認キャラ（ナヴィア・スカーク・リネ）→ 別 Issue で調査
