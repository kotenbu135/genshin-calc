use super::*;

// ===== Faruzan =====
// Burst "The Wind's Secret Ways": Anemo DMG bonus per level (Lv1-15)
// Values from Genshin Wiki (Prayerful Wind's Benefit)
static FARUZAN_BURST_ANEMO_SCALING: [f64; 15] = [
    0.182, 0.196, 0.209, 0.228, 0.241, 0.255, 0.273, 0.291, 0.310, 0.328, 0.346, 0.364, 0.387,
    0.410, 0.432,
];

static FARUZAN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Prayerful Wind's Benefit",
        description: "Anemo DMG Bonus based on burst talent level",
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
        description: "A4: Enemies hit by Pressurized Collapse have Anemo RES -30%",
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
];

// ===== Jahoda =====
// A4 passive: EM+100 when Burst robots heal characters with HP>70%
static JAHODA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Jahoda A4 EM Buff",
    description: "When burst heal target has HP>70%, EM+100 for 6s (assumes active)",
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
}];

// ===== Kazuha =====
// A4 passive "Poetics of Fuubutsu": 0.04% Elemental DMG Bonus per point of EM
static KAZUHA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Poetics of Fuubutsu",
    description: "After triggering Swirl, grants 0.04% Elemental DMG Bonus per point of EM",
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
}];

// ===== Sucrose =====
// A1 passive "Catalyst Conversion": Swirl triggers EM+50 for team 8s
// A4 passive "Mollis Favonius": shares 20% of Sucrose's EM to party
static SUCROSE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Catalyst Conversion",
        description: "After triggering Swirl, grants EM+50 to party members with matching element",
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
        description: "Shares 20% of Sucrose's EM to party",
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
//   対応元素DMGは動的選択のためTalentBuffDefでは表現困難。Anemoのみ定義。
//   TODO: 対応元素DMG+25% を動的に選択する仕組みを検討
// A4: Wind's Vanguard — Normal/Charged ATK DMG (拡散反応時+7.5%/stack、最大4stack=30%)
// C4: Freedom of Song — Team Anemo DMG +20% + 対応元素DMG +20% (対応元素は未実装)
static VARKA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Dawn Wind's March Anemo DMG",
        description: "ATK1000あたり風元素DMG+10%、最大25%。対応元素DMG+25%は未実装(動的選択)。Toggle=25%想定",
        stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
        base_value: 0.25,
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
        name: "Wind's Vanguard Normal ATK DMG",
        description: "拡散反応時+7.5%/stack、最大4stack=30%。Toggle=4stack想定",
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
        description: "拡散反応時+7.5%/stack、最大4stack=30%。Toggle=4stack想定",
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
        name: "Freedom of Song Anemo DMG",
        description: "C4: On Swirl, team Anemo DMG +20%",
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
        description: "C4: On Swirl, team Pyro DMG +20% (if Pyro swirled)",
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
        description: "C4: On Swirl, team Hydro DMG +20% (if Hydro swirled)",
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
        description: "C4: On Swirl, team Electro DMG +20% (if Electro swirled)",
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
        description: "C4: On Swirl, team Cryo DMG +20% (if Cryo swirled)",
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
    description: "C4: Enemies inside Dandelion Field have Anemo RES -40%",
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
static VENTI_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Breeze of Reminiscence Anemo RES Shred",
        description: "C2: Enemies hit by Skyward Sonnet have Anemo RES -12%",
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
        description: "C2: Enemies hit by Skyward Sonnet have Physical RES -12%",
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
];

// ===== Xianyun =====
// Burst: PlungingAtkFlatDmg = total ATK × scaling
// A4: PlungingAtkDmgBonus max +75% (fixed max value)
// C2: CritRate +20% for plunge after burst
static XIANYUN_BURST_PLUNGE_SCALING: [f64; 15] = [
    2.48, 2.666, 2.852, 3.100, 3.286, 3.472, 3.720, 3.968, 4.216, 4.464, 4.712, 4.960, 5.270,
    5.580, 5.890,
];

static XIANYUN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Stars Gather at Dusk Plunging Flat DMG",
        description: "Burst: Plunging ATK gains flat DMG = total ATK × scaling (3 charges)",
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
        name: "Crane Form Plunging DMG Bonus",
        description: "A4: Plunging ATK DMG Bonus max +75% (adopting max value)",
        stat: BuffableStat::PlungingAtkDmgBonus,
        base_value: 0.75,
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
        name: "Trivial Matters CritRate Bonus",
        description: "C2: After burst, plunging attacks gain CritRate +20%",
        stat: BuffableStat::CritRate,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: None,
    },
];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static ANEMO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("faruzan", FARUZAN_BUFFS),
    ("jahoda", JAHODA_BUFFS),
    ("jean", JEAN_BUFFS),
    ("kazuha", KAZUHA_BUFFS),
    ("sucrose", SUCROSE_BUFFS),
    ("varka", VARKA_BUFFS),
    ("venti", VENTI_BUFFS),
    ("xianyun", XIANYUN_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    ANEMO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}
