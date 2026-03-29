use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Enemy {
    pub level: u32,
    pub resistance: f64,
    pub def_reduction: f64,
}
