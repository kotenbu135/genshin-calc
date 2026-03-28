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

pub mod artifacts;
pub mod buff;
pub mod characters;
pub mod enemies;
pub mod moonsign_chars;
pub mod talent_buffs;
pub mod team_builder;
pub mod types;
pub mod weapons;

pub use buff::{
    Activation, AutoCondition, AvailableConditional, ConditionalBuff, ManualActivation,
    ManualCondition,
};
pub use moonsign_chars::{
    ALL_MOONSIGN_BENEDICTIONS, MoonsignBenedictionDef, calculate_benediction_bonus,
    find_moonsign_benediction, find_moonsign_talent_enhancements, is_moonsign_character,
};
pub use talent_buffs::{TalentBuffDef, TalentBuffSource, find_talent_buffs};
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
pub fn find_character(id: &str) -> Option<&'static CharacterData> {
    characters::ALL_CHARACTERS
        .iter()
        .find(|c| c.id == id)
        .copied()
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
pub fn find_weapon(id: &str) -> Option<&'static WeaponData> {
    weapons::ALL_WEAPONS.iter().find(|w| w.id == id).copied()
}

/// Finds an artifact set by ID (lowercase, e.g. `"crimson_witch_of_flames"`).
pub fn find_artifact_set(id: &str) -> Option<&'static ArtifactSet> {
    artifacts::ALL_ARTIFACT_SETS
        .iter()
        .find(|a| a.id == id)
        .copied()
}

/// Finds an enemy by ID (lowercase, e.g. `"hilichurl"`).
pub fn find_enemy(id: &str) -> Option<&'static EnemyData> {
    enemies::ALL_ENEMIES.iter().find(|e| e.id == id).copied()
}

/// Returns all characters with the given element.
pub fn characters_by_element(element: Element) -> Vec<&'static CharacterData> {
    characters::ALL_CHARACTERS
        .iter()
        .filter(|c| c.element == element)
        .copied()
        .collect()
}

/// Returns all weapons of the given type.
pub fn weapons_by_type(weapon_type: WeaponType) -> Vec<&'static WeaponData> {
    weapons::ALL_WEAPONS
        .iter()
        .filter(|w| w.weapon_type == weapon_type)
        .copied()
        .collect()
}
