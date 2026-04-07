use genshin_calc_core::{ResolvedBuff, ScalingStat, combine_stats};

use crate::CharacterBuild;
use genshin_calc_data::buff::{Activation, ManualActivation, ManualCondition};

/// Evaluates talent buffs for a character.
///
/// When `talent_activations` is empty, uses max-value policy:
/// - `activation: None` buffs are always included
/// - `activation: Some(Manual(Toggle))` buffs are included at full value
/// - `activation: Some(Manual(Stacks(max)))` buffs are included at max stacks
///
/// When `talent_activations` is provided, uses them to gate conditional buffs:
/// - `activation: None` buffs are always included
/// - `activation: Some(...)` buffs require matching activation entry
pub fn evaluate_talent_buffs(
    build: &CharacterBuild,
    constellation: u8,
    talent_levels: &[u8; 3],
    talent_activations: &[(String, ManualActivation)],
) -> Vec<ResolvedBuff> {
    let character_id = build.character.id;
    let defs = match genshin_calc_data::talent_buffs::find_talent_buffs(character_id) {
        Some(defs) => defs,
        None => return Vec::new(),
    };

    let profile = crate::build_stat_profile(build);
    let stats = combine_stats(&profile).unwrap_or_default();

    let use_max_value = talent_activations.is_empty();

    defs.iter()
        .filter(|def| def.min_constellation <= constellation)
        .filter_map(|def| {
            let scaling_value = resolve_scaling_value(def, talent_levels);
            let base_value = apply_stat_scaling(def, scaling_value, &profile, &stats);

            let final_value = match &def.activation {
                None => base_value,
                // Max-value policy: return full value for all activation types
                Some(
                    Activation::Manual(ManualCondition::Stacks(max))
                    | Activation::Both(_, ManualCondition::Stacks(max)),
                ) if use_max_value => base_value * f64::from(*max),
                Some(_) if use_max_value => base_value,
                // Use provided activations to gate
                Some(Activation::Manual(cond) | Activation::Both(_, cond)) => {
                    eval_talent_activation(cond, def.name, talent_activations, base_value)?
                }
                Some(Activation::Auto(_)) => base_value,
            };

            Some(ResolvedBuff {
                source: format!("{}:{}", character_id, source_label(&def.source)),
                stat: def.stat,
                value: final_value,
                target: def.target,
                origin: None,
            })
        })
        .collect()
}

/// Evaluates a manual condition for talent buffs against provided activations.
fn eval_talent_activation(
    cond: &ManualCondition,
    name: &str,
    activations: &[(String, ManualActivation)],
    computed_value: f64,
) -> Option<f64> {
    let activation = activations.iter().find(|(n, _)| n.as_str() == name);
    match cond {
        ManualCondition::Toggle => match activation {
            Some((_, ManualActivation::Active)) => Some(computed_value),
            _ => None,
        },
        ManualCondition::Stacks(max) => match activation {
            Some((_, ManualActivation::Stacks(n))) => {
                let effective = (*n).min(*max);
                if effective == 0 {
                    return None;
                }
                Some(computed_value * f64::from(effective))
            }
            Some((_, ManualActivation::Active)) => Some(computed_value * f64::from(*max)),
            _ => None,
        },
    }
}

fn resolve_scaling_value(
    def: &genshin_calc_data::talent_buffs::TalentBuffDef,
    talent_levels: &[u8; 3],
) -> f64 {
    if let Some(scaling) = def.talent_scaling {
        let talent_level = match def.source {
            genshin_calc_data::talent_buffs::TalentBuffSource::ElementalSkill => talent_levels[1],
            genshin_calc_data::talent_buffs::TalentBuffSource::ElementalBurst => talent_levels[2],
            // Constellation buffs with talent scaling (e.g. Furina C1 scales with burst level)
            _ if def.scales_with_talent => talent_levels[2],
            _ => return def.base_value,
        };
        let idx = (talent_level as usize).saturating_sub(1).min(14);
        scaling[idx]
    } else {
        def.base_value
    }
}

/// Applies stat scaling (e.g., base_atk × scaling for Bennett).
///
/// NOTE: `ScalingStat::Atk` here intentionally uses `profile.base_atk` (character + weapon base ATK),
/// NOT total ATK from `combine_stats`. This is because all current talent buffs that scale on ATK
/// (Bennett burst, Kujou Sara skill) scale on **base ATK** per game mechanics.
/// Use `ScalingStat::TotalAtk` for buffs that scale on total ATK (e.g. Ineffa A4, Flins A4).
fn apply_stat_scaling(
    def: &genshin_calc_data::talent_buffs::TalentBuffDef,
    scaling_value: f64,
    profile: &genshin_calc_core::StatProfile,
    stats: &genshin_calc_core::Stats,
) -> f64 {
    match def.scales_on {
        // base_atk (character + weapon) — see doc comment above
        Some(ScalingStat::Atk) => {
            let raw = profile.base_atk * scaling_value;
            match def.cap {
                Some(cap) => raw.min(cap),
                None => raw,
            }
        }
        Some(ScalingStat::TotalAtk) => {
            let raw = stats.atk * scaling_value;
            match def.cap {
                Some(cap) => raw.min(cap),
                None => raw,
            }
        }
        Some(ScalingStat::Def) => {
            let raw = stats.def * scaling_value;
            match def.cap {
                Some(cap) => raw.min(cap),
                None => raw,
            }
        }
        Some(ScalingStat::Hp) => {
            let raw = if scaling_value == 0.0 {
                // Nilou A4: floor((total_hp - 30000) / 1000) * 0.09
                ((stats.hp - 30000.0).max(0.0) / 1000.0).floor() * 0.09
            } else {
                stats.hp * scaling_value
            };
            match def.cap {
                Some(cap) => raw.min(cap),
                None => raw,
            }
        }
        Some(ScalingStat::Em) => {
            // Generic EM scaling: em * coefficient (Sucrose A4, Kazuha A4, Nahida A1, Citlali A4, etc.)
            let raw = stats.elemental_mastery * scaling_value;
            match def.cap {
                Some(cap) => raw.min(cap),
                None => raw,
            }
        }
        Some(ScalingStat::CritRate) => {
            let raw = stats.crit_rate * scaling_value;
            match def.cap {
                Some(cap) => raw.min(cap),
                None => raw,
            }
        }
        _ => scaling_value,
    }
}

fn source_label(source: &genshin_calc_data::talent_buffs::TalentBuffSource) -> &'static str {
    use genshin_calc_data::talent_buffs::TalentBuffSource;
    match source {
        TalentBuffSource::AscensionPassive(1) => "a1",
        TalentBuffSource::AscensionPassive(_) => "a4",
        TalentBuffSource::ElementalSkill => "skill",
        TalentBuffSource::ElementalBurst => "burst",
        TalentBuffSource::Constellation(n) => match *n {
            1 => "c1",
            2 => "c2",
            4 => "c4",
            6 => "c6",
            _ => "cx",
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::import_good;
    use genshin_calc_core::{BuffTarget, BuffableStat};

    fn make_bennett_build() -> CharacterBuild {
        let json = r#"{
            "format": "GOOD", "version": 3, "source": "Test",
            "characters": [{"key": "Bennett", "level": 90, "constellation": 6, "ascension": 6, "talent": {"auto": 1, "skill": 1, "burst": 13}}],
            "weapons": [{"key": "AquilaFavonia", "level": 90, "ascension": 6, "refinement": 1, "location": "Bennett", "lock": false}],
            "artifacts": []
        }"#;
        let import = import_good(json).unwrap();
        import.builds.into_iter().next().unwrap()
    }

    #[test]
    fn test_bennett_c0_burst_lv13() {
        let build = make_bennett_build();
        let buffs = evaluate_talent_buffs(&build, 0, &[1, 1, 13], &[]);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
        assert_eq!(buffs[0].target, BuffTarget::Team);
        assert!(
            buffs[0].value > 1000.0,
            "Expected > 1000, got {}",
            buffs[0].value
        );
        assert!(
            buffs[0].value < 1100.0,
            "Expected < 1100, got {}",
            buffs[0].value
        );
    }

    #[test]
    fn test_bennett_c6_burst_lv13() {
        let build = make_bennett_build();
        let buffs = evaluate_talent_buffs(&build, 6, &[1, 1, 13], &[]);
        assert_eq!(buffs.len(), 3);
        // Burst ATK scaling
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
        // C1 Grand Expectation: +20% Base ATK
        assert_eq!(buffs[1].stat, BuffableStat::AtkFlat);
        // C6 Pyro DMG Bonus
        assert_eq!(
            buffs[2].stat,
            BuffableStat::ElementalDmgBonus(genshin_calc_core::Element::Pyro)
        );
        assert!((buffs[2].value - 0.15).abs() < 1e-6);
    }

    #[test]
    fn test_unknown_character_returns_empty() {
        let json = r#"{
            "format": "GOOD", "version": 3, "source": "Test",
            "characters": [{"key": "Tartaglia", "level": 90, "constellation": 0, "ascension": 6, "talent": {"auto": 10, "skill": 10, "burst": 10}}],
            "weapons": [],
            "artifacts": []
        }"#;
        let import = import_good(json).unwrap();
        let build = &import.builds[0];
        let buffs = evaluate_talent_buffs(build, 0, &[10, 10, 10], &[]);
        assert!(buffs.is_empty());
    }

    #[test]
    fn test_constellation_gating() {
        let build = make_bennett_build();
        // C5: burst ATK + C1 Grand Expectation, but no C6
        let buffs = evaluate_talent_buffs(&build, 5, &[1, 1, 13], &[]);
        assert_eq!(buffs.len(), 2);
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
        assert_eq!(buffs[1].stat, BuffableStat::AtkFlat); // C1
    }

    fn make_sara_build() -> CharacterBuild {
        let json = r#"{
            "format": "GOOD", "version": 3, "source": "Test",
            "characters": [{"key": "KujouSara", "level": 90, "constellation": 6, "ascension": 6, "talent": {"auto": 1, "skill": 10, "burst": 1}}],
            "weapons": [{"key": "ElegyForTheEnd", "level": 90, "ascension": 6, "refinement": 1, "location": "KujouSara", "lock": false}],
            "artifacts": []
        }"#;
        let import = import_good(json).unwrap();
        import.builds.into_iter().next().unwrap()
    }

    #[test]
    fn test_sara_c6_buffs() {
        let build = make_sara_build();
        let buffs = evaluate_talent_buffs(&build, 6, &[1, 10, 1], &[]);
        assert_eq!(buffs.len(), 2);
        let atk_buff = buffs
            .iter()
            .find(|b| b.stat == BuffableStat::AtkFlat)
            .unwrap();
        assert!(atk_buff.value > 0.0);
        let crit_buff = buffs
            .iter()
            .find(|b| b.stat == BuffableStat::CritDmg)
            .unwrap();
        assert!((crit_buff.value - 0.60).abs() < 1e-6);
    }

    #[test]
    fn test_sara_c0_no_crit_buff() {
        let build = make_sara_build();
        let buffs = evaluate_talent_buffs(&build, 0, &[1, 10, 1], &[]);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
    }

    fn make_nahida_build(em_value: f64) -> CharacterBuild {
        // Nahida Lv90 with A Thousand Floating Dreams (EM substat)
        // Use artifact substats to control total EM
        let json = format!(
            r#"{{
                "format": "GOOD", "version": 3, "source": "Test",
                "characters": [{{"key": "Nahida", "level": 90, "constellation": 0, "ascension": 6, "talent": {{"auto": 1, "skill": 10, "burst": 10}}}}],
                "weapons": [{{"key": "AThousandFloatingDreams", "level": 90, "ascension": 6, "refinement": 1, "location": "Nahida", "lock": false}}],
                "artifacts": [{{"setKey": "GildedDreams", "slotKey": "sands", "rarity": 5, "level": 20, "mainStatKey": "eleMas", "substats": [{{"key": "eleMas", "value": {em_value}}}], "location": "Nahida", "lock": false}}]
            }}"#
        );
        let import = import_good(&json).unwrap();
        import.builds.into_iter().next().unwrap()
    }

    #[test]
    fn test_nahida_a1_em_buff_formula() {
        let build = make_nahida_build(0.0);
        let profile = crate::build_stat_profile(&build);
        let em = profile.elemental_mastery;
        // Nahida Lv90 base EM = 115.2, ascension EM bonus = 28.8
        // A Thousand Floating Dreams EM = 265, EM sands = 186.5
        // Total EM ≈ 595.5 (no extra substats)
        assert!(em > 500.0, "EM should be > 500, got {em}");

        let buffs = evaluate_talent_buffs(&build, 0, &[1, 10, 10], &[]);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!(
            buffs[0].source.contains("a1"),
            "Source should contain 'a1', got: {}",
            buffs[0].source
        );
        // Expected: em * 0.25, capped at 250
        let expected = (em * 0.25).min(250.0);
        assert!(
            (buffs[0].value - expected).abs() < 1.0,
            "Expected ~{expected}, got {}",
            buffs[0].value
        );
        assert!(buffs[0].value > 0.0, "Nahida A1 buff should not be zero");
    }

    #[test]
    fn test_nahida_a1_low_em_capped() {
        // With low EM, buff should be em * 0.25 (no -200 offset)
        let json = r#"{
            "format": "GOOD", "version": 3, "source": "Test",
            "characters": [{"key": "Nahida", "level": 90, "constellation": 0, "ascension": 6, "talent": {"auto": 1, "skill": 10, "burst": 10}}],
            "weapons": [{"key": "MagicGuide", "level": 1, "ascension": 0, "refinement": 1, "location": "Nahida", "lock": false}],
            "artifacts": []
        }"#;
        let import = import_good(&json).unwrap();
        let build = &import.builds[0];
        let profile = crate::build_stat_profile(build);

        let buffs = evaluate_talent_buffs(build, 0, &[1, 10, 10], &[]);
        assert_eq!(buffs.len(), 1);
        let expected = (profile.elemental_mastery * 0.25).min(250.0);
        assert!(
            (buffs[0].value - expected).abs() < 1.0,
            "Expected ~{expected}, got {}",
            buffs[0].value
        );
    }

    #[test]
    fn test_nahida_a1_low_level_still_returns_buff() {
        // Max-value policy: ascension passives are always returned regardless of level.
        // Low-level characters will have low stat values, producing small but valid buff values.
        let json = r#"{
            "format": "GOOD", "version": 3, "source": "Test",
            "characters": [{"key": "Nahida", "level": 20, "constellation": 0, "ascension": 1, "talent": {"auto": 1, "skill": 1, "burst": 1}}],
            "weapons": [{"key": "MagicGuide", "level": 1, "ascension": 0, "refinement": 1, "location": "Nahida", "lock": false}],
            "artifacts": []
        }"#;
        let import = import_good(&json).unwrap();
        let build = &import.builds[0];
        let buffs = evaluate_talent_buffs(build, 0, &[1, 1, 1], &[]);
        assert_eq!(buffs.len(), 1, "A1 buff should appear even at level 20");
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
    }

    #[test]
    fn test_sucrose_a4_em_sharing() {
        let json = r#"{
            "format": "GOOD", "version": 3, "source": "Test",
            "characters": [{"key": "Sucrose", "level": 90, "constellation": 0, "ascension": 6, "talent": {"auto": 1, "skill": 1, "burst": 1}}],
            "weapons": [{"key": "SacrificialFragments", "level": 90, "ascension": 6, "refinement": 1, "location": "Sucrose", "lock": false}],
            "artifacts": [{"setKey": "ViridescentVenerer", "slotKey": "sands", "rarity": 5, "level": 20, "mainStatKey": "eleMas", "substats": [], "location": "Sucrose", "lock": false}]
        }"#;
        let import = import_good(json).unwrap();
        let build = &import.builds[0];
        let buffs = evaluate_talent_buffs(build, 0, &[1, 1, 1], &[]);
        let a4 = buffs.iter().find(|b| b.source.contains("a4")).unwrap();
        assert_eq!(a4.stat, BuffableStat::ElementalMastery);
        // EM * 0.20 — Sucrose with SacrificialFragments + EM sands should have ~500+ EM
        // so shared value should be ~100+
        assert!(
            a4.value > 80.0,
            "Sucrose A4 should share EM*0.20 (expect >80), got {}",
            a4.value
        );
        assert!(
            a4.value < 300.0,
            "Sucrose A4 should be reasonable, got {}",
            a4.value
        );
    }

    #[test]
    fn test_yelan_a4_returns_max() {
        let json = r#"{
            "format": "GOOD", "version": 3, "source": "Test",
            "characters": [{"key": "Yelan", "level": 90, "constellation": 0, "ascension": 6, "talent": {"auto": 1, "skill": 1, "burst": 1}}],
            "weapons": [{"key": "FavoniusWarbow", "level": 90, "ascension": 6, "refinement": 1, "location": "Yelan", "lock": false}],
            "artifacts": []
        }"#;
        let import = import_good(json).unwrap();
        let build = &import.builds[0];
        let buffs = evaluate_talent_buffs(build, 0, &[1, 1, 1], &[]);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::DmgBonus);
        assert!(
            (buffs[0].value - 0.50).abs() < 1e-6,
            "Yelan A4 should be 0.50, got {}",
            buffs[0].value
        );
    }

    fn make_furina_build() -> CharacterBuild {
        let json = r#"{
            "format": "GOOD", "version": 3, "source": "Test",
            "characters": [{"key": "Furina", "level": 90, "constellation": 6, "ascension": 6, "talent": {"auto": 1, "skill": 1, "burst": 10}}],
            "weapons": [{"key": "SplendorOfTranquilWaters", "level": 90, "ascension": 6, "refinement": 1, "location": "Furina", "lock": false}],
            "artifacts": []
        }"#;
        let import = import_good(json).unwrap();
        import.builds.into_iter().next().unwrap()
    }

    #[test]
    fn test_furina_c0_burst_lv10() {
        let build = make_furina_build();
        let buffs = evaluate_talent_buffs(&build, 0, &[1, 1, 10], &[]);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::DmgBonus);
        assert!(
            (buffs[0].value - 0.75).abs() < 1e-6,
            "C0 Burst Lv10: 300 × 0.0025 = 0.75, got {}",
            buffs[0].value
        );
    }

    #[test]
    fn test_furina_c1_burst_lv10() {
        let build = make_furina_build();
        let buffs = evaluate_talent_buffs(&build, 1, &[1, 1, 10], &[]);
        assert_eq!(
            buffs.len(),
            2,
            "C1 should return 2 buffs (C0 base + C1 extra)"
        );
        let total: f64 = buffs
            .iter()
            .filter(|b| b.stat == BuffableStat::DmgBonus)
            .map(|b| b.value)
            .sum();
        assert!(
            (total - 1.00).abs() < 1e-6,
            "C1 Burst Lv10: 400 × 0.0025 = 1.00, got {}",
            total
        );
    }

    #[test]
    fn test_furina_c1_burst_lv13() {
        let build = make_furina_build();
        let buffs = evaluate_talent_buffs(&build, 1, &[1, 1, 13], &[]);
        let total: f64 = buffs
            .iter()
            .filter(|b| b.stat == BuffableStat::DmgBonus)
            .map(|b| b.value)
            .sum();
        assert!(
            (total - 1.24).abs() < 1e-6,
            "C1 Burst Lv13: 400 × 0.0031 = 1.24, got {}",
            total
        );
    }

    #[test]
    fn test_furina_c1_toggle_active_with_stacks() {
        // C1 Toggle ON + C0 Stacks 300 → full 400pt equivalent
        let build = make_furina_build();
        let activations = vec![
            (
                "Let the People Rejoice DMG Bonus (C0 300pt)".to_string(),
                ManualActivation::Stacks(300),
            ),
            (
                "Let the People Rejoice DMG Bonus (C1+ extra 100pt)".to_string(),
                ManualActivation::Active,
            ),
        ];
        let buffs = evaluate_talent_buffs(&build, 1, &[1, 1, 10], &activations);
        let total: f64 = buffs.iter().map(|b| b.value).sum();
        assert!(
            (total - 1.00).abs() < 1e-6,
            "C1 active + 300 stacks at Lv10: 300×0.0025 + 100×0.0025 = 1.00, got {}",
            total
        );
    }

    #[test]
    fn test_furina_c1_toggle_inactive_only_stacks() {
        // C1 Toggle not activated → only C0 stacks count
        let build = make_furina_build();
        let activations = vec![(
            "Let the People Rejoice DMG Bonus (C0 300pt)".to_string(),
            ManualActivation::Stacks(300),
        )];
        let buffs = evaluate_talent_buffs(&build, 1, &[1, 1, 10], &activations);
        // C1 Toggle not in activations → filtered out, only C0 buff remains
        assert_eq!(buffs.len(), 1, "Only C0 buff should be present");
        assert!(
            (buffs[0].value - 0.75).abs() < 1e-6,
            "C0 only: 300 × 0.0025 = 0.75, got {}",
            buffs[0].value
        );
    }
}
