use serde::Deserialize;

use genshin_calc_core::enemy::Enemy;
use genshin_calc_core::reaction::Reaction;
use genshin_calc_core::stats::Stats;
use genshin_calc_core::types::{DamageType, Element, ScalingStat};

#[derive(Deserialize)]
pub struct CharacterTestData {
    pub character: CharacterInfo,
    pub cases: Vec<TestCase>,
}

#[derive(Deserialize)]
pub struct CharacterInfo {
    pub name: String,
    #[allow(dead_code)]
    pub element: Option<String>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum TestCase {
    #[serde(rename = "normal")]
    Normal(DamageCase),
    #[serde(rename = "amplifying")]
    Amplifying(DamageCase),
    #[serde(rename = "catalyze")]
    Catalyze(DamageCase),
    #[serde(rename = "transformative")]
    Transformative(TransformativeCase),
    #[serde(rename = "lunar")]
    Lunar(LunarCase),
}

#[derive(Deserialize)]
pub struct DamageCase {
    pub name: String,
    pub character_level: u32,
    pub talent_multiplier: f64,
    #[serde(default = "default_scaling_stat")]
    pub scaling_stat: String,
    pub damage_type: String,
    pub element: Option<String>,
    pub reaction: Option<String>,
    #[serde(default)]
    pub reaction_bonus: f64,
    pub stats: StatsData,
    pub enemy: EnemyData,
    pub expected: DamageExpected,
    pub tolerance: Option<f64>,
}

#[derive(Deserialize)]
pub struct TransformativeCase {
    pub name: String,
    pub character_level: u32,
    pub elemental_mastery: f64,
    pub reaction: String,
    #[serde(default)]
    pub reaction_bonus: f64,
    pub enemy: EnemyData,
    pub expected: TransformativeExpected,
    pub tolerance: Option<f64>,
}

#[derive(Deserialize)]
pub struct LunarCase {
    pub name: String,
    pub character_level: u32,
    pub elemental_mastery: f64,
    pub reaction: String,
    #[serde(default)]
    pub reaction_bonus: f64,
    pub crit_rate: f64,
    pub crit_dmg: f64,
    pub enemy: EnemyData,
    pub expected: DamageExpected,
    pub tolerance: Option<f64>,
}

#[derive(Deserialize)]
pub struct StatsData {
    pub hp: f64,
    pub atk: f64,
    pub def: f64,
    pub elemental_mastery: f64,
    pub crit_rate: f64,
    pub crit_dmg: f64,
    #[serde(default = "default_energy_recharge")]
    pub energy_recharge: f64,
    pub dmg_bonus: f64,
}

#[derive(Deserialize)]
pub struct EnemyData {
    pub level: u32,
    pub resistance: f64,
    #[serde(default)]
    pub def_reduction: f64,
}

#[derive(Deserialize)]
pub struct DamageExpected {
    pub non_crit: f64,
    pub crit: f64,
    pub average: f64,
}

#[derive(Deserialize)]
pub struct TransformativeExpected {
    pub damage: f64,
}

fn default_scaling_stat() -> String {
    "Atk".to_string()
}

fn default_energy_recharge() -> f64 {
    1.0
}

pub fn parse_element(s: &str) -> Element {
    match s {
        "Pyro" => Element::Pyro,
        "Hydro" => Element::Hydro,
        "Electro" => Element::Electro,
        "Cryo" => Element::Cryo,
        "Dendro" => Element::Dendro,
        "Anemo" => Element::Anemo,
        "Geo" => Element::Geo,
        other => panic!("Unknown element: {other}"),
    }
}

pub fn parse_scaling_stat(s: &str) -> ScalingStat {
    match s {
        "Atk" => ScalingStat::Atk,
        "Hp" => ScalingStat::Hp,
        "Def" => ScalingStat::Def,
        "Em" => ScalingStat::Em,
        other => panic!("Unknown scaling stat: {other}"),
    }
}

pub fn parse_damage_type(s: &str) -> DamageType {
    match s {
        "Normal" => DamageType::Normal,
        "Charged" => DamageType::Charged,
        "Plunging" => DamageType::Plunging,
        "Skill" => DamageType::Skill,
        "Burst" => DamageType::Burst,
        other => panic!("Unknown damage type: {other}"),
    }
}

pub fn parse_reaction(s: &str) -> Reaction {
    match s {
        "Vaporize" => Reaction::Vaporize,
        "Melt" => Reaction::Melt,
        "Aggravate" => Reaction::Aggravate,
        "Spread" => Reaction::Spread,
        "Overloaded" => Reaction::Overloaded,
        "Superconduct" => Reaction::Superconduct,
        "ElectroCharged" => Reaction::ElectroCharged,
        "Shattered" => Reaction::Shattered,
        "Bloom" => Reaction::Bloom,
        "Hyperbloom" => Reaction::Hyperbloom,
        "Burgeon" => Reaction::Burgeon,
        "Burning" => Reaction::Burning,
        "SwirlPyro" => Reaction::Swirl(Element::Pyro),
        "SwirlHydro" => Reaction::Swirl(Element::Hydro),
        "SwirlElectro" => Reaction::Swirl(Element::Electro),
        "SwirlCryo" => Reaction::Swirl(Element::Cryo),
        "LunarElectroCharged" => Reaction::LunarElectroCharged,
        "LunarBloom" => Reaction::LunarBloom,
        "LunarCrystallize" => Reaction::LunarCrystallize,
        "LunarCrystallizeSecondary" => Reaction::LunarCrystallizeSecondary,
        other => panic!("Unknown reaction: {other}"),
    }
}

pub fn to_stats(data: &StatsData) -> Stats {
    Stats {
        hp: data.hp,
        atk: data.atk,
        def: data.def,
        elemental_mastery: data.elemental_mastery,
        crit_rate: data.crit_rate,
        crit_dmg: data.crit_dmg,
        energy_recharge: data.energy_recharge,
        dmg_bonus: data.dmg_bonus,
    }
}

pub fn to_enemy(data: &EnemyData) -> Enemy {
    Enemy {
        level: data.level,
        resistance: data.resistance,
        def_reduction: data.def_reduction,
    }
}
