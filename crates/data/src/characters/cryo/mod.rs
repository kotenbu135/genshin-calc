mod aloy;
mod ayaka;
mod charlotte;
mod chongyun;
mod citlali;
mod diona;
mod escoffier;
mod eula;
mod freminet;
mod ganyu;
mod kaeya;
mod layla;
mod mika;
mod qiqi;
mod rosaria;
mod shenhe;
mod skirk;
mod wriothesley;

pub use aloy::ALOY;
pub use ayaka::AYAKA;
pub use charlotte::CHARLOTTE;
pub use chongyun::CHONGYUN;
pub use citlali::CITLALI;
pub use diona::DIONA;
pub use escoffier::ESCOFFIER;
pub use eula::EULA;
pub use freminet::FREMINET;
pub use ganyu::GANYU;
pub use kaeya::KAEYA;
pub use layla::LAYLA;
pub use mika::MIKA;
pub use qiqi::QIQI;
pub use rosaria::ROSARIA;
pub use shenhe::SHENHE;
pub use skirk::SKIRK;
pub use wriothesley::WRIOTHESLEY;

use crate::types::CharacterData;

pub const CHARACTERS: &[&CharacterData] = &[
    &ALOY,
    &AYAKA,
    &CHARLOTTE,
    &CHONGYUN,
    &CITLALI,
    &DIONA,
    &ESCOFFIER,
    &EULA,
    &FREMINET,
    &GANYU,
    &KAEYA,
    &LAYLA,
    &MIKA,
    &QIQI,
    &ROSARIA,
    &SHENHE,
    &SKIRK,
    &WRIOTHESLEY,
];
