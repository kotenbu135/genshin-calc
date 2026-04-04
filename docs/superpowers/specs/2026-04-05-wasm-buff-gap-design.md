# WASM Buff Gap Resolution Design

## Problem

`resolve_team_stats` returns only `Stats`, discarding attack-type-specific DMG bonuses, flat DMG, enemy debuffs (resistance/DEF reduction), and reaction bonuses from team buffs. These `BuffableStat` variants are collected in `applied_buffs` but never surfaced to WASM consumers, making it impossible to calculate accurate damage for teams with characters like Citlali, Chiori, Shenhe, or Yun Jin.

19 existing or planned buff definitions are affected (see `docs/wasm-buff-gap-report.md`).

## Decision Log

| Question | Choice | Rationale |
|----------|--------|-----------|
| Rust/WASM only vs frontend processing | Rust/WASM only | Centralize calc logic, reduce frontend bug risk |
| Extend existing API vs new function | Extend `resolve_team_stats` to return `TeamResolveResult` | Root cause is Stats-only return; `resolve_team_stats_detailed` already returns `TeamResolveResult` internally |
| Context granularity | Flat struct per damage type | WASM-friendly (no HashMap), direct field access from JS |
| Enemy debuffs handling | `EnemyDebuffs` struct + `apply_team_debuffs` helper | Resistance formula has edge cases (negative res folding); Rust-side helper prevents frontend bugs |

## Architecture

### New Structs

#### `DamageContext` (in `crates/core/src/team.rs`)

Aggregates conditional buffs from `applied_buffs` that cannot be applied to `StatProfile`.

```rust
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DamageContext {
    // Attack-type DMG bonuses
    pub normal_atk_dmg_bonus: f64,
    pub charged_atk_dmg_bonus: f64,
    pub plunging_atk_dmg_bonus: f64,
    pub skill_dmg_bonus: f64,
    pub burst_dmg_bonus: f64,
    // Attack-type flat DMG
    pub normal_atk_flat_dmg: f64,
    pub charged_atk_flat_dmg: f64,
    pub plunging_atk_flat_dmg: f64,
    pub skill_flat_dmg: f64,
    pub burst_flat_dmg: f64,
    // Reaction bonuses
    pub amplifying_bonus: f64,
    pub transformative_bonus: f64,
    pub additive_bonus: f64,
}
```

Construction: `DamageContext::from_buffs(&[ResolvedBuff]) -> DamageContext` — single pass over `applied_buffs`, matching on `BuffableStat` variants.

#### `EnemyDebuffs` (in `crates/core/src/enemy.rs`)

Aggregates enemy debuffs from team buffs.

```rust
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct EnemyDebuffs {
    pub pyro_res_reduction: f64,
    pub hydro_res_reduction: f64,
    pub electro_res_reduction: f64,
    pub cryo_res_reduction: f64,
    pub dendro_res_reduction: f64,
    pub anemo_res_reduction: f64,
    pub geo_res_reduction: f64,
    pub physical_res_reduction: f64,
    pub def_reduction: f64,
}
```

Construction: `collect_enemy_debuffs(&[ResolvedBuff]) -> EnemyDebuffs` — single pass, same pattern as `DamageContext::from_buffs`.

Application: `apply_debuffs_to_enemy(&Enemy, &EnemyDebuffs, Option<Element>) -> Enemy` — selects the matching resistance reduction by element, applies to `enemy.resistance`, adds `def_reduction` (clamped to 1.0).

### Modified Structs

#### `TeamResolveResult` (in `crates/core/src/team.rs`)

```rust
pub struct TeamResolveResult {
    pub base_stats: Stats,
    pub applied_buffs: Vec<ResolvedBuff>,
    pub resonances: Vec<ElementalResonance>,
    pub final_stats: Stats,
    pub damage_context: DamageContext,    // NEW
    pub enemy_debuffs: EnemyDebuffs,      // NEW
}
```

### Modified Functions

#### `resolve_team_stats_detailed` (core)

After collecting `applied_buffs`, adds:
```rust
let damage_context = DamageContext::from_buffs(&applied_buffs);
let enemy_debuffs = collect_enemy_debuffs(&applied_buffs);
```

#### `resolve_team_stats` (core)

Changes return type from `Stats` to `TeamResolveResult`:
```rust
pub fn resolve_team_stats(team: &[TeamMember], target_index: usize) -> Result<TeamResolveResult, CalcError>
```

This is a **breaking change**. The function becomes a direct alias for `resolve_team_stats_detailed`.

#### `apply_enemy_debuffs` (core, internal refactor)

Refactored to use `collect_enemy_debuffs` + `apply_debuffs_to_enemy` internally. Public signature preserved for backward compatibility within core.

### New WASM Functions

#### `apply_team_debuffs(enemy, debuffs, element) -> Enemy`

```rust
#[wasm_bindgen]
pub fn apply_team_debuffs(enemy: JsValue, debuffs: JsValue, element: JsValue) -> Result<JsValue, JsError>
```

- `enemy`: `Enemy` JS object
- `debuffs`: `EnemyDebuffs` JS object (from `resolve_team_stats` result)
- `element`: element string (`"Pyro"` etc.) or `null` for physical
- Returns: new `Enemy` with debuffs applied

### WASM `resolve_team_stats` Change

Returns full `TeamResolveResult` instead of `Stats`:

```javascript
// Before
const stats = resolve_team_stats(members, 0);
// stats.atk, stats.crit_rate, ...

// After
const result = resolve_team_stats(members, 0);
// result.final_stats.atk, result.final_stats.crit_rate, ...
// result.damage_context.normal_atk_dmg_bonus, ...
// result.enemy_debuffs.pyro_res_reduction, ...
```

## JS Consumer Usage Flow

```typescript
// 1. Resolve team
const result = resolve_team_stats(members, targetIndex);

// 2. Build DamageInput
const input = {
  character_level: 90,
  stats: { ...result.final_stats },
  talent_multiplier: 1.76,
  scaling_stat: "Atk",
  damage_type: "Normal",
  element: "Pyro",
  reaction: "Vaporize",
  reaction_bonus: result.damage_context.amplifying_bonus,
  flat_dmg: result.damage_context.normal_atk_flat_dmg,
};
// Add type-specific DMG bonus to stats
input.stats.dmg_bonus += result.damage_context.normal_atk_dmg_bonus;

// 3. Apply enemy debuffs
const enemy = apply_team_debuffs(baseEnemy, result.enemy_debuffs, "Pyro");

// 4. Calculate
const damage = calculate_damage(input, enemy);
```

## Test Plan

1. **`DamageContext::from_buffs` unit tests** — each `BuffableStat` variant maps to correct field; mixed buffs aggregate correctly
2. **`collect_enemy_debuffs` unit tests** — each element reduction and def_reduction collected correctly
3. **`apply_debuffs_to_enemy` unit tests** — element matching, negative resistance, DEF clamp at 1.0 (mirrors existing `apply_enemy_debuffs` tests)
4. **`resolve_team_stats_detailed` integration** — team with Chiori (res shred) + Bennett (ATK buff) produces correct `damage_context` and `enemy_debuffs`
5. **Full pipeline golden test** — Mavuika/Citlali/Chiori/Bennett team: `resolve_team_stats` -> `apply_team_debuffs` -> `calculate_damage` with hand-calculated expected values
6. **WASM serde roundtrip** — `TeamResolveResult`, `DamageContext`, `EnemyDebuffs` serialize/deserialize correctly through `serde_wasm_bindgen`

## Versioning

| Crate | Version | Change |
|-------|---------|--------|
| core | 0.3.0 | Breaking: `resolve_team_stats` return type, `TeamResolveResult` new fields |
| wasm | 0.3.0 | Breaking: `resolve_team_stats` return shape; new `apply_team_debuffs` |
| data | unchanged | — |
| good | unchanged | — |

## Out of Scope

- `HealingBonus` / `ShieldStrength` — not relevant to damage calculation
- `DefPercentRaw` — already handled in weapon-specific processing
- smart-paimon frontend changes — separate task after WASM release
