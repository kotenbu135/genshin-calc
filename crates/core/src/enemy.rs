use serde::{Deserialize, Serialize};

/// Enemy parameters for damage calculation.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Enemy {
    /// Enemy level (1-100).
    pub level: u32,
    /// Elemental resistance in decimal form (e.g. 0.10 = 10%).
    pub resistance: f64,
    /// DEF reduction from debuffs in decimal form (0.0 to 1.0).
    pub def_reduction: f64,
}
