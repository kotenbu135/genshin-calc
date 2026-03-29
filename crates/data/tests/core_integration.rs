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
        flat_dmg: 0.0,
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
        flat_dmg: 0.0,
    };

    let result = calculate_damage(&input, &enemy).unwrap();
    assert!(result.non_crit > 0.0);
}

#[test]
fn test_engulfing_lightning_er_scaling_resolved() {
    let char = genshin_calc_data::find_character("raiden_shogun").unwrap();
    let weapon = genshin_calc_data::find_weapon("engulfing_lightning").unwrap();
    let member = genshin_calc_data::TeamMemberBuilder::new(char, weapon)
        .build()
        .unwrap();

    // Should have an AtkPercent buff from ER scaling
    let atk_buff = member.buffs_provided.iter().find(|b| {
        b.stat == genshin_calc_core::BuffableStat::AtkPercent && b.source.contains("engulfing")
    });
    assert!(
        atk_buff.is_some(),
        "Engulfing Lightning should resolve AtkPercent StatScaling buff"
    );
}

#[test]
fn test_redhorn_def_flat_damage_resolved() {
    let char = genshin_calc_data::find_character("itto").unwrap();
    let weapon = genshin_calc_data::find_weapon("redhorn_stonethresher").unwrap();
    let member = genshin_calc_data::TeamMemberBuilder::new(char, weapon)
        .build()
        .unwrap();

    let has_normal_flat = member
        .buffs_provided
        .iter()
        .any(|b| b.stat == genshin_calc_core::BuffableStat::NormalAtkFlatDmg);
    let has_charged_flat = member
        .buffs_provided
        .iter()
        .any(|b| b.stat == genshin_calc_core::BuffableStat::ChargedAtkFlatDmg);
    assert!(has_normal_flat, "Redhorn should resolve NormalAtkFlatDmg");
    assert!(has_charged_flat, "Redhorn should resolve ChargedAtkFlatDmg");
}

#[test]
fn test_staff_of_scarlet_sands_em_atk_resolved() {
    let char = genshin_calc_data::find_character("cyno").unwrap();
    let weapon = genshin_calc_data::find_weapon("staff_of_the_scarlet_sands").unwrap();
    let member = genshin_calc_data::TeamMemberBuilder::new(char, weapon)
        .build()
        .unwrap();

    let atk_flat_buff = member.buffs_provided.iter().find(|b| {
        b.stat == genshin_calc_core::BuffableStat::AtkFlat && b.source.contains("scarlet_sands")
    });
    assert!(
        atk_flat_buff.is_some(),
        "Staff of Scarlet Sands should resolve AtkFlat StatScaling buff"
    );
}
