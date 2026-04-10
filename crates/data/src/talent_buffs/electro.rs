use super::*;

// ===== Ineffa =====
// A4 passive: EM share based on total ATK (6% of ATK)
// C1: Party Lunar-Charged DMG +2.5% per 100 ATK (max +50%)
static INEFFA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Ineffa A4 EM Share",
        description: desc!("Grants EM equal to 6% of Ineffa's total ATK"),
        stat: BuffableStat::ElementalMastery,
        base_value: 0.06,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::TotalAtk),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "ineffa_c1_lunar_charged_dmg",
        description: desc!("C1: Party Lunar-Charged DMG +2.5% per 100 ATK (max +50%)"),
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.025,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::TotalAtk),
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: Some(0.50),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Sara =====
// Elemental Skill/Burst "Tengu Juurai": ATK bonus based on Sara's Base ATK (Lv1-15)
static SARA_ATK_SCALING: [f64; 15] = [
    0.4296, 0.4618, 0.4940, 0.5370, 0.5692, 0.6014, 0.6444, 0.6874, 0.7304, 0.7734, 0.8164, 0.8594,
    0.9131, 0.9668, 1.0206,
];

static SARA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Tengu Juurai ATK Bonus",
        description: desc!("ATK bonus based on Sara's Base ATK"),
        stat: BuffableStat::AtkFlat,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&SARA_ATK_SCALING),
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Sin of Pride",
        description: desc!(
            "Electro CRIT DMG +60% (approximated as generic CritDmg; Electro-only in game)"
        ),
        stat: BuffableStat::CritDmg,
        base_value: 0.60,
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

// ===== Lisa =====
// A4 "Static Electricity Field": Enemies hit by burst have DEF -15% for 10s
static LISA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Static Electricity Field",
    description: desc!("Enemies hit by Lightning Rose have DEF -15% for 10s"),
    stat: BuffableStat::DefReduction,
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive(4),
    min_constellation: 0,
    cap: None,
    activation: None,
}];

// ===== Flins =====
// A4 passive "Whispering Flame": EM += total ATK × 0.08, capped at 160
// C4 "Night on Bald Mountain": ATK +20%
// C2: Opponents' Electro RES -25% during Ascendant Gleam Moonsign
// C6: Flins's Lunar-Charged DMG +35%, Party Lunar-Charged DMG +10% during Moonsign
static FLINS_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Whispering Flame EM Bonus",
        description: desc!("A4: Flins gains EM equal to 8% of her total ATK (max 160)"),
        stat: BuffableStat::ElementalMastery,
        base_value: 0.08,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::TotalAtk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(160.0),
        activation: None,
    },
    TalentBuffDef {
        name: "Night on Bald Mountain ATK Bonus",
        description: desc!("C4: Flins gains ATK +20%"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: None,
    },
    // C4 also enhances A4: 8%→10% (+2% delta), cap 160→220 (+60 delta)
    TalentBuffDef {
        name: "Night on Bald Mountain A4 EM Enhancement",
        description: desc!(
            "C4: A4 EM coefficient enhanced from 8% to 10% (delta +2%, cap raised to 220)"
        ),
        stat: BuffableStat::ElementalMastery,
        base_value: 0.02,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::TotalAtk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: Some(60.0),
        activation: None,
    },
    TalentBuffDef {
        name: "flins_c2_electro_res_shred",
        description: desc!("C2: Opponents' Electro RES -25% during Ascendant Gleam Moonsign"),
        stat: BuffableStat::ElementalResReduction(Element::Electro),
        base_value: 0.25,
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
        name: "flins_c6_lunar_charged_self",
        description: desc!("C6: Flins's Lunar-Charged DMG +35%"),
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.35,
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
        name: "flins_c6_lunar_charged_team",
        description: desc!("C6: Party Lunar-Charged DMG +10% during Moonsign"),
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.10,
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

// ===== Raiden Shogun =====
// Skill "Eye of Stormy Judgment": BurstDmgBonus per energy cost point per skill level
// Value = coefficient per burst cost point; frontend multiplies by target's burst cost
// C4 "Pledge of Propriety": ATK+30% to nearby party after burst ends
static RAIDEN_SKILL_BURST_BONUS_SCALING: [f64; 15] = [
    0.0022, 0.0024, 0.0026, 0.0028, 0.0030, 0.0032, 0.0035, 0.0038, 0.0041, 0.0044, 0.0047, 0.0050,
    0.0053, 0.0056, 0.0059,
];

static RAIDEN_SHOGUN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Eye of Stormy Judgment BurstDmgBonus",
        description: desc!(
            "Skill: BurstDmgBonus coefficient per energy cost point (multiply by target burst cost)"
        ),
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&RAIDEN_SKILL_BURST_BONUS_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    // C2 "Steelbreaker": During Musou Isshin, ignore 60% of enemy DEF
    TalentBuffDef {
        name: "Steelbreaker DEF Ignore",
        description: desc!("C2: During Musou Isshin state, ignore 60% of enemy DEF"),
        stat: BuffableStat::DefIgnore,
        base_value: 0.60,
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
        name: "Pledge of Propriety ATK Bonus",
        description: desc!(
            "C4: After Musou Isshin ends, nearby party members gain ATK+30% for 10s"
        ),
        stat: BuffableStat::AtkPercent,
        base_value: 0.30,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::TeamExcludeSelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: None,
    },
];

// ===== Beidou =====
// A4 "Stunning Revenge": Normal/Charged ATK DMG +15% after being attacked
// C4 "Stunning Revenge": Normal ATK Electro DMG Bonus +20% on hit
// C6 "Bane of Evil": Electro RES -15% during burst
static BEIDOU_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "beidou_a4_normal_dmg",
        description: desc!("A4: Normal Attack DMG Bonus +15%"),
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.15,
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
        name: "beidou_a4_charged_dmg",
        description: desc!("A4: Charged Attack DMG Bonus +15%"),
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.15,
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
        name: "Bane of Evil Electro RES Shred",
        description: desc!("C6: During Stormbreaker, enemies have Electro RES -15%"),
        stat: BuffableStat::ElementalResReduction(Element::Electro),
        base_value: 0.15,
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
        name: "beidou_c4_electro_dmg",
        description: desc!("C4: Normal ATK Electro DMG Bonus +20% on hit"),
        stat: BuffableStat::ElementalDmgBonus(Element::Electro),
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

// ===== Fischl =====
// Hexerei Witch's Eve Rite buffs; coordinated/proc attacks excluded
static FISCHL_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Witch's Eve Rite Hexerei ATK Bonus",
        description: desc!("After Overloaded while Oz is on field, Fischl gains ATK +22.5%"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.225,
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
        name: "Witch's Eve Rite Hexerei EM Bonus",
        description: desc!(
            "After Electro-Charged or Lunar-Charged while Oz is on field, Fischl gains EM +90"
        ),
        stat: BuffableStat::ElementalMastery,
        base_value: 90.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Iansan =====
// Burst "Three Principles of Power": ATK Flat bonus (Iansan ATK × coefficient) to party
// Max ATK Bonus: Lv1=330 ~ Lv15=890 (4-source confirmed: KQM, game8, genshin.gg, paimon.moe)
// A1 "Enhanced Resistance Training": Iansan self ATK +20% for 15s
// A4 "Kinetic Energy Gradient Test": HP recovery (ATK×60%) — not a stat buff, omitted
// C2: Off-field grants active character ATK +30%
// C6: Active character DMG Bonus +25% on Nightsoul overflow
static IANSAN_BURST_ATK_SCALING: [f64; 15] = [
    0.30, 0.3225, 0.345, 0.375, 0.3975, 0.42, 0.45, 0.48, 0.51, 0.54, 0.57, 0.60, 0.6375, 0.675,
    0.7125,
];

static IANSAN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Three Principles AtkFlat",
        description: desc!(
            "Burst: Party gains flat ATK = Iansan ATK × coefficient (scales with burst level)"
        ),
        stat: BuffableStat::AtkFlat,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&IANSAN_BURST_ATK_SCALING),
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Enhanced Resistance Training ATK Bonus",
        description: desc!("A1: Iansan gains ATK +20% for 15s"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.20,
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
        name: "iansan_c2_atk",
        description: desc!("C2: Off-field grants active character ATK +30%"),
        stat: BuffableStat::AtkPercent,
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
        name: "iansan_c6_dmg_bonus",
        description: desc!("C6: Active character DMG Bonus +25% on Nightsoul overflow"),
        stat: BuffableStat::DmgBonus,
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

// ===== Clorinde =====
// A1: Electro flat DMG = 20% ATK/stack (self, Toggle)
// C2: Electro flat DMG = 10% ATK/stack (self, Toggle, cap 900 per attack)
// A4: CR+10% (self, Toggle — simplified from per-stack)
// C6: CR+10% / CD+70% (self, Toggle)
static CLORINDE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Clorinde A1 Electro Flat DMG",
        description: desc!(
            "A1: Normal Attack deals additional Electro DMG equal to 20% of ATK (per stack, max 1800)"
        ),
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: Some(1800.0),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Clorinde A1 Burst Flat DMG",
        description: desc!(
            "A1: Last Lightfall deals additional Electro DMG equal to 20% of ATK (max 1800)"
        ),
        stat: BuffableStat::BurstFlatDmg,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: Some(1800.0),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Clorinde C2 Normal Flat DMG",
        description: desc!(
            "C2: Normal Attacks deal additional Electro DMG equal to 10% of ATK (max 900)"
        ),
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: Some(900.0),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Clorinde C2 Burst Flat DMG",
        description: desc!(
            "C2: Last Lightfall deals additional Electro DMG equal to 10% of ATK (max 900)"
        ),
        stat: BuffableStat::BurstFlatDmg,
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: Some(900.0),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Clorinde A4 CRIT Rate Bonus",
        description: desc!("A4: CRIT Rate +10% while in Ominous Inscription state"),
        stat: BuffableStat::CritRate,
        base_value: 0.10,
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
        name: "Clorinde C6 CRIT Rate",
        description: desc!("C6: CRIT Rate +10%"),
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
        name: "Clorinde C6 CRIT DMG",
        description: desc!("C6: CRIT DMG +70%"),
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

// ===== Cyno =====
// A4: NA flat DMG = 150% EM, Skill flat DMG = 250% EM (self, Toggle)
// C2: Electro DMG+10%/stack max 5 (self, Stacks(5), min_constellation=2)
static CYNO_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Cyno A4 Normal Atk Flat DMG",
        description: desc!(
            "A4: During Pactsworn Pathclearer, Normal Attacks deal additional DMG equal to 150% of EM"
        ),
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 1.50,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Cyno A4 Skill Flat DMG",
        description: desc!(
            "A4: During Pactsworn Pathclearer, Skill deals additional DMG equal to 250% of EM"
        ),
        stat: BuffableStat::SkillFlatDmg,
        base_value: 2.50,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Cyno C2 Electro DMG Bonus",
        description: desc!("C2: Electro DMG Bonus +10% per stack, max 5 stacks (50% total)"),
        stat: BuffableStat::ElementalDmgBonus(Element::Electro),
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(5))),
    },
];

// ===== Keqing =====
// A4: CR+15% + ER+15% after burst infusion (self, Toggle)
// C4: ATK+25% on triggering Electro reaction (self, Toggle, min_constellation=4)
// C6: Electro DMG+24% max (self, Toggle, min_constellation=6)
static KEQING_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Keqing A4 CRIT Rate",
        description: desc!("A4: After using Starward Sword, CRIT Rate +15% for 8s"),
        stat: BuffableStat::CritRate,
        base_value: 0.15,
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
        name: "Keqing A4 Energy Recharge",
        description: desc!("A4: After using Starward Sword, Energy Recharge +15% for 8s"),
        stat: BuffableStat::EnergyRecharge,
        base_value: 0.15,
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
        name: "Keqing C4 ATK Bonus",
        description: desc!("C4: Triggering an Electro-related reaction grants ATK +25% for 10s"),
        stat: BuffableStat::AtkPercent,
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
    TalentBuffDef {
        name: "Keqing C6 Electro DMG Bonus",
        description: desc!(
            "C6: Electro DMG Bonus +6% per Electro-related reaction triggered, max 4 stacks (24% total)"
        ),
        stat: BuffableStat::ElementalDmgBonus(Element::Electro),
        base_value: 0.24,
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

// ===== Yae Miko =====
// A4: Skill DMG +0.15% per point of EM (self, always active via scaling)
// C4: Team Electro DMG +20% (Team, min_constellation=4)
// C6: Sesshou Sakura attacks ignore 60% of opponents' DEF
static YAE_MIKO_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Yae Miko A4 Skill DMG Bonus",
        description: desc!(
            "A4: Sesshou Sakura DMG increased by 0.15% for every point of Elemental Mastery"
        ),
        stat: BuffableStat::SkillDmgBonus,
        base_value: 0.0015,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Yae Miko C4 Electro DMG Bonus",
        description: desc!("C4: All nearby party members' Electro DMG Bonus increased by 20%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Electro),
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
        name: "yae_miko_c6_def_ignore",
        description: desc!("C6: Sesshou Sakura attacks ignore 60% of opponents' DEF"),
        stat: BuffableStat::DefIgnore,
        base_value: 0.60,
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

// ===== Dori =====
// C4: Healing Bonus +50% when HP<50% (self, Toggle, min_constellation=4)
// C4: ER+30% when Energy<50% (self, Toggle, min_constellation=4)
static DORI_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Dori C4 Healing Bonus",
        description: desc!("C4: When HP falls below 50%, Healing Bonus +50%"),
        stat: BuffableStat::HealingBonus,
        base_value: 0.50,
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
        name: "Dori C4 Energy Recharge",
        description: desc!("C4: When Energy falls below 50%, Energy Recharge +30%"),
        stat: BuffableStat::EnergyRecharge,
        base_value: 0.30,
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

// ===== Kuki Shinobu =====
// A1: Healing Bonus +15% when HP<=50% (self, Toggle)
// A4: Skill flat DMG = 25% EM (self, always active)
// C6: EM +150 (self, Toggle, min_constellation=6)
static KUKI_SHINOBU_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Kuki Shinobu A1 Healing Bonus",
        description: desc!("A1: When HP is at or below 50%, Healing Bonus +15%"),
        stat: BuffableStat::HealingBonus,
        base_value: 0.15,
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
        name: "Kuki Shinobu A4 Skill Flat DMG",
        description: desc!("A4: Sanctifying Ring deals additional DMG equal to 25% of Kuki's EM"),
        stat: BuffableStat::SkillFlatDmg,
        base_value: 0.25,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Kuki Shinobu C6 Elemental Mastery",
        description: desc!("C6: Elemental Mastery +150 for 15s after using Elemental Skill"),
        stat: BuffableStat::ElementalMastery,
        base_value: 150.0,
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

// ===== Razor =====
// C1: DMG +10% after picking up Electro Sigil (self, Toggle)
// C2: CR +10% vs HP<30% enemies (self, Toggle, min_constellation=2)
// Hexerei burst enhancement: Burst flat DMG +70% ATK (self, Toggle)
// C6: CRIT Rate +10% / CRIT DMG +50% (self, Toggle)
static RAZOR_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Razor C1 DMG Bonus",
        description: desc!(
            "C1: After picking up an Electro Sigil from Wolf's Instinct, DMG +10% for 8s"
        ),
        stat: BuffableStat::DmgBonus,
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
    TalentBuffDef {
        name: "Razor C2 CRIT Rate vs Low HP",
        description: desc!("C2: CRIT Rate +10% against enemies with HP below 30%"),
        stat: BuffableStat::CritRate,
        base_value: 0.10,
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
        name: "Razor Hexerei Burst Flat DMG",
        description: desc!("Hexerei: Wolf Within's DMG increased by 70% of ATK"),
        stat: BuffableStat::BurstFlatDmg,
        base_value: 0.70,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Razor C6 CRIT Rate",
        description: desc!("C6: CRIT Rate +10% for 15s after using Burst"),
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
        name: "Razor C6 CRIT DMG",
        description: desc!("C6: CRIT DMG +50% for 15s after using Burst"),
        stat: BuffableStat::CritDmg,
        base_value: 0.50,
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

// ===== Sethos =====
// A4: CA flat DMG = 700% EM (self, Toggle)
// C1: CR +15% (self, Toggle, min_constellation=1)
// C2: Electro DMG +15%/stack max 2 (self, Stacks(2), min_constellation=2)
// C4: Team EM +80 (Team, min_constellation=4)
static SETHOS_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Sethos A4 Charged Atk Flat DMG",
        description: desc!("A4: Charged Attack deals additional DMG equal to 700% of EM"),
        stat: BuffableStat::ChargedAtkFlatDmg,
        base_value: 7.00,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Sethos C1 CRIT Rate",
        description: desc!("C1: CRIT Rate +15% for Charged Attacks"),
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
        name: "Sethos C2 Electro DMG Bonus",
        description: desc!("C2: Electro DMG Bonus +15% per stack, max 2 stacks (30% total)"),
        stat: BuffableStat::ElementalDmgBonus(Element::Electro),
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(2))),
    },
    TalentBuffDef {
        name: "Sethos C4 Team Elemental Mastery",
        description: desc!("C4: All nearby party members gain EM +80"),
        stat: BuffableStat::ElementalMastery,
        base_value: 80.0,
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

// ===== Varesa =====
// A1: Plunge flat DMG = 180% ATK (max value, self, Toggle)
// A4: ATK +35%/stack max 2 (self, Stacks(2))
// C6: CR +10% / CD +100% (self, Toggle, min_constellation=6)
static VARESA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Varesa A1 Plunging Atk Flat DMG",
        description: desc!(
            "A1: Plunging Attack deals additional DMG equal to 180% of ATK (max value)"
        ),
        stat: BuffableStat::PlungingAtkFlatDmg,
        base_value: 1.80,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Atk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Varesa A4 ATK Bonus",
        description: desc!("A4: ATK +35% per stack, max 2 stacks (70% total)"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.35,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(2))),
    },
    TalentBuffDef {
        name: "Varesa C6 CRIT Rate",
        description: desc!("C6: CRIT Rate +10%"),
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
        name: "Varesa C6 CRIT DMG",
        description: desc!("C6: CRIT DMG +100%"),
        stat: BuffableStat::CritDmg,
        base_value: 1.00,
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

// ===== Ororon =====
// C2: Electro DMG +32% max (self, Toggle, min_constellation=2)
// C6: Team ATK +10%/stack max 3 (Team, Stacks(3), min_constellation=6)
static ORORON_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Ororon C2 Electro DMG Bonus",
        description: desc!("C2: Electro DMG Bonus +32% (max value)"),
        stat: BuffableStat::ElementalDmgBonus(Element::Electro),
        base_value: 0.32,
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
        name: "Ororon C6 Team ATK Bonus",
        description: desc!(
            "C6: Active character gains ATK +10% per stack, max 3 stacks (30% total)"
        ),
        stat: BuffableStat::AtkPercent,
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
    },
];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static ELECTRO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("beidou", BEIDOU_BUFFS),
    ("clorinde", CLORINDE_BUFFS),
    ("cyno", CYNO_BUFFS),
    ("dori", DORI_BUFFS),
    ("flins", FLINS_BUFFS),
    ("fischl", FISCHL_BUFFS),
    ("iansan", IANSAN_BUFFS),
    ("ineffa", INEFFA_BUFFS),
    ("keqing", KEQING_BUFFS),
    ("kujou_sara", SARA_BUFFS),
    ("kuki_shinobu", KUKI_SHINOBU_BUFFS),
    ("lisa", LISA_BUFFS),
    ("ororon", ORORON_BUFFS),
    ("raiden_shogun", RAIDEN_SHOGUN_BUFFS),
    ("razor", RAZOR_BUFFS),
    ("sethos", SETHOS_BUFFS),
    ("varesa", VARESA_BUFFS),
    ("yae_miko", YAE_MIKO_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    ELECTRO_TALENT_BUFFS
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
    fn test_find_beidou_a4_normal_and_charged_dmg_bonus() {
        let buffs = find("beidou").expect("Beidou talent buffs should exist");
        let normal = find_buff(
            buffs,
            BuffableStat::NormalAtkDmgBonus,
            TalentBuffSource::AscensionPassive(4),
        );
        let charged = find_buff(
            buffs,
            BuffableStat::ChargedAtkDmgBonus,
            TalentBuffSource::AscensionPassive(4),
        );
        for buff in [normal, charged] {
            assert_eq!(buff.target, BuffTarget::OnlySelf);
            assert_eq!(
                buff.activation,
                Some(Activation::Manual(ManualCondition::Toggle))
            );
            assert!((buff.base_value - 0.15).abs() < 1e-6);
        }
    }

    #[test]
    fn test_find_clorinde_a1_and_c2_flat_damage_buffs() {
        let buffs = find("clorinde").expect("Clorinde talent buffs should exist");
        let a1_normal = find_buff(
            buffs,
            BuffableStat::NormalAtkFlatDmg,
            TalentBuffSource::AscensionPassive(1),
        );
        let a1_burst = find_buff(
            buffs,
            BuffableStat::BurstFlatDmg,
            TalentBuffSource::AscensionPassive(1),
        );
        let c2_normal = find_buff(
            buffs,
            BuffableStat::NormalAtkFlatDmg,
            TalentBuffSource::Constellation(2),
        );
        let c2_burst = find_buff(
            buffs,
            BuffableStat::BurstFlatDmg,
            TalentBuffSource::Constellation(2),
        );

        for buff in [a1_normal, a1_burst] {
            assert_eq!(buff.target, BuffTarget::OnlySelf);
            assert_eq!(
                buff.activation,
                Some(Activation::Manual(ManualCondition::Toggle))
            );
            assert_eq!(buff.scales_on, Some(ScalingStat::Atk));
            assert!((buff.base_value - 0.20).abs() < 1e-6);
            assert_eq!(buff.cap, Some(1800.0));
        }
        for buff in [c2_normal, c2_burst] {
            assert_eq!(buff.target, BuffTarget::OnlySelf);
            assert_eq!(
                buff.activation,
                Some(Activation::Manual(ManualCondition::Toggle))
            );
            assert_eq!(buff.scales_on, Some(ScalingStat::Atk));
            assert!((buff.base_value - 0.10).abs() < 1e-6);
            assert_eq!(buff.cap, Some(900.0));
            assert_eq!(buff.min_constellation, 2);
        }
    }

    #[test]
    fn test_find_kuki_c6_is_manual_toggle() {
        let buffs = find("kuki_shinobu").expect("Kuki Shinobu talent buffs should exist");
        let buff = find_buff(
            buffs,
            BuffableStat::ElementalMastery,
            TalentBuffSource::Constellation(6),
        );
        assert_eq!(
            buff.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );
        assert!((buff.base_value - 150.0).abs() < 1e-6);
    }

    #[test]
    fn test_find_ororon_c2_electro_dmg_bonus() {
        let buffs = find("ororon").expect("Ororon talent buffs should exist");
        let buff = find_buff(
            buffs,
            BuffableStat::ElementalDmgBonus(Element::Electro),
            TalentBuffSource::Constellation(2),
        );
        assert!((buff.base_value - 0.32).abs() < 1e-6);
    }

    #[test]
    fn test_find_razor_hexerei_burst_and_c6_buffs() {
        let buffs = find("razor").expect("Razor talent buffs should exist");
        let burst = find_buff(
            buffs,
            BuffableStat::BurstFlatDmg,
            TalentBuffSource::ElementalBurst,
        );
        assert_eq!(burst.target, BuffTarget::OnlySelf);
        assert_eq!(
            burst.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );
        assert_eq!(burst.scales_on, Some(ScalingStat::Atk));
        assert!((burst.base_value - 0.70).abs() < 1e-6);

        let crit_rate = find_buff(
            buffs,
            BuffableStat::CritRate,
            TalentBuffSource::Constellation(6),
        );
        let crit_dmg = find_buff(
            buffs,
            BuffableStat::CritDmg,
            TalentBuffSource::Constellation(6),
        );
        for buff in [crit_rate, crit_dmg] {
            assert_eq!(buff.target, BuffTarget::OnlySelf);
            assert_eq!(
                buff.activation,
                Some(Activation::Manual(ManualCondition::Toggle))
            );
            assert_eq!(buff.source, TalentBuffSource::Constellation(6));
        }
        assert!((crit_rate.base_value - 0.10).abs() < 1e-6);
        assert!((crit_dmg.base_value - 0.50).abs() < 1e-6);
    }
}
