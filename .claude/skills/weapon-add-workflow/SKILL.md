---
name: weapon-add-workflow
description: End-to-end execution skill for adding new Genshin Impact weapons — fetches data, classifies passives, dispatches subagents in a worktree, verifies, and merges. Handles batch addition of multiple weapons.
version: 1.0.0
---

# Weapon Add Workflow

武器追加をワンショットで実行するスキル。
キャラ追加 (`character-add-workflow`) と同様、brainstorming/writing-plans をスキップして直接実行する。

## Trigger

- ユーザーが「武器追加」「{名前}を実装」「新武器データ」に言及したとき
- `/game-update-todo` で生成されたTODOのうち武器関連タスクを実装するとき

## キャラ追加との違い

| 項目 | キャラ | 武器 |
|------|--------|------|
| ファイル | 個別 .rs | 武器種別 .rs に追記 |
| テスト | 個別 TOML | data_integrity テストが自動検証 |
| 天賦バフ | talent_buffs/{element}.rs | conditional_buffs (武器定義内) |
| 月兆 | moonsign_chars.rs | なし |
| バッチ実行 | 1体ずつ | **複数本を1 worktree で実行可** |

---

## Flow

```
Phase 1: Data Fetch + Passive Classification (このセッション)
  ├── 武器一覧確認 (TODO から対象武器を把握)
  ├── Honey Impact WebFetch (各武器)
  ├── パッシブ分類判定 (SKIP / STATIC / CONDITIONAL)
  └── スコープ提示 → ユーザー確認 (1メッセージ)

Phase 2: Worktree + Subagent Dispatch (このセッション)
  ├── git worktree 作成
  ├── 武器種別ごとに subagent dispatch (1 subagent = 同一 .rs の全武器)
  └── mod.rs 登録 (ALL_WEAPONS への追加)

Phase 3: Verify + Merge (このセッション)
  ├── cargo build + test + clippy + fmt
  ├── TODO 更新
  └── main にマージ + worktree 削除
```

---

## Phase 1: Data Fetch + Passive Classification

### 1-1. 対象武器の確認

`docs/v6.0-6.3-data-update-todo.md` のセクション5から未実装武器を抽出。

### 1-2. Honey Impact データ取得

```
WebFetch: https://gensh.honeyhunterworld.com/i_n{weapon_id}/?lang=EN
```

武器名で検索する場合:
```
WebFetch: https://gensh.honeyhunterworld.com/{weapon_name_kebab}/?lang=EN
```

抽出対象:
- 基本情報 (名前, 武器種, ★)
- base_atk [Lv1, Lv20, Lv80, Lv90]
- sub_stat (種類 + [Lv1, Lv20, Lv80, Lv90])
- パッシブ名 + 効果テキスト
- 精錬 R1-R5 の数値

### 1-3. パッシブ分類判定

**CLAUDE.md の武器 ConditionalBuff 分類ルールに従う:**

| パッシブ内容 | 分類 | 実装方法 |
|-------------|------|---------|
| ATK/DMG/CRIT等のステータスバフ (常時) | **STATIC** | `buffs: &[StatBuff {...}]` |
| ATK/DMG/CRIT等 (条件付き) | **CONDITIONAL** | `conditional_buffs: &[ConditionalBuff {...}]` |
| proc damage (追加ヒットのみ) | **SKIP** | `conditional_buffs: &[]` |
| エネルギー生成 (Favonius系) | **SKIP** | `conditional_buffs: &[]` |
| CDリセット (Sacrificial系) | **SKIP** | `conditional_buffs: &[]` |
| HP回復 | **SKIP** | `conditional_buffs: &[]` |
| 純ユーティリティ (移動速度等) | **SKIP** | `conditional_buffs: &[]` |

**StatBuff と ConditionalBuff の重複注意:**
StatBuff にある効果が条件付きの場合、StatBuff を conditional_buffs に移動し `buffs: &[]` にする（二重計上防止）。

### 1-4. Conditional パターン判定

| 条件テキスト | Activation パターン |
|-------------|-------------------|
| 「HP50%以下で」「敵が近くにいる時」等 | `Manual(Toggle)` |
| 「N スタックまで」「N 層まで」 | `Manual(Stacks(N))` |
| 「EM に基づいて」「DEF の N% 分」 | `Auto(StatScaling { stat, offset, cap })` |
| 「チーム内の異なる元素数に応じて」 | `Auto(TeamDiffElementCount { min_count })` |
| 「チーム内に N 人以上の同元素」 | `Auto(TeamSameElementCount { min_count })` |
| 「チームエネルギー合計に基づく」 | `Manual(Toggle)` (StatScaling不可、単一キャラstat参照のみ) |

### 1-5. スコープ提示

```
## 武器追加スコープ ({N}本)

| # | 武器名 | 種類/★ | sub_stat | パッシブ分類 |
|---|--------|--------|----------|------------|
| 1 | {name} | {type}/★{n} | {stat} | STATIC/CONDITIONAL/SKIP |
| 2 | ... | ... | ... | ... |

CONDITIONAL 武器の詳細:
- {weapon}: {条件} → {Activation パターン}

これで進めますか？
```

---

## Phase 2: Worktree + Subagent Dispatch

### 2-0. Worktree 作成

```bash
git worktree add .claude/worktrees/feat-weapons-v{version} -b feat/weapons-v{version}
```

### 2-1. 武器種別ごとの Subagent Dispatch

**同一 .rs ファイルに追加する武器はまとめて1つの subagent で実行。**

例: Sword 3本 + Catalyst 2本 → 2 subagent (sword subagent, catalyst subagent)

subagent に渡す情報:
- Worktree パス
- 追加する武器リスト (各武器の全データ: base_atk, sub_stat, passive 分類結果)
- Honey Impact URL (subagent が倍率を自前取得する場合)
- 該当 .rs ファイルの末尾付近の既存武器パターン（参照用）
- パッシブ分類結果と Activation パターン

subagent の責務:
1. `{weapon_type}.rs` に武器定義を追加
2. `mod.rs` の `ALL_WEAPONS` に追加
3. `cargo build -p genshin-calc-data` で検証
4. `cargo test -p genshin-calc-data` で検証 (data_integrity テスト)
5. コミット

**独立した武器種の subagent は並列 dispatch 可能。** ただし mod.rs を共有するため、最後の subagent が ALL_WEAPONS をまとめて更新するか、順次実行にする。

### 2-2. mod.rs 登録戦略

**Option A (推奨): 各 subagent が自分の武器を ALL_WEAPONS に追加**
- 並列 dispatch 不可（mod.rs 競合）
- 安全で確実

**Option B: 最後に1つの subagent が mod.rs をまとめて更新**
- 並列 dispatch 可能
- mod.rs 更新を忘れるリスク

→ 武器数が少ない場合は Option A（順次実行）を推奨。

---

## Phase 3: Verify + Merge

### 3-1. 最終検証

```bash
cd {worktree_path}
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

確認項目:
- `all_weapons_have_positive_base_atk` パス
- `all_weapons_have_unique_ids` パス
- `test_all_conditional_buff_names_unique` パス
- `test_all_stacks_max_positive` パス

### 3-2. TODO 更新

`docs/v6.0-6.3-data-update-todo.md` のセクション5の該当武器のチェックボックスを更新。コミット。

### 3-3. マージ

ユーザーに確認後:
```bash
cd /home/sakis/genshin-calc
git merge feat/weapons-v{version} --no-ff
git worktree remove .claude/worktrees/feat-weapons-v{version}
git branch -d feat/weapons-v{version}
```

---

## Domain Knowledge (Reference)

### ファイル配置

| 種類 | パス |
|------|------|
| Sword | `crates/data/src/weapons/sword.rs` |
| Claymore | `crates/data/src/weapons/claymore.rs` |
| Polearm | `crates/data/src/weapons/polearm.rs` |
| Bow | `crates/data/src/weapons/bow.rs` |
| Catalyst | `crates/data/src/weapons/catalyst.rs` |
| 登録 | `crates/data/src/weapons/mod.rs` → `ALL_WEAPONS` |
| TODO | `docs/v6.0-6.3-data-update-todo.md` セクション5 |

### WeaponData 構造

```rust
pub const WEAPON_NAME: WeaponData = WeaponData {
    id: "weapon_name",             // snake_case, グローバルユニーク
    name: "Weapon Name",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [Lv1, Lv20, Lv80, Lv90],
    sub_stat: Some(WeaponSubStat::CritRate([Lv1, Lv20, Lv80, Lv90])),
    passive: Some(WeaponPassive {
        name: "パッシブ名",
        effect: PassiveEffect {
            description: "効果説明",
            buffs: &[StatBuff { ... }],             // 常時バフ
            conditional_buffs: &[ConditionalBuff { ... }],  // 条件付きバフ
        },
    }),
};
```

### sub_stat 種類

| Honey Impact 表示 | Rust |
|-------------------|------|
| ATK% | `WeaponSubStat::AtkPercent([...])` |
| HP% | `WeaponSubStat::HpPercent([...])` |
| DEF% | `WeaponSubStat::DefPercent([...])` |
| CRIT Rate | `WeaponSubStat::CritRate([...])` |
| CRIT DMG | `WeaponSubStat::CritDmg([...])` |
| Elemental Mastery | `WeaponSubStat::ElementalMastery([...])` |
| Energy Recharge | `WeaponSubStat::EnergyRecharge([...])` |
| Physical DMG Bonus | `WeaponSubStat::PhysicalDmgBonus([...])` |

**EM/HPフラットは小数変換しない。それ以外のパーセンテージ値は小数変換する。**

### 精錬値

`[R1, R2, R3, R4, R5]` — 必ず5値。

### パッシブなし武器

★3以下や一部の★4 武器はパッシブなし: `passive: None`

### ConditionalBuff テンプレート

```rust
// Toggle パターン
ConditionalBuff {
    name: "{weapon_id}_{effect}",
    description: "条件: ...",
    stat: BuffableStat::AtkPercent,
    value: 0.20,
    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}

// Stacks パターン (非線形)
ConditionalBuff {
    name: "{weapon_id}_{effect}_stacks",
    description: "スタック: ...",
    stat: BuffableStat::AtkPercent,
    value: 0.10,
    refinement_values: None,
    stack_values: Some(&[0.10, 0.20, 0.30, 0.48]),
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Stacks(4)),
}

// StatScaling パターン
ConditionalBuff {
    name: "{weapon_id}_{effect}_scaling",
    description: "EM基づく: ...",
    stat: BuffableStat::ChargedAtkFlatDmg,
    value: 1.60,
    refinement_values: Some([1.60, 2.00, 2.40, 2.80, 3.20]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Auto(AutoCondition::StatScaling {
        stat: BuffableStat::ElementalMastery,
        offset: None,
        cap: None,
    }),
}
```

### Common Pitfalls

| ミス | 正しい対処 |
|------|-----------|
| パーセンテージ小数変換忘れ | EM/HPフラット以外は小数変換 |
| sub_stat の EM を小数変換 | EM はそのまま (58.0, 241.0 等) |
| conditional_buff name の重複 | `{weapon_id}_{effect}` でグローバルユニーク |
| StatBuff と ConditionalBuff の二重計上 | 条件付きなら StatBuff → ConditionalBuff に移動、`buffs: &[]` |
| チームエネルギー系に StatScaling 使用 | Toggle を使う (StatScaling は単一キャラ stat 参照のみ) |
| ALL_WEAPONS への登録忘れ | mod.rs の ALL_WEAPONS スライスに追加 |
| 精錬値が4値 | 必ず5値 [R1-R5] |

---

## Related Skills

| スキル | 役割 | いつ使う |
|--------|------|---------|
| `honeyhunter-data-fetch` | 数値データ取得のフォーマット規約 | subagent がデータ取得時に参照 |
| `character-add-workflow` | キャラ追加スキル | 同バージョンのキャラ追加時 |
| `game-update-todo` | バージョン全体のTODOリスト生成 | 全体把握 |
