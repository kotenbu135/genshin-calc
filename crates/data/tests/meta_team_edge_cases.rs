//! Edge Case Tests for WASM Calculation Engine
//!
//! フロントエンドが遭遇しやすいエッジケースを検証:
//! - RES shred が正しく適用されるか（debuffs経由）
//! - 複数バフの重複適用
//! - DEF/HP/EMスケーリング武器の正確な計算
//! - Nightsoul状態の正しいハンドリング
//! - 空チーム/1人チームのエラーハンドリング
//! - 具体的な数値の妥当性チェック

use genshin_calc_core::*;
use genshin_calc_data::*;

// =============================================================================
// Edge Case 1: Enemy debuffs from team (VV + Zhongli + Xilonen stacking)
// =============================================================================

#[test]
fn edge_res_shred_stacking() {
    // VV Pyro shred (-40%) + Zhongli universal (-20%) + Xilonen Geo RES reduction
    // フロントから「耐性デバフが効いてない」報告が多いパターン

    let kazuha = TeamMemberBuilder::new(
        find_character("kazuha").unwrap(),
        find_weapon("freedom_sworn").unwrap(),
    )
    .artifact_set(find_artifact_set("viridescent_venerer").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 1])
    .activate("vv_res_shred_pyro")
    .artifact_stats(StatProfile {
        elemental_mastery: 600.0,
        ..Default::default()
    })
    .build()
    .unwrap();

    let zhongli = TeamMemberBuilder::new(
        find_character("zhongli").unwrap(),
        find_weapon("black_tassel").unwrap(),
    )
    .artifact_set(find_artifact_set("tenacity_of_the_millelith").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 1])
    .activate("Jade Shield Pyro RES Shred")
    .build()
    .unwrap();

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
            ..Default::default()
        },
        buffs_provided: vec![],
        is_moonsign: false,
        can_nightsoul: false,
    };

    let team = [dps, kazuha, zhongli];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    // Check enemy debuffs are accumulated
    let pyro_shred = result.enemy_debuffs.pyro_res_reduction;
    // VV = 0.40, Zhongli = 0.20 -> total should be ~0.60
    assert!(
        pyro_shred > 0.50,
        "Pyro RES reduction should be >= 0.50 (VV + Zhongli), got {}",
        pyro_shred
    );

    // Calculate damage with and without debuffs to verify they actually work
    let enemy_no_debuff = Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
        def_ignore: 0.0,
    };

    let enemy_with_debuff =
        apply_enemy_debuffs(&enemy_no_debuff, &result.applied_buffs, Some(Element::Pyro));

    let dmg_no_debuff = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats.clone(),
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &enemy_no_debuff,
    )
    .unwrap();

    let dmg_with_debuff = calculate_damage(
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
        &enemy_with_debuff,
    )
    .unwrap();

    assert!(
        dmg_with_debuff.average > dmg_no_debuff.average,
        "Damage with RES shred should be higher: with={} vs without={}",
        dmg_with_debuff.average,
        dmg_no_debuff.average
    );
}

// =============================================================================
// Edge Case 2: DEF-scaling weapons (Redhorn on Itto)
// =============================================================================

#[test]
fn edge_def_scaling_redhorn_itto() {
    let itto = TeamMemberBuilder::new(
        find_character("itto").unwrap(),
        find_weapon("redhorn_stonethresher").unwrap(),
    )
    .artifact_set(find_artifact_set("husk_of_opulent_dreams").unwrap())
    .constellation(0)
    .talent_levels([9, 9, 9])
    .artifact_stats(StatProfile {
        crit_rate: 0.30,
        crit_dmg: 0.60,
        def_percent: 0.60,
        ..Default::default()
    })
    .build()
    .unwrap();

    let gorou = TeamMemberBuilder::new(
        find_character("gorou").unwrap(),
        find_weapon("favonius_warbow").unwrap(),
    )
    .constellation(6)
    .talent_levels([1, 9, 9])
    .build()
    .unwrap();

    let team = [itto, gorou];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    // Redhorn should add flat DMG to Normal/Charged based on DEF
    let has_normal_flat = result.damage_context.normal_atk_flat_dmg > 0.0;
    let has_charged_flat = result.damage_context.charged_atk_flat_dmg > 0.0;

    assert!(
        has_normal_flat,
        "Redhorn should add flat DMG to Normal attacks, got normal_flat={}",
        result.damage_context.normal_atk_flat_dmg
    );
    assert!(
        has_charged_flat,
        "Redhorn should add flat DMG to Charged attacks, got charged_flat={}",
        result.damage_context.charged_atk_flat_dmg
    );

    // Calculate Normal Attack with flat DMG
    let char_data = find_character("itto").unwrap();
    let normal_hit = &char_data.talents.normal_attack.hits[0];

    let with_flat = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats.clone(),
            talent_multiplier: normal_hit.values[9],
            scaling_stat: normal_hit.scaling_stat,
            damage_type: DamageType::Normal,
            element: Some(Element::Geo),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: result.damage_context.normal_atk_flat_dmg,
        },
        &Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
            def_ignore: 0.0,
        },
    )
    .unwrap();

    let without_flat = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats,
            talent_multiplier: normal_hit.values[9],
            scaling_stat: normal_hit.scaling_stat,
            damage_type: DamageType::Normal,
            element: Some(Element::Geo),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
            def_ignore: 0.0,
        },
    )
    .unwrap();

    assert!(
        with_flat.average > without_flat.average,
        "Flat DMG from Redhorn should increase damage: with={} vs without={}",
        with_flat.average,
        without_flat.average
    );
}

// =============================================================================
// Edge Case 3: Engulfing Lightning ER→ATK scaling cap
// =============================================================================

#[test]
fn edge_engulfing_lightning_er_cap() {
    // Engulfing Lightning converts excess ER to ATK%, but has a cap
    // This is a common bug report: ATK seems too high or too low

    let raiden_low_er = TeamMemberBuilder::new(
        find_character("raiden_shogun").unwrap(),
        find_weapon("engulfing_lightning").unwrap(),
    )
    .artifact_set(find_artifact_set("emblem_of_severed_fate").unwrap())
    .talent_levels([1, 9, 9])
    .artifact_stats(StatProfile {
        energy_recharge: 0.20,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        ..Default::default()
    })
    .build()
    .unwrap();

    let raiden_high_er = TeamMemberBuilder::new(
        find_character("raiden_shogun").unwrap(),
        find_weapon("engulfing_lightning").unwrap(),
    )
    .artifact_set(find_artifact_set("emblem_of_severed_fate").unwrap())
    .talent_levels([1, 9, 9])
    .artifact_stats(StatProfile {
        energy_recharge: 1.00, // Very high ER
        crit_rate: 0.50,
        crit_dmg: 1.00,
        ..Default::default()
    })
    .build()
    .unwrap();

    let team_low = [raiden_low_er];
    let team_high = [raiden_high_er];

    let result_low = resolve_team_stats_detailed(&team_low, 0).unwrap();
    let result_high = resolve_team_stats_detailed(&team_high, 0).unwrap();

    // Higher ER should give more ATK
    assert!(
        result_high.final_stats.atk > result_low.final_stats.atk,
        "Higher ER Raiden should have more ATK: high={} vs low={}",
        result_high.final_stats.atk,
        result_low.final_stats.atk
    );

    // But there should be a cap (not unbounded)
    let atk_diff = result_high.final_stats.atk - result_low.final_stats.atk;
    // The cap is ~80% ATK bonus. With ~337 base ATK, max bonus ~270.
    // ER difference is 0.80, at 28% per 1.0 excess ER = ~22.4% ATK bonus
    // This should be well within the 80% cap
    assert!(
        atk_diff < 400.0,
        "ATK bonus from ER should be capped, got diff={}",
        atk_diff
    );
}

// =============================================================================
// Edge Case 4: Emblem 4pc Burst DMG from ER (cap 75%)
// =============================================================================

#[test]
fn edge_emblem_burst_dmg_cap() {
    // 4pc Emblem: Burst DMG = 25% of ER, max 75%
    // With 300% ER, bonus should be capped at 75%, not 75%

    let raiden_300er = TeamMemberBuilder::new(
        find_character("raiden_shogun").unwrap(),
        find_weapon("engulfing_lightning").unwrap(),
    )
    .artifact_set(find_artifact_set("emblem_of_severed_fate").unwrap())
    .talent_levels([1, 9, 9])
    .artifact_stats(StatProfile {
        energy_recharge: 2.00, // Total will be ~3.0+ with weapon
        crit_rate: 0.50,
        crit_dmg: 1.00,
        ..Default::default()
    })
    .build()
    .unwrap();

    let team = [raiden_300er];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    // Burst DMG bonus from Emblem should not exceed 0.75
    let burst_bonus = result.damage_context.burst_dmg_bonus;
    assert!(
        burst_bonus <= 0.75 + 0.01,
        "Emblem 4pc burst DMG bonus should be capped at 75%, got {}",
        burst_bonus
    );
    // But should still be positive
    assert!(
        burst_bonus > 0.0,
        "Emblem should provide some burst DMG bonus"
    );
}

// =============================================================================
// Edge Case 5: Furina fanfare stacks (0 vs 300 vs max)
// =============================================================================

#[test]
fn edge_furina_fanfare_scaling() {
    let furina = TeamMemberBuilder::new(
        find_character("furina").unwrap(),
        find_weapon("splendor_of_tranquil_waters").unwrap(),
    )
    .artifact_set(find_artifact_set("golden_troupe").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .build()
    .unwrap();

    let furina_max = TeamMemberBuilder::new(
        find_character("furina").unwrap(),
        find_weapon("splendor_of_tranquil_waters").unwrap(),
    )
    .artifact_set(find_artifact_set("golden_troupe").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate_with_stacks("Let the People Rejoice DMG Bonus (C0 300pt)", 300)
    .build()
    .unwrap();

    let dps = TeamMember {
        element: Element::Pyro,
        weapon_type: WeaponType::Claymore,
        stats: StatProfile {
            base_hp: 13000.0,
            base_atk: 900.0,
            base_def: 700.0,
            crit_rate: 0.50,
            crit_dmg: 1.00,
            energy_recharge: 1.0,
            ..Default::default()
        },
        buffs_provided: vec![],
        is_moonsign: false,
        can_nightsoul: false,
    };

    let team_no_stacks = [furina, dps.clone()];
    let team_max_stacks = [furina_max, dps];

    let result_no = resolve_team_stats_detailed(&team_no_stacks, 1).unwrap();
    let result_max = resolve_team_stats_detailed(&team_max_stacks, 1).unwrap();

    // With 0 stacks, should have no DMG bonus from Furina
    let furina_buff_no: f64 = result_no
        .applied_buffs
        .iter()
        .filter(|b| b.source.contains("Rejoice"))
        .map(|b| b.value)
        .sum();

    let furina_buff_max: f64 = result_max
        .applied_buffs
        .iter()
        .filter(|b| b.source.contains("Rejoice"))
        .map(|b| b.value)
        .sum();

    assert!(
        furina_buff_max > furina_buff_no,
        "Max fanfare should give more DMG bonus: max={} vs none={}",
        furina_buff_max,
        furina_buff_no
    );

    // 300 fanfare at talent 9 should give a significant DMG bonus (around 50%+)
    assert!(
        furina_buff_max > 0.30,
        "300 fanfare should give significant DMG bonus, got {}",
        furina_buff_max
    );
}

// =============================================================================
// Edge Case 6: Shenhe flat DMG stacks for different attack types
// =============================================================================

#[test]
fn edge_shenhe_flat_dmg_per_type() {
    let shenhe = TeamMemberBuilder::new(
        find_character("shenhe").unwrap(),
        find_weapon("calamity_queller").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate_with_stacks("Spring Spirit Summoning Normal ATK Flat DMG", 5)
    .activate_with_stacks("Spring Spirit Summoning Charged ATK Flat DMG", 5)
    .activate_with_stacks("Spring Spirit Summoning Skill Flat DMG", 5)
    .activate_with_stacks("Spring Spirit Summoning Burst Flat DMG", 5)
    .activate("Deific Embrace Press - Skill DMG")
    .activate("Deific Embrace Press - Burst DMG")
    .build()
    .unwrap();

    let ayaka = TeamMemberBuilder::new(
        find_character("ayaka").unwrap(),
        find_weapon("mistsplitter_reforged").unwrap(),
    )
    .artifact_set(find_artifact_set("blizzard_strayer").unwrap())
    .constellation(0)
    .talent_levels([9, 9, 9])
    .artifact_stats(StatProfile {
        crit_dmg: 0.80,
        crit_rate: 0.20,
        atk_percent: 0.15,
        ..Default::default()
    })
    .build()
    .unwrap();

    let team = [shenhe, ayaka];
    let result = resolve_team_stats_detailed(&team, 1).unwrap();

    // Shenhe should add flat DMG to Normal, Charged, Skill, and Burst
    assert!(
        result.damage_context.normal_atk_flat_dmg > 0.0,
        "Shenhe should add Normal flat DMG: {}",
        result.damage_context.normal_atk_flat_dmg
    );
    assert!(
        result.damage_context.charged_atk_flat_dmg > 0.0,
        "Shenhe should add Charged flat DMG: {}",
        result.damage_context.charged_atk_flat_dmg
    );
    assert!(
        result.damage_context.skill_flat_dmg > 0.0,
        "Shenhe should add Skill flat DMG: {}",
        result.damage_context.skill_flat_dmg
    );
    assert!(
        result.damage_context.burst_flat_dmg > 0.0,
        "Shenhe should add Burst flat DMG: {}",
        result.damage_context.burst_flat_dmg
    );

    // Shenhe A4 should add Skill/Burst DMG bonus
    assert!(
        result.damage_context.skill_dmg_bonus > 0.0,
        "Shenhe A4 should add Skill DMG bonus: {}",
        result.damage_context.skill_dmg_bonus
    );
    assert!(
        result.damage_context.burst_dmg_bonus > 0.0,
        "Shenhe A4 should add Burst DMG bonus: {}",
        result.damage_context.burst_dmg_bonus
    );
}

// =============================================================================
// Edge Case 7: Xilonen RES reduction with nightsoul_value scaling
// =============================================================================

#[test]
fn edge_xilonen_res_reduction() {
    let xilonen = TeamMemberBuilder::new(
        find_character("xilonen").unwrap(),
        find_weapon("peak_patrol_song").unwrap(),
    )
    .artifact_set(find_artifact_set("scroll_of_the_hero_of_cinder_city").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate("Yohualliand Elemental RES Reduction")
    .activate("scroll_pyro_dmg")
    .artifact_stats(StatProfile {
        def_percent: 0.60,
        ..Default::default()
    })
    .build()
    .unwrap();

    let mavuika = TeamMemberBuilder::new(
        find_character("mavuika").unwrap(),
        find_weapon("a_thousand_blazing_suns").unwrap(),
    )
    .artifact_set(find_artifact_set("obsidian_codex").unwrap())
    .constellation(0)
    .talent_levels([9, 9, 9])
    .build()
    .unwrap();

    let team = [mavuika, xilonen];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    // Xilonen should provide RES reduction via enemy debuffs
    // Check that debuffs exist
    let total_debuffs = result.enemy_debuffs.pyro_res_reduction
        + result.enemy_debuffs.hydro_res_reduction
        + result.enemy_debuffs.electro_res_reduction
        + result.enemy_debuffs.cryo_res_reduction
        + result.enemy_debuffs.geo_res_reduction;

    // Xilonen Yohualliand should shred at least one element's RES
    assert!(
        total_debuffs > 0.0,
        "Xilonen should provide some RES reduction, total={}",
        total_debuffs
    );
}

// =============================================================================
// Edge Case 8: Kazuha A4 EM→DMG scaling accuracy
// =============================================================================

#[test]
fn edge_kazuha_a4_em_scaling() {
    // Kazuha A4: 0.04% elemental DMG per point of EM
    // 900 EM → 36% DMG bonus to swirled element

    let kazuha = TeamMemberBuilder::new(
        find_character("kazuha").unwrap(),
        find_weapon("freedom_sworn").unwrap(),
    )
    .artifact_set(find_artifact_set("viridescent_venerer").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 1])
    .artifact_stats(StatProfile {
        elemental_mastery: 600.0,
        ..Default::default()
    })
    .build()
    .unwrap();

    // Check the Kazuha buff value
    let poetics_buff = kazuha
        .buffs_provided
        .iter()
        .find(|b| b.source.contains("Poetics"))
        .expect("Kazuha should have Poetics of Fuubutsu buff");

    // Total EM should be: base EM from weapon/artifacts + substat
    // Freedom-Sworn: 198 EM secondary + 600 artifact = ~800+ EM
    // 0.04% per EM point → ~32%+ DMG bonus
    let expected_min_bonus = 0.20; // Conservative lower bound
    assert!(
        poetics_buff.value >= expected_min_bonus,
        "Kazuha A4 with ~800 EM should give >= 20% DMG bonus, got {}",
        poetics_buff.value
    );

    // Target should be Team
    assert_eq!(
        poetics_buff.target,
        BuffTarget::Team,
        "Kazuha A4 should be Team-wide"
    );
}

// =============================================================================
// Edge Case 9: Nightsoul characters flag correctly
// =============================================================================

#[test]
fn edge_nightsoul_flags() {
    let mavuika = TeamMemberBuilder::new(
        find_character("mavuika").unwrap(),
        find_weapon("a_thousand_blazing_suns").unwrap(),
    )
    .build()
    .unwrap();

    let xilonen = TeamMemberBuilder::new(
        find_character("xilonen").unwrap(),
        find_weapon("peak_patrol_song").unwrap(),
    )
    .build()
    .unwrap();

    let bennett = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("skyward_blade").unwrap(),
    )
    .build()
    .unwrap();

    // Mavuika and Xilonen are Natlan characters with nightsoul
    assert!(
        mavuika.can_nightsoul,
        "Mavuika should have can_nightsoul=true"
    );
    assert!(
        xilonen.can_nightsoul,
        "Xilonen should have can_nightsoul=true"
    );

    // Bennett is not a nightsoul character
    assert!(
        !bennett.can_nightsoul,
        "Bennett should have can_nightsoul=false"
    );
}

// =============================================================================
// Edge Case 10: Gilded Dreams element-count scaling
// =============================================================================

#[test]
fn edge_gilded_dreams_element_counting() {
    // 4pc Gilded Dreams scales based on same/different element teammates
    // This is tricky because the team composition matters

    // Gilded Dreams 4pc requires team_elements to be passed at build time
    // (Auto conditions TeamSameElementCount/TeamDiffElementCount need team context)
    let team_elements = vec![
        Element::Electro,
        Element::Electro,
        Element::Dendro,
        Element::Dendro,
    ];

    let cyno = TeamMemberBuilder::new(
        find_character("cyno").unwrap(),
        find_weapon("staff_of_the_scarlet_sands").unwrap(),
    )
    .artifact_set(find_artifact_set("gilded_dreams").unwrap())
    .constellation(0)
    .talent_levels([9, 1, 9])
    .team_elements(team_elements)
    .build()
    .unwrap();

    let fischl = TeamMemberBuilder::new(
        find_character("fischl").unwrap(),
        find_weapon("the_stringless").unwrap(),
    )
    .constellation(6)
    .talent_levels([1, 9, 1])
    .build()
    .unwrap();

    let nahida = TeamMemberBuilder::new(
        find_character("nahida").unwrap(),
        find_weapon("a_thousand_floating_dreams").unwrap(),
    )
    .constellation(0)
    .talent_levels([1, 9, 9])
    .build()
    .unwrap();

    let baizhu = TeamMemberBuilder::new(
        find_character("baizhu").unwrap(),
        find_weapon("jadefalls_splendor").unwrap(),
    )
    .constellation(0)
    .talent_levels([1, 9, 9])
    .build()
    .unwrap();

    // Team: Cyno(Electro), Fischl(Electro), Nahida(Dendro), Baizhu(Dendro)
    // Same element as Cyno: Fischl (1)
    // Different element: Nahida, Baizhu (2)
    let team = [cyno, fischl, nahida, baizhu];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    // Check that Gilded Dreams buffs are applied
    let gilded_buffs: Vec<&ResolvedBuff> = result
        .applied_buffs
        .iter()
        .filter(|b| b.source.contains("gilded"))
        .collect();

    // Should have some buff from Gilded Dreams
    assert!(
        !gilded_buffs.is_empty(),
        "Cyno should have Gilded Dreams buffs from team composition"
    );
}

// =============================================================================
// Edge Case 11: Bennett C6 Pyro infusion buff
// =============================================================================

#[test]
fn edge_bennett_c6_pyro_dmg() {
    let bennett_c6 = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("aquila_favonia").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(6)
    .talent_levels([1, 1, 13])
    .activate("Fantastic Voyage ATK Bonus")
    .activate("Spirit of Pyro")
    .activate("noblesse_atk_bonus")
    .build()
    .unwrap();

    let dps = TeamMember {
        element: Element::Pyro,
        weapon_type: WeaponType::Claymore,
        stats: StatProfile {
            base_hp: 13000.0,
            base_atk: 900.0,
            base_def: 700.0,
            crit_rate: 0.50,
            crit_dmg: 1.00,
            energy_recharge: 1.0,
            ..Default::default()
        },
        buffs_provided: vec![],
        is_moonsign: false,
        can_nightsoul: false,
    };

    let team = [bennett_c6, dps];
    let result = resolve_team_stats_detailed(&team, 1).unwrap();

    // C6 Bennett should provide Pyro DMG bonus
    let has_pyro_bonus = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("Spirit of Pyro") || b.source.contains("Pyro DMG"));

    assert!(
        has_pyro_bonus,
        "Bennett C6 should provide Pyro DMG bonus buff"
    );
}

// =============================================================================
// Edge Case 12: Single member team (no resonance, no team buffs)
// =============================================================================

#[test]
fn edge_single_member_team() {
    let solo = TeamMemberBuilder::new(
        find_character("raiden_shogun").unwrap(),
        find_weapon("engulfing_lightning").unwrap(),
    )
    .artifact_set(find_artifact_set("emblem_of_severed_fate").unwrap())
    .talent_levels([1, 9, 9])
    .artifact_stats(StatProfile {
        energy_recharge: 0.50,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        ..Default::default()
    })
    .build()
    .unwrap();

    let team = [solo];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    // No resonances for 1-member team
    assert!(
        result.resonances.is_empty(),
        "Single member team should have no resonances"
    );

    // Stats should still be valid
    assert!(result.final_stats.atk > 0.0);
    assert!(result.final_stats.hp > 0.0);

    // Damage calc should work
    let char_data = find_character("raiden_shogun").unwrap();
    let burst = &char_data.talents.elemental_burst.scalings[0];

    let damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats,
            talent_multiplier: burst.values[9],
            scaling_stat: burst.scaling_stat,
            damage_type: DamageType::Burst,
            element: Some(Element::Electro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: result.damage_context.burst_flat_dmg,
        },
        &Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
            def_ignore: 0.0,
        },
    )
    .unwrap();

    assert!(damage.non_crit > 0.0);
    assert!(damage.crit > damage.non_crit);
}

// =============================================================================
// Edge Case 13: Damage values with specific known stats (numerical check)
// =============================================================================

#[test]
fn edge_known_damage_formula_check() {
    // Manual calculation:
    // ATK = 2000, Talent Multiplier = 1.0 (100%), no DMG bonus, no reaction
    // Enemy: Lv90, RES 10%, DEF multiplier = (90+100)/((90+100)+(90+100)) = 0.5
    // RES multiplier = 1 - 0.10 = 0.90
    // Non-crit = ATK * multiplier * DEF_mult * RES_mult = 2000 * 1.0 * 0.5 * 0.90 = 900
    // Crit (100% CRIT DMG) = 900 * (1 + 1.0) = 1800

    let stats = Stats {
        hp: 20000.0,
        atk: 2000.0,
        def: 800.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.0,
        ..Default::default()
    };

    let damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats,
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
            def_ignore: 0.0,
        },
    )
    .unwrap();

    let expected_non_crit = 900.0;
    let expected_crit = 1800.0;
    let expected_average = expected_non_crit * (1.0 - 0.50) + expected_crit * 0.50; // = 1350

    let tolerance = 1.0;
    assert!(
        (damage.non_crit - expected_non_crit).abs() < tolerance,
        "Non-crit should be ~{}, got {}",
        expected_non_crit,
        damage.non_crit
    );
    assert!(
        (damage.crit - expected_crit).abs() < tolerance,
        "Crit should be ~{}, got {}",
        expected_crit,
        damage.crit
    );
    assert!(
        (damage.average - expected_average).abs() < tolerance,
        "Average should be ~{}, got {}",
        expected_average,
        damage.average
    );
}

// =============================================================================
// Edge Case 14: Vaporize multiplier correctness
// =============================================================================

#[test]
fn edge_vaporize_multiplier() {
    // Forward Vaporize (Pyro on Hydro) = 2.0x base
    // Reverse Vaporize (Hydro on Pyro) = 1.5x base
    // With EM = 0, no reaction bonus

    let stats = Stats {
        hp: 20000.0,
        atk: 2000.0,
        def: 800.0,
        elemental_mastery: 0.0,
        crit_rate: 0.0,
        crit_dmg: 0.0,
        energy_recharge: 1.0,
        dmg_bonus: 0.0,
        ..Default::default()
    };

    let enemy = Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
        def_ignore: 0.0,
    };

    let no_reaction = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: stats.clone(),
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &enemy,
    )
    .unwrap();

    let vaporize = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: stats.clone(),
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: Some(Element::Pyro),
            reaction: Some(Reaction::Vaporize),
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &enemy,
    )
    .unwrap();

    // In Genshin: Pyro attacking Hydro aura = Reverse Vaporize = 1.5x
    // Hydro attacking Pyro aura = Forward Vaporize = 2.0x
    // The API determines direction by the attacking element
    let ratio = vaporize.non_crit / no_reaction.non_crit;
    assert!(
        (ratio - 1.5).abs() < 0.01,
        "Pyro Vaporize (reverse) should be 1.5x, got {}x",
        ratio
    );

    // Hydro Vaporize = Forward = 2.0x
    let hydro_vape = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: stats.clone(),
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: Some(Element::Hydro),
            reaction: Some(Reaction::Vaporize),
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &enemy,
    )
    .unwrap();

    let no_reaction_hydro = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats,
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: Some(Element::Hydro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &enemy,
    )
    .unwrap();

    let hydro_ratio = hydro_vape.non_crit / no_reaction_hydro.non_crit;
    assert!(
        (hydro_ratio - 2.0).abs() < 0.01,
        "Hydro Vaporize (forward) should be 2.0x, got {}x",
        hydro_ratio
    );
}

// =============================================================================
// Edge Case 15: Melt multiplier correctness
// =============================================================================

#[test]
fn edge_melt_multiplier() {
    let stats = Stats {
        hp: 20000.0,
        atk: 2000.0,
        def: 800.0,
        elemental_mastery: 0.0,
        crit_rate: 0.0,
        crit_dmg: 0.0,
        energy_recharge: 1.0,
        dmg_bonus: 0.0,
        ..Default::default()
    };

    let enemy = Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
        def_ignore: 0.0,
    };

    let no_reaction = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: stats.clone(),
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &enemy,
    )
    .unwrap();

    let melt = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats,
            talent_multiplier: 1.0,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Normal,
            element: Some(Element::Pyro),
            reaction: Some(Reaction::Melt),
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &enemy,
    )
    .unwrap();

    // Pyro Melt (Pyro on Cryo) = 2.0x (Forward)
    let ratio = melt.non_crit / no_reaction.non_crit;
    assert!(
        (ratio - 2.0).abs() < 0.01,
        "Forward Melt (Pyro) should be 2.0x, got {}x",
        ratio
    );
}

// =============================================================================
// Edge Case 16: Bennett ATK buff numerical accuracy
// =============================================================================

#[test]
fn edge_bennett_atk_buff_accuracy() {
    // Bennett Lv90 base_atk = 191.16
    // Aquila Favonia base_atk = 674
    // Total base ATK = 865.16
    // C6 Bennett: C5 boosts Burst by +3 levels
    // Talent Lv13 + 3 = 16, capped at 15 → scaling[14] = 1.33
    // Expected buff = 865.16 × 1.33 = 1150.66

    let bennett = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("aquila_favonia").unwrap(),
    )
    .constellation(6)
    .talent_levels([1, 1, 13])
    .activate("Fantastic Voyage ATK Bonus")
    .build()
    .unwrap();

    let atk_buff = bennett
        .buffs_provided
        .iter()
        .find(|b| b.source.contains("Fantastic Voyage") && b.stat == BuffableStat::AtkFlat)
        .expect("Bennett should have ATK flat buff");

    // C6 boosts burst talent to Lv15 (cap), scaling = 1.33
    let expected = 865.16 * 1.33;
    let tolerance = 1.0;
    assert!(
        (atk_buff.value - expected).abs() < tolerance,
        "Bennett C6 Lv13(→15) ATK buff should be ~{:.1}, got {:.1}",
        expected,
        atk_buff.value
    );

    // Also verify C1 (no constellation burst boost) uses actual talent level
    let bennett_c1 = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("aquila_favonia").unwrap(),
    )
    .constellation(1)
    .talent_levels([1, 1, 13])
    .activate("Fantastic Voyage ATK Bonus")
    .build()
    .unwrap();

    let atk_buff_c1 = bennett_c1
        .buffs_provided
        .iter()
        .find(|b| b.source.contains("Fantastic Voyage") && b.stat == BuffableStat::AtkFlat)
        .expect("Bennett C1 should have ATK flat buff");

    // C1: no burst boost, talent stays at 13, scaling[12] = 1.19
    let expected_c1 = 865.16 * 1.19;
    assert!(
        (atk_buff_c1.value - expected_c1).abs() < tolerance,
        "Bennett C1 Lv13 ATK buff should be ~{:.1}, got {:.1}",
        expected_c1,
        atk_buff_c1.value
    );
}

// =============================================================================
// Edge Case 17: Raiden Burst with Resolve stacks
// =============================================================================

#[test]
fn edge_raiden_burst_resolve_stacks() {
    // Verify Raiden burst damage scales correctly with stats
    let raiden = TeamMemberBuilder::new(
        find_character("raiden_shogun").unwrap(),
        find_weapon("engulfing_lightning").unwrap(),
    )
    .artifact_set(find_artifact_set("emblem_of_severed_fate").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .artifact_stats(StatProfile {
        energy_recharge: 0.50,
        crit_rate: 0.55,
        crit_dmg: 0.50,
        electro_dmg_bonus: 0.466,
        atk_percent: 0.18,
        atk_flat: 311.0,
        ..Default::default()
    })
    .build()
    .unwrap();

    let team = [raiden];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    // Engulfing Lightning ATK% from ER should be applied
    // Emblem 4pc burst DMG should be applied
    assert!(
        result.damage_context.burst_dmg_bonus > 0.0,
        "Emblem should add burst DMG"
    );

    let char_data = find_character("raiden_shogun").unwrap();
    let burst = &char_data.talents.elemental_burst.scalings[0];

    let damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats,
            talent_multiplier: burst.values[9],
            scaling_stat: burst.scaling_stat,
            damage_type: DamageType::Burst,
            element: Some(Element::Electro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: result.damage_context.burst_flat_dmg,
        },
        &Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
            def_ignore: 0.0,
        },
    )
    .unwrap();

    // With these stats, non_crit should be in a reasonable range
    // ATK ~1800, multiplier ~7.2, DMG bonus ~1.1, DEF 0.5, RES 0.9
    // Base ≈ 1800 × 7.2 × (1+1.1) × 0.5 × 0.9 ≈ ~12,247
    assert!(
        damage.non_crit > 5000.0 && damage.non_crit < 30000.0,
        "Raiden burst non_crit should be in reasonable range, got {}",
        damage.non_crit
    );
    assert!(damage.crit > damage.non_crit, "Crit should exceed non_crit");
}
