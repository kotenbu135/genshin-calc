//! Transformative and lunar reaction examples.
//!
//! Run: `cargo run -p genshin-calc-core --example reactions`

use genshin_calc_core::*;

fn main() {
    let enemy = Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
    };

    // Overloaded (transformative)
    let overloaded = calculate_transformative(
        &TransformativeInput {
            character_level: 90,
            elemental_mastery: 200.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
        },
        &enemy,
    )
    .unwrap();
    println!("Overloaded (EM 200):");
    println!("  damage:  {:.1}", overloaded.damage);
    println!("  element: {:?}", overloaded.damage_element);

    // Superconduct (transformative)
    let superconduct = calculate_transformative(
        &TransformativeInput {
            character_level: 90,
            elemental_mastery: 0.0,
            reaction: Reaction::Superconduct,
            reaction_bonus: 0.0,
        },
        &enemy,
    )
    .unwrap();
    println!("\nSuperconduct (EM 0):");
    println!("  damage:  {:.1}", superconduct.damage);

    // Lunar Electro-Charged (lunar — can crit)
    let lunar_ec = calculate_lunar(
        &LunarInput {
            character_level: 90,
            elemental_mastery: 300.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 0.60,
            crit_dmg: 1.20,
            base_dmg_bonus: 0.0,
        },
        &enemy,
    )
    .unwrap();
    println!("\nLunar Electro-Charged (EM 300, CR 60%, CD 120%):");
    println!("  non-crit: {:.1}", lunar_ec.non_crit);
    println!("  crit:     {:.1}", lunar_ec.crit);
    println!("  average:  {:.1}", lunar_ec.average);
}
