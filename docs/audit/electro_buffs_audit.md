# Electro Talent Buffs Audit

Audit date: 2026-04-10
Sources checked:
- Implementation: `crates/data/src/talent_buffs/electro.rs`
- Mirror: `honeyhunter-mirror/md/characters/*.md`

Scope:
- Damage-relevant talent buffs only.
- Extra damage / proc damage is excluded per user request.
- Defensive, healing-only, energy-only, stamina, cooldown, and pure utility effects are excluded unless they also change damage.

---

## Summary

| Character | Status | Notes |
|-----------|--------|-------|
| Beidou | ISSUES | A4 damage buff missing; C4 is modeled as a generic Electro DMG bonus even though the mirror effect is an extra hit and out of scope |
| Clorinde | ISSUES | A1 only covers Normal Attack side; C2 and C4 damage buffs are missing |
| Cyno | OK | A4/C2 match; C6 extra bolts are out of scope |
| Dori | OK | Only healing/energy effects are present, so they are excluded |
| Fischl | MISSING | Hexerei ATK/EM damage buff from Witch's Eve Rite is not implemented |
| Flins | OK | A4/C2/C6 match; manual toggles are an acceptable approximation for state-gated effects |
| Iansan | ISSUES | C2 and C6 target too broadly; mirror applies them to the current active character only |
| Ineffa | OK | A4 and C1 match |
| Keqing | OK | C6 is a max-value approximation, but the ceiling is correct |
| Kujou Sara | ISSUES | C6 is too broad: generic CritDmg on Team instead of Electro-only crit damage on the buffed character |
| Kuki Shinobu | ISSUES | C6 EM buff is unconditional in code; mirror requires low HP and an internal cooldown |
| Lisa | OK | A4 DEF shred matches; C6 utility is excluded |
| Ororon | ISSUES | C2 value is too high; C6 target is too broad for a current-active-character buff |
| Raiden Shogun | OK | A4/C2/C4 match |
| Razor | ISSUES | Hexerei burst enhancement and C6 crit buffs are missing; the proc hit stays out of scope |
| Sethos | OK | A4/C1/C2/C4 match |
| Varesa | ISSUES | A1 is ceiling-only and C1/C4 damage buffs are missing |
| Yae Miko | OK | A4/C4/C6 match |

No duplicate talent-buff entries were found in `electro.rs`, so there is no obvious double-count risk from duplicated definitions.

---

## Value / Type Discrepancies

### Beidou
- `beidou_c4_electro_dmg` is not a safe stand-in for the mirror C4. The mirror effect is an extra Electro hit on Normal Attacks after Beidou is hit, which is out of scope here. Keeping it as `ElementalDmgBonus(Electro)` would over-buff every Electro hit instead of only the conditional follow-up instance.
- `Lightning Storm` is missing entirely. The mirror A4 grants Normal and Charged Attack DMG +15% for 10s after Beidou unleashes Tidecaller at maximum bonus.

### Clorinde
- `Clorinde A1 Electro Flat DMG` only covers Normal Attacks. The mirror passive also affects `Last Lightfall`, so the Burst half is missing from the current stat model.
- `Dark-Shattering Flame` is not upgraded by C2. The mirror C2 raises the ATK scaling to 30% and keeps the same max cap behavior.
- `To Enshrine Tears, Life, and Love` is missing. The mirror C4 adds Bond of Life based `Last Lightfall` DMG scaling.

### Kujou Sara
- `Sin of Pride` is too broad. The mirror gives Electro-only CRIT DMG to the character buffed by Tengu Juurai, but the code uses generic `CritDmg` on the whole team.

### Kuki Shinobu
- `Kuki Shinobu C6 Elemental Mastery` is unconditional in code. The mirror only grants EM +150 when Shinobu drops below 25% HP, and that effect has a 60s cooldown.

### Iansan
- `iansan_c2_atk` targets `Team`, but the mirror C2 applies ATK +30% only to the current active character while Iansan is off-field.
- `iansan_c6_dmg_bonus` also targets `Team`, but the mirror C6 applies DMG +25% only to the current active character after overflow.

### Ororon
- `Ororon C2 Electro DMG Bonus` uses 40%, but the mirror caps the bonus at 32%.
- `Ororon C6 Team ATK Bonus` is targeted too broadly. The mirror grants ATK to the current active character only, not the whole team.

### Varesa
- `Varesa A1 Plunging Atk Flat DMG` is modeled at the ceiling only. The mirror A1 is state-dependent, and the C1 upgrade that makes the ceiling unconditional is not represented separately.

---

## Missing Implementations

### Fischl
- `Witch's Eve Rite: Phantasmal Nocturne` is missing. The mirror adds ATK +22.5% after Overloaded and EM +90 after Electro-Charged or Lunar-Charged while Oz is on field.
- The C6 buffed-state amplification is also missing. In the mirror, Oz coordinated attacks further increase those ATK and EM bonuses.
- I excluded Fischl's coordinated attack hits themselves because they are proc damage.

### Beidou
- A4 `Lightning Storm` is missing. The mirror grants Normal and Charged Attack DMG +15% for 10s after a max Tidecaller counter.

### Clorinde
- C2 `Dark-Shattering Flame` enhancement is missing. The mirror raises the ATK scaling from 20% to 30% per stack and keeps the same stack cap / max-damage ceiling.
- C4 `To Enshrine Tears, Life, and Love` is missing. The mirror adds `Last Lightfall` DMG scaling based on Bond of Life.

### Razor
- The Hexerei burst enhancement is missing. The mirror adds a burst DMG bonus equal to 70% of Razor's ATK when `Lightning Fang` is active.
- The C6 crit buff after Electro Sigil consumption is also missing. The lightning strike proc itself stays out of scope.

### Varesa
- C1 `Undying Passion` is not represented separately. The current A1 ceiling-only modeling does not capture the base 50% state or the mirror's C1 special-plunge upgrade cleanly.
- C4 `The Courage to Press On` is missing. The mirror grants either plunge ground-impact DMG based on 500% ATK up to 20,000 damage, or a 100% Burst DMG increase in Fiery Passion / Apex Drive.

---

## Acceptable Approximations

- `Keqing C6 Electro DMG Bonus` is modeled as a flat 24% ceiling instead of three separate 6% stacks. That is a reasonable max-value approximation if the calculator only needs the ceiling.
- `Flins` uses manual toggles for several Moonsign-gated effects. That is acceptable as long as the UI expects user-controlled state activation.

---

## Verified OK

- **Cyno**: A4 flat DMG scaling and C2 Electro DMG stacks match the mirror.
- **Dori**: No damage-relevant buffs beyond the excluded healing / energy effects.
- **Ineffa**: A4 EM share and C1 Lunar-Charged DMG bonus match.
- **Keqing**: A4 CRIT Rate / ER and C4 ATK bonus match.
- **Lisa**: A4 DEF shred matches; C6 is utility-only and excluded.
- **Raiden Shogun**: Burst DMG bonus scaling, C2 DEF ignore, and C4 ATK bonus match.
- **Sethos**: A4 Charged Attack scaling, C1 CRIT Rate, C2 Electro DMG stacks, and C4 EM match.
- **Yae Miko**: A4 Skill DMG scaling, C4 Electro DMG bonus, and C6 DEF ignore match.
- **Flins**: A4 EM scaling, C2 Electro RES shred, and C6 Lunar-Charged DMG bonuses match.

---

## Uncertainties

- Several newer effects use state gating or active-character-only targeting that the current model cannot represent exactly. I treated those as issues when the implementation clearly over-buffs the whole team, and as acceptable approximations only when the code was explicitly using a max-value ceiling.
- Fischl and Razor both have new Hexerei damage buffs in the mirror. If the project intentionally defers Witch's Homework content, that would explain the omissions, but the mirror data still shows those buffs as present and damage-relevant.
