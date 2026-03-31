# 設計: タレントデータ TypeScript 型定義追加

> Date: 2026-04-01
> Scope: `crates/wasm/ts/types.ts`
> Status: Draft

## 背景

`find_character()` WASM関数はRust側の `CharacterData`（`TalentSet` 含む）をフルにシリアライズして返す。しかし `types.ts` の `CharacterData` インターフェースにはタレントデータの型定義がなく、UI開発者が戻り値の構造を把握しにくい。

Rust側の `NormalAttackData` は既に `hits`（通常攻撃）、`charged`（重撃）、`plunging`（落下攻撃）を別配列で保持しており、`DamageType` との対応は構造的に明確。問題はTS型定義の不足のみ。

## 設計

### 追加する型

`types.ts` に以下のインターフェースを追加する。

#### TalentScaling

個別のスケーリング項目（1段ダメージ、重撃ダメージ等）。

```typescript
export interface TalentScaling {
  /** スケーリング名（例: "1段ダメージ"） */
  name: string;
  /** スケーリングに使うステータス */
  scaling_stat: ScalingStat;
  /** ダメージ元素。null = 物理 */
  damage_element: Element | null;
  /** 天賦Lv1-15の倍率値 */
  values: number[];
}
```

Rust側の `TalentScaling` と1:1対応。`&'static str` → `string`、`Option<Element>` → `Element | null`、`[f64; 15]` → `number[]`。

#### NormalAttackData

通常攻撃データ。各配列が `DamageType` に対応する。

```typescript
export interface NormalAttackData {
  /** 攻撃名（例: "往生秘伝槍法"） */
  name: string;
  /** 通常攻撃コンボ（DamageType: "Normal"） */
  hits: TalentScaling[];
  /** 重撃（DamageType: "Charged"） */
  charged: TalentScaling[];
  /** 落下攻撃（DamageType: "Plunging"） */
  plunging: TalentScaling[];
}
```

JSDocコメントで各配列と `DamageType` の対応を明記。

#### TalentData

元素スキル/元素爆発データ。

```typescript
export interface TalentData {
  /** 天賦名（例: "蝶導来世"） */
  name: string;
  /** スケーリング項目 */
  scalings: TalentScaling[];
}
```

#### TalentSet

キャラクターの天賦セット。

```typescript
export interface TalentSet {
  /** 通常攻撃（DamageType: "Normal" / "Charged" / "Plunging"） */
  normal_attack: NormalAttackData;
  /** 元素スキル（DamageType: "Skill"） */
  elemental_skill: TalentData;
  /** 元素爆発（DamageType: "Burst"） */
  elemental_burst: TalentData;
}
```

### CharacterData の変更

既存の `CharacterData` インターフェースに `talents` フィールドを追加。

```typescript
export interface CharacterData {
  // ... 既存フィールド（変更なし） ...
  talents: TalentSet;
}
```

## 変更対象

| ファイル | 変更内容 |
|---------|---------|
| `crates/wasm/ts/types.ts` | `TalentScaling`, `NormalAttackData`, `TalentData`, `TalentSet` 追加、`CharacterData` に `talents` 追加 |

## 変更しないもの

- Rust側の型定義（`crates/data/src/types.ts`）
- Rust側のキャラクターデータ
- WASM関数（`find_character()` 等）
- 既存のTS型（`DamageType`, `ScalingStat`, `Element` 等）

## Rust型との対応表

| Rust (`crates/data/src/types.rs`) | TypeScript (`types.ts`) |
|-----------------------------------|------------------------|
| `TalentScaling` | `TalentScaling` |
| `NormalAttackData` | `NormalAttackData` |
| `TalentData` | `TalentData` |
| `TalentSet` | `TalentSet` |
| `CharacterData.talents` | `CharacterData.talents` |

## UI側での使用例

```typescript
import { find_character, calculate_damage } from "genshin-calc-wasm";
import type { CharacterData, DamageType } from "genshin-calc-wasm/types";

const character: CharacterData = find_character("hu_tao");

// 通常攻撃1段目 → DamageType: "Normal"
const hit1 = character.talents.normal_attack.hits[0];
const input = {
  // ...
  talent_multiplier: hit1.values[talentLevel - 1],
  scaling_stat: hit1.scaling_stat,
  damage_type: "Normal" as DamageType,
  element: hit1.damage_element,
  // ...
};

// 重撃 → DamageType: "Charged"
const charged = character.talents.normal_attack.charged[0];
const chargedInput = {
  // ...
  damage_type: "Charged" as DamageType,
  // ...
};
```
