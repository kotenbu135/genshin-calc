mod amber;
mod arlecchino;
mod bennett;
mod chevreuse;
mod dehya;
mod diluc;
mod durin;
mod gaming;
mod hu_tao;
mod klee;
mod lyney;
mod mavuika;
mod thoma;
mod xiangling;
mod xinyan;
mod yanfei;
mod yoimiya;

pub use amber::AMBER;
pub use arlecchino::ARLECCHINO;
pub use bennett::BENNETT;
pub use chevreuse::CHEVREUSE;
pub use dehya::DEHYA;
pub use diluc::DILUC;
pub use durin::DURIN;
pub use gaming::GAMING;
pub use hu_tao::HU_TAO;
pub use klee::KLEE;
pub use lyney::LYNEY;
pub use mavuika::MAVUIKA;
pub use thoma::THOMA;
pub use xiangling::XIANGLING;
pub use xinyan::XINYAN;
pub use yanfei::YANFEI;
pub use yoimiya::YOIMIYA;

use crate::types::CharacterData;

pub const CHARACTERS: &[&CharacterData] = &[
    &AMBER,
    &ARLECCHINO,
    &BENNETT,
    &CHEVREUSE,
    &DEHYA,
    &DILUC,
    &DURIN,
    &GAMING,
    &HU_TAO,
    &KLEE,
    &LYNEY,
    &MAVUIKA,
    &THOMA,
    &XIANGLING,
    &XINYAN,
    &YANFEI,
    &YOIMIYA,
];
