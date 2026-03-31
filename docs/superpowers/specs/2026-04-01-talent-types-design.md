# 設計: タレントデータ TypeScript 型定義追加

> Date: 2026-04-01
> Scope: `crates/wasm/ts/types.ts`
> Status: Draft

## 背景

`find_character()` WASM関数はRust側の `CharacterData`（`TalentSet` 含む）をフルにシリアライズして返す。しかし `types.ts` の `CharacterData` インターフェースはGOOD import用の簡略版であり、`find_character()` の戻り値とはフィールド構成が異なる。タレントデータの型定義もなく、UI開発者が戻り値の構造を把握しにくい。

Rust側の `NormalAttackData` は既に `hits`（通常攻撃）、`charged`（重撃）、`plunging`（落下攻撃）を別配列で保持しており、`DamageType` との対応は構造的に明確。問題はTS型定義の不足のみ。

## 既存 `CharacterData` との型不一致

既存の `types.ts` の `CharacterData` はGOOD import経由（`CharacterBuild.character`）で使用される簡略版。`find_character()` が返すフルのRust `CharacterData` とは以下のフィールドが異なる:

| フィールド | 既存TS型 | Rust型（find_character戻り値） |
|-----------|---------|-------------------------------|
| `rarity` | `number` | `Rarity` enum (`"Star1"` ... `"Star5"`) |
| `base_hp` | `number` | `[number, number, number, number]`（突破段階別） |
| `base_atk` | `number` | `[number, number, number, number]` |
| `base_def` | `number` | `[number, number, number, number]` |
| `ascension_stat` | `string` | `AscensionStat`（タグ付きunion） |
| `region` | なし | `Region` enum |
| `constellation_pattern` | なし | `ConstellationPattern` enum |
| `talents` | なし | `TalentSet` |

## 設計

### 方針

既存の `CharacterData`（GOOD import用）はそのまま維持し、`find_character()` の戻り値用に新インターフェース `CharacterFullData` を導入する。

### 追加する型

`types.ts` に以下のインターフェースを追加する。

#### タレント関連型

```typescript
/** 個別のスケーリング項目（1段ダメージ、重撃ダメージ等） */
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

/**
 * 通常攻撃データ。各配列が DamageType に対応:
 * - hits → "Normal"
 * - charged → "Charged"
 * - plunging → "Plunging"
 */
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

/** 元素スキル/元素爆発データ */
export interface TalentData {
  /** 天賦名（例: "蝶導来世"） */
  name: string;
  /** スケーリング項目 */
  scalings: TalentScaling[];
}

/** キャラクターの天賦セット */
export interface TalentSet {
  /** 通常攻撃（DamageType: "Normal" / "Charged" / "Plunging"） */
  normal_attack: NormalAttackData;
  /** 元素スキル（DamageType: "Skill"） */
  elemental_skill: TalentData;
  /** 元素爆発（DamageType: "Burst"） */
  elemental_burst: TalentData;
}
```

Rust型との対応: `&'static str` → `string`、`Option<Element>` → `Element | null`、`[f64; 15]` → `number[]`。

#### キャラクター関連型

```typescript
export type Rarity = "Star1" | "Star2" | "Star3" | "Star4" | "Star5";

export type Region =
  | "Mondstadt" | "Liyue" | "Inazuma" | "Sumeru"
  | "Fontaine" | "Natlan" | "Snezhnaya" | "NodKrai" | "Other";

export type ConstellationPattern = "C3SkillC5Burst" | "C3BurstC5Skill";

/** AscensionStat: serdeタグ付きunion。例: { "CritDmg": 0.384 }, { "ElementalDmgBonus": ["Pyro", 0.288] } */
export type AscensionStat =
  | { Hp: number }
  | { Atk: number }
  | { Def: number }
  | { CritRate: number }
  | { CritDmg: number }
  | { ElementalMastery: number }
  | { EnergyRecharge: number }
  | { ElementalDmgBonus: [Element, number] }
  | { PhysicalDmgBonus: number }
  | { HealingBonus: number };
```

#### CharacterFullData

`find_character()` の戻り値を正確に型付けする新インターフェース。

```typescript
/** find_character() の戻り値。フルのキャラクターデータ */
export interface CharacterFullData {
  id: string;
  name: string;
  element: Element;
  weapon_type: WeaponType;
  rarity: Rarity;
  region: Region;
  /** 基礎HP [Lv1, Lv20, Lv80, Lv90] */
  base_hp: [number, number, number, number];
  /** 基礎攻撃力 [Lv1, Lv20, Lv80, Lv90] */
  base_atk: [number, number, number, number];
  /** 基礎防御力 [Lv1, Lv20, Lv80, Lv90] */
  base_def: [number, number, number, number];
  ascension_stat: AscensionStat;
  talents: TalentSet;
  constellation_pattern: ConstellationPattern;
}
```

### 既存型への変更

既存の `CharacterData` インターフェースは変更しない。GOOD import用の簡略版としてそのまま維持。

## 変更対象

| ファイル | 変更内容 |
|---------|---------|
| `crates/wasm/ts/types.ts` | `TalentScaling`, `NormalAttackData`, `TalentData`, `TalentSet`, `Rarity`, `Region`, `ConstellationPattern`, `AscensionStat`, `CharacterFullData` を追加 |

## 変更しないもの

- Rust側の型定義（`crates/data/src/types.rs`）
- Rust側のキャラクターデータ
- WASM関数（`find_character()` 等）
- 既存のTS型（`CharacterData`, `DamageType`, `ScalingStat`, `Element` 等）

## Rust型との対応表

| Rust (`crates/data/src/types.rs`) | TypeScript (`types.ts`) |
|-----------------------------------|------------------------|
| `TalentScaling` | `TalentScaling` |
| `NormalAttackData` | `NormalAttackData` |
| `TalentData` | `TalentData` |
| `TalentSet` | `TalentSet` |
| `Rarity` | `Rarity` |
| `Region` | `Region` |
| `ConstellationPattern` | `ConstellationPattern` |
| `AscensionStat` | `AscensionStat` |
| `CharacterData`（フル） | `CharacterFullData` |
| `CharacterData`（GOOD import簡略版） | `CharacterData`（既存、変更なし） |

## UI側での使用例

```typescript
import { find_character, calculate_damage } from "genshin-calc-wasm";
import type { CharacterFullData, DamageType } from "genshin-calc-wasm/types";

const character: CharacterFullData = find_character("hu_tao");

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
