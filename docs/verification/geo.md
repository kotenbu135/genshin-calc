# Geo Character Verification

Verified against: `honeyhunter-mirror/md/characters/`
Scope: 全15レベル倍率、基礎ステータス、突破ステータス、凸パターン、固有天賦バフ、命ノ星座バフ
Excluded: proc damage（追加ヒットのみ）、元素チャージ、HP回復、スタミナ消費、継続時間、クールタイム

## Summary

| Character | Scaling | Base Stats | Constellation Pattern | Passives/Buffs | Status |
|-----------|---------|------------|----------------------|----------------|--------|
| Albedo | Minor (Lv11-15) | OK | OK | OK | WARN |
| Chiori | **CRITICAL** | OK | OK | OK (partial) | FAIL |
| Gorou | Minor (Lv9-15 rounding) | OK | **WRONG** | **MISSING** A4 | FAIL |
| Illuga | **MISSING** N3 hit | OK | OK | **MISSING** A4 | FAIL |
| Itto | Minor (Lv11-15) + **MISSING** | OK | **WRONG** | OK | FAIL |
| Kachina | OK | OK | OK | OK | PASS |
| Navia | OK | OK | **WRONG** | OK | FAIL |
| Ningguang | OK | OK | OK | OK | PASS |
| Noelle | **CRITICAL** | **CRITICAL** | **WRONG** | OK | FAIL |
| Xilonen | OK | OK (ascn stat wrong) | OK | **MISSING** A4 | FAIL |
| Yun Jin | **WRONG** charged | OK | OK | **MISSING** A4 | FAIL |
| Zhongli | **CRITICAL** burst | OK | **WRONG** | OK | FAIL |
| Zibai | **MISSING** 4th hit add | OK | OK | OK | FAIL |

**Total issues: 21**
- CRITICAL/HIGH bugs (wrong damage output): 5
- Missing scalings/hits: 4
- Wrong constellation patterns: 5 characters
- Missing buff implementations: 4
- Minor value divergences: 3+

---

## Critical Issues

### C1) Noelle base stats completely corrupted
**Severity: CRITICAL** | File: `crates/data/src/characters/geo/noelle.rs` lines 179-197

The entire base_hp/base_atk/base_def arrays from index 1 onward are nonsensical. Values are non-monotonic and don't match any ascension breakpoints.

| Stat | Index | Rust | Mirror (Lv20) |
|------|-------|------|---------------|
| base_hp[1] | 10698.00 | 2600.00 |
| base_atk[1] | 172.00 | 41.17 |
| base_def[1] | 709.00 | 172.00 |

Mirror base stats for reference:
```
Lv1:  HP=1012  ATK=16.03  DEF=66.95
Lv20: HP=2600  ATK=41.17  DEF=172.0
Lv40: HP=5027  ATK=79.61  DEF=332.56
Lv60: HP=7953  ATK=125.94 DEF=526.09
Lv80: HP=10698 ATK=169.41 DEF=707.71
Lv90: HP=12071 ATK=191.16 DEF=798.55
```

The Rust array appears to have Lv80 values at index 1, then fluctuates randomly. All 18 entries need to be rebuilt from the mirror.

### C2) Noelle charged attack values wrong (copied from Navia)
**Severity: CRITICAL** | File: `crates/data/src/characters/geo/noelle.rs` lines 56-76

| Entry | Lv1 Rust | Lv1 Mirror | Delta |
|-------|----------|------------|-------|
| Charged Spinning | 0.6252 | 0.5074 (50.74%) | +0.1178 |
| Charged Final | 1.3144 | 0.9047 (90.47%) | +0.4097 |

The Rust values (0.6252, 1.3144) match Navia's charged attack exactly, not Noelle's.

Correct values from mirror:
- Spinning Lv1-15: 50.74%, 54.87%, 59%, 64.9%, 69.03%, 73.75%, 80.24%, 86.73%, 93.22%, 100.3%, 107.38%, 114.46%, 121.54%, 128.62%, 135.7%
- Final Lv1-15: 90.47%, 97.84%, 105.2%, 115.72%, 123.08%, 131.5%, 143.07%, 154.64%, 166.22%, 178.84%, 191.46%, 204.09%, 216.71%, 229.34%, 241.96%

### C3) Zhongli burst scaling wrong from Lv2 onward
**Severity: CRITICAL** | File: `crates/data/src/characters/geo/zhongli.rs` lines 194-203

| Level | Rust | Mirror | Delta |
|-------|------|--------|-------|
| Lv1 | 4.0108 | 4.0108 (401.08%) | OK |
| Lv2 | 4.2528 | 4.4444 (444.44%) | -0.1916 |
| Lv3 | 4.4948 | 4.8780 (487.8%) | -0.3832 |
| Lv4 | 4.8010 | 5.4200 (542%) | -0.6190 |
| Lv5 | 5.0430 | 5.9078 (590.78%) | -0.8648 |
| ... | ... | ... | gap widens |
| Lv15 | 8.4201 | 11.924 (1192.4%) | -3.5039 |

The curve is dramatically flatter than the mirror. Entire array from index 1 onward needs replacement.

### C4) Chiori skill/burst missing DEF scaling component
**Severity: CRITICAL** | File: `crates/data/src/characters/geo/chiori.rs` lines 126-170

Mirror shows dual ATK+DEF scaling for all skill/burst entries:
- Tamoto: 82.08% ATK + **102.6% DEF** → Rust only has ATK (0.8208)
- Upward Sweep: 149.28% ATK + **186.6% DEF** → Rust only has ATK (1.4928)
- Burst: 256.32% ATK + **320.4% DEF** → Rust only has ATK (2.5656, also slightly off: should be 2.5632)

All DEF scaling components are completely absent, causing roughly 50% damage underestimation for these abilities.

Additionally, `CHIORI_SKILL_TURRET` (values[0]=1.4880) does not match any current mirror entry. No "飛び道具" or 148.8% value exists in the mirror. The closest match is Upward Sweep ATK at 149.28% (1.4928), suggesting this may be an outdated version of that scaling. Recommend replacing with correct values rather than removing.

### C5) Yun Jin charged attack wrong at all 15 levels
**Severity: HIGH** | File: `crates/data/src/characters/geo/yun_jin.rs` lines 89-98

| Level | Rust | Mirror | Delta |
|-------|------|--------|-------|
| Lv1 | 1.1596 | 1.2169 (121.69%) | -0.0573 |
| Lv2 | 1.2541 | 1.3160 (131.6%) | -0.0619 |
| Lv10 | 2.2928 | 2.4055 (240.55%) | -0.1127 |
| Lv11 | 2.4546 | 2.6001 (260.01%) | -0.1455 |
| Lv15 | 3.1016 | 3.5361 (353.61%) | -0.4345 |

The error exists at all 15 levels and the gap grows significantly at higher levels. The entire values array needs replacement.

---

## Missing Scalings / Hits

### M1) Illuga N3 missing second hit
**Severity: HIGH** | File: `crates/data/src/characters/geo/illuga.rs` lines 32-41

Mirror: "31.43% + 31.43%" = two hits. Rust: only one `ILLUGA_NORMAL_3` (0.3143).
Need to add `ILLUGA_NORMAL_3_2` with identical values and include it in the hits array.

### M2) Itto missing Saichimonji Slash
**Severity: MEDIUM** | File: `crates/data/src/characters/geo/itto.rs`

Mirror shows "左一文字斬りダメージ" (Saichimonji Slash DMG) as a separate charged attack scaling:
Lv1-15: 90.47%, 97.84%, 105.2%, 115.72%, 123.08%, 131.5%, 143.07%, 154.64%, 166.22%, 178.84%, 193.31%, 210.32%, 227.33%, 244.34%, 262.89%

This is the uncharged heavy attack (used when no Superlative Superstrength stacks are available). Not present in Rust.

### M3) Zibai missing Lunar Phase Shift 4th hit additional damage
**Severity: MEDIUM** | File: `crates/data/src/characters/geo/zibai.rs`

Mirror: "隙を過ぐる月の4段目追加ダメージ" = 29.46% DEF at Lv1, scaling to 69.96% DEF at Lv15.
This is considered Lunar-Crystallize Reaction DMG. Not present in Rust skill scalings.

### M4) Xilonen ascension stat wrong
**Severity: HIGH** | File: `crates/data/src/characters/geo/xilonen.rs` line 221

- Rust: `AscensionStat::Def(0.288)` = 28.8%
- Mirror (80+): Bonus Def = **36.0%** → should be `AscensionStat::Def(0.36)`

---

## Wrong Constellation Patterns

### CP1-5) Five characters have C3/C5 reversed
**Severity: HIGH** | All should be `C3SkillC5Burst` but have `C3BurstC5Skill`

| Character | File | C3 boosts (mirror) | C5 boosts (mirror) |
|-----------|------|--------------------|--------------------|
| Zhongli | zhongli.rs | 地心 (Dominus Lapidis = **Skill**) | 天星 (Planet Befall = **Burst**) |
| Noelle | noelle.rs | 護心鎧 (Breastplate = **Skill**) | 大掃除 (Sweeping Time = **Burst**) |
| Gorou | gorou.rs | 犬坂の遠吠え方円陣 (Inuzaka All-Round Defense = **Skill**) | 獣牙突撃陣形戦法 (Juuga = **Burst**) |
| Itto | itto.rs | 魔殺絶技·岩牛発破！ (Masatsu Zetsugi = **Skill**) | 最凶鬼王·一斗轟臨！！ (Royal Descent = **Burst**) |
| Navia | navia.rs | セレモニアル·クリスタルショット (Ceremonial Crystalshot = **Skill**) | 晴天を衝く霰弾のサルート (Singing Salute = **Burst**) |

---

## Missing Buff Implementations

### B1) Gorou A4「風雨を恐れず」(Heedless of the Wind and Weather) DEF+25% 未実装
**Severity: HIGH** | File: `crates/data/src/talent_buffs/geo.rs`

Mirror: "After using Juuga: Forward Unto Victory, all nearby party members' DEF is increased by 25% for 12s."
Not present in `GOROU_BUFFS`. Affects all DEF-scaling characters in Gorou teams.

### B2) Yun Jin A4「独立嶄然」(Breaking Conventions) 未実装
**Severity: HIGH** | File: `crates/data/src/talent_buffs/geo.rs`

Mirror: Flying Cloud Flag Formation's normal attack DMG bonus is further increased by 2.5/5/7.5/11.5% DEF based on 1/2/3/4 different element types in the party.
Not present in `YUN_JIN_BUFFS`. This is a core buff for Yun Jin's main support role.

### B3) Xilonen A4「Portable Armored Sheath」DEF+20% 未実装
**Severity: MEDIUM** | File: `crates/data/src/talent_buffs/geo.rs`

Mirror: "When nearby party members trigger a Nightsoul Burst, Xilonen's DEF is increased by 20% for 15s."
Not present in `XILONEN_BUFFS`.

### B4) Illuga A4 未実装 (TODO only)
**Severity: MEDIUM** | File: `crates/data/src/talent_buffs/geo.rs`

A4 passive is listed as TODO/dead code only. The `ILLUGA_BURST_GEO_DMG_SCALING` exists with `#[allow(dead_code)]` but is not wired into any buff definition.

---

## Minor Issues

### Min1) Gorou C6 applies Geo CritDMG as global CritDMG
**Severity: LOW** | File: `crates/data/src/talent_buffs/geo.rs`

Mirror: C6 grants Geo DMG CRIT DMG +10/20/40% (element-specific).
Rust: Implemented as `BuffableStat::CritDmg` which applies to all elements.

### Min2) Lv11-15 normal attack value divergence (Albedo, Zhongli, Itto)
**Severity: LOW** | Systematic pattern across multiple characters

Values diverge starting at Lv11, gap widens through Lv15:
- Albedo N1: Lv11 mirror=78.50% vs Rust=0.7777 (delta=0.0073)
- Zhongli N1: Lv11 mirror=65.74% vs Rust=0.6513 (delta=0.0061)
- Itto N1: Lv11 mirror=169.29% vs Rust=1.6763 (delta=0.0166)

Likely a data source version difference for Lv11-15 values.

### Min3) Gorou skill DEF scaling minor rounding (Lv9-15)
**Severity: NEGLIGIBLE** | File: `crates/data/src/talent_buffs/geo.rs` lines 69-72

Multiple values off by 0.01-0.02:
- Lv9: mirror=350.47, Rust=350.48
- Lv10: mirror=371.09, Rust=371.10
- Lv11-15: similar ±0.02 differences

### Min4) Chiori burst ATK portion minor error
**Severity: NEGLIGIBLE** | File: `crates/data/src/characters/geo/chiori.rs` line 166

Lv1: Rust=2.5656, mirror=256.32%=2.5632. Delta=0.0024.

### Min5) Gorou skill DEF scaling Lv11-15 divergence
**Severity: LOW** | Same pattern as Min2, affects Gorou skill DEF flat values at high levels.

---

## Suggested Fixes (priority order)

1. **Noelle base stats**: Rebuild entire base_hp/base_atk/base_def arrays from mirror (C1)
2. **Noelle charged attack**: Replace with correct mirror values (C2)
3. **Zhongli burst**: Replace entire ZHONGLI_BURST values array from mirror (C3)
4. **Chiori skill/burst**: Add DEF scaling components for Tamoto, Upward Sweep, and Burst; remove or fix CHIORI_SKILL_TURRET (C4)
5. **Constellation patterns**: Change C3BurstC5Skill → C3SkillC5Burst for Zhongli, Noelle, Gorou, Itto, Navia (CP1-5)
6. **Xilonen ascension stat**: Change Def(0.288) → Def(0.36) (M4)
7. **Yun Jin charged attack**: Replace entire values array from mirror (Min5)
8. **Illuga N3**: Add second hit entry (M1)
9. **Itto Saichimonji Slash**: Add as charged attack entry (M2)
10. **Zibai 4th hit additional**: Add Lunar Phase Shift 4th hit additional DMG scaling (M3)
11. **Gorou A4 DEF+25%**: Add to GOROU_BUFFS (B1)
12. **Yun Jin A4**: Implement Breaking Conventions DEF bonus by element count (B2)
13. **Xilonen A4 DEF+20%**: Add to XILONEN_BUFFS (B3)
14. **Illuga A4**: Implement A4 passive (B4)
15. **Gorou C6**: Restrict CritDMG to Geo damage only (Min1)
16. **Lv11-15 divergence**: Update normal attack Lv11-15 values for Albedo, Zhongli, Itto (Min2)

---

## Characters Verified Clean

### Kachina
- All 15 levels of all talent scalings match mirror within floating-point tolerance
- Base stats correct at all ascension breakpoints
- Constellation pattern correct (C3SkillC5Burst)
- A4 Geo DMG+20% and C4 DEF+20% implemented

### Ningguang
- All 15 levels of all talent scalings match mirror
- Base stats correct
- Constellation pattern correct
- A4 Geo DMG+12% implemented
