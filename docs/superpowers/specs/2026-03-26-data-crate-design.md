# Data Crate Design Spec

## Overview

`genshin-calc-data` crateにゲームデータ（キャラクター、武器、聖遺物、敵）を定義する。`core`の計算エンジンに渡すデータを一元管理し、Rust定数として埋め込むことでWASM互換性とコンパイル時型安全性を保証する。

対応ゲームバージョン: v5.8

## Design Decisions

| 決定事項 | 選択 | 理由 |
|---|---|---|
| データ管理方法 | Rustコード内定数 | WASM互換、型安全、シンプル |
| キャラ基礎ステータス粒度 | Lv1/80/80+/90 (4ポイント) | 実用的な計算に十分 |
| タレント倍率範囲 | 全タレント全レベル (Lv1-15 × 全段) | 完全なデータ |
| 武器パッシブ表現 | ハイブリッド (構造化バフ + テキスト) | 計算可能なバフは型安全に、条件付き効果はテキスト |
| 聖遺物セット効果 | ハイブリッド (同上) | 2セットは構造化しやすい、4セットは条件付き多い |
| 敵データ | 耐性テンプレート方式 | 実質10-15パターンに集約可能 |
| 公開API | 定数アクセス + 文字列検索 | Rust利用者は定数、WASM/JS利用者は文字列検索 |
| ファイル構成 | 元素別/武器種別に分割 | 1ファイル8-16件で管理しやすい |
| 初期データ量 | 102キャラ全部 + 全武器 | 一気に完成させる |
| 数値単位 | 小数形式 (coreと統一) | 10.8% → 0.108。coreの Stats/StatProfile と一致 |

## File Structure

```
crates/data/src/
├── lib.rs              // モジュールre-export + 検索API
├── types.rs            // CharacterData, WeaponData等の構造体
├── buff.rs             // BuffableStat, StatBuff, PassiveEffect
├── characters/
│   ├── mod.rs          // pub mod per element
│   ├── pyro.rs         // 炎キャラ (16体)
│   ├── hydro.rs        // 水キャラ (14体)
│   ├── electro.rs      // 雷キャラ (17体)
│   ├── cryo.rs         // 氷キャラ (18体)
│   ├── dendro.rs       // 草キャラ (11体)
│   ├── anemo.rs        // 風キャラ (15体)
│   └── geo.rs          // 岩キャラ (11体)
├── weapons/
│   ├── mod.rs          // pub mod per weapon type
│   ├── sword.rs        // 片手剣
│   ├── claymore.rs     // 両手剣
│   ├── polearm.rs      // 長柄武器
│   ├── bow.rs          // 弓
│   └── catalyst.rs     // 法器
├── artifacts.rs        // 聖遺物セット効果
└── enemies.rs          // 耐性テンプレート + 敵マッピング
```

## Conventions

### Derive Traits

全ての公開型に以下のderiveを付与:

- **小型enum** (`WeaponType`, `Rarity`, `Region`, `BuffableStat`, `ArtifactRarity`):
  `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]`
- **データ構造体（`&'static`参照なし）** (`AscensionStat`, `StatBuff`, `WeaponSubStat`, `ResistanceTemplate`):
  `#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]`
  — `AscensionStat`は`f64`フィールドを含むため`Eq`/`Hash`を導出できない
- **データ構造体（`&'static`参照あり）** (`CharacterData`, `WeaponData`, `ArtifactSet`, `EnemyData`, `TalentSet`, `NormalAttackData`, `TalentData`, `TalentScaling`, `SetEffect`, `PassiveEffect`, `WeaponPassive`):
  `#[derive(Debug, Clone, PartialEq, Serialize)]` — **`Deserialize`なし**

**Deserializeについて**: `&'static [T]` や `&'static ResistanceTemplate` を含む型はserdeの`Deserialize`を導出できない（serdeは`&'static`スライス参照のデシリアライズを未サポート）。これらの型はconst定数として構築されるため、デシリアライズは不要。シリアライズ（JSON出力等）のみ対応する。

### 数値単位

全てのパーセンテージ値は**小数形式**で格納する（coreの`Stats`/`StatProfile`と統一）:
- 10.8% → `0.108`
- 49.6% → `0.496`
- ATK+20% → `0.20`

### Game Version

```rust
pub const GAME_VERSION: &str = "5.8";
```

データ更新時にバージョンを更新する。

## Type Definitions

### Core Enums (`types.rs`)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType { Sword, Claymore, Polearm, Bow, Catalyst }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rarity { Star1, Star2, Star3, Star4, Star5 }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Region { Mondstadt, Liyue, Inazuma, Sumeru, Fontaine, Natlan, Snezhnaya, Other }
```

`Element`, `DamageType`, `ScalingStat` は `genshin-calc-core` から再利用。

### 旅人 (Traveler)

旅人は元素ごとに異なる `CharacterData` として定義する（`traveler_anemo`, `traveler_geo` 等）。各バリアントはそれぞれの元素ファイルに配置する（例: `traveler_pyro` は `pyro.rs`）。基礎ステータス（HP/ATK/DEF）は全元素で同一だが、タレントが異なるためデータは複製する（約200bytes/バリアント、シンプルさ優先）。

### Character Data (`types.rs`)

```rust
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CharacterData {
    pub id: &'static str,        // ASCII識別子 (例: "diluc")
    pub name: &'static str,      // 表示名 (例: "Diluc")
    pub element: Element,
    pub weapon_type: WeaponType,
    pub rarity: Rarity,
    pub region: Region,
    pub base_hp: [f64; 4],       // [Lv1, Lv80, Lv80+, Lv90]
    pub base_atk: [f64; 4],
    pub base_def: [f64; 4],
    pub ascension_stat: AscensionStat,  // A6 (Lv90) の最終値
    pub talents: TalentSet,
}
```

`ascension_stat` はA6（Lv90完凸時）の最終値のみ格納する。中間突破段階の値は必要になった時点で対応する。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AscensionStat {
    Hp(f64),
    Atk(f64),
    Def(f64),
    CritRate(f64),
    CritDmg(f64),
    ElementalMastery(f64),
    EnergyRecharge(f64),
    ElementalDmgBonus(Element, f64),
    PhysicalDmgBonus(f64),
    HealingBonus(f64),
}
```

### Talent Data (`types.rs`)

```rust
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentSet {
    pub normal_attack: NormalAttackData,
    pub elemental_skill: TalentData,
    pub elemental_burst: TalentData,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NormalAttackData {
    pub name: &'static str,
    pub hits: &'static [TalentScaling],
    pub charged: &'static [TalentScaling],
    pub plunging: &'static [TalentScaling],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentData {
    pub name: &'static str,
    pub scalings: &'static [TalentScaling],
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentScaling {
    pub name: &'static str,
    pub scaling_stat: ScalingStat,
    pub damage_element: Option<Element>,  // None = 物理, Some = 元素ダメージ
    pub values: [f64; 15],               // Lv1-15
}
```

`damage_element`: 法器キャラの通常攻撃や元素スキル/爆発は `Some(element)` を設定。物理攻撃は `None`。天賦やスキルによる元素付与（胡桃のスキル中等）はコンディショナルなので呼び出し側で指定。

`values` は `[f64; 15]` 固定。全キャラのタレントはLv1-15（ベース10 + 命座+3 + 冠+2 の仕様に統一されている）。

**DamageTypeの決定**: `TalentScaling`に`DamageType`フィールドは持たない。格納位置で暗黙的に決まる:
- `NormalAttackData.hits` → `DamageType::Normal`
- `NormalAttackData.charged` → `DamageType::Charged`
- `NormalAttackData.plunging` → `DamageType::Plunging`
- `TalentData`（elemental_skill） → `DamageType::Skill`
- `TalentData`（elemental_burst） → `DamageType::Burst`

一部キャラ（例: 雷電将軍の爆発中通常攻撃）は単一タレント内でDamageTypeが混在するが、これはNon-Goalsとする。呼び出し側で適切なDamageTypeを指定すること。

### Buff Types (`buff.rs`)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuffableStat {
    HpPercent, AtkPercent, DefPercent,
    HpFlat, AtkFlat, DefFlat,
    CritRate, CritDmg,
    ElementalMastery, EnergyRecharge,
    DmgBonus,
    ElementalDmgBonus(Element),
    PhysicalDmgBonus,
    NormalAtkDmgBonus,
    ChargedAtkDmgBonus,
    PlungingAtkDmgBonus,
    SkillDmgBonus,
    BurstDmgBonus,
    HealingBonus,
    ShieldStrength,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct StatBuff {
    pub stat: BuffableStat,
    pub value: f64,               // 精錬1の値（精錬対応時はrefinement_valuesを使う）
    pub refinement_values: Option<[f64; 5]>,  // 精錬1-5の値（武器パッシブ用）
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PassiveEffect {
    pub description: &'static str,
    pub buffs: &'static [StatBuff],  // 構造化可能なバフ群
}
```

`refinement_values` を `StatBuff` に移動。複数バフがそれぞれ異なる精錬スケーリングを持てる。聖遺物バフでは `refinement_values: None`。

### Weapon Data (`types.rs`)

```rust
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeaponData {
    pub id: &'static str,        // ASCII識別子 (例: "wolfs_gravestone")
    pub name: &'static str,      // 表示名 (例: "狼の末路")
    pub weapon_type: WeaponType,
    pub rarity: Rarity,
    pub base_atk: [f64; 4],     // [Lv1, Lv80, Lv80+, Lv90]
    pub sub_stat: Option<WeaponSubStat>,
    pub passive: Option<WeaponPassive>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WeaponSubStat {
    HpPercent([f64; 4]),
    AtkPercent([f64; 4]),
    DefPercent([f64; 4]),
    CritRate([f64; 4]),
    CritDmg([f64; 4]),
    ElementalMastery([f64; 4]),
    EnergyRecharge([f64; 4]),
    PhysicalDmgBonus([f64; 4]),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeaponPassive {
    pub name: &'static str,
    pub effect: PassiveEffect,
}
```

`WeaponSubStat` の値はすべて小数形式: ATK% 10.8% → `0.108`。

### Artifact Data (`types.rs`)

```rust
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ArtifactSet {
    pub id: &'static str,       // ASCII識別子 (例: "crimson_witch")
    pub name: &'static str,     // 表示名 (例: "燃え盛る炎の魔女")
    pub rarity: ArtifactRarity,
    pub two_piece: SetEffect,
    pub four_piece: SetEffect,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ArtifactRarity {
    Star4,
    Star5,
    Star4And5,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetEffect {
    pub description: &'static str,
    pub buffs: &'static [StatBuff],
}
```

### Enemy Data (`types.rs`)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ResistanceTemplate {
    pub name: &'static str,
    pub pyro: f64,
    pub hydro: f64,
    pub electro: f64,
    pub cryo: f64,
    pub dendro: f64,
    pub anemo: f64,
    pub geo: f64,
    pub physical: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EnemyData {
    pub id: &'static str,       // ASCII識別子 (例: "hilichurl")
    pub name: &'static str,     // 表示名 (例: "ヒルチャール")
    pub resistance: &'static ResistanceTemplate,
}
```

### Core変換ヘルパー

`EnemyData` + `ResistanceTemplate`からcoreの`Enemy`への変換:

```rust
impl ResistanceTemplate {
    /// 指定元素の耐性値を返す。物理はNoneで指定。
    pub fn get(&self, element: Option<Element>) -> f64 {
        match element {
            Some(Element::Pyro) => self.pyro,
            Some(Element::Hydro) => self.hydro,
            Some(Element::Electro) => self.electro,
            Some(Element::Cryo) => self.cryo,
            Some(Element::Dendro) => self.dendro,
            Some(Element::Anemo) => self.anemo,
            Some(Element::Geo) => self.geo,
            None => self.physical,
        }
    }
}

impl EnemyData {
    /// coreのEnemy構造体に変換する。
    /// element: 攻撃の元素 (None = 物理)
    /// level: 敵のレベル
    /// def_reduction: 防御減少率 (例: VVシュレッダー)
    pub fn to_enemy(&self, element: Option<Element>, level: u32, def_reduction: f64) -> Enemy {
        Enemy {
            level,
            resistance: self.resistance.get(element),
            def_reduction,
        }
    }
}
```

## Public API (`lib.rs`)

### Module Re-exports

```rust
pub mod characters;
pub mod weapons;
pub mod artifacts;
pub mod enemies;
pub mod types;
pub mod buff;
```

定数アクセス: `data::characters::pyro::DILUC`
武器アクセス: `data::weapons::claymore::WOLFS_GRAVESTONE`

### Search Functions

`find_*` 関数は `id` フィールドで検索する（ASCII、ロケール非依存）。

```rust
pub fn find_character(id: &str) -> Option<&'static CharacterData>;
pub fn find_weapon(id: &str) -> Option<&'static WeaponData>;
pub fn find_artifact_set(id: &str) -> Option<&'static ArtifactSet>;
pub fn find_enemy(id: &str) -> Option<&'static EnemyData>;
```

### Filter Functions

```rust
pub fn characters_by_element(element: Element) -> Vec<&'static CharacterData>;
pub fn weapons_by_type(weapon_type: WeaponType) -> Vec<&'static WeaponData>;
```

`Vec`を返す。WASM環境でのheap allocationは許容する（データ量が小さく、頻繁に呼ばれない想定）。

### Collection Constants

```rust
pub const ALL_CHARACTERS: &[&CharacterData];
pub const ALL_WEAPONS: &[&WeaponData];
pub const ALL_ARTIFACT_SETS: &[&ArtifactSet];
pub const ALL_ENEMIES: &[&EnemyData];
pub const GAME_VERSION: &str = "5.8";
```

## Data Examples

### Character (Diluc)

```rust
pub const DILUC: CharacterData = CharacterData {
    id: "diluc",
    name: "Diluc",
    element: Element::Pyro,
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    region: Region::Mondstadt,
    base_hp: [1011.0, 10122.0, 11243.0, 12068.0],
    base_atk: [26.0, 260.0, 289.0, 311.0],
    base_def: [61.0, 612.0, 680.0, 729.0],
    ascension_stat: AscensionStat::CritRate(0.192),
    talents: TalentSet { /* ... */ },
};
```

### Weapon (Wolf's Gravestone)

```rust
pub const WOLFS_GRAVESTONE: WeaponData = WeaponData {
    id: "wolfs_gravestone",
    name: "狼の末路",
    weapon_type: WeaponType::Claymore,
    rarity: Rarity::Star5,
    base_atk: [46.0, 520.0, 573.0, 608.0],
    sub_stat: Some(WeaponSubStat::AtkPercent([0.108, 0.406, 0.444, 0.496])),
    passive: Some(WeaponPassive {
        name: "止めの一撃",
        effect: PassiveEffect {
            description: "ATK+20-40%。HP30%以下の敵を攻撃するとチーム全員ATK+40-80%、12秒、30秒に1回",
            buffs: &[StatBuff {
                stat: BuffableStat::AtkPercent,
                value: 0.20,
                refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
            }],
        },
    }),
};
```

### Artifact Set (Crimson Witch)

```rust
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
```

### Enemy (Resistance Template)

```rust
pub const ALL_10: ResistanceTemplate = ResistanceTemplate {
    name: "標準 (全耐性10%)",
    pyro: 0.10, hydro: 0.10, electro: 0.10, cryo: 0.10,
    dendro: 0.10, anemo: 0.10, geo: 0.10, physical: 0.10,
};

pub const HILICHURL: EnemyData = EnemyData {
    id: "hilichurl",
    name: "ヒルチャール",
    resistance: &ALL_10,
};

// Core変換例:
// let enemy = HILICHURL.to_enemy(Some(Element::Pyro), 90, 0.0);
// let result = calculate_damage(&input, &enemy)?;
```

## Testing Strategy

- **型定義テスト**: 各構造体のserialize確認（`&'static`参照型はDeserializeなし）。Deserialize可能な小型enum/StatBuffはroundtripテスト
- **検索APIテスト**: find_character/find_weapon等の正常系・異常系
- **データ整合性テスト**: 全キャラの基礎ステータスが正の値、タレント倍率が0以上、等
- **coreとの統合テスト**: data crateのキャラデータからDamageInputを構築し、coreで計算
- **既存character_verificationとの照合**: テストTOMLの値とdata crateの値が一致するか
- **変換ヘルパーテスト**: `to_enemy`が正しいcoreの`Enemy`を生成するか
- **データ件数テスト**: `ALL_CHARACTERS.len() == 102` 等のアサーションで追加漏れを検出

## Dependencies

```toml
[dependencies]
genshin-calc-core = { path = "../core" }
serde = { version = "1", features = ["derive"] }
```

## Scope & Non-Goals

### In Scope
- キャラ基礎ステータス (Lv1/80/80+/90)
- タレント倍率 (全段 × Lv1-15) + ダメージ元素情報
- 武器データ (基礎ATK + サブステ + パッシブ) — 星1-5全武器
- 聖遺物セット効果 (2セット/4セット)
- 敵耐性テンプレート + core変換ヘルパー
- 検索・フィルタリングAPI
- ASCII id + 表示名の二重識別

### Non-Goals
- 聖遺物の個別ピース（花/羽/砂/杯/冠）のメインステータス候補
- ビルド最適化・推薦
- キャラの固有バフ（パッシブ天賦、命の星座）の構造化
- 元素共鳴の定義（将来coreに追加）
- レベル間の補間計算（4ポイント以外のレベルのステータス推定）
- 中間突破段階のascension stat値
- `StatBuff` → `StatProfile` 自動変換（将来の拡張候補。`BuffableStat`はcoreの`StatProfile`より粒度が細かく、`DmgBonus` vs `ElementalDmgBonus(Element)` 等の集約ロジックが必要）
- タレント内のDamageType混在の構造化（例: 雷電将軍の爆発中通常攻撃）
- 表示名による検索API（将来 `find_character_by_name` 追加可能）
