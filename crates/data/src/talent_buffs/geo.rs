use super::*;

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

// ===== Zhongli =====
// Jade Shield "Dominus Lapidis": All RES -20% for nearby enemies while shield is active
static ZHONGLI_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Jade Shield Pyro RES Shred",
        description: "Nearby enemies' Pyro RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Hydro RES Shred",
        description: "Nearby enemies' Hydro RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Hydro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Electro RES Shred",
        description: "Nearby enemies' Electro RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Electro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Cryo RES Shred",
        description: "Nearby enemies' Cryo RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Cryo),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Dendro RES Shred",
        description: "Nearby enemies' Dendro RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Dendro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Anemo RES Shred",
        description: "Nearby enemies' Anemo RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Anemo),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Geo RES Shred",
        description: "Nearby enemies' Geo RES -20%",
        stat: BuffableStat::ElementalResReduction(Element::Geo),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Jade Shield Physical RES Shred",
        description: "Nearby enemies' Physical RES -20%",
        stat: BuffableStat::PhysicalResReduction,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
    },
];

// ===== Zibai =====
// C2 "At Birth Are Souls Born, and in Death Leave But Husks":
// Lunar-Crystallize Reaction DMG +30% for nearby party members
static ZIBAI_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "At Birth Are Souls Born C2 Reaction DMG",
    description: "C2: Lunar-Crystallize Reaction DMG +30% for nearby party members",
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.30,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
}];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static GEO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("albedo", ALBEDO_BUFFS),
    ("gorou", GOROU_BUFFS),
    ("ningguang", NINGGUANG_BUFFS),
    ("yun_jin", YUN_JIN_BUFFS),
    ("zhongli", ZHONGLI_BUFFS),
    ("zibai", ZIBAI_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    GEO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}
