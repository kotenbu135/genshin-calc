---
name: character-add-workflow
description: End-to-end execution skill for adding a new Genshin Impact character — fetches data, auto-scopes, dispatches subagents in a worktree, verifies, and merges.
version: 3.0.0
shared: ../_shared/data-pipeline.md
---

# Character Add Workflow v3

キャラクター追加をワンショットで実行するスキル。
brainstorming / writing-plans をスキップし、このスキル単体で完結する。

**共通規約**: `_shared/data-pipeline.md` 参照（フォーマット・ファイル配置・元素判定・テンプレート・Pitfalls）

## Trigger

- 「キャラ追加」「{名前}を実装」「新キャラデータ」言及時
- `/game-update-todo` 生成TODOのキャラ関連タスク実装時

---

## Flow

```
Phase 1: Data Fetch + Scope (このセッション)
  ├── honeyhunter-mirror MDファイル読み取り
  ├── スコープ自動判定 (月兆/バフ/デュアルスケーリング)
  └── ユーザーにスコープ提示 → 確認待ち

Phase 2: Worktree + Subagent Dispatch
  ├── git worktree 作成
  ├── Task A: {char}.rs + mod.rs (honeyhunter-data-fetch規約で自前取得)
  ├── Task B: TOML テスト (dummy → actual パターン, Task A完了後)
  ├── Task C: talent_buffs (該当時のみ, Task A完了後 Task Bと並列)
  └── Task D: moonsign (該当時のみ)

Phase 3: Verify + Merge
  ├── cargo build + test + clippy + fmt
  ├── TODO チェックボックス更新
  └── main にマージ + worktree 削除
```

---

## Phase 1: Data Fetch + Scope

### 1-1. データ取得

`honeyhunter-mirror/md/characters/{name}_{id}.md` をReadで取得。

抽出対象:
- 基本情報 (元素, 武器種, ★, 地域)
- ベースステータス (HP/ATK/DEF × [Lv1, Lv20, Lv80, Lv90])
- 突破ステ (種類 + Lv90値)
- 天賦名 + 各段のscaling_stat・damage_element判定
- パッシブ A1/A4 説明文
- 星座 C1-C6 説明文 (C3/C5でどの天賦が+3か)

**天賦倍率(Lv1-15)はこの段階では不要。** subagentが個別取得。

### 1-2. スコープ自動判定

| 判定項目 | 判定方法 |
|---------|---------|
| 月兆 | `moonsign_chars.rs` にエントリがあるか grep |
| 天賦バフ | A1/A4/C1-C6にステバフがあるか |
| デュアルスケーリング | 天賦説明にATK+DEF等の複数stat参照があるか |
| 星座パターン | C3/C5の記述から判定 |

### 1-3. スコープ提示

```
## {CharName} ({Element}/{Weapon}/★{N}) スコープ
| 項目 | 結果 |
|------|------|
| 月兆 | {実装済み/要実装/対象外} |
| 天賦バフ | {なし / C2: TransformativeBonus +30% 等} |
| デュアルスケーリング | {なし / ATK+DEF 等} |
| 星座パターン | {C3SkillC5Burst / C3BurstC5Skill} |
実装タスク: A(データ定義+登録), B(テスト), {C(バフ)}, {D(月兆)}
```

---

## Phase 2: Worktree + Subagent Dispatch

### 2-0. Worktree 作成
```bash
git worktree add .claude/worktrees/feat-{char_id} -b feat/{char_id}-data
```

### 2-1. Task A: CharacterData + mod登録
1 subagentで実行。honeyhunter-data-fetchのMD構造規約を渡す。

**subagentに渡す情報:** Worktreeパス、基本情報、ベースステータス4値、突破ステ、天賦構成(名前/scaling_stat/damage_element)、星座パターン、同元素の参照ファイルパス、mod.rs内容

**subagentの責務:**
1. MDから天賦倍率Lv1-15取得
2. `{char_id}.rs` 作成
3. `mod.rs` に3箇所追加
4. `cargo build -p genshin-calc-data` + `cargo test -p genshin-calc-data`
5. 2コミット (データ定義 + mod登録)

### 2-2. Task B: TOMLテストケース
Task A完了後dispatch。expected値は手計算しない。

**dummy → actual パターン:**
1. TOML作成 (expected `0.1` ダミー)
2. `cargo test -- {char_id}` → 失敗出力から `got {actual}` 読み取り
3. TOML expectedをactual値で上書き
4. 再実行して全パス確認 → コミット

### 2-3. Task C: 天賦バフ (該当時のみ)
Task A完了後、Task Bと並列dispatch可能。

### 2-4. Task D: 月兆データ (該当時のみ)
Phase 1で「要実装」判定時のみ。

---

## Phase 3: Verify + Merge

```bash
cd {worktree_path}
cargo build && cargo test && cargo clippy -- -D warnings && cargo fmt --check
```

1. TODOチェックボックス `[x]` 更新、コミット
2. spec/planファイル削除 (存在時)
3. ユーザー確認後マージ:
```bash
cd /home/sakis/genshin-calc
git merge feat/{char_id}-data --no-ff
git worktree remove .claude/worktrees/feat-{char_id}
git branch -d feat/{char_id}-data
```

---

## Subagent Model Routing

全Task sonnet (mechanical task)。

## Related Skills

| スキル | 役割 |
|--------|------|
| `honeyhunter-data-fetch` | MD構造・抽出ワークフロー |
| `game-update-todo` | バージョン全体のTODO生成 |

## Related Docs

- `docs/dev/character-data-guide.md` — デュアルスケーリング等の実装パターン
- `docs/dev/testing.md` — テスト構造・許容誤差
- `docs/dev/architecture.md` — crate構造
