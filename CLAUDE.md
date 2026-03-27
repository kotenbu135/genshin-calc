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
- serde roundtrip比較もEPSILON比較（JSON往復でf64に1-ULP差が出る）
- WASM互換: stdの重い機能（ファイルI/O等）は使わない
- serde対応: 公開型にはSerialize/Deserializeをderive
- crate内共有関数は`pub(crate)`（例: `resistance_multiplier`）
- rust-analyzerの偽陽性（unlinked-file, missing field等）はキャッシュ古い場合あり。`cargo build`/`cargo test`が正

## Architecture
- `core`はデータに依存しない純粋な計算ロジック
- ゲームデータ（キャラ、武器等）は`data` crateに分離
- 公開APIは最小限に保つ
- `stats.rs`: 最終ステータス構造体（HP/ATK/DEF/会心等）
- `error.rs`: CalcError列挙型（バリデーションエラー等）
- `types.rs`: Element, ScalingStat等の共有型定義
- `enemy.rs`: 敵パラメータ（レベル、耐性、防御減少）
- 3つの計算パイプライン:
  - `calculate_damage`: 通常ダメージ + 増幅反応(蒸発/溶解) + 激化反応(超激化/草激化)
  - `calculate_transformative`: 固定反応（過負荷、超電導、拡散等）— 会心なし、防御なし
  - `calculate_lunar`: 月反応（月感電、月開花、月結晶）— 会心あり、防御なし
- `damage.rs`: メイン計算（ATK×倍率→バフ→防御→耐性→増幅/激化）
- `reaction.rs`: 元素反応タイプ判定、倍率テーブル
- `em.rs`: 元素熟知ボーナス計算（4種の式）
- `level_table.rs`: レベル基礎値テーブル（Lv1-100、データマイニング値）
- `transformative.rs`: 固定反応ダメージ計算
- `lunar.rs`: 月反応ダメージ計算
- `stat_profile.rs`: ステータス合算（StatProfile → Stats）
- `DamageInput`変更時は全構築箇所（テスト含む）を一括修正すること（コンパイル不能防止）

## Testing
- 116テスト（v0.3.0時点）
- v0.3.0でStatProfile合算 + ScalingStatテスト追加
- ゲーム検証済みキャラ: Freminet（完全一致）、Diluc、Ganyu、Raiden、Yanfei蒸発
- goldenテスト: 手計算値との照合（各モジュールに `test_golden_*` テスト）
- 許容誤差: 通常 `< 0.01`、ゲーム検証は `< 1.0`（floor丸め考慮）
