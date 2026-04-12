use serde::{Deserialize, Serialize};

use crate::types::Element;

/// Final character stats used for damage calculation.
///
/// All percentage fields use decimal form (e.g. 75% crit rate = `0.75`).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Stats {
    /// Max HP.
    pub hp: f64,
    /// Total ATK (base + bonus).
    pub atk: f64,
    /// Total DEF (base + bonus).
    pub def: f64,
    /// Elemental mastery.
    pub elemental_mastery: f64,
    /// Crit rate in decimal form (0.0 to 1.0).
    pub crit_rate: f64,
    /// Crit DMG in decimal form (base 0.50 = 50%).
    pub crit_dmg: f64,
    /// Energy recharge in decimal form (base 1.0 = 100%).
    pub energy_recharge: f64,
    /// DMG bonus in decimal form (e.g. 0.466 for Pyro DMG goblet).
    pub dmg_bonus: f64,
    /// Pyro DMG bonus in decimal form.
    #[serde(default)]
    pub pyro_dmg_bonus: f64,
    /// Pyro-only crit DMG bonus in decimal form.
    #[serde(default)]
    pub pyro_crit_dmg_bonus: f64,
    /// Hydro DMG bonus in decimal form.
    #[serde(default)]
    pub hydro_dmg_bonus: f64,
    /// Hydro-only crit DMG bonus in decimal form.
    #[serde(default)]
    pub hydro_crit_dmg_bonus: f64,
    /// Electro DMG bonus in decimal form.
    #[serde(default)]
    pub electro_dmg_bonus: f64,
    /// Electro-only crit DMG bonus in decimal form.
    #[serde(default)]
    pub electro_crit_dmg_bonus: f64,
    /// Cryo DMG bonus in decimal form.
    #[serde(default)]
    pub cryo_dmg_bonus: f64,
    /// Cryo-only crit DMG bonus in decimal form.
    #[serde(default)]
    pub cryo_crit_dmg_bonus: f64,
    /// Dendro DMG bonus in decimal form.
    #[serde(default)]
    pub dendro_dmg_bonus: f64,
    /// Dendro-only crit DMG bonus in decimal form.
    #[serde(default)]
    pub dendro_crit_dmg_bonus: f64,
    /// Anemo DMG bonus in decimal form.
    #[serde(default)]
    pub anemo_dmg_bonus: f64,
    /// Anemo-only crit DMG bonus in decimal form.
    #[serde(default)]
    pub anemo_crit_dmg_bonus: f64,
    /// Geo DMG bonus in decimal form.
    #[serde(default)]
    pub geo_dmg_bonus: f64,
    /// Geo-only crit DMG bonus in decimal form.
    #[serde(default)]
    pub geo_crit_dmg_bonus: f64,
    /// Physical DMG bonus in decimal form.
    #[serde(default)]
    pub physical_dmg_bonus: f64,
    /// Physical-only crit DMG bonus in decimal form.
    #[serde(default)]
    pub physical_crit_dmg_bonus: f64,
}

impl Stats {
    /// Returns the total DMG bonus for the given element.
    /// `None` means physical damage.
    pub fn total_dmg_bonus(&self, element: Option<Element>) -> f64 {
        self.dmg_bonus
            + match element {
                Some(Element::Pyro) => self.pyro_dmg_bonus,
                Some(Element::Hydro) => self.hydro_dmg_bonus,
                Some(Element::Electro) => self.electro_dmg_bonus,
                Some(Element::Cryo) => self.cryo_dmg_bonus,
                Some(Element::Dendro) => self.dendro_dmg_bonus,
                Some(Element::Anemo) => self.anemo_dmg_bonus,
                Some(Element::Geo) => self.geo_dmg_bonus,
                None => self.physical_dmg_bonus,
            }
    }

    /// Returns the total crit DMG for the given damage element.
    /// `None` means physical damage, which only uses generic crit DMG.
    pub fn total_crit_dmg(&self, element: Option<Element>) -> f64 {
        self.crit_dmg
            + match element {
                Some(Element::Pyro) => self.pyro_crit_dmg_bonus,
                Some(Element::Hydro) => self.hydro_crit_dmg_bonus,
                Some(Element::Electro) => self.electro_crit_dmg_bonus,
                Some(Element::Cryo) => self.cryo_crit_dmg_bonus,
                Some(Element::Dendro) => self.dendro_crit_dmg_bonus,
                Some(Element::Anemo) => self.anemo_crit_dmg_bonus,
                Some(Element::Geo) => self.geo_crit_dmg_bonus,
                None => self.physical_crit_dmg_bonus,
            }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Element;

    #[test]
    fn total_dmg_bonus_pyro() {
        let stats = Stats {
            dmg_bonus: 0.10,
            pyro_dmg_bonus: 0.466,
            ..Default::default()
        };
        let result = stats.total_dmg_bonus(Some(Element::Pyro));
        assert!((result - 0.566).abs() < 1e-10);
    }

    #[test]
    fn total_dmg_bonus_physical() {
        let stats = Stats {
            dmg_bonus: 0.10,
            physical_dmg_bonus: 0.583,
            ..Default::default()
        };
        let result = stats.total_dmg_bonus(None);
        assert!((result - 0.683).abs() < 1e-10);
    }

    #[test]
    fn total_dmg_bonus_no_element_bonus() {
        let stats = Stats {
            dmg_bonus: 0.15,
            pyro_dmg_bonus: 0.466,
            ..Default::default()
        };
        // Hydro damage should NOT include Pyro bonus
        let result = stats.total_dmg_bonus(Some(Element::Hydro));
        assert!((result - 0.15).abs() < 1e-10);
    }

    #[test]
    fn total_crit_dmg_geo() {
        let stats = Stats {
            crit_dmg: 0.50,
            geo_crit_dmg_bonus: 0.40,
            ..Default::default()
        };
        let result = stats.total_crit_dmg(Some(Element::Geo));
        assert!((result - 0.90).abs() < 1e-10);
    }

    #[test]
    fn total_crit_dmg_non_matching_element_ignores_geo_bonus() {
        let stats = Stats {
            crit_dmg: 0.50,
            geo_crit_dmg_bonus: 0.40,
            ..Default::default()
        };
        let pyro = stats.total_crit_dmg(Some(Element::Pyro));
        let physical = stats.total_crit_dmg(None);
        assert!((pyro - 0.50).abs() < 1e-10);
        assert!((physical - 0.50).abs() < 1e-10);
    }
}
