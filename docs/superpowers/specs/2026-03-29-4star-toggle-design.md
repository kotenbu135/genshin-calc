# P3: 4-Star Weapon Toggle ConditionalBuff Design

**Date**: 2026-03-29
**Status**: Draft
**Depends on**: P0 (ConditionalBuff system)
**Scope**: 32 four-star weapons with Toggle ConditionalBuff (data-only, no core changes)

## Overview

Add `ManualCondition::Toggle` ConditionalBuff definitions to 32 four-star weapons. All use existing framework. No core crate changes required.

## Pattern

All weapons use the same activation:

```rust
ConditionalBuff {
    name: "<weapon_id>_<stat>",
    stat: BuffableStat::Xxx,
    value: <r1_value>,
    refinement_values: Some([R1, R2, R3, R4, R5]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}
```

## 32 Weapon Definitions

### Swords (9)

**1. Cinnabar Spindle** — DEF-based Skill DMG
```
name: "cinnabar_skill_def_dmg"
stat: SkillDmgBonus  (approximation — actual is flat DEF×40% added to skill DMG)
R1-R5: [0.40, 0.50, 0.60, 0.70, 0.80] (TBV — may need StatScaling instead of Toggle)
```
Note: Cinnabar's actual effect adds DEF×40% as flat damage to Skill. This may better fit `Both(StatScaling{DefPercent}, Toggle)` pattern. Verify during implementation.

**2. Festering Desire** — Skill CR + Skill DMG
```
buff1: name: "festering_skill_dmg", stat: SkillDmgBonus, R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
buff2: name: "festering_skill_cr", stat: CritRate, R1-R5: [0.06, 0.075, 0.09, 0.105, 0.12]
```

**3. Flute of Ezpitzal** — DEF-based DMG bonus
```
name: "flute_ezpitzal_def_dmg"
stat: DmgBonus (approximation — actual is DEF-based)
R1-R5: TBV — may need StatScaling
```
Note: Similar to Cinnabar. Verify if StatScaling or flat Toggle during implementation.

**4. Lion's Roar** — DMG bonus vs Pyro/Electro affected enemies
```
name: "lions_roar_dmg", stat: DmgBonus, R1-R5: [0.20, 0.24, 0.28, 0.32, 0.36]
```

**5. Prized Isshin Blade** — ATK% after Skill hit
```
name: "prized_isshin_atk", stat: AtkPercent, R1-R5: TBV
```

**6. Royal Longsword** — CR stacking (resets on crit)
```
name: "royal_longsword_cr", stat: CritRate, R1-R5: [0.08, 0.10, 0.12, 0.14, 0.16]
```
Note: Actually a unique stacking mechanic (resets on crit). Modeled as Toggle with max value for simplicity.

**7. Serenity's Call** — DMG bonus after Skill
```
name: "serenitys_call_dmg", stat: DmgBonus, R1-R5: TBV
```

**8. Sturdy Bone** — NA DMG after sprint
```
name: "sturdy_bone_na_dmg", stat: NormalAtkDmgBonus, R1-R5: TBV
```

**9. Toukabou Shigure** — DMG bonus after plunge hit
```
name: "toukabou_dmg", stat: DmgBonus, R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
```

### Claymores (3)

**10. Earth Shaker** — Burst DMG after Skill hit
```
name: "earth_shaker_burst_dmg", stat: BurstDmgBonus, R1-R5: TBV
```

**11. Luxurious Sea-Lord** — Burst DMG
```
name: "sea_lord_burst_dmg", stat: BurstDmgBonus, R1-R5: [0.12, 0.15, 0.18, 0.21, 0.24]
```

**12. Master Key** — EM after Skill
```
name: "master_key_em", stat: ElementalMastery, R1-R5: TBV
```

### Bows (9)

**13. Amos' Bow** — DMG bonus (arrow travel stacks, modeled as Toggle at max)
```
name: "amos_arrow_dmg", stat: ChargedAtkDmgBonus, R1-R5: [0.08, 0.10, 0.12, 0.14, 0.16] (per 0.1s, max 5)
```
Note: Actually 5-level distance scaling. Could be Stacks(5) instead. Verify.

**14. Fading Twilight** — Cycling DMG bonus (3 states)
```
name: "fading_twilight_dmg", stat: DmgBonus, R1-R5: [0.12, 0.15, 0.18, 0.21, 0.24] (max state)
```
Note: 3 cycling states (6%/10%/14% at R1). Modeled as Toggle at max state. Could use Stacks(3) with stack_values for exact modeling.

**15. Hamayumi** — NA/CA DMG + energy-full bonus
```
buff1: name: "hamayumi_na_dmg", stat: NormalAtkDmgBonus, R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
buff2: name: "hamayumi_ca_dmg", stat: ChargedAtkDmgBonus, R1-R5: [0.12, 0.15, 0.18, 0.21, 0.24]
buff3: name: "hamayumi_full_energy_na", stat: NormalAtkDmgBonus, R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
buff4: name: "hamayumi_full_energy_ca", stat: ChargedAtkDmgBonus, R1-R5: [0.12, 0.15, 0.18, 0.21, 0.24]
```
Note: Base NA/CA DMG as StatBuff + energy-full bonus as Toggle. Reclassify: base goes to StatBuff, full-energy goes to Toggle.

**16. Mitternachts Waltz** — Cross-buff (NA→Skill DMG, Skill→NA DMG)
```
buff1: name: "mitternachts_skill_dmg", stat: SkillDmgBonus, R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
buff2: name: "mitternachts_na_dmg", stat: NormalAtkDmgBonus, R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
```

**17. Predator** — NA/CA DMG after Cryo hit
```
buff1: name: "predator_na_dmg", stat: NormalAtkDmgBonus, R1-R5: [0.10, 0.10, 0.10, 0.10, 0.10]
buff2: name: "predator_ca_dmg", stat: ChargedAtkDmgBonus, R1-R5: [0.10, 0.10, 0.10, 0.10, 0.10]
```
Note: Predator has no refinement scaling (PS exclusive). Fixed 10% at all refinements.

**18. Prototype Crescent** — ATK% after weak-point hit
```
name: "prototype_crescent_atk", stat: AtkPercent, R1-R5: [0.36, 0.45, 0.54, 0.63, 0.72]
```

**19. Sequence of Solitude** — Skill/Burst DMG on HP change
```
buff1: name: "sequence_skill_dmg", stat: SkillDmgBonus, R1-R5: TBV
buff2: name: "sequence_burst_dmg", stat: BurstDmgBonus, R1-R5: TBV
```

**20. Snare Hook** — DMG after Skill hit
```
name: "snare_hook_dmg", stat: DmgBonus, R1-R5: TBV
```

**21. Windblume Ode** — ATK% after Skill
```
name: "windblume_atk", stat: AtkPercent, R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
```

### Catalysts (11)

**22. Blackmarrow Lantern** — DMG on Nightsoul consume
```
name: "blackmarrow_dmg", stat: DmgBonus, R1-R5: TBV
```

**23. Dawning Frost** — DMG after Skill hit
```
name: "dawning_frost_dmg", stat: DmgBonus, R1-R5: TBV
```

**24. Dodoco Tales** — CA DMG after NA / ATK after CA
```
buff1: name: "dodoco_ca_dmg", stat: ChargedAtkDmgBonus, R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
buff2: name: "dodoco_atk", stat: AtkPercent, R1-R5: [0.08, 0.10, 0.12, 0.14, 0.16]
```

**25. Etherlight Spindlelute** — DMG after Burst
```
name: "etherlight_dmg", stat: DmgBonus, R1-R5: TBV
```

**26. Flowing Purity** — DMG on HP consumption
```
name: "flowing_purity_dmg", stat: DmgBonus, R1-R5: TBV
```

**27. Mappa Mare** — ER after Skill (actually this is already implemented as Stacks)
```
SKIP — already has ConditionalBuff (Stacks)
```
Note: Remove from scope — Mappa Mare already implemented.

**28. Sacrificial Jade** — HP% + EM on field
```
buff1: name: "sacrificial_jade_hp", stat: HpPercent, R1-R5: [0.32, 0.40, 0.48, 0.56, 0.64]
buff2: name: "sacrificial_jade_em", stat: ElementalMastery, R1-R5: [40.0, 50.0, 60.0, 70.0, 80.0]
```

**29. Solar Pearl** — Cross-buff (NA→Skill/Burst DMG, Skill/Burst→NA DMG)
```
buff1: name: "solar_pearl_skill_burst_dmg", stat: DmgBonus, R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
buff2: name: "solar_pearl_na_dmg", stat: NormalAtkDmgBonus, R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
```
Note: Solar Pearl's first buff is Skill/Burst DMG. Using `DmgBonus` as approximation (slightly over-buffs NA).

**30. The Widsith** — Random buff (ATK% / Elemental DMG / EM)
```
buff1: name: "widsith_atk", stat: AtkPercent, R1-R5: [0.60, 0.75, 0.90, 1.05, 1.20]
buff2: name: "widsith_dmg", stat: DmgBonus, R1-R5: [0.48, 0.60, 0.72, 0.84, 0.96]
buff3: name: "widsith_em", stat: ElementalMastery, R1-R5: [240.0, 300.0, 360.0, 420.0, 480.0]
```
Note: Only one activates at a time (random). User toggles the one they want to simulate.

**31. Wine and Song** — ATK% after dash
```
name: "wine_and_song_atk", stat: AtkPercent, R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
```

**Revised total: 31 weapons** (Mappa Mare removed, Sword of Narzissenkreuz removed — too niche)

## Notes

- **(TBV)** = To Be Verified against game data sources during implementation.
- 4-star refinement scaling varies: some use ×[1.0, 1.25, 1.5, 1.75, 2.0], others vary. Verify per weapon.
- `target: BuffTarget::OnlySelf` for all.
- Weapons with DEF-scaling effects (Cinnabar, Flute of Ezpitzal) may need `Both(StatScaling, Toggle)` — implementer should verify and use appropriate Activation.
- Amos' Bow and Fading Twilight have non-standard mechanics — implementer may use Stacks instead of Toggle if more accurate.
- Royal Longsword's crit-reset mechanic is approximated as Toggle at max CR value.

## Impact

### Files Modified

| File | Changes |
|------|---------|
| `crates/data/src/weapons/sword.rs` | 9 weapons |
| `crates/data/src/weapons/claymore.rs` | 3 weapons |
| `crates/data/src/weapons/bow.rs` | 9 weapons |
| `crates/data/src/weapons/catalyst.rs` | 10 weapons |

### Core Crate

No changes required.

## Test Plan

31 unit tests (one per weapon). Each verifies ConditionalBuff count, stat type, R1 value, and Toggle activation.

## Out of Scope

- Proc damage weapons (追加ダメージ, 範囲ダメージ, ダメージ弾)
- Particle generation (Favonius series)
- CD reset (Sacrificial series)
- HP recovery
- Shield generation
- Placeholder weapons (7 polearms with "条件付きバフ効果")
- Mappa Mare, Iron Sting (already implemented)
