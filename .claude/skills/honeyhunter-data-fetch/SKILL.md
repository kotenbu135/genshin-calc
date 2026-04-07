---
name: honeyhunter-data-fetch
description: Fetch Genshin Impact game data (characters, weapons, enemies) from local Honey Hunter mirror and convert to Rust const definitions
version: 2.0.0
source: local-mirror
---

# Honey Impact データ取得スキル

ローカルミラーのMarkdownファイルからゲームの数値データ（キャラ・武器・敵）を読み取り、
本プロジェクトの Rust const 定義に変換するワークフロー。

## Trigger

- `docs/v6.0-6.3-data-update-todo.md` やバージョン別TODOに記載のデータを実装するとき
- ユーザーが「Honeyhunterからデータ取得して」「キャラデータ追加して」と指示したとき
- `/game-update-todo` で生成されたTODOリストの実装時

## データソース

**プライマリ**: ローカルミラー `honeyhunter-mirror/md/`

| カテゴリ | パス | 例 |
|----------|------|-----|
| キャラクター | `honeyhunter-mirror/md/characters/{name}_{id}.md` | `citlali_107.md` |
| 武器 | `honeyhunter-mirror/md/weapons/i_n{id}.md` | `i_n11501.md` |

**ミラー更新手順** (新バージョン追加時):

```bash
# 1. HTMLミラー取得（未取得ページのみダウンロード、再開対応）
./scripts/honeyhunter-mirror.sh characters   # or weapons / all

# 2. HTML → Markdown変換
python3 scripts/honeyhunter-to-md.py characters   # or weapons / all
```

**フォールバック** (ミラーにないデータの場合):

1. `genshin-impact.fandom.com` (Fandom Wiki) — WebFetchで取得
2. WebSearch `"Genshin {entity} talent scaling"` — 最終手段

---

## ワークフロー

### Step 1: ローカルMDファイルを読む

Readツールで該当MDファイルを開く。キャラ名からファイル名を特定するには:

```bash
# キャラ名で検索
ls honeyhunter-mirror/md/characters/ | grep -i citlali
# → citlali_107.md

# 武器名で検索（MDファイル内のタイトルで検索）
grep -rl "Peak Patrol Song" honeyhunter-mirror/md/weapons/
# → i_n11516.md
```

### Step 2: データ抽出・変換

MDファイルから以下のセクションを読み取り、フォーマット規約に従って変換する。

MDファイルの構造:

**キャラ:**
- `## Basic Info` — 名前、レアリティ、武器種、元素、星座名
- `## Base Stats` — レベル別HP/ATK/DEF/CritRate/CritDMG/突破ボーナス
- `## Normal Attack: {name}` — 通常攻撃説明 + 倍率テーブル(Lv1-15)
- `## Elemental Skill: {name}` — スキル説明 + 倍率テーブル(Lv1-15)
- `## Elemental Burst: {name}` — 爆発説明 + 倍率テーブル(Lv1-15)
- `## Passive Talents` — パッシブ天賦一覧
- `## Constellations` — C1-C6の効果テキスト

**武器:**
- `## Basic Info` — 名前、レアリティ、武器種、基礎攻撃力、サブステ
- `## Weapon Affix` — R1-R5精錬効果テキスト
- `## Stats` — レベル別ATK/サブステ

---

## フォーマット規約

### パーセンテージ → 小数

MDファイルの `%` 値はすべて小数に変換する:
- `10.8%` → `0.108`
- `88.2%` → `0.882`
- `33.7%` → `0.337`

### base_hp / base_atk / base_def

4値配列: `[Lv1, Lv20, Lv80, Lv90]`

MDの `## Base Stats` テーブルから以下を抽出:
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

MDの倍率テーブルから直接読み取る。テーブルのカラムはLv1-Lv15。

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
- MDのスキル説明テキストで「ATK」「Max HP」「DEF」「EM」を確認

### 突破ステータス (ascension_stat)

MDの `## Base Stats` テーブルの `Bonus` 列から判定。列ヘッダーに `Bonus EM`, `Bonus CR%` 等の種類が記載される:

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

**値はLv90突破後の合計値**を使用する（テーブル末尾の行）。

### 武器 sub_stat

4値配列: `[Lv1, Lv20, Lv80, Lv90]`

```rust
sub_stat: Some(WeaponSubStat::CritRate([0.048, 0.201, 0.201, 0.221])),
sub_stat: Some(WeaponSubStat::ElementalMastery([58.0, 241.0, 241.0, 265.0])),
```

**EM/HPフラット以外はパーセンテージ → 小数変換を忘れずに。**

### 武器精錬値

MDの `## Weapon Affix` セクションにR1-R5の効果テキストが記載される。
数値部分を抽出して5値配列にする:

```rust
refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
```

### 命の星座パターン

MDの `## Constellations` セクションのC3とC5を確認:
- C3が「スキル+3」→ `ConstellationPattern::C3SkillC5Burst`
- C3が「爆発+3」→ `ConstellationPattern::C3BurstC5Skill`

---

## カテゴリ別ワークフロー

### キャラクター追加

1. **Read**: `honeyhunter-mirror/md/characters/{name}_{id}.md`
2. **抽出データ**:
   - 基本情報: 名前、元素、武器種、★、星座名
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

1. **Read**: `honeyhunter-mirror/md/weapons/i_n{id}.md`
2. **抽出データ**:
   - 基本: 名前、武器種、★
   - base_atk (4値)、sub_stat (種類 + 4値)
   - パッシブ名、R1-R5効果値
3. **出力先**: `crates/data/src/weapons/{weapon_type}.rs`
4. **パッシブ分類**: CLAUDE.mdの武器ConditionalBuff分類ルールに従う
5. **登録**: `crates/data/src/weapons/mod.rs` の `ALL_WEAPONS`

### 聖遺物追加

1. ミラーに聖遺物データはないため、Fandom Wikiから取得
2. **抽出データ**: セット名、2pc効果、4pc効果
3. **出力先**: `crates/data/src/artifacts.rs`
4. **登録**: `ALL_ARTIFACT_SETS`

### 敵追加

1. ミラーに敵データは少数のため、Fandom Wikiから補完
2. **抽出データ**: 名前、各元素耐性値
3. **出力先**: `crates/data/src/enemies.rs`
4. **既存テンプレート再利用を優先**: `ALL_10`, `PHYS_50_ELEM_10` など
5. **登録**: `ALL_ENEMIES`

---

## よくあるミス

- [ ] パーセンテージの小数変換忘れ (`88.2%` を `88.2` にしてしまう → 正しくは `0.882`)
- [ ] Catalyst通常攻撃の `damage_element` を `None` にしてしまう → `Some(Element::Xxx)` が正しい
- [ ] base_statの4値で突破前/後を混同する → Lv20は突破**前**、Lv80は突破**前**
- [ ] 天賦スケーリングが13値しかない → 必ず15値(Lv1-15)必要
- [ ] 武器sub_statのEM/HPフラットを小数変換してしまう → EMとHPフラットはそのまま
- [ ] `ALL_CHARACTERS`/`ALL_WEAPONS` への登録忘れ
