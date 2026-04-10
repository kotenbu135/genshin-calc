# Cryo Buffs Audit

**Source**: `crates/data/src/talent_buffs/cryo.rs`
**Mirror**: `honeyhunter-mirror/md/characters/`
**Date**: 2026-04-08

---

## Summary

| Character | Buffs Implemented | Status |
|-----------|------------------|--------|
| Aloy | A1 (self ATK+16%), A1 (team ATK+8%), A4 (Cryo DMG+35%) | OK |
| Ayaka | A1 (NA+CA DMG+30%), A4 (Cryo DMG+18%), C4 (DEF-30%), C6 (CA DMG+298%) | OK |
| Charlotte | C2 (ATK+10%/stack, max 3), C4 (Burst DMG+10%) | **ISSUE** |
| Chongyun | A4 (Cryo RES-10%), C6 (DMG+15%) | **ISSUE** |
| Citlali | Skill (Pyro/Hydro RES-20%), C2 (EM+250, extra RES-20%), C6 (Pyro/Hydro DMG+1.5%/stack) | **ISSUE** |
| Diona | C6 (EM+200) | OK |
| Escoffier | C1 (Cryo CRIT DMG+60%), C2 (Skill flat DMG = 240% ATK) | **ISSUE** |
| Eula | Skill (Cryo/Phys RES-25%/stack), C1 (Phys DMG+30%), C4 (Burst DMG+25%) | **VALUE DISCREPANCY** |
| Freminet | A4 (Shatter DMG+40%), C1 (CR+15%), C4 (ATK+9%/stack), C6 (CD+12%/stack) | **ISSUE** |
| Ganyu | A4 (Cryo DMG+20%), C1 (Cryo RES-15%), C4 (DMG+25% max) | OK |
| Kaeya | C1 (CR+15% vs Cryo) | **SCOPE ISSUE** |
| Layla | A1 (Shield Str+24%), A4 (Shooting Star flat DMG), C4 (NA+CA flat DMG), C6 (Shooting Star DMG+40%) | **ISSUE** |
| Mika | C6 (Phys DMG+10%), C6 (Phys CRIT DMG+60%) | **MISSING BUFF** |
| Qiqi | C2 (NA+CA DMG+15%) | OK |
| Rosaria | A4 (CRIT Rate share 15%), C1 (NA DMG+10%), C6 (Phys RES-20%) | **VALUE DISCREPANCY** |
| Shenhe | Skill (flat Cryo DMG scaling), A4 (Skill/Burst DMG+15%, NA/CA/Plunge DMG+15%), C2 (Cryo CRIT DMG+15%) | **ISSUE** |
| Skirk | C2 (ATK+70%), C4 (ATK+40% max) | **ISSUE** |
| Wriothesley | A4 (ATK+6%/stack, max 5), C6 (CR+10%, CD+80%) | OK |

---

## Value Discrepancies

### Eula — Icetide Vortex RES Shred (TALENT-LEVEL-DEPENDENT)

**Implementation** (`cryo.rs` lines 437-461):
```
base_value: 0.25  // per stack, Stacks(2)
```
Comment says "max 2 Grimheart stacks" but description says "-25% per Grimheart stack".

**Mirror** (`eula_051.md`): The RES decrease is talent-level-dependent:
- Lv1: **16%** per stack
- Lv5: **20%** per stack
- Lv10+: **25%** per stack (max)

**Verdict**: The implementation uses `0.25` (Lv10 value), treating it as max-talent. This is an **acceptable simplification** (adopting max value, consistent with other talent-scaled buffs in the codebase), but the comment in the code misleadingly says "max 2 Grimheart stacks" without clarifying it's also the max-talent value. Not a bug in practice.

### Rosaria — C1 "Unholy Revelation" (WRONG EFFECT)

**Implementation** (`cryo.rs` lines 134-148):
```
name: "rosaria_c1_normal_dmg"
stat: BuffableStat::NormalAtkDmgBonus
base_value: 0.10
```
Description in code: "C1: Normal ATK DMG +10% on CRIT hit"

**Mirror** (`rosaria_045.md`):
> "When Rosaria deals a CRIT Hit, her ATK SPD increases by 10% and her Normal Attack DMG increases by 10% for 4s."

**Verdict**: The Normal ATK DMG +10% value is correct. However, the mirror also grants **ATK SPD +10%**. ATK SPD is typically excluded from damage calculators as it affects animation speed, not raw damage multipliers, so this omission is acceptable by design. The buff value itself (10%) is correct.

### Mika — C6 "Companion's Counsel" (MISSING CONDITION DETAIL)

**Implementation** (`cryo.rs` lines 510-524):
```
name: "mika_c6_physical_crit_dmg"
stat: BuffableStat::CritDmg
base_value: 0.60
```

**Mirror** (`mika_080.md`):
> "Additionally, active characters affected by Soulwind will deal 60% more Physical CRIT DMG."

The CRIT DMG buff is for **Physical CRIT DMG specifically**, not general CRIT DMG. The implementation uses `BuffableStat::CritDmg` which is a general CRIT DMG bonus. This could over-apply the buff to all elements.

**Verdict**: Potential over-application. Ideally this should be `PhysicalCritDmg` if that stat variant exists. This is worth investigating whether the stat type `CritDmg` correctly narrows to Physical only or is element-agnostic.

---

## Missing Implementations

### Charlotte — A4 "Diversified Investigation" (MISSING)

**Mirror** (`charlotte_088.md`):
> "When the party contains 1/2/3 non-Fontainians, Charlotte gains a 5%/10%/15% Cryo DMG Bonus."

This is a straightforward Cryo DMG bonus (up to +15%) based on party composition. It affects Charlotte's damage output and is damage-relevant.

**Verdict**: Missing implementation. Should add a buff for `ElementalDmgBonus(Element::Cryo)` with `base_value: 0.05`, `Stacks(3)`, `target: OnlySelf`, `source: AscensionPassive(4)`.

### Chongyun — C2 "Atmospheric Revolution" (MISSING)

**Mirror** (`chongyun_036.md`):
> "Elemental Skills and Elemental Bursts cast within the Frost Field created by Spirit Blade: Chonghua's Layered Frost have their CD time decreased by 15%."

CD reduction is a utility effect, but in practice the C6 "Rally of Four Blades" description in the implementation is arguably wrong in scope (see below). C2 is a CD reduction — excluded per task scope.

**Verdict**: Correctly excluded (CD reduction, utility).

### Chongyun — C6 "Rally of Four Blades" (WRONG CONDITION)

**Implementation** (`cryo.rs` lines 709-722):
```
description: "C6: DMG+15% (self)"
stat: BuffableStat::DmgBonus
base_value: 0.15
target: BuffTarget::OnlySelf
```

**Mirror** (`chongyun_036.md`):
> "Spirit Blade: Cloud-Parting Star deals 15% more DMG to opponents with a lower percentage of their Max HP remaining than Chongyun. This skill will also summon 1 additional spirit blade."

The buff only applies to **Cloud-Parting Star's burst damage** when enemies are below Chongyun's HP%. It is **not** a general self DMG bonus. The implementation treats it as a general DmgBonus (self), which over-applies it.

**Verdict**: Implementation overgeneralizes. Should be `BurstDmgBonus` at minimum, with a Toggle for the HP condition. Currently applying as a general `DmgBonus` to all damage types.

### Citlali — C6 "Secret Pact" includes self DMG bonus (MISSING)

**Implementation** (`cryo.rs` lines 396-427):
Only implements Pyro DMG +1.5%/stack and Hydro DMG +1.5%/stack for team.

**Mirror** (`citlali_107.md`):
> "each point Citlali possesses will grant all nearby party members a 1.5% Pyro and Hydro DMG Bonus, **and increase the DMG Citlali deals by 2.5%**."

The +2.5% per count self DMG bonus for Citlali is missing.

**Verdict**: Missing self DMG buff. Should add `DmgBonus` +0.025/stack (max 40 stacks) targeting `OnlySelf` from C6.

### Escoffier — Duplicate C1 Entry (BUG)

**Implementation** (`cryo.rs` lines 728-770):
There are **two separate TalentBuffDef entries** for C1:
1. `"Amuse-bouche de Saveur Cryo CD"` — always-on, no activation
2. `"escoffier_c1_cryo_crit_dmg"` — Toggle, "requires 4 Hydro/Cryo party members"

**Mirror** (`escoffier_112.md`):
> C1 only activates "When 4 party members are Hydro or Cryo"

The first entry (always-on, no activation) is incorrect and double-counts the C1 buff. The C1 bonus requires the 4-Hydro/Cryo condition.

**Verdict**: BUG — the always-on `"Amuse-bouche de Saveur Cryo CD"` entry should be removed. Only the Toggle version with the condition note is correct.

### Escoffier — A4 "Inspiration-Immersed Seasoning" (MISSING)

**Mirror** (`escoffier_112.md`):
> "When there are 1/2/3/4 Hydro or Cryo characters in the party, Escoffier will decrease the Hydro RES and Cryo RES of any opponents hit by her Elemental Skill or Elemental Burst by 5%/10%/15%/55% for 12s."

This A4 passive provides Hydro RES and Cryo RES reduction — a significant damage-relevant buff. The 4-Hydro/Cryo party case gives 55% reduction for both Hydro and Cryo RES.

**Verdict**: Missing implementation. Should add two buffs (`ElementalResReduction(Hydro)` and `ElementalResReduction(Cryo)`) with Stacks(4) × 0.55 max or simplified to the max value, `source: AscensionPassive(4)`.

Note: The C1 conditional Cryo CRIT DMG buff also depends on A4 being unlocked (the mirror states "You must first unlock the Ascension Talent 'Inspiration-Immersed Seasoning'").

### Freminet — C1 Scope Incorrect (WRONG SCOPE)

**Implementation** (`cryo.rs` lines 793-805):
```
name: "Dreams of the Foamy Deep CR"
stat: BuffableStat::CritRate
base_value: 0.15
```

**Mirror** (`freminet_085.md`):
> "The CRIT Rate of Pressurized Floe: Shattering Pressure will be increased by 15%."

This is a CRIT Rate bonus **specifically for Shattering Pressure (Skill)**, not a general CRIT Rate bonus. The implementation applies it as a generic `CritRate` which over-applies to all hits.

**Verdict**: Should be `SkillCritRate` or handled as a conditional if that stat variant exists, otherwise acceptable as approximation with user understanding.

### Shenhe — C6 "Mystical Abandon" (MISSING)

**Mirror** (`shenhe_063.md`):
> "When characters trigger Icy Quill's effects using Normal and Charged Attack DMG, it does not count toward the Trigger Quota."

This is a utility/quota mechanic, not a direct stat buff. Correctly excluded.

### Shenhe — C4 "Insight" (MISSING)

**Mirror** (`shenhe_063.md`):
> "Each stack will increase the DMG of that Spring Spirit Summoning by 5% for each stack consumed. Max 50 stacks."

This is a skill DMG scaling mechanic (Shenhe's own skill gets +5% DMG per Skyfrost Mantra stack consumed, up to 50 stacks = +250%). Since it affects Shenhe's own skill DMG output, it is damage-relevant.

**Verdict**: Missing implementation for Shenhe C4. Self Skill DMG bonus +0.05 per stack (max 50). Source: Constellation(4), `SkillDmgBonus` or `DmgBonus` self.

### Skirk — C4 "Fractured Flow" (VALUE INCONSISTENCY)

**Implementation** (`cryo.rs` lines 952-965):
```
name: "Fractured Boundary ATK"
base_value: 0.40
activation: Toggle (adopting max)
```
Description says: "ATK+10/20/40% based on stacks (adopting max +40%, self)"

**Mirror** (`skirknew_114.md`):
> C4 "Fractured Flow": "Each Death's Crossing stack also increases Skirk's ATK by 10%/20%/40%."

This is a 3-level step function (1/2/3 stacks give 10/20/40%). The implementation simplifies to adopting max (+40%), which is reasonable, but the description calls it "Stacks(3) simplified to Toggle" while using a Toggle activation.

**Verdict**: Acceptable simplification (max value adopted, consistent with other max-value simplifications in the codebase). Minor: code comment says "Stacks(3) simplified to Toggle max" but activation is `Toggle`, not `Stacks(3)`.

### Layla — C6 "Radiant Soulfire" (PARTIAL)

**Implementation** (`cryo.rs` lines 918-931):
```
stat: BuffableStat::SkillDmgBonus
base_value: 0.40
```

**Mirror** (`layla_074.md`):
> "Shooting Stars from Nights of Formal Focus deal 40% increased DMG, **and Starlight Slugs from Dream of the Star-Stream Shaker deal 40% increased DMG**."

The implementation only adds Shooting Star DMG+40% (`SkillDmgBonus`). The Starlight Slug (Burst) DMG+40% from the same C6 is missing.

**Verdict**: Missing `BurstDmgBonus` +0.40 from C6, `source: Constellation(6)`, `target: OnlySelf`.

### Kaeya — C1 Scope (OVERGENERALIZED)

**Implementation** (`cryo.rs` lines 839-852):
```
stat: BuffableStat::CritRate
base_value: 0.15
target: BuffTarget::Team
```

**Mirror** (`kaeya_015.md`):
> "The CRIT Rate of Kaeya's **Normal and Charge Attacks** against opponents affected by Cryo is increased by 15%."

This is specifically Kaeya's own Normal/Charged Attack CRIT Rate, not a general team CRIT Rate buff.

**Verdict**: Wrong target (`Team` should be `OnlySelf`) and wrong stat (should be `NormalAtkCritRate` + `ChargedAtkCritRate` if those variants exist, else a Toggle on `CritRate`). Currently applies a general 15% CRIT Rate to the entire team, which is a significant over-application.

### Mika — A1 "Suppressive Barrage" Physical DMG Stacks (MISSING)

**Mirror** (`mika_080.md`):
> "Per the following circumstances, the Soulwind state will grant characters the Detector effect, increasing their Physical DMG by 10% when they are on the field. [...] The Soulwind state can have a maximum of 3 Detector stacks."

So Mika's A1 grants up to 3 × Physical DMG stacks = effectively up to 30% Physical DMG (with A4 "Topographical Mapping" adding 1 more stack for 4 total = 40%). This is not implemented in `MIKA_BUFFS`.

**Verdict**: Missing. Should add `PhysicalDmgBonus` +0.10 per stack (max 3 or 4 with A4), `source: AscensionPassive(1)`, `target: Team`.

---

## Verified OK

### Diona
- C6 "Cat's Tail Closing Time": EM+200 — **CORRECT** (mirror confirms EM+200 when HP>50%)

### Ganyu
- A4 "Harmony Between Heaven and Earth": Cryo DMG+20% in burst field — **CORRECT**
- C1 "Dew-Drinker": Cryo RES-15% — **CORRECT**
- C4 "Westward Sojourn": DMG+25% max (step-up, adopting max) — **CORRECT** (mirror: starts at 5%, +5% every 3s, max 25%)

### Qiqi
- C2 "Frozen to the Bone": NA+CA DMG+15% vs Cryo-affected — **CORRECT** (mirror confirms)

### Rosaria
- A4 "Shadow Samaritan": grants 15% of Rosaria's CRIT Rate to party (max 15%) — **CORRECT**
- C6 "Divine Retribution": Physical RES-20% for 10s — **CORRECT**
- C1 value (10% Normal ATK DMG) — **CORRECT** (ATK SPD is intentionally excluded)

### Shenhe
- Skill Icy Quill flat DMG scaling array (`SHENHE_SKILL_SCALING`): Lv1=0.4566, Lv7=0.6849, Lv10=0.8219, Lv15=1.0844 — **CORRECT** (mirror confirms: 45.66% ATK, 68.48% ATK, 82.18% ATK, 108.43% ATK, accounting for decimal conversion)
- A4 "Spirit Communion Seal" Press: Skill/Burst DMG+15% — **CORRECT**
- A4 "Spirit Communion Seal" Hold: NA/CA/Plunge DMG+15% — **CORRECT**
- C2 "Centered Spirit": Cryo CRIT DMG+15% in Burst field — **CORRECT**

### Aloy
- A1 "Combat Override": self ATK+16%, team ATK+8% — **CORRECT** (mirror confirms: not "Easy Does It" as labeled in code comment, but "Combat Override")
  - Note: The code comment says "Easy Does It" but the mirror shows the passive is named "Combat Override". The buff effect is correct.
- A4 "Strong Strike": Cryo DMG+35% max — **CORRECT**

### Ayaka
- A1 "Amatsumi Kunitsumi Sanctification": NA+CA DMG+30% — **CORRECT**
- A4 "Kanten Senmyou Blessing": Cryo DMG+18% — **CORRECT**
- C4 "Ebb and Flow": DEF-30% — **CORRECT**
- C6 "Dance of Suigetsu": CA DMG+298% — **CORRECT**

### Eula
- C1 "Tidal Illusion": Physical DMG+30% on Grimheart consume — **CORRECT**
- C4 "The Obstinacy of One's Inferiors": Lightfall Sword DMG+25% vs <50% HP — **CORRECT** (note: buff is for Lightfall Sword damage specifically, implementation uses `BurstDmgBonus` which is close enough)

### Freminet
- A4 "Parallel Condensers": Shatter DMG+40% — **CORRECT**
- C4 "Dance of the Snowy Moon": ATK+9%/stack (max 2) — **CORRECT**
- C6 "Moment of Waking": CD+12%/stack (max 3) — **CORRECT**

### Wriothesley
- A4 "There Shall Be a Reckoning for Sin": ATK+6%/stack (max 5) — **CORRECT**
- C6 "Esteem for the Innocent": CR+10%, CD+80% — **CORRECT**

---

## Priority Summary

### BUGS (must fix)

| # | Character | Issue |
|---|-----------|-------|
| 1 | Escoffier | Duplicate C1 entry — always-on entry is wrong, remove it |
| 2 | Kaeya | C1 target is `Team` but should be `OnlySelf`; scope is NA+CA CRIT Rate only, not general team CRIT Rate |
| 3 | Chongyun | C6 uses `DmgBonus` (all damage) but should be `BurstDmgBonus` with HP condition |

### MISSING BUFFS (high priority — damage-relevant)

| # | Character | Missing Buff |
|---|-----------|-------------|
| 4 | Escoffier | A4 "Inspiration-Immersed Seasoning": Hydro RES-55% + Cryo RES-55% (4 Hydro/Cryo party) |
| 5 | Charlotte | A4 "Diversified Investigation": Cryo DMG+15% (3 non-Fontainian party members) |
| 6 | Citlali | C6 self DMG+2.5%/count (max 40) missing alongside team Pyro/Hydro DMG buff |
| 7 | Layla | C6 Burst DMG+40% (Starlight Slugs) missing alongside Skill DMG+40% |
| 8 | Mika | A1 Detector stacks: Physical DMG+10%/stack (max 3–4 stacks) completely missing |
| 9 | Shenhe | C4 "Insight": Shenhe's own Skill DMG+5%/Skyfrost Mantra stack consumed (max 50 stacks) |

### APPROXIMATIONS / ACCEPTABLE SIMPLIFICATIONS

| # | Character | Note |
|---|-----------|------|
| 10 | Eula | Skill RES shred uses Lv10 max (25%) — correct for max talent, minor doc issue |
| 11 | Mika C6 | `CritDmg` used for Physical-specific CRIT DMG — may over-apply to non-physical |
| 12 | Freminet C1 | `CritRate` used for Shattering Pressure CRIT Rate — over-applies to all hits |
| 13 | Skirk C4 | Toggle (max 40%) instead of 3-step stacks — acceptable max-value simplification |
| 14 | Aloy | Code comment says "Easy Does It" but actual passive name is "Combat Override" |
