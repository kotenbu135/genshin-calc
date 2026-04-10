# Geo Buff Audit: `crates/data/src/talent_buffs/geo.rs`

Audit date: 2026-04-08
Mirror source: `honeyhunter-mirror/md/characters/`

---

## Summary

| Character | Status | Notes |
|-----------|--------|-------|
| Albedo | Partial — missing A4 Hexerei buff | New passive added |
| Chiori | OK | C6 value verified |
| Gorou | Minor rounding discrepancy | Lv9 DEF value |
| Illuga | OK (with known TODOs) | A4 and burst mechanic noted as TODO |
| Itto | OK | All verified |
| Kachina | OK | |
| Navia | OK | |
| Ningguang | Missing C4 RES buff | Not implemented |
| Noelle | Missing burst ATK-scaling buff | Burst ATK bonus not present |
| Xilonen | OK | C2 and C4 verified |
| Yun Jin | OK | |
| Zhongli | OK | All RES shreds verified |
| Zibai | OK | C2 Lunar-Crystallize verified |

---

## Value Discrepancies

### Gorou — GOROU_SKILL_DEF_SCALING Lv9

- **Mirror**: `350.47`
- **Code**: `350.48`
- **Assessment**: Off by 0.01, likely rounding difference. Negligible for gameplay; acceptable.

Also note Lv10–Lv12 in the mirror show `.09`, `.7`, `.32` (e.g., `371.09`, `391.7`, `412.32`) versus code `371.10`, `391.72`, `412.34`. These are rounding artifacts from the mirror's display precision. All values are within ±0.02.

### Gorou — C6 "Valiant Hound: Mountainous Fealty"

- **Mirror**: Three tiers — "Standing Firm" +10%, "Impregnable" +20%, "Crunch" +40% CRIT DMG
- **Code**: Single buff entry `gorou_c6_geo_crit_dmg` with value `0.40` (Crunch tier only)
- **Assessment**: The code implements the max Crunch value only with a Toggle. This is a deliberate simplification (best-case approximation). The "Standing Firm" (+10%) and "Impregnable" (+20%) tiers are not modeled. **Acceptable as documented simplification, but could be improved with Stacks(3) at 10%/20%/40%.**

---

## Missing Implementations

### Albedo — A4 "Witch's Eve Rite: Book of Blinding Light" (Hexerei Passive)

**Mirror description**: After creating a Solar Isotoma, nearby party members' Normal Attack, Charged Attack, Plunging Attack, Elemental Skill, and Elemental Burst DMG are increased by 4% for every 1,000 DEF Albedo has (max +12%). This is a separate new passive that was added alongside the existing "Homuncular Nature" A4.

- **Code**: Only "Homuncular Nature" (EM +125) is implemented for Albedo's A4.
- **Status**: MISSING. The Hexerei passive buff (team DMG +4% per 1000 DEF, cap 12%, scales on Albedo DEF) is not implemented. This is a significant damage buff for Albedo teams.
- **Source**: AscensionPassive (this is listed as a 4th passive on the mirror — likely a special unlockable passive)
- **Note**: The Hexerei Silver Isotoma variant (+10% per 1000 DEF for Hexerei members, cap 30%) is also not implemented.

**Implementation status:** Fixed in this branch.

### Ningguang — C4 "Exquisite be the Jade, Outshining All Beneath"

**Mirror description**: Jade Screen increases nearby characters' Elemental RES by 10%.

- **Code**: NINGGUANG_BUFFS only contains the A4 "Strategic Reserve" Geo DMG +12% buff. C4 is not present.
- **Status**: MISSING. This is a flat +10% all-element RES buff for the active character near the screen. Should be added as a C4 buff (stat: some AllElementalResBonus or per-element, target: Team, toggle).

**Implementation status:** Fixed in this branch.

### Noelle — Elemental Burst "Sweeping Time" ATK Scaling Buff

**Mirror description**: During Sweeping Time, Noelle's ATK increases based on her DEF. Lv1: 40% DEF, ..., Lv10: 72% DEF, Lv13: 85% DEF, Lv15: 95% DEF.

- **Code**: NOELLE_BUFFS only has C2 (CA DMG +15%) and C6 (ATK flat = 50% DEF). The burst's core ATK scaling buff (DEF-based ATK bonus) is absent.
- **Status**: MISSING. Noelle's most important buff — during burst, ATK +X% of DEF (scaling with talent level) — is not implemented. This is a talent-scaling self-buff that requires a scaling table similar to Gorou's DEF table. It is her primary damage mechanism during burst. **High priority to implement.**

**Implementation status:** Fixed in this branch.

### Gorou — A4 "Heedless of the Wind and Weather"

**Mirror description**: After using Juuga: Forward Unto Victory, all nearby party members' DEF is increased by 25% for 12s.

- **Code**: This buff is NOT in GOROU_BUFFS.
- **Status**: MISSING. This is a post-burst team DEF +25% passive. Should be added as AscensionPassive(4), BuffTarget::Team, Toggle or auto.

**Implementation status:** Fixed in this branch.

---

## Verified OK

### Albedo
- A4 "Homuncular Nature": EM +125 for team after burst — CORRECT (value, source, target all match)
- C1 "Flower of Eden": DEF +50% on Skill use — CORRECT
- C4 "Descent of Divinity": Plunging ATK DMG +30% — CORRECT (mirror confirms exactly 30%)
- C6 "Dust of Purification": DMG Bonus +17% with Crystallize shield — CORRECT

### Chiori
- C6 "Sole Principle Pursuit": Normal Attack DMG +235% of Chiori's DEF — CORRECT (mirror: "235% of her own DEF")
- A4 "The Finishing Touch": Geo DMG +20% when Geo Construct created — NOT in CHIORI_BUFFS (omitted)
  - **Assessment**: Omission is acceptable; it's a self-buff conditional on nearby construct creation. However, it is a damage-relevant buff. Consider adding as self-buff with Toggle.

### Gorou
- Skill DEF scaling table values match mirror within rounding tolerance
- Geo DMG +15% (3 Geo): CORRECT
- C6 Crunch value +40%: CORRECT (max tier)

### Illuga
- A1 "Torchforger's Covenant": CRIT Rate +5%, CRIT DMG +10%, EM +50 — CORRECT (mirror: exactly these values)
- C4 "Solarhunting Wolf": DEF +200 flat — CORRECT (mirror: "DEF increased by 200")
- C6 "Nightmare Orioles": Incremental values of +5% CRIT Rate, +20% CRIT DMG, +30 EM — CORRECT (total becomes +10%/+30%/+80 which matches mirror)
- A4 "Demonhunter's Dusk" TODO and Burst mechanic TODO: acknowledged in code comments; complex mechanics legitimately deferred

### Itto
- A4 "Bloodline of the Crimson Oni": Charged ATK (Kesagiri) +35% DEF — CORRECT
- C4 "Jailhouse Bread and Butter": Team DEF +20%, ATK +20% after burst ends — CORRECT
- C6 "Arataki Itto, Present!": CA CRIT DMG +70% — CORRECT (mirror: "Charged Attacks deal +70% CRIT DMG")

### Kachina
- A4 "Mountain Echoes": Geo DMG Bonus +20% — CORRECT (mirror: "Geo DMG Bonus increases by 20%")
- C4 "More Foes, More Caution": DEF +8/12/16/20% based on enemy count (max 20%) — CORRECT (code uses max value 20% with Toggle)

### Navia
- A1 "Undisclosed Distribution Channels": NA/CA/Plunge DMG +40% after skill — CORRECT
- A4 "Mutual Assistance Network": ATK +20% per off-element party member (max 2 stacks = +40%) — CORRECT (code uses 0.40 as max)
- C2 "The President's Pursuit of Victory": CRIT Rate +12% per Shrapnel stack (max 3 = +36%) — CORRECT (Stacks(3) at 0.12)
- C4 "The Oathsworn Never Capitulate": Enemy Geo RES -20% — CORRECT
- C6 "The Flexible Finesse": CRIT DMG +45% per extra stack — NOT in NAVIA_BUFFS
  - **Assessment**: C6 CRIT DMG buff (+45% per extra Crystal Shrapnel beyond 3) is missing. This is damage-relevant but stack-conditional per skill use. Consider adding.

### Ningguang
- A4 "Strategic Reserve": Geo DMG +12% after passing through Jade Screen — CORRECT

### Noelle
- C2 "Combat Maid": CA DMG +15% — CORRECT
- C6 "Must Be Spotless": ATK +50% DEF flat — CORRECT

### Xilonen
- Skill RES reduction scaling table (9% to 51% at Lv1-15) — CORRECT (mirror: "9% | 12% | 15% | ... | 51%")
- C2 "Chiucue Mix": Geo DMG +50%, Pyro ATK +45%, Hydro HP +45%, Cryo CRIT DMG +60% — CORRECT
  - Note: Electro effect (energy + CD) correctly omitted per convention
- C4 "Suchitl's Trance": Normal/Charged/Plunging flat DMG +65% DEF — CORRECT (mirror: "65% of Xilonen's DEF")

### Yun Jin
- Burst "Cliffbreaker's Banner": Normal ATK flat DMG scaling on DEF — table values match mirror (Lv1: 32.16% DEF)
- C2 "Myriad Mise-En-Scène": Normal ATK DMG +15% — CORRECT
- C4 "Flower and a Fighter": DEF +20% on Crystallize — CORRECT

### Zhongli
- All 8 RES shreds (Pyro, Hydro, Electro, Cryo, Dendro, Anemo, Geo, Physical) at -20% — CORRECT (mirror: "decrease the Elemental RES and Physical RES of opponents by 20%")

### Zibai
- C2 "At Birth Are Souls Born": Lunar-Crystallize Reaction DMG +30% for party — CORRECT (mirror: "all nearby party members' Lunar-Crystallize Reaction DMG is increased by 30%")
- Note: Stat type `TransformativeBonus` is used; verify this maps correctly to Lunar-Crystallize reaction DMG in the calc engine.

---

## Action Items (Priority Order)

1. **HIGH — Noelle burst ATK buff**: Add talent-scaling ATK bonus (40%-95% DEF, Lv1-15) as self-buff during burst. This is Noelle's core damage mechanic and its absence will cause significantly wrong DPS calculations.

2. **HIGH — Gorou A4**: Add team DEF +25% (post-burst, toggle). Missing entirely from GOROU_BUFFS.

3. **MEDIUM — Albedo Hexerei passive**: Add team DMG +4% per 1000 DEF (cap 12%, scales_on DEF). New mechanic but expressible with TalentBuffDef if a cap+scaling system is available.

4. **MEDIUM — Ningguang C4**: Add all-element RES +10% for team near Jade Screen.

5. **LOW — Navia C6**: Add self CRIT DMG +45% per extra Crystal Shrapnel (stack-conditional per skill use).

6. **LOW — Chiori A4**: Add self Geo DMG +20% when nearby Geo Construct created.

7. **INFO — Gorou C6 tier modeling**: Currently only Crunch (+40%) is modeled. Consider Stacks(3) at 10% base to represent all three tiers accurately.
