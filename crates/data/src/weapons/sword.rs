use crate::buff::{
    Activation, AutoCondition, BuffTarget, BuffableStat, ConditionalBuff, ManualCondition,
    PassiveEffect, StatBuff,
};
use crate::types::{Rarity, WeaponData, WeaponPassive, WeaponSubStat, WeaponType};

// =============================================================================
// 5-Star Swords
// =============================================================================

pub const ABSOLUTION: WeaponData = WeaponData {
    id: "absolution",
    name: "Absolution",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.096, 0.402, 0.402, 0.441])),
    passive: Some(WeaponPassive {
        name: "Absolution",
        effect: PassiveEffect {
            description: desc!("CRIT DMG+20-40%。罪禍を蓄積して追加CRIT DMGを獲得"),
            buffs: &[StatBuff {
                stat: BuffableStat::CritDmg,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "absolution_crit_dmg",
                description: desc!("罪禍スタックごとにCRIT DMG+16-32%"),
                stat: BuffableStat::CritDmg,
                value: 0.16,
                nightsoul_value: None,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
        },
    }),
};

pub const AQUILA_FAVONIA: WeaponData = WeaponData {
    id: "aquila_favonia",
    name: "Aquila Favonia",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::PhysicalDmgBonus([
        0.090, 0.377, 0.377, 0.413,
    ])),
    passive: Some(WeaponPassive {
        name: "西風の鷹の抗い",
        effect: PassiveEffect {
            description: desc!(
                "ATK+20-40%。ダメージを受けると再生効果と追加ATKダメージ、15秒に1回"
            ),
            buffs: &[StatBuff {
                stat: BuffableStat::AtkPercent,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
            }],
            conditional_buffs: &[],
        },
    }),
};

pub const ATHAME_ARTIS: WeaponData = WeaponData {
    id: "athame_artis",
    name: "Athame Artis",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.072, 0.302, 0.302, 0.331])),
    passive: Some(WeaponPassive {
        name: "Athame Artis",
        effect: PassiveEffect {
            description: desc!(
                "元素スキル使用後にスキルCR+10-20%/スキルDMG+8-16%（CritRate近似値）"
            ),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "athame_artis_skill_cr",
                    description: desc!("元素スキル使用後にCR+10-20%（スキルのみ。CritRate近似値）"),
                    stat: BuffableStat::CritRate,
                    value: 0.10,
                    nightsoul_value: None,
                    refinement_values: Some([0.10, 0.125, 0.15, 0.175, 0.20]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "athame_artis_skill_dmg",
                    description: desc!("元素スキル使用後にスキルDMG+8-16%"),
                    stat: BuffableStat::SkillDmgBonus,
                    value: 0.08,
                    nightsoul_value: None,
                    refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const AZURELIGHT: WeaponData = WeaponData {
    id: "azurelight",
    name: "Azurelight",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.048, 0.201, 0.201, 0.221])),
    passive: Some(WeaponPassive {
        name: "Azurelight",
        effect: PassiveEffect {
            description: desc!("HP上限×0.16-0.32%分をNA DMGボーナスに加算（条件付き、上限40-80%）"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "azurelight_hp_na_dmg",
                description: desc!("HP上限×0.16-0.32%分をNA DMGに加算"),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.0016,
                nightsoul_value: None,
                refinement_values: Some([0.0016, 0.002, 0.0024, 0.0028, 0.0032]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Both(
                    AutoCondition::StatScaling {
                        stat: BuffableStat::HpPercent,
                        offset: None,
                        cap: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
                    },
                    ManualCondition::Toggle,
                ),
            }],
        },
    }),
};

pub const FREEDOM_SWORN: WeaponData = WeaponData {
    id: "freedom_sworn",
    name: "Freedom-Sworn",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([43.0, 181.0, 181.0, 198.0])),
    passive: Some(WeaponPassive {
        name: "革命の翼",
        effect: PassiveEffect {
            description: desc!(
                "DMG+10-20%。元素反応時に印を蓄積、2つでチーム全員にATK+20%/NA・CA・PlungeDMG+16%"
            ),
            buffs: &[StatBuff {
                stat: BuffableStat::DmgBonus,
                value: 0.10,
                refinement_values: Some([0.10, 0.125, 0.15, 0.175, 0.20]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "freedom_sworn_team_atk",
                    description: desc!("印2個蓄積後にチーム全員にATK+20-40%"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.20,
                    nightsoul_value: None,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::Team,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "freedom_sworn_team_na_dmg",
                    description: desc!("印2個蓄積後にチーム全員にNA/CA/PlungeDMG+16-32%"),
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.16,
                    nightsoul_value: None,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::Team,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "freedom_sworn_team_ca_dmg",
                    description: desc!("印2個蓄積後にチーム全員にNA/CA/PlungeDMG+16-32%"),
                    stat: BuffableStat::ChargedAtkDmgBonus,
                    value: 0.16,
                    nightsoul_value: None,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::Team,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "freedom_sworn_team_plunge_dmg",
                    description: desc!("印2個蓄積後にチーム全員にNA/CA/PlungeDMG+16-32%"),
                    stat: BuffableStat::PlungingAtkDmgBonus,
                    value: 0.16,
                    nightsoul_value: None,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::Team,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const HARAN_GEPPAKU_FUTSU: WeaponData = WeaponData {
    id: "haran_geppaku_futsu",
    name: "Haran Geppaku Futsu",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.072, 0.302, 0.302, 0.331])),
    passive: Some(WeaponPassive {
        name: "白月の一太刀",
        effect: PassiveEffect {
            description: desc!(
                "元素DMG+12-24%。チームメンバーが元素スキルを使うと通常攻撃DMGアップ"
            ),
            buffs: &[StatBuff {
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "haran_na_dmg",
                description: desc!("チームメンバーのスキル使用でNA DMG+12-24%（最大2スタック）"),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.12,
                nightsoul_value: None,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(2)),
            }],
        },
    }),
};

pub const KEY_OF_KHAJ_NISUT: WeaponData = WeaponData {
    id: "key_of_khaj_nisut",
    name: "Key of Khaj-Nisut",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.144, 0.603, 0.603, 0.662])),
    passive: Some(WeaponPassive {
        name: "砂漠の導きの鍵",
        effect: PassiveEffect {
            description: desc!(
                "Conditional: HP上限に基づき元素熟知アップ。フルスタックでチーム全員にEM付与"
            ),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "khaj_nisut_hp_em",
                description: desc!("HP上限×0.12-0.24%分をEMに加算"),
                stat: BuffableStat::ElementalMastery,
                value: 0.0012,
                nightsoul_value: None,
                refinement_values: Some([0.0012, 0.0015, 0.0018, 0.0021, 0.0024]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::HpPercent,
                    offset: None,
                    cap: None,
                }),
            }],
        },
    }),
};

pub const LIGHT_OF_FOLIAR_INCISION: WeaponData = WeaponData {
    id: "light_of_foliar_incision",
    name: "Light of Foliar Incision",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.192, 0.804, 0.804, 0.882])),
    passive: Some(WeaponPassive {
        name: "葉に宿る白露",
        effect: PassiveEffect {
            description: desc!("CRIT Rate+4-8%。EMに基づきNA/SkillDMGアップ"),
            buffs: &[StatBuff {
                stat: BuffableStat::CritRate,
                value: 0.04,
                refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "foliar_em_normal_flat",
                    description: desc!("EM×120-240%分を通常攻撃フラットダメージに加算"),
                    stat: BuffableStat::NormalAtkFlatDmg,
                    value: 1.20,
                    nightsoul_value: None,
                    refinement_values: Some([1.20, 1.50, 1.80, 2.10, 2.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Both(
                        AutoCondition::StatScaling {
                            stat: BuffableStat::ElementalMastery,
                            offset: None,
                            cap: None,
                        },
                        ManualCondition::Toggle,
                    ),
                },
                ConditionalBuff {
                    name: "foliar_em_skill_flat",
                    description: desc!("EM×120-240%分をスキルフラットダメージに加算"),
                    stat: BuffableStat::SkillFlatDmg,
                    value: 1.20,
                    nightsoul_value: None,
                    refinement_values: Some([1.20, 1.50, 1.80, 2.10, 2.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Both(
                        AutoCondition::StatScaling {
                            stat: BuffableStat::ElementalMastery,
                            offset: None,
                            cap: None,
                        },
                        ManualCondition::Toggle,
                    ),
                },
            ],
        },
    }),
};

pub const LIGHTBEARING_MOONSHARD: WeaponData = WeaponData {
    id: "lightbearing_moonshard",
    name: "Lightbearing Moonshard",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.192, 0.804, 0.804, 0.882])),
    passive: Some(WeaponPassive {
        name: "Lightbearing Moonshard",
        effect: PassiveEffect {
            description: desc!("月光の力でATK+24-48%/DMG+20-40%"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "lightbearing_atk",
                    description: desc!("月光発動時にATK+24-48%"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.24,
                    nightsoul_value: None,
                    refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "lightbearing_dmg",
                    description: desc!("月光発動時にDMG+20-40%"),
                    stat: BuffableStat::DmgBonus,
                    value: 0.20,
                    nightsoul_value: None,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const MISTSPLITTER_REFORGED: WeaponData = WeaponData {
    id: "mistsplitter_reforged",
    name: "Mistsplitter Reforged",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.096, 0.402, 0.402, 0.441])),
    passive: Some(WeaponPassive {
        name: "霧切の巴紋",
        effect: PassiveEffect {
            description: desc!("元素DMG+12-24%。霧切の巴紋を蓄積して追加元素DMGアップ"),
            buffs: &[StatBuff {
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "mistsplitter_emblem",
                description: desc!("霧切の巴紋: 1/2/3スタックで元素DMG+8%/16%/28% (R1)"),
                stat: BuffableStat::DmgBonus,
                value: 0.08,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: Some(&[0.08, 0.16, 0.28]),
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
        },
    }),
};

pub const PEAK_PATROL_SONG: WeaponData = WeaponData {
    id: "peak_patrol_song",
    name: "Peak Patrol Song",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::DefPercent([0.180, 0.754, 0.754, 0.827])),
    passive: Some(WeaponPassive {
        name: "Peak Patrol Song",
        effect: PassiveEffect {
            description: desc!(
                "Ode to Flowers: スタックごとにDEF%/元素DMGアップ。2スタック時チームにDEFベース元素DMG付与"
            ),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "peak_patrol_self_def",
                    description: desc!("Ode to Flowers: DEF+8-16% per stack (max 2)"),
                    stat: BuffableStat::DefPercent,
                    value: 0.08,
                    nightsoul_value: None,
                    refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(2)),
                },
                ConditionalBuff {
                    name: "peak_patrol_self_dmg",
                    description: desc!(
                        "Ode to Flowers: All Elemental DMG+10-20% per stack (max 2)"
                    ),
                    stat: BuffableStat::AllElementalDmgBonus,
                    value: 0.10,
                    nightsoul_value: None,
                    refinement_values: Some([0.10, 0.125, 0.15, 0.175, 0.20]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(2)),
                },
                ConditionalBuff {
                    name: "peak_patrol_team_dmg",
                    description: desc!(
                        "2 stacks: party All Elemental DMG based on 8-16% of DEF (cap 25.6-51.2%)"
                    ),
                    stat: BuffableStat::AllElementalDmgBonus,
                    value: 0.00008,
                    nightsoul_value: None,
                    refinement_values: Some([0.00008, 0.00010, 0.00012, 0.00014, 0.00016]),
                    stack_values: None,
                    target: BuffTarget::Team,
                    activation: Activation::Both(
                        AutoCondition::StatScaling {
                            stat: BuffableStat::DefPercent,
                            offset: None,
                            cap: Some([0.256, 0.32, 0.384, 0.448, 0.512]),
                        },
                        ManualCondition::Toggle,
                    ),
                },
            ],
        },
    }),
};

pub const PRIMORDIAL_JADE_CUTTER: WeaponData = WeaponData {
    id: "primordial_jade_cutter",
    name: "Primordial Jade Cutter",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.096, 0.402, 0.402, 0.441])),
    passive: Some(WeaponPassive {
        name: "護国の無垢",
        effect: PassiveEffect {
            description: desc!("HP+20-40%。HP上限の1.2-2.4%分ATKアップ"),
            buffs: &[StatBuff {
                stat: BuffableStat::HpPercent,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "jade_cutter_hp_atk",
                description: desc!("HP上限の1.2%分ATKアップ（HP%に比例）"),
                stat: BuffableStat::AtkFlat,
                value: 0.012,
                nightsoul_value: None,
                refinement_values: Some([0.012, 0.015, 0.018, 0.021, 0.024]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::HpPercent,
                    offset: None,
                    cap: None,
                }),
            }],
        },
    }),
};

pub const SKYWARD_BLADE: WeaponData = WeaponData {
    id: "skyward_blade",
    name: "Skyward Blade",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "天空の刃",
        effect: PassiveEffect {
            description: desc!("CRIT Rate+4-8%。元素爆発後にNA/CAの速度とDMGアップ"),
            buffs: &[StatBuff {
                stat: BuffableStat::CritRate,
                value: 0.04,
                refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
            }],
            conditional_buffs: &[],
        },
    }),
};

pub const SPLENDOR_OF_TRANQUIL_WATERS: WeaponData = WeaponData {
    id: "splendor_of_tranquil_waters",
    name: "Splendor of Tranquil Waters",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.192, 0.804, 0.804, 0.882])),
    passive: Some(WeaponPassive {
        name: "静水流転の輝き",
        effect: PassiveEffect {
            description: desc!(
                "Skill DMG+8-16%。HP変動時にNA DMGアップ、NA命中時にSkill DMGアップ"
            ),
            buffs: &[StatBuff {
                stat: BuffableStat::SkillDmgBonus,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "splendor_na_dmg",
                    description: desc!("HP変動時にNA DMG+8-16%（最大3スタック）"),
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.08,
                    nightsoul_value: None,
                    refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(3)),
                },
                ConditionalBuff {
                    name: "splendor_skill_dmg",
                    description: desc!("NA命中時にSkill DMG+6-12%（最大3スタック）"),
                    stat: BuffableStat::SkillDmgBonus,
                    value: 0.06,
                    nightsoul_value: None,
                    refinement_values: Some([0.06, 0.075, 0.09, 0.105, 0.12]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(3)),
                },
            ],
        },
    }),
};

pub const SUMMIT_SHAPER: WeaponData = WeaponData {
    id: "summit_shaper",
    name: "Summit Shaper",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.108, 0.453, 0.453, 0.496])),
    passive: Some(WeaponPassive {
        name: "金璋の頂に登る",
        effect: PassiveEffect {
            description: desc!("攻撃命中でATK+4-8%スタック（最大5）、シールド時は2倍"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "summit_shaper_atk_stacks",
                    description: desc!("攻撃命中でATK+4-8%（1スタック）、最大5スタック"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    nightsoul_value: None,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
                ConditionalBuff {
                    name: "summit_shaper_shield_atk_stacks",
                    description: desc!("シールド時にATKスタック効果2倍分（追加ATK+4-8%/スタック）"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    nightsoul_value: None,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
            ],
        },
    }),
};

pub const URAKU_MISUGIRI: WeaponData = WeaponData {
    id: "uraku_misugiri",
    name: "Uraku Misugiri",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.192, 0.804, 0.804, 0.882])),
    passive: Some(WeaponPassive {
        name: "浮楽の御簾切り",
        effect: PassiveEffect {
            description: desc!("NA DMG+16-32%。DEFに基づき元素スキルDMGアップ"),
            buffs: &[StatBuff {
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "uraku_def_skill",
                description: desc!("DEF増加分×18-36%分をスキルDMGボーナスに加算"),
                stat: BuffableStat::SkillDmgBonus,
                value: 0.18,
                nightsoul_value: None,
                refinement_values: Some([0.18, 0.225, 0.27, 0.315, 0.36]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::DefPercentRaw,
                    offset: None,
                    cap: None,
                }),
            }],
        },
    }),
};

// =============================================================================
// 4-Star Swords
// =============================================================================

pub const AMENOMA_KAGEUCHI: WeaponData = WeaponData {
    id: "amenoma_kageuchi",
    name: "Amenoma Kageuchi",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "天目影打の鍛造",
        effect: PassiveEffect {
            description: desc!(
                "Conditional: 元素スキルで継承の印を獲得、元素爆発時にエネルギー回復"
            ),
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const BLACKCLIFF_LONGSWORD: WeaponData = WeaponData {
    id: "blackcliff_longsword",
    name: "Blackcliff Longsword",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.080, 0.335, 0.335, 0.368])),
    passive: Some(WeaponPassive {
        name: "追撃の一矢",
        effect: PassiveEffect {
            description: desc!("Conditional: 敵を倒すとATK+12%、30秒、3スタックまで"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "blackcliff_longsword_atk",
                description: desc!("敵撃破ごとにATK+12-24%（最大3スタック）"),
                stat: BuffableStat::AtkPercent,
                value: 0.12,
                nightsoul_value: None,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
        },
    }),
};

pub const CALAMITY_OF_ESHU: WeaponData = WeaponData {
    id: "calamity_of_eshu",
    name: "Calamity of Eshu",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Calamity of Eshu",
        effect: PassiveEffect {
            description: desc!("Conditional: 元素反応時にスキルDMGとCRIT Rateがアップ"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "calamity_eshu_skill_dmg",
                    description: desc!("元素反応後にSkill DMG+20-40%"),
                    stat: BuffableStat::SkillDmgBonus,
                    value: 0.20,
                    nightsoul_value: None,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "calamity_eshu_crit_rate",
                    description: desc!("元素反応後にCRIT Rate+6-12%"),
                    stat: BuffableStat::CritRate,
                    value: 0.06,
                    nightsoul_value: None,
                    refinement_values: Some([0.06, 0.075, 0.09, 0.105, 0.12]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const CINNABAR_SPINDLE: WeaponData = WeaponData {
    id: "cinnabar_spindle",
    name: "Cinnabar Spindle",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::DefPercent([0.150, 0.629, 0.629, 0.690])),
    passive: Some(WeaponPassive {
        name: "辰砂の紡錘",
        effect: PassiveEffect {
            description: desc!("元素スキルのDMGにDEFの40-80%分を加算"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "cinnabar_skill_def_flat",
                description: desc!("元素スキルのDMGにDEFの40-80%分を加算"),
                stat: BuffableStat::SkillFlatDmg,
                value: 0.40,
                nightsoul_value: None,
                refinement_values: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::DefPercent,
                    offset: None,
                    cap: None,
                }),
            }],
        },
    }),
};

pub const FAVONIUS_SWORD: WeaponData = WeaponData {
    id: "favonius_sword",
    name: "Favonius Sword",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.133, 0.557, 0.557, 0.613])),
    passive: Some(WeaponPassive {
        name: "順風の加護",
        effect: PassiveEffect {
            description: desc!("Conditional: 会心命中時に元素粒子を生成、12秒に1回"),
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const FESTERING_DESIRE: WeaponData = WeaponData {
    id: "festering_desire",
    name: "Festering Desire",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.100, 0.419, 0.419, 0.459])),
    passive: Some(WeaponPassive {
        name: "腐植の剣",
        effect: PassiveEffect {
            description: desc!("Skill DMG+16-32%、Skill CRIT Rate+6-12%"),
            buffs: &[StatBuff {
                stat: BuffableStat::SkillDmgBonus,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "festering_skill_cr",
                description: desc!("元素スキルのCRIT Rate+6-12%"),
                stat: BuffableStat::CritRate,
                value: 0.06,
                nightsoul_value: None,
                refinement_values: Some([0.06, 0.075, 0.09, 0.105, 0.12]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const FINALE_OF_THE_DEEP: WeaponData = WeaponData {
    id: "finale_of_the_deep",
    name: "Finale of the Deep",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "深淵のフィナーレ",
        effect: PassiveEffect {
            description: desc!("ATK+12-24%。HP増減時にNA/CA/PlungeDMGアップ"),
            buffs: &[StatBuff {
                stat: BuffableStat::AtkPercent,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "finale_deep_na_dmg",
                    description: desc!("HP増減時にNA DMG+12-24%"),
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.12,
                    nightsoul_value: None,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "finale_deep_ca_dmg",
                    description: desc!("HP増減時にCA DMG+12-24%"),
                    stat: BuffableStat::ChargedAtkDmgBonus,
                    value: 0.12,
                    nightsoul_value: None,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "finale_deep_plunge_dmg",
                    description: desc!("HP増減時にPlunge DMG+12-24%"),
                    stat: BuffableStat::PlungingAtkDmgBonus,
                    value: 0.12,
                    nightsoul_value: None,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const FLEUVE_CENDRE_FERRYMAN: WeaponData = WeaponData {
    id: "fleuve_cendre_ferryman",
    name: "Fleuve Cendre Ferryman",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.100, 0.419, 0.419, 0.459])),
    passive: Some(WeaponPassive {
        name: "灰燼の川の渡し守",
        effect: PassiveEffect {
            description: desc!("Conditional: 元素スキルのCRIT RateとER%がアップ"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "fleuve_skill_cr",
                    description: desc!("元素スキル使用後にCRIT Rate+8-16%（スキルのみ）"),
                    stat: BuffableStat::CritRate,
                    value: 0.08,
                    nightsoul_value: None,
                    refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "fleuve_energy_recharge",
                    description: desc!("元素スキル使用後にER+16-32%"),
                    stat: BuffableStat::EnergyRecharge,
                    value: 0.16,
                    nightsoul_value: None,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const FLUTE_OF_EZPITZAL: WeaponData = WeaponData {
    id: "flute_of_ezpitzal",
    name: "Flute of Ezpitzal",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::DefPercent([0.150, 0.629, 0.629, 0.690])),
    passive: Some(WeaponPassive {
        name: "Flute of Ezpitzal",
        effect: PassiveEffect {
            description: desc!("夜魂の加護状態で通常攻撃DMGにDEFの24-48%分を加算"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "flute_ezpitzal_def_flat",
                description: desc!("夜魂の加護状態で通常攻撃DMGにDEFの24-48%分を加算"),
                stat: BuffableStat::NormalAtkFlatDmg,
                value: 0.24,
                nightsoul_value: None,
                refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Both(
                    AutoCondition::StatScaling {
                        stat: BuffableStat::DefPercent,
                        offset: None,
                        cap: None,
                    },
                    ManualCondition::Toggle,
                ),
            }],
        },
    }),
};

pub const IRON_STING: WeaponData = WeaponData {
    id: "iron_sting",
    name: "Iron Sting",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([36.0, 151.0, 151.0, 165.0])),
    passive: Some(WeaponPassive {
        name: "注入の刺突",
        effect: PassiveEffect {
            description: desc!("元素ダメージ命中時にDMG+6-12%、6秒、2スタックまで"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "iron_sting_dmg",
                description: desc!("元素ダメージ命中後、全DMG+6-12%（最大2スタック）"),
                stat: BuffableStat::DmgBonus,
                value: 0.06,
                nightsoul_value: None,
                refinement_values: Some([0.06, 0.075, 0.09, 0.105, 0.12]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(2)),
            }],
        },
    }),
};

pub const KAGOTSURUBE_ISSHIN: WeaponData = WeaponData {
    id: "kagotsurube_isshin",
    name: "Kagotsurube Isshin",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "名刀・籠釣瓶一心",
        effect: PassiveEffect {
            description: desc!("Conditional: NA/CA/Plunge命中時にATK+15%、8秒"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "kagotsurube_atk",
                description: desc!("NA/CA/Plunge命中時にATK+15%（イベント武器、精錬なし）"),
                stat: BuffableStat::AtkPercent,
                value: 0.15,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const LIONS_ROAR: WeaponData = WeaponData {
    id: "lions_roar",
    name: "Lion's Roar",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "獅子の咆哮",
        effect: PassiveEffect {
            description: desc!("炎/雷の影響を受けた敵にDMG+20-36%"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "lions_roar_dmg",
                description: desc!("炎/雷元素の影響下の敵へのDMG+20-36%"),
                stat: BuffableStat::DmgBonus,
                value: 0.20,
                nightsoul_value: None,
                refinement_values: Some([0.20, 0.24, 0.28, 0.32, 0.36]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const MOONWEAVERS_DAWN: WeaponData = WeaponData {
    id: "moonweavers_dawn",
    name: "Moonweaver's Dawn",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Moonweaver's Dawn",
        effect: PassiveEffect {
            description: desc!("Conditional: 元素スキルまたは元素爆発の命中時にDMGバフ獲得"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "moonweavers_dawn_dmg",
                description: desc!("Skill/Burst命中後にDMG+16-32%"),
                stat: BuffableStat::DmgBonus,
                value: 0.16,
                nightsoul_value: None,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const PRIZED_ISSHIN_BLADE: WeaponData = WeaponData {
    id: "prized_isshin_blade",
    name: "Prized Isshin Blade",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "名刀一心",
        effect: PassiveEffect {
            description: desc!("元素スキル命中後にATK+12-24%"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "prized_isshin_atk",
                description: desc!("元素スキル命中後にATK+12-24%"),
                stat: BuffableStat::AtkPercent,
                value: 0.12,
                nightsoul_value: None,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const PROTOTYPE_RANCOUR: WeaponData = WeaponData {
    id: "prototype_rancour",
    name: "Prototype Rancour",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::PhysicalDmgBonus([
        0.075, 0.314, 0.314, 0.345,
    ])),
    passive: Some(WeaponPassive {
        name: "砕石の試練",
        effect: PassiveEffect {
            description: desc!("Conditional: NA/CA命中時にATK/DEF+4%、6秒、4スタックまで"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "prototype_rancour_atk",
                    description: desc!("NA/CA命中時にATK+4-8%（最大4スタック）"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    nightsoul_value: None,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(4)),
                },
                ConditionalBuff {
                    name: "prototype_rancour_def",
                    description: desc!("NA/CA命中時にDEF+4-8%（最大4スタック）"),
                    stat: BuffableStat::DefPercent,
                    value: 0.04,
                    nightsoul_value: None,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(4)),
                },
            ],
        },
    }),
};

pub const ROYAL_LONGSWORD: WeaponData = WeaponData {
    id: "royal_longsword",
    name: "Royal Longsword",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "集中",
        effect: PassiveEffect {
            description: desc!("ダメージ命中で会心でなければCRIT Rate+8%×5スタック（最大40-80%）"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "royal_longsword_cr",
                description: desc!(
                    "ダメージ命中で会心でなければCRIT Rate+8%×5スタック（最大40-80%）"
                ),
                stat: BuffableStat::CritRate,
                value: 0.40,
                nightsoul_value: None,
                refinement_values: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const SACRIFICIAL_SWORD: WeaponData = WeaponData {
    id: "sacrificial_sword",
    name: "Sacrificial Sword",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.133, 0.557, 0.557, 0.613])),
    passive: Some(WeaponPassive {
        name: "気定神閑",
        effect: PassiveEffect {
            description: desc!("Conditional: 元素スキル命中時に40%の確率でCD即リセット、30秒に1回"),
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const SAPWOOD_BLADE: WeaponData = WeaponData {
    id: "sapwood_blade",
    name: "Sapwood Blade",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "森林の活力",
        effect: PassiveEffect {
            description: desc!("Conditional: 草元素反応時に葉を生成、拾うとEM+60、12秒"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "sapwood_blade_em",
                description: desc!("草元素反応後に葉を拾うとEM+60-120"),
                stat: BuffableStat::ElementalMastery,
                value: 60.0,
                nightsoul_value: None,
                refinement_values: Some([60.0, 75.0, 90.0, 105.0, 120.0]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const SERENITYS_CALL: WeaponData = WeaponData {
    id: "serenitys_call",
    name: "Serenity's Call",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.133, 0.557, 0.557, 0.613])),
    passive: Some(WeaponPassive {
        name: "Serenity's Call",
        effect: PassiveEffect {
            description: desc!("元素スキル命中後にDMG+8-16%"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "serenitys_call_dmg",
                description: desc!("元素スキル命中後にDMG+8-16%"),
                stat: BuffableStat::DmgBonus,
                value: 0.08,
                nightsoul_value: None,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const STURDY_BONE: WeaponData = WeaponData {
    id: "sturdy_bone",
    name: "Sturdy Bone",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Sturdy Bone",
        effect: PassiveEffect {
            description: desc!("スプリント後にNA DMG+16-32%"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "sturdy_bone_na_dmg",
                description: desc!("スプリント後にNA DMG+16-32%"),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.16,
                nightsoul_value: None,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const SWORD_OF_DESCENSION: WeaponData = WeaponData {
    id: "sword_of_descension",
    name: "Sword of Descension",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [39.0, 388.0, 414.0, 440.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.077, 0.323, 0.323, 0.352])),
    passive: Some(WeaponPassive {
        name: "降臨の剣",
        effect: PassiveEffect {
            description: desc!("Conditional: 旅人が装備時にNA/CA命中時50%で追加ダメージ、ATK+66"),
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const SWORD_OF_NARZISSENKREUZ: WeaponData = WeaponData {
    id: "sword_of_narzissenkreuz",
    name: "Sword of Narzissenkreuz",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "ナルツィッセンクロイツの剣",
        effect: PassiveEffect {
            description: desc!(
                "Conditional: 装備者が「始まりの大いなる冒険」の影響を受けると元素スキル/元素爆発DMGアップ"
            ),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "narzissenkreuz_skill_dmg",
                    description: desc!("冒険状態時にSkill DMG+32-64%"),
                    stat: BuffableStat::SkillDmgBonus,
                    value: 0.32,
                    nightsoul_value: None,
                    refinement_values: Some([0.32, 0.40, 0.48, 0.56, 0.64]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "narzissenkreuz_burst_dmg",
                    description: desc!("冒険状態時にBurst DMG+32-64%"),
                    stat: BuffableStat::BurstDmgBonus,
                    value: 0.32,
                    nightsoul_value: None,
                    refinement_values: Some([0.32, 0.40, 0.48, 0.56, 0.64]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const THE_ALLEY_FLASH: WeaponData = WeaponData {
    id: "the_alley_flash",
    name: "The Alley Flash",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [45.0, 545.0, 571.0, 620.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([12.0, 50.0, 50.0, 55.0])),
    passive: Some(WeaponPassive {
        name: "裏路地の閃光",
        effect: PassiveEffect {
            description: desc!("DMG+12-24%。ダメージを受けると効果消失、5秒後に再発動"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "alley_flash_dmg",
                description: desc!("DMG+12-24%（被弾で消失、5秒後に再発動）"),
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                nightsoul_value: None,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const THE_BLACK_SWORD: WeaponData = WeaponData {
    id: "the_black_sword",
    name: "The Black Sword",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "黒曜の輝き",
        effect: PassiveEffect {
            description: desc!("NA/CA DMG+20-40%。NA/CA会心時にHP回復"),
            buffs: &[
                StatBuff {
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                },
                StatBuff {
                    stat: BuffableStat::ChargedAtkDmgBonus,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                },
            ],
            conditional_buffs: &[],
        },
    }),
};

pub const THE_DOCKHANDS_ASSISTANT: WeaponData = WeaponData {
    id: "the_dockhands_assistant",
    name: "The Dockhand's Assistant",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "埠頭の助手",
        effect: PassiveEffect {
            description: desc!("Conditional: 元素反応時にEM+40、8秒、3スタックまで"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "dockhands_em",
                description: desc!("元素反応ごとにEM+40-80（最大3スタック）"),
                stat: BuffableStat::ElementalMastery,
                value: 40.0,
                nightsoul_value: None,
                refinement_values: Some([40.0, 50.0, 60.0, 70.0, 80.0]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
        },
    }),
};

pub const THE_FLUTE: WeaponData = WeaponData {
    id: "the_flute",
    name: "The Flute",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "和弦の調べ",
        effect: PassiveEffect {
            description: desc!("Conditional: NA/CA命中時に和音を蓄積、5つでATK100%のDMG"),
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const TOUKABOU_SHIGURE: WeaponData = WeaponData {
    id: "toukabou_shigure",
    name: "Toukabou Shigure",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([36.0, 151.0, 151.0, 165.0])),
    passive: Some(WeaponPassive {
        name: "唐笠の時雨",
        effect: PassiveEffect {
            description: desc!("落下攻撃命中後にDMG+16-32%"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "toukabou_dmg",
                description: desc!("落下攻撃命中後にDMG+16-32%"),
                stat: BuffableStat::DmgBonus,
                value: 0.16,
                nightsoul_value: None,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const WOLF_FANG: WeaponData = WeaponData {
    id: "wolf_fang",
    name: "Wolf-Fang",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "狼牙",
        effect: PassiveEffect {
            description: desc!(
                "Skill/Burst DMG+16-32%。Skill/Burst命中時にDEF-4%、6秒、4スタックまで"
            ),
            buffs: &[
                StatBuff {
                    stat: BuffableStat::SkillDmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                },
                StatBuff {
                    stat: BuffableStat::BurstDmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                },
            ],
            conditional_buffs: &[ConditionalBuff {
                name: "wolf_fang_def_reduction",
                description: desc!("Skill/Burst命中時に敵DEF-4-8%（最大4スタック）"),
                stat: BuffableStat::DefReduction,
                value: 0.04,
                nightsoul_value: None,
                refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(4)),
            }],
        },
    }),
};

pub const XIPHOS_MOONLIGHT: WeaponData = WeaponData {
    id: "xiphos_moonlight",
    name: "Xiphos' Moonlight",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([36.0, 151.0, 151.0, 165.0])),
    passive: Some(WeaponPassive {
        name: "クシフォスの月光",
        effect: PassiveEffect {
            description: desc!("Conditional: EMに基づきERアップ。チームメンバーにもER付与"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "xiphos_self_er",
                    description: desc!("EM×0.036-0.072%分のERを自身に付与"),
                    stat: BuffableStat::EnergyRecharge,
                    value: 0.00036,
                    nightsoul_value: None,
                    refinement_values: Some([0.00036, 0.00045, 0.00054, 0.00063, 0.00072]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::ElementalMastery,
                        offset: None,
                        cap: None,
                    }),
                },
                ConditionalBuff {
                    name: "xiphos_team_er",
                    description: desc!("自身のER付与量の30%をチームメンバーに付与"),
                    stat: BuffableStat::EnergyRecharge,
                    value: 0.000108,
                    nightsoul_value: None,
                    refinement_values: Some([0.000108, 0.0001350, 0.000162, 0.000189, 0.000216]),
                    stack_values: None,
                    target: BuffTarget::TeamExcludeSelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::ElementalMastery,
                        offset: None,
                        cap: None,
                    }),
                },
            ],
        },
    }),
};

// =============================================================================
// 3-Star Swords
// =============================================================================

pub const COOL_STEEL: WeaponData = WeaponData {
    id: "cool_steel",
    name: "Cool Steel",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star3,
    base_atk: [39.0, 355.0, 375.0, 401.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.077, 0.323, 0.323, 0.352])),
    passive: Some(WeaponPassive {
        name: "急凍の恩恵",
        effect: PassiveEffect {
            description: desc!("Conditional: 水/氷の影響を受けた敵にDMG+12%"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "cool_steel_dmg",
                description: desc!("水/氷の影響を受けた敵にDMG+12-24%"),
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                nightsoul_value: None,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const DARK_IRON_SWORD: WeaponData = WeaponData {
    id: "dark_iron_sword",
    name: "Dark Iron Sword",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star3,
    base_atk: [39.0, 355.0, 375.0, 401.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([31.0, 128.0, 128.0, 141.0])),
    passive: Some(WeaponPassive {
        name: "過負荷",
        effect: PassiveEffect {
            description: desc!("Conditional: 雷元素反応時にATK+20%、12秒"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "dark_iron_atk",
                description: desc!("雷元素反応後にATK+20-40%"),
                stat: BuffableStat::AtkPercent,
                value: 0.20,
                nightsoul_value: None,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const FILLET_BLADE: WeaponData = WeaponData {
    id: "fillet_blade",
    name: "Fillet Blade",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star3,
    base_atk: [39.0, 355.0, 375.0, 401.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.077, 0.323, 0.323, 0.352])),
    passive: Some(WeaponPassive {
        name: "鋭利な一刀",
        effect: PassiveEffect {
            description: desc!("Conditional: 命中時に50%の確率でATK240%の追加ダメージ、15秒に1回"),
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const HARBINGER_OF_DAWN: WeaponData = WeaponData {
    id: "harbinger_of_dawn",
    name: "Harbinger of Dawn",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star3,
    base_atk: [39.0, 355.0, 375.0, 401.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.102, 0.427, 0.427, 0.469])),
    passive: Some(WeaponPassive {
        name: "夜明けの曙光",
        effect: PassiveEffect {
            description: desc!("Conditional: HP90%以上でCRIT Rate+14%"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "harbinger_crit_rate",
                description: desc!("HP90%以上でCRIT Rate+14-28%"),
                stat: BuffableStat::CritRate,
                value: 0.14,
                nightsoul_value: None,
                refinement_values: Some([0.14, 0.175, 0.21, 0.245, 0.28]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const SKYRIDER_SWORD: WeaponData = WeaponData {
    id: "skyrider_sword",
    name: "Skyrider Sword",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star3,
    base_atk: [38.0, 314.0, 334.0, 354.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.113, 0.473, 0.473, 0.521])),
    passive: Some(WeaponPassive {
        name: "決意",
        effect: PassiveEffect {
            description: desc!("Conditional: 元素爆発後にNA/CA ATK+12%、15秒"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "skyrider_sword_atk",
                description: desc!("元素爆発後にATK+12-24%"),
                stat: BuffableStat::AtkPercent,
                value: 0.12,
                nightsoul_value: None,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const TRAVELERS_HANDY_SWORD: WeaponData = WeaponData {
    id: "travelers_handy_sword",
    name: "Traveler's Handy Sword",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star3,
    base_atk: [40.0, 396.0, 415.0, 448.0],
    sub_stat: Some(WeaponSubStat::DefPercent([0.064, 0.268, 0.268, 0.293])),
    passive: Some(WeaponPassive {
        name: "旅の道標",
        effect: PassiveEffect {
            description: desc!("Conditional: 元素オーブ/元素粒子を獲得時にHP回復"),
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

// =============================================================================
// 1-2 Star Swords
// =============================================================================

pub const DULL_BLADE: WeaponData = WeaponData {
    id: "dull_blade",
    name: "Dull Blade",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star1,
    base_atk: [23.0, 185.0, 185.0, 185.0],
    sub_stat: None,
    passive: None,
};

pub const SILVER_SWORD: WeaponData = WeaponData {
    id: "silver_sword",
    name: "Silver Sword",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star2,
    base_atk: [33.0, 243.0, 243.0, 243.0],
    sub_stat: None,
    passive: None,
};

/// All sword weapon data references.
pub const ALL_SWORDS: &[&WeaponData] = &[
    // 5-Star
    &ABSOLUTION,
    &AQUILA_FAVONIA,
    &ATHAME_ARTIS,
    &AZURELIGHT,
    &FREEDOM_SWORN,
    &HARAN_GEPPAKU_FUTSU,
    &KEY_OF_KHAJ_NISUT,
    &LIGHT_OF_FOLIAR_INCISION,
    &LIGHTBEARING_MOONSHARD,
    &MISTSPLITTER_REFORGED,
    &PEAK_PATROL_SONG,
    &PRIMORDIAL_JADE_CUTTER,
    &SKYWARD_BLADE,
    &SPLENDOR_OF_TRANQUIL_WATERS,
    &SUMMIT_SHAPER,
    &URAKU_MISUGIRI,
    // 4-Star
    &AMENOMA_KAGEUCHI,
    &BLACKCLIFF_LONGSWORD,
    &CALAMITY_OF_ESHU,
    &CINNABAR_SPINDLE,
    &FAVONIUS_SWORD,
    &FESTERING_DESIRE,
    &FINALE_OF_THE_DEEP,
    &FLEUVE_CENDRE_FERRYMAN,
    &FLUTE_OF_EZPITZAL,
    &IRON_STING,
    &KAGOTSURUBE_ISSHIN,
    &LIONS_ROAR,
    &MOONWEAVERS_DAWN,
    &PRIZED_ISSHIN_BLADE,
    &PROTOTYPE_RANCOUR,
    &ROYAL_LONGSWORD,
    &SACRIFICIAL_SWORD,
    &SAPWOOD_BLADE,
    &SERENITYS_CALL,
    &STURDY_BONE,
    &SWORD_OF_DESCENSION,
    &SWORD_OF_NARZISSENKREUZ,
    &THE_ALLEY_FLASH,
    &THE_BLACK_SWORD,
    &THE_DOCKHANDS_ASSISTANT,
    &THE_FLUTE,
    &TOUKABOU_SHIGURE,
    &WOLF_FANG,
    &XIPHOS_MOONLIGHT,
    // 3-Star
    &COOL_STEEL,
    &DARK_IRON_SWORD,
    &FILLET_BLADE,
    &HARBINGER_OF_DAWN,
    &SKYRIDER_SWORD,
    &TRAVELERS_HANDY_SWORD,
    // 1-2 Star
    &DULL_BLADE,
    &SILVER_SWORD,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buff::AutoCondition;

    #[test]
    fn key_of_khaj_nisut_has_hp_em_conditional() {
        let passive = KEY_OF_KHAJ_NISUT.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "khaj_nisut_hp_em");
        assert_eq!(buff.stat, BuffableStat::ElementalMastery);
        assert!((buff.value - 0.0012).abs() < 1e-7);
        assert!(matches!(
            buff.activation,
            Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::HpPercent,
                offset: None,
                cap: None,
            })
        ));
    }

    #[test]
    fn light_of_foliar_incision_has_em_flatdmg_conditionals() {
        let passive = LIGHT_OF_FOLIAR_INCISION.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);
        assert_eq!(cond_buffs[0].name, "foliar_em_normal_flat");
        assert_eq!(cond_buffs[0].stat, BuffableStat::NormalAtkFlatDmg);
        assert!((cond_buffs[0].value - 1.20).abs() < 1e-6);
        assert_eq!(cond_buffs[1].name, "foliar_em_skill_flat");
        assert_eq!(cond_buffs[1].stat, BuffableStat::SkillFlatDmg);
        assert!((cond_buffs[1].value - 1.20).abs() < 1e-6);
        for buff in cond_buffs {
            assert!(matches!(
                buff.activation,
                Activation::Both(
                    AutoCondition::StatScaling {
                        stat: BuffableStat::ElementalMastery,
                        offset: None,
                        cap: None,
                    },
                    ManualCondition::Toggle
                )
            ));
        }
    }

    #[test]
    fn peak_patrol_song_has_three_conditional_buffs() {
        let passive = PEAK_PATROL_SONG.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 3);

        // Buff 1: Self DEF% per stack
        let self_def = &cond_buffs[0];
        assert_eq!(self_def.name, "peak_patrol_self_def");
        assert_eq!(self_def.stat, BuffableStat::DefPercent);
        assert!((self_def.value - 0.08).abs() < 1e-6);
        assert_eq!(self_def.target, BuffTarget::OnlySelf);
        assert!(matches!(
            self_def.activation,
            Activation::Manual(ManualCondition::Stacks(2))
        ));

        // Buff 2: Self All Elemental DMG% per stack
        let self_dmg = &cond_buffs[1];
        assert_eq!(self_dmg.name, "peak_patrol_self_dmg");
        assert_eq!(self_dmg.stat, BuffableStat::AllElementalDmgBonus);
        assert!((self_dmg.value - 0.10).abs() < 1e-6);
        assert_eq!(self_dmg.target, BuffTarget::OnlySelf);
        assert!(matches!(
            self_dmg.activation,
            Activation::Manual(ManualCondition::Stacks(2))
        ));

        // Buff 3: Team All Elemental DMG based on DEF
        let team_dmg = &cond_buffs[2];
        assert_eq!(team_dmg.name, "peak_patrol_team_dmg");
        assert_eq!(team_dmg.stat, BuffableStat::AllElementalDmgBonus);
        assert!((team_dmg.value - 0.00008).abs() < 1e-8);
        assert_eq!(team_dmg.target, BuffTarget::Team);
        assert!(matches!(
            team_dmg.activation,
            Activation::Both(
                AutoCondition::StatScaling {
                    stat: BuffableStat::DefPercent,
                    offset: None,
                    cap: Some(_),
                },
                ManualCondition::Toggle
            )
        ));
    }

    #[test]
    fn uraku_misugiri_has_def_skill_conditional() {
        let passive = URAKU_MISUGIRI.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "uraku_def_skill");
        assert_eq!(buff.stat, BuffableStat::SkillDmgBonus);
        assert!((buff.value - 0.18).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::DefPercentRaw,
                offset: None,
                cap: None,
            })
        ));
    }

    #[test]
    fn athame_artis_has_skill_cr_and_skill_dmg() {
        let passive = ATHAME_ARTIS.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        let cr_buff = &cond_buffs[0];
        assert_eq!(cr_buff.name, "athame_artis_skill_cr");
        assert_eq!(cr_buff.stat, BuffableStat::CritRate);
        assert!((cr_buff.value - 0.10).abs() < 1e-6);
        assert!(matches!(
            cr_buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));

        let dmg_buff = &cond_buffs[1];
        assert_eq!(dmg_buff.name, "athame_artis_skill_dmg");
        assert_eq!(dmg_buff.stat, BuffableStat::SkillDmgBonus);
        assert!((dmg_buff.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            dmg_buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn azurelight_has_hp_na_dmg_both() {
        let passive = AZURELIGHT.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "azurelight_hp_na_dmg");
        assert_eq!(buff.stat, BuffableStat::NormalAtkDmgBonus);
        assert!((buff.value - 0.0016).abs() < 1e-7);
        assert!(matches!(
            buff.activation,
            Activation::Both(
                AutoCondition::StatScaling {
                    stat: BuffableStat::HpPercent,
                    ..
                },
                ManualCondition::Toggle
            )
        ));
    }

    #[test]
    fn lightbearing_moonshard_has_atk_and_dmg_toggle() {
        let passive = LIGHTBEARING_MOONSHARD.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "lightbearing_atk");
        assert_eq!(cond_buffs[0].stat, BuffableStat::AtkPercent);
        assert!((cond_buffs[0].value - 0.24).abs() < 1e-6);

        assert_eq!(cond_buffs[1].name, "lightbearing_dmg");
        assert_eq!(cond_buffs[1].stat, BuffableStat::DmgBonus);
        assert!((cond_buffs[1].value - 0.20).abs() < 1e-6);

        for buff in cond_buffs {
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Toggle)
            ));
        }
    }

    #[test]
    fn summit_shaper_has_atk_stacks_and_shield_stacks() {
        let passive = SUMMIT_SHAPER.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "summit_shaper_atk_stacks");
        assert_eq!(cond_buffs[1].name, "summit_shaper_shield_atk_stacks");

        for buff in cond_buffs {
            assert_eq!(buff.stat, BuffableStat::AtkPercent);
            assert!((buff.value - 0.04).abs() < 1e-6);
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Stacks(5))
            ));
            assert!(buff.refinement_values.is_some());
        }
    }

    #[test]
    fn cinnabar_spindle_has_def_scaling_skill_flat() {
        let passive = CINNABAR_SPINDLE.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "cinnabar_skill_def_flat");
        assert_eq!(buff.stat, BuffableStat::SkillFlatDmg);
        assert!((buff.value - 0.40).abs() < 1e-6);
        assert!(buff.refinement_values.is_some());
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.40).abs() < 1e-6);
        assert!((rv[4] - 0.80).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::DefPercent,
                offset: None,
                cap: None,
            })
        ));
    }

    #[test]
    fn flute_of_ezpitzal_has_def_scaling_normal_flat() {
        let passive = FLUTE_OF_EZPITZAL.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "flute_ezpitzal_def_flat");
        assert_eq!(buff.stat, BuffableStat::NormalAtkFlatDmg);
        assert!((buff.value - 0.24).abs() < 1e-6);
        assert!(buff.refinement_values.is_some());
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.24).abs() < 1e-6);
        assert!((rv[4] - 0.48).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Both(
                AutoCondition::StatScaling {
                    stat: BuffableStat::DefPercent,
                    offset: None,
                    cap: None,
                },
                ManualCondition::Toggle
            )
        ));
    }

    #[test]
    fn festering_desire_has_skill_cr_toggle() {
        let passive = FESTERING_DESIRE.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "festering_skill_cr");
        assert_eq!(buff.stat, BuffableStat::CritRate);
        assert!((buff.value - 0.06).abs() < 1e-6);
        assert!(buff.refinement_values.is_some());
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.06).abs() < 1e-6);
        assert!((rv[4] - 0.12).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        // Also verify the existing StatBuff is still present
        let buffs = passive.effect.buffs;
        assert_eq!(buffs.len(), 1);
        assert_eq!(buffs[0].stat, BuffableStat::SkillDmgBonus);
    }

    #[test]
    fn lions_roar_has_dmg_toggle() {
        let passive = LIONS_ROAR.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "lions_roar_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.20).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn prized_isshin_blade_has_atk_toggle() {
        let passive = PRIZED_ISSHIN_BLADE.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "prized_isshin_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.12).abs() < 1e-6);
        assert!(buff.refinement_values.is_some());
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.12).abs() < 1e-6);
        assert!((rv[4] - 0.24).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn royal_longsword_has_crit_rate_toggle() {
        let passive = ROYAL_LONGSWORD.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "royal_longsword_cr");
        assert_eq!(buff.stat, BuffableStat::CritRate);
        assert!((buff.value - 0.40).abs() < 1e-6);
        assert!(buff.refinement_values.is_some());
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.40).abs() < 1e-6);
        assert!((rv[4] - 0.80).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn serenitys_call_has_dmg_toggle() {
        let passive = SERENITYS_CALL.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "serenitys_call_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.08).abs() < 1e-6);
        assert!(buff.refinement_values.is_some());
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.08).abs() < 1e-6);
        assert!((rv[4] - 0.16).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn sturdy_bone_has_na_dmg_toggle() {
        let passive = STURDY_BONE.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "sturdy_bone_na_dmg");
        assert_eq!(buff.stat, BuffableStat::NormalAtkDmgBonus);
        assert!((buff.value - 0.16).abs() < 1e-6);
        assert!(buff.refinement_values.is_some());
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.16).abs() < 1e-6);
        assert!((rv[4] - 0.32).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn toukabou_shigure_has_dmg_toggle() {
        let passive = TOUKABOU_SHIGURE.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "toukabou_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.16).abs() < 1e-6);
        assert!(buff.refinement_values.is_some());
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.16).abs() < 1e-6);
        assert!((rv[4] - 0.32).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn mistsplitter_has_emblem_stacks() {
        let passive = MISTSPLITTER_REFORGED.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "mistsplitter_emblem");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        // Non-linear stack values
        let sv = buff.stack_values.unwrap();
        assert_eq!(sv.len(), 3);
        assert!((sv[0] - 0.08).abs() < 1e-6);
        assert!((sv[1] - 0.16).abs() < 1e-6);
        assert!((sv[2] - 0.28).abs() < 1e-6);
    }

    // ===== Phase 3 tests for 21 sword weapons =====

    #[test]
    fn cool_steel_has_conditional_dmg() {
        let passive = COOL_STEEL.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "cool_steel_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.12).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.12).abs() < 1e-6);
        assert!((rv[4] - 0.24).abs() < 1e-6);
    }

    #[test]
    fn dark_iron_sword_has_atk_toggle() {
        let passive = DARK_IRON_SWORD.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "dark_iron_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.20).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.20).abs() < 1e-6);
        assert!((rv[4] - 0.40).abs() < 1e-6);
    }

    #[test]
    fn harbinger_of_dawn_has_crit_rate_toggle() {
        let passive = HARBINGER_OF_DAWN.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "harbinger_crit_rate");
        assert_eq!(buff.stat, BuffableStat::CritRate);
        assert!((buff.value - 0.14).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.14).abs() < 1e-6);
        assert!((rv[4] - 0.28).abs() < 1e-6);
    }

    #[test]
    fn skyrider_sword_has_atk_toggle() {
        let passive = SKYRIDER_SWORD.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "skyrider_sword_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.12).abs() < 1e-6);
        assert!((rv[4] - 0.24).abs() < 1e-6);
    }

    #[test]
    fn kagotsurube_isshin_has_atk_toggle_no_refinement() {
        let passive = KAGOTSURUBE_ISSHIN.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "kagotsurube_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.15).abs() < 1e-6);
        assert!(buff.refinement_values.is_none());
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn sapwood_blade_has_em_toggle() {
        let passive = SAPWOOD_BLADE.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "sapwood_blade_em");
        assert_eq!(buff.stat, BuffableStat::ElementalMastery);
        assert!((buff.value - 60.0).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 60.0).abs() < 1e-6);
        assert!((rv[4] - 120.0).abs() < 1e-6);
    }

    #[test]
    fn alley_flash_moved_to_conditional_dmg() {
        let passive = THE_ALLEY_FLASH.passive.unwrap();
        // StatBuff must be empty (moved to conditional)
        assert_eq!(passive.effect.buffs.len(), 0);
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "alley_flash_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.12).abs() < 1e-6);
        assert!((rv[4] - 0.24).abs() < 1e-6);
    }

    #[test]
    fn sword_of_narzissenkreuz_has_skill_and_burst_dmg() {
        let passive = SWORD_OF_NARZISSENKREUZ.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);

        let skill = &cond[0];
        assert_eq!(skill.name, "narzissenkreuz_skill_dmg");
        assert_eq!(skill.stat, BuffableStat::SkillDmgBonus);
        assert!((skill.value - 0.32).abs() < 1e-6);

        let burst = &cond[1];
        assert_eq!(burst.name, "narzissenkreuz_burst_dmg");
        assert_eq!(burst.stat, BuffableStat::BurstDmgBonus);
        assert!((burst.value - 0.32).abs() < 1e-6);

        for buff in cond {
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Toggle)
            ));
            let rv = buff.refinement_values.unwrap();
            assert!((rv[0] - 0.32).abs() < 1e-6);
            assert!((rv[4] - 0.64).abs() < 1e-6);
        }
    }

    #[test]
    fn finale_of_the_deep_has_na_ca_plunge_toggle() {
        let passive = FINALE_OF_THE_DEEP.passive.unwrap();
        // ATK% StatBuff still present
        assert_eq!(passive.effect.buffs.len(), 1);
        assert_eq!(passive.effect.buffs[0].stat, BuffableStat::AtkPercent);

        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 3);
        assert_eq!(cond[0].stat, BuffableStat::NormalAtkDmgBonus);
        assert_eq!(cond[1].stat, BuffableStat::ChargedAtkDmgBonus);
        assert_eq!(cond[2].stat, BuffableStat::PlungingAtkDmgBonus);
        for buff in cond {
            assert!((buff.value - 0.12).abs() < 1e-6);
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Toggle)
            ));
        }
    }

    #[test]
    fn moonweavers_dawn_has_dmg_toggle() {
        let passive = MOONWEAVERS_DAWN.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "moonweavers_dawn_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.16).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.16).abs() < 1e-6);
        assert!((rv[4] - 0.32).abs() < 1e-6);
    }

    #[test]
    fn calamity_of_eshu_has_skill_dmg_and_crit_rate() {
        let passive = CALAMITY_OF_ESHU.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);

        let skill = &cond[0];
        assert_eq!(skill.name, "calamity_eshu_skill_dmg");
        assert_eq!(skill.stat, BuffableStat::SkillDmgBonus);
        assert!((skill.value - 0.20).abs() < 1e-6);

        let cr = &cond[1];
        assert_eq!(cr.name, "calamity_eshu_crit_rate");
        assert_eq!(cr.stat, BuffableStat::CritRate);
        assert!((cr.value - 0.06).abs() < 1e-6);

        for buff in cond {
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Toggle)
            ));
        }
    }

    #[test]
    fn fleuve_cendre_ferryman_has_cr_and_er_toggle() {
        let passive = FLEUVE_CENDRE_FERRYMAN.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);

        let cr = &cond[0];
        assert_eq!(cr.name, "fleuve_skill_cr");
        assert_eq!(cr.stat, BuffableStat::CritRate);
        assert!((cr.value - 0.08).abs() < 1e-6);
        let rv = cr.refinement_values.unwrap();
        assert!((rv[0] - 0.08).abs() < 1e-6);
        assert!((rv[4] - 0.16).abs() < 1e-6);

        let er = &cond[1];
        assert_eq!(er.name, "fleuve_energy_recharge");
        assert_eq!(er.stat, BuffableStat::EnergyRecharge);
        assert!((er.value - 0.16).abs() < 1e-6);

        for buff in cond {
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Toggle)
            ));
        }
    }

    #[test]
    fn blackcliff_longsword_has_atk_stacks() {
        let passive = BLACKCLIFF_LONGSWORD.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "blackcliff_longsword_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.12).abs() < 1e-6);
        assert!((rv[4] - 0.24).abs() < 1e-6);
    }

    #[test]
    fn prototype_rancour_has_atk_and_def_stacks() {
        let passive = PROTOTYPE_RANCOUR.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);

        let atk = &cond[0];
        assert_eq!(atk.name, "prototype_rancour_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert!((atk.value - 0.04).abs() < 1e-6);

        let def = &cond[1];
        assert_eq!(def.name, "prototype_rancour_def");
        assert_eq!(def.stat, BuffableStat::DefPercent);
        assert!((def.value - 0.04).abs() < 1e-6);

        for buff in cond {
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Stacks(4))
            ));
        }
    }

    #[test]
    fn dockhands_assistant_has_em_stacks() {
        let passive = THE_DOCKHANDS_ASSISTANT.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "dockhands_em");
        assert_eq!(buff.stat, BuffableStat::ElementalMastery);
        assert!((buff.value - 40.0).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 40.0).abs() < 1e-6);
        assert!((rv[4] - 80.0).abs() < 1e-6);
    }

    #[test]
    fn wolf_fang_has_def_reduction_stacks() {
        let passive = WOLF_FANG.passive.unwrap();
        // Skill/Burst StatBuffs still present
        assert_eq!(passive.effect.buffs.len(), 2);

        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "wolf_fang_def_reduction");
        assert_eq!(buff.stat, BuffableStat::DefReduction);
        assert!((buff.value - 0.04).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(4))
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.04).abs() < 1e-6);
        assert!((rv[4] - 0.08).abs() < 1e-6);
    }

    #[test]
    fn absolution_has_crit_dmg_stacks() {
        let passive = ABSOLUTION.passive.unwrap();
        // CritDmg StatBuff still present
        assert_eq!(passive.effect.buffs.len(), 1);
        assert_eq!(passive.effect.buffs[0].stat, BuffableStat::CritDmg);

        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "absolution_crit_dmg");
        assert_eq!(buff.stat, BuffableStat::CritDmg);
        assert!((buff.value - 0.16).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.16).abs() < 1e-6);
        assert!((rv[4] - 0.32).abs() < 1e-6);
    }

    #[test]
    fn freedom_sworn_has_team_buffs() {
        let passive = FREEDOM_SWORN.passive.unwrap();
        // DmgBonus StatBuff still present
        assert_eq!(passive.effect.buffs.len(), 1);

        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 4);

        let atk = &cond[0];
        assert_eq!(atk.name, "freedom_sworn_team_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert_eq!(atk.target, BuffTarget::Team);
        assert!((atk.value - 0.20).abs() < 1e-6);
        let rv = atk.refinement_values.unwrap();
        assert!((rv[0] - 0.20).abs() < 1e-6);
        assert!((rv[4] - 0.40).abs() < 1e-6);

        assert_eq!(cond[1].stat, BuffableStat::NormalAtkDmgBonus);
        assert_eq!(cond[1].target, BuffTarget::Team);
        assert_eq!(cond[2].stat, BuffableStat::ChargedAtkDmgBonus);
        assert_eq!(cond[2].target, BuffTarget::Team);
        assert_eq!(cond[3].stat, BuffableStat::PlungingAtkDmgBonus);
        assert_eq!(cond[3].target, BuffTarget::Team);

        for buff in &cond[1..] {
            assert!((buff.value - 0.16).abs() < 1e-6);
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Toggle)
            ));
        }
    }

    #[test]
    fn haran_geppaku_futsu_has_na_dmg_stacks() {
        let passive = HARAN_GEPPAKU_FUTSU.passive.unwrap();
        // DmgBonus StatBuff still present
        assert_eq!(passive.effect.buffs.len(), 1);

        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "haran_na_dmg");
        assert_eq!(buff.stat, BuffableStat::NormalAtkDmgBonus);
        assert!((buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(2))
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.12).abs() < 1e-6);
        assert!((rv[4] - 0.24).abs() < 1e-6);
    }

    #[test]
    fn splendor_of_tranquil_waters_has_na_and_skill_stacks() {
        let passive = SPLENDOR_OF_TRANQUIL_WATERS.passive.unwrap();
        // SkillDmgBonus StatBuff still present
        assert_eq!(passive.effect.buffs.len(), 1);

        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);

        let na = &cond[0];
        assert_eq!(na.name, "splendor_na_dmg");
        assert_eq!(na.stat, BuffableStat::NormalAtkDmgBonus);
        assert!((na.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            na.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));

        let skill = &cond[1];
        assert_eq!(skill.name, "splendor_skill_dmg");
        assert_eq!(skill.stat, BuffableStat::SkillDmgBonus);
        assert!((skill.value - 0.06).abs() < 1e-6);
        assert!(matches!(
            skill.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
    }

    #[test]
    fn xiphos_moonlight_has_em_scaling_er_self_and_team() {
        let passive = XIPHOS_MOONLIGHT.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);

        let self_buff = &cond[0];
        assert_eq!(self_buff.name, "xiphos_self_er");
        assert_eq!(self_buff.stat, BuffableStat::EnergyRecharge);
        assert!((self_buff.value - 0.00036).abs() < 1e-9);
        assert_eq!(self_buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            self_buff.activation,
            Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::ElementalMastery,
                offset: None,
                cap: None,
            })
        ));
        let rv = self_buff.refinement_values.unwrap();
        assert!((rv[0] - 0.00036).abs() < 1e-9);
        assert!((rv[4] - 0.00072).abs() < 1e-9);

        let team_buff = &cond[1];
        assert_eq!(team_buff.name, "xiphos_team_er");
        assert_eq!(team_buff.stat, BuffableStat::EnergyRecharge);
        assert_eq!(team_buff.target, BuffTarget::TeamExcludeSelf);
        assert!((team_buff.value - 0.000108).abs() < 1e-10);
        assert!(matches!(
            team_buff.activation,
            Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::ElementalMastery,
                offset: None,
                cap: None,
            })
        ));
    }
}
