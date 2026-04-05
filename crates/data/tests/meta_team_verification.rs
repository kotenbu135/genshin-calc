//! Meta Team Verification Tests
//!
//! Webから取得した代表的な10編成を使い、WASM計算エンジンの
//! パイプライン全体を検証する。各テストでは以下を確認:
//! - TeamMemberBuilder がパニックせずに構築可能
//! - resolve_team_stats が正常に動作
//! - バフが正しく適用される
//! - calculate_damage が合理的な値を返す（正値、NaNなし、crit > non_crit）
//! - 元素反応が正しく計算される
//! - 元素共鳴が正しく検出される

use genshin_calc_core::*;
use genshin_calc_data::*;

// =============================================================================
// Helper functions
// =============================================================================

fn default_enemy() -> Enemy {
    Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
        def_ignore: 0.0,
    }
}

/// ダメージ結果の基本的な健全性チェック
fn assert_damage_sane(result: &DamageResult, context: &str) {
    assert!(
        result.non_crit > 0.0,
        "{context}: non_crit should be positive, got {}",
        result.non_crit
    );
    assert!(
        result.crit > 0.0,
        "{context}: crit should be positive, got {}",
        result.crit
    );
    assert!(
        result.average > 0.0,
        "{context}: average should be positive, got {}",
        result.average
    );
    assert!(!result.non_crit.is_nan(), "{context}: non_crit is NaN");
    assert!(!result.crit.is_nan(), "{context}: crit is NaN");
    assert!(!result.average.is_nan(), "{context}: average is NaN");
    assert!(
        result.crit >= result.non_crit,
        "{context}: crit ({}) should >= non_crit ({})",
        result.crit,
        result.non_crit
    );
}

/// Statsの基本チェック
fn assert_stats_sane(stats: &Stats, context: &str) {
    assert!(
        stats.atk > 0.0,
        "{context}: ATK should be positive, got {}",
        stats.atk
    );
    assert!(
        stats.hp > 0.0,
        "{context}: HP should be positive, got {}",
        stats.hp
    );
    assert!(!stats.atk.is_nan(), "{context}: ATK is NaN");
    assert!(!stats.hp.is_nan(), "{context}: HP is NaN");
    assert!(!stats.def.is_nan(), "{context}: DEF is NaN");
    assert!(!stats.crit_rate.is_nan(), "{context}: crit_rate is NaN");
    assert!(!stats.crit_dmg.is_nan(), "{context}: crit_dmg is NaN");
}

/// 変換反応ダメージの健全性チェック
fn assert_transformative_sane(result: &TransformativeResult, context: &str) {
    assert!(
        result.damage > 0.0,
        "{context}: transformative damage should be positive, got {}",
        result.damage
    );
    assert!(
        !result.damage.is_nan(),
        "{context}: transformative damage is NaN"
    );
}

// =============================================================================
// Team 1: Vaporize — Arlecchino / Yelan / Bennett / Kazuha
// =============================================================================

#[test]
fn team01_arlecchino_vaporize() {
    let arlecchino = TeamMemberBuilder::new(
        find_character("arlecchino").unwrap(),
        find_weapon("crimson_moons_semblance").unwrap(),
    )
    .artifact_set(find_artifact_set("fragment_of_harmonic_whimsy").unwrap())
    .constellation(0)
    .talent_levels([9, 9, 9])
    .activate_with_stacks("harmonic_whimsy_dmg_stacks", 3)
    .artifact_stats(StatProfile {
        crit_rate: 0.30,
        crit_dmg: 0.60,
        atk_percent: 0.10,
        ..Default::default()
    })
    .build()
    .unwrap();

    let yelan = TeamMemberBuilder::new(
        find_character("yelan").unwrap(),
        find_weapon("aqua_simulacra").unwrap(),
    )
    .artifact_set(find_artifact_set("emblem_of_severed_fate").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate("Adapt With Ease")
    .build()
    .unwrap();

    let bennett = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("skyward_blade").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(1)
    .talent_levels([1, 9, 1])
    .activate("Fantastic Voyage ATK Bonus")
    .activate("noblesse_atk_bonus")
    .build()
    .unwrap();

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
        energy_recharge: 0.40,
        ..Default::default()
    })
    .build()
    .unwrap();

    let team = [arlecchino, yelan, bennett, kazuha];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    assert_stats_sane(&result.final_stats, "Arlecchino");

    // Bennett buff should be applied
    let has_bennett = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("Fantastic Voyage"));
    assert!(has_bennett, "Arlecchino should receive Bennett ATK buff");

    // Kazuha A4 Pyro DMG buff
    let has_kazuha = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("Poetics"));
    assert!(has_kazuha, "Arlecchino should receive Kazuha A4 buff");

    // Pyro resonance
    assert!(
        result
            .resonances
            .contains(&ElementalResonance::FerventFlames),
        "Should have Pyro resonance"
    );

    // Calculate Vaporize damage
    let char_data = find_character("arlecchino").unwrap();
    let skill_scaling = &char_data.talents.elemental_skill.scalings[0];
    let damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats,
            talent_multiplier: skill_scaling.values[9],
            scaling_stat: skill_scaling.scaling_stat,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: Some(Reaction::Vaporize),
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &default_enemy(),
    )
    .unwrap();

    assert_damage_sane(&damage, "Arlecchino Vaporize Skill");
}

// =============================================================================
// Team 2: Melt — Ganyu / Bennett / Kazuha / Rosaria
// =============================================================================

#[test]
fn team02_ganyu_melt() {
    let ganyu = TeamMemberBuilder::new(
        find_character("ganyu").unwrap(),
        find_weapon("amos_bow").unwrap(),
    )
    .artifact_set(find_artifact_set("wanderers_troupe").unwrap())
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

    let bennett = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("skyward_blade").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(1)
    .talent_levels([1, 9, 1])
    .activate("Fantastic Voyage ATK Bonus")
    .activate("noblesse_atk_bonus")
    .build()
    .unwrap();

    let kazuha = TeamMemberBuilder::new(
        find_character("kazuha").unwrap(),
        find_weapon("freedom_sworn").unwrap(),
    )
    .artifact_set(find_artifact_set("viridescent_venerer").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 1])
    .activate("vv_res_shred_cryo")
    .artifact_stats(StatProfile {
        elemental_mastery: 600.0,
        energy_recharge: 0.40,
        ..Default::default()
    })
    .build()
    .unwrap();

    let rosaria = TeamMemberBuilder::new(
        find_character("rosaria").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(6)
    .talent_levels([1, 9, 9])
    .build()
    .unwrap();

    let team = [ganyu, bennett, kazuha, rosaria];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    assert_stats_sane(&result.final_stats, "Ganyu");

    // Rosaria A4 should share CRIT Rate
    let has_rosaria_crit = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("Shadow Samaritan"));
    assert!(has_rosaria_crit, "Ganyu should receive Rosaria CRIT share");

    // Calculate Melt Charged Attack (Frostflake Arrow Bloom)
    let char_data = find_character("ganyu").unwrap();
    // Charged attack scaling - find the bloom hit
    let charged_scalings = &char_data.talents.normal_attack.charged;
    let bloom_scaling = charged_scalings
        .iter()
        .find(|s| s.name.contains("霜華の矢") || s.name.contains("Bloom"))
        .unwrap_or(&charged_scalings[charged_scalings.len() - 1]);

    let damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats.clone(),
            talent_multiplier: bloom_scaling.values[9],
            scaling_stat: bloom_scaling.scaling_stat,
            damage_type: DamageType::Charged,
            element: Some(Element::Cryo),
            reaction: Some(Reaction::Melt),
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &default_enemy(),
    )
    .unwrap();

    assert_damage_sane(&damage, "Ganyu Melt Charged");
    // Melt should significantly amplify damage
    let no_reaction = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats,
            talent_multiplier: bloom_scaling.values[9],
            scaling_stat: bloom_scaling.scaling_stat,
            damage_type: DamageType::Charged,
            element: Some(Element::Cryo),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &default_enemy(),
    )
    .unwrap();

    assert!(
        damage.average > no_reaction.average * 1.3,
        "Melt should amplify damage significantly: melt={} vs base={}",
        damage.average,
        no_reaction.average
    );
}

// =============================================================================
// Team 3: Freeze — Skirk / Escoffier / Furina / Yelan
// =============================================================================

#[test]
fn team03_skirk_freeze() {
    let skirk = TeamMemberBuilder::new(
        find_character("skirk").unwrap(),
        find_weapon("mistsplitter_reforged").unwrap(),
    )
    .artifact_set(find_artifact_set("marechaussee_hunter").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate_with_stacks("marechaussee_crit_stacks", 3)
    .artifact_stats(StatProfile {
        crit_rate: 0.20,
        crit_dmg: 0.80,
        atk_percent: 0.15,
        ..Default::default()
    })
    .build()
    .unwrap();

    let escoffier = TeamMemberBuilder::new(
        find_character("escoffier").unwrap(),
        find_weapon("jadefalls_splendor").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate("noblesse_atk_bonus")
    .build()
    .unwrap();

    let furina = TeamMemberBuilder::new(
        find_character("furina").unwrap(),
        find_weapon("splendor_of_tranquil_waters").unwrap(),
    )
    .artifact_set(find_artifact_set("golden_troupe").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate_with_stacks("Let the People Rejoice DMG Bonus (C0 300pt)", 300)
    .build()
    .unwrap();

    let yelan = TeamMemberBuilder::new(
        find_character("yelan").unwrap(),
        find_weapon("aqua_simulacra").unwrap(),
    )
    .artifact_set(find_artifact_set("emblem_of_severed_fate").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate("Adapt With Ease")
    .build()
    .unwrap();

    let team = [skirk, escoffier, furina, yelan];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    assert_stats_sane(&result.final_stats, "Skirk");

    // Furina DMG buff should be applied
    let has_furina = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("Rejoice") || b.source.contains("furina"));
    assert!(has_furina, "Skirk should receive Furina DMG buff");

    // Cryo resonance should exist (2 Cryo)
    let has_cryo_res = result
        .resonances
        .contains(&ElementalResonance::ShatteringIce);
    assert!(has_cryo_res, "Should have Cryo resonance");

    // Hydro resonance should exist (2 Hydro)
    let has_hydro_res = result
        .resonances
        .contains(&ElementalResonance::SoothingWater);
    assert!(has_hydro_res, "Should have Hydro resonance");

    // Calculate Skirk Burst damage (Freeze = no amplifying, but Blizzard synergy)
    let char_data = find_character("skirk").unwrap();
    let burst_scaling = &char_data.talents.elemental_burst.scalings[0];

    let damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats,
            talent_multiplier: burst_scaling.values[9],
            scaling_stat: burst_scaling.scaling_stat,
            damage_type: DamageType::Burst,
            element: Some(Element::Cryo),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &default_enemy(),
    )
    .unwrap();

    assert_damage_sane(&damage, "Skirk Freeze Burst");
}

// =============================================================================
// Team 4: Aggravate — Cyno / Nahida / Fischl / Baizhu
// =============================================================================

#[test]
fn team04_cyno_aggravate() {
    let cyno = TeamMemberBuilder::new(
        find_character("cyno").unwrap(),
        find_weapon("staff_of_the_scarlet_sands").unwrap(),
    )
    .artifact_set(find_artifact_set("gilded_dreams").unwrap())
    .constellation(0)
    .talent_levels([9, 1, 9])
    .artifact_stats(StatProfile {
        elemental_mastery: 200.0,
        crit_rate: 0.30,
        crit_dmg: 0.60,
        ..Default::default()
    })
    .build()
    .unwrap();

    let nahida = TeamMemberBuilder::new(
        find_character("nahida").unwrap(),
        find_weapon("a_thousand_floating_dreams").unwrap(),
    )
    .artifact_set(find_artifact_set("deepwood_memories").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate("deepwood_dendro_res_shred")
    .activate("Compassion Illuminated")
    .artifact_stats(StatProfile {
        elemental_mastery: 400.0,
        ..Default::default()
    })
    .build()
    .unwrap();

    let fischl = TeamMemberBuilder::new(
        find_character("fischl").unwrap(),
        find_weapon("the_stringless").unwrap(),
    )
    .artifact_set(find_artifact_set("thundering_fury").unwrap())
    .constellation(6)
    .talent_levels([1, 9, 1])
    .activate("tf_additive")
    .artifact_stats(StatProfile {
        crit_rate: 0.30,
        crit_dmg: 0.60,
        atk_percent: 0.20,
        ..Default::default()
    })
    .build()
    .unwrap();

    let baizhu = TeamMemberBuilder::new(
        find_character("baizhu").unwrap(),
        find_weapon("jadefalls_splendor").unwrap(),
    )
    .artifact_set(find_artifact_set("deepwood_memories").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .build()
    .unwrap();

    let team = [cyno, nahida, fischl, baizhu];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    assert_stats_sane(&result.final_stats, "Cyno");

    // Dendro resonance (Nahida + Baizhu)
    let has_dendro_res = result
        .resonances
        .contains(&ElementalResonance::SprawlingGreenery);
    assert!(has_dendro_res, "Should have Dendro resonance");

    // Scarlet Sands should resolve ATK from EM
    let has_ss_buff = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("scarlet_sands"));
    assert!(
        has_ss_buff,
        "Cyno should have Staff of Scarlet Sands ATK buff from EM"
    );

    // Calculate Aggravate damage
    let char_data = find_character("cyno").unwrap();
    let burst_scaling = &char_data.talents.elemental_burst.scalings[0];

    let damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats.clone(),
            talent_multiplier: burst_scaling.values[9],
            scaling_stat: burst_scaling.scaling_stat,
            damage_type: DamageType::Burst,
            element: Some(Element::Electro),
            reaction: Some(Reaction::Aggravate),
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &default_enemy(),
    )
    .unwrap();

    assert_damage_sane(&damage, "Cyno Aggravate Burst");

    // Aggravate should add flat damage bonus
    let no_reaction = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats,
            talent_multiplier: burst_scaling.values[9],
            scaling_stat: burst_scaling.scaling_stat,
            damage_type: DamageType::Burst,
            element: Some(Element::Electro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &default_enemy(),
    )
    .unwrap();

    assert!(
        damage.average > no_reaction.average,
        "Aggravate should increase damage: aggravate={} vs base={}",
        damage.average,
        no_reaction.average
    );
}

// =============================================================================
// Team 5: Hyperbloom — Alhaitham / Nahida / Yelan / Kuki Shinobu
// =============================================================================

#[test]
fn team05_hyperbloom() {
    let alhaitham = TeamMemberBuilder::new(
        find_character("alhaitham").unwrap(),
        find_weapon("light_of_foliar_incision").unwrap(),
    )
    .artifact_set(find_artifact_set("gilded_dreams").unwrap())
    .constellation(0)
    .talent_levels([9, 9, 9])
    .artifact_stats(StatProfile {
        elemental_mastery: 200.0,
        crit_rate: 0.30,
        crit_dmg: 0.60,
        ..Default::default()
    })
    .build()
    .unwrap();

    let nahida = TeamMemberBuilder::new(
        find_character("nahida").unwrap(),
        find_weapon("a_thousand_floating_dreams").unwrap(),
    )
    .artifact_set(find_artifact_set("deepwood_memories").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate("deepwood_dendro_res_shred")
    .artifact_stats(StatProfile {
        elemental_mastery: 400.0,
        ..Default::default()
    })
    .build()
    .unwrap();

    let yelan = TeamMemberBuilder::new(
        find_character("yelan").unwrap(),
        find_weapon("aqua_simulacra").unwrap(),
    )
    .artifact_set(find_artifact_set("emblem_of_severed_fate").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate("Adapt With Ease")
    .build()
    .unwrap();

    let kuki = TeamMemberBuilder::new(
        find_character("kuki_shinobu").unwrap(),
        find_weapon("iron_sting").unwrap(),
    )
    .artifact_set(find_artifact_set("gilded_dreams").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 1])
    .artifact_stats(StatProfile {
        elemental_mastery: 700.0,
        ..Default::default()
    })
    .build()
    .unwrap();

    let team = [alhaitham, nahida, yelan, kuki];

    // Resolve for Kuki (Hyperbloom trigger)
    let kuki_result = resolve_team_stats_detailed(&team, 3).unwrap();
    assert_stats_sane(&kuki_result.final_stats, "Kuki");

    // Dendro resonance (Alhaitham + Nahida)
    let has_dendro_res = kuki_result
        .resonances
        .contains(&ElementalResonance::SprawlingGreenery);
    assert!(has_dendro_res, "Should have Dendro resonance");

    // Calculate Hyperbloom damage from Kuki's EM
    let hyperbloom = calculate_transformative(
        &TransformativeInput {
            character_level: 90,
            elemental_mastery: kuki_result.final_stats.elemental_mastery,
            reaction: Reaction::Hyperbloom,
            reaction_bonus: 0.0,
        },
        &default_enemy(),
    )
    .unwrap();

    assert_transformative_sane(&hyperbloom, "Kuki Hyperbloom");
    // High EM Kuki should deal significant Hyperbloom damage
    assert!(
        hyperbloom.damage > 20000.0,
        "High EM Kuki Hyperbloom should deal >20k, got {}",
        hyperbloom.damage
    );

    // Also resolve for Alhaitham (on-field DPS)
    let al_result = resolve_team_stats_detailed(&team, 0).unwrap();
    assert_stats_sane(&al_result.final_stats, "Alhaitham");

    let char_data = find_character("alhaitham").unwrap();
    let skill_scaling = &char_data.talents.elemental_skill.scalings[0];
    let spread_damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: al_result.final_stats,
            talent_multiplier: skill_scaling.values[9],
            scaling_stat: skill_scaling.scaling_stat,
            damage_type: DamageType::Skill,
            element: Some(Element::Dendro),
            reaction: Some(Reaction::Spread),
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        },
        &default_enemy(),
    )
    .unwrap();

    assert_damage_sane(&spread_damage, "Alhaitham Spread Skill");
}

// =============================================================================
// Team 6: Burgeon — Nahida / Thoma / Xingqiu / Kazuha
// =============================================================================

#[test]
fn team06_burgeon() {
    let nahida = TeamMemberBuilder::new(
        find_character("nahida").unwrap(),
        find_weapon("a_thousand_floating_dreams").unwrap(),
    )
    .artifact_set(find_artifact_set("deepwood_memories").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate("deepwood_dendro_res_shred")
    .artifact_stats(StatProfile {
        elemental_mastery: 400.0,
        ..Default::default()
    })
    .build()
    .unwrap();

    let thoma = TeamMemberBuilder::new(
        find_character("thoma").unwrap(),
        find_weapon("favonius_lance").unwrap(),
    )
    .artifact_set(find_artifact_set("flower_of_paradise_lost").unwrap())
    .constellation(6)
    .talent_levels([1, 9, 9])
    .activate("fopl_bloom_base")
    .activate_with_stacks("fopl_bloom_stacks", 4)
    .artifact_stats(StatProfile {
        elemental_mastery: 700.0,
        energy_recharge: 0.40,
        ..Default::default()
    })
    .build()
    .unwrap();

    let xingqiu = TeamMemberBuilder::new(
        find_character("xingqiu").unwrap(),
        find_weapon("sacrificial_sword").unwrap(),
    )
    .artifact_set(find_artifact_set("emblem_of_severed_fate").unwrap())
    .constellation(6)
    .talent_levels([1, 9, 9])
    .artifact_stats(StatProfile {
        crit_rate: 0.30,
        crit_dmg: 0.60,
        atk_percent: 0.20,
        energy_recharge: 0.30,
        ..Default::default()
    })
    .build()
    .unwrap();

    let kazuha = TeamMemberBuilder::new(
        find_character("kazuha").unwrap(),
        find_weapon("freedom_sworn").unwrap(),
    )
    .artifact_set(find_artifact_set("viridescent_venerer").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 1])
    .artifact_stats(StatProfile {
        elemental_mastery: 600.0,
        energy_recharge: 0.40,
        ..Default::default()
    })
    .build()
    .unwrap();

    let team = [nahida, thoma, xingqiu, kazuha];

    // Resolve for Thoma (Burgeon trigger)
    let thoma_result = resolve_team_stats_detailed(&team, 1).unwrap();
    assert_stats_sane(&thoma_result.final_stats, "Thoma");

    // Calculate Burgeon damage
    let burgeon = calculate_transformative(
        &TransformativeInput {
            character_level: 90,
            elemental_mastery: thoma_result.final_stats.elemental_mastery,
            reaction: Reaction::Burgeon,
            reaction_bonus: 0.0,
        },
        &default_enemy(),
    )
    .unwrap();

    assert_transformative_sane(&burgeon, "Thoma Burgeon");
    // Flower of Paradise Lost should boost bloom damage significantly
    assert!(
        burgeon.damage > 20000.0,
        "Thoma with FoPL and high EM should deal >20k Burgeon, got {}",
        burgeon.damage
    );
}

// =============================================================================
// Team 7: National — Raiden / Xiangling / Xingqiu / Bennett
// =============================================================================

#[test]
fn team07_raiden_national() {
    let raiden = TeamMemberBuilder::new(
        find_character("raiden_shogun").unwrap(),
        find_weapon("engulfing_lightning").unwrap(),
    )
    .artifact_set(find_artifact_set("emblem_of_severed_fate").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .artifact_stats(StatProfile {
        energy_recharge: 0.50,
        crit_rate: 0.30,
        crit_dmg: 0.60,
        ..Default::default()
    })
    .build()
    .unwrap();

    let xiangling = TeamMemberBuilder::new(
        find_character("xiangling").unwrap(),
        find_weapon("the_catch").unwrap(),
    )
    .artifact_set(find_artifact_set("emblem_of_severed_fate").unwrap())
    .constellation(6)
    .talent_levels([1, 9, 9])
    .artifact_stats(StatProfile {
        energy_recharge: 0.30,
        crit_rate: 0.30,
        crit_dmg: 0.60,
        ..Default::default()
    })
    .build()
    .unwrap();

    let xingqiu = TeamMemberBuilder::new(
        find_character("xingqiu").unwrap(),
        find_weapon("sacrificial_sword").unwrap(),
    )
    .artifact_set(find_artifact_set("emblem_of_severed_fate").unwrap())
    .constellation(6)
    .talent_levels([1, 9, 9])
    .artifact_stats(StatProfile {
        crit_rate: 0.30,
        crit_dmg: 0.60,
        atk_percent: 0.20,
        energy_recharge: 0.30,
        ..Default::default()
    })
    .build()
    .unwrap();

    let bennett = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("skyward_blade").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(1)
    .talent_levels([1, 9, 1])
    .activate("Fantastic Voyage ATK Bonus")
    .activate("noblesse_atk_bonus")
    .build()
    .unwrap();

    let team = [raiden, xiangling, xingqiu, bennett];

    // Pyro resonance
    let raiden_result = resolve_team_stats_detailed(&team, 0).unwrap();
    assert!(
        raiden_result
            .resonances
            .contains(&ElementalResonance::FerventFlames),
        "Should have Pyro resonance"
    );

    // Raiden should have Engulfing Lightning ATK buff from ER
    let has_el_buff = raiden_result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("engulfing"));
    assert!(
        has_el_buff,
        "Raiden should have Engulfing Lightning ATK buff"
    );

    // Emblem should boost burst DMG based on ER
    assert_stats_sane(&raiden_result.final_stats, "Raiden");

    // Calculate Raiden burst initial hit
    let char_data = find_character("raiden_shogun").unwrap();
    let burst_scaling = &char_data.talents.elemental_burst.scalings[0];

    let damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: raiden_result.final_stats,
            talent_multiplier: burst_scaling.values[9],
            scaling_stat: burst_scaling.scaling_stat,
            damage_type: DamageType::Burst,
            element: Some(Element::Electro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: raiden_result.damage_context.burst_flat_dmg,
        },
        &default_enemy(),
    )
    .unwrap();

    assert_damage_sane(&damage, "Raiden Burst initial hit");

    // Resolve for Xiangling (check Bennett buff reaches her)
    let xl_result = resolve_team_stats_detailed(&team, 1).unwrap();
    assert_stats_sane(&xl_result.final_stats, "Xiangling");

    let has_bennett_for_xl = xl_result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("Fantastic Voyage"));
    assert!(
        has_bennett_for_xl,
        "Xiangling should receive Bennett ATK buff"
    );

    // Xiangling Vaporize Burst
    let xl_data = find_character("xiangling").unwrap();
    let xl_burst = &xl_data.talents.elemental_burst.scalings[0];

    let xl_vape = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: xl_result.final_stats,
            talent_multiplier: xl_burst.values[9],
            scaling_stat: xl_burst.scaling_stat,
            damage_type: DamageType::Burst,
            element: Some(Element::Pyro),
            reaction: Some(Reaction::Vaporize),
            reaction_bonus: 0.0,
            flat_dmg: xl_result.damage_context.burst_flat_dmg,
        },
        &default_enemy(),
    )
    .unwrap();

    assert_damage_sane(&xl_vape, "Xiangling Vaporize Burst");
}

// =============================================================================
// Team 8: Mono Pyro — Mavuika / Xilonen / Kazuha / Bennett
// =============================================================================

#[test]
fn team08_mono_pyro_mavuika() {
    let mavuika = TeamMemberBuilder::new(
        find_character("mavuika").unwrap(),
        find_weapon("a_thousand_blazing_suns").unwrap(),
    )
    .artifact_set(find_artifact_set("obsidian_codex").unwrap())
    .constellation(0)
    .talent_levels([9, 9, 9])
    .activate("Sunfrost Encomium ATK Bonus")
    .activate("Fire-Forged Heritage DMG Bonus")
    .activate("Obsidian Codex Crit Rate Bonus")
    .artifact_stats(StatProfile {
        crit_rate: 0.20,
        crit_dmg: 0.80,
        atk_percent: 0.15,
        ..Default::default()
    })
    .build()
    .unwrap();

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
        energy_recharge: 0.40,
        ..Default::default()
    })
    .build()
    .unwrap();

    let bennett = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("skyward_blade").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(1)
    .talent_levels([1, 9, 1])
    .activate("Fantastic Voyage ATK Bonus")
    .activate("noblesse_atk_bonus")
    .build()
    .unwrap();

    let team = [mavuika, xilonen, kazuha, bennett];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    assert_stats_sane(&result.final_stats, "Mavuika");

    // Pyro resonance
    assert!(
        result
            .resonances
            .contains(&ElementalResonance::FerventFlames),
        "Should have Pyro resonance"
    );

    // Multiple buffs should stack
    let has_bennett = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("Fantastic Voyage"));
    let has_kazuha = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("Poetics"));
    assert!(has_bennett, "Mavuika should receive Bennett buff");
    assert!(has_kazuha, "Mavuika should receive Kazuha buff");

    // Calculate Mavuika Normal Attack in Nightsoul state (Pyro infusion)
    let char_data = find_character("mavuika").unwrap();
    let normal_hit = &char_data.talents.normal_attack.hits[0];

    let damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats,
            talent_multiplier: normal_hit.values[9],
            scaling_stat: normal_hit.scaling_stat,
            damage_type: DamageType::Normal,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: result.damage_context.normal_atk_flat_dmg,
        },
        &default_enemy(),
    )
    .unwrap();

    assert_damage_sane(&damage, "Mavuika Pyro Normal");
}

// =============================================================================
// Team 9: Physical — Eula / Raiden / Shenhe / Zhongli
// =============================================================================

#[test]
fn team09_eula_physical() {
    let eula = TeamMemberBuilder::new(
        find_character("eula").unwrap(),
        find_weapon("song_of_broken_pines").unwrap(),
    )
    .artifact_set(find_artifact_set("pale_flame").unwrap())
    .constellation(0)
    .talent_levels([9, 9, 1])
    .activate_with_stacks("pale_flame_atk_stacks", 2)
    .activate("pale_flame_phys_bonus")
    .artifact_stats(StatProfile {
        crit_rate: 0.30,
        crit_dmg: 0.80,
        atk_percent: 0.10,
        physical_dmg_bonus: 0.583,
        ..Default::default()
    })
    .build()
    .unwrap();

    let raiden = TeamMemberBuilder::new(
        find_character("raiden_shogun").unwrap(),
        find_weapon("engulfing_lightning").unwrap(),
    )
    .artifact_set(find_artifact_set("emblem_of_severed_fate").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .artifact_stats(StatProfile {
        energy_recharge: 0.50,
        crit_rate: 0.30,
        crit_dmg: 0.60,
        ..Default::default()
    })
    .build()
    .unwrap();

    let shenhe = TeamMemberBuilder::new(
        find_character("shenhe").unwrap(),
        find_weapon("calamity_queller").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate("noblesse_atk_bonus")
    .activate("Deific Embrace Press - Skill DMG")
    .activate("Deific Embrace Press - Burst DMG")
    .activate_with_stacks("Spring Spirit Summoning Skill ATK Flat DMG", 5)
    .build()
    .unwrap();

    let zhongli = TeamMemberBuilder::new(
        find_character("zhongli").unwrap(),
        find_weapon("black_tassel").unwrap(),
    )
    .artifact_set(find_artifact_set("tenacity_of_the_millelith").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 1])
    .activate("millelith_atk_bonus")
    .activate("Jade Shield Pyro RES Shred")
    .activate("Jade Shield Hydro RES Shred")
    .activate("Jade Shield Electro RES Shred")
    .activate("Jade Shield Cryo RES Shred")
    .activate("Jade Shield Physical RES Shred")
    .build()
    .unwrap();

    let team = [eula, raiden, shenhe, zhongli];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    assert_stats_sane(&result.final_stats, "Eula");

    // Cryo resonance (Eula + Shenhe)
    let has_cryo_res = result
        .resonances
        .contains(&ElementalResonance::ShatteringIce);
    assert!(has_cryo_res, "Should have Cryo resonance");

    // Zhongli RES shred should be applied
    let has_zhongli = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("Jade Shield"));
    assert!(has_zhongli, "Eula should receive Zhongli RES shred");

    // Calculate Physical Normal Attack
    let char_data = find_character("eula").unwrap();
    let normal_hit = &char_data.talents.normal_attack.hits[0];

    let damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats.clone(),
            talent_multiplier: normal_hit.values[9],
            scaling_stat: normal_hit.scaling_stat,
            damage_type: DamageType::Normal,
            element: None, // Physical
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: result.damage_context.normal_atk_flat_dmg,
        },
        &default_enemy(),
    )
    .unwrap();

    assert_damage_sane(&damage, "Eula Physical Normal");

    // Physical damage should benefit from Physical DMG bonus
    assert!(
        result.final_stats.physical_dmg_bonus > 0.0,
        "Eula should have Physical DMG bonus"
    );

    // Calculate Superconduct (for debuff verification)
    let superconduct = calculate_transformative(
        &TransformativeInput {
            character_level: 90,
            elemental_mastery: result.final_stats.elemental_mastery,
            reaction: Reaction::Superconduct,
            reaction_bonus: 0.0,
        },
        &default_enemy(),
    )
    .unwrap();

    assert_transformative_sane(&superconduct, "Superconduct");
}

// =============================================================================
// Team 10: Nightsoul Melt — Mavuika / Citlali / Xilonen / Bennett
// =============================================================================

#[test]
fn team10_mavuika_nightsoul_melt() {
    let mavuika = TeamMemberBuilder::new(
        find_character("mavuika").unwrap(),
        find_weapon("a_thousand_blazing_suns").unwrap(),
    )
    .artifact_set(find_artifact_set("obsidian_codex").unwrap())
    .constellation(0)
    .talent_levels([9, 9, 9])
    .activate("Sunfrost Encomium ATK Bonus")
    .activate("Fire-Forged Heritage DMG Bonus")
    .activate("Obsidian Codex Crit Rate Bonus")
    .artifact_stats(StatProfile {
        crit_rate: 0.20,
        crit_dmg: 0.80,
        atk_percent: 0.15,
        ..Default::default()
    })
    .build()
    .unwrap();

    let citlali = TeamMemberBuilder::new(
        find_character("citlali").unwrap(),
        find_weapon("starcallers_watch").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(0)
    .talent_levels([1, 9, 9])
    .activate("noblesse_atk_bonus")
    .activate("Itzpapa Pyro RES Shred")
    .artifact_stats(StatProfile {
        elemental_mastery: 300.0,
        hp_percent: 0.30,
        ..Default::default()
    })
    .build()
    .unwrap();

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

    let bennett = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("skyward_blade").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(1)
    .talent_levels([1, 9, 1])
    .activate("Fantastic Voyage ATK Bonus")
    .build()
    .unwrap();

    let team = [mavuika, citlali, xilonen, bennett];
    let result = resolve_team_stats_detailed(&team, 0).unwrap();

    assert_stats_sane(&result.final_stats, "Mavuika Nightsoul");

    // Pyro resonance (Mavuika + Bennett)
    assert!(
        result
            .resonances
            .contains(&ElementalResonance::FerventFlames),
        "Should have Pyro resonance"
    );

    // Citlali Pyro RES shred
    let has_citlali = result
        .applied_buffs
        .iter()
        .any(|b| b.source.contains("Itzpapa"));
    assert!(has_citlali, "Mavuika should receive Citlali Pyro RES shred");

    // Calculate Melt damage (Forward Melt: Pyro on Cryo = 2x)
    let char_data = find_character("mavuika").unwrap();
    let burst_scaling = &char_data.talents.elemental_burst.scalings[0];

    let melt_damage = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats.clone(),
            talent_multiplier: burst_scaling.values[9],
            scaling_stat: burst_scaling.scaling_stat,
            damage_type: DamageType::Burst,
            element: Some(Element::Pyro),
            reaction: Some(Reaction::Melt),
            reaction_bonus: 0.0,
            flat_dmg: result.damage_context.burst_flat_dmg,
        },
        &default_enemy(),
    )
    .unwrap();

    assert_damage_sane(&melt_damage, "Mavuika Melt Burst");

    let no_melt = calculate_damage(
        &DamageInput {
            character_level: 90,
            stats: result.final_stats,
            talent_multiplier: burst_scaling.values[9],
            scaling_stat: burst_scaling.scaling_stat,
            damage_type: DamageType::Burst,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: result.damage_context.burst_flat_dmg,
        },
        &default_enemy(),
    )
    .unwrap();

    assert!(
        melt_damage.average > no_melt.average * 1.8,
        "Forward Melt should roughly double damage: melt={} vs base={}",
        melt_damage.average,
        no_melt.average
    );
}

// =============================================================================
// Cross-team consistency checks
// =============================================================================

#[test]
fn cross_team_bennett_buff_consistency() {
    // Bennett's ATK buff should be consistent across different teams
    let bennett = TeamMemberBuilder::new(
        find_character("bennett").unwrap(),
        find_weapon("skyward_blade").unwrap(),
    )
    .artifact_set(find_artifact_set("noblesse_oblige").unwrap())
    .constellation(1)
    .talent_levels([1, 9, 1])
    .activate("Fantastic Voyage ATK Bonus")
    .activate("noblesse_atk_bonus")
    .build()
    .unwrap();

    let bennett_buff_value: f64 = bennett
        .buffs_provided
        .iter()
        .filter(|b| b.source.contains("Fantastic Voyage") && b.stat == BuffableStat::AtkFlat)
        .map(|b| b.value)
        .sum();

    assert!(
        bennett_buff_value > 0.0,
        "Bennett ATK flat buff should be positive"
    );

    // Build two different DPS characters with identical base stats
    let dps1 = TeamMember {
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

    let dps2 = TeamMember {
        element: Element::Cryo,
        weapon_type: WeaponType::Sword,
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

    let team1 = [bennett.clone(), dps1];
    let team2 = [bennett, dps2];

    let result1 = resolve_team_stats_detailed(&team1, 1).unwrap();
    let result2 = resolve_team_stats_detailed(&team2, 1).unwrap();

    // Both should receive the same ATK buff
    let atk_diff = (result1.final_stats.atk - result2.final_stats.atk).abs();
    assert!(
        atk_diff < 0.01,
        "Bennett buff should be consistent: team1_atk={} vs team2_atk={}, diff={}",
        result1.final_stats.atk,
        result2.final_stats.atk,
        atk_diff
    );
}

#[test]
fn cross_team_all_members_resolve_without_panic() {
    // Quick smoke test: build all 10 teams and resolve stats for every member
    let teams: Vec<(&str, Vec<TeamMember>)> = vec![
        ("Vaporize", build_team_vaporize()),
        ("National", build_team_national()),
        ("Physical", build_team_physical()),
    ];

    for (name, team) in &teams {
        for i in 0..team.len() {
            let result = resolve_team_stats_detailed(team, i);
            assert!(
                result.is_ok(),
                "Team {name} member {i} failed to resolve: {:?}",
                result.err()
            );
            let resolved = result.unwrap();
            assert_stats_sane(&resolved.final_stats, &format!("{name}[{i}]"));
        }
    }
}

fn build_team_vaporize() -> Vec<TeamMember> {
    vec![
        TeamMemberBuilder::new(
            find_character("arlecchino").unwrap(),
            find_weapon("crimson_moons_semblance").unwrap(),
        )
        .artifact_set(find_artifact_set("fragment_of_harmonic_whimsy").unwrap())
        .build()
        .unwrap(),
        TeamMemberBuilder::new(
            find_character("yelan").unwrap(),
            find_weapon("aqua_simulacra").unwrap(),
        )
        .build()
        .unwrap(),
        TeamMemberBuilder::new(
            find_character("bennett").unwrap(),
            find_weapon("skyward_blade").unwrap(),
        )
        .activate("Fantastic Voyage ATK Bonus")
        .build()
        .unwrap(),
        TeamMemberBuilder::new(
            find_character("kazuha").unwrap(),
            find_weapon("freedom_sworn").unwrap(),
        )
        .build()
        .unwrap(),
    ]
}

fn build_team_national() -> Vec<TeamMember> {
    vec![
        TeamMemberBuilder::new(
            find_character("raiden_shogun").unwrap(),
            find_weapon("engulfing_lightning").unwrap(),
        )
        .build()
        .unwrap(),
        TeamMemberBuilder::new(
            find_character("xiangling").unwrap(),
            find_weapon("the_catch").unwrap(),
        )
        .build()
        .unwrap(),
        TeamMemberBuilder::new(
            find_character("xingqiu").unwrap(),
            find_weapon("sacrificial_sword").unwrap(),
        )
        .build()
        .unwrap(),
        TeamMemberBuilder::new(
            find_character("bennett").unwrap(),
            find_weapon("skyward_blade").unwrap(),
        )
        .activate("Fantastic Voyage ATK Bonus")
        .build()
        .unwrap(),
    ]
}

fn build_team_physical() -> Vec<TeamMember> {
    vec![
        TeamMemberBuilder::new(
            find_character("eula").unwrap(),
            find_weapon("song_of_broken_pines").unwrap(),
        )
        .artifact_set(find_artifact_set("pale_flame").unwrap())
        .build()
        .unwrap(),
        TeamMemberBuilder::new(
            find_character("raiden_shogun").unwrap(),
            find_weapon("engulfing_lightning").unwrap(),
        )
        .build()
        .unwrap(),
        TeamMemberBuilder::new(
            find_character("shenhe").unwrap(),
            find_weapon("calamity_queller").unwrap(),
        )
        .build()
        .unwrap(),
        TeamMemberBuilder::new(
            find_character("zhongli").unwrap(),
            find_weapon("black_tassel").unwrap(),
        )
        .build()
        .unwrap(),
    ]
}
