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
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Aino A4 Burst DMG from EM",
        description: "Burst DMG increased by 50% of Elemental Mastery",
        stat: BuffableStat::BurstFlatDmg,
        base_value: 0.50,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    // C6 "The Burden of Creative Genius": Reaction DMG bonus varies by Moonsign level.
    // Lv1 (NascentGleam): +15%, Lv2 (AscendantGleam): +35%
    // Moved to MoonsignTalentEnhancement in moonsign_chars.rs (not a fixed talent buff).
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
    cap: None,
    activation: None,
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
    cap: None,
    activation: None,
}];

// ===== Furina =====
// Elemental Burst "Let the People Rejoice": Fanfare stacks grant DMG bonus
// Per-point DMG Bonus (Lv1-15): 0.07%, 0.09%, ..., 0.35%
// C0 max fanfare: 300pt, C1+ max fanfare: 400pt (C1 adds +100)
// Split into two defs: C0 base (300 × per_point) + C1 extra (100 × per_point)
static FURINA_BURST_C0_MAX: [f64; 15] = [
    0.21, 0.27, 0.33, 0.39, 0.45, 0.51, 0.57, 0.63, 0.69, 0.75, 0.81, 0.87, 0.93, 0.99, 1.05,
];
static FURINA_BURST_C1_EXTRA: [f64; 15] = [
    0.07, 0.09, 0.11, 0.13, 0.15, 0.17, 0.19, 0.21, 0.23, 0.25, 0.27, 0.29, 0.31, 0.33, 0.35,
];

static FURINA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Let the People Rejoice DMG Bonus (C0 300pt)",
        description: "Max fanfare (300pt) DMG bonus based on burst talent level",
        stat: BuffableStat::DmgBonus,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&FURINA_BURST_C0_MAX),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(300))),
    },
    TalentBuffDef {
        name: "Let the People Rejoice DMG Bonus (C1+ extra 100pt)",
        description: "C1 extra fanfare (+100pt) DMG bonus based on burst talent level",
        stat: BuffableStat::DmgBonus,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&FURINA_BURST_C1_EXTRA),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(300))),
    },
];

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
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
}];

// ===== Nilou =====
// A2 passive "Dreaming Dance of the Lotuslight":
// For every 1000 HP above 30000, Bloom DMG +9%, max +400%
// Formula: min(floor((total_hp - 30000) / 1000) * 0.09, 4.0)
static NILOU_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Dreaming Dance of the Lotuslight",
    description: "HP30000超過1000ごとにBloom DMG+9%、最大+400%",
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.0, // HP-dependent special formula (scaling_value == 0.0 triggers it)
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: Some(ScalingStat::Hp),
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::AscensionPassive(4),
    min_constellation: 0,
    cap: Some(4.0),
    activation: None,
}];

// ===== Yelan =====
// A4 passive "Adapt With Ease": DMG bonus ramp 1-50% (max value 0.50)
static YELAN_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Adapt With Ease",
    description: "DMG Bonus ramps up to max value 0.50",
    stat: BuffableStat::DmgBonus,
    base_value: 0.50,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive(4),
    min_constellation: 0,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
}];

// ===== Columbina =====
// Each constellation grants passive Lunar Reaction DMG bonus (TransformativeBonus)
// C1: +1.5%, C2: +7%, C3: +1.5%, C4: +1.5%, C5: +1.5%, C6: +7%
// C2 also: Lunar Brilliance — HP+40% for 8s
// C6 also: Corresponding elemental CritDMG +80% for 8s
static COLUMBINA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Columbina C1 Lunar Reaction DMG",
        description: "C1: Lunar Reaction DMG +1.5%",
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.015,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Columbina C2 Lunar Brilliance HP",
        description: "C2: Lunar Brilliance — Columbina HP +40% for 8s (self only)",
        stat: BuffableStat::HpPercent,
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Columbina C2 Lunar Reaction DMG",
        description: "C2: Lunar Reaction DMG +7%",
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.07,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Columbina C3 Lunar Reaction DMG",
        description: "C3: Lunar Reaction DMG +1.5%",
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.015,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(3),
        min_constellation: 3,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Columbina C4 Lunar Reaction DMG",
        description: "C4: Lunar Reaction DMG +1.5%",
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.015,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Columbina C5 Lunar Reaction DMG",
        description: "C5: Lunar Reaction DMG +1.5%",
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.015,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(5),
        min_constellation: 5,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Columbina C6 Elemental CritDMG",
        description: "After Lunar reaction in domain, corresponding element CritDMG +80% for 8s",
        stat: BuffableStat::CritDmg,
        base_value: 0.80,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Columbina C6 Lunar Reaction DMG",
        description: "C6: Lunar Reaction DMG +7%",
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.07,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: None,
    },
];

// ===== Xingqiu =====
// C2: Hydro RES -15% on Rain Sword hit
static XINGQIU_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Rainbow Upon the Azure Sky Hydro RES Shred",
    description: "C2: Enemies hit by Rain Swords have Hydro RES -15% for 4s",
    stat: BuffableStat::ElementalResReduction(Element::Hydro),
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: None,
}];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static HYDRO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("aino", AINO_BUFFS),
    ("barbara", BARBARA_BUFFS),
    ("candace", CANDACE_BUFFS),
    ("columbina", COLUMBINA_BUFFS),
    ("furina", FURINA_BUFFS),
    ("mona", MONA_BUFFS),
    ("nilou", NILOU_BUFFS),
    ("xingqiu", XINGQIU_BUFFS),
    ("yelan", YELAN_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    HYDRO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}
