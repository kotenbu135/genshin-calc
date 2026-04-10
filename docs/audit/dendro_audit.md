# Dendro Characters Audit

Audit of all 13 Dendro characters in `crates/data/src/characters/dendro/*.rs` against `honeyhunter-mirror/md/characters/*.md`.

- Scope: base HP/ATK/DEF arrays, ascension stat, weapon type, element assignment, and damage-relevant talent scalings/structure
- Exclusions: healing-only rows, proc/additional damage (`追加ダメージ`), energy, cooldown, stamina cost
- Tolerance: `0.0002` absolute in decimal form
- Date: 2026-04-10

## Summary

| Character | Base Stats | Ascension Stat | Weapon/Element | Talent Scalings | Status |
|-----------|------------|----------------|----------------|-----------------|--------|
| Alhaitham | FAIL | OK | OK | OK | FAIL |
| Baizhu | OK | OK | OK | OK | PASS |
| Collei | OK | OK | OK | OK | PASS |
| Emilie | OK | OK | OK | OK | PASS |
| Kaveh | OK | OK | OK | OK | PASS |
| Kinich | OK | OK | OK | FAIL | FAIL |
| Kirara | FAIL | OK | OK | FAIL | FAIL |
| Lauma | OK | OK | OK | OK | PASS |
| Nahida | OK | OK | OK | FAIL | FAIL |
| Nefer | OK | OK | OK | OK | PASS |
| Tighnari | OK | OK | OK | OK | PASS |
| Traveler (Dendro) | FAIL | OK | OK | OK | FAIL |
| Yaoyao | OK | OK | OK | OK | PASS |

**PASS: 8 characters | FAIL: 5 characters**

## Detailed Results

### Alhaitham (`alhaitham.rs` / `alhatham_078.md`)

**Status: FAIL**

**Base Stats: FAIL**
- `base_hp` is misordered from Lv20 onward.
- Example: source Lv20 HP is `11777.00`, mirror Lv20 HP is `2695`.
- Lv1 ATK/DEF also miss by rounding and are not just a display artifact:
  - ATK: `24.00` vs `24.39`
  - DEF: `61.00` vs `60.85`

**Talent Scalings: OK**
- Direct damage rows for the normal attack, skill thrust, and burst match the mirror.
- Additional projection-attack rows in the mirror are excluded from this audit.

### Baizhu (`baizhu.rs` / `baizhuer_082.md`)

**Status: PASS**

Base stats, ascension stat, weapon/element, and direct damage rows match within tolerance.

### Collei (`collei.rs` / `collei_067.md`)

**Status: PASS**

Base stats, ascension stat, weapon/element, and direct damage rows match within tolerance.

### Emilie (`emilie.rs` / `emilie_099.md`)

**Status: PASS**

Base stats, ascension stat, weapon/element, and direct damage rows match within tolerance.

### Kaveh (`kaveh.rs` / `kaveh_081.md`)

**Status: PASS**

Base stats, ascension stat, weapon/element, and direct damage rows match within tolerance.

### Kinich (`kinich.rs` / `kinich_101.md`)

**Status: FAIL**

**Talent Scalings: FAIL**
- The mirror includes a direct `Mid-Air Normal Attack DMG` row after the skill-enabled swing.
- Source `KINICH_NA_HITS` only covers the 3 grounded hits and omits that row.
- Mirror reference: `kinich_101.md` normal attack section.

**Base Stats / Ascension / Weapon / Element: OK**

### Kirara (`kirara.rs` / `momoka_061.md`)

**Status: FAIL**

**Base Stats: FAIL**
- Lv1 ATK and DEF are off:
  - ATK: source `19.00`, mirror `18.7`
  - DEF: source `46.00`, mirror `45.78`
- The array is also misaligned from Lv20 onward.
- Example: source Lv20 HP is `10794.00`, mirror Lv20 HP is `2623`.

**Talent Scalings: FAIL**
- Source skill data only has the press kick row (`飛び蹴りダメージ` / `Tail-Flicking Flying Kick DMG`).
- The mirror also has two direct damage rows for hold mode that are missing from source:
  - `Urgent Neko Parcel Hit DMG`
  - `Flipclaw Strike DMG`
- Shield rows in the mirror are excluded from this audit.

### Lauma (`lauma.rs` / `lauma_119.md`)

**Status: PASS**

Base stats, ascension stat, weapon/element, and direct damage rows match within tolerance.

### Nahida (`nahida.rs` / `nahida_073.md`)

**Status: FAIL**

**Talent Scalings: FAIL**
- Charged attack is wrong by a full factor of 2:
  - Source `NAHIDA_CHARGED` = `66.0%`
  - Mirror `Charged Attack DMG` = `132.0%`
- Skill press damage is also off:
  - Source `NAHIDA_SKILL_PRESS` = `103.2%`
  - Mirror `Press DMG` = `98.4%`
- The mirror also has a direct `Hold DMG` row at `130.4%` that is missing from source.
- `Tri-Karma Purification DMG` is excluded as proc/additional damage.

**Base Stats / Ascension / Weapon / Element: OK**

### Nefer (`nefer.rs` / `nefer_122.md`)

**Status: PASS**

Base stats, ascension stat, weapon/element, and direct damage rows match within tolerance.

### Tighnari (`tighnari.rs` / `tighnari_069.md`)

**Status: PASS**

Base stats, ascension stat, weapon/element, and direct damage rows match within tolerance.

### Traveler (Dendro) (`traveler_dendro.rs` / `playerboy_005.md`)

**Status: FAIL**

**Base Stats: FAIL**
- The source array is badly misaligned versus the Dendro Traveler progression in `playerboy_005.md`.
- Example:
  - Lv1 ATK: source `18.00`, mirror `17.81`
  - Lv1 DEF: source `57.00`, mirror `57.23`
  - Lv20 HP: source `9638.00`, mirror `2342`
- `playerboy_005.md` is the correct Dendro Traveler mirror mapping; `playerboy_041.md` is just a generic traveler stub and was not the right file.

**Talent Scalings: OK**

### Yaoyao (`yaoyao.rs` / `yaoyao_077.md`)

**Status: PASS**

Base stats, ascension stat, weapon/element, and direct damage rows match within tolerance.

## Notes

- The only non-obvious mirror mapping was Dendro Traveler: `playerboy_005.md` is the correct file to compare against because it contains the Dendro sections.
- I treated mirror rows for combined ATK/EM scaling as matching their split source rows when the numeric totals aligned.
