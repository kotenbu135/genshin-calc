# Anemo Characters Audit

Audit of all 17 Anemo characters: talent scalings and base stats.  
Source comparison: `crates/data/src/characters/anemo/*.rs` vs `honeyhunter-mirror/md/characters/*.md`  
Tolerance: absolute 0.0002 in decimal (0.02 percentage points)  
Exclusions: non-damage rows, proc/additional damage (healing, energy, etc.)  
Date: 2026-04-08

---

## Summary

| Character | Base Stats | Ascension Stat | Weapon/Element | Talent Scalings | Status |
|-----------|-----------|----------------|----------------|-----------------|--------|
| Chasca    | OK        | OK             | OK             | OK              | PASS   |
| Faruzan   | OK        | OK             | OK             | OK              | PASS   |
| Heizou    | FAIL      | OK             | OK             | OK              | FAIL   |
| Ifa       | OK        | FAIL           | FAIL           | OK              | FAIL   |
| Jahoda    | OK        | FAIL           | OK             | FAIL            | FAIL   |
| Jean      | FAIL      | FAIL (value)   | OK             | OK              | FAIL   |
| Kazuha    | OK        | OK             | OK             | OK              | PASS   |
| Lan Yan   | FAIL      | OK             | OK             | OK              | FAIL   |
| Lynette   | FAIL      | OK             | OK             | OK              | FAIL   |
| Mizuki    | OK        | FAIL           | OK             | OK              | FAIL   |
| Sayu      | OK        | OK             | OK             | OK              | PASS   |
| Sucrose   | OK        | OK             | OK             | OK              | PASS   |
| Varka     | OK        | OK             | OK             | OK              | PASS   |
| Venti     | OK        | OK             | OK             | OK              | PASS   |
| Wanderer  | OK        | OK             | OK             | OK              | PASS   |
| Xianyun   | FAIL      | OK             | OK             | FAIL (minor)    | FAIL   |
| Xiao      | OK        | OK             | OK             | OK              | PASS   |

**PASS: 8 characters | FAIL: 9 characters**

---

## Detailed Results

### Chasca (chasca.rs / chasca_104.md)

**Status: PASS**

All base stats, ascension stat (CritDMG 38.4%), weapon type, element, and talent scalings verified. No discrepancies found.

---

### Faruzan (faruzan.rs / faruzan_076.md)

**Status: PASS**

All base stats, ascension stat (ATK% 24%), weapon type, element, and talent scalings verified. No discrepancies found.

---

### Heizou (heizou.rs / heizo_059.md)

**Status: FAIL**

**Base Stats: FAIL — Arrays scrambled from index 1 onward**

The `base_hp`, `base_atk`, and `base_def` arrays contain incorrect values from index 1 (Lv20) onward. The values at indices 1+ appear to be taken from Lv80+ stat ranges instead of the proper ascension breakpoints.

Example (HP):
| Index | Lv | Rust | Mirror | Match |
|-------|-----|------|--------|-------|
| 0 | Lv1 | 1039 | 1039 | OK |
| 1 | Lv20 | 9445 | 2672 | FAIL (9445 = ~Lv80+ value) |

All subsequent indices similarly contain wrong values.

**Talent Scalings: OK** — Lv1 values match within tolerance.

---

### Ifa (ifa.rs / ifa_113.md)

**Status: FAIL**

**Weapon Type: FAIL**
- Rust: `WeaponType::Sword`
- Mirror: Catalyst
- Impact: character cannot equip correct weapon type in any calculation

**Normal Attack Element: FAIL**
- Rust: `damage_element: None` (Physical)
- Mirror: Anemo DMG for all normal attacks
- Impact: all normal attack damage uses wrong element

**Ascension Stat: FAIL**
- Rust: `AscensionStat::Atk(0.24)` → 24% ATK
- Mirror: Bonus Elemental Mastery (EM 96 at max ascension)
- Impact: ascension bonus is completely wrong type

**Base Stats: OK** — Lv1 HP/ATK/DEF values match mirror.

**Talent Scalings: OK** — Numerical values match within tolerance. (Note: element assignment per-scaling may be affected by the weapon/element issues above but the numbers themselves are correct.)

---

### Jahoda (jahoda.rs / jahoda_124.md)

**Status: FAIL**

**Ascension Stat: FAIL**
- Rust: `AscensionStat::ElementalMastery(96.0)` → EM 96
- Mirror: Bonus Heal (Healing Bonus 18.46% at max)
- Impact: ascension bonus is completely wrong type

**Normal Attack Scalings: FAIL**
- Rust has 5 normal attack hits; mirror shows a 3-hit combo
- Values do not match:

| Hit | Rust Lv1 | Mirror Lv1 | Match |
|-----|----------|------------|-------|
| 1 | 36.12% | 41.67% | FAIL |
| 2 | 33.54% | 19.23%×2 | FAIL (wrong structure too) |
| 3 | 45.58% | 51.2% | FAIL |
| 4 | 43.00% | — (no 4th hit) | FAIL |
| 5 | 53.75% | — (no 5th hit) | FAIL |

**Aimed Shot / Charged: OK**
- JAHODA_AIMED Lv1: 43.86% = mirror 43.86% ✓
- JAHODA_AIMED_FULL Lv1: 124% = mirror 124% ✓

**Plunging: OK**
- Plunge/Low/High values all match mirror ✓

**Elemental Skill: FAIL**
- Rust has single scaling `JAHODA_SKILL` with Lv1 = 112%
- Mirror shows 4 scalings: Smoke Bomb (159%), Unfilled Flask (190.8%), Filled Flask (212%), Meowball (128%)
- None match 112%

**Elemental Burst: FAIL**
- Rust `JAHODA_BURST` Lv1 = 172.8%
- Mirror Skill DMG Lv1 = 207.2%
- Mismatch: 172.8% vs 207.2%

**Base Stats: OK** — All HP/ATK/DEF values match mirror exactly.

---

### Jean (jean.rs / qin_003.md)

**Status: FAIL**

**Base Stats: FAIL — Arrays scrambled from index 1 onward**

Same scrambled-array pattern as Heizou:
| Index | Lv | Field | Rust | Mirror | Match |
|-------|-----|-------|------|--------|-------|
| 0 | Lv1 | ATK | 19.00 | 18.62 | FAIL |
| 0 | Lv1 | DEF | 60.00 | 59.83 | FAIL |
| 1 | Lv20 | HP | ~14695 | 3211 | FAIL |

(Lv1 HP=1144 matches mirror ✓, but ATK and DEF Lv1 values also differ.)

**Ascension Stat: FAIL (value)**
- Rust: `AscensionStat::HealingBonus(0.222)` → 22.2%
- Mirror: 22.15% Healing Bonus
- Difference: 0.05 percentage points (exceeds 0.02% tolerance)

**Talent Scalings: OK** — Lv1 values match within tolerance.

---

### Kazuha (kazuha.rs / kazuha_047.md)

**Status: PASS**

All base stats, ascension stat (EM 115.2), weapon type, element, and talent scalings verified. No discrepancies found.

---

### Lan Yan (lan_yan.rs / lanyan_108.md)

**Status: FAIL**

**Base Stats: FAIL — Arrays scrambled from index 1 onward**

Same scrambled-array pattern:
| Index | Lv | Field | Rust | Mirror | Match |
|-------|-----|-------|------|--------|-------|
| 0 | Lv1 | HP | 912 | 775 | FAIL |
| 0 | Lv1 | ATK | 20.00 | 21.01 | FAIL |
| 0 | Lv1 | DEF | 57.00 | 48.64 | FAIL |

All three Lv1 values are wrong, and values from index 1 onward contain Lv80+ stat values.

**Talent Scalings: OK** — Values match within tolerance.

---

### Lynette (lynette.rs / linette_083.md)

**Status: FAIL**

**Base Stats: FAIL — Arrays scrambled from index 1 onward**

Same scrambled-array pattern:
| Index | Lv | Field | Rust | Mirror | Match |
|-------|-----|-------|------|--------|-------|
| 0 | Lv1 | HP | 1039 | 1039 | OK |
| 0 | Lv1 | ATK | 19.00 | 19.41 | FAIL |
| 0 | Lv1 | DEF | 60.00 | 59.69 | FAIL |
| 1 | Lv20 | HP | ~12858 | 2672 | FAIL |

**Talent Scalings: OK** — Values match within tolerance.

---

### Mizuki (mizuki.rs / mizuki_109.md)

**Status: FAIL**

**Ascension Stat: FAIL**
- Rust: `AscensionStat::Hp(0.288)` → 28.8% HP bonus
- Mirror: Bonus Elemental Mastery (EM 115.2 at max)
- Impact: ascension bonus is completely wrong type

**Base Stats: OK** — Lv1 HP/ATK/DEF values match mirror.

**Talent Scalings: OK** — Values match within tolerance.

---

### Sayu (sayu.rs / sayu_053.md)

**Status: PASS**

All base stats, ascension stat (EM 96), weapon type, element, and talent scalings verified. No discrepancies found.

---

### Sucrose (sucrose.rs / sucrose_043.md)

**Status: PASS**

All base stats, ascension stat (Anemo DMG Bonus 24%), weapon type, element, and talent scalings verified. No discrepancies found.

---

### Varka (varka.rs / varka_128.md)

**Status: PASS**

All base stats, ascension stat (CritDMG 38.4%), weapon type, element, and talent scalings verified. No discrepancies found.

---

### Venti (venti.rs / venti_022.md)

**Status: PASS**

All base stats, ascension stat (Energy Recharge 32%), weapon type, element, and talent scalings verified. No discrepancies found.

---

### Wanderer (wanderer.rs / wanderer_075.md)

**Status: PASS**

All base stats, ascension stat (CritRate 19.2%), weapon type, element, and talent scalings verified. No discrepancies found.

---

### Xianyun (xianyun.rs / liuyun_093.md)

**Status: FAIL**

**Base Stats: FAIL — Lv1 ATK/DEF off and arrays scrambled from index 1 onward**

| Index | Lv | Field | Rust | Mirror | Match |
|-------|-----|-------|------|--------|-------|
| 0 | Lv1 | ATK | 26.00 | 26.07 | FAIL (diff 0.07) |
| 0 | Lv1 | DEF | 45.00 | 44.57 | FAIL (diff 0.43) |
| 1 | Lv20 | HP | ~13850+ | 2695 | FAIL |

**Talent Scalings: FAIL (minor, exceeds tolerance)**

Normal attack Lv1 values deviate slightly (approximately 0.04 percentage points, exceeding 0.02% tolerance):

Example:
- Rust normal hit 1 Lv1: ~40.34%
- Mirror: 40.3%
- Diff: ~0.04 pp > 0.02% tolerance

**Ascension Stat: OK** — `AscensionStat::Atk(0.288)` = 28.8% matches mirror ✓.

---

### Xiao (xiao.rs / xiao_026.md)

**Status: PASS**

All base stats, ascension stat (CritRate 19.2%), weapon type, element, and talent scalings verified. Burst correctly has empty scalings (buff only, no damage row to check). No discrepancies found.

---

## Issue Categorization

### Critical Issues (wrong type / structure)

1. **Ifa** — `weapon_type`: Sword → should be Catalyst
2. **Ifa** — Normal attack `damage_element`: Physical → should be Anemo
3. **Ifa** — `ascension_stat`: ATK 24% → should be EM 96
4. **Mizuki** — `ascension_stat`: HP% 28.8% → should be EM 115.2
5. **Jahoda** — `ascension_stat`: EM 96 → should be Healing Bonus 18.46%
6. **Jahoda** — Normal attack structure: 5 hits with wrong values → should be 3-hit combo matching mirror
7. **Jahoda** — Elemental skill: single wrong scaling → should be 4 scalings (Smoke Bomb, Unfilled Flask, Filled Flask, Meowball)
8. **Jahoda** — Elemental burst: 172.8% → should be 207.2%

### Base Stat Array Scrambling (systematic bug)

The following 5 characters have base stat arrays where values from index 1 onward are wrong (contain Lv80+ values instead of proper Lv20/Lv40/etc. breakpoint values):

- **Heizou** — all three arrays (HP, ATK, DEF)
- **Jean** — all three arrays + Lv1 ATK/DEF values also slightly off
- **Lan Yan** — all three arrays + Lv1 HP/ATK/DEF values also wrong
- **Lynette** — all three arrays + Lv1 ATK/DEF slightly off
- **Xianyun** — all three arrays + Lv1 ATK/DEF slightly off

### Minor Value Discrepancy

- **Jean** — `ascension_stat` HealingBonus: 22.2% vs mirror 22.15% (diff 0.05 pp)
- **Xianyun** — Normal attack Lv1 scalings off by ~0.04 pp (exceeds 0.02% tolerance)
