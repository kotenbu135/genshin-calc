mod albedo;
mod chiori;
mod gorou;
mod itto;
mod kachina;
mod navia;
mod ningguang;
mod noelle;
mod xilonen;
mod yun_jin;
mod zhongli;

pub use albedo::ALBEDO;
pub use chiori::CHIORI;
pub use gorou::GOROU;
pub use itto::ITTO;
pub use kachina::KACHINA;
pub use navia::NAVIA;
pub use ningguang::NINGGUANG;
pub use noelle::NOELLE;
pub use xilonen::XILONEN;
pub use yun_jin::YUN_JIN;
pub use zhongli::ZHONGLI;

use crate::types::CharacterData;

pub const CHARACTERS: &[&CharacterData] = &[
    &ALBEDO, &CHIORI, &GOROU, &ITTO, &KACHINA, &NAVIA, &NINGGUANG, &NOELLE, &XILONEN, &YUN_JIN,
    &ZHONGLI,
];
