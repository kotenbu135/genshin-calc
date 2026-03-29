# P3 Batch 4: 5-Star Shield Series Weapon ConditionalBuff Design

**Date**: 2026-03-29
**Status**: Draft
**Depends on**: P0 (ConditionalBuff), P3 Batch 2 (Stacks pattern)
**Scope**: 4 five-star "Liyue Shield" weapons with Stacks+Shield ConditionalBuff (data-only, no core changes)

## Overview

Add ConditionalBuff definitions to 4 five-star weapons sharing the identical "Liyue Shield Series" passive pattern. Each weapon gets 2 ConditionalBuffs: normal ATK% stacks and shield-bonus ATK% stacks. No core crate changes required.

## Pattern

All 4 weapons share the same passive structure:
1. Shield Strength +20-40% — **not modelable** (irrelevant to damage calculation)
2. On hit: ATK +4-8% per stack, max 5 stacks — `ManualCondition::Stacks(5)`
3. While shielded: stacking effect doubled — modeled as a second `Stacks(5)` with identical per-stack value

### Modeling

Two ConditionalBuffs per weapon:

```rust
// Normal ATK% stacks (always available)
ConditionalBuff {
    name: "<weapon_id>_atk_stacks",
    stat: BuffableStat::AtkPercent,
    value: 0.04, // R1
    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Stacks(5)),
}

// Shield bonus ATK% stacks (activate with same stack count when shielded)
ConditionalBuff {
    name: "<weapon_id>_shield_atk_stacks",
    stat: BuffableStat::AtkPercent,
    value: 0.04, // R1
    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Stacks(5)),
}
```

Usage: User activates `<id>_atk_stacks` with current stack count (0-5). If shielded, also activates `<id>_shield_atk_stacks` with the same stack count. At 5 stacks + shield: total ATK% = 5 × 0.04 × 2 = 0.40 (R1).

## 4 Weapon Definitions

### 1. Summit Shaper (Sword)

```
name: "summit_shaper_atk_stacks" / "summit_shaper_shield_atk_stacks"
stat: AtkPercent
R1-R5: [0.04, 0.05, 0.06, 0.07, 0.08]
activation: Manual(Stacks(5))
```

### 2. The Unforged (Claymore)

```
name: "the_unforged_atk_stacks" / "the_unforged_shield_atk_stacks"
stat: AtkPercent
R1-R5: [0.04, 0.05, 0.06, 0.07, 0.08]
activation: Manual(Stacks(5))
```

### 3. Vortex Vanquisher (Polearm)

```
name: "vortex_vanquisher_atk_stacks" / "vortex_vanquisher_shield_atk_stacks"
stat: AtkPercent
R1-R5: [0.04, 0.05, 0.06, 0.07, 0.08]
activation: Manual(Stacks(5))
```

### 4. Memory of Dust (Catalyst)

```
name: "memory_of_dust_atk_stacks" / "memory_of_dust_shield_atk_stacks"
stat: AtkPercent
R1-R5: [0.04, 0.05, 0.06, 0.07, 0.08]
activation: Manual(Stacks(5))
```

## Notes

- Shield Strength buff (+20-40%) is excluded — not relevant to damage calculation.
- All 4 weapons use identical R1-R5 values and mechanics.
- `stack_values: None` for all (linear scaling).
- `target: BuffTarget::OnlySelf` for all.

## Impact

### Files Modified

| File | Changes |
|------|---------|
| `crates/data/src/weapons/sword.rs` | Summit Shaper: 2 ConditionalBuffs |
| `crates/data/src/weapons/claymore.rs` | The Unforged: 2 ConditionalBuffs |
| `crates/data/src/weapons/polearm.rs` | Vortex Vanquisher: 2 ConditionalBuffs |
| `crates/data/src/weapons/catalyst.rs` | Memory of Dust: 2 ConditionalBuffs |

### Core Crate

No changes required.

## Test Plan

4 unit tests (one per weapon):
- Verify 2 ConditionalBuffs exist
- Verify both have stat AtkPercent
- Verify R1 value = 0.04
- Verify both have Activation::Manual(ManualCondition::Stacks(5))

## Out of Scope

- Shield Strength buff modeling
- ShieldStrength BuffableStat addition
