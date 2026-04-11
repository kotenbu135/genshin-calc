# Hydro Character Verification

Verified against: `honeyhunter-mirror/md/characters/`
Scope: Talent scaling (all 15 levels), passive talents, constellation effects
Excluded: Proc damage, energy effects, HP regen, stamina costs, durations, cooldowns

## Summary

| Character | Scaling | Passives | Constellations | Status |
|-----------|---------|----------|----------------|--------|
| Aino | OK | OK | OK | PASS |
| Ayato | **BUG** | OK | OK | FAIL |
| Barbara | OK | OK | OK | PASS |
| Candace | **BUG** | **MISSING** | OK | FAIL |
| Columbina | OK | OK | OK | PASS |
| Dahlia | OK | OK | OK | PASS |
| Furina | OK | OK | OK | PASS |
| Kokomi | OK | OK | OK | PASS |
| Mona | OK | OK | OK | PASS |
| Mualani | ROUNDING | OK | OK | PASS (minor) |
| Neuvillette | OK | OK | OK | PASS |
| Nilou | OK | OK | OK | PASS |
| Sigewinne | ROUNDING | OK | OK | PASS (minor) |
| Tartaglia | ROUNDING | N/A | N/A | PASS (minor) |
| Xingqiu | OK | **MISSING** | OK | FAIL |
| Yelan | OK | OK | OK | PASS |

**BUG count: 2** (Candace SKILL_PRESS, Ayato SKILL_3HIT Lv8)
**Missing buff count: 2** (Xingqiu A4, Candace A4)

---

## Bugs

### Candace: CANDACE_SKILL_PRESS (一段チャージダメージ) -- ALL LEVELS WRONG

File: `crates/data/src/characters/hydro/candace.rs` line 111-120

All 15 values are hardcoded to `0.1200`. Should scale from 0.1200 (Lv1) to 0.2850 (Lv15).

Mirror source: `honeyhunter-mirror/md/characters/candace_072.md` line 58, "Basic DMG / 基礎ダメージ"

| Level | Code | Mirror | Delta |
|-------|------|--------|-------|
| Lv1 | 0.1200 | 12% = 0.1200 | 0 |
| Lv2 | 0.1200 | 12.9% = 0.1290 | **-0.0090** |
| Lv3 | 0.1200 | 13.8% = 0.1380 | **-0.0180** |
| Lv4 | 0.1200 | 15% = 0.1500 | **-0.0300** |
| Lv5 | 0.1200 | 15.9% = 0.1590 | **-0.0390** |
| Lv6 | 0.1200 | 16.8% = 0.1680 | **-0.0480** |
| Lv7 | 0.1200 | 18% = 0.1800 | **-0.0600** |
| Lv8 | 0.1200 | 19.2% = 0.1920 | **-0.0720** |
| Lv9 | 0.1200 | 20.4% = 0.2040 | **-0.0840** |
| Lv10 | 0.1200 | 21.6% = 0.2160 | **-0.0960** |
| Lv11 | 0.1200 | 22.8% = 0.2280 | **-0.1080** |
| Lv12 | 0.1200 | 24% = 0.2400 | **-0.1200** |
| Lv13 | 0.1200 | 25.5% = 0.2550 | **-0.1350** |
| Lv14 | 0.1200 | 27% = 0.2700 | **-0.1500** |
| Lv15 | 0.1200 | 28.5% = 0.2850 | **-0.1650** |

Expected fix:
```rust
values: [
    0.1200, 0.1290, 0.1380, 0.1500, 0.1590, 0.1680, 0.1800, 0.1920, 0.2040, 0.2160, 0.2280,
    0.2400, 0.2550, 0.2700, 0.2850,
],
```

### Ayato: AYATO_SKILL_3HIT (瞬水剣3段ダメージ) Lv8 -- TRANSPOSED DIGITS

File: `crates/data/src/characters/hydro/ayato.rs` line 138

Mirror source: `honeyhunter-mirror/md/characters/ayato_066.md` line 60

| Level | Code | Mirror | Delta |
|-------|------|--------|-------|
| Lv8 | 1.0999 | 110.99% = 1.1099 | **-0.0100** |

All other levels match. The Lv8 value has transposed digits: `10999` should be `11099`.

---

## Missing Talent Buffs

### Xingqiu A4: Blades Amidst Raindrops / 虚実の筆

File: `crates/data/src/talent_buffs/hydro.rs` (XINGQIU_BUFFS, line 790)

Mirror source: `honeyhunter-mirror/md/characters/xingqiu_025.md` line 90-92
> "Xingqiu gains a 20% Hydro DMG Bonus."

This is an unconditional A4 passive: Hydro DMG Bonus +20%. It is not present in `XINGQIU_BUFFS` (which only has C2 Hydro RES -15% and C4 Skill DMG +50%).

Expected implementation:
```rust
TalentBuffDef {
    name: "Blades Amidst Raindrops",
    description: desc!("A4: Hydro DMG Bonus +20%"),
    stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
    base_value: 0.20,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::AscensionPassive(4),
    min_constellation: 0,
    cap: None,
    activation: None,
},
```

### Candace A4: Celestial Dome of Sand / 砂の円蓋

File: `crates/data/src/talent_buffs/hydro.rs` line 60 (comment only, no implementation)

Mirror source: `honeyhunter-mirror/md/characters/candace_072.md` line 95-97
> "Characters affected by the Prayer of the Crimson Crown [...] will deal 0.5% increased DMG to opponents for every 1,000 points of Candace's Max HP when they deal Elemental DMG with their Normal Attacks."

This A4 passive is commented in the code (line 60) but not implemented as a TalentBuffDef.

---

## Rounding Discrepancies (informational, not bugs)

### Sigewinne: Normal Attack and Burst minor precision differences

File: `crates/data/src/characters/hydro/sigewinne.rs`
Mirror: `honeyhunter-mirror/md/characters/sigewinne_095.md`

| Scaling | Level | Code | Mirror | Delta |
|---------|-------|------|--------|-------|
| N1 (1段ダメージ) | Lv1 | 0.5260 | 52.61% = 0.5261 | -0.0001 |
| N1 (1段ダメージ) | Lv2 | 0.5700 | 56.9% = 0.5690 | +0.0010 |
| N2 (2段ダメージ) | Lv1 | 0.5110 | 51.07% = 0.5107 | +0.0003 |
| Burst (スキルダメージ) | Lv1 | 0.1180 | 11.77% = 0.1177 | +0.0003 |
| Burst (スキルダメージ) | Lv2 | 0.1270 | 12.65% = 0.1265 | +0.0005 |
| Burst (スキルダメージ) | Lv15 | 0.2800 | 27.96% = 0.2796 | +0.0004 |

### Mualani: Normal Attack minor precision differences

File: `crates/data/src/characters/hydro/mualani.rs`
Mirror: `honeyhunter-mirror/md/characters/mualani_102.md`

| Scaling | Level | Code | Mirror | Delta |
|---------|-------|------|--------|-------|
| N1 (1段ダメージ) | Lv2 | 0.5530 | 55.25% = 0.5525 | +0.0005 |
| N3 (3段ダメージ) | Lv1 | 0.7000 | 70.03% = 0.7003 | -0.0003 |
| N3 (3段ダメージ) | Lv3 | 0.8050 | 80.54% = 0.8054 | -0.0004 |
| N3 (3段ダメージ) | Lv6 | 0.9800 | 98.05% = 0.9805 | -0.0005 |

### Tartaglia: Plunge minor precision difference

File: `crates/data/src/characters/hydro/tartaglia.rs`
Mirror: `honeyhunter-mirror/md/characters/tartaglia_033.md`

| Scaling | Level | Code | Mirror | Delta |
|---------|-------|------|--------|-------|
| Plunge (落下期間のダメージ) | Lv4 | 0.8178 | 81.77% = 0.8177 | +0.0001 |

---

## Character Detail

### Aino
- Scalings: All 15 levels verified, all match
- Passives: C1 EM Share, A4 Burst DMG from EM -- implemented
- Constellations: C1 -- implemented

### Ayato
- Scalings: **BUG in SKILL_3HIT Lv8** (see above). All other scalings match.
- Passives: A1 Namisen initial stacks (not a stat buff), A4 energy regen (excluded)
- Constellations: C1/C2 Namisen effects -- implemented in talent_buffs
- Constellation pattern: C3BurstC5Skill -- matches mirror (C3=Burst+3, C5=Skill+3)

### Barbara
- Scalings: All 15 levels verified, all match
- Passives: A1/A4 are healing/utility (excluded)
- Constellations: C2 Hydro DMG +15% -- implemented

### Candace
- Scalings: **BUG in SKILL_PRESS** (see above). All other scalings match.
- Passives: **A4 "Celestial Dome of Sand" MISSING** (see above). Burst Normal DMG +20% -- implemented.
- Constellations: C2 HP +20% -- implemented
- Constellation pattern: C3BurstC5Skill -- matches mirror

### Columbina
- Scalings: All 15 levels verified, all match
- Passives/Constellations: Implemented in talent_buffs

### Dahlia
- Scalings: All 15 levels verified, all match
- Passives/Constellations: C2 implemented in talent_buffs

### Furina
- Scalings: All 15 levels verified, all match
- Passives: Burst fanfare DMG bonus -- implemented with per-point scaling
- Constellation pattern: C3SkillC5Burst -- matches mirror

### Kokomi
- Scalings: All 15 levels verified (burst bonus scalings have higher precision than mirror shows -- code uses 5 decimal places, mirror truncates to 2 decimal places). No meaningful discrepancies.
- Passives: A1 is CritRate -100% (innate, not a buff), A4 healing bonus (excluded)
- Constellations: C6 Hydro DMG Bonus -- implemented

### Mona
- Scalings: All 15 levels verified, all match
- Passives: A4 "Waterborne Destiny" Hydro DMG from ER -- implemented (Omen DMG bonus also implemented)
- Constellations: C1/C4 implemented in talent_buffs

### Mualani
- Scalings: Minor rounding discrepancies (see above). No bugs.
- Passives: A1 generates Puffers (proc, excluded), A4 Burst DMG bonus from Nightsoul Burst stacks
- Constellations: C4 implemented in talent_buffs

### Neuvillette
- Scalings: All 15 levels verified, all match
- Passives: A1 HP recovery/Droplet (excluded), A4 Hydro DMG from current HP -- implemented
- Constellations: C1/C2/C6 effects -- implemented in talent_buffs

### Nilou
- Scalings: All 15 levels verified, all match
- Passives: A2 Bloom bonus, C2 Hydro RES shred, C6 Crit Rate -- implemented
- Constellation pattern: C3BurstC5Skill -- matches mirror

### Sigewinne
- Scalings: Minor rounding discrepancies (see above). No bugs.
- Passives: A1 Hydro DMG +8% -- implemented. A4 healing bonus (excluded).
- Constellations: C2 Hydro RES -35%, C6 -- implemented
- Constellation pattern: C3SkillC5Burst -- matches mirror

### Tartaglia
- Scalings: Minor rounding discrepancy in Plunge Lv4 (see above). No bugs.
- Passives: A1 "Master of Weaponry" Normal ATK Level +1 (not representable in current buff system -- talent level modification, not a stat buff). A4 "Sword of Torrents" Riptide on CRIT (mechanic, not stat buff).
- Constellations: C1 CD reduction, C2 energy, C4 proc, C6 CD reset -- all non-stat-buff effects (correctly not in talent_buffs)
- Constellation pattern: C3SkillC5Burst -- matches mirror

### Xingqiu
- Scalings: All 15 levels verified, all match
- Passives: **A4 "Blades Amidst Raindrops" Hydro DMG +20% MISSING** (see above)
- Constellations: C2 Hydro RES -15%, C4 Skill DMG +50% -- implemented
- Constellation pattern: C3BurstC5Skill -- matches mirror

### Yelan
- Scalings: All 15 levels verified, all match
- Passives: A4 DMG bonus ramp -- implemented
- Constellations: C4 HP bonus -- implemented
