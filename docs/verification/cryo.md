# Cryo Characters Verification

Verified against `honeyhunter-mirror/md/` spec files. Scope: talent scaling ratios (15 levels), passive talents, constellation effects, buff implementations. Excluded: proc damage, energy, HP regen, stamina, durations, cooldowns.

## Summary Table

| Character | Scaling | Passives | Constellations | Status |
|-----------|---------|----------|----------------|--------|
| Kaeya | OK | OK | BUG: pattern swapped | ISSUE |
| Chongyun | OK | OK | OK | OK |
| Qiqi | BUG: N1 Lv1 value | OK | OK | ISSUE |
| Ganyu | OK | OK | OK | OK |
| Diona | OK | OK | OK | OK |
| Rosaria | OK | OK | OK | OK |
| Eula | OK | OK (RES shred simplified) | OK | OK |
| Ayaka | OK | OK | OK | OK |
| Shenhe | BUG: Icy Quill Lv2-15 | MISSING: A1 Cryo DMG | BUG: pattern swapped; C2 simplification | ISSUE |
| Aloy | OK | OK | OK | OK |
| Layla | OK | OK | BUG: pattern swapped | ISSUE |
| Mika | OK | MISSING: A1 Phys DMG | OK | ISSUE |
| Freminet | OK | OK | BUG: C3=Normal Attack (enum limitation) | ISSUE |
| Charlotte | OK | OK | OK | OK |
| Wriothesley | OK | OK | BUG: C3=Normal Attack (enum limitation) | ISSUE |
| Citlali | OK | OK | BUG: pattern swapped | ISSUE |
| Skirk | OK | OK | BUG: pattern swapped | ISSUE |
| Escoffier | OK | OK | BUG: pattern swapped | ISSUE |

## Issues

### BUG: Constellation Pattern Swapped (6 characters)

Characters with `C3BurstC5Skill` that should be `C3SkillC5Burst` (or vice versa):

| Character | File | Current | Should Be |
|-----------|------|---------|-----------|
| Kaeya | `characters/cryo/kaeya.rs` | `C3BurstC5Skill` | `C3SkillC5Burst` |
| Shenhe | `characters/cryo/shenhe.rs` | `C3BurstC5Skill` | `C3SkillC5Burst` |
| Layla | `characters/cryo/layla.rs` | `C3BurstC5Skill` | `C3SkillC5Burst` |
| Citlali | `characters/cryo/citlali.rs` | `C3BurstC5Skill` | `C3SkillC5Burst` |
| Escoffier | `characters/cryo/escoffier.rs` | `C3BurstC5Skill` | `C3SkillC5Burst` |
| Skirk | `characters/cryo/skirk.rs` | `C3SkillC5Burst` | `C3BurstC5Skill` |

### BUG: Constellation Pattern Enum Limitation (2 characters)

Freminet and Wriothesley have C3 = Normal Attack +3, which the `ConstellationPattern` enum cannot represent (it only has Skill and Burst variants).

| Character | File | C3 (mirror) | C5 (mirror) | Current |
|-----------|------|-------------|-------------|---------|
| Freminet | `characters/cryo/freminet.rs` | Normal Attack +3 | Skill +3 | `C3SkillC5Burst` |
| Wriothesley | `characters/cryo/wriothesley.rs` | Normal Attack +3 | Burst +3 | `C3SkillC5Burst` |

### BUG: Qiqi Normal Attack 1 Lv1 Scaling

- **File**: `characters/cryo/qiqi.rs`
- **Field**: `QIQI_NORMAL_1.values[0]`
- **Current**: `0.3754`
- **Expected**: `0.3775` (mirror: 37.75%)

### BUG: Shenhe Icy Quill Scaling Lv2-15

- **File**: `talent_buffs/cryo.rs`
- **Field**: `SHENHE_SKILL_SCALING` array
- **Issue**: Systematic +0.0001 error at levels 2-15
- **Current**: `[0.4566, 0.4909, 0.5251, 0.5708, 0.6050, 0.6393, 0.6849, 0.7306, 0.7763, 0.8219, 0.8676, 0.9132, 0.9703, 1.0274, 1.0844]`
- **Expected**: `[0.4566, 0.4908, 0.5250, 0.5707, 0.6049, 0.6392, 0.6848, 0.7305, 0.7762, 0.8218, 0.8675, 0.9131, 0.9702, 1.0273, 1.0843]`

### MISSING: Shenhe A1 "Deific Embrace" Cryo DMG Buff

- **File**: `talent_buffs/cryo.rs`
- **Issue**: Shenhe A1 passive should grant +15% Cryo DMG Bonus to characters in the burst field. Not implemented.

### MISSING: Mika A1 "Suppressive Barrage" Physical DMG Buff

- **File**: `talent_buffs/cryo.rs`
- **Issue**: Mika A1 passive should grant +10% Physical DMG Bonus per Detector stack (max 3 stacks = 30%). Not implemented.

### SIMPLIFICATION: Shenhe C2 Generic CritDmg

- **File**: `talent_buffs/cryo.rs`
- **Issue**: Shenhe C2 should grant Cryo CRIT DMG +15%, but `BuffableStat` enum has no element-specific CritDmg variant. Currently uses generic `CritDmg`, which applies to all damage types. This is an engine limitation, not a data bug.

### SIMPLIFICATION: Eula Skill RES Shred

- **File**: `talent_buffs/cryo.rs`
- **Issue**: Eula Grimheart RES shred uses max talent level value as a fixed constant rather than scaling per talent level. Acceptable simplification.

## Character Notes

### Kaeya
Mirror: `kaeya_006.md`. All scaling values verified. Constellation pattern swapped.

### Chongyun
Mirror: `chongyun_036.md`. All scaling values and buffs verified. No issues.

### Qiqi
Mirror: `qiqi_035.md`. Normal 1 Lv1 off by 0.0021. All other values verified.

### Ganyu
Mirror: `ganyu_037.md`. All scaling values and buffs verified. No issues.

### Diona
Mirror: `diona_039.md`. All scaling values verified. No issues.

### Rosaria
Mirror: `rosaria_045.md`. All scaling values and buffs verified. No issues.

### Eula
Mirror: `eula_051.md`. All scaling values verified. RES shred simplification noted.

### Ayaka
Mirror: `ayaka_002.md` (note: `ayaka_028.md` contains wrong character "Shiro Maiden"). All scaling values and buffs verified. No issues.

### Shenhe
Mirror: `shenhe_063.md`. Icy Quill scaling has systematic rounding error. A1 Cryo DMG buff missing. C2 uses generic CritDmg. Constellation pattern swapped.

### Aloy
Mirror: `aloy_062.md`. All scaling values verified. No issues.

### Layla
Mirror: `layla_074.md`. All scaling values verified. Constellation pattern swapped.

### Mika
Mirror: `mika_075.md`. All scaling values verified. A1 Physical DMG buff missing.

### Freminet
Mirror: `freminet_085.md`. All scaling values verified. C3=Normal Attack cannot be represented by enum.

### Charlotte
Mirror: `charlotte_088.md`. All scaling values verified. No issues.

### Wriothesley
Mirror: `wriothesley_086.md`. All scaling values verified. C3=Normal Attack cannot be represented by enum.

### Citlali
Mirror: `citlali_098.md`. All scaling values verified. Constellation pattern swapped.

### Skirk
Mirror: `skirk_102.md`. All scaling values verified. Constellation pattern swapped.

### Escoffier
Mirror: `escoffier_104.md`. All scaling values verified. Constellation pattern swapped.
