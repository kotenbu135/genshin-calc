# Talent Lookup API + Constellation Level Boost Design

## Overview

天賦倍率ルックアップAPIと凸3/凸5レベル+3効果のモデル化。全102キャラの天賦倍率テーブル（Lv1-15）は既に`data` crateに実装済み。本設計はその上に「凸ブーストデータ」「lookup API」「DamageInputビルダー」を追加する。

## Motivation

現状ユーザーは`DamageInput.talent_multiplier`に手動で倍率を入力する必要がある。キャラデータから自動的に倍率を引けるAPIがあれば、使いやすさが大幅に向上する。また凸3/凸5のレベル+3効果は原神の基本的な凸仕様であり、正確な計算に必須。

## Scope

### In Scope
- Phase 1: 凸ブーストデータ構造 + `CharacterData`メソッドによるlookup API
- Phase 2: `CharacterData::build_damage_input()`によるDamageInput構築ヘルパー
- 初期対象: 検証済み5キャラ（Freminet, Diluc, Ganyu, Raiden, Yanfei）

### Out of Scope
- 凸による倍率変更・追加ダメージ（条件付きバフ系）
- パッシブ効果のモデル化
- スキルヒット数・ICD・元素付着のシミュレーション

## Phase 1: Constellation Data + Lookup API

### New Types (`crates/data/src/types.rs`)

```rust
/// Constellation pattern for talent level boosts.
///
/// In Genshin Impact, every character's C3 boosts one of Skill/Burst by +3 levels,
/// and C5 boosts the other. This enum encodes the two possible patterns,
/// eliminating invalid states by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstellationPattern {
    /// C3 boosts Elemental Skill +3, C5 boosts Elemental Burst +3.
    C3SkillC5Burst,
    /// C3 boosts Elemental Burst +3, C5 boosts Elemental Skill +3.
    C3BurstC5Skill,
}
```

### CharacterData Changes

```rust
pub struct CharacterData {
    // ... existing fields ...
    pub talents: TalentSet,
    /// Constellation pattern for talent level boosts (C3 and C5).
    pub constellation_pattern: ConstellationPattern,
}
```

### Lookup Methods on CharacterData

`DamageType` (既存の`core`型: Normal, Charged, Plunging, Skill, Burst) を天賦カテゴリの指定にも再利用する。新しい`TalentType` enumは追加しない（意味的に同一のため）。

```rust
impl CharacterData {
    /// Returns the effective talent level after constellation boosts.
    ///
    /// - Normal/Charged/Plunging: no boost (returns talent_level as-is)
    /// - Skill: +3 if constellation >= 3 and C3 targets Skill,
    ///          or >= 5 and C5 targets Skill
    /// - Burst: +3 if constellation >= 3 and C3 targets Burst,
    ///          or >= 5 and C5 targets Burst
    /// - Capped at 15.
    ///
    /// `talent_level` accepts 1-15. In gameplay, base levels are 1-10,
    /// but 1-15 is accepted for testing flexibility.
    pub fn effective_talent_level(
        &self,
        damage_type: DamageType,
        talent_level: u8,
        constellation: u8,
    ) -> u8

    /// Returns the talent multiplier with constellation boost applied.
    ///
    /// - `damage_type`: which talent category (Normal, Charged, Plunging, Skill, Burst)
    /// - `hit_index`: 0-based index (e.g. normal attack 1st hit = 0)
    /// - `talent_level`: base talent level (1-15)
    /// - `constellation`: constellation level (0-6)
    ///
    /// Returns `None` if `hit_index` is out of range.
    pub fn talent_multiplier(
        &self,
        damage_type: DamageType,
        hit_index: usize,
        talent_level: u8,
        constellation: u8,
    ) -> Option<f64>

    /// Returns the TalentScaling entry for a specific talent hit.
    ///
    /// Useful for inspecting scaling_stat, damage_element, name.
    /// Returns `None` if `hit_index` is out of range.
    pub fn talent_scaling(
        &self,
        damage_type: DamageType,
        hit_index: usize,
    ) -> Option<&TalentScaling>
}
```

### Internal Logic

`talent_multiplier` delegates to:
1. `talent_scaling()` to get the `TalentScaling` entry
2. `effective_talent_level()` to apply constellation boost
3. Index into `values[effective_level - 1]`

`effective_talent_level` logic:
```
match (damage_type, constellation_pattern):
    (Skill, C3SkillC5Burst) if constellation >= 3 => min(talent_level + 3, 15)
    (Skill, C3BurstC5Skill) if constellation >= 5 => min(talent_level + 3, 15)
    (Burst, C3SkillC5Burst) if constellation >= 5 => min(talent_level + 3, 15)
    (Burst, C3BurstC5Skill) if constellation >= 3 => min(talent_level + 3, 15)
    _ => talent_level
```

`talent_scaling` maps DamageType to NormalAttackData fields:
```
Normal   → talents.normal_attack.hits[hit_index]
Charged  → talents.normal_attack.charged[hit_index]
Plunging → talents.normal_attack.plunging[hit_index]
Skill    → talents.elemental_skill.scalings[hit_index]
Burst    → talents.elemental_burst.scalings[hit_index]
```

### Character Data Updates

102 characters across 7 files (`crates/data/src/characters/*.rs`) get `constellation_pattern` added. Example:

```rust
pub const DILUC: CharacterData = CharacterData {
    // ... existing fields ...
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
```

Initial implementation: 5 verified characters (Freminet, Diluc, Ganyu, Raiden, Yanfei). Remaining 97 characters added incrementally.

### Existing Talent Buff Fix

`team_builder.rs`の`build()`メソッド内の既存talent buff計算（lines 146-154）も`effective_talent_level()`を使うよう更新する。例: C5 Bennettの爆発バフはLv10ベースでもLv13として計算される。

```rust
// Before (current):
let level = self.talent_levels[talent_idx];

// After:
let damage_type = match def.source {
    TalentBuffSource::ElementalSkill => DamageType::Skill,
    TalentBuffSource::ElementalBurst => DamageType::Burst,
    _ => DamageType::Normal,
};
let level = self.character.effective_talent_level(
    damage_type,
    self.talent_levels[talent_idx],
    self.constellation,
);
```

## Phase 2: DamageInput Builder

### Design Decision: Standalone Method on CharacterData

`TeamMemberBuilder`ではなく`CharacterData`にヘルパーメソッドを置く。理由：

- `TeamMemberBuilder::build()` → `resolve_team_stats()` → `Stats`の2段階解決が既存アーキテクチャ
- `DamageInput`にはチームバフ適用済みの`Stats`が必要
- `TeamMemberBuilder`内でソロStats計算すると不正確な値になる
- `CharacterData`のメソッドなら、事前に解決済みの`Stats`を受け取れる

### Method on CharacterData

```rust
impl CharacterData {
    /// Builds a DamageInput for a specific talent hit.
    ///
    /// Auto-fills from character talent data:
    /// - talent_multiplier (with constellation boost)
    /// - scaling_stat (from TalentScaling)
    /// - element (from TalentScaling.damage_element)
    /// - damage_type (from the damage_type parameter)
    ///
    /// User provides:
    /// - Pre-resolved Stats (from resolve_team_stats pipeline)
    /// - character_level, damage_type, hit_index
    /// - talent_level, constellation
    /// - reaction, reaction_bonus
    ///
    /// Returns `None` if hit_index is out of range.
    pub fn build_damage_input(
        &self,
        stats: Stats,
        character_level: u32,
        damage_type: DamageType,
        hit_index: usize,
        talent_level: u8,
        constellation: u8,
        reaction: Option<Reaction>,
        reaction_bonus: f64,
    ) -> Option<DamageInput>
}
```

### Internal Logic

1. `self.talent_multiplier(damage_type, hit_index, talent_level, constellation)` → 倍率取得
2. `self.talent_scaling(damage_type, hit_index)` → scaling_stat, damage_element取得
3. `DamageInput`組み立て:
   ```rust
   DamageInput {
       character_level,
       stats,
       talent_multiplier: multiplier,
       scaling_stat: scaling.scaling_stat,
       damage_type,
       element: scaling.damage_element,
       reaction,
       reaction_bonus,
   }
   ```

### Usage Example

```rust
// 1. Build team members
let diluc_builder = TeamMemberBuilder::new(diluc, wolfs_gravestone)
    .constellation(3)
    .talent_levels([9, 9, 9])
    .artifact_stats(artifact_stats);
let diluc_member = diluc_builder.build()?;

// 2. Resolve team stats
let team = vec![diluc_member, bennett_member, ...];
let resolved = resolve_team_stats(&team)?;
let diluc_stats = &resolved[0];

// 3. Build DamageInput from character data
let input = diluc.build_damage_input(
    diluc_stats.clone(),
    90,                         // character_level
    DamageType::Skill,          // elemental skill
    0,                          // 1st hit
    9,                          // talent level
    3,                          // constellation
    Some(Reaction::Vaporize),
    0.15,                       // reaction bonus
).unwrap();

// 4. Calculate damage
let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
let result = calculate_damage(&input, &enemy)?;
```

## Testing Strategy

### Phase 1 Tests

- **effective_talent_level**: all combinations of DamageType × constellation (0-6) × both patterns
- **talent_multiplier**: verified against known values for 5 characters
- **talent_scaling**: correct scaling_stat and damage_element returned
- **Out-of-range**: hit_index out of range returns `None`
- **Golden tests**: compare with hand-calculated values from game wiki
- **Data integrity**: all 102 characters have valid `constellation_pattern` (validation test)

### Phase 2 Tests

- **build_damage_input** produces correct `DamageInput` for Diluc skill
- **End-to-end**: `build_damage_input` → `calculate_damage` matches expected values
- **None case**: invalid hit_index returns `None`

### Existing Test Fix

- `build()`のtalent buff計算がconstellation boostを反映することを検証
- 既存テスト`test_bennett_burst_buff_at_lv13`は影響なし（constellation=0のため）

### Verified Characters

| Character | Element | Weapon | Constellation Pattern |
|-----------|---------|--------|-----------------------|
| Freminet  | Cryo    | Claymore | C3SkillC5Burst |
| Diluc     | Pyro    | Claymore | C3SkillC5Burst |
| Ganyu     | Cryo    | Bow      | C3BurstC5Skill |
| Raiden    | Electro | Polearm  | C3BurstC5Skill |
| Yanfei    | Pyro    | Catalyst | C3SkillC5Burst |

## File Changes

| File | Change |
|------|--------|
| `crates/data/src/types.rs` | Add `ConstellationPattern`; add `constellation_pattern` field to `CharacterData`; impl `effective_talent_level`, `talent_multiplier`, `talent_scaling`, `build_damage_input` methods |
| `crates/data/src/characters/pyro.rs` | Add `constellation_pattern` to pyro characters |
| `crates/data/src/characters/hydro.rs` | Add `constellation_pattern` to hydro characters |
| `crates/data/src/characters/electro.rs` | Add `constellation_pattern` to electro characters |
| `crates/data/src/characters/cryo.rs` | Add `constellation_pattern` to cryo characters |
| `crates/data/src/characters/dendro.rs` | Add `constellation_pattern` to dendro characters |
| `crates/data/src/characters/anemo.rs` | Add `constellation_pattern` to anemo characters |
| `crates/data/src/characters/geo.rs` | Add `constellation_pattern` to geo characters |
| `crates/data/src/team_builder.rs` | Update `build()` to use `effective_talent_level()` for talent buff scaling |
| `crates/core` | No changes |

## Breaking Changes

- `CharacterData`に`constellation_pattern`フィールドが追加される
- 全102キャラの`const`定義を更新する必要あり
- `CharacterData`は`Serialize`のみ（`Deserialize`なし）のためserde互換性の問題はない
- 外部コードが`CharacterData`を手動構築している場合は更新が必要

## Risks

- **102キャラの凸データ正確性**: C3/C5のどちらがSkill/Burstかはキャラごとに異なる。Wiki等で要確認
- **ConstellationPattern**: enumで無効状態を排除済み（c3とc5が同じ値にならない）
- **talent_level範囲**: 1-15を受け付ける（ゲーム内は1-10だがテスト柔軟性のため）
