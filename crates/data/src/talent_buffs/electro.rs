use super::*;

// ===== Ineffa =====
// A4 passive: EM share based on total ATK (6% of ATK)
static INEFFA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Ineffa A4 EM Share",
    description: "Grants EM equal to 6% of Ineffa's total ATK",
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
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Sin of Pride",
        description: "Electro CRIT DMG +60% (approximated as generic CritDmg; Electro-only in game)",
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
    description: "Enemies hit by Lightning Rose have DEF -15% for 10s",
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
static FLINS_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Whispering Flame EM Bonus",
        description: "A4: Flins gains EM equal to 8% of her total ATK (max 160)",
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
        description: "C4: Flins gains ATK +20%",
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
        description: "C4: A4 EM coefficient enhanced from 8% to 10% (delta +2%, cap raised to 220)",
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
        description: "Skill: BurstDmgBonus coefficient per energy cost point (multiply by target burst cost)",
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
        description: "C2: During Musou Isshin state, ignore 60% of enemy DEF",
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
        description: "C4: After Musou Isshin ends, nearby party members gain ATK+30% for 10s",
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
// C6 "Bane of Evil": Electro RES -15% during burst
static BEIDOU_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Bane of Evil Electro RES Shred",
    description: "C6: During Stormbreaker, enemies have Electro RES -15%",
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
}];

// ===== Iansan =====
// Burst "Three Principles of Power": ATK Flat bonus (Iansan ATK × coefficient) to party
// Max ATK Bonus: Lv1=330 ~ Lv15=890 (4-source confirmed: KQM, game8, genshin.gg, paimon.moe)
// A1 "Enhanced Resistance Training": Iansan self ATK +20% for 15s
// A4 "Kinetic Energy Gradient Test": HP recovery (ATK×60%) — not a stat buff, omitted
static IANSAN_BURST_ATK_SCALING: [f64; 15] = [
    0.30, 0.3225, 0.345, 0.375, 0.3975, 0.42, 0.45, 0.48, 0.51, 0.54, 0.57, 0.60, 0.6375, 0.675,
    0.7125,
];

static IANSAN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Three Principles AtkFlat",
        description: "Burst: Party gains flat ATK = Iansan ATK × coefficient (scales with burst level)",
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
        description: "A1: Iansan gains ATK +20% for 15s",
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
];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static ELECTRO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("beidou", BEIDOU_BUFFS),
    ("flins", FLINS_BUFFS),
    ("iansan", IANSAN_BUFFS),
    ("ineffa", INEFFA_BUFFS),
    ("kujou_sara", SARA_BUFFS),
    ("lisa", LISA_BUFFS),
    ("raiden_shogun", RAIDEN_SHOGUN_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    ELECTRO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}
