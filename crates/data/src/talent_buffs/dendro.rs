use super::*;

// ===== Lauma =====
// A4 passive "Cleansing for the Spring": Elemental Skill DMG +0.04% per point of EM (max +32% at 800 EM)
static LAUMA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Cleansing for the Spring",
    description: "A4: Elemental Skill DMG +0.04% per point of EM (builder computes EM×0.0004 at resolve time, max +32% at 800 EM)",
    stat: BuffableStat::SkillDmgBonus,
    base_value: 0.0, // EM-dependent — builder computes EM×0.0004 at resolve time
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
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
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static DENDRO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
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
