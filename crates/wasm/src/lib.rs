mod convert;

use wasm_bindgen::prelude::*;

/// Initialize panic hook for better error messages in browser console.
#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

/// Returns the current game data version.
#[wasm_bindgen]
pub fn game_version() -> String {
    genshin_calc_data::GAME_VERSION.to_string()
}

/// Finds a character by ID (lowercase, e.g. "diluc", "hu_tao").
/// Returns the character data as a JS object, or null if not found.
#[wasm_bindgen]
pub fn find_character(id: &str) -> JsValue {
    match genshin_calc_data::find_character(id) {
        Some(c) => serde_wasm_bindgen::to_value(c).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Finds a weapon by ID (lowercase, e.g. "wolfs_gravestone").
/// Returns the weapon data as a JS object, or null if not found.
#[wasm_bindgen]
pub fn find_weapon(id: &str) -> JsValue {
    match genshin_calc_data::find_weapon(id) {
        Some(w) => serde_wasm_bindgen::to_value(w).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Finds an artifact set by ID (lowercase, e.g. "crimson_witch").
/// Returns the artifact set data as a JS object, or null if not found.
#[wasm_bindgen]
pub fn find_artifact_set(id: &str) -> JsValue {
    match genshin_calc_data::find_artifact_set(id) {
        Some(a) => serde_wasm_bindgen::to_value(a).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Finds an enemy by ID (lowercase, e.g. "hilichurl").
/// Returns the enemy data as a JS object, or null if not found.
#[wasm_bindgen]
pub fn find_enemy(id: &str) -> JsValue {
    match genshin_calc_data::find_enemy(id) {
        Some(e) => serde_wasm_bindgen::to_value(e).unwrap_or(JsValue::NULL),
        None => JsValue::NULL,
    }
}

/// Returns all characters with the given element.
/// Element is a lowercase string: "pyro", "hydro", "electro", "cryo", "anemo", "geo", "dendro".
#[wasm_bindgen]
pub fn characters_by_element(element: &str) -> Result<JsValue, JsError> {
    let elem = convert::parse_element(element).map_err(|e| JsError::new(&e))?;
    let chars = genshin_calc_data::characters_by_element(elem);
    serde_wasm_bindgen::to_value(&chars).map_err(|e| JsError::new(&e.to_string()))
}

/// Returns all weapons of the given type.
/// Weapon type is a lowercase string: "sword", "claymore", "polearm", "bow", "catalyst".
#[wasm_bindgen]
pub fn weapons_by_type(weapon_type: &str) -> Result<JsValue, JsError> {
    let wt = convert::parse_weapon_type(weapon_type).map_err(|e| JsError::new(&e))?;
    let weapons = genshin_calc_data::weapons_by_type(wt);
    serde_wasm_bindgen::to_value(&weapons).map_err(|e| JsError::new(&e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::convert;

    #[test]
    fn test_game_version_returns_string() {
        let v = game_version();
        assert!(!v.is_empty());
    }

    #[test]
    fn test_find_character_serde_roundtrip() {
        let diluc = genshin_calc_data::find_character("diluc").unwrap();
        let json = serde_json::to_value(diluc).unwrap();
        assert_eq!(json["name"], "Diluc");
        assert_eq!(json["element"], "Pyro");
    }

    #[test]
    fn test_find_character_not_found() {
        assert!(genshin_calc_data::find_character("nonexistent").is_none());
    }

    #[test]
    fn test_find_weapon_serde_roundtrip() {
        let weapon = genshin_calc_data::find_weapon("wolfs_gravestone").unwrap();
        let json = serde_json::to_value(weapon).unwrap();
        assert_eq!(json["id"], "wolfs_gravestone");
    }

    #[test]
    fn test_find_artifact_set_serde_roundtrip() {
        let set = genshin_calc_data::find_artifact_set("crimson_witch").unwrap();
        let json = serde_json::to_value(set).unwrap();
        assert_eq!(json["id"], "crimson_witch");
    }

    #[test]
    fn test_find_enemy_serde_roundtrip() {
        let enemy = genshin_calc_data::find_enemy("hilichurl").unwrap();
        let json = serde_json::to_value(enemy).unwrap();
        assert_eq!(json["id"], "hilichurl");
    }

    #[test]
    fn test_characters_by_element_pyro() {
        let element = convert::parse_element("pyro").unwrap();
        let chars = genshin_calc_data::characters_by_element(element);
        assert!(chars.len() > 10);
    }

    #[test]
    fn test_weapons_by_type_sword() {
        let wt = convert::parse_weapon_type("sword").unwrap();
        let weapons = genshin_calc_data::weapons_by_type(wt);
        assert!(!weapons.is_empty());
    }
}
