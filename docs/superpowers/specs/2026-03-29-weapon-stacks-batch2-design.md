# P3 Batch 2: 5-Star Weapon Stacks ConditionalBuff Design

**Date**: 2026-03-29
**Status**: Reviewed (v2)
**Depends on**: P0 (ConditionalBuff -- complete), P3 Batch 1 (StatScaling -- complete)
**Scope**: 9 five-star weapons with Stacks-type ConditionalBuff (data-only, no core changes)

## Overview

Add `ManualCondition::Stacks` ConditionalBuff definitions to 9 five-star weapons. All use existing framework -- no core crate changes required. Four weapons additionally have a "full-stack bonus" modeled as a separate `ManualCondition::Toggle` ConditionalBuff.

## Patterns

### Pattern A: Pure Stacks (5 weapons)

Single ConditionalBuff per weapon. Linear scaling: `value * stacks`.

```rust
ConditionalBuff {
    name: "<weapon_id>_<stat>_stacks",
    description: "<effect description>",
    stat: BuffableStat::Xxx,
    value: <per_stack_r1>,
    refinement_values: Some([R1, R2, R3, R4, R5]),
    stack_values: None,  // linear: value * stacks
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Stacks(<max>)),
}
```

### Pattern B: Stacks + Full-Stack Bonus (4 weapons)

Two ConditionalBuffs per weapon:
1. Stacks buff (same as Pattern A)
2. Full-stack toggle bonus:

```rust
ConditionalBuff {
    name: "<weapon_id>_full_stack_bonus",
    description: "<full stack effect>",
    stat: BuffableStat::Xxx,
    value: <bonus_r1>,
    refinement_values: Some([R1, R2, R3, R4, R5]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}
```

Consumer activates both when at full stacks. The Toggle is separate because the full-stack bonus is a distinct, non-linear addition (not `per_stack * max_stacks`).

## 9 Weapon Definitions

### 1. Primordial Jade Winged-Spear (Pattern B) -- Polearm

```
Stacks buff:
  name: "pjws_atk_stacks"
  stat: AtkPercent
  max_stacks: 6
  R1-R5: [0.032, 0.039, 0.046, 0.053, 0.060]

Full-stack bonus:
  name: "pjws_full_stack_dmg"
  stat: DmgBonus
  R1-R5: [0.12, 0.15, 0.18, 0.21, 0.24]
```

6 stacks max from hits. Full-stack DMG bonus modeled as separate Toggle.

### 2. Fang of the Mountain King (Pattern A) -- Claymore

```
Stacks buff:
  name: "fang_mountain_king_elemental_dmg_stacks"
  stat: DmgBonus
  max_stacks: 3
  R1-R5: [0.10, 0.125, 0.15, 0.175, 0.20]
```

Elemental Skill hit grants stacks, each increasing all Elemental DMG. Note: `DmgBonus` used as approximation; actual game effect is "All Elemental DMG" (excludes Physical). Physical damage calculations will slightly over-count.

### 3. Astral Vulture's Crimson Plumage (Pattern B) -- Bow

```
Stacks buff:
  name: "astral_vulture_atk_stacks"
  stat: AtkPercent
  max_stacks: 3
  R1-R5: [0.12, 0.15, 0.18, 0.21, 0.24] (TBV)

Full-stack bonus:
  name: "astral_vulture_full_stack_dmg"
  stat: DmgBonus
  R1-R5: [0.12, 0.15, 0.18, 0.21, 0.24] (TBV)
```

Nightsoul stack accumulation grants ATK%. At full stacks, additional DMG%.

### 4. Silvershower Heartstrings (Pattern B) -- Bow

```
Stacks buff:
  name: "silvershower_hp_stacks"
  stat: HpPercent
  max_stacks: 3
  R1-R5: [0.12, 0.15, 0.18, 0.21, 0.24]

Full-stack bonus:
  name: "silvershower_full_stack_hydro_dmg"
  stat: ElementalDmgBonus(Hydro)
  R1-R5: [0.28, 0.35, 0.42, 0.49, 0.56]
```

Elemental Skill use grants HP% stacks. At 3 stacks, Hydro DMG bonus.

### 5. The Daybreak Chronicles (Pattern A) -- Bow

```
Stacks buff:
  name: "the_daybreak_chronicles_dmg_stacks"
  stat: DmgBonus
  max_stacks: 3
  R1-R5: [0.08, 0.10, 0.12, 0.14, 0.16] (TBV)
```

Story accumulation grants DMG%.

### 6. Kagura's Verity (Pattern B) -- Catalyst

```
Stacks buff:
  name: "kagura_skill_dmg_stacks"
  stat: SkillDmgBonus
  max_stacks: 3
  R1-R5: [0.12, 0.15, 0.18, 0.21, 0.24]

Full-stack bonus:
  name: "kagura_full_stack_elemental_dmg"
  stat: DmgBonus
  R1-R5: [0.12, 0.15, 0.18, 0.21, 0.24]
```

Elemental Skill use grants Skill DMG% stacks. At 3 stacks, all Elemental DMG bonus. Note: Full-stack bonus uses `DmgBonus` as approximation; actual game effect is "All Elemental DMG" (excludes Physical).

### 7. Tulaytullah's Remembrance (Pattern A) -- Catalyst

```
Stacks buff:
  name: "tulaytullah_na_dmg_stacks"
  stat: NormalAtkDmgBonus
  max_stacks: 10
  R1-R5: [0.048, 0.06, 0.072, 0.084, 0.096]
```

Normal Attack DMG% increases per stack (1 stack per second on-field, linear). Attack Speed buff is not modelable and excluded from scope. Verify linearity of per-stack value during implementation -- if non-linear, use `stack_values: Some(...)` instead.

### 8. Nocturne's Curtain Call (Pattern A) -- Catalyst

```
Stacks buff:
  name: "nocturne_dmg_stacks"
  stat: DmgBonus
  max_stacks: 5
  R1-R5: [0.08, 0.10, 0.12, 0.14, 0.16] (TBV)
```

Elemental Skill/Burst hit grants DMG% stacks.

### 9. Vivid Notions (Pattern A) -- Catalyst

```
Stacks buff:
  name: "vivid_notions_dmg_stacks"
  stat: DmgBonus
  max_stacks: 3
  R1-R5: [0.08, 0.10, 0.12, 0.14, 0.16] (TBV)
```

Nightsoul Burst grants DMG% stacks.

## Notes

- **(TBV)** = To Be Verified. R1-R5 values marked TBV should be verified against HoneyhunterWorld/Ambr.top during implementation. The R1 value is from the weapon description; R2-R5 follow standard 5-star scaling (R1 * [1.0, 1.25, 1.5, 1.75, 2.0]).
- All percentage values in decimal form (10% = 0.10) per project convention.
- `stack_values: None` for all weapons (linear scaling).
- `target: BuffTarget::OnlySelf` for all buffs (no team-wide effects in this batch).
- Description fields to be written during implementation following existing codebase style.
- Weapons with non-modelable effects (attack speed) only implement the modelable portion.

## Impact

### Files Modified

| File | Changes |
|------|---------|
| `crates/data/src/weapons/polearm.rs` | PJWS: 2 ConditionalBuffs |
| `crates/data/src/weapons/claymore.rs` | Fang: 1 ConditionalBuff |
| `crates/data/src/weapons/bow.rs` | Astral Vulture: 2, Silvershower: 2, Daybreak: 1 ConditionalBuffs |
| `crates/data/src/weapons/catalyst.rs` | Kagura: 2, Tulaytullah: 1, Nocturne: 1, Vivid Notions: 1 ConditionalBuffs |

### Core Crate

No changes required.

### Existing Tests

No behavioral changes. Existing `data_integrity` tests automatically validate new ConditionalBuff consistency (stack max > 0, refinement_values length, etc.).

## Test Plan

9 unit tests (one per weapon):
- Verify ConditionalBuff count and names
- Verify stack count matches expected max
- Verify stat type matches expected BuffableStat
- Verify R1 value matches expected per-stack value
- For Pattern B weapons: verify Toggle ConditionalBuff exists with correct stat and R1 value

## Out of Scope

- Attack Speed buffs (Tulaytullah's Remembrance) -- not modelable in current framework
- Shield-dependent effects (Summit Shaper, Memory of Dust, The Unforged, Vortex Vanquisher)
- Team-wide buffs (Elegy for the End)
- Non-linear stack values (all 9 weapons use linear scaling)
