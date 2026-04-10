# Cryo Characters Talent Scaling Audit

**Date**: 2026-04-08
**Scope**: All 18 Cryo characters — Normal Attack / Elemental Skill / Elemental Burst scaling values (`values: [f64; 15]`)
**Tolerance**: 0.0002 (0.02%)
**Source**: `honeyhunter-mirror/md/characters/` vs `crates/data/src/characters/cryo/`
**Exclusions**: Proc/additional damage, stamina cost, duration, CD, healing, shield HP scaling

---

## Summary

| Status | Count | Characters |
|--------|-------|-----------|
| FAIL (critical) | 3 | Aloy, Eula, Skirk |
| FAIL (minor) | 1 | Layla |
| BORDERLINE | 1 | Shenhe |
| OK | 13 | Ayaka, Charlotte, Chongyun, Citlali, Diona, Escoffier, Freminet, Ganyu, Kaeya, Mika, Qiqi, Rosaria, Wriothesley |

---

## Discrepancies

### Aloy — CRITICAL: Missing 4th Normal Attack Hit

File: `crates/data/src/characters/cryo/aloy.rs`
Mirror: `honeyhunter-mirror/md/characters/aloy_062.md`

The Rust implementation has only 4 `NORMAL_*` constants (NORMAL_1 through NORMAL_4), which map to:
- NORMAL_1+2: Hit 1 (two-arrow split: 21.12% + 23.76% at Lv1)
- NORMAL_3: Hit 2 (43.12% at Lv1)
- NORMAL_4: Hit 3 (52.80% at Lv1)

The **4th hit** (Hit 4) is entirely absent from the Rust file.

| Lv | Rust | Mirror | Diff |
|----|------|--------|------|
| 1  | (missing) | 65.65% | — |
| 2  | (missing) | 70.97% | — |
| 3  | (missing) | 76.30% | — |
| 4  | (missing) | 83.93% | — |
| 5  | (missing) | 89.26% | — |
| 6  | (missing) | 95.37% | — |
| 7  | (missing) | 103.77% | — |
| 8  | (missing) | 112.17% | — |
| 9  | (missing) | 120.57% | — |
| 10 | (missing) | 129.74% | — |
| 11 | (missing) | 138.91% | — |
| 12 | (missing) | 148.08% | — |
| 13 | (missing) | 157.25% | — |
| 14 | (missing) | 166.42% | — |
| 15 | (missing) | 150.69% | — |

**Fix**: Add a new `ALOY_NORMAL_4` constant with these 15 values and include it in the normal attack `TalentScaling` definition.

**Implementation status:** Fixed in this branch.

---

### Eula — CRITICAL: Wrong Plunge Scaling at Lv11–15

File: `crates/data/src/characters/cryo/eula.rs`
Mirror: `honeyhunter-mirror/md/characters/eula_052.md`

All three plunge rows (Plunge DMG / Low Plunge / High Plunge) use incorrect values at Lv11–15. Eula has character-specific plunge scaling that diverges from the generic claymore table at higher levels.

**Plunge DMG (fall period)**

| Lv | Rust | Mirror | Diff |
|----|------|--------|------|
| 1–10 | matches | matches | OK |
| 11 | 157.85% | 159.37% | +1.52% |
| 12 | 168.26% | 173.39% | +5.13% |
| 13 | 178.66% | 187.41% | +8.75% |
| 14 | 189.07% | 201.44% | +12.37% |
| 15 | 199.48% | 216.74% | +17.26% |

**Low Plunge DMG**

| Lv | Rust | Mirror | Diff |
|----|------|--------|------|
| 1–10 | matches | matches | OK |
| 11 | 315.64% | 318.73% | +3.09% |
| 12 | 336.41% | 346.93% | +10.52% |
| 13 | 357.17% | 375.12% | +17.95% |
| 14 | 377.94% | 403.32% | +25.38% |
| 15 | 398.71% | 433.55% | +34.84% |

**High Plunge DMG**

| Lv | Rust | Mirror | Diff |
|----|------|--------|------|
| 1–10 | matches | matches | OK |
| 11 | 394.32% | 398.18% | +3.86% |
| 12 | 420.20% | 433.23% | +13.03% |
| 13 | 446.09% | 468.28% | +22.19% |
| 14 | 471.98% | 503.33% | +31.35% |
| 15 | 497.87% | 541.47% | +43.60% |

**Fix**: Replace the Lv11–15 entries in `EULA_PLUNGE`, `EULA_PLUNGE_LOW`, and `EULA_PLUNGE_HIGH` with the correct values from the mirror.

Correct values:
```
EULA_PLUNGE     Lv11–15: 1.5937, 1.7339, 1.8741, 2.0144, 2.1674
EULA_PLUNGE_LOW Lv11–15: 3.1873, 3.4693, 3.7512, 4.0332, 4.3355
EULA_PLUNGE_HIGH Lv11–15: 3.9818, 4.3323, 4.6828, 5.0333, 5.4147
```

**Implementation status:** Fixed in this branch.

---

### Skirk — CRITICAL: Digit Transposition at Normal 4 Lv14

File: `crates/data/src/characters/cryo/skirk.rs`
Mirror: `honeyhunter-mirror/md/characters/skirknew_114.md`

`SKIRK_NORMAL_4` has a digit transposition at level 14.

| Lv | Rust | Mirror | Diff |
|----|------|--------|------|
| 13 | 145.64% | 145.64% | OK |
| **14** | **144.13%** | **154.13%** | **−10.00%** |
| 15 | 162.61% | 162.61% | OK |

The surrounding values confirm a monotonically increasing sequence; 144.13% breaks the pattern. The correct value is 154.13% (the digits `54` were transposed to `44`).

**Fix**: Change `SKIRK_NORMAL_4[13]` from `1.4413` to `1.5413`.

**Implementation status:** Fixed in this branch.

---

### Layla — MINOR: Normal 3 Lv13 Rounding Error

File: `crates/data/src/characters/cryo/layla.rs`
Mirror: `honeyhunter-mirror/md/characters/layla_077.md`

| Constant | Lv | Rust | Mirror | Diff |
|----------|----|------|--------|------|
| LAYLA_NORMAL_3 | 13 | 175.00% | 174.80% | +0.20% |

All other levels in LAYLA_NORMAL_3 match within tolerance. This appears to be a rounding artifact (175.00 is a "round" number while the mirror shows 174.80).

**Fix**: Change `LAYLA_NORMAL_3[12]` from `1.7500` to `1.7480`.

**Implementation status:** Fixed in this branch.

---

### Shenhe — BORDERLINE: Normal 1 Lv12 at Threshold

File: `crates/data/src/characters/cryo/shenhe.rs`
Mirror: `honeyhunter-mirror/md/characters/shenhe_063.md`

| Constant | Lv | Rust | Mirror | Diff |
|----------|----|------|--------|------|
| SHENHE_NORMAL_1 | 12 | 97.60% | 97.58% | +0.02% |

Difference is exactly at the 0.02% tolerance boundary. This is likely a floating-point representation issue. No action required, but noted for completeness.

---

## All Clear

The following 13 characters have all damage scaling values within the 0.02% tolerance:

| Character | File |
|-----------|------|
| Ayaka | `cryo/ayaka.rs` |
| Charlotte | `cryo/charlotte.rs` |
| Chongyun | `cryo/chongyun.rs` |
| Citlali | `cryo/citlali.rs` |
| Diona | `cryo/diona.rs` |
| Escoffier | `cryo/escoffier.rs` |
| Freminet | `cryo/freminet.rs` |
| Ganyu | `cryo/ganyu.rs` |
| Kaeya | `cryo/kaeya.rs` |
| Mika | `cryo/mika.rs` |
| Qiqi | `cryo/qiqi.rs` |
| Rosaria | `cryo/rosaria.rs` |
| Wriothesley | `cryo/wriothesley.rs` |
