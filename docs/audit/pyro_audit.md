# Pyro Characters Audit

Comparison of talent scaling values and base stats in `crates/data/src/characters/pyro/` against `honeyhunter-mirror/md/characters/`.
Tolerance: 0.0002 (0.02%). Non-damage rows (stamina cost, duration, CD, healing, etc.) and 追加ダメージ excluded.

## Summary

- Total characters checked: 17
- Characters with issues: 4
- Characters OK: 13

## Discrepancies

### Diluc

**File**: `crates/data/src/characters/pyro/diluc.rs`
**Mirror**: `honeyhunter-mirror/md/characters/diluc_016.md`

#### Charged Attack — Cyclic DMG (高速連続切りダメージ)

Lv11–Lv15 are significantly lower in Rust than mirror. The Rust array appears to extrapolate linearly beyond Lv10, while the mirror values diverge from that linear pattern.

| Level | Rust | Mirror | Diff |
|-------|------|--------|------|
| Lv11  | 1.4560 (145.60%) | 147.0%   | -0.0140 |
| Lv12  | 1.5520 (155.20%) | 159.94%  | -0.0474 |
| Lv13  | 1.6480 (164.80%) | 172.87%  | -0.0807 |
| Lv14  | 1.7440 (174.40%) | 185.81%  | -0.1141 |
| Lv15  | 1.8400 (184.00%) | 199.92%  | -0.1592 |

#### Base Stats

| Stat    | Level | Rust    | Mirror  | Diff   |
|---------|-------|---------|---------|--------|
| base_hp | Lv60 (index 7) | 8392 | 8421 | -29 |
| base_atk | Lv60 (index 7) | 216.22 | 217.22 | -1.00 |

**Implementation status:** Fixed in this branch.

---

### Lyney

**File**: `crates/data/src/characters/pyro/lyney.rs`
**Mirror**: `honeyhunter-mirror/md/characters/liney_084.md`

#### Base Stats — MAJOR ERROR

The entire `base_hp`, `base_atk`, and `base_def` arrays are filled with wrong-level values. The data starting at index 1 (Lv20 position) matches the mirror's Lv80 breakpoint data, not Lv20. All indices from 1 onward are shifted.

Examples comparing Rust array values vs mirror expected values:

| Index | Level | Stat     | Rust Value | Mirror Expected | Mirror Lv80 |
|-------|-------|----------|-----------|-----------------|-------------|
| 0     | Lv1   | base_hp  | 858       | 858 ✓           | —           |
| 0     | Lv1   | base_atk | 25.00     | 24.76           | —           |
| 1     | Lv20  | base_hp  | 9724.00   | 2226            | 9724 ✓      |
| 1     | Lv20  | base_atk | 281.00    | 64.24           | 280.66 ≈281 |

All 17 base stat values from index 1 onward are using Lv80+ level data instead of the correct progression starting at Lv20.

Talent scaling values are correct.

**Implementation status:** Fixed in this branch.

---

### Thoma

**File**: `crates/data/src/characters/pyro/thoma.rs`
**Mirror**: `honeyhunter-mirror/md/characters/tohma_050.md`

#### Base Stats — MAJOR ERROR

Same wrong-level pattern as Lyney. The `base_hp`, `base_atk`, and `base_def` arrays are filled with Lv80-level data starting from index 1 (Lv20 position).

Examples comparing Rust array values vs mirror expected values:

| Index | Level | Stat     | Rust Value | Mirror Expected | Mirror Lv80 |
|-------|-------|----------|-----------|-----------------|-------------|
| 0     | Lv1   | base_hp  | 866       | 866 ✓           | —           |
| 0     | Lv1   | base_atk | 17.00     | 16.92           | —           |
| 0     | Lv1   | base_def | 63.00     | 62.95           | —           |
| 1     | Lv20  | base_hp  | 9156.00   | 2225            | 9156 ✓      |

All 17 base stat values from index 1 onward are using Lv80+ level data instead of the correct progression starting at Lv20.

Talent scaling values are correct.

**Implementation status:** Fixed in this branch.

---

### Yanfei

**File**: `crates/data/src/characters/pyro/yanfei.rs`
**Mirror**: `honeyhunter-mirror/md/characters/feiyan_048.md`

#### Base Stats — MAJOR ERROR

Same wrong-level pattern as Lyney and Thoma. The `base_hp`, `base_atk`, and `base_def` arrays are filled with Lv80-level data starting from index 1 (Lv20 position).

Examples comparing Rust array values vs mirror expected values:

| Index | Level | Stat     | Rust Value | Mirror Expected | Mirror Lv80 |
|-------|-------|----------|-----------|-----------------|-------------|
| 0     | Lv1   | base_hp  | 784       | 784 ✓           | —           |
| 0     | Lv1   | base_atk | 20.00     | 20.12           | —           |
| 1     | Lv20  | base_hp  | 8289.00   | 2014            | 8289 ✓      |
| 1     | Lv20  | base_atk | 213.00    | 51.70           | 212.71 ≈213 |

All 17 base stat values from index 1 onward are using Lv80+ level data instead of the correct progression starting at Lv20.

Talent scaling values are correct.

**Implementation status:** Fixed in this branch.

---

## Characters Confirmed OK

| Character | File | Mirror | Notes |
|-----------|------|--------|-------|
| Amber     | amber.rs | ambor_021.md | All talent scaling and base stats match ✓ |
| Arlecchino | arlecchino.rs | arlecchino_096.md | Talent scaling and base stats to Lv90 match ✓. Lv100 HP differs (Rust extrapolated: 14151.24 vs mirror 14034) — this is an extrapolation beyond mirror range, treated as out-of-scope. |
| Bennett   | bennett.rs | bennett_032.md | All values match ✓ |
| Chevreuse | chevreuse.rs | chevreuse_090.md | All values match ✓ |
| Dehya     | dehya.rs | dehya_079.md | ATK-based talent scaling matches ✓. Burst also scales on HP (Rust captures ATK portion only by design). |
| Durin     | durin.rs | durin_123.md | All values match ✓ |
| Gaming    | gaming.rs | gaming_092.md | All values match ✓ |
| Hu Tao    | hu_tao.rs | hutao_046.md | All values match ✓ |
| Klee      | klee.rs | klee_029.md | All values match ✓ |
| Mavuika   | mavuika.rs | mavuika_106.md | All values match ✓ |
| Xiangling | xiangling.rs | xiangling_023.md | All values match ✓. Note: XIANGLING_NORMAL_3B holds the 4th-hit values and XIANGLING_NORMAL_4 holds the 5th-hit — misleading names but values are correctly assigned. |
| Xinyan    | xinyan.rs | xinyan_044.md | All values match ✓ |
| Yoimiya   | yoimiya.rs | yoimiya_049.md | All values match ✓. Skill stores the additive damage bonus (0.3791 = +37.91% of Normal Attack DMG) which callers apply on top of 100%, yielding 137.91% as shown in mirror. |
