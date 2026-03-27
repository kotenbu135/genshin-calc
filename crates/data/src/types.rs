use genshin_calc_core::{Element, ScalingStat};
use serde::{Deserialize, Serialize};

// -- Core Enums --

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Claymore,
    Polearm,
    Bow,
    Catalyst,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rarity {
    Star1,
    Star2,
    Star3,
    Star4,
    Star5,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtifactRarity {
    Star4,
    Star5,
    Star4And5,
}

// -- Talent Types --

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentScaling {
    pub name: &'static str,
    pub scaling_stat: ScalingStat,
    pub damage_element: Option<Element>,
    pub values: [f64; 15],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentData {
    pub name: &'static str,
    pub scalings: &'static [TalentScaling],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NormalAttackData {
    pub name: &'static str,
    pub hits: &'static [TalentScaling],
    pub charged: &'static [TalentScaling],
    pub plunging: &'static [TalentScaling],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentSet {
    pub normal_attack: NormalAttackData,
    pub elemental_skill: TalentData,
    pub elemental_burst: TalentData,
}

// -- Character --

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CharacterData {
    pub id: &'static str,
    pub name: &'static str,
    pub element: Element,
    pub weapon_type: WeaponType,
    pub rarity: Rarity,
    pub region: Region,
    pub base_hp: [f64; 4],
    pub base_atk: [f64; 4],
    pub base_def: [f64; 4],
    pub ascension_stat: AscensionStat,
    pub talents: TalentSet,
}

// -- Weapon --

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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeaponPassive {
    pub name: &'static str,
    pub effect: crate::buff::PassiveEffect,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeaponData {
    pub id: &'static str,
    pub name: &'static str,
    pub weapon_type: WeaponType,
    pub rarity: Rarity,
    pub base_atk: [f64; 4],
    pub sub_stat: Option<WeaponSubStat>,
    pub passive: Option<WeaponPassive>,
}

// -- Artifact --

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetEffect {
    pub description: &'static str,
    pub buffs: &'static [crate::buff::StatBuff],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ArtifactSet {
    pub id: &'static str,
    pub name: &'static str,
    pub rarity: ArtifactRarity,
    pub two_piece: SetEffect,
    pub four_piece: SetEffect,
}

// -- Enemy --

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

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EnemyData {
    pub id: &'static str,
    pub name: &'static str,
    pub resistance: &'static ResistanceTemplate,
}

impl ResistanceTemplate {
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
