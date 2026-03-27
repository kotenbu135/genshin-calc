use serde::{Deserialize, Serialize};

use crate::buff_types::BuffableStat;
use crate::types::Element;

/// Elemental resonance effects for 4-member teams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ElementalResonance {
    /// Pyro ×2: ATK +25%.
    FerventFlames,
    /// Hydro ×2: HP +25%.
    SoothingWater,
    /// Electro ×2: no stat effect.
    HighVoltage,
    /// Cryo ×2: conditional CRIT Rate +15%.
    ShatteringIce,
    /// Anemo ×2: no stat effect.
    ImpetuousWinds,
    /// Geo ×2: conditional DMG +15%.
    EnduringRock,
    /// Dendro ×2: EM +50.
    SprawlingGreenery,
    /// 4 unique elements: no damage stat effect.
    ProtectiveCanopy,
}

/// Determines elemental resonances from team member elements.
///
/// Returns empty if team has fewer than 4 members.
/// ProtectiveCanopy is exclusive — only triggers when all 4 elements are different.
pub fn determine_resonances(elements: &[Element]) -> Vec<ElementalResonance> {
    if elements.len() != 4 {
        return vec![];
    }

    use std::collections::HashMap;
    let mut counts: HashMap<Element, usize> = HashMap::new();
    for &e in elements {
        *counts.entry(e).or_insert(0) += 1;
    }

    // 全4元素が異なる場合
    if counts.len() == 4 {
        return vec![ElementalResonance::ProtectiveCanopy];
    }

    let mut resonances = vec![];
    for (&element, &count) in &counts {
        if count >= 2 {
            let r = match element {
                Element::Pyro => ElementalResonance::FerventFlames,
                Element::Hydro => ElementalResonance::SoothingWater,
                Element::Electro => ElementalResonance::HighVoltage,
                Element::Cryo => ElementalResonance::ShatteringIce,
                Element::Anemo => ElementalResonance::ImpetuousWinds,
                Element::Geo => ElementalResonance::EnduringRock,
                Element::Dendro => ElementalResonance::SprawlingGreenery,
            };
            resonances.push(r);
        }
    }
    resonances.sort_by_key(|r| *r as u8);
    resonances
}

/// Returns unconditional stat buffs for a resonance.
///
/// Returns empty for resonances with no stat effect or conditional resonances.
pub fn resonance_buffs(resonance: ElementalResonance) -> Vec<(BuffableStat, f64)> {
    match resonance {
        ElementalResonance::FerventFlames => {
            vec![(BuffableStat::AtkPercent, 0.25)]
        }
        ElementalResonance::SoothingWater => {
            vec![(BuffableStat::HpPercent, 0.25)]
        }
        ElementalResonance::SprawlingGreenery => {
            vec![(BuffableStat::ElementalMastery, 50.0)]
        }
        // Conditional or no stat effect
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fewer_than_4_returns_empty() {
        assert!(determine_resonances(&[Element::Pyro, Element::Pyro]).is_empty());
        assert!(determine_resonances(&[Element::Pyro, Element::Pyro, Element::Hydro]).is_empty());
        assert!(determine_resonances(&[]).is_empty());
    }

    #[test]
    fn test_pyro_resonance() {
        let elements = [Element::Pyro, Element::Pyro, Element::Hydro, Element::Cryo];
        let res = determine_resonances(&elements);
        assert!(res.contains(&ElementalResonance::FerventFlames));
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_double_resonance() {
        let elements = [Element::Pyro, Element::Pyro, Element::Hydro, Element::Hydro];
        let res = determine_resonances(&elements);
        assert!(res.contains(&ElementalResonance::FerventFlames));
        assert!(res.contains(&ElementalResonance::SoothingWater));
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_four_unique_elements() {
        let elements = [
            Element::Pyro,
            Element::Hydro,
            Element::Electro,
            Element::Cryo,
        ];
        let res = determine_resonances(&elements);
        assert_eq!(res, vec![ElementalResonance::ProtectiveCanopy]);
    }

    #[test]
    fn test_triple_same_element() {
        let elements = [Element::Pyro, Element::Pyro, Element::Pyro, Element::Hydro];
        let res = determine_resonances(&elements);
        assert!(res.contains(&ElementalResonance::FerventFlames));
        assert_eq!(res.len(), 1);
    }

    #[test]
    fn test_fervent_flames_buffs() {
        let buffs = resonance_buffs(ElementalResonance::FerventFlames);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0], (BuffableStat::AtkPercent, 0.25));
    }

    #[test]
    fn test_soothing_water_buffs() {
        let buffs = resonance_buffs(ElementalResonance::SoothingWater);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0], (BuffableStat::HpPercent, 0.25));
    }

    #[test]
    fn test_sprawling_greenery_buffs() {
        let buffs = resonance_buffs(ElementalResonance::SprawlingGreenery);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0], (BuffableStat::ElementalMastery, 50.0));
    }

    #[test]
    fn test_conditional_resonance_no_buffs() {
        assert!(resonance_buffs(ElementalResonance::ShatteringIce).is_empty());
        assert!(resonance_buffs(ElementalResonance::EnduringRock).is_empty());
        assert!(resonance_buffs(ElementalResonance::HighVoltage).is_empty());
        assert!(resonance_buffs(ElementalResonance::ImpetuousWinds).is_empty());
        assert!(resonance_buffs(ElementalResonance::ProtectiveCanopy).is_empty());
    }
}
