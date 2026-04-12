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
- **TDD必須**: テストを先に書く（RED→GREEN→REFACTOR）。実装を先に書いてからテストを修正するフローは禁止
- 実装はWorktreeを使用して行う
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

## Communication Style
- セッション開始時、自動で `/genshijin 極限` モード有効化。全レスポンス極限圧縮で返答
- 解除: ユーザが「原始人やめて」「normal mode」と言った場合のみ
