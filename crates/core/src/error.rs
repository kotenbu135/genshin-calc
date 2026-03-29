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
}
