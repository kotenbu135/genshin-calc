//! Issue #142: Expose MoonsignContext via TeamResolveResult.
//!
//! TeamResolveResult must include a moonsign_context field so consumers can
//! obtain the per-reaction Base DMG Bonus without passing 0 to LunarInput.

use genshin_calc_core::*;

const EPSILON: f64 = 1e-6;

fn default_profile(base_atk: f64, em: f64) -> StatProfile {
    StatProfile {
        base_atk,
        base_hp: 10000.0,
        base_def: 500.0,
        elemental_mastery: em,
        crit_rate: 0.5,
        crit_dmg: 1.0,
        energy_recharge: 1.0,
        ..Default::default()
    }
}

#[test]
fn test_team_resolve_result_has_moonsign_context_field() {
    // Single non-moonsign member — moonsign_context should exist and be empty.
    let member = TeamMember {
        element: Element::Pyro,
        weapon_type: WeaponType::Sword,
        stats: default_profile(2000.0, 0.0),
        buffs_provided: vec![],
        is_moonsign: false,
        can_nightsoul: false,
        moonsign_benediction: None,
    };
    let result = resolve_team_stats(&[member], 0, &[]).unwrap();
    assert_eq!(result.moonsign_context.level, MoonsignLevel::None);
    assert!(
        result
            .moonsign_context
            .base_dmg_bonus_by_reaction
            .is_empty()
    );
}

#[test]
fn test_ineffa_solo_exposes_lunar_ec_base_dmg_bonus() {
    // Ineffa-like: ATK 2000 → benediction 0.00007 * 2000 = 0.14 for LunarElectroCharged.
    let member = TeamMember {
        element: Element::Electro,
        weapon_type: WeaponType::Catalyst,
        stats: default_profile(2000.0, 0.0),
        buffs_provided: vec![],
        is_moonsign: true,
        can_nightsoul: false,
        moonsign_benediction: Some(MoonsignBenedictionSpec {
            enabled_reactions: vec![Reaction::LunarElectroCharged],
            scaling_stat: Some(ScalingStat::Atk),
            rate: 0.00007,
            max_bonus: 0.14,
        }),
    };
    let result = resolve_team_stats(&[member], 0, &[]).unwrap();
    assert_eq!(result.moonsign_context.level, MoonsignLevel::NascentGleam);
    let bonus = result
        .moonsign_context
        .base_dmg_bonus_for(Reaction::LunarElectroCharged);
    assert!((bonus - 0.14).abs() < EPSILON);
}

#[test]
fn test_two_moonsign_team_ascendant_gleam() {
    // Ineffa (+0.14 LunarEC) + Columbina (+0.07 LunarEC/Bloom/Crystallize).
    // Total LunarEC = 0.21.
    let ineffa = TeamMember {
        element: Element::Electro,
        weapon_type: WeaponType::Catalyst,
        stats: default_profile(2000.0, 0.0),
        buffs_provided: vec![],
        is_moonsign: true,
        can_nightsoul: false,
        moonsign_benediction: Some(MoonsignBenedictionSpec {
            enabled_reactions: vec![Reaction::LunarElectroCharged],
            scaling_stat: Some(ScalingStat::Atk),
            rate: 0.00007,
            max_bonus: 0.14,
        }),
    };
    let columbina = TeamMember {
        element: Element::Hydro,
        weapon_type: WeaponType::Catalyst,
        stats: StatProfile {
            base_hp: 35000.0,
            ..default_profile(1000.0, 0.0)
        },
        buffs_provided: vec![],
        is_moonsign: true,
        can_nightsoul: false,
        moonsign_benediction: Some(MoonsignBenedictionSpec {
            enabled_reactions: vec![
                Reaction::LunarElectroCharged,
                Reaction::LunarBloom,
                Reaction::LunarCrystallize,
            ],
            scaling_stat: Some(ScalingStat::Hp),
            rate: 0.000002,
            max_bonus: 0.07,
        }),
    };
    let result = resolve_team_stats(&[ineffa, columbina], 0, &[]).unwrap();
    assert_eq!(result.moonsign_context.level, MoonsignLevel::AscendantGleam);
    let ec = result
        .moonsign_context
        .base_dmg_bonus_for(Reaction::LunarElectroCharged);
    assert!((ec - 0.21).abs() < EPSILON, "LunarEC bonus: {ec}");
    let bloom = result
        .moonsign_context
        .base_dmg_bonus_for(Reaction::LunarBloom);
    assert!((bloom - 0.07).abs() < EPSILON, "LunarBloom bonus: {bloom}");
}

#[test]
fn test_non_moonsign_lunar_bonus_computed_for_non_moonsign_members() {
    // non_moonsign_lunar_bonus is only active at AscendantGleam (2+ moonsign).
    // Ineffa + Columbina (both moonsign) + Pyro non-moonsign with 2000 ATK.
    // Expected: 0.00009 * 2000 = 0.18 (cap 0.36).
    let ineffa = TeamMember {
        element: Element::Electro,
        weapon_type: WeaponType::Catalyst,
        stats: default_profile(2000.0, 0.0),
        buffs_provided: vec![],
        is_moonsign: true,
        can_nightsoul: false,
        moonsign_benediction: Some(MoonsignBenedictionSpec {
            enabled_reactions: vec![Reaction::LunarElectroCharged],
            scaling_stat: Some(ScalingStat::Atk),
            rate: 0.00007,
            max_bonus: 0.14,
        }),
    };
    let columbina = TeamMember {
        element: Element::Hydro,
        weapon_type: WeaponType::Catalyst,
        stats: default_profile(1000.0, 0.0),
        buffs_provided: vec![],
        is_moonsign: true,
        can_nightsoul: false,
        moonsign_benediction: Some(MoonsignBenedictionSpec {
            enabled_reactions: vec![Reaction::LunarBloom],
            scaling_stat: Some(ScalingStat::Hp),
            rate: 0.000002,
            max_bonus: 0.07,
        }),
    };
    let pyro_dps = TeamMember {
        element: Element::Pyro,
        weapon_type: WeaponType::Sword,
        stats: default_profile(2000.0, 0.0),
        buffs_provided: vec![],
        is_moonsign: false,
        can_nightsoul: false,
        moonsign_benediction: None,
    };
    let result = resolve_team_stats(&[ineffa, columbina, pyro_dps], 0, &[]).unwrap();
    assert!(
        (result.moonsign_context.non_moonsign_lunar_bonus - 0.18).abs() < EPSILON,
        "got {}",
        result.moonsign_context.non_moonsign_lunar_bonus
    );
}

#[test]
fn test_non_moonsign_lunar_bonus_zero_at_nascent_gleam() {
    // At NascentGleam (1 moonsign), non_moonsign_lunar_bonus must stay 0.
    let ineffa = TeamMember {
        element: Element::Electro,
        weapon_type: WeaponType::Catalyst,
        stats: default_profile(2000.0, 0.0),
        buffs_provided: vec![],
        is_moonsign: true,
        can_nightsoul: false,
        moonsign_benediction: Some(MoonsignBenedictionSpec {
            enabled_reactions: vec![Reaction::LunarElectroCharged],
            scaling_stat: Some(ScalingStat::Atk),
            rate: 0.00007,
            max_bonus: 0.14,
        }),
    };
    let pyro_dps = TeamMember {
        element: Element::Pyro,
        weapon_type: WeaponType::Sword,
        stats: default_profile(2000.0, 0.0),
        buffs_provided: vec![],
        is_moonsign: false,
        can_nightsoul: false,
        moonsign_benediction: None,
    };
    let result = resolve_team_stats(&[ineffa, pyro_dps], 0, &[]).unwrap();
    assert!((result.moonsign_context.non_moonsign_lunar_bonus - 0.0).abs() < EPSILON);
}
