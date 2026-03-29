pub mod artifacts;
pub mod buff;
pub mod characters;
pub mod enemies;
pub mod types;
pub mod weapons;

use genshin_calc_core::Element;
use types::{ArtifactSet, CharacterData, EnemyData, WeaponData, WeaponType};

pub const GAME_VERSION: &str = "5.8";

pub fn find_character(id: &str) -> Option<&'static CharacterData> {
    characters::ALL_CHARACTERS
        .iter()
        .find(|c| c.id == id)
        .copied()
}

pub fn find_weapon(id: &str) -> Option<&'static WeaponData> {
    weapons::ALL_WEAPONS.iter().find(|w| w.id == id).copied()
}

pub fn find_artifact_set(id: &str) -> Option<&'static ArtifactSet> {
    artifacts::ALL_ARTIFACT_SETS
        .iter()
        .find(|a| a.id == id)
        .copied()
}

pub fn find_enemy(id: &str) -> Option<&'static EnemyData> {
    enemies::ALL_ENEMIES.iter().find(|e| e.id == id).copied()
}

pub fn characters_by_element(element: Element) -> Vec<&'static CharacterData> {
    characters::ALL_CHARACTERS
        .iter()
        .filter(|c| c.element == element)
        .copied()
        .collect()
}

pub fn weapons_by_type(weapon_type: WeaponType) -> Vec<&'static WeaponData> {
    weapons::ALL_WEAPONS
        .iter()
        .filter(|w| w.weapon_type == weapon_type)
        .copied()
        .collect()
}
