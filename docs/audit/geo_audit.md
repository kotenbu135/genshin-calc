# Geo Characters Audit

Audit of all 13 Geo characters in `crates/data/src/characters/geo/*.rs` against `honeyhunter-mirror/md/characters/*.md`.  
Scope: base HP/ATK/DEF arrays, ascension stat, weapon type, element, and damage-related talent scalings/structure.  
Exclusions: non-damage rows, proc/additional damage, healing-only rows, energy, cooldown, stamina cost, 追加ダメージ.  
Tolerance: absolute 0.0002 in decimal form.  
Date: 2026-04-10.

---

## Summary

| Character | Base Stats | Ascension Stat | Weapon/Element | Talent Scalings | Status |
|-----------|-----------|----------------|----------------|-----------------|--------|
| Albedo    | OK        | OK             | OK             | FAIL (Plunge Lv7 typo) | FAIL |
| Chiori    | OK        | OK             | OK             | FAIL (NA 3rd hit, charged low, Plunge typo) | FAIL |
| Gorou     | OK        | OK             | OK             | OK              | PASS |
| Illuga    | OK        | OK             | OK             | OK              | PASS |
| Itto      | OK        | OK             | OK             | OK              | PASS |
| Kachina   | OK        | OK             | OK             | FAIL (NA structure, skill/burst values, Plunge typo) | FAIL |
| Navia     | OK        | OK             | OK             | FAIL (NA 3rd hit collapsed) | FAIL |
| Ningguang | OK        | OK             | OK             | OK              | PASS |
| Noelle    | OK        | OK             | OK             | OK              | PASS |
| Xilonen   | OK        | OK             | OK             | FAIL (NA 2nd hit, charged attack, Plunge typo) | FAIL |
| Yun Jin   | FAIL      | OK             | OK             | FAIL (Plunge typo) | FAIL |
| Zhongli   | OK        | OK             | OK             | FAIL (NA 5th hit collapsed + Plunge typo) | FAIL |
| Zibai     | FAIL      | OK             | OK             | FAIL (multi-hit rows collapsed) | FAIL |

**PASS: 5 characters | FAIL: 8 characters**

Recurring issue: the standard Plunge DMG row is `0.1011` in Rust but `101.1%` in the mirror for Albedo, Chiori, Kachina, Xilonen, Yun Jin, and Zhongli.

---

## Discrepancies

### Albedo (`crates/data/src/characters/geo/albedo.rs` ↔ `honeyhunter-mirror/md/characters/albedo_038.md`)

**Status: FAIL**

The only mismatch is `ALBEDO_PLUNGE`: Rust has `0.1011` at Lv7, while the mirror shows `101.1%`. All other base stats, ascension stat, weapon/element, and damage rows match.

**Implementation status:** Fixed in this branch.

---

### Chiori (`crates/data/src/characters/geo/chiori.rs` ↔ `honeyhunter-mirror/md/characters/chiori_094.md`)

**Status: FAIL**

- `CHIORI_NORMAL_3` is modeled as a single hit (`0.3048`), but the mirror’s 3rd normal row is `30.42% + 30.42%`.
- `CHIORI_CHARGED_1` / `_2` are `54.25%` each in Rust, while the mirror shows `54.31% + 54.31%`.
- `CHIORI_PLUNGE` has the shared `0.1011` typo at Lv7; the mirror shows `101.1%`.
- Base stats, ascension stat, weapon type, element, skill rows, and burst row match.

**Implementation status:** Fixed in this branch.

---

### Kachina (`crates/data/src/characters/geo/kachina.rs` ↔ `honeyhunter-mirror/md/characters/kachina_100.md`)

**Status: FAIL**

- Rust only has 3 normal-hit entries, but the mirror has 4 hit stages.
- The mirror’s 2nd normal stage is `27.57% + 30.63%`, while Rust stores a single `0.4466` entry.
- The mirror’s 3rd normal stage is `70.43%`, but Rust uses `0.6505` and still lacks the 4th stage entirely.
- The mirror’s 4th normal stage is `77.44%`, but Rust has no matching 4th-hit entry.
- `KACHINA_SKILL_TURBO` / `_DASH` are slightly off the mirror values (`87.68%` vs `87.76%`, `64.00%` vs `63.76%`).
- `KACHINA_BURST` is `380.76% DEF` in Rust versus `380.57% DEF` in the mirror.
- `KACHINA_PLUNGE` also has the shared `0.1011` typo at Lv7; the mirror shows `101.1%`.
- Base stats and ascension stat match; the listed skill and burst rows are the ones with value drift.

**Implementation status:** Fixed in this branch.

---

### Navia (`crates/data/src/characters/geo/navia.rs` ↔ `honeyhunter-mirror/md/characters/navia_091.md`)

**Status: FAIL**

- `NAVIA_NORMAL_3` is stored as one hit (`0.3485`), but the mirror’s 3rd normal stage is `34.89%×3`.
- Other normal hits, charged attack, skill rows, burst rows, base stats, and ascension stat match.

**Implementation status:** Fixed in this branch.

---

### Xilonen (`crates/data/src/characters/geo/xilonen.rs` ↔ `honeyhunter-mirror/md/characters/xilonen_103.md`)

**Status: FAIL**

- `XILONEN_NORMAL_2` is stored as a single `0.2741` entry, but the mirror shows `27.37% + 27.37%`.
- `XILONEN_CHARGED_1` / `_2` are `54.13%` each in Rust, but the mirror has a single `91.33%` charged attack row.
- `XILONEN_PLUNGE` has the shared `0.1011` typo at Lv7; the mirror shows `101.1%`.
- Base stats, ascension stat, skill rows, and burst rows match.

**Implementation status:** Fixed in this branch.

---

### Yun Jin (`crates/data/src/characters/geo/yun_jin.rs` ↔ `honeyhunter-mirror/md/characters/yunjin_064.md`)

**Status: FAIL**

- `base_hp`, `base_atk`, and `base_def` are scrambled from index 1 onward.
- Example: Lv20 HP/ATK/DEF are `9445 / 171 / 651` in Rust, but `2296 / 41.17 / 158.18` in the mirror.
- The Lv100 row is also wrong: Rust ends at `11509.56 HP / 206.28 ATK / 792.72 DEF`, while the mirror shows `11395 / 239.93 / 785.19`.
- Talent scalings match aside from the shared plunge typo (`0.1011` vs `101.1%`).

**Implementation status:** Fixed in this branch.

---

### Zhongli (`crates/data/src/characters/geo/zhongli.rs` ↔ `honeyhunter-mirror/md/characters/zhongli_030.md`)

**Status: FAIL**

- `ZHONGLI_NORMAL_5_1` is a single `0.1075` entry, but the mirror’s 5th normal stage is `10.75%×4`.
- `ZHONGLI_PLUNGE` has the shared `0.1011` typo at Lv7; the mirror shows `101.1%`.
- Base stats, ascension stat, weapon type, element, and the other talent rows match.

**Implementation status:** Fixed in this branch.

---

### Zibai (`crates/data/src/characters/geo/zibai.rs` ↔ `honeyhunter-mirror/md/characters/zibai_126.md`)

**Status: FAIL**

- Base stats diverge at the high end. Rust ends at `13952.52 HP / 242.82 ATK / 1033.40 DEF`, while the mirror shows `13838 / 275.41 / 1024.86` at Lv100.
- `ZIBAI_NA_HIT3` collapses a mirrored `34.89% + 34.89%` stage into a single `0.3089` entry.
- `ZIBAI_CHARGED` collapses the mirrored `73.66% + 73.66%` charged attack into a single `0.7366` entry.
- `ZIBAI_SKILL_LPS_HIT3` collapses the mirrored `34.57% + 34.57% DEF` stage into a single entry.
- `ZIBAI_SKILL_LPS_CHARGED` collapses the mirrored `65.95% + 65.95% DEF` charged attack into a single entry.
- Burst rows match. The `4th-hit additional damage` row is out of scope and was not audited.

**Implementation status:** Fixed in this branch.

---

## All Clear

The following characters matched the mirror on the audited fields:

- Gorou (`crates/data/src/characters/geo/gorou.rs` ↔ `honeyhunter-mirror/md/characters/gorou_055.md`)
- Illuga (`crates/data/src/characters/geo/illuga.rs` ↔ `honeyhunter-mirror/md/characters/illuga_127.md`)
- Itto (`crates/data/src/characters/geo/itto.rs` ↔ `honeyhunter-mirror/md/characters/itto_057.md`)
- Ningguang (`crates/data/src/characters/geo/ningguang.rs` ↔ `honeyhunter-mirror/md/characters/ningguang_027.md`)
- Noelle (`crates/data/src/characters/geo/noelle.rs` ↔ `honeyhunter-mirror/md/characters/noel_034.md`)

---

## Uncertainties

- The mirror files do not expose Lv95/Lv95+ rows, so the base-stat audit relies on the displayed keypoints plus the Rust 18-keypoint arrays.
- For multi-hit rows marked `×2`/`×3`/`×4` in the mirror, the Rust model needs explicit split entries or equivalent repetition handling; otherwise the calculator undercounts those hits.
