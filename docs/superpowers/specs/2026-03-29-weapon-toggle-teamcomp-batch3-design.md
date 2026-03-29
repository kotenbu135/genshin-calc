# P3 Batch 3: 5-Star Weapon Toggle/TeamComp ConditionalBuff Design

**Date**: 2026-03-29
**Status**: Reviewed (v2)
**Depends on**: P0 (ConditionalBuff), P3 Batch 1 (StatScaling), P3 Batch 2 (Stacks)
**Scope**: 13 five-star weapons with Toggle/TeamComp/Both ConditionalBuff (data-only, no core changes)

## Overview

Add ConditionalBuff definitions to 13 five-star weapons using existing `ManualCondition::Toggle`, `AutoCondition::TeamSameElementCount`/`TeamDiffElementCount`, and `Activation::Both` patterns. No core crate changes required.

## Weapon Definitions

### 1. Athame Artis (Sword) — Toggle

```
ConditionalBuff 1:
  name: "athame_artis_skill_cr"
  stat: CritRate
  R1-R5: [0.10, 0.125, 0.15, 0.175, 0.20] (TBV)
  activation: Manual(Toggle)

ConditionalBuff 2:
  name: "athame_artis_skill_dmg"
  stat: SkillDmgBonus
  R1-R5: [0.08, 0.10, 0.12, 0.14, 0.16] (TBV)
  activation: Manual(Toggle)
```

Elemental Skill CritRate and Skill DMG up after using Skill. `CritRate` used as approximation for Skill-only CR (no `SkillCritRate` variant exists; over-buffs non-skill attacks). `SkillDmgBonus` is exact match for Skill DMG portion.

### 2. Azurelight (Sword) — Both(StatScaling, Toggle)

```
ConditionalBuff:
  name: "azurelight_hp_na_dmg"
  stat: NormalAtkDmgBonus
  R1-R5: [0.0016, 0.002, 0.0024, 0.0028, 0.0032] (TBV — multiplier per HP%)
  activation: Both(StatScaling { stat: HpPercent, offset: None, cap: Some([0.40, 0.50, 0.60, 0.70, 0.80]) }, Toggle)
```

NA DMG bonus based on Max HP. Requires Toggle (HP condition or on-field). Per-refinement cap.

### 3. Lightbearing Moonshard (Sword) — Toggle

```
ConditionalBuff 1:
  name: "lightbearing_atk"
  stat: AtkPercent
  R1-R5: [0.24, 0.30, 0.36, 0.42, 0.48] (TBV)
  activation: Manual(Toggle)

ConditionalBuff 2:
  name: "lightbearing_dmg"
  stat: DmgBonus
  R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40] (TBV)
  activation: Manual(Toggle)
```

Moonlight power grants ATK% and DMG%. Exact values TBV.

### 4. Beacon of the Reed Sea (Claymore) — Multi-Toggle

```
ConditionalBuff 1:
  name: "beacon_skill_atk"
  stat: AtkPercent
  R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
  activation: Manual(Toggle)
  description: "元素スキル命中後にATK+20-40%"

ConditionalBuff 2:
  name: "beacon_dmg_taken_atk"
  stat: AtkPercent
  R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
  activation: Manual(Toggle)
  description: "ダメージを受けた後にATK+20-40%"

ConditionalBuff 3:
  name: "beacon_shield_hp"
  stat: HpPercent
  R1-R5: [0.32, 0.40, 0.48, 0.56, 0.64]
  activation: Manual(Toggle)
  description: "シールド保護時にHP+32-64%"
```

Three independent conditions, each a separate Toggle.

### 5. Gest of the Mighty Wolf (Claymore) — Toggle

```
ConditionalBuff 1:
  name: "gest_wolf_dmg"
  stat: DmgBonus
  R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32] (TBV)
  activation: Manual(Toggle)

ConditionalBuff 2:
  name: "gest_wolf_atk"
  stat: AtkPercent
  R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32] (TBV)
  activation: Manual(Toggle)
```

Wolf power grants DMG% and ATK%. Exact values and conditions TBV.

### 6. Elegy for the End (Bow) — TeamBuff Toggle

```
StatBuff (move from conditional to always-on):
  stat: ElementalMastery
  value: 60.0
  refinement_values: Some([60.0, 75.0, 90.0, 105.0, 120.0])

ConditionalBuff 1:
  name: "elegy_team_em"
  stat: ElementalMastery
  R1-R5: [100.0, 125.0, 150.0, 175.0, 200.0]
  target: BuffTarget::Team
  activation: Manual(Toggle)

ConditionalBuff 2:
  name: "elegy_team_atk"
  stat: AtkPercent
  R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
  target: BuffTarget::Team
  activation: Manual(Toggle)
```

Always-on EM+60 as StatBuff. Sigil accumulation trigger activates team-wide EM+100 and ATK+20%.
Update weapon description from "Conditional: EM+60。追憶の印蓄積でチーム全員にEM+100/ATK+20%" to "EM+60-120。追憶の印蓄積でチーム全員にEM+100-200/ATK+20-40%".

### 7. The First Great Magic (Bow) — TeamComp

```
ConditionalBuff 1:
  name: "first_great_magic_atk_1"
  stat: AtkPercent
  R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
  activation: Auto(TeamDiffElementCount { min_count: 1 })

ConditionalBuff 2:
  name: "first_great_magic_atk_2"
  stat: AtkPercent
  R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
  activation: Auto(TeamDiffElementCount { min_count: 2 })

ConditionalBuff 3:
  name: "first_great_magic_atk_3"
  stat: AtkPercent
  R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
  activation: Auto(TeamDiffElementCount { min_count: 3 })
```

CA DMG+16-32% is already in StatBuff. ATK% scales with number of different-element team members (each adds one stack). Modeled as 3 overlapping AutoCondition buffs — if 2 different elements, buffs 1+2 both activate = 2× value.

### 8. A Thousand Floating Dreams (Catalyst) — TeamComp

```
ConditionalBuff 1-3 (same element EM, overlapping):
  name: "thousand_dreams_same1_em" / "thousand_dreams_same2_em" / "thousand_dreams_same3_em"
  stat: ElementalMastery
  R1-R5: [32.0, 40.0, 48.0, 56.0, 64.0]
  activation: Auto(TeamSameElementCount { min_count: 1 / 2 / 3 })

ConditionalBuff 4-6 (different element DMG%, overlapping):
  name: "thousand_dreams_diff1_dmg" / "thousand_dreams_diff2_dmg" / "thousand_dreams_diff3_dmg"
  stat: DmgBonus
  R1-R5: [0.10, 0.14, 0.18, 0.22, 0.26] (TBV)
  activation: Auto(TeamDiffElementCount { min_count: 1 / 2 / 3 })

ConditionalBuff 7 (team EM buff):
  name: "thousand_dreams_team_em"
  stat: ElementalMastery
  R1-R5: [40.0, 44.0, 48.0, 52.0, 56.0] (TBV)
  target: BuffTarget::TeamExcludeSelf
  activation: Manual(Toggle)
```

Per same-element team member: EM+32. Per different-element: DMG+10%. Max 3 members counted. Uses overlapping AutoCondition pattern (same as Gilded Dreams in artifacts.rs). Team EM buff to other party members modeled as Toggle with TeamExcludeSelf target.

### 9. Jadefall's Splendor (Catalyst) — Toggle

```
ConditionalBuff:
  name: "jadefall_em"
  stat: ElementalMastery
  R1-R5: [32.0, 40.0, 48.0, 56.0, 64.0] (TBV)
  activation: Manual(Toggle)
```

After consuming Elemental Energy, grants EM. Single Toggle. Note: actual game mechanic may vary EM by element count in party — verify during TBV pass. If so, expand to multiple overlapping AutoCondition buffs.

### 10. Nightweaver's Looking Glass (Catalyst) — Toggle

```
ConditionalBuff 1:
  name: "nightweaver_na_dmg"
  stat: NormalAtkDmgBonus
  R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32] (TBV)
  activation: Manual(Toggle)

ConditionalBuff 2:
  name: "nightweaver_ca_dmg"
  stat: ChargedAtkDmgBonus
  R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32] (TBV)
  activation: Manual(Toggle)
```

Nightsoul consumption grants NA/CA DMG bonus.

### 11. Reliquary of Truth (Catalyst) — Toggle

```
ConditionalBuff:
  name: "reliquary_truth_dmg"
  stat: DmgBonus
  R1-R5: [0.12, 0.18, 0.24, 0.30, 0.36] (TBV)
  activation: Manual(Toggle)
```

After triggering elemental reaction, DMG bonus.

### 12. Starcaller's Watch (Catalyst) — Toggle

```
ConditionalBuff:
  name: "starcaller_dmg"
  stat: DmgBonus
  R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40] (TBV)
  activation: Manual(Toggle)
```

When team triggers elemental reaction, DMG bonus.

### 13. Sunny Morning Sleep-In (Catalyst) — Toggle

```
ConditionalBuff 1:
  name: "sunny_morning_atk"
  stat: AtkPercent
  R1-R5: [0.14, 0.175, 0.21, 0.245, 0.28] (TBV)
  activation: Manual(Toggle)

ConditionalBuff 2:
  name: "sunny_morning_dmg"
  stat: DmgBonus
  R1-R5: [0.18, 0.225, 0.27, 0.315, 0.36] (TBV)
  activation: Manual(Toggle)
```

Elemental reaction grants ATK% and DMG%.

## Notes

- **(TBV)** = To Be Verified against HoneyhunterWorld/Ambr.top during implementation.
- All percentage values in decimal form (10% = 0.10).
- `target: BuffTarget::OnlySelf` for all buffs unless specified (Elegy = `BuffTarget::Team`).
- `stack_values: None` for all (no stacks in this batch).
- `SkillCritRate` BuffableStat: verify existence. If missing, use `CritRate` with approximation note.
- Elegy: `buffs` array changes from `&[]` to include always-on EM StatBuff.
- TeamComp weapons (First Great Magic, Thousand Floating Dreams): verify `TeamDiffElementCount`/`TeamSameElementCount` behavior with overlapping buffs during implementation.

## Impact

### Files Modified

| File | Changes |
|------|---------|
| `crates/data/src/weapons/sword.rs` | Athame (2), Azurelight (1), Lightbearing (2) ConditionalBuffs |
| `crates/data/src/weapons/claymore.rs` | Beacon (3), Gest (2) ConditionalBuffs |
| `crates/data/src/weapons/bow.rs` | Elegy (2 + StatBuff change), First Great Magic (3) ConditionalBuffs |
| `crates/data/src/weapons/catalyst.rs` | Thousand Dreams (7), Jadefall (1), Nightweaver (2), Reliquary (1), Starcaller (1), Sunny (2) ConditionalBuffs |

### Core Crate

No changes required.

## Test Plan

13 unit tests (one per weapon):
- Verify ConditionalBuff count and names
- Verify stat type matches expected BuffableStat
- Verify R1 value
- Verify Activation type (Toggle / Auto / Both)
- For Elegy: verify StatBuff EM exists + 2 Team ConditionalBuffs
- For TeamComp: verify AutoCondition variant and min_count

## Out of Scope

- Shield-dependent weapons (Summit Shaper, The Unforged, Vortex Vanquisher, Memory of Dust)
- Placeholder weapons (Bloodsoaked Ruins, Crimson Moon's Semblance, Fractured Halo, Symphonist of Scents)
- Attack Speed buffs
- Proc damage (追加ダメージ弾, 真空の刃, etc.)
