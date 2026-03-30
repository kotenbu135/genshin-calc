use crate::types::Element;
use serde::{Deserialize, Serialize};

/// Stat that can be buffed by weapons, artifacts, or character talents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuffableStat {
    /// HP percentage bonus.
    HpPercent,
    /// ATK percentage bonus.
    AtkPercent,
    /// DEF percentage bonus.
    DefPercent,
    /// Flat HP bonus.
    HpFlat,
    /// Flat ATK bonus (e.g. feather artifact, Bennett burst).
    AtkFlat,
    /// Flat DEF bonus.
    DefFlat,
    /// Critical rate bonus.
    CritRate,
    /// Critical damage bonus.
    CritDmg,
    /// Elemental mastery bonus.
    ElementalMastery,
    /// Energy recharge bonus.
    EnergyRecharge,
    /// General DMG bonus (all elements and physical).
    DmgBonus,
    /// Elemental DMG bonus for a specific element.
    ElementalDmgBonus(Element),
    /// Physical DMG bonus.
    PhysicalDmgBonus,
    /// Normal attack DMG bonus.
    NormalAtkDmgBonus,
    /// Charged attack DMG bonus.
    ChargedAtkDmgBonus,
    /// Plunging attack DMG bonus.
    PlungingAtkDmgBonus,
    /// Elemental skill DMG bonus.
    SkillDmgBonus,
    /// Elemental burst DMG bonus.
    BurstDmgBonus,
    /// Healing bonus.
    HealingBonus,
    /// Shield strength bonus.
    ShieldStrength,
    /// Amplifying reaction (vaporize/melt) DMG bonus.
    AmplifyingBonus,
    /// Transformative reaction DMG bonus.
    TransformativeBonus,
    /// Additive (catalyze) reaction DMG bonus.
    AdditiveBonus,
    /// Player-side elemental resistance for a specific element.
    ElementalRes(Element),
    /// Enemy elemental resistance reduction for a specific element.
    ElementalResReduction(Element),
    /// Enemy physical resistance reduction.
    PhysicalResReduction,
    /// Enemy DEF reduction.
    DefReduction,
    /// Flat damage added to normal attacks (ATK * multiplier + flat_dmg).
    NormalAtkFlatDmg,
    /// Flat damage added to charged attacks.
    ChargedAtkFlatDmg,
    /// Flat damage added to plunging attacks.
    PlungingAtkFlatDmg,
    /// Flat damage added to elemental skill.
    SkillFlatDmg,
    /// Flat damage added to elemental burst.
    BurstFlatDmg,
    /// Raw DEF percentage value (not total DEF), for weapons scaling on "DEF increase".
    DefPercentRaw,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elemental_res_serde_roundtrip() {
        use crate::types::Element;
        for element in [
            Element::Pyro,
            Element::Hydro,
            Element::Electro,
            Element::Cryo,
            Element::Dendro,
            Element::Anemo,
            Element::Geo,
        ] {
            let stat = BuffableStat::ElementalRes(element);
            let json = serde_json::to_string(&stat).unwrap();
            let deser: BuffableStat = serde_json::from_str(&json).unwrap();
            assert_eq!(stat, deser);
        }
    }

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
