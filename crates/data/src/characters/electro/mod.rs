mod beidou;
mod clorinde;
mod cyno;
mod dori;
mod fischl;
mod flins;
mod iansan;
mod ineffa;
mod keqing;
mod kujou_sara;
mod kuki_shinobu;
mod lisa;
mod ororon;
mod raiden_shogun;
mod razor;
mod sethos;
mod varesa;
mod yae_miko;

pub use beidou::BEIDOU;
pub use clorinde::CLORINDE;
pub use cyno::CYNO;
pub use dori::DORI;
pub use fischl::FISCHL;
pub use flins::FLINS;
pub use iansan::IANSAN;
pub use ineffa::INEFFA;
pub use keqing::KEQING;
pub use kujou_sara::KUJOU_SARA;
pub use kuki_shinobu::KUKI_SHINOBU;
pub use lisa::LISA;
pub use ororon::ORORON;
pub use raiden_shogun::RAIDEN_SHOGUN;
pub use razor::RAZOR;
pub use sethos::SETHOS;
pub use varesa::VARESA;
pub use yae_miko::YAE_MIKO;

use crate::types::CharacterData;

pub const CHARACTERS: &[&CharacterData] = &[
    &BEIDOU,
    &CLORINDE,
    &CYNO,
    &DORI,
    &FISCHL,
    &FLINS,
    &IANSAN,
    &INEFFA,
    &KEQING,
    &KUJOU_SARA,
    &KUKI_SHINOBU,
    &LISA,
    &ORORON,
    &RAIDEN_SHOGUN,
    &RAZOR,
    &SETHOS,
    &VARESA,
    &YAE_MIKO,
];
