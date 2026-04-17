// genshin-calc-wasm TypeScript type definitions
// These types describe the JSON shapes used by the WASM API.
// Import or reference these when calling WASM functions from TypeScript.

// === Core Types ===

export interface Stats {
  hp: number;
  atk: number;
  def: number;
  elemental_mastery: number;
  crit_rate: number;
  crit_dmg: number;
  energy_recharge: number;
  dmg_bonus: number;
}

export type Element = "Pyro" | "Hydro" | "Electro" | "Cryo" | "Anemo" | "Geo" | "Dendro";
export type ScalingStat = "Atk" | "Hp" | "Def" | "Em";
export type DamageType = "Normal" | "Charged" | "Plunging" | "Skill" | "Burst";
export type WeaponType = "Sword" | "Claymore" | "Polearm" | "Bow" | "Catalyst";

export type Rarity = "Star1" | "Star2" | "Star3" | "Star4" | "Star5";

export type Region =
  | "Mondstadt" | "Liyue" | "Inazuma" | "Sumeru"
  | "Fontaine" | "Natlan" | "Snezhnaya" | "NodKrai" | "Other";

export type ConstellationPattern =
  | "C3SkillC5Burst"
  | "C3BurstC5Skill"
  | "C3NormalC5Burst"
  | "C3BurstC5Normal"
  | "C3NormalC5Skill";

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

// Reaction: simple variants are strings, Swirl is an object with the swirled element
export type Reaction =
  | "Vaporize" | "Melt"
  | "Aggravate" | "Spread"
  | "Overloaded" | "Superconduct" | "ElectroCharged" | "Shattered"
  | { Swirl: Element }
  | "Bloom" | "Hyperbloom" | "Burgeon" | "Burning"
  | "LunarElectroCharged" | "LunarBloom" | "LunarCrystallize" | "LunarCrystallizeSecondary";

export type BuffTarget = "OnlySelf" | "Team" | "TeamExcludeSelf";

// Object-shaped newtype variants
type ElementalDmgBonusStat = { ElementalDmgBonus: Element };
type ElementalResStat = { ElementalRes: Element };
type ElementalResReductionStat = { ElementalResReduction: Element };
type ReactionDmgBonusStat = { ReactionDmgBonus: Reaction };

export type BuffableStat =
  | "HpPercent" | "AtkPercent" | "DefPercent"
  | "HpFlat" | "AtkFlat" | "DefFlat"
  | "CritRate" | "CritDmg"
  | "ElementalMastery" | "EnergyRecharge"
  | "DmgBonus"
  | ElementalDmgBonusStat
  | ReactionDmgBonusStat
  | "PhysicalDmgBonus"
  | "NormalAtkDmgBonus" | "ChargedAtkDmgBonus" | "PlungingAtkDmgBonus"
  | "SkillDmgBonus" | "BurstDmgBonus"
  | "HealingBonus" | "ShieldStrength"
  | "AmplifyingBonus" | "TransformativeBonus" | "AdditiveBonus"
  | ElementalResStat
  | ElementalResReductionStat
  | "PhysicalResReduction"
  | "DefReduction"
  | "NormalAtkFlatDmg" | "ChargedAtkFlatDmg" | "PlungingAtkFlatDmg"
  | "SkillFlatDmg" | "BurstFlatDmg"
  | "DefPercentRaw";

// === Input/Output Types ===

export interface Enemy {
  level: number;
  resistance: number;
  def_reduction: number;
}

export interface DamageInput {
  character_level: number;
  stats: Stats;
  talent_multiplier: number;
  scaling_stat: ScalingStat;
  damage_type: DamageType;
  element: Element | null;
  reaction: Reaction | null;
  reaction_bonus: number;
  flat_dmg: number;
}

export interface DamageResult {
  non_crit: number;
  crit: number;
  average: number;
  reaction: Reaction | null;
}

export interface TransformativeInput {
  character_level: number;
  elemental_mastery: number;
  reaction: Reaction;
  reaction_bonus: number;
}

export interface TransformativeResult {
  damage: number;
  damage_element: Element | null;
}

export interface LunarInput {
  character_level: number;
  elemental_mastery: number;
  reaction: Reaction;
  reaction_bonus: number;
  crit_rate: number;
  crit_dmg: number;
  base_dmg_bonus: number;
}

export interface LunarResult {
  non_crit: number;
  crit: number;
  average: number;
  damage_element: Element | null;
}

// === Team Types ===

export interface StatProfile {
  base_hp: number;
  base_atk: number;
  base_def: number;
  hp_percent: number;
  atk_percent: number;
  def_percent: number;
  hp_flat: number;
  atk_flat: number;
  def_flat: number;
  elemental_mastery: number;
  crit_rate: number;
  crit_dmg: number;
  energy_recharge: number;
  dmg_bonus: number;
}

export interface ResolvedBuff {
  source: string;
  stat: BuffableStat;
  value: number;
  target: BuffTarget;
}

export interface MoonsignBenedictionSpec {
  enabled_reactions: Reaction[];
  scaling_stat: ScalingStat | null;
  rate: number;
  max_bonus: number;
}

export interface TeamMember {
  element: Element;
  weapon_type: WeaponType;
  stats: StatProfile;
  buffs_provided: ResolvedBuff[];
  is_moonsign: boolean;
  can_nightsoul: boolean;
  moonsign_benediction?: MoonsignBenedictionSpec | null;
}

export type MoonsignLevel = "None" | "NascentGleam" | "AscendantGleam";

/**
 * Team-level Moonsign context, exposed via `resolve_team_stats` / `TeamResolveResult`.
 * Use `base_dmg_bonus_by_reaction` to obtain the per-reaction Base DMG Bonus
 * that must be passed to `LunarInput.base_dmg_bonus` / `DirectLunarInput.base_dmg_bonus`.
 * Use `non_moonsign_lunar_bonus` as `reaction_bonus` for non-moonsign members at AscendantGleam.
 */
export interface MoonsignContext {
  level: MoonsignLevel;
  base_dmg_bonus_by_reaction: Array<[Reaction, number]>;
  non_moonsign_lunar_bonus: number;
  // talent_enhancements contains `&'static str` fields (Rust-only); serialized
  // form is available but not deserializable.
  talent_enhancements: unknown[];
}

// === GOOD Import Types ===

export interface GoodImport {
  source: string;
  version: number;
  builds: CharacterBuild[];
  warnings: ImportWarning[];
}

export interface CharacterBuild {
  character: CharacterData;
  level: number;
  ascension: number;
  constellation: number;
  talent_levels: [number, number, number];
  weapon: WeaponBuild | null;
  artifacts: ArtifactsBuild;
}

export interface WeaponBuild {
  weapon: WeaponData;
  level: number;
  refinement: number;
}

export interface ArtifactsBuild {
  sets: ArtifactSetData[];
  stats: StatProfile;
}

export interface ImportWarning {
  kind: string;
  message: string;
}

// === Talent Types ===

/**
 * ダメージパイプラインの振り分け。
 * `"Standard"` は `calculate_damage` 経由の通常天賦ダメージ。
 * `{ DirectLunar: Reaction }` は `calculate_direct_lunar` 経由の月反応ダメージ
 * （コロンビーナ・フリンス中盤/終盤・ラウマホールド2段・ジバイ月結晶等）。
 */
export type DamagePipeline = "Standard" | { DirectLunar: Reaction };

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
  /** ダメージ計算のパイプライン。未指定なら `"Standard"` 相当で扱う。 */
  damage_pipeline: DamagePipeline;
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

/**
 * パッシブ/星座由来の直接月反応スケーリングの発動ゲート。
 * - `"PassiveA1"`: A1パッシブ解放（キャラLv≧40）
 * - `"PassiveA4"`: A4パッシブ解放（4★はLv≧60、5★はLv≧70）
 * - `{ Constellation: number }`: 星座Lv≧N
 */
export type ScalingActivationGate =
  | "PassiveA1"
  | "PassiveA4"
  | { Constellation: number };

/**
 * パッシブ/星座ゲート付きの直接月反応スケーリング。
 * 天賦Lv非依存の固定倍率を持ち、`calculate_direct_lunar` 経由で計算される。
 *
 * `replaces` が空でない場合、consumer は同名の `TalentScaling` エントリを
 * 通常ヒットリストから除去し、この `PassiveScaling` で置き換える必要がある
 * （Nefer C6 の幻影演舞2段 ATK/EM 同時除去等）。
 */
export interface PassiveScaling {
  /** 表示名 */
  name: string;
  /** スケーリングに使うステータス */
  scaling_stat: ScalingStat;
  /** ダメージ元素 */
  damage_element: Element | null;
  /** Lv非依存固定倍率（小数形式、例: 65% = 0.65） */
  multiplier: number;
  /** 月反応種別（Lunar*） */
  reaction: Reaction;
  /** 発動ゲート（パッシブ/星座条件） */
  gate: ScalingActivationGate;
  /** 同キャラの既存 TalentScaling.name 配列。空なら追加のみ。 */
  replaces: string[];
}

/**
 * 既存の TalentScaling エントリに対する修飾子（A1/星座由来）。
 * `PassiveScaling` が「新規ヒット追加」なのに対し、こちらは「既存ヒットの修飾」。
 * - `AdditionalFlat`: scaling_stat × multiplier を当該ヒットの flat_dmg に加算
 * - `DirectLunarReactionBonus`: DirectLunar の reaction_bonus に加算
 * - `DirectLunarMultiplier`: DirectLunar の talent_multiplier に乗算
 */
export type ScalingModifierKind =
  | { AdditionalFlat: { scaling_stat: ScalingStat; multiplier: number } }
  | { DirectLunarReactionBonus: { bonus_ratio: number } }
  | { DirectLunarMultiplier: { factor: number } };

/** TalentScaling 修飾子（issue #141）。 */
export interface ScalingModifier {
  /** 表示名（例: "月下に舞い降りる天女"） */
  name: string;
  /** 同キャラの既存 TalentScaling.name 配列。一致するヒットに修飾を適用。 */
  targets: string[];
  /** 発動ゲート（パッシブ/星座条件） */
  gate: ScalingActivationGate;
  /** 修飾種別 */
  kind: ScalingModifierKind;
}

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
  /** パッシブ/星座ゲート付き直接月反応スケーリング。該当効果なしなら []。 */
  passive_scalings: PassiveScaling[];
  /** 既存 TalentScaling への修飾子。該当効果なしなら []。 */
  scaling_modifiers: ScalingModifier[];
}

/** GOOD import用の簡略キャラクターデータ（CharacterBuild.character）。find_character() の戻り値は CharacterFullData を参照 */
export interface CharacterData {
  id: string;
  name: string;
  element: Element;
  weapon_type: WeaponType;
  rarity: number;
  base_hp: number;
  base_atk: number;
  base_def: number;
  ascension_stat: string;
  ascension_stat_value: number;
}

export interface WeaponData {
  id: string;
  name: string;
  weapon_type: WeaponType;
  rarity: number;
  base_atk: number;
  sub_stat: string | null;
  sub_stat_value: number;
}

export interface ArtifactSetData {
  id: string;
  name: string;
}


/**
 * Return type of `resolve_team_stats`. Includes the team-level Moonsign context
 * (Issue #142) so JS/TS consumers can obtain `base_dmg_bonus_by_reaction` to
 * feed into `LunarInput.base_dmg_bonus` / `DirectLunarInput.base_dmg_bonus`.
 */
export interface TeamResolveResult {
  base_stats: Stats;
  applied_buffs: ResolvedBuff[];
  resonances: string[];
  final_stats: Stats;
  damage_context: unknown;
  enemy_debuffs: unknown;
  moonsign_context: MoonsignContext;
}
