# Electro Characters Audit

Audit of all 18 Electro characters: talent scalings, base HP/ATK/DEF arrays, ascension stat, weapon type, and element assignment.  
Source comparison: `crates/data/src/characters/electro/*.rs` vs `honeyhunter-mirror/md/characters/*.md`  
Tolerance: absolute 0.0002 in decimal form  
Exclusions: non-damage rows, proc/additional damage, healing-only rows, energy, cooldown, stamina cost  
Date: 2026-04-10

---

## Summary

| Character | Base Stats | Ascension Stat | Weapon/Element | Talent Scalings | Status |
|-----------|-----------|----------------|----------------|-----------------|--------|
| Beidou    | OK (Lv100 drift noted) | OK | OK | OK | PASS |
| Clorinde  | OK (Lv100 drift noted) | OK | OK | OK | PASS |
| Cyno      | OK (Lv100 drift noted) | OK | OK | OK | PASS |
| Dori      | OK (Lv100 drift noted) | OK | OK | FAIL (minor rounding) | FAIL |
| Fischl    | OK (Lv100 drift noted) | OK | OK | OK | PASS |
| Flins     | OK (Lv100 drift noted) | OK | OK | OK | PASS |
| Iansan    | OK (Lv100 drift noted) | OK | OK | OK | PASS |
| Ineffa    | OK (Lv100 drift noted) | OK | FAIL | FAIL | FAIL |
| Keqing    | OK (Lv100 drift noted) | OK | OK | OK | PASS |
| Kujou Sara | FAIL (scrambled from Lv20 onward) | OK | OK | OK | FAIL |
| Kuki Shinobu | FAIL (scrambled from Lv20 onward) | OK | OK | OK | FAIL |
| Lisa      | OK (Lv100 drift noted) | OK | OK | FAIL | FAIL |
| Ororon    | FAIL (scrambled from Lv20 onward) | OK | OK | OK | FAIL |
| Raiden Shogun | FAIL (scrambled from Lv20 onward) | OK | OK | OK | FAIL |
| Razor     | OK (Lv100 drift noted) | OK | OK | OK | PASS |
| Sethos    | OK (Lv100 drift noted) | OK | OK | OK | PASS |
| Varesa    | OK (Lv100 drift noted) | OK | OK | OK | PASS |
| Yae Miko  | FAIL (scrambled from Lv20 onward) | OK | OK | OK | FAIL |

**PASS: 10 characters | FAIL: 8 characters**

Note: `Lv100 drift noted` means Lv1 through Lv90+ match the mirror and only the extrapolated Lv100 row differs. Existing audit docs treat Lv100-only extrapolation drift as out of scope, so these are not counted as failures here.

---

## Detailed Results

### Lv100-Only Base Stat Drift (Out of Scope)

These files differ from the mirror only in the Lv100 base stat row in the comparison I ran. This is noted for completeness but not counted as a character failure.

| Character | Rust / Mirror |
|-----------|---------------|
| Beidou (`beidou.rs` / `beidou_024.md`) | Lv100 base stats differ |
| Clorinde (`clorinde.rs` / `clorinde_098.md`) | Lv100 base stats differ |
| Cyno (`cyno.rs` / `cyno_071.md`) | Lv100 base stats differ |
| Fischl (`fischl.rs` / `fischl_031.md`) | Lv100 base stats differ |
| Flins (`flins.rs` / `flins_120.md`) | Lv100 base stats differ |
| Iansan (`iansan.rs` / `iansan_110.md`) | Lv100 base stats differ |
| Keqing (`keqing.rs` / `keqing_042.md`) | Lv100 base stats differ |
| Razor (`razor.rs` / `razor_020.md`) | Lv100 base stats differ |
| Sethos (`sethos.rs` / `sethos_097.md`) | Lv100 base stats differ |
| Varesa (`varesa.rs` / `varesa_111.md`) | Lv100 base stats differ |

---

### Scrambled Base Arrays

The following files have incorrect base HP/ATK/DEF arrays starting at index 1. In each case, Lv1 ATK/DEF are already off, and the later rows are clearly shifted/scrambled versus the mirror.

| Character | Evidence |
|-----------|----------|
| Kujou Sara (`kujou_sara.rs` / `sara_056.md`) | Lv1 ATK 16.00 vs 16.38; Lv1 DEF 53.00 vs 52.65; index 1 onward is wrong |
| Kuki Shinobu (`kuki_shinobu.rs` / `shinobu_065.md`) | Lv1 ATK 18.00 vs 17.81; Lv1 DEF 63.00 vs 62.95; index 1 onward is wrong |
| Ororon (`ororon.rs` / `olorun_105.md`) | Lv1 ATK 20.00 vs 20.48; Lv1 DEF 49.00 vs 49.21; index 1 onward is wrong |
| Raiden Shogun (`raiden_shogun.rs` / `shougun_052.md`) | Lv1 ATK 26.00 vs 26.25; Lv1 DEF 61.00 vs 61.45; index 1 onward is wrong |
| Yae Miko (`yae_miko.rs` / `yae_058.md`) | Lv1 ATK 26.00 vs 26.44; Lv1 DEF 44.00 vs 44.27; index 1 onward is wrong |

**Implementation status:** Fixed in this branch.

---

### Ineffa

**Status: FAIL**

**Weapon Type: FAIL**
- Rust: `WeaponType::Catalyst`
- Mirror: `Polearm`
- Impact: the character is classified under the wrong weapon family, so normal attack behavior is fundamentally wrong.

**Normal Attack: FAIL**
- Rust uses a Catalyst-style normal attack template.
- Mirror expects a Polearm-style combo.
- This is a structural mismatch, not just a naming issue.

**Base Stats: OK (Lv100 drift noted)**
- Lv1 through Lv90+ match the mirror.
- Lv100 base stats differ from the mirror.

**Talent Scalings: OK**
- Skill and burst damage rows match the mirror values within tolerance.

Refs: `crates/data/src/characters/electro/ineffa.rs`, `honeyhunter-mirror/md/characters/ineffa_116.md`

**Implementation status:** Fixed in this branch.

---

### Lisa

**Status: FAIL**

**Base Stats: OK (Lv100 drift noted)**
- Lv1 through Lv90+ match the mirror.
- Lv100 base stats differ from the mirror.

**Elemental Skill: FAIL**
- Rust only has two hold-state scalings: `LISA_SKILL_PRESS` and `LISA_SKILL_HOLD_3STACK`.
- Mirror has four damage rows for the hold state: non-conductive hold, stack 1, stack 2, and stack 3.
- The 0/1/2-stack hold damage rows are missing from Rust.

**Talent values that do exist match**
- The press DMG and 3-stack hold DMG values themselves match the mirror.

Refs: `crates/data/src/characters/electro/lisa.rs`, `honeyhunter-mirror/md/characters/lisa_006.md`

**Implementation status:** Fixed in this branch.

---

### Dori

**Status: FAIL**

**Base Stats: OK (Lv100 drift noted)**
- Lv1 through Lv90+ match the mirror.
- Lv100 base stats differ from the mirror.

**Talent Scalings: FAIL (minor rounding)**
- Normal Attack 3-Hit: Rust `128.36%` vs mirror `128.4%`
- Elemental Burst connector hit: Rust `15.92%` vs mirror `15.88%`
- Both diffs are small, but they exceed the configured tolerance.

Refs: `crates/data/src/characters/electro/dori.rs`, `honeyhunter-mirror/md/characters/dori_068.md`

**Implementation status:** Fixed in this branch.
