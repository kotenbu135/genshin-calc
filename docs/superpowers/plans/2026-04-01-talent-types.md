# タレントデータ TypeScript 型定義追加 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** `find_character()` の戻り値を正確に型付けするTS型定義を追加し、UI開発者がタレントデータと `DamageType` の対応を型から把握できるようにする

**Architecture:** 既存の `CharacterData`（GOOD import用簡略版）は変更せず、`find_character()` 戻り値用の `CharacterFullData` インターフェースを新設。タレント関連型 (`TalentScaling`, `NormalAttackData`, `TalentData`, `TalentSet`) と補助型 (`Rarity`, `Region`, `ConstellationPattern`, `AscensionStat`) を追加。

**Tech Stack:** TypeScript（型定義のみ、ランタイムコードなし）

**Spec:** `docs/superpowers/specs/2026-04-01-talent-types-design.md`

---

### Task 1: 補助型の追加

**Files:**
- Modify: `crates/wasm/ts/types.ts` (Core Types セクション末尾、L21付近に追加)

- [ ] **Step 1: `Rarity`, `Region`, `ConstellationPattern`, `AscensionStat` 型を追加**

`types.ts` の Core Types セクション（`export type WeaponType = ...` の後）に以下を追加:

```typescript
export type Rarity = "Star1" | "Star2" | "Star3" | "Star4" | "Star5";

export type Region =
  | "Mondstadt" | "Liyue" | "Inazuma" | "Sumeru"
  | "Fontaine" | "Natlan" | "Snezhnaya" | "NodKrai" | "Other";

export type ConstellationPattern = "C3SkillC5Burst" | "C3BurstC5Skill";

/** serde externally-tagged format. 例: { "CritDmg": 0.384 }, { "ElementalDmgBonus": ["Pyro", 0.288] } */
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

- [ ] **Step 2: コミット**

```bash
git add crates/wasm/ts/types.ts
git commit -m "feat(wasm): add Rarity, Region, ConstellationPattern, AscensionStat TS types"
```

---

### Task 2: タレント関連型の追加

**Files:**
- Modify: `crates/wasm/ts/types.ts` (Task 1 で追加した型の後に追加)

- [ ] **Step 1: `TalentScaling`, `NormalAttackData`, `TalentData`, `TalentSet` を追加**

Task 1 で追加した型の後に以下を追加:

```typescript
// === Talent Types ===

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

- [ ] **Step 2: コミット**

```bash
git add crates/wasm/ts/types.ts
git commit -m "feat(wasm): add TalentScaling, NormalAttackData, TalentData, TalentSet TS types"
```

---

### Task 3: CharacterFullData の追加

**Files:**
- Modify: `crates/wasm/ts/types.ts` (タレント型の後、既存 `CharacterData` の前付近に追加)

- [ ] **Step 1: `CharacterFullData` インターフェースを追加**

タレント型セクションの後に以下を追加:

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

- [ ] **Step 2: コミット**

```bash
git add crates/wasm/ts/types.ts
git commit -m "feat(wasm): add CharacterFullData interface for find_character() return type"
```

---

### Task 4: 検証

- [ ] **Step 1: Rust側のビルド・テストが壊れていないことを確認**

```bash
cargo build
cargo test -p genshin-calc-wasm
```

Expected: 全パス（TS型定義はコンパイル対象外だが、ワークスペース全体の健全性確認）

- [ ] **Step 2: TypeScript型がRust型と一致することを手動確認**

`crates/data/src/types.rs` の `CharacterData` 構造体と `crates/wasm/ts/types.ts` の `CharacterFullData` を比較し、全フィールドが対応していることを確認。

- [ ] **Step 3: specとplanの削除・コミット**

```bash
git rm docs/superpowers/specs/2026-04-01-talent-types-design.md
git rm docs/superpowers/plans/2026-04-01-talent-types.md
git commit -m "docs: remove talent types spec and plan (implementation complete)"
```
