//! Integration tests for the complete Moonsign pipeline.

use genshin_calc_core::*;

const EPSILON: f64 = 0.01;

/// Test: Ineffa solo — LunarElectroCharged with BaseDMGBonus
#[test]
fn test_ineffa_solo_lunar_ec() {
    // Ineffa: ATK 2000 → BaseDMGBonus = 2000 * 0.00007 = 0.14
    let benediction = MoonsignBenediction {
        base_dmg_bonus: 0.14,
        enabled_reactions: vec![Reaction::LunarElectroCharged],
    };
    let ctx = resolve_moonsign_context(1, &[benediction], 0.0, vec![]);

    let input = LunarInput {
        character_level: 90,
        elemental_mastery: 300.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: 0.0,
        crit_rate: 0.6,
        crit_dmg: 1.2,
        base_dmg_bonus: ctx.base_dmg_bonus_for(Reaction::LunarElectroCharged),
    };

    let enemy = Enemy {
        level: 90,
        resistance: 0.1,
        def_reduction: 0.0,
    };
    let result = calculate_lunar(&input, &enemy).unwrap();

    // Verify BaseDMGBonus is applied
    // em_bonus = 6 * 300 / (300 + 2000) = 0.7826...
    // non_crit = 1446.8535 * 1.8 * (1 + 0.14) * (1 + 0.7826) * 0.9
    let level_base = 1446.8535;
    let em_bonus = 6.0 * 300.0 / (300.0 + 2000.0);
    let expected = level_base * 1.8 * 1.14 * (1.0 + em_bonus) * 0.9;
    assert!((result.non_crit - expected).abs() < EPSILON);
}

/// Test: Lauma NascentGleam — LunarBloom with crit grant
#[test]
fn test_lauma_nascent_gleam_bloom_crit() {
    let enhancements = vec![MoonsignTalentEnhancement {
        character_name: "Lauma",
        required_level: MoonsignLevel::NascentGleam,
        description: "Bloom gains crit",
        effect: MoonsignTalentEffect::GrantReactionCrit {
            reaction: Reaction::LunarBloom,
            crit_rate: 0.15,
            crit_dmg: 1.0,
        },
    }];
    let benediction = MoonsignBenediction {
        base_dmg_bonus: 0.14,
        enabled_reactions: vec![Reaction::LunarBloom],
    };
    let ctx = resolve_moonsign_context(1, &[benediction], 0.0, enhancements);

    let base_input = LunarInput {
        character_level: 90,
        elemental_mastery: 800.0,
        reaction: Reaction::LunarBloom,
        reaction_bonus: 0.0,
        crit_rate: 0.0,
        crit_dmg: 0.0,
        base_dmg_bonus: ctx.base_dmg_bonus_for(Reaction::LunarBloom),
    };

    // Apply Lauma's crit grant
    let input = apply_moonsign_enhancements(&base_input, &ctx.talent_enhancements);
    assert!((input.crit_rate - 0.15).abs() < 1e-6);
    assert!((input.crit_dmg - 1.0).abs() < 1e-6);

    let enemy = Enemy {
        level: 90,
        resistance: 0.1,
        def_reduction: 0.0,
    };
    let result = calculate_lunar(&input, &enemy).unwrap();
    assert!(result.crit > result.non_crit);
}

/// Test: 2-member team (Ineffa + Columbina) with non-moonsign buff
#[test]
fn test_two_moonsign_with_non_moonsign_buff() {
    let benedictions = vec![
        MoonsignBenediction {
            base_dmg_bonus: 0.14,
            enabled_reactions: vec![Reaction::LunarElectroCharged],
        },
        MoonsignBenediction {
            base_dmg_bonus: 0.07,
            enabled_reactions: vec![
                Reaction::LunarElectroCharged,
                Reaction::LunarBloom,
                Reaction::LunarCrystallize,
            ],
        },
    ];

    // Non-moonsign: Pyro character with 2000 ATK → 0.18
    let non_moonsign_bonus = select_non_moonsign_buff(&[(Element::Pyro, 2000.0)]);
    assert!((non_moonsign_bonus - 0.18).abs() < 1e-6);

    let ctx = resolve_moonsign_context(2, &benedictions, non_moonsign_bonus, vec![]);
    assert_eq!(ctx.level, MoonsignLevel::AscendantGleam);
    assert!((ctx.base_dmg_bonus_for(Reaction::LunarElectroCharged) - 0.21).abs() < 1e-6);

    let input = LunarInput {
        character_level: 90,
        elemental_mastery: 200.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: ctx.non_moonsign_lunar_bonus,
        crit_rate: 0.5,
        crit_dmg: 1.0,
        base_dmg_bonus: ctx.base_dmg_bonus_for(Reaction::LunarElectroCharged),
    };

    let enemy = Enemy {
        level: 90,
        resistance: 0.1,
        def_reduction: 0.0,
    };
    let result = calculate_lunar(&input, &enemy).unwrap();
    assert!(result.average > 0.0);
}

/// Test: calculate_lunar_team with 2 contributors
#[test]
fn test_lunar_team_integration() {
    let enemy = Enemy {
        level: 90,
        resistance: 0.1,
        def_reduction: 0.0,
    };
    let contributions = vec![
        LunarContribution {
            input: LunarInput {
                character_level: 90,
                elemental_mastery: 500.0,
                reaction: Reaction::LunarElectroCharged,
                reaction_bonus: 0.0,
                crit_rate: 0.6,
                crit_dmg: 1.2,
                base_dmg_bonus: 0.21,
            },
        },
        LunarContribution {
            input: LunarInput {
                character_level: 90,
                elemental_mastery: 100.0,
                reaction: Reaction::LunarElectroCharged,
                reaction_bonus: 0.0,
                crit_rate: 0.3,
                crit_dmg: 0.6,
                base_dmg_bonus: 0.21,
            },
        },
    ];
    let result = calculate_lunar_team(&contributions, &enemy).unwrap();
    // Team result should be higher than single strongest contributor weighted at 1.0
    let solo = calculate_lunar(&contributions[0].input, &enemy).unwrap();
    assert!(result.average > solo.average);
}
