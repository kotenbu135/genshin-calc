//! Issue #144: Flins A1 "Symphony of Winter" (+20% Lunar-Charged DMG) must not
//! be double-counted. Canonical source is `moonsign_chars.rs::FLINS_TALENT_ENHANCEMENTS`
//! (routed automatically by the moonsign pipeline once team reaches Ascendant
//! Gleam); the duplicate in `talent_buffs/electro.rs` is removed.

use genshin_calc_core::*;
use genshin_calc_data::{TeamMemberBuilder, find_character, find_weapon, talent_buffs};

const EPSILON: f64 = 1e-6;

#[test]
fn test_flins_a1_not_registered_in_talent_buffs() {
    let buffs = talent_buffs::find_talent_buffs("flins").expect("Flins talent buffs");
    assert!(
        !buffs.iter().any(|b| b.name == "Symphony of Winter"),
        "Flins A1 'Symphony of Winter' must not live in talent_buffs (duplicate \
         with moonsign_chars.rs::FLINS_TALENT_ENHANCEMENTS)"
    );
}

#[test]
fn test_flins_a1_still_routes_at_ascendant_gleam() {
    let flins_char = find_character("flins").expect("Flins character data");
    let columbina_char = find_character("columbina").expect("Columbina character data");
    let weapon = find_weapon("prototype_rancour")
        .or_else(|| find_weapon("favonius_greatsword"))
        .or_else(|| find_weapon("the_black_sword"))
        .expect("a neutral fallback weapon");

    let flins = TeamMemberBuilder::new(flins_char, weapon)
        .build()
        .expect("Flins TeamMember");
    let columbina = TeamMemberBuilder::new(columbina_char, weapon)
        .build()
        .expect("Columbina TeamMember");

    let result = resolve_team_stats(&[flins, columbina], 0, &[]).unwrap();
    assert_eq!(result.moonsign_context.level, MoonsignLevel::AscendantGleam);

    // LunarElectroCharged reaction bonus must include exactly +20% from A1
    // (no double-count, no missing).
    let bonus = result
        .damage_context
        .reaction_bonus_for(Reaction::LunarElectroCharged);
    assert!(
        (bonus - 0.20).abs() < EPSILON,
        "expected exactly +20% LunarEC bonus, got {}",
        bonus
    );
}
