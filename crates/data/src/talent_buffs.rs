use genshin_calc_core::{BuffTarget, BuffableStat, ScalingStat};
use serde::{Deserialize, Serialize};

/// Source of a talent buff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TalentBuffSource {
    /// Ascension passive (A1 or A4).
    AscensionPassive,
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
}

// ===== Bennett =====
// Elemental Burst "Fantastic Voyage" ATK buff: 56%~119% of Base ATK (Lv1-15)
static BENNETT_BURST_ATK_SCALING: [f64; 15] = [
    0.56, 0.602, 0.644, 0.70, 0.742, 0.784, 0.84, 0.896, 0.952,
    1.008, 1.064, 1.12, 1.19, 1.26, 1.33,
];

static BENNETT_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Fantastic Voyage ATK Bonus",
    description: "Characters within the burst field gain ATK bonus based on Bennett's Base ATK",
    stat: BuffableStat::AtkFlat,
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&BENNETT_BURST_ATK_SCALING),
    scales_on: Some(ScalingStat::Atk), // base_atk × scaling
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalBurst,
    min_constellation: 0,
}];

// ===== Kazuha =====
// A4 passive "Poetics of Fuubutsu": 0.04% Elemental DMG Bonus per point of EM
static KAZUHA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Poetics of Fuubutsu",
    description: "After triggering Swirl, grants 0.04% Elemental DMG Bonus per point of EM",
    stat: BuffableStat::DmgBonus,
    base_value: 0.0, // EM-dependent — builder computes EM×0.0004 at resolve time
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Nahida =====
// A4 passive "On All Things Meditated": grants 25% of highest EM in team (max 250)
static NAHIDA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "On All Things Meditated",
    description: "Grants EM to party based on highest EM in team (25%, max 250)",
    stat: BuffableStat::ElementalMastery,
    base_value: 0.0, // Team EM-dependent — builder sets value at resolve time
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Shenhe =====
// Elemental Skill "Spring Spirit Summoning": flat Cryo DMG based on ATK (Lv1-15)
static SHENHE_SKILL_SCALING: [f64; 15] = [
    0.4566, 0.4909, 0.5251, 0.5708, 0.6050, 0.6393, 0.6849, 0.7306, 0.7763,
    0.8219, 0.8676, 0.9132, 0.9703, 1.0274, 1.0844,
];

static SHENHE_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Spring Spirit Summoning Quill DMG",
    description: "Adds flat Cryo DMG based on Shenhe's ATK to party's Cryo attacks",
    stat: BuffableStat::AtkFlat, // treated as flat bonus scaled from ATK
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&SHENHE_SKILL_SCALING),
    scales_on: Some(ScalingStat::Atk),
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalSkill,
    min_constellation: 0,
}];

// ===== Yun Jin =====
// Elemental Burst "Cliffbreaker's Banner": flat Normal ATK DMG based on DEF (Lv1-15)
static YUN_JIN_BURST_SCALING: [f64; 15] = [
    0.3216, 0.3457, 0.3699, 0.4020, 0.4262, 0.4503, 0.4824, 0.5145, 0.5466,
    0.5789, 0.6110, 0.6431, 0.6833, 0.7234, 0.7636,
];

static YUN_JIN_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Cliffbreaker's Banner Normal ATK Bonus",
    description: "Normal Attack DMG increased based on Yun Jin's DEF",
    stat: BuffableStat::AtkFlat,
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&YUN_JIN_BURST_SCALING),
    scales_on: Some(ScalingStat::Def),
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalBurst,
    min_constellation: 0,
}];

// ===== Mona =====
// Elemental Burst "Stellaris Phantasm": DMG bonus from Omen (Lv1-15)
static MONA_BURST_DMG_SCALING: [f64; 15] = [
    0.42, 0.44, 0.46, 0.50, 0.52, 0.54, 0.58, 0.62, 0.66,
    0.70, 0.74, 0.78, 0.82, 0.86, 0.90,
];

static MONA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Stellaris Phantasm DMG Bonus",
    description: "Omen increases DMG taken by opponents",
    stat: BuffableStat::DmgBonus,
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&MONA_BURST_DMG_SCALING),
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalBurst,
    min_constellation: 0,
}];

// ===== Sara =====
// Elemental Skill/Burst "Tengu Juurai": ATK bonus based on Sara's Base ATK (Lv1-15)
static SARA_ATK_SCALING: [f64; 15] = [
    0.4296, 0.4618, 0.4940, 0.5370, 0.5692, 0.6014, 0.6444, 0.6874, 0.7304,
    0.7734, 0.8164, 0.8594, 0.9131, 0.9668, 1.0206,
];

static SARA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Tengu Juurai ATK Bonus",
    description: "ATK bonus based on Sara's Base ATK",
    stat: BuffableStat::AtkFlat,
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&SARA_ATK_SCALING),
    scales_on: Some(ScalingStat::Atk),
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalSkill,
    min_constellation: 0,
}];

// ===== Rosaria =====
// A4 passive "Shadow Samaritan": grants 15% of Rosaria's CRIT Rate to party after burst
static ROSARIA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Shadow Samaritan CRIT Rate Share",
    description: "After burst, grants 15% of Rosaria's CRIT Rate to party",
    stat: BuffableStat::CritRate,
    base_value: 0.15, // 15% of own CRIT Rate — builder computes crit_rate * 0.15 at resolve time
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::TeamExcludeSelf,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Furina =====
// Elemental Burst "Let the People Rejoice": Fanfare stacks grant DMG bonus (max per Lv)
static FURINA_BURST_DMG_SCALING: [f64; 15] = [
    0.09, 0.11, 0.13, 0.15, 0.17, 0.19, 0.21, 0.23, 0.25,
    0.27, 0.29, 0.31, 0.33, 0.35, 0.37,
];

static FURINA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Let the People Rejoice DMG Bonus",
    description: "Fanfare stacks grant DMG bonus (max based on talent level)",
    stat: BuffableStat::DmgBonus,
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&FURINA_BURST_DMG_SCALING),
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalBurst,
    min_constellation: 0,
}];

/// All character talent buff definitions.
static ALL_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("bennett", BENNETT_BUFFS),
    ("kazuha", KAZUHA_BUFFS),
    ("nahida", NAHIDA_BUFFS),
    ("shenhe", SHENHE_BUFFS),
    ("yun_jin", YUN_JIN_BUFFS),
    ("mona", MONA_BUFFS),
    ("sara", SARA_BUFFS),
    ("rosaria", ROSARIA_BUFFS),
    ("furina", FURINA_BUFFS),
    // Zhongli provides resistance shred (Enemy-side), not a stat buff — excluded here
];

/// Finds talent buff definitions for a character by ID.
///
/// Returns `None` for characters without defined talent buffs.
pub fn find_talent_buffs(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    ALL_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_bennett_buffs() {
        let buffs = find_talent_buffs("bennett").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
        assert_eq!(buffs[0].target, BuffTarget::Team);
    }

    #[test]
    fn test_find_rosaria_buffs() {
        let buffs = find_talent_buffs("rosaria").unwrap();
        assert_eq!(buffs[0].target, BuffTarget::TeamExcludeSelf);
    }

    #[test]
    fn test_find_nonexistent_character() {
        assert!(find_talent_buffs("zhongli").is_none());
        assert!(find_talent_buffs("unknown").is_none());
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
        let ids: Vec<&str> = ALL_TALENT_BUFFS.iter().map(|(id, _)| *id).collect();
        for (i, id) in ids.iter().enumerate() {
            assert!(
                !ids[i + 1..].contains(id),
                "Duplicate talent buff ID: {id}"
            );
        }
    }
}
