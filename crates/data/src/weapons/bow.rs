use crate::buff::{
    Activation, AutoCondition, BuffTarget, BuffableStat, ConditionalBuff, ManualCondition,
    PassiveEffect, StatBuff,
};
use crate::types::{Rarity, WeaponData, WeaponPassive, WeaponSubStat, WeaponType};
use genshin_calc_core::Element;

// =============================================================================
// 5-Star Bows
// =============================================================================

pub const AMOS_BOW: WeaponData = WeaponData {
    id: "amos_bow",
    name: "Amos' Bow",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.108, 0.453, 0.453, 0.496])),
    passive: Some(WeaponPassive {
        name: "矢についた強風",
        effect: PassiveEffect {
            description: "NA/CA DMG+12-24%。矢が0.1秒飛ぶごとにさらにDMG+8%、最大5スタック",
            buffs: &[
                StatBuff {
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                },
                StatBuff {
                    stat: BuffableStat::ChargedAtkDmgBonus,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                },
            ],
            conditional_buffs: &[],
        },
    }),
};

pub const AQUA_SIMULACRA: WeaponData = WeaponData {
    id: "aqua_simulacra",
    name: "Aqua Simulacra",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.192, 0.804, 0.804, 0.882])),
    passive: Some(WeaponPassive {
        name: "幽水のワルツ",
        effect: PassiveEffect {
            description: "HP+16-32%。付近に敵がいる時DMG+20-40%",
            buffs: &[StatBuff {
                stat: BuffableStat::HpPercent,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "aqua_simulacra_dmg",
                description: "付近に敵が存在する時、DMG+20-40%",
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

pub const ASTRAL_VULTURES_CRIMSON_PLUMAGE: WeaponData = WeaponData {
    id: "astral_vultures_crimson_plumage",
    name: "Astral Vulture's Crimson Plumage",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.144, 0.603, 0.603, 0.662])),
    passive: Some(WeaponPassive {
        name: "Astral Vulture's Crimson Plumage",
        effect: PassiveEffect {
            description: "夜魂スタック蓄積でATK+12-24%（最大3スタック）、フルスタックでDMG+12-24%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "astral_vulture_atk_stacks",
                    description: "夜魂スタックでATK+12-24%（1スタック）、最大3スタック",
                    stat: BuffableStat::AtkPercent,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(3)),
                },
                ConditionalBuff {
                    name: "astral_vulture_full_stack_dmg",
                    description: "フルスタック時にDMG+12-24%",
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

pub const ELEGY_FOR_THE_END: WeaponData = WeaponData {
    id: "elegy_for_the_end",
    name: "Elegy for the End",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "別離の哀歌",
        effect: PassiveEffect {
            description: "EM+60-120。追憶の印蓄積でチーム全員にEM+100-200/ATK+20-40%",
            buffs: &[StatBuff {
                stat: BuffableStat::ElementalMastery,
                value: 60.0,
                refinement_values: Some([60.0, 75.0, 90.0, 105.0, 120.0]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "elegy_team_em",
                    description: "追憶の印フルスタック時にチーム全員EM+100-200",
                    stat: BuffableStat::ElementalMastery,
                    value: 100.0,
                    refinement_values: Some([100.0, 125.0, 150.0, 175.0, 200.0]),
                    stack_values: None,
                    target: BuffTarget::Team,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "elegy_team_atk",
                    description: "追憶の印フルスタック時にチーム全員ATK+20-40%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::Team,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const HUNTERS_PATH: WeaponData = WeaponData {
    id: "hunters_path",
    name: "Hunter's Path",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.096, 0.402, 0.402, 0.441])),
    passive: Some(WeaponPassive {
        name: "狩路の巡星",
        effect: PassiveEffect {
            description: "Ele DMG+12-24%。CAがEMに基づき追加ダメージ",
            buffs: &[StatBuff {
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "hunters_path_em_ca_flat",
                description: "EM×160-320%分を重撃フラットダメージに加算",
                stat: BuffableStat::ChargedAtkFlatDmg,
                value: 1.60,
                refinement_values: Some([1.60, 2.00, 2.40, 2.80, 3.20]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::ElementalMastery,
                    offset: None,
                    cap: None,
                }),
            }],
        },
    }),
};

pub const POLAR_STAR: WeaponData = WeaponData {
    id: "polar_star",
    name: "Polar Star",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.072, 0.302, 0.302, 0.331])),
    passive: Some(WeaponPassive {
        name: "極夜の白星",
        effect: PassiveEffect {
            description: "Skill/Burst DMG+12-24%。白極のスタック蓄積でATKアップ",
            buffs: &[
                StatBuff {
                    stat: BuffableStat::SkillDmgBonus,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                },
                StatBuff {
                    stat: BuffableStat::BurstDmgBonus,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                },
            ],
            conditional_buffs: &[],
        },
    }),
};

pub const SILVERSHOWER_HEARTSTRINGS: WeaponData = WeaponData {
    id: "silvershower_heartstrings",
    name: "Silvershower Heartstrings",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.144, 0.603, 0.603, 0.662])),
    passive: Some(WeaponPassive {
        name: "Silvershower Heartstrings",
        effect: PassiveEffect {
            description: "元素スキル使用でHP+12-24%スタック（最大3）、3スタックでHydro DMG+28-56%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "silvershower_hp_stacks",
                    description: "元素スキル使用でHP+12-24%（1スタック）、最大3スタック",
                    stat: BuffableStat::HpPercent,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(3)),
                },
                ConditionalBuff {
                    name: "silvershower_full_stack_hydro_dmg",
                    description: "3スタック時にHydro DMG+28-56%",
                    stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
                    value: 0.28,
                    refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const SKYWARD_HARP: WeaponData = WeaponData {
    id: "skyward_harp",
    name: "Skyward Harp",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.048, 0.201, 0.201, 0.221])),
    passive: Some(WeaponPassive {
        name: "天空のアルペジオ",
        effect: PassiveEffect {
            description: "CRIT DMG+20-40%。命中時に追加範囲DMG発生",
            buffs: &[StatBuff {
                stat: BuffableStat::CritDmg,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
            }],
            conditional_buffs: &[],
        },
    }),
};

pub const THE_DAYBREAK_CHRONICLES: WeaponData = WeaponData {
    id: "the_daybreak_chronicles",
    name: "The Daybreak Chronicles",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.096, 0.402, 0.402, 0.441])),
    passive: Some(WeaponPassive {
        name: "The Daybreak Chronicles",
        effect: PassiveEffect {
            description: "物語のスタック蓄積でDMG+8-16%（最大3スタック）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "the_daybreak_chronicles_dmg_stacks",
                description: "物語のスタックでDMG+8-16%（1スタック）、最大3スタック",
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

pub const THE_FIRST_GREAT_MAGIC: WeaponData = WeaponData {
    id: "the_first_great_magic",
    name: "The First Great Magic",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.144, 0.603, 0.603, 0.662])),
    passive: Some(WeaponPassive {
        name: "至高の魔術",
        effect: PassiveEffect {
            description: "CA DMG+16-32%。チームメンバーの元素タイプに応じATKアップ",
            buffs: &[StatBuff {
                stat: BuffableStat::ChargedAtkDmgBonus,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "first_great_magic_atk_1",
                    description: "1+異元素チームメンバーでATK+16-32%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount {
                        min_count: 1,
                    }),
                },
                ConditionalBuff {
                    name: "first_great_magic_atk_2",
                    description: "2+異元素チームメンバーでATK+16-32%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount {
                        min_count: 2,
                    }),
                },
                ConditionalBuff {
                    name: "first_great_magic_atk_3",
                    description: "3+異元素チームメンバーでATK+16-32%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount {
                        min_count: 3,
                    }),
                },
            ],
        },
    }),
};

pub const THUNDERING_PULSE: WeaponData = WeaponData {
    id: "thundering_pulse",
    name: "Thundering Pulse",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.144, 0.603, 0.603, 0.662])),
    passive: Some(WeaponPassive {
        name: "飛雷の鳴弦",
        effect: PassiveEffect {
            description: "ATK+20-40%。雷の巴紋を蓄積してNA DMGアップ",
            buffs: &[StatBuff {
                stat: BuffableStat::AtkPercent,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
            }],
            conditional_buffs: &[],
        },
    }),
};

// =============================================================================
// 4-Star Bows
// =============================================================================

pub const ALLEY_HUNTER: WeaponData = WeaponData {
    id: "alley_hunter",
    name: "Alley Hunter",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "裏通りの伏兵",
        effect: PassiveEffect {
            description: "Conditional: 待機時にDMGアップ、戦闘中にDMGダウン",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const BLACKCLIFF_WARBOW: WeaponData = WeaponData {
    id: "blackcliff_warbow",
    name: "Blackcliff Warbow",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.080, 0.335, 0.335, 0.368])),
    passive: Some(WeaponPassive {
        name: "追撃の矢",
        effect: PassiveEffect {
            description: "Conditional: 敵撃破時にATKアップ、最大3スタック",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const CHAIN_BREAKER: WeaponData = WeaponData {
    id: "chain_breaker",
    name: "Chain Breaker",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Chain Breaker",
        effect: PassiveEffect {
            description: "Conditional: チームの元素タイプに応じてEM/ATKアップ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const CLOUDFORGED: WeaponData = WeaponData {
    id: "cloudforged",
    name: "Cloudforged",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([36.0, 151.0, 151.0, 165.0])),
    passive: Some(WeaponPassive {
        name: "Cloudforged",
        effect: PassiveEffect {
            description: "Conditional: 元素エネルギー消費後にチーム全員のEMアップ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const COMPOUND_BOW: WeaponData = WeaponData {
    id: "compound_bow",
    name: "Compound Bow",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::PhysicalDmgBonus([
        0.150, 0.629, 0.629, 0.690,
    ])),
    passive: Some(WeaponPassive {
        name: "重錬の矢",
        effect: PassiveEffect {
            description: "Conditional: NA/CA命中時にATK+4%/NA速度+1.2%、最大4スタック",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const END_OF_THE_LINE: WeaponData = WeaponData {
    id: "end_of_the_line",
    name: "End of the Line",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.100, 0.419, 0.419, 0.459])),
    passive: Some(WeaponPassive {
        name: "川漁の極み",
        effect: PassiveEffect {
            description: "Conditional: 元素エネルギー獲得でフグ蓄積、Skill/Burst命中時に爆発ダメージ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const FADING_TWILIGHT: WeaponData = WeaponData {
    id: "fading_twilight",
    name: "Fading Twilight",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "暮色の薄明",
        effect: PassiveEffect {
            description: "夕暮(6%)/流明(10%)/朝暉(14%)の3状態を循環してDMGアップ",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "fading_twilight_dmg",
                description: "夕暮(6%)/流明(10%)/朝暉(14%)の3状態を循環してDMGアップ",
                stat: BuffableStat::DmgBonus,
                value: 0.06,
                refinement_values: None,
                stack_values: Some(&[0.06, 0.10, 0.14]),
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
        },
    }),
};

pub const FAVONIUS_WARBOW: WeaponData = WeaponData {
    id: "favonius_warbow",
    name: "Favonius Warbow",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.133, 0.557, 0.557, 0.613])),
    passive: Some(WeaponPassive {
        name: "西風の矢",
        effect: PassiveEffect {
            description: "Conditional: 会心時に元素粒子を生成、12秒に1回",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const FLOWER_WREATHED_FEATHERS: WeaponData = WeaponData {
    id: "flower_wreathed_feathers",
    name: "Flower-Wreathed Feathers",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "Flower-Wreathed Feathers",
        effect: PassiveEffect {
            description: "Conditional: 元素反応後にDMGアップ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const HAMAYUMI: WeaponData = WeaponData {
    id: "hamayumi",
    name: "Hamayumi",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "待ち伏せの矢",
        effect: PassiveEffect {
            description: "NA DMG+16-32%/CA DMG+12-24%。エネルギー満タンでさらにNA DMG+16-32%/CA DMG+12-24%",
            buffs: &[
                StatBuff {
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                },
                StatBuff {
                    stat: BuffableStat::ChargedAtkDmgBonus,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                },
            ],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "hamayumi_full_energy_na",
                    description: "エネルギー満タン時にNA DMG+16-32%",
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "hamayumi_full_energy_ca",
                    description: "エネルギー満タン時にCA DMG+12-24%",
                    stat: BuffableStat::ChargedAtkDmgBonus,
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

pub const IBIS_PIERCER: WeaponData = WeaponData {
    id: "ibis_piercer",
    name: "Ibis Piercer",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Ibis Piercer",
        effect: PassiveEffect {
            description: "Conditional: CA命中時にEMアップ、最大2スタック",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const KINGS_SQUIRE: WeaponData = WeaponData {
    id: "kings_squire",
    name: "King's Squire",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "森の従者",
        effect: PassiveEffect {
            description: "Conditional: スキル/バースト使用時にEMアップ。矢を放って追加ダメージ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const MITTERNACHTS_WALTZ: WeaponData = WeaponData {
    id: "mitternachts_waltz",
    name: "Mitternachts Waltz",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::PhysicalDmgBonus([
        0.113, 0.473, 0.473, 0.517,
    ])),
    passive: Some(WeaponPassive {
        name: "影の弾幕",
        effect: PassiveEffect {
            description: "通常攻撃命中時にSkill DMG+20-40%、元素スキル命中時にNA DMG+20-40%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "mitternachts_skill_dmg",
                    description: "通常攻撃命中時にSkill DMG+20-40%",
                    stat: BuffableStat::SkillDmgBonus,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "mitternachts_na_dmg",
                    description: "元素スキル命中時にNA DMG+20-40%",
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

pub const MOUUNS_MOON: WeaponData = WeaponData {
    id: "mouuns_moon",
    name: "Mouun's Moon",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "海淵の月",
        effect: PassiveEffect {
            description: "Conditional: チーム全員のBurstエネルギーの合計に基づきBurst DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const PREDATOR: WeaponData = WeaponData {
    id: "predator",
    name: "Predator",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "Predator",
        effect: PassiveEffect {
            description: "Cryo命中後にNA DMG+10%/CA DMG+10%（精錬固定）。アーロイ装備時にATK+66",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "predator_na_dmg",
                    description: "Cryo命中後にNA DMG+10%（精錬固定）",
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.10,
                    refinement_values: Some([0.10, 0.10, 0.10, 0.10, 0.10]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "predator_ca_dmg",
                    description: "Cryo命中後にCA DMG+10%（精錬固定）",
                    stat: BuffableStat::ChargedAtkDmgBonus,
                    value: 0.10,
                    refinement_values: Some([0.10, 0.10, 0.10, 0.10, 0.10]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const PROTOTYPE_CRESCENT: WeaponData = WeaponData {
    id: "prototype_crescent",
    name: "Prototype Crescent",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "月影の矢",
        effect: PassiveEffect {
            description: "弱点命中時にATK+36-72%、10秒",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "prototype_crescent_atk",
                description: "弱点命中時にATK+36-72%",
                stat: BuffableStat::AtkPercent,
                value: 0.36,
                refinement_values: Some([0.36, 0.45, 0.54, 0.63, 0.72]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const RAINBOW_SERPENTS_RAIN_BOW: WeaponData = WeaponData {
    id: "rainbow_serpents_rain_bow",
    name: "Rainbow Serpent's Rain Bow",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.100, 0.419, 0.419, 0.459])),
    passive: Some(WeaponPassive {
        name: "Rainbow Serpent's Rain Bow",
        effect: PassiveEffect {
            description: "Conditional: チームメンバーの元素タイプに応じてDMGアップ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const RANGE_GAUGE: WeaponData = WeaponData {
    id: "range_gauge",
    name: "Range Gauge",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Range Gauge",
        effect: PassiveEffect {
            description: "Conditional: CA命中時にスタック蓄積でATKアップ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const ROYAL_BOW: WeaponData = WeaponData {
    id: "royal_bow",
    name: "Royal Bow",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "集中",
        effect: PassiveEffect {
            description: "Conditional: ダメージ時にCRIT Rate+8%、最大5スタック。会心でリセット",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const RUST: WeaponData = WeaponData {
    id: "rust",
    name: "Rust",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "速射の弦",
        effect: PassiveEffect {
            description: "NA DMG+40-80%。CA DMG-10%",
            buffs: &[StatBuff {
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.40,
                refinement_values: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
            }],
            conditional_buffs: &[],
        },
    }),
};

pub const SACRIFICIAL_BOW: WeaponData = WeaponData {
    id: "sacrificial_bow",
    name: "Sacrificial Bow",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "気定神閑",
        effect: PassiveEffect {
            description: "Conditional: 元素スキルがダメージを与えた時にCD終了、30秒に1回",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const SCION_OF_THE_BLAZING_SUN: WeaponData = WeaponData {
    id: "scion_of_the_blazing_sun",
    name: "Scion of the Blazing Sun",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.040, 0.168, 0.168, 0.184])),
    passive: Some(WeaponPassive {
        name: "灼熱の太陽の子",
        effect: PassiveEffect {
            description: "Conditional: CA命中でCA DMGアップ。チームがDendro反応を起こすとさらにアップ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const SEQUENCE_OF_SOLITUDE: WeaponData = WeaponData {
    id: "sequence_of_solitude",
    name: "Sequence of Solitude",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "Sequence of Solitude",
        effect: PassiveEffect {
            description: "HP変動時にSkill DMG+14-28%/Burst DMG+14-28%、6秒",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "sequence_skill_dmg",
                    description: "HP変動時にSkill DMG+14-28%",
                    stat: BuffableStat::SkillDmgBonus,
                    value: 0.14,
                    refinement_values: Some([0.14, 0.175, 0.21, 0.245, 0.28]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "sequence_burst_dmg",
                    description: "HP変動時にBurst DMG+14-28%",
                    stat: BuffableStat::BurstDmgBonus,
                    value: 0.14,
                    refinement_values: Some([0.14, 0.175, 0.21, 0.245, 0.28]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const SNARE_HOOK: WeaponData = WeaponData {
    id: "snare_hook",
    name: "Snare Hook",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.133, 0.557, 0.557, 0.613])),
    passive: Some(WeaponPassive {
        name: "Snare Hook",
        effect: PassiveEffect {
            description: "元素スキル命中後にDMG+16-32%、10秒",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "snare_hook_dmg",
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

pub const SONG_OF_STILLNESS: WeaponData = WeaponData {
    id: "song_of_stillness",
    name: "Song of Stillness",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "Song of Stillness",
        effect: PassiveEffect {
            description: "Conditional: HP回復後にDMGアップ、8秒",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const THE_STRINGLESS: WeaponData = WeaponData {
    id: "the_stringless",
    name: "The Stringless",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([36.0, 151.0, 151.0, 165.0])),
    passive: Some(WeaponPassive {
        name: "琴弓の詩",
        effect: PassiveEffect {
            description: "Skill/Burst DMG+24-48%",
            buffs: &[
                StatBuff {
                    stat: BuffableStat::SkillDmgBonus,
                    value: 0.24,
                    refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
                },
                StatBuff {
                    stat: BuffableStat::BurstDmgBonus,
                    value: 0.24,
                    refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
                },
            ],
            conditional_buffs: &[],
        },
    }),
};

pub const THE_VIRIDESCENT_HUNT: WeaponData = WeaponData {
    id: "the_viridescent_hunt",
    name: "The Viridescent Hunt",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "緑の狩猟",
        effect: PassiveEffect {
            description: "Conditional: NA/CA命中時に風の渦を発生、14秒に1回",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const WINDBLUME_ODE: WeaponData = WeaponData {
    id: "windblume_ode",
    name: "Windblume Ode",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([36.0, 151.0, 151.0, 165.0])),
    passive: Some(WeaponPassive {
        name: "風花の願い",
        effect: PassiveEffect {
            description: "元素スキル使用後にATK+16-32%、6秒",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "windblume_atk",
                description: "元素スキル使用後にATK+16-32%",
                stat: BuffableStat::AtkPercent,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

// =============================================================================
// 3-Star Bows
// =============================================================================

pub const MESSENGER: WeaponData = WeaponData {
    id: "messenger",
    name: "Messenger",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star3,
    base_atk: [40.0, 396.0, 415.0, 448.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.068, 0.285, 0.285, 0.312])),
    passive: Some(WeaponPassive {
        name: "射手の教え",
        effect: PassiveEffect {
            description: "Conditional: CA弱点命中時に追加ATKダメージ100%、10秒に1回",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const RAVEN_BOW: WeaponData = WeaponData {
    id: "raven_bow",
    name: "Raven Bow",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star3,
    base_atk: [40.0, 396.0, 415.0, 448.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([20.0, 85.0, 85.0, 94.0])),
    passive: Some(WeaponPassive {
        name: "烏の羽",
        effect: PassiveEffect {
            description: "Conditional: Hydro/Pyro影響を受けた敵へのDMG+12%",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const RECURVE_BOW: WeaponData = WeaponData {
    id: "recurve_bow",
    name: "Recurve Bow",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star3,
    base_atk: [38.0, 314.0, 334.0, 354.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.102, 0.427, 0.427, 0.469])),
    passive: Some(WeaponPassive {
        name: "気力回復",
        effect: PassiveEffect {
            description: "Conditional: 敵撃破時にHP回復8%",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const SHARPSHOOTERS_OATH: WeaponData = WeaponData {
    id: "sharpshooters_oath",
    name: "Sharpshooter's Oath",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star3,
    base_atk: [39.0, 355.0, 375.0, 401.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.102, 0.427, 0.427, 0.469])),
    passive: Some(WeaponPassive {
        name: "精密射撃",
        effect: PassiveEffect {
            description: "Conditional: 弱点命中時にDMG+24%",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const SLINGSHOT: WeaponData = WeaponData {
    id: "slingshot",
    name: "Slingshot",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star3,
    base_atk: [38.0, 314.0, 334.0, 354.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.068, 0.285, 0.285, 0.312])),
    passive: Some(WeaponPassive {
        name: "弾き出す",
        effect: PassiveEffect {
            description: "Conditional: 矢が0.3秒以内に命中でDMG+36%、それ以上でDMG-10%",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

// =============================================================================
// 1-2 Star Bows
// =============================================================================

pub const HUNTERS_BOW: WeaponData = WeaponData {
    id: "hunters_bow",
    name: "Hunter's Bow",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star1,
    base_atk: [23.0, 185.0, 185.0, 185.0],
    sub_stat: None,
    passive: None,
};

pub const SEASONED_HUNTERS_BOW: WeaponData = WeaponData {
    id: "seasoned_hunters_bow",
    name: "Seasoned Hunter's Bow",
    weapon_type: WeaponType::Bow,
    rarity: Rarity::Star2,
    base_atk: [33.0, 243.0, 243.0, 243.0],
    sub_stat: None,
    passive: None,
};

/// All bow weapon data references.
pub const ALL_BOWS: &[&WeaponData] = &[
    // 5-Star
    &AMOS_BOW,
    &AQUA_SIMULACRA,
    &ASTRAL_VULTURES_CRIMSON_PLUMAGE,
    &ELEGY_FOR_THE_END,
    &HUNTERS_PATH,
    &POLAR_STAR,
    &SILVERSHOWER_HEARTSTRINGS,
    &SKYWARD_HARP,
    &THE_DAYBREAK_CHRONICLES,
    &THE_FIRST_GREAT_MAGIC,
    &THUNDERING_PULSE,
    // 4-Star
    &ALLEY_HUNTER,
    &BLACKCLIFF_WARBOW,
    &CHAIN_BREAKER,
    &CLOUDFORGED,
    &COMPOUND_BOW,
    &END_OF_THE_LINE,
    &FADING_TWILIGHT,
    &FAVONIUS_WARBOW,
    &FLOWER_WREATHED_FEATHERS,
    &HAMAYUMI,
    &IBIS_PIERCER,
    &KINGS_SQUIRE,
    &MITTERNACHTS_WALTZ,
    &MOUUNS_MOON,
    &PREDATOR,
    &PROTOTYPE_CRESCENT,
    &RAINBOW_SERPENTS_RAIN_BOW,
    &RANGE_GAUGE,
    &ROYAL_BOW,
    &RUST,
    &SACRIFICIAL_BOW,
    &SCION_OF_THE_BLAZING_SUN,
    &SEQUENCE_OF_SOLITUDE,
    &SNARE_HOOK,
    &SONG_OF_STILLNESS,
    &THE_STRINGLESS,
    &THE_VIRIDESCENT_HUNT,
    &WINDBLUME_ODE,
    // 3-Star
    &MESSENGER,
    &RAVEN_BOW,
    &RECURVE_BOW,
    &SHARPSHOOTERS_OATH,
    &SLINGSHOT,
    // 1-2 Star
    &HUNTERS_BOW,
    &SEASONED_HUNTERS_BOW,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buff::AutoCondition;

    #[test]
    fn hunters_path_has_em_ca_flat_conditional() {
        let passive = HUNTERS_PATH.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "hunters_path_em_ca_flat");
        assert_eq!(buff.stat, BuffableStat::ChargedAtkFlatDmg);
        assert!((buff.value - 1.60).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::ElementalMastery,
                offset: None,
                cap: None,
            })
        ));
    }

    #[test]
    fn astral_vulture_has_atk_stacks_and_full_stack_dmg() {
        let passive = ASTRAL_VULTURES_CRIMSON_PLUMAGE.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        let stacks_buff = &cond_buffs[0];
        assert_eq!(stacks_buff.name, "astral_vulture_atk_stacks");
        assert_eq!(stacks_buff.stat, BuffableStat::AtkPercent);
        assert!((stacks_buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            stacks_buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));

        let full_buff = &cond_buffs[1];
        assert_eq!(full_buff.name, "astral_vulture_full_stack_dmg");
        assert_eq!(full_buff.stat, BuffableStat::DmgBonus);
        assert!((full_buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            full_buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn silvershower_has_hp_stacks_and_hydro_dmg() {
        let passive = SILVERSHOWER_HEARTSTRINGS.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        let stacks_buff = &cond_buffs[0];
        assert_eq!(stacks_buff.name, "silvershower_hp_stacks");
        assert_eq!(stacks_buff.stat, BuffableStat::HpPercent);
        assert!((stacks_buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            stacks_buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));

        let full_buff = &cond_buffs[1];
        assert_eq!(full_buff.name, "silvershower_full_stack_hydro_dmg");
        assert_eq!(
            full_buff.stat,
            BuffableStat::ElementalDmgBonus(genshin_calc_core::Element::Hydro)
        );
        assert!((full_buff.value - 0.28).abs() < 1e-6);
        assert!(matches!(
            full_buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn daybreak_chronicles_has_dmg_stacks() {
        let passive = THE_DAYBREAK_CHRONICLES.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "the_daybreak_chronicles_dmg_stacks");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        assert!(buff.refinement_values.is_some());
    }

    #[test]
    fn elegy_has_em_statbuff_and_team_conditionals() {
        let passive = ELEGY_FOR_THE_END.passive.unwrap();

        assert_eq!(passive.effect.buffs.len(), 1);
        assert_eq!(passive.effect.buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((passive.effect.buffs[0].value - 60.0).abs() < 1e-6);

        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "elegy_team_em");
        assert_eq!(cond_buffs[0].stat, BuffableStat::ElementalMastery);
        assert!((cond_buffs[0].value - 100.0).abs() < 1e-6);
        assert_eq!(cond_buffs[0].target, BuffTarget::Team);

        assert_eq!(cond_buffs[1].name, "elegy_team_atk");
        assert_eq!(cond_buffs[1].stat, BuffableStat::AtkPercent);
        assert!((cond_buffs[1].value - 0.20).abs() < 1e-6);
        assert_eq!(cond_buffs[1].target, BuffTarget::Team);

        for buff in cond_buffs {
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Toggle)
            ));
        }
    }

    #[test]
    fn first_great_magic_has_team_diff_atk_conditionals() {
        let passive = THE_FIRST_GREAT_MAGIC.passive.unwrap();

        assert_eq!(passive.effect.buffs.len(), 1);
        assert_eq!(
            passive.effect.buffs[0].stat,
            BuffableStat::ChargedAtkDmgBonus
        );

        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 3);

        let expected_min_counts = [1u8, 2, 3];
        for (i, buff) in cond_buffs.iter().enumerate() {
            let expected_name = format!("first_great_magic_atk_{}", i + 1);
            assert_eq!(buff.name, expected_name.as_str());
            assert_eq!(buff.stat, BuffableStat::AtkPercent);
            assert!((buff.value - 0.16).abs() < 1e-6);
            assert!(matches!(
                buff.activation,
                Activation::Auto(AutoCondition::TeamDiffElementCount { min_count })
                if min_count == expected_min_counts[i]
            ));
        }
    }

    #[test]
    fn fading_twilight_has_dmg_stacks() {
        let passive = FADING_TWILIGHT.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "fading_twilight_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.06).abs() < 1e-6);
        assert!(buff.refinement_values.is_none());
        assert!(buff.stack_values.is_some());
        let sv = buff.stack_values.unwrap();
        assert_eq!(sv.len(), 3);
        assert!((sv[0] - 0.06).abs() < 1e-6);
        assert!((sv[1] - 0.10).abs() < 1e-6);
        assert!((sv[2] - 0.14).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
    }

    #[test]
    fn hamayumi_has_statbuffs_and_energy_toggle() {
        let passive = HAMAYUMI.passive.unwrap();
        assert_eq!(passive.effect.buffs.len(), 2);
        assert_eq!(
            passive.effect.buffs[0].stat,
            BuffableStat::NormalAtkDmgBonus
        );
        assert!((passive.effect.buffs[0].value - 0.16).abs() < 1e-6);
        assert_eq!(
            passive.effect.buffs[1].stat,
            BuffableStat::ChargedAtkDmgBonus
        );
        assert!((passive.effect.buffs[1].value - 0.12).abs() < 1e-6);
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);
        assert_eq!(cond_buffs[0].name, "hamayumi_full_energy_na");
        assert_eq!(cond_buffs[0].stat, BuffableStat::NormalAtkDmgBonus);
        assert!((cond_buffs[0].value - 0.16).abs() < 1e-6);
        assert!(matches!(
            cond_buffs[0].activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        assert_eq!(cond_buffs[1].name, "hamayumi_full_energy_ca");
        assert_eq!(cond_buffs[1].stat, BuffableStat::ChargedAtkDmgBonus);
        assert!((cond_buffs[1].value - 0.12).abs() < 1e-6);
        assert!(matches!(
            cond_buffs[1].activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn mitternachts_waltz_has_cross_buff_toggle() {
        let passive = MITTERNACHTS_WALTZ.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);
        assert_eq!(cond_buffs[0].name, "mitternachts_skill_dmg");
        assert_eq!(cond_buffs[0].stat, BuffableStat::SkillDmgBonus);
        assert!((cond_buffs[0].value - 0.20).abs() < 1e-6);
        assert_eq!(cond_buffs[1].name, "mitternachts_na_dmg");
        assert_eq!(cond_buffs[1].stat, BuffableStat::NormalAtkDmgBonus);
        assert!((cond_buffs[1].value - 0.20).abs() < 1e-6);
    }

    #[test]
    fn predator_has_fixed_na_ca_toggle() {
        let passive = PREDATOR.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);
        assert_eq!(cond_buffs[0].name, "predator_na_dmg");
        assert_eq!(cond_buffs[0].stat, BuffableStat::NormalAtkDmgBonus);
        assert!((cond_buffs[0].value - 0.10).abs() < 1e-6);
        let rv = cond_buffs[0].refinement_values.unwrap();
        for v in &rv {
            assert!((*v - 0.10).abs() < 1e-6);
        }
        assert_eq!(cond_buffs[1].name, "predator_ca_dmg");
        assert_eq!(cond_buffs[1].stat, BuffableStat::ChargedAtkDmgBonus);
        assert!((cond_buffs[1].value - 0.10).abs() < 1e-6);
    }

    #[test]
    fn sequence_of_solitude_has_skill_burst_toggle() {
        let passive = SEQUENCE_OF_SOLITUDE.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);
        assert_eq!(cond_buffs[0].name, "sequence_skill_dmg");
        assert_eq!(cond_buffs[0].stat, BuffableStat::SkillDmgBonus);
        assert!((cond_buffs[0].value - 0.14).abs() < 1e-6);
        assert_eq!(cond_buffs[1].name, "sequence_burst_dmg");
        assert_eq!(cond_buffs[1].stat, BuffableStat::BurstDmgBonus);
        assert!((cond_buffs[1].value - 0.14).abs() < 1e-6);
    }

    #[test]
    fn prototype_crescent_has_atk_toggle() {
        let passive = PROTOTYPE_CRESCENT.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "prototype_crescent_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.36).abs() < 1e-6);
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.72).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn snare_hook_has_dmg_toggle() {
        let passive = SNARE_HOOK.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "snare_hook_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.16).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn windblume_ode_has_atk_toggle() {
        let passive = WINDBLUME_ODE.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "windblume_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.16).abs() < 1e-6);
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.32).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }
}
