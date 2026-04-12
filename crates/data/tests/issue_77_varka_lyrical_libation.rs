use genshin_calc_core::BuffableStat;
use genshin_calc_data::{TalentBuffSource, find_talent_buffs};

const ANEMO_TALENT_BUFFS_SOURCE: &str = include_str!("../src/talent_buffs/anemo.rs");

#[test]
fn varka_lyrical_libation_is_documented_as_a_c1_system_limitation() {
    let defs = find_talent_buffs("varka").expect("Varka talent buffs should exist");

    assert!(
        ANEMO_TALENT_BUFFS_SOURCE.contains("Lyrical Libation"),
        "Varka talent buff code should document the Lyrical Libation limitation",
    );
    assert!(
        ANEMO_TALENT_BUFFS_SOURCE.contains("one-shot")
            && ANEMO_TALENT_BUFFS_SOURCE.contains("not representable"),
        "Varka talent buff code should explain that Lyrical Libation is a one-shot effect that is not representable in the current stat-buff model",
    );

    assert!(
        defs.iter().all(|buff| {
            !buff.name.contains("Lyrical Libation")
                && !buff.description.contains("Lyrical Libation")
                && !buff.description.contains("200% of the original DMG")
        }),
        "Varka should not expose Lyrical Libation as a fake persistent talent buff",
    );
    assert!(
        defs.iter().all(|buff| {
            !(buff.source == TalentBuffSource::Constellation(1)
                && matches!(
                    buff.stat,
                    BuffableStat::SkillDmgBonus | BuffableStat::ChargedAtkDmgBonus
                ))
        }),
        "Varka C1 should not be approximated as broad Skill/Charged DMG bonuses",
    );
}
