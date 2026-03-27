use crate::reaction::Reaction;
use crate::types::Element;

/// Errors returned by calculation functions.
///
/// Each variant includes the invalid value for debugging.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CalcError {
    #[error("character level must be 1..=90, got {0}")]
    InvalidCharacterLevel(u32),

    #[error("enemy level must be 1..=100, got {0}")]
    InvalidEnemyLevel(u32),

    #[error("crit_rate must be 0.0..=1.0, got {0}")]
    InvalidCritRate(f64),

    #[error("def_reduction must be 0.0..=1.0, got {0}")]
    InvalidDefReduction(f64),

    #[error("talent_multiplier must be > 0.0, got {0}")]
    InvalidTalentMultiplier(f64),

    #[error("amplifying reaction requires an element, but element is None (physical)")]
    AmplifyingRequiresElement,

    #[error("elemental_mastery must be >= 0.0, got {0}")]
    InvalidElementalMastery(f64),

    #[error("reaction_bonus must be >= 0.0, got {0}")]
    InvalidReactionBonus(f64),

    #[error("reaction {0:?} is not amplifying or catalyze; use calculate_transformative")]
    UseTransformativeFunction(Reaction),

    #[error("reaction {0:?} is not amplifying or catalyze; use calculate_lunar")]
    UseLunarFunction(Reaction),

    #[error("reaction {0:?} is not a transformative reaction")]
    NotTransformative(Reaction),

    #[error("reaction {0:?} is not a lunar reaction")]
    NotLunar(Reaction),

    #[error("swirl element must be Pyro, Hydro, Electro, or Cryo, got {0:?}")]
    InvalidSwirlElement(Element),

    #[error("invalid amplifying combination: {0:?} with {1:?} trigger")]
    InvalidAmplifyingCombination(Reaction, Element),

    #[error("character level must be 1..=100 for reaction calculations, got {0}")]
    InvalidReactionLevel(u32),

    #[error("invalid base value: {0} (must be >= 0)")]
    InvalidBaseValue(f64),

    #[error("invalid percent bonus: {0} (must be >= -1.0)")]
    InvalidPercentBonus(f64),

    #[error("invalid flat bonus: {0} (must be >= 0)")]
    InvalidFlatBonus(f64),

    #[error("crit_dmg must be >= 0.0, got {0}")]
    InvalidCritDmg(f64),

    #[error("energy_recharge must be >= 0.0, got {0}")]
    InvalidEnergyRecharge(f64),

    #[error("dmg_bonus must be >= -1.0, got {0}")]
    InvalidDmgBonus(f64),
}
