use serde::{Deserialize, Serialize};

use crate::buff_types::BuffableStat;
use crate::enemy::{EnemyDebuffs, collect_enemy_debuffs};
use crate::error::CalcError;
use crate::reaction::{Reaction, ReactionCategory};
use crate::resonance::{
    ElementalResonance, determine_resonances, resonance_buffs, resonance_conditional_buffs,
};
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
    /// Deduplication key for same-name set effects.
    /// When multiple team members provide buffs with the same `origin` and `stat`,
    /// only the maximum value is kept.
    pub origin: Option<String>,
}

/// A team member with pre-resolved stats and buffs.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TeamMember {
    /// Character element.
    pub element: Element,
    /// Weapon type.
    pub weapon_type: WeaponType,
    /// Base stats before team buffs (character + weapon + artifacts).
    pub stats: StatProfile,
    /// Buffs this member provides to the team.
    pub buffs_provided: Vec<ResolvedBuff>,
    /// Whether this character is a Moonsign (月兆) character from Nod-Krai.
    pub is_moonsign: bool,
    /// Whether this character can use Nightsoul's Blessing (夜魂の加護).
    pub can_nightsoul: bool,
}

/// Aggregated attack-type-specific DMG bonuses, flat DMG, and reaction bonuses
/// from team buffs. These cannot be applied to `StatProfile`/`Stats` because they
/// depend on `DamageType` or reaction context.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct DamageContext {
    /// Normal attack DMG bonus from team buffs.
    pub normal_atk_dmg_bonus: f64,
    /// Charged attack DMG bonus from team buffs.
    pub charged_atk_dmg_bonus: f64,
    /// Plunging attack DMG bonus from team buffs.
    pub plunging_atk_dmg_bonus: f64,
    /// Elemental skill DMG bonus from team buffs.
    pub skill_dmg_bonus: f64,
    /// Elemental burst DMG bonus from team buffs.
    pub burst_dmg_bonus: f64,
    /// Flat damage added to normal attacks from team buffs.
    pub normal_atk_flat_dmg: f64,
    /// Flat damage added to charged attacks from team buffs.
    pub charged_atk_flat_dmg: f64,
    /// Flat damage added to plunging attacks from team buffs.
    pub plunging_atk_flat_dmg: f64,
    /// Flat damage added to elemental skill from team buffs.
    pub skill_flat_dmg: f64,
    /// Flat damage added to elemental burst from team buffs.
    pub burst_flat_dmg: f64,
    /// Amplifying reaction (vaporize/melt) DMG bonus from team buffs.
    pub amplifying_bonus: f64,
    /// Transformative reaction DMG bonus from team buffs.
    pub transformative_bonus: f64,
    /// Additive (catalyze) reaction DMG bonus from team buffs.
    pub additive_bonus: f64,
    /// Exact reaction DMG bonuses that only apply to the matching reaction.
    #[serde(default)]
    pub reaction_dmg_bonuses: Vec<(Reaction, f64)>,
}

impl DamageContext {
    /// Aggregates conditional buffs from resolved buff list into a `DamageContext`.
    pub fn from_buffs(buffs: &[ResolvedBuff]) -> Self {
        let mut ctx = Self::default();
        for buff in buffs {
            match buff.stat {
                BuffableStat::NormalAtkDmgBonus => ctx.normal_atk_dmg_bonus += buff.value,
                BuffableStat::ChargedAtkDmgBonus => ctx.charged_atk_dmg_bonus += buff.value,
                BuffableStat::PlungingAtkDmgBonus => ctx.plunging_atk_dmg_bonus += buff.value,
                BuffableStat::SkillDmgBonus => ctx.skill_dmg_bonus += buff.value,
                BuffableStat::BurstDmgBonus => ctx.burst_dmg_bonus += buff.value,
                BuffableStat::NormalAtkFlatDmg => ctx.normal_atk_flat_dmg += buff.value,
                BuffableStat::ChargedAtkFlatDmg => ctx.charged_atk_flat_dmg += buff.value,
                BuffableStat::PlungingAtkFlatDmg => ctx.plunging_atk_flat_dmg += buff.value,
                BuffableStat::SkillFlatDmg => ctx.skill_flat_dmg += buff.value,
                BuffableStat::BurstFlatDmg => ctx.burst_flat_dmg += buff.value,
                BuffableStat::AmplifyingBonus => ctx.amplifying_bonus += buff.value,
                BuffableStat::TransformativeBonus => ctx.transformative_bonus += buff.value,
                BuffableStat::AdditiveBonus => ctx.additive_bonus += buff.value,
                BuffableStat::ReactionDmgBonus(reaction) => {
                    ctx.reaction_dmg_bonuses.push((reaction, buff.value));
                }
                _ => {}
            }
        }
        ctx
    }

    /// Returns the total reaction bonus for a specific reaction, including
    /// backward-compatible broad category bonuses and exact reaction bonuses.
    pub fn reaction_bonus_for(&self, reaction: Reaction) -> f64 {
        self.broad_reaction_bonus_for(reaction)
            + self
                .reaction_dmg_bonuses
                .iter()
                .filter(|(r, _)| *r == reaction)
                .map(|(_, value)| *value)
                .sum::<f64>()
    }

    fn broad_reaction_bonus_for(&self, reaction: Reaction) -> f64 {
        match reaction.category() {
            ReactionCategory::Amplifying => self.amplifying_bonus,
            ReactionCategory::Catalyze => self.additive_bonus,
            ReactionCategory::Transformative => self.transformative_bonus,
            ReactionCategory::Lunar => self.transformative_bonus,
        }
    }
}

/// Detailed result of team buff resolution.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TeamResolveResult {
    /// Stats before team buffs.
    pub base_stats: Stats,
    /// All buffs applied to the target member.
    pub applied_buffs: Vec<ResolvedBuff>,
    /// Active elemental resonances.
    pub resonances: Vec<ElementalResonance>,
    /// Final stats after all buffs.
    pub final_stats: Stats,
    /// Attack-type-specific DMG bonuses, flat DMG, and reaction bonuses.
    pub damage_context: DamageContext,
    /// Enemy resistance and DEF reduction from team debuffs.
    pub enemy_debuffs: EnemyDebuffs,
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
            | BuffableStat::ElementalCritDmg(_)
            | BuffableStat::ElementalMastery
            | BuffableStat::EnergyRecharge
            | BuffableStat::DmgBonus
            | BuffableStat::AllElementalDmgBonus
            | BuffableStat::ElementalDmgBonus(_)
            | BuffableStat::PhysicalDmgBonus
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
            BuffableStat::ElementalCritDmg(elem) => match elem {
                Element::Pyro => p.pyro_crit_dmg_bonus += buff.value,
                Element::Hydro => p.hydro_crit_dmg_bonus += buff.value,
                Element::Electro => p.electro_crit_dmg_bonus += buff.value,
                Element::Cryo => p.cryo_crit_dmg_bonus += buff.value,
                Element::Dendro => p.dendro_crit_dmg_bonus += buff.value,
                Element::Anemo => p.anemo_crit_dmg_bonus += buff.value,
                Element::Geo => p.geo_crit_dmg_bonus += buff.value,
            },
            BuffableStat::ElementalMastery => p.elemental_mastery += buff.value,
            BuffableStat::EnergyRecharge => p.energy_recharge += buff.value,
            BuffableStat::DmgBonus => p.dmg_bonus += buff.value,
            BuffableStat::AllElementalDmgBonus => {
                p.pyro_dmg_bonus += buff.value;
                p.hydro_dmg_bonus += buff.value;
                p.electro_dmg_bonus += buff.value;
                p.cryo_dmg_bonus += buff.value;
                p.dendro_dmg_bonus += buff.value;
                p.anemo_dmg_bonus += buff.value;
                p.geo_dmg_bonus += buff.value;
            }
            BuffableStat::ElementalDmgBonus(elem) => match elem {
                Element::Pyro => p.pyro_dmg_bonus += buff.value,
                Element::Hydro => p.hydro_dmg_bonus += buff.value,
                Element::Electro => p.electro_dmg_bonus += buff.value,
                Element::Cryo => p.cryo_dmg_bonus += buff.value,
                Element::Dendro => p.dendro_dmg_bonus += buff.value,
                Element::Anemo => p.anemo_dmg_bonus += buff.value,
                Element::Geo => p.geo_dmg_bonus += buff.value,
            },
            BuffableStat::PhysicalDmgBonus => p.physical_dmg_bonus += buff.value,
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

fn collect_buffs(
    team: &[TeamMember],
    target_index: usize,
    resonance_activations: &[(ElementalResonance, bool)],
) -> Vec<ResolvedBuff> {
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

    // Elemental resonance buffs
    let elements: Vec<Element> = team.iter().map(|m| m.element).collect();
    let resonances = determine_resonances(&elements);
    for resonance in &resonances {
        for (stat, value) in resonance_buffs(*resonance) {
            buffs.push(ResolvedBuff {
                source: format!("{:?}", resonance),
                stat,
                value,
                target: BuffTarget::Team,
                origin: None,
            });
        }

        // Conditional resonance buffs (only when activated)
        let is_active = resonance_activations
            .iter()
            .any(|(r, active)| r == resonance && *active);
        if is_active {
            for (stat, value) in resonance_conditional_buffs(*resonance) {
                buffs.push(ResolvedBuff {
                    source: format!("{:?}(conditional)", resonance),
                    stat,
                    value,
                    target: BuffTarget::Team,
                    origin: None,
                });
            }
        }
    }

    dedup_by_origin(&mut buffs);
    buffs
}

/// Deduplicates buffs by `(origin, stat)` pair, keeping only the maximum value.
///
/// Buffs with `origin: None` are always kept (no deduplication).
/// This prevents multiple team members using the same artifact set from stacking
/// the 4pc team buff multiple times.
fn dedup_by_origin(buffs: &mut Vec<ResolvedBuff>) {
    // Pass 1: find max value for each (origin, stat) pair
    let mut max_entries: Vec<(String, BuffableStat, f64)> = Vec::new();
    for buff in buffs.iter() {
        if let Some(ref origin) = buff.origin {
            if let Some(entry) = max_entries
                .iter_mut()
                .find(|(o, s, _)| o == origin && *s == buff.stat)
            {
                if buff.value > entry.2 {
                    entry.2 = buff.value;
                }
            } else {
                max_entries.push((origin.clone(), buff.stat, buff.value));
            }
        }
    }

    // Pass 2: keep origin=None buffs always; for origin=Some, keep only first with max value
    let mut seen: Vec<(String, BuffableStat)> = Vec::new();
    buffs.retain(|buff| {
        if let Some(ref origin) = buff.origin {
            let max_val = max_entries
                .iter()
                .find(|(o, s, _)| o == origin && *s == buff.stat)
                .map(|(_, _, v)| *v)
                .unwrap_or(0.0);
            if (buff.value - max_val).abs() < f64::EPSILON {
                let key = (origin.clone(), buff.stat);
                if seen.iter().any(|s| s.0 == key.0 && s.1 == key.1) {
                    false // already seen this max
                } else {
                    seen.push(key);
                    true
                }
            } else {
                false // not max value
            }
        } else {
            true // no origin, always keep
        }
    });
}

/// Resolves team buffs and returns a [`TeamResolveResult`] for the target member.
///
/// The result includes final stats, applied buffs, elemental resonances,
/// attack-type-specific damage context, and enemy debuffs.
///
/// `resonance_activations` controls conditional resonance buffs (e.g. ShatteringIce CritRate).
/// Pass `&[]` to skip all conditional resonance buffs. If duplicate entries exist for the
/// same resonance, the buff is applied if any entry for that resonance is `true`.
/// Activations for undetected resonances are silently ignored.
///
/// # Errors
///
/// Returns [`CalcError::InvalidTeamSize`] if team is empty or has >4 members.
/// Returns [`CalcError::InvalidTargetIndex`] if target_index is out of bounds.
///
/// # Examples
///
/// ```
/// use genshin_calc_core::*;
///
/// let member = TeamMember {
///     element: Element::Pyro,
///     weapon_type: WeaponType::Sword,
///     stats: StatProfile {
///         base_atk: 800.0,
///         energy_recharge: 1.0,
///         ..Default::default()
///     },
///     buffs_provided: vec![],
///     is_moonsign: false,
///     can_nightsoul: false,
/// };
/// let result = resolve_team_stats(&[member], 0, &[]).unwrap();
/// assert!(result.final_stats.atk > 0.0);
/// ```
pub fn resolve_team_stats(
    team: &[TeamMember],
    target_index: usize,
    resonance_activations: &[(ElementalResonance, bool)],
) -> Result<TeamResolveResult, CalcError> {
    resolve_team_stats_detailed(team, target_index, resonance_activations)
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
    resonance_activations: &[(ElementalResonance, bool)],
) -> Result<TeamResolveResult, CalcError> {
    validate_team(team, target_index)?;

    let base_profile = &team[target_index].stats;
    let base_stats = combine_stats(base_profile)?;

    let applied_buffs = collect_buffs(team, target_index, resonance_activations);
    let buffed_profile = apply_buffs_to_profile(base_profile, &applied_buffs);
    let final_stats = combine_stats(&buffed_profile)?;

    let elements: Vec<Element> = team.iter().map(|m| m.element).collect();
    let resonances = determine_resonances(&elements);

    let damage_context = DamageContext::from_buffs(&applied_buffs);
    let enemy_debuffs = collect_enemy_debuffs(&applied_buffs);

    Ok(TeamResolveResult {
        base_stats,
        applied_buffs,
        resonances,
        final_stats,
        damage_context,
        enemy_debuffs,
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
            is_moonsign: false,
            can_nightsoul: false,
        }
    }

    #[test]
    fn test_empty_team_error() {
        let result = resolve_team_stats(&[], 0, &[]);
        assert_eq!(result, Err(CalcError::InvalidTeamSize(0)));
    }

    #[test]
    fn test_five_member_team_error() {
        let team: Vec<TeamMember> = (0..5).map(|_| make_member(Element::Pyro, 800.0)).collect();
        let result = resolve_team_stats(&team, 0, &[]);
        assert_eq!(result, Err(CalcError::InvalidTeamSize(5)));
    }

    #[test]
    fn test_target_index_out_of_bounds() {
        let team = vec![make_member(Element::Pyro, 800.0)];
        let result = resolve_team_stats(&team, 1, &[]);
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
        let result = resolve_team_stats(&team, 0, &[]).unwrap();
        assert!((result.final_stats.atk - 800.0).abs() < EPSILON);
    }

    #[test]
    fn test_self_buff_applies_to_self() {
        let mut member = make_member(Element::Pyro, 800.0);
        member.buffs_provided.push(ResolvedBuff {
            source: "Weapon Passive".into(),
            stat: BuffableStat::AtkPercent,
            value: 0.20,
            target: BuffTarget::OnlySelf,
            origin: None,
        });
        let team = vec![member, make_member(Element::Hydro, 700.0)];

        // Member 0 gets the self buff
        let result0 = resolve_team_stats(&team, 0, &[]).unwrap();
        assert!((result0.final_stats.atk - 800.0 * (1.0 + 0.20)).abs() < EPSILON);

        // Member 1 does NOT get it
        let result1 = resolve_team_stats(&team, 1, &[]).unwrap();
        assert!((result1.final_stats.atk - 700.0).abs() < EPSILON);
    }

    #[test]
    fn test_team_buff_applies_to_all() {
        let mut bennett = make_member(Element::Pyro, 800.0);
        bennett.buffs_provided.push(ResolvedBuff {
            source: "Bennett Burst".into(),
            stat: BuffableStat::AtkFlat,
            value: 1000.0,
            target: BuffTarget::Team,
            origin: None,
        });
        let dps = make_member(Element::Pyro, 900.0);
        let team = vec![bennett, dps];

        // Both members get the team buff
        let result0 = resolve_team_stats(&team, 0, &[]).unwrap();
        assert!((result0.final_stats.atk - (800.0 + 1000.0)).abs() < EPSILON);

        let result1 = resolve_team_stats(&team, 1, &[]).unwrap();
        assert!((result1.final_stats.atk - (900.0 + 1000.0)).abs() < EPSILON);
    }

    #[test]
    fn test_team_exclude_self_buff() {
        let mut rosaria = make_member(Element::Cryo, 700.0);
        rosaria.buffs_provided.push(ResolvedBuff {
            source: "Rosaria A4".into(),
            stat: BuffableStat::CritRate,
            value: 0.15,
            target: BuffTarget::TeamExcludeSelf,
            origin: None,
        });
        let dps = make_member(Element::Pyro, 900.0);
        let team = vec![rosaria, dps];

        // Rosaria does NOT get her own buff
        let result0 = resolve_team_stats(&team, 0, &[]).unwrap();
        assert!((result0.final_stats.crit_rate - 0.50).abs() < EPSILON);

        // DPS gets the buff
        let result1 = resolve_team_stats(&team, 1, &[]).unwrap();
        assert!((result1.final_stats.crit_rate - 0.65).abs() < EPSILON);
    }

    #[test]
    fn test_pyro_resonance_with_4_members() {
        let team = vec![
            make_member(Element::Pyro, 800.0),
            make_member(Element::Pyro, 700.0),
            make_member(Element::Hydro, 600.0),
            make_member(Element::Cryo, 500.0),
        ];
        let result = resolve_team_stats(&team, 0, &[]).unwrap();
        // base_atk=800, atk_percent=0.25 from resonance → 800*(1+0.25)=1000
        assert!((result.final_stats.atk - 1000.0).abs() < EPSILON);
    }

    #[test]
    fn test_no_resonance_with_3_members() {
        let team = vec![
            make_member(Element::Pyro, 800.0),
            make_member(Element::Pyro, 700.0),
            make_member(Element::Hydro, 600.0),
        ];
        let result = resolve_team_stats(&team, 0, &[]).unwrap();
        // No resonance — base_atk only
        assert!((result.final_stats.atk - 800.0).abs() < EPSILON);
    }

    #[test]
    fn test_detailed_result_includes_resonances() {
        let team = vec![
            make_member(Element::Pyro, 800.0),
            make_member(Element::Pyro, 700.0),
            make_member(Element::Hydro, 600.0),
            make_member(Element::Cryo, 500.0),
        ];
        let result = resolve_team_stats_detailed(&team, 0, &[]).unwrap();
        assert!(
            result
                .resonances
                .contains(&ElementalResonance::FerventFlames)
        );
        assert!(!result.applied_buffs.is_empty());
        assert!(result.final_stats.atk > result.base_stats.atk);
    }

    #[test]
    fn elemental_dmg_bonus_buff_applies_to_element_field() {
        let base = StatProfile::default();
        let buffs = vec![ResolvedBuff {
            source: "test".to_string(),
            stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
            value: 0.15,
            target: BuffTarget::Team,
            origin: None,
        }];
        let result = apply_buffs_to_profile(&base, &buffs);
        assert!((result.pyro_dmg_bonus - 0.15).abs() < 1e-10);
        assert!((result.hydro_dmg_bonus - 0.0).abs() < 1e-10);
    }

    #[test]
    fn physical_dmg_bonus_buff_applies_to_physical_field() {
        let base = StatProfile::default();
        let buffs = vec![ResolvedBuff {
            source: "test".to_string(),
            stat: BuffableStat::PhysicalDmgBonus,
            value: 0.25,
            target: BuffTarget::Team,
            origin: None,
        }];
        let result = apply_buffs_to_profile(&base, &buffs);
        assert!((result.physical_dmg_bonus - 0.25).abs() < 1e-10);
        assert!((result.dmg_bonus - 0.0).abs() < 1e-10);
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
                origin: None,
            },
            ResolvedBuff {
                source: "conditional".into(),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.30,
                target: BuffTarget::Team,
                origin: None,
            },
        ];
        let result = apply_buffs_to_profile(&profile, &buffs);
        // AtkPercent applied, NormalAtkDmgBonus skipped
        assert!((result.atk_percent - 0.20).abs() < EPSILON);
        assert!((result.dmg_bonus - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_damage_context_from_buffs_empty() {
        let ctx = DamageContext::from_buffs(&[]);
        assert!((ctx.normal_atk_dmg_bonus - 0.0).abs() < EPSILON);
        assert!((ctx.skill_flat_dmg - 0.0).abs() < EPSILON);
        assert!((ctx.amplifying_bonus - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_damage_context_deserializes_without_reaction_dmg_bonuses() {
        let json = r#"{
            "normal_atk_dmg_bonus": 0.0,
            "charged_atk_dmg_bonus": 0.0,
            "plunging_atk_dmg_bonus": 0.0,
            "skill_dmg_bonus": 0.0,
            "burst_dmg_bonus": 0.0,
            "normal_atk_flat_dmg": 0.0,
            "charged_atk_flat_dmg": 0.0,
            "plunging_atk_flat_dmg": 0.0,
            "skill_flat_dmg": 0.0,
            "burst_flat_dmg": 0.0,
            "amplifying_bonus": 0.0,
            "transformative_bonus": 0.0,
            "additive_bonus": 0.0
        }"#;

        let ctx: DamageContext = serde_json::from_str(json).unwrap();

        assert!(ctx.reaction_dmg_bonuses.is_empty());
    }

    #[test]
    fn test_damage_context_from_buffs_mixed() {
        let buffs = vec![
            ResolvedBuff {
                source: "Yelan A4".into(),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.25,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "Shenhe E".into(),
                stat: BuffableStat::SkillFlatDmg,
                value: 3000.0,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "Shenhe E".into(),
                stat: BuffableStat::BurstFlatDmg,
                value: 3000.0,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "Bennett Burst".into(),
                stat: BuffableStat::AtkFlat,
                value: 1000.0,
                target: BuffTarget::Team,
                origin: None,
            },
        ];
        let ctx = DamageContext::from_buffs(&buffs);
        assert!((ctx.normal_atk_dmg_bonus - 0.25).abs() < EPSILON);
        assert!((ctx.charged_atk_dmg_bonus - 0.0).abs() < EPSILON);
        assert!((ctx.skill_flat_dmg - 3000.0).abs() < EPSILON);
        assert!((ctx.burst_flat_dmg - 3000.0).abs() < EPSILON);
        assert!((ctx.normal_atk_flat_dmg - 0.0).abs() < EPSILON);
        assert!((ctx.amplifying_bonus - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_damage_context_reaction_bonuses() {
        let buffs = vec![
            ResolvedBuff {
                source: "4pc CW".into(),
                stat: BuffableStat::AmplifyingBonus,
                value: 0.15,
                target: BuffTarget::OnlySelf,
                origin: None,
            },
            ResolvedBuff {
                source: "Sucrose C6".into(),
                stat: BuffableStat::AdditiveBonus,
                value: 0.20,
                target: BuffTarget::Team,
                origin: None,
            },
        ];
        let ctx = DamageContext::from_buffs(&buffs);
        assert!((ctx.amplifying_bonus - 0.15).abs() < EPSILON);
        assert!((ctx.additive_bonus - 0.20).abs() < EPSILON);
        assert!((ctx.transformative_bonus - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_damage_context_all_type_dmg_bonuses() {
        let buffs = vec![
            ResolvedBuff {
                source: "a".into(),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.10,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "b".into(),
                stat: BuffableStat::ChargedAtkDmgBonus,
                value: 0.20,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "c".into(),
                stat: BuffableStat::PlungingAtkDmgBonus,
                value: 0.30,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "d".into(),
                stat: BuffableStat::SkillDmgBonus,
                value: 0.40,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "e".into(),
                stat: BuffableStat::BurstDmgBonus,
                value: 0.50,
                target: BuffTarget::Team,
                origin: None,
            },
        ];
        let ctx = DamageContext::from_buffs(&buffs);
        assert!((ctx.normal_atk_dmg_bonus - 0.10).abs() < EPSILON);
        assert!((ctx.charged_atk_dmg_bonus - 0.20).abs() < EPSILON);
        assert!((ctx.plunging_atk_dmg_bonus - 0.30).abs() < EPSILON);
        assert!((ctx.skill_dmg_bonus - 0.40).abs() < EPSILON);
        assert!((ctx.burst_dmg_bonus - 0.50).abs() < EPSILON);
    }

    #[test]
    fn test_damage_context_all_type_flat_dmgs() {
        let buffs = vec![
            ResolvedBuff {
                source: "a".into(),
                stat: BuffableStat::NormalAtkFlatDmg,
                value: 100.0,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "b".into(),
                stat: BuffableStat::ChargedAtkFlatDmg,
                value: 200.0,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "c".into(),
                stat: BuffableStat::PlungingAtkFlatDmg,
                value: 300.0,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "d".into(),
                stat: BuffableStat::SkillFlatDmg,
                value: 400.0,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "e".into(),
                stat: BuffableStat::BurstFlatDmg,
                value: 500.0,
                target: BuffTarget::Team,
                origin: None,
            },
        ];
        let ctx = DamageContext::from_buffs(&buffs);
        assert!((ctx.normal_atk_flat_dmg - 100.0).abs() < EPSILON);
        assert!((ctx.charged_atk_flat_dmg - 200.0).abs() < EPSILON);
        assert!((ctx.plunging_atk_flat_dmg - 300.0).abs() < EPSILON);
        assert!((ctx.skill_flat_dmg - 400.0).abs() < EPSILON);
        assert!((ctx.burst_flat_dmg - 500.0).abs() < EPSILON);
    }

    #[test]
    fn test_damage_context_stacks_same_type() {
        let buffs = vec![
            ResolvedBuff {
                source: "Freedom-Sworn".into(),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.16,
                target: BuffTarget::Team,
                origin: None,
            },
            ResolvedBuff {
                source: "Yun Jin A4".into(),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.05,
                target: BuffTarget::Team,
                origin: None,
            },
        ];
        let ctx = DamageContext::from_buffs(&buffs);
        assert!((ctx.normal_atk_dmg_bonus - 0.21).abs() < EPSILON);
    }

    #[test]
    fn reaction_dmg_bonus_exact_match_does_not_over_apply() {
        use crate::reaction::Reaction;
        use crate::types::Element;

        let ctx = DamageContext::from_buffs(&[
            ResolvedBuff {
                source: "Bloom Only".to_string(),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Bloom),
                value: 0.40,
                target: BuffTarget::OnlySelf,
                origin: None,
            },
            ResolvedBuff {
                source: "Aggravate Only".to_string(),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Aggravate),
                value: 0.20,
                target: BuffTarget::OnlySelf,
                origin: None,
            },
            ResolvedBuff {
                source: "Pyro Swirl Only".to_string(),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Swirl(Element::Pyro)),
                value: 0.60,
                target: BuffTarget::OnlySelf,
                origin: None,
            },
        ]);

        assert!((ctx.reaction_bonus_for(Reaction::Bloom) - 0.40).abs() < 1e-9);
        assert!((ctx.reaction_bonus_for(Reaction::Overloaded) - 0.0).abs() < 1e-9);
        assert!((ctx.reaction_bonus_for(Reaction::Aggravate) - 0.20).abs() < 1e-9);
        assert!((ctx.reaction_bonus_for(Reaction::Spread) - 0.0).abs() < 1e-9);
        assert!((ctx.reaction_bonus_for(Reaction::Swirl(Element::Pyro)) - 0.60).abs() < 1e-9);
        assert!((ctx.reaction_bonus_for(Reaction::Swirl(Element::Hydro)) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn reaction_dmg_bonus_combines_broad_and_exact_values() {
        use crate::reaction::Reaction;

        let ctx = DamageContext::from_buffs(&[
            ResolvedBuff {
                source: "Broad Transformative".to_string(),
                stat: BuffableStat::TransformativeBonus,
                value: 0.10,
                target: BuffTarget::OnlySelf,
                origin: None,
            },
            ResolvedBuff {
                source: "Bloom Exact".to_string(),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Bloom),
                value: 0.40,
                target: BuffTarget::OnlySelf,
                origin: None,
            },
        ]);

        assert!((ctx.reaction_bonus_for(Reaction::Bloom) - 0.50).abs() < 1e-9);
        assert!((ctx.reaction_bonus_for(Reaction::Overloaded) - 0.10).abs() < 1e-9);
    }

    #[test]
    fn reaction_dmg_bonus_lunar_exact_match_does_not_over_apply() {
        use crate::reaction::Reaction;

        let ctx = DamageContext::from_buffs(&[ResolvedBuff {
            source: "Lunar Bloom Only".to_string(),
            stat: BuffableStat::ReactionDmgBonus(Reaction::LunarBloom),
            value: 0.15,
            target: BuffTarget::OnlySelf,
            origin: None,
        }]);

        assert!((ctx.reaction_bonus_for(Reaction::LunarBloom) - 0.15).abs() < 1e-9);
        assert!((ctx.reaction_bonus_for(Reaction::LunarElectroCharged) - 0.0).abs() < 1e-9);
    }

    #[test]
    fn test_resolve_team_stats_returns_full_result() {
        let mut support = make_member(Element::Cryo, 700.0);
        support.buffs_provided.push(ResolvedBuff {
            source: "Citlali Q".into(),
            stat: BuffableStat::ElementalResReduction(Element::Pyro),
            value: 0.20,
            target: BuffTarget::Team,
            origin: None,
        });
        support.buffs_provided.push(ResolvedBuff {
            source: "Freedom-Sworn".into(),
            stat: BuffableStat::NormalAtkDmgBonus,
            value: 0.16,
            target: BuffTarget::Team,
            origin: None,
        });
        support.buffs_provided.push(ResolvedBuff {
            source: "Shenhe E".into(),
            stat: BuffableStat::NormalAtkFlatDmg,
            value: 2500.0,
            target: BuffTarget::Team,
            origin: None,
        });
        let dps = make_member(Element::Pyro, 900.0);
        let team = vec![support, dps];

        let result = resolve_team_stats(&team, 1, &[]).unwrap();
        assert!(result.final_stats.atk > 0.0);
        assert!((result.damage_context.normal_atk_dmg_bonus - 0.16).abs() < EPSILON);
        assert!((result.damage_context.normal_atk_flat_dmg - 2500.0).abs() < EPSILON);
        assert!((result.enemy_debuffs.pyro_res_reduction - 0.20).abs() < EPSILON);
        assert!((result.enemy_debuffs.cryo_res_reduction - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_dedup_same_origin_keeps_max() {
        // Two team members both providing Noblesse 4pc ATK buff
        // Member with higher value (0.25) should win, lower (0.20) should be deduped
        let mut member1 = make_member(Element::Pyro, 800.0);
        member1.buffs_provided.push(ResolvedBuff {
            source: "ATK% Up (Noblesse 4pc)".into(),
            stat: BuffableStat::AtkPercent,
            value: 0.20,
            target: BuffTarget::Team,
            origin: Some("noblesse_oblige".to_string()),
        });

        let mut member2 = make_member(Element::Hydro, 700.0);
        member2.buffs_provided.push(ResolvedBuff {
            source: "ATK% Up (Noblesse 4pc)".into(),
            stat: BuffableStat::AtkPercent,
            value: 0.25,
            target: BuffTarget::Team,
            origin: Some("noblesse_oblige".to_string()),
        });

        let dps = make_member(Element::Cryo, 900.0);
        let team = vec![member1, member2, dps];

        let result = resolve_team_stats(&team, 2, &[]).unwrap();
        // Only max (0.25) applies; lower (0.20) is deduped
        // base_atk=900, atk_percent=0.25 → 900 * 1.25 = 1125
        assert!((result.final_stats.atk - 1125.0).abs() < EPSILON);
        // Exactly one buff with origin "noblesse_oblige"
        let noblesse_buffs: Vec<_> = result
            .applied_buffs
            .iter()
            .filter(|b| b.origin.as_deref() == Some("noblesse_oblige"))
            .collect();
        assert_eq!(noblesse_buffs.len(), 1);
        assert!((noblesse_buffs[0].value - 0.25).abs() < EPSILON);
    }

    #[test]
    fn test_different_origin_not_deduped() {
        // One member with Noblesse ATK +20%, another with Millelith ATK +20%
        // Both should apply (different origins)
        let mut member1 = make_member(Element::Pyro, 800.0);
        member1.buffs_provided.push(ResolvedBuff {
            source: "ATK% Up (Noblesse 4pc)".into(),
            stat: BuffableStat::AtkPercent,
            value: 0.20,
            target: BuffTarget::Team,
            origin: Some("noblesse_oblige".to_string()),
        });

        let mut member2 = make_member(Element::Hydro, 700.0);
        member2.buffs_provided.push(ResolvedBuff {
            source: "ATK% Up (Millelith 4pc)".into(),
            stat: BuffableStat::AtkPercent,
            value: 0.20,
            target: BuffTarget::Team,
            origin: Some("tenacity_of_the_millelith".to_string()),
        });

        let dps = make_member(Element::Cryo, 900.0);
        let team = vec![member1, member2, dps];

        let result = resolve_team_stats(&team, 2, &[]).unwrap();
        // Both apply: 900 * (1 + 0.20 + 0.20) = 900 * 1.40 = 1260
        assert!((result.final_stats.atk - 1260.0).abs() < EPSILON);
        assert_eq!(
            result
                .applied_buffs
                .iter()
                .filter(|b| b.origin.is_some())
                .count(),
            2
        );
    }

    #[test]
    fn test_origin_none_never_deduped() {
        // Two members both providing weapon ATK buff with origin: None
        // Both should apply (origin=None is never deduped)
        let mut member1 = make_member(Element::Pyro, 800.0);
        member1.buffs_provided.push(ResolvedBuff {
            source: "Weapon ATK%".into(),
            stat: BuffableStat::AtkPercent,
            value: 0.20,
            target: BuffTarget::Team,
            origin: None,
        });

        let mut member2 = make_member(Element::Hydro, 700.0);
        member2.buffs_provided.push(ResolvedBuff {
            source: "Weapon ATK%".into(),
            stat: BuffableStat::AtkPercent,
            value: 0.20,
            target: BuffTarget::Team,
            origin: None,
        });

        let dps = make_member(Element::Cryo, 900.0);
        let team = vec![member1, member2, dps];

        let result = resolve_team_stats(&team, 2, &[]).unwrap();
        // Both apply: 900 * (1 + 0.20 + 0.20) = 900 * 1.40 = 1260
        assert!((result.final_stats.atk - 1260.0).abs() < EPSILON);
        // Both origin=None buffs present
        let no_origin_atk_buffs: Vec<_> = result
            .applied_buffs
            .iter()
            .filter(|b| b.origin.is_none() && b.stat == BuffableStat::AtkPercent)
            .collect();
        assert_eq!(no_origin_atk_buffs.len(), 2);
    }

    #[test]
    fn test_cryo_resonance_activation_applies_crit_rate() {
        let team = vec![
            make_member(Element::Cryo, 800.0),
            make_member(Element::Cryo, 600.0),
            make_member(Element::Pyro, 700.0),
            make_member(Element::Hydro, 500.0),
        ];
        let activations = [(ElementalResonance::ShatteringIce, true)];
        let result = resolve_team_stats(&team, 0, &activations).unwrap();
        let has_crit = result
            .applied_buffs
            .iter()
            .any(|b| b.stat == BuffableStat::CritRate && (b.value - 0.15).abs() < EPSILON);
        assert!(
            has_crit,
            "ShatteringIce conditional CritRate +0.15 should be in applied_buffs"
        );
    }

    #[test]
    fn test_resonance_activation_false_does_not_apply() {
        let team = vec![
            make_member(Element::Cryo, 800.0),
            make_member(Element::Cryo, 600.0),
            make_member(Element::Pyro, 700.0),
            make_member(Element::Hydro, 500.0),
        ];
        let activations = [(ElementalResonance::ShatteringIce, false)];
        let result = resolve_team_stats(&team, 0, &activations).unwrap();
        let has_crit = result
            .applied_buffs
            .iter()
            .any(|b| b.source.contains("ShatteringIce") && b.stat == BuffableStat::CritRate);
        assert!(
            !has_crit,
            "ShatteringIce with active=false should not appear"
        );
    }

    #[test]
    fn test_empty_activations_backward_compatible() {
        let team = vec![
            make_member(Element::Cryo, 800.0),
            make_member(Element::Cryo, 600.0),
            make_member(Element::Pyro, 700.0),
            make_member(Element::Hydro, 500.0),
        ];
        let result = resolve_team_stats(&team, 0, &[]).unwrap();
        let has_crit = result
            .applied_buffs
            .iter()
            .any(|b| b.source.contains("ShatteringIce"));
        assert!(
            !has_crit,
            "Empty activations should not apply conditional buffs"
        );
    }

    #[test]
    fn test_activation_ignored_when_resonance_not_detected() {
        let team = vec![
            make_member(Element::Cryo, 800.0),
            make_member(Element::Cryo, 600.0),
            make_member(Element::Pyro, 700.0),
        ];
        let activations = [(ElementalResonance::ShatteringIce, true)];
        let result = resolve_team_stats(&team, 0, &activations).unwrap();
        assert!(result.resonances.is_empty());
        let has_crit = result
            .applied_buffs
            .iter()
            .any(|b| b.source.contains("ShatteringIce"));
        assert!(
            !has_crit,
            "Activation without detected resonance should be ignored"
        );
    }

    #[test]
    fn test_double_resonance_partial_activation() {
        let team = vec![
            make_member(Element::Cryo, 800.0),
            make_member(Element::Cryo, 600.0),
            make_member(Element::Geo, 700.0),
            make_member(Element::Geo, 500.0),
        ];
        let activations = [(ElementalResonance::ShatteringIce, true)];
        let result = resolve_team_stats(&team, 0, &activations).unwrap();
        let has_crit = result
            .applied_buffs
            .iter()
            .any(|b| b.stat == BuffableStat::CritRate && (b.value - 0.15).abs() < EPSILON);
        assert!(has_crit, "ShatteringIce should be active");
        let has_dmg = result
            .applied_buffs
            .iter()
            .any(|b| b.source.contains("EnduringRock"));
        assert!(
            !has_dmg,
            "EnduringRock should NOT be active (not in activations)"
        );
    }

    #[test]
    fn all_elemental_dmg_bonus_applies_to_seven_elements_not_physical() {
        let profile = StatProfile::default();
        let buffs = vec![ResolvedBuff {
            stat: BuffableStat::AllElementalDmgBonus,
            value: 0.20,
            source: "test".to_string(),
            target: BuffTarget::Team,
            origin: None,
        }];
        let result = apply_buffs_to_profile(&profile, &buffs);

        // Should apply to all 7 elemental bonuses
        assert!((result.pyro_dmg_bonus - 0.20).abs() < EPSILON);
        assert!((result.hydro_dmg_bonus - 0.20).abs() < EPSILON);
        assert!((result.electro_dmg_bonus - 0.20).abs() < EPSILON);
        assert!((result.cryo_dmg_bonus - 0.20).abs() < EPSILON);
        assert!((result.dendro_dmg_bonus - 0.20).abs() < EPSILON);
        assert!((result.anemo_dmg_bonus - 0.20).abs() < EPSILON);
        assert!((result.geo_dmg_bonus - 0.20).abs() < EPSILON);

        // Should NOT apply to generic dmg_bonus or physical
        assert!(result.dmg_bonus.abs() < EPSILON);
        assert!(result.physical_dmg_bonus.abs() < EPSILON);
    }
}
