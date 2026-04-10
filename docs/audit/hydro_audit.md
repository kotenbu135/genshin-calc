# Hydro Characters Audit

**Date:** 2026-04-08
**Scope:** Talent scaling values (Lv1–Lv15), base HP/ATK/DEF arrays, ascension stat
**Tolerance:** 0.0002 (0.02%) absolute difference in decimal form
**Exclusions:** Healing rows, CD rows, stamina cost rows, 追加ダメージ (proc/additional damage)

---

## Summary

| Category | Count |
|----------|-------|
| Total characters audited | 16 |
| Characters with discrepancies | 3 |
| Characters all clear | 13 |
| Critical errors (wrong weapon / asc stat) | 2 (Aino, Dahlia) |
| Major value errors | 1 (Aino) |
| Minor value errors | 1 (Mualani) |
| Missing scaling rows | 1 (Tartaglia) |

---

## Discrepancies

### AINO (`aino.rs` ↔ `aino_121.md`)

**CRITICAL — Multiple structural and value errors**

#### Normal Attack

| Hit | Rust (Lv1) | Mirror (Lv1) | Diff |
|-----|-----------|-------------|------|
| Normal 1 | 70.32% | 66.50% | **+3.82%** |
| Normal 2 | 65.88% | 66.19% | **−0.31%** |
| Normal 3 | 79.20% (single hit) | 49.22% × 2 hits | **Wrong structure** |
| Normal 4 | 96.36% (present) | *(no 4th hit)* | **Extra hit in Rust** |

Mirror has 3-hit normal attack. Rust has 4-hit normal attack with wrong values.

#### Charged Attack

| | Rust (Lv1) | Mirror (Lv1) |
|-|-----------|-------------|
| Structure | Single scaling: 127.80% | Loop DMG: 62.52% + Final DMG: 113.09% |

Wrong structure (Rust uses single-hit, mirror has 2-part charged attack) and wrong values.

#### Elemental Skill

| | Rust (Lv1) | Mirror (Lv1) |
|-|-----------|-------------|
| Structure | Single scaling: 168.00% | Stage 1: 65.60%, Stage 2: 188.80% |

Wrong structure (single vs 2-stage) and wrong values.

#### Elemental Burst

| | Rust (Lv1) | Mirror (Lv1) | Diff |
|-|-----------|-------------|------|
| Water Ball DMG | 201.60% | 20.11% | **10× discrepancy** |

Rust value is approximately 10× too large (201.6% vs 20.11%).

**Implementation status:** Fixed in this branch.

---

### DAHLIA (`dahlia.rs` ↔ `dahlia_115.md`)

**CRITICAL — Wrong weapon type and ascension stat**

| Field | Rust | Mirror |
|-------|------|--------|
| Weapon type | `Bow` | `Sword` |
| Normal attack | Bow-style (aimed shots) | "Favonius Bladework - Ritual" (sword-style, 4-hit) |
| Ascension stat | `CritDmg(0.384)` = 38.4% | Bonus HP% (24% max) |

The entire normal attack structure is wrong because the weapon type is Bow instead of Sword. Skill DMG (232.8% Lv1) and Burst DMG (406.4% Lv1) values happen to match the mirror, but the character is fundamentally mis-categorized.

**Implementation status:** Fixed in this branch.

---

### TARTAGLIA (`tartaglia.rs` ↔ `tartaglia_033.md`)

**Minor — Missing scaling row**

| Issue | Details |
|-------|---------|
| Missing: Stance Change DMG | Mirror shows "Stance Change DMG / 状態切替時に与えるダメージ = 72% Lv1" in Elemental Skill table. This row is absent from the Rust `TARTAGLIA_SKILL_*` scalings array. |

Plunge Lv4: Rust = 0.8178, Mirror = 81.77% (diff = 0.01%, within tolerance ✓)

**Implementation status:** Fixed in this branch.

---

### MUALANI (`mualani.rs` ↔ `mualani_102.md`)

**Minor — Single rounding difference**

| Hit | Rust (Lv1) | Mirror (Lv1) | Diff |
|-----|-----------|-------------|------|
| Normal 2 | 44.60% | 44.63% | 0.03% (just outside 0.02% tolerance) |

Likely a rounding artifact in the source data. All other values match exactly.

**Implementation status:** Fixed in this branch.

---

## All Clear

The following 12 characters have all scaling values and base stats matching within tolerance:

| Character | File | Notes |
|-----------|------|-------|
| Ayato | `ayato.rs` ↔ `ayato_066.md` | All values match |
| Barbara | `barbara.rs` ↔ `barbara_014.md` | All values match |
| Candace | `candace.rs` ↔ `candace_072.md` | Base DEF Lv1 diff = 0.01 (rounding only) |
| Columbina | `columbina.rs` ↔ `columbina_125.md` | All values match |
| Furina | `furina.rs` ↔ `furina_089.md` | All values match |
| Kokomi | `kokomi.rs` ↔ `kokomi_054.md` | All values match |
| Mona | `mona.rs` ↔ `mona_041.md` | All values match |
| Neuvillette | `neuvillette.rs` ↔ `neuvillette_087.md` | All values match |
| Nilou | `nilou.rs` ↔ `nilou_070.md` | All values match |
| Sigewinne | `sigewinne.rs` ↔ `sigewinne_095.md` | All values match |
| Xingqiu | `xingqiu.rs` ↔ `xingqiu_025.md` | All values match |
| Yelan | `yelan.rs` ↔ `yelan_060.md` | All values match |
