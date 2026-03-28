use crate::buff::{
    Activation, BuffableStat, ConditionalBuff, ManualCondition, PassiveEffect, StatBuff,
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
            description: "Conditional: チームの元素タイプに応じてEM/DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "NA/CA DMG+16-32%",
            buffs: &[
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
            conditional_buffs: &[],
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
            description: "Plunging DMG+28-56%",
            buffs: &[StatBuff {
                stat: BuffableStat::PlungingAtkDmgBonus,
                value: 0.28,
                refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
            }],
            conditional_buffs: &[],
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
            description: "Heal+10-20%",
            buffs: &[StatBuff {
                stat: BuffableStat::HealingBonus,
                value: 0.10,
                refinement_values: Some([0.10, 0.125, 0.15, 0.175, 0.20]),
            }],
            conditional_buffs: &[],
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
            description: "Conditional: 元素エネルギー消費後にEM獲得",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素スキル使用でスキルDMGスタック、3スタックで元素DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 命中時にATK%スタック、シールド時に効果2倍",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 夜魂を消耗して通常/重撃ダメージアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素スキル/爆発命中でスタック獲得、DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素反応でDMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: チームの元素反応でDMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素反応時にバフ獲得",
            buffs: &[],
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            description: "Conditional: 通常攻撃速度アップ、NA DMGスタック",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 夜魂バースト時にDMGアップスタック",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 夜魂バースト時にHP基準でDMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            description: "Conditional: 夜魂バースト時にHP基準でNA DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            description: "Conditional: NA命中でSkill/Burst DMGアップ、Skill/Burst命中でNA DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            description: "Conditional: 水元素反応時にATK+20%",
            buffs: &[],
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            conditional_buffs: &[],
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
            description: "Conditional: 敵撃破時に移動速度とATK+12%",
            buffs: &[],
            conditional_buffs: &[],
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
