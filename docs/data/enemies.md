# 敵データ一覧 (v6.4)

実装済み: **40体** / 耐性テンプレート: **25種**

## 耐性テンプレート

| テンプレート | Pyro | Hydro | Electro | Cryo | Dendro | Anemo | Geo | Physical |
|-------------|------|-------|---------|------|--------|-------|-----|----------|
| ALL_10 | 10% | 10% | 10% | 10% | 10% | 10% | 10% | 10% |
| PHYS_30_ELEM_10 | 10% | 10% | 10% | 10% | 10% | 10% | 10% | 30% |
| PHYS_50_ELEM_10 | 10% | 10% | 10% | 10% | 10% | 10% | 10% | 50% |
| PHYS_70_ELEM_10 | 10% | 10% | 10% | 10% | 10% | 10% | 10% | 70% |
| PHYS_MINUS20_ELEM_10 | 10% | 10% | 10% | 10% | 10% | 10% | 10% | -20% |
| PYRO_70_REST_10 | 70% | 10% | 10% | 10% | 10% | 10% | 10% | 10% |
| HYDRO_70_REST_10 | 10% | 70% | 10% | 10% | 10% | 10% | 10% | 10% |
| ELECTRO_70_REST_10 | 10% | 10% | 70% | 10% | 10% | 10% | 10% | 10% |
| CRYO_70_REST_10 | 10% | 10% | 10% | 70% | 10% | 10% | 10% | 10% |
| DENDRO_70_REST_10 | 10% | 10% | 10% | 10% | 70% | 10% | 10% | 10% |
| ANEMO_70_REST_10 | 10% | 10% | 10% | 10% | 10% | 70% | 10% | 10% |
| GEO_70_REST_10 | 10% | 10% | 10% | 10% | 10% | 10% | 70% | 10% |
| CRYO_50_REST_10 | 10% | 10% | 10% | 50% | 10% | 10% | 10% | 10% |
| ELECTRO_50_REST_10 | 10% | 10% | 50% | 10% | 10% | 10% | 10% | 10% |
| HYDRO_50_REST_0 | 0% | 50% | 0% | 0% | 0% | 0% | 0% | 0% |
| ELECTRO_50_REST_0 | 0% | 0% | 50% | 0% | 0% | 0% | 0% | 0% |
| HYDRO_70_ELECTRO_70_REST_0 | 0% | 70% | 70% | 0% | 0% | 0% | 0% | 0% |
| PYRO_70_CRYO_50_REST_10 | 70% | 10% | 10% | 50% | 10% | 10% | 10% | 10% |
| ELECTRO_90_REST_30 | 30% | 30% | 90% | 30% | 30% | 30% | 30% | 30% |
| GEO_70_PHYS_40_REST_10 | 10% | 10% | 10% | 10% | 10% | 10% | 70% | 40% |
| ELECTRO_70_PHYS_30_REST_10 | 10% | 10% | 70% | 10% | 10% | 10% | 10% | 30% |
| CRYO_IMMUNE_PHYS_0_REST_10 | 10% | 10% | 10% | Immune | 10% | 10% | 10% | 0% |
| ELECTRO_IMMUNE_PHYS_0_REST_10 | 10% | 10% | Immune | 10% | 10% | 10% | 10% | 0% |
| DENDRO_IMMUNE_PHYS_0_REST_10 | 10% | 10% | 10% | 10% | Immune | 10% | 10% | 0% |
| DENDRO_50_REST_10 | 10% | 10% | 10% | 10% | 50% | 10% | 10% | 10% |

## 一般敵 (6体)

| Name | ID | Resistance Template |
|------|----|---------------------|
| ヒルチャール | hilichurl | ALL_10 |
| ミタチャール | mitachurl | ALL_10 |
| ラワチャール | lawachurl | ALL_10 |
| トレジャーハンター | treasure_hoarder | ALL_10 |
| ファデュイ先遣隊 | fatui_skirmisher | ALL_10 |
| ファデュイ・デット・エージェント | fatui_agent | PHYS_MINUS20_ELEM_10 |

## 遺跡系 (3体)

| Name | ID | Resistance Template |
|------|----|---------------------|
| 遺跡守衛 | ruin_guard | PHYS_50_ELEM_10 |
| 遺跡ハンター | ruin_hunter | PHYS_50_ELEM_10 |
| 遺跡グレイダー | ruin_grader | PHYS_70_ELEM_10 |

## フィールドボス (8体)

| Name | ID | Resistance Template |
|------|----|---------------------|
| 炎レジスヴァイン | pyro_regisvine | PYRO_70_REST_10 |
| 氷レジスヴァイン | cryo_regisvine | CRYO_70_REST_10 |
| 雷レジスヴァイン | electro_regisvine | ELECTRO_70_PHYS_30_REST_10 |
| マグーケンキ | maguu_kenki | ALL_10 |
| 無相の氷 | cryo_hypostasis | CRYO_IMMUNE_PHYS_0_REST_10 |
| 無相の雷 | electro_hypostasis | ELECTRO_IMMUNE_PHYS_0_REST_10 |
| 無相の草 | dendro_hypostasis | DENDRO_IMMUNE_PHYS_0_REST_10 |
| 雷音の権現 | thunder_manifestation | ELECTRO_70_REST_10 |
| 純水精霊 | hydro_hypostasis | HYDRO_70_REST_10 |
| 岩キューブ | geo_hypostasis | GEO_70_REST_10 |
| 風キューブ | anemo_hypostasis | ANEMO_70_REST_10 |
| Radiant Moongecko | radiant_moongecko | Geo 70% / 他 10% |

## 週ボス (16体)

| Name | ID | Resistance Template |
|------|----|---------------------|
| 風魔龍トワリン | dvalin | ALL_10 |
| タルタリヤ Phase 1 | childe_phase1 | HYDRO_50_REST_0 |
| タルタリヤ Phase 2 | childe_phase2 | ELECTRO_50_REST_0 |
| タルタリヤ Phase 3 | childe_phase3 | HYDRO_70_ELECTRO_70_REST_0 |
| 若陀龍王 | azhdaha | GEO_70_PHYS_40_REST_10 |
| シニョーラ Phase 1 | signora_phase1 | CRYO_50_REST_10 |
| シニョーラ Phase 2 | signora_phase2 | PYRO_70_CRYO_50_REST_10 |
| 雷電将軍 (鍛錬) | raiden_shogun_weekly | ALL_10 |
| 散兵 Phase 1 | scaramouche_phase1 | ELECTRO_50_REST_10 |
| 散兵 Phase 2 | scaramouche_phase2 | ELECTRO_90_REST_30 |
| アペプ Phase 1 | apep_phase1 | DENDRO_70_REST_10 |
| アペプ Phase 2 | apep_phase2 | DENDRO_50_REST_10 |
| アペプ Phase 3 | apep_phase3 | DENDRO_70_REST_10 |
| ナルヴァレット Phase 1 | narwhal_phase1 | HYDRO_70_REST_10 |
| ナルヴァレット Phase 2 | narwhal_phase2 | ELECTRO_70_REST_10 |

## 精鋭 (4体)

| Name | ID | Resistance Template |
|------|----|---------------------|
| アビスの使徒 | abyss_herald | ALL_10 |
| アビスの詠唱者 | abyss_lector | ALL_10 |
| 遺跡ドレイク (飛空) | ruin_drake_skywatch | PHYS_50_ELEM_10 |
| 遺跡ドレイク (陸行) | ruin_drake_earthguard | PHYS_50_ELEM_10 |
| エルマイト旅団・精鋭 | eremite_elite | PHYS_MINUS20_ELEM_10 |
