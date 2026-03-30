# Architecture Detail

## Core Crate (`crates/core`)

データに依存しない純粋な計算ロジック。公開APIは最小限に保つ。

### Files
- `stats.rs`: 最終ステータス構造体（HP/ATK/DEF/会心等）
- `error.rs`: CalcError列挙型（バリデーションエラー等）
- `types.rs`: Element, ScalingStat(Atk/Hp/Def/Em), WeaponType等の共有型定義
- `enemy.rs`: 敵パラメータ（レベル、耐性、防御減少）
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

### Calculation Pipelines
- `calculate_damage`: 通常ダメージ + 増幅反応(蒸発/溶解) + 激化反応(超激化/草激化)
- `calculate_transformative`: 固定反応（過負荷、超電導、拡散等）— 会心なし、防御なし
- `calculate_lunar`: 月反応（月感電、月開花、月結晶）— 会心あり、防御なし、BaseDMGBonus乗算

## Data Crate (`crates/data`)

v5.8全ゲームデータ + v6.0-6.1一部（Lauma/Flins/Nefer）をRust定数として実装。

### Files
- `types.rs`: CharacterData, WeaponData, ArtifactSet, EnemyData等の型定義
- `buff.rs`: BuffableStat(core re-export), StatBuff, PassiveEffect（武器パッシブ・聖遺物バフ）
- `talent_buffs.rs`: 天賦バフ定義（TalentBuffDef, find_talent_buffs）
- `team_builder.rs`: TeamMemberBuilder（キャラ+武器+聖遺物→TeamMember構築）
- `moonsign_chars.rs`: 月兆キャラデータ（月光の祝福パッシブ + タレント強化）
- `characters/`: 元素別ファイル（pyro/hydro/electro/cryo/dendro/anemo/geo）
- `weapons/`: 武器種別ファイル（sword/claymore/polearm/bow/catalyst）
- `artifacts.rs`: 聖遺物セット
- `enemies.rs`: 耐性テンプレート + 敵データ + `to_enemy()`変換
- `lib.rs`: 検索API（find_character/weapon/artifact_set/enemy + filter関数）
