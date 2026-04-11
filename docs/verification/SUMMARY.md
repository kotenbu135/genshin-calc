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
