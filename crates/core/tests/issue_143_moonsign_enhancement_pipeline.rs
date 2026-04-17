//! Issue #143: MoonsignTalentEnhancement must flow through the team resolution
//! pipeline automatically.
//!
//! - `StatBuff` must surface in `applied_buffs` and (for unconditional stat kinds)
//!   reflect in `final_stats`.
//! - `ReactionDmgBonus` must surface in `damage_context.reaction_dmg_bonuses`.
//! - `GrantReactionCrit` remains in `moonsign_context.talent_enhancements` for
//!   consumers to apply via `apply_moonsign_enhancements`.

use genshin_calc_core::*;

const EPSILON: f64 = 1e-6;

fn base_profile(atk: f64, em: f64) -> StatProfile {
    StatProfile {
        base_atk: atk,
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
fn test_nefer_a1_em_stat_buff_applied_to_target_at_ascendant_gleam() {
    // Nefer A1 at Ascendant grants EM +100 to self. Set up a 2-moonsign team
    // (Ascendant level) and resolve targeting Nefer.
    let nefer_enhancement: &'static [MoonsignTalentEnhancement] =
        Box::leak(Box::new([MoonsignTalentEnhancement {
            character_name: "Nefer",
            required_level: MoonsignLevel::AscendantGleam,
            description: "Nefer A1 EM +100 at Ascendant Gleam",
            effect: MoonsignTalentEffect::StatBuff {
                stat: BuffableStat::ElementalMastery,
                value: 100.0,
                target: BuffTarget::OnlySelf,
            },
        }]));
    let nefer = TeamMember {
        element: Element::Dendro,
        weapon_type: WeaponType::Catalyst,
        stats: base_profile(1500.0, 500.0),
        buffs_provided: vec![],
        is_moonsign: true,
        can_nightsoul: false,
        moonsign_benediction: Some(MoonsignBenedictionSpec {
            enabled_reactions: vec![Reaction::LunarBloom],
            scaling_stat: Some(ScalingStat::Em),
            rate: 0.000175,
            max_bonus: 0.14,
        }),
        moonsign_talent_enhancements: nefer_enhancement,
    };
    let columbina = TeamMember {
        element: Element::Hydro,
        weapon_type: WeaponType::Catalyst,
        stats: base_profile(1000.0, 0.0),
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
        moonsign_talent_enhancements: &[],
    };

    let result = resolve_team_stats(&[nefer, columbina], 0, &[]).unwrap();

    // applied_buffs must contain the EM +100 buff
    let em_buff = result.applied_buffs.iter().find(|b| {
        matches!(b.stat, BuffableStat::ElementalMastery) && (b.value - 100.0).abs() < EPSILON
    });
    assert!(
        em_buff.is_some(),
        "expected EM +100 in applied_buffs, got: {:?}",
        result
            .applied_buffs
            .iter()
            .map(|b| (&b.source, &b.stat, b.value))
            .collect::<Vec<_>>()
    );

    // final_stats.elemental_mastery reflects the +100 on top of base 500
    assert!(
        (result.final_stats.elemental_mastery - 600.0).abs() < EPSILON,
        "got {}",
        result.final_stats.elemental_mastery
    );
}

#[test]
fn test_reaction_dmg_bonus_enhancement_reaches_damage_context() {
    // Synthetic Moonsign char granting LunarBloom +20% at Ascendant Gleam (self).
    let ench: &'static [MoonsignTalentEnhancement] =
        Box::leak(Box::new([MoonsignTalentEnhancement {
            character_name: "TestBloomBoost",
            required_level: MoonsignLevel::AscendantGleam,
            description: "LunarBloom +20%",
            effect: MoonsignTalentEffect::ReactionDmgBonus {
                reaction: Reaction::LunarBloom,
                bonus: 0.20,
            },
        }]));
    let bloom_char = TeamMember {
        element: Element::Dendro,
        weapon_type: WeaponType::Catalyst,
        stats: base_profile(1500.0, 500.0),
        buffs_provided: vec![],
        is_moonsign: true,
        can_nightsoul: false,
        moonsign_benediction: Some(MoonsignBenedictionSpec {
            enabled_reactions: vec![Reaction::LunarBloom],
            scaling_stat: Some(ScalingStat::Em),
            rate: 0.000175,
            max_bonus: 0.14,
        }),
        moonsign_talent_enhancements: ench,
    };
    let partner = TeamMember {
        element: Element::Hydro,
        weapon_type: WeaponType::Catalyst,
        stats: base_profile(1000.0, 0.0),
        buffs_provided: vec![],
        is_moonsign: true,
        can_nightsoul: false,
        moonsign_benediction: Some(MoonsignBenedictionSpec {
            enabled_reactions: vec![Reaction::LunarBloom],
            scaling_stat: Some(ScalingStat::Hp),
            rate: 0.000002,
            max_bonus: 0.07,
        }),
        moonsign_talent_enhancements: &[],
    };

    let result = resolve_team_stats(&[bloom_char, partner], 0, &[]).unwrap();

    let bloom_bonus = result
        .damage_context
        .reaction_bonus_for(Reaction::LunarBloom);
    assert!(
        (bloom_bonus - 0.20).abs() < EPSILON,
        "expected LunarBloom +20% in damage_context, got {}",
        bloom_bonus
    );
    // Non-matching reaction must not be affected
    let ec_bonus = result
        .damage_context
        .reaction_bonus_for(Reaction::LunarElectroCharged);
    assert!(
        (ec_bonus - 0.0).abs() < EPSILON,
        "LunarEC must not receive LunarBloom enhancement, got {}",
        ec_bonus
    );
}

#[test]
fn test_nascent_enhancement_not_applied_at_level_none() {
    // Enhancement required_level = NascentGleam. With 0 moonsign members the
    // team level is None, so the enhancement must NOT be applied.
    let ench: &'static [MoonsignTalentEnhancement] =
        Box::leak(Box::new([MoonsignTalentEnhancement {
            character_name: "TestNone",
            required_level: MoonsignLevel::NascentGleam,
            description: "dummy",
            effect: MoonsignTalentEffect::StatBuff {
                stat: BuffableStat::ElementalMastery,
                value: 100.0,
                target: BuffTarget::OnlySelf,
            },
        }]));
    let non_moonsign = TeamMember {
        element: Element::Pyro,
        weapon_type: WeaponType::Sword,
        stats: base_profile(1500.0, 0.0),
        buffs_provided: vec![],
        is_moonsign: false,
        can_nightsoul: false,
        moonsign_benediction: None,
        moonsign_talent_enhancements: ench,
    };
    let result = resolve_team_stats(&[non_moonsign], 0, &[]).unwrap();
    // No EM buff from inactive enhancement
    let has_em_buff = result.applied_buffs.iter().any(|b| {
        matches!(b.stat, BuffableStat::ElementalMastery) && (b.value - 100.0).abs() < EPSILON
    });
    assert!(!has_em_buff, "inactive enhancement must not produce buffs");
}
