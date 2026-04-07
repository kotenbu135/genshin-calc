use crate::buff::TalentConditionalBuff;
pub use crate::buff::{Activation, AutoCondition, ManualCondition};
use genshin_calc_core::{BuffTarget, BuffableStat, Element, ScalingStat};
use serde::{Deserialize, Serialize};

pub mod anemo;
pub mod cryo;
pub mod dendro;
pub mod electro;
pub mod geo;
pub mod hydro;
pub mod pyro;

/// Source of a talent buff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TalentBuffSource {
    /// Ascension passive. `1` = A1 (level >= 40), `4` = A4 (level >= 70).
    AscensionPassive(u8),
    /// Elemental skill.
    ElementalSkill,
    /// Elemental burst.
    ElementalBurst,
    /// Constellation effect.
    Constellation(u8),
}

/// Definition of a talent buff for a character.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentBuffDef {
    /// Buff name.
    pub name: &'static str,
    /// Description.
    pub description: &'static str,
    /// Stat being buffed.
    pub stat: BuffableStat,
    /// Fixed buff value (used when scales_with_talent is false).
    pub base_value: f64,
    /// Whether the buff scales with talent level.
    pub scales_with_talent: bool,
    /// Talent level scaling [Lv1..Lv15]. None if fixed.
    pub talent_scaling: Option<&'static [f64; 15]>,
    /// Base stat the scaling multiplier applies to.
    /// E.g. Bennett burst = Some(Atk) means final_value = provider_base_atk * scaling.
    /// None means the scaling value is used directly.
    pub scales_on: Option<ScalingStat>,
    /// Who receives the buff.
    pub target: BuffTarget,
    /// Buff source.
    pub source: TalentBuffSource,
    /// Minimum constellation required (0 = none).
    pub min_constellation: u8,
    /// Optional cap for scaled value. E.g. Lauma A4 cap at 0.32.
    pub cap: Option<f64>,
    /// How this buff is activated. None = always active (passive).
    pub activation: Option<Activation>,
}

/// Finds talent buff definitions for a character by ID.
///
/// Returns `None` for characters without defined talent buffs.
pub fn find_talent_buffs(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    pyro::find(character_id)
        .or_else(|| hydro::find(character_id))
        .or_else(|| electro::find(character_id))
        .or_else(|| cryo::find(character_id))
        .or_else(|| dendro::find(character_id))
        .or_else(|| anemo::find(character_id))
        .or_else(|| geo::find(character_id))
}

/// Returns conditional talent buffs for a character, resolved at the given talent levels.
///
/// Filters to `activation: Some(...)` entries that meet the constellation requirement,
/// and resolves the value based on talent level scaling.
/// Returns an empty `Vec` if the character has no conditional talent buffs.
pub fn get_talent_conditional_buffs(
    character_id: &str,
    constellation: u8,
    talent_levels: &[u8; 3],
) -> Vec<TalentConditionalBuff> {
    let Some(defs) = find_talent_buffs(character_id) else {
        return Vec::new();
    };
    defs.iter()
        .filter(|def| def.activation.is_some() && constellation >= def.min_constellation)
        .map(|def| {
            let value = resolve_scaling_value(def, talent_levels);
            TalentConditionalBuff {
                name: def.name,
                description: def.description,
                stat: def.stat,
                value,
                target: def.target,
                activation: def.activation.clone().unwrap(),
                scales_on: def.scales_on,
            }
        })
        .collect()
}

/// Resolves the buff value based on talent level scaling.
fn resolve_scaling_value(def: &TalentBuffDef, talent_levels: &[u8; 3]) -> f64 {
    if let Some(scaling) = def.talent_scaling {
        let talent_level = match def.source {
            TalentBuffSource::ElementalSkill => talent_levels[1],
            TalentBuffSource::ElementalBurst => talent_levels[2],
            _ if def.scales_with_talent => talent_levels[2],
            _ => return def.base_value,
        };
        let idx = (talent_level as usize).saturating_sub(1).min(14);
        scaling[idx]
    } else {
        def.base_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_bennett_buffs() {
        let buffs = find_talent_buffs("bennett").unwrap();
        assert_eq!(buffs.len(), 3);
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
        assert_eq!(buffs[0].target, BuffTarget::Team);
        // C1 Grand Expectation
        assert_eq!(buffs[1].stat, BuffableStat::AtkFlat);
        assert!((buffs[1].base_value - 0.20).abs() < 1e-6);
        assert_eq!(buffs[1].min_constellation, 1);
        // C6
        assert_eq!(
            buffs[2].stat,
            BuffableStat::ElementalDmgBonus(Element::Pyro)
        );
        assert!((buffs[2].base_value - 0.15).abs() < 1e-6);
        assert_eq!(buffs[2].min_constellation, 6);
    }

    #[test]
    fn test_find_rosaria_buffs() {
        let buffs = find_talent_buffs("rosaria").unwrap();
        assert_eq!(buffs[0].target, BuffTarget::TeamExcludeSelf);
    }

    #[test]
    fn test_find_nonexistent_character() {
        assert!(find_talent_buffs("nonexistent_character_xyz").is_none());
        assert!(find_talent_buffs("unknown").is_none());
    }

    #[test]
    fn test_find_sara_buffs_by_character_id() {
        // Sara's CharacterData uses id "kujou_sara", talent_buffs must match
        let buffs = find_talent_buffs("kujou_sara").unwrap();
        assert_eq!(buffs.len(), 2);
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
    }

    #[test]
    fn test_bennett_burst_scaling_lv13() {
        let buffs = find_talent_buffs("bennett").unwrap();
        let scaling = buffs[0].talent_scaling.unwrap();
        // Lv13 = index 12 = 1.19
        assert!((scaling[12] - 1.19).abs() < 1e-6);
    }

    #[test]
    fn test_all_talent_buffs_have_unique_ids() {
        let all_ids: Vec<&str> = pyro::PYRO_TALENT_BUFFS
            .iter()
            .chain(hydro::HYDRO_TALENT_BUFFS.iter())
            .chain(electro::ELECTRO_TALENT_BUFFS.iter())
            .chain(cryo::CRYO_TALENT_BUFFS.iter())
            .chain(dendro::DENDRO_TALENT_BUFFS.iter())
            .chain(anemo::ANEMO_TALENT_BUFFS.iter())
            .chain(geo::GEO_TALENT_BUFFS.iter())
            .map(|(id, _)| *id)
            .collect();
        let mut sorted = all_ids.clone();
        sorted.sort();
        sorted.dedup();
        assert_eq!(
            sorted.len(),
            all_ids.len(),
            "Duplicate talent buff IDs found"
        );
    }

    #[test]
    fn test_find_sucrose_buffs() {
        let buffs = find_talent_buffs("sucrose").unwrap();
        // A1 (EM+50) — at least 1 fixed entry
        assert!(buffs.iter().any(
            |b| b.stat == BuffableStat::ElementalMastery && (b.base_value - 50.0).abs() < 1e-6
        ));
    }

    #[test]
    fn test_find_ganyu_buffs() {
        let buffs = find_talent_buffs("ganyu").unwrap();
        assert_eq!(buffs.len(), 3);
        // A4 buff
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalDmgBonus(Element::Cryo)
        );
        assert!((buffs[0].base_value - 0.20).abs() < 1e-6);
        assert_eq!(buffs[0].target, BuffTarget::Team);
        // C4 buff
        assert_eq!(buffs[1].stat, BuffableStat::DmgBonus);
        assert!((buffs[1].base_value - 0.25).abs() < 1e-6);
        assert_eq!(buffs[1].min_constellation, 4);
    }

    #[test]
    fn test_find_albedo_buffs() {
        let buffs = find_talent_buffs("albedo").unwrap();
        assert_eq!(buffs.len(), 4);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((buffs[0].base_value - 125.0).abs() < 1e-6);
    }

    #[test]
    fn test_find_ningguang_buffs() {
        let buffs = find_talent_buffs("ningguang").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalDmgBonus(Element::Geo));
        assert!((buffs[0].base_value - 0.12).abs() < 1e-6);
    }

    #[test]
    fn test_find_traveler_dendro_buffs() {
        let buffs = find_talent_buffs("traveler_dendro").unwrap();
        assert_eq!(buffs.len(), 2);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((buffs[0].base_value - 60.0).abs() < 1e-6);
    }

    #[test]
    fn test_find_aino_buffs() {
        let buffs = find_talent_buffs("aino").unwrap();
        // C1 EM + A4 BurstFlatDmg (C6 moved to MoonsignTalentEnhancement)
        assert_eq!(buffs.len(), 2);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((buffs[0].base_value - 80.0).abs() < 1e-6);
        assert_eq!(buffs[0].min_constellation, 1);
    }

    #[test]
    fn test_aino_a4_burst_dmg_from_em() {
        let buffs = find_talent_buffs("aino").unwrap();
        let a4 = buffs
            .iter()
            .find(|b| b.source == TalentBuffSource::AscensionPassive(4))
            .unwrap();
        assert_eq!(a4.stat, BuffableStat::BurstFlatDmg);
        assert!((a4.base_value - 0.50).abs() < 1e-6);
        assert_eq!(a4.scales_on, Some(ScalingStat::Em));
        assert_eq!(a4.target, BuffTarget::OnlySelf);
    }

    #[test]
    fn test_aino_c6_moved_to_moonsign() {
        // C6 reaction DMG bonus is now Moonsign-level-dependent,
        // defined in moonsign_chars.rs, not talent_buffs.
        let buffs = find_talent_buffs("aino").unwrap();
        assert!(
            buffs
                .iter()
                .all(|b| b.source != TalentBuffSource::Constellation(6)),
            "C6 should not be in talent_buffs (moved to MoonsignTalentEnhancement)"
        );
    }

    #[test]
    fn test_find_yoimiya_buffs() {
        let buffs = find_talent_buffs("yoimiya").unwrap();
        assert_eq!(buffs.len(), 3);
        assert_eq!(buffs[0].stat, BuffableStat::AtkPercent);
        assert!((buffs[0].base_value - 0.20).abs() < 1e-6);
        assert_eq!(buffs[0].target, BuffTarget::TeamExcludeSelf);
    }

    #[test]
    fn test_find_chevreuse_buffs() {
        let buffs = find_talent_buffs("chevreuse").unwrap();
        assert_eq!(buffs.len(), 5);
        assert_eq!(buffs[0].stat, BuffableStat::AtkPercent);
        assert!((buffs[0].base_value - 0.20).abs() < 1e-6);
    }

    #[test]
    fn test_find_diona_buffs() {
        let buffs = find_talent_buffs("diona").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((buffs[0].base_value - 200.0).abs() < 1e-6);
        assert_eq!(buffs[0].min_constellation, 6);
    }

    #[test]
    fn test_find_amber_buffs() {
        let buffs = find_talent_buffs("amber").unwrap();
        assert_eq!(buffs[0].stat, BuffableStat::AtkPercent);
        assert_eq!(buffs[0].min_constellation, 6);
    }

    #[test]
    fn test_find_barbara_buffs() {
        let buffs = find_talent_buffs("barbara").unwrap();
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalDmgBonus(Element::Hydro)
        );
        assert_eq!(buffs[0].min_constellation, 2);
    }

    #[test]
    fn test_find_sara_c6_buff() {
        let buffs = find_talent_buffs("kujou_sara").unwrap();
        // Original skill buff + C6 CritDmg
        let c6 = buffs
            .iter()
            .find(|b| b.stat == BuffableStat::CritDmg)
            .unwrap();
        assert!((c6.base_value - 0.60).abs() < 1e-6);
        assert_eq!(c6.min_constellation, 6);
    }

    #[test]
    fn test_shenhe_a1_press_buffs() {
        let buffs = find_talent_buffs("shenhe").unwrap();
        let skill_dmg = buffs.iter().find(|b| b.stat == BuffableStat::SkillDmgBonus);
        let burst_dmg = buffs.iter().find(|b| b.stat == BuffableStat::BurstDmgBonus);
        assert!(
            skill_dmg.is_some(),
            "Should have SkillDmgBonus from A1 press"
        );
        assert!(
            burst_dmg.is_some(),
            "Should have BurstDmgBonus from A1 press"
        );
        assert!((skill_dmg.unwrap().base_value - 0.15).abs() < 1e-6);
        assert!((burst_dmg.unwrap().base_value - 0.15).abs() < 1e-6);
    }

    #[test]
    fn test_shenhe_skill_flat_dmg_entries() {
        let buffs = find_talent_buffs("shenhe").unwrap();
        // 5 FlatDmg entries + 2 A4 press entries + 3 A4 hold entries + C2 = 11 total
        assert_eq!(buffs.len(), 11);
        assert!(
            buffs.iter().all(|b| b.stat != BuffableStat::AtkFlat),
            "Shenhe should have no AtkFlat entry"
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::NormalAtkFlatDmg)
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::ChargedAtkFlatDmg)
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::PlungingAtkFlatDmg)
        );
        assert!(buffs.iter().any(|b| b.stat == BuffableStat::SkillFlatDmg));
        assert!(buffs.iter().any(|b| b.stat == BuffableStat::BurstFlatDmg));
        // All 5 FlatDmg entries should be ElementalSkill source with ATK scaling
        for b in buffs.iter().filter(|b| b.name.contains("Flat DMG")) {
            assert_eq!(b.source, TalentBuffSource::ElementalSkill);
            assert_eq!(b.scales_on, Some(ScalingStat::Atk));
            assert!(b.scales_with_talent);
        }
    }

    #[test]
    fn test_yun_jin_normal_atk_flat_dmg() {
        let buffs = find_talent_buffs("yun_jin").unwrap();
        assert_eq!(buffs.len(), 3);
        assert_eq!(buffs[0].stat, BuffableStat::NormalAtkFlatDmg);
        assert!(
            buffs.iter().all(|b| b.stat != BuffableStat::AtkFlat),
            "Yun Jin should have no AtkFlat entry"
        );
        assert_eq!(buffs[0].scales_on, Some(ScalingStat::Def));
    }

    #[test]
    fn test_thoma_c6_buffs() {
        let buffs = find_talent_buffs("thoma").unwrap();
        assert_eq!(buffs.len(), 3);
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::NormalAtkDmgBonus)
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::ChargedAtkDmgBonus)
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::PlungingAtkDmgBonus)
        );
        for b in buffs {
            assert!((b.base_value - 0.15).abs() < 1e-6);
            assert_eq!(b.min_constellation, 6);
        }
    }

    #[test]
    fn test_find_faruzan_buffs() {
        let buffs = find_talent_buffs("faruzan").unwrap();
        assert_eq!(buffs.len(), 3);
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalDmgBonus(Element::Anemo)
        );
        assert!(buffs[0].scales_with_talent);
        assert!(buffs[0].talent_scaling.is_some());
        assert_eq!(buffs[0].source, TalentBuffSource::ElementalBurst);
    }

    #[test]
    fn test_faruzan_burst_scaling_lv13() {
        let buffs = find_talent_buffs("faruzan").unwrap();
        let scaling = buffs[0].talent_scaling.unwrap();
        // Verify Lv13 value is positive
        assert!(scaling[12] > 0.0, "Lv13 scaling should be positive");
    }

    #[test]
    fn test_find_candace_buffs() {
        let buffs = find_talent_buffs("candace").unwrap();
        assert_eq!(buffs.len(), 2);
        assert_eq!(buffs[0].stat, BuffableStat::NormalAtkDmgBonus);
        assert!(buffs[0].scales_with_talent);
        assert_eq!(buffs[0].source, TalentBuffSource::ElementalBurst);
    }

    #[test]
    fn test_find_gorou_buffs() {
        let buffs = find_talent_buffs("gorou").unwrap();
        assert_eq!(buffs.len(), 3);
        // DefFlat scaling entry
        let def_buff = buffs
            .iter()
            .find(|b| b.stat == BuffableStat::DefFlat)
            .unwrap();
        assert!(def_buff.scales_with_talent);
        assert_eq!(def_buff.source, TalentBuffSource::ElementalSkill);
        // Geo DMG fixed entry
        let geo_buff = buffs
            .iter()
            .find(|b| b.stat == BuffableStat::ElementalDmgBonus(Element::Geo))
            .unwrap();
        assert!((geo_buff.base_value - 0.15).abs() < 1e-6);
        assert!(!geo_buff.scales_with_talent);
    }

    #[test]
    fn test_sucrose_a4_builder_pattern() {
        let buffs = find_talent_buffs("sucrose").unwrap();
        assert_eq!(buffs.len(), 2); // A1 + A4
        let a4 = buffs.iter().find(|b| b.name == "Mollis Favonius").unwrap();
        assert_eq!(a4.stat, BuffableStat::ElementalMastery);
        assert!((a4.base_value - 0.20).abs() < 1e-6); // 20% of own EM coefficient
        assert_eq!(a4.scales_on, Some(ScalingStat::Em));
        assert_eq!(a4.target, BuffTarget::TeamExcludeSelf);
    }

    #[test]
    fn test_find_yelan_buffs() {
        let buffs = find_talent_buffs("yelan").unwrap();
        assert_eq!(buffs.len(), 2);
        assert_eq!(buffs[0].stat, BuffableStat::DmgBonus);
        assert!((buffs[0].base_value - 0.50).abs() < 1e-6); // max value 0.50
    }

    #[test]
    fn test_find_ineffa_talent_buffs() {
        let buffs = find_talent_buffs("ineffa").unwrap();
        assert_eq!(buffs.len(), 2);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((buffs[0].base_value - 0.06).abs() < 1e-6); // 6% of total ATK coefficient
        assert_eq!(buffs[0].scales_on, Some(ScalingStat::TotalAtk));
    }

    #[test]
    fn test_find_jahoda_buffs() {
        let buffs = find_talent_buffs("jahoda").unwrap();
        assert_eq!(buffs.len(), 3);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((buffs[0].base_value - 100.0).abs() < 1e-6);
    }

    #[test]
    fn test_find_nilou_buffs() {
        let buffs = find_talent_buffs("nilou").unwrap();
        assert_eq!(buffs.len(), 6);
        assert_eq!(buffs[0].stat, BuffableStat::TransformativeBonus);
        assert!((buffs[0].base_value - 0.0).abs() < 1e-6); // scaling_value==0 triggers special formula
        assert!(!buffs[0].scales_with_talent);
        assert!(buffs[0].talent_scaling.is_none());
        assert_eq!(buffs[0].scales_on, Some(ScalingStat::Hp));
        assert_eq!(buffs[0].cap, Some(4.0));
        assert_eq!(buffs[0].target, BuffTarget::OnlySelf);
        assert_eq!(buffs[0].source, TalentBuffSource::AscensionPassive(4));
        assert_eq!(buffs[0].min_constellation, 0);
    }

    #[test]
    fn test_find_lisa_debuffs() {
        let buffs = find_talent_buffs("lisa").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::DefReduction);
        assert!((buffs[0].base_value - 0.15).abs() < 1e-6);
    }

    #[test]
    fn test_find_chevreuse_debuffs() {
        let buffs = find_talent_buffs("chevreuse").unwrap();
        assert_eq!(buffs.len(), 5); // ATK + 2 res shreds + 2 C6 DMG bonus
        let pyro_shred = buffs
            .iter()
            .find(|b| b.stat == BuffableStat::ElementalResReduction(Element::Pyro));
        assert!(pyro_shred.is_some());
        assert!((pyro_shred.unwrap().base_value - 0.40).abs() < 1e-6);
        let electro_shred = buffs
            .iter()
            .find(|b| b.stat == BuffableStat::ElementalResReduction(Element::Electro));
        assert!(electro_shred.is_some());
        assert!((electro_shred.unwrap().base_value - 0.40).abs() < 1e-6);
    }

    #[test]
    fn test_find_faruzan_res_shred() {
        let buffs = find_talent_buffs("faruzan").unwrap();
        assert_eq!(buffs.len(), 3); // existing Anemo DMG + new Anemo RES shred + C6 CritDmg
        let shred = buffs
            .iter()
            .find(|b| b.stat == BuffableStat::ElementalResReduction(Element::Anemo));
        assert!(shred.is_some());
        assert!((shred.unwrap().base_value - 0.30).abs() < 1e-6);
        assert!(!shred.unwrap().scales_with_talent);
    }

    #[test]
    fn test_find_raiden_shogun_buffs() {
        let buffs = find_talent_buffs("raiden_shogun").unwrap();
        assert_eq!(buffs.len(), 3);
        assert_eq!(buffs[0].stat, BuffableStat::BurstDmgBonus);
        assert!(buffs[0].scales_with_talent);
        assert_eq!(buffs[0].source, TalentBuffSource::ElementalSkill);
        // C2 DEF Ignore
        assert_eq!(buffs[1].stat, BuffableStat::DefIgnore);
        assert!((buffs[1].base_value - 0.60).abs() < 1e-6);
        assert_eq!(buffs[1].min_constellation, 2);
        // C4 ATK%
        assert_eq!(buffs[2].stat, BuffableStat::AtkPercent);
        assert!((buffs[2].base_value - 0.30).abs() < 1e-6);
        assert_eq!(buffs[2].min_constellation, 4);
    }

    #[test]
    fn test_find_xilonen_buffs() {
        let buffs = find_talent_buffs("xilonen").unwrap();
        assert_eq!(buffs.len(), 8);
        // Skill: Geo RES reduction (talent-scaled)
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalResReduction(Element::Geo)
        );
        assert!(buffs[0].scales_with_talent);
        // C2 buffs: Geo DMG, Pyro ATK, Hydro HP, Cryo Crit DMG
        assert_eq!(buffs[1].stat, BuffableStat::ElementalDmgBonus(Element::Geo));
        assert_eq!(buffs[1].min_constellation, 2);
        assert_eq!(buffs[2].stat, BuffableStat::AtkPercent);
        assert_eq!(buffs[2].min_constellation, 2);
        assert_eq!(buffs[3].stat, BuffableStat::HpPercent);
        assert_eq!(buffs[3].min_constellation, 2);
        assert_eq!(buffs[4].stat, BuffableStat::CritDmg);
        assert_eq!(buffs[4].min_constellation, 2);
        // C4: flat DMG from DEF for Normal/Charged/Plunging
        assert_eq!(buffs[5].stat, BuffableStat::NormalAtkFlatDmg);
        assert!((buffs[5].base_value - 0.65).abs() < 1e-6);
        assert_eq!(buffs[5].scales_on, Some(ScalingStat::Def));
        assert_eq!(buffs[5].min_constellation, 4);
        assert_eq!(buffs[6].stat, BuffableStat::ChargedAtkFlatDmg);
        assert_eq!(buffs[7].stat, BuffableStat::PlungingAtkFlatDmg);
        for b in &buffs[5..] {
            assert!((b.base_value - 0.65).abs() < 1e-6);
            assert_eq!(b.scales_on, Some(ScalingStat::Def));
            assert_eq!(b.min_constellation, 4);
        }
    }

    #[test]
    fn test_find_citlali_buffs() {
        let buffs = find_talent_buffs("citlali").unwrap();
        assert_eq!(buffs.len(), 7);
        // Skill: Pyro + Hydro RES shred
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalResReduction(Element::Pyro)
        );
        assert!((buffs[0].base_value - 0.20).abs() < 1e-6);
        assert_eq!(
            buffs[1].stat,
            BuffableStat::ElementalResReduction(Element::Hydro)
        );
        // C2: flat EM+250 for party (excl. Citlali)
        assert_eq!(buffs[2].stat, BuffableStat::ElementalMastery);
        assert!((buffs[2].base_value - 250.0).abs() < 1e-6);
        assert_eq!(buffs[2].scales_on, None);
        assert_eq!(buffs[2].cap, None);
        assert_eq!(buffs[2].target, BuffTarget::TeamExcludeSelf);
        assert_eq!(buffs[2].min_constellation, 2);
        assert_eq!(buffs[2].source, TalentBuffSource::Constellation(2));
        // C2: additional RES shred
        assert_eq!(buffs[3].min_constellation, 2);
        assert_eq!(buffs[4].min_constellation, 2);
        // C6: Pyro/Hydro DMG +1.5%/stack (max 40)
        assert_eq!(
            buffs[5].stat,
            BuffableStat::ElementalDmgBonus(Element::Pyro)
        );
        assert!((buffs[5].base_value - 0.015).abs() < 1e-6);
        assert_eq!(buffs[5].min_constellation, 6);
        assert_eq!(buffs[5].target, BuffTarget::Team);
        assert_eq!(
            buffs[5].activation,
            Some(Activation::Manual(ManualCondition::Stacks(40)))
        );
        assert_eq!(
            buffs[6].stat,
            BuffableStat::ElementalDmgBonus(Element::Hydro)
        );
        assert!((buffs[6].base_value - 0.015).abs() < 1e-6);
        assert_eq!(buffs[6].min_constellation, 6);
        assert_eq!(
            buffs[6].activation,
            Some(Activation::Manual(ManualCondition::Stacks(40)))
        );
    }

    #[test]
    fn test_find_eula_buffs() {
        let buffs = find_talent_buffs("eula").unwrap();
        assert_eq!(buffs.len(), 4);
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalResReduction(Element::Cryo)
        );
        assert!((buffs[0].base_value - 0.25).abs() < 1e-6);
        assert_eq!(buffs[1].stat, BuffableStat::PhysicalResReduction);
        assert!((buffs[1].base_value - 0.25).abs() < 1e-6);
    }

    #[test]
    fn test_find_mavuika_buffs() {
        let buffs = find_talent_buffs("mavuika").unwrap();
        assert_eq!(buffs.len(), 3);
        // C2: DEF Reduction -20%
        assert_eq!(buffs[0].stat, BuffableStat::DefReduction);
        assert!((buffs[0].base_value - 0.20).abs() < 1e-6);
        assert_eq!(buffs[0].source, TalentBuffSource::Constellation(2));
        // A1: Mavuika's own ATK+30% (self-buff)
        assert_eq!(buffs[1].stat, BuffableStat::AtkPercent);
        assert!((buffs[1].base_value - 0.30).abs() < 1e-6);
        assert_eq!(buffs[1].source, TalentBuffSource::AscensionPassive(1));
        assert_eq!(buffs[1].target, BuffTarget::OnlySelf);
        // A4: DMG Bonus +40% (max, self-buff)
        assert_eq!(buffs[2].stat, BuffableStat::DmgBonus);
        assert!((buffs[2].base_value - 0.40).abs() < 1e-6);
        assert_eq!(buffs[2].source, TalentBuffSource::AscensionPassive(4));
        assert_eq!(buffs[2].target, BuffTarget::OnlySelf);
    }

    #[test]
    fn test_find_xiangling_buffs() {
        let buffs = find_talent_buffs("xiangling").unwrap();
        assert_eq!(buffs.len(), 4);
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalResReduction(Element::Pyro)
        );
        assert_eq!(buffs[0].min_constellation, 1);
        assert_eq!(
            buffs[1].stat,
            BuffableStat::ElementalDmgBonus(Element::Pyro)
        );
        assert_eq!(buffs[1].min_constellation, 6);
    }

    #[test]
    fn test_find_venti_buffs() {
        let buffs = find_talent_buffs("venti").unwrap();
        assert_eq!(buffs.len(), 4);
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalResReduction(Element::Anemo)
        );
        assert!((buffs[0].base_value - 0.12).abs() < 1e-6);
        assert_eq!(buffs[1].stat, BuffableStat::PhysicalResReduction);
        assert!((buffs[1].base_value - 0.12).abs() < 1e-6);
    }

    #[test]
    fn test_find_xianyun_buffs() {
        let buffs = find_talent_buffs("xianyun").unwrap();
        assert_eq!(buffs.len(), 5);
        assert_eq!(buffs[0].stat, BuffableStat::PlungingAtkFlatDmg);
        assert!(buffs[0].scales_with_talent);
        assert_eq!(buffs[1].stat, BuffableStat::PlungingAtkDmgBonus);
        assert!((buffs[1].base_value - 0.75).abs() < 1e-6);
        assert_eq!(buffs[2].stat, BuffableStat::CritRate);
        assert!((buffs[2].base_value - 0.20).abs() < 1e-6);
        assert_eq!(buffs[2].min_constellation, 2);
    }

    #[test]
    fn test_find_beidou_buffs() {
        let buffs = find_talent_buffs("beidou").unwrap();
        assert_eq!(buffs.len(), 2);
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalResReduction(Element::Electro)
        );
        assert_eq!(buffs[0].min_constellation, 6);
    }

    #[test]
    fn test_find_klee_buffs() {
        let buffs = find_talent_buffs("klee").unwrap();
        assert_eq!(buffs.len(), 4);
        assert_eq!(buffs[0].stat, BuffableStat::DefReduction);
        assert!((buffs[0].base_value - 0.23).abs() < 1e-6);
        assert_eq!(
            buffs[1].stat,
            BuffableStat::ElementalDmgBonus(Element::Pyro)
        );
    }

    #[test]
    fn test_find_xingqiu_buffs() {
        let buffs = find_talent_buffs("xingqiu").unwrap();
        assert_eq!(buffs.len(), 2);
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalResReduction(Element::Hydro)
        );
        assert_eq!(buffs[0].min_constellation, 2);
    }

    #[test]
    fn test_find_jean_buffs() {
        let buffs = find_talent_buffs("jean").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalResReduction(Element::Anemo)
        );
        assert!((buffs[0].base_value - 0.40).abs() < 1e-6);
        assert_eq!(buffs[0].min_constellation, 4);
    }

    #[test]
    fn test_find_iansan_buffs() {
        let buffs = find_talent_buffs("iansan").unwrap();
        assert_eq!(buffs.len(), 4);
        // Burst: AtkFlat scaling on ATK
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
        assert!(buffs[0].scales_with_talent);
        assert_eq!(buffs[0].scales_on, Some(ScalingStat::Atk));
        // A1: AtkPercent +20% self
        assert_eq!(buffs[1].stat, BuffableStat::AtkPercent);
        assert!((buffs[1].base_value - 0.20).abs() < 1e-6);
        assert_eq!(buffs[1].target, BuffTarget::OnlySelf);
    }

    #[test]
    fn test_find_collei_buffs() {
        let buffs = find_talent_buffs("collei").unwrap();
        assert_eq!(buffs.len(), 2);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((buffs[0].base_value - 60.0).abs() < 1e-6);
        assert_eq!(buffs[0].min_constellation, 4);
    }

    #[test]
    fn test_find_mika_buffs() {
        let buffs = find_talent_buffs("mika").unwrap();
        assert_eq!(buffs.len(), 2);
        assert_eq!(buffs[0].stat, BuffableStat::PhysicalDmgBonus);
        assert!((buffs[0].base_value - 0.10).abs() < 1e-6);
        assert_eq!(buffs[0].min_constellation, 6);
    }

    #[test]
    fn test_find_xinyan_buffs() {
        let buffs = find_talent_buffs("xinyan").unwrap();
        assert_eq!(buffs.len(), 3);
        assert_eq!(buffs[0].stat, BuffableStat::PhysicalResReduction);
        assert_eq!(buffs[0].min_constellation, 4);
    }

    #[test]
    fn test_rosaria_has_c6_phys_res_shred() {
        let buffs = find_talent_buffs("rosaria").unwrap();
        let c6 = buffs
            .iter()
            .find(|b| b.stat == BuffableStat::PhysicalResReduction);
        assert!(c6.is_some(), "Rosaria should have C6 Physical RES shred");
        assert!((c6.unwrap().base_value - 0.20).abs() < 1e-6);
        assert_eq!(c6.unwrap().min_constellation, 6);
    }

    #[test]
    fn test_ganyu_has_c4_dmg_bonus() {
        let buffs = find_talent_buffs("ganyu").unwrap();
        let c4 = buffs.iter().find(|b| b.stat == BuffableStat::DmgBonus);
        assert!(c4.is_some(), "Ganyu should have C4 DmgBonus");
        assert!((c4.unwrap().base_value - 0.25).abs() < 1e-6);
        assert_eq!(c4.unwrap().min_constellation, 4);
    }

    #[test]
    fn test_find_zhongli_debuffs() {
        let buffs = find_talent_buffs("zhongli").unwrap();
        assert_eq!(buffs.len(), 8); // 7 elemental + 1 physical
        for b in buffs {
            assert!((b.base_value - 0.20).abs() < 1e-6);
            assert_eq!(b.target, BuffTarget::Team);
        }
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Pyro))
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Hydro))
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Electro))
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Cryo))
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Dendro))
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Anemo))
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::ElementalResReduction(Element::Geo))
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::PhysicalResReduction)
        );
    }

    // --- get_talent_conditional_buffs tests ---

    #[test]
    fn test_conditional_bennett_c6_returns_all_buffs() {
        let buffs = get_talent_conditional_buffs("bennett", 6, &[6, 8, 10]);
        assert_eq!(buffs.len(), 3);
        // ATK buff from burst, value resolved at burst lv10
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
        assert_eq!(buffs[0].scales_on, Some(ScalingStat::Atk));
        assert_eq!(buffs[0].target, BuffTarget::Team);
        assert!(matches!(
            buffs[0].activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        // C1 Grand Expectation: +20% Base ATK
        assert_eq!(buffs[1].stat, BuffableStat::AtkFlat);
        assert!((buffs[1].value - 0.20).abs() < 1e-6);
        assert_eq!(buffs[1].scales_on, Some(ScalingStat::Atk));
        // C6 pyro DMG bonus
        assert_eq!(
            buffs[2].stat,
            BuffableStat::ElementalDmgBonus(Element::Pyro)
        );
        assert!((buffs[2].value - 0.15).abs() < 1e-6);
    }

    #[test]
    fn test_conditional_bennett_c0_returns_only_atk() {
        let buffs = get_talent_conditional_buffs("bennett", 0, &[1, 1, 1]);
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
    }

    #[test]
    fn test_conditional_bennett_burst_scaling_resolves() {
        let buffs_lv1 = get_talent_conditional_buffs("bennett", 0, &[1, 1, 1]);
        let buffs_lv13 = get_talent_conditional_buffs("bennett", 0, &[1, 1, 13]);
        // Burst scaling should differ at different talent levels
        assert!(
            (buffs_lv13[0].value - buffs_lv1[0].value).abs() > 0.01,
            "Talent level should affect resolved value"
        );
    }

    #[test]
    fn test_conditional_furina_returns_stacks() {
        // C0: only the base 300pt entry; C1+ adds the extra 100pt entry
        let buffs_c0 = get_talent_conditional_buffs("furina", 0, &[1, 1, 10]);
        assert_eq!(buffs_c0.len(), 1);
        let buffs = get_talent_conditional_buffs("furina", 1, &[1, 1, 10]);
        assert_eq!(buffs.len(), 2);
        for b in &buffs {
            assert!(matches!(
                b.activation,
                Activation::Manual(ManualCondition::Stacks(300))
            ));
        }
    }

    #[test]
    fn test_conditional_nonexistent_character() {
        let buffs = get_talent_conditional_buffs("nonexistent_character_xyz", 6, &[10, 10, 10]);
        assert!(buffs.is_empty());
    }

    #[test]
    fn test_conditional_no_conditionals_character() {
        // Diona has talent buffs but all are activation: None
        let buffs = get_talent_conditional_buffs("diona", 6, &[10, 10, 10]);
        assert!(
            buffs.is_empty(),
            "Characters with only unconditional buffs should return empty"
        );
    }

    #[test]
    fn test_conditional_shenhe_stacks() {
        let buffs = get_talent_conditional_buffs("shenhe", 0, &[1, 10, 10]);
        // Shenhe has 5 Stacks(5) FlatDmg entries + A1 press entries
        let stacks_buffs: Vec<_> = buffs
            .iter()
            .filter(|b| matches!(b.activation, Activation::Manual(ManualCondition::Stacks(5))))
            .collect();
        assert_eq!(
            stacks_buffs.len(),
            5,
            "Shenhe should have 5 Stacks(5) FlatDmg entries"
        );
    }

    #[test]
    fn test_conditional_mavuika_toggle() {
        let buffs = get_talent_conditional_buffs("mavuika", 0, &[1, 1, 1]);
        assert_eq!(buffs.len(), 2);
        for b in &buffs {
            assert!(matches!(
                b.activation,
                Activation::Manual(ManualCondition::Toggle)
            ));
        }
    }

    #[test]
    fn test_conditional_chevreuse_c6() {
        let c0 = get_talent_conditional_buffs("chevreuse", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("chevreuse", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "chevreuse_c6_pyro_dmg"));
        assert!(c6.iter().any(|b| b.name == "chevreuse_c6_pyro_dmg"));
        assert!(c6.iter().any(|b| b.name == "chevreuse_c6_electro_dmg"));
    }

    #[test]
    fn test_conditional_klee_c2_c6() {
        let c0 = get_talent_conditional_buffs("klee", 0, &[10, 10, 10]);
        let c2 = get_talent_conditional_buffs("klee", 2, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("klee", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "klee_c2_def_reduction"));
        assert!(c2.iter().any(|b| b.name == "klee_c2_def_reduction"));
        assert!(!c2.iter().any(|b| b.name == "klee_c6_pyro_dmg"));
        assert!(c6.iter().any(|b| b.name == "klee_c6_pyro_dmg"));
    }

    #[test]
    fn test_conditional_xiangling_c1_c6() {
        let c0 = get_talent_conditional_buffs("xiangling", 0, &[10, 10, 10]);
        let c1 = get_talent_conditional_buffs("xiangling", 1, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("xiangling", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "xiangling_c1_pyro_res_shred"));
        assert!(c1.iter().any(|b| b.name == "xiangling_c1_pyro_res_shred"));
        let shred = c1
            .iter()
            .find(|b| b.name == "xiangling_c1_pyro_res_shred")
            .unwrap();
        assert_eq!(
            shred.stat,
            BuffableStat::ElementalResReduction(Element::Pyro)
        );
        assert!((shred.value - 0.15).abs() < 1e-9);
        assert!(c6.iter().any(|b| b.name == "xiangling_c6_pyro_dmg"));
    }

    #[test]
    fn test_conditional_xinyan_c2_c6() {
        let c0 = get_talent_conditional_buffs("xinyan", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("xinyan", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "xinyan_c2_burst_crit"));
        assert!(c6.iter().any(|b| b.name == "xinyan_c2_burst_crit"));
        assert!(c6.iter().any(|b| b.name == "xinyan_c6_charged_def_scaling"));
        let def_scaling = c6
            .iter()
            .find(|b| b.name == "xinyan_c6_charged_def_scaling")
            .unwrap();
        assert_eq!(def_scaling.scales_on, Some(ScalingStat::Def));
    }

    #[test]
    fn test_conditional_yoimiya_c1_c2() {
        let c0 = get_talent_conditional_buffs("yoimiya", 0, &[10, 10, 10]);
        let c2 = get_talent_conditional_buffs("yoimiya", 2, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "yoimiya_c1_atk"));
        assert!(c2.iter().any(|b| b.name == "yoimiya_c1_atk"));
        assert!(c2.iter().any(|b| b.name == "yoimiya_c2_pyro_dmg"));
    }

    #[test]
    fn test_conditional_durin_c4_c6() {
        let c0 = get_talent_conditional_buffs("durin", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("durin", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "durin_c4_burst_dmg"));
        assert!(c6.iter().any(|b| b.name == "durin_c4_burst_dmg"));
        assert!(c6.iter().any(|b| b.name == "durin_c6_def_ignore"));
        assert!(c6.iter().any(|b| b.name == "durin_c6_def_reduction"));
    }

    #[test]
    fn test_conditional_candace_c2_hp() {
        let c0 = get_talent_conditional_buffs("candace", 0, &[10, 10, 10]);
        let c2 = get_talent_conditional_buffs("candace", 2, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "candace_c2_hp"));
        assert!(c2.iter().any(|b| b.name == "candace_c2_hp"));
        let hp_buff = c2.iter().find(|b| b.name == "candace_c2_hp").unwrap();
        assert_eq!(hp_buff.stat, BuffableStat::HpPercent);
        assert!((hp_buff.value - 0.20).abs() < 1e-6);
        assert!(matches!(
            hp_buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }
    #[test]
    fn test_conditional_mona_c1_c2_c4_c6() {
        let c0 = get_talent_conditional_buffs("mona", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("mona", 6, &[10, 10, 10]);
        // C0 should have only the burst DMG bonus
        assert!(!c0.iter().any(|b| b.name == "mona_c1_reaction_dmg"));
        assert!(!c0.iter().any(|b| b.name == "mona_c2_em"));
        // C6 should have all new conditional buffs
        assert!(c6.iter().any(|b| b.name == "mona_c1_reaction_dmg"));
        assert!(c6.iter().any(|b| b.name == "mona_c2_em"));
        assert!(c6.iter().any(|b| b.name == "mona_c4_crit_rate"));
        assert!(c6.iter().any(|b| b.name == "mona_c4_crit_dmg"));
        assert!(c6.iter().any(|b| b.name == "mona_c6_charged_dmg"));
        // Check values
        let em_buff = c6.iter().find(|b| b.name == "mona_c2_em").unwrap();
        assert!((em_buff.value - 80.0).abs() < 1e-6);
        let cr_buff = c6.iter().find(|b| b.name == "mona_c4_crit_rate").unwrap();
        assert!((cr_buff.value - 0.15).abs() < 1e-6);
        let charged_buff = c6.iter().find(|b| b.name == "mona_c6_charged_dmg").unwrap();
        assert!((charged_buff.value - 1.80).abs() < 1e-6);
    }
    #[test]
    fn test_conditional_nilou_c2_c4_c6() {
        let c0 = get_talent_conditional_buffs("nilou", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("nilou", 6, &[10, 10, 10]);
        // C0 should have no conditional buffs (A4 has activation: None)
        assert!(!c0.iter().any(|b| b.name == "nilou_c2_hydro_res_shred"));
        // C6 should have all new constellation buffs
        assert!(c6.iter().any(|b| b.name == "nilou_c2_hydro_res_shred"));
        assert!(c6.iter().any(|b| b.name == "nilou_c2_dendro_res_shred"));
        assert!(c6.iter().any(|b| b.name == "nilou_c4_burst_dmg"));
        assert!(c6.iter().any(|b| b.name == "nilou_c6_crit_rate"));
        assert!(c6.iter().any(|b| b.name == "nilou_c6_crit_dmg"));
        // C6 scales_on HP with cap
        let cr = c6.iter().find(|b| b.name == "nilou_c6_crit_rate").unwrap();
        assert_eq!(cr.scales_on, Some(ScalingStat::Hp));
        assert_eq!(cr.stat, BuffableStat::CritRate);
        let cd = c6.iter().find(|b| b.name == "nilou_c6_crit_dmg").unwrap();
        assert_eq!(cd.scales_on, Some(ScalingStat::Hp));
        assert_eq!(cd.stat, BuffableStat::CritDmg);
    }
    #[test]
    fn test_conditional_xingqiu_c4_skill_dmg() {
        let c0 = get_talent_conditional_buffs("xingqiu", 0, &[10, 10, 10]);
        let c4 = get_talent_conditional_buffs("xingqiu", 4, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "xingqiu_c4_skill_dmg"));
        assert!(c4.iter().any(|b| b.name == "xingqiu_c4_skill_dmg"));
        let skill_buff = c4
            .iter()
            .find(|b| b.name == "xingqiu_c4_skill_dmg")
            .unwrap();
        assert_eq!(skill_buff.stat, BuffableStat::SkillDmgBonus);
        assert!((skill_buff.value - 0.50).abs() < 1e-6);
        assert!(matches!(
            skill_buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }
    #[test]
    fn test_conditional_yelan_c4_hp_stacks() {
        let c0 = get_talent_conditional_buffs("yelan", 0, &[10, 10, 10]);
        let c4 = get_talent_conditional_buffs("yelan", 4, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "yelan_c4_hp"));
        assert!(c4.iter().any(|b| b.name == "yelan_c4_hp"));
        let hp_buff = c4.iter().find(|b| b.name == "yelan_c4_hp").unwrap();
        assert_eq!(hp_buff.stat, BuffableStat::HpPercent);
        assert!((hp_buff.value - 0.10).abs() < 1e-6);
        assert!(matches!(
            hp_buff.activation,
            Activation::Manual(ManualCondition::Stacks(4))
        ));
    }
    #[test]
    fn test_conditional_beidou_c4() {
        let c0 = get_talent_conditional_buffs("beidou", 0, &[10, 10, 10]);
        let c4 = get_talent_conditional_buffs("beidou", 4, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "beidou_c4_electro_dmg"));
        assert!(c4.iter().any(|b| b.name == "beidou_c4_electro_dmg"));
    }
    #[test]
    fn test_conditional_flins_c2_c6() {
        let c0 = get_talent_conditional_buffs("flins", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("flins", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "flins_c2_electro_res_shred"));
        assert!(c6.iter().any(|b| b.name == "flins_c2_electro_res_shred"));
        assert!(c6.iter().any(|b| b.name == "flins_c6_lunar_charged_self"));
        assert!(c6.iter().any(|b| b.name == "flins_c6_lunar_charged_team"));
    }
    #[test]
    fn test_conditional_iansan_c2_c6() {
        let c0 = get_talent_conditional_buffs("iansan", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("iansan", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "iansan_c2_atk"));
        assert!(c6.iter().any(|b| b.name == "iansan_c2_atk"));
        assert!(c6.iter().any(|b| b.name == "iansan_c6_dmg_bonus"));
    }
    #[test]
    fn test_conditional_ineffa_c1() {
        let c0 = get_talent_conditional_buffs("ineffa", 0, &[10, 10, 10]);
        let c1 = get_talent_conditional_buffs("ineffa", 1, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "ineffa_c1_lunar_charged_dmg"));
        let buff = c1
            .iter()
            .find(|b| b.name == "ineffa_c1_lunar_charged_dmg")
            .unwrap();
        assert_eq!(buff.scales_on, Some(ScalingStat::TotalAtk));
        // Check cap via find_talent_buffs (cap is on TalentBuffDef, not TalentConditionalBuff)
        let defs = find_talent_buffs("ineffa").unwrap();
        let def = defs
            .iter()
            .find(|b| b.name == "ineffa_c1_lunar_charged_dmg")
            .unwrap();
        assert_eq!(def.cap, Some(0.50));
    }
    #[test]
    fn test_conditional_yae_miko_c6() {
        let c0 = get_talent_conditional_buffs("yae_miko", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("yae_miko", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "yae_miko_c6_def_ignore"));
        let buff = c6
            .iter()
            .find(|b| b.name == "yae_miko_c6_def_ignore")
            .unwrap();
        assert_eq!(buff.stat, BuffableStat::DefIgnore);
        assert!((buff.value - 0.60).abs() < 1e-9);
    }
    #[test]
    fn test_conditional_escoffier_c1_c2() {
        let c0 = get_talent_conditional_buffs("escoffier", 0, &[10, 10, 10]);
        let c2 = get_talent_conditional_buffs("escoffier", 2, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "escoffier_c1_cryo_crit_dmg"));
        assert!(c2.iter().any(|b| b.name == "escoffier_c1_cryo_crit_dmg"));
        assert!(c2.iter().any(|b| b.name == "escoffier_c2_skill_flat_dmg"));
        let c2_buff = c2
            .iter()
            .find(|b| b.name == "escoffier_c2_skill_flat_dmg")
            .unwrap();
        assert_eq!(c2_buff.scales_on, Some(ScalingStat::TotalAtk));
    }
    #[test]
    fn test_conditional_eula_c1_c4() {
        let c0 = get_talent_conditional_buffs("eula", 0, &[10, 10, 10]);
        let c4 = get_talent_conditional_buffs("eula", 4, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "eula_c1_physical_dmg"));
        assert!(c4.iter().any(|b| b.name == "eula_c1_physical_dmg"));
        assert!(c4.iter().any(|b| b.name == "eula_c4_burst_dmg"));
    }
    #[test]
    fn test_conditional_ganyu_c1() {
        let c0 = get_talent_conditional_buffs("ganyu", 0, &[10, 10, 10]);
        let c1 = get_talent_conditional_buffs("ganyu", 1, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "ganyu_c1_cryo_res_shred"));
        assert!(c1.iter().any(|b| b.name == "ganyu_c1_cryo_res_shred"));
    }
    #[test]
    fn test_conditional_mika_c6() {
        let c0 = get_talent_conditional_buffs("mika", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("mika", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "mika_c6_physical_crit_dmg"));
        assert!(c6.iter().any(|b| b.name == "mika_c6_physical_crit_dmg"));
    }
    #[test]
    fn test_conditional_qiqi_c2() {
        let c0 = get_talent_conditional_buffs("qiqi", 0, &[10, 10, 10]);
        let c2 = get_talent_conditional_buffs("qiqi", 2, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "qiqi_c2_normal_dmg"));
        assert!(c2.iter().any(|b| b.name == "qiqi_c2_normal_dmg"));
        assert!(c2.iter().any(|b| b.name == "qiqi_c2_charged_dmg"));
    }
    #[test]
    fn test_conditional_rosaria_c1() {
        let c0 = get_talent_conditional_buffs("rosaria", 0, &[10, 10, 10]);
        let c1 = get_talent_conditional_buffs("rosaria", 1, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "rosaria_c1_normal_dmg"));
        assert!(c1.iter().any(|b| b.name == "rosaria_c1_normal_dmg"));
    }
    #[test]
    fn test_conditional_shenhe_c2() {
        let c0 = get_talent_conditional_buffs("shenhe", 0, &[10, 10, 10]);
        let c2 = get_talent_conditional_buffs("shenhe", 2, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "shenhe_c2_cryo_crit_dmg"));
        assert!(c2.iter().any(|b| b.name == "shenhe_c2_cryo_crit_dmg"));
    }
    #[test]
    fn test_conditional_baizhu_c4_c6() {
        let c0 = get_talent_conditional_buffs("baizhu", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("baizhu", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "baizhu_c4_em"));
        assert!(c6.iter().any(|b| b.name == "baizhu_c4_em"));
        assert!(c6.iter().any(|b| b.name == "baizhu_c6_skill_flat_dmg"));
        let c6_buff = c6
            .iter()
            .find(|b| b.name == "baizhu_c6_skill_flat_dmg")
            .unwrap();
        assert_eq!(c6_buff.scales_on, Some(ScalingStat::Hp));
    }
    #[test]
    fn test_conditional_collei_c4() {
        let c0 = get_talent_conditional_buffs("collei", 0, &[10, 10, 10]);
        let c4 = get_talent_conditional_buffs("collei", 4, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "collei_c4_em"));
        let buff = c4.iter().find(|b| b.name == "collei_c4_em").unwrap();
        assert_eq!(buff.target, BuffTarget::TeamExcludeSelf);
    }
    #[test]
    fn test_conditional_nahida_c2_c4() {
        let c0 = get_talent_conditional_buffs("nahida", 0, &[10, 10, 10]);
        let c4 = get_talent_conditional_buffs("nahida", 4, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "nahida_c2_crit_rate"));
        assert!(c4.iter().any(|b| b.name == "nahida_c2_crit_rate"));
        assert!(c4.iter().any(|b| b.name == "nahida_c2_crit_dmg"));
        assert!(c4.iter().any(|b| b.name == "nahida_c2_def_reduction"));
        assert!(c4.iter().any(|b| b.name == "nahida_c4_em"));
    }
    #[test]
    fn test_conditional_nefer_c2_c4_c6() {
        let c0 = get_talent_conditional_buffs("nefer", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("nefer", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "nefer_c2_em"));
        assert!(c6.iter().any(|b| b.name == "nefer_c2_em"));
        assert!(c6.iter().any(|b| b.name == "nefer_c4_dendro_res_shred"));
        assert!(c6.iter().any(|b| b.name == "nefer_c6_lunar_bloom_dmg"));
    }
    #[test]
    fn test_conditional_yaoyao_c1_c4() {
        let c0 = get_talent_conditional_buffs("yaoyao", 0, &[10, 10, 10]);
        let c4 = get_talent_conditional_buffs("yaoyao", 4, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "yaoyao_c1_dendro_dmg"));
        assert!(c4.iter().any(|b| b.name == "yaoyao_c1_dendro_dmg"));
        let em_buff = c4.iter().find(|b| b.name == "yaoyao_c4_em").unwrap();
        assert_eq!(em_buff.scales_on, Some(ScalingStat::Hp));
        // cap is on TalentBuffDef, verify via find_talent_buffs
        let defs = find_talent_buffs("yaoyao").unwrap();
        let def = defs.iter().find(|b| b.name == "yaoyao_c4_em").unwrap();
        assert_eq!(def.cap, Some(120.0));
    }
    #[test]
    fn test_conditional_kirara_c6() {
        let c0 = get_talent_conditional_buffs("kirara", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("kirara", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "kirara_c6_dmg_bonus"));
        assert!(c6.iter().any(|b| b.name == "kirara_c6_dmg_bonus"));
    }
    #[test]
    fn test_conditional_traveler_dendro_c6() {
        let c0 = get_talent_conditional_buffs("traveler_dendro", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("traveler_dendro", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "traveler_dendro_c6_dendro_dmg"));
        assert!(c6.iter().any(|b| b.name == "traveler_dendro_c6_dendro_dmg"));
    }
    #[test]
    fn test_conditional_faruzan_c6() {
        let c0 = get_talent_conditional_buffs("faruzan", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("faruzan", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "faruzan_c6_anemo_crit_dmg"));
        assert!(c6.iter().any(|b| b.name == "faruzan_c6_anemo_crit_dmg"));
    }
    #[test]
    fn test_conditional_jahoda_c6() {
        let c0 = get_talent_conditional_buffs("jahoda", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("jahoda", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "jahoda_c6_crit_rate"));
        assert!(c6.iter().any(|b| b.name == "jahoda_c6_crit_rate"));
        assert!(c6.iter().any(|b| b.name == "jahoda_c6_crit_dmg"));
    }
    #[test]
    fn test_conditional_kazuha_c2_c6() {
        let c0 = get_talent_conditional_buffs("kazuha", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("kazuha", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "kazuha_c2_em"));
        assert!(c6.iter().any(|b| b.name == "kazuha_c2_em"));
        assert!(c6.iter().any(|b| b.name == "kazuha_c6_normal_dmg"));
        let normal = c6
            .iter()
            .find(|b| b.name == "kazuha_c6_normal_dmg")
            .unwrap();
        assert_eq!(normal.scales_on, Some(ScalingStat::Em));
    }
    #[test]
    fn test_conditional_venti_c4_c6() {
        let c0 = get_talent_conditional_buffs("venti", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("venti", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "venti_c4_anemo_dmg"));
        assert!(c6.iter().any(|b| b.name == "venti_c4_anemo_dmg"));
        assert!(c6.iter().any(|b| b.name == "venti_c6_anemo_res_shred"));
    }
    #[test]
    fn test_conditional_xianyun_c2_c6() {
        let c0 = get_talent_conditional_buffs("xianyun", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("xianyun", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "xianyun_c2_atk"));
        assert!(c6.iter().any(|b| b.name == "xianyun_c2_atk"));
        assert!(c6.iter().any(|b| b.name == "xianyun_c6_crit_dmg"));
    }
    #[test]
    fn test_conditional_albedo_c1_c4_c6() {
        let c0 = get_talent_conditional_buffs("albedo", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("albedo", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "albedo_c1_def"));
        assert!(c6.iter().any(|b| b.name == "albedo_c1_def"));
        assert!(c6.iter().any(|b| b.name == "albedo_c4_plunging_dmg"));
        assert!(c6.iter().any(|b| b.name == "albedo_c6_dmg_bonus"));
    }
    #[test]
    fn test_conditional_gorou_c6() {
        let c0 = get_talent_conditional_buffs("gorou", 0, &[10, 10, 10]);
        let c6 = get_talent_conditional_buffs("gorou", 6, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "gorou_c6_geo_crit_dmg"));
        assert!(c6.iter().any(|b| b.name == "gorou_c6_geo_crit_dmg"));
    }
    #[test]
    fn test_conditional_illuga_c4() {
        let c0 = get_talent_conditional_buffs("illuga", 0, &[10, 10, 10]);
        let c4 = get_talent_conditional_buffs("illuga", 4, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "illuga_c4_def_flat"));
        assert!(c4.iter().any(|b| b.name == "illuga_c4_def_flat"));
    }
    #[test]
    fn test_conditional_yun_jin_c2_c4() {
        let c0 = get_talent_conditional_buffs("yun_jin", 0, &[10, 10, 10]);
        let c4 = get_talent_conditional_buffs("yun_jin", 4, &[10, 10, 10]);
        assert!(!c0.iter().any(|b| b.name == "yun_jin_c2_normal_dmg"));
        assert!(c4.iter().any(|b| b.name == "yun_jin_c2_normal_dmg"));
        assert!(c4.iter().any(|b| b.name == "yun_jin_c4_def"));
    }
}
