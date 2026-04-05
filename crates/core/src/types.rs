use serde::{Deserialize, Serialize};

/// Elements in Genshin Impact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    /// Fire element.
    Pyro,
    /// Water element.
    Hydro,
    /// Lightning element.
    Electro,
    /// Ice element.
    Cryo,
    /// Nature/grass element.
    Dendro,
    /// Wind element.
    Anemo,
    /// Earth/stone element.
    Geo,
}

/// Attack type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageType {
    /// Normal attack (basic combo).
    Normal,
    /// Charged attack (hold or special input).
    Charged,
    /// Plunging attack (falling from height).
    Plunging,
    /// Elemental skill (E key).
    Skill,
    /// Elemental burst (Q key).
    Burst,
}

/// Stat used for damage scaling.
///
/// Most characters scale on ATK. Some scale on HP (e.g. Hu Tao)
/// or DEF (e.g. Albedo, Noelle). Some Moonsign passives scale on EM.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ScalingStat {
    /// Scale on total ATK (default).
    #[default]
    Atk,
    /// Scale on total HP.
    Hp,
    /// Scale on total DEF.
    Def,
    /// Scale on elemental mastery.
    Em,
    /// Scale on critical rate (e.g. Rosaria A4).
    CritRate,
    /// Scale on total ATK (base + bonus). For buffs like Ineffa A4, Flins A4.
    TotalAtk,
}

/// Weapon type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    /// One-handed sword.
    Sword,
    /// Two-handed great sword.
    Claymore,
    /// Spear-type weapon.
    Polearm,
    /// Ranged bow weapon.
    Bow,
    /// Magic catalyst weapon.
    Catalyst,
}
