use crate::buff::{
    Activation, BuffTarget, BuffableStat, ConditionalBuff, ManualCondition, PassiveEffect, StatBuff,
};
use crate::types::{Rarity, WeaponData, WeaponPassive, WeaponSubStat, WeaponType};

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
            description: "CRIT DMG+20-40%。ATK+28-56%",
            buffs: &[
                StatBuff {
                    stat: BuffableStat::CritDmg,
                    value: 0.20,
                    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
                },
                StatBuff {
                    stat: BuffableStat::AtkPercent,
                    value: 0.28,
                    refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
                },
            ],
            conditional_buffs: &[],
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
            description: "Conditional: スキル命中後ATKアップ、ダメージ受けるとATKアップ、シールド時HP%アップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素スキル命中でスタック獲得、スタック数に応じて元素ダメージアップ",
            buffs: &[],
            conditional_buffs: &[],
        },
    }),
};

pub const GEST_OF_THE_MIGHTY_WOLF: WeaponData = WeaponData {
    id: "gest_of_the_mighty_wolf",
    name: "Gest of the Mighty Wolf",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    base_atk: [46.0, 532.0, 563.0, 608.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.072, 0.302, 0.302, 0.331])),
    passive: Some(WeaponPassive {
        name: "Gest of the Mighty Wolf",
        effect: PassiveEffect {
            description: "Conditional: 狼の力でダメージとバフがアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            description: "ATK+16-32%。命中時に印を蓄積、4つでチーム全員にATK速度/ATKアップ",
            buffs: &[StatBuff {
                stat: BuffableStat::AtkPercent,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
            }],
            conditional_buffs: &[],
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
            description: "Conditional: シールド強化+20-40%。命中時ATKアップ、シールド時さらにATKアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Skill DMG+18-36%。シールドでスタック獲得しさらにSkill DMGアップ",
            buffs: &[StatBuff {
                stat: BuffableStat::SkillDmgBonus,
                value: 0.18,
                refinement_values: Some([0.18, 0.225, 0.27, 0.315, 0.36]),
            }],
            conditional_buffs: &[],
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
            description: "Conditional: チーム全員の元素エネルギー上限に応じて元素爆発DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 敵を倒すとATK+12-24%、30秒、3スタックまで",
            buffs: &[],
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            description: "Conditional: 元素反応に基づきダメージバフを獲得",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 草元素反応でEM付与の種を生成",
            buffs: &[],
            conditional_buffs: &[],
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
        name: "千岩の槍",
        effect: PassiveEffect {
            description: "Conditional: チーム内の璃月キャラ人数に応じてATK/CRIT Rateアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: スキル命中でATK/EMアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: EMに基づきATKアップ、チーム全員に付与",
            buffs: &[],
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            description: "Conditional: アークアイテム消費でスタック獲得、EMアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 水/雷元素影響下の敵へのDMG+20-36%",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: ダメージを与えるとCRIT Rate+8-16%、5スタックまで。会心発生でリセット",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素反応で元素DMGまたはATKアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: ダメージを受けるとシールド生成、シールド中DMG+12-24%",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: HP回復時にATKアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: チーム内の元素タイプ数に応じてATK/元素DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 炎/雷元素影響下の敵へのDMG+12-24%",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: HP70%以下で重撃の中断耐性アップ、重撃DMG+30-50%",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 通常/重撃命中でATK+6-10%、6秒、4スタックまで",
            buffs: &[],
            conditional_buffs: &[],
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
