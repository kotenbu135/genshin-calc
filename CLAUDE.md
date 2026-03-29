# genshin-calc

原神のダメージ・元素反応計算エンジン（Rust製ライブラリ）

## Project Structure
- Cargoワークスペース構成: `crates/core`（計算エンジン）, `crates/data`（ゲームデータ、将来）
- MIT License

## Development Commands
- `cargo build` — ビルド
- `cargo test` — 全テスト実行
- `cargo test -p genshin-calc-core` — coreのみテスト
- `cargo clippy -- -D warnings` — lint
- `cargo fmt --check` — フォーマット確認

## Conventions
- イミュータブル設計: 構造体は変更せず新規作成
- 純粋関数: 副作用なし、入力→出力が一意
- f64で数値計算（原神の内部仕様に合わせる）
- 浮動小数点テストは許容誤差で比較（assert_eq!禁止）
- WASM互換: stdの重い機能（ファイルI/O等）は使わない
- serde対応: 公開型にはSerialize/Deserializeをderive

## Architecture
- `core`はデータに依存しない純粋な計算ロジック
- ゲームデータ（キャラ、武器等）は`data` crateに分離
- 公開APIは最小限に保つ
