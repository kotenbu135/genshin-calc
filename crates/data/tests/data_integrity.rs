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

fn stack_values_for(set: &genshin_calc_data::types::ArtifactSet, stat: BuffableStat) -> Vec<f64> {
    let matching_buffs: Vec<&ConditionalBuff> = set
        .four_piece
        .conditional_buffs
        .iter()
        .filter(|buff| buff.stat == stat)
        .collect();

    assert_eq!(
        matching_buffs.len(),
        1,
        "{}: expected exactly one stacked conditional buff for {:?}",
        set.id,
        stat
    );

    matching_buffs[0]
        .stack_values
        .unwrap_or_else(|| panic!("{}: missing stack_values for {:?}", set.id, stat))
        .to_vec()
}

fn reaction_bonus_entries(
    set: &genshin_calc_data::types::ArtifactSet,
    condition: ManualCondition,
) -> Vec<(Reaction, f64)> {
    set.four_piece
        .conditional_buffs
        .iter()
        .filter_map(|buff| match (buff.stat, &buff.activation) {
            (BuffableStat::ReactionDmgBonus(reaction), Activation::Manual(actual_condition))
                if *actual_condition == condition =>
            {
                Some((reaction, buff.value))
            }
            _ => None,
        })
        .collect()
}

fn assert_reaction_bonus_entries(
    actual: &[(Reaction, f64)],
    expected: &[(Reaction, f64)],
    label: &str,
) {
    assert_eq!(
        actual.len(),
        expected.len(),
        "{label}: reaction bonus entry count mismatch; actual entries: {actual:?}"
    );

    for entry in expected {
        assert!(
            actual.contains(entry),
            "{label}: missing reaction bonus entry {entry:?}; actual entries: {actual:?}"
        );
    }
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
        stack_values_for(set, BuffableStat::AtkPercent),
        vec![0.07, 0.16, 0.25]
    );
    assert_eq!(
        stack_values_for(set, BuffableStat::ElementalDmgBonus(Element::Hydro)),
        vec![0.04, 0.09, 0.15]
    );
}

#[test]
fn artifact_audit_scroll_of_cinder_city_two_piece_has_no_damage_relevant_em_bonus() {
    let set = artifact_set("scroll_of_the_hero_of_cinder_city");

    assert!(
        set.two_piece.buffs.is_empty(),
        "Scroll 2pc is energy-only in the mirror and must not grant represented damage stats"
    );
    assert!(
        set.two_piece.conditional_buffs.is_empty(),
        "Scroll 2pc is energy-only in the mirror and must not grant conditional represented damage stats"
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
    assert_reaction_bonus_entries(
        &reaction_bonus_entries(thundering_fury, ManualCondition::Toggle),
        &[
            (Reaction::Overloaded, 0.40),
            (Reaction::ElectroCharged, 0.40),
            (Reaction::Superconduct, 0.40),
            (Reaction::Hyperbloom, 0.40),
            (Reaction::Aggravate, 0.20),
            (Reaction::LunarElectroCharged, 0.20),
        ],
        "Thundering Fury 4pc base reactions",
    );
    assert!(
        thundering_fury
            .four_piece
            .conditional_buffs
            .iter()
            .all(|buff| {
                buff.stat != BuffableStat::TransformativeBonus
                    && buff.stat != BuffableStat::AdditiveBonus
            }),
        "Thundering Fury 4pc must use reaction-specific bonuses, not generic TransformativeBonus or AdditiveBonus"
    );

    let viridescent = artifact_set("viridescent_venerer");
    assert_reaction_bonus_entries(
        &reaction_bonus_entries(viridescent, ManualCondition::Toggle),
        &[
            (Reaction::Swirl(Element::Pyro), 0.60),
            (Reaction::Swirl(Element::Hydro), 0.60),
            (Reaction::Swirl(Element::Electro), 0.60),
            (Reaction::Swirl(Element::Cryo), 0.60),
        ],
        "Viridescent Venerer 4pc Swirl reactions",
    );
    assert!(
        viridescent
            .four_piece
            .conditional_buffs
            .iter()
            .all(|buff| buff.stat != BuffableStat::TransformativeBonus),
        "Viridescent Venerer 4pc must use reaction-specific Swirl bonuses, not generic TransformativeBonus"
    );

    let paradise = artifact_set("flower_of_paradise_lost");
    assert_reaction_bonus_entries(
        &reaction_bonus_entries(paradise, ManualCondition::Toggle),
        &[
            (Reaction::Bloom, 0.40),
            (Reaction::Hyperbloom, 0.40),
            (Reaction::Burgeon, 0.40),
            (Reaction::LunarBloom, 0.10),
        ],
        "Flower of Paradise Lost 4pc base reactions",
    );
    assert_reaction_bonus_entries(
        &reaction_bonus_entries(paradise, ManualCondition::Stacks(4)),
        &[
            (Reaction::Bloom, 0.25),
            (Reaction::Hyperbloom, 0.25),
            (Reaction::Burgeon, 0.25),
            (Reaction::LunarBloom, 0.25),
        ],
        "Flower of Paradise Lost 4pc stack reactions",
    );
    assert!(
        paradise
            .four_piece
            .conditional_buffs
            .iter()
            .all(|buff| buff.stat != BuffableStat::TransformativeBonus),
        "Flower of Paradise Lost 4pc must use reaction-specific bonuses, not generic TransformativeBonus"
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
