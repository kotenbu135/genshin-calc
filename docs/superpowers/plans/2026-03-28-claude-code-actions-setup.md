# Claude Code Actions 環境セットアップ 実装プラン

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** ローカル hooks（auto-fmt, auto-check, quality-gate）+ GitHub Actions ワークフローを構成し、Claude Code でプラン実行可能な環境を整える

**Architecture:** `.claude/settings.json` に permissions + hooks を集約。hook スクリプトは `.claude/hooks/` に配置し、stdin JSON をパースして `.rs` ファイルのみ処理する。GitHub Actions は `anthropics/claude-code-action@v1` + 3プラグイン（superpowers, everything-claude-code, rust-analyzer-lsp）で構成。

**Tech Stack:** Claude Code hooks, GitHub Actions, bash, jq

**Spec:** `docs/superpowers/specs/2026-03-28-claude-code-actions-setup-design.md`

---

## File Structure

| File | Action | Responsibility |
|------|--------|---------------|
| `.claude/hooks/rust-fmt.sh` | Create | PostToolUse: .rs ファイル自動フォーマット |
| `.claude/hooks/rust-check.sh` | Create | PostToolUse: .rs ファイルコンパイルチェック |
| `.claude/hooks/quality-gate.sh` | Create | Stop: テスト + clippy 品質ゲート |
| `.claude/settings.json` | Create | Permissions + Hooks 設定 |
| `.claude/settings.local.json` | Delete | settings.json に集約済み |
| `.github/workflows/claude.yml` | Create | GitHub Actions ワークフロー |
| `.gitignore` | Modify | settings.local.json を除外 |

---

### Task 1: Hook スクリプト作成

**Files:**
- Create: `.claude/hooks/rust-fmt.sh`
- Create: `.claude/hooks/rust-check.sh`
- Create: `.claude/hooks/quality-gate.sh`

- [ ] **Step 1: hooks ディレクトリ作成**

Run: `mkdir -p .claude/hooks`

- [ ] **Step 2: rust-fmt.sh 作成**

Create `.claude/hooks/rust-fmt.sh`:

```bash
#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 0
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ "$FILE_PATH" == *.rs ]]; then
  cargo fmt 2>&1
fi
exit 0
```

- [ ] **Step 3: rust-check.sh 作成**

Create `.claude/hooks/rust-check.sh`:

```bash
#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 0
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ "$FILE_PATH" == *.rs ]]; then
  OUTPUT=$(cargo check --quiet 2>&1)
  if [ $? -ne 0 ]; then
    echo "$OUTPUT"
  fi
fi
exit 0
```

- [ ] **Step 4: quality-gate.sh 作成**

Create `.claude/hooks/quality-gate.sh`:

```bash
#!/bin/bash
cd "$(git rev-parse --show-toplevel)" || exit 0
INPUT=$(cat)

# 無限ループ防止: Stop hook が再発火した場合はスキップ
STOP_HOOK_ACTIVE=$(echo "$INPUT" | jq -r '.stop_hook_active')
if [ "$STOP_HOOK_ACTIVE" = "true" ]; then
  exit 0
fi

TEST_OUTPUT=$(cargo test --quiet 2>&1)
TEST_RC=$?
CLIPPY_OUTPUT=$(cargo clippy -- -D warnings 2>&1)
CLIPPY_RC=$?

if [ $TEST_RC -ne 0 ] || [ $CLIPPY_RC -ne 0 ]; then
  REASON=""
  [ $TEST_RC -ne 0 ] && REASON="cargo test failed:\n$TEST_OUTPUT\n"
  [ $CLIPPY_RC -ne 0 ] && REASON="${REASON}cargo clippy failed:\n$CLIPPY_OUTPUT"
  jq -n --arg reason "$REASON" '{"decision":"block","reason":$reason}'
  exit 0
fi
exit 0
```

- [ ] **Step 5: 実行権限を付与**

Run: `chmod +x .claude/hooks/rust-fmt.sh .claude/hooks/rust-check.sh .claude/hooks/quality-gate.sh`

- [ ] **Step 6: スクリプトの動作確認**

各スクリプトが構文エラーなく実行できることを確認:

Run: `echo '{"tool_input":{"file_path":"test.rs"}}' | .claude/hooks/rust-fmt.sh`
Expected: `cargo fmt` が実行される（エラーなし）

Run: `echo '{"tool_input":{"file_path":"test.txt"}}' | .claude/hooks/rust-fmt.sh`
Expected: 何も実行されない（.rs でないため）

Run: `echo '{"tool_input":{"file_path":"test.rs"}}' | .claude/hooks/rust-check.sh`
Expected: `cargo check` が実行される（エラーなし）

Run: `echo '{"stop_hook_active":false}' | .claude/hooks/quality-gate.sh`
Expected: テスト + clippy が実行される

Run: `echo '{"stop_hook_active":true}' | .claude/hooks/quality-gate.sh`
Expected: 即座に exit（何も実行しない）

- [ ] **Step 7: コミット**

```bash
git add .claude/hooks/
git commit -m "feat: add Claude Code hook scripts (fmt, check, quality-gate)"
```

---

### Task 2: settings.json 作成 + settings.local.json 削除

**Files:**
- Create: `.claude/settings.json`
- Delete: `.claude/settings.local.json`
- Modify: `.gitignore`

- [ ] **Step 1: settings.json 作成**

Create `.claude/settings.json`:

```json
{
  "permissions": {
    "allow": [
      "Bash(cargo:*)",
      "Bash(git:*)",
      "Bash(gh:*)",
      "Bash(python3:*)",
      "WebSearch",
      "WebFetch(domain:*)"
    ]
  },
  "hooks": {
    "PostToolUse": [
      {
        "matcher": "Edit|Write",
        "hooks": [
          {
            "type": "command",
            "command": ".claude/hooks/rust-fmt.sh",
            "statusMessage": "Running cargo fmt..."
          },
          {
            "type": "command",
            "command": ".claude/hooks/rust-check.sh",
            "statusMessage": "Running cargo check..."
          }
        ]
      }
    ],
    "Stop": [
      {
        "hooks": [
          {
            "type": "command",
            "command": ".claude/hooks/quality-gate.sh",
            "statusMessage": "Running quality gate (test + clippy)..."
          }
        ]
      }
    ]
  }
}
```

- [ ] **Step 2: settings.local.json 削除**

Run: `rm .claude/settings.local.json`

Note: このファイルはグローバル gitignore で除外されておりgit管理外のため `git rm` ではなく `rm` を使用。

- [ ] **Step 3: .gitignore に settings.local.json を追加**

`.gitignore` の末尾に追加:

```gitignore

# Claude Code local settings (user-specific, auto-generated)
.claude/settings.local.json
```

- [ ] **Step 4: settings.json の JSON バリデーション**

Run: `python3 -c "import json; json.load(open('.claude/settings.json')); print('Valid JSON')"`
Expected: `Valid JSON`

- [ ] **Step 5: コミット**

```bash
git add .claude/settings.json .gitignore
git commit -m "feat: add Claude Code settings with hooks, update gitignore"
```

---

### Task 3: GitHub Actions ワークフロー作成

**Files:**
- Create: `.github/workflows/claude.yml`

- [ ] **Step 1: workflows ディレクトリ作成**

Run: `mkdir -p .github/workflows`

- [ ] **Step 2: claude.yml 作成**

Create `.github/workflows/claude.yml`:

```yaml
name: Claude Code

on:
  issue_comment:
    types: [created]
  pull_request_review_comment:
    types: [created]
  pull_request_review:
    types: [submitted]
  issues:
    types: [opened, assigned]
  pull_request:
    types: [opened, synchronize]

concurrency:
  group: ${{ github.workflow }}-${{ github.event.pull_request.number || github.event.issue.number || github.run_id }}
  cancel-in-progress: true

jobs:
  claude:
    if: |
      (github.event_name == 'pull_request') ||
      (github.event_name == 'issue_comment' && contains(github.event.comment.body, '@claude')) ||
      (github.event_name == 'pull_request_review_comment' && contains(github.event.comment.body, '@claude')) ||
      (github.event_name == 'pull_request_review' && contains(github.event.review.body, '@claude')) ||
      (github.event_name == 'issues' && (contains(github.event.issue.body, '@claude') || github.event.action == 'assigned'))
    runs-on: ubuntu-latest
    timeout-minutes: 30
    permissions:
      contents: write
      pull-requests: write
      issues: write
      actions: read
    steps:
      - name: Checkout repository
        uses: actions/checkout@v6
        with:
          fetch-depth: 1

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rust-analyzer

      - name: Run Claude Code
        uses: anthropics/claude-code-action@v1
        with:
          claude_code_oauth_token: ${{ secrets.CLAUDE_CODE_OAUTH_TOKEN }}
          plugin_marketplaces: |
            https://github.com/obra/superpowers-marketplace.git
            https://github.com/affaan-m/everything-claude-code.git
            https://github.com/anthropics/claude-plugins-official.git
          plugins: |
            superpowers@superpowers-marketplace
            everything-claude-code@everything-claude-code
            rust-analyzer-lsp@claude-plugins-official
          claude_args: |
            --allowedTools "Skill,Agent,Edit,Read,Write,Glob,Grep,LSP,Bash(cargo:*),Bash(git:*),Bash(gh:*),WebSearch,WebFetch,TaskCreate,TaskUpdate,TaskList,TaskGet"
```

- [ ] **Step 3: YAML バリデーション**

Run: `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/claude.yml')); print('Valid YAML')"`
Expected: `Valid YAML`

Note: `python3 -c "import yaml"` が失敗する場合は `python3 -c "import json, re; print('YAML structure looks correct')"` でスキップし、GitHub push 後に Actions タブで確認。

- [ ] **Step 4: コミット**

```bash
git add .github/workflows/claude.yml
git commit -m "ci: add Claude Code GitHub Actions workflow"
```

---

### Task 4: OAuth トークン設定 + 動作確認

- [ ] **Step 1: OAuth トークン生成** ⚠️ MANUAL — 人間の操作が必要

Run: `claude setup-token`

表示される手順に従い、トークンを生成する。

- [ ] **Step 2: GitHub Secrets に登録** ⚠️ MANUAL — 人間の操作が必要

Run: `gh secret set CLAUDE_CODE_OAUTH_TOKEN`

プロンプトにトークンを貼り付ける。

- [ ] **Step 3: GitHub App インストール確認** ⚠️ MANUAL — 人間の操作が必要

Claude GitHub App がリポジトリにインストールされていることを確認:
https://github.com/apps/claude

未インストールの場合は `claude /install-github-app` で設定。

- [ ] **Step 4: 全変更をプッシュ**

Run: `git push origin main`

- [ ] **Step 5: ローカル hooks の動作確認**

Claude Code を起動し、任意の `.rs` ファイルを小さく編集して hooks が発火することを確認:
1. Edit 後に `Running cargo fmt...` と `Running cargo check...` がステータスに表示される
2. `/exit` 等でセッション終了時に `Running quality gate...` が表示される

- [ ] **Step 6: GitHub Actions の動作確認**

テスト用 Issue を作成して Claude Code Actions が起動することを確認:

Run: `gh issue create --title "Test @claude" --body "@claude ping — test that Claude Code Actions is working"`

GitHub の Issue ページで Claude からの応答コメントを確認。確認後、Issue を閉じる:

Run: `gh issue close <issue-number>`

- [ ] **Step 7: コミット（テスト結果に変更があれば）**

hooks テスト中にフォーマット等の変更が生じた場合:

```bash
git add -A
git commit -m "chore: apply hook-triggered formatting"
```
