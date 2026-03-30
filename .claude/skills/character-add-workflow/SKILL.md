---
name: character-add-workflow
description: Domain knowledge reference for adding a new Genshin Impact character — file structure, implementation steps, validation rules, and common pitfalls. Used by superpowers brainstorming/planning phases.
version: 1.0.0
---

# Character Add Workflow

新キャラクターをデータクレートに追加するためのドメイン知識リファレンス。

**このスキルは実行を制御しない。** superpowers:brainstorming でスコープ確認、superpowers:writing-plans でタスク分解するときに、ドメイン知識として参照する。実際の実行は subagent-driven-development が行う。

## Trigger

- ユーザーが「キャラ追加」「{名前}を実装」「新キャラデータ」に言及したとき
- `/game-update-todo` で生成されたTODOのうちキャラ関連タスクを実装するとき

## Role in Superpowers Flow

```
brainstorming           → このスキルのStep Map でスコープを確認
  ↓                       どのステップが必要か判断（月兆あり？天賦バフあり？）
writing-plans           → このスキルのStep Map をタスク分解の入力にする
  ↓                       各ステップを独立タスクとして計画に落とす
subagent-driven-dev     → 各タスクのsubagentに該当ステップの詳細を渡す
                          subagentは honeyhunter-data-fetch スキルも参照
```

**brainstorming で聞くべき質問（このスキルが提供）:**
- 月兆対応キャラか？ → Step 5 の要否
- チームバフ/自己バフを持つか？ → Step 4 の要否とスコープ
- デュアルスケーリング天賦があるか？ → TalentScaling の分割パターン

**brainstorming で聞かなくてよい質問:**
- ファイル配置（このスキルで定義済み）
- 数値フォーマット（honeyhunter-data-fetch スキルで定義済み）
- テストの書き方（testing.md で定義済み）

---

## Step Map

各ステップは独立タスクとして subagent に dispatch 可能。依存関係に注意。

```
Step 1: キャラデータ定義 (.rs ファイル作成)
  ↓ 依存
Step 2: mod 登録 (mod.rs に追加)
  ↓ 依存
Step 3: TOML テストケース作成
  |
  ├── Step 4: 天賦バフ定義 (該当キャラのみ, Step 2 に依存)
  |
  └── Step 5: 月兆データ定義 (該当キャラのみ, Step 2 に依存)
  ↓ 全完了後
Step 6: 検証 (cargo build + test + clippy + fmt)
```

**並列化可能:** Step 3, 4, 5 は互いに独立。ただし全て Step 2 完了後。

---

## Step 1: キャラデータ定義

**出力先:** `crates/data/src/characters/{element}/{char_id}.rs`（新規ファイル）

**データ取得:** `/honeyhunter-data-fetch` スキルのワークフローに従う

### 必須フィールド

```rust
pub const {CHAR_UPPER}: CharacterData = CharacterData {
    id: "{char_id}",           // snake_case, グローバルユニーク
    name: "{表示名}",
    element: Element::{Xxx},
    weapon_type: WeaponType::{Xxx},
    rarity: Rarity::Star{N},
    region: Region::{Xxx},
    base_hp: [Lv1, Lv20, Lv80, Lv90],    // 突破前の値
    base_atk: [Lv1, Lv20, Lv80, Lv90],
    base_def: [Lv1, Lv20, Lv80, Lv90],
    ascension_stat: AscensionStat::{Xxx}(値),  // Lv90突破後の合計値
    talents: TalentSet { normal, skill, burst },
    constellation_pattern: ConstellationPattern::{C3SkillC5Burst|C3BurstC5Skill},
};
```

### 天賦スケーリング定義

各段ごとに `const` + `TalentScaling`:

```rust
const CHAR_NA_HIT1: TalentScaling = TalentScaling {
    name: "1-Hit DMG",
    scaling_stat: ScalingStat::Atk,    // Hp/Def/Em の場合あり
    damage_element: None,              // 物理=None, 元素=Some(Element::Xxx)
    values: [f64; 15],                 // Lv1-Lv15、必ず15値
};
```

**元素判定ルール:**

| 武器種 | 通常攻撃 | 重撃 | スキル/爆発 |
|--------|---------|------|------------|
| Catalyst | `Some(Element)` | `Some(Element)` | `Some(Element)` |
| Sword/Claymore/Polearm | `None` (物理) | `None` (物理) | `Some(Element)` |
| Bow | `None` (物理) | フルチャージ: `Some(Element)` | `Some(Element)` |

**デュアルスケーリング:** ATK+EM 等の天賦は `_ATK` と `_EM` の2エントリに分割（Lauma/Nefer パターン）。

---

## Step 2: mod 登録

**対象ファイル:** `crates/data/src/characters/{element}/mod.rs`

3箇所の変更:

```rust
// 1. mod宣言
mod {char_id};

// 2. pub use
pub use {char_id}::{CHAR_UPPER};

// 3. CHARACTERS スライスに追加
pub const CHARACTERS: &[CharacterData] = &[
    // ... 既存キャラ ...
    {char_id}::{CHAR_UPPER},  // ← 追加
];
```

---

## Step 3: TOML テストケース

**出力先:** `crates/core/tests/data/characters/{char_id}.toml`

**最小構成:** 通常攻撃1段目、反応なし、ベーシックなステータス

```toml
[character]
name = "{CharacterName}"
element = "{Element}"

[[cases]]
type = "normal"
name = "Normal Attack Hit 1 — no reaction"
character_level = 90
talent_multiplier = {1段目Lv10の値}
scaling_stat = "Atk"           # or "Hp", "Def", "Em"
damage_type = "Normal"
element = "{Element}"          # 物理の場合は "Physical"

[cases.stats]
hp = 15000.0
atk = 2000.0
def = 800.0
elemental_mastery = 100.0
crit_rate = 0.50
crit_dmg = 1.00
energy_recharge = 1.2
dmg_bonus = 0.466

[cases.enemy]
level = 90
resistance = 0.10

[cases.expected]
non_crit = {手計算値}
crit = {手計算値}
average = {手計算値}
```

### expected 値の計算方法

```
base_damage = atk × talent_multiplier
dmg_multiplier = 1 + dmg_bonus
defense_mult = (char_level + 100) / ((char_level + 100) + (enemy_level + 100))
              = 190 / (190 + 190) = 0.5
resistance_mult = 1 - resistance = 0.9

non_crit = base_damage × dmg_multiplier × defense_mult × resistance_mult
crit = non_crit × (1 + crit_dmg)
average = non_crit × (1 + crit_rate × crit_dmg)
```

**推奨追加ケース:**
- 元素スキル/爆発のダメージ
- 蒸発/溶解（増幅反応）
- 超激化/草激化（catalyze — 該当元素のみ）

---

## Step 4: 天賦バフ定義（該当キャラのみ）

**出力先:** `crates/data/src/talent_buffs/{element}.rs`

**実装対象の判別:**

| パッシブ内容 | 実装する？ |
|-------------|-----------|
| ATK/DMG/CRIT等のステータスバフ | YES — TalentBuffDef |
| 元素耐性ダウン | YES — TalentBuffDef |
| 追加ヒット/proc damage | NO |
| HP回復/シールド | NO |
| クールダウン短縮 | NO |

**対象:**
- 固有天賦 A1/A4 でステータスバフがあるもの
- 命の星座 C1-C6 で主要なステータスバフ
- チームバフ: `target: BuffTarget::Team`
- 自己バフ: `target: BuffTarget::Character("{char_id}")`

---

## Step 5: 月兆データ定義（該当キャラのみ）

**出力先:** `crates/data/src/moonsign_chars.rs`

**該当判定:** Honey Impact / Wiki に Moonsign セクションがあるキャラ（Luna以降の新キャラが対象）

**MoonsignTalentEffect の種類:**
- `GrantReactionCrit` — 月反応に会心付与
- `StatBuff` — ステータスバフ
- `ReactionDmgBonus` — 反応ダメージボーナス

---

## Step 6: 検証

```bash
cargo build
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

**特に確認:**
- データ整合性テスト（ID重複なし、base stats正値）
- `character_verification` テスト（TOML ケース通過）
- 天賦バフ名のユニーク性

---

## Common Pitfalls

| ミス | 正しい対処 |
|------|-----------|
| パーセンテージを小数変換し忘れ（`88.2` → `0.882`） | honeyhunter-data-fetch の規約参照 |
| Catalyst通常攻撃の `damage_element: None` | `Some(Element::Xxx)` が正しい |
| base_stat で突破前/後を混同 | Lv20=突破**前**, Lv80=突破**前** |
| 天賦スケーリングが13値 | 必ず15値 (Lv1-15) |
| mod.rs の CHARACTERS スライスへの登録忘れ | Step 2 を忘れずに |
| 天賦バフ名が既存と重複 | `{char_id}_{effect}` でユニークにする |
| TOMLの expected を概算で書く | 上記の計算式で正確に算出 |

---

## Related Skills

| スキル | 役割 | いつ使う |
|--------|------|---------|
| `game-update-todo` | バージョン全体のTODOリスト生成 | 複数キャラ/武器/聖遺物をまとめて把握 |
| `honeyhunter-data-fetch` | 数値データ取得とRust定数変換 | Step 1 のデータ取得時 |

## Related Docs

| ドキュメント | 内容 |
|-------------|------|
| `docs/dev/character-data-guide.md` | デュアルスケーリング等の実装パターン |
| `docs/dev/testing.md` | テスト構造・許容誤差・検証済みキャラ |
| `docs/dev/architecture.md` | crate構造とファイル配置 |
