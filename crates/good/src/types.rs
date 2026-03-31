use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodFormat {
    pub format: String,
    pub source: String,
    pub version: u8,
    pub characters: Option<Vec<GoodCharacter>>,
    pub artifacts: Option<Vec<GoodArtifact>>,
    pub weapons: Option<Vec<GoodWeapon>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodCharacter {
    pub key: String,
    pub level: u32,
    pub constellation: u8,
    pub ascension: u8,
    pub talent: GoodTalent,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodTalent {
    pub auto: u8,
    pub skill: u8,
    pub burst: u8,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodWeapon {
    pub key: String,
    pub level: u32,
    pub ascension: u8,
    pub refinement: u8,
    pub location: Option<String>,
    pub lock: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodArtifact {
    pub set_key: String,
    pub slot_key: String,
    pub level: u8,
    pub rarity: u8,
    pub main_stat_key: String,
    pub location: Option<String>,
    pub lock: bool,
    pub substats: Vec<GoodSubstat>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodSubstat {
    pub key: String,
    pub value: f64,
}
