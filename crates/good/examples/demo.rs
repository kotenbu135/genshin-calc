//! Demo: Import GOOD format and calculate damage.
//!
//! Run: `cargo run -p genshin-calc-good --example demo`

use genshin_calc_core::*;
use genshin_calc_good::import_good;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string("genshin_export_2026-03-31_13-17.json")?;
    let import = import_good(&json)?;

    println!("Imported {} builds\n", import.builds.len());

    for build in import.builds.iter().take(5) {
        let weapon = match build.weapon.as_ref() {
            Some(w) => w,
            None => {
                println!(
                    "=== {} (Lv{}) - No weapon, skipping ===\n",
                    build.character.name, build.level
                );
                continue;
            }
        };

        let member = match genshin_calc_data::TeamMemberBuilder::new(build.character, weapon.weapon)
            .character_level(build.level)
            .constellation(build.constellation)
            .talent_levels(build.talent_levels)
            .weapon_level(weapon.level)
            .artifact_sets(build.artifacts.sets.iter().copied().collect())
            .artifact_stats(build.artifacts.stats.clone())
            .build()
        {
            Ok(m) => m,
            Err(e) => {
                println!(
                    "=== {} (Lv{}) - Build error: {:?} ===\n",
                    build.character.name, build.level, e
                );
                continue;
            }
        };

        let stats = combine_stats(&member.stats)?;

        println!("=== {} (Lv{}) ===", build.character.name, build.level);
        println!("ATK: {:.0}", stats.atk);
        println!("HP: {:.0}", stats.hp);
        println!("DEF: {:.0}", stats.def);
        println!("EM: {:.0}", stats.elemental_mastery);
        println!(
            "CRIT: {:.1}% / {:.1}%",
            stats.crit_rate * 100.0,
            stats.crit_dmg * 100.0
        );
        println!("ER: {:.1}%", stats.energy_recharge * 100.0);

        if stats.dmg_bonus > 0.0 {
            println!("Dmg Bonus: +{:.1}%", stats.dmg_bonus * 100.0);
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
                character_level: build.level,
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

        println!("Normal Attack (Lv1):");
        println!("  non-crit: {:.0}", damage.non_crit);
        println!("  crit:     {:.0}", damage.crit);
        println!("  average:  {:.0}", damage.average);
        println!();
    }

    Ok(())
}
