use genshin_calc_core::{BuffTarget, BuffableStat, Element, ScalingStat};
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
    0.56, 0.602, 0.644, 0.70, 0.742, 0.784, 0.84, 0.896, 0.952, 1.008, 1.064, 1.12, 1.19, 1.26,
    1.33,
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
    0.4566, 0.4909, 0.5251, 0.5708, 0.6050, 0.6393, 0.6849, 0.7306, 0.7763, 0.8219, 0.8676, 0.9132,
    0.9703, 1.0274, 1.0844,
];

static SHENHE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
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
    },
    TalentBuffDef {
        name: "Deific Embrace Press - Skill DMG",
        description: "After press E, party Skill DMG +15% for 10s",
        stat: BuffableStat::SkillDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Deific Embrace Press - Burst DMG",
        description: "After press E, party Burst DMG +15% for 10s",
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
];

// ===== Yun Jin =====
// Elemental Burst "Cliffbreaker's Banner": flat Normal ATK DMG based on DEF (Lv1-15)
static YUN_JIN_BURST_SCALING: [f64; 15] = [
    0.3216, 0.3457, 0.3699, 0.4020, 0.4262, 0.4503, 0.4824, 0.5145, 0.5466, 0.5789, 0.6110, 0.6431,
    0.6833, 0.7234, 0.7636,
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
    0.42, 0.44, 0.46, 0.50, 0.52, 0.54, 0.58, 0.62, 0.66, 0.70, 0.74, 0.78, 0.82, 0.86, 0.90,
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
    0.4296, 0.4618, 0.4940, 0.5370, 0.5692, 0.6014, 0.6444, 0.6874, 0.7304, 0.7734, 0.8164, 0.8594,
    0.9131, 0.9668, 1.0206,
];

static SARA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
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
    },
    TalentBuffDef {
        name: "Sin of Pride",
        description: "Electro CRIT DMG +60% (approximated as generic CritDmg; Electro-only in game)",
        stat: BuffableStat::CritDmg,
        base_value: 0.60,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
];

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
    0.09, 0.11, 0.13, 0.15, 0.17, 0.19, 0.21, 0.23, 0.25, 0.27, 0.29, 0.31, 0.33, 0.35, 0.37,
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

// ===== Sucrose =====
// A1 passive "Catalyst Conversion": Swirl triggers EM+50 for team 8s
// NOTE: A4 "Mollis Favonius" will be added in Task 6 (builder pattern)
static SUCROSE_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Catalyst Conversion",
    description: "After triggering Swirl, grants EM+50 to party members with matching element",
    stat: BuffableStat::ElementalMastery,
    base_value: 50.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Ganyu =====
// A4 passive "Harmony between Heaven and Earth": Cryo DMG+20% in burst field
static GANYU_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Harmony between Heaven and Earth",
    description: "Cryo DMG Bonus +20% for party members in burst field",
    stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
    base_value: 0.20,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Albedo =====
// A4 passive "Homuncular Nature": EM+125 for team after burst
static ALBEDO_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Homuncular Nature",
    description: "After burst, grants EM+125 to nearby party members for 10s",
    stat: BuffableStat::ElementalMastery,
    base_value: 125.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Ningguang =====
// A4 passive "Strategic Reserve": Geo DMG+12% when passing through Jade Screen
static NINGGUANG_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Strategic Reserve",
    description: "Characters passing through Jade Screen gain Geo DMG Bonus +12%",
    stat: BuffableStat::ElementalDmgBonus(Element::Geo),
    base_value: 0.12,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Dendro Traveler =====
// A4 passive "Verdant Luxury": EM+60 in Lea Lotus Lamp field
static TRAVELER_DENDRO_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Verdant Luxury",
    description: "Characters within Lea Lotus Lamp field gain EM+60",
    stat: BuffableStat::ElementalMastery,
    base_value: 60.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Yoimiya =====
// A4 passive "Summer Night's Dawn": ATK+20% to party (excl. self) after burst
static YOIMIYA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Summer Night's Dawn",
    description: "After burst, party members (excluding Yoimiya) gain ATK+20% (max assumption)",
    stat: BuffableStat::AtkPercent,
    base_value: 0.20,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::TeamExcludeSelf,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Chevreuse =====
// A1 passive "Vanguard's Coordinated Tactics": ATK+20% after Overloaded (Pyro+Electro team)
static CHEVREUSE_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Vanguard's Coordinated Tactics",
    description: "After Overloaded, ATK+20% for party (Pyro+Electro teams only, approximation)",
    stat: BuffableStat::AtkPercent,
    base_value: 0.20,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Diona =====
// C6 "Cat's Tail Closing Time": EM+200 in burst field
static DIONA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Cat's Tail Closing Time",
    description: "Characters within burst field gain EM+200",
    stat: BuffableStat::ElementalMastery,
    base_value: 200.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
}];

// ===== Amber =====
// C6 "Wildfire": ATK+15% for party during burst
static AMBER_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Wildfire",
    description: "During burst, party members gain ATK+15%",
    stat: BuffableStat::AtkPercent,
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
}];

// ===== Barbara =====
// C2 "Vitality Burst": Hydro DMG+15% during skill
static BARBARA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Vitality Burst",
    description: "During skill, active character gains Hydro DMG Bonus +15%",
    stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
}];

// ===== Thoma =====
// C6 "Burning Heart": Normal/Charged/Plunging +15% after burst
static THOMA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Burning Heart - Normal ATK",
        description: "After burst, party Normal ATK DMG +15%",
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
    TalentBuffDef {
        name: "Burning Heart - Charged ATK",
        description: "After burst, party Charged ATK DMG +15%",
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
    TalentBuffDef {
        name: "Burning Heart - Plunging ATK",
        description: "After burst, party Plunging ATK DMG +15%",
        stat: BuffableStat::PlungingAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
];

// ===== Faruzan =====
// Burst "The Wind's Secret Ways": Anemo DMG bonus per level (Lv1-15)
// Values from Genshin Wiki (Prayerful Wind's Benefit)
static FARUZAN_BURST_ANEMO_SCALING: [f64; 15] = [
    0.182, 0.196, 0.209, 0.228, 0.241, 0.255, 0.273, 0.291, 0.310, 0.328, 0.346, 0.364, 0.387,
    0.410, 0.432,
];

static FARUZAN_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Prayerful Wind's Benefit",
    description: "Anemo DMG Bonus based on burst talent level",
    stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&FARUZAN_BURST_ANEMO_SCALING),
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalBurst,
    min_constellation: 0,
}];

// ===== Candace =====
// Burst "Sacred Rite: Heron's Sanctum": Normal ATK DMG bonus per level (Lv1-15)
static CANDACE_BURST_NORMAL_SCALING: [f64; 15] = [
    0.20, 0.215, 0.23, 0.25, 0.265, 0.28, 0.30, 0.32, 0.34, 0.36, 0.38, 0.40, 0.425, 0.45, 0.475,
];

static CANDACE_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Sacred Rite: Heron's Sanctum",
    description: "Normal ATK DMG Bonus based on burst talent level",
    stat: BuffableStat::NormalAtkDmgBonus,
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&CANDACE_BURST_NORMAL_SCALING),
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalBurst,
    min_constellation: 0,
}];

// ===== Gorou =====
// Skill "Inuzaka All-Round Defense": DEF flat per level (Lv1-15) + Geo DMG+15% (3 Geo)
static GOROU_SKILL_DEF_SCALING: [f64; 15] = [
    206.16, 221.62, 237.08, 257.70, 273.16, 288.62, 309.24, 329.86, 350.48, 371.10, 391.72, 412.34,
    438.11, 463.88, 489.65,
];

static GOROU_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Inuzaka All-Round Defense",
        description: "DEF increase based on skill talent level",
        stat: BuffableStat::DefFlat,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&GOROU_SKILL_DEF_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Inuzaka All-Round Defense - Geo DMG",
        description: "With 3 Geo members, Geo DMG Bonus +15% (approximation: always registered)",
        stat: BuffableStat::ElementalDmgBonus(Element::Geo),
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
];

/// All character talent buff definitions.
static ALL_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("bennett", BENNETT_BUFFS),
    ("kazuha", KAZUHA_BUFFS),
    ("nahida", NAHIDA_BUFFS),
    ("shenhe", SHENHE_BUFFS),
    ("yun_jin", YUN_JIN_BUFFS),
    ("mona", MONA_BUFFS),
    ("kujou_sara", SARA_BUFFS),
    ("rosaria", ROSARIA_BUFFS),
    ("furina", FURINA_BUFFS),
    // Zhongli provides resistance shred (Enemy-side), not a stat buff — excluded here
    ("sucrose", SUCROSE_BUFFS),
    ("ganyu", GANYU_BUFFS),
    ("albedo", ALBEDO_BUFFS),
    ("ningguang", NINGGUANG_BUFFS),
    ("traveler_dendro", TRAVELER_DENDRO_BUFFS),
    ("yoimiya", YOIMIYA_BUFFS),
    ("chevreuse", CHEVREUSE_BUFFS),
    ("diona", DIONA_BUFFS),
    ("amber", AMBER_BUFFS),
    ("barbara", BARBARA_BUFFS),
    ("thoma", THOMA_BUFFS),
    ("faruzan", FARUZAN_BUFFS),
    ("candace", CANDACE_BUFFS),
    ("gorou", GOROU_BUFFS),
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
    fn test_find_sara_buffs_by_character_id() {
        // Sara's CharacterData uses id "kujou_sara", talent_buffs must match
        let buffs = find_talent_buffs("kujou_sara").unwrap();
        assert_eq!(buffs.len(), 2);
        assert_eq!(buffs[0].stat, BuffableStat::AtkFlat);
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
            assert!(!ids[i + 1..].contains(id), "Duplicate talent buff ID: {id}");
        }
    }

    #[test]
    fn test_find_sucrose_buffs() {
        let buffs = find_talent_buffs("sucrose").unwrap();
        // A1 (EM+50) — at least 1 fixed entry
        assert!(buffs.iter().any(
            |b| b.stat == BuffableStat::ElementalMastery && (b.base_value - 50.0).abs() < 1e-6
        ));
    }

    #[test]
    fn test_find_ganyu_buffs() {
        let buffs = find_talent_buffs("ganyu").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalDmgBonus(Element::Cryo)
        );
        assert!((buffs[0].base_value - 0.20).abs() < 1e-6);
        assert_eq!(buffs[0].target, BuffTarget::Team);
    }

    #[test]
    fn test_find_albedo_buffs() {
        let buffs = find_talent_buffs("albedo").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((buffs[0].base_value - 125.0).abs() < 1e-6);
    }

    #[test]
    fn test_find_ningguang_buffs() {
        let buffs = find_talent_buffs("ningguang").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalDmgBonus(Element::Geo));
        assert!((buffs[0].base_value - 0.12).abs() < 1e-6);
    }

    #[test]
    fn test_find_traveler_dendro_buffs() {
        let buffs = find_talent_buffs("traveler_dendro").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((buffs[0].base_value - 60.0).abs() < 1e-6);
    }

    #[test]
    fn test_find_yoimiya_buffs() {
        let buffs = find_talent_buffs("yoimiya").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::AtkPercent);
        assert!((buffs[0].base_value - 0.20).abs() < 1e-6);
        assert_eq!(buffs[0].target, BuffTarget::TeamExcludeSelf);
    }

    #[test]
    fn test_find_chevreuse_buffs() {
        let buffs = find_talent_buffs("chevreuse").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::AtkPercent);
        assert!((buffs[0].base_value - 0.20).abs() < 1e-6);
    }

    #[test]
    fn test_find_diona_buffs() {
        let buffs = find_talent_buffs("diona").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((buffs[0].base_value - 200.0).abs() < 1e-6);
        assert_eq!(buffs[0].min_constellation, 6);
    }

    #[test]
    fn test_find_amber_buffs() {
        let buffs = find_talent_buffs("amber").unwrap();
        assert_eq!(buffs[0].stat, BuffableStat::AtkPercent);
        assert_eq!(buffs[0].min_constellation, 6);
    }

    #[test]
    fn test_find_barbara_buffs() {
        let buffs = find_talent_buffs("barbara").unwrap();
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalDmgBonus(Element::Hydro)
        );
        assert_eq!(buffs[0].min_constellation, 2);
    }

    #[test]
    fn test_find_sara_c6_buff() {
        let buffs = find_talent_buffs("kujou_sara").unwrap();
        // Original skill buff + C6 CritDmg
        let c6 = buffs
            .iter()
            .find(|b| b.stat == BuffableStat::CritDmg)
            .unwrap();
        assert!((c6.base_value - 0.60).abs() < 1e-6);
        assert_eq!(c6.min_constellation, 6);
    }

    #[test]
    fn test_shenhe_a1_press_buffs() {
        let buffs = find_talent_buffs("shenhe").unwrap();
        // Existing: Spring Spirit Summoning (skill scaling)
        // New: Deific Embrace press = SkillDmgBonus + BurstDmgBonus
        let skill_dmg = buffs.iter().find(|b| b.stat == BuffableStat::SkillDmgBonus);
        let burst_dmg = buffs.iter().find(|b| b.stat == BuffableStat::BurstDmgBonus);
        assert!(
            skill_dmg.is_some(),
            "Should have SkillDmgBonus from A1 press"
        );
        assert!(
            burst_dmg.is_some(),
            "Should have BurstDmgBonus from A1 press"
        );
        assert!((skill_dmg.unwrap().base_value - 0.15).abs() < 1e-6);
        assert!((burst_dmg.unwrap().base_value - 0.15).abs() < 1e-6);
    }

    #[test]
    fn test_thoma_c6_buffs() {
        let buffs = find_talent_buffs("thoma").unwrap();
        assert_eq!(buffs.len(), 3);
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::NormalAtkDmgBonus)
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::ChargedAtkDmgBonus)
        );
        assert!(
            buffs
                .iter()
                .any(|b| b.stat == BuffableStat::PlungingAtkDmgBonus)
        );
        for b in buffs {
            assert!((b.base_value - 0.15).abs() < 1e-6);
            assert_eq!(b.min_constellation, 6);
        }
    }

    #[test]
    fn test_find_faruzan_buffs() {
        let buffs = find_talent_buffs("faruzan").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(
            buffs[0].stat,
            BuffableStat::ElementalDmgBonus(Element::Anemo)
        );
        assert!(buffs[0].scales_with_talent);
        assert!(buffs[0].talent_scaling.is_some());
        assert_eq!(buffs[0].source, TalentBuffSource::ElementalBurst);
    }

    #[test]
    fn test_faruzan_burst_scaling_lv13() {
        let buffs = find_talent_buffs("faruzan").unwrap();
        let scaling = buffs[0].talent_scaling.unwrap();
        // Verify Lv13 value is positive
        assert!(scaling[12] > 0.0, "Lv13 scaling should be positive");
    }

    #[test]
    fn test_find_candace_buffs() {
        let buffs = find_talent_buffs("candace").unwrap();
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::NormalAtkDmgBonus);
        assert!(buffs[0].scales_with_talent);
        assert_eq!(buffs[0].source, TalentBuffSource::ElementalBurst);
    }

    #[test]
    fn test_find_gorou_buffs() {
        let buffs = find_talent_buffs("gorou").unwrap();
        assert_eq!(buffs.len(), 2);
        // DefFlat scaling entry
        let def_buff = buffs
            .iter()
            .find(|b| b.stat == BuffableStat::DefFlat)
            .unwrap();
        assert!(def_buff.scales_with_talent);
        assert_eq!(def_buff.source, TalentBuffSource::ElementalSkill);
        // Geo DMG fixed entry
        let geo_buff = buffs
            .iter()
            .find(|b| b.stat == BuffableStat::ElementalDmgBonus(Element::Geo))
            .unwrap();
        assert!((geo_buff.base_value - 0.15).abs() < 1e-6);
        assert!(!geo_buff.scales_with_talent);
    }
}
