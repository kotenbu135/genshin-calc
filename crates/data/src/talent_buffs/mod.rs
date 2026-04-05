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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_bennett_buffs() {
        let buffs = find_talent_buffs("bennett").unwrap();
        assert_eq!(buffs.len(), 2);
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
        assert_eq!(buffs[0].target, BuffTarget::Team);
        // C6
        assert_eq!(
            buffs[1].stat,
            BuffableStat::ElementalDmgBonus(Element::Pyro)
        );
        assert!((buffs[1].base_value - 0.15).abs() < 1e-6);
        assert_eq!(buffs[1].min_constellation, 6);
    }

    #[test]
    fn test_find_rosaria_buffs() {
        let buffs = find_talent_buffs("rosaria").unwrap();
        assert_eq!(buffs[0].target, BuffTarget::TeamExcludeSelf);
    }

    #[test]
    fn test_find_nonexistent_character() {
        assert!(find_talent_buffs("diluc").is_none());
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
        assert_eq!(buffs.len(), 2);
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
        assert_eq!(buffs.len(), 1);
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
        assert_eq!(buffs.len(), 1);
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
        assert_eq!(buffs.len(), 1);
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
        // 5 FlatDmg entries + 2 A4 press entries + 3 A4 hold entries = 10 total
        assert_eq!(buffs.len(), 10);
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
        assert_eq!(buffs.len(), 1);
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
        assert_eq!(buffs.len(), 2);
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
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::NormalAtkDmgBonus);
        assert!(buffs[0].scales_with_talent);
        assert_eq!(buffs[0].source, TalentBuffSource::ElementalBurst);
    }

    #[test]
    fn test_find_gorou_buffs() {
        let buffs = find_talent_buffs("gorou").unwrap();
        assert_eq!(buffs.len(), 2);
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
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::DmgBonus);
        assert!((buffs[0].base_value - 0.50).abs() < 1e-6); // max value 0.50
    }

    #[test]
    fn test_find_ineffa_talent_buffs() {
        let buffs = find_talent_buffs("ineffa").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((buffs[0].base_value - 0.06).abs() < 1e-6); // 6% of total ATK coefficient
        assert_eq!(buffs[0].scales_on, Some(ScalingStat::TotalAtk));
    }

    #[test]
    fn test_find_jahoda_buffs() {
        let buffs = find_talent_buffs("jahoda").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((buffs[0].base_value - 100.0).abs() < 1e-6);
    }

    #[test]
    fn test_find_nilou_buffs() {
        let buffs = find_talent_buffs("nilou").unwrap();
        assert_eq!(buffs.len(), 1);
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
        assert_eq!(buffs.len(), 2); // existing Anemo DMG + new Anemo RES shred
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
        assert_eq!(buffs.len(), 2);
        assert_eq!(buffs[0].stat, BuffableStat::BurstDmgBonus);
        assert!(buffs[0].scales_with_talent);
        assert_eq!(buffs[0].source, TalentBuffSource::ElementalSkill);
        // C4 ATK%
        assert_eq!(buffs[1].stat, BuffableStat::AtkPercent);
        assert!((buffs[1].base_value - 0.30).abs() < 1e-6);
        assert_eq!(buffs[1].min_constellation, 4);
    }

    #[test]
    fn test_find_xilonen_buffs() {
        let buffs = find_talent_buffs("xilonen").unwrap();
        assert_eq!(buffs.len(), 4);
        // Skill: Geo RES reduction (talent-scaled)
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalResReduction(Element::Geo)
        );
        assert!(buffs[0].scales_with_talent);
        // C4: flat DMG from DEF for Normal/Charged/Plunging
        assert_eq!(buffs[1].stat, BuffableStat::NormalAtkFlatDmg);
        assert!((buffs[1].base_value - 0.65).abs() < 1e-6);
        assert_eq!(buffs[1].scales_on, Some(ScalingStat::Def));
        assert_eq!(buffs[1].min_constellation, 4);
        assert_eq!(buffs[2].stat, BuffableStat::ChargedAtkFlatDmg);
        assert_eq!(buffs[3].stat, BuffableStat::PlungingAtkFlatDmg);
        for b in &buffs[1..] {
            assert!((b.base_value - 0.65).abs() < 1e-6);
            assert_eq!(b.scales_on, Some(ScalingStat::Def));
            assert_eq!(b.min_constellation, 4);
        }
    }

    #[test]
    fn test_find_citlali_buffs() {
        let buffs = find_talent_buffs("citlali").unwrap();
        assert_eq!(buffs.len(), 5);
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
    }

    #[test]
    fn test_find_eula_buffs() {
        let buffs = find_talent_buffs("eula").unwrap();
        assert_eq!(buffs.len(), 2);
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
        assert_eq!(buffs.len(), 2);
        // A1: Mavuika's own ATK+30% (self-buff)
        assert_eq!(buffs[0].stat, BuffableStat::AtkPercent);
        assert!((buffs[0].base_value - 0.30).abs() < 1e-6);
        assert_eq!(buffs[0].source, TalentBuffSource::AscensionPassive(1));
        assert_eq!(buffs[0].target, BuffTarget::OnlySelf);
        // A4: DMG Bonus +40% (max, self-buff)
        assert_eq!(buffs[1].stat, BuffableStat::DmgBonus);
        assert!((buffs[1].base_value - 0.40).abs() < 1e-6);
        assert_eq!(buffs[1].source, TalentBuffSource::AscensionPassive(4));
        assert_eq!(buffs[1].target, BuffTarget::OnlySelf);
    }

    #[test]
    fn test_find_xiangling_buffs() {
        let buffs = find_talent_buffs("xiangling").unwrap();
        assert_eq!(buffs.len(), 2);
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
        assert_eq!(buffs.len(), 2);
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
        assert_eq!(buffs.len(), 3);
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
        assert_eq!(buffs.len(), 1);
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalResReduction(Element::Electro)
        );
        assert_eq!(buffs[0].min_constellation, 6);
    }

    #[test]
    fn test_find_klee_buffs() {
        let buffs = find_talent_buffs("klee").unwrap();
        assert_eq!(buffs.len(), 2);
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
        assert_eq!(buffs.len(), 1);
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
        assert_eq!(buffs.len(), 2);
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
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((buffs[0].base_value - 60.0).abs() < 1e-6);
        assert_eq!(buffs[0].min_constellation, 4);
    }

    #[test]
    fn test_find_mika_buffs() {
        let buffs = find_talent_buffs("mika").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::PhysicalDmgBonus);
        assert!((buffs[0].base_value - 0.10).abs() < 1e-6);
        assert_eq!(buffs[0].min_constellation, 6);
    }

    #[test]
    fn test_find_xinyan_buffs() {
        let buffs = find_talent_buffs("xinyan").unwrap();
        assert_eq!(buffs.len(), 1);
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
}
