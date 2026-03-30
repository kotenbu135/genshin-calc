mod chasca;
mod faruzan;
mod heizou;
mod ifa;
mod jahoda;
mod jean;
mod kazuha;
mod lan_yan;
mod lynette;
mod mizuki;
mod sayu;
mod sucrose;
mod varka;
mod venti;
mod wanderer;
mod xianyun;
mod xiao;

pub use chasca::CHASCA;
pub use faruzan::FARUZAN;
pub use heizou::HEIZOU;
pub use ifa::IFA;
pub use jahoda::JAHODA;
pub use jean::JEAN;
pub use kazuha::KAZUHA;
pub use lan_yan::LAN_YAN;
pub use lynette::LYNETTE;
pub use mizuki::MIZUKI;
pub use sayu::SAYU;
pub use sucrose::SUCROSE;
pub use varka::VARKA;
pub use venti::VENTI;
pub use wanderer::WANDERER;
pub use xianyun::XIANYUN;
pub use xiao::XIAO;

use crate::types::CharacterData;

pub const CHARACTERS: &[&CharacterData] = &[
    &CHASCA, &FARUZAN, &HEIZOU, &IFA, &JAHODA, &JEAN, &KAZUHA, &LAN_YAN, &LYNETTE, &MIZUKI, &SAYU,
    &SUCROSE, &VARKA, &VENTI, &WANDERER, &XIANYUN, &XIAO,
];
