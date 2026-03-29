use crate::buff::{
    Activation, AutoCondition, BuffTarget, BuffableStat, ConditionalBuff, ManualCondition,
    PassiveEffect, StatBuff,
};
use crate::types::{Rarity, Region, WeaponData, WeaponPassive, WeaponSubStat, WeaponType};

// =============================================================================
// 5-Star Polearms
// =============================================================================

pub const BLOODSOAKED_RUINS: WeaponData = WeaponData {
    id: "bloodsoaked_ruins",
    name: "Bloodsoaked Ruins",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.048, 0.201, 0.201, 0.221])),
    passive: Some(WeaponPassive {
        name: "Bloodsoaked Ruins",
        effect: PassiveEffect {
            description: "Conditional: 条件付きバフ効果",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const CALAMITY_QUELLER: WeaponData = WeaponData {
    id: "calamity_queller",
    name: "Calamity Queller",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    base_atk: [49.0, 648.0, 679.0, 741.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.036, 0.151, 0.151, 0.165])),
    passive: Some(WeaponPassive {
        name: "息災",
        effect: PassiveEffect {
            description: "Ele DMG+12-24%。元素スキル使用後にATKアップ、未出場時は2倍",
            buffs: &[StatBuff {
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[],
        },
    }),
};

pub const CRIMSON_MOONS_SEMBLANCE: WeaponData = WeaponData {
    id: "crimson_moons_semblance",
    name: "Crimson Moon's Semblance",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.048, 0.201, 0.201, 0.221])),
    passive: Some(WeaponPassive {
        name: "Crimson Moon's Semblance",
        effect: PassiveEffect {
            description: "Conditional: 条件付きバフ効果",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const ENGULFING_LIGHTNING: WeaponData = WeaponData {
    id: "engulfing_lightning",
    name: "Engulfing Lightning",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "漁獲の雷",
        effect: PassiveEffect {
            description: "Conditional: ERに基づきATKアップ。元素爆発後にER+30%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "engulfing_er_atk",
                    description: "ER超過分の28-56%をATK%に変換 (cap: 80-120%)",
                    stat: BuffableStat::AtkPercent,
                    value: 0.28,
                    refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::EnergyRecharge,
                        offset: Some(1.0),
                        cap: Some([0.80, 0.90, 1.00, 1.10, 1.20]),
                    }),
                },
                ConditionalBuff {
                    name: "engulfing_burst_er",
                    description: "元素爆発後12秒間ER+30-50%",
                    stat: BuffableStat::EnergyRecharge,
                    value: 0.30,
                    refinement_values: Some([0.30, 0.35, 0.40, 0.45, 0.50]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const FRACTURED_HALO: WeaponData = WeaponData {
    id: "fractured_halo",
    name: "Fractured Halo",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.144, 0.603, 0.603, 0.662])),
    passive: Some(WeaponPassive {
        name: "Fractured Halo",
        effect: PassiveEffect {
            description: "Conditional: 条件付きバフ効果",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const LUMIDOUCE_ELEGY: WeaponData = WeaponData {
    id: "lumidouce_elegy",
    name: "Lumidouce Elegy",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.072, 0.302, 0.302, 0.331])),
    passive: Some(WeaponPassive {
        name: "Lumidouce Elegy",
        effect: PassiveEffect {
            description: "ATK+15-31%。条件付きで追加バフ",
            buffs: &[StatBuff {
                stat: BuffableStat::AtkPercent,
                value: 0.15,
                refinement_values: Some([0.15, 0.19, 0.23, 0.27, 0.31]),
            }],
            conditional_buffs: &[],
        },
    }),
};

pub const PRIMORDIAL_JADE_WINGED_SPEAR: WeaponData = WeaponData {
    id: "primordial_jade_winged_spear",
    name: "Primordial Jade Winged-Spear",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.048, 0.201, 0.201, 0.221])),
    passive: Some(WeaponPassive {
        name: "昭理の鳶",
        effect: PassiveEffect {
            description: "命中時にATK+3.2-6%、6スタックまで。フルスタックでDMG+12-24%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "pjws_atk_stacks",
                    description: "命中時にATK+3.2-6%（1スタック）、最大6スタック",
                    stat: BuffableStat::AtkPercent,
                    value: 0.032,
                    refinement_values: Some([0.032, 0.039, 0.046, 0.053, 0.060]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(6)),
                },
                ConditionalBuff {
                    name: "pjws_full_stack_dmg",
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

pub const SKYWARD_SPINE: WeaponData = WeaponData {
    id: "skyward_spine",
    name: "Skyward Spine",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.080, 0.335, 0.335, 0.368])),
    passive: Some(WeaponPassive {
        name: "天空の脊",
        effect: PassiveEffect {
            description: "CRIT Rate+8-16%。攻撃速度+12%。通常/重撃命中時に追加ダメージ",
            buffs: &[StatBuff {
                stat: BuffableStat::CritRate,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
            }],
            conditional_buffs: &[],
        },
    }),
};

pub const STAFF_OF_HOMA: WeaponData = WeaponData {
    id: "staff_of_homa",
    name: "Staff of Homa",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.144, 0.603, 0.603, 0.662])),
    passive: Some(WeaponPassive {
        name: "護摩の杖",
        effect: PassiveEffect {
            description: "HP+20-40%。HP上限の0.8-1.6%分ATKアップ。HP50%以下でさらに1.0-1.8%分アップ",
            buffs: &[StatBuff {
                stat: BuffableStat::HpPercent,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "homa_hp_atk",
                    description: "HP上限の0.8%分ATKアップ（常時）",
                    stat: BuffableStat::AtkFlat,
                    value: 0.008,
                    refinement_values: Some([0.008, 0.010, 0.012, 0.014, 0.016]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::HpPercent,
                        offset: None,
                        cap: None,
                    }),
                },
                ConditionalBuff {
                    name: "homa_low_hp_atk",
                    description: "HP50%以下の時、さらにHP上限の1.0%分ATKアップ",
                    stat: BuffableStat::AtkFlat,
                    value: 0.010,
                    refinement_values: Some([0.010, 0.012, 0.014, 0.016, 0.018]),
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

pub const STAFF_OF_THE_SCARLET_SANDS: WeaponData = WeaponData {
    id: "staff_of_the_scarlet_sands",
    name: "Staff of the Scarlet Sands",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.096, 0.402, 0.402, 0.441])),
    passive: Some(WeaponPassive {
        name: "赤砂の杖",
        effect: PassiveEffect {
            description: "Conditional: EMに基づきATKアップ。スキル命中でさらにATKアップ",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "scarlet_sands_em_atk",
                    description: "EM×52-104%分をATKフラットに加算",
                    stat: BuffableStat::AtkFlat,
                    value: 0.52,
                    refinement_values: Some([0.52, 0.65, 0.78, 0.91, 1.04]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::ElementalMastery,
                        offset: None,
                        cap: None,
                    }),
                },
                ConditionalBuff {
                    name: "scarlet_sands_skill_stacks",
                    description: "スキル命中後10秒間、EM×28-56%分ATKアップ。最大2スタック",
                    stat: BuffableStat::AtkFlat,
                    value: 0.28,
                    refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Both(
                        AutoCondition::StatScaling {
                            stat: BuffableStat::ElementalMastery,
                            offset: None,
                            cap: None,
                        },
                        ManualCondition::Stacks(2),
                    ),
                },
            ],
        },
    }),
};

pub const SYMPHONIST_OF_SCENTS: WeaponData = WeaponData {
    id: "symphonist_of_scents",
    name: "Symphonist of Scents",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.144, 0.603, 0.603, 0.662])),
    passive: Some(WeaponPassive {
        name: "Symphonist of Scents",
        effect: PassiveEffect {
            description: "Conditional: 条件付きバフ効果",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const VORTEX_VANQUISHER: WeaponData = WeaponData {
    id: "vortex_vanquisher",
    name: "Vortex Vanquisher",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.108, 0.453, 0.453, 0.496])),
    passive: Some(WeaponPassive {
        name: "金璋の槍",
        effect: PassiveEffect {
            description: "攻撃命中でATK+4-8%スタック（最大5）、シールド時は2倍",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "vortex_vanquisher_atk_stacks",
                    description: "攻撃命中でATK+4-8%（1スタック）、最大5スタック",
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
                ConditionalBuff {
                    name: "vortex_vanquisher_shield_atk_stacks",
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

// =============================================================================
// 4-Star Polearms
// =============================================================================

pub const BALLAD_OF_THE_FJORDS: WeaponData = WeaponData {
    id: "ballad_of_the_fjords",
    name: "Ballad of the Fjords",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Ballad of the Fjords",
        effect: PassiveEffect {
            description: "Conditional: チーム内の元素タイプ数に応じてEMアップ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const CRESCENT_PIKE: WeaponData = WeaponData {
    id: "crescent_pike",
    name: "Crescent Pike",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::PhysicalDmgBonus([
        0.075, 0.314, 0.314, 0.345,
    ])),
    passive: Some(WeaponPassive {
        name: "白月の槍",
        effect: PassiveEffect {
            description: "Conditional: 元素粒子取得後に通常/重撃で追加ATKダメージ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const DEATHMATCH: WeaponData = WeaponData {
    id: "deathmatch",
    name: "Deathmatch",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.080, 0.335, 0.335, 0.368])),
    passive: Some(WeaponPassive {
        name: "闘志",
        effect: PassiveEffect {
            description: "敵2体以上: ATK/DEF+16-32%。敵1体以下: ATK+24-48%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "deathmatch_atk_multi",
                    description: "敵2体以上でATK+16-32%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "deathmatch_def_multi",
                    description: "敵2体以上でDEF+16-32%",
                    stat: BuffableStat::DefPercent,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "deathmatch_atk_1enemy",
                    description: "敵1体以下でATK+24-48%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.24,
                    refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const DIALOGUES_OF_THE_DESERT_SAGES: WeaponData = WeaponData {
    id: "dialogues_of_the_desert_sages",
    name: "Dialogues of the Desert Sages",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "Dialogues of the Desert Sages",
        effect: PassiveEffect {
            description: "Conditional: 条件付きバフ効果",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const DRAGONS_BANE: WeaponData = WeaponData {
    id: "dragons_bane",
    name: "Dragon's Bane",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([48.0, 201.0, 201.0, 221.0])),
    passive: Some(WeaponPassive {
        name: "炎と水の滅竜",
        effect: PassiveEffect {
            description: "水/炎元素影響下の敵へのDMG+20-36%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "dragons_bane_dmg",
                description: "水/炎元素の影響下の敵へのDMG+20-36%",
                stat: BuffableStat::DmgBonus,
                value: 0.20,
                refinement_values: Some([0.20, 0.24, 0.28, 0.32, 0.36]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const DRAGONSPINE_SPEAR: WeaponData = WeaponData {
    id: "dragonspine_spear",
    name: "Dragonspine Spear",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::PhysicalDmgBonus([
        0.150, 0.629, 0.629, 0.690,
    ])),
    passive: Some(WeaponPassive {
        name: "霜葬の槍",
        effect: PassiveEffect {
            description: "Conditional: 通常/重撃命中時に氷柱落下、氷元素影響下の敵には追加ダメージ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const FAVONIUS_LANCE: WeaponData = WeaponData {
    id: "favonius_lance",
    name: "Favonius Lance",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "西風の息吹",
        effect: PassiveEffect {
            description: "Conditional: 会心命中時に元素粒子を生成、12-6秒に1回",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const FOOTPRINT_OF_THE_RAINBOW: WeaponData = WeaponData {
    id: "footprint_of_the_rainbow",
    name: "Footprint of the Rainbow",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::DefPercent([0.113, 0.473, 0.473, 0.517])),
    passive: Some(WeaponPassive {
        name: "Footprint of the Rainbow",
        effect: PassiveEffect {
            description: "Conditional: 条件付きバフ効果",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const KITAIN_CROSS_SPEAR: WeaponData = WeaponData {
    id: "kitain_cross_spear",
    name: "Kitain Cross Spear",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([24.0, 101.0, 101.0, 110.0])),
    passive: Some(WeaponPassive {
        name: "名刀・北谷の十字槍",
        effect: PassiveEffect {
            description: "Skill DMG+6-12%。スキル命中で元素エネルギー消費後に回復",
            buffs: &[StatBuff {
                stat: BuffableStat::SkillDmgBonus,
                value: 0.06,
                refinement_values: Some([0.06, 0.075, 0.09, 0.105, 0.12]),
            }],
            conditional_buffs: &[],
        },
    }),
};

pub const LITHIC_SPEAR: WeaponData = WeaponData {
    id: "lithic_spear",
    name: "Lithic Spear",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "千岩の槍",
        effect: PassiveEffect {
            description: "Conditional: チーム内の璃月キャラ人数に応じてATK/CRIT Rateアップ",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "lithic_spear_atk",
                    description: "璃月キャラ1人につきATK+7-11%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.07,
                    refinement_values: Some([0.07, 0.08, 0.09, 0.10, 0.11]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamRegionCount {
                        region: Region::Liyue,
                    }),
                },
                ConditionalBuff {
                    name: "lithic_spear_crit",
                    description: "璃月キャラ1人につきCR+3-7%",
                    stat: BuffableStat::CritRate,
                    value: 0.03,
                    refinement_values: Some([0.03, 0.04, 0.05, 0.06, 0.07]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamRegionCount {
                        region: Region::Liyue,
                    }),
                },
            ],
        },
    }),
};

pub const MISSIVE_WINDSPEAR: WeaponData = WeaponData {
    id: "missive_windspear",
    name: "Missive Windspear",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "Missive Windspear",
        effect: PassiveEffect {
            description: "Conditional: 元素反応を起こすとATK/EMアップ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const MOONPIERCER: WeaponData = WeaponData {
    id: "moonpiercer",
    name: "Moonpiercer",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([24.0, 101.0, 101.0, 110.0])),
    passive: Some(WeaponPassive {
        name: "月穿ち",
        effect: PassiveEffect {
            description: "Conditional: 草元素反応でATK付与の種を生成",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const MOUNTAIN_BRACING_BOLT: WeaponData = WeaponData {
    id: "mountain_bracing_bolt",
    name: "Mountain-Bracing Bolt",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "Mountain-Bracing Bolt",
        effect: PassiveEffect {
            description: "Conditional: 条件付きバフ効果",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const PROSPECTORS_DRILL: WeaponData = WeaponData {
    id: "prospectors_drill",
    name: "Prospector's Drill",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Prospector's Drill",
        effect: PassiveEffect {
            description: "Conditional: 条件付きバフ効果",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const PROSPECTORS_SHOVEL: WeaponData = WeaponData {
    id: "prospectors_shovel",
    name: "Prospector's Shovel",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "Prospector's Shovel",
        effect: PassiveEffect {
            description: "Conditional: 条件付きバフ効果",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const PROTOTYPE_STARGLITTER: WeaponData = WeaponData {
    id: "prototype_starglitter",
    name: "Prototype Starglitter",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.100, 0.419, 0.419, 0.459])),
    passive: Some(WeaponPassive {
        name: "魔力の導き",
        effect: PassiveEffect {
            description: "Conditional: 元素スキル使用後に通常/重撃DMG+8-16%、12秒、2スタックまで",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const RIGHTFUL_REWARD: WeaponData = WeaponData {
    id: "rightful_reward",
    name: "Rightful Reward",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Rightful Reward",
        effect: PassiveEffect {
            description: "Conditional: 元素スキル命中でHP回復",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const ROYAL_SPEAR: WeaponData = WeaponData {
    id: "royal_spear",
    name: "Royal Spear",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "集中",
        effect: PassiveEffect {
            description: "Conditional: ダメージを与えるとCRIT Rate+8-16%、5スタックまで。会心発生でリセット",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const SACRIFICERS_STAFF: WeaponData = WeaponData {
    id: "sacrificers_staff",
    name: "Sacrificer's Staff",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [45.0, 545.0, 571.0, 620.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.020, 0.084, 0.084, 0.092])),
    passive: Some(WeaponPassive {
        name: "Sacrificer's Staff",
        effect: PassiveEffect {
            description: "Conditional: 条件付きバフ効果",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const TAMAYURATEI_NO_OHANASHI: WeaponData = WeaponData {
    id: "tamayuratei_no_ohanashi",
    name: "Tamayuratei no Ohanashi",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "Tamayuratei no Ohanashi",
        effect: PassiveEffect {
            description: "Conditional: 条件付きバフ効果",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const THE_CATCH: WeaponData = WeaponData {
    id: "the_catch",
    name: "The Catch",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.100, 0.419, 0.419, 0.459])),
    passive: Some(WeaponPassive {
        name: "漁獲",
        effect: PassiveEffect {
            description: "Burst DMG+16-32%、Burst CRIT Rate+6-12%",
            buffs: &[
                StatBuff {
                    stat: BuffableStat::BurstDmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                },
                StatBuff {
                    stat: BuffableStat::CritRate,
                    value: 0.06,
                    refinement_values: Some([0.06, 0.075, 0.09, 0.105, 0.12]),
                },
            ],
            conditional_buffs: &[],
        },
    }),
};

pub const WAVEBREAKERS_FIN: WeaponData = WeaponData {
    id: "wavebreakers_fin",
    name: "Wavebreaker's Fin",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [45.0, 545.0, 571.0, 620.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.030, 0.126, 0.126, 0.138])),
    passive: Some(WeaponPassive {
        name: "波乗りの鰭",
        effect: PassiveEffect {
            description: "Conditional: チーム全員の元素エネルギー上限に応じて元素爆発DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

// =============================================================================
// 3-Star Polearms
// =============================================================================

pub const BLACK_TASSEL: WeaponData = WeaponData {
    id: "black_tassel",
    name: "Black Tassel",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star3,
    base_atk: [38.0, 314.0, 334.0, 354.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.102, 0.427, 0.427, 0.469])),
    passive: Some(WeaponPassive {
        name: "黒纓の槍",
        effect: PassiveEffect {
            description: "Conditional: スライムへのDMG+40-80%",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const HALBERD: WeaponData = WeaponData {
    id: "halberd",
    name: "Halberd",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star3,
    base_atk: [40.0, 396.0, 415.0, 448.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.051, 0.214, 0.214, 0.235])),
    passive: Some(WeaponPassive {
        name: "鉤戟",
        effect: PassiveEffect {
            description: "Conditional: 通常攻撃命中時に追加ATKダメージ、10秒に1回",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const WHITE_TASSEL: WeaponData = WeaponData {
    id: "white_tassel",
    name: "White Tassel",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star3,
    base_atk: [39.0, 355.0, 375.0, 401.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.051, 0.214, 0.214, 0.234])),
    passive: Some(WeaponPassive {
        name: "白纓の槍",
        effect: PassiveEffect {
            description: "NA DMG+24-48%",
            buffs: &[StatBuff {
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.24,
                refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
            }],
            conditional_buffs: &[],
        },
    }),
};

// =============================================================================
// 1-2 Star Polearms
// =============================================================================

pub const BEGINNERS_PROTECTOR: WeaponData = WeaponData {
    id: "beginners_protector",
    name: "Beginner's Protector",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star1,
    base_atk: [23.0, 185.0, 185.0, 185.0],
    sub_stat: None,
    passive: None,
};

pub const IRON_POINT: WeaponData = WeaponData {
    id: "iron_point",
    name: "Iron Point",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star2,
    base_atk: [33.0, 243.0, 243.0, 243.0],
    sub_stat: None,
    passive: None,
};

/// All polearm weapon data references.
pub const ALL_POLEARMS: &[&WeaponData] = &[
    // 5-Star
    &BLOODSOAKED_RUINS,
    &CALAMITY_QUELLER,
    &CRIMSON_MOONS_SEMBLANCE,
    &ENGULFING_LIGHTNING,
    &FRACTURED_HALO,
    &LUMIDOUCE_ELEGY,
    &PRIMORDIAL_JADE_WINGED_SPEAR,
    &SKYWARD_SPINE,
    &STAFF_OF_HOMA,
    &STAFF_OF_THE_SCARLET_SANDS,
    &SYMPHONIST_OF_SCENTS,
    &VORTEX_VANQUISHER,
    // 4-Star
    &BALLAD_OF_THE_FJORDS,
    &CRESCENT_PIKE,
    &DEATHMATCH,
    &DIALOGUES_OF_THE_DESERT_SAGES,
    &DRAGONS_BANE,
    &DRAGONSPINE_SPEAR,
    &FAVONIUS_LANCE,
    &FOOTPRINT_OF_THE_RAINBOW,
    &KITAIN_CROSS_SPEAR,
    &LITHIC_SPEAR,
    &MISSIVE_WINDSPEAR,
    &MOONPIERCER,
    &MOUNTAIN_BRACING_BOLT,
    &PROSPECTORS_DRILL,
    &PROSPECTORS_SHOVEL,
    &PROTOTYPE_STARGLITTER,
    &RIGHTFUL_REWARD,
    &ROYAL_SPEAR,
    &SACRIFICERS_STAFF,
    &TAMAYURATEI_NO_OHANASHI,
    &THE_CATCH,
    &WAVEBREAKERS_FIN,
    // 3-Star
    &BLACK_TASSEL,
    &HALBERD,
    &WHITE_TASSEL,
    // 1-2 Star
    &BEGINNERS_PROTECTOR,
    &IRON_POINT,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buff::AutoCondition;

    #[test]
    fn engulfing_lightning_has_er_atk_conditional() {
        let passive = ENGULFING_LIGHTNING.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "engulfing_er_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.28).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::EnergyRecharge,
                offset: Some(_),
                cap: Some(_),
            })
        ));
    }

    #[test]
    fn engulfing_lightning_has_er_atk_and_burst_er() {
        let passive = ENGULFING_LIGHTNING.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        // Primary unchanged
        assert_eq!(cond_buffs[0].name, "engulfing_er_atk");

        // Secondary: burst ER toggle
        let buff2 = &cond_buffs[1];
        assert_eq!(buff2.name, "engulfing_burst_er");
        assert_eq!(buff2.stat, BuffableStat::EnergyRecharge);
        assert!((buff2.value - 0.30).abs() < 1e-6);
        assert!(buff2.refinement_values.is_some());
        let rv = buff2.refinement_values.unwrap();
        assert!((rv[0] - 0.30).abs() < 1e-6);
        assert!((rv[4] - 0.50).abs() < 1e-6);
        assert!(matches!(
            buff2.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn staff_of_scarlet_sands_has_em_atk_conditional() {
        let passive = STAFF_OF_THE_SCARLET_SANDS.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        // Primary: EM → ATK flat (Auto StatScaling)
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "scarlet_sands_em_atk");
        assert_eq!(buff.stat, BuffableStat::AtkFlat);
        assert!((buff.value - 0.52).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::ElementalMastery,
                offset: None,
                cap: None,
            })
        ));

        // Secondary: EM → ATK flat × stacks (Both StatScaling + Stacks)
        let buff2 = &cond_buffs[1];
        assert_eq!(buff2.name, "scarlet_sands_skill_stacks");
        assert_eq!(buff2.stat, BuffableStat::AtkFlat);
        assert!((buff2.value - 0.28).abs() < 1e-6);
        assert!(buff2.refinement_values.is_some());
        assert!(matches!(
            buff2.activation,
            Activation::Both(
                AutoCondition::StatScaling {
                    stat: BuffableStat::ElementalMastery,
                    offset: None,
                    cap: None,
                },
                ManualCondition::Stacks(2),
            )
        ));
    }

    #[test]
    fn pjws_has_atk_stacks_and_full_stack_dmg() {
        let passive = PRIMORDIAL_JADE_WINGED_SPEAR.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        let stacks_buff = &cond_buffs[0];
        assert_eq!(stacks_buff.name, "pjws_atk_stacks");
        assert_eq!(stacks_buff.stat, BuffableStat::AtkPercent);
        assert!((stacks_buff.value - 0.032).abs() < 1e-6);
        assert!(matches!(
            stacks_buff.activation,
            Activation::Manual(ManualCondition::Stacks(6))
        ));
        assert!(stacks_buff.refinement_values.is_some());

        let full_buff = &cond_buffs[1];
        assert_eq!(full_buff.name, "pjws_full_stack_dmg");
        assert_eq!(full_buff.stat, BuffableStat::DmgBonus);
        assert!((full_buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            full_buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn vortex_vanquisher_has_atk_stacks_and_shield_stacks() {
        let passive = VORTEX_VANQUISHER.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "vortex_vanquisher_atk_stacks");
        assert_eq!(cond_buffs[1].name, "vortex_vanquisher_shield_atk_stacks");

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
    fn lithic_spear_has_region_conditionals() {
        let passive = LITHIC_SPEAR.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        let atk = &cond_buffs[0];
        assert_eq!(atk.name, "lithic_spear_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert!((atk.value - 0.07).abs() < 1e-6);
        assert!(matches!(
            atk.activation,
            Activation::Auto(AutoCondition::TeamRegionCount {
                region: crate::types::Region::Liyue
            })
        ));

        let cr = &cond_buffs[1];
        assert_eq!(cr.name, "lithic_spear_crit");
        assert_eq!(cr.stat, BuffableStat::CritRate);
        assert!((cr.value - 0.03).abs() < 1e-6);
    }
}
