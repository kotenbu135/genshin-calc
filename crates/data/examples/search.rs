//! Game data search examples.
//!
//! Run: `cargo run -p genshin-calc-data --example search`

use genshin_calc_core::Element;
use genshin_calc_data::types::WeaponType;
use genshin_calc_data::*;

fn main() {
    // Find a character by ID
    let diluc = find_character("diluc").unwrap();
    println!(
        "Character: {} ({:?}, {:?})",
        diluc.name, diluc.element, diluc.weapon_type
    );
    println!("  Base ATK at Lv90: {:.0}", diluc.base_atk[3]);

    // Filter characters by element
    let pyro_chars = characters_by_element(Element::Pyro);
    println!("\nPyro characters ({}):", pyro_chars.len());
    for c in &pyro_chars {
        println!("  - {} ({:?})", c.name, c.rarity);
    }

    // Find a weapon by ID
    let wgs = find_weapon("wolfs_gravestone").unwrap();
    println!(
        "\nWeapon: {} ({:?}, {:?})",
        wgs.name, wgs.weapon_type, wgs.rarity
    );

    // Filter weapons by type
    let swords = weapons_by_type(WeaponType::Sword);
    println!("\nSwords ({}):", swords.len());
    for w in swords.iter().take(5) {
        println!("  - {} ({:?})", w.name, w.rarity);
    }

    // Find an artifact set
    let cw = find_artifact_set("crimson_witch").unwrap();
    println!("\nArtifact: {}", cw.name);
    println!("  2pc: {}", cw.two_piece.description);

    // Find an enemy and convert for calculation
    let enemy_data = find_enemy("hilichurl").unwrap();
    let enemy = enemy_data.to_enemy(Some(Element::Pyro), 90, 0.0);
    println!(
        "\nEnemy: {} (Pyro resistance: {:.0}%)",
        enemy_data.name,
        enemy.resistance * 100.0
    );
}
