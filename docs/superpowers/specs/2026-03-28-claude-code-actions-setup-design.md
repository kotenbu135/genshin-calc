# Claude Code Actions 環境セットアップ 設計仕様

## 概要

genshin-calc リポジトリの Claude Code 環境を整備する。ローカル開発用の hooks + permissions と、GitHub Actions 上で Claude Code を動かすためのワークフローYAML を構成する。

## 目的

- `docs/superpowers/plans/` の実装プランを Claude Code Actions で実行可能にする
- ローカル開発時の品質ゲート（auto-fmt, auto-check, テスト）を hooks で自動化する
- グローバルにインストール済みのプラグイン（superpowers, everything-claude-code, rust-analyzer-lsp）を CI 上でも利用可能にする

## 変更ファイル

| ファイル | アクション | 目的 |
|----------|-----------|------|
| `.claude/settings.json` | 新規作成 | Permissions + Hooks |
| `.claude/settings.local.json` | 削除 | settings.json に集約 |
| `.claude/hooks/rust-fmt.sh` | 新規作成 | .rs ファイル自動フォーマット |
| `.claude/hooks/rust-check.sh` | 新規作成 | .rs ファイルコンパイルチェック |
| `.claude/hooks/quality-gate.sh` | 新規作成 | テスト + clippy 品質ゲート |
| `.github/workflows/claude.yml` | 新規作成 | GitHub Actions ワークフロー |
| `.gitignore` | 修正 | settings.local.json を除外 |

---

## 1. `.claude/settings.json` 完全内容

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

**Permissions 方針:**
- `Bash(cargo:*)` で全 cargo サブコマンドを一括許可（従来: 個別に30行以上）
- `Bash(gh:*)` で GitHub CLI を一括許可
- `WebFetch(domain:*)` で全ドメイン許可（従来: 個別に50行以上）
- 合計100行超 → 6行に整理

---

## 2. Hook スクリプト

hooks の `matcher` はツール名のみマッチし、ファイルパスにはマッチしない。`.rs` ファイルの判定は stdin の JSON をパースして行う。

全スクリプトは `chmod +x` で実行権限を付与すること。

### `.claude/hooks/rust-fmt.sh`

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

### `.claude/hooks/rust-check.sh`

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

コンパイルエラーがあっても exit 0 で返す（ブロックせず、出力で Claude に通知）。

### `.claude/hooks/quality-gate.sh`

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

**設計判断:**
- `exit 0` + JSON decision でブロック（`exit 2` は JSON を捨てるため不可）
- `stop_hook_active` ガードで無限ループ防止（Claude が修正→再Stop→再チェックのループ制御）
- `cargo test` と `cargo clippy` を分離実行し、失敗した方のみエラー出力に含める

---

## 3. `.github/workflows/claude.yml` 完全内容

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

**設計判断:**
- `id-token: write` を削除 — OAuth トークン方式では不要（OIDC は Bedrock/Vertex 用）
- `concurrency` でPR単位の同時実行を制御（二重起動防止）
- `timeout-minutes: 30` でランナウェイ防止
- `if` 条件で `@claude` メンション時のみ起動（PR イベントは常時）
- `--allowedTools` から `EnterPlanMode`/`ExitPlanMode`/`NotebookEdit` を削除（CI の非対話モードでは不要）
- action バージョンは `@v1` にピン

---

## 4. `.claude/settings.local.json` 削除 + `.gitignore`

settings.local.json を削除し、再生成を防ぐため `.gitignore` に追加:

```gitignore
.claude/settings.local.json
```

---

## 5. セキュリティ考慮事項

- OAuth トークンは GitHub Secrets に格納（ハードコード禁止）
- PR からの `.claude/` ディレクトリ変更は claude-code-action が自動的に base ブランチ版で上書きする（インジェクション防止）
- CI 上の Bash は `cargo:*`, `git:*`, `gh:*` に制限（任意コマンド実行不可）
- `cargo publish` は `Bash(cargo:*)` で暗黙的に許可されるが、CI 上に crates.io 認証情報がないため実害なし

## 6. 非スコープ

- プロジェクト固有のスラッシュコマンド（superpowers スキルで代替）
- CLAUDE.md へのワークフローガイダンス追加（スキル側に委任）
- 既存のグローバル settings.json の変更
