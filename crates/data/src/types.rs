use genshin_calc_core::{Element, ScalingStat};
use serde::{Deserialize, Serialize};

// -- Core Enums --

/// Weapon type classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Claymore,
    Polearm,
    Bow,
    Catalyst,
}

/// Character or weapon rarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rarity {
    Star1,
    Star2,
    Star3,
    Star4,
    Star5,
}

/// Character region of origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Region {
    Mondstadt,
    Liyue,
    Inazuma,
    Sumeru,
    Fontaine,
    Natlan,
    Snezhnaya,
    Other,
}

/// Stat gained from character ascension.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AscensionStat {
    Hp(f64),
    Atk(f64),
    Def(f64),
    CritRate(f64),
    CritDmg(f64),
    ElementalMastery(f64),
    EnergyRecharge(f64),
    ElementalDmgBonus(Element, f64),
    PhysicalDmgBonus(f64),
    HealingBonus(f64),
}

/// Artifact set rarity availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtifactRarity {
    Star4,
    Star5,
    Star4And5,
}

// -- Talent Types --

/// Talent scaling entry with multipliers at each talent level.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentScaling {
    /// Scaling name (e.g. "1-Hit DMG").
    pub name: &'static str,
    /// Stat used for scaling.
    pub scaling_stat: ScalingStat,
    /// Element of the talent damage. `None` for physical.
    pub damage_element: Option<Element>,
    /// Multiplier values at talent levels 1-15.
    pub values: [f64; 15],
}

/// Talent data for an elemental skill or burst.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentData {
    /// Talent name.
    pub name: &'static str,
    /// Scaling entries.
    pub scalings: &'static [TalentScaling],
}

/// Normal attack data including charged and plunging attacks.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NormalAttackData {
    /// Attack name.
    pub name: &'static str,
    /// Normal attack hit scalings.
    pub hits: &'static [TalentScaling],
    /// Charged attack scalings.
    pub charged: &'static [TalentScaling],
    /// Plunging attack scalings.
    pub plunging: &'static [TalentScaling],
}

/// Complete talent set for a character.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentSet {
    pub normal_attack: NormalAttackData,
    pub elemental_skill: TalentData,
    pub elemental_burst: TalentData,
}

// -- Character --

/// Character data including base stats, talents, and metadata.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CharacterData {
    /// Unique identifier (lowercase, e.g. `"hu_tao"`).
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Character element.
    pub element: Element,
    /// Weapon type.
    pub weapon_type: WeaponType,
    /// Rarity (4-star or 5-star).
    pub rarity: Rarity,
    /// Region of origin.
    pub region: Region,
    /// Base HP at ascension breakpoints [Lv1, Lv20, Lv80, Lv90].
    pub base_hp: [f64; 4],
    /// Base ATK at ascension breakpoints.
    pub base_atk: [f64; 4],
    /// Base DEF at ascension breakpoints.
    pub base_def: [f64; 4],
    /// Stat gained from ascension.
    pub ascension_stat: AscensionStat,
    /// Character talent set.
    pub talents: TalentSet,
}

// -- Weapon --

/// Weapon sub-stat type with values at each ascension breakpoint.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WeaponSubStat {
    HpPercent([f64; 4]),
    AtkPercent([f64; 4]),
    DefPercent([f64; 4]),
    CritRate([f64; 4]),
    CritDmg([f64; 4]),
    ElementalMastery([f64; 4]),
    EnergyRecharge([f64; 4]),
    PhysicalDmgBonus([f64; 4]),
}

/// Weapon passive effect.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeaponPassive {
    /// Passive name.
    pub name: &'static str,
    /// Effect details.
    pub effect: crate::buff::PassiveEffect,
}

/// Weapon data including base stats, passives, and metadata.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeaponData {
    /// Unique identifier.
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Weapon type.
    pub weapon_type: WeaponType,
    /// Rarity.
    pub rarity: Rarity,
    /// Base ATK at ascension breakpoints.
    pub base_atk: [f64; 4],
    /// Sub-stat with values at ascension breakpoints.
    pub sub_stat: Option<WeaponSubStat>,
    /// Weapon passive effect.
    pub passive: Option<WeaponPassive>,
}

// -- Artifact --

/// Artifact set effect for 2-piece or 4-piece bonus.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetEffect {
    /// Effect description.
    pub description: &'static str,
    /// Stat buffs provided.
    pub buffs: &'static [crate::buff::StatBuff],
}

/// Artifact set data.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ArtifactSet {
    /// Unique identifier.
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Rarity availability.
    pub rarity: ArtifactRarity,
    /// 2-piece set bonus.
    pub two_piece: SetEffect,
    /// 4-piece set bonus.
    pub four_piece: SetEffect,
}

// -- Enemy --

/// Elemental resistance template for an enemy type.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ResistanceTemplate {
    pub name: &'static str,
    pub pyro: f64,
    pub hydro: f64,
    pub electro: f64,
    pub cryo: f64,
    pub dendro: f64,
    pub anemo: f64,
    pub geo: f64,
    pub physical: f64,
}

/// Enemy data with resistance template.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EnemyData {
    /// Unique identifier.
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Resistance values.
    pub resistance: &'static ResistanceTemplate,
}

impl ResistanceTemplate {
    /// Returns the resistance value for the given element, or physical if `None`.
    pub fn get(&self, element: Option<Element>) -> f64 {
        match element {
            Some(Element::Pyro) => self.pyro,
            Some(Element::Hydro) => self.hydro,
            Some(Element::Electro) => self.electro,
            Some(Element::Cryo) => self.cryo,
            Some(Element::Dendro) => self.dendro,
            Some(Element::Anemo) => self.anemo,
            Some(Element::Geo) => self.geo,
            None => self.physical,
        }
    }
}

impl EnemyData {
    /// Converts to a [`genshin_calc_core::Enemy`] for use in damage calculations.
    pub fn to_enemy(
        &self,
        element: Option<Element>,
        level: u32,
        def_reduction: f64,
    ) -> genshin_calc_core::Enemy {
        genshin_calc_core::Enemy {
            level,
            resistance: self.resistance.get(element),
            def_reduction,
        }
    }
}
