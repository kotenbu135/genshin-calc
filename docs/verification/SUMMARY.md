# キャラクター仕様検証 総合バグレポート

検証日: 2026-04-12  
対象: 全110キャラクター（7元素）  
除外: 追加ダメージ（proc）、元素チャージ、HP回復、スタミナ、継続時間、クールタイム

---

## 深刻度別バグ一覧

### CRITICAL — ダメージ計算が大幅に誤る

| キャラ | 問題 | ファイル |
|--------|------|---------|
| Noelle | base_hp/atk/def 全配列破損（Lv80値がindex1に入っている） | `characters/geo/noelle.rs` |
| Noelle | 重撃倍率がNaviaのデータをコピー（全15レベル間違い） | `characters/geo/noelle.rs` |
| Zhongli | 爆発倍率 Lv2以降すべて間違い（Lv15で-3.5の誤差） | `characters/geo/zhongli.rs` |
| Chiori | スキル/爆発のDEFスケーリング成分が完全欠落（約50%過小評価） | `characters/geo/chiori.rs` |
| Raiden Shogun | スキル「天眼」の爆発ダメージボーナス倍率 Lv1以外全レベル間違い | `talent_buffs/electro.rs` |
| Dehya | 爆発の最大HP参照部分が完全欠落（ATK分のみ実装） | `characters/pyro/dehya.rs` |
| Kuki Shinobu | 爆発の scaling_stat が Atk → 正しくは Hp | `characters/electro/kuki_shinobu.rs` |
| Beidou | 通常攻撃全段 Lv11-15 の値が間違い（mirrorより低い） | `characters/electro/beidou.rs` |

### HIGH — 特定条件/レベルで誤差が大きい

| キャラ | 問題 | ファイル |
|--------|------|---------|
| Yun Jin | 重撃倍率が全15レベル間違い（Lv15で-0.43） | `characters/geo/yun_jin.rs` |
| Xilonen | 突破ステータス DEF 28.8% → 正しくは 36.0% | `characters/geo/xilonen.rs` |
| Faruzan | 爆発「嵐の目」の風元素ダメージボーナス倍率 全15レベル間違い（Wikiから転記） | `talent_buffs/anemo.rs` |
| Xiangling | 3段ダメージ(2) の倍率が全15レベル間違い（N4の値を誤使用） | `characters/pyro/xiangling.rs` |
| Thoma | 3段ダメージ(2) の倍率が全15レベル間違い（N4の値を誤使用） | `characters/pyro/thoma.rs` |
| Mavuika | 通常攻撃構造が間違い（N2B/N3が欠落・ずれている） | `characters/pyro/mavuika.rs` |
| Xinyan | 爆発メインヒットが炎元素 → 正しくは物理 | `characters/pyro/xinyan.rs` |
| Raiden Shogun | 御一閃 N4B の値が N4A と同一（全15レベルで第2ヒットが僅かに高いはず） | `characters/electro/raiden_shogun.rs` |
| Yae Miko | 神社3段目 Lv15: 2.2150 → 正しくは 2.2515（桁転置） | `characters/electro/yae_miko.rs` |
| Cyno | 通常攻撃 N3 Lv10: 1.5883 → 正しくは ~1.1586 | `characters/electro/cyno.rs` |
| Lisa | スキルホールド3スタック Lv8: 7.9520 → 正しくは 7.7952 | `characters/electro/lisa.rs` |
| Shenhe | 氷引力 Lv2-15 全値が +0.0001 系統誤差 | `talent_buffs/cryo.rs` |
| Qiqi | 通常攻撃 1段 Lv1: 0.3754 → 正しくは 0.3775 | `characters/cryo/qiqi.rs` |
| Jahoda | エイムショット Lv11-15 に5星の値を使用（4星キャラ） | `characters/anemo/jahoda.rs` |
| Candace | スキル (SKILL_PRESS) 全15レベルが Lv1 値(0.1200)に固定 | `characters/hydro/candace.rs` |
| Ayato | スキル3段ダメージ Lv8: 1.0999 → 正しくは 1.1099（桁転置） | `characters/hydro/ayato.rs` |
| Collei | C6バフ「草元素ダメージ+35%」が実際には存在しないバフ（完全に架空） | `talent_buffs/dendro.rs` |
| Illuga | N3 第2ヒットが欠落（mirror: 31.43% + 31.43%） | `characters/geo/illuga.rs` |
| Itto | 左一文字斬りダメージ（Saichimonji Slash）が未定義 | `characters/geo/itto.rs` |
| Traveler (Dendro) | 重撃 第2ヒット欠落（55.9% + 60.72% のうち60.72%分） | `characters/dendro/traveler_dendro.rs` |

### MEDIUM — 凸パターン逆転（C3↔C5レベルアップ先が逆）

凸パターン `C3BurstC5Skill` と `C3SkillC5Burst` が逆になっているキャラ:

**Geo (5件):** Zhongli, Noelle, Gorou, Itto, Navia  
**Electro (5件):** Fischl, Beidou, Yae Miko, Ineffa, Varesa（逆）  
**Cryo (6件):** Kaeya, Shenhe, Layla, Citlali, Escoffier, Skirk（逆）  
**Dendro (4件):** Collei, Nahida, Yaoyao, Alhaitham  
**Pyro (2件):** Arlecchino, Chevreuse  
**Anemo:** なし（Varka等は確認済み）  
**Hydro:** なし  

計 **22キャラ**で凸パターンが逆。

**ConstellationPattern enum で表現不可（C3=通常攻撃）:**
- Freminet, Wriothesley (C3=通常攻撃+3、enum不対応)
- Sethos (C3=通常攻撃+3)
- Arlecchino (C3=通常攻撃+3 が正確)
- Varesa (C5=通常攻撃+3)

### 未実装バフ（実装すべきで未実装）

| キャラ | 効果 | ソース |
|--------|------|--------|
| Nahida | A4「Awakening Elucidated」EM200超過分→スキルDMG+0.1%・会心率+0.03% | `talent_buffs/dendro.rs` |
| Kirara | A4「Pupillary Variance」HP参照スキル/爆発DMG加算 | `talent_buffs/dendro.rs` |
| Gorou | A4「Heedless of the Wind and Weather」DEF+25%（チーム） | `talent_buffs/geo.rs` |
| Yun Jin | A4「Breaking Conventions」通常攻撃加算 追加係数（編成元素種類依存） | `talent_buffs/geo.rs` |
| Xilonen | A4「Portable Armored Sheath」DEF+20%（自己） | `talent_buffs/geo.rs` |
| Illuga | A4 未実装（TODOコメントのみ） | `talent_buffs/geo.rs` |
| Shenhe | A1「Deific Embrace」氷元素ダメージ+15% | `talent_buffs/cryo.rs` |
| Mika | A1「Suppressive Barrage」物理ダメージ+10%/スタック（最大3） | `talent_buffs/cryo.rs` |
| Xiao | A1「Conqueror of Evil: Tamer of Demons」爆発中全ダメージ+5%→最大+25% | `talent_buffs/anemo.rs` |
| Varka | C6 Azure Fang's Oath スタック毎会心ダメージ+20%（最大+80%） | `talent_buffs/anemo.rs` |
| Xingqiu | A4「Blades Amidst Raindrops」水元素ダメージ+20% | `talent_buffs/hydro.rs` |
| Candace | A4「Celestial Dome of Sand」HP1000毎に通常攻撃元素ダメージ+0.5% | `talent_buffs/hydro.rs` |
| Beidou | A4「Lightning Storm」通常/重撃ダメージ+15% | `talent_buffs/electro.rs` |
| Razor | C4「Bite」スキル命中時DEF-15% | `talent_buffs/electro.rs` |
| Raiden Shogun | A4「Enlightened One」ER100%超過分×0.4%→雷元素ダメージ | `talent_buffs/electro.rs` |
| Cyno | A1「Featherfall Judgment」Endseer状態中スキルDMG+35% | `talent_buffs/electro.rs` |

### その他

| キャラ | 問題 |
|--------|------|
| Xiangling | C1・C6バフが talent_buffs に重複定義（二重計上リスク） |
| Kirara | C6 DmgBonus（物理含む）→ ElementalDmgBonus が正しい |
| Nefer | C6 TransformativeBonus → ReactionDmgBonus(LunarBloom) が正確 |
| Gorou | C6 会心ダメージが全属性適用 → 岩元素限定が正しい（enum未対応） |
| Shenhe | C2 会心ダメージが全属性適用 → 氷元素限定が正しい（enum未対応） |
| Kujou Sara | 爆発ATKボーナス倍率 Lv15: 1.0206 → 1.0203（微小） |
| Zibai | スキル「隙を過ぐる月の4段目追加ダメージ」DEF参照が未定義 |
| Xianyun | A4 フラットダメージ加算をDMGボーナス(+75%)として近似（要調査） |
| Nahida | A1 チーム最高EMを自己EMで近似（設計上の簡略化） |

---

## 統計

| 元素 | CRITICALバグ | HIGHバグ | 凸パターン誤 | 未実装バフ | 総問題数 |
|------|-------------|---------|------------|----------|--------|
| Geo | 4 | 3 | 5 | 4 | 21 |
| Electro | 3 | 5 | 5 | 4 | 18 |
| Pyro | 2 | 4 | 2 | 0 | 10 |
| Cryo | 0 | 2 | 6+2 | 2 | 12 |
| Dendro | 1 | 2 | 4 | 2 | 10 |
| Hydro | 0 | 2 | 0 | 2 | 6 |
| Anemo | 0 | 2 | 0 | 2 | 6 |
| **合計** | **10** | **20** | **24** | **16** | **83** |

---

## 修正優先度

1. **即修正** (CRITICAL + HIGH 倍率バグ): Noelle, Zhongli, Chiori, Raiden Shogun, Dehya, Kuki Shinobu, Beidou, Yun Jin, Faruzan, Xiangling, Thoma, Mavuika, Xinyan, Candace
2. **重要バフ実装**: Nahida A4, Gorou A4, Xiao A1, Shenhe A1
3. **凸パターン一括修正**: 22キャラ（機械的置換で対応可能）
4. **未実装バフ残り**: Kirara A4, Yun Jin A4, Xilonen A4, Mika A1, etc.
5. **Enum拡張要検討**: 元素限定CritDmg、通常攻撃C3パターン

詳細: `docs/verification/{element}.md`

---

## Issue分割案（適切なタスクサイズ）

> 方針: **1 Issue = 1〜4ファイル程度 / 1〜3キャラ程度 / テスト追加込みで半日〜1.5日** を目安に分割。  
> 依存関係の強いもの（enum拡張など）は独立Issue化し、先に着手する。

### Epic A: CRITICAL/HIGH 直接計算バグ修正（優先度: 最優先）

#### Issue A-1: Noelle 基礎ステータス配列破損 + 重撃倍率誤データ修正
- 対象: Noelle
- 修正箇所:
  - `characters/geo/noelle.rs`（base_hp/atk/def 配列）
  - `characters/geo/noelle.rs`（重撃倍率テーブル）
- 完了条件:
  - HoneyHunter mirror値と全レベル一致
  - 回帰テスト追加（基礎値 + 重撃倍率）

#### Issue A-2: Zhongli 爆発倍率テーブル全面修正 + C3/C5パターン確認
- 対象: Zhongli
- 修正箇所:
  - `characters/geo/zhongli.rs`
- 完了条件:
  - Lv1-15 爆発倍率一致
  - 凸パターンも同時に正しいことをテストで検証

#### Issue A-3: Chiori DEFスケーリング欠落修正
- 対象: Chiori
- 修正箇所:
  - `characters/geo/chiori.rs`
- 完了条件:
  - スキル/爆発のATK+DEF複合スケールを再現
  - `applied_buffs` と `final_stats` を明示検証するテスト追加

#### Issue A-4: Raiden 将軍（天眼バフ倍率誤り + 御一閃N4B誤値）修正
- 対象: Raiden Shogun
- 修正箇所:
  - `talent_buffs/electro.rs`
  - `characters/electro/raiden_shogun.rs`
- 完了条件:
  - 天眼バフ倍率Lv1-15一致
  - 御一閃 N4B の独立値を反映

#### Issue A-5: Dehya/Kuki 爆発スケーリング統合修正（HP参照系）
- 対象: Dehya, Kuki Shinobu
- 修正箇所:
  - `characters/pyro/dehya.rs`
  - `characters/electro/kuki_shinobu.rs`
- 完了条件:
  - Dehya 爆発のHP参照成分を追加
  - Kuki 爆発 `scaling_stat` を HP に修正
  - 2キャラ分のデータ駆動テスト更新

#### Issue A-6: 通常攻撃連撃テーブル誤参照修正（Xiangling/Thoma/Beidou）
- 対象: Xiangling, Thoma, Beidou
- 修正箇所:
  - `characters/pyro/xiangling.rs`
  - `characters/pyro/thoma.rs`
  - `characters/electro/beidou.rs`
- 完了条件:
  - N段倍率の誤参照（N4流用等）解消
  - Lv11-15を含め全レベル一致

### Epic B: 構造誤り・欠落実装（優先度: 高）

#### Issue B-1: Mavuika 通常攻撃構造の再構築
- 対象: Mavuika
- 修正箇所: `characters/pyro/mavuika.rs`
- 完了条件:
  - N2B/N3の欠落を補完しヒット構造を正規化
  - ヒット数/倍率の一致テストを追加

#### Issue B-2: Illuga/Traveler/Itto ヒット欠落・未定義倍率修正
- 対象: Illuga, Traveler (Dendro), Itto
- 修正箇所:
  - `characters/geo/illuga.rs`
  - `characters/dendro/traveler_dendro.rs`
  - `characters/geo/itto.rs`
- 完了条件:
  - 欠落ヒット追加（Illuga N3-2, 旅人重撃2段目）
  - Itto 左一文字斬り定義追加

#### Issue B-3: 元素種別ミス修正（Xinyan爆発物理化）
- 対象: Xinyan
- 修正箇所: `characters/pyro/xinyan.rs`
- 完了条件:
  - 爆発メインヒットの元素種別が物理であることをテストで保証

### Epic C: 凸パターンの機械的修正（優先度: 高）

#### Issue C-1: C3/C5逆転 22キャラ一括修正（6元素）
- 対象: 22キャラ（SUMMARY記載の逆転対象）
- 修正箇所:
  - 各 `characters/*/*.rs` の `constellation_pattern`
- 完了条件:
  - 22キャラ全件の pattern を正規化
  - 一覧テスト（テーブル駆動）を追加し再発防止

#### Issue C-2: `ConstellationPattern` 拡張（通常攻撃+3対応）
- 対象: Freminet, Wriothesley, Sethos, Arlecchino, Varesa
- 修正箇所:
  - `crates/core` 側 enum / 分岐ロジック
  - 該当キャラ定義
- 完了条件:
  - C3/C5=通常攻撃+3 を正しく表現可能
  - 後方互換性テスト通過

### Epic D: 未実装バフをドメイン単位で実装（優先度: 中）

#### Issue D-1: Geo/Dendro 未実装パッシブ実装
- 対象: Nahida A4, Kirara A4, Gorou A4, Yun Jin A4, Xilonen A4, Illuga A4
- 修正箇所:
  - `talent_buffs/dendro.rs`
  - `talent_buffs/geo.rs`
- 完了条件:
  - 全効果が `applied_buffs` / `final_stats` で検証される
  - TODOコメントのみ状態を解消

#### Issue D-2: Cryo/Anemo/Hydro/Electro 未実装パッシブ実装
- 対象: Shenhe A1, Mika A1, Xiao A1, Varka C6, Xingqiu A4, Candace A4, Beidou A4, Razor C4, Raiden A4, Cyno A1
- 修正箇所:
  - `talent_buffs/cryo.rs`
  - `talent_buffs/anemo.rs`
  - `talent_buffs/hydro.rs`
  - `talent_buffs/electro.rs`
- 完了条件:
  - すべての未実装効果を個別テスト付きで反映

### Epic E: 設計課題/技術的負債（優先度: 中）

#### Issue E-1: 元素限定CritDmgバフを表現する型拡張
- 対象: Gorou C6, Shenhe C2 など
- 修正箇所: `crates/core` のバフ表現型・適用ロジック
- 完了条件:
  - 全属性適用の誤近似を撤廃し元素限定で計算できる
  - 既存バフの破壊的影響がないことをテストで担保

#### Issue E-2: 重複定義/近似実装の棚卸し修正
- 対象:
  - Xiangling C1/C6 重複定義
  - Kirara C6 DmgBonus型
  - Nefer C6 Reaction系ボーナス型
  - Xianyun A4 近似実装
  - Nahida A1 近似実装
- 完了条件:
  - 「仕様バグ」と「設計上の近似」をIssue本文で明確分離
  - 修正範囲が明示された状態で実装

---

## Issue作成テンプレート（GitHub）

各Issueは以下テンプレートで作成する。

```md
## 背景
- 検証レポート: docs/verification/SUMMARY.md
- 対象: <キャラ/モジュール>

## やること
- [ ] mirror値を確認し実装値を修正
- [ ] 単体テスト追加（倍率・バフ適用）
- [ ] 必要に応じて統合テスト追加
- [ ] ドキュメント更新（必要時）

## 受け入れ条件
- [ ] `cargo test -p genshin-calc-core` が通る
- [ ] 影響範囲のキャラで回帰なし
- [ ] 未実装/近似の場合は理由をコードコメントに残す
```
