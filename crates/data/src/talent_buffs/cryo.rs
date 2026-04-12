use super::*;

// ===== Diona =====
// C6 "Cat's Tail Closing Time": EM+200 in burst field
static DIONA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Cat's Tail Closing Time",
    description: desc!("Characters within burst field gain EM+200"),
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
// C1 "Dew-Drinker": Frostflake Arrow hit reduces Cryo RES -15%
// C4 "Westward Sojourn": DMG+5% every 3s in burst field (max +25%)
static GANYU_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Harmony between Heaven and Earth",
        description: desc!("Cryo DMG Bonus +20% for party members in burst field"),
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
        description: desc!("C4: In burst field, DMG+5% every 3s (max +25%, adopting max value)"),
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
    TalentBuffDef {
        name: "ganyu_c1_cryo_res_shred",
        description: desc!("C1: Frostflake Arrow hit reduces Cryo RES -15%"),
        stat: BuffableStat::ElementalResReduction(Element::Cryo),
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Qiqi =====
// C2 "Frozen to the Bone": Normal/Charged ATK DMG +15% vs Cryo-affected opponents
static QIQI_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "qiqi_c2_normal_dmg",
        description: desc!("C2: Normal ATK DMG +15% vs Cryo-affected opponents"),
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "qiqi_c2_charged_dmg",
        description: desc!("C2: Charged ATK DMG +15% vs Cryo-affected opponents"),
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Rosaria =====
// A4 passive "Shadow Samaritan": grants 15% of Rosaria's CRIT Rate to party after burst
// C1 "Unholy Revelation": Normal ATK DMG +10% on CRIT hit
// C6 "Rites of Termination": Physical RES -20% for 10s after burst
static ROSARIA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Shadow Samaritan CRIT Rate Share",
        description: desc!("After burst, grants 15% of Rosaria's CRIT Rate to party (max 15%)"),
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
        description: desc!("C6: Enemies hit by burst have Physical RES -20% for 10s"),
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
    TalentBuffDef {
        name: "rosaria_c1_normal_dmg",
        description: desc!("C1: Normal ATK DMG +10% on CRIT hit"),
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Shenhe =====
// Elemental Skill "Spring Spirit Summoning": flat Cryo DMG based on ATK (Lv1-15)
// Elemental Burst "Divine Maiden's Deliverance": Cryo/Physical RES shred (talent-level dependent)
// Lv1-9: 6-14%, Lv10+: 15%
static SHENHE_SKILL_SCALING: [f64; 15] = [
    0.4566, 0.4908, 0.5250, 0.5707, 0.6049, 0.6392, 0.6848, 0.7305, 0.7762, 0.8218, 0.8675, 0.9131,
    0.9702, 1.0273, 1.0843,
];

static SHENHE_RES_SHRED_SCALING: [f64; 15] = [
    0.06, 0.07, 0.08, 0.09, 0.10, 0.11, 0.12, 0.13, 0.14, 0.15, 0.15, 0.15, 0.15, 0.15, 0.15,
];

static SHENHE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Deific Embrace",
        description: desc!("Active character in burst field gains Cryo DMG Bonus +15%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Spring Spirit Summoning Normal ATK Flat DMG",
        description: desc!("Adds flat Cryo DMG based on Shenhe's ATK to party's Normal Attacks"),
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
        description: desc!("Adds flat Cryo DMG based on Shenhe's ATK to party's Charged Attacks"),
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
        description: desc!("Adds flat Cryo DMG based on Shenhe's ATK to party's Plunging Attacks"),
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
        description: desc!("Adds flat Cryo DMG based on Shenhe's ATK to party's Elemental Skills"),
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
        description: desc!("Adds flat Cryo DMG based on Shenhe's ATK to party's Elemental Bursts"),
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
    // A4 "Spirit Communion Seal" Press E: Skill/Burst DMG +15%
    // Legacy buff names retained to avoid breaking existing activations/tests.
    TalentBuffDef {
        name: "Deific Embrace Press - Skill DMG",
        description: desc!("After press E, party Skill DMG +15% for 10s"),
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
        description: desc!("After press E, party Burst DMG +15% for 10s"),
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
    // A4 "Spirit Communion Seal" Hold E: Normal/Charged/Plunging ATK DMG +15%
    TalentBuffDef {
        name: "Deific Embrace Hold - Normal ATK DMG",
        description: desc!("After hold E, party Normal ATK DMG +15% for 15s"),
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
        description: desc!("After hold E, party Charged ATK DMG +15% for 15s"),
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
        description: desc!("After hold E, party Plunging ATK DMG +15% for 15s"),
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
    // C2 "Centered Spirit": Cryo CRIT DMG +15% in Burst field
    TalentBuffDef {
        name: "shenhe_c2_cryo_crit_dmg",
        description: desc!("C2: Cryo CRIT DMG +15% in Burst field"),
        stat: BuffableStat::ElementalCritDmg(Element::Cryo),
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    // Burst "Divine Maiden's Deliverance": Cryo/Physical RES shred (talent-level dependent)
    TalentBuffDef {
        name: "Divine Maiden's Deliverance Cryo RES Shred",
        description: desc!("Burst field: Enemy Cryo RES shred (talent-level dependent)"),
        stat: BuffableStat::ElementalResReduction(Element::Cryo),
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&SHENHE_RES_SHRED_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Divine Maiden's Deliverance Physical RES Shred",
        description: desc!("Burst field: Enemy Physical RES shred (talent-level dependent)"),
        stat: BuffableStat::PhysicalResReduction,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&SHENHE_RES_SHRED_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Citlali =====
// Skill: Pyro/Hydro RES -20%
// C2 "Heart Devourer's Travail": Party members (excl. Citlali) gain EM+250 while Opal Shield active or Itzpapa following
// C2: Additional Pyro/Hydro RES -20% (total -40% each)
// C6 "Secret Pact of the Primal Flame": Mystic Counts grant Pyro/Hydro DMG +1.5%/count (max 40 = +60%)
static CITLALI_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Itzpapa Pyro RES Shred",
        description: desc!("Skill: Enemy Pyro RES -20% while Itzpapa is active"),
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
        description: desc!("Skill: Enemy Hydro RES -20% while Itzpapa is active"),
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
        description: desc!(
            "C2: While Opal Shield is active or Itzpapa is following, nearby party members (excl. Citlali) gain EM+250"
        ),
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
        description: desc!("C2: Additional Pyro RES -20% (cumulative with Skill: total -40%)"),
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
        description: desc!("C2: Additional Hydro RES -20% (cumulative with Skill: total -40%)"),
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
    TalentBuffDef {
        name: "Secret Pact Pyro DMG Bonus",
        description: desc!(
            "C6: Mystic Counts grant Pyro DMG +1.5% per count (max 40 counts = +60%)"
        ),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.015,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(40))),
    },
    TalentBuffDef {
        name: "Secret Pact Hydro DMG Bonus",
        description: desc!(
            "C6: Mystic Counts grant Hydro DMG +1.5% per count (max 40 counts = +60%)"
        ),
        stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
        base_value: 0.015,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(40))),
    },
];

// ===== Eula =====
// Skill (hold): Cryo/Physical RES shred per Grimheart stack (max 2), talent-level dependent
// Lv1-9: 16-24%, Lv10+: 25%
// C1 "Dance of the One in Bliss": Physical DMG Bonus +30% on Grimheart consume
// C4 "Gleaming Tempest": Burst DMG +25% vs opponents below 50% HP
static EULA_RES_SHRED_SCALING: [f64; 15] = [
    0.16, 0.17, 0.18, 0.19, 0.20, 0.21, 0.22, 0.23, 0.24, 0.25, 0.25, 0.25, 0.25, 0.25, 0.25,
];

static EULA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Icetide Vortex Cryo RES Shred",
        description: desc!(
            "Hold Skill: Enemy Cryo RES shred per Grimheart stack (max 2, talent-level dependent)"
        ),
        stat: BuffableStat::ElementalResReduction(Element::Cryo),
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&EULA_RES_SHRED_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(2))),
    },
    TalentBuffDef {
        name: "Icetide Vortex Physical RES Shred",
        description: desc!(
            "Hold Skill: Enemy Physical RES shred per Grimheart stack (max 2, talent-level dependent)"
        ),
        stat: BuffableStat::PhysicalResReduction,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&EULA_RES_SHRED_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(2))),
    },
    TalentBuffDef {
        name: "eula_c1_physical_dmg",
        description: desc!("C1: Physical DMG Bonus +30% on Grimheart consume"),
        stat: BuffableStat::PhysicalDmgBonus,
        base_value: 0.30,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "eula_c4_burst_dmg",
        description: desc!("C4: Burst DMG +25% vs opponents below 50% HP"),
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.25,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Mika =====
// C6 "Blood-Red Bristles": Physical CRIT DMG +60%
static MIKA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "mika_c6_physical_crit_dmg",
        description: desc!("C6: Physical CRIT DMG +60%"),
        stat: BuffableStat::PhysicalCritDmg,
        base_value: 0.60,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Suppressive Barrage",
        description: desc!(
            "Soulwind Detector stacks grant on-field characters Physical DMG +10% per stack"
        ),
        stat: BuffableStat::PhysicalDmgBonus,
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
    },
];

// ===== Aloy =====
// A1 "Easy Does It": ATK+16% (self) and ATK+8% (team excl. self) after receiving Coil stacks
// A4 "Strong Strike": Cryo DMG+3.5%/s in Rushing Ice state, max +35% (adopting max value)
static ALOY_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Easy Does It ATK Self",
        description: desc!("A1: After receiving Coil stacks, Aloy's ATK +16%"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.16,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Easy Does It ATK Team",
        description: desc!(
            "A1: After receiving Coil stacks, nearby party members (excl. Aloy) gain ATK+8%"
        ),
        stat: BuffableStat::AtkPercent,
        base_value: 0.08,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::TeamExcludeSelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Strong Strike Cryo DMG",
        description: desc!(
            "A4: In Rushing Ice state, Cryo DMG+3.5%/s (max +35%, adopting max value)"
        ),
        stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
        base_value: 0.35,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(0.35),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Ayaka =====
// A1 "Amatsumi Kunitsumi Sanctification": NA+CA DMG+30% for 6s after using Skill
// A4 "Kanten Senmyou Blessing": Cryo DMG+18% for 10s after burst
// C4 "Ebb and Flow": Enemy DEF-30% for 6s after burst hits
// C6 "Flowers of the Freesia State": CA DMG+298% during burst
static AYAKA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Amatsumi Kunitsumi Sanctification Normal ATK",
        description: desc!("A1: After using Skill, Normal ATK DMG+30% for 6s"),
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.30,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Amatsumi Kunitsumi Sanctification Charged ATK",
        description: desc!("A1: After using Skill, Charged ATK DMG+30% for 6s"),
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.30,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Kanten Senmyou Blessing Cryo DMG",
        description: desc!("A4: After using Burst, Cryo DMG+18% for 10s"),
        stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
        base_value: 0.18,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Ebb and Flow DEF Shred",
        description: desc!("C4: Enemies hit by burst have DEF-30% for 6s"),
        stat: BuffableStat::DefReduction,
        base_value: 0.30,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Flowers of the Freesia State CA DMG",
        description: desc!("C6: During Burst, Charged ATK DMG+298%"),
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 2.98,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Charlotte =====
// C2 "Cherished Discourse": ATK+10% per stack (max 3 stacks) each time Framing: Freezing Point Composition hits
// C4 "Biting Cold": Burst DMG+10%
static CHARLOTTE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Freezing Point Composition Cryo DMG Bonus",
        description: desc!("A4: Cryo DMG Bonus +5% per stack (max 3 stacks)"),
        stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
        base_value: 0.05,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
    },
    TalentBuffDef {
        name: "Cherished Discourse ATK",
        description: desc!("C2: Each time Skill hits, ATK+10% per stack (max 3 stacks)"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
    },
    TalentBuffDef {
        name: "Biting Cold Burst DMG",
        description: desc!("C4: Burst DMG+10%"),
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: None,
    },
];

// ===== Chongyun =====
// A4 "Rimechaser Blade": Enemy Cryo RES-10% after Skill field ends
// C6 "Rally of Four Blades": DMG+15% per Chongyun on team (self)
static CHONGYUN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Rimechaser Blade Cryo RES Shred",
        description: desc!("A4: After Skill field ends, enemy Cryo RES -10% for 8s"),
        stat: BuffableStat::ElementalResReduction(Element::Cryo),
        base_value: 0.10,
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
        name: "Rally of Four Blades DMG Bonus",
        description: desc!("C6: Burst DMG+15% (self)"),
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Escoffier =====
// C1 "Amuse-bouche de Saveur": Team Cryo CD+60%
// C2 "Sauté de Fantôme": Skill flat DMG = 240% of Total ATK
static ESCOFFIER_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "escoffier_c1_cryo_crit_dmg",
        description: desc!("C1: Cryo CRIT DMG +60% (requires 4 Hydro/Cryo party members)"),
        stat: BuffableStat::ElementalCritDmg(Element::Cryo),
        base_value: 0.60,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "escoffier_c2_skill_flat_dmg",
        description: desc!("C2: Skill DMG bonus based on 240% of Total ATK"),
        stat: BuffableStat::SkillFlatDmg,
        base_value: 2.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::TotalAtk),
        target: BuffTarget::TeamExcludeSelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "escoffier_a4_hydro_res_shred",
        description: desc!("A4: Enemy Hydro RES -55% (max approximation, toggle)"),
        stat: BuffableStat::ElementalResReduction(Element::Hydro),
        base_value: 0.55,
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
        name: "escoffier_a4_cryo_res_shred",
        description: desc!("A4: Enemy Cryo RES -55% (max approximation, toggle)"),
        stat: BuffableStat::ElementalResReduction(Element::Cryo),
        base_value: 0.55,
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

// ===== Freminet =====
// A4 "Deepwater Swim": Shatter DMG+40%
// C1 "Dreams of the Foamy Deep": CR+15%
// C4 "Secret Plan to Freeze": ATK+9%/stack (max 2 stacks) on hitting Pers Timer
// C6 "Moment of Waking": CD+12%/stack (max 3 stacks) per Shattering hit
static FREMINET_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Deepwater Swim Shatter DMG",
        description: desc!("A4: Shatter DMG+40%"),
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Dreams of the Foamy Deep CR",
        description: desc!("C1: CR+15%"),
        stat: BuffableStat::CritRate,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Secret Plan to Freeze ATK",
        description: desc!("C4: ATK+9% per stack when Pers Timer hits (max 2 stacks)"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.09,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(2))),
    },
    TalentBuffDef {
        name: "Moment of Waking CD",
        description: desc!("C6: CD+12% per Shattering hit (max 3 stacks)"),
        stat: BuffableStat::CritDmg,
        base_value: 0.12,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
    },
];

// ===== Kaeya =====
// C1 "Excellent Blood": CR+15% vs Cryo-affected enemies (self)
static KAEYA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Excellent Blood CR",
    description: desc!("C1: CR+15% vs Cryo-affected enemies (self)"),
    stat: BuffableStat::CritRate,
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(1),
    min_constellation: 1,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
}];

// ===== Layla =====
// A1 "Like Nascent Light": Shield Str+6%/stack (max 4 stacks) — adopting max value +24%
// A4 "Sweet Slumber Undisturbed": Shooting Stars deal flat DMG = 1.5% HP
// C4 "Starry Illumination": NA+CA flat DMG = 5% HP each
// C6 "Boundless Radiance": Shooting Stars DMG+40%
static LAYLA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Like Nascent Light Shield Strength",
        description: desc!(
            "A1: Shield Str+6%/stack when Night Star is absorbed (max 4 stacks, adopting max +24%)"
        ),
        stat: BuffableStat::ShieldStrength,
        base_value: 0.24,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: Some(0.24),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Sweet Slumber Undisturbed Shooting Star DMG",
        description: desc!("A4: Shooting Stars deal flat DMG = 1.5% of Layla's HP"),
        stat: BuffableStat::SkillFlatDmg,
        base_value: 0.015,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Starry Illumination Normal ATK Flat DMG",
        description: desc!("C4: Normal ATK flat DMG = 5% of Layla's HP"),
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 0.05,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Starry Illumination Charged ATK Flat DMG",
        description: desc!("C4: Charged ATK flat DMG = 5% of Layla's HP"),
        stat: BuffableStat::ChargedAtkFlatDmg,
        base_value: 0.05,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Boundless Radiance Shooting Star DMG",
        description: desc!("C6: Shooting Stars DMG+40%"),
        stat: BuffableStat::SkillDmgBonus,
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Boundless Radiance Burst DMG",
        description: desc!("C6: Burst DMG+40%"),
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Skirk =====
// C2 "Liminal Crossing": ATK+70% (self)
// C4 "Fractured Boundary": ATK+40% max (Stacks(3) simplified to Toggle max, self)
static SKIRK_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Liminal Crossing ATK",
        description: desc!("C2: ATK+70% (self)"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.70,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Fractured Boundary ATK",
        description: desc!("C4: ATK+10/20/40% based on stacks (adopting max +40%, self)"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Wriothesley =====
// A4 "There Shall Be a Plea for Justice": ATK+6%/stack (max 5 stacks = +30%) on consecutive CA hits
// C6 "Pax Perpetua": CR+10% and CD+80% during Darkgold state
static WRIOTHESLEY_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "There Shall Be a Plea for Justice ATK",
        description: desc!("A4: ATK+6% per stack on consecutive CA hits (max 5 stacks = +30%)"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.06,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(5))),
    },
    TalentBuffDef {
        name: "Pax Perpetua CR",
        description: desc!("C6: During Darkgold state, CR+10%"),
        stat: BuffableStat::CritRate,
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Pax Perpetua CD",
        description: desc!("C6: During Darkgold state, CD+80%"),
        stat: BuffableStat::CritDmg,
        base_value: 0.80,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static CRYO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("aloy", ALOY_BUFFS),
    ("ayaka", AYAKA_BUFFS),
    ("charlotte", CHARLOTTE_BUFFS),
    ("chongyun", CHONGYUN_BUFFS),
    ("citlali", CITLALI_BUFFS),
    ("diona", DIONA_BUFFS),
    ("escoffier", ESCOFFIER_BUFFS),
    ("eula", EULA_BUFFS),
    ("freminet", FREMINET_BUFFS),
    ("ganyu", GANYU_BUFFS),
    ("kaeya", KAEYA_BUFFS),
    ("layla", LAYLA_BUFFS),
    ("mika", MIKA_BUFFS),
    ("qiqi", QIQI_BUFFS),
    ("rosaria", ROSARIA_BUFFS),
    ("shenhe", SHENHE_BUFFS),
    ("skirk", SKIRK_BUFFS),
    ("wriothesley", WRIOTHESLEY_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    CRYO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_cryo_remaining_items_part_one() {
        let charlotte = find("charlotte").unwrap();
        assert_eq!(charlotte.len(), 3);
        assert!(charlotte.iter().any(|b| {
            b.source == TalentBuffSource::AscensionPassive(4)
                && b.stat == BuffableStat::ElementalDmgBonus(Element::Cryo)
                && (b.base_value - 0.05).abs() < 1e-6
                && b.target == BuffTarget::OnlySelf
                && matches!(
                    b.activation,
                    Some(Activation::Manual(ManualCondition::Stacks(3)))
                )
        }));

        let chongyun = find("chongyun").unwrap();
        assert_eq!(chongyun.len(), 2);
        assert!(chongyun.iter().any(|b| {
            b.source == TalentBuffSource::Constellation(6)
                && b.stat == BuffableStat::BurstDmgBonus
                && (b.base_value - 0.15).abs() < 1e-6
                && b.target == BuffTarget::OnlySelf
                && matches!(
                    b.activation,
                    Some(Activation::Manual(ManualCondition::Toggle))
                )
        }));

        let citlali = find("citlali").unwrap();
        assert_eq!(citlali.len(), 7);

        let escoffier = find("escoffier").unwrap();
        assert_eq!(escoffier.len(), 4);
        assert!(
            !escoffier
                .iter()
                .any(|b| b.name == "Amuse-bouche de Saveur Cryo CD" && b.activation.is_none())
        );
        assert!(escoffier.iter().any(|b| {
            b.name == "escoffier_c1_cryo_crit_dmg"
                && b.source == TalentBuffSource::Constellation(1)
                && b.stat == BuffableStat::ElementalCritDmg(Element::Cryo)
                && matches!(
                    b.activation,
                    Some(Activation::Manual(ManualCondition::Toggle))
                )
        }));
        assert!(escoffier.iter().any(|b| {
            b.source == TalentBuffSource::AscensionPassive(4)
                && b.stat == BuffableStat::ElementalResReduction(Element::Hydro)
                && (b.base_value - 0.55).abs() < 1e-6
                && b.target == BuffTarget::Team
                && matches!(
                    b.activation,
                    Some(Activation::Manual(ManualCondition::Toggle))
                )
        }));
        assert!(escoffier.iter().any(|b| {
            b.source == TalentBuffSource::AscensionPassive(4)
                && b.stat == BuffableStat::ElementalResReduction(Element::Cryo)
                && (b.base_value - 0.55).abs() < 1e-6
                && b.target == BuffTarget::Team
                && matches!(
                    b.activation,
                    Some(Activation::Manual(ManualCondition::Toggle))
                )
        }));
    }

    #[test]
    fn audit_cryo_remaining_items_part_two() {
        let layla = find("layla").unwrap();
        assert_eq!(layla.len(), 6);
        assert!(layla.iter().any(|b| {
            b.source == TalentBuffSource::Constellation(6)
                && b.stat == BuffableStat::BurstDmgBonus
                && (b.base_value - 0.40).abs() < 1e-6
                && b.target == BuffTarget::OnlySelf
                && matches!(
                    b.activation,
                    Some(Activation::Manual(ManualCondition::Toggle))
                )
        }));

        let mika = find("mika").unwrap();
        assert_eq!(mika.len(), 2);
        assert!(mika.iter().any(|b| {
            b.name == "Suppressive Barrage"
                && b.source == TalentBuffSource::AscensionPassive(1)
                && b.stat == BuffableStat::PhysicalDmgBonus
                && matches!(
                    b.activation,
                    Some(Activation::Manual(ManualCondition::Stacks(3)))
                )
        }));

        let shenhe = find("shenhe").unwrap();
        assert_eq!(shenhe.len(), 14);
    }
}
