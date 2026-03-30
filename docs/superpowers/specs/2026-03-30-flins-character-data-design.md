# Flins (Electro/Polearm/★5) Character Data Implementation

## Overview

v6.0 (Luna I) の新キャラクター Flins のゲームデータを genshin-calc に実装する。

## Scope

- Flins の CharacterData 定義（electro.rs）
- ALL_CHARACTERS への登録
- 天賦バフ（TalentBuffDef）追加（該当するもの）
- 月兆データ追加（該当する場合）
- TOML テストケース作成

## Data Source

genshin-db-api（genshin-db-api.vercel.app）— 既存キャラと同一ソース。

## Design

### 1. CharacterData 定義

`crates/data/src/characters/electro.rs` に追加。既存 Electro/Polearm キャラ（Cyno 等）パターン準拠。

- `id: "flins"`, `name: "Flins"`
- `element: Electro`, `weapon_type: Polearm`, `rarity: Star5`, `region: NodKrai`
- `base_hp/atk/def`: genshin-db-api から [Lv1, Lv20, Lv80, Lv90] の 4 値取得
- `ascension_stat`: API から確認
- `constellation_pattern`: API から確認

天賦スケーリング:
- 通常/重撃/落下: `damage_element: None`（物理ダメージ — Polearm 共通）
- スキル/爆発: `damage_element: Some(Element::Electro)`
- 全スケーリング `values: [f64; 15]`
- 月兆/スキル/爆発による通常攻撃元素付与がある場合は Cyno パターン準拠（`BURST_NORMAL_*` エントリを追加、`damage_element: Some(Element::Electro)`）。API 調査時に確認すること

### 2. 登録

`crates/data/src/characters/mod.rs` の `ALL_CHARACTERS` Electro セクションに `&electro::FLINS` をアルファベット順で追加（FISCHL の後、IANSAN の前）。カウント 17→18。

### 3. 天賦バフ

`crates/data/src/talent_buffs.rs` に `TalentBuffDef` を追加:
- genshin-db-api から A1/A4 パッシブ、星座 (C1-C6) を調査
- ステータスバフ（ATK/DMG/CRIT 等）を付与するもののみ実装
- proc damage、回復、エネルギー生成等はスキップ（CLAUDE.md 分類ルール準拠）

### 4. 月兆データ

`crates/data/src/moonsign_chars.rs` — API から月兆関連パッシブを確認。
該当する場合のみ追加（ベネディクション + タレント強化）。

### 5. TOML テスト

`crates/core/tests/data/characters/flins.toml` を作成:
- 最低 1 ケース: 通常ダメージ（Physical、ATK スケーリング）
- 可能であればスキル（Electro）のケースも追加
- 手計算で検証可能な値（Lv90、既知ステータス）

## Not In Scope

- 紹介武器（Bloodsoaked Ruins）は TODO 5-2 で別途実装
- 他の v6.0 キャラは別タスク
- docs/data/ ドキュメント更新は全キャラ実装後に一括（CLAUDE.md 規約からの意図的な延期。全 v6.0 キャラ実装後に返済）

## Risks

- **低**: Region::NodKrai は Lauma 実装時に追加済み
- **低**: genshin-db-api にデータがない場合は Honey Hunter 等で補完
