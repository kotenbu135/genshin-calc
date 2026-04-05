use genshin_calc_core::{ResolvedBuff, ScalingStat, combine_stats};

use crate::CharacterBuild;

pub fn evaluate_talent_buffs(
    build: &CharacterBuild,
    constellation: u8,
    talent_levels: &[u8; 3],
) -> Vec<ResolvedBuff> {
    let character_id = build.character.id;
    let defs = match genshin_calc_data::talent_buffs::find_talent_buffs(character_id) {
        Some(defs) => defs,
        None => return Vec::new(),
    };

    let profile = crate::build_stat_profile(build);
    let stats = combine_stats(&profile).unwrap_or_default();

    defs.iter()
        .filter(|def| def.min_constellation <= constellation)
        .filter(|def| is_ascension_met(build.level, &def.source))
        .map(|def| {
            let scaling_value = resolve_scaling_value(def, talent_levels);
            let final_value = apply_stat_scaling(def, scaling_value, &profile, &stats);
            ResolvedBuff {
                source: format!("{}:{}", character_id, source_label(&def.source)),
                stat: def.stat,
                value: final_value,
                target: def.target,
            }
        })
        .collect()
}

fn is_ascension_met(
    level: u32,
    source: &genshin_calc_data::talent_buffs::TalentBuffSource,
) -> bool {
    use genshin_calc_data::talent_buffs::TalentBuffSource;
    match source {
        TalentBuffSource::AscensionPassive => level >= 70,
        _ => true,
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
/// This diverges from `ScalingStat::Atk` usage in `damage.rs` where it means total ATK.
/// If a future talent buff needs total ATK scaling, introduce `ScalingStat::BaseAtk` to disambiguate.
fn apply_stat_scaling(
    def: &genshin_calc_data::talent_buffs::TalentBuffDef,
    scaling_value: f64,
    profile: &genshin_calc_core::StatProfile,
    stats: &genshin_calc_core::Stats,
) -> f64 {
    match def.scales_on {
        // base_atk (character + weapon) — see doc comment above
        Some(ScalingStat::Atk) => profile.base_atk * scaling_value,
        Some(ScalingStat::Def) => stats.def * scaling_value,
        Some(ScalingStat::Hp) => stats.hp * scaling_value,
        Some(ScalingStat::Em) => stats.elemental_mastery * scaling_value,
        _ => scaling_value,
    }
}

fn source_label(source: &genshin_calc_data::talent_buffs::TalentBuffSource) -> &'static str {
    use genshin_calc_data::talent_buffs::TalentBuffSource;
    match source {
        TalentBuffSource::AscensionPassive => "a4",
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
        let buffs = evaluate_talent_buffs(&build, 0, &[1, 1, 13]);
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
        let buffs = evaluate_talent_buffs(&build, 6, &[1, 1, 13]);
        assert_eq!(buffs.len(), 2);
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
        assert_eq!(
            buffs[1].stat,
            BuffableStat::ElementalDmgBonus(genshin_calc_core::Element::Pyro)
        );
        assert!((buffs[1].value - 0.15).abs() < 1e-6);
    }

    #[test]
    fn test_unknown_character_returns_empty() {
        let json = r#"{
            "format": "GOOD", "version": 3, "source": "Test",
            "characters": [{"key": "Diluc", "level": 90, "constellation": 0, "ascension": 6, "talent": {"auto": 10, "skill": 10, "burst": 10}}],
            "weapons": [],
            "artifacts": []
        }"#;
        let import = import_good(json).unwrap();
        let build = &import.builds[0];
        let buffs = evaluate_talent_buffs(build, 0, &[10, 10, 10]);
        assert!(buffs.is_empty());
    }

    #[test]
    fn test_constellation_gating() {
        let build = make_bennett_build();
        let buffs = evaluate_talent_buffs(&build, 5, &[1, 1, 13]);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
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
        let buffs = evaluate_talent_buffs(&build, 6, &[1, 10, 1]);
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
        let buffs = evaluate_talent_buffs(&build, 0, &[1, 10, 1]);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
    }
}
