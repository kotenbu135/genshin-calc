pub mod damage;
pub mod enemy;
pub mod error;
pub mod stats;
pub mod types;

pub use damage::{DamageInput, DamageResult, calculate_damage};
pub use enemy::Enemy;
pub use error::CalcError;
pub use stats::Stats;
pub use types::{DamageType, Element};

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
            damage_type: DamageType::Skill,
            element: Some(Element::Pyro),
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
