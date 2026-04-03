use std::collections::HashMap;

use genshin_calc_core::StatProfile;
use genshin_calc_data::buff::ManualActivation;
use genshin_calc_data::team_builder::TeamMemberBuilder;
use genshin_calc_data::types::{ArtifactSet, CharacterData, WeaponData};

use crate::error::ImportWarning;
use crate::key_map;
use crate::stat_map;
use crate::types::{GoodArtifact, GoodFormat};
use crate::{ArtifactsBuild, CharacterBuild, GoodImport, WeaponBuild};

/// CharacterBuild と ManualActivation リストから TeamMemberBuilder を構築。
/// weapon が None の場合は Err を返す（ステータス計算には武器が必須）。
pub fn to_team_member_builder(
    build: &crate::CharacterBuild,
    weapon_activations: &[(&str, ManualActivation)],
    artifact_activations: &[(&str, ManualActivation)],
) -> Result<TeamMemberBuilder, crate::GoodError> {
    let weapon_build = build
        .weapon
        .as_ref()
        .ok_or(crate::GoodError::MissingWeapon)?;

    let mut builder = TeamMemberBuilder::new(build.character, weapon_build.weapon)
        .weapon_level(weapon_build.level)
        .constellation(build.constellation)
        .talent_levels(build.talent_levels)
        .refinement(weapon_build.refinement);

    // 聖遺物ステータス（artifact main/substats のみ）
    builder = builder.artifact_stats(build.artifacts.stats.clone());

    // 聖遺物セット効果（2pc/4pc全部）
    if !build.artifacts.sets.is_empty() {
        builder = builder.artifact_sets(build.artifacts.sets.clone());
    }

    // ManualActivation を登録
    for (name, activation) in weapon_activations.iter().chain(artifact_activations.iter()) {
        builder = match activation {
            ManualActivation::Active => builder.activate(name),
            ManualActivation::Stacks(n) => builder.activate_with_stacks(name, *n),
        };
    }

    Ok(builder)
}

pub(crate) fn build_imports(good: GoodFormat) -> GoodImport {
    let mut warnings = Vec::new();

    let weapons = index_weapons(&good, &mut warnings);
    let artifacts = group_artifacts(&good);

    let mut builds = Vec::new();

    if let Some(chars) = &good.characters {
        for gc in chars {
            let character = match key_map::lookup_character(&gc.key) {
                Some(c) => c,
                None => {
                    warnings.push(ImportWarning::UnknownCharacter(gc.key.clone()));
                    continue;
                }
            };

            let weapon = weapons.get(gc.key.as_str()).copied();
            let arts = artifacts.get(gc.key.as_str());
            let artifacts_build = build_artifacts(
                arts.map(|a| a.as_slice()).unwrap_or(&[]),
                character,
                &mut warnings,
            );

            builds.push(CharacterBuild {
                character,
                level: gc.level,
                constellation: gc.constellation,
                talent_levels: [gc.talent.auto, gc.talent.skill, gc.talent.burst],
                weapon: weapon.map(|(w, level, refinement)| WeaponBuild {
                    weapon: w,
                    level,
                    refinement,
                }),
                artifacts: artifacts_build,
            });
        }
    }

    GoodImport {
        source: good.source,
        version: good.version,
        builds,
        warnings,
    }
}

fn index_weapons<'a>(
    good: &'a GoodFormat,
    warnings: &mut Vec<ImportWarning>,
) -> HashMap<&'a str, (&'static WeaponData, u32, u8)> {
    let mut map = HashMap::new();
    if let Some(weapons) = &good.weapons {
        for gw in weapons {
            let location = match &gw.location {
                Some(loc) if !loc.is_empty() => loc.as_str(),
                _ => continue,
            };
            let weapon = match key_map::lookup_weapon(&gw.key) {
                Some(w) => w,
                None => {
                    warnings.push(ImportWarning::UnknownWeapon(gw.key.clone()));
                    continue;
                }
            };
            map.insert(location, (weapon, gw.level, gw.refinement));
        }
    }
    map
}

fn group_artifacts(good: &GoodFormat) -> HashMap<&str, Vec<&GoodArtifact>> {
    let mut map: HashMap<&str, Vec<&GoodArtifact>> = HashMap::new();
    if let Some(artifacts) = &good.artifacts {
        for ga in artifacts {
            let location = match &ga.location {
                Some(loc) if !loc.is_empty() => loc.as_str(),
                _ => continue,
            };
            map.entry(location).or_default().push(ga);
        }
    }
    map
}

fn build_artifacts(
    artifacts: &[&GoodArtifact],
    _character: &'static CharacterData,
    warnings: &mut Vec<ImportWarning>,
) -> ArtifactsBuild {
    let mut stats = StatProfile::default();
    // Use set ID as key since ArtifactSet doesn't impl Hash/Eq
    let mut set_counts: HashMap<&'static str, (&'static ArtifactSet, u8)> = HashMap::new();

    for art in artifacts {
        // Resolve and count sets
        if let Some(set) = key_map::lookup_artifact_set(&art.set_key) {
            let entry = set_counts.entry(set.id).or_insert((set, 0));
            entry.1 += 1;
        } else {
            warnings.push(ImportWarning::UnknownArtifactSet(art.set_key.clone()));
        }

        let level = art.level;

        // Add main stat
        let rarity = match art.rarity {
            5 => genshin_calc_data::types::ArtifactRarity::Star5,
            4 => genshin_calc_data::types::ArtifactRarity::Star4,
            _ => {
                continue;
            }
        };
        let slot = match art.slot_key.as_str() {
            "flower" => genshin_calc_data::ArtifactSlot::Flower,
            "plume" => genshin_calc_data::ArtifactSlot::Plume,
            "sands" => genshin_calc_data::ArtifactSlot::Sands,
            "goblet" => genshin_calc_data::ArtifactSlot::Goblet,
            "circlet" => genshin_calc_data::ArtifactSlot::Circlet,
            _ => continue,
        };
        if let Some(main_value) = genshin_calc_data::artifact_main_stat_value_by_key(
            rarity,
            slot,
            &art.main_stat_key,
            level,
        ) {
            if let stat_map::StatConvertResult::Converted(field, converted) =
                stat_map::convert_stat(&art.main_stat_key, main_value)
            {
                stat_map::add_to_profile(&mut stats, &field, converted);
            }
        }

        // Add substats
        for sub in &art.substats {
            if sub.key.is_empty() {
                continue;
            }
            match stat_map::convert_stat(&sub.key, sub.value) {
                stat_map::StatConvertResult::Converted(field, converted) => {
                    stat_map::add_to_profile(&mut stats, &field, converted);
                }
                stat_map::StatConvertResult::Skip => {}
                stat_map::StatConvertResult::Unknown => {
                    warnings.push(ImportWarning::UnknownStatKey(sub.key.clone()));
                }
            }
        }
    }

    let sets = detect_sets(&set_counts);

    ArtifactsBuild { sets, stats }
}

fn detect_sets(
    counts: &HashMap<&'static str, (&'static ArtifactSet, u8)>,
) -> Vec<&'static ArtifactSet> {
    let mut four_piece = None;
    let mut two_pieces = Vec::new();

    for &(set, count) in counts.values() {
        if count >= 4 {
            four_piece = Some(set);
        } else if count >= 2 {
            two_pieces.push(set);
        }
    }

    if let Some(fp) = four_piece {
        vec![fp]
    } else {
        two_pieces
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genshin_calc_data::buff::ManualActivation;

    fn make_build(
        character: &'static CharacterData,
        weapon: Option<&'static WeaponData>,
        artifact_set: Option<&'static ArtifactSet>,
    ) -> crate::CharacterBuild {
        let sets: Vec<&'static ArtifactSet> = artifact_set.iter().copied().collect();
        crate::CharacterBuild {
            character,
            level: 90,
            constellation: 0,
            talent_levels: [10, 8, 9],
            weapon: weapon.map(|w| crate::WeaponBuild {
                weapon: w,
                level: 90,
                refinement: 1,
            }),
            artifacts: crate::ArtifactsBuild {
                sets,
                stats: StatProfile::default(),
            },
        }
    }

    #[test]
    fn test_basic_conversion() {
        let char = genshin_calc_data::find_character("diluc").unwrap();
        let weapon = genshin_calc_data::find_weapon("wolfs_gravestone").unwrap();
        let build = make_build(char, Some(weapon), None);
        let result = to_team_member_builder(&build, &[], &[]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_missing_weapon_error() {
        let char = genshin_calc_data::find_character("diluc").unwrap();
        let build = make_build(char, None, None);
        let result = to_team_member_builder(&build, &[], &[]);
        match result {
            Err(crate::GoodError::MissingWeapon) => {
                // Expected error
            }
            _ => {
                panic!("expected MissingWeapon error");
            }
        }
    }

    #[test]
    fn test_with_artifact_set_2pc_buff() {
        let char = genshin_calc_data::find_character("diluc").unwrap();
        let weapon = genshin_calc_data::find_weapon("wolfs_gravestone").unwrap();
        let cw = genshin_calc_data::find_artifact_set("crimson_witch").unwrap();
        let build = make_build(char, Some(weapon), Some(cw));
        let result = to_team_member_builder(&build, &[], &[]);
        assert!(result.is_ok());
        let member = result.unwrap().build().unwrap();
        // CW 2pc: ElementalDmgBonus(Pyro) +15% — source: "燃え盛る炎の魔女 2pc"
        let has_2pc = member
            .buffs_provided
            .iter()
            .any(|b| b.source.contains("2pc"));
        assert!(has_2pc, "should have 2pc static buff");
    }

    #[test]
    fn test_with_artifact_activation_stacks() {
        let char = genshin_calc_data::find_character("diluc").unwrap();
        let weapon = genshin_calc_data::find_weapon("wolfs_gravestone").unwrap();
        let cw = genshin_calc_data::find_artifact_set("crimson_witch").unwrap();
        let build = make_build(char, Some(weapon), Some(cw));

        // CW 4pc: cwof_pyro_stacks = ElementalDmgBonus(Pyro) +0.075/stack, max 3
        let activations = [("cwof_pyro_stacks", ManualActivation::Stacks(2))];
        let result = to_team_member_builder(&build, &[], &activations);
        assert!(result.is_ok());
        let member = result.unwrap().build().unwrap();
        // source format: "cwof_pyro_stacks (燃え盛る炎の魔女 4pc)"
        let stack_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("cwof_pyro_stacks"));
        assert!(stack_buff.is_some(), "should have CW 4pc stack buff");
        let buff = stack_buff.unwrap();
        // 2 stacks × 0.075 = 0.15
        assert!(
            (buff.value - 0.15).abs() < 1e-10,
            "2 stacks should give 0.15, got {}",
            buff.value
        );
    }
}
