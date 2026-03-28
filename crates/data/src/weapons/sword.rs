use crate::buff::{
    Activation, AutoCondition, BuffableStat, ConditionalBuff, ManualCondition, PassiveEffect,
    StatBuff,
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
            description: "CRIT DMG+20-40%。罪禍を蓄積して追加CRIT DMGを獲得",
            buffs: &[StatBuff {
                stat: BuffableStat::CritDmg,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
            }],
            conditional_buffs: &[],
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
            description: "ATK+20-40%。ダメージを受けると再生効果と追加ATKダメージ、15秒に1回",
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
            description: "Conditional: 元素スキルの会心率と元素ダメージがアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: HP上限を基にした通常攻撃ダメージアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "DMG+10-20%。元素反応時に印を蓄積、2つでチーム全員にATK+20%/NA・CA・PlungeDMG+16%",
            buffs: &[StatBuff {
                stat: BuffableStat::DmgBonus,
                value: 0.10,
                refinement_values: Some([0.10, 0.125, 0.15, 0.175, 0.20]),
            }],
            conditional_buffs: &[],
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
            description: "元素DMG+12-24%。チームメンバーが元素スキルを使うと通常攻撃DMGアップ",
            buffs: &[StatBuff {
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[],
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
            description: "Conditional: HP上限に基づき元素熟知アップ。フルスタックでチーム全員にEM付与",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "CRIT Rate+4-8%。EMに基づきNA/SkillDMGアップ",
            buffs: &[StatBuff {
                stat: BuffableStat::CritRate,
                value: 0.04,
                refinement_values: Some([0.04, 0.05, 0.06, 0.07, 0.08]),
            }],
            conditional_buffs: &[],
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
            description: "Conditional: 月光の力で攻撃力とダメージがアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "元素DMG+12-24%。霧切の巴紋を蓄積して追加元素DMGアップ",
            buffs: &[StatBuff {
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[],
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
            description: "Conditional: DEFに基づき元素DMGアップ。フルスタックでチームにDEF%/元素DMG付与",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "HP+20-40%。HP上限の1.2-2.4%分ATKアップ",
            buffs: &[StatBuff {
                stat: BuffableStat::HpPercent,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "jade_cutter_hp_atk",
                description: "HP上限の1.2%分ATKアップ（HP%に比例）",
                stat: BuffableStat::AtkFlat,
                value: 0.012,
                refinement_values: Some([0.012, 0.015, 0.018, 0.021, 0.024]),
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::HpPercent,
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
            description: "CRIT Rate+4-8%。元素爆発後にNA/CAの速度とDMGアップ",
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
            description: "Skill DMG+8-16%。HP変動時にNA DMGアップ、NA命中時にSkill DMGアップ",
            buffs: &[StatBuff {
                stat: BuffableStat::SkillDmgBonus,
                value: 0.08,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
            }],
            conditional_buffs: &[],
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
            description: "Conditional: シールド強化+20-40%。攻撃命中でATK+4-8%、シールド時は2倍",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "NA DMG+16-32%。DEFに基づき元素スキルDMGアップ",
            buffs: &[StatBuff {
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
            }],
            conditional_buffs: &[],
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
            description: "Conditional: 元素スキルで継承の印を獲得、元素爆発時にエネルギー回復",
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
            description: "Conditional: 敵を倒すとATK+12%、30秒、3スタックまで",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素反応時にスキルDMGとCRIT Rateがアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素スキルのDMGがDEFの40%分アップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 会心命中時に元素粒子を生成、12秒に1回",
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
            description: "Skill DMG+16-32%、Skill CRIT Rate+6-12%",
            buffs: &[StatBuff {
                stat: BuffableStat::SkillDmgBonus,
                value: 0.16,
                refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
            }],
            conditional_buffs: &[],
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
            description: "ATK+12-24%。HP増減時にNA/CA/PlungeDMGアップ",
            buffs: &[StatBuff {
                stat: BuffableStat::AtkPercent,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[],
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
            description: "Conditional: 元素スキルのCRIT RateとER%がアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素スキル発動時にDEFに基づきダメージバフ獲得",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "元素ダメージ命中時にDMG+6-12%、6秒、2スタックまで",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "iron_sting_dmg",
                description: "元素ダメージ命中後、全DMG+6-12%（最大2スタック）",
                stat: BuffableStat::DmgBonus,
                value: 0.06,
                refinement_values: Some([0.06, 0.075, 0.09, 0.105, 0.12]),
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
            description: "Conditional: NA/CA/Plunge命中時にATK+15%、8秒",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "炎/雷の影響を受けた敵にDMG+20-36%",
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "lions_roar_dmg",
                description: "炎/雷元素の影響下の敵へのDMG+20-36%",
                stat: BuffableStat::DmgBonus,
                value: 0.20,
                refinement_values: Some([0.20, 0.24, 0.28, 0.32, 0.36]),
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
            description: "Conditional: 元素スキルまたは元素爆発の命中時にDMGバフ獲得",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素スキル命中時にATKアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: NA/CA命中時にATK/DEF+4%、6秒、4スタックまで",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: ダメージ命中で会心でなければCRIT Rate+8%、会心発生でリセット",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素スキル命中時に40%の確率でCD即リセット、30秒に1回",
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
            description: "Conditional: 草元素反応時に葉を生成、拾うとEM+60、12秒",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素スキル発動時にDMGバフ獲得",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: スプリント後にNA DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 旅人が装備時にNA/CA命中時50%で追加ダメージ、ATK+66",
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
            description: "Conditional: 装備者が「始まりの大いなる冒険」の影響を受けると元素スキル/元素爆発DMGアップ",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "DMG+12-24%。ダメージを受けると効果消失、5秒後に再発動",
            buffs: &[StatBuff {
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[],
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
            description: "NA/CA DMG+20-40%。NA/CA会心時にHP回復",
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
            description: "Conditional: 元素反応時にEM+40、8秒、3スタックまで",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: NA/CA命中時に和音を蓄積、5つでATK100%のDMG",
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
            description: "Conditional: 落下攻撃命中後にDMG+16%、8秒",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Skill/Burst DMG+16-32%。Skill/Burst命中時にDEF-4%、6秒、4スタックまで",
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
            conditional_buffs: &[],
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
            description: "Conditional: EMに基づきERアップ。チームメンバーにもER付与",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 水/氷の影響を受けた敵にDMG+12%",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 雷元素反応時にATK+20%、12秒",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 命中時に50%の確率でATK240%の追加ダメージ、15秒に1回",
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
            description: "Conditional: HP90%以上でCRIT Rate+14%",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素爆発後にNA/CA ATK+12%、15秒",
            buffs: &[],
            conditional_buffs: &[],
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
            description: "Conditional: 元素オーブ/元素粒子を獲得時にHP回復",
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
