mod test_types;

use std::fs;

use genshin_calc_core::damage::{DamageInput, calculate_damage};
use genshin_calc_core::lunar::{LunarInput, calculate_lunar};
use genshin_calc_core::transformative::{TransformativeInput, calculate_transformative};

use test_types::*;

const DEFAULT_TOLERANCE: f64 = 0.01;

fn assert_approx(actual: f64, expected: f64, tolerance: f64, context: &str) {
    let diff = (actual - expected).abs();
    assert!(
        diff < tolerance,
        "{context}: expected {expected}, got {actual} (diff {diff}, tolerance {tolerance})"
    );
}

fn run_damage_case(character_name: &str, case: &DamageCase) {
    let tolerance = case.tolerance.unwrap_or(DEFAULT_TOLERANCE);
    let ctx = format!("{character_name} - {}", case.name);

    let input = DamageInput {
        character_level: case.character_level,
        stats: to_stats(&case.stats),
        talent_multiplier: case.talent_multiplier,
        scaling_stat: parse_scaling_stat(&case.scaling_stat),
        damage_type: parse_damage_type(&case.damage_type),
        element: case.element.as_deref().map(parse_element),
        reaction: case.reaction.as_deref().map(parse_reaction),
        reaction_bonus: case.reaction_bonus,
        flat_dmg: 0.0,
    };
    let enemy = to_enemy(&case.enemy);
    let result = calculate_damage(&input, &enemy).unwrap_or_else(|e| {
        panic!("{ctx}: calculate_damage failed: {e}");
    });

    assert_approx(
        result.non_crit,
        case.expected.non_crit,
        tolerance,
        &format!("{ctx} non_crit"),
    );
    assert_approx(
        result.crit,
        case.expected.crit,
        tolerance,
        &format!("{ctx} crit"),
    );
    assert_approx(
        result.average,
        case.expected.average,
        tolerance,
        &format!("{ctx} average"),
    );
}

fn run_transformative_case(character_name: &str, case: &TransformativeCase) {
    let tolerance = case.tolerance.unwrap_or(DEFAULT_TOLERANCE);
    let ctx = format!("{character_name} - {}", case.name);

    let input = TransformativeInput {
        character_level: case.character_level,
        elemental_mastery: case.elemental_mastery,
        reaction: parse_reaction(&case.reaction),
        reaction_bonus: case.reaction_bonus,
    };
    let enemy = to_enemy(&case.enemy);
    let result = calculate_transformative(&input, &enemy).unwrap_or_else(|e| {
        panic!("{ctx}: calculate_transformative failed: {e}");
    });

    assert_approx(
        result.damage,
        case.expected.damage,
        tolerance,
        &format!("{ctx} damage"),
    );
}

fn run_lunar_case(character_name: &str, case: &LunarCase) {
    let tolerance = case.tolerance.unwrap_or(DEFAULT_TOLERANCE);
    let ctx = format!("{character_name} - {}", case.name);

    let input = LunarInput {
        character_level: case.character_level,
        elemental_mastery: case.elemental_mastery,
        reaction: parse_reaction(&case.reaction),
        reaction_bonus: case.reaction_bonus,
        crit_rate: case.crit_rate,
        crit_dmg: case.crit_dmg,
        base_dmg_bonus: 0.0,
    };
    let enemy = to_enemy(&case.enemy);
    let result = calculate_lunar(&input, &enemy).unwrap_or_else(|e| {
        panic!("{ctx}: calculate_lunar failed: {e}");
    });

    assert_approx(
        result.non_crit,
        case.expected.non_crit,
        tolerance,
        &format!("{ctx} non_crit"),
    );
    assert_approx(
        result.crit,
        case.expected.crit,
        tolerance,
        &format!("{ctx} crit"),
    );
    assert_approx(
        result.average,
        case.expected.average,
        tolerance,
        &format!("{ctx} average"),
    );
}

fn run_character(data: &CharacterTestData) {
    let char_name = &data.character.name;

    for case in &data.cases {
        match case {
            TestCase::Normal(c) | TestCase::Amplifying(c) | TestCase::Catalyze(c) => {
                run_damage_case(char_name, c);
            }
            TestCase::Transformative(c) => {
                run_transformative_case(char_name, c);
            }
            TestCase::Lunar(c) => {
                run_lunar_case(char_name, c);
            }
        }
    }
}

#[test]
fn test_all_characters() {
    let pattern = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/characters/*.toml");
    let paths: Vec<_> = glob::glob(pattern)
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
        .collect();

    assert!(
        !paths.is_empty(),
        "No TOML files found in tests/data/characters/"
    );

    let mut total_cases = 0;
    for path in &paths {
        let content = fs::read_to_string(path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {e}", path.display()));
        let data: CharacterTestData = toml::from_str(&content)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {e}", path.display()));
        total_cases += data.cases.len();
        run_character(&data);
    }

    eprintln!(
        "Character verification: {} characters, {} cases — all passed",
        paths.len(),
        total_cases
    );
}
