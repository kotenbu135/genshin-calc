# キャラステータス計算 API の問題点

## 概要

GOOD データをインポートしてダメージ計算を行う際のアーキテクチャ上の問題点を整理する。

## 前提

- **入力**: GOOD (Genshin Optimizer Data) 形式 JSON
- **出力**: `TeamMember` (stats + buffs_provided)
- **主要モジュール**: `crates/good/` (インポート), `crates/data/` (計算), `crates/core/` (計算引擎)

## 現状のデータフロー

```
GOOD JSON
    ↓
import_good() / convert_good()
    ↓
GoodImport { builds: Vec<CharacterBuild> }
    ├─ CharacterBuild
    │   ├─ character: &CharacterData
    │   ├─ level, ascension, constellation, talent_levels
    │   ├─ weapon: Option<WeaponBuild>
    │   └─ artifacts: ArtifactsBuild
    │       ├─ sets: Vec<&ArtifactSet>
    │       ├─ four_piece_set: Option<&ArtifactSet>
    │       └─ stats: StatProfile  ← main + sub + 2pc buffs
    ↓
to_team_member_builder()
    ↓
TeamMemberBuilder
    ├─ build_base_profile() → base + ascension + weapon sub
    ├─ merge_artifact_stats() → artifact stats をmerge
    ├─ collect_static_buffs() → weapon passive + artifact 4pc buffs
    └─ resolve_conditional_buffs() → conditional activation
    ↓
TeamMember { stats, buffs_provided }
```

---

## 問題点一覧

### 1. 武器レベルの無視 (高優先度)

**場所**: `crates/good/src/convert.rs:26-29`

```rust
let mut builder = TeamMemberBuilder::new(build.character, weapon_build.weapon)
    .constellation(build.constellation)
    .talent_levels(build.talent_levels)
    .refinement(weapon_build.refinement);
    // ↑ weapon.level が渡されていない！
```

**問題**: `WeaponBuild` には `level: u32` フィールドがあるのに、`TeamMemberBuilder` に渡されていない。常に Lv90 相当の武器ステータス使われる。

**影響**: Lv90 以外の武器を使用している場合、計算結果が合わない。

**対応**: `TeamMemberBuilder::new()` に武器レベルを渡すか、`weapon_level()` をチェーンする。

---

### 2. 2pc buffs の handling 分散 (高優先度)

**場所**: `crates/good/src/convert.rs:186-191`

```rust
// For 2pc/2pc+2pc, pre-merge 2pc buffs into stats
if four_piece_set.is_none() {
    for set in &sets {
        apply_two_piece_buffs(set, character, &mut stats);
    }
}
```

**問題**:
- 4pc 選択時 → 2pc buffs を `stats` に直接加算、4pc 効果を `buffs_provided` に追加
- 2pc+2pc 時 → 両方の 2pc buffs を `stats` に直接加算

**結果**: 同じ「2pc set bonus」が状況によって `stats` または `buffs_provided` のどちらに含まれるか変わる。

| 状況 | 2pc buffs | 4pc buffs |
|------|------------|-----------|
| 4pc 選択 | `stats` | `buffs_provided` |
| 2pc+2pc | `stats` | - |

**影響**: 計算ロジックの一貫性がなく、buff の source 追跡が困難。

**対応案**:
- 案A: すべての set bonus を `buffs_provided` で管理し、`stats` には artifact main/substats のみ格納
- 案B: `stats` と `buffs_provided` の境界を明確化し、ドキュメント化

---

### 3. base stats 構築ロジックの重複 (中優先度)

**場所**: 
- `crates/good/src/build_stats.rs:8-54`
- `crates/data/src/team_builder.rs:215-232`

```rust
// build_stats.rs (good crate)
let mut profile = StatProfile {
    base_hp: build.character.base_hp_at_level(build.level),
    base_atk: build.character.base_atk_at_level(build.level),
    base_def: build.character.base_def_at_level(build.level),
    crit_rate: 0.05,
    crit_dmg: 0.50,
    energy_recharge: 1.0,
    ..Default::default()
};

// team_builder.rs (data crate)
let mut profile = StatProfile {
    base_hp: self.character.base_hp_at_level(self.character_level),
    base_atk: self.character.base_atk_at_level(self.character_level)
        + self.weapon.base_atk[weapon_idx],  // ↑ 武器ATKを含む
    base_def: self.character.base_def_at_level(self.character_level),
    crit_rate: 0.05,
    crit_dmg: 0.50,
    energy_recharge: 1.0,
    ..Default::default()
};
```

**問題**:
- 同じ base stats 構築ロジックが2箇所に重複
- weapon base ATK の handling が異なる（`build_stats.rs` は weapon ATK を含まない）
- base value (5% CR, 50% CD, 100% ER) が hardcode されている

**対応**: 共通ロジックを `genshin_calc_data` に抽取。

---

### 4. `ascension` フィールドの未使用 (中優先度)

**場所**: `crates/good/src/lib.rs:29`

```rust
pub struct CharacterBuild {
    pub character: &'static CharacterData,
    pub level: u32,
    pub ascension: u8,  // ← 使われていない
    pub constellation: u8,
    pub talent_levels: [u8; 3],
    // ...
}
```

**問題**: ユーザーが入力する `ascension` が `build_stat_profile()` で使われていない。代わりに `level` から計算。

**理由**: `CharacterData::base_hp_at_level()` が level に対応する昇華後の値を返すため。实际上は `ascension` は冗長。

**対応**:
- 案A: `ascension` フィールドを削除
- 案B: バリデーション用途のみ使用（level と ascension の一致確認）

---

### 5. 武器レベル index 計算の不一致 (中優先度)

**場所**: 
- `crates/data/src/team_builder.rs:14-21`
- `crates/good/src/build_stats.rs:23`

```rust
// team_builder.rs
fn weapon_stat_index(level: u32) -> usize {
    match level {
        1..=20 => 0,
        21..=40 => 1,
        41..=70 => 2,
        _ => 3,  // 71-90
    }
}

// build_stats.rs
profile.base_atk += wb.weapon.base_atk[3];  // 常に index 3
```

**問題**:
- `TeamMemberBuilder` は武器レベルに応じた index を使用
- `build_stat_profile()` は常に index 3 (Lv90相当) を使用

**対応**: `build_stat_profile()` に武器レベル引数を追加し、一貫した index 計算を使用。

---

### 6. 2つの main stat 計算関数の存在 (低優先度)

**場所**:
- `crates/data/src/artifact_stats.rs` → `artifact_main_stat_value()`
- `crates/good/src/stat_map.rs` → `main_stat_value()`

| 関数 | 入出力 | 補間方式 |
|------|--------|----------|
| `artifact_main_stat_value()` | `ArtifactRarity`, `ArtifactSlot`, `&BuffableStat`, level | Table lookup (1/4/8/12/16/20) |
| `main_stat_value()` | `rarity: u8`, `main_stat_key: &str`, level | 線形補間 |

**問題**:
- 同じ機能を持つ関数が2つある
- 補間方式が異なるため、計算結果に微小な誤差が出る可能性

**対応**: 統一した関数を使用し、`stat_map.rs` から `artifact_stats.rs` への依存に統一。

---

### 7. `ArtifactSet` 所有権と参照の混乱 (低優先度)

**場所**: `crates/good/src/lib.rs:44-49`

```rust
pub struct ArtifactsBuild {
    pub sets: Vec<&'static ArtifactSet>,           // 所有権なし (参照)
    pub four_piece_set: Option<&'static ArtifactSet>,  // 所有権なし (参照)
    pub stats: StatProfile,                       // main + sub + 2pc buffs
}
```

**問題**:
- `sets` と `four_piece_set` が同じデータへの参照
- `stats` には artifact main/substats と 2pc buffs の両方が含まれる
- 何を buff として処理し、何を直接 stats として処理するか呼び出し側が判断できない

**対応**: `ArtifactsBuild` の構造を見直し、`stats` に含む内容を明確化。

---

## 対応優先度まとめ

| 優先度 | 問題 | 対応状況 |
|--------|------|----------|
| ~~**高**~~ | ~~武器レベルの無視~~ | ✅ 解決済み (convert.rsでweapon_level()使用) |
| ~~**高**~~ | ~~2pc buffs handling 分散~~ | ✅ 解決済み (全set bonusをbuffs_providedに統一) |
| ~~**中**~~ | ~~base stats ロジック重複~~ | ✅ 解決済み (build_stat_profile()武器レベル对应) |
| ~~**中**~~ | ~~武器レベル index 不一致~~ | ✅ 解決済み (build_stat_profile()武器レベル对应) |
| ~~**中**~~ | ~~ascension フィールド未使用~~ | ✅ 解決済み (CharacterBuildからascension削除) |
| ~~**低**~~ | ~~2つの main stat 関数~~ | ✅ 解決済み (good→data依存に統一) |

---

## 理想アーキテクチャ

```
GOOD JSON
    ↓
[Step 1: Parsing]
GoodFormat (raw JSON)
    ↓
CharacterBuild (resolved pointers: CharacterData*, WeaponData*, ArtifactData[])
    ↓
[Step 2: Stat Calculation]
StatProfile {
    base_hp, base_atk, base_def,  // character level
    weapon_base_atk,               // weapon level
    hp_percent, atk_percent, ...   // artifact main/substats only
}
    ↓
[Step 3: Buff Resolution]
TeamMember {
    stats: StatProfile,
    buffs_provided: Vec<ResolvedBuff>  // weapon passive + all set effects
}
```

### 変更ポイント

1. **`build_stat_profile()`** に武器レベル引数を追加
2. **`ArtifactsBuild.stats`** を artifact main/substats のみに限定
3. **2pc/4pc buffs** をすべて `buffs_provided` に移動
4. **base stats 構築** を `genshin_calc_data::team_builder` に抽取

---

## 関連ファイル

- `crates/good/src/lib.rs` - `CharacterBuild`, `ArtifactsBuild` 定義
- `crates/good/src/convert.rs` - GOOD → CharacterBuild 変換
- `crates/good/src/build_stats.rs` - CharacterBuild → StatProfile
- `crates/good/src/stat_map.rs` - StatKey → StatField 変換
- `crates/data/src/team_builder.rs` - TeamMemberBuilder
- `crates/data/src/artifact_stats.rs` - アーティファクト main stat 定義
- `crates/core/src/stat_profile.rs` - StatProfile, combine_stats
