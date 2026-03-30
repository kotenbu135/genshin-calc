---
name: honeyhunter-data-fetch
description: Fetch Genshin Impact game data (characters, weapons, artifacts, enemies) from Honey Impact (gensh.honeyhunterworld.com) and convert to Rust const definitions
version: 1.0.0
source: local-git-analysis
analyzed_commits: 50
---

# Honey Impact データ取得スキル

Webからゲームの数値データ（キャラ・武器・聖遺物・敵）を Honey Impact から取得し、
本プロジェクトの Rust const 定義に変換するワークフロー。

## Trigger

- `docs/v6.0-6.3-data-update-todo.md` やバージョン別TODOに記載のデータを実装するとき
- ユーザーが「Honeyhunterからデータ取得して」「キャラデータ追加して」と指示したとき
- `/game-update-todo` で生成されたTODOリストの実装時

## データソース

**プライマリ**: `https://gensh.honeyhunterworld.com/`（Honey Impact）

URL パターン:

| カテゴリ | URL | 例 |
|----------|-----|-----|
| キャラクター | `/{name}/?lang=EN` | `/lauma/?lang=EN` |
| 武器 | `/i_n{id}/?lang=EN` | `/i_n11501/?lang=EN` |
| 聖遺物 | サイト内検索 or カテゴリページ | — |
| 敵 | `/mcat_{code}/?lang=EN` | — |

**補助**: `genshin-impact.fandom.com`（Wikiに掲載がある場合の補完用）

---

## ワークフロー

### Step 1: ページ取得

WebFetch で該当ページを取得する。`?lang=EN` を付けて英語版を使用。

```
WebFetch: https://gensh.honeyhunterworld.com/{entity}/?lang=EN
```

**注意点:**
- サイトはJavaScript動的レンダリングのため、WebFetchで全データが取れない場合がある
- その場合は Fandom Wiki (`genshin-impact.fandom.com/wiki/{CharacterName}`) にフォールバック
- 天賦倍率が取れない場合は `genshin-db-api.vercel.app` も参照可

### Step 2: データ抽出・変換

取得したHTMLからデータを読み取り、以下のフォーマット規約に従って変換する。

---

## フォーマット規約

### パーセンテージ → 小数

Honey Impact表示の `%` 値はすべて小数に変換する:
- `10.8%` → `0.108`
- `88.2%` → `0.882`
- `33.7%` → `0.337`

### base_hp / base_atk / base_def

4値配列: `[Lv1, Lv20, Lv80, Lv90]`

Honey Impactのレベル進行テーブルから以下を抽出:
- Lv1: 初期値
- Lv20: Ascension 0の最大（突破前）
- Lv80: Ascension 5の最大（突破前）
- Lv90: Ascension 6の最大（最終値）

```rust
base_hp: [829.0, 2148.0, 9461.0, 10654.0],
base_atk: [20.0, 51.0, 226.0, 255.0],
base_def: [53.0, 138.0, 607.0, 684.0],
```

### 天賦スケーリング

15値配列 `[f64; 15]` — Lv1からLv15:

```rust
const CHAR_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,  // 物理 = None, 元素 = Some(Element::Xxx)
    values: [
        0.4453, 0.4815, 0.5178, 0.5696, 0.6058, 0.6472, 0.7042, 0.7611,
        0.8181, 0.8802, 0.9424, 1.0045, 1.0666, 1.1288, 1.1909,
    ],
};
```

**元素判定ルール:**
- Catalyst通常攻撃: `damage_element: Some(Element::Xxx)` （法器キャラの通常攻撃は必ず元素付与）
- Sword/Claymore/Polearm通常攻撃: `damage_element: None` （物理）
- Bow通常攻撃: `damage_element: None` （物理）
- Bow狙い撃ち(フルチャージ): `damage_element: Some(Element::Xxx)`
- 元素スキル/爆発: `damage_element: Some(Element::Xxx)` （原則元素）

**scaling_stat判定:**
- 大半は `ScalingStat::Atk`
- HP参照系: `ScalingStat::Hp` （Zhongli柱、Barbara回復など）
- 防御参照系: `ScalingStat::Def` （Noelle、Albedo花など）
- EM参照系: `ScalingStat::Em` （一部の天賦）
- Honey Impactのスキル説明で「ATK」「Max HP」「DEF」「EM」を確認

### 突破ステータス (ascension_stat)

Honey Impactの突破ボーナス欄から判定:

| 表示 | Rust |
|------|------|
| CRIT Rate | `AscensionStat::CritRate(0.192)` |
| CRIT DMG | `AscensionStat::CritDmg(0.384)` |
| ATK% | `AscensionStat::Atk(0.240)` |
| HP% | `AscensionStat::Hp(0.288)` |
| DEF% | `AscensionStat::Def(0.240)` |
| Elemental Mastery | `AscensionStat::ElementalMastery(115.2)` |
| Energy Recharge | `AscensionStat::EnergyRecharge(0.320)` |
| Pyro DMG Bonus | `AscensionStat::ElementalDmgBonus(Element::Pyro, 0.288)` |
| Physical DMG Bonus | `AscensionStat::PhysicalDmgBonus(0.300)` |
| Healing Bonus | `AscensionStat::HealingBonus(0.222)` |

**値はLv90突破後の合計値**を使用する。

### 武器 sub_stat

4値配列: `[Lv1, Lv20, Lv80, Lv90]`

```rust
sub_stat: Some(WeaponSubStat::CritRate([0.048, 0.201, 0.201, 0.221])),
sub_stat: Some(WeaponSubStat::ElementalMastery([58.0, 241.0, 241.0, 265.0])),
```

**EM/HPフラット以外はパーセンテージ → 小数変換を忘れずに。**

### 武器精錬値

5値配列 `[R1, R2, R3, R4, R5]`:

```rust
refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
```

### 命の星座パターン

Honey Impactの星座一覧からC3とC5を確認:
- C3が「スキル+3」→ `ConstellationPattern::C3SkillC5Burst`
- C3が「爆発+3」→ `ConstellationPattern::C3BurstC5Skill`

---

## カテゴリ別ワークフロー

### キャラクター追加

1. **WebFetch**: `https://gensh.honeyhunterworld.com/{name}/?lang=EN`
2. **抽出データ**:
   - 基本情報: 名前、元素、武器種、★、地域
   - ベースステータス: base_hp/atk/def (4値)
   - 突破ステ: ascension_stat
   - 通常攻撃: 各段のLv1-15倍率
   - 重撃: Lv1-15倍率
   - 落下攻撃: 落下/低空/高空のLv1-15倍率
   - 元素スキル: 各ヒットのLv1-15倍率
   - 元素爆発: 各ヒットのLv1-15倍率
   - 星座パターン: C3/C5判定
3. **出力先**: `crates/data/src/characters/{element}/{char}.rs`（新規ファイル作成）
4. **mod登録**: `crates/data/src/characters/{element}/mod.rs` の `CHARACTERS` スライスに追加、`mod` 宣言、`pub use` 追加
5. **テスト**: `crates/core/tests/data/characters/{id}.toml`
6. **天賦バフ**: `crates/data/src/talent_buffs/{element}.rs`（A1/A4/星座でステバフがある場合）
7. **月兆**: `crates/data/src/moonsign_chars.rs`（月兆対応キャラの場合）

### 武器追加

1. **WebFetch**: `https://gensh.honeyhunterworld.com/i_n{id}/?lang=EN` またはサイト内検索
2. **抽出データ**:
   - 基本: 名前、武器種、★
   - base_atk (4値)、sub_stat (種類 + 4値)
   - パッシブ名、R1-R5効果値
3. **出力先**: `crates/data/src/weapons/{weapon_type}.rs`
4. **パッシブ分類**: CLAUDE.mdの武器ConditionalBuff分類ルールに従う
5. **登録**: `crates/data/src/weapons/mod.rs` の `ALL_WEAPONS`

### 聖遺物追加

1. **WebFetch**: サイト内検索 or Fandom Wiki
2. **抽出データ**: セット名、2pc効果、4pc効果
3. **出力先**: `crates/data/src/artifacts.rs`
4. **登録**: `ALL_ARTIFACT_SETS`

### 敵追加

1. **WebFetch**: サイト内検索 or Fandom Wiki
2. **抽出データ**: 名前、各元素耐性値
3. **出力先**: `crates/data/src/enemies.rs`
4. **既存テンプレート再利用を優先**: `ALL_10`, `PHYS_50_ELEM_10` など
5. **登録**: `ALL_ENEMIES`

---

## データ取得できない場合のフォールバック

優先順:

1. `gensh.honeyhunterworld.com` (Honey Impact) — プライマリ
2. `genshin-impact.fandom.com` (Fandom Wiki) — セカンダリ
3. `genshin-db-api.vercel.app` (genshin-db API) — 天賦倍率に強い
4. WebSearch `"Genshin {entity} talent scaling"` — 最終手段

## よくあるミス

- [ ] パーセンテージの小数変換忘れ (`88.2%` を `88.2` にしてしまう → 正しくは `0.882`)
- [ ] Catalyst通常攻撃の `damage_element` を `None` にしてしまう → `Some(Element::Xxx)` が正しい
- [ ] base_statの4値で突破前/後を混同する → Lv20は突破**前**、Lv80は突破**前**
- [ ] 天賦スケーリングが13値しかない → 必ず15値(Lv1-15)必要
- [ ] 武器sub_statのEM/HPフラットを小数変換してしまう → EMとHPフラットはそのまま
- [ ] `ALL_CHARACTERS`/`ALL_WEAPONS` への登録忘れ
