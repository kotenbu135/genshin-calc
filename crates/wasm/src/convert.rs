use genshin_calc_core::{Element, WeaponType};

/// Parses a lowercase element string to Element enum.
///
/// Accepts: "pyro", "hydro", "electro", "cryo", "anemo", "geo", "dendro"
pub(crate) fn parse_element(s: &str) -> Result<Element, String> {
    match s {
        "pyro" => Ok(Element::Pyro),
        "hydro" => Ok(Element::Hydro),
        "electro" => Ok(Element::Electro),
        "cryo" => Ok(Element::Cryo),
        "anemo" => Ok(Element::Anemo),
        "geo" => Ok(Element::Geo),
        "dendro" => Ok(Element::Dendro),
        _ => Err(format!(
            "Unknown element: '{s}'. Expected one of: pyro, hydro, electro, cryo, anemo, geo, dendro"
        )),
    }
}

/// Parses a lowercase weapon type string to WeaponType enum.
///
/// Accepts: "sword", "claymore", "polearm", "bow", "catalyst"
pub(crate) fn parse_weapon_type(s: &str) -> Result<WeaponType, String> {
    match s {
        "sword" => Ok(WeaponType::Sword),
        "claymore" => Ok(WeaponType::Claymore),
        "polearm" => Ok(WeaponType::Polearm),
        "bow" => Ok(WeaponType::Bow),
        "catalyst" => Ok(WeaponType::Catalyst),
        _ => Err(format!(
            "Unknown weapon type: '{s}'. Expected one of: sword, claymore, polearm, bow, catalyst"
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_element_all_valid() {
        assert_eq!(parse_element("pyro").unwrap(), Element::Pyro);
        assert_eq!(parse_element("hydro").unwrap(), Element::Hydro);
        assert_eq!(parse_element("electro").unwrap(), Element::Electro);
        assert_eq!(parse_element("cryo").unwrap(), Element::Cryo);
        assert_eq!(parse_element("anemo").unwrap(), Element::Anemo);
        assert_eq!(parse_element("geo").unwrap(), Element::Geo);
        assert_eq!(parse_element("dendro").unwrap(), Element::Dendro);
    }

    #[test]
    fn test_parse_element_invalid() {
        assert!(parse_element("fire").is_err());
        assert!(parse_element("Pyro").is_err());
        assert!(parse_element("").is_err());
    }

    #[test]
    fn test_parse_weapon_type_all_valid() {
        assert_eq!(parse_weapon_type("sword").unwrap(), WeaponType::Sword);
        assert_eq!(parse_weapon_type("claymore").unwrap(), WeaponType::Claymore);
        assert_eq!(parse_weapon_type("polearm").unwrap(), WeaponType::Polearm);
        assert_eq!(parse_weapon_type("bow").unwrap(), WeaponType::Bow);
        assert_eq!(parse_weapon_type("catalyst").unwrap(), WeaponType::Catalyst);
    }

    #[test]
    fn test_parse_weapon_type_invalid() {
        assert!(parse_weapon_type("greatsword").is_err());
        assert!(parse_weapon_type("Sword").is_err());
        assert!(parse_weapon_type("").is_err());
    }
}
