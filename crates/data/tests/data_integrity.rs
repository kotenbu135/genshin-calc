use genshin_calc_core::{BuffableStat, Element, Reaction};
use genshin_calc_data::artifacts::ALL_ARTIFACT_SETS;
use genshin_calc_data::buff::{Activation, ConditionalBuff, ManualCondition};
use genshin_calc_data::characters::all_characters;
use genshin_calc_data::enemies::ALL_ENEMIES;
use genshin_calc_data::weapons::ALL_WEAPONS;

fn artifact_set(id: &str) -> &'static genshin_calc_data::types::ArtifactSet {
    ALL_ARTIFACT_SETS
        .iter()
        .copied()
        .find(|set| set.id == id)
        .unwrap_or_else(|| panic!("artifact set not found: {id}"))
}

fn conditional_values(
    set: &genshin_calc_data::types::ArtifactSet,
    stat: BuffableStat,
) -> Vec<f64> {
    set.four_piece
        .conditional_buffs
        .iter()
        .filter(|buff| buff.stat == stat)
        .map(|buff| buff.value)
        .collect()
}

#[test]
fn all_characters_have_positive_base_stats() {
    for c in all_characters() {
        for &hp in &c.base_hp {
            assert!(hp > 0.0, "{}: base_hp has non-positive value {}", c.id, hp);
        }
        for &atk in &c.base_atk {
            assert!(
                atk > 0.0,
                "{}: base_atk has non-positive value {}",
                c.id,
                atk
            );
        }
        for &def in &c.base_def {
            assert!(
                def > 0.0,
                "{}: base_def has non-positive value {}",
                c.id,
                def
            );
        }
    }
}

#[test]
fn all_characters_base_stats_ascending() {
    for c in all_characters() {
        // Index mapping: [Lv1, Lv20, Lv20+, Lv40, Lv40+, Lv50, Lv50+, Lv60, Lv60+, Lv70, Lv70+, Lv80, Lv80+, Lv90, Lv90+, Lv95, Lv95+, Lv100]
        assert!(c.base_hp[0] < c.base_hp[1], "{}: HP Lv1 >= Lv20", c.id);
        assert!(c.base_hp[1] <= c.base_hp[2], "{}: HP Lv20 > Lv20+", c.id);
        assert!(c.base_hp[2] < c.base_hp[3], "{}: HP Lv20+ >= Lv40", c.id);
        assert!(c.base_hp[3] <= c.base_hp[4], "{}: HP Lv40 > Lv40+", c.id);
        assert!(c.base_hp[12] <= c.base_hp[13], "{}: HP Lv80+ > Lv90", c.id);
        assert!(c.base_atk[0] < c.base_atk[1], "{}: ATK Lv1 >= Lv20", c.id);
        assert!(c.base_atk[1] <= c.base_atk[2], "{}: ATK Lv20 > Lv20+", c.id);
        assert!(c.base_atk[2] < c.base_atk[3], "{}: ATK Lv20+ >= Lv40", c.id);
        assert!(c.base_atk[3] <= c.base_atk[4], "{}: ATK Lv40 > Lv40+", c.id);
        assert!(
            c.base_atk[12] <= c.base_atk[13],
            "{}: ATK Lv80+ > Lv90",
            c.id
        );
        assert!(c.base_def[0] < c.base_def[1], "{}: DEF Lv1 >= Lv20", c.id);
        assert!(c.base_def[1] <= c.base_def[2], "{}: DEF Lv20 > Lv20+", c.id);
        assert!(c.base_def[2] < c.base_def[3], "{}: DEF Lv20+ >= Lv40", c.id);
        assert!(c.base_def[3] <= c.base_def[4], "{}: DEF Lv40 > Lv40+", c.id);
        assert!(
            c.base_def[12] <= c.base_def[13],
            "{}: DEF Lv80+ > Lv90",
            c.id
        );
    }
}

#[test]
fn all_characters_have_unique_ids() {
    let mut ids: Vec<&str> = all_characters().map(|c| c.id).collect();
    let total = ids.len();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), total, "Duplicate character IDs found");
}

#[test]
fn all_talent_values_non_negative() {
    for c in all_characters() {
        for scaling in c.talents.normal_attack.hits {
            for &v in &scaling.values {
                assert!(
                    v >= 0.0,
                    "{}: talent {} has negative value",
                    c.id,
                    scaling.name
                );
            }
        }
        for scaling in c.talents.normal_attack.charged {
            for &v in &scaling.values {
                assert!(
                    v >= 0.0,
                    "{}: charged {} has negative value",
                    c.id,
                    scaling.name
                );
            }
        }
        for scaling in c.talents.normal_attack.plunging {
            for &v in &scaling.values {
                assert!(
                    v >= 0.0,
                    "{}: plunging {} has negative value",
                    c.id,
                    scaling.name
                );
            }
        }
        for scaling in c.talents.elemental_skill.scalings {
            for &v in &scaling.values {
                assert!(
                    v >= 0.0,
                    "{}: skill {} has negative value",
                    c.id,
                    scaling.name
                );
            }
        }
        for scaling in c.talents.elemental_burst.scalings {
            for &v in &scaling.values {
                assert!(
                    v >= 0.0,
                    "{}: burst {} has negative value",
                    c.id,
                    scaling.name
                );
            }
        }
    }
}

#[test]
fn artifact_audit_nymphs_dream_stack_values_match_mirror() {
    let set = artifact_set("nymphs_dream");

    assert_eq!(
        conditional_values(set, BuffableStat::AtkPercent),
        vec![0.07, 0.16, 0.25]
    );
    assert_eq!(
        conditional_values(set, BuffableStat::ElementalDmgBonus(Element::Hydro)),
        vec![0.04, 0.09, 0.15]
    );
}

#[test]
fn artifact_audit_scroll_of_cinder_city_two_piece_has_no_damage_relevant_em_bonus() {
    let set = artifact_set("scroll_of_the_hero_of_cinder_city");

    assert!(
        set.two_piece
            .buffs
            .iter()
            .all(|buff| buff.stat != BuffableStat::ElementalMastery),
        "Scroll 2pc is energy-only in the mirror and must not grant EM"
    );
}

#[test]
fn artifact_audit_missing_damage_relevant_artifact_sets_are_registered() {
    for id in [
        "adventurer",
        "lucky_dog",
        "finale_of_the_deep_galleries",
        "long_nights_oath",
    ] {
        artifact_set(id);
    }
}

#[test]
fn artifact_audit_reaction_specific_buffs_are_not_generic() {
    let thundering_fury = artifact_set("thundering_fury");
    assert!(
        thundering_fury
            .four_piece
            .conditional_buffs
            .iter()
            .any(|buff| buff.stat == BuffableStat::ReactionDmgBonus(Reaction::Aggravate))
    );
    assert!(
        thundering_fury
            .four_piece
            .conditional_buffs
            .iter()
            .all(|buff| buff.stat != BuffableStat::AdditiveBonus)
    );

    let viridescent = artifact_set("viridescent_venerer");
    assert!(
        viridescent
            .four_piece
            .conditional_buffs
            .iter()
            .any(|buff| buff.stat == BuffableStat::ReactionDmgBonus(Reaction::Swirl(Element::Pyro)))
    );

    let paradise = artifact_set("flower_of_paradise_lost");
    assert!(
        paradise
            .four_piece
            .conditional_buffs
            .iter()
            .any(|buff| buff.stat == BuffableStat::ReactionDmgBonus(Reaction::LunarBloom))
    );
}

#[test]
fn talent_values_generally_ascending() {
    for c in all_characters() {
        for scaling in c.talents.normal_attack.hits {
            assert!(
                scaling.values[0] <= scaling.values[14],
                "{}: talent {} Lv1 ({}) > Lv15 ({})",
                c.id,
                scaling.name,
                scaling.values[0],
                scaling.values[14]
            );
        }
    }
}

#[test]
fn all_weapons_have_positive_base_atk() {
    for w in ALL_WEAPONS {
        for &atk in &w.base_atk {
            assert!(atk > 0.0, "{}: base_atk non-positive {}", w.id, atk);
        }
    }
}

#[test]
fn all_weapons_have_unique_ids() {
    let mut ids: Vec<&str> = ALL_WEAPONS.iter().map(|w| w.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), ALL_WEAPONS.len());
}

#[test]
fn all_artifact_sets_have_unique_ids() {
    let mut ids: Vec<&str> = ALL_ARTIFACT_SETS.iter().map(|a| a.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), ALL_ARTIFACT_SETS.len());
}

#[test]
fn all_enemies_have_unique_ids() {
    let mut ids: Vec<&str> = ALL_ENEMIES.iter().map(|e| e.id).collect();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), ALL_ENEMIES.len());
}

#[test]
fn test_all_conditional_buff_names_unique() {
    let mut names: Vec<&str> = Vec::new();
    // Check artifacts
    for set in ALL_ARTIFACT_SETS {
        for buff in set.two_piece.conditional_buffs {
            assert!(!names.contains(&buff.name), "Duplicate: {}", buff.name);
            names.push(buff.name);
        }
        for buff in set.four_piece.conditional_buffs {
            assert!(!names.contains(&buff.name), "Duplicate: {}", buff.name);
            names.push(buff.name);
        }
    }
    // Check weapons
    for weapon in ALL_WEAPONS {
        if let Some(passive) = &weapon.passive {
            for buff in passive.effect.conditional_buffs {
                assert!(!names.contains(&buff.name), "Duplicate: {}", buff.name);
                names.push(buff.name);
            }
        }
    }
}

#[test]
fn test_all_stacks_max_positive() {
    let check_buffs = |buffs: &[ConditionalBuff]| {
        for buff in buffs {
            match &buff.activation {
                Activation::Manual(ManualCondition::Stacks(max)) => {
                    assert!(*max > 0, "Stacks max must be > 0 for {}", buff.name);
                }
                Activation::Both(_, ManualCondition::Stacks(max)) => {
                    assert!(*max > 0, "Stacks max must be > 0 for {}", buff.name);
                }
                _ => {}
            }
        }
    };
    for set in ALL_ARTIFACT_SETS {
        check_buffs(set.two_piece.conditional_buffs);
        check_buffs(set.four_piece.conditional_buffs);
    }
    for weapon in ALL_WEAPONS {
        if let Some(passive) = &weapon.passive {
            check_buffs(passive.effect.conditional_buffs);
        }
    }
}
