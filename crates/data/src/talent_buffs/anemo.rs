use super::*;

// ===== Faruzan =====
// Burst "The Wind's Secret Ways": Anemo DMG bonus per level (Lv1-15)
// Values from honeyhunter-mirror/md/characters/faruzan_076.md (Prayerful Wind's Benefit)
static FARUZAN_BURST_ANEMO_SCALING: [f64; 15] = [
    0.18, 0.1935, 0.207, 0.225, 0.2385, 0.252, 0.27, 0.288, 0.306, 0.324, 0.342, 0.36, 0.3825,
    0.405, 0.4275,
];

static FARUZAN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Prayerful Wind's Benefit",
        description: desc!("Anemo DMG Bonus based on burst talent level"),
        stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&FARUZAN_BURST_ANEMO_SCALING),
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Perfidious Wind's Bale",
        description: desc!("A4: Enemies hit by Pressurized Collapse have Anemo RES -30%"),
        stat: BuffableStat::ElementalResReduction(Element::Anemo),
        base_value: 0.30,
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
        name: "faruzan_c6_anemo_crit_dmg",
        description: desc!("C6: Anemo DMG CRIT DMG +40%"),
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

// ===== Jahoda =====
// A4 passive: EM+100 when Burst robots heal characters with HP>70%
static JAHODA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Jahoda A4 EM Buff",
        description: desc!("When burst heal target has HP>70%, EM+100 for 6s (assumes active)"),
        stat: BuffableStat::ElementalMastery,
        base_value: 100.0,
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
        name: "jahoda_c6_crit_rate",
        description: desc!("C6: Moonsign characters CRIT Rate +5%"),
        stat: BuffableStat::CritRate,
        base_value: 0.05,
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
        name: "jahoda_c6_crit_dmg",
        description: desc!("C6: Moonsign characters CRIT DMG +40%"),
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

// ===== Kazuha =====
// A4 passive "Poetics of Fuubutsu": 0.04% Elemental DMG Bonus per point of EM
// C2: Party EM+200 in Burst field
// C6: Normal/Charged/Plunging ATK DMG bonus = 0.2% of EM
static KAZUHA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Poetics of Fuubutsu",
        description: desc!(
            "After triggering Swirl, grants 0.04% Elemental DMG Bonus per point of EM"
        ),
        stat: BuffableStat::DmgBonus,
        base_value: 0.0004,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "kazuha_c2_em",
        description: desc!("C2: Party EM +200 in Burst field"),
        stat: BuffableStat::ElementalMastery,
        base_value: 200.0,
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
        name: "kazuha_c6_normal_dmg",
        description: desc!("C6: Normal ATK DMG bonus based on 0.2% of EM"),
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.002,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "kazuha_c6_charged_dmg",
        description: desc!("C6: Charged ATK DMG bonus based on 0.2% of EM"),
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.002,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "kazuha_c6_plunging_dmg",
        description: desc!("C6: Plunging ATK DMG bonus based on 0.2% of EM"),
        stat: BuffableStat::PlungingAtkDmgBonus,
        base_value: 0.002,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Sucrose =====
// A1 passive "Catalyst Conversion": Swirl triggers EM+50 for team 8s
// A4 passive "Mollis Favonius": shares 20% of Sucrose's EM to party
static SUCROSE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Catalyst Conversion",
        description: desc!(
            "After triggering Swirl, grants EM+50 to party members with matching element"
        ),
        stat: BuffableStat::ElementalMastery,
        base_value: 50.0,
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
        name: "Mollis Favonius",
        description: desc!("Shares 20% of Sucrose's EM to party"),
        stat: BuffableStat::ElementalMastery,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::TeamExcludeSelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Varka =====
// A1: Dawn Wind's March — ATK1000あたり Anemo DMG +10% AND 対応元素DMG +10%、各最大25%
//   対応元素優先度: Pyro > Hydro > Electro > Cryo
// A4: Wind's Vanguard — Normal/Charged ATK DMG (拡散反応時+7.5%/stack、最大4stack=30%)
// C4: Freedom of Song — Team Anemo DMG +20% + 対応元素DMG +20% (対応元素は未実装)
// C1: "Lyrical Libation" is intentionally not representable as a talent buff.
// Mirror confirms this is a one-shot consumed multiplier that makes Four Winds' Ascension
// or Azure Devour deal 200% of their original DMG once after entering Sturm und Drang.
// The current stat-buff model only supports persistent additive bonuses, so approximating
// this with broad Skill/Charged DMG bonuses would both leak onto other attacks and fail
// to model the exact final-damage doubling. Keep it documented rather than adding a fake buff.
static VARKA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Dawn Wind's March Anemo DMG",
        description: desc!("ATK1000あたり風元素DMG+10%、最大25%。Toggle = max 25%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
        base_value: 0.0001,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::TotalAtk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: Some(0.25),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Dawn Wind's March Pyro DMG",
        description: desc!("ATK1000あたり炎元素DMG+10%、最大25%。Toggle = max 25%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.0001,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::TotalAtk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: Some(0.25),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Dawn Wind's March Hydro DMG",
        description: desc!("ATK1000あたり水元素DMG+10%、最大25%。Toggle = max 25%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
        base_value: 0.0001,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::TotalAtk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: Some(0.25),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Dawn Wind's March Electro DMG",
        description: desc!("ATK1000あたり雷元素DMG+10%、最大25%。Toggle = max 25%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Electro),
        base_value: 0.0001,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::TotalAtk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: Some(0.25),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Dawn Wind's March Cryo DMG",
        description: desc!("ATK1000あたり氷元素DMG+10%、最大25%。Toggle = max 25%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
        base_value: 0.0001,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::TotalAtk),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: Some(0.25),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Wind's Vanguard Normal ATK DMG",
        description: desc!("拡散反応時+7.5%/stack、最大4stack=30%。Toggle=4stack想定"),
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.30,
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
        name: "Wind's Vanguard Charged ATK DMG",
        description: desc!("拡散反応時+7.5%/stack、最大4stack=30%。Toggle=4stack想定"),
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.30,
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
        name: "Azure Fang's Oath CRIT DMG",
        description: desc!("C6: Each Azure Fang's Oath stack grants CRIT DMG +20%, max 4 stacks"),
        stat: BuffableStat::CritDmg,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(4))),
    },
    TalentBuffDef {
        name: "Freedom of Song Anemo DMG",
        description: desc!("C4: On Swirl, team Anemo DMG +20%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
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
    // C4 swirled element DMG +20% (element selected by Toggle; priority: Pyro > Hydro > Electro > Cryo)
    TalentBuffDef {
        name: "Freedom of Song Pyro DMG",
        description: desc!("C4: On Swirl, team Pyro DMG +20% (if Pyro swirled)"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
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
        name: "Freedom of Song Hydro DMG",
        description: desc!("C4: On Swirl, team Hydro DMG +20% (if Hydro swirled)"),
        stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
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
        name: "Freedom of Song Electro DMG",
        description: desc!("C4: On Swirl, team Electro DMG +20% (if Electro swirled)"),
        stat: BuffableStat::ElementalDmgBonus(Element::Electro),
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
        name: "Freedom of Song Cryo DMG",
        description: desc!("C4: On Swirl, team Cryo DMG +20% (if Cryo swirled)"),
        stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
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

// ===== Jean =====
// C4: Anemo RES -40% in burst field
static JEAN_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Lands of Dandelion Anemo RES Shred",
    description: desc!("C4: Enemies inside Dandelion Field have Anemo RES -40%"),
    stat: BuffableStat::ElementalResReduction(Element::Anemo),
    base_value: 0.40,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
}];

// ===== Venti =====
// C2: Anemo RES -12% + Physical RES -12%
// C4: Anemo DMG Bonus +25% on pickup
// C6: Opponents hit by Wind's Grand Ode have Anemo RES -20%
static VENTI_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Breeze of Reminiscence Anemo RES Shred",
        description: desc!("C2: Enemies hit by Skyward Sonnet have Anemo RES -12%"),
        stat: BuffableStat::ElementalResReduction(Element::Anemo),
        base_value: 0.12,
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
        name: "Breeze of Reminiscence Physical RES Shred",
        description: desc!("C2: Enemies hit by Skyward Sonnet have Physical RES -12%"),
        stat: BuffableStat::PhysicalResReduction,
        base_value: 0.12,
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
        name: "venti_c4_anemo_dmg",
        description: desc!("C4: Anemo DMG Bonus +25% on pickup"),
        stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
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
        name: "venti_c6_anemo_res_shred",
        description: desc!("C6: Opponents hit by Wind's Grand Ode have Anemo RES -20%"),
        stat: BuffableStat::ElementalResReduction(Element::Anemo),
        base_value: 0.20,
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

// ===== Xianyun =====
// Spec bug fix: A4 is an exact flat shockwave DMG bonus (200% Total ATK, cap 9000), not a
// generic plunging DMG bonus. Design approximation: A1's 4/6/8/10% per-stack curve is exposed
// as a max-stack toggle until TalentBuffDef supports non-linear stack tables.
static XIANYUN_BURST_PLUNGE_SCALING: [f64; 15] = [
    2.48, 2.666, 2.852, 3.100, 3.286, 3.472, 3.720, 3.968, 4.216, 4.464, 4.712, 4.960, 5.270,
    5.580, 5.890,
];

static XIANYUN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Stars Gather at Dusk Plunging Flat DMG",
        description: desc!("Burst: Plunging ATK gains flat DMG = total ATK × scaling (3 charges)"),
        stat: BuffableStat::PlungingAtkFlatDmg,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&XIANYUN_BURST_PLUNGE_SCALING),
        scales_on: Some(ScalingStat::TotalAtk),
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Galefeather Pursuit",
        description: desc!(
            "A1: Plunging Attack CRIT Rate +10% at max Storm Pinion stacks (design approximation: 4/6/8/10% curve condensed to max toggle)"
        ),
        stat: BuffableStat::CritRate,
        base_value: 0.10,
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
        name: "Consider, the Adeptus in Her Realm",
        description: desc!(
            "A4: Plunging Attack shockwave Flat DMG +200% of Xianyun's Total ATK, cap 9000"
        ),
        stat: BuffableStat::PlungingAtkFlatDmg,
        base_value: 2.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::TotalAtk),
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(9000.0),
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "xianyun_c2_atk",
        description: desc!("C2: ATK +20% after Skyladder"),
        stat: BuffableStat::AtkPercent,
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
    TalentBuffDef {
        name: "xianyun_c6_crit_dmg",
        description: desc!("C6: Max Driftcloud Wave CRIT DMG +70%"),
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

// ===== Wanderer =====
// A1: ATK+30% from Pyro contact, CR+20% from Cryo contact (self, Toggle)
// C2: Burst DMG up to +200% (self, Toggle, min_constellation=2)
static WANDERER_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Gales of Reverie ATK Bonus",
        description: desc!("A1: On Pyro contact during Windfavored state, ATK+30%"),
        stat: BuffableStat::AtkPercent,
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
        name: "Gales of Reverie CritRate Bonus",
        description: desc!("A1: On Cryo contact during Windfavored state, CritRate+20%"),
        stat: BuffableStat::CritRate,
        base_value: 0.20,
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
        name: "Stormborne Burst DMG Bonus",
        description: desc!("C2: Hanega: Song of the Wind DMG up to +200% (max value)"),
        stat: BuffableStat::BurstDmgBonus,
        base_value: 2.00,
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

// ===== Xiao =====
// A4: Skill DMG+15%/stack max 3 stacks (self, Stacks(3), cap 0.45)
// C2: ER+25% off-field — skip (not relevant for damage calc)
// C4: DEF+100% when HP<50% (self, Toggle, min_constellation=4)
static XIAO_BANE_OF_ALL_EVIL_SCALING: [f64; 15] = [
    0.5845, 0.6195, 0.6545, 0.7000, 0.7350, 0.7700, 0.8155, 0.8610, 0.9065, 0.9520, 0.9975, 1.0430,
    1.0885, 1.1340, 1.1795,
];

static XIAO_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Conqueror of Evil: Tamer of Demons",
        description: desc!("During burst, all DMG dealt increases by 5% per stack, max 5 stacks"),
        stat: BuffableStat::DmgBonus,
        base_value: 0.05,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(5))),
    },
    TalentBuffDef {
        name: "Transcension: Gravity Defier Skill DMG Bonus",
        description: desc!(
            "A4: Using Lemniscatic Wind Cycling increases subsequent Skill DMG by 15% per stack, max 3 stacks"
        ),
        stat: BuffableStat::SkillDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: Some(0.45),
        activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
    },
    TalentBuffDef {
        name: "Bane of All Evil Normal ATK DMG Bonus",
        description: desc!("Burst: Normal Attack DMG Bonus while Yaksha's Mask is active"),
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.5845,
        scales_with_talent: true,
        talent_scaling: Some(&XIAO_BANE_OF_ALL_EVIL_SCALING),
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Bane of All Evil Charged ATK DMG Bonus",
        description: desc!("Burst: Charged Attack DMG Bonus while Yaksha's Mask is active"),
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.5845,
        scales_with_talent: true,
        talent_scaling: Some(&XIAO_BANE_OF_ALL_EVIL_SCALING),
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Bane of All Evil Plunging ATK DMG Bonus",
        description: desc!("Burst: Plunging Attack DMG Bonus while Yaksha's Mask is active"),
        stat: BuffableStat::PlungingAtkDmgBonus,
        base_value: 0.5845,
        scales_with_talent: true,
        talent_scaling: Some(&XIAO_BANE_OF_ALL_EVIL_SCALING),
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Conqueror of Evil: Tamer of Demons DEF Bonus",
        description: desc!("C4: When HP<50%, DEF+100%"),
        stat: BuffableStat::DefPercent,
        base_value: 1.00,
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

// ===== Chasca =====
// A1: Skill DMG+15/35/65% based on element count. Max SkillDmgBonus 0.65 Toggle (self)
// C6: CritDmg+120% (self, Toggle, min_constellation=6)
static CHASCA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Galesplitting Soulseeker Shell Charged ATK DMG Bonus",
        description: desc!(
            "A1: Shining Shadowhunt Shell DMG+15/35/65% based on element count. Toggle = max 65%"
        ),
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.65,
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
        name: "Ride the Wind CritDmg Bonus",
        description: desc!("C6: CritDmg+120%"),
        stat: BuffableStat::CritDmg,
        base_value: 1.20,
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

// ===== Heizou =====
// A4: team EM+80 after burst (Team)
// C6: CR+4%/stack max 4 stacks (+16%) + CD+32% (self, Toggle, min_constellation=6)
static HEIZOU_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Penetrative Reasoning EM Bonus",
        description: desc!("A4: After using Elemental Burst, team EM+80 for 10s"),
        stat: BuffableStat::ElementalMastery,
        base_value: 80.0,
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
        name: "Paradoxical Practice CritRate Bonus",
        description: desc!(
            "C6: CritRate+4%/Declension stack, max 4 stacks (+16%). Toggle = max value"
        ),
        stat: BuffableStat::CritRate,
        base_value: 0.16,
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
        name: "Paradoxical Practice CritDmg Bonus",
        description: desc!("C6: CritDmg+32% at max Declension stacks"),
        stat: BuffableStat::CritDmg,
        base_value: 0.32,
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

// ===== Ifa =====
// A4: Swirl/EC DMG from Nightsoul scaling — too complex, skip with TODO
// C4: EM+100 (self, min_constellation=4)
// TODO: A4 — Swirl/EC DMG scaling from Nightsoul points; too complex for TalentBuffDef
static IFA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Eye of Stormy Judgment EM Bonus",
    description: desc!("C4: EM+100"),
    stat: BuffableStat::ElementalMastery,
    base_value: 100.0,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
    activation: None,
}];

// ===== Lan Yan =====
// A4: Skill DMG = 309% EM, Burst DMG = 774% EM
// C4: team EM+60 (Team, min_constellation=4)
static LAN_YAN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Lan Yan A4 Skill Flat DMG",
        description: desc!("A4: Elemental Skill DMG up to 309% EM. Toggle = max value"),
        stat: BuffableStat::SkillFlatDmg,
        base_value: 3.09,
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
        name: "Lan Yan A4 Burst Flat DMG",
        description: desc!("A4: Elemental Burst DMG up to 774% EM. Toggle = max value"),
        stat: BuffableStat::BurstFlatDmg,
        base_value: 7.74,
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
        name: "Lan Yan C4 Team EM Bonus",
        description: desc!("C4: team EM+60 for 12s after Burst"),
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

// ===== Lynette =====
// A4: team ATK+8~20% based on element diversity. Max AtkPercent 0.20 Toggle (Team)
// C6: Anemo DMG+20% (self, min_constellation=6)
static LYNETTE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Sophisticated Synergy ATK Bonus",
        description: desc!("A4: team ATK+8~20% based on element diversity. Toggle = max 20%"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.20,
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
        name: "Magic Trick: Astonishing Shift Anemo DMG Bonus",
        description: desc!("C6: Anemo DMG Bonus+20%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
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
];

// ===== Mizuki =====
// A4: self EM+100 when nearby party members trigger elemental attacks (Toggle)
// C2: team DMG+0.04%/EM excluding self (TeamExcludeSelf, DmgBonus scales_on=Em, base_value=0.0004)
// C6: Swirl CritRate+30% + CritDmg+100% (self, Toggle, min_constellation=6)
static MIZUKI_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Thoughts by Day Bring Dreams by Night EM Bonus",
        description: desc!(
            "A4: While in Dreamdrifter state, Mizuki gains EM+100 when nearby party members trigger elemental attacks"
        ),
        stat: BuffableStat::ElementalMastery,
        base_value: 100.0,
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
        name: "Mizuki C2 Team DMG Bonus",
        description: desc!("C2: team DMG+0.04%/EM (base_value=0.0004 × EM)"),
        stat: BuffableStat::DmgBonus,
        base_value: 0.0004,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Em),
        target: BuffTarget::TeamExcludeSelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Mizuki C6 Swirl CritRate Bonus",
        description: desc!("C6: Swirl CritRate+30%"),
        stat: BuffableStat::CritRate,
        base_value: 0.30,
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
        name: "Mizuki C6 Swirl CritDmg Bonus",
        description: desc!("C6: Swirl CritDmg+100%"),
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

// ===== Sayu =====
// C2: Skill DMG up to +66% (self, Toggle, min_constellation=2)
// C6: EM→Daruma DMG — too complex, skip with TODO
// TODO: C6 — EM-scaling Daruma DMG; too complex for TalentBuffDef
static SAYU_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Sayu C2 Skill DMG Bonus",
    description: desc!("C2: Skill DMG up to +66% based on EM. Toggle = max value"),
    stat: BuffableStat::SkillDmgBonus,
    base_value: 0.66,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::Constellation(2),
    min_constellation: 2,
    cap: None,
    activation: Some(Activation::Manual(ManualCondition::Toggle)),
}];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static ANEMO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("chasca", CHASCA_BUFFS),
    ("faruzan", FARUZAN_BUFFS),
    ("heizou", HEIZOU_BUFFS),
    ("ifa", IFA_BUFFS),
    ("jahoda", JAHODA_BUFFS),
    ("jean", JEAN_BUFFS),
    ("kazuha", KAZUHA_BUFFS),
    ("lan_yan", LAN_YAN_BUFFS),
    ("lynette", LYNETTE_BUFFS),
    ("mizuki", MIZUKI_BUFFS),
    ("sayu", SAYU_BUFFS),
    ("sucrose", SUCROSE_BUFFS),
    ("varka", VARKA_BUFFS),
    ("venti", VENTI_BUFFS),
    ("wanderer", WANDERER_BUFFS),
    ("xiao", XIAO_BUFFS),
    ("xianyun", XIANYUN_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    ANEMO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn find_buffs(character_id: &str) -> &'static [TalentBuffDef] {
        find(character_id).unwrap()
    }

    #[test]
    fn chasca_a1_is_charged_attack_shell_bonus() {
        let buff = find_buffs("chasca")
            .iter()
            .find(|buff| buff.source == TalentBuffSource::AscensionPassive(1))
            .unwrap();

        assert_eq!(buff.stat, BuffableStat::ChargedAtkDmgBonus);
        assert!((buff.base_value - 0.65).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert_eq!(
            buff.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );
    }

    #[test]
    fn varka_a1_scales_on_total_atk_and_covers_all_elements() {
        let buffs = find_buffs("varka");
        let a1_buffs: Vec<_> = buffs
            .iter()
            .filter(|buff| buff.source == TalentBuffSource::AscensionPassive(1))
            .collect();

        assert_eq!(a1_buffs.len(), 5);
        for element in [
            Element::Anemo,
            Element::Pyro,
            Element::Hydro,
            Element::Electro,
            Element::Cryo,
        ] {
            let buff = a1_buffs
                .iter()
                .find(|buff| buff.stat == BuffableStat::ElementalDmgBonus(element))
                .unwrap();
            assert!((buff.base_value - 0.0001).abs() < 1e-6);
            assert_eq!(buff.scales_on, Some(ScalingStat::TotalAtk));
            assert_eq!(buff.target, BuffTarget::OnlySelf);
            assert_eq!(buff.cap, Some(0.25));
            assert_eq!(
                buff.activation,
                Some(Activation::Manual(ManualCondition::Toggle))
            );
        }
    }

    #[test]
    fn xiao_burst_and_a4_match_mirror_tables() {
        let buffs = find_buffs("xiao");

        let a4 = buffs
            .iter()
            .find(|buff| buff.source == TalentBuffSource::AscensionPassive(4))
            .unwrap();
        assert_eq!(a4.stat, BuffableStat::SkillDmgBonus);
        assert!((a4.base_value - 0.15).abs() < 1e-6);
        assert_eq!(a4.target, BuffTarget::OnlySelf);
        assert_eq!(a4.cap, Some(0.45));
        assert_eq!(
            a4.activation,
            Some(Activation::Manual(ManualCondition::Stacks(3)))
        );

        let burst_buffs: Vec<_> = buffs
            .iter()
            .filter(|buff| buff.source == TalentBuffSource::ElementalBurst)
            .collect();
        assert_eq!(burst_buffs.len(), 3);
        let expected_scaling = [
            0.5845, 0.6195, 0.6545, 0.7000, 0.7350, 0.7700, 0.8155, 0.8610, 0.9065, 0.9520, 0.9975,
            1.0430, 1.0885, 1.1340, 1.1795,
        ];

        for stat in [
            BuffableStat::NormalAtkDmgBonus,
            BuffableStat::ChargedAtkDmgBonus,
            BuffableStat::PlungingAtkDmgBonus,
        ] {
            let buff = burst_buffs.iter().find(|buff| buff.stat == stat).unwrap();
            assert_eq!(buff.target, BuffTarget::OnlySelf);
            assert_eq!(
                buff.activation,
                Some(Activation::Manual(ManualCondition::Toggle))
            );
            assert_eq!(buff.talent_scaling.unwrap(), &expected_scaling);
        }
    }

    #[test]
    fn lan_yan_a4_uses_skill_and_burst_flat_damage_and_c4_toggles() {
        let buffs = find_buffs("lan_yan");

        let skill = buffs
            .iter()
            .find(|buff| {
                buff.source == TalentBuffSource::AscensionPassive(4)
                    && buff.stat == BuffableStat::SkillFlatDmg
            })
            .unwrap();
        assert!((skill.base_value - 3.09).abs() < 1e-6);
        assert_eq!(skill.scales_on, Some(ScalingStat::Em));
        assert_eq!(skill.target, BuffTarget::OnlySelf);
        assert_eq!(
            skill.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );

        let burst = buffs
            .iter()
            .find(|buff| {
                buff.source == TalentBuffSource::AscensionPassive(4)
                    && buff.stat == BuffableStat::BurstFlatDmg
            })
            .unwrap();
        assert!((burst.base_value - 7.74).abs() < 1e-6);
        assert_eq!(burst.scales_on, Some(ScalingStat::Em));
        assert_eq!(burst.target, BuffTarget::OnlySelf);
        assert_eq!(
            burst.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );

        let c4 = buffs
            .iter()
            .find(|buff| buff.source == TalentBuffSource::Constellation(4))
            .unwrap();
        assert_eq!(c4.stat, BuffableStat::ElementalMastery);
        assert_eq!(c4.target, BuffTarget::Team);
        assert_eq!(
            c4.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );
    }

    #[test]
    fn lynette_c6_is_toggle_activated() {
        let buff = find_buffs("lynette")
            .iter()
            .find(|buff| buff.source == TalentBuffSource::Constellation(6))
            .unwrap();

        assert_eq!(
            buff.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );
    }

    #[test]
    fn mizuki_a4_replaces_the_fabricated_a1_em_buff() {
        let buffs = find_buffs("mizuki");

        assert!(
            buffs.iter().all(
                |buff| !(buff.source == TalentBuffSource::AscensionPassive(1)
                    && buff.stat == BuffableStat::ElementalMastery
                    && (buff.base_value - 100.0).abs() < 1e-6)
            ),
            "Mizuki should not have a fabricated A1 team EM buff"
        );

        let a4 = buffs
            .iter()
            .find(|buff| buff.source == TalentBuffSource::AscensionPassive(4))
            .unwrap();
        assert_eq!(a4.stat, BuffableStat::ElementalMastery);
        assert_eq!(a4.target, BuffTarget::OnlySelf);
        assert_eq!(
            a4.activation,
            Some(Activation::Manual(ManualCondition::Toggle))
        );
        assert!((a4.base_value - 100.0).abs() < 1e-6);

        let c2 = buffs
            .iter()
            .find(|buff| buff.source == TalentBuffSource::Constellation(2))
            .unwrap();
        assert_eq!(c2.target, BuffTarget::TeamExcludeSelf);
    }
}
