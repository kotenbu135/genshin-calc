//! Demo: Import GOOD format and calculate damage.
//!
//! Run: `cargo run -p genshin-calc-good --example demo`

use genshin_calc_core::*;
use genshin_calc_good::import_good;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string("genshin_export_2026-04-03_22-17.json")?;
    let import = import_good(&json)?;

    println!("# Character Status Report\n");
    println!("Imported: {} builds\n", import.builds.len());

    for build in &import.builds {
        let weapon = match build.weapon.as_ref() {
            Some(w) => w,
            None => {
                println!(
                    "## {} (Lv{}) - No weapon\n",
                    build.character.name, build.level
                );
                continue;
            }
        };

        let level = build.level.min(100);
        let weapon_level = weapon.level.min(90);

        let member = match genshin_calc_data::TeamMemberBuilder::new(build.character, weapon.weapon)
            .character_level(level)
            .constellation(build.constellation)
            .talent_levels(build.talent_levels)
            .weapon_level(weapon_level)
            .artifact_sets(build.artifacts.sets.clone())
            .artifact_stats(build.artifacts.stats.clone())
            .build()
        {
            Ok(m) => m,
            Err(e) => {
                println!(
                    "## {} (Lv{}) - Build error: {:?}\n",
                    build.character.name, level, e
                );
                continue;
            }
        };

        let stats = combine_stats(&member.stats).map_err(|e| {
            eprintln!(
                "Error combining stats for {}: {:?}",
                build.character.name, e
            );
            std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", e))
        })?;

        println!("## {} (Lv{})\n", build.character.name, level);
        println!("| Stat | Value |");
        println!("|------|-------|");
        println!("| ATK | {:.0} |", stats.atk);
        println!("| HP | {:.0} |", stats.hp);
        println!("| DEF | {:.0} |", stats.def);
        println!("| EM | {:.0} |", stats.elemental_mastery);
        println!("| CRIT Rate | {:.1}% |", stats.crit_rate * 100.0);
        println!("| CRIT DMG | {:.1}% |", stats.crit_dmg * 100.0);
        println!("| ER | {:.1}% |", stats.energy_recharge * 100.0);

        if stats.dmg_bonus > 0.0 {
            println!("| Dmg Bonus | +{:.1}% |", stats.dmg_bonus * 100.0);
        }

        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };

        let normal_attack = &build.character.talents.normal_attack;
        let talent_multiplier = normal_attack
            .hits
            .first()
            .map(|h| h.values[0])
            .unwrap_or(1.76)
            / 100.0;

        let damage = calculate_damage(
            &DamageInput {
                character_level: level,
                stats: stats.clone(),
                talent_multiplier,
                scaling_stat: ScalingStat::Atk,
                damage_type: DamageType::Normal,
                element: Some(build.character.element),
                reaction: None,
                reaction_bonus: 0.0,
                flat_dmg: 0.0,
            },
            &enemy,
        )?;

        println!("\n### Normal Attack (Lv1)\n");
        println!("| Type | Value |");
        println!("|------|-------|");
        println!("| non-crit | {:.0} |", damage.non_crit);
        println!("| crit | {:.0} |", damage.crit);
        println!("| average | {:.0} |", damage.average);
        println!();
    }

    Ok(())
}
