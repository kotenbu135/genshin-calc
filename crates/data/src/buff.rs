use serde::{Deserialize, Serialize};

pub use genshin_calc_core::BuffableStat;

/// A stat buff with a value and optional refinement scaling.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StatBuff {
    /// Which stat is buffed.
    pub stat: BuffableStat,
    /// Buff value at refinement 1 (or fixed value if no refinement).
    pub value: f64,
    /// Values at refinements 1-5. `None` for fixed buffs.
    pub refinement_values: Option<[f64; 5]>,
}

/// Passive effect from a weapon or artifact set.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PassiveEffect {
    /// Human-readable description.
    pub description: &'static str,
    /// Stat buffs provided by this effect.
    pub buffs: &'static [StatBuff],
}
