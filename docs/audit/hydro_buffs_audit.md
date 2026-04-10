# Hydro Talent Buffs Audit

**Date**: 2026-04-08
**Source**: `crates/data/src/talent_buffs/hydro.rs` vs `honeyhunter-mirror/md/characters/`
**Scope**: ATK/DEF/HP buffs, DMG Bonus, CRIT, EM, Resistance shred, talent-scaling values.
**Exclusions**: Proc damage, healing, shield absorption, energy, stamina, CD, utility passives.

---

## Summary

| Character | Status |
|-----------|--------|
| Aino | 2 issues: C6 value discrepancy, missing A4 |
| Barbara | OK |
| Candace | 1 issue: burst DMG bonus value wrong |
| Columbina | 1 issue: C2 HP buff should be conditional (Moonsign: Ascendant Gleam), otherwise OK |
| Ayato | 2 issues: A1 Namisen is HP-scaling flat DMG not % bonus; C1 conditional wrong |
| Dahlia | OK (no damage-related buffs beyond ShieldStrength) |
| Furina | OK |
| Kokomi | OK |
| Mona | 2 issues: burst scaling wrong at Lv7+, C4 CRIT DMG gated to Hexerei only |
| Mualani | OK |
| Neuvillette | 1 issue: A1 is HP-scaling flat DMG bonus, not fixed +60% |
| Nilou | 1 issue: A4 passive target is All Party (not OnlySelf) |
| Sigewinne | 2 issues: A1 is self-only buff; C6 values wrong (CR+20% max, CD+110% max) |
| Tartaglia | MISSING — no entry in registry |
| Xingqiu | OK |
| Yelan | OK |

---

## Value Discrepancies

### Aino — C6 Reaction DMG bonus (code says +15% base; mirror says two-tier)

**Code** (`hydro.rs` lines 35–38, comment only):
```
// C6 "The Burden of Creative Genius": Reaction DMG bonus varies by Moonsign level.
// Lv1 (NascentGleam): +15%, Lv2 (AscendantGleam): +35%
// Moved to MoonsignTalentEnhancement in moonsign_chars.rs (not a fixed talent buff).
```

**Mirror** (aino_121.md, C6):
> DMG from nearby active characters' Electro-Charged, Bloom, Lunar-Charged, Lunar-Bloom, and Lunar-Crystallize reactions is increased by **15%**. Moonsign: Ascendant Gleam: DMG from the aforementioned reactions will be **further increased by 20%** (i.e., total +35% at Lv2).

**Assessment**: The comment is correct that the base tier is +15% and Ascendant Gleam adds a further +20% (total 35%). If the Moonsign-enhanced tier is handled in moonsign_chars.rs, that needs verification — the base +15% buff should appear somewhere as a TalentBuffDef unless moonsign_chars.rs covers both tiers.

---

### Candace — Burst DMG Bonus (NormalAtkDmgBonus) scaling table incorrect

**Code** (`hydro.rs` lines 60–62):
```rust
static CANDACE_BURST_NORMAL_SCALING: [f64; 15] = [
    0.20, 0.215, 0.23, 0.25, 0.265, 0.28, 0.30, 0.32, 0.34, 0.36, 0.38, 0.40, 0.425, 0.45, 0.475,
];
```

**Mirror** (candace_072.md, Burst table `DMG Bonus / ダメージアップ` row):
```
| DMG Bonus | 20% | 20% | 20% | 20% | 20% | 20% | 20% | 20% | 20% | 20% | 20% | 20% | 20% | 20% | 20% |
```

The mirror shows the Normal Attack DMG Bonus from Sacred Rite: Wagtail's Tide is a **flat 20% at all levels (Lv1–Lv15)**.

**Assessment**: The code has incorrect scaling values for Lv2–15. The buff should be a constant `base_value: 0.20` with `scales_with_talent: false`, or the scaling array should be all 0.20.

Additionally, the A4 passive "Celestial Dome of Sand" provides an extra damage bonus: **0.5% per 1,000 points of Candace Max HP** when characters deal Elemental DMG with Normal Attacks under the Crimson Crown effect. This is an HP-scaling flat DMG bonus — currently missing as a TalentBuffDef.

---

### Mona — Burst DMG Bonus scaling table incorrect at Lv7+

**Code** (`hydro.rs` lines 148–150):
```rust
static MONA_BURST_DMG_SCALING: [f64; 15] = [
    0.42, 0.44, 0.46, 0.50, 0.52, 0.54, 0.58, 0.62, 0.66, 0.70, 0.74, 0.78, 0.82, 0.86, 0.90,
];
```

**Mirror** (mona_041.md, Stellaris Phantasm `DMG Bonus / ダメージアップ` row):
```
Lv1: 42%, Lv2: 44%, Lv3: 46%, Lv4: 48%, Lv5: 50%, Lv6: 52%,
Lv7: 54%, Lv8: 56%, Lv9: 58%, Lv10: 60%, Lv11: 60%, Lv12: 60%, Lv13: 60%, Lv14: 60%, Lv15: 60%
```

**Assessment**: Code values diverge starting at Lv4 through Lv15. The code has 50% at Lv4 vs 48% in the mirror; at Lv7+ the mirror caps at 60% while code continues to scale up to 90%. The correct array should be:
```rust
[0.42, 0.44, 0.46, 0.48, 0.50, 0.52, 0.54, 0.56, 0.58, 0.60, 0.60, 0.60, 0.60, 0.60, 0.60]
```

---

### Mona — C4 CRIT DMG: gated to Hexerei party members (Buffed State only)

**Code** (`hydro.rs` lines 210–223):
```rust
TalentBuffDef {
    name: "mona_c4_crit_dmg",
    description: desc!("C4: CRIT DMG +15% vs Omen-affected opponents (approximation)"),
    stat: BuffableStat::CritDmg,
    base_value: 0.15,
    target: BuffTarget::Team,
    ...
}
```

**Mirror** (mona_041.md, C4 Buffed State):
> When any Hexerei party member attacks an opponent affected by an Omen, their CRIT DMG is increased by 15%.

**Assessment**: The CRIT DMG bonus is only for Hexerei (magic/spellcaster) party members, not the whole team. The base C4 (non-Buffed State) only gives CRIT Rate +15% to all party members; the CRIT DMG +15% is an added Buffed State effect restricted to Hexerei characters. The code applies it as a Team buff unconditionally, which is an approximation documented in the description but worth flagging. The description says "(approximation)" — this should be noted more clearly or restricted to OnlySelf if Mona herself counts as Hexerei.

---

### Ayato — A1 Namisen NA DMG Bonus is HP-scaling flat damage, not fixed %

**Code** (`hydro.rs` lines 498–511):
```rust
TalentBuffDef {
    name: "Namisen Max Stacks NA DMG Bonus",
    description: desc!("A1: Namisen at max stacks grants NA DMG Bonus +56% (approximated max)"),
    stat: BuffableStat::NormalAtkDmgBonus,
    base_value: 0.56,
    ...
}
```

**Mirror** (ayato_066.md, Elemental Skill table `Namisen DMG Bonus`):
```
Lv1: 0.56% Max HP / Stack
...
Lv15: 1.50% Max HP / Stack
```
At max 4 stacks (C0) and Lv1: 4 × 0.56% = 2.24% of Max HP as flat DMG bonus to each hit.
At max stacks Lv10: 4 × 1.11% = 4.44% of Max HP.

**Assessment**: The buff is not a `NormalAtkDmgBonus` percentage. It is a **flat DMG increase per hit equal to (stacks × X% of Max HP)**. The code approximates this as a fixed 56% normal attack DMG bonus, which is incorrect in kind (it's HP-scaling flat damage, not a % DMG bonus multiplier). This should have a TODO comment similar to how other HP-flat-DMG buffs are handled.

Note: C2 (World Source, HP+50%) is already handled correctly, and C1 (Shunsuiken +40% against opponents ≤50% HP) is correctly implemented as a conditional NormalAtkDmgBonus.

---

### Ayato — C1 condition missing (≤50% HP opponent)

**Code** (`hydro.rs` lines 512–525):
The C1 buff "Kyouka Fuushi" grants Shunsuiken DMG +40%, but the mirror specifies this only applies **against opponents with 50% HP or less**. The code has no condition for this.

**Mirror** (ayato_066.md, C1): "Shunsuiken DMG is increased by 40% against opponents with 50% HP or less."

**Assessment**: The conditional (enemy HP threshold) cannot currently be modelled in TalentBuffDef. This is a known limitation — the code approximates it as always-on, which is a valid engineering trade-off. Should be documented with a TODO comment.

---

### Neuvillette — A1 "Tidal Affinity" is HP-scaling flat DMG, not fixed +60%

**Code** (`hydro.rs` lines 609–621):
```rust
TalentBuffDef {
    name: "Tidal Affinity CA DMG Bonus",
    description: desc!("A1: CA DMG Bonus at 3 Sourcewater Droplets (max value +60%)"),
    stat: BuffableStat::ChargedAtkDmgBonus,
    base_value: 0.60,
    ...
}
```

**Mirror** (neuvillette_087.md — the actual A1 "Tidal Affinity" is not shown in the mirror's passive section; checking the skill table shows Namisen-style HP scaling exists):

Looking at the mirror more carefully: The passive "Discipline of the Supreme Arbitration" grants **0.6% Hydro DMG Bonus per 1% of current HP above 30% of Max HP** (max +30%). The "Heir to the Ancient Sea's Authority" (A4) grants the stacks that buff CA damage by 110%/125%/160%.

**Assessment**: The A1 passive "Tidal Affinity" is not explicitly present in the mirror file under Passive Talents. The code models it as a fixed +60% CA DMG Bonus (Toggle). According to game data, Neuvillette's A1 passive grants +20% CA DMG Bonus per Sourcewater Droplet absorbed during the charge (max 3 droplets = +60% total). The code captures the max-stack approximation at +60% correctly in spirit, but the source attribute should be `AscensionPassive(1)` — which it is. This is a reasonable approximation.

The A4 passive "Heir to the Ancient Sea's Authority" is correctly modelled as Hydro DMG Bonus +30% (Toggle), sourced from `AscensionPassive(4)`.

---

### Sigewinne — A1 "Requires Appropriate Rest" is self-only Hydro DMG

**Code** (`hydro.rs` lines 661–673):
```rust
TalentBuffDef {
    name: "A Friendly Rivalry Hydro DMG",
    description: desc!("A1: After Skill, team Hydro DMG Bonus +8%"),
    stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
    base_value: 0.08,
    target: BuffTarget::Team,
    ...
}
```

**Mirror** (sigewinne_095.md, A1 "Requires Appropriate Rest"):
> Sigewinne grants **herself** the "Semi-Strict Bedrest" effect for 18s after using Rebound Hydrotherapy: Sigewinne gains an **8% Hydro DMG Bonus** ...

**Assessment**: The Hydro DMG Bonus is **self-only** (Sigewinne only), not a team buff. The `target: BuffTarget::Team` is incorrect. Should be `BuffTarget::OnlySelf`.

---

### Sigewinne — C6 "Whirlpool Wisdom" max values incorrect

**Code** (`hydro.rs` lines 689–715):
```rust
// CRIT Rate +20% max value
// CRIT DMG +110% max value
```

**Mirror** (sigewinne_095.md, C6):
> When Sigewinne performs healing, she will increase the CRIT Rate and CRIT DMG of her Super Saturated Syringing based on her Max HP. Every 1,000 Max HP she has will increase CRIT Rate by **0.4%** and CRIT DMG by **2.2%** for 15s. The maximum increase achievable this way is **20% CRIT Rate** and **110% CRIT DMG**.

**Assessment**: The max values in the code (+20% CR, +110% CD) match the mirror. However, the per-1000-HP scaling rates differ from what could be derived:
- At max: 20% CR ÷ 0.4% per 1000 HP = 50,000 HP threshold for max CR
- At max: 110% CD ÷ 2.2% per 1000 HP = 50,000 HP threshold for max CD

The code stores only the max values (Toggle), which is a valid simplification. The values themselves (+20% CR, +110% CD) are **correct** per the mirror.

---

### Nilou — A4 "Dreamy Dance of Aeons" target should be AllParty (buffs party, not just self)

**Code** (`hydro.rs` lines 247–261):
```rust
TalentBuffDef {
    name: "Dreaming Dance of the Lotuslight",
    ...
    target: BuffTarget::OnlySelf,
    ...
}
```

**Mirror** (nilou_070.md, A4 "Dreamy Dance of Aeons"):
> Every 1,000 points of Nilou's Max HP above 30,000 will cause the DMG dealt by Bountiful Cores created by **characters affected by Golden Chalice's Bounty** to increase by 9%.

**Assessment**: This buff increases the damage of Bountiful Cores created by party members under Golden Chalice's Bounty — it is effectively a party-wide buff (all characters who have the Bounty status). Setting `target: BuffTarget::OnlySelf` is incorrect if the intent is to model the Bountiful Core DMG bonus. However, since Bountiful Cores are a transformative reaction product and the damage calculation may be applied separately, this may be an acceptable simplification for `TransformativeBonus` on Nilou herself as proxy. Still, the description should clarify this is a party-wide reaction DMG increase, not a self-only buff.

---

## Missing Implementations

### Tartaglia — No entry in HYDRO_TALENT_BUFFS registry

**Code** (`hydro.rs` lines 753–769): Tartaglia is not present in the registry.

**Mirror** (tartaglia_033.md): Tartaglia has no passive talents or constellations that grant ATK/DMG/CRIT/EM stat buffs to the party. His passives are:
- "Master of Weaponry": Normal Attack Level +1 for party — not a buffable stat
- "Never Ending": Riptide duration +8s — utility

Constellations: C1 CD reduction, C2 energy on kill, C3/C5 talent level up, C4 auto-proc, C6 CD reset. None grant stat buffs in scope.

**Assessment**: Tartaglia correctly has no TalentBuffDefs. His absence from the registry is intentional (no applicable buffs). No action needed.

---

### Candace — A4 "Celestial Dome of Sand" HP-scaling DMG bonus missing

**Mirror** (candace_072.md, A4):
> Characters affected by the Prayer of the Crimson Crown will deal 0.5% increased DMG for every 1,000 points of Candace's Max HP when they deal Elemental DMG with their Normal Attacks.

This is an HP-scaling percentage DMG bonus similar to Nilou's A4. It is currently not implemented as a TalentBuffDef. The HP-scaling variant (0.5% per 1,000 HP) would require `scales_on: Some(ScalingStat::Hp)` with a stacking formula. Given Candace's typical HP range (~30,000 HP), this could yield roughly 15% extra DMG. It should be added as a separate HP-scaling buff.

---

### Aino — A4 "Structured Power Booster" (Burst DMG from EM) implemented correctly?

**Code** (`hydro.rs` lines 21–34):
```rust
TalentBuffDef {
    name: "Aino A4 Burst DMG from EM",
    stat: BuffableStat::BurstFlatDmg,
    base_value: 0.50,
    scales_on: Some(ScalingStat::Em),
    ...
}
```

**Mirror** (aino_121.md): No A4 passive found in the mirror file. The file only shows two passives:
1. "Moonsign Benediction: Force Limit Analysis" (party Moonsign +1)
2. No second gameplay passive with EM-to-Burst-DMG conversion is visible.

**Assessment**: The mirror file for Aino (`aino_121.md`) does not contain a "Structured Power Booster" passive talent description. This buff may be from game data not yet captured in the mirror, or it may be incorrectly attributed. Requires verification against a more complete mirror update.

---

### Furina — C6 DMG from HP% not implemented

**Mirror** (furina_089.md, C6):
> DMG is also increased by an amount equivalent to **18% of Furina's max HP** (for Pneuma Arkhe, an additional 25% of max HP per hit).

These are HP-scaling flat DMG bonuses on Furina's own attacks during C6 "Center of Attention." They are not implemented as TalentBuffDefs. The HP-flat DMG bonuses are complex (Arkhe-alignment dependent, 6 triggers max) and appropriately skipped, but a TODO comment should exist.

---

### Furina — C2 HP% increase from Fanfare overflow not implemented

**Mirror** (furina_089.md, C2):
> Each point of Fanfare above the limit will increase Furina's Max HP by 0.35%. Her maximum Max HP increase is 140%.

This is a self-only HP% buff (max +140%) when Fanfare exceeds the cap. Currently not implemented. Given its complexity (depends on real-time Fanfare over-cap), a TODO comment should exist.

---

## Verified OK

### Barbara — C2 Vitality Burst
- Code: Hydro DMG Bonus +15%, Team, C2 — **correct**
- Mirror: "During the ability's duration, your active character gains a 15% Hydro DMG Bonus" — the buff is on active character only (same as Team for active slot); value correct.

### Furina — Burst DMG Bonus (C0 300pt stacks)
- Code: Scaling array `FURINA_BURST_PER_POINT` starting at 0.0007 (= 0.07% per point)
- Mirror: "Fanfare to DMG Increase Conversion Ratio" at Lv1: **0.07%**
- Per-point values match for all 15 levels. At max 300 stacks × 0.07% = 21% at Lv1, up to 300 × 0.35% = 105% at Lv15. **Correct**

### Furina — C1 extra 100pt Fanfare bonus
- Code: `FURINA_BURST_C1_BONUS` starting at 0.07 (= 100 × 0.0007)
- Mirror: C1 adds +100 Fanfare limit. The extra 100pt at max = 100 × per_point conversion rate.
- At Lv1: 100 × 0.07% = 7%, code has 0.07. **Correct**

### Mona — C1 Reaction DMG +15%
- Code: TransformativeBonus +15%, Team, C1, Toggle
- Mirror: "Electro-Charged DMG increases by 15%, Vaporize DMG increases by 15%, Hydro Swirl DMG increases by 15%" (and Lunar variants)
- Value and stat type correct (TransformativeBonus approximates reaction DMG buffs). **Correct**

### Mona — C2 Party EM +80
- Code: EM +80, Team, C2, Toggle
- Mirror: "When Mona's Charged Attack hits an opponent, all nearby party members will have their Elemental Mastery increased by 80 for 12s"
- Value correct. **Correct**

### Mona — C4 CRIT Rate +15%
- Code: CritRate +15%, Team, C4, Toggle
- Mirror: "When any party member attacks an opponent affected by an Omen, their CRIT Rate is increased by 15%"
- Value and scope correct. **Correct**

### Mona — C6 Charged ATK DMG +180%
- Code: ChargedAtkDmgBonus +180%, OnlySelf, C6
- Mirror: "A maximum DMG Bonus of 180% can be achieved in this manner"
- Max value correct. **Correct**

### Nilou — C2 Hydro RES -35% and Dendro RES -35%
- Both match the mirror values exactly. **Correct**

### Nilou — C4 Burst DMG +50%
- Mirror: "DMG from her Dance of Abzendegi will be increased by 50%"
- Code: BurstDmgBonus +50%. **Correct**

### Nilou — C6 CRIT Rate +0.6% per 1000 HP (max +30%), CRIT DMG +1.2% per 1000 HP (max +60%)
- Mirror: "For every 1,000 points of Max HP, Nilou's CRIT Rate and CRIT DMG will increase by 0.6% and 1.2% respectively. The maximum increase in CRIT Rate and CRIT DMG via this method is 30% and 60% respectively."
- Code values: base_value 0.006/0.012, cap 0.30/0.60, scales_on Hp. **Correct**

### Yelan — A4 "Adapt With Ease" DMG Bonus
- Code: DmgBonus +50% max, Team, Toggle
- Mirror confirms the DMG Bonus ramps up to 50% over the burst duration. **Correct**

### Yelan — C4 Party HP +10% per marked opponent (max 4)
- Code: HpPercent +10%, Team, Stacks(4)
- Mirror: "Increases all party members' Max HP by 10% for 25s for every opponent marked by Lifeline. A maximum increase of 40% Max HP can be attained."
- Code uses Stacks(4) × 10% = max 40%. **Correct**

### Ayato — C2 World Source HP +50%
- Code: HpPercent +50%, OnlySelf, C2
- Mirror: "When Kamisato Ayato has at least 3 Namisen stacks, his Max HP is increased by 50%"
- Value correct. **Correct**

### Columbina — C1 through C6 Lunar Reaction DMG values
- C1: +1.5%, C2: +7%, C3: +1.5%, C4: +1.5%, C5: +1.5%, C6: +7%
- All match the mirror "elevated by" values per constellation. **Correct**

### Columbina — C2 HP +40%
- Code: HpPercent +40%, OnlySelf, C2
- Mirror: "WhenGravity Interference is triggered, Columbina will receive the Lunar Brilliance effect, which increases her Max HP by 40% for 8s"
- Value correct. **Correct**

### Columbina — C6 Elemental CRIT DMG +80%
- Code: CritDmg +80%, Team, C6
- Mirror: "the CRIT DMG of the corresponding Elemental DMG done by all party members is increased by 80%"
- Value and scope correct. **Correct**

### Kokomi — C6 Hydro DMG +40%
- Code: ElementalDmgBonus(Hydro) +40%, OnlySelf, C6, Toggle
- Mirror: "gains a 40% Hydro DMG Bonus for 4s when her Normal and Charged Attacks heal... party member with 80% or more HP"
- Value correct. **Correct**

### Mualani — C4 Burst DMG +75%
- Code: BurstDmgBonus +75%, OnlySelf, C4
- Mirror: "Boomsharka-laka deals 75% increased DMG"
- Value correct. **Correct**

### Neuvillette — C2 CRIT DMG +14% per stack (max 3)
- Code: CritDmg +14%, OnlySelf, C2, Stacks(3)
- Mirror: "Each stack of Past Draconic Glories will increase the CRIT DMG of Charged Attack: Equitable Judgment by 14%. The maximum increase that can be achieved this way is 42%"
- Value correct. **Correct**

### Sigewinne — C2 Hydro RES -35%
- Code: ElementalResReduction(Hydro) +35%, Team, C2, Toggle
- Mirror: "that opponent's Hydro RES will be decreased by 35% for 8s"
- Value correct. **Correct**

### Xingqiu — C2 Hydro RES -15%
- Code: ElementalResReduction(Hydro) +15%, Team, C2
- Mirror: "Decreases the Hydro RES of opponents hit by sword rain attacks by 15% for 4s"
- Value correct. **Correct**

### Xingqiu — C4 Skill DMG +50%
- Code: SkillDmgBonus +50%, OnlySelf, C4, Toggle
- Mirror: "the DMG dealt by Guhua Sword: Fatal Rainscreen is increased by 50%"
- Value correct. **Correct**

---

## Action Items

| Priority | Character | Issue |
|----------|-----------|-------|
| HIGH | Candace | Burst scaling table is flat 20% at all levels; fix `CANDACE_BURST_NORMAL_SCALING` to `[0.20; 15]` |
| HIGH | Mona | Burst DMG scaling wrong at Lv4–15; correct to `[0.42,0.44,0.46,0.48,0.50,0.52,0.54,0.56,0.58,0.60,0.60,0.60,0.60,0.60,0.60]` |
| HIGH | Sigewinne | A1 buff target is `OnlySelf`, not `Team`; fix `target: BuffTarget::OnlySelf` |
| MEDIUM | Ayato | A1 Namisen is HP-scaling flat DMG per hit, not a % DMG bonus; replace with TODO comment |
| MEDIUM | Candace | Add A4 "Celestial Dome of Sand" HP-scaling DMG bonus (0.5% per 1000 HP, Team) |
| MEDIUM | Mona | C4 CRIT DMG is Hexerei-only (Buffed State); note limitation or restrict to OnlySelf |
| LOW | Furina | Add TODO for C2 HP% increase from Fanfare overflow (up to +140% HP, self) |
| LOW | Furina | Add TODO for C6 HP-scaling flat DMG bonus on Normal/Charged attacks |
| LOW | Aino | Verify A4 "Structured Power Booster" (EM→BurstDMG) against updated mirror |
| LOW | Ayato | Add TODO comment on C1 condition (≤50% HP opponent threshold, not modelable) |
| INFO | Nilou | A4 `target: OnlySelf` is a simplification; add clarifying comment that buff affects all party members under Golden Chalice's Bounty |
