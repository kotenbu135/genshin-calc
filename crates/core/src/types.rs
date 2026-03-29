use serde::{Deserialize, Serialize};

/// Elements in Genshin Impact.
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

/// Attack type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    Normal,
    Charged,
    Plunging,
    Skill,
    Burst,
}

/// Stat used for damage scaling.
///
/// Most characters scale on ATK. Some scale on HP (e.g. Hu Tao)
/// or DEF (e.g. Albedo, Noelle).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ScalingStat {
    #[default]
    Atk,
    Hp,
    Def,
}
