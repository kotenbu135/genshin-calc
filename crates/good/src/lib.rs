//! # genshin-calc-good
//!
//! GOOD (Genshin Open Object Description) format importer for [genshin-calc](https://github.com/kotenbu/genshin-calc).
//!
//! This crate parses GOOD JSON exports from scanner apps (Inventory Kamera, Genshin Calculator, etc.)
//! and converts them into `CharacterBuild` structs ready for damage calculation.
//!
//! ## Quick Start
//!
//! ```rust
//! use genshin_calc_good::import_good;
//!
//! let json = r#"{
//!     "format": "GOOD",
//!     "version": 3,
//!     "source": "Test",
//!     "characters": [{"key": "Amber", "level": 50, "constellation": 0, "ascension": 0, "talent": {"auto": 1, "skill": 1, "burst": 1}}],
//!     "artifacts": [],
//!     "weapons": []
//! }"#;
//!
//! let import = import_good(json).unwrap();
//! assert_eq!(import.builds.len(), 1);
//! assert_eq!(import.builds[0].character.name, "Amber");
//! ```
//!
//! ## Full Example
//!
//! See [`demo`](https://github.com/kotenbu/genshin-calc/blob/main/crates/good/examples/demo.rs) for complete usage.

mod build_stats;
mod convert;
pub mod error;
pub mod key_map;
pub mod stat_map;
mod types;

pub use build_stats::build_stat_profile;
pub use convert::to_team_member_builder;
pub use error::{GoodError, ImportWarning};
pub use types::GoodFormat;

use convert::build_imports;
use error::validate_format;

/// Result of importing a GOOD JSON file.
///
/// Contains all character builds extracted from the export, along with any warnings
/// for data that couldn't be processed.
///
/// # Example
///
/// ```rust
/// use genshin_calc_good::import_good;
///
/// let json = r#"{
///     "format": "GOOD",
///     "version": 3,
///     "source": "Test",
///     "characters": [{"key": "Amber", "level": 50, "constellation": 0, "ascension": 0, "talent": {"auto": 1, "skill": 1, "burst": 1}}],
///     "artifacts": [],
///     "weapons": []
/// }"#;
///
/// let import = import_good(json).unwrap();
/// assert_eq!(import.builds.len(), 1);
/// assert_eq!(import.builds[0].character.name, "Amber");
/// ```
#[derive(Debug, Clone, serde::Serialize)]
pub struct GoodImport {
    /// Source of the export (e.g., "Irminsul", "Inventory Kamera").
    pub source: String,
    /// GOOD format version (1, 2, or 3).
    pub version: u8,
    /// All character builds found in the export.
    pub builds: Vec<CharacterBuild>,
    /// Warnings for data that couldn't be processed.
    /// These are non-fatal - the import still succeeds.
    pub warnings: Vec<ImportWarning>,
}

/// A single character build extracted from GOOD format.
///
/// Contains all data needed to build a [`genshin_calc_core::TeamMember`] for damage calculation.
///
/// # Fields
///
/// - `character`: Reference to [`genshin_calc_data::types::CharacterData`]
/// - `level`: Character level (1-100)
/// - `constellation`: Number of constellations (0-6)
/// - `talent_levels`: `[auto, skill, burst]` talent levels (1-15)
/// - `weapon`: Weapon information (if equipped)
/// - `artifacts`: Artifact sets and aggregated stats
///
/// # Example
///
/// ```
/// use genshin_calc_good::GoodImport;
///
/// fn process_build(build: &genshin_calc_good::CharacterBuild) {
///     println!("Building {} at constellation {}", build.character.name, build.constellation);
///
///     if let Some(ref weapon) = build.weapon {
///         println!("  Weapon: {} (Lv{})", weapon.weapon.name, weapon.level);
///     }
///
///     println!("  Artifacts: {:?}", build.artifacts.sets.iter().map(|s| s.name).collect::<Vec<_>>());
/// }
/// ```
#[derive(Debug, Clone, serde::Serialize)]
pub struct CharacterBuild {
    /// Character data from genshin-calc-data.
    pub character: &'static genshin_calc_data::types::CharacterData,
    /// Character level (1-100).
    pub level: u32,
    /// Number of constellations (0-6).
    pub constellation: u8,
    /// Talent levels as `[auto, skill, burst]`.
    pub talent_levels: [u8; 3],
    /// Weapon build (if equipped).
    pub weapon: Option<WeaponBuild>,
    /// Artifact sets and aggregated stats.
    pub artifacts: ArtifactsBuild,
}

/// Weapon information in a character build.
///
/// Contains weapon reference and level/refinement data.
#[derive(Debug, Clone, serde::Serialize)]
pub struct WeaponBuild {
    /// Weapon data from genshin-calc-data.
    pub weapon: &'static genshin_calc_data::types::WeaponData,
    /// Weapon level (1-90).
    pub level: u32,
    /// Refinement level (1-5).
    pub refinement: u8,
}

/// Artifact data in a character build.
///
/// Contains detected artifact sets and aggregated main+substats.
#[derive(Debug, Clone, serde::Serialize)]
pub struct ArtifactsBuild {
    /// Detected artifact sets.
    /// Returns 4pc set first, or up to two 2pc sets.
    pub sets: Vec<&'static genshin_calc_data::types::ArtifactSet>,
    /// Aggregated stats from all artifacts (main stat + substats).
    /// This is a raw [`genshin_calc_core::StatProfile`] that can be passed directly to
    /// [`genshin_calc_data::team_builder::TeamMemberBuilder::artifact_stats`].
    pub stats: genshin_calc_core::StatProfile,
}

/// Parses a GOOD JSON string and converts to character builds.
///
/// This is the main entry point for importing GOOD format data.
///
/// # Arguments
///
/// * `json` - Valid GOOD JSON string (from file or API)
///
/// # Returns
///
/// Returns [`Ok(GoodImport)`] on success, or [`Err(GoodError)`] if parsing fails.
///
/// # Example
///
/// ```rust
/// use genshin_calc_good::import_good;
///
/// let json = r#"{
///     "format": "GOOD",
///     "version": 3,
///     "source": "Test",
///     "characters": [{"key": "Amber", "level": 50, "constellation": 0, "ascension": 0, "talent": {"auto": 1, "skill": 1, "burst": 1}}],
///     "artifacts": [],
///     "weapons": []
/// }"#;
///
/// let import = import_good(json).unwrap();
/// assert_eq!(import.builds.len(), 1);
/// assert_eq!(import.builds[0].character.name, "Amber");
/// ```
///
/// # Workflow
///
/// 1. Export GOOD JSON from a scanner app (Inventory Kamera, etc.)
/// 2. Call `import_good()` with the JSON string
/// 3. Iterate over `import.builds` for each character
/// 4. Use [`CharacterBuild`] data to build [`genshin_calc_core::TeamMember`]
/// 5. Calculate damage using [`genshin_calc_core::calculate_damage`]
pub fn import_good(json: &str) -> Result<GoodImport, GoodError> {
    let good: GoodFormat = serde_json::from_str(json)?;
    convert_good(good)
}

/// Converts an already-parsed [`GoodFormat`] into character builds.
///
/// Use this if you've already deserialized the JSON using your own [`GoodFormat`] type.
///
/// # Arguments
///
/// * `good` - A valid [`GoodFormat`] struct
///
/// # Returns
///
/// Returns [`Ok(GoodImport)`] on success, or [`Err(GoodError)`] if validation fails.
pub fn convert_good(good: GoodFormat) -> Result<GoodImport, GoodError> {
    validate_format(&good)?;
    Ok(build_imports(good))
}
