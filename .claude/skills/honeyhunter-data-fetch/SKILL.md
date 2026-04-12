---
name: honeyhunter-data-fetch
description: Fetch Genshin Impact game data from local Honey Hunter mirror and convert to Rust const definitions
version: 3.0.0
source: local-mirror
shared: ../_shared/data-pipeline.md
---

# Honey Impact データ取得スキル

ローカルミラーのMDファイルからゲーム数値データを読み取り、Rust const定義に変換する。

**共通規約**: `_shared/data-pipeline.md` 参照（フォーマット・ファイル配置・元素判定・Pitfalls）

## Trigger

- TODOに記載のデータ実装時
- 「Honeyhunterからデータ取得して」「キャラデータ追加して」指示時
- `/game-update-todo` 生成TODOの実装時

---

## MDファイル構造

**キャラ** (`honeyhunter-mirror/md/characters/{name}_{id}.md`):
- `## Basic Info` — 名前、レアリティ、武器種、元素、星座名
- `## Base Stats` — レベル別HP/ATK/DEF/CritRate/CritDMG/突破ボーナス
- `## Normal Attack: {name}` — 通常攻撃説明 + 倍率テーブル(Lv1-15)
- `## Elemental Skill: {name}` — スキル説明 + 倍率テーブル(Lv1-15)
- `## Elemental Burst: {name}` — 爆発説明 + 倍率テーブル(Lv1-15)
- `## Passive Talents` — パッシブ天賦一覧
- `## Constellations` — C1-C6の効果テキスト

**武器** (`honeyhunter-mirror/md/weapons/i_n{id}.md`):
- `## Basic Info` — 名前、レアリティ、武器種、基礎攻撃力、サブステ
- `## Weapon Affix` — R1-R5精錬効果テキスト
- `## Stats` — レベル別ATK/サブステ

---

## カテゴリ別ワークフロー

### キャラクター追加

1. **Read**: `honeyhunter-mirror/md/characters/{name}_{id}.md`
2. **抽出**: 基本情報、base_hp/atk/def(4値)、突破ステ、通常/重撃/落下/スキル/爆発の各段Lv1-15倍率、星座パターン
3. **出力**: `crates/data/src/characters/{element}/{char}.rs` (新規)
4. **mod登録**: `mod`宣言 + `pub use` + `CHARACTERS`スライス追加
5. **テスト**: `crates/core/tests/data/characters/{id}.toml`
6. **天賦バフ**: `talent_buffs/{element}.rs` (A1/A4/星座にステバフがある場合)
7. **月兆**: `moonsign_chars.rs` (月兆対応キャラの場合)

### 武器追加

1. **Read**: `honeyhunter-mirror/md/weapons/i_n{id}.md`
2. **抽出**: 名前、武器種、★、base_atk(4値)、sub_stat(種類+4値)、パッシブ名、R1-R5効果値
3. **出力**: `crates/data/src/weapons/{weapon_type}.rs` に追記
4. **パッシブ分類**: `_shared/data-pipeline.md` のConditionalBuff分類に従う
5. **登録**: `weapons/mod.rs` → `ALL_WEAPONS`

### 聖遺物追加

1. ミラーにデータなし → Fandom Wikiから取得
2. **出力**: `crates/data/src/artifacts.rs` → `ALL_ARTIFACT_SETS`

### 敵追加

1. ミラーにデータ少数 → Fandom Wikiで補完
2. **出力**: `crates/data/src/enemies.rs` → `ALL_ENEMIES`
3. 既存テンプレート再利用優先: `ALL_10`, `PHYS_50_ELEM_10` 等
