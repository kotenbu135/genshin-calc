# P5: 敵データ拡充 設計

## 概要

既存の `ResistanceTemplate` + `EnemyData` パターンで敵データを拡充する。
API変更なし。`crates/data/src/enemies.rs` のみ変更。

## 設計判断

### フェーズ持ちボス
フェーズごとに別 `EnemyData` エントリとして登録する。
例: `CHILDE_PHASE1`, `CHILDE_PHASE2`, `CHILDE_PHASE3`

### 元素免疫（無相系）
自元素の耐性値を `10.0`（1000%）で表現する。
計算上ダメージ倍率がマイナスになり、ゲーム内の0クランプと一致する。
テンプレートに `// Immune` コメントで意味を注記。

### エルマイト旅団
7種とも基本耐性は同一（物理-20%, 元素全10%）のため、1エントリにまとめる。

### 水/草レジスヴァイン
ゲーム内に存在しない。TODOリストの記載は誤りのため、スキップ。

## 既存テンプレートの再利用

以下の既存テンプレートをそのまま使用:

| 既存テンプレート | 用途 |
|---|---|
| `ALL_10` | トワリン、雷電将軍、マグーケンキ、アビスの使徒、アビスの詠唱者 |
| `PHYS_50_ELEM_10` | 遺跡ドレイク（飛空/陸行） |
| `PHYS_MINUS20_ELEM_10` | エルマイト旅団・精鋭 |
| `ELECTRO_70_REST_10` | ナルヴァレットPhase2 |
| `HYDRO_70_REST_10` | ナルヴァレットPhase1 |
| `DENDRO_70_REST_10` | アペプPhase1/3 |

## 追加する耐性テンプレート（13個）

値は全て小数形式（10% → 0.10）。免疫は `10.0`。

| テンプレート名 | Phys | Pyro | Hydro | Electro | Cryo | Dendro | Anemo | Geo | 用途 |
|---|---|---|---|---|---|---|---|---|---|
| `HYDRO_50_REST_0` | 0.0 | 0.0 | 0.50 | 0.0 | 0.0 | 0.0 | 0.0 | 0.0 | タルタリヤPhase1 |
| `ELECTRO_50_REST_0` | 0.0 | 0.0 | 0.0 | 0.50 | 0.0 | 0.0 | 0.0 | 0.0 | タルタリヤPhase2 |
| `HYDRO_70_ELECTRO_70_REST_0` | 0.0 | 0.0 | 0.70 | 0.70 | 0.0 | 0.0 | 0.0 | 0.0 | タルタリヤPhase3 |
| `GEO_70_PHYS_40_REST_10` | 0.40 | 0.10 | 0.10 | 0.10 | 0.10 | 0.10 | 0.10 | 0.70 | 若陀龍王 |
| `CRYO_50_REST_10` | 0.10 | 0.10 | 0.10 | 0.10 | 0.50 | 0.10 | 0.10 | 0.10 | シニョーラPhase1 |
| `PYRO_70_CRYO_50_REST_10` | 0.10 | 0.70 | 0.10 | 0.10 | 0.50 | 0.10 | 0.10 | 0.10 | シニョーラPhase2 |
| `ELECTRO_50_REST_10` | 0.10 | 0.10 | 0.10 | 0.50 | 0.10 | 0.10 | 0.10 | 0.10 | 散兵Phase1 |
| `ELECTRO_90_REST_30` | 0.30 | 0.30 | 0.30 | 0.90 | 0.30 | 0.30 | 0.30 | 0.30 | 散兵Phase2 |
| `DENDRO_50_REST_10` | 0.10 | 0.10 | 0.10 | 0.10 | 0.10 | 0.50 | 0.10 | 0.10 | アペプPhase2 |
| `ELECTRO_70_PHYS_30_REST_10` | 0.30 | 0.10 | 0.10 | 0.70 | 0.10 | 0.10 | 0.10 | 0.10 | 雷レジスヴァイン |
| `CRYO_IMMUNE_PHYS_0_REST_10` | 0.0 | 0.10 | 0.10 | 0.10 | 10.0 | 0.10 | 0.10 | 0.10 | 無相の氷 |
| `ELECTRO_IMMUNE_PHYS_0_REST_10` | 0.0 | 0.10 | 0.10 | 10.0 | 0.10 | 0.10 | 0.10 | 0.10 | 無相の雷 |
| `DENDRO_IMMUNE_PHYS_0_REST_10` | 0.0 | 0.10 | 0.10 | 0.10 | 0.10 | 10.0 | 0.10 | 0.10 | 無相の草 |

## 追加する敵データ

### 週ボス（15エントリ）

| Const名 | ID | 名称 | テンプレート |
|---|---|---|---|
| `DVALIN` | `dvalin` | 風魔龍トワリン | `ALL_10` |
| `CHILDE_PHASE1` | `childe_phase1` | タルタリヤ・第1段階 | `HYDRO_50_REST_0` |
| `CHILDE_PHASE2` | `childe_phase2` | タルタリヤ・第2段階 | `ELECTRO_50_REST_0` |
| `CHILDE_PHASE3` | `childe_phase3` | タルタリヤ・第3段階 | `HYDRO_70_ELECTRO_70_REST_0` |
| `AZHDAHA` | `azhdaha` | 若陀龍王 | `GEO_70_PHYS_40_REST_10` |
| `SIGNORA_PHASE1` | `signora_phase1` | シニョーラ・第1段階 | `CRYO_50_REST_10` |
| `SIGNORA_PHASE2` | `signora_phase2` | シニョーラ・第2段階 | `PYRO_70_CRYO_50_REST_10` |
| `RAIDEN_SHOGUN_WEEKLY` | `raiden_shogun_weekly` | 禍津御建鳴神命 | `ALL_10` |
| `SCARAMOUCHE_PHASE1` | `scaramouche_phase1` | 正機の神・第1段階 | `ELECTRO_50_REST_10` |
| `SCARAMOUCHE_PHASE2` | `scaramouche_phase2` | 正機の神・第2段階 | `ELECTRO_90_REST_30` |
| `APEP_PHASE1` | `apep_phase1` | アペプ・第1段階 | `DENDRO_70_REST_10` |
| `APEP_PHASE2` | `apep_phase2` | アペプ・第2段階 | `DENDRO_50_REST_10` |
| `APEP_PHASE3` | `apep_phase3` | アペプ・第3段階 | `DENDRO_70_REST_10` |
| `NARWHAL_PHASE1` | `narwhal_phase1` | 呑星の鯨・第1段階 | `HYDRO_70_REST_10` |
| `NARWHAL_PHASE2` | `narwhal_phase2` | 呑星の鯨・第2段階 | `ELECTRO_70_REST_10` |

### フィールドボス（5エントリ）

| Const名 | ID | 名称 | テンプレート |
|---|---|---|---|
| `ELECTRO_REGISVINE` | `electro_regisvine` | 雷レジスヴァイン | `ELECTRO_70_PHYS_30_REST_10` |
| `MAGUU_KENKI` | `maguu_kenki` | 魔偶剣鬼 | `ALL_10` |
| `CRYO_HYPOSTASIS` | `cryo_hypostasis` | 無相の氷 | `CRYO_IMMUNE_PHYS_0_REST_10` |
| `ELECTRO_HYPOSTASIS` | `electro_hypostasis` | 無相の雷 | `ELECTRO_IMMUNE_PHYS_0_REST_10` |
| `DENDRO_HYPOSTASIS` | `dendro_hypostasis` | 無相の草 | `DENDRO_IMMUNE_PHYS_0_REST_10` |

### 精鋭（5エントリ）

| Const名 | ID | 名称 | テンプレート |
|---|---|---|---|
| `ABYSS_HERALD` | `abyss_herald` | アビスの使徒 | `ALL_10` |
| `ABYSS_LECTOR` | `abyss_lector` | アビスの詠唱者 | `ALL_10` |
| `RUIN_DRAKE_SKYWATCH` | `ruin_drake_skywatch` | 遺跡ドレイク・飛空 | `PHYS_50_ELEM_10` |
| `RUIN_DRAKE_EARTHGUARD` | `ruin_drake_earthguard` | 遺跡ドレイク・陸行 | `PHYS_50_ELEM_10` |
| `EREMITE_ELITE` | `eremite_elite` | エルマイト旅団・精鋭 | `PHYS_MINUS20_ELEM_10` |

## 合計

- 耐性テンプレート: 12既存 + 13新規 = 25
- 敵データ: 15既存 + 25新規 = 40

## テスト計画

- `test_all_templates_count`: 12 → 25 に更新
- `test_all_enemies_count`: 15 → 40 に更新
- 新テンプレートの耐性値検証（特に免疫テンプレートの `10.0` 値）
- 新敵のID一意性検証（既存 `test_all_enemies_unique_ids` で自動カバー）
- `to_enemy()` 変換テスト: 免疫元素で `10.0` が返ることを確認
- 既存テスト全パス

## 変更ファイル

- `crates/data/src/enemies.rs` — テンプレート追加 + EnemyData追加 + テスト追加/更新
- `ALL_TEMPLATES` 配列に13テンプレート追加
- `ALL_ENEMIES` 配列に25敵追加
