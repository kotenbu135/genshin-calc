use genshin_calc_core::Element;
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

/// GOODキャラクターキーからルックアップ（Traveler対応）。
/// "Traveler" キーの場合、traveler_element で指定した要素の旅人を返す。
pub fn lookup_character_with_traveler(
    good_key: &str,
    traveler_element: Option<Element>,
) -> Option<&'static CharacterData> {
    if good_key == "Traveler" || good_key == "traveler" {
        let element = traveler_element?;
        let id = format!("traveler_{}", element_to_suffix(element));
        return genshin_calc_data::find_character(&id);
    }
    lookup_character(good_key)
}

fn element_to_suffix(element: Element) -> &'static str {
    match element {
        Element::Pyro => "pyro",
        Element::Hydro => "hydro",
        Element::Electro => "electro",
        Element::Cryo => "cryo",
        Element::Dendro => "dendro",
        Element::Anemo => "anemo",
        Element::Geo => "geo",
    }
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
