mod alhaitham;
mod baizhu;
mod collei;
mod emilie;
mod kaveh;
mod kinich;
mod kirara;
mod lauma;
mod nahida;
mod nefer;
mod tighnari;
mod traveler_dendro;
mod yaoyao;

pub use alhaitham::ALHAITHAM;
pub use baizhu::BAIZHU;
pub use collei::COLLEI;
pub use emilie::EMILIE;
pub use kaveh::KAVEH;
pub use kinich::KINICH;
pub use kirara::KIRARA;
pub use lauma::LAUMA;
pub use nahida::NAHIDA;
pub use nefer::NEFER;
pub use tighnari::TIGHNARI;
pub use traveler_dendro::TRAVELER_DENDRO;
pub use yaoyao::YAOYAO;

use crate::types::CharacterData;

pub const CHARACTERS: &[&CharacterData] = &[
    &ALHAITHAM,
    &BAIZHU,
    &COLLEI,
    &EMILIE,
    &KAVEH,
    &KINICH,
    &KIRARA,
    &LAUMA,
    &NAHIDA,
    &NEFER,
    &TIGHNARI,
    &TRAVELER_DENDRO,
    &YAOYAO,
];
