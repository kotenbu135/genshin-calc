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
        .constellation(build.constellation)
        .talent_levels(build.talent_levels)
        .refinement(weapon_build.refinement);

    // 聖遺物ステータス（2pc無条件バフはimport時にstatsにマージ済み）
    builder = builder.artifact_stats(build.artifacts.stats.clone());

    // 4セット効果
    if let Some(set) = build.artifacts.four_piece_set {
        builder = builder.artifact_set(set);
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
                ascension: gc.ascension,
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
    character: &'static CharacterData,
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

        // Add main stat
        if let Some(main_value) =
            stat_map::main_stat_value(art.rarity, &art.main_stat_key, art.level)
        {
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

    let (sets, four_piece_set) = detect_sets(&set_counts);

    // For 2pc/2pc+2pc, pre-merge 2pc buffs into stats
    if four_piece_set.is_none() {
        for set in &sets {
            apply_two_piece_buffs(set, character, &mut stats);
        }
    }

    ArtifactsBuild {
        sets,
        four_piece_set,
        stats,
    }
}

fn detect_sets(
    counts: &HashMap<&'static str, (&'static ArtifactSet, u8)>,
) -> (Vec<&'static ArtifactSet>, Option<&'static ArtifactSet>) {
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
        (vec![fp], Some(fp))
    } else {
        (two_pieces, None)
    }
}

fn apply_two_piece_buffs(
    set: &'static ArtifactSet,
    _character: &'static CharacterData,
    stats: &mut StatProfile,
) {
    use genshin_calc_core::{BuffableStat, Element};

    for buff in set.two_piece.buffs {
        match buff.stat {
            BuffableStat::HpPercent => stats.hp_percent += buff.value,
            BuffableStat::AtkPercent => stats.atk_percent += buff.value,
            BuffableStat::DefPercent => stats.def_percent += buff.value,
            BuffableStat::HpFlat => stats.hp_flat += buff.value,
            BuffableStat::AtkFlat => stats.atk_flat += buff.value,
            BuffableStat::DefFlat => stats.def_flat += buff.value,
            BuffableStat::ElementalMastery => stats.elemental_mastery += buff.value,
            BuffableStat::EnergyRecharge => stats.energy_recharge += buff.value,
            BuffableStat::CritRate => stats.crit_rate += buff.value,
            BuffableStat::CritDmg => stats.crit_dmg += buff.value,
            BuffableStat::DmgBonus => stats.dmg_bonus += buff.value,
            BuffableStat::ElementalDmgBonus(elem) => match elem {
                Element::Pyro => stats.pyro_dmg_bonus += buff.value,
                Element::Hydro => stats.hydro_dmg_bonus += buff.value,
                Element::Electro => stats.electro_dmg_bonus += buff.value,
                Element::Cryo => stats.cryo_dmg_bonus += buff.value,
                Element::Dendro => stats.dendro_dmg_bonus += buff.value,
                Element::Anemo => stats.anemo_dmg_bonus += buff.value,
                Element::Geo => stats.geo_dmg_bonus += buff.value,
            },
            BuffableStat::PhysicalDmgBonus => stats.physical_dmg_bonus += buff.value,
            _ => {}
        }
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
        crate::CharacterBuild {
            character,
            level: 90,
            ascension: 6,
            constellation: 0,
            talent_levels: [10, 8, 9],
            weapon: weapon.map(|w| crate::WeaponBuild {
                weapon: w,
                level: 90,
                refinement: 1,
            }),
            artifacts: crate::ArtifactsBuild {
                sets: vec![],
                four_piece_set: artifact_set,
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
