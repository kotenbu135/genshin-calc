use genshin_calc_core::*;
use genshin_calc_data::*;

#[test]
fn test_bennett_kazuha_team_damage() {
    // Bennett: Aquila Favonia, Noblesse 4pc, C6, talents 1/1/13
    let bennett = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("aquila_favonia").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(6)
    .talent_levels([1, 1, 13])
    .activate("Fantastic Voyage ATK Bonus")
    .build()
    .unwrap();

    // Simple DPS: Pyro claymore user
    let dps = TeamMember {
        element: Element::Pyro,
        weapon_type: WeaponType::Claymore,
        stats: StatProfile {
            base_hp: 13000.0,
            base_atk: 900.0,
            base_def: 700.0,
            crit_rate: 0.70,
            crit_dmg: 1.50,
            energy_recharge: 1.0,
            dmg_bonus: 0.466,
            ..Default::default()
        },
        buffs_provided: vec![],
        is_moonsign: false,
    };

    let team = [bennett, dps];

    // Resolve stats for DPS (index 1)
    let result = resolve_team_stats_detailed(&team, 1).unwrap();

    // DPS should have received Bennett's ATK flat buff
    let has_bennett_buff = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("Fantastic Voyage"));
    assert!(has_bennett_buff, "DPS should receive Bennett burst buff");

    // Final ATK should be higher than base
    assert!(result.final_stats.atk > 900.0);

    // Calculate damage with buffed stats
    let damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats,
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        },
    )
    .unwrap();

    assert!(damage.average > 0.0);
    assert!(damage.crit > damage.non_crit);
}

#[test]
fn test_four_member_team_with_resonance() {
    let pyro1 = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("aquila_favonia").unwrap(),
    )
    .build()
    .unwrap();

    let pyro2 = TeamMemberBuilder::new(
        find_character("xiangling").unwrap(),
        find_weapon("the_catch").unwrap(),
    )
    .build()
    .unwrap();

    let hydro = TeamMemberBuilder::new(
        find_character("xingqiu").unwrap(),
        find_weapon("sacrificial_sword").unwrap(),
    )
    .build()
    .unwrap();

    let cryo = TeamMemberBuilder::new(
        find_character("rosaria").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .build()
    .unwrap();

    let team = [pyro1, pyro2, hydro, cryo];
    let result = resolve_team_stats_detailed(&team, 1).unwrap();

    // Should have Pyro resonance (FerventFlames)
    assert!(
        result
            .resonances
            .contains(&ElementalResonance::FerventFlames),
        "Should have Pyro resonance"
    );

    // ATK should include 25% bonus from resonance
    assert!(result.final_stats.atk > result.base_stats.atk);
}
