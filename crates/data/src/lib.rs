//! # genshin-calc-data
//!
//! Genshin Impact v5.8 game data as Rust constants.
//!
//! Includes:
//! - 102 playable characters
//! - 230 weapons (all types)
//! - 52 artifact sets
//! - 15 enemies with resistance templates
//!
//! ## Example
//!
//! ```
//! use genshin_calc_data::*;
//!
//! let diluc = find_character("diluc").unwrap();
//! assert_eq!(diluc.name, "Diluc");
//!
//! let pyro_chars = characters_by_element(genshin_calc_core::Element::Pyro);
//! assert!(pyro_chars.len() > 10);
//! ```

#![deny(missing_docs)]

#[allow(missing_docs)]
pub mod artifacts;
#[allow(missing_docs)]
pub mod buff;
#[allow(missing_docs)]
pub mod characters;
#[allow(missing_docs)]
pub mod enemies;
pub mod moonsign_chars;
#[allow(missing_docs)]
pub mod talent_buffs;
#[allow(missing_docs)]
pub mod team_builder;
#[allow(missing_docs)]
pub mod types;
#[allow(missing_docs)]
pub mod weapons;

/// Conditional buff types for weapon passives and artifact set effects.
pub use buff::{
    Activation, AutoCondition, AvailableConditional, ConditionalBuff, ManualActivation,
    ManualCondition,
};
/// Moonsign character data: benedictions, talent enhancements, and lookup functions.
pub use moonsign_chars::{
    ALL_MOONSIGN_BENEDICTIONS, MoonsignBenedictionDef, calculate_benediction_bonus,
    find_moonsign_benediction, find_moonsign_talent_enhancements, is_moonsign_character,
};
/// Talent buff definitions and lookup.
pub use talent_buffs::{TalentBuffDef, TalentBuffSource, find_talent_buffs};
/// Builder pattern for constructing [`genshin_calc_core::TeamMember`] from game data.
pub use team_builder::TeamMemberBuilder;

use genshin_calc_core::Element;
use types::{ArtifactSet, CharacterData, EnemyData, WeaponData, WeaponType};

/// Current game version for the included data.
pub const GAME_VERSION: &str = "5.8";

/// Finds a character by ID (lowercase, e.g. `"diluc"`, `"hu_tao"`).
///
/// # Examples
///
/// ```
/// use genshin_calc_data::find_character;
///
/// let ganyu = find_character("ganyu").unwrap();
/// assert_eq!(ganyu.name, "Ganyu");
/// assert!(find_character("nonexistent").is_none());
/// ```
#[must_use]
pub fn find_character(id: &str) -> Option<&'static CharacterData> {
    characters::all_characters().find(|c| c.id == id).copied()
}

/// Finds a weapon by ID (lowercase, e.g. `"wolfs_gravestone"`).
///
/// # Examples
///
/// ```
/// use genshin_calc_data::find_weapon;
///
/// let weapon = find_weapon("wolfs_gravestone").unwrap();
/// assert_eq!(weapon.id, "wolfs_gravestone");
/// assert!(find_weapon("nonexistent").is_none());
/// ```
#[must_use]
pub fn find_weapon(id: &str) -> Option<&'static WeaponData> {
    weapons::ALL_WEAPONS.iter().find(|w| w.id == id).copied()
}

/// Finds an artifact set by ID (lowercase, e.g. `"crimson_witch"`).
///
/// # Examples
///
/// ```
/// use genshin_calc_data::find_artifact_set;
///
/// let cw = find_artifact_set("crimson_witch").unwrap();
/// assert_eq!(cw.name, "燃え盛る炎の魔女");
/// assert!(find_artifact_set("nonexistent").is_none());
/// ```
#[must_use]
pub fn find_artifact_set(id: &str) -> Option<&'static ArtifactSet> {
    artifacts::ALL_ARTIFACT_SETS
        .iter()
        .find(|a| a.id == id)
        .copied()
}

/// Finds an enemy by ID (lowercase, e.g. `"hilichurl"`).
///
/// # Examples
///
/// ```
/// use genshin_calc_data::find_enemy;
///
/// let enemy = find_enemy("hilichurl").unwrap();
/// assert_eq!(enemy.id, "hilichurl");
/// assert!(find_enemy("nonexistent").is_none());
/// ```
#[must_use]
pub fn find_enemy(id: &str) -> Option<&'static EnemyData> {
    enemies::ALL_ENEMIES.iter().find(|e| e.id == id).copied()
}

/// Returns all characters with the given element.
///
/// # Examples
///
/// ```
/// use genshin_calc_data::characters_by_element;
/// use genshin_calc_core::Element;
///
/// let pyro = characters_by_element(Element::Pyro);
/// assert!(pyro.len() > 10);
/// ```
#[must_use]
pub fn characters_by_element(element: Element) -> Vec<&'static CharacterData> {
    characters::characters_by_element_slice(element).to_vec()
}

/// Returns all weapons of the given type.
///
/// # Examples
///
/// ```
/// use genshin_calc_data::weapons_by_type;
/// use genshin_calc_data::types::WeaponType;
///
/// let swords = weapons_by_type(WeaponType::Sword);
/// assert!(!swords.is_empty());
/// ```
#[must_use]
pub fn weapons_by_type(weapon_type: WeaponType) -> Vec<&'static WeaponData> {
    weapons::ALL_WEAPONS
        .iter()
        .filter(|w| w.weapon_type == weapon_type)
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use genshin_calc_core::WeaponType;

    #[test]
    fn test_find_jahoda_character() {
        let jahoda = find_character("jahoda").unwrap();
        assert_eq!(jahoda.element, Element::Anemo);
        assert_eq!(jahoda.weapon_type, WeaponType::Bow);
        assert!(jahoda.base_hp[3] > 0.0);
        assert!(jahoda.base_atk[3] > 0.0);
        assert!(jahoda.base_def[3] > 0.0);
    }

    #[test]
    fn test_find_aino_character() {
        let aino = find_character("aino").unwrap();
        assert_eq!(aino.element, Element::Hydro);
        assert_eq!(aino.weapon_type, WeaponType::Claymore);
        assert!(aino.base_hp[3] > 0.0);
    }
}
