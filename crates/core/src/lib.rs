//! # genshin-calc-core
//!
//! Damage and elemental reaction calculation engine for Genshin Impact.
//!
//! ## Calculation Pipelines
//!
//! | Pipeline | Function | Reactions |
//! |----------|----------|-----------|
//! | Standard | [`calculate_damage`] | Amplifying (vaporize/melt), Catalyze (spread/aggravate), or none |
//! | Transformative | [`calculate_transformative`] | Overloaded, superconduct, electro-charged, swirl, bloom, etc. |
//! | Lunar | [`calculate_lunar`] | Lunar electro-charged, lunar bloom, lunar crystallize |
//!
//! Standard damage passes through ATK/HP/DEF scaling, crit, defense, and resistance.
//! Transformative damage scales with character level and elemental mastery only (no crit, no defense).
//! Lunar damage scales like transformative but can crit.
//!
//! ## Example
//!
//! ```
//! use genshin_calc_core::*;
//!
//! let input = DamageInput {
//!     character_level: 90,
//!     stats: Stats {
//!         atk: 2000.0,
//!         crit_rate: 0.75,
//!         crit_dmg: 1.50,
//!         dmg_bonus: 0.466,
//!         ..Default::default()
//!     },
//!     talent_multiplier: 1.76,
//!     scaling_stat: ScalingStat::Atk,
//!     damage_type: DamageType::Skill,
//!     element: Some(Element::Pyro),
//!     reaction: None,
//!     reaction_bonus: 0.0,
//! };
//! let enemy = Enemy {
//!     level: 90,
//!     resistance: 0.10,
//!     def_reduction: 0.0,
//! };
//! let result = calculate_damage(&input, &enemy).unwrap();
//! assert!(result.average > 0.0);
//! ```

pub mod damage;
pub mod em;
pub mod enemy;
pub mod error;
pub mod level_table;
pub mod lunar;
pub mod reaction;
pub mod stat_profile;
pub mod stats;
pub mod transformative;
pub mod types;

pub use damage::{DamageInput, DamageResult, calculate_damage};
pub use em::{amplifying_em_bonus, catalyze_em_bonus, lunar_em_bonus, transformative_em_bonus};
pub use enemy::Enemy;
pub use error::CalcError;
pub use level_table::reaction_base_value;
pub use lunar::{LunarInput, LunarResult, calculate_lunar};
pub use reaction::{Reaction, ReactionCategory, determine_reaction};
pub use stat_profile::{StatProfile, combine_stats};
pub use stats::Stats;
pub use transformative::{TransformativeInput, TransformativeResult, calculate_transformative};
pub use types::{DamageType, Element, ScalingStat};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_api_usage_example() {
        let stats = Stats {
            hp: 20000.0,
            atk: 2000.0,
            def: 800.0,
            elemental_mastery: 100.0,
            crit_rate: 0.75,
            crit_dmg: 1.50,
            energy_recharge: 1.20,
            dmg_bonus: 0.466,
        };

        let input = DamageInput {
            character_level: 90,
            stats,
            talent_multiplier: 1.76,
            scaling_stat: ScalingStat::Atk,
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
            reaction: None,
            reaction_bonus: 0.0,
        };

        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };

        let result = calculate_damage(&input, &enemy).unwrap();
        assert!(result.non_crit > 0.0);
        assert!(result.crit > result.non_crit);
        assert!(result.average > result.non_crit);
        assert!(result.average < result.crit);
    }
}
