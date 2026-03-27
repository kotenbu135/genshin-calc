/// Helper test to compute expected values for character TOML data files.
///
/// Run with:
/// ```
/// cargo test -p genshin-calc-core --test generate_expected -- --nocapture --ignored
/// ```
use genshin_calc_core::damage::{DamageInput, calculate_damage};
use genshin_calc_core::enemy::Enemy;
use genshin_calc_core::lunar::{LunarInput, calculate_lunar};
use genshin_calc_core::reaction::Reaction;
use genshin_calc_core::stats::Stats;
use genshin_calc_core::transformative::{TransformativeInput, calculate_transformative};
use genshin_calc_core::types::*;

fn std_enemy() -> Enemy {
    Enemy {
        level: 90,
        resistance: 0.10,
        def_reduction: 0.0,
    }
}

fn print_damage(label: &str, input: &DamageInput, enemy: &Enemy) {
    let r = calculate_damage(input, enemy).unwrap();
    eprintln!("  {label}:");
    eprintln!("    non_crit = {:.6}", r.non_crit);
    eprintln!("    crit     = {:.6}", r.crit);
    eprintln!("    average  = {:.6}", r.average);
}

fn print_transformative(label: &str, input: &TransformativeInput, enemy: &Enemy) {
    let r = calculate_transformative(input, enemy).unwrap();
    eprintln!("  {label}:");
    eprintln!("    damage = {:.6}", r.damage);
}

fn print_lunar(label: &str, input: &LunarInput, enemy: &Enemy) {
    let r = calculate_lunar(input, enemy).unwrap();
    eprintln!("  {label}:");
    eprintln!("    non_crit = {:.6}", r.non_crit);
    eprintln!("    crit     = {:.6}", r.crit);
    eprintln!("    average  = {:.6}", r.average);
}

#[test]
#[ignore]
fn generate_hu_tao() {
    eprintln!("\n=== Hu Tao ===");
    let stats = Stats {
        hp: 36000.0,
        atk: 1200.0,
        def: 800.0,
        elemental_mastery: 100.0,
        crit_rate: 0.70,
        crit_dmg: 2.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Normal (HP scaling)
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 0.8389,
        scaling_stat: ScalingStat::Hp,
        damage_type: DamageType::Normal,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Normal (HP scaling)", &input, &enemy);

    // Case 2: Vaporize (Pyro trigger = 1.5x)
    let input_vape = DamageInput {
        reaction: Some(Reaction::Vaporize),
        reaction_bonus: 0.0,
        ..input
    };
    print_damage("Vaporize (Pyro 1.5x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_ganyu() {
    eprintln!("\n=== Ganyu ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 2200.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.45,
        crit_dmg: 1.80,
        energy_recharge: 1.0,
        dmg_bonus: 0.616,
    };
    let enemy = std_enemy();

    // Case 1: Normal Charged
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 3.9616,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Charged,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Charged (no reaction)", &input, &enemy);

    // Case 2: Reverse Melt EM0 (Cryo trigger = 1.5x)
    let input_melt = DamageInput {
        reaction: Some(Reaction::Melt),
        ..input.clone()
    };
    print_damage("Reverse Melt EM0", &input_melt, &enemy);

    // Case 3: Reverse Melt EM200
    let stats_em200 = Stats {
        elemental_mastery: 200.0,
        ..stats
    };
    let input_melt_em200 = DamageInput {
        stats: stats_em200,
        reaction: Some(Reaction::Melt),
        ..input
    };
    print_damage("Reverse Melt EM200", &input_melt_em200, &enemy);
}

#[test]
#[ignore]
fn generate_raiden() {
    eprintln!("\n=== Raiden ===");
    let stats = Stats {
        hp: 20000.0,
        atk: 2000.0,
        def: 800.0,
        elemental_mastery: 0.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 2.5,
        dmg_bonus: 0.466,
    };
    let enemy = Enemy {
        level: 100,
        resistance: 0.10,
        def_reduction: 0.0,
    };

    // Case 1: Normal Burst
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 6.4128,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Electro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Burst (no reaction)", &input, &enemy);

    // Case 2: Aggravate (EM150)
    let stats_em150 = Stats {
        elemental_mastery: 150.0,
        ..stats
    };
    let input_agg = DamageInput {
        stats: stats_em150,
        reaction: Some(Reaction::Aggravate),
        ..input
    };
    print_damage("Aggravate EM150", &input_agg, &enemy);
}

#[test]
#[ignore]
fn generate_fischl() {
    eprintln!("\n=== Fischl ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1800.0,
        def: 600.0,
        elemental_mastery: 150.0,
        crit_rate: 0.60,
        crit_dmg: 1.40,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 2.0988,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Electro),
        reaction: Some(Reaction::Aggravate),
        reaction_bonus: 0.0,
    };
    print_damage("Aggravate", &input, &enemy);
}

#[test]
#[ignore]
fn generate_yae_miko() {
    eprintln!("\n=== Yae Miko ===");
    let stats = Stats {
        hp: 17000.0,
        atk: 1800.0,
        def: 600.0,
        elemental_mastery: 200.0,
        crit_rate: 0.65,
        crit_dmg: 1.60,
        energy_recharge: 1.3,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Aggravate
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 1.7014,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Electro),
        reaction: Some(Reaction::Aggravate),
        reaction_bonus: 0.0,
    };
    print_damage("Aggravate", &input, &enemy);

    // Case 2: Lunar EC
    let lunar_input = LunarInput {
        character_level: 90,
        elemental_mastery: 200.0,
        reaction: Reaction::LunarElectroCharged,
        reaction_bonus: 0.0,
        crit_rate: 0.65,
        crit_dmg: 1.60,
    };
    print_lunar("Lunar EC", &lunar_input, &enemy);
}

#[test]
#[ignore]
fn generate_yelan() {
    eprintln!("\n=== Yelan ===");
    let stats = Stats {
        hp: 30000.0,
        atk: 1000.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.70,
        crit_dmg: 2.00,
        energy_recharge: 1.6,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Normal (HP scaling)
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 0.2,
        scaling_stat: ScalingStat::Hp,
        damage_type: DamageType::Skill,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Normal (HP scaling)", &input, &enemy);

    // Case 2: Vaporize (Hydro trigger = 2.0x)
    let input_vape = DamageInput {
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Vaporize (Hydro 2.0x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_ayato() {
    eprintln!("\n=== Ayato ===");
    let stats = Stats {
        hp: 20000.0,
        atk: 1900.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.65,
        crit_dmg: 1.50,
        energy_recharge: 1.3,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Normal
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 1.3154,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Normal (no reaction)", &input, &enemy);

    // Case 2: Vaporize EM100 (Hydro trigger = 2.0x)
    let stats_em100 = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_vape = DamageInput {
        stats: stats_em100,
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Vaporize EM100", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_nilou() {
    eprintln!("\n=== Nilou ===");
    let stats = Stats {
        hp: 60000.0,
        atk: 800.0,
        def: 700.0,
        elemental_mastery: 100.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Normal (HP scaling, Skill)
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 0.0534,
        scaling_stat: ScalingStat::Hp,
        damage_type: DamageType::Skill,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Normal (HP scaling)", &input, &enemy);

    // Case 2: Bloom (transformative, EM800)
    let trans_input = TransformativeInput {
        character_level: 90,
        elemental_mastery: 800.0,
        reaction: Reaction::Bloom,
        reaction_bonus: 0.0,
    };
    print_transformative("Bloom EM800", &trans_input, &enemy);

    // Case 3: Lunar Bloom (EM800)
    let lunar_input = LunarInput {
        character_level: 90,
        elemental_mastery: 800.0,
        reaction: Reaction::LunarBloom,
        reaction_bonus: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
    };
    print_lunar("Lunar Bloom EM800", &lunar_input, &enemy);
}

#[test]
#[ignore]
fn generate_itto() {
    eprintln!("\n=== Itto ===");
    let stats = Stats {
        hp: 20000.0,
        atk: 1000.0,
        def: 2500.0,
        elemental_mastery: 0.0,
        crit_rate: 0.70,
        crit_dmg: 1.80,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 1.7884,
        scaling_stat: ScalingStat::Def,
        damage_type: DamageType::Normal,
        element: Some(Element::Geo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Normal (DEF scaling)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_xiao() {
    eprintln!("\n=== Xiao ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 2300.0,
        def: 800.0,
        elemental_mastery: 0.0,
        crit_rate: 0.70,
        crit_dmg: 1.80,
        energy_recharge: 1.2,
        dmg_bonus: 0.951,
    };
    let enemy = std_enemy();

    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 4.0436,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Plunging,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Plunging (no reaction)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_freminet() {
    eprintln!("\n=== Freminet ===");
    let stats = Stats {
        hp: 3000.0,
        atk: 94.0,
        def: 200.0,
        elemental_mastery: 0.0,
        crit_rate: 0.05,
        crit_dmg: 0.50,
        energy_recharge: 1.0,
        dmg_bonus: 0.0,
    };
    let enemy = Enemy {
        level: 85,
        resistance: 0.10,
        def_reduction: 0.0,
    };

    let input = DamageInput {
        character_level: 20,
        stats,
        talent_multiplier: 1.077,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: None, // Physical
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Physical Normal Lv20 vs Lv85", &input, &enemy);
}

#[test]
#[ignore]
fn generate_nahida() {
    eprintln!("\n=== Nahida ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 1200.0,
        def: 600.0,
        elemental_mastery: 800.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.2,
        dmg_bonus: 0.15,
    };
    let enemy = std_enemy();

    // Case 1: Normal
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 1.8576,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Dendro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Skill (no reaction)", &input, &enemy);

    // Case 2: Spread
    let input_spread = DamageInput {
        reaction: Some(Reaction::Spread),
        ..input
    };
    print_damage("Spread", &input_spread, &enemy);
}

#[test]
#[ignore]
fn generate_tighnari() {
    eprintln!("\n=== Tighnari ===");
    let stats = Stats {
        hp: 17000.0,
        atk: 1600.0,
        def: 600.0,
        elemental_mastery: 300.0,
        crit_rate: 0.60,
        crit_dmg: 1.50,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Normal Charged
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 1.5618,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Charged,
        element: Some(Element::Dendro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Charged (no reaction)", &input, &enemy);

    // Case 2: Spread
    let input_spread = DamageInput {
        reaction: Some(Reaction::Spread),
        ..input
    };
    print_damage("Spread", &input_spread, &enemy);
}

#[test]
#[ignore]
fn generate_kazuha() {
    eprintln!("\n=== Kazuha ===");
    let enemy = std_enemy();

    // All 4 Swirl cases: EM960, VV 4pc reaction_bonus=0.60
    let elements = [
        ("SwirlPyro", Reaction::Swirl(Element::Pyro)),
        ("SwirlHydro", Reaction::Swirl(Element::Hydro)),
        ("SwirlElectro", Reaction::Swirl(Element::Electro)),
        ("SwirlCryo", Reaction::Swirl(Element::Cryo)),
    ];

    for (label, reaction) in &elements {
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 960.0,
            reaction: *reaction,
            reaction_bonus: 0.60,
        };
        print_transformative(label, &input, &enemy);
    }
}
