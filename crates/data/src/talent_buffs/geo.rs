use super::*;

// ===== Albedo =====
// A4 passive "Homuncular Nature": EM+125 for team after burst
// C1 "Flower of Eden": DEF +50% on Skill use
// C4 "Descent of Divinity": Plunging ATK DMG +30% in Solar Isotoma
// C6 "Dust of Purification": DMG Bonus +17% with Crystallize shield
static ALBEDO_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Homuncular Nature",
        description: desc!("After burst, grants EM+125 to nearby party members for 10s"),
        stat: BuffableStat::ElementalMastery,
        base_value: 125.0,
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
        name: "albedo_c1_def",
        description: desc!("C1: DEF +50% on Skill use"),
        stat: BuffableStat::DefPercent,
        base_value: 0.50,
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
        name: "albedo_c4_plunging_dmg",
        description: desc!("C4: Plunging ATK DMG +30% in Solar Isotoma"),
        stat: BuffableStat::PlungingAtkDmgBonus,
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
        name: "albedo_c6_dmg_bonus",
        description: desc!("C6: DMG Bonus +17% with Crystallize shield"),
        stat: BuffableStat::DmgBonus,
        base_value: 0.17,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Gorou =====
// Skill "Inuzaka All-Round Defense": DEF flat per level (Lv1-15) + Geo DMG+15% (3 Geo)
static GOROU_SKILL_DEF_SCALING: [f64; 15] = [
    206.16, 221.62, 237.08, 257.70, 273.16, 288.62, 309.24, 329.86, 350.48, 371.10, 391.72, 412.34,
    438.11, 463.88, 489.65,
];

static GOROU_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Inuzaka All-Round Defense",
        description: desc!("DEF increase based on skill talent level"),
        stat: BuffableStat::DefFlat,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&GOROU_SKILL_DEF_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Inuzaka All-Round Defense - Geo DMG",
        description: desc!(
            "With 3 Geo members, Geo DMG Bonus +15% (approximation: always registered)"
        ),
        stat: BuffableStat::ElementalDmgBonus(Element::Geo),
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Heedless of the Wind and Weather",
        description: desc!("A4: After burst, nearby party members gain DEF +25%"),
        stat: BuffableStat::DefPercent,
        base_value: 0.25,
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
        name: "gorou_c6_geo_crit_dmg",
        description: desc!("C6: Geo CRIT DMG +40% at Crunch"),
        stat: BuffableStat::CritDmg,
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Ningguang =====
// A4 passive "Strategic Reserve": Geo DMG+12% when passing through Jade Screen
static NINGGUANG_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Strategic Reserve",
    description: desc!("Characters passing through Jade Screen gain Geo DMG Bonus +12%"),
    stat: BuffableStat::ElementalDmgBonus(Element::Geo),
    base_value: 0.12,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive(4),
    min_constellation: 0,
    cap: None,
    activation: None,
}];

// ===== Yun Jin =====
// Elemental Burst "Cliffbreaker's Banner": flat Normal ATK DMG based on DEF (Lv1-15)
// C2 "Myriad Mise-En-Scène": Normal ATK DMG +15% after Burst
// C4 "Flower Stage": DEF +20% on Crystallize
static YUN_JIN_BURST_SCALING: [f64; 15] = [
    0.3216, 0.3457, 0.3699, 0.4020, 0.4262, 0.4503, 0.4824, 0.5145, 0.5466, 0.5789, 0.6110, 0.6431,
    0.6833, 0.7234, 0.7636,
];

static YUN_JIN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Cliffbreaker's Banner Normal ATK Bonus",
        description: desc!("Normal Attack DMG increased based on Yun Jin's DEF"),
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&YUN_JIN_BURST_SCALING),
        scales_on: Some(ScalingStat::Def),
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Breaking Conventions (1 Element Type)",
        description: desc!(
            "A4: Flying Cloud Flag Formation gains extra Normal ATK DMG = 2.5% of Yun Jin's DEF"
        ),
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 0.025,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Def),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Breaking Conventions (2 Element Types)",
        description: desc!(
            "A4: Flying Cloud Flag Formation gains extra Normal ATK DMG = 5.0% of Yun Jin's DEF"
        ),
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 0.05,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Def),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Breaking Conventions (3 Element Types)",
        description: desc!(
            "A4: Flying Cloud Flag Formation gains extra Normal ATK DMG = 7.5% of Yun Jin's DEF"
        ),
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 0.075,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Def),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Breaking Conventions (4 Element Types)",
        description: desc!(
            "A4: Flying Cloud Flag Formation gains extra Normal ATK DMG = 11.5% of Yun Jin's DEF"
        ),
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 0.115,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Def),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "yun_jin_c2_normal_dmg",
        description: desc!("C2: Normal ATK DMG +15% after Burst"),
        stat: BuffableStat::NormalAtkDmgBonus,
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
    TalentBuffDef {
        name: "yun_jin_c4_def",
        description: desc!("C4: DEF +20% on Crystallize"),
        stat: BuffableStat::DefPercent,
        base_value: 0.20,
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

// ===== Zhongli =====
// Jade Shield "Dominus Lapidis": All RES -20% for nearby enemies while shield is active
static ZHONGLI_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Jade Shield Pyro RES Shred",
        description: desc!("Nearby enemies' Pyro RES -20%"),
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
        name: "Jade Shield Hydro RES Shred",
        description: desc!("Nearby enemies' Hydro RES -20%"),
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
        name: "Jade Shield Electro RES Shred",
        description: desc!("Nearby enemies' Electro RES -20%"),
        stat: BuffableStat::ElementalResReduction(Element::Electro),
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
        name: "Jade Shield Cryo RES Shred",
        description: desc!("Nearby enemies' Cryo RES -20%"),
        stat: BuffableStat::ElementalResReduction(Element::Cryo),
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
        name: "Jade Shield Dendro RES Shred",
        description: desc!("Nearby enemies' Dendro RES -20%"),
        stat: BuffableStat::ElementalResReduction(Element::Dendro),
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
        name: "Jade Shield Anemo RES Shred",
        description: desc!("Nearby enemies' Anemo RES -20%"),
        stat: BuffableStat::ElementalResReduction(Element::Anemo),
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
        name: "Jade Shield Geo RES Shred",
        description: desc!("Nearby enemies' Geo RES -20%"),
        stat: BuffableStat::ElementalResReduction(Element::Geo),
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
        name: "Jade Shield Physical RES Shred",
        description: desc!("Nearby enemies' Physical RES -20%"),
        stat: BuffableStat::PhysicalResReduction,
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
];

// ===== Zibai =====
// C2 "At Birth Are Souls Born, and in Death Leave But Husks":
// Lunar-Crystallize Reaction DMG +30% for nearby party members
static ZIBAI_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "At Birth Are Souls Born C2 Reaction DMG",
    description: desc!("C2: Lunar-Crystallize Reaction DMG +30% for nearby party members"),
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.30,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: None,
}];

// ===== Illuga =====
static ILLUGA_BUFFS: &[TalentBuffDef] = &[
    // A1 "Torchforger's Covenant": After Geo DMG hits, party CRIT/EM buff
    TalentBuffDef {
        name: "Torchforger's Covenant - CRIT Rate",
        description: desc!("A1: After Geo DMG hits opponent, party CRIT Rate +5% for 20s"),
        stat: BuffableStat::CritRate,
        base_value: 0.05,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Torchforger's Covenant - CRIT DMG",
        description: desc!("A1: After Geo DMG hits opponent, party CRIT DMG +10% for 20s"),
        stat: BuffableStat::CritDmg,
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Torchforger's Covenant - EM (Moonsign)",
        description: desc!("A1: With Moonsign active, party EM +50 for 20s"),
        stat: BuffableStat::ElementalMastery,
        base_value: 50.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Demonhunter's Dusk (1 Hydro/Geo Characters)",
        description: desc!(
            "A4: Nightingale's Song base DMG increase +7% of Illuga's EM (burst flat DMG approximation)"
        ),
        stat: BuffableStat::BurstFlatDmg,
        base_value: 0.07,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Demonhunter's Dusk (2 Hydro/Geo Characters)",
        description: desc!(
            "A4: Nightingale's Song base DMG increase +14% of Illuga's EM (burst flat DMG approximation)"
        ),
        stat: BuffableStat::BurstFlatDmg,
        base_value: 0.14,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Demonhunter's Dusk (3 Hydro/Geo Characters)",
        description: desc!(
            "A4: Nightingale's Song base DMG increase +24% of Illuga's EM (burst flat DMG approximation)"
        ),
        stat: BuffableStat::BurstFlatDmg,
        base_value: 0.24,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    // C4 "Unfolding of Starlight": Party DEF +200 during Burst
    TalentBuffDef {
        name: "illuga_c4_def_flat",
        description: desc!("C4: Party DEF +200 during Burst"),
        stat: BuffableStat::DefFlat,
        base_value: 200.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    // C6 "Nightmare Orioles": Enhances A1 values
    TalentBuffDef {
        name: "Nightmare Orioles - CRIT Rate",
        description: desc!("C6: A1 CRIT Rate enhanced to +10%"),
        stat: BuffableStat::CritRate,
        base_value: 0.05,
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
        name: "Nightmare Orioles - CRIT DMG",
        description: desc!("C6: A1 CRIT DMG enhanced to +30%"),
        stat: BuffableStat::CritDmg,
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
        name: "Nightmare Orioles - EM",
        description: desc!("C6: A1 EM enhanced to +80"),
        stat: BuffableStat::ElementalMastery,
        base_value: 30.0,
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

// ===== Xilonen =====
// Skill: Elemental RES Reduction per skill level (element depends on party; Geo always active)
// C4 "Suchitl's Trance": Normal/Charged/Plunging ATK deal additional DMG equal to 65% of Xilonen's DEF
static XILONEN_SKILL_RES_SCALING: [f64; 15] = [
    0.09, 0.12, 0.15, 0.18, 0.21, 0.24, 0.27, 0.30, 0.33, 0.36, 0.39, 0.42, 0.45, 0.48, 0.51,
];

static XILONEN_BUFFS: &[TalentBuffDef] = &[
    // Yohual's Scratch: Samplers reduce elemental RES based on party composition.
    // Geo is always active; up to 3 Geo Samplers convert to Pyro/Hydro/Cryo/Electro per party members.
    TalentBuffDef {
        name: "Yohual's Scratch Geo RES Reduction",
        description: desc!("Skill: Geo Sampler reduces nearby enemy Geo RES (always active)"),
        stat: BuffableStat::ElementalResReduction(Element::Geo),
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&XILONEN_SKILL_RES_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Yohual's Scratch Pyro RES Reduction",
        description: desc!(
            "Skill: Pyro Sampler reduces nearby enemy Pyro RES (if Pyro party member present)"
        ),
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&XILONEN_SKILL_RES_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Yohual's Scratch Hydro RES Reduction",
        description: desc!(
            "Skill: Hydro Sampler reduces nearby enemy Hydro RES (if Hydro party member present)"
        ),
        stat: BuffableStat::ElementalResReduction(Element::Hydro),
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&XILONEN_SKILL_RES_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Yohual's Scratch Cryo RES Reduction",
        description: desc!(
            "Skill: Cryo Sampler reduces nearby enemy Cryo RES (if Cryo party member present)"
        ),
        stat: BuffableStat::ElementalResReduction(Element::Cryo),
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&XILONEN_SKILL_RES_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Yohual's Scratch Electro RES Reduction",
        description: desc!(
            "Skill: Electro Sampler reduces nearby enemy Electro RES (if Electro party member present)"
        ),
        stat: BuffableStat::ElementalResReduction(Element::Electro),
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&XILONEN_SKILL_RES_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Portable Armored Sheath",
        description: desc!(
            "A4: When a nearby party member triggers Nightsoul Burst, Xilonen gains DEF +20%"
        ),
        stat: BuffableStat::DefPercent,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    // C2 "Chiucue Mix": Element-specific team buffs based on active Source Samples
    // Geo Source Sample always active at C2; other elements depend on party composition
    // Electro effect (energy restore + burst CD) is not a stat buff, omitted
    TalentBuffDef {
        name: "Chiucue Mix Geo DMG Bonus",
        description: desc!("C2: Geo Source Sample grants nearby active character Geo DMG +50%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Geo),
        base_value: 0.50,
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
        name: "Chiucue Mix Pyro ATK Bonus",
        description: desc!("C2: Pyro Source Sample grants nearby active Pyro character ATK +45%"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.45,
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
        name: "Chiucue Mix Hydro Max HP Bonus",
        description: desc!(
            "C2: Hydro Source Sample grants nearby active Hydro character Max HP +45%"
        ),
        stat: BuffableStat::HpPercent,
        base_value: 0.45,
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
        name: "Chiucue Mix Cryo CRIT DMG Bonus",
        description: desc!(
            "C2: Cryo Source Sample grants nearby active Cryo character CRIT DMG +60%"
        ),
        stat: BuffableStat::CritDmg,
        base_value: 0.60,
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
        name: "Suchitl's Trance Normal ATK Flat DMG",
        description: desc!(
            "C4: Normal, Charged, and Plunging Attacks deal additional DMG equal to 65% of Xilonen's DEF"
        ),
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 0.65,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Def),
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Suchitl's Trance Charged ATK Flat DMG",
        description: desc!(
            "C4: Normal, Charged, and Plunging Attacks deal additional DMG equal to 65% of Xilonen's DEF"
        ),
        stat: BuffableStat::ChargedAtkFlatDmg,
        base_value: 0.65,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Def),
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Suchitl's Trance Plunging ATK Flat DMG",
        description: desc!(
            "C4: Normal, Charged, and Plunging Attacks deal additional DMG equal to 65% of Xilonen's DEF"
        ),
        stat: BuffableStat::PlungingAtkFlatDmg,
        base_value: 0.65,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Def),
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: None,
    },
];

// ===== Chiori =====
// A4 "The Finishing Touch": Geo DMG +20% after nearby party member creates a Geo Construct
// C6 "A Guru's Inbred Nature": NA DMG +235% DEF as flat DMG
static CHIORI_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "The Finishing Touch Geo DMG Bonus",
        description: desc!(
            "After a nearby party member creates a Geo Construct, Chiori gains Geo DMG Bonus +20%"
        ),
        stat: BuffableStat::ElementalDmgBonus(Element::Geo),
        base_value: 0.20,
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
        name: "A Guru's Inbred Nature - NA Flat DMG",
        description: desc!("C6: Normal Attack DMG +235% of Chiori's DEF as flat DMG"),
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 2.35,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Def),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Itto =====
// A4 "Bloodline of the Crimson Oni": Kesagiri (Charged ATK) +35% DEF as flat DMG
// C4 "Alright, More Dumplings!": Team DEF+20% / ATK+20% when Raging Oni King ends
// C6 "Crimson Oni King's Legacy": CA CRIT DMG +70%
static ITTO_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Bloodline of the Crimson Oni",
        description: desc!("A4: Arataki Kesagiri DMG +35% of Itto's DEF as flat DMG"),
        stat: BuffableStat::ChargedAtkFlatDmg,
        base_value: 0.35,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Def),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Alright More Dumplings DEF Bonus",
        description: desc!("C4: After Raging Oni King ends, team DEF +20%"),
        stat: BuffableStat::DefPercent,
        base_value: 0.20,
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
        name: "Alright More Dumplings ATK Bonus",
        description: desc!("C4: After Raging Oni King ends, team ATK +20%"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.20,
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
        name: "Crimson Oni King's Legacy CA CRIT DMG",
        description: desc!("C6: Arataki Kesagiri CRIT DMG +70%"),
        stat: BuffableStat::CritDmg,
        base_value: 0.70,
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

// ===== Kachina =====
// A4 "Flowy Mountain": Geo DMG +20%
// C4 "Stand Together": DEF +8~20% based on enemy count (use max 20%)
static KACHINA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Flowy Mountain Geo DMG Bonus",
        description: desc!("A4: Geo DMG Bonus +20%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Geo),
        base_value: 0.20,
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
        name: "Stand Together DEF Bonus",
        description: desc!("C4: DEF +8~20% based on nearby enemies (max 20%)"),
        stat: BuffableStat::DefPercent,
        base_value: 0.20,
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

// ===== Navia =====
// A1 "That Shard's Ours, Undoubtedly!": NA/CA/Plunge DMG +40% after burst
// A4 "Turns Out, She's a Damn Good Miner!": ATK +20% (up to +40% max)
// C2 "Candlelight": CRIT Rate +12% per Shockwave stack (max 3 stacks = +36%)
// C4 "The Little Bit of Kindness We Show": enemy Geo RES -20%
static NAVIA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "That Shard's Ours - NA DMG Bonus",
        description: desc!("A1: After burst, Normal Attack DMG +40%"),
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.40,
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
        name: "That Shard's Ours - CA DMG Bonus",
        description: desc!("A1: After burst, Charged Attack DMG +40%"),
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.40,
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
        name: "That Shard's Ours - Plunge DMG Bonus",
        description: desc!("A1: After burst, Plunging Attack DMG +40%"),
        stat: BuffableStat::PlungingAtkDmgBonus,
        base_value: 0.40,
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
        name: "Turns Out She's a Damn Good Miner ATK Bonus",
        description: desc!("A4: ATK +20% per Shockwave stack (max +40%)"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.40,
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
        name: "Candlelight CRIT Rate",
        description: desc!("C2: CRIT Rate +12% per Shockwave stack, up to 3 stacks (+36% max)"),
        stat: BuffableStat::CritRate,
        base_value: 0.12,
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
        name: "The Little Bit of Kindness Geo RES Reduction",
        description: desc!("C4: Enemy Geo RES -20%"),
        stat: BuffableStat::ElementalResReduction(Element::Geo),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Noelle =====
// Elemental Burst "Sweeping Time": ATK +DEF-based flat ATK (Lv1-15)
// C2 "Combat Maid": CA DMG +15%
// C6 "Must Be Spotless": ATK +50% DEF as flat ATK
static NOELLE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Sweeping Time ATK Flat",
        description: desc!("Elemental Burst: ATK increases based on Noelle's DEF"),
        stat: BuffableStat::AtkFlat,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&[
            0.40, 0.43, 0.46, 0.50, 0.53, 0.56, 0.60, 0.64, 0.68, 0.72, 0.76, 0.80, 0.85, 0.90,
            0.95,
        ]),
        scales_on: Some(ScalingStat::Def),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Combat Maid CA DMG Bonus",
        description: desc!("C2: Charged Attack DMG +15%"),
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.15,
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
        name: "Must Be Spotless ATK Flat",
        description: desc!("C6: During burst, ATK +50% of Noelle's DEF as flat ATK"),
        stat: BuffableStat::AtkFlat,
        base_value: 0.50,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Def),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static GEO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("albedo", ALBEDO_BUFFS),
    ("chiori", CHIORI_BUFFS),
    ("gorou", GOROU_BUFFS),
    ("illuga", ILLUGA_BUFFS),
    ("itto", ITTO_BUFFS),
    ("kachina", KACHINA_BUFFS),
    ("navia", NAVIA_BUFFS),
    ("ningguang", NINGGUANG_BUFFS),
    ("noelle", NOELLE_BUFFS),
    ("xilonen", XILONEN_BUFFS),
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

#[cfg(test)]
mod tests {
    use super::*;

    fn find_buff(
        buffs: &[TalentBuffDef],
        stat: BuffableStat,
        source: TalentBuffSource,
    ) -> &TalentBuffDef {
        buffs
            .iter()
            .find(|buff| buff.stat == stat && buff.source == source)
            .expect("expected buff to exist")
    }

    #[test]
    fn test_find_chiori_a4_geo_damage_bonus() {
        let buffs = find("chiori").expect("Chiori talent buffs should exist");
        let buff = find_buff(
            buffs,
            BuffableStat::ElementalDmgBonus(Element::Geo),
            TalentBuffSource::AscensionPassive(4),
        );
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert_eq!(
            buff.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );
        assert!((buff.base_value - 0.20).abs() < 1e-6);
    }

    #[test]
    fn test_find_noelle_burst_flat_atk_scaling() {
        let buffs = find("noelle").expect("Noelle talent buffs should exist");
        let buff = find_buff(
            buffs,
            BuffableStat::AtkFlat,
            TalentBuffSource::ElementalBurst,
        );
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert_eq!(
            buff.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );
        assert_eq!(buff.scales_on, Some(ScalingStat::Def));
        assert!(buff.scales_with_talent);
        let scaling = buff.talent_scaling.expect("expected scaling table");
        let expected = [
            0.40, 0.43, 0.46, 0.50, 0.53, 0.56, 0.60, 0.64, 0.68, 0.72, 0.76, 0.80, 0.85, 0.90,
            0.95,
        ];
        for (actual, expected) in scaling.iter().zip(expected.iter()) {
            assert!((actual - expected).abs() < 1e-6);
        }
    }
}
