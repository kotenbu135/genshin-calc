# Shared: Data Pipeline Reference

全データ追加スキル共通のドメイン知識・フォーマット規約・ファイル配置。
各スキルはこのファイルを参照し、固有ロジックのみ記述する。

---

## データソース

**プライマリ**: ローカルミラー `honeyhunter-mirror/md/`

| カテゴリ | パス | 例 |
|----------|------|-----|
| キャラクター | `honeyhunter-mirror/md/characters/{name}_{id}.md` | `citlali_107.md` |
| 武器 | `honeyhunter-mirror/md/weapons/i_n{id}.md` | `i_n11501.md` |

```bash
# キャラ名で検索
ls honeyhunter-mirror/md/characters/ | grep -i citlali
# 武器名で検索
grep -rl "Peak Patrol Song" honeyhunter-mirror/md/weapons/
```

**ミラー更新**:
```bash
./scripts/honeyhunter-mirror.sh characters   # or weapons / all
python3 scripts/honeyhunter-to-md.py characters   # or weapons / all
```

**フォールバック** (ミラーにない場合): `genshin-impact.fandom.com` → WebSearch

---

## ファイル配置マップ

| 種類 | パス |
|------|------|
| キャラデータ | `crates/data/src/characters/{element}/{char_id}.rs` |
| キャラmod | `crates/data/src/characters/{element}/mod.rs` |
| TOML テスト | `crates/core/tests/data/characters/{char_id}.toml` |
| 天賦バフ | `crates/data/src/talent_buffs/{element}.rs` |
| 月兆 | `crates/data/src/moonsign_chars.rs` |
| Sword | `crates/data/src/weapons/sword.rs` |
| Claymore | `crates/data/src/weapons/claymore.rs` |
| Polearm | `crates/data/src/weapons/polearm.rs` |
| Bow | `crates/data/src/weapons/bow.rs` |
| Catalyst | `crates/data/src/weapons/catalyst.rs` |
| 武器mod | `crates/data/src/weapons/mod.rs` → `ALL_WEAPONS` |
| 聖遺物 | `crates/data/src/artifacts.rs` → `ALL_ARTIFACT_SETS` |
| 敵 | `crates/data/src/enemies.rs` → `ALL_ENEMIES` |

---

## フォーマット規約

### パーセンテージ → 小数
`10.8%` → `0.108` / `88.2%` → `0.882`
**例外: EM・HPフラットは変換しない** (58.0, 241.0 等そのまま)

### base_hp / base_atk / base_def
4値配列 `[Lv1, Lv20, Lv80, Lv90]` — Lv20/Lv80は**突破前**の値。

### 天賦スケーリング
`[f64; 15]` — Lv1からLv15まで必ず15値。13値は不可。

### 武器精錬値
`[R1, R2, R3, R4, R5]` — 必ず5値。

### 突破ステータス

| MD表示 | Rust | 値例 |
|--------|------|------|
| CRIT Rate | `AscensionStat::CritRate(v)` | 0.192 |
| CRIT DMG | `AscensionStat::CritDmg(v)` | 0.384 |
| ATK% | `AscensionStat::Atk(v)` | 0.240 |
| HP% | `AscensionStat::Hp(v)` | 0.288 |
| DEF% | `AscensionStat::Def(v)` | 0.240 |
| Elemental Mastery | `AscensionStat::ElementalMastery(v)` | 115.2 |
| Energy Recharge | `AscensionStat::EnergyRecharge(v)` | 0.320 |
| {Elem} DMG Bonus | `AscensionStat::ElementalDmgBonus(Element::Xxx, v)` | 0.288 |
| Physical DMG Bonus | `AscensionStat::PhysicalDmgBonus(v)` | 0.300 |
| Healing Bonus | `AscensionStat::HealingBonus(v)` | 0.222 |

値はLv90突破後の合計値。

### 武器 sub_stat 種類

| MD表示 | Rust |
|--------|------|
| ATK% | `WeaponSubStat::AtkPercent([...])` |
| HP% | `WeaponSubStat::HpPercent([...])` |
| DEF% | `WeaponSubStat::DefPercent([...])` |
| CRIT Rate | `WeaponSubStat::CritRate([...])` |
| CRIT DMG | `WeaponSubStat::CritDmg([...])` |
| Elemental Mastery | `WeaponSubStat::ElementalMastery([...])` |
| Energy Recharge | `WeaponSubStat::EnergyRecharge([...])` |
| Physical DMG Bonus | `WeaponSubStat::PhysicalDmgBonus([...])` |

---

## 元素判定ルール

| 武器種 | 通常攻撃 | 重撃 | スキル/爆発 |
|--------|---------|------|------------|
| Catalyst | `Some(Element)` | `Some(Element)` | `Some(Element)` |
| Sword/Claymore/Polearm | `None` (物理) | `None` (物理) | `Some(Element)` |
| Bow | `None` (物理) | フルチャージ: `Some(Element)` | `Some(Element)` |

### scaling_stat判定
- 大半: `ScalingStat::Atk`
- HP参照系: `ScalingStat::Hp` (Zhongli柱等)
- DEF参照系: `ScalingStat::Def` (Noelle/Albedo等)
- EM参照系: `ScalingStat::Em`
- MD説明テキストの「ATK」「Max HP」「DEF」「EM」で確認

---

## ConditionalBuff 分類

### 武器パッシブ

| パッシブ内容 | 分類 | 実装 |
|-------------|------|------|
| ATK/DMG/CRIT等 (常時) | STATIC | `buffs: &[StatBuff]` |
| ATK/DMG/CRIT等 (条件付き) | CONDITIONAL | `conditional_buffs: &[ConditionalBuff]` |
| proc damage / エネルギー生成 / CDリセット / HP回復 / ユーティリティ | SKIP | `conditional_buffs: &[]` |

**StatBuff/ConditionalBuff重複時**: StatBuffをconditional_buffsに移動、`buffs: &[]`に（二重計上防止）

### Activation パターン

| 条件テキスト | Activation |
|-------------|------------|
| HP閾値/敵近接/スキル使用後 | `Manual(Toggle)` |
| Nスタック/N層 | `Manual(Stacks(N))` |
| EM/DEFのN%分 | `Auto(StatScaling { stat, offset, cap })` |
| チーム異元素数 | `Auto(TeamDiffElementCount { min_count })` |
| チーム同元素N人以上 | `Auto(TeamSameElementCount { min_count })` |
| チームエネルギー合計 | `Manual(Toggle)` (StatScaling不可) |
| ステ依存+トグル | `Both(AutoCondition, ManualCondition)` |

### 天賦バフ判定

| パッシブ内容 | 実装? |
|-------------|-------|
| ATK/DMG/CRIT等ステバフ | YES — TalentBuffDef |
| 元素耐性ダウン | YES — TalentBuffDef |
| TransformativeBonus | YES — `BuffableStat::TransformativeBonus` |
| proc damage/HP回復/シールド/CD短縮 | NO |

### 命名規約
`{weapon_or_artifact_id}_{effect}` / `{char_id}_{effect}` — グローバルユニーク必須

---

## 星座パターン

MD `## Constellations` のC3/C5を確認:
- C3が「スキル+3」→ `ConstellationPattern::C3SkillC5Burst`
- C3が「爆発+3」→ `ConstellationPattern::C3BurstC5Skill`

---

## Common Pitfalls

| ミス | 正しい対処 |
|------|-----------|
| パーセンテージ小数変換忘れ | `88.2%` → `0.882` |
| EM/HPフラットを小数変換 | EMとHPフラットはそのまま |
| Catalyst通常攻撃 `damage_element: None` | `Some(Element::Xxx)` が正しい |
| base_stat突破前/後混同 | Lv20=突破前, Lv80=突破前 |
| 天賦スケーリング13値 | 必ず15値 (Lv1-15) |
| mod.rs登録忘れ | CHARACTERS/ALL_WEAPONS に追加必須 |
| 天賦バフ名の重複 | `{id}_{effect}` でユニーク化 |
| TOML expectedの手計算 | dummy → actual パターンで自動算出 |
| StatBuff/ConditionalBuff二重計上 | 条件付きならStatBuff→ConditionalBuffに移動 |
| 精錬値が4値 | 必ず5値 [R1-R5] |

---

## Rust テンプレート

### キャラクター定義

```rust
const CHAR_NA_HIT1: TalentScaling = TalentScaling {
    name: "1段ダメージ",
    scaling_stat: ScalingStat::Atk,
    damage_element: None,  // 物理=None, 元素=Some(Element::Xxx)
    values: [0.445, 0.481, /* ... 15値 */ 1.191],
};

pub const CHARACTER_NAME: CharacterData = CharacterData {
    id: "character_id",
    name: "キャラ名",
    element: Element::Pyro,
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    region: Region::Natlan,
    base_hp: [1000.0, 2500.0, 9000.0, 10000.0],
    base_atk: [25.0, 250.0, 280.0, 300.0],
    base_def: [60.0, 600.0, 700.0, 750.0],
    ascension_stat: AscensionStat::CritDmg(0.384),
    talents: TalentSet { /* ... */ },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
```

### 武器定義

```rust
pub const WEAPON_NAME: WeaponData = WeaponData {
    id: "weapon_id",
    name: "武器名",
    weapon_type: WeaponType::Sword,
    rarity: Rarity::Star5,
    base_atk: [48.0, 590.0, 621.0, 674.0],
    sub_stat: Some(WeaponSubStat::CritRate([0.048, 0.201, 0.201, 0.221])),
    passive: Some(WeaponPassive {
        name: "パッシブ名",
        effect: PassiveEffect {
            description: "効果説明",
            buffs: &[StatBuff { stat: BuffableStat::AtkPercent, value: 0.20,
                      refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]) }],
            conditional_buffs: &[],
        },
    }),
};
```

### ConditionalBuff テンプレート

```rust
// Toggle
ConditionalBuff {
    name: "{id}_{effect}", description: "条件: ...",
    stat: BuffableStat::AtkPercent, value: 0.20,
    refinement_values: Some([0.20, 0.25, 0.30, 0.35, 0.40]),
    stack_values: None, target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}
// Stacks (非線形)
ConditionalBuff {
    name: "{id}_{effect}_stacks", description: "スタック: ...",
    stat: BuffableStat::AtkPercent, value: 0.10,
    refinement_values: None,
    stack_values: Some(&[0.10, 0.20, 0.30, 0.48]),
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Stacks(4)),
}
// StatScaling
ConditionalBuff {
    name: "{id}_{effect}_scaling", description: "EM基づく: ...",
    stat: BuffableStat::ChargedAtkFlatDmg, value: 1.60,
    refinement_values: Some([1.60, 2.00, 2.40, 2.80, 3.20]),
    stack_values: None, target: BuffTarget::OnlySelf,
    activation: Activation::Auto(AutoCondition::StatScaling {
        stat: BuffableStat::ElementalMastery, offset: None, cap: None,
    }),
}
```

### TOML テストケース

```toml
[character]
name = "CharacterName"
element = "Pyro"

[[cases]]
type = "normal"
name = "Normal Attack Hit 1 — no reaction"
character_level = 90
talent_multiplier = 0.4452
scaling_stat = "Atk"
damage_type = "Normal"
element = "Pyro"

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
non_crit = 1234.56
crit = 2469.12
average = 1851.84
```
