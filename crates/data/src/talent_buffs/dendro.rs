use super::*;
use genshin_calc_core::Reaction;

// ===== Lauma =====
// A4 passive "Cleansing for the Spring": Elemental Skill DMG +0.04% per point of EM (max +32% at 800 EM)
// C2 "Light Devouring": Party Lunar-Bloom DMG +40% (Toggle)
// C6 "Nightland's Mournful Song": Party Lunar-Bloom DMG +25% (Toggle)
static LAUMA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Cleansing for the Spring",
        description: desc!("A4: Elemental Skill DMG +0.04% per point of EM (max +32% at 800 EM)"),
        stat: BuffableStat::SkillDmgBonus,
        base_value: 0.0004,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(0.32),
        activation: None,
    },
    TalentBuffDef {
        name: "Light Devouring Lunar-Bloom DMG Bonus",
        description: desc!("C2: Party Lunar-Bloom DMG +40% (Toggle)"),
        stat: BuffableStat::ReactionDmgBonus(Reaction::LunarBloom),
        base_value: 0.40,
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
        name: "Nightland's Mournful Song Lunar-Bloom DMG Bonus",
        description: desc!("C6: Party Lunar-Bloom DMG +25% (Toggle)"),
        stat: BuffableStat::ReactionDmgBonus(Reaction::LunarBloom),
        base_value: 0.25,
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

// ===== Nahida =====
// Design approximation: game uses highest party EM, but TalentBuffDef resolution only has the
// provider's build-time stats, so this entry scales from Nahida's own EM instead.
// C2 "The Root of All Fullness": Reaction CRIT Rate +20%, CRIT DMG +100%, DEF -30%
// C4 "Wakening Elucidation": EM +100 per nearby Skandha enemy (max 4)
static NAHIDA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Compassion Illuminated",
        description: desc!(
            "A1: Grants EM to party = highest_party_EM × 25%, max 250 (design approximation: uses Nahida's own EM at build time)"
        ),
        stat: BuffableStat::ElementalMastery,
        base_value: 0.25,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: Some(250.0),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Awakening Elucidated Skill DMG Bonus",
        description: desc!("A4: Tri-Karma Purification DMG +0.1% per EM above 200, max +80%"),
        stat: BuffableStat::SkillDmgBonus,
        base_value: 0.001,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Auto(AutoCondition::StatScaling {
            stat: BuffableStat::ElementalMastery,
            offset: Some(200.0),
            cap: Some([0.80; 5]),
        })),
    },
    TalentBuffDef {
        name: "Awakening Elucidated Skill CRIT Rate",
        description: desc!(
            "A4: Tri-Karma Purification CRIT Rate +0.03% per EM above 200, max +24%"
        ),
        stat: BuffableStat::CritRate,
        base_value: 0.0003,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Auto(AutoCondition::StatScaling {
            stat: BuffableStat::ElementalMastery,
            offset: Some(200.0),
            cap: Some([0.24; 5]),
        })),
    },
    TalentBuffDef {
        name: "nahida_c2_crit_rate",
        description: desc!("C2: Reaction CRIT Rate +20%"),
        stat: BuffableStat::CritRate,
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
        name: "nahida_c2_crit_dmg",
        description: desc!("C2: Reaction CRIT DMG +100%"),
        stat: BuffableStat::CritDmg,
        base_value: 1.00,
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
        name: "nahida_c2_def_reduction",
        description: desc!("C2: DEF -30% on Quicken/Aggravate/Spread"),
        stat: BuffableStat::DefReduction,
        base_value: 0.30,
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
        name: "nahida_c4_em",
        description: desc!("C4: EM +100 per nearby Skandha enemy (max 4)"),
        stat: BuffableStat::ElementalMastery,
        base_value: 100.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(4))),
    },
];

// ===== Nefer =====
// C2 "Observation Feeds Strategy": EM +200 at 5 Veil stacks (Toggle)
// C4 "Delusion Ensnares Reason": Dendro RES -20% during Shadow Dance (Toggle)
// C6 "A Thousand Faces of Naught": Party Lunar-Bloom DMG +15% at Ascendant Gleam
static NEFER_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "nefer_c2_em",
        description: desc!("C2: EM +200 at 5 Veil stacks"),
        stat: BuffableStat::ElementalMastery,
        base_value: 200.0,
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
        name: "nefer_c4_dendro_res_shred",
        description: desc!("C4: Opponents' Dendro RES -20% in Shadow Dance state"),
        stat: BuffableStat::ElementalResReduction(Element::Dendro),
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
    TalentBuffDef {
        name: "nefer_c6_lunar_bloom_dmg",
        description: desc!("C6: Party Lunar-Bloom DMG +15% at Ascendant Gleam"),
        stat: BuffableStat::ReactionDmgBonus(Reaction::LunarBloom),
        base_value: 0.15,
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

// ===== Dendro Traveler =====
// A4 passive "Verdant Luxury": EM+60 in Lea Lotus Lamp field
// C6 "The Planted Seed": Party Dendro DMG +12% inside Lamp
static TRAVELER_DENDRO_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Verdant Luxury",
        description: desc!("Characters within Lea Lotus Lamp field gain EM+60"),
        stat: BuffableStat::ElementalMastery,
        base_value: 60.0,
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
        name: "traveler_dendro_c6_dendro_dmg",
        description: desc!("C6: Party Dendro DMG Bonus +12% inside Lamp"),
        stat: BuffableStat::ElementalDmgBonus(Element::Dendro),
        base_value: 0.12,
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

// ===== Alhaitham =====
// A4 "Four-Causal Correction": DMG +0.1% per EM, max +100%
// C2 "Debate": EM +50 per mirror generation event (max 4 stacks)
// C4 "Insight": Team EM +30 per mirror consumed; Dendro DMG +10% per mirror generated (Stacks(3))
// C6 "Structuring": CR +10%, CD +70% (Toggle)
static ALHAITHAM_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Four-Causal Correction DMG Bonus",
        description: desc!("A4: DMG +0.1% per point of EM, max +100%"),
        stat: BuffableStat::DmgBonus,
        base_value: 0.001,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(1.00),
        activation: None,
    },
    TalentBuffDef {
        name: "Chisel-Light Mirror EM Bonus",
        description: desc!("C2: EM +50 per mirror generation event (max 4 stacks)"),
        stat: BuffableStat::ElementalMastery,
        base_value: 50.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(4))),
    },
    TalentBuffDef {
        name: "Insight Team EM Bonus",
        description: desc!("C4: Team EM +30 per Chisel-Light Mirror consumed"),
        stat: BuffableStat::ElementalMastery,
        base_value: 30.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
    },
    TalentBuffDef {
        name: "Insight Dendro DMG Bonus",
        description: desc!("C4: Self Dendro DMG Bonus +10% per Chisel-Light Mirror generated"),
        stat: BuffableStat::ElementalDmgBonus(Element::Dendro),
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
    },
    TalentBuffDef {
        name: "Structuring Crit Rate Bonus",
        description: desc!("C6: CR +10% (Toggle)"),
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
        name: "Structuring Crit DMG Bonus",
        description: desc!("C6: CD +70% (Toggle)"),
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

// ===== Baizhu =====
// A4 "All Things Are of the Earth": exact reaction bonuses based on HP (cap 50,000 HP)
// C4 "Ancient Art of Perception": Team EM +80
// C6 "Weal and Woe Twain": Skill DMG +8% of Max HP (flat)
static BAIZHU_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Year of Verdant Favor Burning DMG Bonus",
        description: desc!("A4: Burning DMG +2% per 1000 HP, max +100%"),
        stat: BuffableStat::ReactionDmgBonus(Reaction::Burning),
        base_value: 0.00002,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(1.00),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Year of Verdant Favor Bloom DMG Bonus",
        description: desc!("A4: Bloom DMG +2% per 1000 HP, max +100%"),
        stat: BuffableStat::ReactionDmgBonus(Reaction::Bloom),
        base_value: 0.00002,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(1.00),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Year of Verdant Favor Hyperbloom DMG Bonus",
        description: desc!("A4: Hyperbloom DMG +2% per 1000 HP, max +100%"),
        stat: BuffableStat::ReactionDmgBonus(Reaction::Hyperbloom),
        base_value: 0.00002,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(1.00),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Year of Verdant Favor Burgeon DMG Bonus",
        description: desc!("A4: Burgeon DMG +2% per 1000 HP, max +100%"),
        stat: BuffableStat::ReactionDmgBonus(Reaction::Burgeon),
        base_value: 0.00002,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(1.00),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Year of Verdant Favor Lunar-Bloom DMG Bonus",
        description: desc!("A4: Lunar-Bloom DMG +0.7% per 1000 HP, max +35%"),
        stat: BuffableStat::ReactionDmgBonus(Reaction::LunarBloom),
        base_value: 0.000007,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(0.35),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Year of Verdant Favor Aggravate DMG Bonus",
        description: desc!("A4: Aggravate DMG bonus +0.8% per 1000 HP, max +40%"),
        stat: BuffableStat::ReactionDmgBonus(Reaction::Aggravate),
        base_value: 0.000008,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(0.40),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Year of Verdant Favor Spread DMG Bonus",
        description: desc!("A4: Spread DMG bonus +0.8% per 1000 HP, max +40%"),
        stat: BuffableStat::ReactionDmgBonus(Reaction::Spread),
        base_value: 0.000008,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(0.40),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "baizhu_c4_em",
        description: desc!("C4: Party EM +80 after Burst"),
        stat: BuffableStat::ElementalMastery,
        base_value: 80.0,
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
        name: "baizhu_c6_skill_flat_dmg",
        description: desc!("C6: Skill DMG bonus based on 8% of Max HP"),
        stat: BuffableStat::SkillFlatDmg,
        base_value: 0.08,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Collei =====
// C4: EM+60 in Cuilein-Anbar field (excludes self)
// Note: C6 creates a miniature Cuilein-Anbar proc hit; it is not a stat buff.
static COLLEI_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "collei_c4_em",
    description: desc!("C4: Party EM +60 after Burst (excludes self)"),
    stat: BuffableStat::ElementalMastery,
    base_value: 60.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::TeamExcludeSelf,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
}];

// ===== Emilie =====
// A4 "Rectification": ATK → DMG bonus (0.015% per ATK, max +36%)
// C1 "Consequence of Karma": Elemental Skill DMG +20% (Toggle)
// C2 "The Controlled Blaze": Enemy Dendro RES -30% (Toggle, Team)
static EMILIE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Rectification DMG Bonus",
        description: desc!("A4: DMG +0.015% per ATK, max +36%"),
        stat: BuffableStat::DmgBonus,
        base_value: 0.00015,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::TotalAtk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(0.36),
        activation: None,
    },
    TalentBuffDef {
        name: "Consequence of Karma Skill DMG Bonus",
        description: desc!("C1: Elemental Skill DMG +20%"),
        stat: BuffableStat::SkillDmgBonus,
        base_value: 0.20,
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
        name: "The Controlled Blaze Dendro RES Down",
        description: desc!("C2: Enemy Dendro RES -30%"),
        stat: BuffableStat::ElementalResReduction(Element::Dendro),
        base_value: 0.30,
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

// ===== Kaveh =====
// A4 "A Craftsman's Curious Conceptions": EM +25 per Bloom core consumed (max 4 stacks = +100 EM)
// C1 "Sublime Salvo": Healing Bonus +25%
// C4 "Feast of Apadana": Bloom DMG +60% (Toggle)
static KAVEH_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "A Craftsman's Curious Conceptions EM Bonus",
        description: desc!("A4: EM +25 per Bloom core consumed (max 4 stacks)"),
        stat: BuffableStat::ElementalMastery,
        base_value: 25.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(4))),
    },
    TalentBuffDef {
        name: "Sublime Salvo Healing Bonus",
        description: desc!("C1: Healing Bonus +25%"),
        stat: BuffableStat::HealingBonus,
        base_value: 0.25,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Feast of Apadana Bloom DMG Bonus",
        description: desc!("C4: Bloom DMG +60% (Toggle)"),
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.60,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Painted Dome Bloom DMG Bonus",
        description: desc!("Burst: Dendro Core Burst DMG Bonus scales with talent level"),
        stat: BuffableStat::ReactionDmgBonus(Reaction::Bloom),
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&KAVEH_BURST_SCALING),
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

static KAVEH_BURST_SCALING: [f64; 15] = [
    0.2749, 0.2955, 0.3161, 0.3436, 0.3642, 0.3848, 0.4123, 0.4398, 0.4673, 0.4948, 0.5223, 0.5498,
    0.5841, 0.6185, 0.6528,
];

// ===== Kinich =====
// C2 "Parrot Squawks, Jaguars Roar": Enemy Dendro RES -30% + DMG +100% (Team/Self, Toggle)
// C4 "Canopy Hunter: Glamorous Debut": Burst DMG +70%
static KINICH_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Parrot Squawks Dendro RES Down",
        description: desc!("C2: Enemy Dendro RES -30% (Toggle)"),
        stat: BuffableStat::ElementalResReduction(Element::Dendro),
        base_value: 0.30,
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
        name: "Parrot Squawks DMG Bonus",
        description: desc!("C2: DMG +100% (Toggle)"),
        stat: BuffableStat::DmgBonus,
        base_value: 1.00,
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
        name: "Canopy Hunter Burst DMG Bonus",
        description: desc!("C4: Burst DMG +70%"),
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.70,
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

// ===== Kirara =====
// A4: HP → Skill/Burst DMG
// C6 "Kindred of the Sinovamakara": Team All Elem DMG +12%
static KIRARA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Pupillary Variance Skill DMG Bonus",
        description: desc!("A4: Skill DMG +0.4% per 1000 Max HP"),
        stat: BuffableStat::SkillDmgBonus,
        base_value: 0.000004,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Auto(AutoCondition::StatScaling {
            stat: BuffableStat::HpPercent,
            offset: None,
            cap: None,
        })),
    },
    TalentBuffDef {
        name: "Pupillary Variance Burst DMG Bonus",
        description: desc!("A4: Burst DMG +0.3% per 1000 Max HP"),
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.000003,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Auto(AutoCondition::StatScaling {
            stat: BuffableStat::HpPercent,
            offset: None,
            cap: None,
        })),
    },
    TalentBuffDef {
        name: "kirara_c6_elemental_dmg_bonus",
        description: desc!("C6: Pyro DMG Bonus +12%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.12,
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
        name: "kirara_c6_elemental_dmg_bonus",
        description: desc!("C6: Hydro DMG Bonus +12%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
        base_value: 0.12,
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
        name: "kirara_c6_elemental_dmg_bonus",
        description: desc!("C6: Electro DMG Bonus +12%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Electro),
        base_value: 0.12,
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
        name: "kirara_c6_elemental_dmg_bonus",
        description: desc!("C6: Cryo DMG Bonus +12%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
        base_value: 0.12,
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
        name: "kirara_c6_elemental_dmg_bonus",
        description: desc!("C6: Dendro DMG Bonus +12%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Dendro),
        base_value: 0.12,
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
        name: "kirara_c6_elemental_dmg_bonus",
        description: desc!("C6: Anemo DMG Bonus +12%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
        base_value: 0.12,
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
        name: "kirara_c6_elemental_dmg_bonus",
        description: desc!("C6: Geo DMG Bonus +12%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Geo),
        base_value: 0.12,
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

// ===== Tighnari =====
// A1 "Keen Sight": EM +50 (self)
// A4 "Scholarly Blade": EM → CA DMG bonus (0.06% per EM, max +60%), EM → Burst DMG bonus (same)
// C1 "Beginners' Luck": CA CR +15% (Toggle)
// C2 "Ingenuity of the Smiths": Dendro DMG +20%
// C4 "Fortuitous Arrival": Team EM +60 (Toggle), plus reaction-triggered +60 (Toggle)
static TIGHNARI_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Keen Sight EM Bonus",
        description: desc!("A1: EM +50"),
        stat: BuffableStat::ElementalMastery,
        base_value: 50.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Scholarly Blade CA DMG Bonus",
        description: desc!("A4: Charged Attack DMG +0.06% per EM, max +60%"),
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.0006,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(0.60),
        activation: None,
    },
    TalentBuffDef {
        name: "Scholarly Blade Burst DMG Bonus",
        description: desc!("A4: Burst DMG +0.06% per EM, max +60%"),
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.0006,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(0.60),
        activation: None,
    },
    TalentBuffDef {
        name: "Beginners' Luck CA Crit Rate",
        description: desc!("C1: Charged Attack CR +15%"),
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
        name: "Ingenuity of the Smiths Dendro DMG Bonus",
        description: desc!("C2: Dendro DMG +20%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Dendro),
        base_value: 0.20,
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
        name: "Fortuitous Arrival Team EM Bonus",
        description: desc!("C4: Team EM +60 (Toggle)"),
        stat: BuffableStat::ElementalMastery,
        base_value: 60.0,
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
        name: "Fortuitous Arrival Reaction EM Bonus",
        description: desc!("C4: Reaction-triggered Team EM +60"),
        stat: BuffableStat::ElementalMastery,
        base_value: 60.0,
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

// ===== Yaoyao =====
// C1 "Nutcracker": Dendro DMG +15%
// C4 "Whitesun Wheel": HP → EM (0.3% per HP, max 120 EM)
static YAOYAO_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Nutcracker Dendro DMG Bonus",
        description: desc!("C1: Dendro DMG +15%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Dendro),
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "yaoyao_c1_dendro_dmg",
        description: desc!("C1: Party Dendro DMG Bonus +15% on radish explosion"),
        stat: BuffableStat::ElementalDmgBonus(Element::Dendro),
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
    TalentBuffDef {
        name: "yaoyao_c4_em",
        description: desc!("C4: EM bonus based on 0.3% of Max HP (max 120)"),
        stat: BuffableStat::ElementalMastery,
        base_value: 0.003,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: Some(120.0),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static DENDRO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("alhaitham", ALHAITHAM_BUFFS),
    ("baizhu", BAIZHU_BUFFS),
    ("collei", COLLEI_BUFFS),
    ("emilie", EMILIE_BUFFS),
    ("kaveh", KAVEH_BUFFS),
    ("kinich", KINICH_BUFFS),
    ("kirara", KIRARA_BUFFS),
    ("lauma", LAUMA_BUFFS),
    ("nahida", NAHIDA_BUFFS),
    ("nefer", NEFER_BUFFS),
    ("tighnari", TIGHNARI_BUFFS),
    ("traveler_dendro", TRAVELER_DENDRO_BUFFS),
    ("yaoyao", YAOYAO_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    DENDRO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use genshin_calc_core::Reaction;

    #[test]
    fn audit_dendro_emilie_a4_and_kaveh_a4_values() {
        let emilie = find("emilie").expect("Emilie talent buffs should exist");
        let emilie_a4 = emilie
            .iter()
            .find(|b| b.source == TalentBuffSource::AscensionPassive(4))
            .expect("Emilie A4 should exist");
        assert_eq!(emilie_a4.stat, BuffableStat::DmgBonus);
        assert!((emilie_a4.base_value - 0.00015).abs() < 1e-9);
        assert_eq!(emilie_a4.scales_on, Some(ScalingStat::TotalAtk));
        assert_eq!(emilie_a4.cap, Some(0.36));

        let kaveh = find("kaveh").expect("Kaveh talent buffs should exist");
        let kaveh_a4 = kaveh
            .iter()
            .find(|b| b.source == TalentBuffSource::AscensionPassive(4))
            .expect("Kaveh A4 should exist");
        assert_eq!(kaveh_a4.name, "A Craftsman's Curious Conceptions EM Bonus");

        let burst = get_talent_conditional_buffs("kaveh", 0, &[1, 1, 1])
            .into_iter()
            .find(|b| b.stat == BuffableStat::ReactionDmgBonus(Reaction::Bloom))
            .expect("Kaveh burst Bloom bonus should exist");
        assert!((burst.value - 0.2749).abs() < 1e-9);

        let burst_lv15 = get_talent_conditional_buffs("kaveh", 0, &[1, 1, 15])
            .into_iter()
            .find(|b| b.stat == BuffableStat::ReactionDmgBonus(Reaction::Bloom))
            .expect("Kaveh burst Bloom bonus should exist at Lv15");
        assert!((burst_lv15.value - 0.6528).abs() < 1e-9);
    }

    #[test]
    fn audit_dendro_baizhu_alhaitham_lauma_tighnari_exact_entries() {
        let baizhu = find("baizhu").expect("Baizhu talent buffs should exist");
        let baizhu_a4: Vec<_> = baizhu
            .iter()
            .filter(|b| b.source == TalentBuffSource::AscensionPassive(4))
            .collect();
        assert_eq!(baizhu_a4.len(), 7);
        assert!(baizhu_a4.iter().all(|b| b.activation.is_some()));
        assert!(baizhu_a4.iter().any(|b| {
            b.stat == BuffableStat::ReactionDmgBonus(Reaction::Burning)
                && (b.base_value - 0.00002).abs() < 1e-9
                && b.cap == Some(1.0)
        }));
        assert!(baizhu_a4.iter().any(|b| {
            b.stat == BuffableStat::ReactionDmgBonus(Reaction::LunarBloom)
                && (b.base_value - 0.000007).abs() < 1e-9
                && b.cap == Some(0.35)
        }));
        assert!(baizhu_a4.iter().any(|b| {
            b.stat == BuffableStat::ReactionDmgBonus(Reaction::Aggravate)
                && (b.base_value - 0.000008).abs() < 1e-9
                && b.cap == Some(0.40)
        }));
        assert!(baizhu_a4.iter().any(|b| {
            b.stat == BuffableStat::ReactionDmgBonus(Reaction::Spread)
                && (b.base_value - 0.000008).abs() < 1e-9
                && b.cap == Some(0.40)
        }));
        assert!(
            baizhu_a4
                .iter()
                .all(|b| b.stat != BuffableStat::TransformativeBonus),
            "Baizhu A4 should use exact reaction stats, not generic TransformativeBonus"
        );

        let alhaitham = get_talent_conditional_buffs("alhaitham", 4, &[1, 1, 1]);
        let alhaitham_c4_team_em = alhaitham
            .iter()
            .find(|b| b.stat == BuffableStat::ElementalMastery && b.target == BuffTarget::Team)
            .expect("Alhaitham C4 team EM should exist");
        assert_eq!(
            alhaitham_c4_team_em.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        );
        assert!((alhaitham_c4_team_em.value - 30.0).abs() < 1e-9);
        let alhaitham_c4_self_dmg = alhaitham
            .iter()
            .find(|b| {
                b.stat == BuffableStat::ElementalDmgBonus(Element::Dendro)
                    && b.target == BuffTarget::OnlySelf
            })
            .expect("Alhaitham C4 self Dendro DMG Bonus should exist");
        assert_eq!(
            alhaitham_c4_self_dmg.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        );
        assert!((alhaitham_c4_self_dmg.value - 0.10).abs() < 1e-9);

        let lauma = get_talent_conditional_buffs("lauma", 6, &[1, 1, 1]);
        let lauma_c2 = lauma
            .iter()
            .find(|b| {
                b.name.contains("lunar_bloom")
                    || b.stat == BuffableStat::ReactionDmgBonus(Reaction::LunarBloom)
            })
            .expect("Lauma C2 Lunar-Bloom bonus should exist");
        assert_eq!(lauma_c2.target, BuffTarget::Team);
        assert_eq!(
            lauma_c2.activation,
            Activation::Manual(ManualCondition::Toggle)
        );
        assert!((lauma_c2.value - 0.40).abs() < 1e-9);
        let lauma_c6 = lauma
            .iter()
            .filter(|b| b.stat == BuffableStat::ReactionDmgBonus(Reaction::LunarBloom))
            .collect::<Vec<_>>();
        assert!(
            lauma_c6.iter().any(|b| (b.value - 0.25).abs() < 1e-9),
            "Lauma C6 Lunar-Bloom bonus should exist"
        );

        let tighnari = get_talent_conditional_buffs("tighnari", 4, &[1, 1, 1]);
        let tighnari_c4: Vec<_> = tighnari
            .iter()
            .filter(|b| b.stat == BuffableStat::ElementalMastery && b.target == BuffTarget::Team)
            .collect();
        assert_eq!(tighnari_c4.len(), 2);
        assert!(tighnari_c4.iter().any(|b| {
            b.activation == Activation::Manual(ManualCondition::Toggle)
                && (b.value - 60.0).abs() < 1e-9
        }));
    }

    #[test]
    fn audit_dendro_duplicate_non_toggle_entries_removed() {
        let nefer = find("nefer").expect("Nefer talent buffs should exist");
        let nefer_c2: Vec<_> = nefer
            .iter()
            .filter(|b| b.source == TalentBuffSource::Constellation(2))
            .collect();
        assert_eq!(nefer_c2.len(), 1);
        assert_eq!(
            nefer_c2[0].activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );
        let nefer_c4: Vec<_> = nefer
            .iter()
            .filter(|b| b.source == TalentBuffSource::Constellation(4))
            .collect();
        assert_eq!(nefer_c4.len(), 1);
        assert_eq!(
            nefer_c4[0].activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );

        let baizhu = find("baizhu").expect("Baizhu talent buffs should exist");
        let baizhu_c4: Vec<_> = baizhu
            .iter()
            .filter(|b| b.source == TalentBuffSource::Constellation(4))
            .collect();
        assert_eq!(baizhu_c4.len(), 1);
        assert_eq!(
            baizhu_c4[0].activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );

        let collei = find("collei").expect("Collei talent buffs should exist");
        let collei_c4: Vec<_> = collei
            .iter()
            .filter(|b| b.source == TalentBuffSource::Constellation(4))
            .collect();
        assert_eq!(collei_c4.len(), 1);
        assert_eq!(collei_c4[0].target, BuffTarget::TeamExcludeSelf);

        let kirara = find("kirara").expect("Kirara talent buffs should exist");
        let kirara_c6: Vec<_> = kirara
            .iter()
            .filter(|b| b.source == TalentBuffSource::Constellation(6))
            .collect();
        assert_eq!(kirara_c6.len(), 7);
        assert!(kirara_c6.iter().all(|b| {
            b.activation == Some(Activation::Manual(ManualCondition::Toggle))
                && matches!(
                    b.stat,
                    BuffableStat::ElementalDmgBonus(Element::Pyro)
                        | BuffableStat::ElementalDmgBonus(Element::Hydro)
                        | BuffableStat::ElementalDmgBonus(Element::Electro)
                        | BuffableStat::ElementalDmgBonus(Element::Cryo)
                        | BuffableStat::ElementalDmgBonus(Element::Dendro)
                        | BuffableStat::ElementalDmgBonus(Element::Anemo)
                        | BuffableStat::ElementalDmgBonus(Element::Geo)
                )
        }));

        let yaoyao = find("yaoyao").expect("Yaoyao talent buffs should exist");
        let yaoyao_c4: Vec<_> = yaoyao
            .iter()
            .filter(|b| b.source == TalentBuffSource::Constellation(4))
            .collect();
        assert_eq!(yaoyao_c4.len(), 1);
        assert!((yaoyao_c4[0].base_value - 0.003).abs() < 1e-9);
        assert_eq!(yaoyao_c4[0].scales_on, Some(ScalingStat::Hp));
    }
}
