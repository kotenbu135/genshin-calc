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
}
