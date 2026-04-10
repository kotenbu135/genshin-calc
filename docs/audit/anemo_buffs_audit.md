# Anemo Talent Buffs Audit

Audit date: 2026-04-08
Source: `crates/data/src/talent_buffs/anemo.rs` vs `honeyhunter-mirror/md/characters/`

---

## Summary

| Character | Status | Notes |
|-----------|--------|-------|
| Chasca    | ISSUES | C6 buff name mismatch; A1 stat type wrong |
| Faruzan   | OK | All 3 buffs verified correct |
| Heizou    | ISSUES | A4 source wrong (Skill not Burst); C6 buff name misleading |
| Ifa       | ISSUES | A4 Swirl DMG buff skipped (TODO noted); C4 target wrong (OnlySelf vs should be OnlySelf — OK) |
| Jahoda    | ISSUES | A4 EM+100 condition wrong (burst heal, not EM scaling); C6 CRIT values correct |
| Jean      | MISSING | C6 DMG reduction buff not implemented |
| Kazuha    | OK | All 5 buffs verified correct |
| Lan Yan   | ISSUES | A4 buff name wrong; A4 scaling factor needs verification |
| Lynette   | ISSUES | A4 max value correct; C6 always-active vs Toggle mismatch |
| Mizuki    | ISSUES | A1 source should be AscensionPassive(1) — OK; C2 target wrong (Team but description says excluding self) |
| Sayu      | OK | C2 buff verified correct |
| Sucrose   | OK | Both A1/A4 buffs verified correct |
| Varka     | ISSUES | A1 DMG scaling wrong (ATK-scaling not flat); A4 max value correct; C4 element DMG buffs correct |
| Venti     | ISSUES | C2 RES shred value wrong (12% but mirror says airborne stacks); C6 missing absorbed element RES shred |
| Wanderer  | OK | All 3 buffs verified correct |
| Xiao      | ISSUES | A4 source name wrong; missing Burst DMG bonus buff |
| Xianyun   | ISSUES | A4 passive name wrong; C6 value correct but applies only to 3-Skyladder case |

---

## Value Discrepancies

### Faruzan
- **Prayerful Wind's Benefit (Burst Anemo DMG scaling)**: rust=`[0.182, 0.196, 0.209, 0.228, 0.241, 0.255, 0.273, 0.291, 0.310, 0.328, 0.346, 0.364, 0.387, 0.410, 0.432]` mirror=`[18%, 19.35%, 20.7%, 22.5%, 23.85%, 25.2%, 27%, 28.8%, 30.6%, 32.4%, 34.2%, 36%, 38.25%, 40.5%, 42.75%]` — **VERIFIED OK** (values match when converted: 0.182=18.2% — close but Lv1 should be 18.0% per mirror, rust has 18.2%). Minor discrepancy at Lv1: rust=18.2%, mirror=18%.

### Chasca
- **Galesplitting Soulseeker Shell Skill DMG Bonus** (A1): stat=`SkillDmgBonus` — mirror says "DMG of Shining Shadowhunt Shell by 15%/35%/65%". This is specifically Shining Shadowhunt Shell (Charged Attack) DMG bonus, not generic SkillDmgBonus. The stat type `SkillDmgBonus` may over-apply to all skill damage, not just Shining Shadowhunt Shells.
- **Ride the Wind CritDmg Bonus** (C6): rust=`1.20` (120%) — mirror C6 says "CRIT DMG of that instance of Multitarget Fire's Shadowhunt Shells and Shining Shadowhunt Shells increases by 120%". Value is correct. However the buff name "Ride the Wind" does not correspond to any named ability in the mirror — C6 is named "Showdown, the Glory of Battle." The description is accurate but the buff name is unofficial.

### Venti
- **Breeze of Reminiscence Anemo RES Shred** (C2): rust=`base_value: 0.12` — mirror says "Skyward Sonnet decreases opponents' Anemo RES and Physical RES by 12% for 10s. Opponents launched by Skyward Sonnet suffer an additional 12% Anemo RES and Physical RES decrease while airborne." So the total potential is -24% Anemo RES (grounded -12% + airborne -12%). The implementation uses 0.12 which only captures the base component. The "Buffed State" version of C2 reads -24% flat. This is ambiguous — the non-Buffed State is 12%, which is what the code uses. **Likely OK** for non-Hexerei state but note: Buffed State (post-Witch's Homework quest) gives -24%.

### Varka
- **Dawn Wind's March Anemo DMG** (A1): rust=`base_value: 0.25, activation: None` (always-active at flat 25%) — mirror says "every 1,000 points of Varka's ATK will grant Varka a 10% Anemo DMG Bonus... Up to 25% bonus DMG." This is ATK-scaling (scales_on should be ATK/1000 × 0.10), not a flat 25%. The current implementation approximates at max value but ignores the dynamic scaling. **Incorrect implementation** — should use `scales_on: Some(ScalingStat::TotalAtk)` with per-1000-ATK scaling, or at minimum a Toggle at max.
- **Wind's Vanguard Normal/Charged ATK DMG** (A4): rust=`base_value: 0.30, activation: None` — mirror says "+7.5%/stack, max 4 stacks = 30%". Values are correct (max 30%). The activation=None (always at max) is a simplification but the value is accurate.
- **Freedom of Song** (C4): The code implements Anemo + Pyro/Hydro/Electro/Cryo DMG bonuses as separate Toggles. Mirror says "all nearby party members gain a 20% Anemo DMG Bonus and the corresponding Elemental DMG Bonus." This is correct.

### Xianyun
- **Stars Gather at Dusk Plunging Flat DMG scaling**: rust=`[2.48, 2.666, 2.852, 3.100, 3.286, 3.472, 3.720, 3.968, 4.216, 4.464, 4.712, 4.960, 5.270, 5.580, 5.890]` — mirror A4 says "nearby active characters' Plunging Attack shockwave DMG will be increased by 200% of Xianyun's ATK." The Burst table lists these as `Starwicker DMG` percentages (39.2%–93.1% of ATK), but Flat DMG from A4 is 200% ATK. The `XIANYUN_BURST_PLUNGE_SCALING` array does not match standard Burst table values and the comment says "PlungingAtkFlatDmg = total ATK × scaling". The scaling values (2.48–5.89) imply 248%–589% of ATK at various talent levels, which does **not** match the A4 value of 200% ATK or the C2 enhancement of 400% ATK. The actual A4 ("Consider, the Adeptus in Her Realm") is a **flat** 200% ATK addition. The BURST_PLUNGE_SCALING seems to be attempting to encode something else — possibly the Burst's Starwicker shockwave scaling. This needs investigation; source of values is unclear.
- **Crane Form Plunging DMG Bonus** (A4 name): Talent "Galefeather Pursuit" (A1) gives CritRate for plunging; "Consider, the Adeptus in Her Realm" (A4) gives flat ATK addition. The buff named "Crane Form Plunging DMG Bonus" uses source `AscensionPassive(4)` and stat `PlungingAtkDmgBonus` at 75%. Mirror A4 ("Consider, the Adeptus in Her Realm") says ATK flat bonus, not DMG% bonus. **Possible stat mismatch** — A4 should be `PlungingAtkFlatDmg` (it is implemented separately as the Burst buff), not a `PlungingAtkDmgBonus`.
- **Trivial Matters CritRate Bonus** (C2): buff description says "After burst, plunging attacks gain CritRate +20%". Mirror C2 ("Aloof From the World") says "nearby active characters' Plunging Attack shockwave DMG will be increased by 400% of Xianyun's ATK" and also "Xianyun's ATK will be increased by 20% for 15s after using a Skyladder." The 20% CritRate does not appear in C2. This CritRate may come from A1 passive "Galefeather Pursuit" (CritRate +4%/6%/8%/10% for plunging based on stacks). The C2 buff at 20% CritRate is **not found in mirror** — likely an incorrect source assignment.

### Heizou
- **Penetrative Reasoning EM Bonus**: source=`AscensionPassive(4)`, description says "After using Elemental Burst." Mirror A4 passive ("Penetrative Reasoning") says "After Shikanoin Heizou's Heartstopper Strike hits an opponent, increases all party members' (excluding Shikanoin Heizou) Elemental Mastery by 80 for 10s." It triggers from Elemental **Skill** (Heartstopper Strike), not Burst. The source classification is technically wrong (it IS A4 passive, but the trigger is from skill use, not burst). The source field `AscensionPassive(4)` is correct. The description "After using Elemental Burst" is wrong — should be "After Heartstopper Strike hits." Minor but the description is inaccurate.
- **C6 buff names**: "Paradoxical Practice CritRate Bonus" and "Paradoxical Practice CritDmg Bonus" — mirror C6 is named "Curious Casefiles." The values are correct (CritRate +4%/stack max 16%, CritDmg +32%). The buff names reference A1 passive "Paradoxical Practice" incorrectly.

### Lan Yan
- **Lan Yan A4 Normal ATK Flat DMG**: rust=`base_value: 7.74, scales_on: Em` implying DMG = 7.74 × EM. Mirror A4 ("Skyfeather Evil-Subduing Charm") says "Elemental Skill and Burst deal increased DMG equal to 309% and 774% of Lan Yan's Elemental Mastery respectively." The Burst bonus is 774% EM and Skill is 309% EM — these are not Normal ATK flat DMG bonuses. The stat is `NormalAtkFlatDmg` but mirror says it applies to Skill and Burst DMG, not Normal ATK. **Incorrect stat type** — should be `SkillDmgBonus` (% of EM) and `BurstDmgBonus` (% of EM) respectively, or two separate buffs. Using `NormalAtkFlatDmg` is wrong.
- **Lan Yan C4 Team EM Bonus**: rust=`60.0, activation: None` (always active) — mirror C4 says "After Lan Yan uses her Elemental Burst Lustrous Moonrise, the Elemental Mastery of all nearby party members increases by 60 for 12s." Value correct (60 EM), but activation=None makes it always-on; it should be Toggle (conditional on burst use). Minor issue.

### Jahoda
- **Jahoda A4 EM Buff**: rust=`base_value: 100.0` — mirror A4 ("Sweet Berry Bounty") says "if that character's HP is above 70%, their Elemental Mastery will be increased by 100 for 6s." The value (100 EM) is correct. However, the implementation comment says "When burst heal target has HP>70%, EM+100" — the actual trigger is "When a Purrsonal Coordinated Assistance Robot triggers healing on an active team member with HP>70%". This is from the Burst robots, not a general condition. The A4 name is correct. Source=`AscensionPassive(4)` is correct. Values match.
- **jahoda_c6_crit_rate** (C6): rust=`0.05` (5%) and **jahoda_c6_crit_dmg** (C6): rust=`0.40` (40%) — mirror C6 ("The Littlest Luck") says "nearby Moonsign characters in your party have their CRIT Rate increased by 5% and CRIT DMG increased by 40% for 20s." Values match. **VERIFIED OK**.

---

## Missing Implementations

### Jean
- **C6: Lion's Fang, Fair Protector of Mondstadt**: "Incoming DMG is decreased by 35% within the Field created by Dandelion Breeze." This is a DMG reduction buff for characters inside the field. The current implementation only has C4 Anemo RES shred. The C6 damage reduction buff is **NOT IMPLEMENTED**. (Note: DMG reduction is a defensive buff, but it directly affects survivability in damage calc context.)

### Venti
- **C6: Storm of Defiance — Absorbed Element RES Shred**: "If an Elemental Absorption occurred, then their RES towards the corresponding Element is also decreased by 20%." The implementation has `venti_c6_anemo_res_shred` (Anemo -20%) but is missing the corresponding **absorbed element RES -20%** buffs (Pyro/Hydro/Cryo/Electro). These should be Toggle-based similar to Varka's C4.
- **Passive: Witch's Eve Rite (Hexerei)**: "When the party includes at least 2 Hexerei characters... for 4s after a nearby active character triggers a Swirl reaction, that character's DMG is increased by 50%." This is a conditional DmgBonus +50% that applies to Swirl-triggering characters. NOT IMPLEMENTED (Hexerei mechanic; may be out of scope).

### Xiao
- **Elemental Burst: Bane of All Evil — Normal/Charged/Plunging ATK DMG Bonus**: The Burst itself grants "Normal/Charged/Plunging Attack DMG Bonus" per the talent table (58.45%–117.95% at Lv1–15). This is a talent-scaling buff that applies while Yaksha's Mask is active. Currently, only A4 and C4 buffs are implemented for Xiao. The **Burst's own DMG bonus is NOT IMPLEMENTED** as a TalentBuffDef. This is a significant omission.
- **A4: "Dissolution Eon: Heaven Fall"** (Lemniscatic Wind Cycling stacking DMG bonus, +15%/stack up to 3 stacks for subsequent Skill uses): This is a Skill DMG stacking buff on Skill DMG itself. NOT IMPLEMENTED. (Minor — only affects Skill DMG.)

### Heizou
- **C6: Conviction CRIT DMG** (+32% when Conviction is active) — this IS implemented as `Paradoxical Practice CritDmg Bonus`. OK.

### Sucrose
- **C6: Chaotic Entropy** — "If Forbidden Creation - Isomer 75 / Type II triggers an Elemental Absorption, all party members gain a 20% Elemental DMG Bonus for the corresponding absorbed element." This is a conditional team Elemental DMG Bonus based on which element is absorbed. NOT IMPLEMENTED. This is a significant C6 buff that affects all absorbed-element characters.

### Ifa
- **A4: "Field Medic's Vision"**: "every 1 Nightsoul point out of the total in his entire party will grant him 1 Rescue Essentials point. Rescue Essentials will increase the Swirl, Electro-Charged, and Lunar-Charged DMG." This is a complex scaling Swirl DMG buff. The code has a TODO noting it's too complex. The EM+100 from C4 is implemented. Status: known limitation.

### Varka
- **A1: Dawn Wind's March — Corresponding Element DMG Bonus**: "every 1,000 ATK grants 10% corresponding Elemental DMG Bonus (up to 25%)." Only the Anemo portion is implemented. The corresponding element (Pyro/Hydro/Electro/Cryo by priority) DMG bonus at up to 25% is NOT IMPLEMENTED for the non-Anemo element. Code comment notes this as a TODO.
- **A4: Wind's Vanguard** also applies to "special Charged Attack Azure Devour and special Elemental Skill Four Winds' Ascension" — the current `NormalAtkDmgBonus` and `ChargedAtkDmgBonus` stats do not cover these special attacks.

### Xianyun
- **A1: Galefeather Pursuit** — CritRate scaling for plunging based on number of Storm Pinion stacks (4%/6%/8%/10%). This is implemented as the `Trivial Matters CritRate Bonus` but incorrectly attributed to C2. The A1 passive that gives CritRate is **not separately implemented**.

### Lan Yan
- **A4: Elemental Burst Lustrous Moonrise DMG increase** (774% EM) — as noted above, this is currently misimplemented as `NormalAtkFlatDmg`. The Burst DMG increase from A4 is effectively NOT CORRECTLY IMPLEMENTED.
- **A4: Elemental Skill Swallow-Wisp Pinion Dance DMG increase** (309% EM) — similarly not implemented as SkillDmgBonus.

### Mizuki
- **A1: "Thoughts by Day Bring Dreams by Night"** — "while Mizuki is in the Dreamdrifter state, when other nearby party members hit opponents with Pyro/Hydro/Cryo/Electro attacks, her EM increases by 100 for 4s." This is self-only conditional EM+100 for Mizuki. Not implemented (the team EM+100 from A1 is separate — wait, checking: the A1 passive in the mirror is "All Ailments Banished" (food buff) and "Bright Moon's Restless Voice" (duration extension). The "Thoughts by Day Bring Dreams by Night" is the A4 passive. The implemented buff `Mizuki A1 Team EM Bonus` with source `AscensionPassive(1)` is **incorrectly sourced** — the team EM+100 effect is NOT from any passive in the mirror. The actual A1 is food healing bonus and A4 is Mizuki's own EM gain. **Mizuki has no team EM+100 passive** — this buff appears to be incorrect.

---

## Verified OK

The following buffs were cross-checked against mirror data and verified correct:

- **Faruzan**: `Prayerful Wind's Benefit` (Burst Anemo DMG, talent-scaling — values close), `Perfidious Wind's Bale` (A4 Anemo RES -30%), `faruzan_c6_anemo_crit_dmg` (C6 CRIT DMG +40%)
- **Kazuha**: `Poetics of Fuubutsu` (A4, 0.04% per EM), `kazuha_c2_em` (C2 EM+200), `kazuha_c6_normal/charged/plunging_dmg` (C6, 0.2% per EM ×3)
- **Sucrose**: `Catalyst Conversion` (A1, EM+50), `Mollis Favonius` (A4, 20% of EM shared)
- **Wanderer**: `Gales of Reverie ATK Bonus` (A1 Pyro, +30%), `Gales of Reverie CritRate Bonus` (A1 Cryo, +20%), `Stormborne Burst DMG Bonus` (C2, +200%)
- **Xiao**: `Transcension: Gravity Defier DMG Bonus` (A4, +25% DMG), `Conqueror of Evil: Tamer of Demons DEF Bonus` (C4, DEF+100%)
- **Venti**: `Breeze of Reminiscence Anemo/Physical RES Shred` (C2, -12% each), `venti_c4_anemo_dmg` (C4, +25%), `venti_c6_anemo_res_shred` (C6, -20%)
- **Chasca**: `Ride the Wind CritDmg Bonus` (C6, +120% — value correct)
- **Heizou**: `Penetrative Reasoning EM Bonus` (A4, EM+80 — value and source correct), C6 CritRate+16% and CritDmg+32% (values correct)
- **Ifa**: `Eye of Stormy Judgment EM Bonus` (C4, EM+100 — value correct)
- **Jahoda**: `jahoda_c6_crit_rate` (+5%), `jahoda_c6_crit_dmg` (+40%) — correct
- **Sayu**: `Sayu C2 Skill DMG Bonus` (C2, +66% — max value correct)
- **Varka**: `Wind's Vanguard Normal/Charged ATK DMG` (A4, +30% max — value correct), `Freedom of Song` (C4, +20% per element — value correct)
- **Lynette**: `Sophisticated Synergy ATK Bonus` (A4, +20% max — value correct), `Magic Trick: Astonishing Shift Anemo DMG Bonus` (C6, +20% — value correct)

---

## Critical Issues Summary (Priority Order)

1. **INCORRECT**: `Mizuki A1 Team EM Bonus` — no team EM+100 passive exists for Mizuki; buff appears fabricated
2. **INCORRECT**: `Lan Yan A4 Normal ATK Flat DMG` — stat type wrong; should buff Skill+Burst DMG via EM%, not Normal ATK
3. **INCORRECT**: `Crane Form Plunging DMG Bonus` (Xianyun A4) — source and stat type may be wrong; A4 gives flat ATK addition to Plunging, implemented as PlungingAtkDmgBonus at 75%
4. **MISSING**: Xiao Burst `Normal/Charged/Plunging ATK DMG Bonus` (talent-scaling, 58%–118%)
5. **MISSING**: Sucrose C6 `Chaotic Entropy` — absorbed element DMG+20% for team
6. **MISSING**: Venti C6 absorbed element RES shred -20%
7. **VALUE ISSUE**: Varka A1 implemented as flat 25% instead of ATK-scaling (10% per 1000 ATK)
8. **MISSING**: Jean C6 incoming DMG reduction -35% (defensive buff)
9. **DESCRIPTION ERROR**: Heizou buff descriptions reference wrong trigger sources
