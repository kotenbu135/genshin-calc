use crate::reaction::Reaction;
use crate::types::Element;

/// Errors returned by calculation functions.
///
/// Each variant includes the invalid value for debugging.
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CalcError {
    /// Character level is outside the valid range of 1..=90.
    #[error("character level must be 1..=90, got {0}")]
    InvalidCharacterLevel(u32),

    /// Enemy level is outside the valid range of 1..=200.
    #[error("enemy level must be 1..=200, got {0}")]
    InvalidEnemyLevel(u32),

    /// Crit rate is outside the valid range of 0.0..=1.0.
    #[error("crit_rate must be 0.0..=1.0, got {0}")]
    InvalidCritRate(f64),

    /// DEF reduction is outside the valid range of 0.0..=1.0.
    #[error("def_reduction must be 0.0..=1.0, got {0}")]
    InvalidDefReduction(f64),

    /// DEF ignore is outside the valid range of 0.0..=1.0.
    #[error("def_ignore must be 0.0..=1.0, got {0}")]
    InvalidDefIgnore(f64),

    /// Talent multiplier must be positive.
    #[error("talent_multiplier must be > 0.0, got {0}")]
    InvalidTalentMultiplier(f64),

    /// Amplifying reactions require an elemental attack (not physical).
    #[error("amplifying reaction requires an element, but element is None (physical)")]
    AmplifyingRequiresElement,

    /// Elemental mastery must be non-negative.
    #[error("elemental_mastery must be >= 0.0, got {0}")]
    InvalidElementalMastery(f64),

    /// Reaction bonus must be non-negative.
    #[error("reaction_bonus must be >= 0.0, got {0}")]
    InvalidReactionBonus(f64),

    /// Reaction is transformative; use `calculate_transformative` instead.
    #[error("reaction {0:?} is not amplifying or catalyze; use calculate_transformative")]
    UseTransformativeFunction(Reaction),

    /// Reaction is lunar; use `calculate_lunar` instead.
    #[error("reaction {0:?} is not amplifying or catalyze; use calculate_lunar")]
    UseLunarFunction(Reaction),

    /// Reaction is not a transformative reaction.
    #[error("reaction {0:?} is not a transformative reaction")]
    NotTransformative(Reaction),

    /// Reaction is not a lunar reaction.
    #[error("reaction {0:?} is not a lunar reaction")]
    NotLunar(Reaction),

    /// Swirl element must be Pyro, Hydro, Electro, or Cryo.
    #[error("swirl element must be Pyro, Hydro, Electro, or Cryo, got {0:?}")]
    InvalidSwirlElement(Element),

    /// Invalid amplifying reaction and trigger element combination.
    #[error("invalid amplifying combination: {0:?} with {1:?} trigger")]
    InvalidAmplifyingCombination(Reaction, Element),

    /// Character level for reaction calculations must be 1..=100.
    #[error("character level must be 1..=100 for reaction calculations, got {0}")]
    InvalidReactionLevel(u32),

    /// Base stat value must be non-negative.
    #[error("invalid base value: {0} (must be >= 0)")]
    InvalidBaseValue(f64),

    /// Percent bonus must be >= -1.0.
    #[error("invalid percent bonus: {0} (must be >= -1.0)")]
    InvalidPercentBonus(f64),

    /// Flat bonus must be non-negative.
    #[error("invalid flat bonus: {0} (must be >= 0)")]
    InvalidFlatBonus(f64),

    /// Crit DMG must be non-negative.
    #[error("crit_dmg must be >= 0.0, got {0}")]
    InvalidCritDmg(f64),

    /// Energy recharge must be non-negative.
    #[error("energy_recharge must be >= 0.0, got {0}")]
    InvalidEnergyRecharge(f64),

    /// DMG bonus must be >= -1.0.
    #[error("dmg_bonus must be >= -1.0, got {0}")]
    InvalidDmgBonus(f64),

    /// Team must have 1 to 4 members.
    #[error("team must have 1..=4 members, got {0}")]
    InvalidTeamSize(usize),

    /// Target index is out of bounds for the given team size.
    #[error("target index {index} out of bounds for team of size {team_size}")]
    InvalidTargetIndex {
        /// The requested target index.
        index: usize,
        /// The actual team size.
        team_size: usize,
    },

    /// Constellation must be 0..=6.
    #[error("constellation must be 0..=6, got {0}")]
    InvalidConstellation(u8),

    /// Talent level must be 1..=15.
    #[error("talent level must be 1..=15, got {0}")]
    InvalidTalentLevel(u8),

    /// Weapon refinement must be 1..=5.
    #[error("weapon refinement must be 1..=5, got {0}")]
    InvalidRefinement(u8),
}
