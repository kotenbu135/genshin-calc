use genshin_calc_data::types::{ArtifactSet, CharacterData, WeaponData};

/// PascalCase を snake_case に変換
pub fn pascal_to_snake(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            if i > 0 {
                result.push('_');
            }
            result.push(ch.to_lowercase().next().unwrap());
        } else {
            result.push(ch);
        }
    }
    result
}

/// GOODキャラクターキーから内部CharacterDataをルックアップ
pub fn lookup_character(good_key: &str) -> Option<&'static CharacterData> {
    let snake = pascal_to_snake(good_key);
    if let Some(c) = genshin_calc_data::find_character(&snake) {
        return Some(c);
    }
    if let Some(alias) = character_alias(&snake) {
        return genshin_calc_data::find_character(alias);
    }
    None
}

fn character_alias(snake_key: &str) -> Option<&'static str> {
    static ALIASES: &[(&str, &str)] = &[
        ("kaedehara_kazuha", "kazuha"),
        ("shikanoin_heizou", "heizou"),
        ("yumemizuki_mizuki", "mizuki"),
    ];
    ALIASES
        .iter()
        .find(|(k, _)| *k == snake_key)
        .map(|(_, v)| *v)
}

/// GOOD武器キーから内部WeaponDataをルックアップ
pub fn lookup_weapon(good_key: &str) -> Option<&'static WeaponData> {
    let snake = pascal_to_snake(good_key);
    genshin_calc_data::find_weapon(&snake)
}

/// GOOD聖遺物セットキーから内部ArtifactSetをルックアップ
pub fn lookup_artifact_set(good_key: &str) -> Option<&'static ArtifactSet> {
    let snake = pascal_to_snake(good_key);
    if let Some(set) = genshin_calc_data::find_artifact_set(&snake) {
        return Some(set);
    }
    if let Some(alias) = artifact_alias(&snake) {
        return genshin_calc_data::find_artifact_set(alias);
    }
    None
}

fn artifact_alias(snake_key: &str) -> Option<&'static str> {
    static ALIASES: &[(&str, &str)] = &[
        ("crimson_witch_of_flames", "crimson_witch"),
        ("the_exile", "exile"),
    ];
    ALIASES
        .iter()
        .find(|(k, _)| *k == snake_key)
        .map(|(_, v)| *v)
}
