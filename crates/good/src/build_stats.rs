use crate::CharacterBuild;
use genshin_calc_core::StatProfile;

/// Converts a CharacterBuild into a StatProfile.
///
/// Uses character level for base stats, weapon stats, and ascension stats.
/// Merges artifact stats and applies base defaults (CR 5%, CD 50%, ER 100%).
pub fn build_stat_profile(build: &CharacterBuild) -> StatProfile {
    use genshin_calc_data::team_builder::{apply_ascension_stat, apply_weapon_sub_stat};

    let mut profile = StatProfile {
        base_hp: build.character.base_hp_at_level(build.level),
        base_atk: build.character.base_atk_at_level(build.level),
        base_def: build.character.base_def_at_level(build.level),
        crit_rate: 0.05,
        crit_dmg: 0.50,
        energy_recharge: 1.0,
        ..Default::default()
    };

    // Weapon base ATK
    if let Some(ref wb) = build.weapon {
        profile.base_atk += wb.weapon.base_atk[3];
        // Weapon sub-stat
        if let Some(ref sub) = wb.weapon.sub_stat {
            apply_weapon_sub_stat(&mut profile, sub);
        }
    }

    // Ascension stat
    apply_ascension_stat(&mut profile, &build.character.ascension_stat);

    // Artifact stats merge
    profile.hp_percent += build.artifacts.stats.hp_percent;
    profile.atk_percent += build.artifacts.stats.atk_percent;
    profile.def_percent += build.artifacts.stats.def_percent;
    profile.hp_flat += build.artifacts.stats.hp_flat;
    profile.atk_flat += build.artifacts.stats.atk_flat;
    profile.def_flat += build.artifacts.stats.def_flat;
    profile.elemental_mastery += build.artifacts.stats.elemental_mastery;
    profile.crit_rate += build.artifacts.stats.crit_rate;
    profile.crit_dmg += build.artifacts.stats.crit_dmg;
    profile.energy_recharge += build.artifacts.stats.energy_recharge;
    profile.dmg_bonus += build.artifacts.stats.dmg_bonus;
    profile.pyro_dmg_bonus += build.artifacts.stats.pyro_dmg_bonus;
    profile.hydro_dmg_bonus += build.artifacts.stats.hydro_dmg_bonus;
    profile.electro_dmg_bonus += build.artifacts.stats.electro_dmg_bonus;
    profile.cryo_dmg_bonus += build.artifacts.stats.cryo_dmg_bonus;
    profile.dendro_dmg_bonus += build.artifacts.stats.dendro_dmg_bonus;
    profile.anemo_dmg_bonus += build.artifacts.stats.anemo_dmg_bonus;
    profile.geo_dmg_bonus += build.artifacts.stats.geo_dmg_bonus;
    profile.physical_dmg_bonus += build.artifacts.stats.physical_dmg_bonus;

    profile
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_stats_from_character() {
        let character = genshin_calc_data::find_character("hu_tao").expect("Hu Tao should exist");
        let build = CharacterBuild {
            character,
            level: 90,
            ascension: 6,
            constellation: 0,
            talent_levels: [10, 10, 10],
            weapon: None,
            artifacts: crate::ArtifactsBuild {
                sets: vec![],
                four_piece_set: None,
                stats: StatProfile::default(),
            },
        };
        let profile = build_stat_profile(&build);
        // Hu Tao Lv90 base HP = 15552.0 (index 3)
        assert!(profile.base_hp > 15000.0);
        // Base defaults (crit_rate and energy_recharge are unaffected by Hu Tao ascension stat)
        assert!((profile.crit_rate - 0.05).abs() < 1e-10);
        assert!((profile.energy_recharge - 1.0).abs() < 1e-10);
        // Hu Tao ascension stat: CritDmg(0.384) — final crit_dmg = 0.50 base + 0.384 ascension
        assert!(profile.crit_dmg > 0.50);
    }
}
