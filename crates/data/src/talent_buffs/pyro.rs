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
        description: desc!(
            "Characters within the burst field gain ATK bonus based on Bennett's Base ATK"
        ),
        stat: BuffableStat::AtkFlat,
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&BENNETT_BURST_ATK_SCALING),
        scales_on: Some(ScalingStat::Atk), // base_atk × scaling
        target: BuffTarget::Team,
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    // C1 "Grand Expectation": +20% of Base ATK added to Fantastic Voyage ATK bonus
    TalentBuffDef {
        name: "Grand Expectation",
        description: desc!(
            "C1: Fantastic Voyage ATK bonus gains additional 20% of Bennett's Base ATK"
        ),
        stat: BuffableStat::AtkFlat,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Atk), // base_atk × 0.20
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Spirit of Pyro",
        description: desc!("C6: Characters within the burst field gain Pyro DMG Bonus +15%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
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

// ===== Amber =====
// C6 "Wildfire": ATK+15% for party during burst
static AMBER_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Wildfire",
    description: desc!("During burst, party members gain ATK+15%"),
    stat: BuffableStat::AtkPercent,
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

// ===== Chevreuse =====
// A1 passive "Vanguard's Coordinated Tactics": ATK+20% after Overloaded (Pyro+Electro team)
// A4 passive: After Overloaded, enemy Pyro RES and Electro RES -40% for 6s
static CHEVREUSE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Vanguard's Coordinated Tactics",
        description: desc!(
            "After Overloaded, ATK+20% for party (Pyro+Electro teams only, approximation)"
        ),
        stat: BuffableStat::AtkPercent,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Both(
            AutoCondition::TeamElementsOnly(&[Element::Pyro, Element::Electro]),
            ManualCondition::Toggle,
        )),
    },
    TalentBuffDef {
        name: "Overloaded Pyro RES Shred",
        description: desc!("After Overloaded reaction, enemy Pyro RES -40% for 6s"),
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
        base_value: 0.40,
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
        name: "Overloaded Electro RES Shred",
        description: desc!("After Overloaded reaction, enemy Electro RES -40% for 6s"),
        stat: BuffableStat::ElementalResReduction(Element::Electro),
        base_value: 0.40,
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
        name: "chevreuse_c6_pyro_dmg",
        description: desc!("C6: Party Pyro DMG Bonus +20% per stack (max 3)"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
    },
    TalentBuffDef {
        name: "chevreuse_c6_electro_dmg",
        description: desc!("C6: Party Electro DMG Bonus +20% per stack (max 3)"),
        stat: BuffableStat::ElementalDmgBonus(Element::Electro),
        base_value: 0.20,
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

// ===== Thoma =====
// C6 "Burning Heart": Normal/Charged/Plunging +15% after burst
static THOMA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Burning Heart - Normal ATK",
        description: desc!("After burst, party Normal ATK DMG +15%"),
        stat: BuffableStat::NormalAtkDmgBonus,
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
        name: "Burning Heart - Charged ATK",
        description: desc!("After burst, party Charged ATK DMG +15%"),
        stat: BuffableStat::ChargedAtkDmgBonus,
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
        name: "Burning Heart - Plunging ATK",
        description: desc!("After burst, party Plunging ATK DMG +15%"),
        stat: BuffableStat::PlungingAtkDmgBonus,
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
];

// ===== Yoimiya =====
// A4 passive "Summer Night's Dawn": ATK+20% to party (excl. self) after burst
// C1: ATK +20% on Aurous Blaze defeat
// C2: Pyro DMG +25% on Pyro CRIT
static YOIMIYA_BUFFS: &[TalentBuffDef] = &[
    // A1: Pyro DMG +2% per stack (max 10) during Niwabi Fire-Dance
    TalentBuffDef {
        name: "Tricks of the Trouble-Maker Pyro DMG",
        description: desc!("A1: Pyro DMG +2% per NA hit during Niwabi Fire-Dance, max 10 stacks"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.02,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(10))),
    },
    TalentBuffDef {
        name: "Summer Night's Dawn",
        description: desc!(
            "After burst, party members (excluding Yoimiya) gain ATK+20% (max assumption)"
        ),
        stat: BuffableStat::AtkPercent,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::TeamExcludeSelf,
        source: TalentBuffSource::AscensionPassive(4),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    // C1: ATK +20% on Aurous Blaze defeat
    TalentBuffDef {
        name: "yoimiya_c1_atk",
        description: desc!("C1: ATK +20% for 20s when Aurous Blaze-affected opponent defeated"),
        stat: BuffableStat::AtkPercent,
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
    // C2: Pyro DMG +25% on Pyro CRIT
    TalentBuffDef {
        name: "yoimiya_c2_pyro_dmg",
        description: desc!("C2: Pyro DMG Bonus +25% for 6s on Pyro CRIT hit"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.25,
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

// ===== Durin =====
// A1 (Purity) "Light Manifest of the Divine Calculus": Pyro RES -20% after Burning/Overloaded/Pyro Swirl/Pyro Crystallize
// A1 (Darkness) "Light Manifest of the Divine Calculus": Vaporize/Melt DMG +40%
// A4 "Chaos Formed Like the Night": ATK-scaled burst DMG bonus — complex, not implemented
// C2 "Unground Visions": Pyro DMG +50% for party after reaction
// C4 "Emanare's Source": Burst DMG +40%
static DURIN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Light Manifest (Purity) Pyro RES Down",
        description: desc!(
            "A1: Enemy Pyro RES -20% after Burning/Overloaded/Pyro Swirl/Pyro Crystallize"
        ),
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: None,
    },
    TalentBuffDef {
        name: "Light Manifest (Darkness) Amplifying Bonus",
        description: desc!("A1: Vaporize/Melt DMG +40% in Darkness form"),
        stat: BuffableStat::AmplifyingBonus,
        base_value: 0.40,
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
        name: "Unground Visions Pyro DMG Bonus",
        description: desc!("C2: Pyro DMG +50% for party after triggering reactions"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.50,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: None,
    },
    // C4 "Emanare's Source": Burst DMG +40% (conditional toggle)
    TalentBuffDef {
        name: "durin_c4_burst_dmg",
        description: desc!("C4: Elemental Burst DMG +40%"),
        stat: BuffableStat::BurstDmgBonus,
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    // C6: DEF Ignore 30%
    TalentBuffDef {
        name: "durin_c6_def_ignore",
        description: desc!("C6: Burst DMG ignores 30% of opponents' DEF"),
        stat: BuffableStat::DefIgnore,
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
    // C6: DEF Reduction 30% (Light form)
    TalentBuffDef {
        name: "durin_c6_def_reduction",
        description: desc!("C6: Light form decreases opponent DEF by 30%"),
        stat: BuffableStat::DefReduction,
        base_value: 0.30,
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

// ===== Klee =====
// C2: DEF -23% on mine hit
// C6: Pyro DMG +10% during burst
static KLEE_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "klee_c2_def_reduction",
        description: desc!("C2: Jumpy Dumpty mines reduce opponent DEF by 23%"),
        stat: BuffableStat::DefReduction,
        base_value: 0.23,
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
        name: "klee_c6_pyro_dmg",
        description: desc!("C6: Sparks 'n' Splash grants party Pyro DMG Bonus +10%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
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
    TalentBuffDef {
        name: "Sparkling Burst Self Pyro DMG Bonus",
        description: desc!("C6: During Sparks 'n' Splash, Klee gains Pyro DMG +50%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
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
    TalentBuffDef {
        name: "Sparkling Burst ATK Bonus",
        description: desc!("C1: Klee gains ATK+60% (self, approximation)"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.60,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(1),
        min_constellation: 1,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Mavuika =====
// A1 "Sunfrost Encomium": Mavuika's own ATK+30% while in Nightsoul's Blessing state
// A4 "Fire-Forged Heritage" "Kiongozi": DMG Bonus +0.2% per Fighting Spirit point consumed (max +40% at C0, 200pt)
static MAVUIKA_BUFFS: &[TalentBuffDef] = &[
    // C2 "The Ashen Price": Ring of Searing Radiance form reduces nearby enemies' DEF -20%
    TalentBuffDef {
        name: "The Ashen Price DEF Reduction",
        description: desc!("C2: In Ring of Searing Radiance form, nearby enemies' DEF -20%"),
        stat: BuffableStat::DefReduction,
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
        name: "Sunfrost Encomium ATK Bonus",
        description: desc!("A1: Mavuika's own ATK+30% while in Nightsoul's Blessing state"),
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
        name: "Fire-Forged Heritage DMG Bonus",
        description: desc!(
            "A4: DMG Bonus from Fighting Spirit consumed (max +40% at C0/200pt, adopting max value)"
        ),
        stat: BuffableStat::DmgBonus,
        base_value: 0.40,
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

// ===== Arlecchino =====
// A1 "Crimson Flower Flutters Freely": Pyro DMG Bonus +40% (self)
// A4: ATK scales into All RES reduction — complex, TODO
// C6 "Foul Legacy: Tide Withholder": CR+10%, CD+70% (self)
static ARLECCHINO_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Crimson Flower Pyro DMG Bonus",
        description: desc!("A1: Arlecchino gains Pyro DMG Bonus +40%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.40,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    // TODO: A4 "The Balemoon Alone May Know" — ATK scales into All RES reduction; complex, not implemented
    TalentBuffDef {
        name: "Foul Legacy CRIT Rate Bonus",
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
        name: "Foul Legacy CRIT DMG Bonus",
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

// ===== Dehya =====
// C1 "The Flame Incandescent": MaxHP +20% (self)
// C2 "The Sand-Blanketed Fortress": Skill DMG +50% (self, Toggle)
// C6 "The Burning Claws Cleave": CR+10%, CD+60% (self, Toggle, max values)
static DEHYA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "The Flame Incandescent HP Bonus",
        description: desc!("C1: Dehya's Max HP +20%"),
        stat: BuffableStat::HpPercent,
        base_value: 0.20,
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
        name: "The Sand-Blanketed Fortress Skill DMG Bonus",
        description: desc!("C2: Skill DMG +50%"),
        stat: BuffableStat::SkillDmgBonus,
        base_value: 0.50,
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
        name: "The Burning Claws Cleave CRIT Rate Bonus",
        description: desc!("C6: CRIT Rate +10% (max value)"),
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
        name: "The Burning Claws Cleave CRIT DMG Bonus",
        description: desc!("C6: CRIT DMG +60% (max value)"),
        stat: BuffableStat::CritDmg,
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

// ===== Diluc =====
// A4 "Flowing Flame": Pyro DMG +20% after skill (self)
// C1 "Conviction": DMG +15% vs enemies with HP>50% (self, Toggle)
// C2 "Searing Ember": ATK +10% per stack, max 3 stacks (self, Stacks(3))
// C4 "Flowing Ember": Skill DMG +40% after combo (self, Toggle)
// C6 "Flaming Sword, Nemesis of the Dark": NA DMG +30% during burst (self, Toggle)
static DILUC_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Flowing Flame Pyro DMG Bonus",
        description: desc!("A4: After using Elemental Skill, Pyro DMG +20%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.20,
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
        name: "Conviction DMG Bonus",
        description: desc!("C1: DMG +15% when enemy HP is above 50%"),
        stat: BuffableStat::DmgBonus,
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
        name: "Searing Ember ATK Bonus",
        description: desc!("C2: ATK +10% per stack, max 3 stacks"),
        stat: BuffableStat::AtkPercent,
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
    },
    TalentBuffDef {
        name: "Flowing Ember Skill DMG Bonus",
        description: desc!("C4: After skill combo, Skill DMG +40%"),
        stat: BuffableStat::SkillDmgBonus,
        base_value: 0.40,
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
        name: "Flaming Sword Normal ATK DMG Bonus",
        description: desc!("C6: Normal ATK DMG +30% during burst"),
        stat: BuffableStat::NormalAtkDmgBonus,
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
];

// ===== Gaming =====
// A4 "Dance of Amity": Plunge DMG +20% when HP>=50% (self, Toggle)
// C2 "Savage Hunting Fangs": ATK +20% (self, Toggle)
// C6 "Dance of Auspicious Crane": CR+20%, CD+40% (self, Toggle)
static GAMING_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Dance of Amity Plunging ATK DMG Bonus",
        description: desc!("A4: Plunging ATK DMG +20% when HP is 50% or above"),
        stat: BuffableStat::PlungingAtkDmgBonus,
        base_value: 0.20,
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
        name: "Savage Hunting Fangs ATK Bonus",
        description: desc!("C2: ATK +20%"),
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
        name: "Dance of Auspicious Crane CRIT Rate Bonus",
        description: desc!("C6: CRIT Rate +20%"),
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
        name: "Dance of Auspicious Crane CRIT DMG Bonus",
        description: desc!("C6: CRIT DMG +40%"),
        stat: BuffableStat::CritDmg,
        base_value: 0.40,
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

// ===== Hu Tao =====
// A1 "Flutter By": Team CR+12% after skill ends (TeamExcludeSelf, Toggle)
// A4 "Sanguine Rouge": Pyro DMG +33% when HP≤50% (self, Toggle)
// C4 "Garden of Eternal Rest": After defeating Blood Blossom enemy, party (excl. Hu Tao) CR+12% for 15s
static HU_TAO_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Flutter By CRIT Rate Bonus",
        description: desc!(
            "A1: After Paramita Papilio ends, party members (excl. Hu Tao) gain CRIT Rate +12%"
        ),
        stat: BuffableStat::CritRate,
        base_value: 0.12,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::TeamExcludeSelf,
        source: TalentBuffSource::AscensionPassive(1),
        min_constellation: 0,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Sanguine Rouge Pyro DMG Bonus",
        description: desc!("A4: When HP is 50% or lower, Pyro DMG Bonus +33%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
        base_value: 0.33,
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
        name: "Garden of Eternal Rest",
        description: desc!(
            "C4: After defeating Blood Blossom enemy, party (excl. Hu Tao) CRIT Rate +12% for 15s"
        ),
        stat: BuffableStat::CritRate,
        base_value: 0.12,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::TeamExcludeSelf,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    TalentBuffDef {
        name: "Moment of Death CRIT Rate Bonus",
        description: desc!("C6: When triggered, Hu Tao gains CRIT Rate +100% (max value)"),
        stat: BuffableStat::CritRate,
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

// ===== Xiangling =====
// C1: Pyro RES -15% on Guoba hit
// C6: Pyro DMG +15% during Pyronado
static XIANGLING_BUFFS: &[TalentBuffDef] = &[
    // Spec bug fix: C1/C6 are conditional and must not exist as always-on duplicates.
    TalentBuffDef {
        name: "xiangling_c1_pyro_res_shred",
        description: desc!("C1: Opponents hit by Guoba have Pyro RES -15%"),
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
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
        name: "xiangling_c6_pyro_dmg",
        description: desc!("C6: During Pyronado, party gains Pyro DMG Bonus +15%"),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
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

// ===== Xinyan =====
// C2: Burst Physical CRIT Rate +100%
// C4: Physical RES -15% on skill hit
// C6: Charged ATK gains ATK bonus equal to 50% of DEF
static XINYAN_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Blazing Eye Physical RES Shred",
        description: desc!("C4: Enemies hit by Riff Revolution have Physical RES -15% for 12s"),
        stat: BuffableStat::PhysicalResReduction,
        base_value: 0.15,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::Constellation(4),
        min_constellation: 4,
        cap: None,
        activation: None,
    },
    // C2: Burst Physical CRIT Rate +100% (approximation)
    TalentBuffDef {
        name: "xinyan_c2_burst_crit",
        description: desc!("C2: Riff Revolution Physical DMG CRIT Rate +100% (approximation)"),
        stat: BuffableStat::CritRate,
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
    // C6: Charged ATK flat DMG = 50% DEF
    TalentBuffDef {
        name: "xinyan_c6_charged_def_scaling",
        description: desc!("C6: Charged ATK gains ATK bonus equal to 50% of DEF"),
        stat: BuffableStat::ChargedAtkFlatDmg,
        base_value: 0.50,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: Some(ScalingStat::Def),
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(6),
        min_constellation: 6,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
];

// ===== Lyney =====
// A4 "Prestidigitation": DMG Bonus based on Pyro party count (60-100%); adopting max value 1.00 with Toggle
// C2 "Locquacious Cajoling": CD +20% per stack, max 3 stacks (self, Stacks(3))
// C4 "Seated Next to Polished Nacre": Enemy Pyro RES -20% (Team, Toggle)
static LYNEY_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Prestidigitation DMG Bonus",
        description: desc!(
            "A4: DMG Bonus based on number of Pyro party members (max value 100% with all Pyro, Toggle)"
        ),
        stat: BuffableStat::DmgBonus,
        base_value: 1.00,
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
        name: "Locquacious Cajoling CRIT DMG Bonus",
        description: desc!("C2: CRIT DMG +20% per stack, max 3 stacks"),
        stat: BuffableStat::CritDmg,
        base_value: 0.20,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::OnlySelf,
        source: TalentBuffSource::Constellation(2),
        min_constellation: 2,
        cap: None,
        activation: Some(Activation::Manual(ManualCondition::Stacks(3))),
    },
    TalentBuffDef {
        name: "Seated Next to Polished Nacre Pyro RES Shred",
        description: desc!("C4: Enemy Pyro RES -20%"),
        stat: BuffableStat::ElementalResReduction(Element::Pyro),
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

// ===== Yanfei =====
// A1 "Encyclopedic Expertise": Pyro DMG Bonus +5% per Scarlet Seal, max 4 seals = +20% (self, adopting max value 0.20 with Toggle)
// C2 "Blazing Eye": Charged ATK CRIT Rate +20% (self, Toggle)
static YANFEI_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Encyclopedic Expertise Pyro DMG Bonus",
        description: desc!(
            "A1: Pyro DMG Bonus +5% per Scarlet Seal (max 4 seals = +20%, adopting max value)"
        ),
        stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
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
        name: "Blazing Eye Charged ATK CRIT Rate Bonus",
        description: desc!("C2: Charged-attack-only CRIT Rate +20% (approximation)"),
        stat: BuffableStat::CritRate,
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

// Registry (pub(super) for cross-element uniqueness test)
pub(super) static PYRO_TALENT_BUFFS: &[(&str, &[TalentBuffDef])] = &[
    ("amber", AMBER_BUFFS),
    ("arlecchino", ARLECCHINO_BUFFS),
    ("bennett", BENNETT_BUFFS),
    ("chevreuse", CHEVREUSE_BUFFS),
    ("dehya", DEHYA_BUFFS),
    ("diluc", DILUC_BUFFS),
    ("durin", DURIN_BUFFS),
    ("gaming", GAMING_BUFFS),
    ("hu_tao", HU_TAO_BUFFS),
    ("klee", KLEE_BUFFS),
    ("lyney", LYNEY_BUFFS),
    ("mavuika", MAVUIKA_BUFFS),
    ("thoma", THOMA_BUFFS),
    ("xiangling", XIANGLING_BUFFS),
    ("xinyan", XINYAN_BUFFS),
    ("yanfei", YANFEI_BUFFS),
    ("yoimiya", YOIMIYA_BUFFS),
];

pub fn find(character_id: &str) -> Option<&'static [TalentBuffDef]> {
    PYRO_TALENT_BUFFS
        .iter()
        .find(|(id, _)| *id == character_id)
        .map(|(_, buffs)| *buffs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn audit_pyro_remaining_items() {
        let amber = find("amber").unwrap();
        assert_eq!(amber.len(), 1);

        let chevreuse = find("chevreuse").unwrap();
        assert_eq!(chevreuse.len(), 5);
        assert!(chevreuse.iter().all(|b| {
            b.stat != BuffableStat::ElementalResReduction(Element::Pyro)
                || b.source == TalentBuffSource::AscensionPassive(1)
        }));
        assert!(chevreuse.iter().all(|b| {
            b.stat != BuffableStat::ElementalResReduction(Element::Electro)
                || b.source == TalentBuffSource::AscensionPassive(1)
        }));

        let durin = find("durin").unwrap();
        assert_eq!(durin.len(), 6);
        assert!(durin.iter().any(|b| b.name == "durin_c4_burst_dmg"
            && matches!(
                b.activation,
                Some(Activation::Manual(ManualCondition::Toggle))
            )));
        assert!(
            !durin
                .iter()
                .any(|b| b.name == "Emanare's Source Burst DMG Bonus" && b.activation.is_none())
        );

        let hu_tao = find("hu_tao").unwrap();
        assert_eq!(hu_tao.len(), 4);
        assert!(
            hu_tao.iter().any(|b| {
                b.source == TalentBuffSource::Constellation(6)
                    && b.stat == BuffableStat::CritRate
                    && (b.base_value - 1.0).abs() < 1e-6
                    && b.target == BuffTarget::OnlySelf
                    && matches!(
                        b.activation,
                        Some(Activation::Manual(ManualCondition::Toggle))
                    )
            }),
            "Hu Tao C6 self CRIT Rate buff missing"
        );

        let klee = find("klee").unwrap();
        assert_eq!(klee.len(), 4);
        assert!(klee.iter().any(|b| {
            b.source == TalentBuffSource::Constellation(1)
                && b.stat == BuffableStat::AtkPercent
                && (b.base_value - 0.60).abs() < 1e-6
                && b.target == BuffTarget::OnlySelf
                && matches!(
                    b.activation,
                    Some(Activation::Manual(ManualCondition::Toggle))
                )
        }));
        assert!(klee.iter().any(|b| {
            b.source == TalentBuffSource::Constellation(6)
                && b.stat == BuffableStat::ElementalDmgBonus(Element::Pyro)
                && (b.base_value - 0.50).abs() < 1e-6
                && b.target == BuffTarget::OnlySelf
                && matches!(
                    b.activation,
                    Some(Activation::Manual(ManualCondition::Toggle))
                )
        }));
        assert!(klee.iter().any(|b| {
            b.source == TalentBuffSource::Constellation(6)
                && b.stat == BuffableStat::ElementalDmgBonus(Element::Pyro)
                && (b.base_value - 0.10).abs() < 1e-6
                && b.target == BuffTarget::Team
                && matches!(
                    b.activation,
                    Some(Activation::Manual(ManualCondition::Toggle))
                )
        }));
    }

    #[test]
    fn audit_pyro_more_remaining_items() {
        let mavuika = find("mavuika").unwrap();
        assert_eq!(mavuika.len(), 3);

        let xinyan = find("xinyan").unwrap();
        assert_eq!(xinyan.len(), 3);

        let yanfei = find("yanfei").unwrap();
        assert_eq!(yanfei.len(), 2);
        assert!(yanfei.iter().any(|b| {
            b.source == TalentBuffSource::Constellation(2)
                && b.stat == BuffableStat::CritRate
                && (b.base_value - 0.20).abs() < 1e-6
                && b.target == BuffTarget::OnlySelf
                && matches!(
                    b.activation,
                    Some(Activation::Manual(ManualCondition::Toggle))
                )
                && b.description.contains("approximation")
        }));

        let yoimiya = find("yoimiya").unwrap();
        assert_eq!(yoimiya.len(), 4);
    }
}
