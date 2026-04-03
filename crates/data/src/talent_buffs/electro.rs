use super::*;

// ===== Ineffa =====
// A4 passive: EM share based on ATK (builder computes ATK * 0.06)
static INEFFA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Ineffa A4 EM Share",
    description: "Grants EM equal to 6% of Ineffa's ATK (builder computes ATK * 0.06)",
    stat: BuffableStat::ElementalMastery,
    base_value: 0.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
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
        description:
            "Electro CRIT DMG +60% (approximated as generic CritDmg; Electro-only in game)",
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

// ===== Lisa =====
// A4 "Static Electricity Field": Enemies hit by burst have DEF -15% for 10s
static LISA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Static Electricity Field",
    description: "Enemies hit by Lightning Rose have DEF -15% for 10s",
    stat: BuffableStat::DefReduction,
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Flins =====
// A4 passive "Whispering Flame": EM += ATK × 0.08, capped at 160 (computed at resolve time)
// C4 "Night on Bald Mountain": ATK +20%
static FLINS_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Whispering Flame EM Bonus",
        description: "A4: Flins gains EM equal to 8% of her ATK (max 160)",
        stat: BuffableStat::ElementalMastery,
        base_value: 0.0, // ATK-dependent — builder computes ATK×0.08 (cap 160) at resolve time
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Night on Bald Mountain ATK Bonus",
        description: "C4: Flins gains ATK +20%",
        stat: BuffableStat::AtkPercent,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
    },
];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static ELECTRO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("ineffa", INEFFA_BUFFS),
    ("kujou_sara", SARA_BUFFS),
    ("lisa", LISA_BUFFS),
    ("flins", FLINS_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    ELECTRO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}
