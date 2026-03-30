# WASM Bindings Design Spec

## Goal

`genshin-calc` の計算エンジン＋ゲームデータをブラウザから使えるWASMパッケージとして提供する。

## Scope

- 新クレート `crates/wasm` を追加
- ブラウザ向け（`--target web`）
- npm公開は対象外（将来の拡張として残す）
- CI/CDは対象外

## Architecture

### クレート構成

```
crates/
├── core/    # 計算エンジン（既存）
├── data/    # ゲームデータ（既存）
└── wasm/    # WASMバインディング（新規）
    ├── Cargo.toml
    ├── src/
    │   └── lib.rs
    └── pkg/           # wasm-pack出力（.gitignore対象）
```

- `crates/wasm` は `genshin-calc-core` と `genshin-calc-data` に依存
- wasm-bindgen + serde-wasm-bindgen で JSON in / JSON out API を公開
- TypeScript型定義は手書き `.d.ts` で提供

### 依存関係

```toml
[dependencies]
genshin-calc-core = { path = "../core" }
genshin-calc-data = { path = "../data" }
wasm-bindgen = "0.2"
serde-wasm-bindgen = "0.6"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

### crate-type

```toml
[lib]
crate-type = ["cdylib", "rlib"]
```

- `cdylib`: wasm-pack が WASM バイナリを生成するために必要
- `rlib`: `cargo test` でユニットテストを実行するために必要

## Public API

全関数は `#[wasm_bindgen]` でエクスポート。入出力は `JsValue` を使った JSON in/out パターン。

### 計算関数

| 関数 | 入力 | 出力 | 元の関数 |
|------|------|------|----------|
| `calculate_damage` | `JsValue` (DamageInput) | `JsValue` (DamageResult) | `core::calculate_damage` |
| `calculate_transformative` | `JsValue` (TransformativeInput) | `JsValue` (TransformativeResult) | `core::calculate_transformative` |
| `calculate_lunar` | `JsValue` (LunarInput) | `JsValue` (LunarResult) | `core::calculate_lunar` |
| `resolve_team_stats` | `JsValue` (Vec<TeamMember>), `usize` | `JsValue` (Stats) | `core::resolve_team_stats` |

### データ検索関数

| 関数 | 入力 | 出力 | 元の関数 |
|------|------|------|----------|
| `find_character` | `&str` | `JsValue` (CharacterData or null) | `data::find_character` |
| `find_weapon` | `&str` | `JsValue` (WeaponData or null) | `data::find_weapon` |
| `find_artifact_set` | `&str` | `JsValue` (ArtifactSet or null) | `data::find_artifact_set` |
| `find_enemy` | `&str` | `JsValue` (EnemyData or null) | `data::find_enemy` |
| `characters_by_element` | `&str` | `JsValue` (Vec<CharacterData>) | `data::characters_by_element` |
| `weapons_by_type` | `&str` | `JsValue` (Vec<WeaponData>) | `data::weapons_by_type` |

### メタデータ関数

| 関数 | 入力 | 出力 |
|------|------|------|
| `game_version` | なし | `String` |

### エラーハンドリング

- 計算関数: Rust側の `CalcError` を `JsError` に変換して throw
- データ検索: `None` の場合は `JsValue::NULL` を返す（throwしない）
- JSON デシリアライズ失敗: `JsError` で throw（"Invalid input: {serde error message}"）

### 文字列→enum変換

`characters_by_element` と `weapons_by_type` はJS側から文字列で enum を指定する:

- Element: `"pyro"`, `"hydro"`, `"electro"`, `"cryo"`, `"anemo"`, `"geo"`, `"dendro"`
- WeaponType: `"sword"`, `"claymore"`, `"polearm"`, `"bow"`, `"catalyst"`

不正な文字列は `JsError` で throw。

## TypeScript型定義

`crates/wasm/ts/types.d.ts` に手書きで提供。wasm-bindgen が生成する `.d.ts` を拡張する形。

主要な型:

```typescript
export interface Stats {
  hp: number;
  atk: number;
  def: number;
  elemental_mastery: number;
  crit_rate: number;
  crit_dmg: number;
  energy_recharge: number;
  dmg_bonus: number;
}

export interface DamageInput {
  character_level: number;
  stats: Stats;
  talent_multiplier: number;
  scaling_stat: "atk" | "hp" | "def";
  damage_type: "normal" | "charged" | "plunging" | "skill" | "burst";
  element: "pyro" | "hydro" | "electro" | "cryo" | "anemo" | "geo" | "dendro" | null;
  reaction: string | null;
  reaction_bonus: number;
  flat_dmg: number;
}

export interface DamageResult {
  non_crit: number;
  crit: number;
  average: number;
}

export interface Enemy {
  level: number;
  resistance: number;
  def_reduction: number;
}
```

## ビルド

```bash
# wasm-pack インストール（未インストールの場合）
cargo install wasm-pack

# ビルド
cd crates/wasm
wasm-pack build --target web

# 出力: pkg/ に .wasm, .js, .d.ts
```

### .gitignore

`crates/wasm/pkg/` をリポジトリから除外。

## テスト

### Rustユニットテスト

`cargo test -p genshin-calc-wasm` で実行:

- 文字列→enum変換の正常系・異常系
- 各ラッパー関数が正しい型を返すか（serde_json でラウンドトリップ確認）

### wasm-pack テスト（将来）

`wasm-pack test --headless --chrome` によるブラウザ内テストは将来の拡張として残す。

## Workspace変更

`Cargo.toml`（ルート）の `members` に `"crates/wasm"` を追加:

```toml
[workspace]
members = ["crates/core", "crates/data", "crates/wasm"]
```

## 制約・注意事項

- WASM バイナリサイズ: ゲームデータ全量を含むため数MB程度になる可能性がある。最適化は将来対応
- `serde-wasm-bindgen` は serde の Serialize/Deserialize を JsValue に直接変換する（JSON文字列を経由しない）
- `&'static` 参照型（CharacterData等）は Serialize のみ対応。WASM側では所有権のあるJsValueとして返す
