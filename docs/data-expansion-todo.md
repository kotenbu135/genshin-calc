# データ拡張 TODO

genshin-calc-data crateのデータカバレッジ拡充ロードマップ。

## 現状サマリ (v5.8)

| カテゴリ | 総数 | 実装済み | 未実装 | カバレッジ |
|----------|-----:|--------:|-------:|-----------:|
| 武器パッシブ (StatBuff) | 220 | 220 | 0 | 100% |
| 武器パッシブ (ConditionalBuff) | 220 | 77 | 143 | 35% |
| 聖遺物2pc (StatBuff) | 52 | 45 | 7 | 87% |
| 聖遺物4pc効果 | 52 | 52 | 0 | 100% |
| 天賦バフ/デバフ (TalentBuffDef) | 29キャラ | 47定義 | 1(Nilou) | ~97% |
| 敵データ | 多数 | 40 | 多数 | - |
| 武器精錬値 (R1-R5) | 220 | 220 | 0 | 100% |
| 敵側デバフ (P6) | 5種 | 5種 | 0 | 100% |

## ボトルネック: 条件付き効果の表現力不足

P0で `ConditionalBuff` システムを導入済み。
聖遺物4pc効果は全52セット実装完了。武器パッシブは74本にConditionalBuff実装済み（5星武器Batch 1-4 + 4星武器28本）、残り146本が未対応。

### 表現できない効果の例

- **HP閾値**: 護摩の杖「HP50%以下でATK+1.0% of Max HP」
- **元素反応トリガー**: 燃え盛る炎の魔女4pc「蒸発・溶解反応倍率+15%」
- **スタック効果**: 渡火4pc「烈晶反応後8秒間炎ダメ+40%、3スタック」
- **武器種限定**: 剣闘士4pc「片手剣・両手剣・長柄キャラのみ通常攻撃+35%」
- **敵側デバフ**: Zhongliシールド「全元素耐性・物理耐性-20%」
- **EM依存バフ**: Kazuha A4「EM×0.04%の元素ダメバフ」(現在はbuilder側で手動計算)

---

## P0: 条件付き効果システムの設計 ✅

武器・聖遺物・天賦の条件付き効果を統一的に表現できるデータ構造を設計する。
P2/P3/P4/P6のアンロックキー。

- [x] 条件付きバフ型の設計 (`ConditionalBuff` + `AutoCondition`/`ManualCondition`/`Activation` enum)
- [x] 条件の種類を網羅: `WeaponTypeRequired`, `ElementRequired`, `StatScaling`, `TeamElementCount`, `TeamElementsOnly`, `Toggle`, `Stacks`
- [x] core/data crateの責務分離を維持した設計
- [x] WASM互換性の維持（動的ディスパッチ最小化）
- [x] イミュータブル設計の維持
- [x] 影響範囲の実装: data (`buff.rs`, `artifacts.rs`, `team_builder.rs`)

## P1: 天賦バフ拡充 (既存枠内) ✅

現在の `TalentBuffDef` で表現可能なサポートキャラを追加する。29キャラ・47定義実装済み。

### P1で即追加可能 (条件なし or builder側計算)

- [x] Sucrose — EM共有 (A1: +50 EM固定, A4: チームにEM×20%, base_value方式)
- [x] Faruzan — 風ダメバフ部分のみ (風耐性シュレッドはP6)
- [x] Ganyu — 氷ダメ+20% (A4)
- [x] Albedo — EM+125 (A4)
- [x] Ningguang — 岩ダメ+12% (A4)
- [x] Traveler Dendro — EM+60 (A4)
- [x] Yoimiya — チームATK+20% (A4)
- [x] Chevreuse — ATK+20% (A1固定バフ部分のみ)
- [x] Diona — EM+200 (C6)
- [x] Amber — チームATK% (C6)
- [x] Barbara — 水ダメ (C2)
- [x] Shenhe — 氷ダメ倍率 (A1 press: SkillDmgBonus+BurstDmgBonus+15%)
- [x] Thoma — 通常/重撃/落下+15% (C6)
- [x] Candace — 通常攻撃ダメ+スケーリング (爆発)
- [x] Gorou — DEFフラット+Geo DMG+15% (スキル+岩3体)
- [x] Yelan — DMGボーナス (base_value方式)
- [x] Ineffa — EM (base_value方式)
- [x] Jahoda — EM+100 (スキル)
- [x] Aino — EM+80 (C1)

### P0完了後に追加可能

- [ ] Nilou — 開花反応ボーナス (水草限定条件)
- [x] Lisa — DEF-15%デバフ (A4) → P6で実装済み
- [x] Zhongli — 全耐性-20% (シールド) → P6で実装済み (8 TalentBuffDef)
- [x] Chevreuse — 炎/雷耐性-40% → P6で実装済み (2 TalentBuffDef追加)
- [x] Faruzan — 風耐性-30%シュレッド → P6で実装済み

### 対象外 (ダメージ計算へのバフが少ない)

- [x] Kuki Shinobu — スキップ
- [x] Chiori — スキップ

## P2: 聖遺物セット効果の充実 ✅

4pc効果は全52セット実装完了。2pcは45/52実装済み。

### 2pc効果 (単純バフ)

- [ ] 未実装の2pc効果を洗い出して埋める (大半は無条件バフ、既に高カバレッジ)

### 4pc効果 (条件付き — P0依存)

- [x] 剣闘士のフィナーレ — 通常攻撃+35% (武器種限定: 片手剣/両手剣/長柄)
- [x] 大地を流浪する楽団 — 重撃+35% (武器種限定: 法器/弓)
- [x] 燃え盛る炎の魔女 — 蒸発/溶解+15%, 過負荷等+40% + スタック (反応トリガー)
- [x] しめ縄の記憶 — 通常/重撃/落下+50% (スキル使用後)
- [x] 絶縁の旗印 — 爆発ダメ+25%(ER×) (ER依存)
- [x] 金メッキの夢 — EM/ATK% (チーム元素依存、TeamSameElementCount/TeamDiffElementCount)
- [x] 花海甘露の光 — スキル/爆発+10%ベース + +8%/stack×5 (HP変動スタック)
- [x] 全52セットの4pc効果実装完了 (ConditionalBuff×35セット, description-only×13, 4pc効果なし×4)

## P3: 武器パッシブの充実

StatBuffは全220本実装完了。ConditionalBuffは77/220本実装済み（35%）。

### 無条件パッシブ (StatBuff)

- [x] 全220本のStatBuff実装完了 (100%)

### 条件付きパッシブ — 初期実装 (P0直後)

- [x] 磐岩結緑 — HP上限の1.2-2.4%分ATKアップ (Auto StatScaling)
- [x] 護摩の杖 — HP上限基準ATKアップ (常時) + HP<50%時追加ATK (Both)
- [x] 狼の末路 — チームATK+40-80% (HP<30%時, Manual Toggle)
- [x] 鉄刺 — 元素DMG命中後DMG+6-12% (Stacks(2))
- [x] 蛇骨大剣 — DMG+6-10%スタック (Stacks(5))
- [x] 白辰の輪 — ATK/DEF+6-12%スタック (Stacks(4))
- [x] 死闘の槍 — ATK/DEF+16-32%または ATK+24-48% (Toggle)
- [x] 滅竜 — 水/炎影響敵へDMG+20-36% (Toggle)
- [x] 幽水のワルツ — 付近に敵がいる時DMG+20-40% (Toggle)
- [x] 獅子の咆哮 — 炎/雷影響敵へDMG+20-36% (Toggle)
- [x] 辿-失路の典籍 — 元素DMG+8-16%スタック (Stacks(4))
- [x] 万国諸海の図 — 元素DMG+8-16%スタック (Stacks(2))

### 条件付きパッシブ — P3 Batch 1-4 (5星武器拡充)

- [x] **Batch 1** (StatScaling拡張) — 5星武器8本 (#15)
- [x] **Batch 2** (Stacks) — 5星武器9本 (#16)
- [x] **Batch 3** (Toggle/TeamComp) — 5星武器13本 (#17)
- [x] **Batch 4** (Shield Series) — 4本 (#19)

### 条件付きパッシブ — P3 4星武器 ✅

- [x] 4星武器28本のConditionalBuff実装完了
  - Pattern A: Toggle (single buff) — 17本
  - Pattern B: Toggle (multiple buffs) — 7本
  - Pattern C: Auto(StatScaling) — 2本
  - Pattern D: Stacks (non-linear) — 1本
  - Pattern E: StatBuff + Toggle hybrid — 1本

### 複雑な武器 ✅

- [x] 霧切の廻光 — 非線形Stacks(3)、stack_values R1絶対値
- [x] 千岩古剣/千岩長槍 — TeamRegionCount(Liyue)、Auto完全自動評価
- [x] 赤砂の杖 — 二次効果 Both(StatScaling(EM), Stacks(2))
- [x] 草薙の稲光 — 二次効果 Manual(Toggle) ER+30-50%

## P4: 武器精錬値 (R1-R5) ✅

`refinement_values: Option<[f64; 5]>` フィールドにR1-R5の値を入力する。

- [x] 星5武器のR1-R5値を入力 (実装済みの37本すべて)
- [x] 星4武器のR1-R5値を入力 (実装済みの37本すべて)
- [x] TeamMemberBuilder で精錬レベルを受け取り、対応する値を使用するよう修正 (#8で実施済み)

## P5: 敵データ拡充 ✅

既存の枠組みで即追加可能（耐性テンプレート + `EnemyData` を追加するだけ）。全データ実装済み。

### 週ボス

- [x] 風魔龍トワリン
- [x] タルタリヤ (第1〜3段階)
- [x] 若陀龍王
- [x] シニョーラ (第1〜2段階)
- [x] 雷電将軍 (鍛錬)
- [x] 散兵 (第1〜2段階)
- [x] アペプ (第1〜3段階)
- [x] ナルヴァレット (第1〜2段階)

### フィールドボス

- [x] 雷レジスヴァイン
- [x] スキップ (水レジスヴァイン — ゲーム内に存在しない)
- [x] スキップ (草レジスヴァイン — ゲーム内に存在しない)
- [x] マグーケンキ
- [x] 氷キューブ (無相の氷)
- [x] 雷キューブ (無相の雷)
- [x] 草キューブ (無相の草)

### 精鋭

- [x] アビスの使徒
- [x] アビスの詠唱者
- [x] 遺跡ドレイク (飛空・陸行)
- [x] エルマイト旅団・精鋭

## P6: 敵側デバフシステム ✅

core crateに `apply_enemy_debuffs(enemy, buffs, element) -> Enemy` 関数を実装済み。
`BuffableStat` による判別で耐性減少・DEF減少を適用。`BuffableStat::DefReduction` 追加済み。
`superconduct_debuff()` ヘルパー関数も提供。

- [x] Consumer側実装: `apply_enemy_debuffs` で ResolvedBuff の耐性減少/DEF減少を Enemy に適用
- [x] 超電導 — 物理耐性-40% (`superconduct_debuff()` ヘルパー)
- [x] Zhongli シールド — 全耐性-20% (8 TalentBuffDef)
- [x] Chevreuse — 炎/雷耐性-40% (2 TalentBuffDef追加)
- [x] Lisa A4 — DEF-15% (BuffableStat::DefReduction)
- [x] Faruzan — 風耐性-30% (固定値、天賦レベル非依存)

---

## 優先度マトリクス

```
影響度 高 │  ✅ P0 条件付き効果   ✅ P2 聖遺物4pc
          │  ✅ P1 天賦バフ        P3 武器パッシブ (77/220)
          │
          │
影響度 低 │  ✅ P5 敵データ        ✅ P4 精錬値
          │                        ✅ P6 敵側デバフ
          └──────────────────────────────────
            実装コスト 低 ──────── 実装コスト 高
```

## 推奨実装順序

1. ~~**P5 敵データ**~~ ✅ 完了
2. ~~**P1 天賦バフ**~~ ✅ 完了 (29キャラ・47定義)
3. ~~**P0 条件付き効果**~~ ✅ 完了 (ConditionalBuff + AutoCondition/ManualCondition)
4. ~~**P2 聖遺物4pc**~~ ✅ 完了 (全52セット)
5. **P3 武器パッシブ** — 5星Batch 1-4 + 4星28本 + 複雑武器4本完了 (77/220)
6. ~~**P4 精錬値**~~ ✅ 完了 (全220本R1-R5)
7. ~~**P6 敵側デバフ**~~ ✅ 完了 (超電導/Zhongli/Chevreuse/Lisa/Faruzan)

### 残タスク

- **P1 Nilou** — 開花反応ボーナス (水草限定条件、特殊実装が必要)
- **P2 2pc効果** — 残り7セットのStatBuff
