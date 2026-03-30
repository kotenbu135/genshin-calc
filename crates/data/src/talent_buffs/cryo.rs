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

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static CRYO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("diona", DIONA_BUFFS),
    ("ganyu", GANYU_BUFFS),
    ("rosaria", ROSARIA_BUFFS),
    ("shenhe", SHENHE_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    CRYO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}
