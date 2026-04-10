# Pyro Talent Buffs Audit

Sources checked:
- Implementation: `crates/data/src/talent_buffs/pyro.rs`
- Mirror: `honeyhunter-mirror/md/characters/*.md`

## Summary

- Characters with buff implementations: 17 (amber, arlecchino, bennett, chevreuse, dehya, diluc, durin, gaming, hu_tao, klee, lyney, mavuika, thoma, xiangling, xinyan, yanfei, yoimiya)
- Buff value discrepancies: 3
- Wrong stat type: 1
- Wrong source label / name: 5
- Duplicate buff entries (double-count risk): 1
- Missing buff implementations: 12

---

## Value / Type Discrepancies

### Chevreuse — A1 "Vanguard's Coordinated Tactics"

**Problem**: The ATK+20% buff labeled as `Vanguard's Coordinated Tactics` (AscensionPassive(1)) does not exist.

- **Mirror A1** "Vanguard's Coordinated Tactics": Grants party "Coordinated Tactics" → after Overloaded, enemy Pyro/Electro RES -40% for 6s. **No ATK buff at A1.**
- **Mirror A4** "Vertical Force Coordination": After firing Overcharged Ball, Pyro/Electro party members' ATK +1% per 1,000 of Chevreuse's Max HP, cap +40%.
  - This is an HP-scaling ATK buff (complex), currently modeled as a flat +20% Toggle. The value is wrong (max is +40%) and the source label is wrong (should be AscensionPassive(4)).

**Rust**: `base_value: 0.20, source: AscensionPassive(1)` → **should be AscensionPassive(4), and max value is 40% not 20%**

The RES shred buffs `Overloaded Pyro RES Shred` and `Overloaded Electro RES Shred` at AscensionPassive(4) are also mislabeled — they belong to A1, not A4.

**Implementation status:** Fixed in this branch.

### Yanfei — C2 "Right of Final Interpretation"

**Problem**: Wrong stat type.

- **Mirror C2**: "Increases Yanfei's Charged Attack CRIT Rate by 20% against enemies below 50% HP."
- **Rust**: `stat: BuffableStat::ChargedAtkDmgBonus, base_value: 0.20` — this uses `ChargedAtkDmgBonus` instead of `CritRate`.

The buff is **CRIT Rate +20%** (conditional), not Charged ATK DMG Bonus +20%.

**Implementation status:** Fixed in this branch.

### Yoimiya — A4 "Summer Night's Dawn"

**Partial discrepancy**: Mirror says base ATK +10% plus 1% per stack of "Tricks of the Trouble-Maker" (max 10 stacks = +10% more), for a total max of +20%.

- **Rust**: `base_value: 0.20` (flat +20%) — this models the maximum correctly but does not track the stack-dependent component separately.
- The minimum value with 0 stacks is actually +10%, not +20%. For static max-value modelling this may be acceptable, but the base value is technically wrong if stacks are 0.

**Implementation status:** Fixed in this branch.

---

## Wrong Source Label / Name (values correct)

### Arlecchino

- Rust `Crimson Flower Pyro DMG Bonus` at `AscensionPassive(1)` — correct value (+40%), but the actual passive name is **"The Balemoon Alone May Know"**, not "Crimson Flower". Comment in code says "Crimson Flower Flutters Freely" which does not match the mirror.
- The TODO comment for A4 also has the names swapped: "The Balemoon Alone May Know" in the comment refers to the A1 passive; the actual A4 is "Agony Alone May Be Repaid" (Bond of Life mechanics, correctly not implemented).

**Implementation status:** Fixed in this branch.

### Mavuika

- Rust `Sunfrost Encomium ATK Bonus` at `AscensionPassive(1)` — mirror's A1 passive name is **"Gift of Flaming Flowers"** (ATK +30% after party Nightsoul Burst). Value is correct.
- Rust `Fire-Forged Heritage DMG Bonus` at `AscensionPassive(4)` — mirror's A4 passive name is **"Kiongozi"** (DMG bonus based on Fighting Spirit consumed, max +40%). Value is correct.

**Implementation status:** Fixed in this branch.

### Diluc

- Rust `Flowing Flame Pyro DMG Bonus` at `AscensionPassive(4)` — mirror's A4 passive name is **"Blessing of Phoenix"**, not "Flowing Flame". Value (+20% Pyro DMG) is correct.
- Mirror C4 is named "Flowing Flame / 流火焼灼" (≈ +40% to the next Searing Onslaught if cast in rhythm). Rust labels it "Flowing Ember Skill DMG Bonus" at C4 — name does not match but the modeled effect (+40% Skill DMG) is a reasonable approximation.

**Implementation status:** Fixed in this branch.

---

## Duplicate Entry Bug (Durin C4)

### Durin — C4 "Emanare's Source"

There are **two separate `TalentBuffDef` entries** for the same C4 Burst DMG +40% effect:

1. Lines ~309-322: `name: "Emanare's Source Burst DMG Bonus"`, `activation: None`
2. Lines ~324-337: `name: "durin_c4_burst_dmg"`, `activation: Some(Toggle)`

Both have `stat: BurstDmgBonus, base_value: 0.40, source: Constellation(4), min_constellation: 4`.

If both are applied simultaneously, Durin's Burst DMG will be double-buffed (+80% total instead of +40%).

**Implementation status:** Fixed in this branch.

---

## Missing Buff Implementations

### Bennett

No missing buff-type effects. C2 ER+30% (utility), C4 proc damage — correctly excluded. **OK.**

### Amber

- **A4 "Precise Shot"**: Aimed Shot hits on weak points increase ATK by 15% for 10s. This is a damage-relevant self-buff.
  - Source: AscensionPassive(4) | Stat: AtkPercent +15% | Target: OnlySelf | Toggle
  - **NOT IMPLEMENTED**

**Implementation status:** Fixed in this branch.

### Chevreuse

- **A4 ATK value wrong** (see Value Discrepancies above; should be max +40%, source AscensionPassive(4)).
- No additional missing buffs beyond what's noted.

**Implementation status:** Fixed in this branch.

### Xiangling

No missing buff-type effects. C2 proc damage, C4 duration extension — correctly excluded. **OK.**

### Yoimiya

- **A1 "Tricks of the Trouble-Maker"**: During Niwabi Fire-Dance, Normal Attack hits increase Pyro DMG Bonus by 2% per hit, max 10 stacks (+20%). This is a self-stacking Pyro DMG buff.
  - Source: AscensionPassive(1) | Stat: ElementalDmgBonus(Pyro) | max +20% | Target: OnlySelf | Stacks(10)
  - **NOT IMPLEMENTED**

**Implementation status:** Fixed in this branch.

### Diluc

No additional missing buffs. C3/C5 skill levels, C4 proc DMG increase — C4 as approximation is OK. **OK.**

### Hu Tao

- **C6 "Butterfly's Embrace"**: When HP drops below 25% or on lethal hit, Hu Tao's CRIT Rate +100% for 10s.
  - Source: Constellation(6) | Stat: CritRate +1.00 | Target: OnlySelf | Toggle
  - **NOT IMPLEMENTED** (significant CRIT Rate buff)

**Implementation status:** Fixed in this branch.

### Klee

- **C1 "Chained Reactions" (Buffed State)**: After sparks bombard opponents, Klee's ATK +60% for 12s.
  - Source: Constellation(1) | Stat: AtkPercent +0.60 | Target: OnlySelf | Toggle
  - **NOT IMPLEMENTED** (base game C1 is proc damage only, but Buffed State adds an ATK buff)
- **C6 "Blazing Delight" (Buffed State)**: Klee herself gains Pyro DMG +50% (self); other party members get +10%. The implementation only has +10% for Team (party), missing Klee's personal +50% in Buffed State.
  - **PARTIALLY IMPLEMENTED** (team +10% OK, Klee's +50% self-buff missing)

**Implementation status:** Fixed in this branch.

### Yanfei

- Yanfei's stat type error in C2 noted above (see Value Discrepancies).
- No additional missing buffs.

### Xinyan

- **A4 "...Now That's Rock 'N' Roll!"**: When shielded by Sweeping Fervor (Level 3), party members deal 15% increased Physical DMG.
  - Source: AscensionPassive(4) | Stat: PhysicalDmgBonus +0.15 | Target: Team | Toggle
  - **NOT IMPLEMENTED**

**Implementation status:** Fixed in this branch.

### Thoma

No missing buff-type effects. A1 Shield Strength (utility), A4 proc damage scaling, C1/C2/C4 utility — correctly excluded. **OK.**

### Dehya

No missing buff-type effects. A1/A4 are defensive/HP-recovery. C4 energy/HP restore, C1 HP% is implemented. **OK.**

### Lyney

No missing buff-type effects. C1 and C6 are proc damage. **OK.**

### Gaming

No missing buff-type effects. C1/C4 are healing/energy. **OK.**

### Arlecchino

- **A4 "Agony Alone May Be Repaid"** is correctly marked TODO (Bond of Life mechanics, complex).
- **C2**: All Elemental RES/Physical RES +20% for self — this is a defensive buff, not a DMG buff. Skip is acceptable.
- No damage-relevant missing buffs.

### Mavuika

- **C1 "The Night-Lord's Explication"**: After gaining Fighting Spirit, ATK +40% for 8s.
  - Source: Constellation(1) | Stat: AtkPercent +0.40 | Target: OnlySelf | Toggle
  - **NOT IMPLEMENTED**
- **C2 "The Ashen Price" — Flamestrider flat ATK bonus**: Base ATK +200 (flat) in Nightsoul's Blessing state.
  - Source: Constellation(2) | Stat: AtkFlat +200 | Target: OnlySelf | Toggle
  - **NOT IMPLEMENTED** (only the DEF Reduction for Ring form is implemented)
- **C4 "The Leader's Resolve"**: Enhances "Kiongozi" — DMG decay stops, adds 10% additional DMG Bonus.
  - Source: Constellation(4) | Stat: DmgBonus +0.10 additional | Target: OnlySelf | Toggle
  - **NOT IMPLEMENTED** (the base A4 max +40% is already implemented, but C4 enhancement is not)

**Implementation status:** Fixed in this branch.

### Durin

- **C1 "Adamah's Redemption" — Purity form**: Party members gain Cycle of Enlightenment stacks → flat ATK-based DMG increase (60% of Durin's ATK per stack consumed). This is a flat DMG add scaling on Durin's ATK, which is complex. Marking as not implemented / intentional skip.

**Implementation status:** Out of scope for this pass because this is an added-damage/proc-damage or non-damage effect.

---

## Verified OK

- **Bennett**: 3 buffs implemented (Burst ATK scaling Lv1-15 matches mirror exactly, C1 +20% Base ATK, C6 Pyro DMG +15%). All correct.
- **Amber**: C6 ATK +15% for party — correct. (Note: A4 Precise Shot ATK+15% is missing, see above.)
- **Chevreuse**: RES shred values (+40% Pyro/Electro) correct. C6 stacks correct. Source mislabeling noted above.
- **Xiangling**: C1 Pyro RES -15% correct. C6 Pyro DMG +15% correct. Both have duplicate with/without Toggle entries — consistent pattern, acceptable.
- **Yoimiya**: C1 ATK +20% correct. C2 Pyro DMG +25% correct.
- **Diluc**: A4 Pyro DMG +20% correct. C1 DMG+15% correct. C2 ATK+10% per stack (max 3) correct. C4 Skill DMG +40% correct. C6 NA DMG +30% correct.
- **Hu Tao**: A1 CR+12% (TeamExcludeSelf) correct. A4 Pyro DMG +33% correct. C4 CR+12% (TeamExcludeSelf) correct.
- **Klee**: C2 DEF -23% correct (has duplicate with/without Toggle). C6 Pyro DMG +10% (Team) correct for base game.
- **Thoma**: C6 Normal/Charged/Plunging DMG +15% for party correct.
- **Dehya**: C1 HP+20% correct. C2 Skill DMG +50% correct. C6 CR+10%, CD+60% (max values) correct.
- **Lyney**: A4 DMG Bonus max +100% (Toggle) correct. C2 CD +20% per stack (max 3) correct. C4 Pyro RES -20% correct.
- **Gaming**: A4 Plunge DMG +20% when HP≥50% correct. C2 ATK +20% correct. C6 CR+20%, CD+40% correct.
- **Xinyan**: C2 CRIT Rate +100% for Burst Physical DMG correct. C4 Physical RES -15% correct. C6 Charged ATK flat DMG = 50% DEF correct.
- **Yanfei**: A1 Pyro DMG +20% max (Stacks modelled as Toggle at max) — acceptable approximation. C2 wrong stat type (see above).
- **Arlecchino**: A1 Pyro DMG +40% correct value. C6 CR+10%, CD+70% correct.
- **Mavuika**: C2 DEF Reduction -20% correct. A1 ATK+30% correct value. A4 DMG Bonus +40% correct value.
- **Durin**: A1 Purity Pyro RES -20% correct. A1 Darkness Amplifying Bonus +40% correct. C2 Pyro DMG +50% correct. C4 Burst DMG +40% correct (but duplicate entry is a bug). C6 DEF Ignore 30% and DEF Reduction 30% correct.
