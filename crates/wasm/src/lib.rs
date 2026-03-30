mod convert;

use serde::Serialize;
use wasm_bindgen::prelude::*;

/// Serialize a value to JsValue, with None mapped to null (not undefined).
fn to_js<T: Serialize>(value: &T) -> Result<JsValue, JsError> {
    let serializer = serde_wasm_bindgen::Serializer::new().serialize_missing_as_null(true);
    value
        .serialize(&serializer)
        .map_err(|e| JsError::new(&e.to_string()))
}

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
pub fn find_character(id: &str) -> Result<JsValue, JsError> {
    match genshin_calc_data::find_character(id) {
        Some(c) => to_js(c),
        None => Ok(JsValue::NULL),
    }
}

/// Finds a weapon by ID (lowercase, e.g. "wolfs_gravestone").
/// Returns the weapon data as a JS object, or null if not found.
#[wasm_bindgen]
pub fn find_weapon(id: &str) -> Result<JsValue, JsError> {
    match genshin_calc_data::find_weapon(id) {
        Some(w) => to_js(w),
        None => Ok(JsValue::NULL),
    }
}

/// Finds an artifact set by ID (lowercase, e.g. "crimson_witch").
/// Returns the artifact set data as a JS object, or null if not found.
#[wasm_bindgen]
pub fn find_artifact_set(id: &str) -> Result<JsValue, JsError> {
    match genshin_calc_data::find_artifact_set(id) {
        Some(a) => to_js(a),
        None => Ok(JsValue::NULL),
    }
}

/// Finds an enemy by ID (lowercase, e.g. "hilichurl").
/// Returns the enemy data as a JS object, or null if not found.
#[wasm_bindgen]
pub fn find_enemy(id: &str) -> Result<JsValue, JsError> {
    match genshin_calc_data::find_enemy(id) {
        Some(e) => to_js(e),
        None => Ok(JsValue::NULL),
    }
}

/// Returns all characters with the given element.
/// Element is a lowercase string: "pyro", "hydro", "electro", "cryo", "anemo", "geo", "dendro".
#[wasm_bindgen]
pub fn characters_by_element(element: &str) -> Result<JsValue, JsError> {
    let elem = convert::parse_element(element).map_err(|e| JsError::new(&e))?;
    let chars = genshin_calc_data::characters_by_element(elem);
    to_js(&chars)
}

/// Returns all weapons of the given type.
/// Weapon type is a lowercase string: "sword", "claymore", "polearm", "bow", "catalyst".
#[wasm_bindgen]
pub fn weapons_by_type(weapon_type: &str) -> Result<JsValue, JsError> {
    let wt = convert::parse_weapon_type(weapon_type).map_err(|e| JsError::new(&e))?;
    let weapons = genshin_calc_data::weapons_by_type(wt);
    to_js(&weapons)
}

/// Calculates standard damage (ATK/HP/DEF scaling with crit, defense, resistance).
///
/// # Arguments
/// * `input` - DamageInput as a JS object (PascalCase enum variants, e.g. element: "Pyro")
/// * `enemy` - Enemy as a JS object
///
/// # Returns
/// DamageResult as a JS object with non_crit, crit, average, reaction fields.
///
/// # Errors
/// Throws JsError on invalid input or calculation error.
#[wasm_bindgen]
pub fn calculate_damage(input: JsValue, enemy: JsValue) -> Result<JsValue, JsError> {
    let input: genshin_calc_core::DamageInput = serde_wasm_bindgen::from_value(input)
        .map_err(|e| JsError::new(&format!("Invalid input: {e}")))?;
    let enemy: genshin_calc_core::Enemy = serde_wasm_bindgen::from_value(enemy)
        .map_err(|e| JsError::new(&format!("Invalid enemy: {e}")))?;
    let result = genshin_calc_core::calculate_damage(&input, &enemy)
        .map_err(|e| JsError::new(&e.to_string()))?;
    to_js(&result)
}

/// Calculates transformative reaction damage (overloaded, superconduct, swirl, etc.).
///
/// # Arguments
/// * `input` - TransformativeInput as a JS object
/// * `enemy` - Enemy as a JS object
///
/// # Returns
/// TransformativeResult as a JS object with damage and damage_element fields.
#[wasm_bindgen]
pub fn calculate_transformative(input: JsValue, enemy: JsValue) -> Result<JsValue, JsError> {
    let input: genshin_calc_core::TransformativeInput = serde_wasm_bindgen::from_value(input)
        .map_err(|e| JsError::new(&format!("Invalid input: {e}")))?;
    let enemy: genshin_calc_core::Enemy = serde_wasm_bindgen::from_value(enemy)
        .map_err(|e| JsError::new(&format!("Invalid enemy: {e}")))?;
    let result = genshin_calc_core::calculate_transformative(&input, &enemy)
        .map_err(|e| JsError::new(&e.to_string()))?;
    to_js(&result)
}

/// Calculates lunar reaction damage (Nod-Krai exclusive crittable reactions).
///
/// # Arguments
/// * `input` - LunarInput as a JS object
/// * `enemy` - Enemy as a JS object
///
/// # Returns
/// LunarResult as a JS object with non_crit, crit, average, damage_element fields.
#[wasm_bindgen]
pub fn calculate_lunar(input: JsValue, enemy: JsValue) -> Result<JsValue, JsError> {
    let input: genshin_calc_core::LunarInput = serde_wasm_bindgen::from_value(input)
        .map_err(|e| JsError::new(&format!("Invalid input: {e}")))?;
    let enemy: genshin_calc_core::Enemy = serde_wasm_bindgen::from_value(enemy)
        .map_err(|e| JsError::new(&format!("Invalid enemy: {e}")))?;
    let result = genshin_calc_core::calculate_lunar(&input, &enemy)
        .map_err(|e| JsError::new(&e.to_string()))?;
    to_js(&result)
}

/// Resolves team buffs and returns final stats for the target member.
///
/// # Arguments
/// * `members` - Array of TeamMember objects
/// * `target_index` - Index of the DPS/target member (0-based)
///
/// # Returns
/// Stats as a JS object with hp, atk, def, elemental_mastery, crit_rate, crit_dmg, energy_recharge, dmg_bonus.
#[wasm_bindgen]
pub fn resolve_team_stats(members: JsValue, target_index: u32) -> Result<JsValue, JsError> {
    let members: Vec<genshin_calc_core::TeamMember> = serde_wasm_bindgen::from_value(members)
        .map_err(|e| JsError::new(&format!("Invalid members: {e}")))?;
    let result = genshin_calc_core::resolve_team_stats(&members, target_index as usize)
        .map_err(|e| JsError::new(&e.to_string()))?;
    to_js(&result)
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

    #[test]
    fn test_damage_input_serde_roundtrip() {
        use genshin_calc_core::*;
        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                hp: 20000.0,
                atk: 2000.0,
                def: 800.0,
                elemental_mastery: 100.0,
                crit_rate: 0.75,
                crit_dmg: 1.50,
                energy_recharge: 1.20,
                dmg_bonus: 0.466,
            },
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let json = serde_json::to_string(&input).unwrap();
        let back: DamageInput = serde_json::from_str(&json).unwrap();
        assert_eq!(input.character_level, back.character_level);
        assert!((input.talent_multiplier - back.talent_multiplier).abs() < 1e-10);
    }

    #[test]
    fn test_damage_calculation_via_core() {
        use genshin_calc_core::*;
        let input = DamageInput {
            character_level: 90,
            stats: Stats {
                hp: 20000.0,
                atk: 2000.0,
                def: 800.0,
                elemental_mastery: 100.0,
                crit_rate: 0.75,
                crit_dmg: 1.50,
                energy_recharge: 1.20,
                dmg_bonus: 0.466,
            },
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
            flat_dmg: 0.0,
        };
        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };
        let result = calculate_damage(&input, &enemy).unwrap();
        assert!(result.average > 0.0);
        assert!(result.crit > result.non_crit);
    }

    #[test]
    fn test_transformative_calculation_via_core() {
        use genshin_calc_core::*;
        let input = TransformativeInput {
            character_level: 90,
            elemental_mastery: 200.0,
            reaction: Reaction::Overloaded,
            reaction_bonus: 0.0,
        };
        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };
        let result = calculate_transformative(&input, &enemy).unwrap();
        assert!(result.damage > 0.0);
    }

    #[test]
    fn test_lunar_calculation_via_core() {
        use genshin_calc_core::*;
        let input = LunarInput {
            character_level: 90,
            elemental_mastery: 200.0,
            reaction: Reaction::LunarElectroCharged,
            reaction_bonus: 0.0,
            crit_rate: 0.5,
            crit_dmg: 1.0,
            base_dmg_bonus: 0.0,
        };
        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };
        let result = calculate_lunar(&input, &enemy).unwrap();
        assert!(result.average > 0.0);
    }

    #[test]
    fn test_resolve_team_stats_via_core() {
        use genshin_calc_core::*;
        let dps = TeamMember {
            element: Element::Pyro,
            weapon_type: WeaponType::Claymore,
            stats: StatProfile {
                base_atk: 900.0,
                crit_rate: 0.60,
                crit_dmg: 1.50,
                energy_recharge: 1.0,
                ..Default::default()
            },
            buffs_provided: vec![],
            is_moonsign: false,
        };
        let result = resolve_team_stats(&[dps], 0).unwrap();
        assert!(result.atk > 0.0);
    }

    #[test]
    fn test_reaction_swirl_serde() {
        use genshin_calc_core::Reaction;
        let swirl = Reaction::Swirl(genshin_calc_core::Element::Pyro);
        let json = serde_json::to_string(&swirl).unwrap();
        assert_eq!(json, r#"{"Swirl":"Pyro"}"#);
        let back: Reaction = serde_json::from_str(&json).unwrap();
        assert_eq!(swirl, back);
    }
}
