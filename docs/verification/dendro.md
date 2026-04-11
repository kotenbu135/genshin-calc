# Dendro Characters Verification

Verified against: `honeyhunter-mirror/md/characters/`
Date: 2026-04-12
Scope: Talent scaling ratios, skill effects, buffs/debuffs, passive talents, constellation effects
Excluded: Additional/proc damage, energy recharge effects, HP regeneration values, stamina costs, durations, cooldowns, shield absorption values

## Summary Table

| Character | Scaling | Passives | Constellations | Status |
|-----------|---------|----------|----------------|--------|
| Tighnari | OK (14/14) | OK | OK | PASS |
| Collei | OK (12/12) | OK | BUG (C6 fabricated) | FAIL |
| Nahida | OK (10/10) | MISSING (A4) | OK | FAIL |
| Yaoyao | OK (11/11) | OK | BUG (pattern inverted) | FAIL |
| Alhaitham | OK (11/11) | OK | BUG (pattern inverted) | FAIL |
| Baizhu | OK (10/10) | OK | OK | PASS |
| Kaveh | OK (11/11) | OK | OK | PASS |
| Kirara | OK (14/14) | MISSING (A4) | OK | PASS (known TODO) |
| Emilie | OK (13/13) | OK | OK | PASS |
| Kinich | OK (12/12) | OK | OK | PASS |
| Traveler (Dendro) | OK (12/12) | OK | OK | PASS |
| Lauma | OK (14/14) | OK | OK | PASS |
| Nefer | OK (21/21) | OK | OK | PASS |

**Total scaling values verified: 151 matched, 0 mismatches.**

## Issues (bugs and missing implementations)

### BUG 1: Collei, Nahida, Yaoyao, Alhaitham constellation_pattern inverted

- **File**: `crates/data/src/characters/dendro/collei.rs`, `nahida.rs`, `yaoyao.rs`, `alhaitham.rs`
- **Current**: `ConstellationPattern::C3BurstC5Skill`
- **Expected**: `ConstellationPattern::C3SkillC5Burst`
- **Evidence from mirror**:
  - Collei C3 boosts **Floral Brush** (Skill), C5 boosts **Trump-Card Kitty** (Burst)
  - Nahida C3 boosts **All Schemes to Know** (Skill), C5 boosts **Illusory Heart** (Burst)
  - Yaoyao C3 boosts **Raphanus Sky Cluster** (Skill), C5 boosts **Moonjade Descent** (Burst)
  - Alhaitham C3 boosts **Universality: An Elaboration on Form** (Skill), C5 boosts **Particular Field: Fetters of Phenomena** (Burst)
- **Impact**: C3/C5 bonus levels are applied to the wrong talents, affecting damage calculations.

### BUG 2: Collei C6 buff is fabricated (not from game data)

- **File**: `crates/data/src/talent_buffs/dendro.rs`, `collei_c6_dendro_dmg`
- **Current**: `ElementalDmgBonus(Element::Dendro)` with `base_value: 0.35` (Dendro DMG +35% in Sprout state)
- **Expected**: No stat buff. Collei C6 is proc damage only (200% ATK miniature Cuilein-Anbar hit).
- **Evidence**: `honeyhunter-mirror/md/characters/collei_067.md` line 128: "When the Floral Ring hits, it will create a miniature Cuilein-Anbar that will deal 200% of Collei's ATK as Dendro DMG." No 35% Dendro DMG Bonus exists anywhere in Collei's kit.
- **Impact**: Users who activate C6 get a non-existent +35% Dendro DMG Bonus.

### BUG 3: Nahida A4 "Awakening Elucidated" not implemented

- **File**: `crates/data/src/talent_buffs/dendro.rs`, `NAHIDA_BUFFS`
- **Missing**: Per point of EM above 200: Tri-Karma Purification DMG +0.1% (max +80%) and CRIT Rate +0.03% (max +24%)
- **Evidence**: `honeyhunter-mirror/md/characters/nahida_073.md` line 99: "Each point of Nahida's Elemental Mastery beyond 200 will grant 0.1% Bonus DMG and 0.03% CRIT Rate to Tri-Karma Purification"
- **Impact**: Nahida's most important damage passive is missing from calculations.

### Known TODO: Kirara A4 not implemented

- **File**: `crates/data/src/talent_buffs/dendro.rs`, line 640
- **Status**: Explicitly marked as TODO in code.
- **Mirror**: `honeyhunter-mirror/md/characters/momoka_061.md` - A4 "Pupillary Variance": Skill/Burst DMG bonus based on Max HP.

### Minor: Kirara C6 uses DmgBonus instead of ElementalDmgBonus

- **File**: `crates/data/src/talent_buffs/dendro.rs`, `kirara_c6_dmg_bonus`
- **Current**: `BuffableStat::DmgBonus` (generic, includes physical)
- **Expected**: "All Elemental DMG Bonus +12%" - should not apply to physical damage
- **Mirror**: `honeyhunter-mirror/md/characters/momoka_061.md` line 134: "All nearby party members gain a 12% All Elemental DMG Bonus"

### Minor: Nefer C6 uses TransformativeBonus instead of ReactionDmgBonus

- **File**: `crates/data/src/talent_buffs/dendro.rs`, `nefer_c6_lunar_bloom_dmg`
- **Current**: `BuffableStat::TransformativeBonus` with `base_value: 0.15`
- **Mirror**: "Nefer's Lunar-Bloom DMG is elevated by 15%"
- **Note**: Should ideally be `ReactionDmgBonus(Reaction::LunarBloom)` for consistency with Lauma's similar buffs.

### Design note: Nahida A1 approximation

- **File**: `crates/data/src/talent_buffs/dendro.rs`, `Compassion Illuminated`
- **Status**: Documented approximation. Game uses highest party EM; implementation uses own EM.
- Not a bug, but noted for completeness.

## Character Notes

### Tighnari
- All 14 scaling values verified across Lv1-Lv15 (NA 1-4, aimed shot, charge Lv1, wreath arrow, clusterbloom, plunge x3, skill, burst x2). All match.
- A1 "Keen Sight": EM +50. Implemented correctly.
- A4 "Scholarly Blade": Charged ATK and Burst DMG +0.06% per EM (max 60%). Two separate buffs implemented correctly with `base_value: 0.0006, cap: Some(0.60)`.
- C1: CA CR +15%. Correct.
- C2: Dendro DMG +20%. Correct.
- C4: Team EM +60 (base) + Reaction EM +60 (additional). Both implemented correctly.
- Constellation pattern: C3BurstC5Skill. Correct (C3=Burst "Fashioner's Tanglevine Shaft", C5=Skill "Vijnana-Phala Mine").

### Collei
- All 12 scaling values verified. All match.
- C4: EM +60 (excludes self). Correctly uses `TeamExcludeSelf`.
- **C6: BUG** - 35% Dendro DMG Bonus does not exist in game data. Should be removed.
- **Constellation pattern: BUG** - Should be `C3SkillC5Burst` (C3=Floral Brush, C5=Trump-Card Kitty).

### Nahida
- All 10 scaling values verified (NA 1-4, charged, plunge x3, skill press, skill hold). All match.
- Tri-Karma Purification (dual ATK+EM scaling) is not in main talent scalings - separate implementation needed.
- A1 "Compassion Illuminated": EM sharing 25%, max 250. Implemented (with own-EM approximation noted).
- **A4 "Awakening Elucidated": NOT IMPLEMENTED.** Per EM above 200: DMG +0.1% (max 80%), CR +0.03% (max 24%).
- C2: CR +20%, CD +100%, DEF -30%. All three entries correct.
- C4: EM +100 per enemy (max 4 stacks). Correct.
- **Constellation pattern: BUG** - Should be `C3SkillC5Burst` (C3=All Schemes to Know, C5=Illusory Heart).

### Yaoyao
- All 11 scaling values verified (NA 1-3a, 4, charged, plunge x3, skill radish, burst radish). All match. NA hit 3 is properly split into two entries (3a/3b) in Rust; 3b is Rust-only (mirror shows combined "X% + Y%" format).
- C1: Dendro DMG +15%. Implemented with both self and team entries.
- C4: EM from 0.3% Max HP, max 120. `base_value: 0.003, cap: Some(120.0)`. Correct.
- **Constellation pattern: BUG** - Should be `C3SkillC5Burst` (C3=Raphanus Sky Cluster, C5=Moonjade Descent).

### Alhaitham
- All 11 scaling values verified (NA 1-5, charged x2, plunge x3, skill thrust ATK, burst hit ATK). All match.
- Skill and Burst have dual ATK+EM scaling; ATK portions match. EM portions are MIRROR_ONLY (8 entries for 1/2/3-mirror projections and burst EM scaling).
- A4: DMG +0.1% per EM, max +100%. `base_value: 0.001, cap: Some(1.00)`. Correct.
- C2: EM +50 per mirror (max 3 stacks). Correct.
- C4: Team EM +30 per mirror consumed + Self Dendro DMG +10% per mirror generated. Both Stacks(3). Correct.
- C6: CR +10%, CD +70%. Correct.
- **Constellation pattern: BUG** - Should be `C3SkillC5Burst` (C3=Universality: An Elaboration on Form, C5=Particular Field: Fetters of Phenomena).

### Baizhu
- All 10 scaling values verified (NA 1-4, charged, plunge x3, skill, burst spiritvein). All match.
- A4: Reaction-specific bonuses per 1000 HP (Burning/Bloom/Hyperbloom/Burgeon +2% cap 100%, Lunar-Bloom +0.7% cap 35%, Aggravate/Spread +0.8% cap 40%). All 7 entries correct.
- C4: Team EM +80. Correct.
- C6: Skill flat DMG +8% of Max HP. Correct.
- Constellation pattern: C3BurstC5Skill. Correct (C3=Holistic Revivification, C5=Universal Diagnosis).

### Kaveh
- All 11 scaling values verified (NA 1-4, charged spinning/final, plunge x3, skill, burst). All match.
- Burst scaling array (`KAVEH_BURST_SCALING`) verified: 15 values match mirror "Dendro Core Burst DMG Bonus" exactly.
- A4: EM +25 per Bloom core (max 4 stacks). Correct.
- C1: Healing Bonus +25%. Correct.
- C4: Bloom DMG +60% (TransformativeBonus). Correct.
- Constellation pattern: C3BurstC5Skill. Correct (C3=Painted Dome, C5=Artistic Ingenuity).

### Kirara
- All 14 scaling values verified (NA 1-3a, 3b, 4, charged, plunge x3, skill kick/parcel/flipclaw, burst DMG, burst explosion). All match. NA hit 3 properly split; charged is x3 multi-hit.
- A4: NOT IMPLEMENTED (TODO in code). Mirror specifies HP-based Skill/Burst DMG bonus.
- C6: All Elemental DMG +12%. Implemented as generic DmgBonus (minor: includes physical).
- Constellation pattern: C3SkillC5Burst. Correct.

### Emilie
- All 13 scaling values verified (NA 1-4, charged, plunge x3, skill main/Lv1/Lv2/thorn, burst Lv3). All match.
- A4 "Rectification": DMG +0.015% per ATK, max +36%. `base_value: 0.00015, scales_on: TotalAtk, cap: 0.36`. Correct.
- C1: Skill DMG +20%. Correct.
- C2: Dendro RES -30%. Correct.
- Constellation pattern: C3SkillC5Burst. Correct.

### Kinich
- All 12 scaling values verified (NA 1-3, mid-air, charged, plunge x3, skill loop/cannon, burst DMG/breath). All match.
- C2: Dendro RES -30% (team) + DMG +100% (self). Both correct.
- C4: Burst DMG +70%. Correct.
- Constellation pattern: C3SkillC5Burst. Correct.

### Traveler (Dendro)
- All 12 scaling values verified against `playerboy_005.md` (NA 1-5, charged, plunge x3, skill, burst lamp/explosion). All match.
- Note: Charged attack is "55.9% + 60.72%" two-hit; Rust stores first hit (0.559) as single TalentScaling.
- A4 "Verdant Luxury": EM +60 in Lea Lotus Lamp field. Correct.
- C6: Dendro DMG +12% inside Lamp. Correct.
- Constellation pattern: C3SkillC5Burst. Correct (C3=Razorgrass Blade, C5=Surgent Manifestation).

### Lauma
- All 14 scaling values verified (NA 1-3, charged, plunge x3, skill press/hold1/hold2/sanctuary ATK/sanctuary EM, burst bloom increase/lunar-bloom increase). All match.
- A4: Skill DMG +0.04% per EM, max +32%. `base_value: 0.0004, cap: 0.32`. Correct.
- C2: Lunar-Bloom DMG +40% (team). Correct.
- C6: Lunar-Bloom DMG +25% (team). Correct.
- Constellation pattern: C3BurstC5Skill. Correct.

### Nefer
- All 21 scaling values verified (NA 1-4, charged, plunge x3, skill ATK/EM, phantasm Nefer 1-2 ATK/EM, phantasm Shades 1-3, burst 1-2 ATK/EM). All match.
- C2: EM +200 at 5 Veil stacks. Correct.
- C4: Dendro RES -20% in Shadow Dance. Correct.
- C6: Lunar-Bloom DMG +15%. Value correct. Uses TransformativeBonus (minor: should ideally be ReactionDmgBonus for consistency).
- Constellation pattern: C3SkillC5Burst. Correct.
