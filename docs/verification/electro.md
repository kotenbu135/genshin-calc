# Electro Characters Verification

## Fischl
### Scaling Ratios
- OK (all 15 levels verified for normal attacks, aimed shots, skill, and burst)

### Passive Talents
- OK (Witch's Eve Rite ATK +22.5% and EM +90 implemented as talent buffs)

### Constellation Effects
- **BUG: constellation_pattern is C3BurstC5Skill but should be C3SkillC5Burst**
  - Mirror: C3 = Nightrider (Skill) +3, C5 = Midnight Phantasmagoria (Burst) +3
  - Code: `ConstellationPattern::C3BurstC5Skill` (reversed)

### Missing Implementations
- Fischl C6 Hexerei enhancement (C6 Buffed State: after coordinated attack, Witch's Eve Rite buffs increased by 100% for 10s) -- not implemented as a separate buff


## Beidou
### Scaling Ratios
- **BUG: Normal attack Lv11-15 values are wrong for all hits**
  - Example N1: Lv11 mirror=151.96% code=150.55%, Lv12 mirror=165.33% code=160.49%, Lv13 mirror=178.71% code=170.43%, Lv14 mirror=192.08% code=180.37%, Lv15 mirror=206.67% code=190.31%
  - Same pattern affects N2, N3, N4, N5 at Lv11-15 (mirror values jump up relative to linear extrapolation)
  - This also affects charged spinning and charged final attacks at Lv11-15
- Skill and Burst scalings: OK

### Passive Talents
- Beidou A4 "Lightning Storm": Normal/Charged DMG +15% after max counter -- **not implemented as talent buff** (should be BuffableStat for NormalDmgBonus/ChargedDmgBonus)

### Constellation Effects
- **BUG: constellation_pattern is C3BurstC5Skill but should be C3SkillC5Burst**
  - Mirror: C3 = Tidecaller (Skill) +3, C5 = Stormbreaker (Burst) +3
  - Code: `ConstellationPattern::C3BurstC5Skill` (reversed)
- C6 Electro RES -15%: OK (implemented in talent_buffs)
- C4 Electro DMG +20%: OK (implemented in talent_buffs)

### Missing Implementations
- A4 "Lightning Storm" Normal/Charged ATK DMG +15% buff not implemented


## Lisa
### Scaling Ratios
- **BUG: Skill Hold 3-stack Lv8 value is wrong**
  - Mirror: 779.52% = 7.7952, Code: 7.9520
  - All other values verified OK (Lv1-7, Lv9-15 for 3-stack, and all levels for other scalings)

### Passive Talents
- A4 "Static Electricity Field" DEF -15%: OK (implemented in talent_buffs)

### Constellation Effects
- OK (C3 = Lightning Rose/Burst, C5 = Violet Arc/Skill matches C3BurstC5Skill)

### Missing Implementations
- None (C2 DEF+25% during hold is a defensive buff, Lisa has no stat-affecting passives besides DEF shred)


## Razor
### Scaling Ratios
- OK (all verified)

### Passive Talents
- Hexerei burst enhancement (Wolf Within DMG +70% ATK): OK (implemented as BurstFlatDmg)

### Constellation Effects
- OK (C3 = Lightning Fang/Burst, C5 = Claw and Thunder/Skill matches C3BurstC5Skill)
- C1 DMG +10%, C2 CR +10%, C6 CR +10% / CD +50%: All OK

### Missing Implementations
- C4 "Bite": DEF -15% on Skill press hit -- **not implemented as talent buff**


## Keqing
### Scaling Ratios
- OK (all verified including multi-hit N4 and charged attacks)

### Passive Talents
- A4 CR +15% and ER +15%: OK (implemented in talent_buffs)

### Constellation Effects
- OK (C3 = Starward Sword/Burst, C5 = Stellar Restoration/Skill matches C3BurstC5Skill)
- C4 ATK +25%: OK
- C6 Electro DMG +24%: OK (implemented as max value)

### Missing Implementations
- None


## Kujou Sara
### Scaling Ratios
- **BUG: ATK Bonus Ratio Lv15 value is wrong in talent_buffs**
  - Mirror: 102.03% = 1.0203, Code: 1.0206 (diff: +0.03%)

### Passive Talents
- Tengu Juurai ATK Bonus: OK (implemented as talent scaling)

### Constellation Effects
- OK (C3 = Burst, C5 = Skill matches C3BurstC5Skill)
- C6 "Sin of Pride" Electro CRIT DMG +60%: OK

### Missing Implementations
- None


## Raiden Shogun
### Scaling Ratios
- **BUG: Musou Isshin N4 uses identical values for both hits but mirror has different values**
  - Mirror Lv1: 30.89% + 30.98%, Code N4A=0.3089, N4B=0.3089 (should be 0.3098 for N4B)
  - Mirror Lv2: 32.99% + 33.09%, Code N4A=0.3299, N4B=0.3299 (should be 0.3309 for N4B)
  - This pattern continues for all 15 levels -- the second hit is consistently ~0.1% higher than the first
- Normal attack Lv11-15 values are wrong (same pattern as Beidou: mirror shows higher values than code)
  - Example N1: Lv11 mirror=84.71% code=84.71% -- actually these do match. Let me recheck.
  - Actually Raiden normal N1 Lv11: mirror=84.71%, code=0.8471. OK.
  - Raiden N1 Lv12: mirror=92.16%, code=0.9216. OK.
  - Raiden normals DO match even at Lv11-15. The Beidou pattern is specific to Beidou.

### Passive Talents / Talent Buffs
- **BUG: Skill "Eye of Stormy Judgment" Burst DMG Bonus scaling values are wrong**
  - Mirror: 0.22%, 0.23%, 0.24%, 0.25%, 0.26%, 0.27%, 0.28%, 0.29%, 0.3%, 0.3%, 0.3%, 0.3%, 0.3%, 0.3%, 0.3%
  - Code: 0.0022, 0.0024, 0.0026, 0.0028, 0.0030, 0.0032, 0.0035, 0.0038, 0.0041, 0.0044, 0.0047, 0.0050, 0.0053, 0.0056, 0.0059
  - Lv2: mirror 0.0023, code 0.0024 -- off by 0.0001
  - Lv3: mirror 0.0024, code 0.0026 -- off by 0.0002
  - Lv9-15: mirror caps at 0.003, code continues increasing to 0.0059
  - This is a significant bug affecting all levels except Lv1
- A4 "Enlightened One" Electro DMG +0.4% per 1% ER above 100%: Not implemented as talent buff (would require StatScaling on ER)
- C2 DEF ignore 60%: OK
- C4 ATK +30% to party (excluding self): OK

### Constellation Effects
- OK (C3 = Burst, C5 = Skill matches C3BurstC5Skill)

### Missing Implementations
- A4 "Enlightened One" Electro DMG Bonus from excess ER -- not implemented as talent buff


## Yae Miko
### Scaling Ratios
- **BUG: Sesshou Sakura Level 3 Lv15 value is wrong**
  - Mirror: 225.15% = 2.2515, Code: 2.2150 (diff: transposed digits, 2.2150 vs 2.2515)
- All other values verified OK

### Passive Talents
- A4 Skill DMG +0.15% per EM: OK (implemented in talent_buffs)

### Constellation Effects
- **BUG: constellation_pattern is C3BurstC5Skill but should be C3SkillC5Burst**
  - Mirror: C3 = Yakan Evocation: Sesshou Sakura (Skill) +3, C5 = Great Secret Art: Tenko Kenshin (Burst) +3
  - Code: `ConstellationPattern::C3BurstC5Skill` (reversed)
- C4 Electro DMG +20% to team: OK
- C6 DEF ignore 60%: OK

### Missing Implementations
- None


## Kuki Shinobu
### Scaling Ratios
- **BUG: Burst scaling_stat is ScalingStat::Atk but should be ScalingStat::Hp**
  - Mirror: "3.6% Max HP" per instance, Code: scaling_stat = ScalingStat::Atk
  - The burst damage scales off Max HP, not ATK
- All other values verified OK (numbers themselves are correct, just the stat reference is wrong)

### Passive Talents
- A1 Healing Bonus +15% when HP<=50%: OK
- A4 Skill flat DMG +25% of EM: OK

### Constellation Effects
- OK (C3 = Sanctifying Ring/Skill, C5 = Gyoei Narukami/Burst matches C3SkillC5Burst)
- C6 EM +150: OK

### Missing Implementations
- None


## Cyno
### Scaling Ratios
- **BUG: Normal Attack N3 Lv10 (index 9) value is wrong**
  - Mirror Lv10: 57.93% + 57.93% (combined: 115.86%), Code: 1.5883 (should be ~1.1586)
  - The value 1.5883 appears to be a typo/corruption
- Normal Attack N3 is a 2-hit attack (29.31% + 29.31%) but implemented as single TalentScaling
  - This is a design choice (combined value) but the combined Lv10 value is wrong

### Passive Talents
- A4 Normal flat DMG +150% EM: OK
- A4 Skill flat DMG +250% EM: OK

### Constellation Effects
- OK (C3 = Burst, C5 = Skill matches C3BurstC5Skill)
- C2 Electro DMG +10%/stack max 5: OK

### Missing Implementations
- A1 "Featherfall Judgment": Skill DMG +35% during Endseer stance -- not implemented as talent buff


## Dori
### Scaling Ratios
- Minor: Charged spinning Lv1 mirror=62.55%, code=0.6254 (diff 0.01%) -- negligible rounding
- All other values verified OK

### Passive Talents
- No damage-stat buffs from passives

### Constellation Effects
- OK (C3 = Burst, C5 = Skill matches C3BurstC5Skill)
- C4 Healing Bonus +50% / ER +30%: OK

### Missing Implementations
- None


## Sethos
### Scaling Ratios
- Not fully verified (spot-checked OK)

### Passive Talents
- A4 Charged ATK flat DMG +700% EM: OK

### Constellation Effects
- **BUG: constellation_pattern is C3BurstC5Skill but C3 boosts Normal Attack, not Burst**
  - Mirror: C3 = Normal Attack: Royal Reed Archery +3, C5 = Secret Rite: Twilight Shadowpiercer (Burst) +3
  - Code: `ConstellationPattern::C3BurstC5Skill`
  - Neither C3SkillC5Burst nor C3BurstC5Skill is correct since C3 boosts Normal Attack (unusual pattern)
- C1 CR +15%, C2 Electro DMG +15%/stack max 2, C4 Team EM +80: All OK

### Missing Implementations
- None (C3 boosting Normal Attack needs special handling not supported by current enum)


## Clorinde
### Scaling Ratios
- Not fully verified (spot-checked OK)

### Passive Talents
- A1 Normal/Burst flat DMG +20% ATK (cap 1800): OK
- A4 CR +10%: OK

### Constellation Effects
- OK (C3 = Hunter's Vigil/Skill, C5 = Last Lightfall/Burst matches C3SkillC5Burst)
- C2 Normal/Burst flat DMG +10% ATK (cap 900): OK (delta design on top of A1)
- C6 CR +10% / CD +70%: OK

### Missing Implementations
- None


## Ororon
### Scaling Ratios
- Not fully verified (spot-checked OK)

### Passive Talents
- No damage-stat buff passives implemented

### Constellation Effects
- OK (C3 = Dark Voices Echo/Burst, C5 = Night's Sling/Skill matches C3BurstC5Skill)
- C2 Electro DMG +32%: OK
- C6 Team ATK +10%/stack max 3: OK

### Missing Implementations
- None


## Iansan
### Scaling Ratios
- Not fully verified (spot-checked OK)

### Passive Talents
- A1 ATK +20%: OK
- Burst ATK flat bonus (talent scaling): OK

### Constellation Effects
- OK (C3 = Thunderbolt Rush/Skill, C5 = Three Principles of Power/Burst matches C3SkillC5Burst)
- C2 ATK +30% off-field: OK
- C6 DMG Bonus +25%: OK

### Missing Implementations
- None


## Varesa
### Scaling Ratios
- Not fully verified (spot-checked OK)

### Passive Talents
- A1 Plunging flat DMG +180% ATK: OK
- A4 ATK +35%/stack max 2: OK

### Constellation Effects
- **BUG: constellation_pattern is C3SkillC5Burst but C3 boosts Burst, not Skill**
  - Mirror: C3 = Guardian Vent! (Burst) +3, C5 = Normal Attack: By the Horns +3
  - Code: `ConstellationPattern::C3SkillC5Burst`
  - Should be C3BurstC5Skill (though C5 boosts Normal Attack, similar to Sethos)
- C6 CR +10% / CD +100%: OK

### Missing Implementations
- None


## Ineffa
### Scaling Ratios
- Not fully verified (spot-checked OK)

### Passive Talents
- A4 EM share (6% of ATK): OK
- C1 Lunar-Charged DMG +2.5% per 100 ATK (max 50%): OK

### Constellation Effects
- **BUG: constellation_pattern is C3BurstC5Skill but should be C3SkillC5Burst**
  - Mirror: C3 = Cleaning Mode: Carrier Frequency (Skill) +3, C5 = Supreme Instruction: Cyclonic Exterminator (Burst) +3
  - Code: `ConstellationPattern::C3BurstC5Skill` (reversed)

### Missing Implementations
- None


## Flins
### Scaling Ratios
- Not fully verified (spot-checked OK)

### Passive Talents
- A4 EM from ATK 8% (cap 160): OK
- C4 ATK +20% + A4 enhancement (+2%, cap +60): OK
- C2 Electro RES -25%: OK
- C6 Lunar-Charged DMG (self +35%, team +10%): OK

### Constellation Effects
- OK (C3 = Ancient Ritual: Cometh the Night/Burst, C5 = Ancient Rite: Arcane Light/Skill matches C3BurstC5Skill)

### Missing Implementations
- None


---

## Summary of Bugs Found

### Critical Bugs (Wrong Constellation Patterns)
1. **Fischl**: `C3BurstC5Skill` should be `C3SkillC5Burst`
2. **Beidou**: `C3BurstC5Skill` should be `C3SkillC5Burst`
3. **Yae Miko**: `C3BurstC5Skill` should be `C3SkillC5Burst`
4. **Ineffa**: `C3BurstC5Skill` should be `C3SkillC5Burst`
5. **Varesa**: `C3SkillC5Burst` should be `C3BurstC5Skill`
6. **Sethos**: `C3BurstC5Skill` -- C3 actually boosts Normal Attack (not Skill/Burst), needs special handling

### Scaling Value Bugs
7. **Lisa**: Skill Hold 3-stack Lv8: code 7.9520, should be 7.7952
8. **Raiden Shogun**: Musou Isshin N4B values identical to N4A but should differ (e.g., Lv1: 0.3098 not 0.3089)
9. **Raiden Shogun**: Skill Burst DMG Bonus scaling (talent_buffs): all 15 values wrong except Lv1
10. **Yae Miko**: Sesshou Sakura Level 3 Lv15: code 2.2150, should be 2.2515
11. **Kuki Shinobu**: Burst scaling_stat is ScalingStat::Atk but should be ScalingStat::Hp
12. **Cyno**: Normal N3 Lv10 (index 9): code 1.5883, should be ~1.1586
13. **Beidou**: Normal attack Lv11-15 values diverge from mirror for all hits
14. **Kujou Sara**: ATK Bonus Ratio Lv15 in talent_buffs: code 1.0206, mirror 1.0203 (minor)

### Missing Buff Implementations
15. **Beidou A4** "Lightning Storm": Normal/Charged ATK DMG +15% not implemented
16. **Razor C4** "Bite": DEF -15% on skill press not implemented
17. **Raiden A4** "Enlightened One": Electro DMG +0.4% per 1% excess ER not implemented
18. **Cyno A1** "Featherfall Judgment": Skill DMG +35% during Endseer stance not implemented
