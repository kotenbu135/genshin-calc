use genshin_calc_core::*;
use genshin_calc_data::*;

#[test]
fn diluc_normal_attack_with_data_crate() {
    let diluc = find_character("diluc").unwrap();
    let talent_lv = 9; // 0-indexed -> Lv10

    let hit1 = &diluc.talents.normal_attack.hits[0];
    let multiplier = hit1.values[talent_lv];

    let stats = Stats {
        hp: 20000.0,
        atk: 2000.0,
        def: 800.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };

    let hilichurl = find_enemy("hilichurl").unwrap();
    let enemy = hilichurl.to_enemy(hit1.damage_element, 90, 0.0);

    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: multiplier,
        scaling_stat: hit1.scaling_stat,
        damage_type: DamageType::Normal,
        element: hit1.damage_element,
        reaction: None,
        reaction_bonus: 0.0,
    };

    let result = calculate_damage(&input, &enemy).unwrap();
    assert!(result.non_crit > 0.0);
    assert!(result.crit > result.non_crit);
    assert!(result.average > result.non_crit);
}

#[test]
fn diluc_skill_pyro_damage() {
    let diluc = find_character("diluc").unwrap();
    let skill_hit = &diluc.talents.elemental_skill.scalings[0];

    assert_eq!(skill_hit.damage_element, Some(Element::Pyro));

    let stats = Stats {
        hp: 20000.0,
        atk: 2000.0,
        def: 800.0,
        elemental_mastery: 100.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };

    let hilichurl = find_enemy("hilichurl").unwrap();
    let enemy = hilichurl.to_enemy(skill_hit.damage_element, 90, 0.0);

    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: skill_hit.values[9],
        scaling_stat: skill_hit.scaling_stat,
        damage_type: DamageType::Skill,
        element: skill_hit.damage_element,
        reaction: None,
        reaction_bonus: 0.0,
    };

    let result = calculate_damage(&input, &enemy).unwrap();
    assert!(result.non_crit > 0.0);
}
