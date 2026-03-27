use serde::{Deserialize, Serialize};

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
}
