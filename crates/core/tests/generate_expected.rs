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

// ===== Pyro Characters =====

#[test]
#[ignore]
fn generate_amber() {
    eprintln!("\n=== Amber ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1500.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Fiery Rain total (Burst) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 9.0979,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Fiery Rain total (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Pyro trigger = 1.5x), EM 150
    let stats_em = Stats {
        elemental_mastery: 150.0,
        ..stats
    };
    let input_vape = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Fiery Rain total — Vaporize (1.5x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_arlecchino() {
    eprintln!("\n=== Arlecchino ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 2100.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.70,
        crit_dmg: 1.80,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Normal 1st hit — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 0.939,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Normal 1st hit (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Pyro trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_vape = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Normal 1st hit — Vaporize (1.5x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_bennett() {
    eprintln!("\n=== Bennett ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1300.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.8,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Passion Overload press (Skill) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.4768,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Passion Overload press (no reaction)", &input, &enemy);

    // Case 2: Forward Melt (Pyro trigger = 2.0x), EM 150
    let stats_em = Stats {
        elemental_mastery: 150.0,
        ..stats
    };
    let input_melt = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Melt),
        ..input
    };
    print_damage("Passion Overload press — Melt (2.0x)", &input_melt, &enemy);
}

#[test]
#[ignore]
fn generate_chevreuse() {
    eprintln!("\n=== Chevreuse ===");
    let stats = Stats {
        hp: 32000.0,
        atk: 1000.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Short-Range Rapid Fire press (Skill, HP scaling) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 2.0736,
        scaling_stat: ScalingStat::Hp,
        damage_type: DamageType::Skill,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Short-Range Rapid Fire press (no reaction)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_dehya() {
    eprintln!("\n=== Dehya ===");
    let stats = Stats {
        hp: 20000.0,
        atk: 1900.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Incineration Drive kick (Burst, ATK scaling only) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.5074,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Incineration Drive kick (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Pyro trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_vape = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage(
        "Incineration Drive kick — Vaporize (1.5x)",
        &input_vape,
        &enemy,
    );
}

#[test]
#[ignore]
fn generate_gaming() {
    eprintln!("\n=== Gaming ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 1600.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Man Chai plunge (Skill plunge) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 4.1472,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Plunging,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Man Chai plunge (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Pyro trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_vape = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Man Chai plunge — Vaporize (1.5x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_klee() {
    eprintln!("\n=== Klee ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 1900.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.65,
        crit_dmg: 1.50,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Charged Attack — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.8325,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Charged,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Charged Attack (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Pyro trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_vape = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Charged Attack — Vaporize (1.5x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_lyney() {
    eprintln!("\n=== Lyney ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 2100.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.70,
        crit_dmg: 1.80,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Prop Arrow Lv2 (Charged) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 3.2832,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Charged,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Prop Arrow Lv2 (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Pyro trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_vape = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Prop Arrow Lv2 — Vaporize (1.5x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_mavuika() {
    eprintln!("\n=== Mavuika ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 2200.0,
        def: 800.0,
        elemental_mastery: 0.0,
        crit_rate: 0.70,
        crit_dmg: 1.80,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Flamestrider Normal 1st — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 1.132,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Flamestrider Normal 1st (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Pyro trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_vape = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage(
        "Flamestrider Normal 1st — Vaporize (1.5x)",
        &input_vape,
        &enemy,
    );
}

#[test]
#[ignore]
fn generate_thoma() {
    eprintln!("\n=== Thoma ===");
    let stats = Stats {
        hp: 20000.0,
        atk: 1200.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 2.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Fiery Collapse (Burst coord) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 1.044,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Fiery Collapse (no reaction)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_xiangling() {
    eprintln!("\n=== Xiangling ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 1600.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 1.8,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Pyronado swing (Burst) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.016,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Pyronado swing (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Pyro trigger = 1.5x), EM 200
    let stats_em = Stats {
        elemental_mastery: 200.0,
        ..stats
    };
    let input_vape = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Pyronado swing — Vaporize (1.5x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_xinyan() {
    eprintln!("\n=== Xinyan ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 1600.0,
        def: 900.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.30,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Riff Revolution physical burst — no reaction (physical)
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 6.4752,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: None, // Physical
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Riff Revolution physical (no reaction)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_yanfei() {
    eprintln!("\n=== Yanfei ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1600.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Charged Attack 3 seals — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.4469,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Charged,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Charged Attack 3 seals (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Pyro trigger = 1.5x), EM 150
    let stats_em = Stats {
        elemental_mastery: 150.0,
        ..stats
    };
    let input_vape = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage(
        "Charged Attack 3 seals — Vaporize (1.5x)",
        &input_vape,
        &enemy,
    );
}

#[test]
#[ignore]
fn generate_yoimiya() {
    eprintln!("\n=== Yoimiya ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 2000.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.65,
        crit_dmg: 1.60,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Niwabi Normal 1st hit: base 63.59%×2 = 127.18% × Niwabi 161.74% = 205.67%
    // Case 1: Normal — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.0567,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Pyro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Niwabi Normal 1st (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Pyro trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_vape = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Niwabi Normal 1st — Vaporize (1.5x)", &input_vape, &enemy);
}

// ==================== Hydro Characters ====================

#[test]
#[ignore]
fn generate_barbara() {
    eprintln!("\n=== Barbara ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1200.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.4,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Normal Attack 1st Hit (Lv10: 72.05%)
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 0.7205,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Normal Attack 1st Hit (no reaction)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_candace() {
    eprintln!("\n=== Candace ===");
    let stats = Stats {
        hp: 30000.0,
        atk: 1000.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.4,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Burst initial hit (Lv10: 11.9% Max HP)
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 0.119,
        scaling_stat: ScalingStat::Hp,
        damage_type: DamageType::Burst,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Burst initial hit (no reaction)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_furina() {
    eprintln!("\n=== Furina ===");
    let stats = Stats {
        hp: 40000.0,
        atk: 1000.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.65,
        crit_dmg: 1.50,
        energy_recharge: 1.4,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Salon Solitaire Ousia Bubble (Lv10: 14.96% Max HP)
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 0.1496,
        scaling_stat: ScalingStat::Hp,
        damage_type: DamageType::Skill,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Salon Solitaire Bubble (no reaction)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_kokomi() {
    eprintln!("\n=== Kokomi ===");
    let stats = Stats {
        hp: 45000.0,
        atk: 900.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.05,
        crit_dmg: 0.50,
        energy_recharge: 1.4,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Burst mode Normal Attack (Lv10: 9.76% Max HP bonus)
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 0.0976,
        scaling_stat: ScalingStat::Hp,
        damage_type: DamageType::Normal,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Burst mode NA (HP scaling, no reaction)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_mona() {
    eprintln!("\n=== Mona ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1900.0,
        def: 600.0,
        elemental_mastery: 150.0,
        crit_rate: 0.65,
        crit_dmg: 1.40,
        energy_recharge: 1.8,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Burst Explosion (Lv10: 796% ATK) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 7.96,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Burst Explosion (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Hydro trigger = 2.0x)
    let input_vape = DamageInput {
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Burst Explosion — Vaporize (2.0x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_mualani() {
    eprintln!("\n=== Mualani ===");
    let stats = Stats {
        hp: 35000.0,
        atk: 900.0,
        def: 700.0,
        elemental_mastery: 150.0,
        crit_rate: 0.65,
        crit_dmg: 1.50,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Sharky's Surging Bite (Lv10: 78.11% Max HP) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 0.7811,
        scaling_stat: ScalingStat::Hp,
        damage_type: DamageType::Normal,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Surging Bite (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Hydro trigger = 2.0x)
    let input_vape = DamageInput {
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Surging Bite — Vaporize (2.0x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_neuvillette() {
    eprintln!("\n=== Neuvillette ===");
    let stats = Stats {
        hp: 40000.0,
        atk: 900.0,
        def: 700.0,
        elemental_mastery: 100.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Equitable Judgment (Lv10: 14.47% Max HP) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 0.1447,
        scaling_stat: ScalingStat::Hp,
        damage_type: DamageType::Charged,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Equitable Judgment (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Hydro trigger = 2.0x)
    let input_vape = DamageInput {
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Equitable Judgment — Vaporize (2.0x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_sigewinne() {
    eprintln!("\n=== Sigewinne ===");
    let stats = Stats {
        hp: 40000.0,
        atk: 800.0,
        def: 600.0,
        elemental_mastery: 100.0,
        crit_rate: 0.55,
        crit_dmg: 1.20,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Bouncing Bubblebalm (Lv10: 4.10% Max HP) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 0.041,
        scaling_stat: ScalingStat::Hp,
        damage_type: DamageType::Skill,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Bubblebalm (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Hydro trigger = 2.0x)
    let input_vape = DamageInput {
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Bubblebalm — Vaporize (2.0x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_tartaglia() {
    eprintln!("\n=== Tartaglia ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 1900.0,
        def: 700.0,
        elemental_mastery: 100.0,
        crit_rate: 0.65,
        crit_dmg: 1.40,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Melee Stance Normal 1st (Lv10: 76.8%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 0.768,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Melee NA 1st (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Hydro trigger = 2.0x)
    let input_vape = DamageInput {
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Melee NA 1st — Vaporize (2.0x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_xingqiu() {
    eprintln!("\n=== Xingqiu ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 1500.0,
        def: 700.0,
        elemental_mastery: 100.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 2.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Raincutter Sword Rain (Lv10: 97.7%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 0.977,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Raincutter (no reaction)", &input, &enemy);

    // Case 2: Vaporize (Hydro trigger = 2.0x)
    let input_vape = DamageInput {
        reaction: Some(Reaction::Vaporize),
        ..input
    };
    print_damage("Raincutter — Vaporize (2.0x)", &input_vape, &enemy);
}

#[test]
#[ignore]
fn generate_dahlia() {
    eprintln!("\n=== Dahlia ===");
    let stats = Stats {
        hp: 20000.0,
        atk: 1300.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.6,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Immersive Ordinance Skill (Lv10: 419.0%)
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 4.19,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Hydro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Immersive Ordinance (no reaction)", &input, &enemy);
}

// ==================== Cryo Characters ====================

#[test]
#[ignore]
fn generate_aloy() {
    eprintln!("\n=== Aloy ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 1800.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Freeze Bomb (Skill Lv10: 646.56%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 6.4656,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Freeze Bomb (no reaction)", &input, &enemy);

    // Case 2: Reverse Melt (Cryo trigger = 1.5x), EM 150
    let stats_em = Stats {
        elemental_mastery: 150.0,
        ..stats
    };
    let input_melt = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Melt),
        ..input
    };
    print_damage("Freeze Bomb — Melt (1.5x)", &input_melt, &enemy);
}

#[test]
#[ignore]
fn generate_ayaka() {
    eprintln!("\n=== Ayaka ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 2000.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.40,
        crit_dmg: 1.80,
        energy_recharge: 1.2,
        dmg_bonus: 0.616,
    };
    let enemy = std_enemy();

    // Case 1: Soumetsu Cutting DMG (Burst Lv10: 202.14%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.0214,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Soumetsu Cutting (no reaction)", &input, &enemy);

    // Case 2: Reverse Melt (Cryo trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_melt = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Melt),
        ..input
    };
    print_damage("Soumetsu Cutting — Melt (1.5x)", &input_melt, &enemy);
}

#[test]
#[ignore]
fn generate_charlotte() {
    eprintln!("\n=== Charlotte ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1300.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.6,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Photo DMG press (Skill Lv10: 127.68%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 1.2768,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Photo DMG press (no reaction)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_chongyun() {
    eprintln!("\n=== Chongyun ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1500.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 1.4,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Spirit Blade per blade (Burst Lv10: 256.32%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.5632,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Spirit Blade per blade (no reaction)", &input, &enemy);

    // Case 2: Reverse Melt (Cryo trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_melt = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Melt),
        ..input
    };
    print_damage("Spirit Blade per blade — Melt (1.5x)", &input_melt, &enemy);
}

#[test]
#[ignore]
fn generate_citlali() {
    eprintln!("\n=== Citlali ===");
    let stats = Stats {
        hp: 20000.0,
        atk: 1200.0,
        def: 700.0,
        elemental_mastery: 800.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.4,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Frostfall Storm (Skill Lv10: 30.64% ATK) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 0.3064,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Frostfall Storm (no reaction)", &input, &enemy);

    // Case 2: Reverse Melt (Cryo trigger = 1.5x)
    let input_melt = DamageInput {
        reaction: Some(Reaction::Melt),
        ..input
    };
    print_damage("Frostfall Storm — Melt (1.5x)", &input_melt, &enemy);
}

#[test]
#[ignore]
fn generate_diona() {
    eprintln!("\n=== Diona ===");
    let stats = Stats {
        hp: 20000.0,
        atk: 1200.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.6,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Icy Paws per paw (Skill Lv10: 75.46%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 0.7546,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Icy Paws per paw (no reaction)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_escoffier() {
    eprintln!("\n=== Escoffier ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 1800.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Frosty Parfait (Skill Lv10: 216%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.16,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Frosty Parfait (no reaction)", &input, &enemy);

    // Case 2: Reverse Melt (Cryo trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_melt = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Melt),
        ..input
    };
    print_damage("Frosty Parfait — Melt (1.5x)", &input_melt, &enemy);
}

#[test]
#[ignore]
fn generate_eula() {
    eprintln!("\n=== Eula ===");
    let stats = Stats {
        hp: 20000.0,
        atk: 2100.0,
        def: 800.0,
        elemental_mastery: 0.0,
        crit_rate: 0.65,
        crit_dmg: 1.60,
        energy_recharge: 1.4,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Lightfall Sword explosion (Burst Lv10: 725.56%) — PHYSICAL
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 7.2556,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: None, // Physical
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Lightfall Sword explosion (Physical)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_kaeya() {
    eprintln!("\n=== Kaeya ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1500.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Frostgnaw (Skill Lv10: 344.16%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 3.4416,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Frostgnaw (no reaction)", &input, &enemy);

    // Case 2: Reverse Melt (Cryo trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_melt = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Melt),
        ..input
    };
    print_damage("Frostgnaw — Melt (1.5x)", &input_melt, &enemy);
}

#[test]
#[ignore]
fn generate_layla() {
    eprintln!("\n=== Layla ===");
    let stats = Stats {
        hp: 35000.0,
        atk: 1000.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.4,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Shooting Star (Skill Lv10: 26.5% Max HP) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 0.265,
        scaling_stat: ScalingStat::Hp,
        damage_type: DamageType::Skill,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Shooting Star (no reaction)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_mika() {
    eprintln!("\n=== Mika ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 1300.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.8,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Starfrost Swirl press (Skill Lv10: 159.6%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 1.596,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Starfrost Swirl press (no reaction)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_qiqi() {
    eprintln!("\n=== Qiqi ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 1400.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.6,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Normal Attack 1st Hit (Lv10: 74.63%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 0.7463,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Normal Attack 1st (no reaction)", &input, &enemy);
}

#[test]
#[ignore]
fn generate_rosaria() {
    eprintln!("\n=== Rosaria ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1500.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.6,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Rites of Termination per slash (Burst Lv10: 237.6%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.376,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage(
        "Rites of Termination per slash (no reaction)",
        &input,
        &enemy,
    );

    // Case 2: Reverse Melt (Cryo trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_melt = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Melt),
        ..input
    };
    print_damage(
        "Rites of Termination per slash — Melt (1.5x)",
        &input_melt,
        &enemy,
    );
}

#[test]
#[ignore]
fn generate_shenhe() {
    eprintln!("\n=== Shenhe ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 1500.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.8,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Spring Spirit Summoning press (Skill Lv10: 250.56%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 2.5056,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage(
        "Spring Spirit Summoning press (no reaction)",
        &input,
        &enemy,
    );
}

#[test]
#[ignore]
fn generate_skirk() {
    eprintln!("\n=== Skirk ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 2100.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.70,
        crit_dmg: 1.80,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Seven-Phase Flash Normal 1st (Skill Lv10: 262.56%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.6256,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Seven-Phase Flash NA 1st (no reaction)", &input, &enemy);

    // Case 2: Reverse Melt (Cryo trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_melt = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Melt),
        ..input
    };
    print_damage(
        "Seven-Phase Flash NA 1st — Melt (1.5x)",
        &input_melt,
        &enemy,
    );
}

#[test]
#[ignore]
fn generate_wriothesley() {
    eprintln!("\n=== Wriothesley ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 2000.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.65,
        crit_dmg: 1.60,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Normal Attack 1st (Lv10: 105.48%) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 1.0548,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Cryo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Normal Attack 1st (no reaction)", &input, &enemy);

    // Case 2: Reverse Melt (Cryo trigger = 1.5x), EM 100
    let stats_em = Stats {
        elemental_mastery: 100.0,
        ..stats
    };
    let input_melt = DamageInput {
        stats: stats_em,
        reaction: Some(Reaction::Melt),
        ..input
    };
    print_damage("Normal Attack 1st — Melt (1.5x)", &input_melt, &enemy);
}

// ============================================================
// Electro characters
// ============================================================

#[test]
#[ignore]
fn generate_beidou() {
    eprintln!("\n=== Beidou ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 1500.0,
        def: 700.0,
        elemental_mastery: 160.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 1.6,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Stormbreaker discharge — Aggravate
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 1.728,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Electro),
        reaction: Some(Reaction::Aggravate),
        reaction_bonus: 0.0,
    };
    print_damage("Stormbreaker discharge — Aggravate", &input, &enemy);
}

#[test]
#[ignore]
fn generate_clorinde() {
    eprintln!("\n=== Clorinde ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 2000.0,
        def: 700.0,
        elemental_mastery: 150.0,
        crit_rate: 0.65,
        crit_dmg: 1.50,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Pistol Shot Normal 1 — Aggravate
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 1.26,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Electro),
        reaction: Some(Reaction::Aggravate),
        reaction_bonus: 0.0,
    };
    print_damage("Pistol Shot Normal 1 — Aggravate", &input, &enemy);
}

#[test]
#[ignore]
fn generate_cyno() {
    eprintln!("\n=== Cyno ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 1900.0,
        def: 700.0,
        elemental_mastery: 200.0,
        crit_rate: 0.65,
        crit_dmg: 1.50,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Pactsworn Normal 1st — Aggravate
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 1.5475,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Electro),
        reaction: Some(Reaction::Aggravate),
        reaction_bonus: 0.0,
    };
    print_damage("Pactsworn Normal 1st — Aggravate", &input, &enemy);
}

#[test]
#[ignore]
fn generate_dori() {
    eprintln!("\n=== Dori ===");
    let stats = Stats {
        hp: 17000.0,
        atk: 1300.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.8,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Troubleshooter Shot (no reaction)
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 2.79832,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Electro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Troubleshooter Shot — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_keqing() {
    eprintln!("\n=== Keqing ===");
    let stats = Stats {
        hp: 17000.0,
        atk: 2000.0,
        def: 700.0,
        elemental_mastery: 150.0,
        crit_rate: 0.65,
        crit_dmg: 1.50,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Charged Attack (2-hit total 321.81%) — Aggravate
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 3.2181,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Charged,
        element: Some(Element::Electro),
        reaction: Some(Reaction::Aggravate),
        reaction_bonus: 0.0,
    };
    print_damage("Charged Attack — Aggravate", &input, &enemy);
}

#[test]
#[ignore]
fn generate_kuki_shinobu() {
    eprintln!("\n=== Kuki Shinobu ===");
    let stats = Stats {
        hp: 30000.0,
        atk: 1000.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Grass Ring tick (HP scaling, no reaction)
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 0.4543,
        scaling_stat: ScalingStat::Hp,
        damage_type: DamageType::Skill,
        element: Some(Element::Electro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Grass Ring tick — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_lisa() {
    eprintln!("\n=== Lisa ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1500.0,
        def: 600.0,
        elemental_mastery: 180.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Violet Arc Hold (3 stacks) — Aggravate
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 6.624,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Electro),
        reaction: Some(Reaction::Aggravate),
        reaction_bonus: 0.0,
    };
    print_damage("Violet Arc Hold (3 stacks) — Aggravate", &input, &enemy);
}

#[test]
#[ignore]
fn generate_razor() {
    eprintln!("\n=== Razor ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 1600.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Normal 1st — PHYSICAL (no element, no reaction)
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 1.7113,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: None,
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Normal 1st — Physical", &input, &enemy);
}

#[test]
#[ignore]
fn generate_iansan() {
    eprintln!("\n=== Iansan ===");
    let stats = Stats {
        hp: 17000.0,
        atk: 1400.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Thunderbolt Rush Skill (no reaction)
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 5.4416,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Electro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Thunderbolt Rush — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_ineffa() {
    eprintln!("\n=== Ineffa ===");
    let stats = Stats {
        hp: 17000.0,
        atk: 1900.0,
        def: 700.0,
        elemental_mastery: 200.0,
        crit_rate: 0.65,
        crit_dmg: 1.50,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Burst nuke — Aggravate
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 16.074,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Electro),
        reaction: Some(Reaction::Aggravate),
        reaction_bonus: 0.0,
    };
    print_damage("Cyclonic Exterminator Burst — Aggravate", &input, &enemy);
}

#[test]
#[ignore]
fn generate_ororon() {
    eprintln!("\n=== Ororon ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 1400.0,
        def: 600.0,
        elemental_mastery: 180.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 1.4,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Night's Sling Spirit Orb — Aggravate
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 4.693,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Electro),
        reaction: Some(Reaction::Aggravate),
        reaction_bonus: 0.0,
    };
    print_damage("Night's Sling Spirit Orb — Aggravate", &input, &enemy);
}

#[test]
#[ignore]
fn generate_kujou_sara() {
    eprintln!("\n=== Kujou Sara ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1300.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 2.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Tengu Juurai: Ambush (no reaction)
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 2.2637,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Electro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Tengu Juurai: Ambush — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_sethos() {
    eprintln!("\n=== Sethos ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1500.0,
        def: 600.0,
        elemental_mastery: 300.0,
        crit_rate: 0.55,
        crit_dmg: 1.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Shadowpiercing Shot (ATK portion) — Aggravate
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 1.1135,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Charged,
        element: Some(Element::Electro),
        reaction: Some(Reaction::Aggravate),
        reaction_bonus: 0.0,
    };
    print_damage("Shadowpiercing Shot — Aggravate", &input, &enemy);
}

#[test]
#[ignore]
fn generate_varesa() {
    eprintln!("\n=== Varesa ===");
    let stats = Stats {
        hp: 17000.0,
        atk: 2000.0,
        def: 700.0,
        elemental_mastery: 150.0,
        crit_rate: 0.65,
        crit_dmg: 1.50,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Flying Kick Burst — Aggravate
    let input = DamageInput {
        character_level: 90,
        stats,
        talent_multiplier: 8.166,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Electro),
        reaction: Some(Reaction::Aggravate),
        reaction_bonus: 0.0,
    };
    print_damage("Flying Kick Burst — Aggravate", &input, &enemy);
}

#[test]
#[ignore]
fn generate_alhaitham() {
    eprintln!("\n=== Alhaitham ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 2000.0,
        def: 700.0,
        elemental_mastery: 200.0,
        crit_rate: 0.65,
        crit_dmg: 1.50,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: 1-Mirror Projection Attack (ATK portion only) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 1.2096,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Dendro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("1-Mirror Projection Skill — no reaction", &input, &enemy);

    // Case 2: Spread
    let input_spread = DamageInput {
        reaction: Some(Reaction::Spread),
        ..input
    };
    print_damage("1-Mirror Projection Skill — Spread", &input_spread, &enemy);
}

#[test]
#[ignore]
fn generate_baizhu() {
    eprintln!("\n=== Baizhu ===");
    let stats = Stats {
        hp: 30000.0,
        atk: 1200.0,
        def: 600.0,
        elemental_mastery: 200.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.4,
        dmg_bonus: 0.15,
    };
    let enemy = std_enemy();

    // Case 1: Spiritvein DMG (Burst) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 1.7472,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Dendro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Spiritvein Burst — no reaction", &input, &enemy);

    // Case 2: Spread
    let input_spread = DamageInput {
        reaction: Some(Reaction::Spread),
        ..input
    };
    print_damage("Spiritvein Burst — Spread", &input_spread, &enemy);
}

#[test]
#[ignore]
fn generate_collei() {
    eprintln!("\n=== Collei ===");
    let stats = Stats {
        hp: 14000.0,
        atk: 1500.0,
        def: 550.0,
        elemental_mastery: 150.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Floral Brush Skill — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.7216,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Dendro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Floral Brush Skill — no reaction", &input, &enemy);

    // Case 2: Spread
    let input_spread = DamageInput {
        reaction: Some(Reaction::Spread),
        ..input
    };
    print_damage("Floral Brush Skill — Spread", &input_spread, &enemy);
}

#[test]
#[ignore]
fn generate_emilie() {
    eprintln!("\n=== Emilie ===");
    let stats = Stats {
        hp: 17000.0,
        atk: 1900.0,
        def: 650.0,
        elemental_mastery: 150.0,
        crit_rate: 0.65,
        crit_dmg: 1.40,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Lumidouce Case Lv1 Attack — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 0.7524,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Dendro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Lumidouce Case Lv1 Skill — no reaction", &input, &enemy);

    // Case 2: Spread
    let input_spread = DamageInput {
        reaction: Some(Reaction::Spread),
        ..input
    };
    print_damage("Lumidouce Case Lv1 Skill — Spread", &input_spread, &enemy);
}

#[test]
#[ignore]
fn generate_kaveh() {
    eprintln!("\n=== Kaveh ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 1500.0,
        def: 600.0,
        elemental_mastery: 200.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Painted Dome Burst — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.88,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Dendro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Painted Dome Burst — no reaction", &input, &enemy);

    // Case 2: Spread
    let input_spread = DamageInput {
        reaction: Some(Reaction::Spread),
        ..input
    };
    print_damage("Painted Dome Burst — Spread", &input_spread, &enemy);
}

#[test]
#[ignore]
fn generate_kinich() {
    eprintln!("\n=== Kinich ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 2100.0,
        def: 700.0,
        elemental_mastery: 150.0,
        crit_rate: 0.65,
        crit_dmg: 1.60,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Scalespiker Cannon Skill — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 12.3739,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Dendro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Scalespiker Cannon Skill — no reaction", &input, &enemy);

    // Case 2: Spread
    let input_spread = DamageInput {
        reaction: Some(Reaction::Spread),
        ..input
    };
    print_damage("Scalespiker Cannon Skill — Spread", &input_spread, &enemy);
}

#[test]
#[ignore]
fn generate_kirara() {
    eprintln!("\n=== Kirara ===");
    let stats = Stats {
        hp: 30000.0,
        atk: 1200.0,
        def: 600.0,
        elemental_mastery: 100.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.2,
        dmg_bonus: 0.15,
    };
    let enemy = std_enemy();

    // Case 1: Tail-Flicking Flying Kick Skill — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 1.976,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Dendro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage(
        "Tail-Flicking Flying Kick Skill — no reaction",
        &input,
        &enemy,
    );

    // Case 2: Spread
    let input_spread = DamageInput {
        reaction: Some(Reaction::Spread),
        ..input
    };
    print_damage(
        "Tail-Flicking Flying Kick Skill — Spread",
        &input_spread,
        &enemy,
    );
}

#[test]
#[ignore]
fn generate_yaoyao() {
    eprintln!("\n=== Yaoyao ===");
    let stats = Stats {
        hp: 25000.0,
        atk: 1200.0,
        def: 600.0,
        elemental_mastery: 150.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.4,
        dmg_bonus: 0.15,
    };
    let enemy = std_enemy();

    // Case 1: White Jade Radish Skill — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 0.56848,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Dendro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("White Jade Radish Skill — no reaction", &input, &enemy);

    // Case 2: Spread
    let input_spread = DamageInput {
        reaction: Some(Reaction::Spread),
        ..input
    };
    print_damage("White Jade Radish Skill — Spread", &input_spread, &enemy);
}

// ========================
// Anemo Characters
// ========================

#[test]
#[ignore]
fn generate_faruzan() {
    eprintln!("\n=== Faruzan ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1400.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Pressurized Collapse (Charged) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 1.944,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Charged,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Pressurized Collapse Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_heizou() {
    eprintln!("\n=== Heizou ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1500.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Heartstopper Strike (Skill) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 4.0954,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Heartstopper Strike Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_jean() {
    eprintln!("\n=== Jean ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 1900.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Gale Blade (Skill) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 5.548,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Gale Blade Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_lynette() {
    eprintln!("\n=== Lynette ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1400.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Enigma Thrust (Skill) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 4.824,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Enigma Thrust Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_sayu() {
    eprintln!("\n=== Sayu ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1300.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Fuufuu Whirlwind Kick (Skill press) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 3.0096,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Fuufuu Whirlwind Kick Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_sucrose() {
    eprintln!("\n=== Sucrose ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1400.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Astable Anemohypostasis (Skill) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 3.8016,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Astable Anemohypostasis Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_venti() {
    eprintln!("\n=== Venti ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 1900.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Skyward Sonnet press (Skill) — no reaction
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 4.968,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Skyward Sonnet press Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_wanderer() {
    eprintln!("\n=== Wanderer ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 2000.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.65,
        crit_dmg: 1.50,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Normal Attack 1st (Windfavored) — no reaction
    // Kuugo: Fushoudan = 153.72% of Normal Attack DMG
    // Normal 1st Hit Lv10 = 135.83%
    // Effective multiplier = 1.3583 * 1.5372 = 2.08779876
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.0878,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Normal,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Kuugo Fushoudan 1st Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_xianyun() {
    eprintln!("\n=== Xianyun ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 1800.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Driftcloud Wave (Skill plunge, base) — no reaction
    // White Clouds at Dawn Driftcloud Wave at Lv10 = 220.4%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.204,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Plunging,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Driftcloud Wave Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_chasca() {
    eprintln!("\n=== Chasca ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 2000.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.65,
        crit_dmg: 1.50,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Shadowhunt Shell (Charged, Anemo) — no reaction
    // Spirit Reins, Shadow Hunt: Shadowhunt Shell DMG Lv10 = 87.84%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 0.8784,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Charged,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Shadowhunt Shell Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_ifa() {
    eprintln!("\n=== Ifa ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1400.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Tonicshot (Skill) — no reaction
    // Airborne Disease Prevention: Tonicshot DMG Lv10 = 280.48%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.8048,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Tonicshot Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_lan_yan() {
    eprintln!("\n=== Lan Yan ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1400.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Feathermoon Ring (Skill) — no reaction
    // Swallow-Wisp Pinion Dance: Feathermoon Ring DMG Lv10 = 228.608%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.28608,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Feathermoon Ring Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_mizuki() {
    eprintln!("\n=== Mizuki ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 1800.0,
        def: 700.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Aisa Utamakura Pilgrimage (Skill) — no reaction
    // Skill DMG Lv10 = 106.66%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 1.0666,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Anemo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage(
        "Aisa Utamakura Pilgrimage Lv10 — no reaction",
        &input,
        &enemy,
    );
}

// ── Geo characters ──────────────────────────────────────────────

#[test]
#[ignore]
fn generate_albedo() {
    eprintln!("\n=== Albedo ===");
    let stats = Stats {
        hp: 20000.0,
        atk: 1000.0,
        def: 2300.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Transient Blossom (Skill) — DEF scaling, Lv10 = 136.8%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 1.368,
        scaling_stat: ScalingStat::Def,
        damage_type: DamageType::Skill,
        element: Some(Element::Geo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Transient Blossom Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_chiori() {
    eprintln!("\n=== Chiori ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 1900.0,
        def: 800.0,
        elemental_mastery: 0.0,
        crit_rate: 0.65,
        crit_dmg: 1.30,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Tamoto Turret DMG (Skill) — ATK scaling, Lv10 = 155.952%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 1.55952,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Geo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Tamoto Turret DMG Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_gorou() {
    eprintln!("\n=== Gorou ===");
    let stats = Stats {
        hp: 15000.0,
        atk: 1300.0,
        def: 600.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Inuzaka All-Round Defense (Skill) — ATK scaling, Lv10 = 254.6%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.546,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Geo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage(
        "Inuzaka All-Round Defense Lv10 — no reaction",
        &input,
        &enemy,
    );
}

#[test]
#[ignore]
fn generate_kachina() {
    eprintln!("\n=== Kachina ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 800.0,
        def: 2000.0,
        elemental_mastery: 0.0,
        crit_rate: 0.55,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Turbo Twirly mounted DMG (Skill) — DEF scaling, Lv10 = 208.43%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.0843,
        scaling_stat: ScalingStat::Def,
        damage_type: DamageType::Skill,
        element: Some(Element::Geo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Turbo Twirly Mounted Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_navia() {
    eprintln!("\n=== Navia ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 2000.0,
        def: 800.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.50,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Rosula Shardshot Base DMG (Skill) — ATK scaling, Lv10 = 710.64%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 7.1064,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Geo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage(
        "Rosula Shardshot Base DMG Lv10 — no reaction",
        &input,
        &enemy,
    );
}

#[test]
#[ignore]
fn generate_ningguang() {
    eprintln!("\n=== Ningguang ===");
    let stats = Stats {
        hp: 14000.0,
        atk: 1500.0,
        def: 500.0,
        elemental_mastery: 0.0,
        crit_rate: 0.50,
        crit_dmg: 1.00,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Star Shatter per gem (Burst) — ATK scaling, Lv10 = 206.53%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.0653,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Geo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Star Shatter per Gem Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_noelle() {
    eprintln!("\n=== Noelle ===");
    let stats = Stats {
        hp: 18000.0,
        atk: 800.0,
        def: 2200.0,
        elemental_mastery: 0.0,
        crit_rate: 0.55,
        crit_dmg: 1.00,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Normal 1st (Burst mode) — DEF scaling, Lv10 = 167.44%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 1.6744,
        scaling_stat: ScalingStat::Def,
        damage_type: DamageType::Normal,
        element: Some(Element::Geo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage(
        "Normal 1st Hit Burst Mode Lv10 — no reaction",
        &input,
        &enemy,
    );
}

#[test]
#[ignore]
fn generate_xilonen() {
    eprintln!("\n=== Xilonen ===");
    let stats = Stats {
        hp: 20000.0,
        atk: 800.0,
        def: 2400.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Blade Roller 1-Hit (Normal) — DEF scaling, Lv10 = 59.6%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 0.596,
        scaling_stat: ScalingStat::Def,
        damage_type: DamageType::Normal,
        element: Some(Element::Geo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Blade Roller 1-Hit Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_yun_jin() {
    eprintln!("\n=== Yun Jin ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 800.0,
        def: 2100.0,
        elemental_mastery: 0.0,
        crit_rate: 0.55,
        crit_dmg: 1.00,
        energy_recharge: 1.2,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Opening Flourish Press (Skill) — DEF scaling, Lv10 = 285.6%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 2.856,
        scaling_stat: ScalingStat::Def,
        damage_type: DamageType::Skill,
        element: Some(Element::Geo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Opening Flourish Press Lv10 — no reaction", &input, &enemy);
}

#[test]
#[ignore]
fn generate_zhongli() {
    eprintln!("\n=== Zhongli ===");
    let stats = Stats {
        hp: 25000.0,
        atk: 1800.0,
        def: 800.0,
        elemental_mastery: 0.0,
        crit_rate: 0.60,
        crit_dmg: 1.20,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Planet Befall (Burst) — ATK scaling, Lv10 = 899.72%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 8.9972,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Burst,
        element: Some(Element::Geo),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Planet Befall Lv10 — no reaction", &input, &enemy);
}

// ── Traveler (Dendro) ───────────────────────────────────────────

#[test]
#[ignore]
fn generate_traveler_dendro() {
    eprintln!("\n=== Traveler (Dendro) ===");
    let stats = Stats {
        hp: 16000.0,
        atk: 1500.0,
        def: 600.0,
        elemental_mastery: 200.0,
        crit_rate: 0.55,
        crit_dmg: 1.10,
        energy_recharge: 1.0,
        dmg_bonus: 0.466,
    };
    let enemy = std_enemy();

    // Case 1: Razorgrass Blade (Skill) — no reaction, Lv10 = 414.72%
    let input = DamageInput {
        character_level: 90,
        stats: stats.clone(),
        talent_multiplier: 4.1472,
        scaling_stat: ScalingStat::Atk,
        damage_type: DamageType::Skill,
        element: Some(Element::Dendro),
        reaction: None,
        reaction_bonus: 0.0,
    };
    print_damage("Razorgrass Blade Lv10 — no reaction", &input, &enemy);

    // Case 2: Razorgrass Blade + Spread
    let input_spread = DamageInput {
        reaction: Some(Reaction::Spread),
        ..input
    };
    print_damage("Razorgrass Blade Lv10 — Spread", &input_spread, &enemy);
}
