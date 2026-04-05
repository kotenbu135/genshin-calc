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
}];

// ===== Iansan =====
// Burst: NormalAtkDmgBonus per burst level
// A4: AtkFlat from HP (HP × 0.40 coefficient)
static IANSAN_BURST_NORMAL_SCALING: [f64; 15] = [
    0.30, 0.3225, 0.345, 0.375, 0.3975, 0.42, 0.45, 0.48, 0.51, 0.54, 0.57, 0.60, 0.6375, 0.675,
    0.7125,
];

static IANSAN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Three Principles NormalAtkDmgBonus",
        description: "Burst: Party Normal ATK DMG Bonus based on burst talent level",
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&IANSAN_BURST_NORMAL_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
    },
    TalentBuffDef {
        name: "The Law of Power ATK Bonus",
        description: "A4: Grants ATK Flat = HP × 0.40 coefficient (builder computes at resolve time)",
        stat: BuffableStat::AtkFlat,
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
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
