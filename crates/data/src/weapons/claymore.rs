use crate::buff::{
    Activation, AutoCondition, BuffTarget, BuffableStat, ConditionalBuff, ManualCondition,
    PassiveEffect, StatBuff,
};
use crate::types::{Rarity, Region, WeaponData, WeaponPassive, WeaponSubStat, WeaponType};

// =============================================================================
// 5-Star Claymores
// =============================================================================

pub const A_THOUSAND_BLAZING_SUNS: WeaponData = WeaponData {
    id: "a_thousand_blazing_suns",
    name: "A Thousand Blazing Suns",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    base_atk: [49.0, 648.0, 679.0, 741.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.024, 0.101, 0.101, 0.110])),
    passive: Some(WeaponPassive {
        name: "A Thousand Blazing Suns",
        effect: PassiveEffect {
            description: "重撃命中→ATK+28-56%。HP減少→CRIT DMG+20-40%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "thousand_suns_atk",
                    description: "重撃命中時にATK+28-56%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.28,
                    refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "thousand_suns_critdmg",
                    description: "HP減少時にCRIT DMG+20-40%",
                    stat: BuffableStat::CritDmg,
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

pub const BEACON_OF_THE_REED_SEA: WeaponData = WeaponData {
    id: "beacon_of_the_reed_sea",
    name: "Beacon of the Reed Sea",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.072, 0.302, 0.302, 0.331])),
    passive: Some(WeaponPassive {
        name: "Beacon of the Reed Sea",
        effect: PassiveEffect {
            description: "スキル命中後ATK+20-40%、被弾後ATK+20-40%、シールド時HP+32-64%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "beacon_skill_atk",
                    description: "元素スキル命中後にATK+20-40%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "beacon_dmg_taken_atk",
                    description: "ダメージを受けた後にATK+20-40%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "beacon_shield_hp",
                    description: "シールド保護時にHP+32-64%",
                    stat: BuffableStat::HpPercent,
                    value: 0.32,
                    refinement_values: Some([0.32, 0.40, 0.48, 0.56, 0.64]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const FANG_OF_THE_MOUNTAIN_KING: WeaponData = WeaponData {
    id: "fang_of_the_mountain_king",
    name: "Fang of the Mountain King",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    base_atk: [49.0, 648.0, 679.0, 741.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.024, 0.101, 0.101, 0.110])),
    passive: Some(WeaponPassive {
        name: "Fang of the Mountain King",
        effect: PassiveEffect {
            description: "元素スキル命中でDMG+10-20%スタック（最大3スタック）。DmgBonus近似値（実際は全元素DMG、物理除外）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "fang_mountain_king_elemental_dmg_stacks",
                description: "元素スキル命中でDMG+10-20%（1スタック）、最大3スタック（DmgBonus近似値、物理除外）",
                stat: BuffableStat::DmgBonus,
                value: 0.10,
                refinement_values: Some([0.10, 0.125, 0.15, 0.175, 0.20]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
        },
    }),
};

pub const GEST_OF_THE_MIGHTY_WOLF: WeaponData = WeaponData {
    id: "gest_of_the_mighty_wolf",
    name: "Gest of the Mighty Wolf",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    base_atk: [45.94, 121.73, 532.23, 608.07],
    sub_stat: Some(WeaponSubStat::CritRate([0.072, 0.1272, 0.3017, 0.3308])),
    passive: Some(WeaponPassive {
        name: "不屈の騎士道",
        effect: PassiveEffect {
            description: "ATK SPD+10%。通常/スキル/重撃でFour Winds' Hymnスタック獲得(最大4)。\
                スタックごとにDMG+7.5-15.5%。Hexerei: Secret Rite時、スタックごとにCRIT DMGも同値増加",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "gest_mighty_wolf_hymn_dmg",
                    description: "Four Winds' Hymn: スタックごとにDMG+7.5-15.5%（最大4スタック）",
                    stat: BuffableStat::DmgBonus,
                    value: 0.075,
                    refinement_values: Some([0.075, 0.095, 0.115, 0.135, 0.155]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(4)),
                },
                ConditionalBuff {
                    name: "gest_mighty_wolf_hymn_hexerei_crit",
                    description: "Hexerei: Secret Rite時、Four Winds' HymnスタックごとにCRIT DMG+7.5-15.5%",
                    stat: BuffableStat::CritDmg,
                    value: 0.075,
                    refinement_values: Some([0.075, 0.095, 0.115, 0.135, 0.155]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(4)),
                },
            ],
        },
    }),
};

pub const REDHORN_STONETHRESHER: WeaponData = WeaponData {
    id: "redhorn_stonethresher",
    name: "Redhorn Stonethresher",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    base_atk: [44.0, 475.0, 506.0, 542.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.192, 0.804, 0.804, 0.882])),
    passive: Some(WeaponPassive {
        name: "赤角の石塵滅砕",
        effect: PassiveEffect {
            description: "DEF+28-56%。通常攻撃と重撃にDEF基準の追加ダメージ",
            buffs: &[StatBuff {
                stat: BuffableStat::DefPercent,
                value: 0.28,
                refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "redhorn_def_normal_flat",
                    description: "DEF×40-80%分を通常攻撃フラットダメージに加算",
                    stat: BuffableStat::NormalAtkFlatDmg,
                    value: 0.40,
                    refinement_values: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::DefPercent,
                        offset: None,
                        cap: None,
                    }),
                },
                ConditionalBuff {
                    name: "redhorn_def_charged_flat",
                    description: "DEF×40-80%分を重撃フラットダメージに加算",
                    stat: BuffableStat::ChargedAtkFlatDmg,
                    value: 0.40,
                    refinement_values: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::DefPercent,
                        offset: None,
                        cap: None,
                    }),
                },
            ],
        },
    }),
};

pub const SKYWARD_PRIDE: WeaponData = WeaponData {
    id: "skyward_pride",
    name: "Skyward Pride",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.080, 0.335, 0.335, 0.368])),
    passive: Some(WeaponPassive {
        name: "天空の矜持",
        effect: PassiveEffect {
            description: "DMG+8-16%。元素爆発後に真空の刃で追加ダメージ",
            buffs: &[StatBuff {
                stat: BuffableStat::DmgBonus,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
            }],
            conditional_buffs: &[],
        },
    }),
};

pub const SONG_OF_BROKEN_PINES: WeaponData = WeaponData {
    id: "song_of_broken_pines",
    name: "Song of Broken Pines",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    base_atk: [49.0, 648.0, 679.0, 741.0],
    sub_stat: Some(WeaponSubStat::PhysicalDmgBonus([
        0.045, 0.189, 0.189, 0.207,
    ])),
    passive: Some(WeaponPassive {
        name: "千年の大楽章・揺らぎ無き心",
        effect: PassiveEffect {
            description: "ATK+16-32%。印4蓄積でチーム全員ATK+20-40%/通常ATK速度アップ",
            buffs: &[StatBuff {
                stat: BuffableStat::AtkPercent,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "song_broken_pines_team_atk",
                    description: "印4蓄積時にチーム全員ATK+20-40%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                    stack_values: None,
                    target: BuffTarget::Team,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "song_broken_pines_team_normal_dmg",
                    description: "印4蓄積時にチーム全員通常攻撃DMG+12-24%",
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::Team,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const THE_UNFORGED: WeaponData = WeaponData {
    id: "the_unforged",
    name: "The Unforged",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.108, 0.453, 0.453, 0.496])),
    passive: Some(WeaponPassive {
        name: "金璋の剣",
        effect: PassiveEffect {
            description: "攻撃命中でATK+4-8%スタック（最大5）、シールド時は2倍",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "the_unforged_atk_stacks",
                    description: "攻撃命中でATK+4-8%（1スタック）、最大5スタック",
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(5)),
                },
                ConditionalBuff {
                    name: "the_unforged_shield_atk_stacks",
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

pub const VERDICT: WeaponData = WeaponData {
    id: "verdict",
    name: "Verdict",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.048, 0.201, 0.201, 0.221])),
    passive: Some(WeaponPassive {
        name: "裁定",
        effect: PassiveEffect {
            description: "Skill DMG+18-36%。シールド保護中スタック獲得でさらにSkill DMG+18-36%/スタック（最大2）",
            buffs: &[StatBuff {
                stat: BuffableStat::SkillDmgBonus,
                value: 0.18,
                refinement_values: Some([0.18, 0.225, 0.27, 0.315, 0.36]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "verdict_skill_dmg_stacks",
                description: "シールド保護中スタック毎にSkill DMG+18-36%（最大2スタック）",
                stat: BuffableStat::SkillDmgBonus,
                value: 0.18,
                refinement_values: Some([0.18, 0.225, 0.27, 0.315, 0.36]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(2)),
            }],
        },
    }),
};

pub const WOLFS_GRAVESTONE: WeaponData = WeaponData {
    id: "wolfs_gravestone",
    name: "狼の末路",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.108, 0.453, 0.453, 0.496])),
    passive: Some(WeaponPassive {
        name: "止めの一撃",
        effect: PassiveEffect {
            description: "ATK+20-40%。HP30%以下の敵を攻撃するとチーム全員ATK+40-80%、12秒、30秒に1回",
            buffs: &[StatBuff {
                stat: BuffableStat::AtkPercent,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "wolfs_gravestone_team_atk",
                description: "HP30%以下の敵に命中時、チーム全員ATK+40-80%（12秒）",
                stat: BuffableStat::AtkPercent,
                value: 0.40,
                refinement_values: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

// =============================================================================
// 4-Star Claymores
// =============================================================================

pub const AKUOUMARU: WeaponData = WeaponData {
    id: "akuoumaru",
    name: "Akuoumaru",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "曚雲の海鯨波",
        effect: PassiveEffect {
            description: "チーム総エネルギー上限に応じてBurst DMGアップ（最大Burst DMG+40-80%、総EP280超時）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "akuoumaru_burst_dmg",
                description: "チーム総エネルギー上限に応じてBurst DMG+最大40-80%（280EP基準）",
                stat: BuffableStat::BurstDmgBonus,
                value: 0.40,
                refinement_values: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const BLACKCLIFF_SLASHER: WeaponData = WeaponData {
    id: "blackcliff_slasher",
    name: "Blackcliff Slasher",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::CritDmg([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "追撃の一撃",
        effect: PassiveEffect {
            description: "敵を倒すとATK+12-24%（1スタック）、30秒、最大3スタック",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "blackcliff_slasher_atk_stacks",
                description: "敵撃破毎にATK+12-24%（1スタック）、最大3スタック",
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

pub const EARTH_SHAKER: WeaponData = WeaponData {
    id: "earth_shaker",
    name: "Earth Shaker",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Earth Shaker",
        effect: PassiveEffect {
            description: "Conditional: 元素スキル命中で元素爆発ダメージアップ",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "earth_shaker_burst_dmg",
                description: "元素スキル命中後にBurst DMG+16-32%",
                stat: BuffableStat::BurstDmgBonus,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const FAVONIUS_GREATSWORD: WeaponData = WeaponData {
    id: "favonius_greatsword",
    name: "Favonius Greatsword",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.133, 0.557, 0.557, 0.613])),
    passive: Some(WeaponPassive {
        name: "西風の息吹",
        effect: PassiveEffect {
            description: "Conditional: 会心命中時に元素粒子を生成、12-6秒に1回",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const FLAME_FORGED_INSIGHT: WeaponData = WeaponData {
    id: "flame_forged_insight",
    name: "Flame-Forged Insight",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([36.0, 151.0, 151.0, 165.0])),
    passive: Some(WeaponPassive {
        name: "Flame-Forged Insight",
        effect: PassiveEffect {
            description: "元素反応を発生させるとDMG+16-32%（12秒）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "flame_forged_insight_dmg",
                description: "元素反応発生後にDMG+16-32%",
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

pub const FOREST_REGALIA: WeaponData = WeaponData {
    id: "forest_regalia",
    name: "Forest Regalia",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "森林の瑞祥",
        effect: PassiveEffect {
            description: "草元素反応発動後に種を生成。種を取得するとEM+60-120（12秒）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "forest_regalia_em",
                description: "草元素反応後の種取得でEM+60-120",
                stat: BuffableStat::ElementalMastery,
                value: 60.0,
                refinement_values: Some([60.0, 75.0, 90.0, 105.0, 120.0]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const FRUITFUL_HOOK: WeaponData = WeaponData {
    id: "fruitful_hook",
    name: "Fruitful Hook",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Fruitful Hook",
        effect: PassiveEffect {
            description: "Conditional: 落下攻撃命中で追加ダメージ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const KATSURAGIKIRI_NAGAMASA: WeaponData = WeaponData {
    id: "katsuragikiri_nagamasa",
    name: "Katsuragikiri Nagamasa",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.100, 0.419, 0.419, 0.459])),
    passive: Some(WeaponPassive {
        name: "名刀・廻りの灯",
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

pub const LITHIC_BLADE: WeaponData = WeaponData {
    id: "lithic_blade",
    name: "Lithic Blade",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "千岩の刃",
        effect: PassiveEffect {
            description: "Conditional: チーム内の璃月キャラ人数に応じてATK/CRIT Rateアップ",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "lithic_blade_atk",
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
                    name: "lithic_blade_crit",
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

pub const LUXURIOUS_SEA_LORD: WeaponData = WeaponData {
    id: "luxurious_sea_lord",
    name: "Luxurious Sea-Lord",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "海獣の宴",
        effect: PassiveEffect {
            description: "Burst DMG+12-24%。元素爆発命中時にマグロを召喚して追加ダメージ",
            buffs: &[StatBuff {
                stat: BuffableStat::BurstDmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[],
        },
    }),
};

pub const MAILED_FLOWER: WeaponData = WeaponData {
    id: "mailed_flower",
    name: "Mailed Flower",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([24.0, 101.0, 101.0, 110.0])),
    passive: Some(WeaponPassive {
        name: "Mailed Flower",
        effect: PassiveEffect {
            description: "元素スキル/元素爆発命中後にATK+12-24%/EM+48-96（8秒）",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "mailed_flower_atk",
                    description: "スキル/爆発命中後にATK+12-24%",
                    stat: BuffableStat::AtkPercent,
                    value: 0.12,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "mailed_flower_em",
                    description: "スキル/爆発命中後にEM+48-96",
                    stat: BuffableStat::ElementalMastery,
                    value: 48.0,
                    refinement_values: Some([48.0, 60.0, 72.0, 84.0, 96.0]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
        },
    }),
};

pub const MAKHAIRA_AQUAMARINE: WeaponData = WeaponData {
    id: "makhaira_aquamarine",
    name: "Makhaira Aquamarine",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([36.0, 151.0, 151.0, 165.0])),
    passive: Some(WeaponPassive {
        name: "砂海の渡し守",
        effect: PassiveEffect {
            description: "EM×0.024-0.048%分のATK(フラット)を自身に付与。その30%をチームメンバーにも付与",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "makhaira_self_atk",
                    description: "EM×0.024-0.048%分のATK(フラット)を自身に付与",
                    stat: BuffableStat::AtkFlat,
                    value: 0.00024,
                    refinement_values: Some([0.00024, 0.00030, 0.00036, 0.00042, 0.00048]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::StatScaling {
                        stat: BuffableStat::ElementalMastery,
                        offset: None,
                        cap: None,
                    }),
                },
                ConditionalBuff {
                    name: "makhaira_team_atk",
                    description: "自身のATK付与量の30%をチームメンバーに付与",
                    stat: BuffableStat::AtkFlat,
                    value: 0.000072,
                    refinement_values: Some([0.000072, 0.000090, 0.000108, 0.000126, 0.000144]),
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

pub const MASTER_KEY: WeaponData = WeaponData {
    id: "master_key",
    name: "Master Key",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.133, 0.557, 0.557, 0.613])),
    passive: Some(WeaponPassive {
        name: "Master Key",
        effect: PassiveEffect {
            description: "Conditional: 元素スキル使用後にEMアップ",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "master_key_em",
                description: "元素スキル命中後にEM+36-72",
                stat: BuffableStat::ElementalMastery,
                value: 36.0,
                refinement_values: Some([36.0, 45.0, 54.0, 63.0, 72.0]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const PORTABLE_POWER_SAW: WeaponData = WeaponData {
    id: "portable_power_saw",
    name: "Portable Power Saw",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [41.0, 401.0, 427.0, 454.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.120, 0.503, 0.503, 0.551])),
    passive: Some(WeaponPassive {
        name: "Portable Power Saw",
        effect: PassiveEffect {
            description: "アークアイテム消費毎にEM+40-80（1スタック）、最大3スタック",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "portable_power_saw_em_stacks",
                description: "アークアイテム消費毎にEM+40-80（1スタック）、最大3スタック",
                stat: BuffableStat::ElementalMastery,
                value: 40.0,
                refinement_values: Some([40.0, 50.0, 60.0, 70.0, 80.0]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            }],
        },
    }),
};

pub const PROTOTYPE_ARCHAIC: WeaponData = WeaponData {
    id: "prototype_archaic",
    name: "Prototype Archaic",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "砕石",
        effect: PassiveEffect {
            description: "Conditional: 通常/重撃命中時に追加ATK範囲ダメージ、15秒に1回",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const RAINSLASHER: WeaponData = WeaponData {
    id: "rainslasher",
    name: "Rainslasher",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([36.0, 151.0, 151.0, 165.0])),
    passive: Some(WeaponPassive {
        name: "雷電の止め",
        effect: PassiveEffect {
            description: "水/雷元素影響下の敵へのDMG+20-36%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "rainslasher_dmg",
                description: "水/雷元素影響下の敵に対してDMG+20-36%",
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

pub const ROYAL_GREATSWORD: WeaponData = WeaponData {
    id: "royal_greatsword",
    name: "Royal Greatsword",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "集中",
        effect: PassiveEffect {
            description: "ダメージを与える毎にCRIT Rate+8-16%（1スタック）、最大5スタック。会心発生でリセット",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "royal_greatsword_crit_rate_stacks",
                description: "ダメージ毎にCRIT Rate+8-16%（1スタック）、最大5スタック",
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

pub const SACRIFICIAL_GREATSWORD: WeaponData = WeaponData {
    id: "sacrificial_greatsword",
    name: "Sacrificial Greatsword",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "祭礼",
        effect: PassiveEffect {
            description: "Conditional: スキルがダメージを与えるとCD即リセット、30-16秒に1回",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const SERPENT_SPINE: WeaponData = WeaponData {
    id: "serpent_spine",
    name: "Serpent Spine",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "波乱",
        effect: PassiveEffect {
            description: "4秒毎にDMG+6-10%スタック（最大5スタック）。ダメージを受けるとスタック減少",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "serpent_spine_dmg",
                description: "フィールド上で4秒毎にDMG+6-10%（1スタック分）、最大5スタック",
                stat: BuffableStat::DmgBonus,
                value: 0.06,
                refinement_values: Some([0.06, 0.07, 0.08, 0.09, 0.10]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(5)),
            }],
        },
    }),
};

pub const SNOW_TOMBED_STARSILVER: WeaponData = WeaponData {
    id: "snow_tombed_starsilver",
    name: "Snow-Tombed Starsilver",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::PhysicalDmgBonus([
        0.075, 0.314, 0.314, 0.345,
    ])),
    passive: Some(WeaponPassive {
        name: "霜葬の星銀",
        effect: PassiveEffect {
            description: "Conditional: 通常/重撃命中時に氷柱落下、氷元素影響下の敵には追加ダメージ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const TALKING_STICK: WeaponData = WeaponData {
    id: "talking_stick",
    name: "Talking Stick",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.040, 0.168, 0.168, 0.184])),
    passive: Some(WeaponPassive {
        name: "Talking Stick",
        effect: PassiveEffect {
            description: "炎/水/氷/雷反応→Ele DMG+16-32%（炎/水/氷/雷）。風/岩/草反応→ATK+20-40%",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "talking_stick_ele_dmg",
                    description: "炎/水/氷/雷反応発生時に該当元素DMG+16-32%",
                    stat: BuffableStat::DmgBonus,
                    value: 0.16,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "talking_stick_atk",
                    description: "風/岩/草反応発生時にATK+20-40%",
                    stat: BuffableStat::AtkPercent,
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

pub const THE_BELL: WeaponData = WeaponData {
    id: "the_bell",
    name: "The Bell",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "反響の金鐘",
        effect: PassiveEffect {
            description: "ダメージを受けるとシールド生成（45秒CD）。シールド保護中にDMG+12-24%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "the_bell_shield_dmg",
                description: "シールド保護中にDMG+12-24%",
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

pub const TIDAL_SHADOW: WeaponData = WeaponData {
    id: "tidal_shadow",
    name: "Tidal Shadow",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.090, 0.377, 0.377, 0.413])),
    passive: Some(WeaponPassive {
        name: "Tidal Shadow",
        effect: PassiveEffect {
            description: "HP回復時にATK+24-48%（8秒）",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "tidal_shadow_atk",
                description: "HP回復時にATK+24-48%",
                stat: BuffableStat::AtkPercent,
                value: 0.24,
                refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const ULTIMATE_OVERLORDS_MEGA_MAGIC_SWORD: WeaponData = WeaponData {
    id: "ultimate_overlords_mega_magic_sword",
    name: "Ultimate Overlord's Mega Magic Sword",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "魔王の究極の一撃",
        effect: PassiveEffect {
            description: "チーム内に異なる元素タイプ1つにつきATK+4-8%/DMG+3.5-7%（最大4タイプ）",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "ultimate_sword_atk",
                    description: "チーム内の異なる元素タイプ1つにつきATK+4-8%（最大4タイプ）",
                    stat: BuffableStat::AtkPercent,
                    value: 0.04,
                    refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount {
                        min_count: 1,
                    }),
                },
                ConditionalBuff {
                    name: "ultimate_sword_dmg",
                    description: "チーム内の異なる元素タイプ1つにつきDMG+3.5-7%（最大4タイプ）",
                    stat: BuffableStat::DmgBonus,
                    value: 0.035,
                    refinement_values: Some([0.035, 0.044, 0.052, 0.061, 0.070]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamDiffElementCount {
                        min_count: 1,
                    }),
                },
            ],
        },
    }),
};

pub const WHITEBLIND: WeaponData = WeaponData {
    id: "whiteblind",
    name: "Whiteblind",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star4,
    base_atk: [42.0, 449.0, 475.0, 510.0],
    sub_stat: Some(WeaponSubStat::DefPercent([0.113, 0.473, 0.473, 0.517])),
    passive: Some(WeaponPassive {
        name: "精錬の極み",
        effect: PassiveEffect {
            description: "通常/重撃命中でATK+6-12%/DEF+6-12%（1スタック分）、最大4スタック",
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "whiteblind_atk",
                    description: "通常/重撃命中でATK+6-12%（1スタック）、最大4スタック",
                    stat: BuffableStat::AtkPercent,
                    value: 0.06,
                    refinement_values: Some([0.06, 0.075, 0.09, 0.105, 0.12]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(4)),
                },
                ConditionalBuff {
                    name: "whiteblind_def",
                    description: "通常/重撃命中でDEF+6-12%（1スタック）、最大4スタック",
                    stat: BuffableStat::DefPercent,
                    value: 0.06,
                    refinement_values: Some([0.06, 0.075, 0.09, 0.105, 0.12]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(4)),
                },
            ],
        },
    }),
};

// =============================================================================
// 3-Star Claymores
// =============================================================================

pub const BLOODTAINTED_GREATSWORD: WeaponData = WeaponData {
    id: "bloodtainted_greatsword",
    name: "Bloodtainted Greatsword",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star3,
    base_atk: [38.0, 314.0, 334.0, 354.0],
    sub_stat: Some(WeaponSubStat::ElementalMastery([41.0, 171.0, 171.0, 187.0])),
    passive: Some(WeaponPassive {
        name: "炎と雷の渇き",
        effect: PassiveEffect {
            description: "炎/雷元素影響下の敵へのDMG+12-24%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "bloodtainted_dmg",
                description: "炎/雷元素影響下の敵に対してDMG+12-24%",
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

pub const DEBATE_CLUB: WeaponData = WeaponData {
    id: "debate_club",
    name: "Debate Club",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star3,
    base_atk: [39.0, 355.0, 375.0, 401.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.077, 0.323, 0.323, 0.352])),
    passive: Some(WeaponPassive {
        name: "議論",
        effect: PassiveEffect {
            description: "Conditional: スキル使用後に通常/重撃で追加ATKダメージ、15秒に1回",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const FERROUS_SHADOW: WeaponData = WeaponData {
    id: "ferrous_shadow",
    name: "Ferrous Shadow",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star3,
    base_atk: [39.0, 355.0, 375.0, 401.0],
    sub_stat: Some(WeaponSubStat::HpPercent([0.077, 0.323, 0.323, 0.352])),
    passive: Some(WeaponPassive {
        name: "鉄の影",
        effect: PassiveEffect {
            description: "HP70%以下の時に重撃の中断耐性上昇、重撃DMG+30-50%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "ferrous_shadow_charged_dmg",
                description: "HP70%以下の時に重撃DMG+30-50%",
                stat: BuffableStat::ChargedAtkDmgBonus,
                value: 0.30,
                refinement_values: Some([0.30, 0.35, 0.40, 0.45, 0.50]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
        },
    }),
};

pub const SKYRIDER_GREATSWORD: WeaponData = WeaponData {
    id: "skyrider_greatsword",
    name: "Skyrider Greatsword",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star3,
    base_atk: [39.0, 355.0, 375.0, 401.0],
    sub_stat: Some(WeaponSubStat::PhysicalDmgBonus([
        0.096, 0.402, 0.402, 0.439,
    ])),
    passive: Some(WeaponPassive {
        name: "勇気",
        effect: PassiveEffect {
            description: "通常/重撃命中でATK+6-10%（1スタック）、6秒、最大4スタック",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "skyrider_greatsword_atk_stacks",
                description: "通常/重撃命中毎にATK+6-10%（1スタック）、最大4スタック",
                stat: BuffableStat::AtkPercent,
                value: 0.06,
                refinement_values: Some([0.06, 0.07, 0.08, 0.09, 0.10]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(4)),
            }],
        },
    }),
};

pub const WHITE_IRON_GREATSWORD: WeaponData = WeaponData {
    id: "white_iron_greatsword",
    name: "White Iron Greatsword",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star3,
    base_atk: [39.0, 355.0, 375.0, 401.0],
    sub_stat: Some(WeaponSubStat::DefPercent([0.096, 0.402, 0.402, 0.439])),
    passive: Some(WeaponPassive {
        name: "白鉄の意志",
        effect: PassiveEffect {
            description: "Conditional: 敵を倒すとHP回復",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

// =============================================================================
// 1-2 Star Claymores
// =============================================================================

pub const WASTER_GREATSWORD: WeaponData = WeaponData {
    id: "waster_greatsword",
    name: "Waster Greatsword",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star1,
    base_atk: [23.0, 185.0, 185.0, 185.0],
    sub_stat: None,
    passive: None,
};

pub const OLD_MERCS_PAL: WeaponData = WeaponData {
    id: "old_mercs_pal",
    name: "Old Merc's Pal",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star2,
    base_atk: [33.0, 243.0, 243.0, 243.0],
    sub_stat: None,
    passive: None,
};

/// All claymore weapon data references.
pub const ALL_CLAYMORES: &[&WeaponData] = &[
    // 5-Star
    &A_THOUSAND_BLAZING_SUNS,
    &BEACON_OF_THE_REED_SEA,
    &FANG_OF_THE_MOUNTAIN_KING,
    &GEST_OF_THE_MIGHTY_WOLF,
    &REDHORN_STONETHRESHER,
    &SKYWARD_PRIDE,
    &SONG_OF_BROKEN_PINES,
    &THE_UNFORGED,
    &VERDICT,
    &WOLFS_GRAVESTONE,
    // 4-Star
    &AKUOUMARU,
    &BLACKCLIFF_SLASHER,
    &EARTH_SHAKER,
    &FAVONIUS_GREATSWORD,
    &FLAME_FORGED_INSIGHT,
    &FOREST_REGALIA,
    &FRUITFUL_HOOK,
    &KATSURAGIKIRI_NAGAMASA,
    &LITHIC_BLADE,
    &LUXURIOUS_SEA_LORD,
    &MAILED_FLOWER,
    &MAKHAIRA_AQUAMARINE,
    &MASTER_KEY,
    &PORTABLE_POWER_SAW,
    &PROTOTYPE_ARCHAIC,
    &RAINSLASHER,
    &ROYAL_GREATSWORD,
    &SACRIFICIAL_GREATSWORD,
    &SERPENT_SPINE,
    &SNOW_TOMBED_STARSILVER,
    &TALKING_STICK,
    &THE_BELL,
    &TIDAL_SHADOW,
    &ULTIMATE_OVERLORDS_MEGA_MAGIC_SWORD,
    &WHITEBLIND,
    // 3-Star
    &BLOODTAINTED_GREATSWORD,
    &DEBATE_CLUB,
    &FERROUS_SHADOW,
    &SKYRIDER_GREATSWORD,
    &WHITE_IRON_GREATSWORD,
    // 1-2 Star
    &WASTER_GREATSWORD,
    &OLD_MERCS_PAL,
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::buff::AutoCondition;

    #[test]
    fn redhorn_stonethresher_has_def_flatdmg_conditionals() {
        let passive = REDHORN_STONETHRESHER.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);
        assert_eq!(cond_buffs[0].name, "redhorn_def_normal_flat");
        assert_eq!(cond_buffs[0].stat, BuffableStat::NormalAtkFlatDmg);
        assert!((cond_buffs[0].value - 0.40).abs() < 1e-6);
        assert_eq!(cond_buffs[1].name, "redhorn_def_charged_flat");
        assert_eq!(cond_buffs[1].stat, BuffableStat::ChargedAtkFlatDmg);
        assert!((cond_buffs[1].value - 0.40).abs() < 1e-6);
        for buff in cond_buffs {
            assert!(matches!(
                buff.activation,
                Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::DefPercent,
                    offset: None,
                    cap: None,
                })
            ));
        }
    }

    #[test]
    fn fang_of_mountain_king_has_dmg_stacks() {
        let passive = FANG_OF_THE_MOUNTAIN_KING.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "fang_mountain_king_elemental_dmg_stacks");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.10).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        assert!(buff.refinement_values.is_some());
    }

    #[test]
    fn beacon_reed_sea_has_three_toggles() {
        let passive = BEACON_OF_THE_REED_SEA.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 3);

        assert_eq!(cond_buffs[0].name, "beacon_skill_atk");
        assert_eq!(cond_buffs[0].stat, BuffableStat::AtkPercent);
        assert!((cond_buffs[0].value - 0.20).abs() < 1e-6);

        assert_eq!(cond_buffs[1].name, "beacon_dmg_taken_atk");
        assert_eq!(cond_buffs[1].stat, BuffableStat::AtkPercent);
        assert!((cond_buffs[1].value - 0.20).abs() < 1e-6);

        assert_eq!(cond_buffs[2].name, "beacon_shield_hp");
        assert_eq!(cond_buffs[2].stat, BuffableStat::HpPercent);
        assert!((cond_buffs[2].value - 0.32).abs() < 1e-6);

        for buff in cond_buffs {
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Toggle)
            ));
        }
    }

    #[test]
    fn gest_mighty_wolf_has_dmg_and_atk_toggle() {
        let passive = GEST_OF_THE_MIGHTY_WOLF.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "gest_mighty_wolf_hymn_dmg");
        assert_eq!(cond_buffs[0].stat, BuffableStat::DmgBonus);
        assert!((cond_buffs[0].value - 0.075).abs() < 1e-6);

        assert_eq!(cond_buffs[1].name, "gest_mighty_wolf_hymn_hexerei_crit");
        assert_eq!(cond_buffs[1].stat, BuffableStat::CritDmg);
        assert!((cond_buffs[1].value - 0.075).abs() < 1e-6);

        for buff in cond_buffs {
            assert!(matches!(
                buff.activation,
                Activation::Manual(ManualCondition::Stacks(4))
            ));
        }
    }

    #[test]
    fn the_unforged_has_atk_stacks_and_shield_stacks() {
        let passive = THE_UNFORGED.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        assert_eq!(cond_buffs[0].name, "the_unforged_atk_stacks");
        assert_eq!(cond_buffs[1].name, "the_unforged_shield_atk_stacks");

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
    fn earth_shaker_has_burst_dmg_toggle() {
        let passive = EARTH_SHAKER.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "earth_shaker_burst_dmg");
        assert_eq!(buff.stat, BuffableStat::BurstDmgBonus);
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
    fn master_key_has_em_toggle() {
        let passive = MASTER_KEY.passive.unwrap();
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 1);
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "master_key_em");
        assert_eq!(buff.stat, BuffableStat::ElementalMastery);
        assert!((buff.value - 36.0).abs() < 1e-6);
        assert!(buff.refinement_values.is_some());
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 36.0).abs() < 1e-6);
        assert!((rv[4] - 72.0).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn lithic_blade_has_region_conditionals() {
        let passive = LITHIC_BLADE.passive.unwrap();
        assert_eq!(passive.name, "千岩の刃");
        let cond_buffs = passive.effect.conditional_buffs;
        assert_eq!(cond_buffs.len(), 2);

        let atk = &cond_buffs[0];
        assert_eq!(atk.name, "lithic_blade_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert!((atk.value - 0.07).abs() < 1e-6);
        assert!(atk.refinement_values.is_some());
        assert!(matches!(
            atk.activation,
            Activation::Auto(AutoCondition::TeamRegionCount {
                region: crate::types::Region::Liyue
            })
        ));

        let cr = &cond_buffs[1];
        assert_eq!(cr.name, "lithic_blade_crit");
        assert_eq!(cr.stat, BuffableStat::CritRate);
        assert!((cr.value - 0.03).abs() < 1e-6);
        assert!(cr.refinement_values.is_some());
    }

    #[test]
    fn a_thousand_blazing_suns_moved_to_conditionals() {
        let passive = A_THOUSAND_BLAZING_SUNS.passive.unwrap();
        // StatBuff must be empty (moved to conditional)
        assert_eq!(passive.effect.buffs.len(), 0);
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);

        let atk = &cond[0];
        assert_eq!(atk.name, "thousand_suns_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert!((atk.value - 0.28).abs() < 1e-6);
        assert!(matches!(
            atk.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = atk.refinement_values.unwrap();
        assert!((rv[0] - 0.28).abs() < 1e-6);
        assert!((rv[4] - 0.56).abs() < 1e-6);

        let cd = &cond[1];
        assert_eq!(cd.name, "thousand_suns_critdmg");
        assert_eq!(cd.stat, BuffableStat::CritDmg);
        assert!((cd.value - 0.20).abs() < 1e-6);
        assert!(matches!(
            cd.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv2 = cd.refinement_values.unwrap();
        assert!((rv2[0] - 0.20).abs() < 1e-6);
        assert!((rv2[4] - 0.40).abs() < 1e-6);
    }

    #[test]
    fn song_of_broken_pines_has_team_buffs() {
        let passive = SONG_OF_BROKEN_PINES.passive.unwrap();
        // StatBuff for self ATK still present
        assert_eq!(passive.effect.buffs.len(), 1);
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);

        let team_atk = &cond[0];
        assert_eq!(team_atk.name, "song_broken_pines_team_atk");
        assert_eq!(team_atk.stat, BuffableStat::AtkPercent);
        assert!((team_atk.value - 0.20).abs() < 1e-6);
        assert_eq!(team_atk.target, BuffTarget::Team);
        assert!(matches!(
            team_atk.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = team_atk.refinement_values.unwrap();
        assert!((rv[0] - 0.20).abs() < 1e-6);
        assert!((rv[4] - 0.40).abs() < 1e-6);

        let team_normal = &cond[1];
        assert_eq!(team_normal.name, "song_broken_pines_team_normal_dmg");
        assert_eq!(team_normal.stat, BuffableStat::NormalAtkDmgBonus);
        assert!((team_normal.value - 0.12).abs() < 1e-6);
        assert_eq!(team_normal.target, BuffTarget::Team);
        assert!(matches!(
            team_normal.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
    }

    #[test]
    fn verdict_has_skill_dmg_stacks() {
        let passive = VERDICT.passive.unwrap();
        // Base StatBuff still present
        assert_eq!(passive.effect.buffs.len(), 1);
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "verdict_skill_dmg_stacks");
        assert_eq!(buff.stat, BuffableStat::SkillDmgBonus);
        assert!((buff.value - 0.18).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(2))
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.18).abs() < 1e-6);
        assert!((rv[4] - 0.36).abs() < 1e-6);
    }

    #[test]
    fn akuoumaru_has_burst_dmg_toggle() {
        let passive = AKUOUMARU.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "akuoumaru_burst_dmg");
        assert_eq!(buff.stat, BuffableStat::BurstDmgBonus);
        assert!((buff.value - 0.40).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.40).abs() < 1e-6);
        assert!((rv[4] - 0.80).abs() < 1e-6);
    }

    #[test]
    fn blackcliff_slasher_has_atk_stacks() {
        let passive = BLACKCLIFF_SLASHER.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "blackcliff_slasher_atk_stacks");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.12).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.12).abs() < 1e-6);
        assert!((rv[4] - 0.24).abs() < 1e-6);
    }

    #[test]
    fn flame_forged_insight_has_dmg_toggle() {
        let passive = FLAME_FORGED_INSIGHT.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "flame_forged_insight_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.16).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.16).abs() < 1e-6);
        assert!((rv[4] - 0.32).abs() < 1e-6);
    }

    #[test]
    fn forest_regalia_has_em_toggle() {
        let passive = FOREST_REGALIA.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "forest_regalia_em");
        assert_eq!(buff.stat, BuffableStat::ElementalMastery);
        assert!((buff.value - 60.0).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 60.0).abs() < 1e-6);
        assert!((rv[4] - 120.0).abs() < 1e-6);
    }

    #[test]
    fn mailed_flower_has_atk_and_em_toggle() {
        let passive = MAILED_FLOWER.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);

        let atk = &cond[0];
        assert_eq!(atk.name, "mailed_flower_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert!((atk.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            atk.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = atk.refinement_values.unwrap();
        assert!((rv[0] - 0.12).abs() < 1e-6);
        assert!((rv[4] - 0.24).abs() < 1e-6);

        let em = &cond[1];
        assert_eq!(em.name, "mailed_flower_em");
        assert_eq!(em.stat, BuffableStat::ElementalMastery);
        assert!((em.value - 48.0).abs() < 1e-6);
        assert!(matches!(
            em.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv2 = em.refinement_values.unwrap();
        assert!((rv2[0] - 48.0).abs() < 1e-6);
        assert!((rv2[4] - 96.0).abs() < 1e-6);
    }

    #[test]
    fn makhaira_aquamarine_has_em_scaling_atk_self_and_team() {
        let passive = MAKHAIRA_AQUAMARINE.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);

        let self_buff = &cond[0];
        assert_eq!(self_buff.name, "makhaira_self_atk");
        assert_eq!(self_buff.stat, BuffableStat::AtkFlat);
        assert!((self_buff.value - 0.00024).abs() < 1e-9);
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
        assert!((rv[0] - 0.00024).abs() < 1e-9);
        assert!((rv[4] - 0.00048).abs() < 1e-9);

        let team_buff = &cond[1];
        assert_eq!(team_buff.name, "makhaira_team_atk");
        assert_eq!(team_buff.stat, BuffableStat::AtkFlat);
        assert_eq!(team_buff.target, BuffTarget::TeamExcludeSelf);
        assert!((team_buff.value - 0.000072).abs() < 1e-12);
        assert!(matches!(
            team_buff.activation,
            Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::ElementalMastery,
                offset: None,
                cap: None,
            })
        ));
    }

    #[test]
    fn portable_power_saw_has_em_stacks() {
        let passive = PORTABLE_POWER_SAW.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "portable_power_saw_em_stacks");
        assert_eq!(buff.stat, BuffableStat::ElementalMastery);
        assert!((buff.value - 40.0).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 40.0).abs() < 1e-6);
        assert!((rv[4] - 80.0).abs() < 1e-6);
    }

    #[test]
    fn rainslasher_has_dmg_toggle() {
        let passive = RAINSLASHER.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "rainslasher_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.20).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.20).abs() < 1e-6);
        assert!((rv[4] - 0.36).abs() < 1e-6);
    }

    #[test]
    fn royal_greatsword_has_crit_rate_stacks() {
        let passive = ROYAL_GREATSWORD.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "royal_greatsword_crit_rate_stacks");
        assert_eq!(buff.stat, BuffableStat::CritRate);
        assert!((buff.value - 0.08).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(5))
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.08).abs() < 1e-6);
        assert!((rv[4] - 0.16).abs() < 1e-6);
    }

    #[test]
    fn talking_stick_has_ele_dmg_and_atk_toggles() {
        let passive = TALKING_STICK.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);

        let ele_dmg = &cond[0];
        assert_eq!(ele_dmg.name, "talking_stick_ele_dmg");
        assert_eq!(ele_dmg.stat, BuffableStat::DmgBonus);
        assert!((ele_dmg.value - 0.16).abs() < 1e-6);
        assert!(matches!(
            ele_dmg.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = ele_dmg.refinement_values.unwrap();
        assert!((rv[0] - 0.16).abs() < 1e-6);
        assert!((rv[4] - 0.32).abs() < 1e-6);

        let atk = &cond[1];
        assert_eq!(atk.name, "talking_stick_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert!((atk.value - 0.20).abs() < 1e-6);
        assert!(matches!(
            atk.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv2 = atk.refinement_values.unwrap();
        assert!((rv2[0] - 0.20).abs() < 1e-6);
        assert!((rv2[4] - 0.40).abs() < 1e-6);
    }

    #[test]
    fn the_bell_has_shield_dmg_toggle() {
        let passive = THE_BELL.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "the_bell_shield_dmg");
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
    fn tidal_shadow_has_atk_toggle() {
        let passive = TIDAL_SHADOW.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "tidal_shadow_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.24).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.24).abs() < 1e-6);
        assert!((rv[4] - 0.48).abs() < 1e-6);
    }

    #[test]
    fn ultimate_overlords_has_element_count_conditionals() {
        let passive = ULTIMATE_OVERLORDS_MEGA_MAGIC_SWORD.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);

        let atk = &cond[0];
        assert_eq!(atk.name, "ultimate_sword_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert!((atk.value - 0.04).abs() < 1e-6);
        assert!(matches!(
            atk.activation,
            Activation::Auto(AutoCondition::TeamDiffElementCount { min_count: 1 })
        ));
        let rv = atk.refinement_values.unwrap();
        assert!((rv[0] - 0.04).abs() < 1e-6);
        assert!((rv[4] - 0.08).abs() < 1e-6);

        let dmg = &cond[1];
        assert_eq!(dmg.name, "ultimate_sword_dmg");
        assert_eq!(dmg.stat, BuffableStat::DmgBonus);
        assert!((dmg.value - 0.035).abs() < 1e-6);
        assert!(matches!(
            dmg.activation,
            Activation::Auto(AutoCondition::TeamDiffElementCount { min_count: 1 })
        ));
    }

    #[test]
    fn bloodtainted_greatsword_has_dmg_toggle() {
        let passive = BLOODTAINTED_GREATSWORD.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "bloodtainted_dmg");
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
    fn ferrous_shadow_has_charged_dmg_toggle() {
        let passive = FERROUS_SHADOW.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "ferrous_shadow_charged_dmg");
        assert_eq!(buff.stat, BuffableStat::ChargedAtkDmgBonus);
        assert!((buff.value - 0.30).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.30).abs() < 1e-6);
        assert!((rv[4] - 0.50).abs() < 1e-6);
    }

    #[test]
    fn skyrider_greatsword_has_atk_stacks() {
        let passive = SKYRIDER_GREATSWORD.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "skyrider_greatsword_atk_stacks");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.06).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(4))
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[0] - 0.06).abs() < 1e-6);
        assert!((rv[4] - 0.10).abs() < 1e-6);
    }
}
