# Moonsign System Design

## Overview

月兆（Moonsign）システムをgenshin-calcに実装する。ナドクライキャラが持つ「月光の祝福」パッシブによる月反応BaseDMGBonus、満照時の非月兆キャラバフ、月兆レベル依存タレント強化、および複数キャラの貢献度合算を含む完全なパイプライン。

## Scope

- 月兆レベル判定（初照/満照）
- 月光の祝福パッシブによるBaseDMGBonus → `LunarInput` への統合
- 満照時の非月兆キャラによるLunar DMG Bonus（元素依存、最大36%）
- 個別キャラの月兆レベル依存タレント強化（Lauma開花会心化）
- 貢献度合算（`A1 + A2/2 + A3/12 + A4/12`）

### Out of Scope (v1)

- **Illuga の月兆レベル依存タレント強化**: EM × パーティ内Hydro/Geoキャラ数でスケーリングする効果は、現在の `MoonsignTalentEffect` enum では表現不可。将来の拡張ポイントとして残す。
- **Aino の爆発頻度/AoE強化**: ダメージ計算に影響しない。
- **Jahoda の満照効果**: 詳細データが不十分。

## Architecture

### Approach: LunarInput拡張 + 独立モジュール

- `core/moonsign.rs` に月兆ロジックを集約（純粋計算）
- `data/moonsign_chars.rs` に9キャラのデータ定義
- 既存の `team.rs` / `lunar.rs` への変更は最小限

### New Files

- `crates/core/src/moonsign.rs` — MoonsignLevel, MoonsignContext, 非月兆バフ計算, 貢献度合算
- `crates/data/src/moonsign_chars.rs` — 9キャラの月光の祝福データ + タレント強化

### Modified Files

- `crates/core/src/lunar.rs` — `LunarInput` に `base_dmg_bonus` 追加、計算式更新
- `crates/core/src/team.rs` — `TeamMember` に `is_moonsign` 追加、`TeamResolveResult` に `MoonsignContext` 追加
- `crates/core/src/types.rs` — `ScalingStat` に `Em` バリアント追加
- `crates/core/src/lib.rs` — `moonsign` モジュール公開
- `crates/data/src/lib.rs` — `moonsign_chars` モジュール公開
- `crates/data/src/team_builder.rs` — `is_moonsign` 自動設定

## Design Details

### 1. ScalingStat Extension

`ScalingStat` に `Em` バリアントを追加。月兆の月光の祝福パッシブ（Lauma/Nefer）や非月兆バフ（Anemo/Dendro）がEMスケーリングを使うため。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ScalingStat {
    #[default]
    Atk,
    Hp,
    Def,
    Em,  // NEW
}
```

### 2. core/moonsign.rs — Types

```rust
/// 月兆レベル
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MoonsignLevel {
    None,
    NascentGleam,   // 初照（月兆キャラ1人）
    AscendantGleam, // 満照（月兆キャラ2人以上）
}

/// 月兆キャラの月光の祝福パッシブ情報（計算済み）
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoonsignBenediction {
    pub base_dmg_bonus: f64,
    pub enabled_reactions: Vec<Reaction>,
}

/// チーム全体の月兆コンテキスト
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoonsignContext {
    pub level: MoonsignLevel,
    /// 反応タイプ別の合算BaseDMGBonus。Vec で保持（WASM互換性のためHashMap不使用）。
    pub base_dmg_bonus_by_reaction: Vec<(Reaction, f64)>,
    /// 非月兆キャラからの月反応DMGボーナス（満照時のみ、最大0.36）。
    pub non_moonsign_lunar_bonus: f64,
    /// 月兆レベル依存タレント強化のリスト。
    pub talent_enhancements: Vec<MoonsignTalentEnhancement>,
}
```

`base_dmg_bonus_by_reaction` のルックアップヘルパー:

```rust
impl MoonsignContext {
    pub fn base_dmg_bonus_for(&self, reaction: Reaction) -> f64 {
        self.base_dmg_bonus_by_reaction
            .iter()
            .find(|(r, _)| *r == reaction)
            .map(|(_, v)| *v)
            .unwrap_or(0.0)
    }
}
```

### 3. MoonsignLevel Determination

```rust
pub fn determine_moonsign_level(moonsign_count: usize) -> MoonsignLevel {
    match moonsign_count {
        0 => MoonsignLevel::None,
        1 => MoonsignLevel::NascentGleam,
        _ => MoonsignLevel::AscendantGleam,
    }
}
```

### 4. MoonsignContext Resolution

`resolve_moonsign_context` はチームレベルの関数として独立させる（target_indexに依存しない）。

```rust
/// チーム全体の月兆コンテキストを解決する。
/// benedictions: 各月兆キャラの計算済みパッシブ情報
/// non_moonsign_bonus: 満照時の非月兆キャラバフ値（事前計算済み）
/// enhancements: 適用可能なタレント強化
pub fn resolve_moonsign_context(
    moonsign_count: usize,
    benedictions: &[MoonsignBenediction],
    non_moonsign_bonus: f64,
    enhancements: Vec<MoonsignTalentEnhancement>,
) -> MoonsignContext
```

BaseDMGBonusの合算: 同一反応タイプの `base_dmg_bonus` を加算。
例: Ineffa (0.14) + Columbina (0.07) → LunarElectroCharged = 0.21

### 5. LunarInput Extension

```rust
pub struct LunarInput {
    pub character_level: u32,
    pub elemental_mastery: f64,
    pub reaction: Reaction,
    pub reaction_bonus: f64,
    pub crit_rate: f64,
    pub crit_dmg: f64,
    pub base_dmg_bonus: f64,  // NEW: default 0.0
}
```

Updated formula:

```rust
let non_crit = level_base * reaction_mult
    * (1.0 + input.base_dmg_bonus)
    * (1.0 + em_bonus + input.reaction_bonus)
    * res_mult;
```

**Formula justification**: `(1 + BaseDMGBonus)` は EM/反応ボーナスとは独立した乗算項。
KQM Lunar Reactions Guide (https://keqingmains.com/misc/lunar-reactions/) および
日本語Wiki ダメージ計算式 (https://wikiwiki.jp/genshinwiki/ダメージ計算式) で確認済み。
公式: `DMG = LevelMult × ReactionCoeff × (1 + BaseDMGBonus) × (1 + EMBonus + ReactionBonus) × RESMult`

`base_dmg_bonus` defaults to `0.0`, so existing tests pass without change.

### 6. Contribution Combination

```rust
const CONTRIBUTION_WEIGHTS: [f64; 4] = [1.0, 0.5, 1.0 / 12.0, 1.0 / 12.0];

pub struct LunarContribution {
    pub input: LunarInput,
}

/// 複数キャラの月反応貢献度を加重合算する。
///
/// # Errors
/// - `CalcError::InvalidTeamSize` if contributions is empty or > 4
/// - Propagates errors from `calculate_lunar`
pub fn calculate_lunar_team(
    contributions: &[LunarContribution],
    enemy: &Enemy,
) -> Result<LunarResult, CalcError>
```

**Validation**: `contributions.len()` must be 1..=4. Empty or > 4 returns `CalcError::InvalidTeamSize`.

Process:
1. Validate contribution count (1-4)
2. Calculate `calculate_lunar()` for each contribution
3. Sort by `average` descending
4. Apply weights: `final = sum(result_i.field * weight_i)` for non_crit, crit, average
5. Single contribution degrades to normal `calculate_lunar`

### 7. Non-Moonsign Character Lunar Buff

At Ascendant Gleam, non-Moonsign characters provide Lunar Reaction DMG bonus (added to `reaction_bonus`).

```rust
pub struct NonMoonsignLunarBuff {
    pub scaling_stat: ScalingStat,
    pub rate: f64,
    pub max_bonus: f64,  // always 0.36
}

pub fn non_moonsign_scaling(element: Element) -> NonMoonsignLunarBuff
```

All rates normalized to "per 1 unit of stat" (same convention as MoonsignBenedictionDef):

| Element | Scaling Stat | Rate (per 1 stat) | Max |
|---------|-------------|------|-----|
| Pyro / Electro / Cryo | Atk | 0.00009 | 0.36 |
| Hydro | Hp | 0.000006 | 0.36 |
| Geo | Def | 0.0001 | 0.36 |
| Anemo / Dendro | Em | 0.000225 | 0.36 |

**Canonical formula** (same pattern as `calculate_benediction_bonus`):

```rust
pub fn calculate_non_moonsign_bonus(buff: &NonMoonsignLunarBuff, stat_value: f64) -> f64 {
    (buff.rate * stat_value).min(buff.max_bonus)
}
```

Only one non-Moonsign buff applies (no stacking). Helper for selection:

```rust
/// 非月兆キャラの中から最大のLunar DMGボーナスを選択する。
/// members: (element, stat_value) のペア。stat_valueはその元素のスケーリング先のステータス最終値。
pub fn select_non_moonsign_buff(members: &[(Element, f64)]) -> f64 {
    members.iter()
        .map(|(elem, stat)| {
            let buff = non_moonsign_scaling(*elem);
            calculate_non_moonsign_bonus(&buff, *stat)
        })
        .fold(0.0_f64, f64::max)
}
```

### 8. Moonsign Character Data (data crate)

```rust
pub struct MoonsignBenedictionDef {
    pub character_name: &'static str,
    pub enabled_reactions: &'static [Reaction],
    pub scaling_stat: Option<ScalingStat>,
    pub rate: f64,       // normalized: per 1 unit of stat
    pub max_bonus: f64,
}
```

**Canonical formula for computing BaseDMGBonus:**

```rust
pub fn calculate_benediction_bonus(def: &MoonsignBenedictionDef, stat_value: f64) -> f64 {
    (def.rate * stat_value).min(def.max_bonus)
}
```

All rates are normalized to "per 1 unit of stat":

| Character | Enabled Reactions | Scaling | Rate (per 1 stat) | Max |
|-----------|------------------|---------|-----|-----|
| Ineffa | LunarElectroCharged | Atk | 0.00007 | 0.14 |
| Flins | LunarElectroCharged | Atk | 0.00007 | 0.14 |
| Lauma | LunarBloom | Em | 0.000175 | 0.14 |
| Nefer | LunarBloom | Em | 0.000175 | 0.14 |
| Zibai | LunarCrystallize | Def | 0.00007 | 0.14 |
| Columbina | All lunar | Hp | 0.000002 | 0.07 |
| Aino | (none) | — | 0.0 | 0.0 |
| Jahoda | (none) | — | 0.0 | 0.0 |
| Illuga | (none) | — | 0.0 | 0.0 |

`data` crate provides `fn calculate_benediction(def, stats) -> MoonsignBenediction` that takes the def and character stats, computes the bonus, and returns the `core::MoonsignBenediction` type.

### 9. Moonsign-Level Dependent Talent Enhancements

```rust
pub struct MoonsignTalentEnhancement {
    pub character_name: &'static str,
    pub required_level: MoonsignLevel,
    pub description: &'static str,
    pub effect: MoonsignTalentEffect,
}

pub enum MoonsignTalentEffect {
    /// 月反応に会心を付与（例: Laumaの開花会心化）
    GrantReactionCrit {
        reaction: Reaction,
        crit_rate: f64,
        crit_dmg: f64,
    },
    /// ステータスバフ
    StatBuff {
        stat: BuffableStat,
        value: f64,
        target: BuffTarget,
    },
}
```

**Helper for applying enhancements to LunarInput:**

```rust
/// MoonsignTalentEnhancementをLunarInputに適用する。
/// 該当する GrantReactionCrit があれば crit_rate/crit_dmg を加算する。
pub fn apply_moonsign_enhancements(
    input: &LunarInput,
    enhancements: &[MoonsignTalentEnhancement],
) -> LunarInput {
    let mut result = input.clone();
    for enh in enhancements {
        if let MoonsignTalentEffect::GrantReactionCrit { reaction, crit_rate, crit_dmg } = &enh.effect {
            if *reaction == input.reaction {
                result.crit_rate = (result.crit_rate + crit_rate).min(1.0);
                result.crit_dmg += crit_dmg;
            }
        }
    }
    result
}
```

**v1 implementations:**
- **Lauma** (NascentGleam): LunarBloom gains crit (CR +0.15, CD +1.0)

**Deferred (v2 — requires enum extension):**
- **Illuga** (AscendantGleam): Lunar-Crystallize DMG bonus based on EM × Hydro/Geo party member count. Requires a new `MoonsignTalentEffect` variant that takes party composition as input.

**Excluded (not modelable):**
- Aino: Burst frequency / AoE increase
- Jahoda: Details insufficient

### 10. Integration Pipeline

```
TeamMember[] (with is_moonsign flag)
    │
    ├── resolve_moonsign_context()  ← team-level, computed once
    │     ├── MoonsignLevel判定 (count is_moonsign members)
    │     ├── 月光の祝福 → BaseDMGBonus合算（反応タイプ別）
    │     ├── 非月兆キャラ → select_non_moonsign_buff()
    │     └── 月兆レベル依存タレント強化 収集
    │     → MoonsignContext
    │
    ├── resolve_team_stats_detailed()
    │     → TeamResolveResult (includes MoonsignContext)
    │
    ▼
MoonsignContext
    │
    ▼
LunarInput (per character)
    ├── base_dmg_bonus ← context.base_dmg_bonus_for(reaction)
    ├── reaction_bonus += context.non_moonsign_lunar_bonus
    └── apply_moonsign_enhancements() ← Lauma crit etc.
    │
    ▼
calculate_lunar_team(contributions[1..=4], enemy)
    ├── Per-character: calculate_lunar()
    ├── Sort by average descending
    └── Weighted sum: A1 + A2/2 + A3/12 + A4/12
    → Final LunarResult
```

## Testing Strategy

- Unit tests for `determine_moonsign_level`
- Unit tests for BaseDMGBonus calculation with various stat values and caps
- Unit tests for `calculate_non_moonsign_bonus` per element type
- Unit tests for `select_non_moonsign_buff` with multiple non-Moonsign members
- Unit tests for contribution combination weights (1, 2, 3, 4 contributors)
- Unit tests for `apply_moonsign_enhancements` (Lauma crit application)
- Unit tests for `calculate_lunar_team` validation (empty, >4 → error)
- Integration tests: full team → MoonsignContext → LunarInput → calculate_lunar_team
- **Golden tests**: hand-calculated values for known team compositions, including at least one test verified against in-game data or KQM calculations
- Existing lunar.rs tests must pass unchanged (base_dmg_bonus defaults to 0.0)

## Sources

- KQM Lunar Reactions Guide: https://keqingmains.com/misc/lunar-reactions/
- 日本語Wiki ダメージ計算式: https://wikiwiki.jp/genshinwiki/ダメージ計算式
- Game8 月兆レベル解説: https://game8.jp/genshin/717692
- GameWith 月光の祝福解説: https://gamewith.jp/genshin/article/show/517481

## Design Principles

- **Immutability**: All structs are immutable, functions return new values
- **Pure functions**: No side effects, deterministic output
- **Separation of concerns**: core = pure calculation, data = game constants
- **Backward compatibility**: `base_dmg_bonus: 0.0` preserves existing behavior
- **WASM compatible**: `Vec` instead of `HashMap` for public structs
- **Normalized rates**: All scaling rates stored as "per 1 unit of stat" for uniform formula
