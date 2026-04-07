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
        name: "Mournful Tribute",
        effect: PassiveEffect {
            description: desc!(
                "爆発使用後3.5秒間、月感電DMG+36-84%。月感電反応トリガー後6秒間CRIT DMG+28-56%。エネルギー12-16回復（14秒に1回）"
            ),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "bloodsoaked_ruins_lunar_dmg",
                    description: desc!("爆発使用後3.5秒間、月感電DMG+36-84%（DmgBonusで近似）"),
                    stat: BuffableStat::DmgBonus,
                    value: 0.36,
                    nightsoul_value: None,
                    refinement_values: Some([0.36, 0.48, 0.60, 0.72, 0.84]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "bloodsoaked_ruins_crit_dmg",
                    description: desc!("月感電反応トリガー後6秒間CRIT DMG+28-56%"),
                    stat: BuffableStat::CritDmg,
                    value: 0.28,
                    nightsoul_value: None,
                    refinement_values: Some([0.28, 0.35, 0.42, 0.49, 0.56]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
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
            description: desc!(
                "Ele DMG+12-24%。元素スキル使用後ATK+3.2-6.4%/スタック、未出場時は2倍（最大6スタック）"
            ),
            buffs: &[StatBuff {
                stat: BuffableStat::DmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "calamity_queller_atk_onfield",
                    description: desc!(
                        "元素スキル使用後ATK+3.2-6.4%（1スタック）、最大6スタック（フィールド上）"
                    ),
                    stat: BuffableStat::AtkPercent,
                    value: 0.032,
                    nightsoul_value: None,
                    refinement_values: Some([0.032, 0.040, 0.048, 0.056, 0.064]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(6)),
                },
                ConditionalBuff {
                    name: "calamity_queller_atk_offfield",
                    description: desc!("未出場時はATKスタック効果2倍（追加ATK+3.2-6.4%/スタック）"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.032,
                    nightsoul_value: None,
                    refinement_values: Some([0.032, 0.040, 0.048, 0.056, 0.064]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(6)),
                },
            ],
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
        name: "Ashen Sun's Shadow",
        effect: PassiveEffect {
            description: desc!(
                "Bond of Life時DMG+12-28%。BoLがHP上限30%以上の時さらにDMG+24-56%。重撃命中でHP上限の25%分BoL付与（14秒に1回）"
            ),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "crimson_moons_semblance_bol_dmg",
                    description: desc!("Bond of Life保有時にDMG+12-28%"),
                    stat: BuffableStat::DmgBonus,
                    value: 0.12,
                    nightsoul_value: None,
                    refinement_values: Some([0.12, 0.16, 0.20, 0.24, 0.28]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "crimson_moons_semblance_bol_high_dmg",
                    description: desc!("BoLがHP上限の30%以上の時さらにDMG+24-56%"),
                    stat: BuffableStat::DmgBonus,
                    value: 0.24,
                    nightsoul_value: None,
                    refinement_values: Some([0.24, 0.32, 0.40, 0.48, 0.56]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
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
            description: desc!("Conditional: ERに基づきATKアップ。元素爆発後にER+30%"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "engulfing_er_atk",
                    description: desc!("ER超過分の28-56%をATK%に変換 (cap: 80-120%)"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.28,
                    nightsoul_value: None,
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
                    description: desc!("元素爆発後12秒間ER+30-50%"),
                    stat: BuffableStat::EnergyRecharge,
                    value: 0.30,
                    nightsoul_value: None,
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
        name: "Purifying Crown",
        effect: PassiveEffect {
            description: desc!(
                "スキル/爆発使用後20秒間ATK+24-48%。シールド生成時に20秒間、チーム全員の月感電DMG+40-80%"
            ),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "fractured_halo_atk",
                    description: desc!("スキル/爆発使用後20秒間ATK+24-48%"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.24,
                    nightsoul_value: None,
                    refinement_values: Some([0.24, 0.30, 0.36, 0.42, 0.48]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "fractured_halo_team_lunar_dmg",
                    description: desc!(
                        "シールド生成時チーム全員の月感電DMG+40-80%（DmgBonusで近似）"
                    ),
                    stat: BuffableStat::DmgBonus,
                    value: 0.40,
                    nightsoul_value: None,
                    refinement_values: Some([0.40, 0.50, 0.60, 0.70, 0.80]),
                    stack_values: None,
                    target: BuffTarget::Team,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
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
        name: "Bright Dawn Overture",
        effect: PassiveEffect {
            description: desc!(
                "ATK+15-31%。燃焼反応トリガー後/燃焼状態の敵に草元素DMG後8秒間ダメージ+18-38%（最大2スタック）。2スタック時/更新時エネルギー12-16回復（12秒に1回）"
            ),
            buffs: &[StatBuff {
                stat: BuffableStat::AtkPercent,
                value: 0.15,
                refinement_values: Some([0.15, 0.19, 0.23, 0.27, 0.31]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "lumidouce_elegy_burning_dmg",
                description: desc!(
                    "燃焼反応トリガー/燃焼状態の敵に草元素DMG後8秒間DMG+18-38%（1スタック）、最大2スタック"
                ),
                stat: BuffableStat::DmgBonus,
                value: 0.18,
                nightsoul_value: None,
                refinement_values: Some([0.18, 0.23, 0.28, 0.33, 0.38]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(2)),
            }],
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
            description: desc!("命中時にATK+3.2-6%、6スタックまで。フルスタックでDMG+12-24%"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "pjws_atk_stacks",
                    description: desc!("命中時にATK+3.2-6%（1スタック）、最大6スタック"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.032,
                    nightsoul_value: None,
                    refinement_values: Some([0.032, 0.039, 0.046, 0.053, 0.060]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(6)),
                },
                ConditionalBuff {
                    name: "pjws_full_stack_dmg",
                    description: desc!("フルスタック時にDMG+12-24%"),
                    stat: BuffableStat::DmgBonus,
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
            description: desc!("CRIT Rate+8-16%。攻撃速度+12%。通常/重撃命中時に追加ダメージ"),
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
            description: desc!(
                "HP+20-40%。HP上限の0.8-1.6%分ATKアップ。HP50%以下でさらに1.0-1.8%分アップ"
            ),
            buffs: &[StatBuff {
                stat: BuffableStat::HpPercent,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "homa_hp_atk",
                    description: desc!("HP上限の0.8%分ATKアップ（常時）"),
                    stat: BuffableStat::AtkFlat,
                    value: 0.008,
                    nightsoul_value: None,
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
                    description: desc!("HP50%以下の時、さらにHP上限の1.0%分ATKアップ"),
                    stat: BuffableStat::AtkFlat,
                    value: 0.010,
                    nightsoul_value: None,
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
            description: desc!("Conditional: EMに基づきATKアップ。スキル命中でさらにATKアップ"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "scarlet_sands_em_atk",
                    description: desc!("EM×52-104%分をATKフラットに加算"),
                    stat: BuffableStat::AtkFlat,
                    value: 0.52,
                    nightsoul_value: None,
                    refinement_values: Some([0.52, 0.65, 0.78, 0.91, 1.04]),
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
                    name: "scarlet_sands_skill_stacks",
                    description: desc!("スキル命中後10秒間、EM×28-56%分ATKアップ。最大2スタック"),
                    stat: BuffableStat::AtkFlat,
                    value: 0.28,
                    nightsoul_value: None,
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
        name: "Seasoned Symphony",
        effect: PassiveEffect {
            description: desc!(
                "ATK+12-24%。未出場時さらにATK+12-24%。ヒール発動後に装備者とヒール対象のATK+32-64%（3秒間）"
            ),
            buffs: &[StatBuff {
                stat: BuffableStat::AtkPercent,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "symphonist_of_scents_offfield_atk",
                    description: desc!("未出場時さらにATK+12-24%"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.12,
                    nightsoul_value: None,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "symphonist_of_scents_sweet_echoes",
                    description: desc!("ヒール発動後に装備者とヒール対象のATK+32-64%（3秒間）"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.32,
                    nightsoul_value: None,
                    refinement_values: Some([0.32, 0.40, 0.48, 0.56, 0.64]),
                    stack_values: None,
                    target: BuffTarget::Team,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
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
            description: desc!("攻撃命中でATK+4-8%スタック（最大5）、シールド時は2倍"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "vortex_vanquisher_atk_stacks",
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
                    name: "vortex_vanquisher_shield_atk_stacks",
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
            description: desc!("チーム内に3種以上の異なる元素があるときEM+120-240"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "ballad_of_the_fjords_em",
                description: desc!("チーム内に3種以上の異なる元素があるときEM+120-240"),
                stat: BuffableStat::ElementalMastery,
                value: 120.0,
                nightsoul_value: None,
                refinement_values: Some([120.0, 150.0, 180.0, 210.0, 240.0]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Toggle),
            }],
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
            description: desc!("Conditional: 元素粒子取得後に通常/重撃で追加ATKダメージ"),
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
            description: desc!("敵2体以上: ATK/DEF+16-32%。敵1体以下: ATK+24-48%"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "deathmatch_atk_multi",
                    description: desc!("敵2体以上でATK+16-32%"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.16,
                    nightsoul_value: None,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "deathmatch_def_multi",
                    description: desc!("敵2体以上でDEF+16-32%"),
                    stat: BuffableStat::DefPercent,
                    value: 0.16,
                    nightsoul_value: None,
                    refinement_values: Some([0.16, 0.20, 0.24, 0.28, 0.32]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "deathmatch_atk_1enemy",
                    description: desc!("敵1体以下でATK+24-48%"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.24,
                    nightsoul_value: None,
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
        name: "Principle of Equilibrium",
        effect: PassiveEffect {
            description: desc!(
                "SKIP: ヒール発動時にエネルギー8-16回復（10秒に1回、未出場時も有効）"
            ),
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
            description: desc!("水/炎元素影響下の敵へのDMG+20-36%"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "dragons_bane_dmg",
                description: desc!("水/炎元素の影響下の敵へのDMG+20-36%"),
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
            description: desc!(
                "Conditional: 通常/重撃命中時に氷柱落下、氷元素影響下の敵には追加ダメージ"
            ),
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
            description: desc!("Conditional: 会心命中時に元素粒子を生成、12-6秒に1回"),
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
        name: "Pact of Flowing Springs",
        effect: PassiveEffect {
            description: desc!("元素スキル使用後15秒間DEF+16-32%"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "footprint_of_the_rainbow_def",
                description: desc!("元素スキル使用後15秒間DEF+16-32%"),
                stat: BuffableStat::DefPercent,
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
            description: desc!("Skill DMG+6-12%。スキル命中で元素エネルギー消費後に回復"),
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
            description: desc!("Conditional: チーム内の璃月キャラ人数に応じてATK/CRIT Rateアップ"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "lithic_spear_atk",
                    description: desc!("璃月キャラ1人につきATK+7-11%"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.07,
                    nightsoul_value: None,
                    refinement_values: Some([0.07, 0.08, 0.09, 0.10, 0.11]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Auto(AutoCondition::TeamRegionCount {
                        region: Region::Liyue,
                    }),
                },
                ConditionalBuff {
                    name: "lithic_spear_crit",
                    description: desc!("璃月キャラ1人につきCR+3-7%"),
                    stat: BuffableStat::CritRate,
                    value: 0.03,
                    nightsoul_value: None,
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
            description: desc!("元素反応トリガー後10秒間ATK+12-24%、EM+48-96"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "missive_windspear_atk",
                    description: desc!("元素反応トリガー後10秒間ATK+12-24%"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.12,
                    nightsoul_value: None,
                    refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
                ConditionalBuff {
                    name: "missive_windspear_em",
                    description: desc!("元素反応トリガー後10秒間EM+48-96"),
                    stat: BuffableStat::ElementalMastery,
                    value: 48.0,
                    nightsoul_value: None,
                    refinement_values: Some([48.0, 60.0, 72.0, 84.0, 96.0]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Toggle),
                },
            ],
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
            description: desc!("草元素反応で種を生成、種を取得するとATK+16-32%"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "moonpiercer_atk",
                description: desc!("草元素反応で生成した種を取得するとATK+16-32%"),
                stat: BuffableStat::AtkPercent,
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

pub const MOUNTAIN_BRACING_BOLT: WeaponData = WeaponData {
    id: "mountain_bracing_bolt",
    name: "Mountain-Bracing Bolt",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "Hope Beyond the Peaks",
        effect: PassiveEffect {
            description: desc!(
                "Skill DMG+12-24%（常時）。他のパーティメンバーがスキル使用後8秒間、さらにSkill DMG+12-24%。登山スタミナ消費-15%"
            ),
            buffs: &[StatBuff {
                stat: BuffableStat::SkillDmgBonus,
                value: 0.12,
                refinement_values: Some([0.12, 0.15, 0.18, 0.21, 0.24]),
            }],
            conditional_buffs: &[ConditionalBuff {
                name: "mountain_bracing_bolt_skill_dmg_team",
                description: desc!(
                    "他のパーティメンバーがスキル使用後8秒間、さらにSkill DMG+12-24%"
                ),
                stat: BuffableStat::SkillDmgBonus,
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

pub const PROSPECTORS_DRILL: WeaponData = WeaponData {
    id: "prospectors_drill",
    name: "Prospector's Drill",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.060, 0.251, 0.251, 0.276])),
    passive: Some(WeaponPassive {
        name: "Masons' Ditty",
        effect: PassiveEffect {
            description: desc!(
                "ヒール時にUnityの印蓄積（最大3）。スキル/爆発使用で全スタック消費→1スタックごとにATK+3-7%+全元素DMG+7-13%（Struggle効果、15秒に1回）"
            ),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "prospectors_drill_atk",
                    description: desc!("Unityの印消費時（1スタックごと）ATK+3-7%、最大3スタック"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.03,
                    nightsoul_value: None,
                    refinement_values: Some([0.03, 0.04, 0.05, 0.06, 0.07]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(3)),
                },
                ConditionalBuff {
                    name: "prospectors_drill_ele_dmg",
                    description: desc!(
                        "Unityの印消費時（1スタックごと）全元素DMG+7-13%、最大3スタック"
                    ),
                    stat: BuffableStat::DmgBonus,
                    value: 0.07,
                    nightsoul_value: None,
                    refinement_values: Some([0.07, 0.085, 0.10, 0.115, 0.13]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(3)),
                },
            ],
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
        name: "Swift and Sure",
        effect: PassiveEffect {
            description: desc!(
                "SKIP(反応特化): 感電DMG+48-96%、月感電DMG+12-24%（常時）。Nod-Kraiキャラ2人以上時さらに月感電DMG+12-24%。反応特化BuffableStatなし→実装対象外"
            ),
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
            description: desc!("元素スキル使用後に通常/重撃DMG+8-16%（最大2スタック）"),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "prototype_starglitter_na_dmg",
                    description: desc!(
                        "元素スキル使用後に通常攻撃DMG+8-16%（1スタック）、最大2スタック"
                    ),
                    stat: BuffableStat::NormalAtkDmgBonus,
                    value: 0.08,
                    nightsoul_value: None,
                    refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(2)),
                },
                ConditionalBuff {
                    name: "prototype_starglitter_ca_dmg",
                    description: desc!(
                        "元素スキル使用後に重撃DMG+8-16%（1スタック）、最大2スタック"
                    ),
                    stat: BuffableStat::ChargedAtkDmgBonus,
                    value: 0.08,
                    nightsoul_value: None,
                    refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(2)),
                },
            ],
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
            description: desc!("Conditional: 元素スキル命中でHP回復"),
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
            description: desc!(
                "ダメージを与えるとCRIT Rate+8-16%（最大5スタック）、会心発生でリセット"
            ),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "royal_spear_cr",
                description: desc!(
                    "ダメージを与えるとCRIT Rate+8-16%（1スタック）、最大5スタック（会心でリセット）"
                ),
                stat: BuffableStat::CritRate,
                value: 0.08,
                nightsoul_value: None,
                refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                stack_values: None,
                target: BuffTarget::OnlySelf,
                activation: Activation::Manual(ManualCondition::Stacks(5)),
            }],
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
        name: "Untainted Desire",
        effect: PassiveEffect {
            description: desc!(
                "スキル命中後6秒間、ATK+8-16%・ER+6-12%（最大3スタック、未出場時も有効）"
            ),
            buffs: &[],
            conditional_buffs: &[
                ConditionalBuff {
                    name: "sacrificers_staff_atk",
                    description: desc!("スキル命中後6秒間ATK+8-16%（1スタック）、最大3スタック"),
                    stat: BuffableStat::AtkPercent,
                    value: 0.08,
                    nightsoul_value: None,
                    refinement_values: Some([0.08, 0.10, 0.12, 0.14, 0.16]),
                    stack_values: None,
                    target: BuffTarget::OnlySelf,
                    activation: Activation::Manual(ManualCondition::Stacks(3)),
                },
                ConditionalBuff {
                    name: "sacrificers_staff_er",
                    description: desc!("スキル命中後6秒間ER+6-12%（1スタック）、最大3スタック"),
                    stat: BuffableStat::EnergyRecharge,
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

pub const TAMAYURATEI_NO_OHANASHI: WeaponData = WeaponData {
    id: "tamayuratei_no_ohanashi",
    name: "Tamayuratei no Ohanashi",
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    base_atk: [44.0, 497.0, 523.0, 565.0],
    sub_stat: Some(WeaponSubStat::EnergyRecharge([0.067, 0.281, 0.281, 0.306])),
    passive: Some(WeaponPassive {
        name: "Busybody's Running Light",
        effect: PassiveEffect {
            description: desc!(
                "元素スキル使用後10秒間ATK+20-40%。移動速度+10%（ダメージ計算無関係）"
            ),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "tamayuratei_no_ohanashi_atk",
                description: desc!("元素スキル使用後10秒間ATK+20-40%"),
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
            description: desc!("Burst DMG+16-32%、Burst CRIT Rate+6-12%"),
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
            description: desc!(
                "チーム全員の元素エネルギー上限合計に基づきBurst DMG+最大40-80%（総EP240超時）"
            ),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "wavebreakers_fin_burst_dmg",
                description: desc!(
                    "チーム全員の元素エネルギー上限合計に基づきBurst DMG+最大40-80%（240EP基準）"
                ),
                stat: BuffableStat::BurstDmgBonus,
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
            description: desc!("スライムへのDMG+40-80%"),
            buffs: &[],
            conditional_buffs: &[ConditionalBuff {
                name: "black_tassel_slime_dmg",
                description: desc!("スライムへのDMG+40-80%"),
                stat: BuffableStat::DmgBonus,
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
            description: desc!("Conditional: 通常攻撃命中時に追加ATKダメージ、10秒に1回"),
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
            description: desc!("NA DMG+24-48%"),
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

        // Primary: EM → ATK flat (Both StatScaling + Toggle)
        let buff = &cond_buffs[0];
        assert_eq!(buff.name, "scarlet_sands_em_atk");
        assert_eq!(buff.stat, BuffableStat::AtkFlat);
        assert!((buff.value - 0.52).abs() < 1e-6);
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

    #[test]
    fn calamity_queller_has_atk_stacks() {
        let passive = CALAMITY_QUELLER.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);
        assert_eq!(cond[0].name, "calamity_queller_atk_onfield");
        assert_eq!(cond[0].stat, BuffableStat::AtkPercent);
        assert!((cond[0].value - 0.032).abs() < 1e-6);
        assert!(matches!(
            cond[0].activation,
            Activation::Manual(ManualCondition::Stacks(6))
        ));
        assert_eq!(cond[1].name, "calamity_queller_atk_offfield");
        assert!(cond[0].refinement_values.is_some());
    }

    #[test]
    fn lumidouce_elegy_has_atk_buff_and_burning_stacks() {
        let passive = LUMIDOUCE_ELEGY.passive.unwrap();
        assert_eq!(passive.effect.buffs.len(), 1);
        assert_eq!(passive.effect.buffs[0].stat, BuffableStat::AtkPercent);
        assert!((passive.effect.buffs[0].value - 0.15).abs() < 1e-6);
        let rv = passive.effect.buffs[0].refinement_values.unwrap();
        assert!((rv[4] - 0.31).abs() < 1e-6);
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "lumidouce_elegy_burning_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.18).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(2))
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.38).abs() < 1e-6);
    }

    #[test]
    fn ballad_of_the_fjords_has_em_toggle() {
        let passive = BALLAD_OF_THE_FJORDS.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "ballad_of_the_fjords_em");
        assert_eq!(buff.stat, BuffableStat::ElementalMastery);
        assert!((buff.value - 120.0).abs() < 1e-6);
        assert_eq!(buff.target, BuffTarget::OnlySelf);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 240.0).abs() < 1e-6);
    }

    #[test]
    fn missive_windspear_has_atk_and_em_toggle() {
        let passive = MISSIVE_WINDSPEAR.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);
        let atk = &cond[0];
        assert_eq!(atk.name, "missive_windspear_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert!((atk.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            atk.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = atk.refinement_values.unwrap();
        assert!((rv[4] - 0.24).abs() < 1e-6);
        let em = &cond[1];
        assert_eq!(em.name, "missive_windspear_em");
        assert_eq!(em.stat, BuffableStat::ElementalMastery);
        assert!((em.value - 48.0).abs() < 1e-6);
        let rv2 = em.refinement_values.unwrap();
        assert!((rv2[4] - 96.0).abs() < 1e-6);
    }

    #[test]
    fn moonpiercer_has_atk_toggle() {
        let passive = MOONPIERCER.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "moonpiercer_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.16).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.32).abs() < 1e-6);
    }

    #[test]
    fn prototype_starglitter_has_na_ca_stacks() {
        let passive = PROTOTYPE_STARGLITTER.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);
        let na = &cond[0];
        assert_eq!(na.name, "prototype_starglitter_na_dmg");
        assert_eq!(na.stat, BuffableStat::NormalAtkDmgBonus);
        assert!((na.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            na.activation,
            Activation::Manual(ManualCondition::Stacks(2))
        ));
        let ca = &cond[1];
        assert_eq!(ca.name, "prototype_starglitter_ca_dmg");
        assert_eq!(ca.stat, BuffableStat::ChargedAtkDmgBonus);
        assert!((ca.value - 0.08).abs() < 1e-6);
        let rv = ca.refinement_values.unwrap();
        assert!((rv[4] - 0.16).abs() < 1e-6);
    }

    #[test]
    fn royal_spear_has_cr_stacks() {
        let passive = ROYAL_SPEAR.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "royal_spear_cr");
        assert_eq!(buff.stat, BuffableStat::CritRate);
        assert!((buff.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Stacks(5))
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.16).abs() < 1e-6);
    }

    #[test]
    fn wavebreakers_fin_has_burst_dmg_toggle() {
        let passive = WAVEBREAKERS_FIN.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "wavebreakers_fin_burst_dmg");
        assert_eq!(buff.stat, BuffableStat::BurstDmgBonus);
        assert!((buff.value - 0.40).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.80).abs() < 1e-6);
    }

    #[test]
    fn black_tassel_has_slime_dmg_toggle() {
        let passive = BLACK_TASSEL.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "black_tassel_slime_dmg");
        assert_eq!(buff.stat, BuffableStat::DmgBonus);
        assert!((buff.value - 0.40).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.80).abs() < 1e-6);
    }

    #[test]
    fn bloodsoaked_ruins_has_lunar_dmg_and_crit_dmg() {
        let passive = BLOODSOAKED_RUINS.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);
        let lunar = &cond[0];
        assert_eq!(lunar.name, "bloodsoaked_ruins_lunar_dmg");
        assert_eq!(lunar.stat, BuffableStat::DmgBonus);
        assert!((lunar.value - 0.36).abs() < 1e-6);
        assert!(matches!(
            lunar.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = lunar.refinement_values.unwrap();
        assert!((rv[4] - 0.84).abs() < 1e-6);
        let cd = &cond[1];
        assert_eq!(cd.name, "bloodsoaked_ruins_crit_dmg");
        assert_eq!(cd.stat, BuffableStat::CritDmg);
        assert!((cd.value - 0.28).abs() < 1e-6);
        let rv2 = cd.refinement_values.unwrap();
        assert!((rv2[4] - 0.56).abs() < 1e-6);
    }

    #[test]
    fn crimson_moons_semblance_has_bol_dmg_toggles() {
        let passive = CRIMSON_MOONS_SEMBLANCE.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);
        let base = &cond[0];
        assert_eq!(base.name, "crimson_moons_semblance_bol_dmg");
        assert_eq!(base.stat, BuffableStat::DmgBonus);
        assert!((base.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            base.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = base.refinement_values.unwrap();
        assert!((rv[4] - 0.28).abs() < 1e-6);
        let high = &cond[1];
        assert_eq!(high.name, "crimson_moons_semblance_bol_high_dmg");
        assert!((high.value - 0.24).abs() < 1e-6);
        let rv2 = high.refinement_values.unwrap();
        assert!((rv2[4] - 0.56).abs() < 1e-6);
    }

    #[test]
    fn fractured_halo_has_atk_and_team_lunar_dmg() {
        let passive = FRACTURED_HALO.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);
        let atk = &cond[0];
        assert_eq!(atk.name, "fractured_halo_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert!((atk.value - 0.24).abs() < 1e-6);
        assert_eq!(atk.target, BuffTarget::OnlySelf);
        let rv = atk.refinement_values.unwrap();
        assert!((rv[4] - 0.48).abs() < 1e-6);
        let team = &cond[1];
        assert_eq!(team.name, "fractured_halo_team_lunar_dmg");
        assert_eq!(team.stat, BuffableStat::DmgBonus);
        assert!((team.value - 0.40).abs() < 1e-6);
        assert_eq!(team.target, BuffTarget::Team);
        let rv2 = team.refinement_values.unwrap();
        assert!((rv2[4] - 0.80).abs() < 1e-6);
    }

    #[test]
    fn symphonist_of_scents_has_statbuff_and_conditionals() {
        let passive = SYMPHONIST_OF_SCENTS.passive.unwrap();
        assert_eq!(passive.effect.buffs.len(), 1);
        assert_eq!(passive.effect.buffs[0].stat, BuffableStat::AtkPercent);
        assert!((passive.effect.buffs[0].value - 0.12).abs() < 1e-6);
        let rv = passive.effect.buffs[0].refinement_values.unwrap();
        assert!((rv[4] - 0.24).abs() < 1e-6);
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);
        assert_eq!(cond[0].name, "symphonist_of_scents_offfield_atk");
        assert_eq!(cond[0].stat, BuffableStat::AtkPercent);
        assert!((cond[0].value - 0.12).abs() < 1e-6);
        assert!(matches!(
            cond[0].activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        assert_eq!(cond[1].name, "symphonist_of_scents_sweet_echoes");
        assert!((cond[1].value - 0.32).abs() < 1e-6);
        assert_eq!(cond[1].target, BuffTarget::Team);
        let rv2 = cond[1].refinement_values.unwrap();
        assert!((rv2[4] - 0.64).abs() < 1e-6);
    }

    #[test]
    fn dialogues_of_the_desert_sages_is_skip() {
        let passive = DIALOGUES_OF_THE_DESERT_SAGES.passive.unwrap();
        assert_eq!(passive.effect.buffs.len(), 0);
        assert_eq!(passive.effect.conditional_buffs.len(), 0);
    }

    #[test]
    fn footprint_of_the_rainbow_has_def_toggle() {
        let passive = FOOTPRINT_OF_THE_RAINBOW.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "footprint_of_the_rainbow_def");
        assert_eq!(buff.stat, BuffableStat::DefPercent);
        assert!((buff.value - 0.16).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.32).abs() < 1e-6);
    }

    #[test]
    fn mountain_bracing_bolt_has_statbuff_and_team_conditional() {
        let passive = MOUNTAIN_BRACING_BOLT.passive.unwrap();
        assert_eq!(passive.effect.buffs.len(), 1);
        assert_eq!(passive.effect.buffs[0].stat, BuffableStat::SkillDmgBonus);
        assert!((passive.effect.buffs[0].value - 0.12).abs() < 1e-6);
        let rv = passive.effect.buffs[0].refinement_values.unwrap();
        assert!((rv[4] - 0.24).abs() < 1e-6);
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "mountain_bracing_bolt_skill_dmg_team");
        assert_eq!(buff.stat, BuffableStat::SkillDmgBonus);
        assert!((buff.value - 0.12).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv2 = buff.refinement_values.unwrap();
        assert!((rv2[4] - 0.24).abs() < 1e-6);
    }

    #[test]
    fn prospectors_drill_has_atk_and_ele_dmg_stacks() {
        let passive = PROSPECTORS_DRILL.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);
        let atk = &cond[0];
        assert_eq!(atk.name, "prospectors_drill_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert!((atk.value - 0.03).abs() < 1e-6);
        assert!(matches!(
            atk.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        let rv = atk.refinement_values.unwrap();
        assert!((rv[4] - 0.07).abs() < 1e-6);
        let ele = &cond[1];
        assert_eq!(ele.name, "prospectors_drill_ele_dmg");
        assert_eq!(ele.stat, BuffableStat::DmgBonus);
        assert!((ele.value - 0.07).abs() < 1e-6);
        let rv2 = ele.refinement_values.unwrap();
        assert!((rv2[4] - 0.13).abs() < 1e-6);
    }

    #[test]
    fn prospectors_shovel_is_skip() {
        let passive = PROSPECTORS_SHOVEL.passive.unwrap();
        assert_eq!(passive.effect.buffs.len(), 0);
        assert_eq!(passive.effect.conditional_buffs.len(), 0);
    }

    #[test]
    fn sacrificers_staff_has_atk_and_er_stacks() {
        let passive = SACRIFICERS_STAFF.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 2);
        let atk = &cond[0];
        assert_eq!(atk.name, "sacrificers_staff_atk");
        assert_eq!(atk.stat, BuffableStat::AtkPercent);
        assert!((atk.value - 0.08).abs() < 1e-6);
        assert!(matches!(
            atk.activation,
            Activation::Manual(ManualCondition::Stacks(3))
        ));
        let rv = atk.refinement_values.unwrap();
        assert!((rv[4] - 0.16).abs() < 1e-6);
        let er = &cond[1];
        assert_eq!(er.name, "sacrificers_staff_er");
        assert_eq!(er.stat, BuffableStat::EnergyRecharge);
        assert!((er.value - 0.06).abs() < 1e-6);
        let rv2 = er.refinement_values.unwrap();
        assert!((rv2[4] - 0.12).abs() < 1e-6);
    }

    #[test]
    fn tamayuratei_no_ohanashi_has_atk_toggle() {
        let passive = TAMAYURATEI_NO_OHANASHI.passive.unwrap();
        let cond = passive.effect.conditional_buffs;
        assert_eq!(cond.len(), 1);
        let buff = &cond[0];
        assert_eq!(buff.name, "tamayuratei_no_ohanashi_atk");
        assert_eq!(buff.stat, BuffableStat::AtkPercent);
        assert!((buff.value - 0.20).abs() < 1e-6);
        assert!(matches!(
            buff.activation,
            Activation::Manual(ManualCondition::Toggle)
        ));
        let rv = buff.refinement_values.unwrap();
        assert!((rv[4] - 0.40).abs() < 1e-6);
    }
}
