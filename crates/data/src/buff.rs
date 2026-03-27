use genshin_calc_core::Element;
use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StatBuff {
    pub stat: BuffableStat,
    pub value: f64,
    pub refinement_values: Option<[f64; 5]>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PassiveEffect {
    pub description: &'static str,
    pub buffs: &'static [StatBuff],
}
