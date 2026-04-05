//! Nightsoul character lookup.
//!
//! Characters that can use the Nightsoul's Blessing (夜魂の加護) mechanic.
//! Primarily Natlan characters plus some exceptions.

/// Returns true if the character can use Nightsoul's Blessing.
pub fn is_nightsoul_character(id: &str) -> bool {
    NIGHTSOUL_CHARACTER_IDS.contains(&id)
}

/// All character IDs that can use Nightsoul's Blessing.
const NIGHTSOUL_CHARACTER_IDS: &[&str] = &[
    "kachina",
    "kinich",
    "mualani",
    "xilonen",
    "chasca",
    "ororon",
    "citlali",
    "mavuika",
    "iansan",
    "varesa",
    "yumemizuki_mizuki",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_nightsoul_known_characters() {
        assert!(is_nightsoul_character("kachina"));
        assert!(is_nightsoul_character("kinich"));
        assert!(is_nightsoul_character("mualani"));
        assert!(is_nightsoul_character("xilonen"));
        assert!(is_nightsoul_character("mavuika"));
        assert!(!is_nightsoul_character("bennett"));
        assert!(!is_nightsoul_character("diluc"));
        assert!(!is_nightsoul_character("hu_tao"));
    }

    #[test]
    fn test_non_nightsoul_characters() {
        assert!(!is_nightsoul_character("unknown_char"));
        assert!(!is_nightsoul_character(""));
    }
}
