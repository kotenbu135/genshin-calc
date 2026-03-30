mod aino;
mod ayato;
mod barbara;
mod candace;
mod dahlia;
mod furina;
mod kokomi;
mod mona;
mod mualani;
mod neuvillette;
mod nilou;
mod sigewinne;
mod tartaglia;
mod xingqiu;
mod yelan;

pub use aino::AINO;
pub use ayato::AYATO;
pub use barbara::BARBARA;
pub use candace::CANDACE;
pub use dahlia::DAHLIA;
pub use furina::FURINA;
pub use kokomi::KOKOMI;
pub use mona::MONA;
pub use mualani::MUALANI;
pub use neuvillette::NEUVILLETTE;
pub use nilou::NILOU;
pub use sigewinne::SIGEWINNE;
pub use tartaglia::TARTAGLIA;
pub use xingqiu::XINGQIU;
pub use yelan::YELAN;

use crate::types::CharacterData;

pub const CHARACTERS: &[&CharacterData] = &[
    &AINO,
    &AYATO,
    &BARBARA,
    &CANDACE,
    &DAHLIA,
    &FURINA,
    &KOKOMI,
    &MONA,
    &MUALANI,
    &NEUVILLETTE,
    &NILOU,
    &SIGEWINNE,
    &TARTAGLIA,
    &XINGQIU,
    &YELAN,
];
