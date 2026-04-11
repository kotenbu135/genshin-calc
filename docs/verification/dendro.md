# Dendro Character Verification

Verified against: `honeyhunter-mirror/md/characters/`
Scope: 倍率、スキル効果、バフ/デバフ、固有天賦、凸効果（追加ダメージ・元素チャージ系は除外）
Excluded: 追加ダメージ、元素チャージ、クールタイム短縮、回復量/シールド量、スタミナ

## Summary

| Character | Scaling | Passives | Constellations | Status |
|-----------|---------|----------|----------------|--------|
| Alhaitham | OK | OK | OK | PASS |
| Baizhu | OK | OK（対象範囲） | OK（対象範囲） | PASS |
| Collei | OK | OK（対象範囲） | OK（対象範囲） | PASS |
| Emilie | OK | OK | OK | PASS |
| Kaveh | OK | OK（対象範囲） | OK（対象範囲） | PASS |
| Kinich | OK | OK（対象範囲） | OK（対象範囲） | PASS |
| Kirara | OK | **MISSING** (A4) | OK | FAIL |
| Lauma | OK | OK | OK | PASS |
| Nahida | OK | **SPEC GAP** (A1は最高EM参照→実装は自己EM近似) | OK（対象範囲） | FAIL |
| Nefer | OK | OK（対象範囲） | OK（対象範囲） | PASS |
| Tighnari | OK | OK | OK | PASS |
| Traveler (Dendro) | OK | OK（対象範囲） | OK（対象範囲） | PASS |
| Yaoyao | OK | OK（対象範囲） | OK（対象範囲） | PASS |

**Bug / Spec-gap count: 2**
- Missing implementation: 1 (Kirara A4)
- Approximation gap: 1 (Nahida A1 highest party EM simplification)

---

## Issues

### 1) Kirara A4 が未実装（TODOコメントのみ）

- 対象コード: `crates/data/src/talent_buffs/dendro.rs`
  - `KIRARA_BUFFS` セクションで A4 が `TODO` としてスキップされている。
- Mirror仕様: `honeyhunter-mirror/md/characters/momoka_061.md`
  - A4はHP基準でスキル/爆発系に影響する効果。
- 影響: 綺良々のA4由来火力寄与が計算に入らない。

### 2) Nahida A1 は仕様上「チーム最高EM」参照だが、実装は自己EMベース近似

- 対象コード: `crates/data/src/talent_buffs/dendro.rs`
  - `Compassion Illuminated` のコメントで、実装が単体評価時の近似であることを明示。
- Mirror仕様: `honeyhunter-mirror/md/characters/nahida_073.md`
  - A1は「領域内キャラに、チーム内最高元素熟知の25%（上限250）」。
- 影響: ナヒーダ非場/他キャラEM主軸編成で配布EMの差異が出る可能性。

---

## Character-by-character Notes (対象範囲のみ)

### Alhaitham
- A4 (EM→DMG 0.1%/最大100%)、C2/C4/C6のバフ定義は実装済み。

### Baizhu
- A4（反応別係数: 燃焼/開花/超開花/烈開花/超激化/草激化/月開花）が実装済み。
- C4 EM+80 実装済み。

### Collei
- C4 EM配布（自分以外）、C6草ダメ+35% 実装済み。

### Emilie
- A4（総攻撃力依存DMG%）、C1/C2 実装済み。

### Kaveh
- A4 EMスタック、Burst中の開花ダメージ補正、C4開花+60% 実装済み。

### Kinich
- C2草耐性-30% / 自己DMG+100%、C4爆発+70% 実装済み。

### Kirara
- C6全元素ダメ+12% 実装済み。
- A4は未実装（Issue #1）。

### Lauma
- A4（EM依存スキルDMG%）およびC2/C6（月開花ダメージ）実装済み。

### Nahida
- C2/C4は実装済み。
- A1は仕様簡略化（Issue #2）。

### Nefer
- C2/C4/C6（対象範囲内）実装済み。

### Tighnari
- A1/A4、C1/C2/C4のバフ実装済み。

### Traveler (Dendro)
- A4 EM+60、C6草ダメ+12% 実装済み。

### Yaoyao
- C1草ダメ配布、C4 HP依存EM（上限120）実装済み。

---

## Suggested Fixes (shortlist)

1. `KIRARA_BUFFS` のA4 TODOを実装し、HP依存のスキル/爆発寄与を反映する。  
2. Nahida A1はチーム最高EM参照に寄せる（TeamBuilder文脈で最高EM入力、または近似である旨をより明示）。

