---
name: game-update-todo
description: Generate implementation TODO list when a new Genshin Impact version brings new characters, weapons, artifacts, or enemies
version: 2.0.0
shared: ../_shared/data-pipeline.md
---

# Game Update TODO Generator

ゲーム更新で追加されるコンテンツの実装TODOリストを生成するスキル。

**共通規約**: `_shared/data-pipeline.md` 参照（ファイル配置・ConditionalBuff分類・テンプレート）

## Usage

```
/game-update-todo <version>
/game-update-todo 5.9
/game-update-todo 6.0 キャラ2体 武器3本 聖遺物1セット
```

## Trigger

- 「新バージョン」「ゲーム更新」「v5.9」「新キャラ追加」等の言及時
- `/game-update-todo` コマンド実行時

---

## Workflow

### Step 1: 情報収集（Wiki自動リサーチ）

ユーザーへの質問は不要 — 全てWebから取得する。

#### 1-1. バージョン概要取得
WebSearch `"Genshin Impact {version} new characters weapons"` → 新コンテンツ一覧特定

主要ソース: `genshin-impact.fandom.com` > `ambr.top` > Project Amber

#### 1-2. キャラデータ収集
Wiki WebFetchで抽出: 名前/元素/武器種/★/地域、base stats(Lv1/20/80/90)、突破ステ、天賦構成、命の星座パターン、パッシブ、チームバフ、月兆対応

#### 1-3. 武器データ収集
Wiki WebFetchで抽出: 名前/武器種/★、base_atk(4値)、sub_stat(種類+4値)、パッシブ名+効果、R1-R5

#### 1-4. 聖遺物・敵データ収集
セット名、2pc/4pc効果、敵名、各元素耐性値

#### 1-5. 収集結果提示
```
## v{VERSION} 新コンテンツ確認
### 新キャラクター ({N}体)
- {名前} ({元素}/{武器種}/★{星}/{地域})
### 新武器 ({N}本)
- {名前} ({武器種}/★{星}/サブ:{ステ})
### 新聖遺物 ({N}セット) / 新敵 ({N}体)
この内容でTODOリストを生成しますか？
```

---

### Step 2: TODO生成

以下のテンプレートでチェックリストを生成する。

````markdown
# v{VERSION} データ更新 TODO

## 概要
| カテゴリ | 追加数 |
|----------|--------|
| キャラクター | {N}体 |
| 武器 | {N}本 |
| 聖遺物セット | {N}セット |
| 敵 | {N}体 |

---

## 1. 新キャラクター

### {キャラ名} ({元素}/{武器種}/★{星})

#### 1-1. キャラデータ定義
- [ ] `crates/data/src/characters/{element}/{char}.rs` にconst定義追加
  - id/name/element/weapon_type/rarity/region
  - base_hp/atk/def (4値)、ascension_stat、constellation_pattern
- [ ] 天賦スケーリング定義 (通常N段+重撃+落下, スキルN段, 爆発N段, 各[f64;15])
- [ ] NormalAttackData + TalentData×2 → TalentSet構築

#### 1-2. 登録
- [ ] `characters/{element}/mod.rs`: mod宣言 + pub use + CHARACTERSスライス追加

#### 1-3. テストケース
- [ ] `crates/core/tests/data/characters/{id}.toml` — 最低1ケース(通常ダメージ)

#### 1-4. 天賦バフ（該当キャラのみ）
- [ ] `talent_buffs/{element}.rs` にTalentBuffDef追加

#### 1-5. 月兆データ（該当キャラのみ）
- [ ] `moonsign_chars.rs` にMoonsignBenedictionDef追加

---

## 2. 新武器

### {武器名} ({武器種}/★{星})

#### 2-1. 武器データ定義
- [ ] `weapons/{weapon_type}.rs` にconst定義追加

#### 2-2. パッシブ効果実装
- [ ] 無条件: StatBuff / 条件付き: ConditionalBuff / SKIP: &[]
- [ ] ConditionalBuff判別は `_shared/data-pipeline.md` 参照

#### 2-3. 登録
- [ ] `weapons/mod.rs` → ALL_WEAPONS追加

---

## 3. 新聖遺物セット

### {セット名}
- [ ] `artifacts.rs` にconst定義 (id/name/rarity, 2pc: SetEffect, 4pc: SetEffect)
- [ ] ALL_ARTIFACT_SETS追加

---

## 4. 新敵

### {敵名}
- [ ] `enemies.rs` に追加 (既存ResistanceTemplate再利用 or 新規)
- [ ] ALL_ENEMIES追加

既存テンプレート: ALL_10, PHYS_50_ELEM_10, PHYS_70_ELEM_10, {ELEM}_70_REST_10, ALL_MINUS_10

---

## 5. 検証
- [ ] `cargo build` / `cargo test` / `cargo clippy -- -D warnings` / `cargo fmt --check`

## 6. ドキュメント更新
- [ ] `docs/data-expansion-todo.md` サマリ表更新
- [ ] CLAUDE.md 統計情報更新
````
