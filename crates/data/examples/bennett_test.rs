//! Test Bennett stats calculation with specific artifacts and weapon.

use genshin_calc_core::{BuffableStat, Element, StatProfile, combine_stats};
use genshin_calc_data::{
    ArtifactSlot, artifact_main_stat_value, find_artifact_set, find_character, find_weapon,
    team_builder::TeamMemberBuilder, types::ArtifactRarity,
};

fn main() {
    let bennett = find_character("bennett").expect("Bennett not found");
    let aquila = find_weapon("aquila_favonia").expect("Aquila Favonia not found");
    let blizzard_strayer =
        find_artifact_set("blizzard_strayer").expect("Blizzard Strayer not found");

    println!("=== Bennett Stats Test ===");
    println!(
        "Character: {} (Element: {:?})",
        bennett.name, bennett.element
    );
    println!("Weapon: {} (R2)", aquila.name);
    println!();

    // Base stats at Lv90
    println!("--- Base Stats at Lv90 ---");
    println!("  base_hp: {:.2}", bennett.base_hp_at_level(90));
    println!("  base_atk: {:.2}", bennett.base_atk_at_level(90));
    println!("  base_def: {:.2}", bennett.base_def_at_level(90));
    println!();

    // Aquila Favonia R2
    // base_atk: [48.0, 590.0, 621.0, 674.0] -> R2 = (590+621)/2 = 605.5
    // sub_stat (Physical DMG): [0.090, 0.377, 0.377, 0.413] -> Lv90 = 0.377
    // passive (ATK%): [0.20, 0.25, 0.30, 0.35, 0.40] -> R2 = 0.25
    let aquila_atk_r2 = (590.0 + 621.0) / 2.0;
    println!("--- Aquila Favonia R2 ---");
    println!("  Base ATK: {:.1}", aquila_atk_r2);
    println!("  Physical DMG Bonus: {:.1}%", 0.377 * 100.0);
    println!("  Passive ATK%: {:.1}%", 0.25 * 100.0);
    println!();

    // Artifacts (5-star +20) - Using artifact_main_stat_value function
    // These are ONLY the artifact bonuses, NOT including base values (5% CR, 50% CD, 100% ER)
    // Circlet (Blizzard): critDMG 62.4% main
    // Plume (Noblesse): atk main
    // Goblet (Noblesse): hydro_dmg 46.6% main
    // Sands (Noblesse): er main
    // Flower (Noblesse): hp main
    // Aquila Favonia R2 passive: ATK +25%
    let flower_hp = artifact_main_stat_value(
        ArtifactRarity::Star5,
        ArtifactSlot::Flower,
        &BuffableStat::HpFlat,
        20,
    )
    .unwrap();
    let plume_atk = artifact_main_stat_value(
        ArtifactRarity::Star5,
        ArtifactSlot::Plume,
        &BuffableStat::AtkFlat,
        20,
    )
    .unwrap();
    let sands_er = artifact_main_stat_value(
        ArtifactRarity::Star5,
        ArtifactSlot::Sands,
        &BuffableStat::EnergyRecharge,
        20,
    )
    .unwrap();
    let circlet_cd = artifact_main_stat_value(
        ArtifactRarity::Star5,
        ArtifactSlot::Circlet,
        &BuffableStat::CritDmg,
        20,
    )
    .unwrap();
    let goblet_hydro = artifact_main_stat_value(
        ArtifactRarity::Star5,
        ArtifactSlot::Goblet,
        &BuffableStat::ElementalDmgBonus(Element::Hydro),
        20,
    )
    .unwrap();

    println!("--- Artifact Main Stats (5-star Lv20) ---");
    println!("  Flower HP: {:.0}", flower_hp);
    println!("  Plume ATK: {:.1}", plume_atk);
    println!("  Sands ER: {:.1}%", sands_er);
    println!("  Circlet CRIT DMG: {:.1}%", circlet_cd);
    println!("  Goblet Hydro DMG: {:.1}%", goblet_hydro);
    println!();

    let artifact_stats = StatProfile {
        base_hp: 0.0,
        base_atk: 0.0,
        base_def: 0.0,
        hp_flat: flower_hp + 269.0, // Flower main stat + Circlet substat hp
        atk_flat: plume_atk + 16.0, // Plume main stat + Flower substat atk flat
        hp_percent: 0.128 + 0.047,  // Goblet + Sands substats hp%
        atk_percent: 0.099 + 0.146 + 0.25 + 0.20, // Sands + Flower substats + Aquila R2 + NO 4pc
        def_percent: 0.051 + 0.197, // Goblet + Sands substats def%
        elemental_mastery: 37.0,    // Plume substat em
        crit_rate: 0.074 + 0.105 + 0.070, // Circlet + Plume + Flower (NO base 5%)
        crit_dmg: circlet_cd / 100.0 + 0.078, // Circlet mainstat + Plume sub
        energy_recharge: sands_er / 100.0 + 0.246 + 0.175 + 0.188 + 0.175, // Sands main + others (NO base 100%)
        dmg_bonus: 0.0,
        pyro_dmg_bonus: 0.0,
        pyro_crit_dmg_bonus: 0.0,
        hydro_dmg_bonus: goblet_hydro / 100.0, // Goblet main stat
        hydro_crit_dmg_bonus: 0.0,
        electro_dmg_bonus: 0.0,
        electro_crit_dmg_bonus: 0.0,
        cryo_dmg_bonus: 0.0,
        cryo_crit_dmg_bonus: 0.0,
        dendro_dmg_bonus: 0.0,
        dendro_crit_dmg_bonus: 0.0,
        anemo_dmg_bonus: 0.0,
        anemo_crit_dmg_bonus: 0.0,
        geo_dmg_bonus: 0.0,
        geo_crit_dmg_bonus: 0.0,
        physical_dmg_bonus: 0.0,      // Added via weapon sub-stat
        def_flat: 23.0 + 39.0 + 46.0, // Circlet + Goblet + Sands substats
    };

    println!("--- Artifact Stats (5-star +20) ---");
    println!("  HP Flat: {:.0}", artifact_stats.hp_flat);
    println!("  ATK Flat: {:.1}", artifact_stats.atk_flat);
    println!("  ATK%: {:.1}%", artifact_stats.atk_percent * 100.0);
    println!("  Crit Rate: {:.1}%", artifact_stats.crit_rate * 100.0);
    println!("  Crit DMG: {:.1}%", artifact_stats.crit_dmg * 100.0);
    println!(
        "  Energy Recharge: {:.1}%",
        artifact_stats.energy_recharge * 100.0
    );
    println!(
        "  Hydro DMG%: {:.1}%",
        artifact_stats.hydro_dmg_bonus * 100.0
    );
    println!("  DEF Flat: {:.1}", artifact_stats.def_flat);
    println!();

    // Build TeamMember
    let member = TeamMemberBuilder::new(bennett, aquila)
        .character_level(90)
        .weapon_level(90)
        .constellation(6)
        .refinement(2)
        .artifact_set(blizzard_strayer)
        .artifact_stats(artifact_stats)
        .build()
        .expect("Failed to build");

    let final_stats = combine_stats(&member.stats).expect("Failed to combine stats");

    println!("=== Final Stats (Lv90, C6, Aquila R2, 4NO+1BS +20) ===");
    println!("  HP: {:.0}", final_stats.hp);
    println!("  ATK: {:.1}", final_stats.atk);
    println!("  DEF: {:.1}", final_stats.def);
    println!("  Elemental Mastery: {:.0}", final_stats.elemental_mastery);
    println!("  Crit Rate: {:.1}%", final_stats.crit_rate * 100.0);
    println!("  Crit DMG: {:.1}%", final_stats.crit_dmg * 100.0);
    println!(
        "  Energy Recharge: {:.1}%",
        final_stats.energy_recharge * 100.0
    );
    println!(
        "  Physical DMG Bonus: {:.1}%",
        final_stats.physical_dmg_bonus * 100.0
    );
    println!(
        "  Hydro DMG Bonus: {:.1}%",
        final_stats.hydro_dmg_bonus * 100.0
    );
    println!();

    // C6 Bennett passive (20% Physical DMG Bonus)
    println!("=== With C6 Bennett Physical DMG Bonus (20%) ===");
    let physical_total = 0.377 + 0.20; // Aquila sub-stat + C6 passive
    println!("  Physical DMG Bonus: {:.1}%", physical_total * 100.0);
}
