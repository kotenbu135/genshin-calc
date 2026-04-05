use super::*;

// ===== Bennett =====
// Elemental Burst "Fantastic Voyage" ATK buff: 56%~119% of Base ATK (Lv1-15)
static BENNETT_BURST_ATK_SCALING: [f64; 15] = [
    0.56, 0.602, 0.644, 0.70, 0.742, 0.784, 0.84, 0.896, 0.952, 1.008, 1.064, 1.12, 1.19, 1.26,
    1.33,
];

static BENNETT_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Fantastic Voyage ATK Bonus",
        description: "Characters within the burst field gain ATK bonus based on Bennett's Base ATK",
        stat: BuffableStat::AtkFlat,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&BENNETT_BURST_ATK_SCALING),
        scales_on: Some(ScalingStat::Atk), // base_atk × scaling
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Spirit of Pyro",
        description: "C6: Characters within the burst field gain Pyro DMG Bonus +15%",
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
];

// ===== Amber =====
// C6 "Wildfire": ATK+15% for party during burst
static AMBER_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Wildfire",
    description: "During burst, party members gain ATK+15%",
    stat: BuffableStat::AtkPercent,
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(6),
    min_constellation: 6,
}];

// ===== Chevreuse =====
// A1 passive "Vanguard's Coordinated Tactics": ATK+20% after Overloaded (Pyro+Electro team)
// A4 passive: After Overloaded, enemy Pyro RES and Electro RES -40% for 6s
static CHEVREUSE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Vanguard's Coordinated Tactics",
        description: "After Overloaded, ATK+20% for party (Pyro+Electro teams only, approximation)",
        stat: BuffableStat::AtkPercent,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Overloaded Pyro RES Shred",
        description: "After Overloaded reaction, enemy Pyro RES -40% for 6s",
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Overloaded Electro RES Shred",
        description: "After Overloaded reaction, enemy Electro RES -40% for 6s",
        stat: BuffableStat::ElementalResReduction(Element::Electro),
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
];

// ===== Thoma =====
// C6 "Burning Heart": Normal/Charged/Plunging +15% after burst
static THOMA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Burning Heart - Normal ATK",
        description: "After burst, party Normal ATK DMG +15%",
        stat: BuffableStat::NormalAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
    TalentBuffDef {
        name: "Burning Heart - Charged ATK",
        description: "After burst, party Charged ATK DMG +15%",
        stat: BuffableStat::ChargedAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
    TalentBuffDef {
        name: "Burning Heart - Plunging ATK",
        description: "After burst, party Plunging ATK DMG +15%",
        stat: BuffableStat::PlungingAtkDmgBonus,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
    },
];

// ===== Yoimiya =====
// A4 passive "Summer Night's Dawn": ATK+20% to party (excl. self) after burst
static YOIMIYA_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Summer Night's Dawn",
    description: "After burst, party members (excluding Yoimiya) gain ATK+20% (max assumption)",
    stat: BuffableStat::AtkPercent,
    base_value: 0.20,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::TeamExcludeSelf,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];

// ===== Durin =====
// A4 (Purity) "Light Manifest of the Divine Calculus": Pyro RES -20% after Burning/Overloaded/Pyro Swirl/Pyro Crystallize
// A4 (Darkness) "Light Manifest of the Divine Calculus": Vaporize/Melt DMG +40%
// C2 "Unground Visions": Pyro DMG +50% for party after reaction
// C4 "Emanare's Source": Burst DMG +40%
static DURIN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Light Manifest (Purity) Pyro RES Down",
        description: "A4: Enemy Pyro RES -20% after Burning/Overloaded/Pyro Swirl/Pyro Crystallize",
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Light Manifest (Darkness) Amplifying Bonus",
        description: "A4: Vaporize/Melt DMG +40% in Darkness form",
        stat: BuffableStat::AmplifyingBonus,
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Unground Visions Pyro DMG Bonus",
        description: "C2: Pyro DMG +50% for party after triggering reactions",
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.50,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
    },
    TalentBuffDef {
        name: "Emanare's Source Burst DMG Bonus",
        description: "C4: Elemental Burst DMG +40%",
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
    },
];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static PYRO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("amber", AMBER_BUFFS),
    ("bennett", BENNETT_BUFFS),
    ("chevreuse", CHEVREUSE_BUFFS),
    ("durin", DURIN_BUFFS),
    ("thoma", THOMA_BUFFS),
    ("yoimiya", YOIMIYA_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    PYRO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}
