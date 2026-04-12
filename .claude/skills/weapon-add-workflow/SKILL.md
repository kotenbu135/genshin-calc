---
name: weapon-add-workflow
description: End-to-end execution skill for adding new Genshin Impact weapons — fetches data, classifies passives, dispatches subagents in a worktree, verifies, and merges.
version: 2.0.0
shared: ../_shared/data-pipeline.md
---

# Weapon Add Workflow

武器追加をワンショットで実行するスキル。

**共通規約**: `_shared/data-pipeline.md` 参照（フォーマット・ファイル配置・ConditionalBuff分類・テンプレート・Pitfalls）

## Trigger

- 「武器追加」「{名前}を実装」「新武器データ」言及時
- `/game-update-todo` 生成TODOの武器関連タスク実装時

## キャラ追加との違い

| 項目 | キャラ | 武器 |
|------|--------|------|
| ファイル | 個別 .rs | 武器種別 .rs に追記 |
| テスト | 個別 TOML | data_integrity テストが自動検証 |
| 天賦バフ | talent_buffs/{element}.rs | conditional_buffs (武器定義内) |
| バッチ | 1体ずつ | **複数本を1 worktreeで実行可** |

---

## Flow

```
Phase 1: Data Fetch + Passive Classification
  ├── 武器一覧確認 (TODOから対象把握)
  ├── honeyhunter-mirror MDファイル読み取り
  ├── パッシブ分類判定 (SKIP/STATIC/CONDITIONAL)
  └── スコープ提示 → ユーザー確認

Phase 2: Worktree + Subagent Dispatch
  ├── git worktree 作成
  ├── 武器種別ごとにsubagent dispatch (1 subagent = 同一.rsの全武器)
  └── mod.rs ALL_WEAPONS登録

Phase 3: Verify + Merge
  ├── cargo build + test + clippy + fmt
  ├── TODO更新
  └── mainにマージ + worktree削除
```

---

## Phase 1: Data Fetch + Passive Classification

### 1-1. データ取得
`honeyhunter-mirror/md/weapons/i_n{id}.md` をReadで取得。

抽出対象: 名前、武器種、★、base_atk[4値]、sub_stat(種類+4値)、パッシブ名+効果テキスト、R1-R5数値

### 1-2. パッシブ分類
`_shared/data-pipeline.md` のConditionalBuff分類・Activationパターンに従う。

### 1-3. スコープ提示

```
## 武器追加スコープ ({N}本)
| # | 武器名 | 種類/★ | sub_stat | パッシブ分類 |
|---|--------|--------|----------|------------|
| 1 | {name} | {type}/★{n} | {stat} | STATIC/CONDITIONAL/SKIP |

CONDITIONAL武器の詳細:
- {weapon}: {条件} → {Activation パターン}
```

---

## Phase 2: Worktree + Subagent Dispatch

### 2-0. Worktree 作成
```bash
git worktree add .claude/worktrees/feat-weapons-v{version} -b feat/weapons-v{version}
```

### 2-1. 武器種別ごとのSubagent
同一.rsファイルの武器は1 subagentにまとめる。

**subagentに渡す情報:** Worktreeパス、武器リスト(全データ+パッシブ分類結果+Activationパターン)、該当.rsの既存パターン

**subagentの責務:**
1. `{weapon_type}.rs` に武器定義追加
2. `mod.rs` の `ALL_WEAPONS` に追加
3. `cargo build -p genshin-calc-data` + `cargo test -p genshin-calc-data`
4. コミット

### 2-2. mod.rs登録戦略

**Option A (推奨):** 各subagentが自分の武器をALL_WEAPONSに追加 — 順次実行、安全確実
**Option B:** 最後に1 subagentがまとめて更新 — 並列可能だが登録忘れリスク

武器数が少ない場合はOption A推奨。

---

## Phase 3: Verify + Merge

```bash
cd {worktree_path}
cargo build && cargo test && cargo clippy -- -D warnings && cargo fmt --check
```

確認項目: `all_weapons_have_positive_base_atk`, `all_weapons_have_unique_ids`, `test_all_conditional_buff_names_unique`, `test_all_stacks_max_positive`

TODO更新 → ユーザー確認後マージ:
```bash
cd /home/sakis/genshin-calc
git merge feat/weapons-v{version} --no-ff
git worktree remove .claude/worktrees/feat-weapons-v{version}
git branch -d feat/weapons-v{version}
```

---

## Related Skills

| スキル | 役割 |
|--------|------|
| `honeyhunter-data-fetch` | MD構造・抽出ワークフロー |
| `character-add-workflow` | キャラ追加スキル |
| `game-update-todo` | バージョン全体のTODO生成 |
