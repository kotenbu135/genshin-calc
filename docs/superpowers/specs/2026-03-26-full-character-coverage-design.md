# 全キャラ対応 Character Verification 拡張設計

## 概要

既存の15キャラ・31ケースのデータ駆動テストを全プレイアブルキャラ（v5.8時点、約102体）に拡張する。既存インフラ（test_types.rs, character_verification.rs）はそのまま使用し、TOMLデータファイルのみ追加する。

## 方針

- **変更なし**: test_types.rs, character_verification.rs, 既存TOML 15ファイル
- **追加のみ**: 87キャラ分のTOMLファイル + generate_expected.rs への追記
- **データソース**: KQM/Genshin Wiki/Honey Impact から天賦倍率・代表ビルドを取得
- **元素バッチ方式**: 7バッチ（Pyro→Hydro→Cryo→Electro→Dendro→Anemo→Geo）+ 旅人

## テストケース構成ルール

各キャラは1-4ケースで以下のルールに従う:

1. **必須**: 代表天賦の通常ダメージ（type="normal"）1ケース
2. **該当する場合**: そのキャラの元素で可能な反応テスト
   - Pyro: 蒸発(Vaporize) or 溶解(Melt)
   - Hydro: 蒸発(Vaporize, 2.0x)
   - Cryo: 逆溶解(Melt, 1.5x)
   - Electro: 超激化(Aggravate)
   - Dendro: 草激化(Spread)
   - Anemo: 拡散はKazuhaでカバー済み → 通常のみ
   - Geo: 反応なし → 通常のみ
3. **サポートキャラ（Bennett, Barbara等）**: ダメージ用天賦があれば通常ダメージ1ケース
4. **特殊スケーリング**: HP/DEFスケーリングのキャラは `scaling_stat` を適切に設定

## 代表天賦の選定基準

各キャラの代表天賦は以下の優先順位で選ぶ:
1. メインDPSとして最も使う天賦（通常攻撃、重撃、スキル、爆発のうち倍率が高いもの）
2. そのキャラの特徴的な天賦（例: Ganyuの重撃Bloom、Raidenの爆発初撃）
3. KQMガイドで推奨されるメイン火力源

天賦レベルは **Lv10**（crown済み）を標準とし、キャラレベルは **Lv90** を標準とする。

## ステータス設定基準

代表的なビルドを想定:
- **ATKスケーリングDPS**: ATK 1800-2200, CR 60-70%, CD 120-180%, 元素DMG 46.6%
- **HPスケーリング**: HP 30000-60000, CR 50-70%, CD 100-200%
- **DEFスケーリング**: DEF 2000-2500, CR 60-70%, CD 150-180%
- **サポート/ヒーラー**: ATK 1200-1500, CR 50%, CD 100%, 元素DMG 46.6%
- **EM特化（拡散等）**: EM 800-960, 低ATK
- **Enemy**: 標準 Lv90, 10% res, 0 def_red（特殊ケース除く）

## バッチ構成

### Batch 1: Pyro（14体追加）
Amber, Arlecchino, Bennett, Chevreuse, Dehya, Gaming, Klee, Lyney, Mavuika, Thoma, Xiangling, Xinyan, Yanfei, Yoimiya

反応テスト: 蒸発(Pyro trigger 1.5x) or 溶解(Pyro trigger 2.0x)

### Batch 2: Hydro（11体追加）
Barbara, Candace, Dahlia, Furina, Kokomi, Mona, Mualani, Neuvillette, Sigewinne, Tartaglia, Xingqiu

反応テスト: 蒸発(Hydro trigger 2.0x)

### Batch 3: Cryo（16体追加）
Aloy, Ayaka, Charlotte, Chongyun, Citlali, Diona, Escoffier, Eula, Kaeya, Layla, Mika, Qiqi, Rosaria, Shenhe, Skirk, Wriothesley

反応テスト: 逆溶解(Cryo trigger 1.5x)
注: Eulaは物理DPSだが Cryo Skill もあり

### Batch 4: Electro（14体追加）
Beidou, Clorinde, Cyno, Dori, Iansan, Ineffa, Keqing, Kuki Shinobu, Lisa, Ororon, Razor, Kujou Sara, Sethos, Varesa

反応テスト: 超激化(Aggravate)

### Batch 5: Dendro（8体追加）
Alhaitham, Baizhu, Collei, Emilie, Kaveh, Kinich, Kirara, Yaoyao

反応テスト: 草激化(Spread)

### Batch 6: Anemo（13体追加）
Chasca, Faruzan, Heizou, Ifa, Jean, Lan Yan, Lynette, Sayu, Sucrose, Venti, Wanderer, Xianyun, Mizuki

反応テスト: 拡散はKazuhaでカバー済み → 通常ダメージのみ

### Batch 7: Geo（10体追加）+ 旅人
Albedo, Chiori, Gorou, Kachina, Navia, Ningguang, Noelle, Xilonen, Yun Jin, Zhongli
+ Traveler（Dendro版として1体）

反応テスト: なし（Geoは増幅/激化反応不可）

## 推定ケース数

| バッチ | キャラ数 | ケース/キャラ | 推定ケース数 |
|--------|---------|-------------|-------------|
| Batch 1 Pyro | 14 | 2 (通常+蒸発) | 28 |
| Batch 2 Hydro | 11 | 2 (通常+蒸発) | 22 |
| Batch 3 Cryo | 16 | 2 (通常+溶解) | 32 |
| Batch 4 Electro | 14 | 2 (通常+激化) | 28 |
| Batch 5 Dendro | 8 | 2 (通常+激化) | 16 |
| Batch 6 Anemo | 13 | 1 (通常のみ) | 13 |
| Batch 7 Geo+旅人 | 11 | 1 (通常のみ) | 11 |
| **合計** | **87** | | **~150** |

既存31ケースと合わせて **合計約181ケース** 。

## 許容誤差

既存と同じ:
- 通常: `< 0.01`
- 反応ケース: `tolerance = 0.1`
- ゲーム検証: `tolerance = 1.0`

## 成功基準

- 102キャラ全員のTOMLファイルが存在する
- `cargo test --test character_verification` で全ケースパス
- 既存127ユニットテストに影響なし
- `cargo clippy` / `cargo fmt` クリーン

## スコープ外

- test_types.rs / character_verification.rs の変更
- 計算ロジック（src/）の変更
- 既存15キャラのTOMLファイルの変更
