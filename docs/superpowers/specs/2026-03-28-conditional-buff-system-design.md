# Conditional Buff System Design

Date: 2026-03-28
Status: Draft
Priority: P0 (unlocks P2/P3/P4/P6)

## Problem

The current `StatBuff` / `PassiveEffect` / `SetEffect` model can only express unconditional, fixed-value buffs. This leaves 174 weapon passives and 59 artifact set effects empty in the data crate. Conditions like HP thresholds, weapon type restrictions, stat-dependent scaling, reaction triggers, and stacking mechanics cannot be represented.

## Design Decisions

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Usage model | User-driven with future auto capability | Caller provides condition state; core stays pure |
| Condition evaluation location | data crate (TeamMemberBuilder) | Core unchanged; builder already handles constellation/talent logic |
| Data extension | Add `conditional_buffs` field to `PassiveEffect` and `SetEffect` | Backward compatible; clear separation from unconditional buffs |
| Condition metadata | Compound `Auto + Manual + Both` | Covers weapon-type auto-check + user toggles + compound cases (Nilou) |
| Builder API | Name-based activation | Self-documenting, stable across data reordering |
| Architecture | Static data + builder one-pass evaluation | WASM safe, immutable, minimal blast radius |

## Core Crate Changes

**None.** All new types live in the data crate. No enum additions, no logic changes.

The `apply_buffs_to_profile` function continues to skip type-specific buffs (`NormalAtkDmgBonus`, etc.); callers resolve these from `applied_buffs` based on `DamageType`.

Reaction-specific bonuses (Crimson Witch 4pc Vaporize/Melt +15%) are not representable as a `BuffableStat` because the current `ReactionBonus` concept is too coarse (CW boosts Vaporize/Melt but not Aggravate/Spread). These remain caller-provided via `DamageInput.reaction_bonus`. A future P6-adjacent design may introduce `BuffableStat::AmplifyingReactionBonus` / `TransformativeReactionBonus`.

## New Types (data crate: `buff.rs`)

All new types derive `Serialize` only (not `Deserialize`), matching the existing `PassiveEffect` and `SetEffect` pattern — `&'static` slice references cannot be deserialized.

### AutoCondition

Conditions the builder evaluates automatically from character/team data.

```rust
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AutoCondition {
    /// Buff only applies to specific weapon types (e.g. Gladiator 4pc).
    WeaponTypeRequired(&'static [WeaponType]),
    /// Buff only applies to characters of specific elements.
    ElementRequired(&'static [Element]),
    /// Buff value computed from a stat.
    /// The multiplier comes from `ConditionalBuff.value` (or `refinement_values[r]`).
    /// Final value = stat_value * multiplier, capped at `cap` if set.
    /// Example: Emblem 4pc — stat=EnergyRecharge, cap=Some(0.75), value=0.25.
    StatScaling {
        stat: BuffableStat,
        cap: Option<f64>,
    },
    /// Requires N or more team members of a specific element (e.g. Gorou).
    TeamElementCount {
        element: Element,
        min_count: u8,
    },
    /// Team must consist only of specified elements (e.g. Nilou: Hydro+Dendro).
    TeamElementsOnly(&'static [Element]),
}
```

### ManualCondition

Conditions requiring user input (game state the builder cannot determine).

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManualCondition {
    /// Simple on/off toggle (e.g. "HP below 50%", "after using skill").
    Toggle,
    /// Stackable buff with maximum stack count (e.g. CW 4pc max 3).
    /// The u8 value is the maximum allowed stacks (practical max ~4 in Genshin).
    Stacks(u8),
}
```

### Activation

```rust
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Activation {
    /// Evaluated automatically by the builder.
    Auto(AutoCondition),
    /// Requires user input.
    Manual(ManualCondition),
    /// Both conditions must be satisfied (Auto evaluated first; short-circuits on failure).
    Both(AutoCondition, ManualCondition),
}
```

### ConditionalBuff

```rust
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConditionalBuff {
    /// Machine-readable identifier (e.g. "homa_hp_bonus").
    pub name: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Which stat is buffed.
    pub stat: BuffableStat,
    /// Buff value at refinement 1 (or fixed value if no refinement).
    /// For StatScaling: this is the multiplier applied to the source stat.
    pub value: f64,
    /// Values at refinements 1-5. None for non-weapon / fixed buffs.
    /// For StatScaling: these are the multipliers at each refinement level.
    pub refinement_values: Option<[f64; 5]>,
    /// Activation condition.
    pub activation: Activation,
}
```

### ManualActivation (builder input from user)

```rust
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ManualActivation {
    /// Toggle ON.
    Active,
    /// Stackable buff with specified stack count.
    Stacks(u8),
}
```

### PassiveEffect (extended)

```rust
pub struct PassiveEffect {
    pub description: &'static str,
    pub buffs: &'static [StatBuff],                    // unconditional (existing)
    pub conditional_buffs: &'static [ConditionalBuff],  // conditional (new)
}
```

### SetEffect (extended)

```rust
pub struct SetEffect {
    pub description: &'static str,
    pub buffs: &'static [StatBuff],                    // unconditional (existing)
    pub conditional_buffs: &'static [ConditionalBuff],  // conditional (new)
}
```

### Migration macro

To avoid touching all 334 existing definitions in Phase 1, provide a helper macro:

```rust
/// Creates a PassiveEffect with no conditional buffs (migration helper).
macro_rules! passive_effect {
    ($desc:expr, $buffs:expr) => {
        PassiveEffect {
            description: $desc,
            buffs: $buffs,
            conditional_buffs: &[],
        }
    };
}

/// Creates a SetEffect with no conditional buffs (migration helper).
macro_rules! set_effect {
    ($desc:expr, $buffs:expr) => {
        SetEffect {
            description: $desc,
            buffs: $buffs,
            conditional_buffs: &[],
        }
    };
}
```

Existing data files can adopt the macro gradually. Both the macro form and the explicit struct form are valid, allowing incremental migration.

## Builder API (data crate: `team_builder.rs`)

### New fields

```rust
pub struct TeamMemberBuilder {
    // ... existing fields ...
    manual_activations: Vec<(&'static str, ManualActivation)>,
    team_elements: Vec<Element>,
}
```

### New methods

```rust
impl TeamMemberBuilder {
    /// Activate a manual conditional buff by name.
    pub fn activate(mut self, name: &'static str) -> Self

    /// Activate a stackable conditional buff with stack count.
    pub fn activate_with_stacks(mut self, name: &'static str, stacks: u8) -> Self

    /// Set team element composition for Auto team-based conditions.
    /// Known limitation: caller must provide this manually. A future TeamBuilder
    /// or resolve_conditional_team_buffs() may automate this.
    pub fn team_elements(mut self, elements: Vec<Element>) -> Self

    /// Returns available conditional buffs with source context.
    pub fn available_conditionals(&self) -> Vec<AvailableConditional>
}

/// A conditional buff with its source context, for UI display.
pub struct AvailableConditional {
    /// Source name (e.g. "Staff of Homa", "Crimson Witch 4pc").
    pub source: &'static str,
    /// The conditional buff definition.
    pub buff: &'static ConditionalBuff,
}
```

### Evaluation flow inside build()

```
build() {
    // ... existing processing (steps 1-7: base stats, ascension, weapon sub, artifacts, buffs) ...

    // Step 9: Resolve conditional buffs
    for each conditional_buff in [weapon_conditionals, artifact_2pc, artifact_4pc]:
        match activation:
            Auto(cond)          → eval_auto(cond, buff, profile, self) → emit ResolvedBuff
            Manual(cond)        → eval_manual(cond, buff, manual_activations) → emit ResolvedBuff
            Both(auto, manual)  → eval_auto first; if pass → eval_manual → emit ResolvedBuff
}
```

### eval_auto behavior

| AutoCondition | Evaluation | Returns |
|---------------|------------|---------|
| `WeaponTypeRequired(types)` | `types.contains(&character.weapon_type)` | `value` if pass |
| `ElementRequired(elements)` | `elements.contains(&character.element)` | `value` if pass |
| `StatScaling { stat, cap }` | read stat from profile, `min(stat_val * value, cap)` | computed value |
| `TeamElementCount { element, min_count }` | count in `team_elements` >= `min_count` | `value` if pass |
| `TeamElementsOnly(elements)` | all `team_elements` in `elements` | `value` if pass |

Notes:
- If `team_elements` is empty, team-based Auto conditions are **skipped** (treated as not evaluable, not failed).
- For `StatScaling`, the multiplier is taken from `ConditionalBuff.value` (or `refinement_values[r]` when refinement is set).

### eval_manual behavior

| ManualCondition | User input | Result |
|-----------------|------------|--------|
| `Toggle` | `Active` present | `value` |
| `Toggle` | `Stacks(n)` (type mismatch) | `None` (ignored) |
| `Toggle` | Not present | `None` |
| `Stacks(max)` | `Stacks(n)` | `value * min(n, max)` |
| `Stacks(max)` | `Active` (type mismatch) | Treated as `Stacks(max)` (max stacks) |
| `Stacks(max)` | Not present | `None` |

Type mismatch behavior: Toggle + Stacks input is silently ignored. Stacks + Active input is treated as max stacks (user explicitly activated, reasonable default).

### Evaluation ordering

The conditional buff evaluation occurs **after** all unconditional buffs (weapon passive, artifact set, talent buffs) have been collected but **before** they are applied to the profile. This means `StatScaling` reads from the base profile (character + weapon + artifacts stats), not from the buff-applied profile.

This matches Genshin's actual behavior: Emblem 4pc reads the character's total ER from artifacts/substats/weapon, not from temporary team buffs.

If a future use case requires reading post-buff stats, a two-pass approach can be introduced without breaking the current API.

## Data Examples

### Staff of Homa

```rust
static HOMA_CONDITIONALS: &[ConditionalBuff] = &[
    ConditionalBuff {
        name: "homa_hp_bonus",
        description: "ATK increased by 0.8% of Max HP",
        stat: BuffableStat::AtkFlat,
        value: 0.008,  // multiplier for StatScaling
        refinement_values: Some([0.008, 0.01, 0.012, 0.014, 0.016]),
        activation: Activation::Auto(AutoCondition::StatScaling {
            stat: BuffableStat::HpPercent,  // reads from profile HP
            cap: None,
        }),
    },
    ConditionalBuff {
        name: "homa_low_hp_bonus",
        description: "When HP < 50%, ATK increased by additional 1.0% of Max HP",
        stat: BuffableStat::AtkFlat,
        value: 0.01,
        refinement_values: Some([0.01, 0.012, 0.014, 0.016, 0.018]),
        activation: Activation::Both(
            AutoCondition::StatScaling {
                stat: BuffableStat::HpPercent,
                cap: None,
            },
            ManualCondition::Toggle,
        ),
    },
];
```

Note: `StatScaling` with `stat: BuffableStat::HpPercent` means the builder reads the character's total HP (computed from `base_hp * (1 + hp_percent) + hp_flat`) and multiplies by `value`. The specific stat-to-profile mapping for `StatScaling` is:

| BuffableStat in StatScaling | Profile value read |
|-----------------------------|-------------------|
| `HpPercent` | `base_hp * (1 + hp_percent) + hp_flat` |
| `AtkPercent` | `base_atk * (1 + atk_percent) + atk_flat` |
| `DefPercent` | `base_def * (1 + def_percent) + def_flat` |
| `ElementalMastery` | `elemental_mastery` |
| `EnergyRecharge` | `energy_recharge` |

### Gladiator's Finale 4pc

```rust
static GLADIATOR_4PC_CONDITIONALS: &[ConditionalBuff] = &[
    ConditionalBuff {
        name: "gladiator_normal_bonus",
        description: "Normal Attack DMG +35% for sword/claymore/polearm",
        stat: BuffableStat::NormalAtkDmgBonus,
        value: 0.35,
        refinement_values: None,
        activation: Activation::Auto(AutoCondition::WeaponTypeRequired(&[
            WeaponType::Sword,
            WeaponType::Claymore,
            WeaponType::Polearm,
        ])),
    },
];
```

### Emblem of Severed Fate 4pc

```rust
static EMBLEM_4PC_CONDITIONALS: &[ConditionalBuff] = &[
    ConditionalBuff {
        name: "emblem_burst_bonus",
        description: "Burst DMG +25% of Energy Recharge, max 75%",
        stat: BuffableStat::BurstDmgBonus,
        value: 0.25,  // multiplier: ER * 0.25
        refinement_values: None,
        activation: Activation::Auto(AutoCondition::StatScaling {
            stat: BuffableStat::EnergyRecharge,
            cap: Some(0.75),
        }),
    },
];
```

### Crimson Witch of Flames 4pc

```rust
static CRIMSON_WITCH_4PC_CONDITIONALS: &[ConditionalBuff] = &[
    // Note: Vaporize/Melt +15% is NOT included here because BuffableStat
    // cannot distinguish reaction subtypes. This bonus remains caller-provided
    // via DamageInput.reaction_bonus. See "Core Crate Changes" section.
    ConditionalBuff {
        name: "cwof_pyro_stacks",
        description: "Using Skill increases Pyro DMG by 7.5%, max 3 stacks",
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        value: 0.075,  // per stack
        refinement_values: None,
        activation: Activation::Manual(ManualCondition::Stacks(3)),
    },
];
```

Known gap: CW 4pc Overloaded/Burning/Burgeon +40% (transformative reaction bonus) is also not representable with the current `BuffableStat` set. This will be addressed alongside P6 (enemy debuff system) when reaction-specific bonus types are designed.

## Impact Analysis

### Change map

| File | Change | Breaking |
|------|--------|:--------:|
| data `buff.rs` | New types: `ConditionalBuff`, `Activation`, `AutoCondition`, `ManualCondition`, `ManualActivation`, `AvailableConditional` | No (additions) |
| data `types.rs` | `PassiveEffect` + `SetEffect`: add `conditional_buffs` field | **Yes** |
| data `team_builder.rs` | New methods + eval logic in `build()` | No |
| data weapon files (230) | Add `conditional_buffs: &[]` to each `PassiveEffect` | No (data) |
| data `artifacts.rs` (104) | Add `conditional_buffs: &[]` to each `SetEffect` (52 sets x 2pc/4pc) | No (data) |
| core | **No changes** | - |

Total mechanical additions: **334 definitions** (230 `PassiveEffect` + 104 `SetEffect`). The `passive_effect!` / `set_effect!` migration macros eliminate most of this by defaulting `conditional_buffs` to `&[]`.

### Backward compatibility

- `TeamMemberBuilder::new(char, weapon).build()` works unchanged — new methods are all optional
- `ResolvedBuff` type unchanged — downstream `resolve_team_stats` / `calculate_damage` unaffected
- New types: `ConditionalBuff`, `Activation`, `AutoCondition` are `Serialize`-only (not `Deserialize`) because they contain `&'static` references, matching existing `PassiveEffect`/`SetEffect` pattern
- `ManualCondition`, `ManualActivation` are both `Serialize + Deserialize` (no `&'static` refs)

### Migration phases

```
Phase 1: Type definitions + migration macros
         + PassiveEffect/SetEffect extension
         + existing data adopts macros (or explicit empty arrays)
         → cargo test all pass (zero behavior change)

Phase 2: Builder eval logic (eval_auto, eval_manual)
         + unit tests + integration tests

Phase 3: Populate conditional_buffs for priority weapons/artifacts
         (aligns with P2/P3 roadmap items)
```

## Test Strategy

### Unit tests (data crate)

**eval_auto (all variants):**
- `WeaponTypeRequired`: match / mismatch
- `ElementRequired`: match / mismatch
- `StatScaling`: normal value / cap reached / zero stat
- `TeamElementCount`: sufficient / insufficient / empty team_elements (skip)
- `TeamElementsOnly`: valid / invalid / empty team_elements (skip)

**eval_manual (all variants):**
- `Toggle`: activated / not activated
- `Stacks`: normal / exceeds max (capped) / not activated
- Type mismatch: `Toggle` + `Stacks(n)` → None; `Stacks` + `Active` → max stacks

**Activation::Both:**
- Auto pass + Manual pass → active
- Auto fail + Manual pass → inactive (short-circuit)
- Auto pass + Manual fail → inactive

### Integration tests (builder → resolve → damage)

- Staff of Homa: HP-dependent ATK buff + low HP toggle
- Gladiator 4pc: weapon type auto-check (sword pass, catalyst fail)
- Emblem 4pc: ER-dependent burst bonus with cap
- Crimson Witch 4pc: pyro stacks
- Nilou: team composition restriction (Both condition)
- Regression: existing Bennett/Noblesse tests unchanged

### Golden tests

- Hu Tao + Homa R1 + CW 4pc: charged attack vaporize vs Lv90 Hilichurl
  (hand-calculated reference value, tolerance < 1.0)

### Data integrity tests

- All `ConditionalBuff.name` values are unique within their source
- All `ConditionalBuff.stat` values are valid `BuffableStat` variants
- All `Stacks(max)` have max > 0
- All `StatScaling.cap` values are > 0 when present

### Coverage targets

| Target | Goal |
|--------|------|
| eval_auto / eval_manual | 100% variant coverage |
| Builder conditional resolution | >= 90% |
| Data integrity | 100% of ConditionalBuff entries validated |

## Known Limitations and Future Work

1. **Reaction-specific bonuses** (CW 4pc Vaporize/Melt +15%, Overloaded +40%) cannot be expressed as `BuffableStat`. These require a future `BuffableStat::AmplifyingReactionBonus` or similar, designed alongside P6.

2. **`team_elements` is caller-provided.** The builder builds one member at a time and doesn't know the full team composition. A future `TeamBuilder` or `resolve_conditional_team_buffs()` could automate this.

3. **StatScaling reads pre-buff stats.** The evaluation reads from the base profile (character + weapon + artifacts), not from the profile after team buff application. This matches Genshin's actual behavior but may need revisiting for edge cases.

4. **Refinement selection not yet in builder.** The `refinement_values` field is defined but `TeamMemberBuilder` does not yet accept a refinement level parameter. This will be addressed in P4 (weapon refinement values).
