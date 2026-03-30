# Project Conventions

## Coding Rules
- イミュータブル設計: 構造体は変更せず新規作成
- 純粋関数: 副作用なし、入力→出力が一意
- f64で数値計算（原神の内部仕様に合わせる）
- 浮動小数点テストは許容誤差で比較（assert_eq!禁止）
- serde roundtrip比較もEPSILON比較（JSON往復でf64に1-ULP差が出る）
- WASM互換: stdの重い機能（ファイルI/O等）は使わない。公開structにはHashMapよりVecを優先
- serde対応: 公開型にはSerialize/Deserializeをderive
- crate内共有関数は`pub(crate)`（例: `resistance_multiplier`）
- rust-analyzerの偽陽性（unlinked-file, missing field等）はキャッシュ古い場合あり。`cargo build`/`cargo test`が正

## Data Crate Rules
- 数値単位: 全パーセンテージは小数形式（10.8% → 0.108）
- `&'static`参照型はSerializeのみ（Deserializeなし）
- 武器ConditionalBuff分類: 全パッシブが対象ではない
  - SKIP: proc damage（追加ヒットのみ）、エネルギー生成（Favonius系）、CDリセット（Sacrificial系）、HP回復、純ユーティリティ
  - IMPLEMENT: ATK/DMG/CRIT等のステータスバフを付与する条件付き効果
  - StatBuffと重複する武器（THE_ALLEY_FLASH等）はStatBuffをconditional_buffsに移動し、buffsを&[]にすること（二重計上防止）
- チームエネルギー合計系武器（AKUOUMARU/WAVEBREAKERS_FIN/MOUUNS_MOON）はToggle使用（StatScalingは単一キャラのstat参照のみ）
- `team_builder.rs`注意: `Both(auto, manual)`はauto_valをeval_manualのbase_valueとして渡す。Toggle→auto_val返却、Stacks→auto_val×n

## Critical Change Warning
- `DamageInput`/`LunarInput`/`TeamMember`変更時は全構築箇所（テスト・docコメント・README・examples含む）を一括修正すること（コンパイル不能防止）
