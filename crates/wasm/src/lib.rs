mod convert;

use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Clone, Deserialize)]
struct WasmManualActivation {
    name: String,
    active: bool,
    stacks: Option<u8>,
}

fn convert_activations(
    input: &[WasmManualActivation],
) -> Vec<(&str, genshin_calc_data::buff::ManualActivation)> {
    input
        .iter()
        .filter(|a| a.active)
        .map(|a| {
            let activation = match a.stacks {
                Some(n) => genshin_calc_data::buff::ManualActivation::Stacks(n),
                None => genshin_calc_data::buff::ManualActivation::Active,
            };
            (a.name.as_str(), activation)
        })
        .collect()
}

fn make_import_options(traveler_element: Option<String>) -> genshin_calc_good::ImportOptions {
    genshin_calc_good::ImportOptions {
        traveler_element: traveler_element.and_then(|e| convert::parse_element(&e).ok()),
    }
}

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

/// Resolves team buffs and returns detailed result for the target member.
///
/// # Arguments
/// * `members` - Array of TeamMember objects
/// * `target_index` - Index of the DPS/target member (0-based)
///
/// # Returns
/// TeamResolveResult as a JS object with:
/// - `base_stats`: Stats before team buffs
/// - `applied_buffs`: All resolved buffs applied to target
/// - `resonances`: Active elemental resonances
/// - `final_stats`: Stats after all unconditional buffs
/// - `damage_context`: Attack-type DMG bonuses, flat DMG, reaction bonuses
/// - `enemy_debuffs`: Enemy resistance and DEF reduction from team debuffs
#[wasm_bindgen]
pub fn resolve_team_stats(members: JsValue, target_index: u32) -> Result<JsValue, JsError> {
    let members: Vec<genshin_calc_core::TeamMember> = serde_wasm_bindgen::from_value(members)
        .map_err(|e| JsError::new(&format!("Invalid members: {e}")))?;
    let result = genshin_calc_core::resolve_team_stats(&members, target_index as usize)
        .map_err(|e| JsError::new(&e.to_string()))?;
    to_js(&result)
}

/// Calculates final stats for a character from a GOOD JSON export.
///
/// # Arguments
/// * `json` - GOOD format JSON string (same as `import_good`)
/// * `character_id` - Character ID to find (e.g. "hu_tao")
///
/// # Returns
/// Stats with per-element DMG bonuses in separate fields.
/// Returns null if the character is not found in the GOOD data.
#[wasm_bindgen]
pub fn build_stats_from_good(
    json: &str,
    character_id: &str,
    traveler_element: Option<String>,
) -> Result<JsValue, JsError> {
    let options = make_import_options(traveler_element);
    let import = genshin_calc_good::import_good_with_options(json, &options)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let build = import
        .builds
        .iter()
        .find(|b| b.character.id == character_id);
    match build {
        Some(b) => {
            let profile = genshin_calc_good::build_stat_profile(b);
            let stats = genshin_calc_core::combine_stats(&profile)
                .map_err(|e| JsError::new(&e.to_string()))?;
            to_js(&stats)
        }
        None => Ok(JsValue::NULL),
    }
}

/// Calculates final stats for a character build with conditional buff activations.
///
/// Unlike `build_stats_from_good`, this function resolves weapon and artifact
/// conditional buffs (toggles, stacks) via TeamMemberBuilder.
///
/// # Arguments
/// * `json` - GOOD format JSON string
/// * `character_id` - Character ID (e.g. "diluc")
/// * `weapon_activations` - JS array of {name, active, stacks?} for weapon buffs
/// * `artifact_activations` - JS array of {name, active, stacks?} for artifact set buffs
///
/// # Returns
/// Stats as a JS object, or null if character not found.
#[wasm_bindgen]
pub fn build_stats(
    json: &str,
    character_id: &str,
    weapon_activations: JsValue,
    artifact_activations: JsValue,
    traveler_element: Option<String>,
) -> Result<JsValue, JsError> {
    let options = make_import_options(traveler_element);
    let import = genshin_calc_good::import_good_with_options(json, &options)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let build = import
        .builds
        .iter()
        .find(|b| b.character.id == character_id);
    match build {
        Some(b) => {
            let w_acts: Vec<WasmManualActivation> =
                serde_wasm_bindgen::from_value(weapon_activations).unwrap_or_default();
            let a_acts: Vec<WasmManualActivation> =
                serde_wasm_bindgen::from_value(artifact_activations).unwrap_or_default();
            let w_converted = convert_activations(&w_acts);
            let a_converted = convert_activations(&a_acts);
            let builder = genshin_calc_good::to_team_member_builder(b, &w_converted, &a_converted)
                .map_err(|e| JsError::new(&e.to_string()))?;
            let member = builder.build().map_err(|e| JsError::new(&e.to_string()))?;
            let result = genshin_calc_core::resolve_team_stats(&[member], 0)
                .map_err(|e| JsError::new(&e.to_string()))?;
            let stats = result.final_stats;
            to_js(&stats)
        }
        None => Ok(JsValue::NULL),
    }
}

/// Applies team enemy debuffs to a base enemy for a specific damage element.
///
/// # Arguments
/// * `enemy` - Base Enemy as a JS object
/// * `debuffs` - EnemyDebuffs from resolve_team_stats result
/// * `element` - Element string in PascalCase ("Pyro", "Hydro", etc.) or null for physical.
///   Uses the same format as DamageInput.element (serde PascalCase).
///
/// # Returns
/// New Enemy with debuffs applied.
#[wasm_bindgen]
pub fn apply_team_debuffs(
    enemy: JsValue,
    debuffs: JsValue,
    element: JsValue,
) -> Result<JsValue, JsError> {
    let enemy: genshin_calc_core::Enemy = serde_wasm_bindgen::from_value(enemy)
        .map_err(|e| JsError::new(&format!("Invalid enemy: {e}")))?;
    let debuffs: genshin_calc_core::EnemyDebuffs = serde_wasm_bindgen::from_value(debuffs)
        .map_err(|e| JsError::new(&format!("Invalid debuffs: {e}")))?;
    let elem: Option<genshin_calc_core::Element> = if element.is_null() || element.is_undefined() {
        None
    } else {
        let e: genshin_calc_core::Element = serde_wasm_bindgen::from_value(element)
            .map_err(|e| JsError::new(&format!("Invalid element: {e}")))?;
        Some(e)
    };
    let result = genshin_calc_core::apply_debuffs_to_enemy(&enemy, &debuffs, elem);
    to_js(&result)
}

/// Evaluates character talent buffs for team use.
///
/// Returns `Vec<ResolvedBuff>` for the specified character. Characters without
/// defined talent buffs return an empty array.
#[wasm_bindgen]
pub fn get_character_team_buffs(
    json: &str,
    character_id: &str,
    constellation: u32,
    talent_levels: Vec<u32>,
    traveler_element: Option<String>,
) -> Result<JsValue, JsError> {
    if talent_levels.len() != 3 {
        return Err(JsError::new(
            "talent_levels must have exactly 3 elements [auto, skill, burst]",
        ));
    }
    for (i, &tl) in talent_levels.iter().enumerate() {
        if tl == 0 || tl > 15 {
            return Err(JsError::new(&format!(
                "talent_levels[{}] must be 1-15, got {}",
                i, tl
            )));
        }
    }
    if constellation > 6 {
        return Err(JsError::new(&format!(
            "constellation must be 0-6, got {}",
            constellation
        )));
    }

    let options = make_import_options(traveler_element);
    let import = genshin_calc_good::import_good_with_options(json, &options)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let build = import
        .builds
        .iter()
        .find(|b| b.character.id == character_id)
        .ok_or_else(|| {
            JsError::new(&format!(
                "Character '{}' not found in GOOD data",
                character_id
            ))
        })?;

    let tl: [u8; 3] = [
        talent_levels[0] as u8,
        talent_levels[1] as u8,
        talent_levels[2] as u8,
    ];
    let buffs = genshin_calc_good::evaluate_talent_buffs(build, constellation as u8, &tl);
    to_js(&buffs)
}

/// Builds a TeamMember from GOOD JSON with conditional buff activations.
///
/// Returns a fully resolved `TeamMember` (element, weapon_type, stats, buffs_provided,
/// is_moonsign) that can be passed directly to `resolve_team_stats`.
///
/// This bridges the gap between `build_member_stats` (no activations) and `build_stats`
/// (returns final Stats, not TeamMember). Use this when building a team for
/// `resolve_team_stats` with weapon/artifact conditional buffs enabled.
///
/// # Arguments
/// * `json` - GOOD format JSON string
/// * `character_id` - Character ID (e.g. "hu_tao")
/// * `weapon_activations` - JS array of {name, active, stacks?} for weapon conditional buffs
/// * `artifact_activations` - JS array of {name, active, stacks?} for artifact set conditional buffs
///
/// # Returns
/// TeamMember as a JS object, or throws if character not found or weapon missing.
#[wasm_bindgen]
pub fn build_team_member(
    json: &str,
    character_id: &str,
    weapon_activations: JsValue,
    artifact_activations: JsValue,
    traveler_element: Option<String>,
) -> Result<JsValue, JsError> {
    let options = make_import_options(traveler_element);
    let import = genshin_calc_good::import_good_with_options(json, &options)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let build = import
        .builds
        .iter()
        .find(|b| b.character.id == character_id)
        .ok_or_else(|| {
            JsError::new(&format!(
                "Character '{}' not found in GOOD data",
                character_id
            ))
        })?;

    let w_acts: Vec<WasmManualActivation> =
        serde_wasm_bindgen::from_value(weapon_activations).unwrap_or_default();
    let a_acts: Vec<WasmManualActivation> =
        serde_wasm_bindgen::from_value(artifact_activations).unwrap_or_default();
    let w_converted = convert_activations(&w_acts);
    let a_converted = convert_activations(&a_acts);

    let builder = genshin_calc_good::to_team_member_builder(build, &w_converted, &a_converted)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let member = builder.build().map_err(|e| JsError::new(&e.to_string()))?;
    to_js(&member)
}

/// Builds a StatProfile from GOOD JSON for use with `resolve_team_stats`.
///
/// Unlike `build_stats_from_good` which returns final `Stats`, this returns
/// the decomposed `StatProfile` (base values, percentages, flats) that
/// `resolve_team_stats` requires as input.
///
/// Note: This function does not support conditional buff activations.
/// Use `build_team_member` instead for full activation support.
#[wasm_bindgen]
pub fn build_member_stats(
    json: &str,
    character_id: &str,
    traveler_element: Option<String>,
) -> Result<JsValue, JsError> {
    let options = make_import_options(traveler_element);
    let import = genshin_calc_good::import_good_with_options(json, &options)
        .map_err(|e| JsError::new(&e.to_string()))?;
    let build = import
        .builds
        .iter()
        .find(|b| b.character.id == character_id)
        .ok_or_else(|| {
            JsError::new(&format!(
                "Character '{}' not found in GOOD data",
                character_id
            ))
        })?;

    let profile = genshin_calc_good::build_stat_profile(build);
    to_js(&profile)
}

/// Imports a GOOD (Genshin Open Object Description) JSON string and returns parsed character builds.
///
/// # Arguments
/// * `json` - GOOD format JSON string (e.g. exported from Genshin Optimizer, Scanner tools)
///
/// # Returns
/// GoodImport object with source, version, builds array, and warnings.
///
/// # Errors
/// Throws JsError on invalid JSON or unsupported GOOD format.
#[wasm_bindgen]
pub fn import_good(json: &str) -> Result<JsValue, JsError> {
    let result = genshin_calc_good::import_good(json).map_err(|e| JsError::new(&e.to_string()))?;
    to_js(&result)
}

/// Imports GOOD JSON with Traveler element support.
///
/// # Arguments
/// * `json` - GOOD format JSON string
/// * `traveler_element` - Optional element string ("pyro", "dendro", etc.) for Traveler
#[wasm_bindgen]
pub fn import_good_with_options(
    json: &str,
    traveler_element: Option<String>,
) -> Result<JsValue, JsError> {
    let options = make_import_options(traveler_element);
    let result = genshin_calc_good::import_good_with_options(json, &options)
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
                ..Default::default()
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
                ..Default::default()
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
        assert!(result.final_stats.atk > 0.0);
    }

    #[test]
    fn test_import_good_via_core() {
        let json = include_str!("../../good/tests/data/minimal.json");
        let result = genshin_calc_good::import_good(json).unwrap();
        assert_eq!(result.source, "TestScanner");
        assert_eq!(result.version, 2);
        assert!(!result.builds.is_empty());
        assert_eq!(result.builds[0].character.name, "Hu Tao");
    }

    #[test]
    fn test_import_good_invalid_json() {
        let result = genshin_calc_good::import_good("not json");
        assert!(result.is_err());
    }

    #[test]
    fn test_build_stats_basic() {
        let char = genshin_calc_data::find_character("hu_tao").unwrap();
        let weapon = genshin_calc_data::find_weapon("staff_of_homa").unwrap();
        let build = genshin_calc_good::CharacterBuild {
            character: char,
            level: 90,
            constellation: 0,
            talent_levels: [10, 10, 10],
            weapon: Some(genshin_calc_good::WeaponBuild {
                weapon,
                level: 90,
                refinement: 1,
            }),
            artifacts: genshin_calc_good::ArtifactsBuild {
                sets: vec![],
                stats: genshin_calc_core::StatProfile::default(),
            },
        };
        let builder = genshin_calc_good::to_team_member_builder(&build, &[], &[]).unwrap();
        let member = builder.build().unwrap();
        let result = genshin_calc_core::resolve_team_stats(&[member], 0).unwrap();
        let stats = result.final_stats;
        assert!(stats.atk > 0.0, "ATK should be positive");
        assert!(stats.hp > 0.0, "HP should be positive");
    }

    #[test]
    fn test_build_stats_with_artifact_activation() {
        use genshin_calc_data::buff::ManualActivation;

        let char = genshin_calc_data::find_character("diluc").unwrap();
        let weapon = genshin_calc_data::find_weapon("wolfs_gravestone").unwrap();
        let cw = genshin_calc_data::find_artifact_set("crimson_witch").unwrap();
        let build = genshin_calc_good::CharacterBuild {
            character: char,
            level: 90,
            constellation: 0,
            talent_levels: [10, 10, 10],
            weapon: Some(genshin_calc_good::WeaponBuild {
                weapon,
                level: 90,
                refinement: 1,
            }),
            artifacts: genshin_calc_good::ArtifactsBuild {
                sets: vec![genshin_calc_good::ArtifactSetEntry {
                    set: cw,
                    piece_count: 4,
                }],
                stats: genshin_calc_core::StatProfile::default(),
            },
        };

        // Without activation
        let builder_no = genshin_calc_good::to_team_member_builder(&build, &[], &[]).unwrap();
        let member_no = builder_no.build().unwrap();
        let stats_no = genshin_calc_core::resolve_team_stats(&[member_no], 0)
            .unwrap()
            .final_stats;

        // With 2 stacks of CW pyro buff
        let artifact_acts = [("cwof_pyro_stacks", ManualActivation::Stacks(2))];
        let builder_with =
            genshin_calc_good::to_team_member_builder(&build, &[], &artifact_acts).unwrap();
        let member_with = builder_with.build().unwrap();
        let stats_with = genshin_calc_core::resolve_team_stats(&[member_with], 0)
            .unwrap()
            .final_stats;

        // With stacks, pyro_dmg_bonus should be higher by 0.15 (2 × 0.075)
        let diff = stats_with.pyro_dmg_bonus - stats_no.pyro_dmg_bonus;
        assert!(
            (diff - 0.15).abs() < 1e-10,
            "CW 4pc 2 stacks should add 0.15 pyro_dmg_bonus, got diff={}",
            diff
        );
    }

    #[test]
    fn test_apply_debuffs_to_enemy_via_core() {
        use genshin_calc_core::*;

        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };
        let debuffs = EnemyDebuffs {
            pyro_res_reduction: 0.40,
            def_reduction: 0.15,
            ..Default::default()
        };
        let result = apply_debuffs_to_enemy(&enemy, &debuffs, Some(Element::Pyro));
        assert!((result.resistance - (-0.30)).abs() < 1e-6);
        assert!((result.def_reduction - 0.15).abs() < 1e-6);
    }

    #[test]
    fn test_build_team_member_without_activations() {
        let char = genshin_calc_data::find_character("hu_tao").unwrap();
        let weapon = genshin_calc_data::find_weapon("staff_of_homa").unwrap();
        let build = genshin_calc_good::CharacterBuild {
            character: char,
            level: 90,
            constellation: 0,
            talent_levels: [10, 10, 10],
            weapon: Some(genshin_calc_good::WeaponBuild {
                weapon,
                level: 90,
                refinement: 1,
            }),
            artifacts: genshin_calc_good::ArtifactsBuild {
                sets: vec![],
                stats: genshin_calc_core::StatProfile::default(),
            },
        };
        let builder = genshin_calc_good::to_team_member_builder(&build, &[], &[]).unwrap();
        let member = builder.build().unwrap();

        assert_eq!(member.element, genshin_calc_core::Element::Pyro);
        assert_eq!(member.weapon_type, genshin_calc_core::WeaponType::Polearm);
        assert!(member.stats.base_hp > 0.0);
        assert!(member.stats.base_atk > 0.0);

        // Should be usable as resolve_team_stats input
        let result = genshin_calc_core::resolve_team_stats(&[member], 0).unwrap();
        assert!(result.final_stats.hp > 0.0);
    }

    #[test]
    fn test_build_team_member_with_artifact_activation() {
        use genshin_calc_data::buff::ManualActivation;

        let char = genshin_calc_data::find_character("eula").unwrap();
        let weapon = genshin_calc_data::find_weapon("wolfs_gravestone").unwrap();
        let ph = genshin_calc_data::find_artifact_set("marechaussee_hunter").unwrap();
        let build = genshin_calc_good::CharacterBuild {
            character: char,
            level: 90,
            constellation: 0,
            talent_levels: [10, 10, 10],
            weapon: Some(genshin_calc_good::WeaponBuild {
                weapon,
                level: 90,
                refinement: 1,
            }),
            artifacts: genshin_calc_good::ArtifactsBuild {
                sets: vec![genshin_calc_good::ArtifactSetEntry {
                    set: ph,
                    piece_count: 4,
                }],
                stats: genshin_calc_core::StatProfile::default(),
            },
        };

        // Without activation
        let builder_no = genshin_calc_good::to_team_member_builder(&build, &[], &[]).unwrap();
        let member_no = builder_no.build().unwrap();

        // With 3 stacks of Marechaussee Hunter crit buff
        let artifact_acts = [("marechaussee_crit_stacks", ManualActivation::Stacks(3))];
        let builder_with =
            genshin_calc_good::to_team_member_builder(&build, &[], &artifact_acts).unwrap();
        let member_with = builder_with.build().unwrap();

        // member_with should have more buffs (crit rate +0.36)
        let crit_buff = member_with
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("marechaussee_crit_stacks"));
        assert!(crit_buff.is_some(), "Should have marechaussee crit buff");
        let buff = crit_buff.unwrap();
        assert!(
            (buff.value - 0.36).abs() < 1e-10,
            "3 stacks × 0.12 = 0.36, got {}",
            buff.value
        );

        // No activation = no buff
        let no_crit_buff = member_no
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("marechaussee_crit_stacks"));
        assert!(no_crit_buff.is_none());

        // Both should be valid resolve_team_stats input
        let result = genshin_calc_core::resolve_team_stats(&[member_with], 0).unwrap();
        assert!(result.final_stats.crit_rate > member_no.stats.crit_rate);
    }

    #[test]
    fn test_build_team_member_serde_roundtrip() {
        let char = genshin_calc_data::find_character("diluc").unwrap();
        let weapon = genshin_calc_data::find_weapon("wolfs_gravestone").unwrap();
        let build = genshin_calc_good::CharacterBuild {
            character: char,
            level: 90,
            constellation: 0,
            talent_levels: [10, 10, 10],
            weapon: Some(genshin_calc_good::WeaponBuild {
                weapon,
                level: 90,
                refinement: 1,
            }),
            artifacts: genshin_calc_good::ArtifactsBuild {
                sets: vec![],
                stats: genshin_calc_core::StatProfile::default(),
            },
        };
        let builder = genshin_calc_good::to_team_member_builder(&build, &[], &[]).unwrap();
        let member = builder.build().unwrap();

        // Serialize → Deserialize roundtrip (simulates JS boundary)
        let json = serde_json::to_string(&member).unwrap();
        let back: genshin_calc_core::TeamMember = serde_json::from_str(&json).unwrap();
        assert_eq!(member.element, back.element);
        assert_eq!(member.weapon_type, back.weapon_type);
        assert!((member.stats.base_atk - back.stats.base_atk).abs() < 1e-10);
        assert_eq!(member.buffs_provided.len(), back.buffs_provided.len());

        // Deserialized member should work with resolve_team_stats
        let result = genshin_calc_core::resolve_team_stats(&[back], 0).unwrap();
        assert!(result.final_stats.atk > 0.0);
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
