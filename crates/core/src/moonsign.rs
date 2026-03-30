use serde::{Deserialize, Serialize};

use crate::buff_types::BuffableStat;
use crate::enemy::Enemy;
use crate::error::CalcError;
use crate::lunar::{LunarInput, LunarResult, calculate_lunar};
use crate::reaction::Reaction;
use crate::team::BuffTarget;
use crate::types::{Element, ScalingStat};

/// Moonsign level determined by the number of Moonsign characters in the team.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MoonsignLevel {
    /// No Moonsign characters.
    None,
    /// 初照: 1 Moonsign character.
    NascentGleam,
    /// 満照: 2+ Moonsign characters.
    AscendantGleam,
}

/// Calculated Moonsign Benediction passive info for a character.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MoonsignBenediction {
    /// Calculated Base DMG Bonus value (0.0 to max_bonus).
    pub base_dmg_bonus: f64,
    /// Lunar reaction types this character enables.
    pub enabled_reactions: Vec<Reaction>,
}

/// Moonsign-level dependent talent enhancement.
///
/// Note: only `Serialize` is derived (not `Deserialize`) because `&'static str`
/// fields cannot implement `Deserialize` generically.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MoonsignTalentEnhancement {
    pub character_name: &'static str,
    pub required_level: MoonsignLevel,
    pub description: &'static str,
    pub effect: MoonsignTalentEffect,
}

/// Effect type for talent enhancements.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MoonsignTalentEffect {
    /// Grant crit to a specific lunar reaction.
    GrantReactionCrit {
        reaction: Reaction,
        crit_rate: f64,
        crit_dmg: f64,
    },
    /// Stat buff.
    StatBuff {
        stat: BuffableStat,
        value: f64,
        target: BuffTarget,
    },
    /// Flat DMG bonus added to reaction_bonus for a specific lunar reaction.
    ReactionDmgBonus { reaction: Reaction, bonus: f64 },
}

/// Team-level Moonsign context.
///
/// Note: only `Serialize` is derived (not `Deserialize`) because `talent_enhancements`
/// contains `MoonsignTalentEnhancement` which has `&'static str` fields.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct MoonsignContext {
    pub level: MoonsignLevel,
    /// BaseDMGBonus per reaction type (Vec for WASM compatibility).
    pub base_dmg_bonus_by_reaction: Vec<(Reaction, f64)>,
    /// Non-moonsign character lunar DMG bonus (max 0.36, only at AscendantGleam).
    pub non_moonsign_lunar_bonus: f64,
    /// Active talent enhancements.
    pub talent_enhancements: Vec<MoonsignTalentEnhancement>,
}

impl MoonsignContext {
    /// Look up BaseDMGBonus for a specific reaction type.
    pub fn base_dmg_bonus_for(&self, reaction: Reaction) -> f64 {
        self.base_dmg_bonus_by_reaction
            .iter()
            .find(|(r, _)| *r == reaction)
            .map(|(_, v)| *v)
            .unwrap_or(0.0)
    }
}

pub fn determine_moonsign_level(moonsign_count: usize) -> MoonsignLevel {
    match moonsign_count {
        0 => MoonsignLevel::None,
        1 => MoonsignLevel::NascentGleam,
        _ => MoonsignLevel::AscendantGleam,
    }
}

/// Non-Moonsign character lunar buff definition.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct NonMoonsignLunarBuff {
    pub scaling_stat: ScalingStat,
    /// Rate per 1 unit of stat.
    pub rate: f64,
    pub max_bonus: f64,
}

/// Returns the non-moonsign buff scaling for a given element.
pub fn non_moonsign_scaling(element: Element) -> NonMoonsignLunarBuff {
    match element {
        Element::Pyro | Element::Electro | Element::Cryo => NonMoonsignLunarBuff {
            scaling_stat: ScalingStat::Atk,
            rate: 0.00009,
            max_bonus: 0.36,
        },
        Element::Hydro => NonMoonsignLunarBuff {
            scaling_stat: ScalingStat::Hp,
            rate: 0.000006,
            max_bonus: 0.36,
        },
        Element::Geo => NonMoonsignLunarBuff {
            scaling_stat: ScalingStat::Def,
            rate: 0.0001,
            max_bonus: 0.36,
        },
        Element::Anemo | Element::Dendro => NonMoonsignLunarBuff {
            scaling_stat: ScalingStat::Em,
            rate: 0.000225,
            max_bonus: 0.36,
        },
    }
}

pub fn calculate_non_moonsign_bonus(buff: &NonMoonsignLunarBuff, stat_value: f64) -> f64 {
    (buff.rate * stat_value).min(buff.max_bonus)
}

pub fn select_non_moonsign_buff(members: &[(Element, f64)]) -> f64 {
    members
        .iter()
        .map(|(elem, stat)| {
            let buff = non_moonsign_scaling(*elem);
            calculate_non_moonsign_bonus(&buff, *stat)
        })
        .fold(0.0_f64, f64::max)
}

const CONTRIBUTION_WEIGHTS: [f64; 4] = [1.0, 0.5, 1.0 / 12.0, 1.0 / 12.0];

/// A single character's contribution to a lunar reaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LunarContribution {
    pub input: LunarInput,
}

/// Calculate weighted lunar reaction damage from multiple contributors.
///
/// # Errors
/// - `CalcError::InvalidTeamSize` if contributions is empty or > 4
pub fn calculate_lunar_team(
    contributions: &[LunarContribution],
    enemy: &Enemy,
) -> Result<LunarResult, CalcError> {
    if contributions.is_empty() || contributions.len() > 4 {
        return Err(CalcError::InvalidTeamSize(contributions.len()));
    }

    let mut results: Vec<LunarResult> = contributions
        .iter()
        .map(|c| calculate_lunar(&c.input, enemy))
        .collect::<Result<Vec<_>, _>>()?;

    // Sort by average descending
    results.sort_by(|a, b| {
        b.average
            .partial_cmp(&a.average)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut non_crit = 0.0;
    let mut crit = 0.0;
    let mut average = 0.0;

    for (i, result) in results.iter().enumerate() {
        let weight = CONTRIBUTION_WEIGHTS[i];
        non_crit += result.non_crit * weight;
        crit += result.crit * weight;
        average += result.average * weight;
    }

    // Use the damage element from the highest contributor
    let damage_element = results[0].damage_element;

    Ok(LunarResult {
        non_crit,
        crit,
        average,
        damage_element,
    })
}

/// Apply Moonsign talent enhancements to a LunarInput.
/// Matching GrantReactionCrit effects add crit_rate (capped at 1.0) and crit_dmg.
/// Matching ReactionDmgBonus effects add to reaction_bonus.
pub fn apply_moonsign_enhancements(
    input: &LunarInput,
    enhancements: &[MoonsignTalentEnhancement],
) -> LunarInput {
    let mut result = input.clone();
    for enh in enhancements {
        match &enh.effect {
            MoonsignTalentEffect::GrantReactionCrit {
                reaction,
                crit_rate,
                crit_dmg,
            } => {
                if *reaction == input.reaction {
                    result.crit_rate = (result.crit_rate + crit_rate).min(1.0);
                    result.crit_dmg += crit_dmg;
                }
            }
            MoonsignTalentEffect::ReactionDmgBonus { reaction, bonus } => {
                if *reaction == input.reaction {
                    result.reaction_bonus += bonus;
                }
            }
            MoonsignTalentEffect::StatBuff { .. } => {
                // StatBuff is applied at the stat-profile level, not directly to LunarInput
            }
        }
    }
    result
}

/// Resolve the team-level Moonsign context.
pub fn resolve_moonsign_context(
    moonsign_count: usize,
    benedictions: &[MoonsignBenediction],
    non_moonsign_bonus: f64,
    enhancements: Vec<MoonsignTalentEnhancement>,
) -> MoonsignContext {
    let level = determine_moonsign_level(moonsign_count);

    let mut by_reaction: Vec<(Reaction, f64)> = Vec::new();
    for bene in benedictions {
        for &reaction in &bene.enabled_reactions {
            if let Some(entry) = by_reaction.iter_mut().find(|(r, _)| *r == reaction) {
                entry.1 += bene.base_dmg_bonus;
            } else {
                by_reaction.push((reaction, bene.base_dmg_bonus));
            }
        }
    }

    MoonsignContext {
        level,
        base_dmg_bonus_by_reaction: by_reaction,
        non_moonsign_lunar_bonus: non_moonsign_bonus,
        talent_enhancements: enhancements,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-9;

    fn default_enemy() -> Enemy {
        Enemy {
            level: 90,
            resistance: 0.1,
            def_reduction: 0.0,
        }
    }

    fn lunar_ec_input() -> LunarInput {
        LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 0.5,
            crit_dmg: 1.0,
            base_dmg_bonus: 0.0,
        }
    }

    fn lunar_bloom_input() -> LunarInput {
        LunarInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::LunarBloom,
            reaction_bonus: 0.0,
            crit_rate: 0.5,
            crit_dmg: 1.0,
            base_dmg_bonus: 0.0,
        }
    }

    // =====================================================================
    // MoonsignLevel tests
    // =====================================================================

    #[test]
    fn test_moonsign_level_none() {
        assert_eq!(determine_moonsign_level(0), MoonsignLevel::None);
    }

    #[test]
    fn test_moonsign_level_nascent() {
        assert_eq!(determine_moonsign_level(1), MoonsignLevel::NascentGleam);
    }

    #[test]
    fn test_moonsign_level_ascendant() {
        assert_eq!(determine_moonsign_level(2), MoonsignLevel::AscendantGleam);
        assert_eq!(determine_moonsign_level(4), MoonsignLevel::AscendantGleam);
    }

    // =====================================================================
    // Context lookup tests
    // =====================================================================

    #[test]
    fn test_moonsign_context_lookup_found() {
        let ctx = MoonsignContext {
            level: MoonsignLevel::AscendantGleam,
            base_dmg_bonus_by_reaction: vec![(Reaction::LunarElectroCharged, 0.14)],
            non_moonsign_lunar_bonus: 0.0,
            talent_enhancements: vec![],
        };
        let bonus = ctx.base_dmg_bonus_for(Reaction::LunarElectroCharged);
        assert!((bonus - 0.14).abs() < EPSILON);
    }

    #[test]
    fn test_moonsign_context_lookup_not_found() {
        let ctx = MoonsignContext {
            level: MoonsignLevel::None,
            base_dmg_bonus_by_reaction: vec![],
            non_moonsign_lunar_bonus: 0.0,
            talent_enhancements: vec![],
        };
        let bonus = ctx.base_dmg_bonus_for(Reaction::LunarElectroCharged);
        assert!((bonus - 0.0).abs() < EPSILON);
    }

    // =====================================================================
    // Non-moonsign buff tests
    // =====================================================================

    #[test]
    fn test_non_moonsign_scaling_pyro() {
        let buff = non_moonsign_scaling(Element::Pyro);
        assert_eq!(buff.scaling_stat, ScalingStat::Atk);
        assert!((buff.max_bonus - 0.36).abs() < EPSILON);
    }

    #[test]
    fn test_non_moonsign_scaling_hydro() {
        let buff = non_moonsign_scaling(Element::Hydro);
        assert_eq!(buff.scaling_stat, ScalingStat::Hp);
    }

    #[test]
    fn test_non_moonsign_scaling_anemo() {
        let buff = non_moonsign_scaling(Element::Anemo);
        assert_eq!(buff.scaling_stat, ScalingStat::Em);
    }

    #[test]
    fn test_calculate_non_moonsign_bonus_atk_capped() {
        let buff = non_moonsign_scaling(Element::Pyro);
        // 4000 ATK * 0.00009 = 0.36, exactly at cap
        let result_4000 = calculate_non_moonsign_bonus(&buff, 4000.0);
        assert!((result_4000 - 0.36).abs() < EPSILON);
        // 5000 ATK * 0.00009 = 0.45, still capped at 0.36
        let result_5000 = calculate_non_moonsign_bonus(&buff, 5000.0);
        assert!((result_5000 - 0.36).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_non_moonsign_bonus_below_cap() {
        let buff = non_moonsign_scaling(Element::Pyro);
        // 2000 ATK * 0.00009 = 0.18
        let result = calculate_non_moonsign_bonus(&buff, 2000.0);
        assert!((result - 0.18).abs() < EPSILON);
    }

    #[test]
    fn test_select_non_moonsign_buff_picks_max() {
        // Pyro 3000 ATK: 3000 * 0.00009 = 0.27
        // Dendro 800 EM: 800 * 0.000225 = 0.18
        let members = vec![(Element::Pyro, 3000.0), (Element::Dendro, 800.0)];
        let result = select_non_moonsign_buff(&members);
        assert!((result - 0.27).abs() < EPSILON);
    }

    #[test]
    fn test_select_non_moonsign_buff_empty() {
        let result = select_non_moonsign_buff(&[]);
        assert!((result - 0.0).abs() < EPSILON);
    }

    // =====================================================================
    // Contribution combination tests
    // =====================================================================

    #[test]
    fn test_lunar_team_single_contribution() {
        let input = lunar_ec_input();
        let solo_result = calculate_lunar(&input, &default_enemy()).unwrap();

        let contributions = vec![LunarContribution {
            input: input.clone(),
        }];
        let team_result = calculate_lunar_team(&contributions, &default_enemy()).unwrap();

        // Single contributor with weight 1.0 — should match solo result exactly
        assert!((team_result.non_crit - solo_result.non_crit).abs() < EPSILON);
        assert!((team_result.crit - solo_result.crit).abs() < EPSILON);
        assert!((team_result.average - solo_result.average).abs() < EPSILON);
        assert_eq!(team_result.damage_element, solo_result.damage_element);
    }

    #[test]
    fn test_lunar_team_two_contributions() {
        // Contributor A: LunarEC, 0.5 crit rate, 1.0 crit dmg
        let input_a = lunar_ec_input();
        // Contributor B: LunarEC, 0.3 crit rate, 0.8 crit dmg — lower average
        let input_b = LunarInput {
            crit_rate: 0.3,
            crit_dmg: 0.8,
            ..lunar_ec_input()
        };

        let result_a = calculate_lunar(&input_a, &default_enemy()).unwrap();
        let result_b = calculate_lunar(&input_b, &default_enemy()).unwrap();

        // Sort: whichever has higher average is weight 1.0, other is weight 0.5
        let (high, low) = if result_a.average >= result_b.average {
            (&result_a, &result_b)
        } else {
            (&result_b, &result_a)
        };

        let expected_non_crit = high.non_crit * 1.0 + low.non_crit * 0.5;
        let expected_crit = high.crit * 1.0 + low.crit * 0.5;
        let expected_average = high.average * 1.0 + low.average * 0.5;

        let contributions = vec![
            LunarContribution { input: input_a },
            LunarContribution { input: input_b },
        ];
        let team_result = calculate_lunar_team(&contributions, &default_enemy()).unwrap();

        assert!((team_result.non_crit - expected_non_crit).abs() < EPSILON);
        assert!((team_result.crit - expected_crit).abs() < EPSILON);
        assert!((team_result.average - expected_average).abs() < EPSILON);
    }

    #[test]
    fn test_lunar_team_two_contributions_reverse_order() {
        let input_a = lunar_ec_input();
        let input_b = LunarInput {
            crit_rate: 0.3,
            crit_dmg: 0.8,
            ..lunar_ec_input()
        };

        let contributions_ab = vec![
            LunarContribution {
                input: input_a.clone(),
            },
            LunarContribution {
                input: input_b.clone(),
            },
        ];
        let contributions_ba = vec![
            LunarContribution { input: input_b },
            LunarContribution { input: input_a },
        ];

        let result_ab = calculate_lunar_team(&contributions_ab, &default_enemy()).unwrap();
        let result_ba = calculate_lunar_team(&contributions_ba, &default_enemy()).unwrap();

        assert!((result_ab.non_crit - result_ba.non_crit).abs() < EPSILON);
        assert!((result_ab.crit - result_ba.crit).abs() < EPSILON);
        assert!((result_ab.average - result_ba.average).abs() < EPSILON);
    }

    #[test]
    fn test_lunar_team_empty_error() {
        let result = calculate_lunar_team(&[], &default_enemy());
        assert!(matches!(result, Err(CalcError::InvalidTeamSize(0))));
    }

    #[test]
    fn test_lunar_team_five_error() {
        let input = lunar_ec_input();
        let contributions = vec![
            LunarContribution {
                input: input.clone(),
            },
            LunarContribution {
                input: input.clone(),
            },
            LunarContribution {
                input: input.clone(),
            },
            LunarContribution {
                input: input.clone(),
            },
            LunarContribution { input },
        ];
        let result = calculate_lunar_team(&contributions, &default_enemy());
        assert!(matches!(result, Err(CalcError::InvalidTeamSize(5))));
    }

    // =====================================================================
    // Enhancement tests
    // =====================================================================

    fn make_bloom_crit_enhancement() -> MoonsignTalentEnhancement {
        MoonsignTalentEnhancement {
            character_name: "TestChar",
            required_level: MoonsignLevel::NascentGleam,
            description: "LunarBloom gets crit",
            effect: MoonsignTalentEffect::GrantReactionCrit {
                reaction: Reaction::LunarBloom,
                crit_rate: 0.20,
                crit_dmg: 0.50,
            },
        }
    }

    #[test]
    fn test_apply_moonsign_enhancement_matching_reaction() {
        let input = lunar_bloom_input();
        let enhancements = vec![make_bloom_crit_enhancement()];
        let result = apply_moonsign_enhancements(&input, &enhancements);
        assert!((result.crit_rate - (input.crit_rate + 0.20)).abs() < EPSILON);
        assert!((result.crit_dmg - (input.crit_dmg + 0.50)).abs() < EPSILON);
    }

    #[test]
    fn test_apply_moonsign_enhancement_non_matching_reaction() {
        // Input is LunarEC, enhancement targets LunarBloom — should not apply
        let input = lunar_ec_input();
        let enhancements = vec![make_bloom_crit_enhancement()];
        let result = apply_moonsign_enhancements(&input, &enhancements);
        assert!((result.crit_rate - input.crit_rate).abs() < EPSILON);
        assert!((result.crit_dmg - input.crit_dmg).abs() < EPSILON);
    }

    #[test]
    fn test_apply_moonsign_enhancement_crit_rate_capped() {
        let input = LunarInput {
            crit_rate: 0.95,
            ..lunar_bloom_input()
        };
        let enhancements = vec![make_bloom_crit_enhancement()]; // adds 0.20 → 1.15, should cap at 1.0
        let result = apply_moonsign_enhancements(&input, &enhancements);
        assert!((result.crit_rate - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_apply_moonsign_enhancement_empty() {
        let input = lunar_ec_input();
        let result = apply_moonsign_enhancements(&input, &[]);
        assert!((result.crit_rate - input.crit_rate).abs() < EPSILON);
        assert!((result.crit_dmg - input.crit_dmg).abs() < EPSILON);
        assert_eq!(result.reaction, input.reaction);
    }

    // =====================================================================
    // resolve_moonsign_context tests
    // =====================================================================

    #[test]
    fn test_resolve_context_no_moonsign() {
        let ctx = resolve_moonsign_context(0, &[], 0.0, vec![]);
        assert_eq!(ctx.level, MoonsignLevel::None);
        assert!(ctx.base_dmg_bonus_by_reaction.is_empty());
        assert!((ctx.non_moonsign_lunar_bonus - 0.0).abs() < EPSILON);
        assert!(ctx.talent_enhancements.is_empty());
    }

    #[test]
    fn test_resolve_context_single_benediction() {
        let bene = MoonsignBenediction {
            base_dmg_bonus: 0.14,
            enabled_reactions: vec![Reaction::LunarElectroCharged],
        };
        let ctx = resolve_moonsign_context(1, &[bene], 0.0, vec![]);
        assert_eq!(ctx.level, MoonsignLevel::NascentGleam);
        assert_eq!(ctx.base_dmg_bonus_by_reaction.len(), 1);
        let bonus = ctx.base_dmg_bonus_for(Reaction::LunarElectroCharged);
        assert!((bonus - 0.14).abs() < EPSILON);
    }

    #[test]
    fn test_resolve_context_stacking_same_reaction() {
        // Ineffa contributes 0.14 for LunarEC, Columbina contributes 0.07 for LunarEC
        let ineffa = MoonsignBenediction {
            base_dmg_bonus: 0.14,
            enabled_reactions: vec![Reaction::LunarElectroCharged],
        };
        let columbina = MoonsignBenediction {
            base_dmg_bonus: 0.07,
            enabled_reactions: vec![Reaction::LunarElectroCharged],
        };
        let ctx = resolve_moonsign_context(2, &[ineffa, columbina], 0.0, vec![]);
        assert_eq!(ctx.level, MoonsignLevel::AscendantGleam);
        let bonus = ctx.base_dmg_bonus_for(Reaction::LunarElectroCharged);
        assert!((bonus - 0.21).abs() < EPSILON);
    }
}
