pub mod anemo;
pub mod cryo;
pub mod dendro;
pub mod electro;
pub mod geo;
pub mod hydro;
pub mod pyro;

use crate::types::CharacterData;
use genshin_calc_core::Element;

/// Iterator over all characters across all elements.
///
/// Replaces the former `ALL_CHARACTERS` const slice.
/// Element ordering: Pyro → Hydro → Electro → Cryo → Dendro → Anemo → Geo.
pub fn all_characters() -> impl Iterator<Item = &'static &'static CharacterData> {
    pyro::CHARACTERS
        .iter()
        .chain(hydro::CHARACTERS.iter())
        .chain(electro::CHARACTERS.iter())
        .chain(cryo::CHARACTERS.iter())
        .chain(dendro::CHARACTERS.iter())
        .chain(anemo::CHARACTERS.iter())
        .chain(geo::CHARACTERS.iter())
}

/// Returns the element-specific character slice directly (O(1), no allocation).
pub fn characters_by_element_slice(element: Element) -> &'static [&'static CharacterData] {
    match element {
        Element::Pyro => pyro::CHARACTERS,
        Element::Hydro => hydro::CHARACTERS,
        Element::Electro => electro::CHARACTERS,
        Element::Cryo => cryo::CHARACTERS,
        Element::Dendro => dendro::CHARACTERS,
        Element::Anemo => anemo::CHARACTERS,
        Element::Geo => geo::CHARACTERS,
    }
}
