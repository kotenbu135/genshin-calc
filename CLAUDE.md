# genshin-calc

原神のダメージ・元素反応計算エンジン（Rust製ライブラリ）

## Project Structure
- Cargoワークスペース構成: `crates/core`（計算エンジン）, `crates/data`（ゲームデータ v5.8）
- MIT License

## Development Commands
- `cargo build` — ビルド
- `cargo test` — 全テスト実行
- `cargo test -p genshin-calc-core` — coreのみテスト
- `cargo test -p genshin-calc-data` — dataのみテスト
- `cargo test -p genshin-calc-core --test moonsign_integration` — 月兆統合テスト
- `cargo clippy -- -D warnings` — lint
- `cargo fmt --check` — フォーマット確認

## Conventions
- イミュータブル設計: 構造体は変更せず新規作成
- 純粋関数: 副作用なし、入力→出力が一意
- f64で数値計算（原神の内部仕様に合わせる）
- 浮動小数点テストは許容誤差で比較（assert_eq!禁止）
- serde roundtrip比較もEPSILON比較（JSON往復でf64に1-ULP差が出る）
- WASM互換: stdの重い機能（ファイルI/O等）は使わない。公開structにはHashMapよりVecを優先
- serde対応: 公開型にはSerialize/Deserializeをderive
- crate内共有関数は`pub(crate)`（例: `resistance_multiplier`）
- rust-analyzerの偽陽性（unlinked-file, missing field等）はキャッシュ古い場合あり。`cargo build`/`cargo test`が正

## Architecture
- `core`はデータに依存しない純粋な計算ロジック
- ゲームデータ（キャラ、武器等）は`data` crateに分離
- 公開APIは最小限に保つ
- `stats.rs`: 最終ステータス構造体（HP/ATK/DEF/会心等）
- `error.rs`: CalcError列挙型（バリデーションエラー等）
- `types.rs`: Element, ScalingStat(Atk/Hp/Def/Em), WeaponType等の共有型定義
- `enemy.rs`: 敵パラメータ（レベル、耐性、防御減少）
- 3つの計算パイプライン:
  - `calculate_damage`: 通常ダメージ + 増幅反応(蒸発/溶解) + 激化反応(超激化/草激化)
  - `calculate_transformative`: 固定反応（過負荷、超電導、拡散等）— 会心なし、防御なし
  - `calculate_lunar`: 月反応（月感電、月開花、月結晶）— 会心あり、防御なし、BaseDMGBonus乗算
- `damage.rs`: メイン計算（ATK×倍率→バフ→防御→耐性→増幅/激化）
- `reaction.rs`: 元素反応タイプ判定、倍率テーブル
- `em.rs`: 元素熟知ボーナス計算（4種の式）
- `level_table.rs`: レベル基礎値テーブル（Lv1-100、データマイニング値）
- `transformative.rs`: 固定反応ダメージ計算
- `lunar.rs`: 月反応ダメージ計算
- `stat_profile.rs`: ステータス合算（StatProfile → Stats）
- `buff_types.rs`: BuffableStat（バフ対象ステータス型 — data crateから移動）
- `team.rs`: チームバフ解決（BuffTarget, ResolvedBuff, TeamMember, resolve_team_stats）
- `resonance.rs`: 元素共鳴判定・バフ生成（ElementalResonance, determine_resonances）
- `moonsign.rs`: 月兆システム（MoonsignLevel, MoonsignContext, 非月兆バフ, 貢献度合算, タレント強化適用）
- `DamageInput`/`LunarInput`/`TeamMember`変更時は全構築箇所（テスト・docコメント・README・examples含む）を一括修正すること（コンパイル不能防止）

### Data Crate (`crates/data`)
- v5.8全ゲームデータをRust定数として実装
- `types.rs`: CharacterData, WeaponData, ArtifactSet, EnemyData等の型定義
- `buff.rs`: BuffableStat(core re-export), StatBuff, PassiveEffect（武器パッシブ・聖遺物バフ）
- `talent_buffs.rs`: 天賦バフ定義（TalentBuffDef, find_talent_buffs — 9キャラ分）
- `team_builder.rs`: TeamMemberBuilder（キャラ+武器+聖遺物→TeamMember構築）
- `moonsign_chars.rs`: 月兆キャラデータ（9キャラの月光の祝福パッシブ + Laumaタレント強化）
- `characters/`: 元素別ファイル（pyro/hydro/electro/cryo/dendro/anemo/geo）— 102キャラ
- `weapons/`: 武器種別ファイル（sword/claymore/polearm/bow/catalyst）— 230武器
- `artifacts.rs`: 52聖遺物セット
- `enemies.rs`: 12耐性テンプレート + 15敵データ + `to_enemy()`変換
- `lib.rs`: 検索API（find_character/weapon/artifact_set/enemy + filter関数）
- 数値単位: 全パーセンテージは小数形式（10.8% → 0.108）
- `&'static`参照型はSerializeのみ（Deserializeなし）

## Testing
- core: 173ユニットテスト + 統合テスト2種（character_verification 153ケース + moonsign_integration 4テスト）
- data: 61テスト（検索API、serde roundtrip、データ整合性、core統合、チーム統合）
- v0.3.0でStatProfile合算 + ScalingStatテスト追加
- ゲーム検証済みキャラ: Freminet（完全一致）、Diluc、Ganyu、Raiden、Yanfei蒸発
- goldenテスト: 手計算値との照合（各モジュールに `test_golden_*` テスト）
- データ駆動テスト: `tests/data/characters/*.toml` に102キャラ・153ケース（v5.8全キャラ対応）
  - `cargo test --test character_verification` で実行
  - 新キャラ追加: TOMLファイル1つ追加するだけ
- 許容誤差: 通常 `< 0.01`、ゲーム検証は `< 1.0`（floor丸め考慮）
