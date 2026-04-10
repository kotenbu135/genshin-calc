use super::*;
use genshin_calc_core::Reaction;

// ===== Aino =====
// C1: EM+80 to Aino and active party member after skill/burst for 15s
// A4: Structured Power Booster — Burst DMG increased by 50% of EM
// C6: The Burden of Creative Genius — Reaction DMG +15% for 15s after Burst
static AINO_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Aino C1 EM Share",
        description: desc!("After Skill or Burst, Aino and active character gain EM+80 for 15s"),
        stat: BuffableStat::ElementalMastery,
        base_value: 80.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Aino A4 Burst DMG from EM",
        description: desc!("Burst DMG increased by 50% of Elemental Mastery"),
        stat: BuffableStat::BurstFlatDmg,
        base_value: 0.50,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    // C6 "The Burden of Creative Genius": Reaction DMG bonus varies by Moonsign level.
    // Lv1 (NascentGleam): +15%, Lv2 (AscendantGleam): +35%
    // Moved to MoonsignTalentEnhancement in moonsign_chars.rs (not a fixed talent buff).
];

// ===== Barbara =====
// C2 "Vitality Burst": Hydro DMG+15% during skill
static BARBARA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Vitality Burst",
    description: desc!("During skill, active character gains Hydro DMG Bonus +15%"),
    stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: None,
}];

// ===== Candace =====
// A4 "Celestial Dome of Sand": Normal ATK DMG +0.5% per 1000 Max HP
// Burst "Sacred Rite: Heron's Sanctum": Normal ATK DMG bonus per level (Lv1-15)
// C2 "The Forgiving Stars": Max HP +20% for 15s when Skill hits opponents
static CANDACE_BURST_NORMAL_SCALING: [f64; 15] = [
    0.20, 0.20, 0.20, 0.20, 0.20, 0.20, 0.20, 0.20, 0.20, 0.20, 0.20, 0.20, 0.20, 0.20, 0.20,
];

static CANDACE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Celestial Dome of Sand",
        description: desc!("A4: Normal Attack DMG +0.5% per 1000 Max HP"),
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.000005,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Sacred Rite: Wagtail's Tide",
        description: desc!("Normal ATK DMG Bonus based on burst talent level"),
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&CANDACE_BURST_NORMAL_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "candace_c2_hp",
        description: desc!("C2: Max HP +20% for 15s when Skill hits"),
        stat: BuffableStat::HpPercent,
        base_value: 0.20,
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

// ===== Furina =====
// Elemental Burst "Let the People Rejoice": Fanfare stacks grant DMG bonus
// Per-point DMG Bonus coefficient (Lv1-15): each fanfare point grants this much DMG bonus
// C0 max fanfare: 300pt, C1+ max fanfare: 400pt (C1 adds +100)
// Split into two defs: C0 base (per_point, max 300) + C1 extra (per_point, max 300)
// Values are per-stack: multiply by stack count to get total
static FURINA_BURST_PER_POINT: [f64; 15] = [
    0.0007, 0.0009, 0.0011, 0.0013, 0.0015, 0.0017, 0.0019, 0.0021, 0.0023, 0.0025, 0.0027, 0.0029,
    0.0031, 0.0033, 0.0035,
];
// C1 extra: Toggle activation grants fixed bonus equal to 100 × per_point
// (the extra 100pt fanfare capacity from C1)
static FURINA_BURST_C1_BONUS: [f64; 15] = [
    0.07, 0.09, 0.11, 0.13, 0.15, 0.17, 0.19, 0.21, 0.23, 0.25, 0.27, 0.29, 0.31, 0.33, 0.35,
];

static FURINA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Let the People Rejoice DMG Bonus (C0 300pt)",
        description: desc!("Max fanfare (300pt) DMG bonus based on burst talent level"),
        stat: BuffableStat::DmgBonus,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&FURINA_BURST_PER_POINT),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(300))),
    },
    TalentBuffDef {
        name: "furina_c2_hp",
        description: desc!("C2: Furina Max HP +140% while burst is active"),
        stat: BuffableStat::HpPercent,
        base_value: 1.40,
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
        name: "Let the People Rejoice DMG Bonus (C1+ extra 100pt)",
        description: desc!("C1 extra fanfare (+100pt) DMG bonus based on burst talent level"),
        stat: BuffableStat::DmgBonus,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&FURINA_BURST_C1_BONUS),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Furina C6 Normal ATK Flat DMG",
        description: desc!(
            "C6: Normal Attacks deal additional DMG equal to 18% of Furina's Max HP"
        ),
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 0.18,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Furina C6 Charged ATK Flat DMG",
        description: desc!(
            "C6: Charged Attacks deal additional DMG equal to 18% of Furina's Max HP"
        ),
        stat: BuffableStat::ChargedAtkFlatDmg,
        base_value: 0.18,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Furina C6 Plunging ATK Flat DMG",
        description: desc!(
            "C6: Plunging Attacks deal additional DMG equal to 18% of Furina's Max HP"
        ),
        stat: BuffableStat::PlungingAtkFlatDmg,
        base_value: 0.18,
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

// ===== Mona =====
// Elemental Burst "Stellaris Phantasm": DMG bonus from Omen (Lv1-15)
// C1 "Prophecy of Submersion": Reaction DMG +15% vs Omen-affected opponents
// C2 "Lunar Chain": Party EM +80 on Charged ATK hit
// C4 "Prophecy of Oblivion": CRIT Rate +15% and CRIT DMG +15% vs Omen-affected opponents
// C6 "Rhetorics of Calamitas": Charged ATK DMG +180% (max in Illusory Torrent)
static MONA_BURST_DMG_SCALING: [f64; 15] = [
    0.42, 0.44, 0.46, 0.48, 0.50, 0.52, 0.54, 0.56, 0.58, 0.60, 0.60, 0.60, 0.60, 0.60, 0.60,
];

static MONA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Stellaris Phantasm DMG Bonus",
        description: desc!("Omen increases DMG taken by opponents"),
        stat: BuffableStat::DmgBonus,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&MONA_BURST_DMG_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "mona_c1_reaction_dmg",
        description: desc!("C1: Reaction DMG +15% vs Omen-affected opponents"),
        stat: BuffableStat::TransformativeBonus,
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
        name: "mona_c2_em",
        description: desc!("C2: Party EM +80 on Charged ATK hit"),
        stat: BuffableStat::ElementalMastery,
        base_value: 80.0,
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
        name: "mona_c4_crit_rate",
        description: desc!("C4: CRIT Rate +15% vs Omen-affected opponents"),
        stat: BuffableStat::CritRate,
        base_value: 0.15,
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
        name: "mona_c4_crit_dmg",
        description: desc!("C4: CRIT DMG +15% vs Omen-affected opponents (approximation)"),
        stat: BuffableStat::CritDmg,
        base_value: 0.15,
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
        name: "mona_c6_charged_dmg",
        description: desc!("C6: Charged ATK DMG +180% (max in Illusory Torrent)"),
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 1.80,
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

// ===== Nilou =====
// A2 passive "Dreaming Dance of the Lotuslight":
// For every 1000 HP above 30000, Bloom DMG +9%, max +400%
// Formula: min(floor((total_hp - 30000) / 1000) * 0.09, 4.0)
// C2 "The Starry Skies Their Flowers Rain": Hydro RES -35% and Dendro RES -35% after relevant DMG
// C4 "Elegy of Rites": Dance of Abzendegi DMG +50% after 3rd dance step
// C6 "Frostbreaker's Melody": CRIT Rate +0.6% and CRIT DMG +1.2% per 1000 Max HP (max +30%/+60%)
static NILOU_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Dreaming Dance of the Lotuslight",
        description: desc!("HP30000超過1000ごとにBloom DMG+9%、最大+400%"),
        stat: BuffableStat::ReactionDmgBonus(Reaction::Bloom),
        base_value: 0.0, // HP-dependent special formula (scaling_value == 0.0 triggers it)
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(4.0),
        activation: None,
    },
    TalentBuffDef {
        name: "nilou_c2_hydro_res_shred",
        description: desc!("C2: Opponents' Hydro RES -35% after Hydro DMG"),
        stat: BuffableStat::ElementalResReduction(Element::Hydro),
        base_value: 0.35,
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
        name: "nilou_c2_dendro_res_shred",
        description: desc!("C2: Opponents' Dendro RES -35% after Bloom DMG"),
        stat: BuffableStat::ElementalResReduction(Element::Dendro),
        base_value: 0.35,
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
        name: "nilou_c4_burst_dmg",
        description: desc!("C4: Dance of Abzendegi DMG +50% after 3rd dance step"),
        stat: BuffableStat::BurstDmgBonus,
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
        name: "nilou_c6_crit_rate",
        description: desc!("C6: CRIT Rate +0.6% per 1000 Max HP (max +30%)"),
        stat: BuffableStat::CritRate,
        base_value: 0.006,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: Some(0.30),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "nilou_c6_crit_dmg",
        description: desc!("C6: CRIT DMG +1.2% per 1000 Max HP (max +60%)"),
        stat: BuffableStat::CritDmg,
        base_value: 0.012,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: Some(0.60),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Yelan =====
// A4 passive "Adapt With Ease": DMG bonus ramp 1-50% (max value 0.50)
// C4 "Brocaded Nets": Party Max HP +10% per marked opponent (max 4 marks)
static YELAN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Adapt With Ease",
        description: desc!("DMG Bonus ramps up to max value 0.50"),
        stat: BuffableStat::DmgBonus,
        base_value: 0.50,
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
        name: "yelan_c4_hp",
        description: desc!("C4: Party Max HP +10% per marked opponent (max 4)"),
        stat: BuffableStat::HpPercent,
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(4))),
    },
];

// ===== Columbina =====
// Each constellation grants passive Lunar Reaction DMG bonus (TransformativeBonus)
// C1: +1.5%, C2: +7%, C3: +1.5%, C4: +1.5%, C5: +1.5%, C6: +7%
// C2 also: Lunar Brilliance — HP+40% for 8s
// C6 also: Corresponding elemental CritDMG +80% for 8s
static COLUMBINA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Columbina C1 Lunar Reaction DMG",
        description: desc!("C1: Lunar Reaction DMG +1.5%"),
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.015,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Columbina C2 Lunar Brilliance HP",
        description: desc!("C2: Lunar Brilliance — Columbina HP +40% for 8s (self only)"),
        stat: BuffableStat::HpPercent,
        base_value: 0.40,
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
        name: "Columbina C2 Lunar Reaction DMG",
        description: desc!("C2: Lunar Reaction DMG +7%"),
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.07,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Columbina C3 Lunar Reaction DMG",
        description: desc!("C3: Lunar Reaction DMG +1.5%"),
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.015,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(3),
        min_constellation: 3,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Columbina C4 Lunar Reaction DMG",
        description: desc!("C4: Lunar Reaction DMG +1.5%"),
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.015,
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
        name: "Columbina C5 Lunar Reaction DMG",
        description: desc!("C5: Lunar Reaction DMG +1.5%"),
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.015,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(5),
        min_constellation: 5,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Columbina C6 Elemental CritDMG",
        description: desc!(
            "After Lunar reaction in domain, corresponding element CritDMG +80% for 8s"
        ),
        stat: BuffableStat::CritDmg,
        base_value: 0.80,
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
        name: "Columbina C6 Lunar Reaction DMG",
        description: desc!("C6: Lunar Reaction DMG +7%"),
        stat: BuffableStat::TransformativeBonus,
        base_value: 0.07,
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

// ===== Ayato =====
// Elemental Skill "Kamisato Art: Kyouka": Namisen at max stacks grants flat NA DMG scaling on HP
//   Simplified to max-stack total using the Lv1-15 table from the talent page (Toggle)
// C1 "Kyouka Fuushi": Shunsuiken (E) DMG +40% — approximated as NormalAtkDmgBonus
// C2 "World Source": MaxHP +50%
// C4 "Boundless Origin": team NA ATK SPD — ATK SPD not a buffable stat, skip
//   TODO: C4 team NA ATK SPD +15% not implementable (no AtkSpd BuffableStat)
static AYATO_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Namisen Max Stacks Flat DMG",
        description: desc!(
            "Elemental Skill: Namisen at max stacks grants additional flat DMG based on Ayato's Max HP"
        ),
        stat: BuffableStat::NormalAtkFlatDmg,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&[
            0.0224, 0.0244, 0.0260, 0.0288, 0.0304, 0.0328, 0.0356, 0.0384, 0.0412, 0.0444, 0.0476,
            0.0508, 0.0536, 0.0568, 0.0600,
        ]),
        scales_on: Some(ScalingStat::Hp),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::ElementalSkill,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Kyouka Fuushi Shunsuiken DMG",
        description: desc!("C1: Shunsuiken DMG +40% (approximated as Normal ATK DMG Bonus)"),
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.40,
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
        name: "World Source Max HP",
        description: desc!("C2: Max HP +50%"),
        stat: BuffableStat::HpPercent,
        base_value: 0.50,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: None,
    },
];

// ===== Dahlia =====
// A4 "Flowering Rainforest": ATK SPD — not a buffable stat, skip
//   TODO: A4 ATK SPD buff not implementable (no AtkSpd BuffableStat)
// C2 "Petal Rain": Shield Strength +25%
// C6 "Floral Tempest": ATK SPD — not a buffable stat, skip
//   TODO: C6 ATK SPD buff not implementable (no AtkSpd BuffableStat)
static DAHLIA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Petal Rain Shield Strength",
    description: desc!("C2: Shield Strength +25%"),
    stat: BuffableStat::ShieldStrength,
    base_value: 0.25,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: None,
}];

// ===== Kokomi =====
// A4 "Flawless Strategy": HealBonus→DMG conversion (15% of HealingBonus as DmgBonus)
//   HealingBonus is not a ScalingStat, too complex for TalentBuffDef — skip with TODO
//   TODO: A4 Flawless Strategy — 15% of HealingBonus converts to DmgBonus; requires ScalingStat::HealingBonus
// C4 "Retained Medicinal Efficacy": NA ATK SPD — not a buffable stat, skip
//   TODO: C4 NA ATK SPD +10% not implementable (no AtkSpd BuffableStat)
// C6 "Tamanooya's Casket": Hydro DMG +40% during Burst when NA/CA hit
static KOKOMI_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Tamanooya's Casket Hydro DMG",
    description: desc!("C6: Hydro DMG Bonus +40% during Burst when Normal/Charged Attacks hit"),
    stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
    base_value: 0.40,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
}];

// ===== Mualani =====
// C1 "The Shark Bites!": DMG +66% MaxHP — flat DMG scaling on HP, too complex — skip with TODO
//   TODO: C1 flat DMG +66% MaxHP scaling not implementable as TalentBuffDef (HpFlat not HP%)
// C4 "Sharky's Pal": Burst DMG +75%
static MUALANI_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Sharky's Pal Burst DMG",
    description: desc!("C4: Elemental Burst DMG +75%"),
    stat: BuffableStat::BurstDmgBonus,
    base_value: 0.75,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
    activation: None,
}];

// ===== Neuvillette =====
// A1 "Tidal Affinity": CA DMG Bonus at max Sourcewater Droplets — use max value 0.60 (Toggle)
// A4 "Heir to the Ancient Sea's Authority": Hydro DMG +30% from HP conditions (Toggle)
// C2 "The Law's Final Remains": CRIT DMG +14%/stack, max 3 stacks (+42% total)
static NEUVILLETTE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Tidal Affinity CA DMG Bonus",
        description: desc!("A1: CA DMG Bonus at 3 Sourcewater Droplets (max value +60%)"),
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.60,
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
        name: "Heir to the Ancient Sea's Authority Hydro DMG",
        description: desc!("A4: Hydro DMG Bonus +30% when HP conditions are met"),
        stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
        base_value: 0.30,
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
        name: "The Law's Final Remains CRIT DMG",
        description: desc!("C2: CRIT DMG +14% per stack, max 3 stacks (+42%)"),
        stat: BuffableStat::CritDmg,
        base_value: 0.14,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
    },
];

// ===== Sigewinne =====
// A1 "A Friendly Rivalry": Hydro DMG +8% for team (after Skill)
//   HP→flat DMG part is too complex — skip with TODO
//   TODO: A1 HP-scaling flat DMG bonus not implementable as TalentBuffDef
// C2 "Targeted Treatment": enemy Hydro RES -35% (Team, Toggle)
// C6 "Whirlpool Wisdom": CR +20% / CD +110% from HP scaling — use max values (self, Toggle)
static SIGEWINNE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Requires Appropriate Rest Hydro DMG",
        description: desc!("A1: After Skill, self Hydro DMG Bonus +8%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
        base_value: 0.08,
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
        name: "Targeted Treatment Hydro RES Shred",
        description: desc!("C2: Enemy Hydro RES -35%"),
        stat: BuffableStat::ElementalResReduction(Element::Hydro),
        base_value: 0.35,
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
        name: "Whirlpool Wisdom CRIT Rate",
        description: desc!("C6: CRIT Rate +20% (max value from HP scaling)"),
        stat: BuffableStat::CritRate,
        base_value: 0.20,
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
        name: "Whirlpool Wisdom CRIT DMG",
        description: desc!("C6: CRIT DMG +110% (max value from HP scaling)"),
        stat: BuffableStat::CritDmg,
        base_value: 1.10,
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

// ===== Xingqiu =====
// C2: Hydro RES -15% on Rain Sword hit
// C4 "Evilsoother": Skill DMG +50% during Burst
static XINGQIU_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Rainbow Upon the Azure Sky Hydro RES Shred",
        description: desc!("C2: Enemies hit by Rain Swords have Hydro RES -15% for 4s"),
        stat: BuffableStat::ElementalResReduction(Element::Hydro),
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "xingqiu_c4_skill_dmg",
        description: desc!("C4: Skill DMG +50% during Burst"),
        stat: BuffableStat::SkillDmgBonus,
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
];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static HYDRO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("aino", AINO_BUFFS),
    ("ayato", AYATO_BUFFS),
    ("barbara", BARBARA_BUFFS),
    ("candace", CANDACE_BUFFS),
    ("columbina", COLUMBINA_BUFFS),
    ("dahlia", DAHLIA_BUFFS),
    ("furina", FURINA_BUFFS),
    ("kokomi", KOKOMI_BUFFS),
    ("mona", MONA_BUFFS),
    ("mualani", MUALANI_BUFFS),
    ("neuvillette", NEUVILLETTE_BUFFS),
    ("nilou", NILOU_BUFFS),
    ("sigewinne", SIGEWINNE_BUFFS),
    ("xingqiu", XINGQIU_BUFFS),
    ("yelan", YELAN_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    HYDRO_TALENT_BUFFS
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
    fn test_find_candace_a4_team_normal_attack_damage_bonus() {
        let buffs = find("candace").expect("Candace talent buffs should exist");
        let buff = find_buff(
            buffs,
            BuffableStat::NormalAtkDmgBonus,
            TalentBuffSource::AscensionPassive(4),
        );
        assert_eq!(buff.target, BuffTarget::Team);
        assert_eq!(
            buff.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );
        assert_eq!(buff.scales_on, Some(ScalingStat::Hp));
        assert!((buff.base_value - 0.000005).abs() < 1e-12);
    }

    #[test]
    fn test_find_ayato_a1_normal_attack_flat_damage_scaling() {
        let buffs = find("ayato").expect("Ayato talent buffs should exist");
        let buff = find_buff(
            buffs,
            BuffableStat::NormalAtkFlatDmg,
            TalentBuffSource::ElementalSkill,
        );
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert_eq!(
            buff.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );
        assert_eq!(buff.scales_on, Some(ScalingStat::Hp));
        assert!(buff.scales_with_talent);
        let scaling = buff.talent_scaling.expect("expected scaling table");
        let expected = [
            0.0224, 0.0244, 0.0260, 0.0288, 0.0304, 0.0328, 0.0356, 0.0384, 0.0412, 0.0444, 0.0476,
            0.0508, 0.0536, 0.0568, 0.0600,
        ];
        for (actual, expected) in scaling.iter().zip(expected.iter()) {
            assert!((actual - expected).abs() < 1e-6);
        }
    }

    #[test]
    fn test_find_furina_c2_and_c6_buffs() {
        let buffs = find("furina").expect("Furina talent buffs should exist");
        let c2 = find_buff(
            buffs,
            BuffableStat::HpPercent,
            TalentBuffSource::Constellation(2),
        );
        assert_eq!(c2.target, BuffTarget::OnlySelf);
        assert_eq!(
            c2.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );
        assert!((c2.base_value - 1.40).abs() < 1e-6);

        for stat in [
            BuffableStat::NormalAtkFlatDmg,
            BuffableStat::ChargedAtkFlatDmg,
            BuffableStat::PlungingAtkFlatDmg,
        ] {
            let buff = find_buff(buffs, stat, TalentBuffSource::Constellation(6));
            assert_eq!(buff.target, BuffTarget::OnlySelf);
            assert_eq!(
                buff.activation,
                Some(Activation::Manual(ManualCondition::Toggle))
            );
            assert_eq!(buff.scales_on, Some(ScalingStat::Hp));
            assert!((buff.base_value - 0.18).abs() < 1e-6);
        }
    }
}
