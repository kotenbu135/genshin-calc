//! Moonsign character data: benedictions, talent enhancements, and lookup functions.

use genshin_calc_core::{
    BuffTarget, BuffableStat, MoonsignLevel, MoonsignTalentEffect, MoonsignTalentEnhancement,
    Reaction, ScalingStat,
};

/// Definition of a Moonsign Benediction passive.
#[derive(Debug, Clone, serde::Serialize)]
pub struct MoonsignBenedictionDef {
    /// Character identifier (lowercase, e.g. `"ineffa"`).
    pub character_id: &'static str,
    /// Character display name.
    pub character_name: &'static str,
    /// Lunar reactions this benediction applies to.
    pub enabled_reactions: &'static [Reaction],
    /// Stat used for bonus scaling. `None` if the character has no personal scaling.
    pub scaling_stat: Option<ScalingStat>,
    /// Rate per 1 unit of stat.
    pub rate: f64,
    /// Maximum bonus value (cap).
    pub max_bonus: f64,
}

/// All moonsign benediction definitions.
pub const ALL_MOONSIGN_BENEDICTIONS: &[MoonsignBenedictionDef] = &[
    MoonsignBenedictionDef {
        character_id: "ineffa",
        character_name: "Ineffa",
        enabled_reactions: &[Reaction::LunarElectroCharged],
        scaling_stat: Some(ScalingStat::Atk),
        rate: 0.00007,
        max_bonus: 0.14,
    },
    MoonsignBenedictionDef {
        character_id: "flins",
        character_name: "Flins",
        enabled_reactions: &[Reaction::LunarElectroCharged],
        scaling_stat: Some(ScalingStat::Atk),
        rate: 0.00007,
        max_bonus: 0.14,
    },
    MoonsignBenedictionDef {
        character_id: "lauma",
        character_name: "Lauma",
        enabled_reactions: &[Reaction::LunarBloom],
        scaling_stat: Some(ScalingStat::Em),
        rate: 0.000175,
        max_bonus: 0.14,
    },
    MoonsignBenedictionDef {
        character_id: "nefer",
        character_name: "Nefer",
        enabled_reactions: &[Reaction::LunarBloom],
        scaling_stat: Some(ScalingStat::Em),
        rate: 0.000175,
        max_bonus: 0.14,
    },
    MoonsignBenedictionDef {
        character_id: "zibai",
        character_name: "Zibai",
        enabled_reactions: &[Reaction::LunarCrystallize],
        scaling_stat: Some(ScalingStat::Def),
        rate: 0.00007,
        max_bonus: 0.14,
    },
    MoonsignBenedictionDef {
        character_id: "columbina",
        character_name: "Columbina",
        enabled_reactions: &[
            Reaction::LunarElectroCharged,
            Reaction::LunarBloom,
            Reaction::LunarCrystallize,
        ],
        scaling_stat: Some(ScalingStat::Hp),
        rate: 0.000002,
        max_bonus: 0.07,
    },
    // Aino — A1 raises party Moonsign level by 1 (handled by MoonsignContext.level,
    // not via benediction stat scaling). No personal lunar reaction DMG scaling.
    MoonsignBenedictionDef {
        character_id: "aino",
        character_name: "Aino",
        enabled_reactions: &[],
        scaling_stat: None,
        rate: 0.0,
        max_bonus: 0.0,
    },
    MoonsignBenedictionDef {
        character_id: "jahoda",
        character_name: "Jahoda",
        enabled_reactions: &[],
        scaling_stat: None,
        rate: 0.0,
        max_bonus: 0.0,
    },
    MoonsignBenedictionDef {
        character_id: "illuga",
        character_name: "Illuga",
        enabled_reactions: &[],
        scaling_stat: None,
        rate: 0.0,
        max_bonus: 0.0,
    },
];

/// Talent enhancements for Lauma.
pub const LAUMA_TALENT_ENHANCEMENTS: &[MoonsignTalentEnhancement] = &[MoonsignTalentEnhancement {
    character_name: "Lauma",
    required_level: MoonsignLevel::NascentGleam,
    description: desc!("Bloom gains crit at Nascent Gleam"),
    effect: MoonsignTalentEffect::GrantReactionCrit {
        reaction: Reaction::LunarBloom,
        crit_rate: 0.15,
        crit_dmg: 1.0,
    },
}];

/// Talent enhancements for Flins.
pub const FLINS_TALENT_ENHANCEMENTS: &[MoonsignTalentEnhancement] = &[MoonsignTalentEnhancement {
    character_name: "Flins",
    required_level: MoonsignLevel::AscendantGleam,
    description: desc!("Lunar-Charged DMG +20% at Ascendant Gleam"),
    effect: MoonsignTalentEffect::ReactionDmgBonus {
        reaction: Reaction::LunarElectroCharged,
        bonus: 0.20,
    },
}];

/// Talent enhancements for Nefer.
pub const NEFER_TALENT_ENHANCEMENTS: &[MoonsignTalentEnhancement] = &[MoonsignTalentEnhancement {
    character_name: "Nefer",
    required_level: MoonsignLevel::AscendantGleam,
    description: desc!(
        "At Ascendant Gleam, absorbing Seeds of Deceit grants Veil of Falsehood stacks (max 3). At 3 stacks, EM +100 for 8s"
    ),
    effect: MoonsignTalentEffect::StatBuff {
        stat: BuffableStat::ElementalMastery,
        value: 100.0,
        target: BuffTarget::OnlySelf,
    },
}];

/// Talent enhancements for Aino.
/// C6 reaction DMG bonus varies by Moonsign level: Lv1=+15%, Lv2=+35%.
pub const AINO_TALENT_ENHANCEMENTS: &[MoonsignTalentEnhancement] = &[
    MoonsignTalentEnhancement {
        character_name: "Aino",
        required_level: MoonsignLevel::NascentGleam,
        description: desc!("C6: At Nascent Gleam+, reaction DMG +15% for 15s after Burst"),
        effect: MoonsignTalentEffect::StatBuff {
            stat: BuffableStat::TransformativeBonus,
            value: 0.15,
            target: BuffTarget::Team,
        },
    },
    MoonsignTalentEnhancement {
        character_name: "Aino",
        required_level: MoonsignLevel::AscendantGleam,
        description: desc!(
            "C6: At Ascendant Gleam, reaction DMG bonus increases by +20% (total +35%)"
        ),
        effect: MoonsignTalentEffect::ReactionDmgBonus {
            reaction: Reaction::LunarElectroCharged,
            bonus: 0.20,
        },
    },
];

/// Returns `true` if the character ID belongs to a moonsign character.
#[must_use]
pub fn is_moonsign_character(id: &str) -> bool {
    ALL_MOONSIGN_BENEDICTIONS
        .iter()
        .any(|d| d.character_id == id)
}

/// Finds a moonsign benediction definition by character ID.
#[must_use]
pub fn find_moonsign_benediction(id: &str) -> Option<&'static MoonsignBenedictionDef> {
    ALL_MOONSIGN_BENEDICTIONS
        .iter()
        .find(|d| d.character_id == id)
}

/// Calculate the BaseDMGBonus from a benediction definition.
#[must_use]
pub fn calculate_benediction_bonus(def: &MoonsignBenedictionDef, stat_value: f64) -> f64 {
    (def.rate * stat_value).min(def.max_bonus)
}

/// Get talent enhancements for a character, filtered by moonsign level.
#[must_use]
pub fn find_moonsign_talent_enhancements(
    id: &str,
    level: MoonsignLevel,
) -> Vec<&'static MoonsignTalentEnhancement> {
    let enhancements: &[MoonsignTalentEnhancement] = match id {
        "lauma" => LAUMA_TALENT_ENHANCEMENTS,
        "flins" => FLINS_TALENT_ENHANCEMENTS,
        "nefer" => NEFER_TALENT_ENHANCEMENTS,
        "aino" => AINO_TALENT_ENHANCEMENTS,
        _ => &[],
    };
    enhancements
        .iter()
        .filter(|e| {
            matches!(
                (e.required_level, level),
                (MoonsignLevel::None, _)
                    | (
                        MoonsignLevel::NascentGleam,
                        MoonsignLevel::NascentGleam | MoonsignLevel::AscendantGleam,
                    )
                    | (MoonsignLevel::AscendantGleam, MoonsignLevel::AscendantGleam)
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-6;

    #[test]
    fn test_is_moonsign_known_characters() {
        assert!(is_moonsign_character("ineffa"));
        assert!(is_moonsign_character("flins"));
        assert!(is_moonsign_character("lauma"));
        assert!(is_moonsign_character("columbina"));
        assert!(is_moonsign_character("aino"));
        assert!(!is_moonsign_character("bennett"));
        assert!(!is_moonsign_character("diluc"));
    }

    #[test]
    fn test_find_moonsign_benediction_ineffa() {
        let def = find_moonsign_benediction("ineffa").unwrap();
        assert_eq!(def.enabled_reactions, &[Reaction::LunarElectroCharged]);
        assert!((def.max_bonus - 0.14).abs() < EPSILON);
    }

    #[test]
    fn test_find_moonsign_benediction_columbina_all_reactions() {
        let def = find_moonsign_benediction("columbina").unwrap();
        assert_eq!(def.enabled_reactions.len(), 3);
        assert!(
            def.enabled_reactions
                .contains(&Reaction::LunarElectroCharged)
        );
        assert!(def.enabled_reactions.contains(&Reaction::LunarBloom));
        assert!(def.enabled_reactions.contains(&Reaction::LunarCrystallize));
        assert!((def.max_bonus - 0.07).abs() < EPSILON);
    }

    #[test]
    fn test_find_moonsign_benediction_aino_none() {
        let def = find_moonsign_benediction("aino").unwrap();
        assert!(def.enabled_reactions.is_empty());
        assert!((def.max_bonus - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_benediction_bonus_ineffa() {
        let def = find_moonsign_benediction("ineffa").unwrap();
        assert!((calculate_benediction_bonus(def, 2000.0) - 0.14).abs() < EPSILON);
        assert!((calculate_benediction_bonus(def, 1000.0) - 0.07).abs() < EPSILON);
        assert!((calculate_benediction_bonus(def, 3000.0) - 0.14).abs() < EPSILON);
    }

    #[test]
    fn test_calculate_benediction_bonus_lauma_em() {
        let def = find_moonsign_benediction("lauma").unwrap();
        assert!((calculate_benediction_bonus(def, 800.0) - 0.14).abs() < EPSILON);
        assert!((calculate_benediction_bonus(def, 400.0) - 0.07).abs() < EPSILON);
    }

    #[test]
    fn test_find_moonsign_talent_enhancements_lauma() {
        let enhancements = find_moonsign_talent_enhancements("lauma", MoonsignLevel::NascentGleam);
        assert_eq!(enhancements.len(), 1);
        assert_eq!(enhancements[0].required_level, MoonsignLevel::NascentGleam);
    }

    #[test]
    fn test_find_moonsign_talent_enhancements_lauma_none_level() {
        let enhancements = find_moonsign_talent_enhancements("lauma", MoonsignLevel::None);
        assert!(enhancements.is_empty());
    }

    #[test]
    fn test_aino_talent_enhancements_at_ascendant_gleam() {
        let enhancements = find_moonsign_talent_enhancements("aino", MoonsignLevel::AscendantGleam);
        // NascentGleam (+15%) + AscendantGleam (+20%) = 2 enhancements
        assert_eq!(enhancements.len(), 2);
    }

    #[test]
    fn test_aino_talent_enhancements_at_nascent_gleam() {
        let enhancements = find_moonsign_talent_enhancements("aino", MoonsignLevel::NascentGleam);
        // C6 base +15% activates at NascentGleam
        assert_eq!(enhancements.len(), 1);
        assert_eq!(enhancements[0].required_level, MoonsignLevel::NascentGleam);
    }

    #[test]
    fn test_all_moonsign_characters_count() {
        assert_eq!(ALL_MOONSIGN_BENEDICTIONS.len(), 9);
    }
}
