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
}];

// ===== Kazuha =====
// A4 passive "Poetics of Fuubutsu": 0.04% Elemental DMG Bonus per point of EM
static KAZUHA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Poetics of Fuubutsu",
    description: "After triggering Swirl, grants 0.04% Elemental DMG Bonus per point of EM",
    stat: BuffableStat::DmgBonus,
    base_value: 0.0, // EM-dependent — builder computes EM×0.0004 at resolve time
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive(4),
    min_constellation: 0,
}];

// ===== Sucrose =====
// A1 passive "Catalyst Conversion": Swirl triggers EM+50 for team 8s
// A4 passive "Mollis Favonius": shares 20% of Sucrose's EM to party (builder computes EM * 0.20)
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
    },
    TalentBuffDef {
        name: "Mollis Favonius",
        description: "Shares 20% of Sucrose's EM to party (builder computes EM * 0.20)",
        stat: BuffableStat::ElementalMastery,
        base_value: 0.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::TeamExcludeSelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
    },
];

// ===== Varka =====
// A1: Dawn Wind's March — Anemo DMG Bonus (ATK1000あたり+10%、最大25%)
// A4: Wind's Vanguard — Normal/Charged ATK DMG (拡散反応時+7.5%/stack、最大4stack=30%)
// C4: Freedom of Song — Team Anemo DMG (拡散反応時+20%)
static VARKA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Dawn Wind's March Anemo DMG",
        description: "ATK1000あた��風元素/対応元素DMG+10%、最大25%。Toggle=25%想定",
        stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
        base_value: 0.25,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
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
    },
    TalentBuffDef {
        name: "Freedom of Song Anemo DMG",
        description: "拡散反応時、チーム全員にAnemo DMG+20%＋対応元素DMG+20%(対応元素は手動設定)",
        stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
    },
];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static ANEMO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("faruzan", FARUZAN_BUFFS),
    ("jahoda", JAHODA_BUFFS),
    ("kazuha", KAZUHA_BUFFS),
    ("sucrose", SUCROSE_BUFFS),
    ("varka", VARKA_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    ANEMO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}
