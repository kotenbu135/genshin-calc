/// Issue #96: Fix constellation patterns for 15 characters
/// Source: honeyhunter-mirror/md/characters/
use genshin_calc_data::{find_character, types::ConstellationPattern};

#[test]
fn test_anemo_c3skill_c5burst() {
    // All verified from HH: C3 boosts Elemental Skill, C5 boosts Elemental Burst
    let chars = [
        "faruzan", "heizou", "ifa", "kazuha", "lan_yan", "mizuki", "sucrose", "xiao",
    ];
    for id in chars {
        let c = find_character(id).unwrap_or_else(|| panic!("character not found: {}", id));
        assert_eq!(
            c.constellation_pattern,
            ConstellationPattern::C3SkillC5Burst,
            "{} should be C3SkillC5Burst",
            id
        );
    }
}

#[test]
fn test_pyro_c3skill_c5burst() {
    // gaming: C3=Bestial Ascent (Skill), C5=Suanni's Gilded Dance (Burst)
    // klee: C3=Jumpy Dumpty (Skill), C5=Sparks 'n' Splash (Burst)
    // yoimiya: C3=Niwabi Fire-Dance (Skill), C5=Ryuukin Saxifrage (Burst)
    let chars = ["gaming", "klee", "yoimiya"];
    for id in chars {
        let c = find_character(id).unwrap_or_else(|| panic!("character not found: {}", id));
        assert_eq!(
            c.constellation_pattern,
            ConstellationPattern::C3SkillC5Burst,
            "{} should be C3SkillC5Burst",
            id
        );
    }
}

#[test]
fn test_lyney_c3normal_c5burst() {
    // Lyney: C3 boosts Normal Attack, C5 boosts Elemental Burst
    let c = find_character("lyney").expect("lyney not found");
    assert_eq!(
        c.constellation_pattern,
        ConstellationPattern::C3NormalC5Burst,
        "lyney should be C3NormalC5Burst"
    );
}

#[test]
fn test_ayato_c3skill_c5burst() {
    // Ayato: C3=Kamisato Art: Kyouka (Skill), C5=Kamisato Art: Suiyuu (Burst)
    let c = find_character("ayato").expect("ayato not found");
    assert_eq!(
        c.constellation_pattern,
        ConstellationPattern::C3SkillC5Burst,
        "ayato should be C3SkillC5Burst"
    );
}

#[test]
fn test_neuvillette_c3normal_c5burst() {
    // Neuvillette: C3=Normal Attack (As Water Seeks Equilibrium), C5=O Tides I Have Returned (Burst)
    let c = find_character("neuvillette").expect("neuvillette not found");
    assert_eq!(
        c.constellation_pattern,
        ConstellationPattern::C3NormalC5Burst,
        "neuvillette should be C3NormalC5Burst"
    );
}

#[test]
fn test_freminet_c3normal_c5skill() {
    // Freminet: C3=Song of the Eddies (Normal Attack), C5=Nights of Hearth (Pressurized Floe = Skill)
    let c = find_character("freminet").expect("freminet not found");
    assert_eq!(
        c.constellation_pattern,
        ConstellationPattern::C3NormalC5Skill,
        "freminet should be C3NormalC5Skill"
    );
}

#[test]
fn test_freminet_skill_boosted_at_c5_not_burst() {
    // With new C3NormalC5Skill: C5 freminet should boost Skill, not Burst
    use genshin_calc_core::DamageType;
    let c = find_character("freminet").expect("freminet not found");
    // At C5, Skill gets +3 boost
    let skill_level = c.effective_talent_level(DamageType::Skill, 9, 5);
    assert_eq!(
        skill_level, 12,
        "C5 Freminet: Skill level 9 should become 12"
    );
    // Burst is NOT boosted at C5
    let burst_level = c.effective_talent_level(DamageType::Burst, 9, 5);
    assert_eq!(burst_level, 9, "C5 Freminet: Burst level 9 should stay 9");
    // At C3, Normal gets +3 boost
    let normal_level = c.effective_talent_level(DamageType::Normal, 9, 3);
    assert_eq!(
        normal_level, 12,
        "C3 Freminet: Normal level 9 should become 12"
    );
}
