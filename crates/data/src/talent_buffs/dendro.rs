use super::*;

// ===== Lauma =====
// A4 passive "Cleansing for the Spring": Elemental Skill DMG +0.04% per point of EM (max +32% at 800 EM)
static LAUMA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Cleansing for the Spring",
    description: "A4: Elemental Skill DMG +0.04% per point of EM (max +32% at 800 EM)",
    stat: BuffableStat::SkillDmgBonus,
    base_value: 0.0004,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: Some(ScalingStat::Em),
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::AscensionPassive(4),
    min_constellation: 0,
    cap: Some(0.32),
}];

// ===== Nahida =====
// A1 passive "Compassion Illuminated": grants highest_party_EM × 25%, max 250
// Note: Game uses highest party EM; single-character eval uses own EM (known simplification).
// The -200 offset previously reported does not exist (confirmed by Fandom + KQM).
static NAHIDA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Compassion Illuminated",
    description: "A1: Grants EM to party = highest_party_EM × 25%, max 250",
    stat: BuffableStat::ElementalMastery,
    base_value: 0.25,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: Some(ScalingStat::Em),
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive(1),
    min_constellation: 0,
    cap: Some(250.0),
}];

// ===== Nefer =====
// C2 "Observation Feeds Strategy": EM +200 at 5 Veil stacks (Toggle)
// C4 "Delusion Ensnares Reason": Dendro RES -20% during Shadow Dance (Toggle)
static NEFER_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Observation Feeds Strategy EM Bonus",
        description: "C2: EM +200 at 5 Veil of Falsehood stacks",
        stat: BuffableStat::ElementalMastery,
        base_value: 200.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
    },
    TalentBuffDef {
        name: "Delusion Ensnares Reason Dendro RES Down",
        description: "C4: Enemy Dendro RES -20% during Shadow Dance",
        stat: BuffableStat::ElementalResReduction(Element::Dendro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
    },
];

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
    source: TalentBuffSource::AscensionPassive(4),
    min_constellation: 0,
    cap: None,
}];

// ===== Collei =====
// C4: EM+60 in Cuilein-Anbar field
static COLLEI_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Floral Sidewinder EM Bonus",
    description: "C4: Party gains EM+60 while inside Cuilein-Anbar field",
    stat: BuffableStat::ElementalMastery,
    base_value: 60.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
}];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static DENDRO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("collei", COLLEI_BUFFS),
    ("lauma", LAUMA_BUFFS),
    ("nahida", NAHIDA_BUFFS),
    ("nefer", NEFER_BUFFS),
    ("traveler_dendro", TRAVELER_DENDRO_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    DENDRO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}
