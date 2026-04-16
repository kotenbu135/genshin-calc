//! Issue #139: DamagePipeline field on TalentScaling
//!
//! Verifies that scalings classified as direct-lunar damage carry the correct
//! `damage_pipeline: DirectLunar(reaction)` marker so frontends can route them
//! through `calculate_direct_lunar` instead of `calculate_damage`.

use genshin_calc_core::{DamageType, Reaction};
use genshin_calc_data::find_character;
use genshin_calc_data::types::{CharacterData, DamagePipeline, TalentScaling};

fn get_skill_scaling<'a>(ch: &'a CharacterData, name: &str) -> &'a TalentScaling {
    ch.talents
        .elemental_skill
        .scalings
        .iter()
        .find(|s| s.name == name)
        .unwrap_or_else(|| panic!("skill scaling '{name}' not found for {}", ch.id))
}

fn get_burst_scaling<'a>(ch: &'a CharacterData, name: &str) -> &'a TalentScaling {
    ch.talents
        .elemental_burst
        .scalings
        .iter()
        .find(|s| s.name == name)
        .unwrap_or_else(|| panic!("burst scaling '{name}' not found for {}", ch.id))
}

fn get_charged_scaling<'a>(ch: &'a CharacterData, name: &str) -> &'a TalentScaling {
    ch.talents
        .normal_attack
        .charged
        .iter()
        .find(|s| s.name == name)
        .unwrap_or_else(|| panic!("charged scaling '{name}' not found for {}", ch.id))
}

#[test]
fn columbina_moondew_cleanse_is_direct_lunar_bloom() {
    let ch = find_character("columbina").unwrap();
    let s = get_charged_scaling(ch, "Moondew Cleanse DMG");
    assert_eq!(
        s.damage_pipeline,
        DamagePipeline::DirectLunar(Reaction::LunarBloom)
    );
}

#[test]
fn columbina_gravity_interference_scalings_are_direct_lunar() {
    let ch = find_character("columbina").unwrap();
    assert_eq!(
        get_skill_scaling(ch, "Gravity Interference: Lunar-Charged DMG").damage_pipeline,
        DamagePipeline::DirectLunar(Reaction::LunarElectroCharged)
    );
    assert_eq!(
        get_skill_scaling(ch, "Gravity Interference: Lunar-Bloom DMG").damage_pipeline,
        DamagePipeline::DirectLunar(Reaction::LunarBloom)
    );
    assert_eq!(
        get_skill_scaling(ch, "Gravity Interference: Lunar-Crystallize DMG").damage_pipeline,
        DamagePipeline::DirectLunar(Reaction::LunarCrystallize)
    );
}

#[test]
fn flins_burst_lunar_hits_are_direct_lunar_electro_charged() {
    let ch = find_character("flins").unwrap();
    assert_eq!(
        get_burst_scaling(ch, "中盤月感電ダメージ").damage_pipeline,
        DamagePipeline::DirectLunar(Reaction::LunarElectroCharged)
    );
    assert_eq!(
        get_burst_scaling(ch, "終盤月感電ダメージ").damage_pipeline,
        DamagePipeline::DirectLunar(Reaction::LunarElectroCharged)
    );
}

#[test]
fn lauma_skill_hold_hit2_is_direct_lunar_bloom() {
    let ch = find_character("lauma").unwrap();
    assert_eq!(
        get_skill_scaling(ch, "月開花ダメージ").damage_pipeline,
        DamagePipeline::DirectLunar(Reaction::LunarBloom)
    );
}

#[test]
fn zibai_lunar_crystallize_scalings_are_direct_lunar() {
    let ch = find_character("zibai").unwrap();
    assert_eq!(
        get_skill_scaling(ch, "月相転移4段追加ダメージ").damage_pipeline,
        DamagePipeline::DirectLunar(Reaction::LunarCrystallize)
    );
    assert_eq!(
        get_skill_scaling(ch, "霊駿突進2段ダメージ").damage_pipeline,
        DamagePipeline::DirectLunar(Reaction::LunarCrystallize)
    );
}

#[test]
fn standard_talent_damage_keeps_standard_pipeline() {
    // Spot-check representative talent scalings that should remain Standard.
    let hu_tao = find_character("hu_tao").unwrap();
    for s in hu_tao.talents.normal_attack.hits {
        assert_eq!(
            s.damage_pipeline,
            DamagePipeline::Standard,
            "Hu Tao normal hit '{}' should be Standard",
            s.name
        );
    }

    // Columbina's non-lunar skill damage should stay Standard too.
    let col = find_character("columbina").unwrap();
    assert_eq!(
        get_skill_scaling(col, "Skill DMG").damage_pipeline,
        DamagePipeline::Standard
    );
    assert_eq!(
        get_skill_scaling(col, "Gravity Ripple Continuous DMG").damage_pipeline,
        DamagePipeline::Standard
    );
}

#[test]
fn talent_scaling_by_index_exposes_pipeline_field() {
    // The public helper that frontends call must expose the new field.
    let col = find_character("columbina").unwrap();
    let (idx, _) = col
        .talents
        .elemental_skill
        .scalings
        .iter()
        .enumerate()
        .find(|(_, s)| s.name == "Gravity Interference: Lunar-Charged DMG")
        .expect("scaling must exist");
    let scaling = col
        .talent_scaling(DamageType::Skill, idx)
        .expect("scaling must exist");
    assert!(matches!(
        scaling.damage_pipeline,
        DamagePipeline::DirectLunar(Reaction::LunarElectroCharged)
    ));
}
