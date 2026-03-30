use genshin_calc_data::artifacts::ALL_ARTIFACT_SETS;
use genshin_calc_data::buff::{Activation, ConditionalBuff, ManualCondition};
use genshin_calc_data::characters::all_characters;
use genshin_calc_data::enemies::ALL_ENEMIES;
use genshin_calc_data::weapons::ALL_WEAPONS;

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
        assert!(c.base_hp[0] < c.base_hp[1], "{}: HP not ascending", c.id);
        assert!(c.base_hp[1] <= c.base_hp[2], "{}: HP Lv80 > Lv80+", c.id);
        assert!(c.base_hp[2] < c.base_hp[3], "{}: HP Lv80+ > Lv90", c.id);
        assert!(c.base_atk[0] < c.base_atk[1], "{}: ATK not ascending", c.id);
        assert!(c.base_atk[1] <= c.base_atk[2], "{}: ATK Lv80 > Lv80+", c.id);
        assert!(c.base_atk[2] < c.base_atk[3], "{}: ATK Lv80+ > Lv90", c.id);
        assert!(c.base_def[0] < c.base_def[1], "{}: DEF not ascending", c.id);
        assert!(c.base_def[1] <= c.base_def[2], "{}: DEF Lv80 > Lv80+", c.id);
        assert!(c.base_def[2] < c.base_def[3], "{}: DEF Lv80+ > Lv90", c.id);
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
