# import_good: ArtifactSetEntry + Traveler対応 (issue #37)

## Problem

`import_good()` が返す `ArtifactsBuild` に以下の問題がある:

1. **piece_count 情報の欠落**: `sets: Vec<&'static ArtifactSet>` は各セットのピース数を保持しない。4pcか2pcかを区別できず、JS側で conditional buff の出し分けが不可能
2. **TeamMemberBuilder の誤適用**: `team_builder.rs` は `artifact_sets` 内の全セットに対して2pc/4pc両方のbuffを無条件適用している。2pc-onlyセットにも4pc buffが適用されるバグが潜在
3. **Traveler 未対応**: GOODの `"Traveler"` キーは要素情報を持たないが、内部IDは `traveler_dendro` 等の要素別。マッピング不可能でスキップされる

## Scope

- `ArtifactsBuild.sets` を `Vec<ArtifactSetEntry>` に変更（piece_count付き）
- `TeamMemberBuilder` のbuff適用を piece_count でフィルタ
- `import_good()` に `traveler_element: Option<Element>` を追加
- 未実装データ（Manekina, LongNightsOath, FinaleOfTheDeepGalleries）は別issue

## Design

### 1. ArtifactSetEntry 型

`crates/good/src/lib.rs` に新型を追加:

```rust
/// An artifact set with its detected piece count.
///
/// `Serialize` only — `Deserialize` is not derived because the `set` field
/// is a `&'static` reference (per project convention: `&'static` types are Serialize-only).
#[derive(Debug, Clone, serde::Serialize)]
pub struct ArtifactSetEntry {
    /// Artifact set data.
    pub set: &'static genshin_calc_data::types::ArtifactSet,
    /// Effective piece threshold: always exactly 2 or 4.
    /// Represents the highest activated set bonus, not the raw equipped count
    /// (e.g., 5 pieces of the same set → piece_count = 4).
    pub piece_count: u8,
}
```

`ArtifactsBuild.sets` の型を `Vec<&'static ArtifactSet>` → `Vec<ArtifactSetEntry>` に変更。

### 2. detect_sets() 変更

`crates/good/src/convert.rs`:

```rust
fn detect_sets(
    counts: &HashMap<&'static str, (&'static ArtifactSet, u8)>,
) -> Vec<ArtifactSetEntry> {
    let mut four_piece: Option<ArtifactSetEntry> = None;
    let mut two_pieces: Vec<ArtifactSetEntry> = Vec::new();

    for &(set, count) in counts.values() {
        if count >= 4 {
            four_piece = Some(ArtifactSetEntry { set, piece_count: 4 });
        } else if count >= 2 {
            two_pieces.push(ArtifactSetEntry { set, piece_count: 2 });
        }
    }

    if let Some(fp) = four_piece {
        vec![fp]
    } else {
        two_pieces
    }
}
```

4pc/2pcの排他制御を維持: 4pcセットがあれば2pcセットは返さない（既存の動作と同一）。

### 3. TeamMemberBuilder の piece_count 対応

`crates/data/src/team_builder.rs`:

- `artifact_sets: Vec<&'static ArtifactSet>` → `Vec<ArtifactSetEntry>` に変更
- `ArtifactSetEntry` は `genshin_calc_good` crate に定義されるが、`data` crate は `good` に依存していない

**依存方向の問題**: `data` crate が `good` crate の型を使うのは依存逆転。解決策:

**A) ArtifactSetEntry を data crate に定義** — `good` crate がそれを re-export。`data` は外部依存なし。

```
data::types::ArtifactSetEntry  ← 定義元
good::ArtifactSetEntry         ← pub use data::types::ArtifactSetEntry
```

これを採用する。`ArtifactSetEntry` は `crates/data/src/types.rs` に定義し、`good` crate は re-export する。

#### collect_static_buffs() 変更

```rust
for entry in &self.artifact_sets {
    // 2pc buffs: always apply (piece_count >= 2)
    for stat_buff in entry.set.two_piece.buffs {
        buffs.push(ResolvedBuff { source: format!("{} 2pc", entry.set.name), ... });
    }
    // 4pc buffs: only if piece_count >= 4
    if entry.piece_count >= 4 {
        for stat_buff in entry.set.four_piece.buffs {
            buffs.push(ResolvedBuff { source: format!("{} 4pc", entry.set.name), ... });
        }
    }
}
```

#### resolve_conditional_buffs() 変更

同様のフィルタ。2pc conditional は常に、4pc conditional は `piece_count >= 4` の場合のみ。

#### available_conditionals() 変更

同様。4pc conditional は `piece_count >= 4` の場合のみ返す。

#### convenience メソッド更新

```rust
pub fn artifact_set(mut self, set: &'static ArtifactSet) -> Self {
    self.artifact_sets = vec![ArtifactSetEntry { set, piece_count: 4 }];
    self
}

pub fn artifact_sets(mut self, sets: Vec<ArtifactSetEntry>) -> Self {
    self.artifact_sets = sets;
    self
}
```

### 4. import_good() シグネチャ変更

`crates/good/src/lib.rs`:

```rust
/// Import options for GOOD format parsing.
#[derive(Debug, Clone, Default)]
pub struct ImportOptions {
    /// Element to use for the Traveler character.
    /// If None, Traveler entries produce an UnknownCharacter warning.
    pub traveler_element: Option<genshin_calc_core::Element>,
}

pub fn import_good(json: &str) -> Result<GoodImport, GoodError> {
    import_good_with_options(json, &ImportOptions::default())
}

pub fn import_good_with_options(json: &str, options: &ImportOptions) -> Result<GoodImport, GoodError> {
    let good: GoodFormat = serde_json::from_str(json)?;
    convert_good_with_options(good, options)
}
```

後方互換: 既存の `import_good()` はそのまま動作。新しい `import_good_with_options()` で Traveler 対応。

### 5. Traveler ルックアップ

`crates/good/src/key_map.rs`:

```rust
pub fn lookup_character_with_traveler(
    good_key: &str,
    traveler_element: Option<Element>,
) -> Option<&'static CharacterData> {
    if good_key == "Traveler" || good_key == "traveler" {
        let element = traveler_element?;
        let id = format!("traveler_{}", element_to_suffix(element));
        return genshin_calc_data::find_character(&id);
    }
    lookup_character(good_key)
}

fn element_to_suffix(element: Element) -> &'static str {
    match element {
        Element::Pyro => "pyro",
        Element::Hydro => "hydro",
        Element::Electro => "electro",
        Element::Cryo => "cryo",
        Element::Dendro => "dendro",
        Element::Anemo => "anemo",
        Element::Geo => "geo",
    }
}
```

### 6. to_team_member_builder() 更新

`crates/good/src/convert.rs`:

```rust
builder = builder.artifact_sets(build.artifacts.sets.clone());
```

型が一致するので `.clone()` で直接渡す。

### 7. WASM API 更新

`crates/wasm/src/lib.rs`:

```rust
#[wasm_bindgen]
pub fn import_good(json: &str) -> Result<JsValue, JsError> { ... }

#[wasm_bindgen]
pub fn import_good_with_options(json: &str, traveler_element: Option<String>) -> Result<JsValue, JsError> {
    let options = ImportOptions {
        traveler_element: traveler_element.and_then(|e| parse_element(&e)),
    };
    let result = genshin_calc_good::import_good_with_options(json, &options)
        .map_err(|e| JsError::new(&e.to_string()))?;
    to_js(&result)
}
```

既存の `import_good` WASM関数は後方互換維持。以下の5つのWASM関数が内部で `import_good()` を呼んでおり、全てに `traveler_element: Option<String>` 引数を追加する:

- `build_stats_from_good` (lib.rs:200)
- `build_stats` (lib.rs:237)
- `get_character_team_buffs` (lib.rs:324)
- `build_team_member` (lib.rs:369)
- `build_member_stats` (lib.rs:404)

各関数は内部で `import_good()` → `import_good_with_options()` に切り替える。引数は `Option<String>` (WASM境界) → `Option<Element>` (Rust内部) に変換。

## Files to Modify

| File | Changes |
|------|---------|
| `crates/data/src/types.rs` | `ArtifactSetEntry` 定義追加 |
| `crates/data/src/lib.rs` | `ArtifactSetEntry` re-export |
| `crates/data/src/team_builder.rs` | `artifact_sets` 型変更、buff適用にpiece_countフィルタ |
| `crates/good/src/lib.rs` | `ImportOptions` 追加、`import_good_with_options()` 追加、`ArtifactsBuild.sets` 型変更、`ArtifactSetEntry` re-export |
| `crates/good/src/convert.rs` | `detect_sets()` 戻り値変更、`build_imports()` Traveler処理、`to_team_member_builder()` 更新 |
| `crates/good/src/key_map.rs` | `lookup_character_with_traveler()` 追加 |
| `crates/wasm/src/lib.rs` | `import_good_with_options()` WASM binding、既存関数の `ArtifactSetEntry` 対応 |
| `crates/good/examples/demo.rs` | `.iter().copied().collect()` → `.clone()` に変更 |
| `crates/good/src/convert.rs` (tests mod) | `make_build()` ヘルパーの `sets` 型を `Vec<ArtifactSetEntry>` に更新 |
| `crates/good/tests/test_convert.rs` | piece_count 検証テスト追加 |
| `crates/good/tests/evaluate_talent_buffs_integration.rs` | 必要に応じ更新 |
| `crates/data/tests/` | TeamMemberBuilder の piece_count フィルタテスト |

## Breaking Changes

| Change | Impact | Migration |
|--------|--------|-----------|
| `ArtifactsBuild.sets` 型変更 | JS: `build.artifacts.sets[0]` → `build.artifacts.sets[0].set` | smart-paimon 側で `.set` アクセス追加 |
| `TeamMemberBuilder::artifact_sets()` 型変更 | Rust API 利用者 | `ArtifactSetEntry { set, piece_count: 4 }` でラップ |
| `import_good()` シグネチャ | 変更なし（後方互換） | `import_good_with_options()` を新規利用 |

## Notes

### Traveler の要素制限

現在 data crate に実装済みの Traveler は `traveler_dendro` のみ。他の要素（Pyro, Hydro等）を `traveler_element` に指定した場合、`find_character()` が `None` を返し `UnknownCharacter` 警告になる。これは想定動作であり、他の要素の Traveler は data crate への追加時に自動的に対応される。テストではこのケース（有効な要素だが未実装）もカバーすること。

### Public API exports

`ImportOptions` は `crates/good/src/lib.rs` の crate root で `pub use` する（Rust API 利用者が直接構築する必要があるため）。`ArtifactSetEntry` は `data` crate で定義、`good` crate で re-export。

### Doc comment 更新

`lib.rs:109` の doc example `build.artifacts.sets.iter().map(|s| s.name)` は `|s| s.set.name` に変更。

## JSON Output Change

Before:
```json
{ "sets": [{ "id": "crimson_witch", ... }], "stats": {...} }
```

After:
```json
{ "sets": [{ "set": { "id": "crimson_witch", ... }, "piece_count": 4 }], "stats": {...} }
```
