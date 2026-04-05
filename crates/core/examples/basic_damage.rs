//! Basic damage calculation examples.
//!
//! Run: `cargo run -p genshin-calc-core --example basic_damage`

use genshin_calc_core::*;

fn main() {
    let stats = Stats {
        hp: 20000.0,
        atk: 2000.0,
        def: 800.0,
        elemental_mastery: 100.0,
        crit_rate: 0.75,
        crit_dmg: 1.50,
        energy_recharge: 1.20,
        dmg_bonus: 0.466,
        ..Default::default()
    };

    let enemy = Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
        def_ignore: 0.0,
    };

    // Physical damage (no element)
    let physical = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: stats.clone(),
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: None,
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &enemy,
    )
    .unwrap();
    println!("Physical Normal:");
    println!("  non-crit: {:.1}", physical.non_crit);
    println!("  crit:     {:.1}", physical.crit);
    println!("  average:  {:.1}", physical.average);

    // Pyro skill damage
    let pyro = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: stats.clone(),
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &enemy,
    )
    .unwrap();
    println!("\nPyro Skill:");
    println!("  non-crit: {:.1}", pyro.non_crit);
    println!("  crit:     {:.1}", pyro.crit);
    println!("  average:  {:.1}", pyro.average);

    // Vaporize (Pyro trigger = 1.5x)
    let vaporize = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats,
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: Some(Reaction::Vaporize),
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &enemy,
    )
    .unwrap();
    println!("\nVaporize (Pyro 1.5x):");
    println!("  non-crit: {:.1}", vaporize.non_crit);
    println!("  crit:     {:.1}", vaporize.crit);
    println!("  average:  {:.1}", vaporize.average);
}
