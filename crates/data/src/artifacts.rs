//! Artifact set definitions for Genshin Impact.
//!
//! Each constant represents a complete artifact set with 2-piece and 4-piece bonuses.
//! Use [`ALL_ARTIFACT_SETS`] to iterate or [`super::find_artifact_set`] to look up by ID.

use crate::buff::{
    Activation, AutoCondition, BuffableStat, ConditionalBuff, ManualCondition, StatBuff,
};
use crate::types::{ArtifactRarity, ArtifactSet, SetEffect};
use genshin_calc_core::{BuffTarget, Element, Reaction, WeaponType};

// =============================================================================
// 5-star Artifact Sets
// =============================================================================

pub const CRIMSON_WITCH: ArtifactSet = ArtifactSet {
    id: "crimson_witch",
    name: "燃え盛る炎の魔女",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("炎元素ダメージ+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "過負荷、燃焼、烈開花反応ダメージ+40%。蒸発、溶解反応倍率+15%。元素スキル使用後2セット効果+50%、最大3スタック"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "cwof_pyro_stacks",
                description: desc!("Using Skill increases Pyro DMG by 7.5%, max 3 stacks"),
                stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
                value: 0.075,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            },
            ConditionalBuff {
                name: "cw_amplifying",
                description: desc!("Vaporize/Melt reaction bonus +15%"),
                stat: BuffableStat::AmplifyingBonus,
                value: 0.15,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "cw_transformative",
                description: desc!("Overloaded/Burning/Burgeon reaction DMG +40%"),
                stat: BuffableStat::TransformativeBonus,
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const GLADIATORS_FINALE: ArtifactSet = ArtifactSet {
    id: "gladiators_finale",
    name: "剣闘士のフィナーレ",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("攻撃力+18%"),
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "該当キャラクターが片手剣、両手剣、長柄武器キャラの場合、通常攻撃ダメージ+35%"
        ),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "gladiator_normal_bonus",
            description: desc!("Normal Attack DMG +35% for sword/claymore/polearm"),
            stat: BuffableStat::NormalAtkDmgBonus,
            value: 0.35,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Auto(AutoCondition::WeaponTypeRequired(&[
                WeaponType::Sword,
                WeaponType::Claymore,
                WeaponType::Polearm,
            ])),
        }],
    },
};

pub const WANDERERS_TROUPE: ArtifactSet = ArtifactSet {
    id: "wanderers_troupe",
    name: "大地を流浪する楽団",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("元素熟知+80"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalMastery,
            value: 80.0,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("該当キャラクターが法器、弓キャラの場合、重撃ダメージ+35%"),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "wanderers_charged_bonus",
            description: desc!("Charged Attack DMG +35% for Catalyst/Bow users"),
            stat: BuffableStat::ChargedAtkDmgBonus,
            value: 0.35,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Auto(AutoCondition::WeaponTypeRequired(&[
                WeaponType::Catalyst,
                WeaponType::Bow,
            ])),
        }],
    },
};

pub const NOBLESSE_OBLIGE: ArtifactSet = ArtifactSet {
    id: "noblesse_oblige",
    name: "旧貴族のしつけ",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("元素爆発のダメージ+20%"),
        buffs: &[StatBuff {
            stat: BuffableStat::BurstDmgBonus,
            value: 0.20,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素爆発を発動すると、チーム全員の攻撃力+20%、継続時間12秒、重ねがけ不可"
        ),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "noblesse_atk_bonus",
            description: desc!("After using Elemental Burst, team ATK +20%"),
            stat: BuffableStat::AtkPercent,
            value: 0.20,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::Team,
            activation: Activation::Manual(ManualCondition::Toggle),
        }],
    },
};

pub const BLOODSTAINED_CHIVALRY: ArtifactSet = ArtifactSet {
    id: "bloodstained_chivalry",
    name: "血染めの騎士道",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("物理ダメージ+25%"),
        buffs: &[StatBuff {
            stat: BuffableStat::PhysicalDmgBonus,
            value: 0.25,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("敵を倒した10秒以内に、重撃のスタミナ消費-0、ダメージ+50%"),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "bloodstained_charged_bonus",
            description: desc!("After defeating enemy, Charged Attack DMG +50%"),
            stat: BuffableStat::ChargedAtkDmgBonus,
            value: 0.50,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Toggle),
        }],
    },
};

pub const THUNDERING_FURY: ArtifactSet = ArtifactSet {
    id: "thundering_fury",
    name: "雷のような怒り",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("雷元素ダメージ+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Electro),
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "過負荷、感電、超電導、超激化反応ダメージ+40%。超開花反応ダメージ+20%。上記反応発生後、元素スキルのクールタイム-1秒。0.8秒ごとに1回のみ発動可能"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "tf_overloaded_bonus",
                description: desc!("Overloaded reaction DMG +40%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Overloaded),
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "tf_electro_charged_bonus",
                description: desc!("Electro-Charged reaction DMG +40%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::ElectroCharged),
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "tf_superconduct_bonus",
                description: desc!("Superconduct reaction DMG +40%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Superconduct),
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "tf_hyperbloom_bonus",
                description: desc!("Hyperbloom reaction DMG +40%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Hyperbloom),
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "tf_aggravate_bonus",
                description: desc!("Aggravate reaction DMG +20%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Aggravate),
                value: 0.20,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "tf_lunar_electro_charged_bonus",
                description: desc!("Lunar-Electro-Charged reaction DMG +20%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::LunarElectroCharged),
                value: 0.20,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const THUNDERSOOTHER: ArtifactSet = ArtifactSet {
    id: "thundersoother",
    name: "雷を鎮める尊者",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("雷元素耐性+40%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalRes(Element::Electro),
            value: 0.40,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("雷元素の影響を受けた敵に対するダメージ+35%"),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "thundersoother_dmg_bonus",
            description: desc!("DMG +35% against enemies affected by Electro"),
            stat: BuffableStat::DmgBonus,
            value: 0.35,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Toggle),
        }],
    },
};

pub const VIRIDESCENT_VENERER: ArtifactSet = ArtifactSet {
    id: "viridescent_venerer",
    name: "翠緑の影",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("風元素ダメージ+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "拡散反応ダメージ+60%。拡散反応に対応する元素の敵耐性-40%、継続時間10秒"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "vv_swirl_pyro_bonus",
                description: desc!("Pyro Swirl DMG +60%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Swirl(Element::Pyro)),
                value: 0.60,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "vv_swirl_hydro_bonus",
                description: desc!("Hydro Swirl DMG +60%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Swirl(Element::Hydro)),
                value: 0.60,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "vv_swirl_electro_bonus",
                description: desc!("Electro Swirl DMG +60%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Swirl(Element::Electro)),
                value: 0.60,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "vv_swirl_cryo_bonus",
                description: desc!("Cryo Swirl DMG +60%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Swirl(Element::Cryo)),
                value: 0.60,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "vv_res_shred_pyro",
                description: desc!("Enemy Pyro RES -40%"),
                stat: BuffableStat::ElementalResReduction(Element::Pyro),
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "vv_res_shred_hydro",
                description: desc!("Enemy Hydro RES -40%"),
                stat: BuffableStat::ElementalResReduction(Element::Hydro),
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "vv_res_shred_electro",
                description: desc!("Enemy Electro RES -40%"),
                stat: BuffableStat::ElementalResReduction(Element::Electro),
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "vv_res_shred_cryo",
                description: desc!("Enemy Cryo RES -40%"),
                stat: BuffableStat::ElementalResReduction(Element::Cryo),
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const MAIDEN_BELOVED: ArtifactSet = ArtifactSet {
    id: "maiden_beloved",
    name: "愛される少女",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("与える治療効果+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::HealingBonus,
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素スキルまたは元素爆発を発動した後10秒間、チーム全員が受ける治療効果+20%"
        ),
        buffs: &[],
        conditional_buffs: &[],
    },
};

pub const ARCHAIC_PETRA: ArtifactSet = ArtifactSet {
    id: "archaic_petra",
    name: "悠久の磐岩",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("岩元素ダメージ+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Geo),
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "結晶反応またはLunar-Crystallizeで形成された欠片を獲得すると、チーム全員の該当元素ダメージ+35%、継続時間10秒。同時に1つの元素ダメージボーナスのみ獲得可能"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "archaic_petra_pyro",
                description: desc!("Pyro Crystallize/Lunar-Crystallize shard: team Pyro DMG +35%"),
                stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
                value: 0.35,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "archaic_petra_hydro",
                description: desc!(
                    "Hydro Crystallize/Lunar-Crystallize shard: team Hydro DMG +35%"
                ),
                stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
                value: 0.35,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "archaic_petra_electro",
                description: desc!(
                    "Electro Crystallize/Lunar-Crystallize shard: team Electro DMG +35%"
                ),
                stat: BuffableStat::ElementalDmgBonus(Element::Electro),
                value: 0.35,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "archaic_petra_cryo",
                description: desc!("Cryo Crystallize/Lunar-Crystallize shard: team Cryo DMG +35%"),
                stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
                value: 0.35,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const RETRACING_BOLIDE: ArtifactSet = ArtifactSet {
    id: "retracing_bolide",
    name: "逆飛びの流星",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("シールド強化+35%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ShieldStrength,
            value: 0.35,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("シールド状態の時、通常攻撃と重撃ダメージ+40%"),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "bolide_normal_charged_bonus",
                description: desc!("While shielded, Normal and Charged Attack DMG +40%"),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "bolide_charged_bonus",
                description: desc!("While shielded, Charged Attack DMG +40%"),
                stat: BuffableStat::ChargedAtkDmgBonus,
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const LAVAWALKER: ArtifactSet = ArtifactSet {
    id: "lavawalker",
    name: "烈火を渡る賢者",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("炎元素耐性+40%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalRes(Element::Pyro),
            value: 0.40,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("炎元素の影響を受けた敵に対するダメージ+35%"),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "lavawalker_dmg_bonus",
            description: desc!("DMG +35% against enemies affected by Pyro"),
            stat: BuffableStat::DmgBonus,
            value: 0.35,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Toggle),
        }],
    },
};

pub const BLIZZARD_STRAYER: ArtifactSet = ArtifactSet {
    id: "blizzard_strayer",
    name: "氷風を彷徨う勇士",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("氷元素ダメージ+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "氷元素の影響を受けた敵を攻撃した場合、会心率+20%。敵が凍結状態の場合、会心率は更に+20%"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "blizzard_crit_cryo",
                description: desc!("Crit Rate +20% vs Cryo-affected enemies (1 stack)"),
                stat: BuffableStat::CritRate,
                value: 0.20,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "blizzard_crit_frozen",
                description: desc!("Crit Rate +20% vs Frozen enemies (additional, 2nd stack)"),
                stat: BuffableStat::CritRate,
                value: 0.20,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const HEART_OF_DEPTH: ArtifactSet = ArtifactSet {
    id: "heart_of_depth",
    name: "沈淪の心",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("水元素ダメージ+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("元素スキル発動後15秒間、通常攻撃と重撃のダメージ+30%"),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "hod_normal_bonus",
                description: desc!("After using Elemental Skill, Normal Attack DMG +30%"),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "hod_charged_bonus",
                description: desc!("After using Elemental Skill, Charged Attack DMG +30%"),
                stat: BuffableStat::ChargedAtkDmgBonus,
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const TENACITY_OF_THE_MILLELITH: ArtifactSet = ArtifactSet {
    id: "tenacity_of_the_millelith",
    name: "千岩牢固",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("HP+20%"),
        buffs: &[StatBuff {
            stat: BuffableStat::HpPercent,
            value: 0.20,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素スキルが敵に命中すると、周囲のチーム全員の攻撃力+20%、シールド強化+30%、継続時間3秒。0.5秒ごとに1回のみ発動可能。この効果はキャラクターが待機中でも発動可能"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "millelith_atk_bonus",
                description: desc!("After Skill hits, team ATK +20%"),
                stat: BuffableStat::AtkPercent,
                value: 0.20,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "millelith_shield_bonus",
                description: desc!("After Skill hits, team Shield Strength +30%"),
                stat: BuffableStat::ShieldStrength,
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const PALE_FLAME: ArtifactSet = ArtifactSet {
    id: "pale_flame",
    name: "蒼白の炎",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("物理ダメージ+25%"),
        buffs: &[StatBuff {
            stat: BuffableStat::PhysicalDmgBonus,
            value: 0.25,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素スキルが敵に命中すると、攻撃力+9%、継続時間7秒、最大2スタック。2スタック時に2セットの効果+100%"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "pale_flame_atk_stacks",
                description: desc!("Skill hit: ATK +9% per stack, max 2"),
                stat: BuffableStat::AtkPercent,
                value: 0.09,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(2)),
            },
            ConditionalBuff {
                name: "pale_flame_phys_bonus",
                description: desc!("At 2 stacks, Physical DMG +25% (2pc effect doubled)"),
                stat: BuffableStat::PhysicalDmgBonus,
                value: 0.25,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const SHIMENAWAS_REMINISCENCE: ArtifactSet = ArtifactSet {
    id: "shimenawas_reminiscence",
    name: "追憶のしめ縄",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("攻撃力+18%"),
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素スキルを発動した時、元素エネルギーが15以上の場合、元素エネルギーを15消費し、通常攻撃、重撃、落下攻撃ダメージ+50%、継続時間10秒。この効果は継続時間中に再発動不可"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "shimenawa_normal_bonus",
                description: desc!("After Skill use (15 energy consumed), Normal Attack DMG +50%"),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.50,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "shimenawa_charged_bonus",
                description: desc!("After Skill use (15 energy consumed), Charged Attack DMG +50%"),
                stat: BuffableStat::ChargedAtkDmgBonus,
                value: 0.50,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "shimenawa_plunging_bonus",
                description: desc!(
                    "After Skill use (15 energy consumed), Plunging Attack DMG +50%"
                ),
                stat: BuffableStat::PlungingAtkDmgBonus,
                value: 0.50,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const EMBLEM_OF_SEVERED_FATE: ArtifactSet = ArtifactSet {
    id: "emblem_of_severed_fate",
    name: "絶縁の旗印",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("元素チャージ効率+20%"),
        buffs: &[StatBuff {
            stat: BuffableStat::EnergyRecharge,
            value: 0.20,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素チャージ効率の25%を基準に、元素爆発のダメージがアップする。この方式でアップできるダメージは最大75%まで"
        ),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "emblem_burst_bonus",
            description: desc!("Burst DMG +25% of Energy Recharge, max 75%"),
            stat: BuffableStat::BurstDmgBonus,
            value: 0.25,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Auto(AutoCondition::StatScaling {
                stat: BuffableStat::EnergyRecharge,
                offset: None,
                cap: Some([0.75; 5]),
            }),
        }],
    },
};

pub const HUSK_OF_OPULENT_DREAMS: ArtifactSet = ArtifactSet {
    id: "husk_of_opulent_dreams",
    name: "華館夢醒形骸記",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("防御力+30%"),
        buffs: &[StatBuff {
            stat: BuffableStat::DefPercent,
            value: 0.30,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "装備キャラクターがフィールドにいる時、岩元素ダメージを与えた0.3秒後に「問答」スタックを1獲得、最大4スタック。1スタックにつき防御力+6%と岩元素ダメージ+6%。6秒ごとに「問答」スタックを獲得できない場合、スタック-1。装備キャラクターが待機中の場合、3秒ごとに「問答」スタック+1"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "husk_def_stacks",
                description: desc!("Curiosity stack: DEF +6% per stack, max 4"),
                stat: BuffableStat::DefPercent,
                value: 0.06,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(4)),
            },
            ConditionalBuff {
                name: "husk_geo_stacks",
                description: desc!("Curiosity stack: Geo DMG +6% per stack, max 4"),
                stat: BuffableStat::ElementalDmgBonus(Element::Geo),
                value: 0.06,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(4)),
            },
        ],
    },
};

pub const OCEAN_HUED_CLAM: ArtifactSet = ArtifactSet {
    id: "ocean_hued_clam",
    name: "海染硨磲",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("与える治療効果+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::HealingBonus,
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "装備キャラクターが治療を行うと「海染の泡」が生成され、3秒間治療量を蓄積する。蓄積時間終了時、泡が破裂し周囲の敵にHP回復量の90%分のダメージを与える（物理ダメージ）。最大30000まで蓄積可能"
        ),
        buffs: &[],
        conditional_buffs: &[],
    },
};

pub const VERMILLION_HEREAFTER: ArtifactSet = ArtifactSet {
    id: "vermillion_hereafter",
    name: "辰砂往生録",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("攻撃力+18%"),
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素爆発を発動した後、攻撃力+8%。さらにHPが減少するたびに攻撃力+10%、最大4スタック。この効果は継続時間16秒。HPが増加した場合もスタック解除されない。スタック数は0.8秒に最大1回のみ変動"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "vermillion_atk_base",
                description: desc!("After Elemental Burst, ATK +8%"),
                stat: BuffableStat::AtkPercent,
                value: 0.08,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "vermillion_atk_stacks",
                description: desc!("HP decreases: ATK +10% per stack, max 4"),
                stat: BuffableStat::AtkPercent,
                value: 0.10,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(4)),
            },
        ],
    },
};

pub const ECHOES_OF_AN_OFFERING: ArtifactSet = ArtifactSet {
    id: "echoes_of_an_offering",
    name: "来歆の余響",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("攻撃力+18%"),
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "通常攻撃が命中した時、36%の確率で「谷の念」が発動し、通常攻撃のダメージが攻撃力の70%分アップする。発動しなかった場合、次回の発動確率+20%。0.2秒ごとに1回のみ判定"
        ),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "echoes_valley_rite",
            description: desc!("Valley Rite: Normal ATK flat DMG +70% of ATK"),
            stat: BuffableStat::NormalAtkFlatDmg,
            value: 0.70,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Both(
                AutoCondition::StatScaling {
                    stat: BuffableStat::AtkPercent,
                    offset: None,
                    cap: None,
                },
                ManualCondition::Toggle,
            ),
        }],
    },
};

pub const DEEPWOOD_MEMORIES: ArtifactSet = ArtifactSet {
    id: "deepwood_memories",
    name: "深林の記憶",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("草元素ダメージ+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Dendro),
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素スキルまたは元素爆発が敵に命中した後、敵の草元素耐性-30%、継続時間8秒。装備キャラクターが待機中でも効果を発動可能"
        ),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "deepwood_dendro_res_shred",
            description: desc!("After Skill/Burst hit, enemy Dendro RES -30%"),
            stat: BuffableStat::ElementalResReduction(Element::Dendro),
            value: 0.30,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::Team,
            activation: Activation::Manual(ManualCondition::Toggle),
        }],
    },
};

pub const GILDED_DREAMS: ArtifactSet = ArtifactSet {
    id: "gilded_dreams",
    name: "金メッキの夢",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("元素熟知+80"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalMastery,
            value: 80.0,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素反応を起こした後8秒間、チーム内の自身以外のキャラクターの元素タイプに応じてバフを獲得。自身と同じ元素タイプのキャラ1人につき攻撃力+14%、異なる元素タイプのキャラ1人につき元素熟知+50。上記効果は最大3人分まで。0.8秒ごとに1回のみ発動可能。待機中でも効果を発動可能"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "gilded_same1_atk",
                description: desc!("1 same-element teammate: ATK +14%"),
                stat: BuffableStat::AtkPercent,
                value: 0.14,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamSameElementCount { min_count: 1 }),
            },
            ConditionalBuff {
                name: "gilded_same2_atk",
                description: desc!("2 same-element teammates: ATK +14%"),
                stat: BuffableStat::AtkPercent,
                value: 0.14,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamSameElementCount { min_count: 2 }),
            },
            ConditionalBuff {
                name: "gilded_same3_atk",
                description: desc!("3 same-element teammates: ATK +14%"),
                stat: BuffableStat::AtkPercent,
                value: 0.14,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamSameElementCount { min_count: 3 }),
            },
            ConditionalBuff {
                name: "gilded_diff1_em",
                description: desc!("1 different-element teammate: EM +50"),
                stat: BuffableStat::ElementalMastery,
                value: 50.0,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamDiffElementCount { min_count: 1 }),
            },
            ConditionalBuff {
                name: "gilded_diff2_em",
                description: desc!("2 different-element teammates: EM +50"),
                stat: BuffableStat::ElementalMastery,
                value: 50.0,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamDiffElementCount { min_count: 2 }),
            },
            ConditionalBuff {
                name: "gilded_diff3_em",
                description: desc!("3 different-element teammates: EM +50"),
                stat: BuffableStat::ElementalMastery,
                value: 50.0,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamDiffElementCount { min_count: 3 }),
            },
        ],
    },
};

pub const DESERT_PAVILION_CHRONICLE: ArtifactSet = ArtifactSet {
    id: "desert_pavilion_chronicle",
    name: "砂上の楼閣の史話",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("風元素ダメージ+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "重撃が敵に命中すると、通常攻撃の攻撃速度+10%、通常攻撃、重撃、落下攻撃のダメージ+40%、継続時間15秒"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "desert_pavilion_normal_bonus",
                description: desc!("After Charged Attack hits, Normal Attack DMG +40%"),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "desert_pavilion_charged_bonus",
                description: desc!("After Charged Attack hits, Charged Attack DMG +40%"),
                stat: BuffableStat::ChargedAtkDmgBonus,
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "desert_pavilion_plunging_bonus",
                description: desc!("After Charged Attack hits, Plunging Attack DMG +40%"),
                stat: BuffableStat::PlungingAtkDmgBonus,
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const FLOWER_OF_PARADISE_LOST: ArtifactSet = ArtifactSet {
    id: "flower_of_paradise_lost",
    name: "楽園の絶花",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("元素熟知+80"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalMastery,
            value: 80.0,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "開花、超開花、烈開花反応ダメージ+40%。さらに装備キャラクターが開花、超開花、烈開花反応を起こした後、上記効果+25%、最大4スタック、継続時間10秒。0.8秒ごとに1回のみスタック獲得可能。待機中でも効果を発動可能"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "fopl_bloom_base",
                description: desc!("Bloom DMG +40%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Bloom),
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "fopl_hyperbloom_base",
                description: desc!("Hyperbloom DMG +40%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Hyperbloom),
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "fopl_burgeon_base",
                description: desc!("Burgeon DMG +40%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Burgeon),
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "fopl_lunar_bloom_base",
                description: desc!("Lunar-Bloom DMG +10%"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::LunarBloom),
                value: 0.10,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "fopl_bloom_stacks",
                description: desc!("After triggering Bloom: Bloom DMG +25% per stack, max 4"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Bloom),
                value: 0.25,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(4)),
            },
            ConditionalBuff {
                name: "fopl_hyperbloom_stacks",
                description: desc!("After triggering Bloom: Hyperbloom DMG +25% per stack, max 4"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Hyperbloom),
                value: 0.25,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(4)),
            },
            ConditionalBuff {
                name: "fopl_burgeon_stacks",
                description: desc!("After triggering Bloom: Burgeon DMG +25% per stack, max 4"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::Burgeon),
                value: 0.25,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(4)),
            },
            ConditionalBuff {
                name: "fopl_lunar_bloom_stacks",
                description: desc!("After triggering Bloom: Lunar-Bloom DMG +25% per stack, max 4"),
                stat: BuffableStat::ReactionDmgBonus(Reaction::LunarBloom),
                value: 0.25,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(4)),
            },
        ],
    },
};

pub const NYMPHS_DREAM: ArtifactSet = ArtifactSet {
    id: "nymphs_dream",
    name: "水仙の夢",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("水元素ダメージ+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "通常攻撃、重撃、元素スキル、元素爆発が命中した後、1/2/3スタックでそれぞれ攻撃力+7%/16%/25%、水元素ダメージ+4%/9%/15%。各攻撃が命中するたびに、他種類のスタックの持続時間をリセット。各種類のスタックは0.8秒に1回のみ獲得可能"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "nymphs_dream_atk_stacks",
                description: desc!("Attack hits: ATK +7%/16%/25% at 1/2/3 stacks"),
                stat: BuffableStat::AtkPercent,
                value: 0.07,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: Some(&[0.07, 0.16, 0.25]),
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            },
            ConditionalBuff {
                name: "nymphs_dream_hydro_stacks",
                description: desc!("Attack hits: Hydro DMG +4%/9%/15% at 1/2/3 stacks"),
                stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
                value: 0.04,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: Some(&[0.04, 0.09, 0.15]),
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(3)),
            },
        ],
    },
};

pub const VOURUKASHAS_GLOW: ArtifactSet = ArtifactSet {
    id: "vourukashas_glow",
    name: "花海甘露の光",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("HP+20%"),
        buffs: &[StatBuff {
            stat: BuffableStat::HpPercent,
            value: 0.20,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素スキルと元素爆発のダメージ+10%。装備キャラクターがダメージを受けた後、上記効果+80%、継続時間5秒、最大5スタック。各スタックの持続時間は独立。待機中でも効果を発動可能"
        ),
        buffs: &[
            StatBuff {
                stat: BuffableStat::SkillDmgBonus,
                value: 0.10,
                refinement_values: None,
            },
            StatBuff {
                stat: BuffableStat::BurstDmgBonus,
                value: 0.10,
                refinement_values: None,
            },
        ],
        conditional_buffs: &[
            ConditionalBuff {
                name: "vourukasha_skill_stacks",
                description: desc!("After taking DMG: Skill DMG +8% per stack, max 5"),
                stat: BuffableStat::SkillDmgBonus,
                value: 0.08,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(5)),
            },
            ConditionalBuff {
                name: "vourukasha_burst_stacks",
                description: desc!("After taking DMG: Burst DMG +8% per stack, max 5"),
                stat: BuffableStat::BurstDmgBonus,
                value: 0.08,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(5)),
            },
        ],
    },
};

pub const MARECHAUSSEE_HUNTER: ArtifactSet = ArtifactSet {
    id: "marechaussee_hunter",
    name: "ファントムハンター",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("通常攻撃と重撃ダメージ+15%"),
        buffs: &[
            StatBuff {
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.15,
                refinement_values: None,
            },
            StatBuff {
                stat: BuffableStat::ChargedAtkDmgBonus,
                value: 0.15,
                refinement_values: None,
            },
        ],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("現在のHPが変動した後、会心率+12%、継続時間5秒、最大3スタック"),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "marechaussee_crit_stacks",
            description: desc!("HP changes: Crit Rate +12% per stack, max 3"),
            stat: BuffableStat::CritRate,
            value: 0.12,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Stacks(3)),
        }],
    },
};

pub const GOLDEN_TROUPE: ArtifactSet = ArtifactSet {
    id: "golden_troupe",
    name: "黄金の劇団",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("元素スキルのダメージ+20%"),
        buffs: &[StatBuff {
            stat: BuffableStat::SkillDmgBonus,
            value: 0.20,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素スキルのダメージ+25%。さらにキャラクターが待機中の場合、元素スキルのダメージ+25%"
        ),
        buffs: &[StatBuff {
            stat: BuffableStat::SkillDmgBonus,
            value: 0.25,
            refinement_values: None,
        }],
        conditional_buffs: &[ConditionalBuff {
            name: "golden_troupe_off_field_bonus",
            description: desc!("While off-field, Skill DMG +25%"),
            stat: BuffableStat::SkillDmgBonus,
            value: 0.25,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Toggle),
        }],
    },
};

pub const SONG_OF_DAYS_PAST: ArtifactSet = ArtifactSet {
    id: "song_of_days_past",
    name: "在りし日の歌",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("与える治療効果+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::HealingBonus,
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "装備キャラクターがチームメイトを治療した時、治療量を記録する「往日の協奏」効果が発動。6秒後に「往日の協奏」は解消され、その記録した治療量の8%分、チーム全員の通常攻撃、重撃、落下攻撃、元素スキル、元素爆発のダメージをアップ。最大2000まで。「往日の協奏」の持続中に再発動された場合、既存の記録はクリア"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "song_days_past_normal",
                description: desc!("8% of healing as Normal ATK flat DMG, max 2000"),
                stat: BuffableStat::NormalAtkFlatDmg,
                value: 2000.0,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "song_days_past_charged",
                description: desc!("8% of healing as Charged ATK flat DMG, max 2000"),
                stat: BuffableStat::ChargedAtkFlatDmg,
                value: 2000.0,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "song_days_past_plunging",
                description: desc!("8% of healing as Plunging ATK flat DMG, max 2000"),
                stat: BuffableStat::PlungingAtkFlatDmg,
                value: 2000.0,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "song_days_past_skill",
                description: desc!("8% of healing as Skill flat DMG, max 2000"),
                stat: BuffableStat::SkillFlatDmg,
                value: 2000.0,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "song_days_past_burst",
                description: desc!("8% of healing as Burst flat DMG, max 2000"),
                stat: BuffableStat::BurstFlatDmg,
                value: 2000.0,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const NIGHTTIME_WHISPERS_IN_THE_ECHOING_WOODS: ArtifactSet = ArtifactSet {
    id: "nighttime_whispers_in_the_echoing_woods",
    name: "残響の森で囁く夜",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("攻撃力+18%"),
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素スキルを使用した後、岩元素ダメージ+20%。Crystallize shieldまたはLunar-Crystallize Moondriftsを受けている時、この効果がさらに+30%"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "nighttime_geo_base",
                description: desc!("After using Elemental Skill, Geo DMG +20%"),
                stat: BuffableStat::ElementalDmgBonus(Element::Geo),
                value: 0.20,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "nighttime_geo_nightsoul",
                description: desc!(
                    "While protected by Crystallize shield/Moondrifts, Geo DMG +30% extra"
                ),
                stat: BuffableStat::ElementalDmgBonus(Element::Geo),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const FRAGMENT_OF_HARMONIC_WHIMSY: ArtifactSet = ArtifactSet {
    id: "fragment_of_harmonic_whimsy",
    name: "諧律奇想の断章",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("攻撃力+18%"),
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "命の契約の数値が変動した時、与えるダメージ+18%、継続時間6秒、最大3スタック。0.2秒ごとに1回のみ発動可能。待機中でも効果を発動可能"
        ),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "harmonic_whimsy_dmg_stacks",
            description: desc!("Bond of Life changes: DMG +18% per stack, max 3"),
            stat: BuffableStat::DmgBonus,
            value: 0.18,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Stacks(3)),
        }],
    },
};

pub const UNFINISHED_REVERIE: ArtifactSet = ArtifactSet {
    id: "unfinished_reverie",
    name: "未完の想起",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("攻撃力+18%"),
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "戦闘状態を離れて3秒後、与えるダメージ+50%。戦闘中、近くに燃焼状態の敵がいない場合は徐々に低下し、燃焼状態の敵がいる場合は最大+50%まで上昇する"
        ),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "unfinished_reverie_dmg_bonus",
            description: desc!("Out of combat or maintained near Burning enemy: DMG +50%"),
            stat: BuffableStat::DmgBonus,
            value: 0.50,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Toggle),
        }],
    },
};

pub const SCROLL_OF_THE_HERO_OF_CINDER_CITY: ArtifactSet = ArtifactSet {
    id: "scroll_of_the_hero_of_cinder_city",
    name: "灰燼の都の英雄譚",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!(
            "夜魂バースト発生時に元素エネルギーを6回復。エネルギーのみの効果のためダメージステータスとしては表現しない"
        ),
        buffs: &[],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "装備キャラクターが夜魂バースト状態にある、または燃焼、超電導等の元素反応を起こした後、チーム全員の対応する元素ダメージ+12%。同時に最大2種類の元素に効果。待機中でも発動可能。持続時間12秒"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "scroll_pyro_dmg",
                description: desc!("Team Pyro DMG +12%"),
                stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
                value: 0.12,
                nightsoul_value: Some(0.40),
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "scroll_hydro_dmg",
                description: desc!("Team Hydro DMG +12%"),
                stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
                value: 0.12,
                nightsoul_value: Some(0.40),
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "scroll_electro_dmg",
                description: desc!("Team Electro DMG +12%"),
                stat: BuffableStat::ElementalDmgBonus(Element::Electro),
                value: 0.12,
                nightsoul_value: Some(0.40),
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "scroll_cryo_dmg",
                description: desc!("Team Cryo DMG +12%"),
                stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
                value: 0.12,
                nightsoul_value: Some(0.40),
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "scroll_anemo_dmg",
                description: desc!("Team Anemo DMG +12%"),
                stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
                value: 0.12,
                nightsoul_value: Some(0.40),
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "scroll_geo_dmg",
                description: desc!("Team Geo DMG +12%"),
                stat: BuffableStat::ElementalDmgBonus(Element::Geo),
                value: 0.12,
                nightsoul_value: Some(0.40),
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "scroll_dendro_dmg",
                description: desc!("Team Dendro DMG +12%"),
                stat: BuffableStat::ElementalDmgBonus(Element::Dendro),
                value: 0.12,
                nightsoul_value: Some(0.40),
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const OBSIDIAN_CODEX: ArtifactSet = ArtifactSet {
    id: "obsidian_codex",
    name: "黒曜の秘典",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!(
            "夜魂の加護状態にあり、かつフィールド上にいるキャラクターの与えるダメージ+15%"
        ),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "Obsidian Codex Nightsoul DMG Bonus",
            description: desc!("While in Nightsoul's Blessing and on field, DMG +15%"),
            stat: BuffableStat::DmgBonus,
            value: 0.15,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Both(AutoCondition::NightsoulRequired, ManualCondition::Toggle),
        }],
    },
    four_piece: SetEffect {
        description: desc!(
            "装備キャラクターがフィールド上で夜魂ポイントを1消費した後、会心率+40%、継続時間6秒。1秒ごとに1回のみ発動可能"
        ),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "Obsidian Codex Crit Rate Bonus",
            description: desc!("After on-field Nightsoul point consumption, Crit Rate +40%"),
            stat: BuffableStat::CritRate,
            value: 0.40,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Both(AutoCondition::NightsoulRequired, ManualCondition::Toggle),
        }],
    },
};

pub const ADVENTURER: ArtifactSet = ArtifactSet {
    id: "adventurer",
    name: "冒険者",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("HP上限+1000"),
        buffs: &[StatBuff {
            stat: BuffableStat::HpFlat,
            value: 1000.0,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("宝箱を開けた後、HPを回復。治療効果のためダメージ計算では未対応"),
        buffs: &[],
        conditional_buffs: &[],
    },
};

pub const LUCKY_DOG: ArtifactSet = ArtifactSet {
    id: "lucky_dog",
    name: "幸運",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("防御力+100"),
        buffs: &[StatBuff {
            stat: BuffableStat::DefFlat,
            value: 100.0,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("モラを拾得した時、HPを回復。治療効果のためダメージ計算では未対応"),
        buffs: &[],
        conditional_buffs: &[],
    },
};

pub const FINALE_OF_THE_DEEP_GALLERIES: ArtifactSet = ArtifactSet {
    id: "finale_of_the_deep_galleries",
    name: "深廊の終曲",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("氷元素ダメージ+15%"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
            value: 0.15,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("条件達成時、通常攻撃ダメージ+60%、元素爆発ダメージ+60%"),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "finale_deep_galleries_normal_bonus",
                description: desc!("Conditional: Normal Attack DMG +60%"),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.60,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "finale_deep_galleries_burst_bonus",
                description: desc!("Conditional: Burst DMG +60%"),
                stat: BuffableStat::BurstDmgBonus,
                value: 0.60,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const LONG_NIGHTS_OATH: ArtifactSet = ArtifactSet {
    id: "long_nights_oath",
    name: "長き夜の誓い",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("落下攻撃ダメージ+25%"),
        buffs: &[StatBuff {
            stat: BuffableStat::PlungingAtkDmgBonus,
            value: 0.25,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("条件達成時、落下攻撃ダメージ+15%、最大5スタック"),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "long_nights_oath_plunging_stacks",
            description: desc!("Conditional: Plunging Attack DMG +15% per stack, max 5"),
            stat: BuffableStat::PlungingAtkDmgBonus,
            value: 0.15,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Stacks(5)),
        }],
    },
};

pub const RESOLUTION_OF_SOJOURNER: ArtifactSet = ArtifactSet {
    id: "resolution_of_sojourner",
    name: "旅人の心",
    rarity: ArtifactRarity::Star4And5,
    two_piece: SetEffect {
        description: desc!("攻撃力+18%"),
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("重撃の会心率+30%"),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "sojourner_charged_crit",
            description: desc!("Charged Attack Crit Rate +30%"),
            stat: BuffableStat::CritRate,
            value: 0.30,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Toggle),
        }],
    },
};

pub const TINY_MIRACLE: ArtifactSet = ArtifactSet {
    id: "tiny_miracle",
    name: "奇跡",
    rarity: ArtifactRarity::Star4And5,
    two_piece: SetEffect {
        description: desc!("全元素耐性+20%"),
        buffs: &[
            StatBuff {
                stat: BuffableStat::ElementalRes(Element::Pyro),
                value: 0.20,
                refinement_values: None,
            },
            StatBuff {
                stat: BuffableStat::ElementalRes(Element::Hydro),
                value: 0.20,
                refinement_values: None,
            },
            StatBuff {
                stat: BuffableStat::ElementalRes(Element::Electro),
                value: 0.20,
                refinement_values: None,
            },
            StatBuff {
                stat: BuffableStat::ElementalRes(Element::Cryo),
                value: 0.20,
                refinement_values: None,
            },
            StatBuff {
                stat: BuffableStat::ElementalRes(Element::Dendro),
                value: 0.20,
                refinement_values: None,
            },
            StatBuff {
                stat: BuffableStat::ElementalRes(Element::Anemo),
                value: 0.20,
                refinement_values: None,
            },
            StatBuff {
                stat: BuffableStat::ElementalRes(Element::Geo),
                value: 0.20,
                refinement_values: None,
            },
        ],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素ダメージを受けた後、その元素の耐性+30%、継続時間10秒。10秒ごとに1回のみ発動可能"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "tiny_miracle_pyro_res",
                description: desc!("After taking Pyro DMG: Pyro RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Pyro),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "tiny_miracle_hydro_res",
                description: desc!("After taking Hydro DMG: Hydro RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Hydro),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "tiny_miracle_electro_res",
                description: desc!("After taking Electro DMG: Electro RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Electro),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "tiny_miracle_cryo_res",
                description: desc!("After taking Cryo DMG: Cryo RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Cryo),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "tiny_miracle_dendro_res",
                description: desc!("After taking Dendro DMG: Dendro RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Dendro),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "tiny_miracle_anemo_res",
                description: desc!("After taking Anemo DMG: Anemo RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Anemo),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "tiny_miracle_geo_res",
                description: desc!("After taking Geo DMG: Geo RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Geo),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const BERSERKER: ArtifactSet = ArtifactSet {
    id: "berserker",
    name: "狂戦士",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("会心率+12%"),
        buffs: &[StatBuff {
            stat: BuffableStat::CritRate,
            value: 0.12,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("HPが70%以下になると、会心率+24%"),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "berserker_low_hp_crit",
            description: desc!("When HP below 70%, Crit Rate +24%"),
            stat: BuffableStat::CritRate,
            value: 0.24,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Toggle),
        }],
    },
};

pub const INSTRUCTOR: ArtifactSet = ArtifactSet {
    id: "instructor",
    name: "教官",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("元素熟知+80"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalMastery,
            value: 80.0,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("元素反応を起こした後、チーム全員の元素熟知+120、継続時間8秒"),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "instructor_em_bonus",
            description: desc!("After triggering reaction, team EM +120"),
            stat: BuffableStat::ElementalMastery,
            value: 120.0,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::Team,
            activation: Activation::Manual(ManualCondition::Toggle),
        }],
    },
};

pub const EXILE: ArtifactSet = ArtifactSet {
    id: "exile",
    name: "亡命者",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("元素チャージ効率+20%"),
        buffs: &[StatBuff {
            stat: BuffableStat::EnergyRecharge,
            value: 0.20,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素爆発を発動すると、2秒ごとにチームメイト全員の元素エネルギーを2回復、継続時間6秒。重ねがけ不可"
        ),
        buffs: &[],
        conditional_buffs: &[],
    },
};

pub const MARTIAL_ARTIST: ArtifactSet = ArtifactSet {
    id: "martial_artist",
    name: "武人",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("通常攻撃と重撃ダメージ+15%"),
        buffs: &[
            StatBuff {
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.15,
                refinement_values: None,
            },
            StatBuff {
                stat: BuffableStat::ChargedAtkDmgBonus,
                value: 0.15,
                refinement_values: None,
            },
        ],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("元素スキルを発動した後、通常攻撃と重撃のダメージ+25%、継続時間8秒"),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "martial_artist_normal_bonus",
                description: desc!("After Elemental Skill use, Normal Attack DMG +25%"),
                stat: BuffableStat::NormalAtkDmgBonus,
                value: 0.25,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "martial_artist_charged_bonus",
                description: desc!("After Elemental Skill use, Charged Attack DMG +25%"),
                stat: BuffableStat::ChargedAtkDmgBonus,
                value: 0.25,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const DEFENDERS_WILL: ArtifactSet = ArtifactSet {
    id: "defenders_will",
    name: "守護の心",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("防御力+30%"),
        buffs: &[StatBuff {
            stat: BuffableStat::DefPercent,
            value: 0.30,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("チーム内キャラクターの元素タイプ1種類につき、対応する元素耐性+30%"),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "defenders_will_pyro_res",
                description: desc!("Pyro character on team: Pyro RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Pyro),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamElementCount {
                    element: Element::Pyro,
                    min_count: 1,
                }),
            },
            ConditionalBuff {
                name: "defenders_will_hydro_res",
                description: desc!("Hydro character on team: Hydro RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Hydro),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamElementCount {
                    element: Element::Hydro,
                    min_count: 1,
                }),
            },
            ConditionalBuff {
                name: "defenders_will_electro_res",
                description: desc!("Electro character on team: Electro RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Electro),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamElementCount {
                    element: Element::Electro,
                    min_count: 1,
                }),
            },
            ConditionalBuff {
                name: "defenders_will_cryo_res",
                description: desc!("Cryo character on team: Cryo RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Cryo),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamElementCount {
                    element: Element::Cryo,
                    min_count: 1,
                }),
            },
            ConditionalBuff {
                name: "defenders_will_dendro_res",
                description: desc!("Dendro character on team: Dendro RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Dendro),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamElementCount {
                    element: Element::Dendro,
                    min_count: 1,
                }),
            },
            ConditionalBuff {
                name: "defenders_will_anemo_res",
                description: desc!("Anemo character on team: Anemo RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Anemo),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamElementCount {
                    element: Element::Anemo,
                    min_count: 1,
                }),
            },
            ConditionalBuff {
                name: "defenders_will_geo_res",
                description: desc!("Geo character on team: Geo RES +30%"),
                stat: BuffableStat::ElementalRes(Element::Geo),
                value: 0.30,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::TeamElementCount {
                    element: Element::Geo,
                    min_count: 1,
                }),
            },
        ],
    },
};

pub const BRAVE_HEART: ArtifactSet = ArtifactSet {
    id: "brave_heart",
    name: "勇士の心",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("攻撃力+18%"),
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!("HPが50%以上の敵に対するダメージ+30%"),
        buffs: &[],
        conditional_buffs: &[ConditionalBuff {
            name: "brave_heart_dmg_bonus",
            description: desc!("DMG +30% vs enemies with HP above 50%"),
            stat: BuffableStat::DmgBonus,
            value: 0.30,
            nightsoul_value: None,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Toggle),
        }],
    },
};

pub const GAMBLER: ArtifactSet = ArtifactSet {
    id: "gambler",
    name: "博徒",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("元素スキルのダメージ+20%"),
        buffs: &[StatBuff {
            stat: BuffableStat::SkillDmgBonus,
            value: 0.20,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "敵を倒した時、100%の確率で元素スキルのクールタイムをリセットする。15秒ごとに1回のみ発動可能"
        ),
        buffs: &[],
        conditional_buffs: &[],
    },
};

pub const SCHOLAR: ArtifactSet = ArtifactSet {
    id: "scholar",
    name: "学者",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("元素チャージ効率+20%"),
        buffs: &[StatBuff {
            stat: BuffableStat::EnergyRecharge,
            value: 0.20,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素エネルギーを獲得した時、チーム全員の弓、法器キャラの元素エネルギーを3回復。3秒ごとに1回のみ発動可能"
        ),
        buffs: &[],
        conditional_buffs: &[],
    },
};

pub const PRAYERS_FOR_ILLUMINATION: ArtifactSet = ArtifactSet {
    id: "prayers_for_illumination",
    name: "火祭りの人",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("炎元素に影響されている時間-40%"),
        buffs: &[],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(""),
        buffs: &[],
        conditional_buffs: &[],
    },
};

pub const PRAYERS_FOR_DESTINY: ArtifactSet = ArtifactSet {
    id: "prayers_for_destiny",
    name: "水祭りの人",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("水元素に影響されている時間-40%"),
        buffs: &[],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(""),
        buffs: &[],
        conditional_buffs: &[],
    },
};

pub const PRAYERS_FOR_WISDOM: ArtifactSet = ArtifactSet {
    id: "prayers_for_wisdom",
    name: "雷祭りの人",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("雷元素に影響されている時間-40%"),
        buffs: &[],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(""),
        buffs: &[],
        conditional_buffs: &[],
    },
};

pub const PRAYERS_TO_SPRINGTIME: ArtifactSet = ArtifactSet {
    id: "prayers_to_springtime",
    name: "氷祭りの人",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: desc!("氷元素に影響されている時間-40%"),
        buffs: &[],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(""),
        buffs: &[],
        conditional_buffs: &[],
    },
};

// =============================================================================
// v5.4+ Artifact Sets
// =============================================================================

pub const GLORY_OF_THE_ANCIENT_SEA: ArtifactSet = ArtifactSet {
    id: "glory_of_the_ancient_sea",
    name: "古海の栄光",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("HP+20%"),
        buffs: &[StatBuff {
            stat: BuffableStat::HpPercent,
            value: 0.20,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "現在のHP上限と上限を超えた治療量に基づき通常攻撃と重撃のダメージがアップ。HP上限が30000を超えた場合、超過量1000ごとに通常攻撃と重撃のダメージが+24。超過回復量1000ごとに通常攻撃と重撃のダメージがさらに+32。超過回復効果のスタックは最大6スタック。持続時間6秒"
        ),
        buffs: &[],
        conditional_buffs: &[
            // HP excess: (total_HP - 30000) * 0.024 = +24 per 1000 HP over 30000
            ConditionalBuff {
                name: "glory_ancient_sea_hp_normal",
                description: desc!("Per 1000 HP over 30000: Normal ATK flat DMG +24"),
                stat: BuffableStat::NormalAtkFlatDmg,
                value: 0.024,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::HpPercent,
                    offset: Some(30000.0),
                    cap: None,
                }),
            },
            ConditionalBuff {
                name: "glory_ancient_sea_hp_charged",
                description: desc!("Per 1000 HP over 30000: Charged ATK flat DMG +24"),
                stat: BuffableStat::ChargedAtkFlatDmg,
                value: 0.024,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::HpPercent,
                    offset: Some(30000.0),
                    cap: None,
                }),
            },
            // Excess healing stacks: +32 per stack, max 6
            ConditionalBuff {
                name: "glory_ancient_sea_heal_normal",
                description: desc!("Excess healing stacks: Normal ATK flat DMG +32/stack, max 6"),
                stat: BuffableStat::NormalAtkFlatDmg,
                value: 32.0,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(6)),
            },
            ConditionalBuff {
                name: "glory_ancient_sea_heal_charged",
                description: desc!("Excess healing stacks: Charged ATK flat DMG +32/stack, max 6"),
                stat: BuffableStat::ChargedAtkFlatDmg,
                value: 32.0,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(6)),
            },
        ],
    },
};

pub const CHRONICLED_SANDS_AND_WATER: ArtifactSet = ArtifactSet {
    id: "chronicled_sands_and_water",
    name: "記憶の砂と水",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("元素チャージ効率+20%"),
        buffs: &[StatBuff {
            stat: BuffableStat::EnergyRecharge,
            value: 0.20,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素チャージ効率の40%に基づいて、元素スキルと元素爆発のダメージアップ。この方式でアップできるダメージは最大80%まで"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "chronicled_skill_bonus",
                description: desc!("Skill DMG: 40% of ER, max 80%"),
                stat: BuffableStat::SkillDmgBonus,
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::EnergyRecharge,
                    offset: None,
                    cap: Some([0.80; 5]),
                }),
            },
            ConditionalBuff {
                name: "chronicled_burst_bonus",
                description: desc!("Burst DMG: 40% of ER, max 80%"),
                stat: BuffableStat::BurstDmgBonus,
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Auto(AutoCondition::StatScaling {
                    stat: BuffableStat::EnergyRecharge,
                    offset: None,
                    cap: Some([0.80; 5]),
                }),
            },
        ],
    },
};

// v6.0 Artifact Sets

pub const NIGHT_OF_THE_SKYS_UNVEILING: ArtifactSet = ArtifactSet {
    id: "night_of_the_skys_unveiling",
    name: "天幕の夜の露見",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("元素熟知+80"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalMastery,
            value: 80.0,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "近くのチームメンバーが月反応を起こした時、装備キャラがフィールド上にいると「月明かり：意志」効果を4秒間獲得：月兆が新月の光/昇天の光の時、会心率+15%/30%。チーム全員の月反応ダメージが、チームメンバーが持つ異なる月明かり効果1つにつき10%アップ"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "gleaming_moon_intent_crit",
                description: desc!(
                    "Gleaming Moon: Intent — CRIT Rate +15%/30% (Nascent/Ascendant Gleam)"
                ),
                stat: BuffableStat::CritRate,
                value: 0.15,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: Some(&[0.15, 0.30]),
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(2)),
            },
            ConditionalBuff {
                name: "gleaming_moon_intent_lunar_dmg",
                description: desc!("Gleaming Moon: Intent — Lunar Reaction DMG +10%"),
                stat: BuffableStat::TransformativeBonus,
                value: 0.10,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const SILKEN_MOONS_SERENADE: ArtifactSet = ArtifactSet {
    id: "silken_moons_serenade",
    name: "絹月の小夜曲",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("元素チャージ効率+20%"),
        buffs: &[StatBuff {
            stat: BuffableStat::EnergyRecharge,
            value: 0.20,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "元素ダメージを与えた時、「月明かり：献身」効果を8秒間獲得：月兆が新月の光/昇天の光の時、チーム全員の元素熟知+60/120。装備キャラがフィールド外でも発動可。チーム全員の月反応ダメージが、チームメンバーが持つ異なる月明かり効果1つにつき10%アップ"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "gleaming_moon_devotion_em",
                description: desc!(
                    "Gleaming Moon: Devotion — EM +60/120 (Nascent/Ascendant Gleam)"
                ),
                stat: BuffableStat::ElementalMastery,
                value: 60.0,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: Some(&[60.0, 120.0]),
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Stacks(2)),
            },
            ConditionalBuff {
                name: "gleaming_moon_devotion_lunar_dmg",
                description: desc!("Gleaming Moon: Devotion — Lunar Reaction DMG +10%"),
                stat: BuffableStat::TransformativeBonus,
                value: 0.10,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::Team,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

// v6.3 Artifact Sets

pub const AUBADE_OF_MORNINGSTAR_AND_MOON: ArtifactSet = ArtifactSet {
    id: "aubade_of_morningstar_and_moon",
    name: "明星と月のオーバード",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("元素熟知+80"),
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalMastery,
            value: 80.0,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "装備キャラがフィールド外の時、月反応ダメージ+20%。月兆レベルが昇天の光以上の場合、月反応ダメージがさらに+40%。装備キャラがフィールドに復帰して3秒後に効果消滅"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "aubade_off_field_lunar",
                description: desc!("Off-field Lunar Reaction DMG +20%"),
                stat: BuffableStat::TransformativeBonus,
                value: 0.20,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "aubade_ascendant_lunar",
                description: desc!("Ascendant Gleam: additional Lunar Reaction DMG +40%"),
                stat: BuffableStat::TransformativeBonus,
                value: 0.40,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

pub const A_DAY_CARVED_FROM_RISING_WINDS: ArtifactSet = ArtifactSet {
    id: "a_day_carved_from_rising_winds",
    name: "昇風に刻まれた一日",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: desc!("攻撃力+18%"),
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
        conditional_buffs: &[],
    },
    four_piece: SetEffect {
        description: desc!(
            "通常攻撃、重撃、元素スキルまたは元素爆発が敵に命中後、「牧風の祝福」効果を6秒間獲得：攻撃力+25%。装備キャラが「魔女の宿題」を完了している場合、「牧風の決意」に昇格し、会心率がさらに+20%。フィールド外でも発動可"
        ),
        buffs: &[],
        conditional_buffs: &[
            ConditionalBuff {
                name: "blessing_of_pastoral_winds_atk",
                description: desc!("Blessing of Pastoral Winds: ATK +25%"),
                stat: BuffableStat::AtkPercent,
                value: 0.25,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
            ConditionalBuff {
                name: "resolve_of_pastoral_winds_crit",
                description: desc!("Resolve of Pastoral Winds (Witch's Homework): CRIT Rate +20%"),
                stat: BuffableStat::CritRate,
                value: 0.20,
                nightsoul_value: None,
                refinement_values: None,
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            },
        ],
    },
};

// =============================================================================
// ALL_ARTIFACT_SETS
// =============================================================================

/// All artifact sets, ordered by release version.
pub const ALL_ARTIFACT_SETS: &[&ArtifactSet] = &[
    // 5-star sets
    &CRIMSON_WITCH,
    &GLADIATORS_FINALE,
    &WANDERERS_TROUPE,
    &NOBLESSE_OBLIGE,
    &BLOODSTAINED_CHIVALRY,
    &THUNDERING_FURY,
    &THUNDERSOOTHER,
    &VIRIDESCENT_VENERER,
    &MAIDEN_BELOVED,
    &ARCHAIC_PETRA,
    &RETRACING_BOLIDE,
    &LAVAWALKER,
    &BLIZZARD_STRAYER,
    &HEART_OF_DEPTH,
    &TENACITY_OF_THE_MILLELITH,
    &PALE_FLAME,
    &SHIMENAWAS_REMINISCENCE,
    &EMBLEM_OF_SEVERED_FATE,
    &HUSK_OF_OPULENT_DREAMS,
    &OCEAN_HUED_CLAM,
    &VERMILLION_HEREAFTER,
    &ECHOES_OF_AN_OFFERING,
    &DEEPWOOD_MEMORIES,
    &GILDED_DREAMS,
    &DESERT_PAVILION_CHRONICLE,
    &FLOWER_OF_PARADISE_LOST,
    &NYMPHS_DREAM,
    &VOURUKASHAS_GLOW,
    &MARECHAUSSEE_HUNTER,
    &GOLDEN_TROUPE,
    &SONG_OF_DAYS_PAST,
    &NIGHTTIME_WHISPERS_IN_THE_ECHOING_WOODS,
    &FRAGMENT_OF_HARMONIC_WHIMSY,
    &UNFINISHED_REVERIE,
    &SCROLL_OF_THE_HERO_OF_CINDER_CITY,
    &OBSIDIAN_CODEX,
    &GLORY_OF_THE_ANCIENT_SEA,
    &CHRONICLED_SANDS_AND_WATER,
    &NIGHT_OF_THE_SKYS_UNVEILING,
    &SILKEN_MOONS_SERENADE,
    &AUBADE_OF_MORNINGSTAR_AND_MOON,
    &A_DAY_CARVED_FROM_RISING_WINDS,
    // 4-star / mixed rarity sets
    &ADVENTURER,
    &LUCKY_DOG,
    &FINALE_OF_THE_DEEP_GALLERIES,
    &LONG_NIGHTS_OATH,
    &RESOLUTION_OF_SOJOURNER,
    &TINY_MIRACLE,
    &BERSERKER,
    &INSTRUCTOR,
    &EXILE,
    &MARTIAL_ARTIST,
    &DEFENDERS_WILL,
    &BRAVE_HEART,
    &GAMBLER,
    &SCHOLAR,
    &PRAYERS_FOR_ILLUMINATION,
    &PRAYERS_FOR_DESTINY,
    &PRAYERS_FOR_WISDOM,
    &PRAYERS_TO_SPRINGTIME,
];
