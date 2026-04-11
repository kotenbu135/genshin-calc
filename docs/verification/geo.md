# Geo Character Verification

Verified against: `honeyhunter-mirror/md/characters/`
Scope: 倍率、スキル効果、バフ/デバフ、固有天賦、凸効果（追加ダメージ系・元素チャージ系は除外）
Excluded: 追加ダメージ（flat追加ヒット系）、元素チャージ、クールタイム短縮、シールド量/回復量、スタミナ、継続時間

## Summary

| Character | Scaling | Passives | Constellations | Status |
|-----------|---------|----------|----------------|--------|
| Albedo | OK | OK（除外項目あり） | OK | PASS |
| Chiori | OK | OK | OK | PASS |
| Gorou | OK | **MISSING** (A1 DEF+25%) | **SPEC GAP** (C6 Geo限定会心ダメージを全体会心ダメとして実装) | FAIL |
| Illuga | OK（倍率定義は存在） | **SPEC GAP** (A4未実装コメントのみ) | 部分実装 | FAIL |
| Itto | OK | OK | OK | PASS |
| Kachina | OK | OK | OK | PASS |
| Navia | OK | OK | OK（対象範囲内） | PASS |
| Ningguang | OK | OK | OK（対象範囲内） | PASS |
| Noelle | OK | OK（除外項目あり） | OK（対象範囲内） | PASS |
| Xilonen | OK | **MISSING** (A4 DEF+20%) | OK（対象範囲内） | FAIL |
| Yun Jin | OK | **MISSING** (A4 Breaking Conventions) | OK（対象範囲内） | FAIL |
| Zhongli | OK | OK（除外項目あり） | OK（対象範囲内） | PASS |
| Zibai | OK（対象範囲） | OK（対象範囲） | OK（対象範囲） | PASS |

**Bug / Spec-gap count: 5**
- Missing buffs: 3 (Gorou A1, Yun Jin A4, Xilonen A4)
- Implementation approximation gap: 2 (Gorou C6, Illuga A4)

---

## Issues

### 1) Gorou A1「風雨を恐れず」(DEF+25%) が未実装

- Mirror: `honeyhunter-mirror/md/characters/gorou_055.md`
  - `After using Juuga: Forward Unto Victory, all nearby party members' DEF is increased by 25% for 12s.`
- 現状 `crates/data/src/talent_buffs/geo.rs` の `GOROU_BUFFS` には A1 の `DefPercent +0.25` が存在しない。
- 影響: ゴロー編成の防御参照キャラ（雲菫/ノエル/荒瀧など）で最終ダメージが過小評価される。

### 2) Yun Jin A4「独立嶄然」未実装

- Mirror: `honeyhunter-mirror/md/characters/yunjin_064.md`
  - `Flying Cloud Flag Formation` の通常攻撃ダメージ加算を、編成元素種類に応じてさらに `2.5/5/7.5/11.5% DEF` 上乗せ。
- 現状 `crates/data/src/talent_buffs/geo.rs` の `YUN_JIN_BUFFS` は Burst本体/C2/C4のみで、A4追加係数が未実装。
- 影響: 雲菫の主用途（通常攻撃サポート）の中核バフが不足。

### 3) Xilonen A4「Portable Armored Sheath」DEF+20% 未実装

- Mirror: `honeyhunter-mirror/md/characters/xilonen_103.md`
  - `when nearby party members trigger a Nightsoul Burst, Xilonen's DEF is increased by 20% for 15s.`
- 現状 `crates/data/src/talent_buffs/geo.rs` の `XILONEN_BUFFS` には A4由来の `DefPercent +0.20 (OnlySelf)` が存在しない。
- 影響: シロネン自身DEF参照（スキル由来耐性デバフ運用やC4加算系計算）で乖離。

### 4) Gorou C6 は「岩元素ダメージの会心ダメージ」なのに、実装は全会心ダメージとして扱う近似

- Mirror: `honeyhunter-mirror/md/characters/gorou_055.md`
  - C6は **Geo DMG のみ** に対する CRIT DMG +10/20/40%。
- 現状実装: `BuffableStat::CritDmg` で全属性に適用される形。
- 影響: 岩元素以外の会心ダメージにも誤って上乗せされる可能性。

### 5) Illuga A4 が未実装（コードコメントのみ）

- Mirror: `honeyhunter-mirror/md/characters/illuga_127.md`
  - A4は編成条件に応じた Geo DMG 系強化効果。
- 現状 `crates/data/src/talent_buffs/geo.rs` に `TODO` コメントのみで有効な `TalentBuffDef` 不在。
- 影響: イルーガ運用時の主要パッシブが反映されない。

---

## Character-by-character Notes (対象範囲のみ)

### Albedo
- 倍率テーブル: 実装値とmirror記載の主要倍率は一致。
- A4 (EM+125) 実装済み。
- C4/C6 実装済み（対象範囲内）。

### Chiori
- 倍率テーブル: 実装値とmirror記載は一致。
- A4 (Geo DMG+20%) 実装済み。
- C6通常攻撃フラット加算（DEF参照）実装済み。

### Gorou
- 倍率テーブル: 通常/スキル/爆発値は一致。
- Skill DEF固定値スケーリング、Geo DMG+15% は実装あり。
- A1 DEF+25% 未実装（Issue #1）。
- C6の属性限定条件が近似化（Issue #4）。

### Illuga
- 倍率テーブル: キャラデータ側には定義あり。
- A1（CR/CD/EM）実装済み。
- A4未実装（Issue #5）。

### Itto
- 倍率テーブル: 実装値はmirror記載と整合。
- A4（重撃DEF参照加算）実装済み。
- C4/C6 実装済み。

### Kachina
- 倍率テーブル: 実装値は整合。
- A4 Geo DMG+20% 実装済み。
- C4 DEFバフ（最大値）実装済み。

### Navia
- 倍率テーブル: 対象倍率は整合。
- A1通常/重撃/落下 +40% 実装済み。
- A4攻撃力バフ（最大40%）実装済み。
- C2/C4 実装済み。

### Ningguang
- 倍率テーブル: 実装値は整合。
- A4「璇璣屏通過で岩ダメ+12%」実装済み。

### Noelle
- 倍率テーブル: 通常/スキル/爆発の主要値は整合。
- Burst DEF→ATK変換 実装済み。
- C2/C6 実装済み。

### Xilonen
- 倍率テーブル: 主要倍率および耐性低下スケールは整合。
- C2/C4 実装済み（雷効果は対象外）。
- A4 DEF+20% 未実装（Issue #3）。

### Yun Jin
- 倍率テーブル: スキル・爆発係数は整合。
- Burst通常攻撃加算、C2/C4 実装済み。
- A4 `Breaking Conventions` 未実装（Issue #2）。

### Zhongli
- 倍率テーブル: 実装値は整合。
- Jade Shield全耐性-20%（元素/物理）実装済み。

### Zibai
- 対象範囲（バフ/デバフ・凸）に関して、実装との明確な不一致は確認されず。

---

## Suggested Fixes (shortlist)

1. `GOROU_BUFFS` に A1 DEF+25% (`BuffableStat::DefPercent`, Team, A1) を追加。  
2. `YUN_JIN_BUFFS` に A4追加係数（編成元素種類条件つき）を追加。  
3. `XILONEN_BUFFS` に A4 DEF+20% (`OnlySelf`, A4, 条件付きトグル可) を追加。  
4. Gorou C6は Geo限定会心ダメージとして扱えるstat拡張、または明示的な「仕様近似」注記を追加。  
5. Illuga A4をTODO解除し、専用メカニクスまたは近似実装を追加。

