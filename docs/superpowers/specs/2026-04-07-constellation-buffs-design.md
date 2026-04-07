# Constellation Conditional Buffs: Comprehensive Implementation

**Issue:** #51
**Date:** 2026-04-07

## Overview

61 characters were identified as lacking constellation-dependent conditional buffs in `get_talent_conditional_buffs`. After investigation of all constellation effects via Honey Impact and cross-referencing with existing implementations, **37 characters require 69 new buff entries**.

## Classification Criteria

### IMPLEMENT
- ATK%, DEF%, HP%, EM, CRIT Rate, CRIT DMG, DMG Bonus (elemental/physical/type-specific)
- RES shred (elemental/physical), DEF reduction, DEF ignore
- Flat DMG additions (Normal/Charged/Plunging/Skill/Burst)
- Transformative/Amplifying reaction bonuses

### SKIP
- Proc damage (extra hits), energy generation, CD reduction
- HP healing, shield strength, resistance to interruption
- Extra charges, talent level increases (+3)
- ATK SPD (not in `BuffableStat`)
- Enemy ATK reduction (not modeled in outgoing damage calc)
- Attack mode transformations (not expressible as ConditionalBuff)
- Specific-hit DMG multiplier changes (proc-like, not stat buffs)

## Implementation Plan

### Approach: Element-by-Element Batches

Each element's `talent_buffs/<element>.rs` gets a separate commit/PR. This matches the existing code structure and provides natural review boundaries.

### Buff Entry Patterns

**Pattern 1: Simple Toggle**
```rust
TalentBuffDef {
    name: "xiangling_c1_pyro_res_shred",
    description: "C1: Guoba reduces Pyro RES -15%",
    stat: BuffableStat::ElementalResReduction(Element::Pyro),
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(1),
    min_constellation: 1,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
}
```

**Pattern 2: Stacks**
```rust
// Ganyu C4: DMG taken +5% per 3s, max 25% (5 stacks)
TalentBuffDef {
    min_constellation: 4,
    activation: Some(Activation::Manual(ManualCondition::Stacks(5))),
    base_value: 0.05,
    // ...
}
```

**Pattern 3: StatScaling**
```rust
// Nilou C6: per 1000 HP → CRIT Rate +0.6%
TalentBuffDef {
    scales_on: Some(ScalingStat::Hp),
    base_value: 0.006, // per 1000 HP
    cap: Some(0.30),   // max 30%
    // ...
}
```

## Complete Implementation List

### Pyro (8 characters, 14 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| amber | C6 | AtkPercent | 0.15 | Team | Toggle | Fiery Rain party ATK+15% |
| chevreuse | C6 | ElementalDmgBonus(Pyro) | 0.20 | Team | Stacks(3) | Per stack, also Electro DMG |
| chevreuse | C6 | ElementalDmgBonus(Electro) | 0.20 | Team | Stacks(3) | Paired with Pyro entry |
| durin | C1 | BurstDmgBonus | (complex) | OnlySelf | Toggle | Burst DMG +150% ATK (StatScaling) |
| durin | C1 | NormalAtkFlatDmg | (complex) | Team | Toggle | Party flat DMG +60% ATK (StatScaling) |
| durin | C2 | ElementalDmgBonus(Pyro) | 0.50 | Team | Toggle | On reaction trigger |
| durin | C4 | BurstDmgBonus | 0.40 | OnlySelf | Toggle | |
| durin | C6 | DefIgnore | 0.30 | OnlySelf | Toggle | Base DEF ignore |
| durin | C6 | DefReduction | 0.30 | Team | Toggle | Light form DEF shred |
| klee | C2 | DefReduction | 0.23 | Team | Toggle | Mine DEF reduction |
| klee | C6 | ElementalDmgBonus(Pyro) | 0.10 | Team | Toggle | Sparks 'n' Splash |
| thoma | C6 | NormalAtkDmgBonus | 0.15 | Team | Toggle | Also Charged/Plunging |
| thoma | C6 | ChargedAtkDmgBonus | 0.15 | Team | Toggle | |
| thoma | C6 | PlungingAtkDmgBonus | 0.15 | Team | Toggle | |
| xiangling | C1 | ElementalResReduction(Pyro) | 0.15 | Team | Toggle | Guoba hit |
| xiangling | C6 | ElementalDmgBonus(Pyro) | 0.15 | Team | Toggle | Pyronado duration |
| xinyan | C2 | CritRate | 1.00 | OnlySelf | Toggle | Burst Physical only (BurstDmgBonus context) |
| xinyan | C4 | PhysicalResReduction | 0.15 | Team | Toggle | Sweeping Fervor swing |
| xinyan | C6 | ChargedAtkFlatDmg | 0.50 | OnlySelf | Toggle | scales_on: Def |
| yoimiya | C1 | AtkPercent | 0.20 | OnlySelf | Toggle | On Aurous Blaze kill |
| yoimiya | C2 | ElementalDmgBonus(Pyro) | 0.25 | OnlySelf | Toggle | On Pyro CRIT |

### Hydro (8 characters, 17 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| aino | C1 | ElementalMastery | 80.0 | Team | Toggle | After Skill/Burst |
| aino | C6 | TransformativeBonus | 0.15 | Team | Toggle | Reaction DMG +15/20% |
| barbara | C2 | ElementalDmgBonus(Hydro) | 0.15 | Team | Toggle | Active char during Skill |
| candace | C2 | HpPercent | 0.20 | OnlySelf | Toggle | On Skill hit |
| columbina | C1 | TransformativeBonus | 0.015 | Team | Toggle | Lunar Reaction DMG |
| columbina | C2 | HpPercent | 0.40 | OnlySelf | Toggle | On Gravity Interference |
| columbina | C2 | TransformativeBonus | 0.07 | Team | Toggle | Lunar DMG +7% |
| columbina | C4 | TransformativeBonus | 0.015 | Team | Toggle | Lunar DMG +1.5% |
| columbina | C6 | CritRate | 0.80 | Team | Toggle | For elemental DMG |
| columbina | C6 | TransformativeBonus | 0.07 | Team | Toggle | Lunar DMG +7% |
| mona | C1 | TransformativeBonus | 0.15 | Team | Toggle | Reaction DMG vs Omen |
| mona | C2 | ElementalMastery | 80.0 | Team | Toggle | On Charged ATK |
| mona | C4 | CritRate | 0.15 | Team | Toggle | vs Omen targets |
| mona | C4 | CritDmg | 0.15 | Team | Toggle | Hexerei party only |
| mona | C6 | ChargedAtkDmgBonus | 1.80 | OnlySelf | Toggle | Max +180% |
| mualani | C4 | BurstDmgBonus | 0.75 | OnlySelf | Toggle | Boomsharka-laka +75% |
| nilou | C2 | ElementalResReduction(Hydro) | 0.35 | Team | Toggle | On Hydro DMG |
| nilou | C2 | ElementalResReduction(Dendro) | 0.35 | Team | Toggle | On Bloom DMG |
| nilou | C4 | BurstDmgBonus | 0.50 | OnlySelf | Toggle | After 3rd dance step |
| nilou | C6 | CritRate | (scaling) | OnlySelf | Toggle | scales_on: Hp, cap 0.30 |
| nilou | C6 | CritDmg | (scaling) | OnlySelf | Toggle | scales_on: Hp, cap 0.60 |
| xingqiu | C2 | ElementalResReduction(Hydro) | 0.15 | Team | Toggle | Sword rain hit |
| xingqiu | C4 | SkillDmgBonus | 0.50 | OnlySelf | Toggle | During Burst |
| yelan | C4 | HpPercent | 0.10 | Team | Stacks(4) | Per marked opponent |

### Electro (5 characters, 7 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| beidou | C4 | ElementalDmgBonus(Electro) | 0.20 | OnlySelf | Toggle | Normal ATK Electro DMG on hit |
| flins | C2 | ElementalResReduction(Electro) | 0.25 | Team | Toggle | Ascendant Gleam Moonsign |
| flins | C6 | TransformativeBonus | 0.35 | OnlySelf | Toggle | Lunar-Charged DMG |
| flins | C6 | TransformativeBonus | 0.10 | Team | Toggle | Team Lunar-Charged DMG |
| iansan | C2 | AtkPercent | 0.30 | Team | Toggle | Off-field, active char |
| iansan | C6 | DmgBonus | 0.25 | Team | Toggle | On Nightsoul overflow |
| ineffa | C1 | TransformativeBonus | (scaling) | Team | Toggle | scales_on: Atk, cap 0.50 |
| yae_miko | C6 | DefIgnore | 0.60 | OnlySelf | Toggle | Sesshou Sakura attacks |

### Cryo (7 characters, 10 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| diona | C6 | ElementalMastery | 200.0 | Team | Toggle | When HP >50% |
| escoffier | C1 | CritDmg | 0.60 | Team | Toggle | Cryo CRIT DMG, 4 Hydro/Cryo |
| escoffier | C2 | SkillFlatDmg | (scaling) | Team | Toggle | scales_on: Atk, ×2.40 |
| eula | C1 | PhysicalDmgBonus | 0.30 | OnlySelf | Toggle | On Grimheart consume |
| eula | C4 | BurstDmgBonus | 0.25 | OnlySelf | Toggle | vs <50% HP enemies |
| ganyu | C1 | ElementalResReduction(Cryo) | 0.15 | Team | Toggle | Frostflake hit |
| ganyu | C4 | DmgBonus | 0.05 | Team | Stacks(5) | Inside Celestial Shower |
| mika | C6 | CritDmg | 0.60 | Team | Toggle | Physical CRIT DMG |
| qiqi | C2 | NormalAtkDmgBonus | 0.15 | OnlySelf | Toggle | vs Cryo-affected |
| qiqi | C2 | ChargedAtkDmgBonus | 0.15 | OnlySelf | Toggle | vs Cryo-affected |
| rosaria | C1 | NormalAtkDmgBonus | 0.10 | OnlySelf | Toggle | On CRIT hit |
| rosaria | C6 | PhysicalResReduction | 0.20 | Team | Toggle | Burst hit |
| shenhe | C2 | CritDmg | 0.15 | Team | Toggle | Cryo CRIT DMG in Burst field |

### Dendro (8 characters, 14 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| baizhu | C4 | ElementalMastery | 80.0 | Team | Toggle | After Burst |
| baizhu | C6 | SkillFlatDmg | (scaling) | OnlySelf | Toggle | scales_on: Hp, ×0.08 |
| collei | C4 | ElementalMastery | 60.0 | TeamExcludeSelf | Toggle | After Burst |
| kirara | C6 | DmgBonus | 0.12 | Team | Toggle | All Elemental DMG (approximation) |
| lauma | C2 | TransformativeBonus | (scaling) | Team | Toggle | scales_on: Em, Bloom/etc |
| lauma | C6 | TransformativeBonus | 0.25 | Team | Toggle | Lunar-Bloom DMG at Ascendant Gleam |
| nahida | C1 | (multiple) | (varies) | OnlySelf | Toggle | A4 Burst +1 count each element |
| nahida | C2 | CritRate | 0.20 | Team | Toggle | Reaction CRIT |
| nahida | C2 | CritDmg | 1.00 | Team | Toggle | Reaction CRIT DMG |
| nahida | C2 | DefReduction | 0.30 | Team | Toggle | On Quicken/Aggravate/Spread |
| nahida | C4 | ElementalMastery | 100.0 | OnlySelf | Stacks(4) | Per nearby Skandha enemy |
| nefer | C2 | ElementalMastery | 200.0 | OnlySelf | Toggle | At 5 Veil stacks |
| nefer | C4 | ElementalResReduction(Dendro) | 0.20 | Team | Toggle | Shadow Dance state |
| nefer | C6 | TransformativeBonus | 0.15 | Team | Toggle | Lunar-Bloom at Ascendant Gleam |
| traveler_dendro | C6 | ElementalDmgBonus(Dendro) | 0.12 | Team | Toggle | Inside Lamp |
| yaoyao | C1 | ElementalDmgBonus(Dendro) | 0.15 | Team | Toggle | Radish explosion |
| yaoyao | C4 | ElementalMastery | (scaling) | OnlySelf | Toggle | scales_on: Hp, ×0.003, cap 120 |

### Anemo (4 characters, 7 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| faruzan | C6 | CritDmg | 0.40 | Team | Toggle | Anemo DMG CRIT DMG |
| jahoda | C6 | CritRate | 0.05 | Team | Toggle | Moonsign characters |
| jahoda | C6 | CritDmg | 0.40 | Team | Toggle | Moonsign characters |
| kazuha | C2 | ElementalMastery | 200.0 | Team | Toggle | In Burst field |
| kazuha | C6 | NormalAtkDmgBonus | (scaling) | OnlySelf | Toggle | scales_on: Em, ×0.002 |
| kazuha | C6 | ChargedAtkDmgBonus | (scaling) | OnlySelf | Toggle | scales_on: Em, ×0.002 |
| kazuha | C6 | PlungingAtkDmgBonus | (scaling) | OnlySelf | Toggle | scales_on: Em, ×0.002 |
| sucrose | C6 | ElementalDmgBonus(Pyro) | 0.20 | Team | Toggle | Absorbed element; need 1 per element |
| venti | C4 | ElementalDmgBonus(Anemo) | 0.25 | OnlySelf | Toggle | On pickup |
| venti | C6 | ElementalResReduction(Anemo) | 0.20 | Team | Toggle | Wind's Grand Ode hit |
| venti | C6 | CritDmg | 1.00 | OnlySelf | Toggle | vs affected targets |
| xianyun | C2 | AtkPercent | 0.20 | OnlySelf | Toggle | After Skyladder |
| xianyun | C6 | CritDmg | 0.70 | OnlySelf | Toggle | Max Driftcloud Wave CRIT DMG |

### Geo (4 characters, 7 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| albedo | C1 | DefPercent | 0.50 | OnlySelf | Toggle | On Skill use |
| albedo | C4 | PlungingAtkDmgBonus | 0.30 | Team | Toggle | In Solar Isotoma |
| albedo | C6 | DmgBonus | 0.17 | Team | Toggle | With Crystallize shield |
| gorou | C6 | CritDmg | 0.40 | Team | Toggle | Geo CRIT DMG at Crunch |
| illuga | C4 | DefFlat | 200.0 | Team | Toggle | During Burst |
| ningguang | C4 | ElementalRes(Pyro) | 0.10 | Team | Toggle | Near Jade Screen; all 7 elements |
| yun_jin | C2 | NormalAtkDmgBonus | 0.15 | Team | Toggle | After Burst |
| yun_jin | C4 | DefPercent | 0.20 | OnlySelf | Toggle | On Crystallize |

## Deferred / Complex Cases

These require additional architectural consideration and may be addressed in follow-up issues:

1. **Nahida C1** — Modifies A4 Burst element counting logic, not a simple buff entry. Needs conditional logic in existing A4 implementation.
2. **Columbina C2 ATK/EM/DEF** — Three different stats based on dominant reaction type. Needs 3 separate Toggle entries.
3. **Durin C1** — Team flat DMG scaling off Durin's ATK. Requires StatScaling with teammate stat reference.
4. **Albedo C2** — Burst DMG scaling off DEF × consumed stacks. Complex mechanic.
5. **Lauma C2** — Bloom/Hyperbloom/Burgeon DMG scaling off EM. May need multiple entries.
6. **Sucrose C6** — Needs one entry per absorbable element (Pyro/Hydro/Electro/Cryo).

## Testing Strategy

Per `CLAUDE.md` testing requirements:
- Each character's constellation buffs tested at both C0 (should NOT appear) and max constellation (should appear)
- Verify `applied_buffs` contains expected buff names
- Verify `final_stats` reflect numerical changes
- Test values sourced from Honey Impact (no self-estimation)
- Existing tests for characters with C0 buffs must continue to pass

## Implementation Order

1. Pyro (8 chars, 14+ entries)
2. Hydro (8 chars, 17+ entries)
3. Electro (5 chars, 7 entries)
4. Cryo (7 chars, 10+ entries)
5. Dendro (8 chars, 14+ entries)
6. Anemo (4 chars, 7+ entries)
7. Geo (4 chars, 7 entries)
