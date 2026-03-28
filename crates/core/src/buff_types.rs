use crate::types::Element;
use serde::{Deserialize, Serialize};

/// Stat that can be buffed by weapons, artifacts, or character talents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuffableStat {
    HpPercent,
    AtkPercent,
    DefPercent,
    HpFlat,
    AtkFlat,
    DefFlat,
    CritRate,
    CritDmg,
    ElementalMastery,
    EnergyRecharge,
    DmgBonus,
    ElementalDmgBonus(Element),
    PhysicalDmgBonus,
    NormalAtkDmgBonus,
    ChargedAtkDmgBonus,
    PlungingAtkDmgBonus,
    SkillDmgBonus,
    BurstDmgBonus,
    HealingBonus,
    ShieldStrength,
    // Reaction bonus stats (consumed by damage calculation, not applied to StatProfile)
    AmplifyingBonus,
    TransformativeBonus,
    AdditiveBonus,
    // Enemy resistance reduction (consumed by damage calculation, not applied to StatProfile)
    ElementalResReduction(Element),
    PhysicalResReduction,
    DefReduction,
    // Flat damage added to base (ATK*multiplier + flat_dmg), used by weapon passives
    NormalAtkFlatDmg,
    ChargedAtkFlatDmg,
    PlungingAtkFlatDmg,
    SkillFlatDmg,
    BurstFlatDmg,
    // Raw def_percent value (not total DEF), for weapons scaling on "DEF increase"
    DefPercentRaw,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_def_reduction_serde_roundtrip() {
        let stat = BuffableStat::DefReduction;
        let json = serde_json::to_string(&stat).unwrap();
        let deser: BuffableStat = serde_json::from_str(&json).unwrap();
        assert_eq!(stat, deser);
    }

    #[test]
    fn test_flat_dmg_variants_serde_roundtrip() {
        for stat in [
            BuffableStat::NormalAtkFlatDmg,
            BuffableStat::ChargedAtkFlatDmg,
            BuffableStat::PlungingAtkFlatDmg,
            BuffableStat::SkillFlatDmg,
            BuffableStat::BurstFlatDmg,
            BuffableStat::DefPercentRaw,
        ] {
            let json = serde_json::to_string(&stat).unwrap();
            let deser: BuffableStat = serde_json::from_str(&json).unwrap();
            assert_eq!(stat, deser);
        }
    }
}
