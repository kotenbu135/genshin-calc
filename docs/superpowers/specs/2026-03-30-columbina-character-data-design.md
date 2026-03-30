# Columbina (Hydro/Catalyst/★5) Character Data Implementation

## Overview

v6.3 (Luna IV) の新キャラクター Columbina のゲームデータを genshin-calc に実装する。
HP参照スケーリングが主体のHydro Catalystキャラ。月兆Benedictionは実装済み。

## Scope

- Columbina の CharacterData 定義（`characters/hydro/columbina.rs`）
- `hydro/mod.rs` への登録
- TOML テストケース作成
- 天賦バフ（TalentBuffDef）追加
- 月兆 TalentEnhancement 追加（Benediction は実装済み）

## Data Source

Honey Impact (`gensh.honeyhunterworld.com/columbina/?lang=EN`)

## Design

### 1. CharacterData 定義

`crates/data/src/characters/hydro/columbina.rs` に新規ファイル作成。

基本情報:
- `id: "columbina"`, `name: "Columbina"`
- `element: Hydro`, `weapon_type: Catalyst`, `rarity: Star5`, `region: NodKrai`
- `base_hp: [1144.0, 2967.0, 12965.0, 14695.0]`
- `base_atk: [7.45, 19.32, 84.41, 95.67]`
- `base_def: [40.09, 103.98, 454.31, 514.93]`
- `ascension_stat: AscensionStat::CritRate(0.192)`
- `constellation_pattern: ConstellationPattern::C3SkillC5Burst`

天賦スケーリング特記事項:
- **通常攻撃 (Moondew Cascade)**: ATK参照、`damage_element: Some(Element::Hydro)` (Catalyst)
  - 3段構成 (1-Hit, 2-Hit, 3-Hit)
- **重撃**: 2系統
  - ATK参照の通常重撃ダメージ
  - HP参照の Moondew Cleanse DMG (×3hit) → `ScalingStat::Hp`, 別TalentScaling
- **落下攻撃**: ATK参照、標準3種（落下/低空/高空）
- **元素スキル (Eternal Tides)**: 全てHP参照 (`ScalingStat::Hp`)
  - Skill DMG
  - Gravity Ripple Continuous DMG
  - Gravity Interference: Lunar-Charged DMG
  - Gravity Interference: Lunar-Bloom DMG (×5hit)
  - Gravity Interference: Lunar-Crystallize DMG
- **元素爆発 (Moonlit Melancholy)**: HP参照
  - Skill DMG

爆発の「Lunar Reaction DMG Bonus」(13%-55%) は天賦バフとして Step 4 で実装（スケーリング値ではなくバフ効果のため）。

### 2. mod 登録

`crates/data/src/characters/hydro/mod.rs` に3箇所追加:
- `mod columbina;`
- `pub use columbina::COLUMBINA;`
- `CHARACTERS` スライスにアルファベット順で追加

### 3. TOML テスト

`crates/core/tests/data/characters/columbina.toml` を作成:

最低ケース:
1. **通常攻撃1段** — Hydro, ATK参照, 反応なし
2. **元素スキル Skill DMG** — Hydro, HP参照, 反応なし
3. **蒸発 (Vaporize)** — スキルダメージ × 2.0倍率

### 4. 天賦バフ

`crates/data/src/talent_buffs/hydro.rs` に Columbina セクション追加:

実装対象:
- **爆発 Lunar Reaction DMG Bonus**: `BuffableStat::TransformativeBonus`, Lv10で40%、チーム対象
  - `scales_with_talent: true`, 爆発レベル依存

スコープ外（初回）:
- C1 Lunar Reaction DMG +1.5% — 小さすぎて実用的でない
- C2 Max HP +40%, ATK/EM/DEF buff — 条件複雑（月反応タイプ依存）
- C4 Gravity Interference DMG増 — proc damage扱い
- C6 CRIT DMG +80% — 条件複雑（元素別、同元素不可）

### 5. 月兆 TalentEnhancement

`crates/data/src/moonsign_chars.rs` に `COLUMBINA_TALENT_ENHANCEMENTS` 追加。

Columbina のA4パッシブ「Law of the New Moon」の効果を TalentEnhancement として定義。
具体的な効果は Honey Impact のデータから確認して実装。

既存の Benediction (`rate: 0.000002`, `max_bonus: 0.07`, 全3反応対応) は実装済みで変更不要。

### 6. 検証

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

## Notes

- base_atk が 95.67 (Lv90) と非常に低い — HP特化キャラのため正常
- Honey Impact の数値は beta/pre-release の可能性あり。リリース後に要検証
