use serde::{Deserialize, Serialize};

pub use genshin_calc_core::BuffableStat;
use genshin_calc_core::{Element, WeaponType};

/// A stat buff with a value and optional refinement scaling.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StatBuff {
    /// Which stat is buffed.
    pub stat: BuffableStat,
    /// Buff value at refinement 1 (or fixed value if no refinement).
    pub value: f64,
    /// Values at refinements 1-5. `None` for fixed buffs.
    pub refinement_values: Option<[f64; 5]>,
}

/// Passive effect from a weapon or artifact set.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PassiveEffect {
    /// Human-readable description.
    pub description: &'static str,
    /// Stat buffs provided by this effect.
    pub buffs: &'static [StatBuff],
    /// Conditional buffs that require activation.
    pub conditional_buffs: &'static [ConditionalBuff],
}

/// Condition the builder can evaluate automatically from character/team data.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum AutoCondition {
    /// Buff only applies to specific weapon types (e.g. Gladiator 4pc).
    WeaponTypeRequired(&'static [WeaponType]),
    /// Buff only applies to characters of specific elements.
    ElementRequired(&'static [Element]),
    /// Buff value computed from a stat. Multiplier comes from ConditionalBuff.value.
    /// Final = stat_value * multiplier, capped at `cap` if set.
    /// The BuffableStat here indicates which "stat family" to read the total value from:
    /// - HpPercent → total HP (base_hp * (1 + hp_percent) + hp_flat)
    /// - AtkPercent → total ATK (base_atk * (1 + atk_percent) + atk_flat)
    /// - DefPercent → total DEF (base_def * (1 + def_percent) + def_flat)
    /// - ElementalMastery → elemental_mastery
    /// - EnergyRecharge → energy_recharge
    StatScaling {
        stat: BuffableStat,
        cap: Option<f64>,
    },
    /// Requires N+ team members of a specific element (e.g. Gorou).
    TeamElementCount { element: Element, min_count: u8 },
    /// Team must consist only of specified elements (e.g. Nilou: Hydro+Dendro).
    TeamElementsOnly(&'static [Element]),
}

/// Condition requiring user input (game state the builder cannot determine).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ManualCondition {
    /// Simple on/off toggle (e.g. "HP below 50%").
    Toggle,
    /// Stackable buff with max stack count (e.g. CW 4pc max 3).
    Stacks(u8),
}

/// How a conditional buff is activated.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub enum Activation {
    /// Evaluated automatically by the builder.
    Auto(AutoCondition),
    /// Requires user input.
    Manual(ManualCondition),
    /// Both conditions must be satisfied (Auto first, short-circuits).
    Both(AutoCondition, ManualCondition),
}

/// A stat buff that requires a condition to be active.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ConditionalBuff {
    /// Machine-readable identifier (e.g. "homa_hp_bonus").
    pub name: &'static str,
    /// Human-readable description.
    pub description: &'static str,
    /// Which stat is buffed.
    pub stat: BuffableStat,
    /// Buff value (or multiplier for StatScaling).
    pub value: f64,
    /// Values at refinements 1-5. None for non-weapon / fixed buffs.
    /// TODO(P4): Builder will use refinement_values[r] when refinement level is available.
    pub refinement_values: Option<[f64; 5]>,
    /// Activation condition.
    pub activation: Activation,
}

/// User-provided activation state for a manual condition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ManualActivation {
    /// Toggle ON.
    Active,
    /// Stackable buff with specified stack count.
    Stacks(u8),
}

/// A conditional buff with source context, for UI display.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AvailableConditional {
    /// Source name (e.g. "Staff of Homa", "Crimson Witch 4pc").
    pub source: &'static str,
    /// The conditional buff definition.
    pub buff: &'static ConditionalBuff,
}
