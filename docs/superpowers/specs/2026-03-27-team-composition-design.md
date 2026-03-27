# チーム編成バフ解決エンジン 設計

## 概要

core crateにチーム編成のバフ解決機能を追加。1-4人のチームメンバーが提供するバフを収集・解決し、指定メンバーの最終Statsを算出する。

## 背景

- ロードマップ「チーム編成・ローテーションシミュレーション」の前半
- ローテーション（時間軸シミュレーション）は範囲外
- 現在の計算パイプラインは単体キャラのみ対応

## スコープ

### IN
- チームメンバー構造体（1-4人対応）
- バフ解決パイプライン（自己バフ/チームバフ/チーム除外自己バフ）
- 元素共鳴の自動判定・バフ生成（4人パーティのみ）
- data crateにTeamMemberBuilder（キャラ+武器+聖遺物→TeamMember構築）
- data crateに天賦バフ定義（初期10キャラ分）
- WeaponType をcore crateに移動

### OUT
- ローテーション/時間軸シミュレーション
- 元素付着/ゲージ管理
- 92キャラ分のバフデータ（後で段階的に追加）
- 命ノ星座の全効果（バフに関係するもののみ対応）
- 条件付きバフの自動判定（ユーザーが有効/無効を指定）

---

## 1. Core Crate: データモデル

### 1.1 types.rs への追加

`WeaponType` を data crate から core に移動:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Claymore,
    Polearm,
    Bow,
    Catalyst,
}
```

data crate では `pub use genshin_calc_core::WeaponType;` で re-export。
既存の `data/src/types.rs` から `WeaponType` enum定義を削除し、`data/src/weapons/*.rs` 等で
`crate::types::WeaponType` を参照している箇所は `genshin_calc_core::WeaponType`（re-export経由）に自動的に解決される。

### 1.2 team.rs（新規）

```rust
/// Target of a buff effect.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuffTarget {
    /// Applies to the buff provider only (e.g. weapon passives).
    OnlySelf,
    /// Applies to all team members (e.g. Bennett burst).
    Team,
    /// Applies to all team members except the provider.
    TeamExcludeSelf,
}

/// A resolved buff from a team member, weapon, artifact, or resonance.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedBuff {
    /// Name of the buff source (e.g. "Bennett Burst", "Noblesse 4pc").
    pub source: String,
    /// Which stat is buffed.
    pub stat: BuffableStat,
    /// Buff value in decimal form.
    pub value: f64,
    /// Who receives this buff.
    pub target: BuffTarget,
}

/// A team member with pre-resolved stats and buffs.
#[derive(Debug, Clone, PartialEq)]
pub struct TeamMember {
    /// Character element.
    pub element: Element,
    /// Weapon type.
    pub weapon_type: WeaponType,
    /// Base stats before team buffs (character + weapon + artifacts).
    pub stats: StatProfile,
    /// Buffs this member provides to the team.
    /// Built from weapon passives (OnlySelf), artifact set effects (OnlySelf),
    /// and talent buffs (target from TalentBuffDef).
    pub buffs_provided: Vec<ResolvedBuff>,
}

// Note: 既存の StatBuff (data crate) には target フィールドがない。
// ビルダーが StatBuff → ResolvedBuff に変換する際、武器パッシブと聖遺物セット効果は
// 常に BuffTarget::OnlySelf として扱う。天賦バフは TalentBuffDef.target を使用。

/// Detailed result of team buff resolution.
#[derive(Debug, Clone, PartialEq)]
pub struct TeamResolveResult {
    /// Stats before team buffs.
    pub base_stats: Stats,
    /// All buffs applied to the target member.
    pub applied_buffs: Vec<ResolvedBuff>,
    /// Active elemental resonances.
    pub resonances: Vec<ElementalResonance>,
    /// Final stats after all buffs.
    pub final_stats: Stats,
}
```

### 1.3 公開関数

```rust
/// Resolves team buffs and returns final stats for the target member.
///
/// Collects buffs from all team members, applies elemental resonances
/// (if 4 members), and computes final stats for the member at `target_index`.
///
/// Note: DamageType/Element-dependent buffs (NormalAtkDmgBonus, ElementalDmgBonus, etc.)
/// are NOT included in the returned Stats. Use `resolve_team_stats_detailed()` to see
/// which conditional buffs were collected but not applied.
pub fn resolve_team_stats(
    team: &[TeamMember],
    target_index: usize,
) -> Result<Stats, CalcError>;

/// Resolves team buffs with detailed breakdown.
///
/// `applied_buffs` contains all buffs including conditional ones.
/// `final_stats` only includes unconditional buffs.
/// Conditional buffs (DamageType/Element-dependent) are in `applied_buffs`
/// but NOT reflected in `final_stats` — caller must apply them manually
/// based on the DamageInput being calculated.
pub fn resolve_team_stats_detailed(
    team: &[TeamMember],
    target_index: usize,
) -> Result<TeamResolveResult, CalcError>;
```

### 1.4 バフ適用ヘルパー

StatProfile はイミュータブル設計のため、cloneした上でバフを適用する:

```rust
/// Applies a list of resolved buffs to a StatProfile, returning a new one.
///
/// Only applies unconditional buffs (HpPercent, AtkPercent, AtkFlat, CritRate, etc.).
/// DamageType/Element-dependent buffs are skipped.
pub fn apply_buffs_to_profile(
    profile: &StatProfile,
    buffs: &[ResolvedBuff],
) -> StatProfile;
```

---

## 2. Core Crate: 元素共鳴

### 2.1 resonance.rs（新規）

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementalResonance {
    /// Pyro ×2: ATK +25%.
    FerventFlames,
    /// Hydro ×2: HP +25%.
    SoothingWater,
    /// Electro ×2: Electro particle generation (no stat effect).
    HighVoltage,
    /// Cryo ×2: CRIT Rate +15% against frozen/cryo-affected enemies (conditional).
    ShatteringIce,
    /// Anemo ×2: Stamina -15%, move speed +10%, CD -5% (no stat effect).
    ImpetuousWinds,
    /// Geo ×2: Shield strength +15%, DMG +15% while shielded (conditional).
    EnduringRock,
    /// Dendro ×2: EM +50, additional EM after reactions.
    SprawlingGreenery,
    /// 4 unique elements: All RES +15% (no damage stat effect).
    ProtectiveCanopy,
}
```

### 2.2 共鳴判定ルール

- チームが4人の場合のみ発動
- 同元素2人以上で対応する共鳴が発動
- 4つの異なる元素 → ProtectiveCanopy（他の共鳴とは排他的）
- 同元素共鳴は複数同時に発動可能（例: 炎×2 + 水×2 の4人パーティ）
- ProtectiveCanopy は全4元素が異なる場合のみ（同元素2+があれば発動しない）

### 2.3 共鳴バフマッピング

| 共鳴 | BuffableStat | value | 条件 |
|------|-------------|-------|------|
| FerventFlames | AtkPercent | 0.25 | なし |
| SoothingWater | HpPercent | 0.25 | なし |
| ShatteringIce | CritRate | 0.15 | 凍結/氷元素付着の敵（条件付き） |
| EnduringRock | DmgBonus | 0.15 | シールド中（条件付き） |
| SprawlingGreenery | ElementalMastery | 50.0 | なし |
| HighVoltage | — | — | ステータス影響なし |
| ImpetuousWinds | — | — | ステータス影響なし |
| ProtectiveCanopy | — | — | ダメージステータスに影響なし |

条件付きバフ（ShatteringIce, EnduringRock）はデフォルト無効。ユーザーがResolvedBuffとして手動追加する形で対応。

### 2.4 公開関数

```rust
/// Determines elemental resonances from team member elements.
///
/// Returns empty if team has fewer than 4 members.
pub fn determine_resonances(elements: &[Element]) -> Vec<ElementalResonance>;

/// Returns stat buffs for a resonance.
///
/// Returns empty for resonances with no stat effect
/// (HighVoltage, ImpetuousWinds, ProtectiveCanopy)
/// or conditional resonances (ShatteringIce, EnduringRock).
pub fn resonance_buffs(resonance: ElementalResonance) -> Vec<ResolvedBuff>;
```

---

## 3. バフ解決パイプライン

### 3.1 処理フロー

```
Input: team: &[TeamMember], target_index: usize

1. team[target_index] の stats (StatProfile) をベースとして取得
2. 各メンバーの buffs_provided をイテレート:
   - BuffTarget::OnlySelf → target_index のメンバー自身が提供者の場合のみ適用
   - BuffTarget::Team → 常に適用
   - BuffTarget::TeamExcludeSelf → 提供者 ≠ target_index の場合のみ適用
3. team.len() == 4 の場合:
   - determine_resonances() で共鳴判定
   - resonance_buffs() で共鳴バフ生成
   - 共鳴バフを適用バフリストに追加
4. 適用バフをStatProfileの対応フィールドに加算
5. combine_stats() で最終Stats算出
6. Result 返却
```

### 3.2 バフの StatProfile への適用

BuffableStat → StatProfile フィールドのマッピング:

| BuffableStat | StatProfile フィールド |
|-------------|---------------------|
| HpPercent | hp_percent |
| AtkPercent | atk_percent |
| DefPercent | def_percent |
| HpFlat | hp_flat |
| AtkFlat | atk_flat |
| DefFlat | def_flat |
| CritRate | crit_rate |
| CritDmg | crit_dmg |
| ElementalMastery | elemental_mastery |
| EnergyRecharge | energy_recharge |
| DmgBonus | dmg_bonus |
| ElementalDmgBonus(_) | dmg_bonus（対象元素一致時のみ） |
| PhysicalDmgBonus | dmg_bonus（物理ダメ時のみ） |
| NormalAtkDmgBonus | dmg_bonus（通常攻撃時のみ） |
| その他のDmgBonus系 | dmg_bonus（対応条件時のみ） |

注: DamageType/Element依存のバフ（NormalAtkDmgBonus, ElementalDmgBonus等）は、計算時のDamageInput.damage_type/elementと照合が必要。resolve_team_statsにはDamageInputの情報がないため、これらは `applied_buffs` リストに含めるが、StatProfileへの自動加算はDamageType/Element非依存のバフのみ行う。DamageType/Element依存のバフはユーザーが手動でdmg_bonusに加算するか、将来の拡張で対応。

---

## 4. Data Crate: 天賦バフ定義

### 4.1 新しい型

```rust
/// Source of a talent buff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TalentBuffSource {
    /// Ascension passive (A1 or A4).
    AscensionPassive,
    /// Elemental skill.
    ElementalSkill,
    /// Elemental burst.
    ElementalBurst,
    /// Constellation effect.
    Constellation(u8),
}

/// Definition of a talent buff for a character.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentBuffDef {
    /// Buff name (e.g. "Fantastic Voyage ATK Bonus").
    pub name: &'static str,
    /// Description of the buff.
    pub description: &'static str,
    /// Stat being buffed.
    pub stat: BuffableStat,
    /// Buff value (fixed or level-dependent via callback).
    pub base_value: f64,
    /// Whether the buff scales with talent level.
    pub scales_with_talent: bool,
    /// Talent level scaling values [Lv1..Lv15]. None if fixed.
    pub talent_scaling: Option<&'static [f64; 15]>,
    /// Who receives the buff.
    pub target: BuffTarget,
    /// Where the buff comes from.
    pub source: TalentBuffSource,
    /// Minimum constellation required (0 = no constellation needed).
    pub min_constellation: u8,
}
```

### 4.2 初期10キャラのバフ定義

各キャラの `talent_buffs` フィールドを `CharacterData` に追加するのではなく、別ファイル `talent_buffs.rs` に定義してキャラIDで検索可能にする:

```rust
pub fn find_talent_buffs(character_id: &str) -> Option<&'static [TalentBuffDef]>;
```

| キャラ | バフ | stat | target | source |
|--------|------|------|--------|--------|
| Bennett | 爆発ATKバフ | AtkFlat | Team | ElementalBurst |
| Kazuha | A4元素ダメバフ | DmgBonus | Team | AscensionPassive |
| Nahida | A4 EMバフ | ElementalMastery | Team | AscensionPassive |
| Shenhe | スキル氷フラット加算 | AtkFlat | Team | ElementalSkill |
| Zhongli | シールド耐性ダウン | — | — | ElementalSkill |
| Yun Jin | 爆発通常攻撃バフ | AtkFlat | Team | ElementalBurst |
| Mona | 爆発ダメバフ | DmgBonus | Team | ElementalBurst |
| Sara | スキルATKバフ | AtkFlat | Team | ElementalSkill |
| Rosaria | A4会心率共有 | CritRate | TeamExcludeSelf | AscensionPassive |
| Furina | 爆発ダメバフ | DmgBonus | Team | ElementalBurst |

注: Zhongliの耐性ダウンはステータスバフではなく敵の耐性変更。これは `ResolvedBuff` ではなく別の仕組み（`EnemyDebuff` 型等）が必要。v1では Zhongli の耐性ダウンは TeamMember の buffs_provided ではなく、ユーザーが Enemy.resistance を手動調整する形で対応。

---

## 5. Data Crate: TeamMemberBuilder

### 5.1 ビルダー

```rust
pub struct TeamMemberBuilder {
    character: &'static CharacterData,
    weapon: &'static WeaponData,
    artifact_set: Option<&'static ArtifactSet>,
    artifact_stats: StatProfile,
    constellation: u8,
    talent_levels: [u8; 3],
}
```

### 5.2 build() の処理

署名: `pub fn build(self) -> Result<TeamMember, genshin_calc_core::CalcError>`

エラーケース:
- constellation > 6
- talent_levels の各値が 1-15 の範囲外

処理フロー:
1. キャラ基礎ステータス（Lv90）+ 武器基礎ATK → StatProfile の base_hp/base_atk/base_def
2. キャラ昇格ステータス → StatProfile の対応フィールド
3. 武器サブステータス → StatProfile の対応フィールド
4. 聖遺物ステータス（ユーザー入力）→ StatProfile にマージ
5. 武器パッシブ → buffs_provided に OnlySelf バフとして追加（StatBuff → ResolvedBuff変換）
6. 聖遺物2セット効果 → buffs_provided に OnlySelf バフとして追加
7. 聖遺物4セット効果 → buffs_provided に OnlySelf バフとして追加
8. 天賦バフ（find_talent_buffs） → constellation と talent_levels でフィルタ → buffs_provided に追加

---

## 6. エラーハンドリング

CalcError に追加するバリアント:

```rust
/// Team must have 1-4 members.
InvalidTeamSize(usize),
/// Target index is out of bounds.
InvalidTargetIndex { index: usize, team_size: usize },
```

---

## 7. テスト方針

### core テスト
- 元素共鳴判定: 全パターン（炎×2、水×2、4元素等）
- 共鳴バフ生成: 各共鳴のバフ値確認
- バフ解決: OnlySelf/Team/TeamExcludeSelf の適用ルール
- 統合: 2人/3人/4人チームでの最終Stats計算
- エラーケース: 空チーム、5人以上、範囲外index

### data テスト
- TeamMemberBuilder: キャラ+武器からの基礎ステ算出
- 10キャラのバフデータ: 値の正確性
- 統合: Bennett+Kazuha チームでのダメージ計算

### golden テスト
- Bennett(Lv90, アクアファヴォニア, 旧貴族4, C6, 天賦1/10/13) + メインDPS のチーム
- 手計算値との照合

---

## 8. ファイル構成まとめ

### core crate
- 新規: `src/team.rs` — TeamMember, BuffTarget, ResolvedBuff, resolve_team_stats
- 新規: `src/resonance.rs` — ElementalResonance, determine_resonances, resonance_buffs
- 変更: `src/types.rs` — WeaponType 追加
- 変更: `src/error.rs` — CalcError にバリアント追加
- 変更: `src/lib.rs` — 新モジュール・型の pub use

### data crate
- 新規: `src/talent_buffs.rs` — TalentBuffDef, TalentBuffSource, 10キャラ分のデータ
- 新規: `src/team_builder.rs` — TeamMemberBuilder
- 変更: `src/types.rs` — WeaponType を core から re-export
- 変更: `src/lib.rs` — 新モジュールの pub mod + pub use
