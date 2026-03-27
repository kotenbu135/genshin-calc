use serde::{Deserialize, Serialize};

use crate::types::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Reaction {
    // Amplifying
    Vaporize,
    Melt,
    // Catalyze
    Aggravate,
    Spread,
    // Transformative
    Overloaded,
    Superconduct,
    ElectroCharged,
    Shattered,
    Swirl(Element),
    Bloom,
    Hyperbloom,
    Burgeon,
    Burning,
    // Lunar
    LunarElectroCharged,
    LunarBloom,
    LunarCrystallize,
    LunarCrystallizeSecondary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReactionCategory {
    Amplifying,
    Catalyze,
    Transformative,
    Lunar,
}

impl Reaction {
    pub fn category(&self) -> ReactionCategory {
        match self {
            Reaction::Vaporize | Reaction::Melt => ReactionCategory::Amplifying,
            Reaction::Aggravate | Reaction::Spread => ReactionCategory::Catalyze,
            Reaction::Overloaded
            | Reaction::Superconduct
            | Reaction::ElectroCharged
            | Reaction::Shattered
            | Reaction::Swirl(_)
            | Reaction::Bloom
            | Reaction::Hyperbloom
            | Reaction::Burgeon
            | Reaction::Burning => ReactionCategory::Transformative,
            Reaction::LunarElectroCharged
            | Reaction::LunarBloom
            | Reaction::LunarCrystallize
            | Reaction::LunarCrystallizeSecondary => ReactionCategory::Lunar,
        }
    }
}

pub fn determine_reaction(trigger: Element, aura: Element) -> Option<Reaction> {
    match (trigger, aura) {
        // Vaporize
        (Element::Pyro, Element::Hydro) => Some(Reaction::Vaporize),
        (Element::Hydro, Element::Pyro) => Some(Reaction::Vaporize),
        // Melt
        (Element::Pyro, Element::Cryo) => Some(Reaction::Melt),
        (Element::Cryo, Element::Pyro) => Some(Reaction::Melt),
        // Overloaded
        (Element::Pyro, Element::Electro) | (Element::Electro, Element::Pyro) => {
            Some(Reaction::Overloaded)
        }
        // Superconduct
        (Element::Cryo, Element::Electro) | (Element::Electro, Element::Cryo) => {
            Some(Reaction::Superconduct)
        }
        // Electro-Charged
        (Element::Hydro, Element::Electro) | (Element::Electro, Element::Hydro) => {
            Some(Reaction::ElectroCharged)
        }
        // Swirl
        (
            Element::Anemo,
            aura @ (Element::Pyro | Element::Hydro | Element::Electro | Element::Cryo),
        ) => Some(Reaction::Swirl(aura)),
        // Bloom
        (Element::Hydro, Element::Dendro) | (Element::Dendro, Element::Hydro) => {
            Some(Reaction::Bloom)
        }
        // Burning
        (Element::Pyro, Element::Dendro) | (Element::Dendro, Element::Pyro) => {
            Some(Reaction::Burning)
        }
        // Shattered is triggered by heavy attacks on frozen enemies, not by element combination
        // Hyperbloom/Burgeon are triggered on Bloom cores, not by direct element combination
        // Catalyze (Aggravate/Spread) requires Quicken state, not representable here
        // Lunar reactions have special triggers not representable here
        _ => None,
    }
}

pub fn amplifying_multiplier(trigger: Element, aura: Element) -> Option<f64> {
    match (trigger, aura) {
        (Element::Pyro, Element::Hydro) => Some(1.5), // Reverse Vaporize
        (Element::Hydro, Element::Pyro) => Some(2.0), // Forward Vaporize
        (Element::Pyro, Element::Cryo) => Some(2.0),  // Forward Melt
        (Element::Cryo, Element::Pyro) => Some(1.5),  // Reverse Melt
        _ => None,
    }
}

pub fn catalyze_coefficient(reaction: Reaction) -> Option<f64> {
    match reaction {
        Reaction::Aggravate => Some(1.15),
        Reaction::Spread => Some(1.25),
        _ => None,
    }
}

pub fn transformative_multiplier(reaction: Reaction) -> Option<f64> {
    match reaction {
        Reaction::Overloaded => Some(2.75),
        Reaction::Superconduct => Some(1.5),
        Reaction::ElectroCharged => Some(2.0),
        Reaction::Shattered => Some(3.0),
        Reaction::Swirl(_) => Some(0.6),
        Reaction::Bloom => Some(2.0),
        Reaction::Hyperbloom => Some(3.0),
        Reaction::Burgeon => Some(3.0),
        Reaction::Burning => Some(0.25),
        _ => None,
    }
}

pub fn lunar_multiplier(reaction: Reaction) -> Option<f64> {
    match reaction {
        Reaction::LunarElectroCharged => Some(1.8),
        Reaction::LunarBloom => Some(1.0),
        Reaction::LunarCrystallize => Some(0.96),
        Reaction::LunarCrystallizeSecondary => Some(1.6),
        _ => None,
    }
}

pub fn transformative_element(reaction: Reaction) -> Option<Option<Element>> {
    match reaction {
        Reaction::Overloaded => Some(Some(Element::Pyro)),
        Reaction::Superconduct => Some(Some(Element::Cryo)),
        Reaction::ElectroCharged => Some(Some(Element::Electro)),
        Reaction::Shattered => Some(None), // Physical
        Reaction::Swirl(elem) => Some(Some(elem)),
        Reaction::Bloom | Reaction::Hyperbloom | Reaction::Burgeon => Some(Some(Element::Dendro)),
        Reaction::Burning => Some(Some(Element::Pyro)),
        _ => None,
    }
}

pub fn lunar_element(reaction: Reaction) -> Option<Option<Element>> {
    match reaction {
        Reaction::LunarElectroCharged => Some(Some(Element::Electro)),
        Reaction::LunarBloom => Some(Some(Element::Dendro)),
        Reaction::LunarCrystallize | Reaction::LunarCrystallizeSecondary => {
            Some(Some(Element::Geo))
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_amplifying() {
        assert_eq!(Reaction::Vaporize.category(), ReactionCategory::Amplifying);
        assert_eq!(Reaction::Melt.category(), ReactionCategory::Amplifying);
    }

    #[test]
    fn test_category_catalyze() {
        assert_eq!(Reaction::Aggravate.category(), ReactionCategory::Catalyze);
        assert_eq!(Reaction::Spread.category(), ReactionCategory::Catalyze);
    }

    #[test]
    fn test_category_transformative() {
        assert_eq!(
            Reaction::Overloaded.category(),
            ReactionCategory::Transformative
        );
        assert_eq!(
            Reaction::Swirl(Element::Pyro).category(),
            ReactionCategory::Transformative
        );
    }

    #[test]
    fn test_category_lunar() {
        assert_eq!(
            Reaction::LunarElectroCharged.category(),
            ReactionCategory::Lunar
        );
        assert_eq!(Reaction::LunarBloom.category(), ReactionCategory::Lunar);
    }

    #[test]
    fn test_determine_vaporize() {
        assert_eq!(
            determine_reaction(Element::Pyro, Element::Hydro),
            Some(Reaction::Vaporize)
        );
        assert_eq!(
            determine_reaction(Element::Hydro, Element::Pyro),
            Some(Reaction::Vaporize)
        );
    }

    #[test]
    fn test_determine_melt() {
        assert_eq!(
            determine_reaction(Element::Pyro, Element::Cryo),
            Some(Reaction::Melt)
        );
        assert_eq!(
            determine_reaction(Element::Cryo, Element::Pyro),
            Some(Reaction::Melt)
        );
    }

    #[test]
    fn test_determine_overloaded() {
        assert_eq!(
            determine_reaction(Element::Pyro, Element::Electro),
            Some(Reaction::Overloaded)
        );
        assert_eq!(
            determine_reaction(Element::Electro, Element::Pyro),
            Some(Reaction::Overloaded)
        );
    }

    #[test]
    fn test_determine_superconduct() {
        assert_eq!(
            determine_reaction(Element::Cryo, Element::Electro),
            Some(Reaction::Superconduct)
        );
    }

    #[test]
    fn test_determine_electro_charged() {
        assert_eq!(
            determine_reaction(Element::Hydro, Element::Electro),
            Some(Reaction::ElectroCharged)
        );
    }

    #[test]
    fn test_determine_swirl() {
        assert_eq!(
            determine_reaction(Element::Anemo, Element::Pyro),
            Some(Reaction::Swirl(Element::Pyro))
        );
        assert_eq!(
            determine_reaction(Element::Anemo, Element::Hydro),
            Some(Reaction::Swirl(Element::Hydro))
        );
        assert_eq!(
            determine_reaction(Element::Anemo, Element::Electro),
            Some(Reaction::Swirl(Element::Electro))
        );
        assert_eq!(
            determine_reaction(Element::Anemo, Element::Cryo),
            Some(Reaction::Swirl(Element::Cryo))
        );
    }

    #[test]
    fn test_determine_swirl_invalid_elements() {
        assert_eq!(determine_reaction(Element::Anemo, Element::Dendro), None);
        assert_eq!(determine_reaction(Element::Anemo, Element::Geo), None);
        assert_eq!(determine_reaction(Element::Anemo, Element::Anemo), None);
    }

    #[test]
    fn test_determine_bloom() {
        assert_eq!(
            determine_reaction(Element::Hydro, Element::Dendro),
            Some(Reaction::Bloom)
        );
        assert_eq!(
            determine_reaction(Element::Dendro, Element::Hydro),
            Some(Reaction::Bloom)
        );
    }

    #[test]
    fn test_determine_burning() {
        assert_eq!(
            determine_reaction(Element::Pyro, Element::Dendro),
            Some(Reaction::Burning)
        );
    }

    #[test]
    fn test_determine_no_reaction() {
        assert_eq!(determine_reaction(Element::Geo, Element::Geo), None);
        assert_eq!(determine_reaction(Element::Hydro, Element::Hydro), None);
    }

    #[test]
    fn test_amplifying_multiplier_vaporize() {
        assert_eq!(
            amplifying_multiplier(Element::Pyro, Element::Hydro),
            Some(1.5)
        );
        assert_eq!(
            amplifying_multiplier(Element::Hydro, Element::Pyro),
            Some(2.0)
        );
    }

    #[test]
    fn test_amplifying_multiplier_melt() {
        assert_eq!(
            amplifying_multiplier(Element::Pyro, Element::Cryo),
            Some(2.0)
        );
        assert_eq!(
            amplifying_multiplier(Element::Cryo, Element::Pyro),
            Some(1.5)
        );
    }

    #[test]
    fn test_amplifying_multiplier_non_amplifying() {
        assert_eq!(amplifying_multiplier(Element::Pyro, Element::Electro), None);
    }

    #[test]
    fn test_catalyze_coefficient_values() {
        assert_eq!(catalyze_coefficient(Reaction::Aggravate), Some(1.15));
        assert_eq!(catalyze_coefficient(Reaction::Spread), Some(1.25));
        assert_eq!(catalyze_coefficient(Reaction::Vaporize), None);
    }

    #[test]
    fn test_transformative_multiplier_values() {
        assert_eq!(transformative_multiplier(Reaction::Overloaded), Some(2.75));
        assert_eq!(transformative_multiplier(Reaction::Superconduct), Some(1.5));
        assert_eq!(
            transformative_multiplier(Reaction::ElectroCharged),
            Some(2.0)
        );
        assert_eq!(transformative_multiplier(Reaction::Shattered), Some(3.0));
        assert_eq!(
            transformative_multiplier(Reaction::Swirl(Element::Pyro)),
            Some(0.6)
        );
        assert_eq!(transformative_multiplier(Reaction::Bloom), Some(2.0));
        assert_eq!(transformative_multiplier(Reaction::Hyperbloom), Some(3.0));
        assert_eq!(transformative_multiplier(Reaction::Burgeon), Some(3.0));
        assert_eq!(transformative_multiplier(Reaction::Burning), Some(0.25));
    }

    #[test]
    fn test_lunar_multiplier_values() {
        assert_eq!(lunar_multiplier(Reaction::LunarElectroCharged), Some(1.8));
        assert_eq!(lunar_multiplier(Reaction::LunarBloom), Some(1.0));
        assert_eq!(lunar_multiplier(Reaction::LunarCrystallize), Some(0.96));
        assert_eq!(
            lunar_multiplier(Reaction::LunarCrystallizeSecondary),
            Some(1.6)
        );
    }

    #[test]
    fn test_transformative_element_values() {
        assert_eq!(
            transformative_element(Reaction::Overloaded),
            Some(Some(Element::Pyro))
        );
        assert_eq!(
            transformative_element(Reaction::Superconduct),
            Some(Some(Element::Cryo))
        );
        assert_eq!(transformative_element(Reaction::Shattered), Some(None));
        assert_eq!(
            transformative_element(Reaction::Swirl(Element::Hydro)),
            Some(Some(Element::Hydro))
        );
        assert_eq!(
            transformative_element(Reaction::Bloom),
            Some(Some(Element::Dendro))
        );
        assert_eq!(transformative_element(Reaction::Vaporize), None);
    }
}
