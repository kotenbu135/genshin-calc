use super::*;

// ===== Aino =====
// C1: EM+80 to Aino and active party member after skill/burst for 15s
// A4: Structured Power Booster — Burst DMG increased by 50% of EM
// C6: The Burden of Creative Genius — Reaction DMG +15% for 15s after Burst
static AINO_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Aino C1 EM Share",
        description: "After Skill or Burst, Aino and active character gain EM+80 for 15s",
        stat: BuffableStat::ElementalMastery,
        base_value: 80.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
    },
    TalentBuffDef {
        name: "Aino A4 Burst DMG from EM",
        description: "Burst DMG increased by 50% of Elemental Mastery",
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.50,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Aino C6 Reaction DMG Bonus",
        description: "After Burst, nearby active characters' reaction DMG +15% for 15s",
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
];

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

// ===== Nilou =====
// A2 passive "Dreaming Dance of the Lotuslight":
// For every 1000 HP above 30000, Bloom DMG +9%, max +400%
// Formula: min(floor((total_hp - 30000) / 1000) * 0.09, 4.0)
static NILOU_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Dreaming Dance of the Lotuslight",
    description: "HP30000超過1000ごとにBloom DMG+9%、最大+400%",
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.0, // HP-dependent — consumer computes value
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Yelan =====
// A4 passive "Adapt With Ease": DMG bonus ramp 1-50% (builder sets desired value)
static YELAN_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Adapt With Ease",
    description: "DMG Bonus ramps 1-50% over time (builder sets value, max 0.50)",
    stat: BuffableStat::DmgBonus,
    base_value: 0.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static HYDRO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("aino", AINO_BUFFS),
    ("barbara", BARBARA_BUFFS),
    ("candace", CANDACE_BUFFS),
    ("furina", FURINA_BUFFS),
    ("mona", MONA_BUFFS),
    ("nilou", NILOU_BUFFS),
    ("yelan", YELAN_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    HYDRO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}
