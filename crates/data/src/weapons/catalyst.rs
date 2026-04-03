use crate::buff::{
    Activation, AutoCondition, BuffTarget, BuffableStat, ConditionalBuff, ManualCondition,
    PassiveEffect, StatBuff,
};
use crate::types::{Rarity, WeaponData, WeaponPassive, WeaponSubStat, WeaponType};

// =============================================================================
// 5-Star Catalysts
// =============================================================================

pub const A_THOUSAND_FLOATING_DREAMS: WeaponData = WeaponData {
    id: "a_thousand_floating_dreams",
    name: "A Thousand Floating Dreams",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([58.0, 241.0, 241.0, 265.0])),
    passive: Some(WeaponPassive {
        name: "A Thousand Floating Dreams",
        effect: PassiveEffect {
            description: "同元素1人毎にEM+32-64、異元素1人毎にDMG+10-26%。チームにEM+40-56",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "thousand_dreams_same1_em",
                    description: "1+同元素チームメンバーでEM+32-64",
                    stat: BuffableStat::ElementalMastery,
                    value: 32.0,
                    refinement_values: Some([32.0, 40.0, 48.0, 56.0, 64.0]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamSameElementCount {
                        min_count: 1,
                    }),
                },
                ConditionalBuff {
                    name: "thousand_dreams_same2_em",
                    description: "2+同元素チームメンバーでEM+32-64",
                    stat: BuffableStat::ElementalMastery,
                    value: 32.0,
                    refinement_values: Some([32.0, 40.0, 48.0, 56.0, 64.0]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamSameElementCount {
                        min_count: 2,
                    }),
                },
                ConditionalBuff {
                    name: "thousand_dreams_same3_em",
                    description: "3+同元素チームメンバーでEM+32-64",
                    stat: BuffableStat::ElementalMastery,
                    value: 32.0,
                    refinement_values: Some([32.0, 40.0, 48.0, 56.0, 64.0]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamSameElementCount {
                        min_count: 3,
                    }),
                },
                ConditionalBuff {
                    name: "thousand_dreams_diff1_dmg",
                    description: "1+異元素チームメンバーでDMG+10-26%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.10,
                    refinement_values: Some([0.10, 0.14, 0.18, 0.22, 0.26]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount {
                        min_count: 1,
                    }),
                },
                ConditionalBuff {
                    name: "thousand_dreams_diff2_dmg",
                    description: "2+異元素チームメンバーでDMG+10-26%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.10,
                    refinement_values: Some([0.10, 0.14, 0.18, 0.22, 0.26]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount {
                        min_count: 2,
                    }),
                },
                ConditionalBuff {
                    name: "thousand_dreams_diff3_dmg",
                    description: "3+異元素チームメンバーでDMG+10-26%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.10,
                    refinement_values: Some([0.10, 0.14, 0.18, 0.22, 0.26]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount {
                        min_count: 3,
                    }),
                },
                ConditionalBuff {
                    name: "thousand_dreams_team_em",
                    description: "チームメンバーにEM+40-56",
                    stat: BuffableStat::ElementalMastery,
                    value: 40.0,
                    refinement_values: Some([40.0, 44.0, 48.0, 52.0, 56.0]),
                    stack_values: None,
                    target: BuffTarget::TeamExcludeSelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const CASHFLOW_SUPERVISION: WeaponData = WeaponData {
    id: "cashflow_supervision",
    name: "Cashflow Supervision",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.048, 0.201, 0.201, 0.221])),
    passive: Some(WeaponPassive {
        name: "Cashflow Supervision",
        effect: PassiveEffect {
            description: "ATK+16-32%、NA/CA DMG+16-32%。HP変動後CA DMG追加+16-32%",
            buffs: &[
                StatBuff {
                    stat: BuffableStat::AtkPercent,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                },
                StatBuff {
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                },
                StatBuff {
                    stat: BuffableStat::ChargedAtkDmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                },
            ],
            conditional_buffs: &[ConditionalBuff {
                name: "cashflow_supervision_ca_dmg",
                description: "HP変動（回復/被ダメ）後にCA DMG追加+16-32%",
                stat: BuffableStat::ChargedAtkDmgBonus,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const CRANES_ECHOING_CALL: WeaponData = WeaponData {
    id: "cranes_echoing_call",
    name: "Crane's Echoing Call",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [49.0, 648.0, 679.0, 741.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.036, 0.151, 0.151, 0.165])),
    passive: Some(WeaponPassive {
        name: "Crane's Echoing Call",
        effect: PassiveEffect {
            description: "Plunging DMG+28-56%。落下攻撃命中後チーム全員の落下攻撃DMG+28-56%",
            buffs: &[StatBuff {
                stat: BuffableStat::PlungingAtkDmgBonus,
                value: 0.28,
                refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "cranes_echoing_call_team_plunge",
                description: "落下攻撃命中後にチーム全員の落下攻撃DMG+28-56%",
                stat: BuffableStat::PlungingAtkDmgBonus,
                value: 0.28,
                refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const EVERLASTING_MOONGLOW: WeaponData = WeaponData {
    id: "everlasting_moonglow",
    name: "Everlasting Moonglow",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.108, 0.453, 0.453, 0.496])),
    passive: Some(WeaponPassive {
        name: "Everlasting Moonglow",
        effect: PassiveEffect {
            description:
                "Heal+10-20%。NA DMG+HP上限の1-2%。元素爆発後12秒間NA DMG+HP上限の0.7-1.4%",
            buffs: &[StatBuff {
                stat: BuffableStat::HealingBonus,
                value: 0.10,
                refinement_values: Some([0.10, 0.125, 0.15, 0.175, 0.20]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "everlasting_moonglow_na_hp",
                    description: "HP上限の1-2%分、通常攻撃に追加ダメージ（常時）",
                    stat: BuffableStat::NormalAtkFlatDmg,
                    value: 0.010,
                    refinement_values: Some([0.010, 0.0125, 0.015, 0.0175, 0.020]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::HpPercent,
                        offset: None,
                        cap: None,
                    }),
                },
                ConditionalBuff {
                    name: "everlasting_moonglow_burst_na_hp",
                    description: "元素爆発後12秒間、HP上限の0.7-1.4%分の通常攻撃追加ダメージ",
                    stat: BuffableStat::NormalAtkFlatDmg,
                    value: 0.007,
                    refinement_values: Some([0.007, 0.00875, 0.0105, 0.01225, 0.014]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Both(
                        AutoCondition::StatScaling {
                            stat: BuffableStat::HpPercent,
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

pub const JADEFALLS_SPLENDOR: WeaponData = WeaponData {
    id: "jadefalls_splendor",
    name: "Jadefall's Splendor",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.108, 0.453, 0.453, 0.496])),
    passive: Some(WeaponPassive {
        name: "Jadefall's Splendor",
        effect: PassiveEffect {
            description: "元素エネルギー消費後にEM+32-64",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "jadefall_em",
                description: "元素エネルギー消費後にEM+32-64",
                stat: BuffableStat::ElementalMastery,
                value: 32.0,
                refinement_values: Some([32.0, 40.0, 48.0, 56.0, 64.0]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const KAGURAS_VERITY: WeaponData = WeaponData {
    id: "kaguras_verity",
    name: "Kagura's Verity",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.144, 0.603, 0.603, 0.662])),
    passive: Some(WeaponPassive {
        name: "Kagura's Verity",
        effect: PassiveEffect {
            description:
                "元素スキル使用でスキルDMG+12-24%スタック（最大3）、3スタックで元素DMG+12-24%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "kagura_skill_dmg_stacks",
                    description: "元素スキル使用でスキルDMG+12-24%（1スタック）、最大3スタック",
                    stat: BuffableStat::SkillDmgBonus,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(3)),
                },
                ConditionalBuff {
                    name: "kagura_full_stack_elemental_dmg",
                    description: "3スタック時に元素DMG+12-24%（DmgBonus近似値、物理除外）",
                    stat: BuffableStat::DmgBonus,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const LOST_PRAYER_TO_THE_SACRED_WINDS: WeaponData = WeaponData {
    id: "lost_prayer_to_the_sacred_winds",
    name: "Lost Prayer to the Sacred Winds",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.072, 0.302, 0.302, 0.331])),
    passive: Some(WeaponPassive {
        name: "Lost Prayer to the Sacred Winds",
        effect: PassiveEffect {
            description: "フィールド上で4秒毎に元素DMG+8-16%スタック（最大4スタック）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "lost_prayer_dmg",
                description: "フィールド上で4秒毎に元素DMG+8-16%（1スタック）、最大4スタック",
                stat: BuffableStat::DmgBonus,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(4)),
            }],
        },
    }),
};

pub const MEMORY_OF_DUST: WeaponData = WeaponData {
    id: "memory_of_dust",
    name: "Memory of Dust",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.108, 0.453, 0.453, 0.496])),
    passive: Some(WeaponPassive {
        name: "Memory of Dust",
        effect: PassiveEffect {
            description: "攻撃命中でATK+4-8%スタック（最大5）、シールド時は2倍",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "memory_of_dust_atk_stacks",
                    description: "攻撃命中でATK+4-8%（1スタック）、最大5スタック",
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
                ConditionalBuff {
                    name: "memory_of_dust_shield_atk_stacks",
                    description: "シールド時にATKスタック効果2倍分（追加ATK+4-8%/スタック）",
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
            ],
        },
    }),
};

pub const NIGHTWEAVERS_LOOKING_GLASS: WeaponData = WeaponData {
    id: "nightweavers_looking_glass",
    name: "Nightweaver's Looking Glass",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([58.0, 241.0, 241.0, 265.0])),
    passive: Some(WeaponPassive {
        name: "Nightweaver's Looking Glass",
        effect: PassiveEffect {
            description: "夜魂消費でNA DMG+16-32%/CA DMG+16-32%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "nightweaver_na_dmg",
                    description: "夜魂消費でNA DMG+16-32%",
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "nightweaver_ca_dmg",
                    description: "夜魂消費でCA DMG+16-32%",
                    stat: BuffableStat::ChargedAtkDmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const NOCTURNES_CURTAIN_CALL: WeaponData = WeaponData {
    id: "nocturnes_curtain_call",
    name: "Nocturne's Curtain Call",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.192, 0.804, 0.804, 0.882])),
    passive: Some(WeaponPassive {
        name: "Nocturne's Curtain Call",
        effect: PassiveEffect {
            description: "元素スキル/爆発命中でDMG+8-16%スタック（最大5スタック）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "nocturne_dmg_stacks",
                description: "元素スキル/爆発命中でDMG+8-16%（1スタック）、最大5スタック",
                stat: BuffableStat::DmgBonus,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(5)),
            }],
        },
    }),
};

pub const RELIQUARY_OF_TRUTH: WeaponData = WeaponData {
    id: "reliquary_of_truth",
    name: "Reliquary of Truth",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.192, 0.804, 0.804, 0.882])),
    passive: Some(WeaponPassive {
        name: "Reliquary of Truth",
        effect: PassiveEffect {
            description: "元素反応後にDMG+12-36%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "reliquary_truth_dmg",
                description: "元素反応後にDMG+12-36%",
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.18, 0.24, 0.30, 0.36]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const SKYWARD_ATLAS: WeaponData = WeaponData {
    id: "skyward_atlas",
    name: "Skyward Atlas",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.072, 0.302, 0.302, 0.331])),
    passive: Some(WeaponPassive {
        name: "Skyward Atlas",
        effect: PassiveEffect {
            description: "Ele DMG+12-24%。通常攻撃命中時に追加攻撃",
            buffs: &[StatBuff {
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[],
        },
    }),
};

pub const STARCALLERS_WATCH: WeaponData = WeaponData {
    id: "starcallers_watch",
    name: "Starcaller's Watch",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([58.0, 241.0, 241.0, 265.0])),
    passive: Some(WeaponPassive {
        name: "Starcaller's Watch",
        effect: PassiveEffect {
            description: "チームの元素反応でDMG+20-40%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "starcaller_dmg",
                description: "チームが元素反応を起こした後にDMG+20-40%",
                stat: BuffableStat::DmgBonus,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const SUNNY_MORNING_SLEEP_IN: WeaponData = WeaponData {
    id: "sunny_morning_sleep_in",
    name: "Sunny Morning Sleep-In",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([58.0, 241.0, 241.0, 265.0])),
    passive: Some(WeaponPassive {
        name: "Sunny Morning Sleep-In",
        effect: PassiveEffect {
            description: "元素反応時にATK+14-28%/DMG+18-36%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "sunny_morning_atk",
                    description: "元素反応時にATK+14-28%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.14,
                    refinement_values: Some([0.14, 0.175, 0.21, 0.245, 0.28]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "sunny_morning_dmg",
                    description: "元素反応時にDMG+18-36%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.18,
                    refinement_values: Some([0.18, 0.225, 0.27, 0.315, 0.36]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const SURFS_UP: WeaponData = WeaponData {
    id: "surfs_up",
    name: "Surf's Up",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.192, 0.804, 0.804, 0.882])),
    passive: Some(WeaponPassive {
        name: "Surf's Up",
        effect: PassiveEffect {
            description: "HP+20-40%。通常攻撃DMGスタック効果",
            buffs: &[StatBuff {
                stat: BuffableStat::HpPercent,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "surfs_up_na_stacks",
                description: "夜魂バースト時にNA DMG+10-20%（1スタック/1000HP基準）、最大5スタック",
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.10,
                refinement_values: Some([0.10, 0.125, 0.15, 0.175, 0.20]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(5)),
            }],
        },
    }),
};

pub const TOME_OF_THE_ETERNAL_FLOW: WeaponData = WeaponData {
    id: "tome_of_the_eternal_flow",
    name: "Tome of the Eternal Flow",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.192, 0.804, 0.804, 0.882])),
    passive: Some(WeaponPassive {
        name: "Tome of the Eternal Flow",
        effect: PassiveEffect {
            description: "HP+16-32%。HP変動時にCA DMGスタック",
            buffs: &[StatBuff {
                stat: BuffableStat::HpPercent,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "tome_eternal_flow_ca",
                description: "HP変動時にCA DMG+14-28%スタック、最大3スタック",
                stat: BuffableStat::ChargedAtkDmgBonus,
                value: 0.14,
                refinement_values: Some([0.14, 0.175, 0.21, 0.245, 0.28]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
        },
    }),
};

pub const TULAYTULLAHS_REMEMBRANCE: WeaponData = WeaponData {
    id: "tulaytullahs_remembrance",
    name: "Tulaytullah's Remembrance",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.096, 0.402, 0.402, 0.441])),
    passive: Some(WeaponPassive {
        name: "Tulaytullah's Remembrance",
        effect: PassiveEffect {
            description:
                "通常攻撃DMG+4.8-9.6%スタック（最大10スタック、1秒毎）。攻撃速度バフは非対応",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "tulaytullah_na_dmg_stacks",
                description: "通常攻撃DMG+4.8-9.6%（1スタック）、最大10スタック",
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.048,
                refinement_values: Some([0.048, 0.06, 0.072, 0.084, 0.096]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(10)),
            }],
        },
    }),
};

pub const VIVID_NOTIONS: WeaponData = WeaponData {
    id: "vivid_notions",
    name: "Vivid Notions",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.096, 0.402, 0.402, 0.441])),
    passive: Some(WeaponPassive {
        name: "Vivid Notions",
        effect: PassiveEffect {
            description: "夜魂バースト時にDMG+8-16%スタック（最大3スタック）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "vivid_notions_dmg_stacks",
                description: "夜魂バースト時にDMG+8-16%（1スタック）、最大3スタック",
                stat: BuffableStat::DmgBonus,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
        },
    }),
};

// =============================================================================
// 4-Star Catalysts
// =============================================================================

pub const ASH_GRAVEN_DRINKING_HORN: WeaponData = WeaponData {
    id: "ash_graven_drinking_horn",
    name: "Ash-Graven Drinking Horn",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "Ash-Graven Drinking Horn",
        effect: PassiveEffect {
            description: "夜魂バースト中に全攻撃にフラットDMG+HP上限の2-4%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "ash_graven_na_hp",
                    description: "夜魂バースト中に通常攻撃フラットDMG+HP上限の2-4%",
                    stat: BuffableStat::NormalAtkFlatDmg,
                    value: 0.02,
                    refinement_values: Some([0.02, 0.025, 0.03, 0.035, 0.04]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Both(
                        AutoCondition::StatScaling {
                            stat: BuffableStat::HpPercent,
                            offset: None,
                            cap: None,
                        },
                        ManualCondition::Toggle,
                    ),
                },
                ConditionalBuff {
                    name: "ash_graven_ca_hp",
                    description: "夜魂バースト中に重撃フラットDMG+HP上限の2-4%",
                    stat: BuffableStat::ChargedAtkFlatDmg,
                    value: 0.02,
                    refinement_values: Some([0.02, 0.025, 0.03, 0.035, 0.04]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Both(
                        AutoCondition::StatScaling {
                            stat: BuffableStat::HpPercent,
                            offset: None,
                            cap: None,
                        },
                        ManualCondition::Toggle,
                    ),
                },
                ConditionalBuff {
                    name: "ash_graven_plunge_hp",
                    description: "夜魂バースト中に落下攻撃フラットDMG+HP上限の2-4%",
                    stat: BuffableStat::PlungingAtkFlatDmg,
                    value: 0.02,
                    refinement_values: Some([0.02, 0.025, 0.03, 0.035, 0.04]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Both(
                        AutoCondition::StatScaling {
                            stat: BuffableStat::HpPercent,
                            offset: None,
                            cap: None,
                        },
                        ManualCondition::Toggle,
                    ),
                },
                ConditionalBuff {
                    name: "ash_graven_skill_hp",
                    description: "夜魂バースト中に元素スキルフラットDMG+HP上限の2-4%",
                    stat: BuffableStat::SkillFlatDmg,
                    value: 0.02,
                    refinement_values: Some([0.02, 0.025, 0.03, 0.035, 0.04]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Both(
                        AutoCondition::StatScaling {
                            stat: BuffableStat::HpPercent,
                            offset: None,
                            cap: None,
                        },
                        ManualCondition::Toggle,
                    ),
                },
                ConditionalBuff {
                    name: "ash_graven_burst_hp",
                    description: "夜魂バースト中に元素爆発フラットDMG+HP上限の2-4%",
                    stat: BuffableStat::BurstFlatDmg,
                    value: 0.02,
                    refinement_values: Some([0.02, 0.025, 0.03, 0.035, 0.04]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Both(
                        AutoCondition::StatScaling {
                            stat: BuffableStat::HpPercent,
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

pub const BALLAD_OF_THE_BOUNDLESS_BLUE: WeaponData = WeaponData {
    id: "ballad_of_the_boundless_blue",
    name: "Ballad of the Boundless Blue",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "Ballad of the Boundless Blue",
        effect: PassiveEffect {
            description: "NA/CA DMG+8-16%",
            buffs: &[
                StatBuff {
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.08,
                    refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                },
                StatBuff {
                    stat: BuffableStat::ChargedAtkDmgBonus,
                    value: 0.08,
                    refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                },
            ],
            conditional_buffs: &[],
        },
    }),
};

pub const BLACKCLIFF_AGATE: WeaponData = WeaponData {
    id: "blackcliff_agate",
    name: "Blackcliff Agate",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "Blackcliff Agate",
        effect: PassiveEffect {
            description: "Conditional: 敵撃破時にATK%スタック",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "blackcliff_agate_atk",
                description: "敵撃破後にATK+12-24%、最大3スタック",
                stat: BuffableStat::AtkPercent,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
        },
    }),
};

pub const BLACKMARROW_LANTERN: WeaponData = WeaponData {
    id: "blackmarrow_lantern",
    name: "Blackmarrow Lantern",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([48.0, 201.0, 201.0, 221.0])),
    passive: Some(WeaponPassive {
        name: "Blackmarrow Lantern",
        effect: PassiveEffect {
            description: "Conditional: 夜魂ポイント消費でDMGアップ",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "blackmarrow_dmg",
                description: "夜魂ポイント消費時にDMG+12-24%",
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const DAWNING_FROST: WeaponData = WeaponData {
    id: "dawning_frost",
    name: "Dawning Frost",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "Dawning Frost",
        effect: PassiveEffect {
            description: "Conditional: 元素スキル命中でDMGアップ",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "dawning_frost_dmg",
                description: "元素スキル命中後にDMG+16-32%",
                stat: BuffableStat::DmgBonus,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const DODOCO_TALES: WeaponData = WeaponData {
    id: "dodoco_tales",
    name: "Dodoco Tales",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "Dodoco Tales",
        effect: PassiveEffect {
            description: "Conditional: 通常攻撃命中でCA DMGアップ、CA命中でATKアップ",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "dodoco_ca_dmg",
                    description: "通常攻撃命中時にCA DMG+16-32%",
                    stat: BuffableStat::ChargedAtkDmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "dodoco_atk",
                    description: "重撃命中時にATK+8-16%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.08,
                    refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const ETHERLIGHT_SPINDLELUTE: WeaponData = WeaponData {
    id: "etherlight_spindlelute",
    name: "Etherlight Spindlelute",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.100, 0.419, 0.419, 0.459])),
    passive: Some(WeaponPassive {
        name: "Etherlight Spindlelute",
        effect: PassiveEffect {
            description: "Conditional: 元素爆発後にDMGアップ",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "etherlight_dmg",
                description: "元素爆発命中後にDMG+16-32%",
                stat: BuffableStat::DmgBonus,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const EYE_OF_PERCEPTION: WeaponData = WeaponData {
    id: "eye_of_perception",
    name: "Eye of Perception",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "Eye of Perception",
        effect: PassiveEffect {
            description: "Conditional: 通常/重撃命中時に追加ダメージ弾発射",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const FAVONIUS_CODEX: WeaponData = WeaponData {
    id: "favonius_codex",
    name: "Favonius Codex",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.100, 0.419, 0.419, 0.459])),
    passive: Some(WeaponPassive {
        name: "Favonius Codex",
        effect: PassiveEffect {
            description: "Conditional: 会心命中時に元素粒子生成",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const FLOWING_PURITY: WeaponData = WeaponData {
    id: "flowing_purity",
    name: "Flowing Purity",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Flowing Purity",
        effect: PassiveEffect {
            description: "Conditional: HP消費時にDMGアップ、治療効果でバフ延長",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "flowing_purity_dmg",
                description: "HP増減時にDMG+8-16%",
                stat: BuffableStat::DmgBonus,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const FROSTBEARER: WeaponData = WeaponData {
    id: "frostbearer",
    name: "Frostbearer",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "Frostbearer",
        effect: PassiveEffect {
            description: "Conditional: 通常/重撃命中時に氷柱ダメージ、氷/凍結時にダメージ増加",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const FRUIT_OF_FULFILLMENT: WeaponData = WeaponData {
    id: "fruit_of_fulfillment",
    name: "Fruit of Fulfillment",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.100, 0.419, 0.419, 0.459])),
    passive: Some(WeaponPassive {
        name: "Fruit of Fulfillment",
        effect: PassiveEffect {
            description: "Conditional: 元素反応時にEM獲得/ATK減少スタック",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "fruit_of_fulfillment_em",
                description: "元素反応発動時にEM+24-48スタック（最大5スタック）。ATK減少は非対応",
                stat: BuffableStat::ElementalMastery,
                value: 24.0,
                refinement_values: Some([24.0, 30.0, 36.0, 42.0, 48.0]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(5)),
            }],
        },
    }),
};

pub const HAKUSHIN_RING: WeaponData = WeaponData {
    id: "hakushin_ring",
    name: "Hakushin Ring",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "Hakushin Ring",
        effect: PassiveEffect {
            description: "Conditional: 雷元素反応時に関連元素のDMGアップ",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "hakushin_ring_elem_dmg",
                description: "雷元素反応発動後、関連元素DMG+10-20%（チームメンバーも対象）",
                stat: BuffableStat::DmgBonus,
                value: 0.10,
                refinement_values: Some([0.10, 0.125, 0.15, 0.175, 0.20]),
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const MAPPA_MARE: WeaponData = WeaponData {
    id: "mappa_mare",
    name: "Mappa Mare",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([24.0, 101.0, 101.0, 110.0])),
    passive: Some(WeaponPassive {
        name: "Mappa Mare",
        effect: PassiveEffect {
            description: "元素反応発動後に元素DMG+8-16%スタック（最大2スタック、10秒）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "mappa_mare_dmg",
                description: "元素反応後に元素DMG+8-16%（1スタック）、最大2スタック",
                stat: BuffableStat::DmgBonus,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(2)),
            }],
        },
    }),
};

pub const OATHSWORN_EYE: WeaponData = WeaponData {
    id: "oathsworn_eye",
    name: "Oathsworn Eye",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Oathsworn Eye",
        effect: PassiveEffect {
            description: "Conditional: 元素スキル使用後にER+24%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "oathsworn_eye_er",
                description: "元素スキル使用後にER+24-48%",
                stat: BuffableStat::EnergyRecharge,
                value: 0.24,
                refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const PROTOTYPE_AMBER: WeaponData = WeaponData {
    id: "prototype_amber",
    name: "Prototype Amber",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "Prototype Amber",
        effect: PassiveEffect {
            description: "Conditional: 元素爆発使用後にHP回復とエネルギー回復",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const RING_OF_YAXCHE: WeaponData = WeaponData {
    id: "ring_of_yaxche",
    name: "Ring of Yaxche",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "Ring of Yaxche",
        effect: PassiveEffect {
            description: "夜魂バースト中にNA DMG+HP上限の2-4%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "ring_of_yaxche_na_hp",
                description: "夜魂バースト中にNA DMG+HP上限の2-4%（フラットダメージ）",
                stat: BuffableStat::NormalAtkFlatDmg,
                value: 0.02,
                refinement_values: Some([0.02, 0.025, 0.03, 0.035, 0.04]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Both(
                    AutoCondition::StatScaling {
                        stat: BuffableStat::HpPercent,
                        offset: None,
                        cap: None,
                    },
                    ManualCondition::Toggle,
                ),
            }],
        },
    }),
};

pub const ROYAL_GRIMOIRE: WeaponData = WeaponData {
    id: "royal_grimoire",
    name: "Royal Grimoire",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Royal Grimoire",
        effect: PassiveEffect {
            description: "Conditional: ダメージ時にCRIT Rate+8%スタック、会心時リセット",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "royal_grimoire_cr",
                description: "ダメージ時にCRIT Rate+8-16%、最大5スタック（会心命中でリセット）",
                stat: BuffableStat::CritRate,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(5)),
            }],
        },
    }),
};

pub const SACRIFICIAL_FRAGMENTS: WeaponData = WeaponData {
    id: "sacrificial_fragments",
    name: "Sacrificial Fragments",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([48.0, 201.0, 201.0, 221.0])),
    passive: Some(WeaponPassive {
        name: "Sacrificial Fragments",
        effect: PassiveEffect {
            description: "Conditional: 元素スキル命中時にCD即リセット",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const SACRIFICIAL_JADE: WeaponData = WeaponData {
    id: "sacrificial_jade",
    name: "Sacrificial Jade",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.080, 0.335, 0.335, 0.368])),
    passive: Some(WeaponPassive {
        name: "Sacrificial Jade",
        effect: PassiveEffect {
            description: "Conditional: フィールドに出た時にHP%/EM獲得",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "sacrificial_jade_hp",
                    description: "フィールドに出た時にHP+32-64%",
                    stat: BuffableStat::HpPercent,
                    value: 0.32,
                    refinement_values: Some([0.32, 0.40, 0.48, 0.56, 0.64]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "sacrificial_jade_em",
                    description: "フィールドに出た時にEM+40-80",
                    stat: BuffableStat::ElementalMastery,
                    value: 40.0,
                    refinement_values: Some([40.0, 50.0, 60.0, 70.0, 80.0]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const SOLAR_PEARL: WeaponData = WeaponData {
    id: "solar_pearl",
    name: "Solar Pearl",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Solar Pearl",
        effect: PassiveEffect {
            description:
                "Conditional: NA命中でSkill/Burst DMGアップ、Skill/Burst命中でNA DMGアップ",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "solar_pearl_skill_dmg",
                    description: "通常攻撃命中後にSkill DMG+20-40%",
                    stat: BuffableStat::SkillDmgBonus,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "solar_pearl_burst_dmg",
                    description: "通常攻撃命中後にBurst DMG+20-40%",
                    stat: BuffableStat::BurstDmgBonus,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "solar_pearl_na_dmg",
                    description: "元素スキル/爆発命中後にNA DMG+20-40%",
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const THE_WIDSITH: WeaponData = WeaponData {
    id: "the_widsith",
    name: "The Widsith",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "The Widsith",
        effect: PassiveEffect {
            description: "Conditional: フィールドに出た時にランダムバフ（ATK%/元素DMG/EM）",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "widsith_atk",
                    description: "登場時にランダム: ATK+60-120%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.60,
                    refinement_values: Some([0.60, 0.75, 0.90, 1.05, 1.20]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "widsith_dmg",
                    description: "登場時にランダム: 全元素DMG+48-96%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.48,
                    refinement_values: Some([0.48, 0.60, 0.72, 0.84, 0.96]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "widsith_em",
                    description: "登場時にランダム: EM+240-480",
                    stat: BuffableStat::ElementalMastery,
                    value: 240.0,
                    refinement_values: Some([240.0, 300.0, 360.0, 420.0, 480.0]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const WANDERING_EVENSTAR: WeaponData = WeaponData {
    id: "wandering_evenstar",
    name: "Wandering Evenstar",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([36.0, 151.0, 151.0, 165.0])),
    passive: Some(WeaponPassive {
        name: "Wandering Evenstar",
        effect: PassiveEffect {
            description: "Conditional: EM基準でATKアップ、チームにも付与",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "wandering_evenstar_atk",
                    description: "スキル使用後: EM×0.24-0.48分をATKフラットに加算",
                    stat: BuffableStat::AtkFlat,
                    value: 0.24,
                    refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::ElementalMastery,
                        offset: None,
                        cap: None,
                    }),
                },
                ConditionalBuff {
                    name: "wandering_evenstar_team_atk",
                    description:
                        "スキル使用後: EM×0.072-0.144分をチームメンバーのATKフラットに加算（30%）",
                    stat: BuffableStat::AtkFlat,
                    value: 0.072,
                    refinement_values: Some([0.072, 0.090, 0.108, 0.126, 0.144]),
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

pub const WAVERIDING_WHIRL: WeaponData = WeaponData {
    id: "waveriding_whirl",
    name: "Waveriding Whirl",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.133, 0.557, 0.557, 0.613])),
    passive: Some(WeaponPassive {
        name: "Waveriding Whirl",
        effect: PassiveEffect {
            description: "Conditional: 元素反応時にEMアップ",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "waveriding_whirl_em",
                description: "元素反応発動時にEM+24-48",
                stat: BuffableStat::ElementalMastery,
                value: 24.0,
                refinement_values: Some([24.0, 30.0, 36.0, 42.0, 48.0]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const WINE_AND_SONG: WeaponData = WeaponData {
    id: "wine_and_song",
    name: "Wine and Song",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "Wine and Song",
        effect: PassiveEffect {
            description: "Conditional: ダッシュ後にATKアップ",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "wine_and_song_atk",
                description: "ダッシュ後にATK+20-40%",
                stat: BuffableStat::AtkPercent,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

// =============================================================================
// 3-Star Catalysts
// =============================================================================

pub const EMERALD_ORB: WeaponData = WeaponData {
    id: "emerald_orb",
    name: "Emerald Orb",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star3,
    base_atk: [40.0, 396.0, 415.0, 448.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([20.0, 85.0, 85.0, 94.0])),
    passive: Some(WeaponPassive {
        name: "Emerald Orb",
        effect: PassiveEffect {
            description: "水元素反応トリガー後10秒間ATK+20-40%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "emerald_orb_atk",
                description: "水元素反応トリガー後10秒間ATK+20-40%",
                stat: BuffableStat::AtkPercent,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const MAGIC_GUIDE: WeaponData = WeaponData {
    id: "magic_guide",
    name: "Magic Guide",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star3,
    base_atk: [38.0, 314.0, 334.0, 354.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([41.0, 171.0, 171.0, 187.0])),
    passive: Some(WeaponPassive {
        name: "Magic Guide",
        effect: PassiveEffect {
            description: "Conditional: 水/雷の影響を受けた敵にDMG+12%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "magic_guide_dmg",
                description: "水/雷の影響を受けた敵へのDMG+12-24%",
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const OTHERWORLDLY_STORY: WeaponData = WeaponData {
    id: "otherworldly_story",
    name: "Otherworldly Story",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star3,
    base_atk: [39.0, 355.0, 375.0, 401.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.085, 0.356, 0.356, 0.390])),
    passive: Some(WeaponPassive {
        name: "Otherworldly Story",
        effect: PassiveEffect {
            description: "Conditional: 元素オーブ/粒子獲得時にHP回復",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const THRILLING_TALES_OF_DRAGON_SLAYERS: WeaponData = WeaponData {
    id: "thrilling_tales_of_dragon_slayers",
    name: "Thrilling Tales of Dragon Slayers",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star3,
    base_atk: [39.0, 355.0, 375.0, 401.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.077, 0.323, 0.323, 0.352])),
    passive: Some(WeaponPassive {
        name: "Thrilling Tales of Dragon Slayers",
        effect: PassiveEffect {
            description: "Conditional: キャラ交代時に次のキャラのATK+24%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "ttds_team_atk",
                description: "キャラ交代時、次の出場キャラATK+24-48%",
                stat: BuffableStat::AtkPercent,
                value: 0.24,
                refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
                stack_values: None,
                target: BuffTarget::TeamExcludeSelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const TWIN_NEPHRITE: WeaponData = WeaponData {
    id: "twin_nephrite",
    name: "Twin Nephrite",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star3,
    base_atk: [40.0, 396.0, 415.0, 448.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.034, 0.142, 0.142, 0.156])),
    passive: Some(WeaponPassive {
        name: "Twin Nephrite",
        effect: PassiveEffect {
            description: "敵撃破後10秒間ATK+12-24%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "twin_nephrite_atk",
                description: "敵撃破後10秒間ATK+12-24%",
                stat: BuffableStat::AtkPercent,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

// =============================================================================
// 1-2 Star Catalysts
// =============================================================================

pub const APPRENTICES_NOTES: WeaponData = WeaponData {
    id: "apprentices_notes",
    name: "Apprentice's Notes",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star1,
    base_atk: [23.0, 185.0, 185.0, 185.0],
    sub_stat: None,
    passive: None,
};

pub const POCKET_GRIMOIRE: WeaponData = WeaponData {
    id: "pocket_grimoire",
    name: "Pocket Grimoire",
    weapon_type: WeaponType::Catalyst,
    rarity: Rarity::Star2,
    base_atk: [33.0, 243.0, 243.0, 243.0],
    sub_stat: None,
    passive: None,
};

/// All catalyst weapon data references.
pub const ALL_CATALYSTS: &[&WeaponData] = &[
    // 5-Star
    &A_THOUSAND_FLOATING_DREAMS,
    &CASHFLOW_SUPERVISION,
    &CRANES_ECHOING_CALL,
    &EVERLASTING_MOONGLOW,
    &JADEFALLS_SPLENDOR,
    &KAGURAS_VERITY,
    &LOST_PRAYER_TO_THE_SACRED_WINDS,
    &MEMORY_OF_DUST,
    &NIGHTWEAVERS_LOOKING_GLASS,
    &NOCTURNES_CURTAIN_CALL,
    &RELIQUARY_OF_TRUTH,
    &SKYWARD_ATLAS,
    &STARCALLERS_WATCH,
    &SUNNY_MORNING_SLEEP_IN,
    &SURFS_UP,
    &TOME_OF_THE_ETERNAL_FLOW,
    &TULAYTULLAHS_REMEMBRANCE,
    &VIVID_NOTIONS,
    // 4-Star
    &ASH_GRAVEN_DRINKING_HORN,
    &BALLAD_OF_THE_BOUNDLESS_BLUE,
    &BLACKCLIFF_AGATE,
    &BLACKMARROW_LANTERN,
    &DAWNING_FROST,
    &DODOCO_TALES,
    &ETHERLIGHT_SPINDLELUTE,
    &EYE_OF_PERCEPTION,
    &FAVONIUS_CODEX,
    &FLOWING_PURITY,
    &FROSTBEARER,
    &FRUIT_OF_FULFILLMENT,
    &HAKUSHIN_RING,
    &MAPPA_MARE,
    &OATHSWORN_EYE,
    &PROTOTYPE_AMBER,
    &RING_OF_YAXCHE,
    &ROYAL_GRIMOIRE,
    &SACRIFICIAL_FRAGMENTS,
    &SACRIFICIAL_JADE,
    &SOLAR_PEARL,
    &THE_WIDSITH,
    &WANDERING_EVENSTAR,
    &WAVERIDING_WHIRL,
    &WINE_AND_SONG,
    // 3-Star
    &EMERALD_ORB,
    &MAGIC_GUIDE,
    &OTHERWORLDLY_STORY,
    &THRILLING_TALES_OF_DRAGON_SLAYERS,
    &TWIN_NEPHRITE,
    // 1-2 Star
    &APPRENTICES_NOTES,
    &POCKET_GRIMOIRE,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buff::AutoCondition;

    #[test]
    fn kagura_has_skill_dmg_stacks_and_full_stack_bonus() {
        let passive = KAGURAS_VERITY.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        let stacks_buff = &cond_buffs[0];
        assert_eq!(stacks_buff.name, "kagura_skill_dmg_stacks");
        assert_eq!(stacks_buff.stat, BuffableStat::SkillDmgBonus);
        assert!((stacks_buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            stacks_buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));

        let full_buff = &cond_buffs[1];
        assert_eq!(full_buff.name, "kagura_full_stack_elemental_dmg");
        assert_eq!(full_buff.stat, BuffableStat::DmgBonus);
        assert!((full_buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            full_buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn tulaytullah_has_na_dmg_stacks() {
        let passive = TULAYTULLAHS_REMEMBRANCE.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "tulaytullah_na_dmg_stacks");
        assert_eq!(buff.stat, BuffableStat::NormalAtkDmgBonus);
        assert!((buff.value - 0.048).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(10))
        ));
        assert!(buff.refinement_values.is_some());
    }

    #[test]
    fn nocturne_has_dmg_stacks() {
        let passive = NOCTURNES_CURTAIN_CALL.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "nocturne_dmg_stacks");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(5))
        ));
        assert!(buff.refinement_values.is_some());
    }

    #[test]
    fn vivid_notions_has_dmg_stacks() {
        let passive = VIVID_NOTIONS.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "vivid_notions_dmg_stacks");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        assert!(buff.refinement_values.is_some());
    }

    #[test]
    fn thousand_floating_dreams_has_team_comp_and_team_buff() {
        let passive = A_THOUSAND_FLOATING_DREAMS.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 7);

        for i in 0..3 {
            let name = format!("thousand_dreams_same{}_em", i + 1);
            let expected_min = (i + 1) as u8;
            assert_eq!(cond_buffs[i].name, name.as_str());
            assert_eq!(cond_buffs[i].stat, BuffableStat::ElementalMastery);
            assert!((cond_buffs[i].value - 32.0).abs() < 1e-6);
            assert!(matches!(
                cond_buffs[i].activation,
                Activation::Auto(AutoCondition::TeamSameElementCount { min_count })
                if min_count == expected_min
            ));
        }

        for i in 3..6 {
            let name = format!("thousand_dreams_diff{}_dmg", i - 2);
            let expected_min = (i - 2) as u8;
            assert_eq!(cond_buffs[i].name, name.as_str());
            assert_eq!(cond_buffs[i].stat, BuffableStat::DmgBonus);
            assert!((cond_buffs[i].value - 0.10).abs() < 1e-6);
            assert!(matches!(
                cond_buffs[i].activation,
                Activation::Auto(AutoCondition::TeamDiffElementCount { min_count })
                if min_count == expected_min
            ));
        }

        assert_eq!(cond_buffs[6].name, "thousand_dreams_team_em");
        assert_eq!(cond_buffs[6].stat, BuffableStat::ElementalMastery);
        assert!((cond_buffs[6].value - 40.0).abs() < 1e-6);
        assert_eq!(cond_buffs[6].target, BuffTarget::TeamExcludeSelf);
    }

    #[test]
    fn jadefall_has_em_toggle() {
        let passive = JADEFALLS_SPLENDOR.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        assert_eq!(cond_buffs[0].name, "jadefall_em");
        assert_eq!(cond_buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((cond_buffs[0].value - 32.0).abs() < 1e-6);
        assert!(matches!(
            cond_buffs[0].activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn nightweaver_has_na_ca_dmg_toggle() {
        let passive = NIGHTWEAVERS_LOOKING_GLASS.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);
        assert_eq!(cond_buffs[0].name, "nightweaver_na_dmg");
        assert_eq!(cond_buffs[0].stat, BuffableStat::NormalAtkDmgBonus);
        assert!((cond_buffs[0].value - 0.16).abs() < 1e-6);
        assert_eq!(cond_buffs[1].name, "nightweaver_ca_dmg");
        assert_eq!(cond_buffs[1].stat, BuffableStat::ChargedAtkDmgBonus);
        assert!((cond_buffs[1].value - 0.16).abs() < 1e-6);
        for buff in cond_buffs {
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Toggle)
            ));
        }
    }

    #[test]
    fn reliquary_truth_has_dmg_toggle() {
        let passive = RELIQUARY_OF_TRUTH.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        assert_eq!(cond_buffs[0].name, "reliquary_truth_dmg");
        assert_eq!(cond_buffs[0].stat, BuffableStat::DmgBonus);
        assert!((cond_buffs[0].value - 0.12).abs() < 1e-6);
        assert!(matches!(
            cond_buffs[0].activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn starcaller_has_dmg_toggle() {
        let passive = STARCALLERS_WATCH.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        assert_eq!(cond_buffs[0].name, "starcaller_dmg");
        assert_eq!(cond_buffs[0].stat, BuffableStat::DmgBonus);
        assert!((cond_buffs[0].value - 0.20).abs() < 1e-6);
        assert!(matches!(
            cond_buffs[0].activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn sunny_morning_has_atk_and_dmg_toggle() {
        let passive = SUNNY_MORNING_SLEEP_IN.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);
        assert_eq!(cond_buffs[0].name, "sunny_morning_atk");
        assert_eq!(cond_buffs[0].stat, BuffableStat::AtkPercent);
        assert!((cond_buffs[0].value - 0.14).abs() < 1e-6);
        assert_eq!(cond_buffs[1].name, "sunny_morning_dmg");
        assert_eq!(cond_buffs[1].stat, BuffableStat::DmgBonus);
        assert!((cond_buffs[1].value - 0.18).abs() < 1e-6);
        for buff in cond_buffs {
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Toggle)
            ));
        }
    }

    #[test]
    fn memory_of_dust_has_atk_stacks_and_shield_stacks() {
        let passive = MEMORY_OF_DUST.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "memory_of_dust_atk_stacks");
        assert_eq!(cond_buffs[1].name, "memory_of_dust_shield_atk_stacks");

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
    fn dodoco_tales_has_ca_dmg_and_atk_toggle() {
        let passive = DODOCO_TALES.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);
        assert_eq!(cond_buffs[0].name, "dodoco_ca_dmg");
        assert_eq!(cond_buffs[0].stat, BuffableStat::ChargedAtkDmgBonus);
        assert!((cond_buffs[0].value - 0.16).abs() < 1e-6);
        assert_eq!(cond_buffs[1].name, "dodoco_atk");
        assert_eq!(cond_buffs[1].stat, BuffableStat::AtkPercent);
        assert!((cond_buffs[1].value - 0.08).abs() < 1e-6);
    }

    #[test]
    fn sacrificial_jade_has_hp_and_em_toggle() {
        let passive = SACRIFICIAL_JADE.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);
        assert_eq!(cond_buffs[0].name, "sacrificial_jade_hp");
        assert_eq!(cond_buffs[0].stat, BuffableStat::HpPercent);
        assert!((cond_buffs[0].value - 0.32).abs() < 1e-6);
        let rv = cond_buffs[0].refinement_values.unwrap();
        assert!((rv[4] - 0.64).abs() < 1e-6);
        assert_eq!(cond_buffs[1].name, "sacrificial_jade_em");
        assert_eq!(cond_buffs[1].stat, BuffableStat::ElementalMastery);
        assert!((cond_buffs[1].value - 40.0).abs() < 1e-6);
        let rv2 = cond_buffs[1].refinement_values.unwrap();
        assert!((rv2[4] - 80.0).abs() < 1e-6);
    }

    #[test]
    fn solar_pearl_has_skill_burst_na_toggle() {
        let passive = SOLAR_PEARL.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 3);
        assert_eq!(cond_buffs[0].name, "solar_pearl_skill_dmg");
        assert_eq!(cond_buffs[0].stat, BuffableStat::SkillDmgBonus);
        assert!((cond_buffs[0].value - 0.20).abs() < 1e-6);
        assert_eq!(cond_buffs[1].name, "solar_pearl_burst_dmg");
        assert_eq!(cond_buffs[1].stat, BuffableStat::BurstDmgBonus);
        assert!((cond_buffs[1].value - 0.20).abs() < 1e-6);
        assert_eq!(cond_buffs[2].name, "solar_pearl_na_dmg");
        assert_eq!(cond_buffs[2].stat, BuffableStat::NormalAtkDmgBonus);
        assert!((cond_buffs[2].value - 0.20).abs() < 1e-6);
    }

    #[test]
    fn widsith_has_three_random_toggles() {
        let passive = THE_WIDSITH.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 3);
        assert_eq!(cond_buffs[0].name, "widsith_atk");
        assert_eq!(cond_buffs[0].stat, BuffableStat::AtkPercent);
        assert!((cond_buffs[0].value - 0.60).abs() < 1e-6);
        let rv = cond_buffs[0].refinement_values.unwrap();
        assert!((rv[4] - 1.20).abs() < 1e-6);
        assert_eq!(cond_buffs[1].name, "widsith_dmg");
        assert_eq!(cond_buffs[1].stat, BuffableStat::DmgBonus);
        assert!((cond_buffs[1].value - 0.48).abs() < 1e-6);
        assert_eq!(cond_buffs[2].name, "widsith_em");
        assert_eq!(cond_buffs[2].stat, BuffableStat::ElementalMastery);
        assert!((cond_buffs[2].value - 240.0).abs() < 1e-6);
    }

    #[test]
    fn blackmarrow_lantern_has_dmg_toggle() {
        let passive = BLACKMARROW_LANTERN.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "blackmarrow_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn dawning_frost_has_dmg_toggle() {
        let passive = DAWNING_FROST.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "dawning_frost_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.16).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn etherlight_spindlelute_has_dmg_toggle() {
        let passive = ETHERLIGHT_SPINDLELUTE.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "etherlight_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.16).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn flowing_purity_has_dmg_toggle() {
        let passive = FLOWING_PURITY.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "flowing_purity_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn wine_and_song_has_atk_toggle() {
        let passive = WINE_AND_SONG.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "wine_and_song_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.20).abs() < 1e-6);
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.40).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn cashflow_supervision_has_atk_statbuff_and_ca_conditional() {
        let passive = CASHFLOW_SUPERVISION.passive.unwrap();
        let buffs = passive.effect.buffs;
        assert_eq!(buffs.len(), 3);
        assert_eq!(buffs[0].stat, BuffableStat::AtkPercent);
        assert!((buffs[0].value - 0.16).abs() < 1e-6);
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "cashflow_supervision_ca_dmg");
        assert_eq!(buff.stat, BuffableStat::ChargedAtkDmgBonus);
        assert!((buff.value - 0.16).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.32).abs() < 1e-6);
    }

    #[test]
    fn cranes_echoing_call_has_team_plunge_toggle() {
        let passive = CRANES_ECHOING_CALL.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "cranes_echoing_call_team_plunge");
        assert_eq!(buff.stat, BuffableStat::PlungingAtkDmgBonus);
        assert!((buff.value - 0.28).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::Team);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.56).abs() < 1e-6);
    }

    #[test]
    fn everlasting_moonglow_has_hp_scaling_na_dmg() {
        let passive = EVERLASTING_MOONGLOW.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);

        let base_buff = &cond[0];
        assert_eq!(base_buff.name, "everlasting_moonglow_na_hp");
        assert_eq!(base_buff.stat, BuffableStat::NormalAtkFlatDmg);
        assert!((base_buff.value - 0.010).abs() < 1e-6);
        assert_eq!(base_buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            base_buff.activation,
            Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::HpPercent,
                offset: None,
                cap: None,
            })
        ));

        let burst_buff = &cond[1];
        assert_eq!(burst_buff.name, "everlasting_moonglow_burst_na_hp");
        assert_eq!(burst_buff.stat, BuffableStat::NormalAtkFlatDmg);
        assert!((burst_buff.value - 0.007).abs() < 1e-6);
        assert!(matches!(
            burst_buff.activation,
            Activation::Both(
                AutoCondition::StatScaling {
                    stat: BuffableStat::HpPercent,
                    offset: None,
                    cap: None,
                },
                ManualCondition::Toggle,
            )
        ));
        let rv = base_buff.refinement_values.unwrap();
        assert!((rv[4] - 0.020).abs() < 1e-6);
    }

    #[test]
    fn ash_graven_drinking_horn_has_hp_scaling_flat_dmg() {
        let passive = ASH_GRAVEN_DRINKING_HORN.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 5);
        let buff = &cond[0];
        assert_eq!(buff.name, "ash_graven_na_hp");
        assert_eq!(buff.stat, BuffableStat::NormalAtkFlatDmg);
        assert!((buff.value - 0.02).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Both(
                AutoCondition::StatScaling {
                    stat: BuffableStat::HpPercent,
                    offset: None,
                    cap: None,
                },
                ManualCondition::Toggle,
            )
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.04).abs() < 1e-6);
        // All 5 entries cover all damage types
        assert_eq!(cond[1].stat, BuffableStat::ChargedAtkFlatDmg);
        assert_eq!(cond[2].stat, BuffableStat::PlungingAtkFlatDmg);
        assert_eq!(cond[3].stat, BuffableStat::SkillFlatDmg);
        assert_eq!(cond[4].stat, BuffableStat::BurstFlatDmg);
    }

    #[test]
    fn ring_of_yaxche_has_hp_scaling_na_dmg() {
        let passive = RING_OF_YAXCHE.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "ring_of_yaxche_na_hp");
        assert_eq!(buff.stat, BuffableStat::NormalAtkFlatDmg);
        assert!((buff.value - 0.02).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Both(
                AutoCondition::StatScaling {
                    stat: BuffableStat::HpPercent,
                    offset: None,
                    cap: None,
                },
                ManualCondition::Toggle,
            )
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.04).abs() < 1e-6);
    }

    #[test]
    fn emerald_orb_has_atk_toggle() {
        let passive = EMERALD_ORB.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "emerald_orb_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.20).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.40).abs() < 1e-6);
    }

    #[test]
    fn twin_nephrite_has_atk_toggle() {
        let passive = TWIN_NEPHRITE.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "twin_nephrite_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.24).abs() < 1e-6);
    }
}
