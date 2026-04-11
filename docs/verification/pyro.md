# Pyro Character Verification

Verified against: `honeyhunter-mirror/md/characters/`
Scope: Talent scaling (all 15 levels), passive talents, constellation effects
Excluded: Proc damage, energy effects, HP regen, stamina costs, durations, cooldowns

## Summary

| Character | Scaling | Passives | Constellations | Status |
|-----------|---------|----------|----------------|--------|
| Amber | OK | OK | OK | PASS |
| Arlecchino | OK | OK | **BUG** | FAIL |
| Bennett | OK | OK | OK | PASS |
| Chevreuse | OK | OK | **BUG** | FAIL |
| Dehya | **BUG** | OK | OK | FAIL |
| Diluc | OK | OK | OK | PASS |
| Durin | OK | OK | OK | PASS |
| Gaming | OK | OK | OK | PASS |
| Hu Tao | OK | OK | OK | PASS |
| Klee | OK | OK | OK | PASS |
| Lyney | OK | OK | OK | PASS |
| Mavuika | **BUG** | OK | OK | FAIL |
| Thoma | **BUG** | OK | OK | FAIL |
| Xiangling | **BUG** | **DUPLICATE** | OK | FAIL |
| Xinyan | **BUG** | OK | OK | FAIL |
| Yanfei | OK | OK | OK | PASS |
| Yoimiya | OK | OK | OK | PASS |

**BUG count: 7** (Dehya dual scaling, Xiangling N3B, Thoma N3B, Mavuika N2B/N3 structure, Xinyan burst element, Arlecchino constellation, Chevreuse constellation)
**Duplicate buff count: 1** (Xiangling C1+C6 buffs)

---

## Bugs

### 1. Dehya: Burst missing HP scaling component -- ALL LEVELS

File: `crates/data/src/characters/pyro/dehya.rs`
Mirror: `honeyhunter-mirror/md/characters/dehya_079.md`

Dehya's Elemental Burst has dual ATK+HP scaling that is only partially implemented.
Only the ATK portion is present; the Max HP portion is missing.

| Talent | Mirror | Code |
|--------|--------|------|
| Flame-Mane's Fist DMG | 98.7% ATK + 1.69% Max HP | Only 0.9870 ATK |
| Incineration Drive DMG | 139.3% ATK + 2.39% Max HP | Only 1.3930 ATK |

The Skill's "Field DMG" also has dual scaling (60.2% ATK + 1.03% Max HP at Lv1) but only ATK is stored.

### 2. Xiangling: NORMAL_3B has wrong values -- ALL LEVELS

File: `crates/data/src/characters/pyro/xiangling.rs` line 43-52
Mirror: `honeyhunter-mirror/md/characters/xiangling_023.md`

NORMAL_3B "3段ダメージ (2)" has Lv1=0.1410 (14.10%) but mirror shows N3 = "26.06% + 26.06%".
Both parts of hit 3 should be 0.2606. The 0.1410 value is actually the per-hit value of N4 (14.1%x4 at Lv1).

| Level | Code | Mirror | Expected |
|-------|------|--------|----------|
| Lv1 | 0.1410 | 26.06% | 0.2606 |
| Lv2 | 0.1525 | 28.18% | 0.2818 |
| Lv3 | 0.1640 | 30.3% | 0.3030 |

### 3. Thoma: NORMAL_3B mislabeled with N4 values -- ALL LEVELS

File: `crates/data/src/characters/pyro/thoma.rs` line 43-52
Mirror: `honeyhunter-mirror/md/characters/thoma_050.md`

NORMAL_3B "3段ダメージ (2)" has Lv1=0.6736 (67.36%) but mirror shows N3 = "26.79% x 2".
Both parts of hit 3 should be 0.2679. The 0.6736 value matches N4 (67.36%).
The actual second part of hit 3 (another 26.79% hit) is missing, and N4 is duplicated.

| Level | Code | Mirror N3 (each hit) | Code matches |
|-------|------|---------------------|--------------|
| Lv1 | 0.6736 | 26.79% = 0.2679 | N4 (67.36%) |

### 4. Mavuika: Normal attack structure incorrect

File: `crates/data/src/characters/pyro/mavuika.rs`
Mirror: `honeyhunter-mirror/md/characters/mavuika_106.md`

Mirror normal attack structure:
- N1: 80.04% (single hit)
- N2: 36.48% x 2 (two identical hits)
- N3: 33.22% x 3 (three identical hits)
- N4: 116.19% (single hit)

Implementation hits array has 4 entries:
- NORMAL_1: 0.8004 (correct, N1)
- NORMAL_2A: 0.3648 (correct, N2 hit 1)
- NORMAL_2B: 0.3322 (WRONG -- should be 0.3648 for N2 hit 2, but has N3's per-hit value)
- NORMAL_3: 1.1619 (WRONG -- this is N4's value, N3's three hits are missing)

Missing entirely: N3 (33.22% x 3 = three separate scaling entries of 0.3322 each)
Also missing from skill: Flamestrider Sprint DMG, Flamestrider Charged (spinning + final), Flamestrider Plunge DMG

### 5. Xinyan: Burst main hit incorrectly marked as Pyro

File: `crates/data/src/characters/pyro/xinyan.rs` line 139-148
Mirror: `honeyhunter-mirror/md/characters/xinyan_044.md`

XINYAN_BURST has `damage_element: Some(Element::Pyro)` but the burst's main hit "Skill DMG" (スキルダメージ) deals **Physical DMG** according to the mirror description: "launches nearby opponents and deals Physical DMG to them".

The burst DoT (XINYAN_BURST_DOT) correctly uses `Some(Element::Pyro)`.

Fix: Change XINYAN_BURST's `damage_element` to `None`.

### 6. Arlecchino: Constellation pattern wrong

File: `crates/data/src/characters/pyro/arlecchino.rs` line 229
Mirror: `honeyhunter-mirror/md/characters/arlecchino_096.md`

Code has `C3BurstC5Skill`, but mirror shows:
- C3: "Increases the Level of Normal Attack: Invitation to a Beheading by 3" (Normal +3)
- C5: "Increases the Level of Balemoon Rising by 3" (Burst +3)

C5 boosts Burst, so the pattern should be `C3SkillC5Burst` (the C3 Skill boost is a side effect -- Normal Attack +3 is not tracked by the system, but at least C5 Burst will be correct).

Current code `C3BurstC5Skill` incorrectly gives C3=Burst+3 and C5=Skill+3.

### 7. Chevreuse: Constellation pattern inverted

File: `crates/data/src/characters/pyro/chevreuse.rs` line 229
Mirror: `honeyhunter-mirror/md/characters/chevreuse_090.md`

Code has `C3BurstC5Skill`, but mirror shows:
- C3: "Increases the Level of Short-Range Rapid Interdiction Fire by 3" (Skill +3)
- C5: "Increases the Level of Ring of Bursting Grenades by 3" (Burst +3)

Should be `C3SkillC5Burst`.

---

## Duplicate Buff Entries

### Xiangling: C1 and C6 buffs appear twice each

File: `crates/data/src/talent_buffs/pyro.rs`

C1 Pyro RES Shred appears in two forms:
1. "Crispy Outside Pyro RES Shred" with no activation condition
2. "xiangling_c1_pyro_res_shred" with Toggle activation

C6 Pyro DMG Bonus appears in two forms:
1. "Condensed Pyronado Pyro DMG Bonus" with no activation condition
2. "xiangling_c6_pyro_dmg" with Toggle activation

This may cause double-counting if both are active simultaneously.

---

## Lv95-100 Base Stat Note

Multiple characters show Lv100 base ATK values in the mirror that are significantly higher than what can be derived from Lv90 values by normal scaling. For example:
- Arlecchino: Mirror Lv100 ATK=418.98, Rust Lv100=369.39
- Mavuika: Mirror Lv100 ATK=439.49, Rust Lv100=387.47
- Chevreuse: Mirror Lv100 ATK=242.59, Rust Lv100=208.74

The mirror Lv100 ATK values appear to include the ascension stat bonus (for non-ATK ascension characters, this discrepancy may not appear). This is a systematic data interpretation difference in the Lv95-100 stat estimation, not individual per-character bugs.

---

## Character Notes

### Amber
Mirror: `ambor_021.md` (NOT ambor_008.md which is a test file with fake stats)
All scalings match. Base stats Lv1-90 match. C3SkillC5Burst correct.

### Arlecchino
Scalings match. N4 stored as single entry with "×2" notation matching mirror "37.15% + 37.15%".
Masque of the Red Death increase is stored in talent_buffs, not in scaling.
**Constellation pattern bug** (see above).

### Bennett
All scalings match. Burst ATK Bonus Ratio scaling in talent_buffs matches mirror values.

### Chevreuse
All scalings match. Skill has Press/Hold/Overcharged variants matching mirror.
Ascension stat HP% 24% correct.
**Constellation pattern bug** (see above).
Note: Surging Blade DMG (28.8% ATK at Lv1) from skill is not implemented as a separate TalentScaling entry.

### Dehya
Normal/Charged/Plunge scalings match. Skill Ranging Flame matches.
**Burst dual scaling missing** (see above).

### Diluc
All scalings match (minor rounding at sub-0.01% level on some values). C3SkillC5Burst correct.

### Durin
All scalings match. Skill has Purity + 3-part Dark correctly split.
Burst has both Purity (3-part) and Dark (3-part) variants plus dragon scalings. C3BurstC5Skill correct.

### Gaming
All scalings match.

### Hu Tao
All scalings match. Skill stores bonus portion (Blood Blossom DMG, Charged ATK DMG increase).
Lv100 base stats show systematic discrepancy with mirror (see Lv95-100 note above).

### Klee
All scalings match.

### Lyney
All scalings match. Charged has aimed/charge1/prop/pyrotechnic variants. C3SkillC5Burst correct.

### Mavuika
Skill Flamestrider NA 1-5 match mirror. Burst + Fighting Spirit dynamic bonus match.
**Normal attack structure bug** (see above).
Missing Flamestrider Sprint/Charged/Plunge scalings from skill.

### Thoma
**N3B bug** (see above). Other scalings match. Burst has main + fiery assistance correctly.

### Xiangling
**N3B bug** (see above). Other scalings match.
**Duplicate C1/C6 buffs** in talent_buffs (see above).

### Xinyan
**Burst element bug** (see above). Scaling values themselves are correct.
Skill has swing DMG + DoT. C3SkillC5Burst correct.

### Yanfei
All scalings match. 5 charged attack variants (0-4 seals) correct. C3SkillC5Burst correct.

### Yoimiya
All scalings match. Skill stores bonus portion (37.91% at Lv1 = mirror "137.91%" minus 100%).
This is intentional design -- the system adds the bonus on top of normal attack base damage.
