---
name: character-add-workflow
description: End-to-end execution skill for adding a new Genshin Impact character — fetches data, auto-scopes, dispatches subagents in a worktree, verifies, and merges. Replaces brainstorming/writing-plans/SDD chain for character additions.
version: 2.0.0
---

# Character Add Workflow v2

キャラクター追加をワンショットで実行するスキル。
brainstorming / writing-plans / subagent-driven-development の連鎖を**スキップ**し、このスキル単体で完結する。

## Trigger

- ユーザーが「キャラ追加」「{名前}を実装」「新キャラデータ」に言及したとき
- `/game-update-todo` で生成されたTODOのうちキャラ関連タスクを実装するとき

## Flow

```
Phase 1: Data Fetch + Scope (このセッション)
  ├── Honey Impact WebFetch
  ├── スコープ自動判定 (月兆/バフ/デュアルスケーリング)
  └── ユーザーにスコープ提示 → 確認待ち (1メッセージ)

Phase 2: Worktree + Subagent Dispatch (このセッション)
  ├── git worktree 作成
  ├── Task A: zibai.rs + mod.rs (subagent — honeyhunter-data-fetch で自前取得)
  ├── Task B: TOML テスト (subagent — dummy expected → test → actual 読取 → 修正)
  ├── Task C: talent_buffs (subagent — 該当時のみ, Task A 完了後 Task B と並列)
  └── Task D: moonsign (subagent — 該当時のみ, 未実装の場合)

Phase 3: Verify + Merge (このセッション)
  ├── cargo build + test + clippy + fmt
  ├── TODO チェックボックス更新
  ├── spec/plan ファイル削除 (存在する場合)
  └── main にマージ + worktree 削除
```

**重要:** brainstorming / writing-plans / executing-plans スキルは呼び出さない。

---

## Phase 1: Data Fetch + Scope

### 1-1. Honey Impact データ取得

```
WebFetch: https://gensh.honeyhunterworld.com/{char_name}/?lang=EN
```

抽出対象:
- 基本情報 (元素, 武器種, ★, 地域)
- ベースステータス (HP/ATK/DEF × [Lv1, Lv20, Lv80, Lv90])
- 突破ステ (種類 + Lv90値)
- 天賦名 + 各段のスケーリングstat (ATK/DEF/HP/EM) と元素判定
- パッシブ A1/A4 の説明文
- 星座 C1-C6 の説明文 (C3/C5 でどの天賦が+3か)

**天賦倍率の数値 (Lv1-15) はこの段階では不要。** subagent が個別に取得する。

### 1-2. スコープ自動判定

| 判定項目 | 判定方法 | 結果 |
|---------|---------|------|
| 月兆 | `moonsign_chars.rs` に既存エントリがあるか grep | 実装済み / 要実装 / 対象外 |
| 天賦バフ | A1/A4/C1-C6 にステータスバフがあるか | バフ一覧 (stat, value, target, source) |
| デュアルスケーリング | 天賦説明に ATK+DEF 等の複数stat参照があるか | 分割パターン |
| 星座パターン | C3/C5 の記述から判定 | C3SkillC5Burst or C3BurstC5Skill |

### 1-3. スコープ提示

ユーザーに1メッセージで提示:
```
## {CharName} ({Element}/{Weapon}/★{N}) スコープ

| 項目 | 結果 |
|------|------|
| 月兆 | {実装済み/要実装/対象外} |
| 天賦バフ | {なし / C2: TransformativeBonus +30% 等} |
| デュアルスケーリング | {なし / ATK+DEF 等} |
| 星座パターン | {C3SkillC5Burst / C3BurstC5Skill} |

実装タスク: A(データ定義+登録), B(テスト), {C(バフ)}, {D(月兆)}
これで進めますか？
```

ユーザー確認後、Phase 2 へ。

---

## Phase 2: Worktree + Subagent Dispatch

### 2-0. Worktree 作成

```bash
git worktree add .claude/worktrees/feat-{char_id} -b feat/{char_id}-data
```

### 2-1. Task A: CharacterData + mod 登録

**1つの subagent で実行。** honeyhunter-data-fetch のフォーマット規約を subagent に渡す。

subagent に渡す情報:
- Worktree パス
- Honey Impact URL
- 基本情報 (element, weapon, rarity, region)
- ベースステータス 4値 (Phase 1 で取得済み)
- 突破ステ
- 天賦構成 (各天賦名, scaling_stat, damage_element の判定結果)
- 星座パターン
- 既存キャラの参照ファイルパス (同元素で最も近いパターン)
- mod.rs の現在の内容

subagent の責務:
1. WebFetch で天賦倍率 Lv1-15 を取得
2. `{char_id}.rs` 作成 (honeyhunter-data-fetch のフォーマット規約に従う)
3. `mod.rs` に3箇所追加
4. `cargo build -p genshin-calc-data` で検証
5. `cargo test -p genshin-calc-data` で検証
6. 2コミット (データ定義 + mod登録)

### 2-2. Task B: TOML テストケース

**Task A 完了後に dispatch。** expected 値は手計算しない。

subagent に渡す情報:
- Worktree パス
- キャラ名, 元素
- テストケース構成 (scaling_stat, damage_type, element, talent_multiplier の Lv10 値)
- 既存 TOML の参照ファイルパス

subagent の責務 (dummy → actual パターン):
1. TOML ファイルを作成 (expected は `0.1` のダミー値)
2. `cargo test -p genshin-calc-core --test character_verification -- {char_id}` を実行
3. テスト失敗出力から `got {actual}` の値を読み取る
4. TOML の expected をactual値で上書き
5. テストを再実行して全パス確認
6. コミット

**これにより手計算ミスを完全に排除する。**

### 2-3. Task C: 天賦バフ (該当時のみ)

**Task A 完了後、Task B と並列 dispatch 可能。**

subagent に渡す情報:
- Worktree パス
- バフ定義 (Phase 1 で判定済み: stat, base_value, target, source, min_constellation)
- `talent_buffs/{element}.rs` の現在の内容

### 2-4. Task D: 月兆データ (該当時のみ)

**Phase 1 で「要実装」と判定された場合のみ。**

subagent に渡す情報:
- Worktree パス
- 月兆データ (reaction, scaling_stat, rate, max_bonus)
- `moonsign_chars.rs` の現在の内容

---

## Phase 3: Verify + Merge

### 3-1. 最終検証

```bash
cd {worktree_path}
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

### 3-2. TODO 更新

`docs/v6.0-6.3-data-update-todo.md` の該当セクションのチェックボックスを `[x]` に更新。コミット。

### 3-3. クリーンアップ

- `docs/superpowers/specs/` と `docs/superpowers/plans/` の該当ファイルを削除 (存在する場合)

### 3-4. マージ

ユーザーに確認後:
```bash
cd /home/sakis/genshin-calc
git merge feat/{char_id}-data --no-ff
git worktree remove .claude/worktrees/feat-{char_id}
git branch -d feat/{char_id}-data
```

---

## Subagent Model Routing

CLAUDE.md の Agent Model Routing に従う。キャラデータ実装は mechanical task なので sonnet を使用。

| Task | Model |
|------|-------|
| Task A (データ定義 + mod登録) | sonnet |
| Task B (TOML テスト) | sonnet |
| Task C (天賦バフ) | sonnet |
| Task D (月兆) | sonnet |

---

## Domain Knowledge (Reference)

以下はスキル実行中にsubagentへ渡すドメイン知識。

### ファイル配置

| 種類 | パス |
|------|------|
| キャラデータ | `crates/data/src/characters/{element}/{char_id}.rs` |
| mod登録 | `crates/data/src/characters/{element}/mod.rs` |
| TOML テスト | `crates/core/tests/data/characters/{char_id}.toml` |
| 天賦バフ | `crates/data/src/talent_buffs/{element}.rs` |
| 月兆 | `crates/data/src/moonsign_chars.rs` |
| TODO | `docs/v6.0-6.3-data-update-todo.md` |

### 元素判定ルール

| 武器種 | 通常攻撃 | 重撃 | スキル/爆発 |
|--------|---------|------|------------|
| Catalyst | `Some(Element)` | `Some(Element)` | `Some(Element)` |
| Sword/Claymore/Polearm | `None` (物理) | `None` (物理) | `Some(Element)` |
| Bow | `None` (物理) | フルチャージ: `Some(Element)` | `Some(Element)` |

### パーセンテージ変換

Honey Impact の `%` 値は全て小数に変換: `88.2%` → `0.882`

### base_stat 配列

`[Lv1, Lv20, Lv80, Lv90]` — 4値。Lv20/Lv80 は突破前の値。

### 天賦スケーリング

`[f64; 15]` — Lv1 から Lv15 まで必ず15値。

### 天賦バフ判定

| パッシブ内容 | 実装する？ |
|-------------|-----------|
| ATK/DMG/CRIT等のステータスバフ | YES — TalentBuffDef |
| 元素耐性ダウン | YES — TalentBuffDef |
| Lunar反応DMG+N% | YES — `BuffableStat::TransformativeBonus` |
| 追加ヒット/proc damage | NO |
| HP回復/シールド | NO |
| クールダウン短縮 | NO |

### Common Pitfalls

| ミス | 正しい対処 |
|------|-----------|
| パーセンテージ小数変換忘れ | honeyhunter-data-fetch 規約参照 |
| Catalyst通常攻撃の `damage_element: None` | `Some(Element::Xxx)` が正しい |
| base_stat 突破前/後混同 | Lv20=突破前, Lv80=突破前 |
| 天賦スケーリング13値 | 必ず15値 (Lv1-15) |
| mod.rs CHARACTERS 登録忘れ | Step 2 で3箇所変更 |
| 天賦バフ名の重複 | `{char_id}_{effect}` でユニーク化 |
| TOML expected の手計算ミス | dummy → actual パターンで自動算出 |

---

## Related Skills

| スキル | 役割 | いつ使う |
|--------|------|---------|
| `honeyhunter-data-fetch` | 数値データ取得のフォーマット規約 | Task A の subagent が参照 |
| `game-update-todo` | バージョン全体のTODOリスト生成 | 複数キャラ実装時の全体把握 |

## Related Docs

| ドキュメント | 内容 |
|-------------|------|
| `docs/dev/character-data-guide.md` | デュアルスケーリング等の実装パターン |
| `docs/dev/testing.md` | テスト構造・許容誤差 |
| `docs/dev/architecture.md` | crate構造とファイル配置 |
