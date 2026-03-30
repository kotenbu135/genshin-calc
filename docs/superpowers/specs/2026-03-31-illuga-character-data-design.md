# Illuga (Geo/Polearm/★4) Character Data Implementation

## Overview

v6.2 の新キャラクター Illuga のゲームデータを genshin-calc に実装する。
EM+DEF デュアルスケーリングを持つ Geo サポーター。

## Scope

- Illuga の CharacterData 定義（`characters/geo/illuga.rs`）
- `geo/mod.rs` への登録
- 天賦バフ（A4: CRIT Rate/CRIT DMG/EM、Burst: Geo DMG Bonus）
- 月兆データ: benediction エントリ既存（`moonsign_chars.rs:88-95`）、talent enhancement 追加の要否確認
- TOML テストケース作成

## Data Source

Honey Impact (`gensh.honeyhunterworld.com/illuga/?lang=EN`)

## Design

### 1. CharacterData 定義

**ファイル:** `crates/data/src/characters/geo/illuga.rs`

```rust
static ILLUGA_NA_HITS: &[TalentScaling] = &[
    ILLUGA_NORMAL_1, ILLUGA_NORMAL_2, ILLUGA_NORMAL_3, ILLUGA_NORMAL_4,
];
static ILLUGA_CHARGED_ATTACKS: &[TalentScaling] = &[ILLUGA_CHARGED];
static ILLUGA_PLUNGING: &[TalentScaling] = &[
    ILLUGA_PLUNGE, ILLUGA_PLUNGE_LOW, ILLUGA_PLUNGE_HIGH,
];
static ILLUGA_SKILL_SCALINGS: &[TalentScaling] = &[
    ILLUGA_SKILL_PRESS_EM, ILLUGA_SKILL_PRESS_DEF,
    ILLUGA_SKILL_HOLD_EM, ILLUGA_SKILL_HOLD_DEF,
];
static ILLUGA_BURST_SCALINGS: &[TalentScaling] = &[
    ILLUGA_BURST_EM, ILLUGA_BURST_DEF, ILLUGA_BURST_LUNAR_CRYSTALLIZE,
];

pub const ILLUGA: CharacterData = CharacterData {
    id: "illuga",
    name: "Illuga",
    element: Element::Geo,
    weapon_type: WeaponType::Polearm,
    rarity: Rarity::Star4,
    region: Region::Natlan,
    base_hp: [1003.0, 2577.0, 10602.0, 11962.0],
    base_atk: [16.03, 41.17, 169.41, 191.16],
    base_def: [68.21, 175.24, 721.02, 813.57],
    ascension_stat: AscensionStat::ElementalMastery(96.0),
    talents: TalentSet {
        normal_attack: NormalAttackData {
            name: "Oathkeeper's Spear",
            hits: ILLUGA_NA_HITS,
            charged: ILLUGA_CHARGED_ATTACKS,
            plunging: ILLUGA_PLUNGING,
        },
        elemental_skill: TalentData {
            name: "Dawnbearing Songbird",
            scalings: ILLUGA_SKILL_SCALINGS,
        },
        elemental_burst: TalentData {
            name: "Shadowless Reflection",
            scalings: ILLUGA_BURST_SCALINGS,
        },
    },
    constellation_pattern: ConstellationPattern::C3BurstC5Skill,
};
```

### 2. 天賦スケーリング

#### 通常攻撃 — Oathkeeper's Spear (物理, ATK)

| エントリ | スケーリング | 備考 |
|----------|-------------|------|
| `ILLUGA_NORMAL_1` | ATK, None (物理) | |
| `ILLUGA_NORMAL_2` | ATK, None | |
| `ILLUGA_NORMAL_3` | ATK, None | ×2ヒット（値は1ヒット分） |
| `ILLUGA_NORMAL_4` | ATK, None | |
| `ILLUGA_CHARGED` | ATK, None | |
| `ILLUGA_PLUNGE` | ATK, None | |
| `ILLUGA_PLUNGE_LOW` | ATK, None | |
| `ILLUGA_PLUNGE_HIGH` | ATK, None | |

#### 元素スキル — Dawnbearing Songbird (Geo, EM+DEF デュアル)

| エントリ | scaling_stat | 備考 |
|----------|-------------|------|
| `ILLUGA_SKILL_PRESS_EM` | Em | Press DMG の EM 部分 |
| `ILLUGA_SKILL_PRESS_DEF` | Def | Press DMG の DEF 部分 |
| `ILLUGA_SKILL_HOLD_EM` | Em | Hold DMG の EM 部分 |
| `ILLUGA_SKILL_HOLD_DEF` | Def | Hold DMG の DEF 部分 |

#### 元素爆発 — Shadowless Reflection (Geo, EM+DEF デュアル)

| エントリ | scaling_stat | 備考 |
|----------|-------------|------|
| `ILLUGA_BURST_EM` | Em | Skill DMG の EM 部分 |
| `ILLUGA_BURST_DEF` | Def | Skill DMG の DEF 部分 |
| `ILLUGA_BURST_LUNAR_CRYSTALLIZE` | Em | 月結晶化ダメージ増加（Lauma パターン） |

Geo DMG Bonus は天賦バフ（§3）で実装。Lunar-Crystallize DMG Bonus は TalentScaling として burst に含める。

### 3. 天賦バフ

**ファイル:** `crates/data/src/talent_buffs/geo.rs`

#### A4 — Torchforger's Covenant

Geo DMG で敵にヒット後、パーティ全員に20秒間:

```rust
static ILLUGA_BUFFS: &[TalentBuffDef] = &[
    TalentBuffDef {
        name: "Torchforger's Covenant - CRIT Rate",
        description: "After Geo DMG hits opponent, party CRIT Rate +5% for 20s",
        stat: BuffableStat::CritRate,
        base_value: 0.05,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Torchforger's Covenant - CRIT DMG",
        description: "After Geo DMG hits opponent, party CRIT DMG +10% for 20s",
        stat: BuffableStat::CritDmg,
        base_value: 0.10,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Torchforger's Covenant - EM (Moonsign)",
        description: "With Moonsign active, party EM +50 for 20s (A4 Ascendant Gleam condition)",
        stat: BuffableStat::ElementalMastery,
        base_value: 50.0,
        scales_with_talent: false,
        talent_scaling: None,
        scales_on: None,
        target: BuffTarget::Team,
        source: TalentBuffSource::AscensionPassive,
        min_constellation: 0,
    },
    TalentBuffDef {
        name: "Shadowless Reflection - Geo DMG",
        description: "During burst, Geo DMG Bonus based on talent level (flat%, not EM-scaled)",
        stat: BuffableStat::ElementalDmgBonus(Element::Geo),
        base_value: 0.0,
        scales_with_talent: true,
        talent_scaling: Some(&ILLUGA_BURST_GEO_DMG_SCALING),
        scales_on: None,  // flat bonus per talent level, not multiplied by EM
        target: BuffTarget::Character("illuga"),
        source: TalentBuffSource::ElementalBurst,
        min_constellation: 0,
    },
];
```

**Geo DMG Bonus の解釈:** Honey Impact 表記 "33.6% EM" は爆発天賦レベルに応じた直接的な Geo DMG Bonus%（scales_on: None）。EM は爆発の主スケーリングステータスを示す表記であり、EM値との乗算ではない（乗算だと 500 EM × 0.336 = 168% で非現実的）。

Lv1-15 talent_scaling: `[0.336, 0.3612, 0.3864, 0.42, 0.4452, 0.4704, 0.504, 0.5376, 0.5712, 0.6048, 0.6384, 0.672, 0.714, 0.756, 0.798]`

**Lunar-Crystallize DMG Bonus:** Lauma の `LAUMA_BURST_BLOOM_INCREASE` パターンに従い、TalentScaling として burst scalings に追加（TalentBuffDef ではない）。`BuffableStat` に `ReactionDmgBonus` バリアントが存在しないため。

```rust
// burst scalings に追加
const ILLUGA_BURST_LUNAR_CRYSTALLIZE: TalentScaling = TalentScaling {
    name: "月結晶化ダメージ増加",
    scaling_stat: ScalingStat::Em,
    damage_element: Some(Element::Geo),
    values: [2.2592, 2.4286, 2.5981, 2.824, 2.9934, 3.1629, 3.3888, 3.6147, 3.8406, 4.0666, 4.2925, 4.5184, 4.8008, 5.0832, 5.3656],
};
```

#### GEO_TALENT_BUFFS 登録

`geo.rs` の `GEO_TALENT_BUFFS` スライスに追加:
```rust
("illuga", ILLUGA_BUFFS),
```

### 4. 月兆データ

`moonsign_chars.rs` の ALL_MOONSIGN_BENEDICTIONS に Illuga エントリ済み（空 reactions, Aino パターン）。

Illuga の burst が Lunar-Crystallize DMG Bonus を付与するが、これは talent_buff として実装済み（上記 §3）。月兆 talent enhancement は burst talent level 依存のため不要（MoonsignTalentEnhancement は moonsign level 依存の効果用）。

### 5. mod.rs 登録

`crates/data/src/characters/geo/mod.rs` に3箇所追加:
- `mod illuga;`
- `pub use illuga::ILLUGA;`
- `CHARACTERS` スライスに `&ILLUGA`

### 6. TOML テストケース

**ファイル:** `crates/core/tests/data/characters/illuga.toml`

最小構成:
1. 通常攻撃1段 — 物理, ATK スケーリング
2. スキル Press — Geo, EM+DEF デュアルスケーリング

## Numerical Data (Lv1-15)

### Normal Attack

```
1-Hit:    [0.4737, 0.5122, 0.5508, 0.6058, 0.6444, 0.6885, 0.7490, 0.8096, 0.8702, 0.9363, 1.0024, 1.0685, 1.1346, 1.2007, 1.2668]
2-Hit:    [0.4853, 0.5248, 0.5642, 0.6207, 0.6602, 0.7053, 0.7674, 0.8294, 0.8915, 0.9592, 1.0269, 1.0946, 1.1624, 1.2301, 1.2978]
3-Hit×2:  [0.3143, 0.3399, 0.3655, 0.4020, 0.4276, 0.4569, 0.4971, 0.5373, 0.5775, 0.6213, 0.6652, 0.7091, 0.7529, 0.7968, 0.8407]
4-Hit:    [0.7628, 0.8249, 0.8870, 0.9757, 1.0377, 1.1087, 1.2063, 1.3038, 1.4014, 1.5078, 1.6143, 1.7207, 1.8271, 1.9336, 2.0400]
Charged:  [1.1103, 1.2006, 1.2910, 1.4201, 1.5105, 1.6137, 1.7558, 1.8978, 2.0398, 2.1947, 2.3496, 2.5045, 2.6595, 2.8144, 2.9693]
Plunge:   [0.6393, 0.6914, 0.7434, 0.8177, 0.8698, 0.9293, 1.0110, 1.0928, 1.1746, 1.2638, 1.3530, 1.4422, 1.5314, 1.6206, 1.7098]
Low:      [1.2784, 1.3824, 1.4865, 1.6351, 1.7392, 1.8581, 2.0216, 2.1851, 2.3486, 2.5270, 2.7054, 2.8838, 3.0622, 3.2405, 3.4189]
High:     [1.5968, 1.7267, 1.8567, 2.0424, 2.1723, 2.3209, 2.5251, 2.7293, 2.9336, 3.1564, 3.3792, 3.6020, 3.8248, 4.0476, 4.2704]
```

### Elemental Skill

```
Press EM:   [4.8256, 5.1875, 5.5494, 6.0320, 6.3939, 6.7558, 7.2384, 7.7210, 8.2035, 8.6861, 9.1686, 9.6512, 10.2544, 10.8576, 11.4608]
Press DEF:  [2.4128, 2.5938, 2.7747, 3.0160, 3.1970, 3.3779, 3.6192, 3.8605, 4.1018, 4.3430, 4.5843, 4.8256, 5.1272, 5.4288, 5.7304]
Hold EM:    [6.0320, 6.4844, 6.9368, 7.5400, 7.9924, 8.4448, 9.0480, 9.6512, 10.2544, 10.8576, 11.4608, 12.0640, 12.8180, 13.5720, 14.3260]
Hold DEF:   [3.0160, 3.2422, 3.4684, 3.7700, 3.9962, 4.2224, 4.5240, 4.8256, 5.1272, 5.4288, 5.7304, 6.0320, 6.4090, 6.7860, 7.1630]
```

### Elemental Burst

```
Burst EM:   [8.2720, 8.8924, 9.5128, 10.3400, 10.9604, 11.5808, 12.4080, 13.2352, 14.0624, 14.8896, 15.7168, 16.5440, 17.5780, 18.6120, 19.6460]
Burst DEF:  [4.1360, 4.4462, 4.7564, 5.1700, 5.4802, 5.7904, 6.2040, 6.6176, 7.0312, 7.4448, 7.8584, 8.2720, 8.7890, 9.3060, 9.8230]
```
