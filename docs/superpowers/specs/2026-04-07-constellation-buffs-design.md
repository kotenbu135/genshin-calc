# Constellation Conditional Buffs: Comprehensive Implementation

**Issue:** #51
**Date:** 2026-04-07

## Overview

61 characters were identified as lacking constellation-dependent conditional buffs in `get_talent_conditional_buffs`. After investigation via Honey Impact and cross-referencing with existing implementations, **33 characters require ~62 new buff entries**. Several characters previously thought to be missing were already implemented.

### Already Implemented (no work needed)
amber(C6), thoma(C6), xinyan(C4), diona(C6), rosaria(C6), ganyu(C4 as `activation: None`), aloy, tartaglia, dahlia, fischl, lisa, kujou_sara, kuki_shinobu, zhongli, zibai, ifa, lan_yan, lynette

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

### Activation Policy

Existing codebase uses `activation: None` for constellation buffs that are assumed always active during their window (e.g., Amber C6, Thoma C6, Diona C6). New entries follow the same convention:
- `activation: None` — buff is always active when constellation is met (auto-applied during skill/burst window)
- `activation: Some(Toggle)` — buff requires manual activation (conditional on player action, enemy state, or positional requirement)
- `activation: Some(Stacks(N))` — buff has variable intensity

For consistency with existing entries, buffs tied to skill/burst duration windows use `activation: None` unless the user needs to choose whether the condition is met.

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
// Chevreuse C6: Pyro/Electro DMG +20% per stack (max 3)
TalentBuffDef {
    min_constellation: 6,
    activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
    base_value: 0.20,
    // ...
}
```

**Pattern 3: StatScaling**
```rust
// Nilou C6: per 1000 HP -> CRIT Rate +0.6%
TalentBuffDef {
    scales_on: Some(ScalingStat::Hp),
    base_value: 0.006, // per 1000 HP
    cap: Some(0.30),   // max 30%
    // ...
}
```

## Complete Implementation List

### Pyro (6 characters, 13 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| chevreuse | C6 | ElementalDmgBonus(Pyro) | 0.20 | Team | Stacks(3) | Per stack |
| chevreuse | C6 | ElementalDmgBonus(Electro) | 0.20 | Team | Stacks(3) | Paired with Pyro entry |
| durin | C4 | BurstDmgBonus | 0.40 | OnlySelf | Toggle | |
| durin | C6 | DefIgnore | 0.30 | OnlySelf | Toggle | Base DEF ignore |
| durin | C6 | DefReduction | 0.30 | Team | Toggle | Light form DEF shred |
| klee | C2 | DefReduction | 0.23 | Team | Toggle | Mine DEF reduction |
| klee | C6 | ElementalDmgBonus(Pyro) | 0.10 | Team | Toggle | Sparks 'n' Splash |
| xiangling | C1 | ElementalResReduction(Pyro) | 0.15 | Team | Toggle | Guoba hit |
| xiangling | C6 | ElementalDmgBonus(Pyro) | 0.15 | Team | Toggle | Pyronado duration |
| xinyan | C2 | CritRate | 1.00 | OnlySelf | Toggle | Burst Physical only; approximation (applies generically, same as Sara C6 pattern) |
| xinyan | C6 | ChargedAtkFlatDmg | 0.50 | OnlySelf | Toggle | scales_on: Def |
| yoimiya | C1 | AtkPercent | 0.20 | OnlySelf | Toggle | On Aurous Blaze kill |
| yoimiya | C2 | ElementalDmgBonus(Pyro) | 0.25 | OnlySelf | Toggle | On Pyro CRIT |

**Deferred Pyro:**
- Durin C1: Team flat DMG (scales_on: Atk × 0.60) + Self Burst flat DMG (scales_on: Atk × 1.50). Requires verifying that `team_builder.rs` correctly passes provider's stat when `target: Team` + `scales_on` are combined.
- Durin C2: Team Pyro DMG +50% + corresponding element DMG +50% on reaction trigger. Needs one entry per reaction-triggered element (similar to Sucrose C6 pattern).

### Hydro (8 characters, 17 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| aino | C1 | ElementalMastery | 80.0 | Team | Toggle | After Skill/Burst |
| aino | C6 | TransformativeBonus | 0.15 | Team | Toggle | Reaction DMG +15/20% |
| barbara | C2 | ElementalDmgBonus(Hydro) | 0.15 | Team | Toggle | Active char during Skill |
| candace | C2 | HpPercent | 0.20 | OnlySelf | Toggle | On Skill hit |
| columbina | C1 | TransformativeBonus | 0.015 | Team | Toggle | Lunar Reaction DMG |
| columbina | C2 | HpPercent | 0.40 | OnlySelf | Toggle | On Gravity Interference (always applies) |
| columbina | C2 | TransformativeBonus | 0.07 | Team | Toggle | Lunar DMG +7% (always applies with C2) |
| columbina | C4 | TransformativeBonus | 0.015 | Team | Toggle | Lunar DMG +1.5% |
| columbina | C6 | CritRate | 0.80 | Team | Toggle | For elemental DMG |
| columbina | C6 | TransformativeBonus | 0.07 | Team | Toggle | Lunar DMG +7% |
| mona | C1 | TransformativeBonus | 0.15 | Team | Toggle | Reaction DMG vs Omen |
| mona | C2 | ElementalMastery | 80.0 | Team | Toggle | On Charged ATK |
| mona | C4 | CritRate | 0.15 | Team | Toggle | vs Omen targets; approximation (game limits to party, applied generically) |
| mona | C4 | CritDmg | 0.15 | Team | Toggle | vs Omen targets; approximation (game limits to Hexerei party, applied generically) |
| mona | C6 | ChargedAtkDmgBonus | 1.80 | OnlySelf | Toggle | Max +180% |
| mualani | C4 | BurstDmgBonus | 0.75 | OnlySelf | Toggle | Boomsharka-laka +75% |
| nilou | C2 | ElementalResReduction(Hydro) | 0.35 | Team | Toggle | On Hydro DMG |
| nilou | C2 | ElementalResReduction(Dendro) | 0.35 | Team | Toggle | On Bloom DMG |
| nilou | C4 | BurstDmgBonus | 0.50 | OnlySelf | Toggle | After 3rd dance step |
| nilou | C6 | CritRate | (scaling) | OnlySelf | Toggle | scales_on: Hp, per 1000 HP +0.006, cap 0.30 |
| nilou | C6 | CritDmg | (scaling) | OnlySelf | Toggle | scales_on: Hp, per 1000 HP +0.012, cap 0.60 |
| xingqiu | C2 | ElementalResReduction(Hydro) | 0.15 | Team | Toggle | Sword rain hit |
| xingqiu | C4 | SkillDmgBonus | 0.50 | OnlySelf | Toggle | During Burst |
| yelan | C4 | HpPercent | 0.10 | Team | Stacks(4) | Per marked opponent |

**Deferred Hydro:**
- Columbina C2 reaction-dependent stats (ATK/EM/DEF based on dominant reaction type): Needs 3 separate Toggle entries, one per reaction type.

### Electro (5 characters, 8 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| beidou | C4 | ElementalDmgBonus(Electro) | 0.20 | OnlySelf | Toggle | Normal ATK Electro DMG on hit |
| flins | C2 | ElementalResReduction(Electro) | 0.25 | Team | Toggle | Ascendant Gleam Moonsign |
| flins | C6 | TransformativeBonus | 0.35 | OnlySelf | Toggle | Lunar-Charged DMG |
| flins | C6 | TransformativeBonus | 0.10 | Team | Toggle | Team Lunar-Charged DMG |
| iansan | C2 | AtkPercent | 0.30 | Team | Toggle | Off-field, active char |
| iansan | C6 | DmgBonus | 0.25 | Team | Toggle | On Nightsoul overflow |
| ineffa | C1 | TransformativeBonus | (scaling) | Team | Toggle | scales_on: TotalAtk, +0.025 per 100 ATK, cap 0.50 |
| yae_miko | C6 | DefIgnore | 0.60 | OnlySelf | Toggle | Sesshou Sakura attacks |

### Cryo (6 characters, 8 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| escoffier | C1 | CritDmg | 0.60 | Team | Toggle | Cryo CRIT DMG, requires 4 Hydro/Cryo |
| escoffier | C2 | SkillFlatDmg | (scaling) | Team | Toggle | scales_on: TotalAtk, ×2.40 |
| eula | C1 | PhysicalDmgBonus | 0.30 | OnlySelf | Toggle | On Grimheart consume |
| eula | C4 | BurstDmgBonus | 0.25 | OnlySelf | Toggle | vs <50% HP enemies |
| ganyu | C1 | ElementalResReduction(Cryo) | 0.15 | Team | Toggle | Frostflake hit |
| mika | C6 | CritDmg | 0.60 | Team | Toggle | Physical CRIT DMG |
| qiqi | C2 | NormalAtkDmgBonus | 0.15 | OnlySelf | Toggle | vs Cryo-affected |
| qiqi | C2 | ChargedAtkDmgBonus | 0.15 | OnlySelf | Toggle | vs Cryo-affected |
| rosaria | C1 | NormalAtkDmgBonus | 0.10 | OnlySelf | Toggle | On CRIT hit |
| shenhe | C2 | CritDmg | 0.15 | Team | Toggle | Cryo CRIT DMG in Burst field |

### Dendro (8 characters, 13 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| baizhu | C4 | ElementalMastery | 80.0 | Team | Toggle | After Burst |
| baizhu | C6 | SkillFlatDmg | (scaling) | OnlySelf | Toggle | scales_on: Hp, ×0.08 |
| collei | C4 | ElementalMastery | 60.0 | TeamExcludeSelf | Toggle | After Burst |
| kirara | C6 | DmgBonus | 0.12 | Team | Toggle | All Elemental DMG (approximation) |
| nahida | C2 | CritRate | 0.20 | Team | Toggle | Reaction CRIT |
| nahida | C2 | CritDmg | 1.00 | Team | Toggle | Reaction CRIT DMG |
| nahida | C2 | DefReduction | 0.30 | Team | Toggle | On Quicken/Aggravate/Spread |
| nahida | C4 | ElementalMastery | 100.0 | OnlySelf | Stacks(4) | +100 per nearby Skandha enemy |
| nefer | C2 | ElementalMastery | 200.0 | OnlySelf | Toggle | At 5 Veil stacks |
| nefer | C4 | ElementalResReduction(Dendro) | 0.20 | Team | Toggle | Shadow Dance state |
| nefer | C6 | TransformativeBonus | 0.15 | Team | Toggle | Lunar-Bloom at Ascendant Gleam |
| traveler_dendro | C6 | ElementalDmgBonus(Dendro) | 0.12 | Team | Toggle | Inside Lamp |
| yaoyao | C1 | ElementalDmgBonus(Dendro) | 0.15 | Team | Toggle | Radish explosion |
| yaoyao | C4 | ElementalMastery | (scaling) | OnlySelf | Toggle | scales_on: Hp, ×0.003, cap 120 |

**Deferred Dendro:**
- Nahida C1: Modifies A4 Burst element counting logic (+1 to each element count), not a simple buff entry. Requires conditional logic change in existing A4 implementation.
- Lauma C2: Bloom/Hyperbloom/Burgeon DMG scaling off EM (scales_on: Em). May need multiple entries for each reaction type. Also +40% Lunar-Bloom at Ascendant Gleam.
- Lauma C6: Lunar-Bloom DMG procs + team Lunar-Bloom DMG +25% at Ascendant Gleam. The +25% team bonus is implementable; the proc damage is SKIP.
- Nefer C1: Lunar-Bloom DMG +60% EM (scales_on: Em). Complex scaling.

### Anemo (4 characters, 10 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| faruzan | C6 | CritDmg | 0.40 | Team | Toggle | Anemo DMG CRIT DMG |
| jahoda | C6 | CritRate | 0.05 | Team | Toggle | Moonsign characters |
| jahoda | C6 | CritDmg | 0.40 | Team | Toggle | Moonsign characters |
| kazuha | C2 | ElementalMastery | 200.0 | Team | Toggle | In Burst field |
| kazuha | C6 | NormalAtkDmgBonus | (scaling) | OnlySelf | Toggle | scales_on: Em, ×0.002 |
| kazuha | C6 | ChargedAtkDmgBonus | (scaling) | OnlySelf | Toggle | scales_on: Em, ×0.002 |
| kazuha | C6 | PlungingAtkDmgBonus | (scaling) | OnlySelf | Toggle | scales_on: Em, ×0.002 |
| venti | C4 | ElementalDmgBonus(Anemo) | 0.25 | OnlySelf | Toggle | On pickup |
| venti | C6 | ElementalResReduction(Anemo) | 0.20 | Team | Toggle | Wind's Grand Ode hit |
| xianyun | C2 | AtkPercent | 0.20 | OnlySelf | Toggle | After Skyladder |
| xianyun | C6 | CritDmg | 0.70 | OnlySelf | Toggle | Max Driftcloud Wave CRIT DMG |

**Deferred Anemo:**
- Sucrose C6: Needs one entry per absorbable element (Pyro/Hydro/Electro/Cryo), each +20% ElementalDmgBonus. 4 entries total.
- Venti C6 self CRIT DMG: Game text is ambiguous (some sources say +100%, others vary). Needs verification from in-game data before implementation. Deferred until confirmed.

### Geo (4 characters, 6 entries)

| Character | Const | Stat | Value | Target | Activation | Notes |
|-----------|-------|------|-------|--------|------------|-------|
| albedo | C1 | DefPercent | 0.50 | OnlySelf | Toggle | On Skill use |
| albedo | C4 | PlungingAtkDmgBonus | 0.30 | Team | Toggle | In Solar Isotoma |
| albedo | C6 | DmgBonus | 0.17 | Team | Toggle | With Crystallize shield |
| gorou | C6 | CritDmg | 0.40 | Team | Toggle | Geo CRIT DMG at Crunch |
| illuga | C4 | DefFlat | 200.0 | Team | Toggle | During Burst |
| yun_jin | C2 | NormalAtkDmgBonus | 0.15 | Team | Toggle | After Burst |
| yun_jin | C4 | DefPercent | 0.20 | OnlySelf | Toggle | On Crystallize |

**Deferred Geo:**
- Ningguang C4: Team all Elemental RES +10%. Requires 7 entries (one per element: Pyro/Hydro/Electro/Cryo/Dendro/Anemo/Geo). Straightforward but verbose.
- Albedo C2: Burst DMG scaling off DEF x consumed stacks (max 4). Complex mechanic requiring stack-based StatScaling.

## Deferred / Complex Cases Summary

| Case | Reason | Effort |
|------|--------|--------|
| Nahida C1 | Modifies A4 counting logic, not a buff entry | Medium |
| Columbina C2 ATK/EM/DEF | 3 stats conditional on reaction type | Low |
| Durin C1 | Team flat DMG via provider's ATK StatScaling | Medium (pipeline verification) |
| Durin C2 | Multi-element DMG bonus on reaction | Low (Sucrose C6 pattern) |
| Albedo C2 | DEF x stacks burst DMG | Medium |
| Lauma C2 | Reaction DMG scaling off EM, multiple reaction types | Medium |
| Lauma C6 | Team Lunar-Bloom +25% (implementable) + proc (skip) | Low |
| Nefer C1 | Lunar-Bloom DMG scaling off EM | Low |
| Sucrose C6 | 4 element entries | Low |
| Ningguang C4 | 7 element RES entries | Low |
| Venti C6 CRIT DMG | Value needs in-game verification | Low |

## Testing Strategy

Per `CLAUDE.md` testing requirements:
- Each character's constellation buffs tested at both C0 (should NOT appear) and max constellation (should appear)
- Verify `applied_buffs` contains expected buff names
- Verify `final_stats` reflect numerical changes
- Test values sourced from Honey Impact (no self-estimation)
- Existing tests for characters with C0 buffs must continue to pass
- For characters where existing `activation: None` entries are being replaced with `Stacks(N)`, verify that 0 stacks contributes 0 value (regression test)

## Implementation Order

1. Pyro (6 chars, 13 entries)
2. Hydro (8 chars, 17 entries)
3. Electro (5 chars, 8 entries)
4. Cryo (6 chars, 8 entries)
5. Dendro (8 chars, 13 entries)
6. Anemo (4 chars, 10 entries)
7. Geo (4 chars, 6 entries)
