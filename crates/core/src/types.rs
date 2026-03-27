use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    Pyro,
    Hydro,
    Electro,
    Cryo,
    Dendro,
    Anemo,
    Geo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    Normal,
    Charged,
    Plunging,
    Skill,
    Burst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ScalingStat {
    #[default]
    Atk,
    Hp,
    Def,
}
