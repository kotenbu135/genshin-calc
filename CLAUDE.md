# genshin-calc

原神のダメージ・元素反応計算エンジン（Rust製ライブラリ）

## Project Structure
- Cargoワークスペース構成: `crates/core`（計算エンジン）, `crates/data`（ゲームデータ v6.4）
- MIT License

## Development Commands
- `cargo build` — ビルド
- `cargo test` — 全テスト実行
- `cargo test -p genshin-calc-core` — coreのみテスト
- `cargo test -p genshin-calc-data` — dataのみテスト
- `cargo test -p genshin-calc-core --test moonsign_integration` — 月兆統合テスト
- `cargo clippy -- -D warnings` — lint
- `cargo fmt --check` — フォーマット確認

## Architecture (Summary)
- `core`: データに依存しない純粋な計算ロジック（3パイプライン: damage/transformative/lunar）
- `data`: ゲームデータをRust定数として実装
- 詳細: `docs/dev/architecture.md`

## Testing (Summary)
- 浮動小数点テストは許容誤差で比較（assert_eq!禁止）
- データ駆動テスト: `tests/data/characters/*.toml` — 新キャラはTOMLファイル1つ追加
- バフ検証テストは「applied_buffsに期待バフが含まれるか」「final_statsに数値が反映されているか」を明示的にassertすること（`damage > 0` のスモークテストでは未実装を検知できない）
- 凸バフテスト: C0だけでなくC2/C4/C6も網羅すること。凸バフ未実装はエラーにならず正常動作と区別できない
- 未実装検出: `honeyhunter-mirror/md/`から全効果一覧を取得し、`talent_buffs/`の実装とdiffを取ること。「実装済みコードの正しさ」だけでなく「実装すべきコードの欠落」を検出する
- テスト期待値は必ず`honeyhunter-mirror/md/`から取得。自前推定は禁止
- 詳細: `docs/dev/testing.md`

## Documentation
- `docs/data/`: 実装済みゲームデータ一覧
- `docs/dev/`: 開発者向け詳細ドキュメント（architecture, testing, character-data-guide）
- `.claude/skills/game-update-todo/`: ゲーム更新時の実装TODOリスト生成スキル
- `.claude/skills/honeyhunter-data-fetch/`: ローカルミラーからゲームデータ取得・Rust定数変換スキル
- `.claude/skills/character-add-workflow/`: キャラクター追加E2Eワークフロー（ドメイン知識リファレンス、superpowersと併用）

## Game Data Source
- ゲームデータは `honeyhunter-mirror/md/` のローカルMarkdownから取得（Webアクセス不要）
- キャラ: `honeyhunter-mirror/md/characters/{name}_{id}.md` — 倍率・ステータス・星座・パッシブ（日英紐づけ対応）
- 武器: `honeyhunter-mirror/md/weapons/i_n{id}.md` — ステータス・精錬効果
- ミラー更新: `./scripts/honeyhunter-mirror.sh characters [ja]` → `python3 scripts/honeyhunter-to-md.py`
- WebFetch/ブラウザでHoney Impactにアクセスしないこと

## Superpowers Preferences

Superpowers:brainstormingスキルは以下について質問せず、記載の通り進めること。

### 自動採用する機能

- **Visual companion**: 常にYES（提案メッセージ不要、即座に使用開始）
- **SubAgent Driven Development**: 常にYES（実装は常にsubagent-driven-developmentスキルを使用）
- **Spec review loop**: 常に実行（確認不要）

### 対話スタイル

- 選択肢形式を優先（A/B/C で回答できる形式）
- 推奨選択肢を常に即採用（質問不要）。設計判断は推奨を提示→即採用→次へ進む
- 1メッセージ1質問の制約は守るが、コンテキストから自明な質問はスキップ
- CLAUDE.mdに回答が書いてある質問は聞かない

### ブレインストーミング後の処理

- 終了時にこの項目に追加したほうが良い質問内容を提案する
- 実装完了後、`docs/superpowers/specs/` と `docs/superpowers/plans/` の該当ファイルを削除する（実装済みの設計書は不要。コードとテストが正）

## Agent Model Routing

サブエージェント起動時、Agentツールの`model`パラメータで以下を指定すること:

| エージェント | モデル | 理由 |
|------------|--------|------|
| planner | sonnet | メインセッションがOpusなので計画はSonnetで十分 |
| architect | sonnet | 同上 |
| build-error-resolver | haiku | ビルドエラー修正はパターン化されている |
| rust-build-resolver | haiku | 同上 |
| code-reviewer | sonnet | デフォルト通り |
| rust-reviewer | sonnet | デフォルト通り |
| security-reviewer | sonnet | デフォルト通り |
| tdd-guide | sonnet | デフォルト通り |
| doc-updater | haiku | デフォルト通り |

## superpowers:execute-plan
実装はWorktreeを使用して行う
