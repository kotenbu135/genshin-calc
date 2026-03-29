use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Stats {
    pub hp: f64,
    pub atk: f64,
    pub def: f64,
    pub elemental_mastery: f64,
    pub crit_rate: f64,
    pub crit_dmg: f64,
    pub energy_recharge: f64,
    pub dmg_bonus: f64,
}
