use crate::buff::{BuffableStat, StatBuff};
use crate::types::{ArtifactRarity, ArtifactSet, SetEffect};
use genshin_calc_core::Element;

// =============================================================================
// 5-star Artifact Sets
// =============================================================================

pub const CRIMSON_WITCH: ArtifactSet = ArtifactSet {
    id: "crimson_witch",
    name: "燃え盛る炎の魔女",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "炎元素ダメージ+15%",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Pyro),
            value: 0.15,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "過負荷、燃焼、烈開花反応ダメージ+40%。蒸発、溶解反応倍率+15%。元素スキル使用後2セット効果+50%、最大3スタック",
        buffs: &[],
    },
};

pub const GLADIATORS_FINALE: ArtifactSet = ArtifactSet {
    id: "gladiators_finale",
    name: "剣闘士のフィナーレ",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "攻撃力+18%",
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "該当キャラクターが片手剣、両手剣、長柄武器キャラの場合、通常攻撃ダメージ+35%",
        buffs: &[],
    },
};

pub const WANDERERS_TROUPE: ArtifactSet = ArtifactSet {
    id: "wanderers_troupe",
    name: "大地を流浪する楽団",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "元素熟知+80",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalMastery,
            value: 80.0,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "該当キャラクターが法器、弓キャラの場合、重撃ダメージ+35%",
        buffs: &[],
    },
};

pub const NOBLESSE_OBLIGE: ArtifactSet = ArtifactSet {
    id: "noblesse_oblige",
    name: "旧貴族のしつけ",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "元素爆発のダメージ+20%",
        buffs: &[StatBuff {
            stat: BuffableStat::BurstDmgBonus,
            value: 0.20,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素爆発を発動すると、チーム全員の攻撃力+20%、継続時間12秒、重ねがけ不可",
        buffs: &[],
    },
};

pub const BLOODSTAINED_CHIVALRY: ArtifactSet = ArtifactSet {
    id: "bloodstained_chivalry",
    name: "血染めの騎士道",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "物理ダメージ+25%",
        buffs: &[StatBuff {
            stat: BuffableStat::PhysicalDmgBonus,
            value: 0.25,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "敵を倒した10秒以内に、重撃のスタミナ消費-0、ダメージ+50%",
        buffs: &[],
    },
};

pub const THUNDERING_FURY: ArtifactSet = ArtifactSet {
    id: "thundering_fury",
    name: "雷のような怒り",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "雷元素ダメージ+15%",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Electro),
            value: 0.15,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "過負荷、感電、超電導、超激化反応ダメージ+40%。超開花反応ダメージ+20%。上記反応発生後、元素スキルのクールタイム-1秒。0.8秒ごとに1回のみ発動可能",
        buffs: &[],
    },
};

pub const THUNDERSOOTHER: ArtifactSet = ArtifactSet {
    id: "thundersoother",
    name: "雷を鎮める尊者",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "雷元素耐性+40%",
        buffs: &[],
    },
    four_piece: SetEffect {
        description: "雷元素の影響を受けた敵に対するダメージ+35%",
        buffs: &[],
    },
};

pub const VIRIDESCENT_VENERER: ArtifactSet = ArtifactSet {
    id: "viridescent_venerer",
    name: "翠緑の影",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "風元素ダメージ+15%",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
            value: 0.15,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "拡散反応ダメージ+60%。拡散反応に対応する元素の敵耐性-40%、継続時間10秒",
        buffs: &[],
    },
};

pub const MAIDEN_BELOVED: ArtifactSet = ArtifactSet {
    id: "maiden_beloved",
    name: "愛される少女",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "与える治療効果+15%",
        buffs: &[StatBuff {
            stat: BuffableStat::HealingBonus,
            value: 0.15,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素スキルまたは元素爆発を発動した後10秒間、チーム全員が受ける治療効果+20%",
        buffs: &[],
    },
};

pub const ARCHAIC_PETRA: ArtifactSet = ArtifactSet {
    id: "archaic_petra",
    name: "悠久の磐岩",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "岩元素ダメージ+15%",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Geo),
            value: 0.15,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "結晶反応で形成された欠片を獲得すると、チーム全員の該当元素ダメージ+35%、継続時間10秒。同時に1つの元素ダメージボーナスのみ獲得可能",
        buffs: &[],
    },
};

pub const RETRACING_BOLIDE: ArtifactSet = ArtifactSet {
    id: "retracing_bolide",
    name: "逆飛びの流星",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "シールド強化+35%",
        buffs: &[StatBuff {
            stat: BuffableStat::ShieldStrength,
            value: 0.35,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "シールド状態の時、通常攻撃と重撃ダメージ+40%",
        buffs: &[],
    },
};

pub const LAVAWALKER: ArtifactSet = ArtifactSet {
    id: "lavawalker",
    name: "烈火を渡る賢者",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "炎元素耐性+40%",
        buffs: &[],
    },
    four_piece: SetEffect {
        description: "炎元素の影響を受けた敵に対するダメージ+35%",
        buffs: &[],
    },
};

pub const BLIZZARD_STRAYER: ArtifactSet = ArtifactSet {
    id: "blizzard_strayer",
    name: "氷風を彷徨う勇士",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "氷元素ダメージ+15%",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
            value: 0.15,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "氷元素の影響を受けた敵を攻撃した場合、会心率+20%。敵が凍結状態の場合、会心率は更に+20%",
        buffs: &[],
    },
};

pub const HEART_OF_DEPTH: ArtifactSet = ArtifactSet {
    id: "heart_of_depth",
    name: "沈淪の心",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "水元素ダメージ+15%",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
            value: 0.15,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素スキル発動後15秒間、通常攻撃と重撃のダメージ+30%",
        buffs: &[],
    },
};

pub const TENACITY_OF_THE_MILLELITH: ArtifactSet = ArtifactSet {
    id: "tenacity_of_the_millelith",
    name: "千岩牢固",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "HP+20%",
        buffs: &[StatBuff {
            stat: BuffableStat::HpPercent,
            value: 0.20,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素スキルが敵に命中すると、周囲のチーム全員の攻撃力+20%、シールド強化+30%、継続時間3秒。0.5秒ごとに1回のみ発動可能。この効果はキャラクターが待機中でも発動可能",
        buffs: &[],
    },
};

pub const PALE_FLAME: ArtifactSet = ArtifactSet {
    id: "pale_flame",
    name: "蒼白の炎",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "物理ダメージ+25%",
        buffs: &[StatBuff {
            stat: BuffableStat::PhysicalDmgBonus,
            value: 0.25,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素スキルが敵に命中すると、攻撃力+9%、継続時間7秒、最大2スタック。2スタック時に2セットの効果+100%",
        buffs: &[],
    },
};

pub const SHIMENAWAS_REMINISCENCE: ArtifactSet = ArtifactSet {
    id: "shimenawas_reminiscence",
    name: "追憶のしめ縄",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "攻撃力+18%",
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素スキルを発動した時、元素エネルギーが15以上の場合、元素エネルギーを15消費し、通常攻撃、重撃、落下攻撃ダメージ+50%、継続時間10秒。この効果は継続時間中に再発動不可",
        buffs: &[],
    },
};

pub const EMBLEM_OF_SEVERED_FATE: ArtifactSet = ArtifactSet {
    id: "emblem_of_severed_fate",
    name: "絶縁の旗印",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "元素チャージ効率+20%",
        buffs: &[StatBuff {
            stat: BuffableStat::EnergyRecharge,
            value: 0.20,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素チャージ効率の25%を基準に、元素爆発のダメージがアップする。この方式でアップできるダメージは最大75%まで",
        buffs: &[],
    },
};

pub const HUSK_OF_OPULENT_DREAMS: ArtifactSet = ArtifactSet {
    id: "husk_of_opulent_dreams",
    name: "華館夢醒形骸記",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "防御力+30%",
        buffs: &[StatBuff {
            stat: BuffableStat::DefPercent,
            value: 0.30,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "装備キャラクターがフィールドにいる時、岩元素ダメージを与えた0.3秒後に「問答」スタックを1獲得、最大4スタック。1スタックにつき防御力+6%と岩元素ダメージ+6%。6秒ごとに「問答」スタックを獲得できない場合、スタック-1。装備キャラクターが待機中の場合、3秒ごとに「問答」スタック+1",
        buffs: &[],
    },
};

pub const OCEAN_HUED_CLAM: ArtifactSet = ArtifactSet {
    id: "ocean_hued_clam",
    name: "海染硨磲",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "与える治療効果+15%",
        buffs: &[StatBuff {
            stat: BuffableStat::HealingBonus,
            value: 0.15,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "装備キャラクターが治療を行うと「海染の泡」が生成され、3秒間治療量を蓄積する。蓄積時間終了時、泡が破裂し周囲の敵にHP回復量の90%分のダメージを与える（物理ダメージ）。最大30000まで蓄積可能",
        buffs: &[],
    },
};

pub const VERMILLION_HEREAFTER: ArtifactSet = ArtifactSet {
    id: "vermillion_hereafter",
    name: "辰砂往生録",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "攻撃力+18%",
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素爆発を発動した後、攻撃力+8%。さらにHPが減少するたびに攻撃力+10%、最大4スタック。この効果は継続時間16秒。HPが増加した場合もスタック解除されない。スタック数は0.8秒に最大1回のみ変動",
        buffs: &[],
    },
};

pub const ECHOES_OF_AN_OFFERING: ArtifactSet = ArtifactSet {
    id: "echoes_of_an_offering",
    name: "来歆の余響",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "攻撃力+18%",
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "通常攻撃が命中した時、36%の確率で「谷の念」が発動し、通常攻撃のダメージが攻撃力の70%分アップする。発動しなかった場合、次回の発動確率+20%。0.2秒ごとに1回のみ判定",
        buffs: &[],
    },
};

pub const DEEPWOOD_MEMORIES: ArtifactSet = ArtifactSet {
    id: "deepwood_memories",
    name: "深林の記憶",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "草元素ダメージ+15%",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Dendro),
            value: 0.15,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素スキルまたは元素爆発が敵に命中した後、敵の草元素耐性-30%、継続時間8秒。装備キャラクターが待機中でも効果を発動可能",
        buffs: &[],
    },
};

pub const GILDED_DREAMS: ArtifactSet = ArtifactSet {
    id: "gilded_dreams",
    name: "金メッキの夢",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "元素熟知+80",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalMastery,
            value: 80.0,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素反応を起こした後8秒間、チーム内の自身以外のキャラクターの元素タイプに応じてバフを獲得。自身と同じ元素タイプのキャラ1人につき攻撃力+14%、異なる元素タイプのキャラ1人につき元素熟知+50。上記効果は最大3人分まで。0.8秒ごとに1回のみ発動可能。待機中でも効果を発動可能",
        buffs: &[],
    },
};

pub const DESERT_PAVILION_CHRONICLE: ArtifactSet = ArtifactSet {
    id: "desert_pavilion_chronicle",
    name: "砂上の楼閣の史話",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "風元素ダメージ+15%",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
            value: 0.15,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "重撃が敵に命中すると、通常攻撃の攻撃速度+10%、通常攻撃、重撃、落下攻撃のダメージ+40%、継続時間15秒",
        buffs: &[],
    },
};

pub const FLOWER_OF_PARADISE_LOST: ArtifactSet = ArtifactSet {
    id: "flower_of_paradise_lost",
    name: "楽園の絶花",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "元素熟知+80",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalMastery,
            value: 80.0,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "開花、超開花、烈開花反応ダメージ+40%。さらに装備キャラクターが開花、超開花、烈開花反応を起こした後、上記効果+25%、最大4スタック、継続時間10秒。0.8秒ごとに1回のみスタック獲得可能。待機中でも効果を発動可能",
        buffs: &[],
    },
};

pub const NYMPHS_DREAM: ArtifactSet = ArtifactSet {
    id: "nymphs_dream",
    name: "水仙の夢",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "水元素ダメージ+15%",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalDmgBonus(Element::Hydro),
            value: 0.15,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "通常攻撃、重撃、元素スキル、元素爆発が命中した後、1/2/3スタックでそれぞれ水元素ダメージ+7%/16%/25%、攻撃力+4%/9%/15%。各攻撃が命中するたびに、他種類のスタックの持続時間をリセット。各種類のスタックは0.8秒に1回のみ獲得可能",
        buffs: &[],
    },
};

pub const VOURUKASHAS_GLOW: ArtifactSet = ArtifactSet {
    id: "vourukashas_glow",
    name: "花海甘露の光",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "HP+20%",
        buffs: &[StatBuff {
            stat: BuffableStat::HpPercent,
            value: 0.20,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素スキルと元素爆発のダメージ+10%。装備キャラクターがダメージを受けた後、上記効果+80%、継続時間5秒、最大5スタック。各スタックの持続時間は独立。待機中でも効果を発動可能",
        buffs: &[],
    },
};

pub const MARECHAUSSEE_HUNTER: ArtifactSet = ArtifactSet {
    id: "marechaussee_hunter",
    name: "ファントムハンター",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "通常攻撃と重撃ダメージ+15%",
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
    },
    four_piece: SetEffect {
        description: "現在のHPが変動した後、会心率+12%、継続時間5秒、最大3スタック",
        buffs: &[],
    },
};

pub const GOLDEN_TROUPE: ArtifactSet = ArtifactSet {
    id: "golden_troupe",
    name: "黄金の劇団",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "元素スキルのダメージ+20%",
        buffs: &[StatBuff {
            stat: BuffableStat::SkillDmgBonus,
            value: 0.20,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素スキルのダメージ+25%。さらにキャラクターが待機中の場合、元素スキルのダメージ+25%",
        buffs: &[],
    },
};

pub const SONG_OF_DAYS_PAST: ArtifactSet = ArtifactSet {
    id: "song_of_days_past",
    name: "在りし日の歌",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "与える治療効果+15%",
        buffs: &[StatBuff {
            stat: BuffableStat::HealingBonus,
            value: 0.15,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "装備キャラクターがチームメイトを治療した時、治療量を記録する「往日の協奏」効果が発動。6秒後に「往日の協奏」は解消され、その記録した治療量の8%分、チーム全員の通常攻撃、重撃、落下攻撃、元素スキル、元素爆発のダメージをアップ。最大2000まで。「往日の協奏」の持続中に再発動された場合、既存の記録はクリア",
        buffs: &[],
    },
};

pub const NIGHTTIME_WHISPERS_IN_THE_ECHOING_WOODS: ArtifactSet = ArtifactSet {
    id: "nighttime_whispers_in_the_echoing_woods",
    name: "残響の森で囁く夜",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "攻撃力+18%",
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素スキルを使用した後、岩元素ダメージ+20%。夜魂ポイントを消費したとき、岩元素ダメージがさらに+20%、継続時間20秒",
        buffs: &[],
    },
};

pub const FRAGMENT_OF_HARMONIC_WHIMSY: ArtifactSet = ArtifactSet {
    id: "fragment_of_harmonic_whimsy",
    name: "諧律奇想の断章",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "攻撃力+18%",
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "キャラクターのHPが変動した時、与えるダメージ+18%、継続時間6秒、最大3スタック。0.2秒ごとに1回のみ発動可能。待機中でも効果を発動可能",
        buffs: &[],
    },
};

pub const UNFINISHED_REVERIE: ArtifactSet = ArtifactSet {
    id: "unfinished_reverie",
    name: "未完の想起",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "攻撃力+18%",
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "燃焼反応または烈開花反応を起こした後、与えるダメージ+50%、継続時間10秒。上記の効果の継続中にフィールド上にいる場合、3秒後に効果が消える",
        buffs: &[],
    },
};

pub const SCROLL_OF_THE_HERO_OF_CINDER_CITY: ArtifactSet = ArtifactSet {
    id: "scroll_of_the_hero_of_cinder_city",
    name: "灰燼の都の英雄譚",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "元素熟知+80",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalMastery,
            value: 80.0,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "装備キャラクターが夜魂バースト状態にある、または燃焼、超電導等の元素反応を起こした後、チーム全員の対応する元素ダメージ+12%。同時に最大2種類の元素に効果。待機中でも発動可能。持続時間12秒",
        buffs: &[],
    },
};

pub const OBSIDIAN_CODEX: ArtifactSet = ArtifactSet {
    id: "obsidian_codex",
    name: "黒曜の秘典",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "通常攻撃と重撃ダメージ+15%",
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
    },
    four_piece: SetEffect {
        description: "キャラクターが夜魂バースト状態にある時、与えるダメージ+25%。さらにナイトソウルポイントが50%以下の場合、会心率+40%",
        buffs: &[],
    },
};

pub const RESOLUTION_OF_SOJOURNER: ArtifactSet = ArtifactSet {
    id: "resolution_of_sojourner",
    name: "旅人の心",
    rarity: ArtifactRarity::Star4And5,
    two_piece: SetEffect {
        description: "攻撃力+18%",
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "重撃の会心率+30%",
        buffs: &[],
    },
};

pub const TINY_MIRACLE: ArtifactSet = ArtifactSet {
    id: "tiny_miracle",
    name: "奇跡",
    rarity: ArtifactRarity::Star4And5,
    two_piece: SetEffect {
        description: "全元素耐性+20%",
        buffs: &[],
    },
    four_piece: SetEffect {
        description: "元素ダメージを受けた後、その元素の耐性+30%、継続時間10秒。10秒ごとに1回のみ発動可能",
        buffs: &[],
    },
};

pub const BERSERKER: ArtifactSet = ArtifactSet {
    id: "berserker",
    name: "狂戦士",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: "会心率+12%",
        buffs: &[StatBuff {
            stat: BuffableStat::CritRate,
            value: 0.12,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "HPが70%以下になると、会心率+24%",
        buffs: &[],
    },
};

pub const INSTRUCTOR: ArtifactSet = ArtifactSet {
    id: "instructor",
    name: "教官",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: "元素熟知+80",
        buffs: &[StatBuff {
            stat: BuffableStat::ElementalMastery,
            value: 80.0,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素反応を起こした後、チーム全員の元素熟知+120、継続時間8秒",
        buffs: &[],
    },
};

pub const EXILE: ArtifactSet = ArtifactSet {
    id: "exile",
    name: "亡命者",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: "元素チャージ効率+20%",
        buffs: &[StatBuff {
            stat: BuffableStat::EnergyRecharge,
            value: 0.20,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素爆発を発動すると、2秒ごとにチームメイト全員の元素エネルギーを2回復、継続時間6秒。重ねがけ不可",
        buffs: &[],
    },
};

pub const MARTIAL_ARTIST: ArtifactSet = ArtifactSet {
    id: "martial_artist",
    name: "武人",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: "通常攻撃と重撃ダメージ+15%",
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
    },
    four_piece: SetEffect {
        description: "元素スキルまたは元素爆発を発動した後、通常攻撃と重撃のダメージ+25%、継続時間8秒",
        buffs: &[],
    },
};

pub const DEFENDERS_WILL: ArtifactSet = ArtifactSet {
    id: "defenders_will",
    name: "守護の心",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: "防御力+30%",
        buffs: &[StatBuff {
            stat: BuffableStat::DefPercent,
            value: 0.30,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "チーム内キャラクターの元素タイプ1種類につき、対応する元素耐性+30%",
        buffs: &[],
    },
};

pub const BRAVE_HEART: ArtifactSet = ArtifactSet {
    id: "brave_heart",
    name: "勇士の心",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: "攻撃力+18%",
        buffs: &[StatBuff {
            stat: BuffableStat::AtkPercent,
            value: 0.18,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "HPが50%以上の敵に対するダメージ+30%",
        buffs: &[],
    },
};

pub const GAMBLER: ArtifactSet = ArtifactSet {
    id: "gambler",
    name: "博徒",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: "元素スキルのダメージ+20%",
        buffs: &[StatBuff {
            stat: BuffableStat::SkillDmgBonus,
            value: 0.20,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "敵を倒した時、100%の確率で元素スキルのクールタイムをリセットする。15秒ごとに1回のみ発動可能",
        buffs: &[],
    },
};

pub const SCHOLAR: ArtifactSet = ArtifactSet {
    id: "scholar",
    name: "学者",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: "元素チャージ効率+20%",
        buffs: &[StatBuff {
            stat: BuffableStat::EnergyRecharge,
            value: 0.20,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素エネルギーを獲得した時、チーム全員の弓、法器キャラの元素エネルギーを3回復。3秒ごとに1回のみ発動可能",
        buffs: &[],
    },
};

pub const PRAYERS_FOR_ILLUMINATION: ArtifactSet = ArtifactSet {
    id: "prayers_for_illumination",
    name: "火祭りの人",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: "炎元素に影響されている時間-40%",
        buffs: &[],
    },
    four_piece: SetEffect {
        description: "",
        buffs: &[],
    },
};

pub const PRAYERS_FOR_DESTINY: ArtifactSet = ArtifactSet {
    id: "prayers_for_destiny",
    name: "水祭りの人",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: "水元素に影響されている時間-40%",
        buffs: &[],
    },
    four_piece: SetEffect {
        description: "",
        buffs: &[],
    },
};

pub const PRAYERS_FOR_WISDOM: ArtifactSet = ArtifactSet {
    id: "prayers_for_wisdom",
    name: "雷祭りの人",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: "雷元素に影響されている時間-40%",
        buffs: &[],
    },
    four_piece: SetEffect {
        description: "",
        buffs: &[],
    },
};

pub const PRAYERS_TO_SPRINGTIME: ArtifactSet = ArtifactSet {
    id: "prayers_to_springtime",
    name: "氷祭りの人",
    rarity: ArtifactRarity::Star4,
    two_piece: SetEffect {
        description: "氷元素に影響されている時間-40%",
        buffs: &[],
    },
    four_piece: SetEffect {
        description: "",
        buffs: &[],
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
        description: "HP+20%",
        buffs: &[StatBuff {
            stat: BuffableStat::HpPercent,
            value: 0.20,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "現在のHP上限と上限を超えた治療量に基づき通常攻撃と重撃のダメージがアップ。HP上限が30000を超えた場合、超過量1000ごとに通常攻撃と重撃のダメージが+24。超過回復量1000ごとに通常攻撃と重撃のダメージがさらに+32。超過回復効果のスタックは最大6スタック。持続時間6秒",
        buffs: &[],
    },
};

pub const CHRONICLED_SANDS_AND_WATER: ArtifactSet = ArtifactSet {
    id: "chronicled_sands_and_water",
    name: "記憶の砂と水",
    rarity: ArtifactRarity::Star5,
    two_piece: SetEffect {
        description: "元素チャージ効率+20%",
        buffs: &[StatBuff {
            stat: BuffableStat::EnergyRecharge,
            value: 0.20,
            refinement_values: None,
        }],
    },
    four_piece: SetEffect {
        description: "元素チャージ効率の40%に基づいて、元素スキルと元素爆発のダメージアップ。この方式でアップできるダメージは最大80%まで",
        buffs: &[],
    },
};

// =============================================================================
// ALL_ARTIFACT_SETS
// =============================================================================

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
    // 4-star / mixed rarity sets
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
