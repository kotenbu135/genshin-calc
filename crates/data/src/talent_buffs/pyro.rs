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
        cap: None,
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
        cap: None,
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
    cap: None,
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
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
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
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
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
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
    },
    TalentBuffDef {
        name: "In Pursuit of Ending Evil Pyro DMG Bonus",
        description: "C6: After healing from skill, Pyro DMG +20% per stack (max 3 stacks = +60%)",
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.60,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
    },
    TalentBuffDef {
        name: "In Pursuit of Ending Evil Electro DMG Bonus",
        description: "C6: After healing from skill, Electro DMG +20% per stack (max 3 stacks = +60%)",
        stat: BuffableStat::ElementalDmgBonus(Element::Electro),
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
        cap: None,
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
        cap: None,
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
        cap: None,
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
    source: TalentBuffSource::AscensionPassive(4),
    min_constellation: 0,
    cap: None,
}];

// ===== Durin =====
// A1 (Purity) "Light Manifest of the Divine Calculus": Pyro RES -20% after Burning/Overloaded/Pyro Swirl/Pyro Crystallize
// A1 (Darkness) "Light Manifest of the Divine Calculus": Vaporize/Melt DMG +40%
// C2 "Unground Visions": Pyro DMG +50% for party after reaction
// C4 "Emanare's Source": Burst DMG +40%
// (A4 "Chaos Formed Like the Night" is a complex per-hit ATK-scaling DMG bonus — not expressible as TalentBuffDef)
static DURIN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Light Manifest (Purity) Pyro RES Down",
        description: "A1: Enemy Pyro RES -20% after Burning/Overloaded/Pyro Swirl/Pyro Crystallize",
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
    },
    TalentBuffDef {
        name: "Light Manifest (Darkness) Amplifying Bonus",
        description: "A1: Vaporize/Melt DMG +40% in Darkness form",
        stat: BuffableStat::AmplifyingBonus,
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
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
        cap: None,
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
        cap: None,
    },
];

// ===== Klee =====
// C2: DEF -23% on mine hit
// C6: Pyro DMG +10% during burst
static KLEE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Explosive Frags DEF Reduction",
        description: "C2: Enemies hit by Jumpy Dumpty mines have DEF -23%",
        stat: BuffableStat::DefReduction,
        base_value: 0.23,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
    },
    TalentBuffDef {
        name: "Blazing Delight Pyro DMG Bonus",
        description: "C6: During Sparks 'n' Splash, party gains Pyro DMG +10%",
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
    },
];

// ===== Mavuika =====
// A1 "Sunfrost Encomium": Mavuika's own ATK+30% while in Nightsoul's Blessing state
// A4 "Fire-Forged Heritage" "Kiongozi": DMG Bonus +0.2% per Fighting Spirit point consumed (max +40% at C0, 200pt)
static MAVUIKA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Sunfrost Encomium ATK Bonus",
        description: "A1: Mavuika's own ATK+30% while in Nightsoul's Blessing state",
        stat: BuffableStat::AtkPercent,
        base_value: 0.30,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
    },
    TalentBuffDef {
        name: "Fire-Forged Heritage DMG Bonus",
        description: "A4: DMG Bonus from Fighting Spirit consumed (max +40% at C0/200pt, adopting max value)",
        stat: BuffableStat::DmgBonus,
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
    },
];

// ===== Xiangling =====
// C1: Pyro RES -15% on Guoba hit
// C6: Pyro DMG +15% during Pyronado
static XIANGLING_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Crispy Outside Pyro RES Shred",
        description: "C1: Enemies hit by Guoba have Pyro RES -15% for 6s",
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
    },
    TalentBuffDef {
        name: "Condensed Pyronado Pyro DMG Bonus",
        description: "C6: During Pyronado, party gains Pyro DMG +15%",
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
    },
];

// ===== Xinyan =====
// C4: Physical RES -15% on skill hit
static XINYAN_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Blazing Eye Physical RES Shred",
    description: "C4: Enemies hit by Riff Revolution have Physical RES -15% for 12s",
    stat: BuffableStat::PhysicalResReduction,
    base_value: 0.15,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::Constellation(4),
    min_constellation: 4,
    cap: None,
}];

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static PYRO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("amber", AMBER_BUFFS),
    ("bennett", BENNETT_BUFFS),
    ("chevreuse", CHEVREUSE_BUFFS),
    ("durin", DURIN_BUFFS),
    ("klee", KLEE_BUFFS),
    ("mavuika", MAVUIKA_BUFFS),
    ("thoma", THOMA_BUFFS),
    ("xiangling", XIANGLING_BUFFS),
    ("xinyan", XINYAN_BUFFS),
    ("yoimiya", YOIMIYA_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    PYRO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}
