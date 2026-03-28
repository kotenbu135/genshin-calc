use crate::buff::{
    Activation, BuffableStat, ConditionalBuff, ManualCondition, PassiveEffect, StatBuff,
};
use crate::types::{Rarity, WeaponData, WeaponPassive, WeaponSubStat, WeaponType};

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
            description: "Conditional: 夜魂のスタック蓄積でATKとDMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: EM+60。追憶の印蓄積でチーム全員にEM+100/ATK+20%",
            buffs: &[],
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            description: "Conditional: 元素スキル使用でHP上限アップ。3スタックでHydro DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 物語のスタック蓄積でDMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            description: "Conditional: 夕暮・流明・朝暉の状態を循環してDMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: NA DMG+16%/CA DMG+12%。エネルギー満タンでさらにアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: NA命中でSkill DMG+20%、Skill命中でNA DMG+20%",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: Cryo命中でNA/CA DMGアップ。アーロイ装備時にATK+66",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 弱点命中時にATK+36%、10秒",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: HP変動時にSkill/Burst DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素スキル命中でDMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素スキル使用後にATK+16%、6秒",
            buffs: &[],
            conditional_buffs: &[],
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
