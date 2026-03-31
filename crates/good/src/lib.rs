mod build_stats;
mod convert;
mod error;
pub mod key_map;
pub mod stat_map;
mod types;

pub use build_stats::build_stat_profile;
pub use convert::to_team_member_builder;
pub use error::{GoodError, ImportWarning};
pub use types::GoodFormat;

use convert::build_imports;
use error::validate_format;

/// GOOD形式インポートの結果
#[derive(Debug, Clone, serde::Serialize)]
pub struct GoodImport {
    pub source: String,
    pub version: u8,
    pub builds: Vec<CharacterBuild>,
    pub warnings: Vec<ImportWarning>,
}

/// 1キャラ分のビルド
#[derive(Debug, Clone, serde::Serialize)]
pub struct CharacterBuild {
    pub character: &'static genshin_calc_data::types::CharacterData,
    pub level: u32,
    pub ascension: u8,
    pub constellation: u8,
    pub talent_levels: [u8; 3],
    pub weapon: Option<WeaponBuild>,
    pub artifacts: ArtifactsBuild,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct WeaponBuild {
    pub weapon: &'static genshin_calc_data::types::WeaponData,
    pub level: u32,
    pub refinement: u8,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct ArtifactsBuild {
    pub sets: Vec<&'static genshin_calc_data::types::ArtifactSet>,
    pub four_piece_set: Option<&'static genshin_calc_data::types::ArtifactSet>,
    pub stats: genshin_calc_core::StatProfile,
}

/// GOOD JSONをパースしてビルド一覧に変換
pub fn import_good(json: &str) -> Result<GoodImport, GoodError> {
    let good: GoodFormat = serde_json::from_str(json)?;
    convert_good(good)
}

/// 既にデシリアライズ済みのGoodFormatから変換
pub fn convert_good(good: GoodFormat) -> Result<GoodImport, GoodError> {
    validate_format(&good)?;
    Ok(build_imports(good))
}
