pub fn amplifying_em_bonus(em: f64) -> f64 {
    2.78 * em / (em + 1400.0)
}

pub fn catalyze_em_bonus(em: f64) -> f64 {
    5.0 * em / (em + 1200.0)
}

pub fn transformative_em_bonus(em: f64) -> f64 {
    16.0 * em / (em + 2000.0)
}

pub fn lunar_em_bonus(em: f64) -> f64 {
    6.0 * em / (em + 2000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_amplifying_em_zero() {
        assert!((amplifying_em_bonus(0.0) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_amplifying_em_200() {
        // 2.78 * 200 / (200 + 1400) = 556 / 1600 = 0.3475
        assert!((amplifying_em_bonus(200.0) - 0.3475).abs() < EPSILON);
    }

    #[test]
    fn test_amplifying_em_1000() {
        // 2.78 * 1000 / (1000 + 1400) = 2780 / 2400 = 1.158333...
        assert!((amplifying_em_bonus(1000.0) - 2780.0 / 2400.0).abs() < EPSILON);
    }

    #[test]
    fn test_catalyze_em_zero() {
        assert!((catalyze_em_bonus(0.0) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_catalyze_em_200() {
        // 5.0 * 200 / (200 + 1200) = 1000 / 1400 = 0.714285...
        assert!((catalyze_em_bonus(200.0) - 1000.0 / 1400.0).abs() < EPSILON);
    }

    #[test]
    fn test_transformative_em_zero() {
        assert!((transformative_em_bonus(0.0) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_transformative_em_800() {
        // 16.0 * 800 / (800 + 2000) = 12800 / 2800 = 4.571428...
        assert!((transformative_em_bonus(800.0) - 12800.0 / 2800.0).abs() < EPSILON);
    }

    #[test]
    fn test_lunar_em_zero() {
        assert!((lunar_em_bonus(0.0) - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_lunar_em_300() {
        // 6.0 * 300 / (300 + 2000) = 1800 / 2300 = 0.782608...
        assert!((lunar_em_bonus(300.0) - 1800.0 / 2300.0).abs() < EPSILON);
    }
}
