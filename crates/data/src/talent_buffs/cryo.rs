use super::*;

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
    cap: None,
    activation: None,
}];

// ===== Ganyu =====
// A4 passive "Harmony between Heaven and Earth": Cryo DMG+20% in burst field
// C4 "Westward Sojourn": DMG+5% every 3s in burst field (max +25%)
static GANYU_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Harmony between Heaven and Earth",
        description: "Cryo DMG Bonus +20% for party members in burst field",
        stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Westward Sojourn DmgBonus",
        description: "C4: In burst field, DMG+5% every 3s (max +25%, adopting max value)",
        stat: BuffableStat::DmgBonus,
        base_value: 0.25,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: None,
    },
];

// ===== Rosaria =====
// A4 passive "Shadow Samaritan": grants 15% of Rosaria's CRIT Rate to party after burst
// C6 "Rites of Termination": Physical RES -20% for 10s after burst
static ROSARIA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Shadow Samaritan CRIT Rate Share",
        description: "After burst, grants 15% of Rosaria's CRIT Rate to party (max 15%)",
        stat: BuffableStat::CritRate,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::CritRate),
        target: BuffTarget::TeamExcludeSelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(0.15),
        activation: None,
    },
    TalentBuffDef {
        name: "Rites of Termination Physical RES Shred",
        description: "C6: Enemies hit by burst have Physical RES -20% for 10s",
        stat: BuffableStat::PhysicalResReduction,
        base_value: 0.20,
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

// ===== Shenhe =====
// Elemental Skill "Spring Spirit Summoning": flat Cryo DMG based on ATK (Lv1-15)
static SHENHE_SKILL_SCALING: [f64; 15] = [
    0.4566, 0.4909, 0.5251, 0.5708, 0.6050, 0.6393, 0.6849, 0.7306, 0.7763, 0.8219, 0.8676, 0.9132,
    0.9703, 1.0274, 1.0844,
];

static SHENHE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Spring Spirit Summoning Normal ATK Flat DMG",
        description: "Adds flat Cryo DMG based on Shenhe's ATK to party's Normal Attacks",
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&SHENHE_SKILL_SCALING),
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(5))),
    },
    TalentBuffDef {
        name: "Spring Spirit Summoning Charged ATK Flat DMG",
        description: "Adds flat Cryo DMG based on Shenhe's ATK to party's Charged Attacks",
        stat: BuffableStat::ChargedAtkFlatDmg,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&SHENHE_SKILL_SCALING),
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(5))),
    },
    TalentBuffDef {
        name: "Spring Spirit Summoning Plunging ATK Flat DMG",
        description: "Adds flat Cryo DMG based on Shenhe's ATK to party's Plunging Attacks",
        stat: BuffableStat::PlungingAtkFlatDmg,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&SHENHE_SKILL_SCALING),
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(5))),
    },
    TalentBuffDef {
        name: "Spring Spirit Summoning Skill Flat DMG",
        description: "Adds flat Cryo DMG based on Shenhe's ATK to party's Elemental Skills",
        stat: BuffableStat::SkillFlatDmg,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&SHENHE_SKILL_SCALING),
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(5))),
    },
    TalentBuffDef {
        name: "Spring Spirit Summoning Burst Flat DMG",
        description: "Adds flat Cryo DMG based on Shenhe's ATK to party's Elemental Bursts",
        stat: BuffableStat::BurstFlatDmg,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&SHENHE_SKILL_SCALING),
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(5))),
    },
    // A4 "Deific Embrace" Press E: Skill/Burst DMG +15%
    TalentBuffDef {
        name: "Deific Embrace Press - Skill DMG",
        description: "After press E, party Skill DMG +15% for 10s",
        stat: BuffableStat::SkillDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
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
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    // A4 "Deific Embrace" Hold E: Normal/Charged/Plunging ATK DMG +15%
    TalentBuffDef {
        name: "Deific Embrace Hold - Normal ATK DMG",
        description: "After hold E, party Normal ATK DMG +15% for 15s",
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Deific Embrace Hold - Charged ATK DMG",
        description: "After hold E, party Charged ATK DMG +15% for 15s",
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Deific Embrace Hold - Plunging ATK DMG",
        description: "After hold E, party Plunging ATK DMG +15% for 15s",
        stat: BuffableStat::PlungingAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Citlali =====
// Skill: Pyro/Hydro RES -20%
// C2 "Heart Devourer's Travail": Party members (excl. Citlali) gain EM+250 while Opal Shield active or Itzpapa following
// C2: Additional Pyro/Hydro RES -20% (total -40% each)
static CITLALI_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Itzpapa Pyro RES Shred",
        description: "Skill: Enemy Pyro RES -20% while Itzpapa is active",
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Itzpapa Hydro RES Shred",
        description: "Skill: Enemy Hydro RES -20% while Itzpapa is active",
        stat: BuffableStat::ElementalResReduction(Element::Hydro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Heart Devourer's Travail EM Bonus",
        description: "C2: While Opal Shield is active or Itzpapa is following, nearby party members (excl. Citlali) gain EM+250",
        stat: BuffableStat::ElementalMastery,
        base_value: 250.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::TeamExcludeSelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Cold Moon Pyro RES Shred",
        description: "C2: Additional Pyro RES -20% (cumulative with Skill: total -40%)",
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Cold Moon Hydro RES Shred",
        description: "C2: Additional Hydro RES -20% (cumulative with Skill: total -40%)",
        stat: BuffableStat::ElementalResReduction(Element::Hydro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Eula =====
// Skill (hold): Cryo RES -50% + Physical RES -50% (max 2 Grimheart stacks)
static EULA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Icetide Vortex Cryo RES Shred",
        description: "Hold Skill: Enemy Cryo RES -25% per Grimheart stack (max -50%)",
        stat: BuffableStat::ElementalResReduction(Element::Cryo),
        base_value: 0.50,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(2))),
    },
    TalentBuffDef {
        name: "Icetide Vortex Physical RES Shred",
        description: "Hold Skill: Enemy Physical RES -25% per Grimheart stack (max -50%)",
        stat: BuffableStat::PhysicalResReduction,
        base_value: 0.50,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(2))),
    },
];

// ===== Mika =====
// C6: Physical DMG Bonus +10% during burst recovery (HP>50%)
static MIKA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Blood-Red Bristles Physical DMG Bonus",
    description: "C6: During Skyfeather Song recovery, Physical DMG +10% (HP>50%)",
    stat: BuffableStat::PhysicalDmgBonus,
    base_value: 0.10,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: None,
}];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static CRYO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("citlali", CITLALI_BUFFS),
    ("diona", DIONA_BUFFS),
    ("eula", EULA_BUFFS),
    ("ganyu", GANYU_BUFFS),
    ("mika", MIKA_BUFFS),
    ("rosaria", ROSARIA_BUFFS),
    ("shenhe", SHENHE_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    CRYO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}
