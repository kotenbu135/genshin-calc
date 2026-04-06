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
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
    },
    // C1 "Grand Expectation": +20% of Base ATK added to Fantastic Voyage ATK bonus
    TalentBuffDef {
        name: "Grand Expectation",
        description: "C1: Fantastic Voyage ATK bonus gains additional 20% of Bennett's Base ATK",
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
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
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
    activation: None,
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
        activation: Some(Activation::Both(
            AutoCondition::TeamElementsOnly(&[Element::Pyro, Element::Electro]),
            ManualCondition::Toggle,
        )),
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
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
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
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
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
        activation: None,
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
        activation: None,
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
        activation: None,
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
        activation: None,
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
        activation: None,
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
    activation: None,
}];

// ===== Durin =====
// A1 (Purity) "Light Manifest of the Divine Calculus": Pyro RES -20% after Burning/Overloaded/Pyro Swirl/Pyro Crystallize
// A1 (Darkness) "Light Manifest of the Divine Calculus": Vaporize/Melt DMG +40%
// A4 "Chaos Formed Like the Night": ATK-scaled burst DMG bonus — complex, not implemented
// C2 "Unground Visions": Pyro DMG +50% for party after reaction
// C4 "Emanare's Source": Burst DMG +40%
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
        activation: None,
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
        activation: None,
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
        activation: None,
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
        activation: None,
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
        activation: None,
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
        activation: None,
    },
];

// ===== Mavuika =====
// A1 "Sunfrost Encomium": Mavuika's own ATK+30% while in Nightsoul's Blessing state
// A4 "Fire-Forged Heritage" "Kiongozi": DMG Bonus +0.2% per Fighting Spirit point consumed (max +40% at C0, 200pt)
static MAVUIKA_BUFFS: &[TalentBuffDef] = &[
    // C2 "The Ashen Price": Ring of Searing Radiance form reduces nearby enemies' DEF -20%
    TalentBuffDef {
        name: "The Ashen Price DEF Reduction",
        description: "C2: In Ring of Searing Radiance form, nearby enemies' DEF -20%",
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
        activation: Some(Activation::Manual(ManualCondition::Toggle)),
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
        description: "A1: Arlecchino gains Pyro DMG Bonus +40%",
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
        description: "C6: CRIT Rate +10%",
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
        description: "C6: CRIT DMG +70%",
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
        description: "C1: Dehya's Max HP +20%",
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
        description: "C2: Skill DMG +50%",
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
        description: "C6: CRIT Rate +10% (max value)",
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
        description: "C6: CRIT DMG +60% (max value)",
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
        description: "A4: After using Elemental Skill, Pyro DMG +20%",
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
        description: "C1: DMG +15% when enemy HP is above 50%",
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
        description: "C2: ATK +10% per stack, max 3 stacks",
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
        description: "C4: After skill combo, Skill DMG +40%",
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
        description: "C6: Normal ATK DMG +30% during burst",
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
        description: "A4: Plunging ATK DMG +20% when HP is 50% or above",
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
        description: "C2: ATK +20%",
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
        description: "C6: CRIT Rate +20%",
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
        description: "C6: CRIT DMG +40%",
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
        description: "A1: After Paramita Papilio ends, party members (excl. Hu Tao) gain CRIT Rate +12%",
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
        description: "A4: When HP is 50% or lower, Pyro DMG Bonus +33%",
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
        description: "C4: After defeating Blood Blossom enemy, party (excl. Hu Tao) CRIT Rate +12% for 15s",
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
        activation: None,
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
        activation: None,
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
    activation: None,
}];

// ===== Lyney =====
// A4 "Prestidigitation": DMG Bonus based on Pyro party count (60-100%); adopting max value 1.00 with Toggle
// C2 "Locquacious Cajoling": CD +20% per stack, max 3 stacks (self, Stacks(3))
// C4 "Seated Next to Polished Nacre": Enemy Pyro RES -20% (Team, Toggle)
static LYNEY_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Prestidigitation DMG Bonus",
        description: "A4: DMG Bonus based on number of Pyro party members (max value 100% with all Pyro, Toggle)",
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
        description: "C2: CRIT DMG +20% per stack, max 3 stacks",
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
        description: "C4: Enemy Pyro RES -20%",
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
        description: "A1: Pyro DMG Bonus +5% per Scarlet Seal (max 4 seals = +20%, adopting max value)",
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
        description: "C2: Charged ATK CRIT Rate +20%",
        stat: BuffableStat::ChargedAtkDmgBonus,
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
