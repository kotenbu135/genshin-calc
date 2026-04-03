//! Internal types for parsing GOOD format JSON.
//!
//! These types mirror the GOOD (Genshin Open Object Description) format structure.
//! They are used internally to deserialize JSON exports from scanner apps.

use serde::Deserialize;

/// Root GOOD format structure.
///
/// This is the top-level structure parsed from JSON exports.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodFormat {
    /// Must be "GOOD".
    pub format: String,
    /// Source application (e.g., "Irminsul", "Inventory Kamera").
    pub source: String,
    /// GOOD format version (1, 2, or 3).
    pub version: u8,
    /// Character list.
    pub characters: Option<Vec<GoodCharacter>>,
    /// Artifact list.
    pub artifacts: Option<Vec<GoodArtifact>>,
    /// Weapon list.
    pub weapons: Option<Vec<GoodWeapon>>,
}

/// Character data in GOOD format.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodCharacter {
    /// Character identifier (PascalCase, e.g., "HuTao").
    pub key: String,
    /// Character level (1-100).
    pub level: u32,
    /// Number of constellations (0-6).
    pub constellation: u8,
    /// Ascension phase (0-6).
    pub ascension: u8,
    /// Talent levels.
    pub talent: GoodTalent,
}

/// Talent levels for a character.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodTalent {
    /// Normal attack talent level.
    pub auto: u8,
    /// Elemental skill talent level.
    pub skill: u8,
    /// Elemental burst talent level.
    pub burst: u8,
}

/// Weapon data in GOOD format.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodWeapon {
    /// Weapon identifier (PascalCase, e.g., "PolarStar").
    pub key: String,
    /// Weapon level (1-90).
    pub level: u32,
    /// Ascension phase (0-6).
    pub ascension: u8,
    /// Refinement level (1-5).
    pub refinement: u8,
    /// Assigned character name (if equipped).
    pub location: Option<String>,
    /// Whether the weapon is locked.
    pub lock: bool,
}

/// Artifact data in GOOD format.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodArtifact {
    /// Artifact set name (PascalCase, e.g., "GladiatorFinale").
    pub set_key: String,
    /// Slot type: "flower", "plume", "sands", "goblet", or "circlet".
    pub slot_key: String,
    /// Artifact level (1-20 for 5-star, 1-16 for 4-star).
    pub level: u8,
    /// Rarity: 5 for 5-star, 4 for 4-star.
    pub rarity: u8,
    /// Main stat key (e.g., "atk_", "critRate_", "pyroDmg_").
    pub main_stat_key: String,
    /// Assigned character name (if equipped).
    pub location: Option<String>,
    /// Whether the artifact is locked.
    pub lock: bool,
    /// Substats (up to 4).
    pub substats: Vec<GoodSubstat>,
}

/// A single substat on an artifact.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodSubstat {
    /// Stat key (e.g., "atk", "critRate_", "elementalMastery").
    pub key: String,
    /// Stat value (flat or percentage, depending on stat type).
    pub value: f64,
}
