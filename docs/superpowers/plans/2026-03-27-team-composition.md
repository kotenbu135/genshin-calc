# チーム編成バフ解決エンジン 実装計画

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** チーム編成のバフ解決機能を core/data crate に追加し、1-4人チームのバフを自動計算してダメージ計算に利用できるようにする

**Architecture:** core crate にバフ解決エンジン（team.rs）と元素共鳴（resonance.rs）を追加。data crate に天賦バフ定義（talent_buffs.rs）とTeamMemberBuilder（team_builder.rs）を追加。WeaponType と BuffableStat を data → core に移動。

**Tech Stack:** Rust, serde, thiserror

**Spec:** `docs/superpowers/specs/2026-03-27-team-composition-design.md`

---

## File Structure

### Core Crate (crates/core/)
- Modify: `src/types.rs` — WeaponType 追加
- Create: `src/buff_types.rs` — BuffableStat を data から移動（core で使うため）
- Create: `src/resonance.rs` — ElementalResonance, determine_resonances, resonance_buffs
- Create: `src/team.rs` — BuffTarget, ResolvedBuff, TeamMember, TeamResolveResult, apply_buffs_to_profile, resolve_team_stats, resolve_team_stats_detailed
- Modify: `src/error.rs` — InvalidTeamSize, InvalidTargetIndex 追加
- Modify: `src/lib.rs` — 新モジュール・型の pub mod/pub use

### Data Crate (crates/data/)
- Modify: `src/types.rs` — WeaponType を core から re-export に変更
- Modify: `src/buff.rs` — BuffableStat を core から re-export に変更
- Create: `src/talent_buffs.rs` — TalentBuffDef, TalentBuffSource, find_talent_buffs, 10キャラ分データ
- Create: `src/team_builder.rs` — TeamMemberBuilder
- Modify: `src/lib.rs` — 新モジュールの pub mod + pub use

---

### Task 1: WeaponType + BuffableStat を core に移動

**Files:**
- Modify: `crates/core/src/types.rs`
- Create: `crates/core/src/buff_types.rs`
- Modify: `crates/core/src/lib.rs`
- Modify: `crates/data/src/types.rs`
- Modify: `crates/data/src/buff.rs`

- [ ] **Step 1: core/src/types.rs に WeaponType を追加**

`crates/core/src/types.rs` の末尾に追加:

```rust
/// Weapon type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Claymore,
    Polearm,
    Bow,
    Catalyst,
}
```

- [ ] **Step 2: core/src/buff_types.rs を作成**

`crates/core/src/buff_types.rs` を新規作成。data crate の `buff.rs` から `BuffableStat` をコピー:

```rust
use crate::types::Element;
use serde::{Deserialize, Serialize};

/// Stat that can be buffed by weapons, artifacts, or character talents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuffableStat {
    HpPercent,
    AtkPercent,
    DefPercent,
    HpFlat,
    AtkFlat,
    DefFlat,
    CritRate,
    CritDmg,
    ElementalMastery,
    EnergyRecharge,
    DmgBonus,
    ElementalDmgBonus(Element),
    PhysicalDmgBonus,
    NormalAtkDmgBonus,
    ChargedAtkDmgBonus,
    PlungingAtkDmgBonus,
    SkillDmgBonus,
    BurstDmgBonus,
    HealingBonus,
    ShieldStrength,
}
```

- [ ] **Step 3: core/src/lib.rs にモジュールと re-export を追加**

`pub mod types;` の後に:
```rust
pub mod buff_types;
```

`pub use types::` の行の後に:
```rust
pub use buff_types::BuffableStat;
pub use types::WeaponType;
```

- [ ] **Step 4: data/src/types.rs の WeaponType を core からの re-export に変更**

`data/src/types.rs` から `WeaponType` enum 定義（`pub enum WeaponType { Sword, Claymore, ... }`）を削除し、代わりにファイル先頭の import に追加:

```rust
pub use genshin_calc_core::WeaponType;
```

- [ ] **Step 5: data/src/buff.rs の BuffableStat を core からの re-export に変更**

`data/src/buff.rs` から `BuffableStat` enum 定義を削除し、代わりに:

```rust
pub use genshin_calc_core::BuffableStat;
```

`StatBuff` と `PassiveEffect` の `BuffableStat` 参照はそのまま動く（re-export経由）。

- [ ] **Step 6: ビルド確認**

Run: `cargo build`
Expected: 成功（全crateで WeaponType と BuffableStat が解決される）

- [ ] **Step 7: テスト確認**

Run: `cargo test`
Expected: 既存テスト全パス

- [ ] **Step 8: コミット**

```bash
git add crates/core/src/types.rs crates/core/src/buff_types.rs crates/core/src/lib.rs crates/data/src/types.rs crates/data/src/buff.rs
git commit -m "refactor: move WeaponType and BuffableStat to core crate"
```

---

### Task 2: Core 元素共鳴モジュール

**Files:**
- Create: `crates/core/src/resonance.rs`
- Modify: `crates/core/src/lib.rs`

- [ ] **Step 1: resonance.rs を作成（型定義 + determine_resonances + テスト）**

`crates/core/src/resonance.rs` を新規作成:

```rust
use serde::{Deserialize, Serialize};

use crate::buff_types::BuffableStat;
use crate::types::Element;

/// Elemental resonance effects for 4-member teams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementalResonance {
    /// Pyro ×2: ATK +25%.
    FerventFlames,
    /// Hydro ×2: HP +25%.
    SoothingWater,
    /// Electro ×2: no stat effect.
    HighVoltage,
    /// Cryo ×2: conditional CRIT Rate +15%.
    ShatteringIce,
    /// Anemo ×2: no stat effect.
    ImpetuousWinds,
    /// Geo ×2: conditional DMG +15%.
    EnduringRock,
    /// Dendro ×2: EM +50.
    SprawlingGreenery,
    /// 4 unique elements: no damage stat effect.
    ProtectiveCanopy,
}

/// Determines elemental resonances from team member elements.
///
/// Returns empty if team has fewer than 4 members.
/// ProtectiveCanopy is exclusive — only triggers when all 4 elements are different.
pub fn determine_resonances(elements: &[Element]) -> Vec<ElementalResonance> {
    if elements.len() != 4 {
        return vec![];
    }

    use std::collections::HashMap;
    let mut counts: HashMap<Element, usize> = HashMap::new();
    for &e in elements {
        *counts.entry(e).or_insert(0) += 1;
    }

    // 全4元素が異なる場合
    if counts.len() == 4 {
        return vec![ElementalResonance::ProtectiveCanopy];
    }

    let mut resonances = vec![];
    for (&element, &count) in &counts {
        if count >= 2 {
            let r = match element {
                Element::Pyro => ElementalResonance::FerventFlames,
                Element::Hydro => ElementalResonance::SoothingWater,
                Element::Electro => ElementalResonance::HighVoltage,
                Element::Cryo => ElementalResonance::ShatteringIce,
                Element::Anemo => ElementalResonance::ImpetuousWinds,
                Element::Geo => ElementalResonance::EnduringRock,
                Element::Dendro => ElementalResonance::SprawlingGreenery,
            };
            resonances.push(r);
        }
    }
    resonances.sort_by_key(|r| *r as u8);
    resonances
}

/// Returns unconditional stat buffs for a resonance.
///
/// Returns empty for resonances with no stat effect or conditional resonances.
pub fn resonance_buffs(resonance: ElementalResonance) -> Vec<(BuffableStat, f64)> {
    match resonance {
        ElementalResonance::FerventFlames => {
            vec![(BuffableStat::AtkPercent, 0.25)]
        }
        ElementalResonance::SoothingWater => {
            vec![(BuffableStat::HpPercent, 0.25)]
        }
        ElementalResonance::SprawlingGreenery => {
            vec![(BuffableStat::ElementalMastery, 50.0)]
        }
        // Conditional or no stat effect
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fewer_than_4_returns_empty() {
        assert!(determine_resonances(&[Element::Pyro, Element::Pyro]).is_empty());
        assert!(determine_resonances(&[Element::Pyro, Element::Pyro, Element::Hydro]).is_empty());
        assert!(determine_resonances(&[]).is_empty());
    }

    #[test]
    fn test_pyro_resonance() {
        let elements = [Element::Pyro, Element::Pyro, Element::Hydro, Element::Cryo];
        let res = determine_resonances(&elements);
        assert!(res.contains(&ElementalResonance::FerventFlames));
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_double_resonance() {
        let elements = [Element::Pyro, Element::Pyro, Element::Hydro, Element::Hydro];
        let res = determine_resonances(&elements);
        assert!(res.contains(&ElementalResonance::FerventFlames));
        assert!(res.contains(&ElementalResonance::SoothingWater));
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_four_unique_elements() {
        let elements = [Element::Pyro, Element::Hydro, Element::Electro, Element::Cryo];
        let res = determine_resonances(&elements);
        assert_eq!(res, vec![ElementalResonance::ProtectiveCanopy]);
    }

    #[test]
    fn test_triple_same_element() {
        let elements = [Element::Pyro, Element::Pyro, Element::Pyro, Element::Hydro];
        let res = determine_resonances(&elements);
        assert!(res.contains(&ElementalResonance::FerventFlames));
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_fervent_flames_buffs() {
        let buffs = resonance_buffs(ElementalResonance::FerventFlames);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0], (BuffableStat::AtkPercent, 0.25));
    }

    #[test]
    fn test_soothing_water_buffs() {
        let buffs = resonance_buffs(ElementalResonance::SoothingWater);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0], (BuffableStat::HpPercent, 0.25));
    }

    #[test]
    fn test_sprawling_greenery_buffs() {
        let buffs = resonance_buffs(ElementalResonance::SprawlingGreenery);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0], (BuffableStat::ElementalMastery, 50.0));
    }

    #[test]
    fn test_conditional_resonance_no_buffs() {
        assert!(resonance_buffs(ElementalResonance::ShatteringIce).is_empty());
        assert!(resonance_buffs(ElementalResonance::EnduringRock).is_empty());
        assert!(resonance_buffs(ElementalResonance::HighVoltage).is_empty());
        assert!(resonance_buffs(ElementalResonance::ImpetuousWinds).is_empty());
        assert!(resonance_buffs(ElementalResonance::ProtectiveCanopy).is_empty());
    }
}
```

- [ ] **Step 2: lib.rs に resonance モジュールを追加**

```rust
pub mod resonance;
pub use resonance::{ElementalResonance, determine_resonances, resonance_buffs};
```

- [ ] **Step 3: テスト実行**

Run: `cargo test -p genshin-calc-core`
Expected: 既存テスト + 新規9テスト全パス

- [ ] **Step 4: コミット**

```bash
git add crates/core/src/resonance.rs crates/core/src/lib.rs
git commit -m "feat(core): add elemental resonance detection and buff generation"
```

---

### Task 3: Core チームバフ解決モジュール（型 + ヘルパー）

**Files:**
- Create: `crates/core/src/team.rs`
- Modify: `crates/core/src/error.rs`
- Modify: `crates/core/src/lib.rs`

- [ ] **Step 1: error.rs に新バリアント追加**

`crates/core/src/error.rs` の CalcError enum に追加:

```rust
    #[error("team must have 1..=4 members, got {0}")]
    InvalidTeamSize(usize),

    #[error("target index {index} out of bounds for team of size {team_size}")]
    InvalidTargetIndex { index: usize, team_size: usize },

    #[error("constellation must be 0..=6, got {0}")]
    InvalidConstellation(u8),

    #[error("talent level must be 1..=15, got {0}")]
    InvalidTalentLevel(u8),
```

- [ ] **Step 2: team.rs を作成（型定義 + apply_buffs_to_profile + テスト）**

`crates/core/src/team.rs` を新規作成:

```rust
use serde::{Deserialize, Serialize};

use crate::buff_types::BuffableStat;
use crate::error::CalcError;
use crate::resonance::{determine_resonances, resonance_buffs, ElementalResonance};
use crate::stat_profile::{StatProfile, combine_stats};
use crate::stats::Stats;
use crate::types::{Element, WeaponType};

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
    pub buffs_provided: Vec<ResolvedBuff>,
}

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

/// Returns true if the buff is unconditional (can be applied to StatProfile directly).
fn is_unconditional(stat: &BuffableStat) -> bool {
    matches!(
        stat,
        BuffableStat::HpPercent
            | BuffableStat::AtkPercent
            | BuffableStat::DefPercent
            | BuffableStat::HpFlat
            | BuffableStat::AtkFlat
            | BuffableStat::DefFlat
            | BuffableStat::CritRate
            | BuffableStat::CritDmg
            | BuffableStat::ElementalMastery
            | BuffableStat::EnergyRecharge
            | BuffableStat::DmgBonus
    )
}

/// Applies unconditional buffs to a StatProfile, returning a new one.
///
/// DamageType/Element-dependent buffs (NormalAtkDmgBonus, ElementalDmgBonus, etc.)
/// are skipped.
pub fn apply_buffs_to_profile(profile: &StatProfile, buffs: &[ResolvedBuff]) -> StatProfile {
    let mut p = profile.clone();
    for buff in buffs {
        if !is_unconditional(&buff.stat) {
            continue;
        }
        match buff.stat {
            BuffableStat::HpPercent => p.hp_percent += buff.value,
            BuffableStat::AtkPercent => p.atk_percent += buff.value,
            BuffableStat::DefPercent => p.def_percent += buff.value,
            BuffableStat::HpFlat => p.hp_flat += buff.value,
            BuffableStat::AtkFlat => p.atk_flat += buff.value,
            BuffableStat::DefFlat => p.def_flat += buff.value,
            BuffableStat::CritRate => p.crit_rate += buff.value,
            BuffableStat::CritDmg => p.crit_dmg += buff.value,
            BuffableStat::ElementalMastery => p.elemental_mastery += buff.value,
            BuffableStat::EnergyRecharge => p.energy_recharge += buff.value,
            BuffableStat::DmgBonus => p.dmg_bonus += buff.value,
            _ => {} // conditional — skipped
        }
    }
    p
}

fn validate_team(team: &[TeamMember], target_index: usize) -> Result<(), CalcError> {
    if team.is_empty() || team.len() > 4 {
        return Err(CalcError::InvalidTeamSize(team.len()));
    }
    if target_index >= team.len() {
        return Err(CalcError::InvalidTargetIndex {
            index: target_index,
            team_size: team.len(),
        });
    }
    Ok(())
}

fn collect_buffs(team: &[TeamMember], target_index: usize) -> Vec<ResolvedBuff> {
    let mut buffs = Vec::new();

    for (i, member) in team.iter().enumerate() {
        for buff in &member.buffs_provided {
            let applies = match buff.target {
                BuffTarget::OnlySelf => i == target_index,
                BuffTarget::Team => true,
                BuffTarget::TeamExcludeSelf => i != target_index,
            };
            if applies {
                buffs.push(buff.clone());
            }
        }
    }

    // 元素共鳴
    let elements: Vec<Element> = team.iter().map(|m| m.element).collect();
    let resonances = determine_resonances(&elements);
    for resonance in &resonances {
        for (stat, value) in resonance_buffs(*resonance) {
            buffs.push(ResolvedBuff {
                source: format!("{:?}", resonance),
                stat,
                value,
                target: BuffTarget::Team,
            });
        }
    }

    buffs
}

/// Resolves team buffs and returns final stats for the target member.
///
/// Note: DamageType/Element-dependent buffs are NOT included.
/// Use [`resolve_team_stats_detailed`] to see all collected buffs.
///
/// # Errors
///
/// Returns [`CalcError::InvalidTeamSize`] if team is empty or has >4 members.
/// Returns [`CalcError::InvalidTargetIndex`] if target_index is out of bounds.
pub fn resolve_team_stats(
    team: &[TeamMember],
    target_index: usize,
) -> Result<Stats, CalcError> {
    let result = resolve_team_stats_detailed(team, target_index)?;
    Ok(result.final_stats)
}

/// Resolves team buffs with detailed breakdown.
///
/// `applied_buffs` contains all buffs including conditional ones.
/// `final_stats` only includes unconditional buffs.
///
/// # Errors
///
/// Same as [`resolve_team_stats`].
pub fn resolve_team_stats_detailed(
    team: &[TeamMember],
    target_index: usize,
) -> Result<TeamResolveResult, CalcError> {
    validate_team(team, target_index)?;

    let base_profile = &team[target_index].stats;
    let base_stats = combine_stats(base_profile)?;

    let applied_buffs = collect_buffs(team, target_index);
    let buffed_profile = apply_buffs_to_profile(base_profile, &applied_buffs);
    let final_stats = combine_stats(&buffed_profile)?;

    let elements: Vec<Element> = team.iter().map(|m| m.element).collect();
    let resonances = determine_resonances(&elements);

    Ok(TeamResolveResult {
        base_stats,
        applied_buffs,
        resonances,
        final_stats,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    fn make_member(element: Element, base_atk: f64) -> TeamMember {
        TeamMember {
            element,
            weapon_type: WeaponType::Sword,
            stats: StatProfile {
                base_atk,
                base_hp: 10000.0,
                base_def: 500.0,
                crit_rate: 0.50,
                crit_dmg: 1.00,
                energy_recharge: 1.00,
                ..Default::default()
            },
            buffs_provided: vec![],
        }
    }

    #[test]
    fn test_empty_team_error() {
        let result = resolve_team_stats(&[], 0);
        assert_eq!(result, Err(CalcError::InvalidTeamSize(0)));
    }

    #[test]
    fn test_five_member_team_error() {
        let team: Vec<TeamMember> = (0..5).map(|_| make_member(Element::Pyro, 800.0)).collect();
        let result = resolve_team_stats(&team, 0);
        assert_eq!(result, Err(CalcError::InvalidTeamSize(5)));
    }

    #[test]
    fn test_target_index_out_of_bounds() {
        let team = vec![make_member(Element::Pyro, 800.0)];
        let result = resolve_team_stats(&team, 1);
        assert_eq!(
            result,
            Err(CalcError::InvalidTargetIndex {
                index: 1,
                team_size: 1
            })
        );
    }

    #[test]
    fn test_single_member_no_buffs() {
        let team = vec![make_member(Element::Pyro, 800.0)];
        let stats = resolve_team_stats(&team, 0).unwrap();
        assert!((stats.atk - 800.0).abs() < EPSILON);
    }

    #[test]
    fn test_self_buff_applies_to_self() {
        let mut member = make_member(Element::Pyro, 800.0);
        member.buffs_provided.push(ResolvedBuff {
            source: "Weapon Passive".into(),
            stat: BuffableStat::AtkPercent,
            value: 0.20,
            target: BuffTarget::OnlySelf,
        });
        let team = vec![member, make_member(Element::Hydro, 700.0)];

        // Member 0 gets the self buff
        let stats0 = resolve_team_stats(&team, 0).unwrap();
        assert!((stats0.atk - 800.0 * (1.0 + 0.20)).abs() < EPSILON);

        // Member 1 does NOT get it
        let stats1 = resolve_team_stats(&team, 1).unwrap();
        assert!((stats1.atk - 700.0).abs() < EPSILON);
    }

    #[test]
    fn test_team_buff_applies_to_all() {
        let mut bennett = make_member(Element::Pyro, 800.0);
        bennett.buffs_provided.push(ResolvedBuff {
            source: "Bennett Burst".into(),
            stat: BuffableStat::AtkFlat,
            value: 1000.0,
            target: BuffTarget::Team,
        });
        let dps = make_member(Element::Pyro, 900.0);
        let team = vec![bennett, dps];

        // Both members get the team buff
        let stats0 = resolve_team_stats(&team, 0).unwrap();
        assert!((stats0.atk - (800.0 + 1000.0)).abs() < EPSILON);

        let stats1 = resolve_team_stats(&team, 1).unwrap();
        assert!((stats1.atk - (900.0 + 1000.0)).abs() < EPSILON);
    }

    #[test]
    fn test_team_exclude_self_buff() {
        let mut rosaria = make_member(Element::Cryo, 700.0);
        rosaria.buffs_provided.push(ResolvedBuff {
            source: "Rosaria A4".into(),
            stat: BuffableStat::CritRate,
            value: 0.15,
            target: BuffTarget::TeamExcludeSelf,
        });
        let dps = make_member(Element::Pyro, 900.0);
        let team = vec![rosaria, dps];

        // Rosaria does NOT get her own buff
        let stats0 = resolve_team_stats(&team, 0).unwrap();
        assert!((stats0.crit_rate - 0.50).abs() < EPSILON);

        // DPS gets the buff
        let stats1 = resolve_team_stats(&team, 1).unwrap();
        assert!((stats1.crit_rate - 0.65).abs() < EPSILON);
    }

    #[test]
    fn test_pyro_resonance_with_4_members() {
        let team = vec![
            make_member(Element::Pyro, 800.0),
            make_member(Element::Pyro, 700.0),
            make_member(Element::Hydro, 600.0),
            make_member(Element::Cryo, 500.0),
        ];
        let stats = resolve_team_stats(&team, 0).unwrap();
        // base_atk=800, atk_percent=0.25 from resonance → 800*(1+0.25)=1000
        assert!((stats.atk - 1000.0).abs() < EPSILON);
    }

    #[test]
    fn test_no_resonance_with_3_members() {
        let team = vec![
            make_member(Element::Pyro, 800.0),
            make_member(Element::Pyro, 700.0),
            make_member(Element::Hydro, 600.0),
        ];
        let stats = resolve_team_stats(&team, 0).unwrap();
        // No resonance — base_atk only
        assert!((stats.atk - 800.0).abs() < EPSILON);
    }

    #[test]
    fn test_detailed_result_includes_resonances() {
        let team = vec![
            make_member(Element::Pyro, 800.0),
            make_member(Element::Pyro, 700.0),
            make_member(Element::Hydro, 600.0),
            make_member(Element::Cryo, 500.0),
        ];
        let result = resolve_team_stats_detailed(&team, 0).unwrap();
        assert!(result.resonances.contains(&ElementalResonance::FerventFlames));
        assert!(!result.applied_buffs.is_empty());
        assert!(result.final_stats.atk > result.base_stats.atk);
    }

    #[test]
    fn test_apply_buffs_skips_conditional() {
        let profile = StatProfile {
            base_atk: 800.0,
            ..Default::default()
        };
        let buffs = vec![
            ResolvedBuff {
                source: "test".into(),
                stat: BuffableStat::AtkPercent,
                value: 0.20,
                target: BuffTarget::Team,
            },
            ResolvedBuff {
                source: "conditional".into(),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.30,
                target: BuffTarget::Team,
            },
        ];
        let result = apply_buffs_to_profile(&profile, &buffs);
        // AtkPercent applied, NormalAtkDmgBonus skipped
        assert!((result.atk_percent - 0.20).abs() < EPSILON);
        assert!((result.dmg_bonus - 0.0).abs() < EPSILON);
    }
}
```

- [ ] **Step 3: lib.rs に team モジュールを追加**

```rust
pub mod team;
pub use team::{
    BuffTarget, ResolvedBuff, TeamMember, TeamResolveResult,
    apply_buffs_to_profile, resolve_team_stats, resolve_team_stats_detailed,
};
```

- [ ] **Step 4: テスト実行**

Run: `cargo test -p genshin-calc-core`
Expected: 全テスト（既存 + resonance 9件 + team 11件）パス

- [ ] **Step 5: コミット**

```bash
git add crates/core/src/team.rs crates/core/src/error.rs crates/core/src/lib.rs
git commit -m "feat(core): add team buff resolution engine with tests"
```

---

### Task 4: Data 天賦バフ定義

**Files:**
- Create: `crates/data/src/talent_buffs.rs`
- Modify: `crates/data/src/lib.rs`

- [ ] **Step 1: talent_buffs.rs を作成（型 + find_talent_buffs + 10キャラ分データ）**

`crates/data/src/talent_buffs.rs` を新規作成:

```rust
use genshin_calc_core::{BuffTarget, BuffableStat};
use serde::{Deserialize, Serialize};

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
    /// Buff name.
    pub name: &'static str,
    /// Description.
    pub description: &'static str,
    /// Stat being buffed.
    pub stat: BuffableStat,
    /// Fixed buff value (used when scales_with_talent is false).
    pub base_value: f64,
    /// Whether the buff scales with talent level.
    pub scales_with_talent: bool,
    /// Talent level scaling [Lv1..Lv15]. None if fixed.
    pub talent_scaling: Option<&'static [f64; 15]>,
    /// Base stat the scaling multiplier applies to.
    /// E.g. Bennett burst = Some(Atk) means final_value = provider_base_atk * scaling.
    /// None means the scaling value is used directly.
    pub scales_on: Option<genshin_calc_core::ScalingStat>,
    /// Who receives the buff.
    pub target: BuffTarget,
    /// Buff source.
    pub source: TalentBuffSource,
    /// Minimum constellation required (0 = none).
    pub min_constellation: u8,
}

// ===== Bennett =====
// 元素爆発「素晴らしい旅」ATKバフ: 基礎ATKの56%~119% (Lv1-15)
static BENNETT_BURST_ATK_SCALING: [f64; 15] = [
    0.56, 0.602, 0.644, 0.70, 0.742, 0.784, 0.84, 0.896, 0.952,
    1.008, 1.064, 1.12, 1.19, 1.26, 1.33,
];

static BENNETT_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Fantastic Voyage ATK Bonus",
    description: "Characters within the burst field gain ATK bonus based on Bennett's Base ATK",
    stat: BuffableStat::AtkFlat,
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&BENNETT_BURST_ATK_SCALING),
    scales_on: Some(genshin_calc_core::ScalingStat::Atk), // base_atk × scaling
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalBurst,
    min_constellation: 0,
}];

// ===== Kazuha =====
// A4パッシブ「風物の詩吟」: EM × 0.04% の元素ダメバフ（拡散した元素）
static KAZUHA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Poetics of Fuubutsu",
    description: "After triggering Swirl, grants 0.04% Elemental DMG Bonus per point of EM",
    stat: BuffableStat::DmgBonus,
    base_value: 0.0, // EM依存 — ビルダーが計算時に EM×0.0004 を設定
    scales_with_talent: false,
    talent_scaling: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Nahida =====
// A4パッシブ「浄善摂受明論」: チーム最高EMの25% をEM加算（最大250）
static NAHIDA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "On All Things Meditated",
    description: "Grants EM to party based on highest EM in team (25%, max 250)",
    stat: BuffableStat::ElementalMastery,
    base_value: 0.0, // チームEM依存 — ビルダーが計算時に設定
    scales_with_talent: false,
    talent_scaling: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Shenhe =====
// 元素スキル「仰霊威召将役呪」: ATKの一定割合を氷元素DMGにフラット加算
static SHENHE_SKILL_SCALING: [f64; 15] = [
    0.4566, 0.4909, 0.5251, 0.5708, 0.6050, 0.6393, 0.6849, 0.7306, 0.7763,
    0.8219, 0.8676, 0.9132, 0.9703, 1.0274, 1.0844,
];

static SHENHE_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Spring Spirit Summoning Quill DMG",
    description: "Adds flat Cryo DMG based on Shenhe's ATK to party's Cryo attacks",
    stat: BuffableStat::AtkFlat, // 実際はフラットダメ加算だが、ATK基準のフラット値として扱う
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&SHENHE_SKILL_SCALING),
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalSkill,
    min_constellation: 0,
}];

// ===== Yun Jin =====
// 元素爆発「破嶂の旌儀」: DEFの一定割合を通常攻撃にフラット加算
static YUN_JIN_BURST_SCALING: [f64; 15] = [
    0.3216, 0.3457, 0.3699, 0.4020, 0.4262, 0.4503, 0.4824, 0.5145, 0.5466,
    0.5789, 0.6110, 0.6431, 0.6833, 0.7234, 0.7636,
];

static YUN_JIN_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Cliffbreaker's Banner Normal ATK Bonus",
    description: "Normal Attack DMG increased based on Yun Jin's DEF",
    stat: BuffableStat::AtkFlat,
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&YUN_JIN_BURST_SCALING),
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalBurst,
    min_constellation: 0,
}];

// ===== Mona =====
// 元素爆発「星命定軌」: ダメージバフ（Lv依存）
static MONA_BURST_DMG_SCALING: [f64; 15] = [
    0.42, 0.44, 0.46, 0.50, 0.52, 0.54, 0.58, 0.62, 0.66,
    0.70, 0.74, 0.78, 0.82, 0.86, 0.90,
];

static MONA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Stellaris Phantasm DMG Bonus",
    description: "Omen increases DMG taken by opponents",
    stat: BuffableStat::DmgBonus,
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&MONA_BURST_DMG_SCALING),
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalBurst,
    min_constellation: 0,
}];

// ===== Sara =====
// 元素スキル/爆発: 基礎ATKの一定割合のATKバフ
static SARA_ATK_SCALING: [f64; 15] = [
    0.4296, 0.4618, 0.4940, 0.5370, 0.5692, 0.6014, 0.6444, 0.6874, 0.7304,
    0.7734, 0.8164, 0.8594, 0.9131, 0.9668, 1.0206,
];

static SARA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Tengu Juurai ATK Bonus",
    description: "ATK bonus based on Sara's Base ATK",
    stat: BuffableStat::AtkFlat,
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&SARA_ATK_SCALING),
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalSkill,
    min_constellation: 0,
}];

// ===== Rosaria =====
// A4パッシブ「影についての隠喩」: 爆発後、自身CRの15%をチームに付与
static ROSARIA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Shadow Samaritan CRIT Rate Share",
    description: "After burst, grants 15% of Rosaria's CRIT Rate to party",
    stat: BuffableStat::CritRate,
    base_value: 0.15, // 自身CRの15% — ビルダーが計算時に crit_rate * 0.15 を設定
    scales_with_talent: false,
    talent_scaling: None,
    target: BuffTarget::TeamExcludeSelf,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Furina =====
// 元素爆発「万衆狂歓」: ファンファーレスタックでダメバフ（最大値はLv依存）
static FURINA_BURST_DMG_SCALING: [f64; 15] = [
    0.09, 0.11, 0.13, 0.15, 0.17, 0.19, 0.21, 0.23, 0.25,
    0.27, 0.29, 0.31, 0.33, 0.35, 0.37,
];

static FURINA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Let the People Rejoice DMG Bonus",
    description: "Fanfare stacks grant DMG bonus (max based on talent level)",
    stat: BuffableStat::DmgBonus,
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&FURINA_BURST_DMG_SCALING),
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalBurst,
    min_constellation: 0,
}];

/// All character talent buff definitions.
static ALL_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("bennett", BENNETT_BUFFS),
    ("kazuha", KAZUHA_BUFFS),
    ("nahida", NAHIDA_BUFFS),
    ("shenhe", SHENHE_BUFFS),
    ("yun_jin", YUN_JIN_BUFFS),
    ("mona", MONA_BUFFS),
    ("sara", SARA_BUFFS),
    ("rosaria", ROSARIA_BUFFS),
    ("furina", FURINA_BUFFS),
    // Zhongli は耐性ダウン（Enemy側）のため、バフ定義なし
];

/// Finds talent buff definitions for a character by ID.
///
/// Returns `None` for characters without defined talent buffs.
pub fn find_talent_buffs(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    ALL_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_bennett_buffs() {
        let buffs = find_talent_buffs("bennett").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
        assert_eq!(buffs[0].target, BuffTarget::Team);
    }

    #[test]
    fn test_find_rosaria_buffs() {
        let buffs = find_talent_buffs("rosaria").unwrap();
        assert_eq!(buffs[0].target, BuffTarget::TeamExcludeSelf);
    }

    #[test]
    fn test_find_nonexistent_character() {
        assert!(find_talent_buffs("zhongli").is_none());
        assert!(find_talent_buffs("unknown").is_none());
    }

    #[test]
    fn test_bennett_burst_scaling_lv13() {
        let buffs = find_talent_buffs("bennett").unwrap();
        let scaling = buffs[0].talent_scaling.unwrap();
        // Lv13 = index 12 = 1.19
        assert!((scaling[12] - 1.19).abs() < 1e-6);
    }

    #[test]
    fn test_all_talent_buffs_have_unique_ids() {
        let ids: Vec<&str> = ALL_TALENT_BUFFS.iter().map(|(id, _)| *id).collect();
        for (i, id) in ids.iter().enumerate() {
            assert!(
                !ids[i + 1..].contains(id),
                "Duplicate talent buff ID: {id}"
            );
        }
    }
}
```

- [ ] **Step 2: lib.rs に talent_buffs モジュールを追加**

`crates/data/src/lib.rs` に追加:

```rust
pub mod talent_buffs;
pub use talent_buffs::{TalentBuffDef, TalentBuffSource, find_talent_buffs};
```

- [ ] **Step 3: テスト実行**

Run: `cargo test -p genshin-calc-data`
Expected: 既存テスト + 新規5テスト全パス

- [ ] **Step 4: コミット**

```bash
git add crates/data/src/talent_buffs.rs crates/data/src/lib.rs
git commit -m "feat(data): add talent buff definitions for 9 characters"
```

---

### Task 5: Data TeamMemberBuilder

**Files:**
- Create: `crates/data/src/team_builder.rs`
- Modify: `crates/data/src/lib.rs`

- [ ] **Step 1: team_builder.rs を作成**

`crates/data/src/team_builder.rs` を新規作成:

```rust
use genshin_calc_core::{
    BuffTarget, BuffableStat, CalcError, ResolvedBuff, StatProfile, TeamMember, WeaponType,
};

use crate::talent_buffs::find_talent_buffs;
use crate::types::{ArtifactSet, AscensionStat, CharacterData, WeaponData, WeaponSubStat};

/// Builder for constructing a [`TeamMember`] from game data.
pub struct TeamMemberBuilder {
    character: &'static CharacterData,
    weapon: &'static WeaponData,
    artifact_set: Option<&'static ArtifactSet>,
    artifact_stats: StatProfile,
    constellation: u8,
    talent_levels: [u8; 3],
}

impl TeamMemberBuilder {
    /// Creates a new builder with a character and weapon.
    ///
    /// Defaults: no artifact set, empty artifact stats, constellation 0, talents [1, 1, 1].
    pub fn new(character: &'static CharacterData, weapon: &'static WeaponData) -> Self {
        Self {
            character,
            weapon,
            artifact_set: None,
            artifact_stats: StatProfile::default(),
            constellation: 0,
            talent_levels: [1, 1, 1],
        }
    }

    /// Sets the artifact set (4-piece).
    pub fn artifact_set(mut self, set: &'static ArtifactSet) -> Self {
        self.artifact_set = Some(set);
        self
    }

    /// Sets the artifact substats.
    pub fn artifact_stats(mut self, stats: StatProfile) -> Self {
        self.artifact_stats = stats;
        self
    }

    /// Sets the constellation level (0-6).
    pub fn constellation(mut self, c: u8) -> Self {
        self.constellation = c;
        self
    }

    /// Sets talent levels [normal, skill, burst] (1-15 each).
    pub fn talent_levels(mut self, levels: [u8; 3]) -> Self {
        self.talent_levels = levels;
        self
    }

    /// Builds the [`TeamMember`].
    ///
    /// # Errors
    ///
    /// Returns [`CalcError::InvalidConstellation`] if constellation > 6.
    /// Returns [`CalcError::InvalidTalentLevel`] if any talent level is 0 or > 15.
    pub fn build(self) -> Result<TeamMember, CalcError> {
        if self.constellation > 6 {
            return Err(CalcError::InvalidConstellation(self.constellation));
        }
        for &level in &self.talent_levels {
            if level == 0 || level > 15 {
                return Err(CalcError::InvalidTalentLevel(level));
            }
        }

        let char_data = self.character;
        let weapon = self.weapon;

        // 1. ベースステータス（Lv90 = index 3）
        let mut profile = StatProfile {
            base_hp: char_data.base_hp[3],
            base_atk: char_data.base_atk[3] + weapon.base_atk[3],
            base_def: char_data.base_def[3],
            ..Default::default()
        };

        // 2. キャラ昇格ステータス
        apply_ascension_stat(&mut profile, &char_data.ascension_stat);

        // 3. 武器サブステータス
        if let Some(sub) = &weapon.sub_stat {
            apply_weapon_sub_stat(&mut profile, sub);
        }

        // 4. 聖遺物ステータス（ユーザー入力）マージ
        profile.hp_percent += self.artifact_stats.hp_percent;
        profile.atk_percent += self.artifact_stats.atk_percent;
        profile.def_percent += self.artifact_stats.def_percent;
        profile.hp_flat += self.artifact_stats.hp_flat;
        profile.atk_flat += self.artifact_stats.atk_flat;
        profile.def_flat += self.artifact_stats.def_flat;
        profile.elemental_mastery += self.artifact_stats.elemental_mastery;
        profile.crit_rate += self.artifact_stats.crit_rate;
        profile.crit_dmg += self.artifact_stats.crit_dmg;
        profile.energy_recharge += self.artifact_stats.energy_recharge;
        profile.dmg_bonus += self.artifact_stats.dmg_bonus;

        // 5-7. バフ収集
        let mut buffs = Vec::new();

        // 武器パッシブ
        if let Some(passive) = &weapon.passive {
            for stat_buff in passive.effect.buffs {
                buffs.push(ResolvedBuff {
                    source: format!("{} ({})", passive.name, weapon.name),
                    stat: stat_buff.stat,
                    value: stat_buff.value,
                    target: BuffTarget::OnlySelf,
                });
            }
        }

        // 聖遺物セット効果
        if let Some(set) = self.artifact_set {
            for stat_buff in set.two_piece.buffs {
                buffs.push(ResolvedBuff {
                    source: format!("{} 2pc", set.name),
                    stat: stat_buff.stat,
                    value: stat_buff.value,
                    target: BuffTarget::OnlySelf,
                });
            }
            for stat_buff in set.four_piece.buffs {
                buffs.push(ResolvedBuff {
                    source: format!("{} 4pc", set.name),
                    stat: stat_buff.stat,
                    value: stat_buff.value,
                    target: BuffTarget::OnlySelf,
                });
            }
        }

        // 8. 天賦バフ
        if let Some(talent_defs) = find_talent_buffs(char_data.id) {
            for def in talent_defs {
                if self.constellation < def.min_constellation {
                    continue;
                }
                // 天賦レベルに応じた倍率を取得
                let raw_value = if def.scales_with_talent {
                    if let Some(scaling) = def.talent_scaling {
                        let talent_idx = match def.source {
                            crate::talent_buffs::TalentBuffSource::ElementalSkill => 1,
                            crate::talent_buffs::TalentBuffSource::ElementalBurst => 2,
                            _ => 0,
                        };
                        let level = self.talent_levels[talent_idx];
                        scaling[(level - 1) as usize]
                    } else {
                        def.base_value
                    }
                } else {
                    def.base_value
                };

                // scales_on がある場合、提供者のベースステータスで乗算
                let value = if let Some(scaling_stat) = def.scales_on {
                    let base = match scaling_stat {
                        genshin_calc_core::ScalingStat::Atk => profile.base_atk,
                        genshin_calc_core::ScalingStat::Hp => profile.base_hp,
                        genshin_calc_core::ScalingStat::Def => profile.base_def,
                    };
                    base * raw_value
                } else {
                    raw_value
                };

                buffs.push(ResolvedBuff {
                    source: def.name.to_string(),
                    stat: def.stat,
                    value,
                    target: def.target,
                });
            }
        }

        Ok(TeamMember {
            element: char_data.element,
            weapon_type: char_data.weapon_type,
            stats: profile,
            buffs_provided: buffs,
        })
    }
}

fn apply_ascension_stat(profile: &mut StatProfile, stat: &AscensionStat) {
    match stat {
        AscensionStat::Hp(v) => profile.hp_percent += v,
        AscensionStat::Atk(v) => profile.atk_percent += v,
        AscensionStat::Def(v) => profile.def_percent += v,
        AscensionStat::CritRate(v) => profile.crit_rate += v,
        AscensionStat::CritDmg(v) => profile.crit_dmg += v,
        AscensionStat::ElementalMastery(v) => profile.elemental_mastery += v,
        AscensionStat::EnergyRecharge(v) => profile.energy_recharge += v,
        AscensionStat::ElementalDmgBonus(_, v) => profile.dmg_bonus += v,
        AscensionStat::PhysicalDmgBonus(v) => profile.dmg_bonus += v,
        AscensionStat::HealingBonus(_) => {} // ダメージ計算に影響なし
    }
}

fn apply_weapon_sub_stat(profile: &mut StatProfile, sub: &WeaponSubStat) {
    // Lv90 = index 3
    match sub {
        WeaponSubStat::HpPercent(v) => profile.hp_percent += v[3],
        WeaponSubStat::AtkPercent(v) => profile.atk_percent += v[3],
        WeaponSubStat::DefPercent(v) => profile.def_percent += v[3],
        WeaponSubStat::CritRate(v) => profile.crit_rate += v[3],
        WeaponSubStat::CritDmg(v) => profile.crit_dmg += v[3],
        WeaponSubStat::ElementalMastery(v) => profile.elemental_mastery += v[3],
        WeaponSubStat::EnergyRecharge(v) => profile.energy_recharge += v[3],
        WeaponSubStat::PhysicalDmgBonus(v) => profile.dmg_bonus += v[3],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{find_character, find_weapon, find_artifact_set};

    const EPSILON: f64 = 1e-4;

    #[test]
    fn test_basic_build() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let member = TeamMemberBuilder::new(bennett, weapon).build().unwrap();
        assert_eq!(member.element, genshin_calc_core::Element::Pyro);
        assert!(member.stats.base_atk > 0.0);
        assert!(member.stats.base_hp > 0.0);
    }

    #[test]
    fn test_bennett_burst_buff_at_lv13() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let member = TeamMemberBuilder::new(bennett, weapon)
            .talent_levels([1, 1, 13])
            .build()
            .unwrap();

        // Bennett burst buff should be in buffs_provided
        let burst_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Fantastic Voyage"))
            .unwrap();
        // Lv13 = index 12 = 1.19 (multiplier of base ATK)
        assert!((burst_buff.value - 1.19).abs() < EPSILON);
        assert_eq!(burst_buff.target, BuffTarget::Team);
    }

    #[test]
    fn test_artifact_set_buffs() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let noblesse = find_artifact_set("noblesse_oblige").unwrap();
        let member = TeamMemberBuilder::new(bennett, weapon)
            .artifact_set(noblesse)
            .build()
            .unwrap();

        // Should have weapon passive + noblesse 2pc/4pc buffs
        assert!(
            member.buffs_provided.iter().any(|b| b.source.contains("2pc")),
            "Should have 2pc buff"
        );
    }

    #[test]
    fn test_invalid_constellation() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let result = TeamMemberBuilder::new(bennett, weapon)
            .constellation(7)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_talent_level() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let result = TeamMemberBuilder::new(bennett, weapon)
            .talent_levels([0, 1, 1])
            .build();
        assert!(result.is_err());
    }
}
```

- [ ] **Step 2: lib.rs に team_builder モジュールを追加**

```rust
pub mod team_builder;
pub use team_builder::TeamMemberBuilder;
```

- [ ] **Step 3: テスト実行**

Run: `cargo test -p genshin-calc-data`
Expected: 既存テスト + 新規5テスト全パス

- [ ] **Step 4: コミット**

```bash
git add crates/data/src/team_builder.rs crates/data/src/lib.rs
git commit -m "feat(data): add TeamMemberBuilder for constructing team members from game data"
```

---

### Task 6: 統合テスト + doc comments + lib.rs exports 整理

**Files:**
- Modify: `crates/core/src/lib.rs` — doc comments
- Create: `crates/data/tests/team_integration.rs`

- [ ] **Step 1: core lib.rs の crate doc を更新**

`crates/core/src/lib.rs` の `//!` crate doc に Team セクションを追加:

```rust
//! ## Team Composition
//!
//! Build teams with [`TeamMember`] and resolve buffs with [`resolve_team_stats`]:
//!
//! ```
//! use genshin_calc_core::*;
//!
//! let dps = TeamMember {
//!     element: Element::Pyro,
//!     weapon_type: WeaponType::Claymore,
//!     stats: StatProfile { base_atk: 900.0, crit_rate: 0.60, crit_dmg: 1.50, energy_recharge: 1.0, ..Default::default() },
//!     buffs_provided: vec![],
//! };
//! let support = TeamMember {
//!     element: Element::Pyro,
//!     weapon_type: WeaponType::Sword,
//!     stats: StatProfile { base_atk: 800.0, energy_recharge: 1.0, ..Default::default() },
//!     buffs_provided: vec![ResolvedBuff {
//!         source: "Bennett Burst".into(),
//!         stat: BuffableStat::AtkFlat,
//!         value: 1000.0,
//!         target: BuffTarget::Team,
//!     }],
//! };
//! let stats = resolve_team_stats(&[dps, support], 0).unwrap();
//! assert!(stats.atk > 900.0); // DPS gets Bennett's ATK buff
//! ```
```

- [ ] **Step 2: 統合テスト作成**

`crates/data/tests/team_integration.rs` を新規作成:

```rust
use genshin_calc_core::*;
use genshin_calc_data::*;

const EPSILON: f64 = 1.0;

#[test]
fn test_bennett_kazuha_team_damage() {
    // Bennett: Aquila Favonia, Noblesse 4pc, C6, talents 1/1/13
    let bennett = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("aquila_favonia").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(6)
    .talent_levels([1, 1, 13])
    .build()
    .unwrap();

    // Simple DPS: Pyro claymore user
    let dps = TeamMember {
        element: Element::Pyro,
        weapon_type: WeaponType::Claymore,
        stats: StatProfile {
            base_hp: 13000.0,
            base_atk: 900.0,
            base_def: 700.0,
            crit_rate: 0.70,
            crit_dmg: 1.50,
            energy_recharge: 1.0,
            dmg_bonus: 0.466,
            ..Default::default()
        },
        buffs_provided: vec![],
    };

    let team = [bennett, dps];

    // Resolve stats for DPS (index 1)
    let result = resolve_team_stats_detailed(&team, 1).unwrap();

    // DPS should have received Bennett's ATK flat buff
    let has_bennett_buff = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("Fantastic Voyage"));
    assert!(has_bennett_buff, "DPS should receive Bennett burst buff");

    // Final ATK should be higher than base
    assert!(result.final_stats.atk > 900.0);

    // Calculate damage with buffed stats
    let damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats,
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
        },
        &Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        },
    )
    .unwrap();

    assert!(damage.average > 0.0);
    assert!(damage.crit > damage.non_crit);
}

#[test]
fn test_four_member_team_with_resonance() {
    let pyro1 = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("aquila_favonia").unwrap(),
    )
    .build()
    .unwrap();

    let pyro2 = TeamMemberBuilder::new(
        find_character("xiangling").unwrap(),
        find_weapon("the_catch").unwrap(),
    )
    .build()
    .unwrap();

    let hydro = TeamMemberBuilder::new(
        find_character("xingqiu").unwrap(),
        find_weapon("sacrificial_sword").unwrap(),
    )
    .build()
    .unwrap();

    let cryo = TeamMemberBuilder::new(
        find_character("rosaria").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .build()
    .unwrap();

    let team = [pyro1, pyro2, hydro, cryo];
    let result = resolve_team_stats_detailed(&team, 1).unwrap();

    // Should have Pyro resonance (FerventFlames)
    assert!(
        result
            .resonances
            .contains(&ElementalResonance::FerventFlames),
        "Should have Pyro resonance"
    );

    // ATK should include 25% bonus from resonance
    assert!(result.final_stats.atk > result.base_stats.atk);
}
```

- [ ] **Step 3: 全テスト実行**

Run: `cargo test`
Expected: 全テストパス

- [ ] **Step 4: clippy + doc 確認**

Run: `cargo clippy -- -D warnings`
Expected: 0 warnings

Run: `cargo doc --no-deps 2>&1`
Expected: 警告なし

- [ ] **Step 5: コミット**

```bash
git add crates/core/src/lib.rs crates/data/tests/team_integration.rs
git commit -m "feat: add team integration tests and update crate docs"
```

---

### Task 7: CLAUDE.md 更新

**Files:**
- Modify: `CLAUDE.md`

- [ ] **Step 1: Architecture セクションに team/resonance の説明を追加**

`CLAUDE.md` の Architecture セクションに追加:

```
- `team.rs`: チームバフ解決（BuffTarget, ResolvedBuff, TeamMember, resolve_team_stats）
- `resonance.rs`: 元素共鳴判定・バフ生成（ElementalResonance, determine_resonances）
- `buff_types.rs`: バフ関連型（BuffableStat — data crateから移動）
```

Data Crate セクションに追加:

```
- `talent_buffs.rs`: 天賦バフ定義（TalentBuffDef, find_talent_buffs — 9キャラ分）
- `team_builder.rs`: TeamMemberBuilder（キャラ+武器+聖遺物→TeamMember構築）
- `buff.rs`: BuffableStat は core から re-export、StatBuff/PassiveEffect は data に残留
```

- [ ] **Step 2: コミット**

```bash
git add CLAUDE.md
git commit -m "docs: update CLAUDE.md with team composition architecture"
```
