# GOOD形式インポート機能 設計書

## 概要

Genshin Open Object Description (GOOD) 形式のJSONファイルを読み込み、当プロジェクトの計算エンジンに渡せる `CharacterBuild` 構造体に変換する機能を `crates/good` クレートとして実装する。

GOODはGenshin Optimizerが策定したオープンフォーマットで、各種スキャナー（Inventory Kamera等）がエクスポートするプレイヤーデータの標準形式である。

## クレート構造

**新クレート: `crates/good`** (`genshin-calc-good`)

```
crates/good/
├── Cargo.toml
└── src/
    ├── lib.rs          # 公開API
    ├── types.rs        # GOOD JSON構造体（デシリアライズ用）
    ├── key_map.rs      # GOOD PascalCase ↔ 内部snake_case変換
    ├── stat_map.rs     # GOOD StatKey → StatProfile フィールドマッピング
    ├── convert.rs      # GoodFormat → Vec<CharacterBuild> 変換ロジック
    └── error.rs        # エラー型
```

**依存関係:**

```toml
[dependencies]
genshin-calc-core = { path = "../core" }
genshin-calc-data = { path = "../data" }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
```

- `core`: `StatProfile`, `Element`, `WeaponType` 等の型を使用
- `data`: `find_character()`, `find_weapon()`, `find_artifact_set()` でルックアップ
- WASM互換を維持（`std`重い機能なし）

## GOOD JSON入力型

GOODのJSON構造をそのままマッピングするserde構造体。フィールド名はGOODの `camelCase` に合わせ `#[serde(rename_all = "camelCase")]` で対応。

```rust
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodFormat {
    pub format: String,           // "GOOD"
    pub source: String,           // スキャナー名
    pub version: u8,              // 1 | 2 | 3
    pub characters: Option<Vec<GoodCharacter>>,
    pub artifacts: Option<Vec<GoodArtifact>>,
    pub weapons: Option<Vec<GoodWeapon>>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodCharacter {
    pub key: String,              // "HuTao", "KamisatoAyaka"
    pub level: u32,               // 1-90
    pub constellation: u8,        // 0-6
    pub ascension: u8,            // 0-6
    pub talent: GoodTalent,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodTalent {
    pub auto: u8,                 // 1-15
    pub skill: u8,
    pub burst: u8,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodWeapon {
    pub key: String,              // "WolfsGravestone"
    pub level: u32,               // 1-90
    pub ascension: u8,            // 0-6
    pub refinement: u8,           // 1-5
    pub location: Option<String>, // 装備キャラのCharacterKey ("" = 未装備)
    pub lock: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodArtifact {
    pub set_key: String,          // "CrimsonWitchOfFlames"
    pub slot_key: String,         // "flower" | "plume" | "sands" | "goblet" | "circlet"
    pub level: u8,                // 0-20
    pub rarity: u8,               // 3-5
    pub main_stat_key: String,    // "hp", "atk_", "critRate_" 等
    pub location: Option<String>, // 装備キャラのCharacterKey
    pub lock: bool,
    pub substats: Vec<GoodSubstat>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoodSubstat {
    pub key: String,              // "hp", "hp_", "atk", "critRate_" 等
    pub value: f64,               // 実数値（例: 311, 5.8）
}
```

### 注意点

- `location` が空文字列 `""` の場合は未装備扱い
- `substats` の value はパーセンテージ系も実数値（例: CR 5.8 = 5.8%）。内部変換時に ÷100 する

## キーマッピング戦略

### 方針: プログラム的変換 + 例外テーブル

GOOD形式のPascalCaseキーを内部のsnake_caseに変換する。

**基本変換（自動）:**

| GOOD Key | → snake_case | data crate ID | 一致 |
|----------|-------------|---------------|------|
| `"HuTao"` | `"hu_tao"` | `"hu_tao"` | ✓ |
| `"KamisatoAyaka"` | `"kamisato_ayaka"` | `"kamisato_ayaka"` | ✓ |
| `"WolfsGravestone"` | `"wolfs_gravestone"` | `"wolfs_gravestone"` | ✓ |

**例外テーブル（自動変換で不一致のもの）:**

聖遺物は内部IDが省略形のため、例外テーブルで対応:

```rust
static ARTIFACT_ALIASES: &[(&str, &str)] = &[
    ("crimson_witch_of_flames", "crimson_witch"),
    // ...必要な分だけ追加
];
```

**ルックアップ手順:**

1. PascalCase → snake_case に自動変換
2. `data::find_character(id)` / `find_weapon(id)` / `find_artifact_set(id)` でルックアップ
3. 見つからない → 例外テーブル参照
4. それでも見つからない → `ImportWarning` として報告（エラーにしない）

**メリット:**
- 新キャラ/武器追加時、大半は自動変換で対応 → メンテ不要
- 例外テーブルは小規模（聖遺物の省略ID分のみ）
- 未対応キーはエラーにせず warning で返す → 部分インポート可能

## ステータスマッピングと聖遺物集計

### GOOD StatKey → StatProfile フィールド変換

| GOOD StatKey | → StatProfile フィールド | 変換 |
|-------------|------------------------|------|
| `"hp"` | `hp_flat` | そのまま |
| `"hp_"` | `hp_percent` | ÷100 |
| `"atk"` | `atk_flat` | そのまま |
| `"atk_"` | `atk_percent` | ÷100 |
| `"def"` | `def_flat` | そのまま |
| `"def_"` | `def_percent` | ÷100 |
| `"eleMas"` | `elemental_mastery` | そのまま |
| `"enerRech_"` | `energy_recharge` | ÷100 |
| `"critRate_"` | `crit_rate` | ÷100 |
| `"critDMG_"` | `crit_dmg` | ÷100 |
| `"pyro_dmg_"` 等 | `dmg_bonus` | ÷100（キャラ元素一致時のみ） |
| `"physical_dmg_"` | `dmg_bonus` | ÷100 |
| `"heal_"` | （集計対象外） | StatProfile に該当フィールドなし |

### パーセント値の変換ルール

- GOOD は人間読み形式（例: `critRate_ = 31.1` = 31.1%）
- 内部は小数形式（例: `0.311`）
- `_` サフィックス付きキー + `enerRech_`, `critRate_`, `critDMG_`, `*_dmg_`, `heal_` → ÷100

### 聖遺物集計処理

1. キャラに装備された聖遺物を `location` フィールドで紐付け
2. 各聖遺物のメインステ + サブステを合算して1つの `StatProfile` を構築
3. メインステの値は `level` + `rarity` + `main_stat_key` からルックアップテーブルで算出（GOOD JSON にメインステ値は含まれない）
4. 元素ダメージボーナスはキャラの元素と一致する場合のみ `dmg_bonus` に加算

### メインステ値テーブル

星3/4/5 × レベル0-20 の全組み合わせを定数テーブルとして保持。

例（星5 Lv20）: `hp_ = 46.6%`, `atk_ = 46.6%`, `critRate_ = 31.1%`, `critDMG_ = 62.2%`

### dmg_bonus の扱い

- ゴブレットの元素ダメージ → キャラの `Element` と照合し、一致すれば `dmg_bonus` に加算
- 物理ダメージボーナスも `dmg_bonus` へ加算
- 元素不一致の場合 → `ImportWarning::ElementMismatchGoblet` で警告

## 出力型

```rust
/// GOODインポートの結果
pub struct GoodImport {
    pub source: String,
    pub version: u8,
    pub builds: Vec<CharacterBuild>,
    pub warnings: Vec<ImportWarning>,
}

/// 1キャラ分のビルド（キャラ+武器+聖遺物がセット）
pub struct CharacterBuild {
    pub character: &'static CharacterData,
    pub level: u32,
    pub constellation: u8,
    pub talent_levels: [u8; 3],   // [auto, skill, burst]
    pub weapon: Option<WeaponBuild>,
    pub artifacts: ArtifactsBuild,
}

pub struct WeaponBuild {
    pub weapon: &'static WeaponData,
    pub level: u32,
    pub refinement: u8,
}

pub struct ArtifactsBuild {
    pub set: Option<&'static ArtifactSet>,  // 検出された4pcまたは2pcセット
    pub stats: StatProfile,                  // 聖遺物全体の集計ステータス
}
```

### 聖遺物セット検出

装備中の5個の聖遺物から最多セットを自動検出:
- 同一セット4個以上 → 4pcセットとして認識
- 同一セット2個以上 → 2pcセットとして認識
- 4pcが存在すれば4pc優先

## 公開API

```rust
/// GOOD JSONをパースしてビルド一覧に変換
pub fn import_good(json: &str) -> Result<GoodImport, GoodError>;

/// 既にデシリアライズ済みのGoodFormatから変換
pub fn convert_good(good: GoodFormat) -> GoodImport;
```

### 使用例

```rust
let json = std::fs::read_to_string("good_export.json")?;
let import = genshin_calc_good::import_good(&json)?;

for build in &import.builds {
    let mut builder = TeamMemberBuilder::new(
        build.character,
        build.weapon.as_ref().map(|w| w.weapon).unwrap_or(default_weapon),
    );
    builder
        .constellation(build.constellation)
        .talent_levels(build.talent_levels)
        .artifact_stats(build.artifacts.stats.clone());
    if let Some(set) = build.artifacts.set {
        builder.artifact_set(set);
    }
    if let Some(ref wb) = build.weapon {
        builder.refinement(wb.refinement);
    }
}
```

### エラー vs 警告の境界

- `GoodError`（`Result::Err`）: JSON構文エラー、`format` フィールドが `"GOOD"` でない、未サポートバージョン
- `ImportWarning`（`GoodImport::warnings`）: 未知のキャラ/武器/聖遺物/ステータスキー、元素不一致ゴブレット

## エラー型

```rust
#[derive(Debug, thiserror::Error)]
pub enum GoodError {
    #[error("JSON parse error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("invalid GOOD format: expected \"GOOD\", got \"{0}\"")]
    InvalidFormat(String),

    #[error("unsupported GOOD version: {0}")]
    UnsupportedVersion(u8),
}

#[derive(Debug, Clone)]
pub enum ImportWarning {
    UnknownCharacter(String),
    UnknownWeapon(String),
    UnknownArtifactSet(String),
    UnknownStatKey(String),
    ElementMismatchGoblet {
        character: String,
        goblet_element: String,
    },
}
```

## テスト戦略

```
tests/
├── data/
│   ├── minimal.json         # 最小GOOD JSON（キャラ1人、武器1本、聖遺物5個）
│   ├── full_team.json       # 4人パーティ全装備
│   ├── partial.json         # 聖遺物欠け、武器なし等の不完全データ
│   ├── unknown_keys.json    # data crateに未登録のキャラ/武器を含む
│   └── invalid.json         # 不正なJSON（エラーケース）
├── test_parse.rs            # GoodFormat デシリアライズテスト
├── test_key_map.rs          # PascalCase→snake_case変換テスト
├── test_stat_map.rs         # StatKey変換 + パーセント÷100テスト
├── test_convert.rs          # 統合テスト（JSON → CharacterBuild）
└── test_artifact_stats.rs   # 聖遺物メインステ値テーブル + 集計テスト
```

### 重点テストケース

- パーセント変換の精度（浮動小数点、許容誤差比較）
- 聖遺物メインステ値がレベル・レアリティから正しく算出されるか
- 武器/聖遺物の `location` によるキャラへの紐付け
- 未知キーが warning に入り、既知キーは正常処理されるか
- 空の `characters` / `weapons` / `artifacts` フィールド（`None` や `[]`）
