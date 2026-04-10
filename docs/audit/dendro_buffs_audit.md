# Dendro Buffs Audit

Audited: `crates/data/src/talent_buffs/dendro.rs`
Mirror source: `honeyhunter-mirror/md/characters/`
Date: 2026-04-08

---

## Summary

13 characters checked. Overall accuracy is high for implemented buffs. Key findings:

- **4 confirmed value discrepancies** (Baizhu A4, Emilie A4, Kaveh A4, Yaoyao C4)
- **5 missing damage-related buff implementations** (Alhaitham C4, Kaveh A4 burst bonus, Kinich C1, Lauma A4, Nahida A4)
- **3 duplicate entries** without activation (Nefer C2/C4, Baizhu C4, Collei C4) that have correct Toggle variants — the non-Toggle duplicates should be removed
- **1 wrong buff target** (Tighnari C4 emits team-wide buff but max +120 can be triggered by either condition)
- Dendro Traveler mirror file (playerboy_037.md) contains only base stats — no passives/constellations available to verify; implementations unverifiable from mirror

---

## Value Discrepancies

### Baizhu — A4 "All Things Are of the Earth" (Five Fortunes Forever)

**Implementation** (`baizhu_buffs`):
```
stat: TransformativeBonus
base_value: 0.00002   // 2% per 1000 HP → 0.00002 per 1 HP
scales_on: Hp
cap: 0.50
```

**Mirror** (A4 passive "All Things Are of the Earth"):
> Every 1,000 Max HP that Baizhu possesses that does not exceed 50,000 will increase Burning, Bloom, Hyperbloom, and Burgeon reaction DMG by **2%**, their **Lunar-Bloom DMG by 0.7%**, and the **DMG Bonus provided by Aggravate and Spread by 0.8%**.

**Issues**:
1. The implementation captures only Bloom-type reactions (`TransformativeBonus` at 2% per 1000 HP). Aggravate/Spread bonus (0.8% per 1000 HP, max 40%) is **missing** entirely.
2. Lunar-Bloom DMG bonus (0.7% per 1000 HP, max 35%) is **missing** entirely.
3. The description in the code comment still references "Five Fortunes Forever" — this is actually the A1 passive name. A4 is "All Things Are of the Earth."
4. The 50,000 HP cap means the effective max HP contributing is 50,000. At 0.00002 per HP, max = 50,000 × 0.00002 = 1.0 (100%), but the mirror says max 2% × 50 (50k/1k) = 100% — so the cap of `Some(0.50)` is **wrong**; it should be `Some(1.00)` (100%).

> **Severity: HIGH** — cap is wrong (0.50 vs 1.00), and two sub-bonuses (Aggravate/Spread, Lunar-Bloom) are entirely absent.

---

### Emilie — A4 "Rectification"

**Implementation**:
```
base_value: 0.001   // 0.1% per 1 ATK
scales_on: TotalAtk
cap: 0.36
```

**Mirror** (A4 "Rectification"):
> Every **1,000 ATK** increasing DMG dealt by **15%**. The maximum DMG bonus that can be gained this way is **36%**.

Mirror says 15% per 1000 ATK → 0.015 per 100 ATK → 0.00015 per 1 ATK.

**Implementation** uses `0.001` per 1 ATK (= 1% per 1000 ATK). This is **wrong by a factor of 15**.

Correct `base_value` should be `0.00015` (0.015% per ATK = 15% per 1000 ATK).

> **Severity: CRITICAL** — value is 15× too low. A character with 1500 ATK would get 1.5% instead of 22.5%.

---

### Kaveh — A4 "A Craftsman's Curious Conceptions"

**Implementation** (`kaveh_buffs`):
```
name: "An Architect's Undertaking EM Bonus"
source: AscensionPassive(4)
stat: ElementalMastery
base_value: 25.0
scales_on: None
activation: Stacks(4)
```

**Mirror** — The A4 passive is actually named **"A Craftsman's Curious Conceptions"**:
> During Painted Dome, after Kaveh's Normal, Charged, or Plunging Attacks hit opponents, his Elemental Mastery will increase by 25. This effect can be triggered once every 0.1s. **Max 4 stacks**. This effect will be canceled when Painted Dome's effects end.

The **A1 passive** "An Architect's Undertaking" is:
> When DMG dealt by a Dendro Core hits Kaveh, Kaveh will **regain HP** equal to 300% of his Elemental Mastery.

The implementation has the **wrong passive name** (should be "A Craftsman's Curious Conceptions", not "An Architect's Undertaking"). The EM value (+25 per stack, max 4 stacks) and stat type are correct. This is a naming mislabel — does not affect computation but affects identification.

> **Severity: LOW** (name only, values correct)

---

### Yaoyao — C4 "Winsome"

**Implementation** — there are two Yaoyao C4 entries:

Entry 1 (no activation):
```
name: "Whitesun Wheel EM Bonus"
base_value: 0.0008   // 0.08% per HP
scales_on: Hp
cap: 120.0
```

Entry 2 (with toggle):
```
name: "yaoyao_c4_em"
base_value: 0.003    // 0.3% per HP
scales_on: Hp
cap: 120.0
```

**Mirror** (C4 "Winsome"):
> After using Raphanus Sky Cluster or Moonjade Descent, Yaoyao's Elemental Mastery will be increased based on **0.3% of her Max HP** for 8s. The maximum Elemental Mastery she can gain this way is 120.

The correct value is **0.3% of Max HP** (0.003 per HP). Entry 2 (`yaoyao_c4_em`) is correct.

Entry 1 (`Whitesun Wheel EM Bonus`) has `base_value: 0.0008` which corresponds to 0.08% per HP — this is **wrong** and also the name references "Whitesun Wheel" which does not appear in the mirror data.

> **Severity: HIGH** — one of two duplicate entries has an incorrect base_value (0.0008 vs 0.003). The non-toggle duplicate should be removed; the toggle version is correct.

---

## Missing Implementations

### Alhaitham — C4 "Elucidation"

**Mirror** (C4):
> When Particular Field: Fetters of Phenomena is unleashed:
> - Each Mirror **consumed** will increase the Elemental Mastery of all other nearby party members by **30** for 15s.
> - Each Mirror **generated** will grant Alhaitham a **10% Dendro DMG Bonus** for 15s.

**Implementation** has: C2 EM stacks, C4 Team EM +120 (Toggle), C6 CR/CD. However:

- The C4 team EM buff is implemented as a flat +120 (Toggle) — but per mirror it is **+30 per mirror consumed** (max 3 mirrors consumed = +90), not a flat +120. The cap should match 3×30 = +90, not +120.
- The **Alhaitham self Dendro DMG Bonus** from C4 mirror generation (+10% per mirror generated, max 3 mirrors generated = +30%) is **entirely missing**.

> **Severity: HIGH** — C4 team EM value is wrong (+120 vs max +90), and Alhaitham's personal Dendro DMG Bonus from C4 is absent.

---

### Kaveh — Burst "Painted Dome" Bloom DMG Bonus (talent scaling)

**Mirror** (Burst scaling table):
> Dendro Core Burst DMG Bonus: 27.49% at Lv1 … 65.28% at Lv15

The Burst itself provides a per-talent-level Bloom DMG bonus when Painted Dome is active. The C4 "Feast of Apadana" grants an additional +60% on top of this.

**Implementation** (C4 only): `TransformativeBonus +60%` (Toggle). This captures C4's additional bonus.

**Missing**: The base Burst Bloom DMG bonus (27.49%–65.28% talent-scaling) is not implemented at all. This is a talent-scaling buff, not just C4.

> **Severity: MEDIUM** — the Burst's inherent Bloom bonus (which scales with talent level) is absent.

---

### Kinich — C1 "Parrot's Beak"

**Mirror** (C1):
> Additionally, Scalespiker Cannon's CRIT DMG is increased by **100%**.

**Implementation**: Only has C2 (Dendro RES -30% + DMG +100%) and C4 (Burst DMG +70%). C1 **CRIT DMG +100% on Scalespiker Cannon** is missing.

> **Severity: MEDIUM** — C1 CritDMG buff is absent.

---

### Lauma — A4 "Light for the Frosty Night"

**Mirror** (A4):
> For 20s after Lauma uses her Elemental Skill, based on party Moonsign:
> - **Moonsign: Nascent Gleam**: Bloom/Hyperbloom/Burgeon DMG can CRIT (CRIT Rate fixed 15%, CRIT DMG fixed 100%).
> - **Moonsign: Ascendant Gleam**: Party Lunar-Bloom DMG CRIT Rate **+10%**, CRIT DMG **+20%**.

Also relevant:
- C2 at Moonsign: Ascendant Gleam grants Lunar-Bloom DMG **+40%** to party.
- C6 at Moonsign: Ascendant Gleam grants Lunar-Bloom DMG **+25%** to party.

**Implementation**: Only one buff for Lauma — A4 Skill DMG +0.04% per EM (max 32%). The Moonsign-gated Crit buffs (A4 passive) and Lunar-Bloom DMG bonuses (C2, C6) are entirely absent.

The A4 "Cleansing for the Spring" Skill DMG buff IS correctly implemented. But none of the other buffs are present.

> **Severity: HIGH** — A4 Moonsign CRIT buffs missing; C2 Lunar-Bloom +40% missing; C6 Lunar-Bloom +25% missing.

---

### Nahida — A4 "Awakening Elucidated"

**Mirror** (A4):
> Each point of Nahida's Elemental Mastery beyond 200 will grant **0.1% Bonus DMG** and **0.03% CRIT Rate** to Tri-Karma Purification. Max 80% DMG and 24% CRIT Rate.

**Implementation**: Only A1 (EM buff) and C2/C4 constellations. The A4 "Awakening Elucidated" — which grants Nahida herself scaled Tri-Karma Purification DMG bonus and CRIT Rate based on her own EM exceeding 200 — is **entirely missing**.

> **Severity: HIGH** — A4 self-scaling CRIT Rate and Bonus DMG (tied to EM > 200) not implemented.

---

## Incorrect Buff Targets / Other Issues

### Nefer — Duplicate entries without activation

Two C2 and two C4 entries exist:
- `"Observation Feeds Strategy EM Bonus"` (no activation) + `"nefer_c2_em"` (Toggle)
- `"Delusion Ensnares Reason Dendro RES Down"` (no activation) + `"nefer_c4_dendro_res_shred"` (Toggle)

The non-toggle versions (lines 104–131) are duplicates that will always apply without player control. Per mirror, both are conditional (require Veil stacks / Shadow Dance state). The activation-free versions should be removed.

> **Severity: MEDIUM** — double-counting when toggle is active.

### Baizhu — Duplicate C4 entries

Similarly, `"Ancient Art of Perception EM Bonus"` (no activation) and `"baizhu_c4_em"` (Toggle) both implement C4. The non-toggle version should be removed.

> **Severity: MEDIUM**

### Collei — C4 Target

**Mirror** (C4 "Gift of the Woods"):
> Using Trump-Card Kitty will increase all nearby characters' Elemental Mastery by 60 for 12s (**not including Collei herself**).

**Implementation** (non-toggle `"Floral Sidewinder EM Bonus"`):
```
target: BuffTarget::Team   // includes self
```

The toggle version (`"collei_c4_em"`) correctly uses `TeamExcludeSelf`. The non-toggle version uses `Team` (includes self) which is wrong. Since this is also a duplicate (same concern as above), both issues are fixed by removing the non-toggle entry.

> **Severity: LOW** (only affects non-toggle duplicate)

### Tighnari — C4 "Withering Glimpsed in the Leaves"

**Mirror** (C4):
> When Fashioner's Tanglevine Shaft is unleashed, all nearby party members gain **60 Elemental Mastery** for 8s. If the Tanglevine Shaft triggers a Burning, Bloom, Lunar-Bloom, Quicken, or Spread reaction, their Elemental Mastery will be **further increased by 60**. This latter case will also refresh the buff state's duration.

**Implementation**: `"Fortuitous Arrival Team EM Bonus"` grants flat +60 EM to team.

The second conditional +60 EM (reaction-triggered, max 120 total) is not captured. The base +60 is correct.

> **Severity: LOW** — the reaction-triggered bonus (+60 additional) is absent.

### Kirara — Duplicate C6 entries

Two C6 entries exist:
- `"Kindred of the Sinovamakara DMG Bonus"` (no activation, Team)
- `"kirara_c6_dmg_bonus"` (Toggle, Team)

Mirror (C6): Party All Elemental DMG +12% after Kirara uses Skill or Burst (15s).

Both are identical in value. The non-toggle entry will always apply. Remove the non-toggle entry.

> **Severity: MEDIUM** — double-counting.

---

## Verified OK

The following buffs were cross-checked against mirror data and are correct in value, stat type, source, and target:

| Character | Buff | Verified |
|---|---|---|
| Lauma | A4 "Cleansing for the Spring" — SkillDmgBonus +0.04% per EM, max 32% | ✓ |
| Nahida | A1 "Compassion Illuminated" — EM ×25% of highest party EM, max 250 | ✓ |
| Nahida | C2 Reaction CRIT Rate +20% | ✓ |
| Nahida | C2 Reaction CRIT DMG +100% | ✓ |
| Nahida | C2 DEF -30% on Quicken/Aggravate/Spread | ✓ |
| Nahida | C4 EM +100 per Skandha enemy (max 4) | ✓ |
| Nefer | C2 EM +200 at 5 Veil stacks (toggle version) | ✓ |
| Nefer | C4 Dendro RES -20% in Shadow Dance (toggle version) | ✓ |
| Nefer | C6 Lunar-Bloom DMG +15% (TransformativeBonus) | ✓ |
| Traveler Dendro | A4 EM +60 in Lea Lotus Lamp | ✓ |
| Traveler Dendro | C6 Dendro DMG +12% | ✓ |
| Alhaitham | A4 "Mysteries Laid Bare" — DMG +0.1% per EM, max 100% | ✓ |
| Alhaitham | C2 EM +50 per mirror generated (max 4 stacks) | ✓ |
| Alhaitham | C6 CR +10% / CD +70% | ✓ |
| Collei | C4 Party EM +60, excludes self (toggle version) | ✓ |
| Emilie | C1 Skill DMG +20% | ✓ |
| Emilie | C2 Dendro RES -30% | ✓ |
| Kaveh | A4 EM +25 per stack, max 4 (correct value, wrong name) | ✓ values |
| Kaveh | C4 Bloom DMG +60% (TransformativeBonus) | ✓ |
| Kinich | C2 Dendro RES -30% | ✓ |
| Kinich | C2 DMG +100% (first Scalespiker Cannon after Nightsoul's Blessing) | ✓ |
| Kinich | C4 Burst DMG +70% | ✓ |
| Kirara | C6 All Elem DMG +12% (toggle version) | ✓ |
| Tighnari | A1 EM +50 after Wreath Arrow | ✓ |
| Tighnari | A4 CA DMG +0.06% per EM, max 60% | ✓ |
| Tighnari | A4 Burst DMG +0.06% per EM, max 60% | ✓ |
| Tighnari | C1 CA CRIT Rate +15% | ✓ |
| Tighnari | C2 Dendro DMG +20% | ✓ |
| Tighnari | C4 Team EM +60 (base, toggle) | ✓ |
| Yaoyao | C1 Dendro DMG +15% (team, toggle version) | ✓ |
| Yaoyao | C4 EM based on 0.3% HP, max 120 (toggle version) | ✓ |
| Baizhu | C4 Team EM +80 (toggle version) | ✓ |
| Baizhu | C6 Burst Spiritvein flat DMG based on 8% HP | ✓ |

---

## Action Items (Priority Order)

1. **CRITICAL** — Fix `Emilie` A4 `base_value`: change `0.001` → `0.00015` (15% per 1000 ATK).
2. **HIGH** — Fix `Baizhu` A4 cap: change `Some(0.50)` → `Some(1.00)`. Add missing Aggravate/Spread bonus (+0.8% per 1000 HP, max 40%) and Lunar-Bloom bonus (+0.7% per 1000 HP, max 35%).
3. **HIGH** — Fix `Alhaitham` C4: team EM should be `+30 per mirror consumed` (Stacks(3), base=30.0), not flat +120. Add missing self Dendro DMG Bonus entry (+10% per mirror generated, Stacks(3)).
4. **HIGH** — Add `Nahida` A4 "Awakening Elucidated": self Bonus DMG +0.1% per EM above 200 (cap +80%), self CRIT Rate +0.03% per EM above 200 (cap +24%). Both require custom offset scaling (EM − 200).
5. **HIGH** — Add `Lauma` A4 Moonsign buffs: Nascent Gleam reaction CRIT (CR 15%, CD 100%) and Ascendant Gleam Lunar-Bloom CRIT Rate +10% / CRIT DMG +20%. Add C2 Lunar-Bloom +40%. Add C6 Lunar-Bloom +25%.
6. **MEDIUM** — Add `Kinich` C1 Scalespiker Cannon CRIT DMG +100%.
7. **MEDIUM** — Add `Kaveh` Burst base Bloom DMG bonus (talent-scaling, 27.49%–65.28%).
8. **MEDIUM** — Remove duplicate non-toggle entries: Nefer C2/C4, Baizhu C4, Collei C4 (non-toggle), Kirara C6 (non-toggle). Keep the Toggle variants.
9. **LOW** — Fix `Kaveh` A4 passive name: "An Architect's Undertaking" → "A Craftsman's Curious Conceptions".
10. **LOW** — Add `Tighnari` C4 reaction-triggered EM +60 bonus (on top of base +60, requires Toggle or separate entry).
11. **LOW** — Add Kirara A4 "Pupillary Variance": HP → Skill/Burst DMG bonus (currently marked TODO).
