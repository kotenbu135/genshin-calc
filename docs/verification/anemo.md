# Anemo Characters Verification

Verified against `honeyhunter-mirror/md/characters/*.md` spec files.
Date: 2026-04-12

## Summary Table

| Character | Scaling | Passives | Constellations | Status |
|-----------|---------|----------|----------------|--------|
| Chasca | OK | OK | OK | PASS |
| Faruzan | OK | BUG (burst Anemo DMG values) | OK | FAIL |
| Heizou | OK | OK | OK | PASS |
| Ifa | OK | OK | OK | PASS |
| Jahoda | BUG (aimed shot Lv11-15) | OK | OK | FAIL |
| Jean | OK | OK | OK | PASS |
| Kazuha | OK | OK | OK | PASS |
| Lan Yan | OK | OK | OK | PASS |
| Lynette | OK | OK | OK | PASS |
| Mizuki | OK | OK | OK | PASS |
| Sayu | OK | OK | OK | PASS |
| Sucrose | OK | OK | OK | PASS |
| Varka | OK | MISSING (A1 Lyrical Libation) | MISSING (C6 CRIT DMG) | FAIL |
| Venti | OK | OK | OK | PASS |
| Wanderer | OK | OK | OK | PASS |
| Xiao | OK | MISSING (A1 all DMG bonus) | OK | FAIL |
| Xianyun | OK | REVIEW (A4 flat DMG vs DMG bonus) | OK | REVIEW |

**Result: 12 PASS / 4 FAIL / 1 REVIEW**

## Issues

### BUG-1: Jahoda aimed shot scaling uses 5-star values at Lv11-15

**File:** `crates/data/src/characters/anemo/jahoda.rs`
**Severity:** Medium

Jahoda is a 4-star bow character but `JAHODA_AIMED` and `JAHODA_AIMED_FULL` use 5-star aimed shot scaling at Lv11-15.

| Entry | Lv11 (current) | Lv11 (expected) | Lv15 (current) | Lv15 (expected) |
|-------|----------------|-----------------|-----------------|-----------------|
| JAHODA_AIMED | 0.9371 | 0.9282 | 1.2745 | 1.1730 |
| JAHODA_AIMED_FULL | 2.3610 | 2.356 | 3.0355 | 2.945 |

Expected 4-star aimed shot values (from mirror `jahoda_124.md`):
- JAHODA_AIMED Lv11-15: `0.9282, 0.9894, 1.0506, 1.1118, 1.1730`
- JAHODA_AIMED_FULL Lv11-15: `2.356, 2.48, 2.635, 2.79, 2.945`

### BUG-2: Faruzan burst Anemo DMG Bonus scaling — all 15 levels wrong

**File:** `crates/data/src/talent_buffs/anemo.rs` line 5-8
**Severity:** High (affects all Faruzan team comps)

`FARUZAN_BURST_ANEMO_SCALING` values do not match the mirror's "Anemo DMG Bonus" row.

| Level | Current | Expected (mirror) |
|-------|---------|-------------------|
| Lv1 | 0.182 | 0.18 |
| Lv2 | 0.196 | 0.1935 |
| Lv3 | 0.209 | 0.207 |
| Lv4 | 0.228 | 0.225 |
| Lv5 | 0.241 | 0.2385 |
| Lv6 | 0.255 | 0.252 |
| Lv7 | 0.273 | 0.27 |
| Lv8 | 0.291 | 0.288 |
| Lv9 | 0.310 | 0.306 |
| Lv10 | 0.328 | 0.324 |
| Lv11 | 0.346 | 0.342 |
| Lv12 | 0.364 | 0.36 |
| Lv13 | 0.387 | 0.3825 |
| Lv14 | 0.410 | 0.405 |
| Lv15 | 0.432 | 0.4275 |

Source comment says "Values from Genshin Wiki" -- should be sourced from honeyhunter mirror instead.

### MISSING-1: Xiao A1 "Conqueror of Evil: Tamer of Demons" — all DMG bonus

**File:** `crates/data/src/talent_buffs/anemo.rs`
**Severity:** Medium

Mirror (`xiao_026.md`): "While under the effects of Bane of All Evil, all DMG dealt by Xiao increases by 5%. DMG increases by a further 5% for every 3s the ability persists. The maximum DMG Bonus is 25%."

This passive is entirely missing from `XIAO_BUFFS`. The existing Xiao buff named "Transcension: Gravity Defier Skill DMG Bonus" corresponds to the A4 passive "Dissolution Eon: Heaven Fall" (Skill DMG +15%/stack), not A1. The A1 all-DMG bonus (up to 25%) during burst is not implemented.

Note: The buff name "Transcension: Gravity Defier Skill DMG Bonus" is also incorrect — it should reference "Dissolution Eon: Heaven Fall."

### MISSING-2: Varka C6 CRIT DMG per Azure Fang's Oath stack

**File:** `crates/data/src/talent_buffs/anemo.rs`
**Severity:** Medium

Mirror (`varka_128.md`) C6: "Every stack of Azure Fang's Oath will increase Varka's CRIT DMG by 20%."

Max 4 stacks = up to 80% CRIT DMG. This C6 enhancement to the A4 "Wind's Vanguard" effect is not implemented.

### MISSING-3: Varka A1 "Lyrical Libation" — 200% DMG multiplier

**File:** `crates/data/src/talent_buffs/anemo.rs`
**Severity:** Low (one-time conditional, may be out of scope for stat-buff system)

Mirror (`varka_128.md`) A1 second effect: "After switching to Sturm und Drang, Varka will gain the Lyrical Libation effect: When unleashing Four Winds' Ascension, or performing Azure Devour, Varka will consume this effect to deal 200% of the original DMG."

This is a one-time consumed effect that doubles DMG on the first use. May not be representable in the current stat-buff system (it's a conditional multiplier, not a persistent stat buff).

### REVIEW-1: Xianyun burst plunging flat DMG — talent-scaling vs fixed 200% ATK

**File:** `crates/data/src/talent_buffs/anemo.rs` line 479-498
**Severity:** Needs investigation

The buff "Stars Gather at Dusk Plunging Flat DMG" uses `XIANYUN_BURST_PLUNGE_SCALING` with talent-level-dependent values [2.48, 2.666, ...]. However:
- The A4 passive "Consider, the Adeptus in Her Realm" states a fixed "200% of Xianyun's ATK" (capped at 9000) as flat plunging DMG.
- The Starwicker DMG from the burst talent table (39.2% ATK at Lv1) is already captured separately in `XIANYUN_BURST_STARWICKER`.
- The values [2.48, 2.666, ...] do not appear in the mirror's talent tables.

Additionally, the A4 buff is implemented as `PlungingAtkDmgBonus = 0.75` but the mirror describes it as a flat DMG addition (200% ATK), not a percentage damage bonus. This needs further investigation to determine the correct modeling.

## Character Notes

### Chasca (5-star Bow, Natlan)
Mirror: `chasca_104.md`. All normal/charged/plunge/skill/burst scalings verified. 5-star aimed shot values correctly used. Ascension stat CritRate 19.2% matches.

### Faruzan (4-star Bow, Sumeru)
Mirror: `faruzan_076.md`. Talent scalings in `faruzan.rs` verified correct. Burst Anemo DMG Bonus buff values in `talent_buffs/anemo.rs` have wrong source (Wiki vs mirror). See BUG-2. A4 Anemo RES -30% correctly implemented.

### Heizou (4-star Catalyst, Inazuma)
Mirror: `heizo_059.md`. All normal/charged/plunge/skill/burst scalings verified. Declension stack bonus and Conviction bonus match.

### Ifa (4-star Catalyst, Natlan)
Mirror: `ifa_113.md`. All scalings verified. Burst and Sedation Mark values match.

### Jahoda (4-star Bow, Snezhnaya)
Mirror: `jahoda_124.md`. Normal attack and skill/burst scalings correct. Aimed shot Lv1-10 correct. Lv11-15 uses wrong 5-star values. See BUG-1.

### Jean (5-star Sword, Mondstadt)
Mirror: `qin_003.md`. All scalings verified via automated comparison. C4 Anemo RES -40% correctly implemented.

### Kazuha (5-star Sword, Inazuma)
Mirror: `kazuha_047.md`. All scalings manually verified. A4 EM-to-Elemental DMG conversion correctly implemented. C2 EM+200 correctly implemented.

### Lan Yan (4-star Catalyst, Liyue)
Mirror: `lanyan_108.md`. All scalings verified. Burst 241.06% x3 matches code value 2.41064.

### Lynette (4-star Sword, Fontaine)
Mirror: `linette_083.md`. All normal/charged/plunge/skill/burst scalings verified. 3-hit two-part damage correctly split into LYNETTE_NORMAL_3A and LYNETTE_NORMAL_3B.

### Mizuki (5-star Catalyst, Inazuma)
Mirror: `mizuki_109.md`. All scalings verified. Skill continuous attack and burst shockwave values match. Ascension stat EM 115.2 matches.

### Sayu (4-star Claymore, Inazuma)
Mirror: `sayu_053.md`. All scalings manually verified. Muji-Muji Daruma and Fuufuu Whirlwind Kick values match.

### Sucrose (4-star Catalyst, Mondstadt)
Mirror: `sucrose_043.md`. All scalings verified via automated comparison. A4 EM share correctly implemented.

### Varka (5-star Claymore, Mondstadt)
Mirror: `varka_128.md`. All talent scalings verified. Sturm und Drang multi-hit entries correctly split. A1 elemental DMG per ATK and A4 Normal/Charged DMG bonus implemented. Missing C6 CRIT DMG buff and A1 Lyrical Libation 200% multiplier. See MISSING-2, MISSING-3.

### Venti (5-star Bow, Mondstadt)
Mirror: `venti_022.md`. All scalings manually verified. C2/C4/C6 buffs correctly implemented.

### Wanderer (5-star Catalyst, Sumeru)
Mirror: `wanderer_075.md`. All scalings verified via automated comparison. Burst scaling matches.

### Xiao (5-star Polearm, Liyue)
Mirror: `xiao_026.md`. All talent scalings verified. Burst Normal/Charged/Plunging DMG bonus [58.45%...117.95%] matches. A4 "Dissolution Eon: Heaven Fall" Skill DMG +15%/stack implemented but named incorrectly. A1 "Conqueror of Evil: Tamer of Demons" all DMG +5% (max 25%) missing. See MISSING-1.

### Xianyun (5-star Catalyst, Liyue)
Mirror: `liuyun_093.md`. All talent scalings verified. Burst plunging flat DMG buff uses unexplained talent-level scaling; A4 flat DMG modeled as percentage bonus instead of flat addition. See REVIEW-1.
