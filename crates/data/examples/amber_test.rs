//! Test Amber stats calculation at various levels.

use genshin_calc_core::combine_stats;
use genshin_calc_data::{find_character, find_weapon, team_builder::TeamMemberBuilder};

fn main() {
    let amber = find_character("amber").expect("Amber not found");
    let rust = find_weapon("rust").expect("Rust not found");

    println!("=== Amber Stats Test ===");
    println!("Character: {} (Element: {:?})", amber.name, amber.element);
    println!("Weapon: {} (ATK: {:?})", rust.name, rust.base_atk);
    println!("Constellation Pattern: {:?}", amber.constellation_pattern);
    println!();

    // Check base stats at different levels
    println!("--- Base Stats by Level (Lv50=突破後, Lv49=突破前) ---");
    println!("{:<8} {:>12} {:>12} {:>12}", "Level", "HP", "ATK", "DEF");
    for lvl in [1, 20, 40, 49, 50, 60, 70, 80, 90, 95, 100] {
        println!(
            "Lv{:<6} {:>12.2} {:>12.2} {:>12.2}",
            lvl,
            amber.base_hp_at_level(lvl),
            amber.base_atk_at_level(lvl),
            amber.base_def_at_level(lvl)
        );
    }
    println!();

    // Build with TeamMemberBuilder at Lv50+
    println!("=== TeamMember at Lv50+ (A3後) with Rust (Lv50), C2 ===");
    let member = TeamMemberBuilder::new(amber, rust)
        .character_level(50) // Lv50 = 突破後(A3後)の値5577を返す
        .weapon_level(50)
        .constellation(2)
        .build()
        .expect("Failed to build");

    let final_stats = combine_stats(&member.stats).expect("Failed to combine stats");

    println!("Profile Stats:");
    println!("  base_hp: {:.2}", member.stats.base_hp);
    println!(
        "  base_atk: {:.2} (char: {:.2} + weapon: {:.2})",
        member.stats.base_atk,
        member.stats.base_atk - rust.base_atk[1],
        rust.base_atk[1]
    );
    println!("  base_def: {:.2}", member.stats.base_def);
    println!();
    println!("Final Stats (with bonuses applied):");
    println!("  HP: {:.2}", final_stats.hp);
    println!("  ATK: {:.2}", final_stats.atk);
    println!("  DEF: {:.2}", final_stats.def);
    println!("  Elemental Mastery: {:.0}", final_stats.elemental_mastery);
    println!("  Crit Rate: {:.1}%", final_stats.crit_rate * 100.0);
    println!("  Crit DMG: {:.1}%", final_stats.crit_dmg * 100.0);
    println!(
        "  Energy Recharge: {:.1}%",
        final_stats.energy_recharge * 100.0
    );
}
