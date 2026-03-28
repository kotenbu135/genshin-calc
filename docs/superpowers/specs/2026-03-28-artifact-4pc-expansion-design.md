# P2: 聖遺物4pc効果 全セット実装 設計書 (v3)

## 概要

既存の `ConditionalBuff` システムを活用し、全52聖遺物セットの4pc効果を実装する。
不足する `BuffableStat` variant を core crate に追加し、反応ボーナス・耐性減少も表現可能にする。

## 決定事項

- **スコープ**: 全52セット網羅
- **アプローチ**: 既存パターン踏襲（BuffableStat追加 + ResolvedBuffで返却）
- **耐性減少**: P2に含める（P6の基盤）
- **スタック値**: `stack_values: Option<&'static [f64]>` で非線形対応
- **計算範囲外の効果**: description のみ（CDR、エネルギー回復、確率、フラットダメ加算、変換系）
- **反応ボーナス**: 全て ConditionalBuff(Toggle) — consumer が反応計算時に選択的に activate

## 1. core crate 変更

### 1.1 BuffableStat 新 variant（5つ）

`crates/core/src/buff_types.rs` に追加:

```rust
// 反応ボーナス系
AmplifyingBonus,                  // 蒸発/溶解反応ボーナス → DamageInput.reaction_bonus
TransformativeBonus,              // 過負荷/超電導/拡散/感電/氷砕き/開花系 → TransformativeInput.reaction_bonus
AdditiveBonus,                    // 超激化/草激化 → DamageInput.reaction_bonus

// 敵側耐性減少
ElementalResReduction(Element),   // 元素耐性減少（翠緑4pc, 深林4pc等）
PhysicalResReduction,             // 物理耐性減少（超電導等）
```

処理:
- `apply_buffs_to_profile()` の `is_unconditional()` で除外（ElementalDmgBonus等と同じ扱い）
- `ResolvedBuff` として返却され、consumer が各 Input に適用

**反応ボーナスの消費パターン:**
- Consumer が計算対象の反応を選択（例: Overloaded）
- ResolvedBuff から該当する反応ボーナスを取得（buff name で判別: "cw_transformative", "tf_transformative" 等）
- `TransformativeInput.reaction_bonus` に加算して計算実行
- `TransformativeBonus` 自体は反応種別を持たない（汎用）。反応の適用対象は ConditionalBuff の name/description で判別

**耐性減少の消費パターン:**
- Consumer が攻撃元素と ResolvedBuff の元素を照合
- `Enemy.resistance` から減算してから `calculate_damage` に渡す

### 1.2 AutoCondition 新 variant（2つ）

`crates/data/src/buff.rs` に追加（金メッキの夢用）:

```rust
pub enum AutoCondition {
    // ... 既存 ...
    TeamSameElementCount { min_count: u8 },  // NEW: チーム内同元素キャラ数
    TeamDiffElementCount { min_count: u8 },  // NEW: チーム内異元素キャラ数
}
```

Gilded Dreams 用: 装着者と同元素/異元素のチームメイト数で段階的にバフ。
eval_auto で TeamMember の element を参照して自動判定。

### 1.3 ConditionalBuff に stack_values と target 追加

`crates/data/src/buff.rs`:

```rust
pub struct ConditionalBuff {
    pub name: &'static str,
    pub description: &'static str,
    pub stat: BuffableStat,
    pub value: f64,
    pub refinement_values: Option<[f64; 5]>,
    pub stack_values: Option<&'static [f64]>,  // NEW: 非線形スタック値
    pub target: BuffTarget,                     // NEW: Team or OnlySelf
    pub activation: Activation,
}
```

**stack_values セマンティクス:**
- `stack_values: None` → `value × stack数`（線形、従来互換）
- `stack_values: Some(&[v1, v2, v3])` → 各段階の累積値を直接使用

**target フィールド:**
- 旧貴族・千岩・教官・巻物等のチームバフを表現
- `resolve_conditionals` クロージャから `target` パラメータを削除し、`buff.target` を直接参照

**マイグレーション:** 既存の ConditionalBuff const 定義3箇所（剣闘士、絶縁、炎魔女スタック）に `stack_values: None, target: BuffTarget::OnlySelf` を追加

### 1.4 eval_manual シグネチャ変更

`crates/data/src/team_builder.rs`:

現在:
```rust
fn eval_manual(cond: &ManualCondition, buff_name: &str, value: f64, activations: &[...]) -> Option<f64>
```

変更後:
```rust
fn eval_manual(cond: &ManualCondition, buff: &ConditionalBuff, activations: &[...]) -> Option<f64>
```

stack_values 対応:
```rust
ManualCondition::Stacks(max) => {
    // ... stacks 取得ロジック ...
    if stacks == 0 { return None; }  // ゼロスタックガード
    match buff.stack_values {
        Some(values) => Some(values[stacks.min(values.len()) - 1]),
        None => Some(buff.value * stacks as f64),
    }
}
```

### 1.5 resolve_conditionals の target 対応

`crates/data/src/team_builder.rs` の `resolve_conditionals` クロージャ:

現在（target パラメータをハードコード）:
```rust
let resolve_conditionals = |conditionals: &[ConditionalBuff], target: BuffTarget, ...| { ... };
// 呼び出し: resolve_conditionals(conds, BuffTarget::OnlySelf, ...)
```

変更後（buff.target を使用）:
```rust
let resolve_conditionals = |conditionals: &[ConditionalBuff], ...| {
    for buff in conditionals {
        // ... eval_auto / eval_manual ...
        resolved.push(ResolvedBuff {
            stat: buff.stat,
            value: resolved_value,
            source: ...,
            target: buff.target,  // ← buff 自身の target を使用
        });
    }
};
```

## 2. 全52セット分類

### 2.1 実装済み・変更なし（2セット）

| セット | ID | パターン |
|---|---|---|
| 剣闘士のフィナーレ | gladiators_finale | Auto(WeaponType) → NormalAtkDmgBonus +0.35 |
| 絶縁の旗印 | emblem_of_severed_fate | Auto(StatScaling) → BurstDmgBonus |

### 2.2 実装済み・バフ追加（1セット）

| セット | ID | 追加内容 |
|---|---|---|
| 燃え盛る炎の魔女 | crimson_witch | Toggle: AmplifyingBonus +0.15 ("cw_amplifying"), Toggle: TransformativeBonus +0.40 ("cw_transformative") |

炎魔女の反応ボーナスは常時4pc効果だが、consumer が反応選択時に適用するため ConditionalBuff(Toggle) として定義。
既存のスタック ConditionalBuff（Pyro DMG +7.5%/stack）はそのまま維持。

### 2.3 新規 ConditionalBuff（34セット）

#### Auto 条件のみ（2）

| セット | ID | 条件 | Stat |
|---|---|---|---|
| 大地を流浪する楽団 | wanderers_troupe | WeaponType(Catalyst,Bow) | ChargedAtkDmgBonus +0.35 |
| 年代記 | chronicled_sands_and_water | StatScaling(ER), cap有 | SkillDmgBonus + BurstDmgBonus |

#### Manual(Toggle)（18）

| セット | ID | Stat | 備考 |
|---|---|---|---|
| しめ縄の記憶 | shimenawas_reminiscence | Normal+Charged+Plunging +0.50 | スキル使用後 |
| 沈淪の心 | heart_of_depth | Normal+Charged +0.30 | スキル使用後 |
| 逆飛びの流星 | retracing_bolide | Normal+Charged +0.40 | シールド中 |
| 砂上の楼閣の史話 | desert_pavilion_chronicle | Normal+Charged+Plunging +0.40 | 重撃ヒット後 |
| 未完の遺響 | unfinished_reverie | DmgBonus +0.50 | 燃焼/烈開花後 |
| 夜語りの織 | nighttime_whispers | ElementalDmgBonus(Geo) +0.20 | 夜魂/スキル後 |
| 黒曜の秘典 | obsidian_codex | DmgBonus +0.25 (Toggle①夜魂), CritRate +0.40 (Toggle②低NP) | |
| 勇士の心 | brave_heart | DmgBonus +0.30 | 敵HP50%以上 |
| 狂戦士 | berserker | CritRate +0.24 | 自HP70%以下 |
| 武人 | martial_artist | Normal+Charged +0.25 | スキル/爆発後 |
| 雷を鎮める尊者 | thundersoother | DmgBonus +0.35 | 敵が雷元素影響下 |
| 烈火を渡る賢者 | lavawalker | DmgBonus +0.35 | 敵が炎元素影響下 |
| 血染めの騎士道 | bloodstained_chivalry | ChargedAtkDmgBonus +0.50 | 敵撃破後 |
| 旅人の心 | resolution_of_sojourner | CritRate +0.30 | 重撃時のみ |
| 旧貴族のしつけ | noblesse_oblige | AtkPercent +0.20 (target: Team) | 爆発使用後 |
| 千岩牢固 | tenacity_of_the_millelith | AtkPercent +0.20 + ShieldStrength +0.30 (target: Team) | スキルヒット後 |
| 教官 | instructor | EM +120 (target: Team) | 反応トリガー後 |
| 雷のような怒り | thundering_fury | TransformativeBonus +0.40 ("tf_transformative"), AdditiveBonus +0.20 ("tf_additive") | CDR部分はdesc only |

#### Manual(Stacks)（6）

| セット | ID | Max | Stat | stack_values |
|---|---|---|---|---|
| ブリザードストレイヤー | blizzard_strayer | 2 | CritRate +0.20/stack | None。規約: 1stack=氷元素付着, 2stack=凍結 |
| 華館の夢 | husk_of_opulent_dreams | 4 | DefPercent +0.06/stack, GeoDmg +0.06/stack | None (線形) |
| マレショセハンター | marechaussee_hunter | 3 | CritRate +0.12/stack | None (線形) |
| ニンフの夢 | nymphs_dream | 3 | AtkPercent, HydroDmg | ATK: Some(&[0.04, 0.09, 0.15]), Hydro: Some(&[0.07, 0.16, 0.25]) |
| 調和の断章 | fragment_of_harmonic_whimsy | 3 | DmgBonus +0.18/stack | None (線形) |

#### Stacks + Toggle 複合（1）

| セット | ID | パターン |
|---|---|---|
| 蒼白の炎 | pale_flame | AtkPercent: Stacks(2) +0.09/stack (None), PhysDMG+0.25: Toggle (2stack到達時にactivate) |

#### ベース（無条件 StatBuff）+ Toggle（1）

base 効果を `four_piece.buffs`（無条件 StatBuff）に、条件部分を ConditionalBuff(Toggle) に分離。

| セット | ID | ベース (StatBuff) | 条件付き (Toggle) |
|---|---|---|---|
| 黄金の劇団 | golden_troupe | SkillDmgBonus +0.25 | SkillDmgBonus +0.25 (待機中) |

#### ベース（無条件 StatBuff）+ Stacks（2）

base 効果を `four_piece.buffs`、スタック増分を ConditionalBuff(Stacks) に分離。

| セット | ID | ベース (StatBuff) | スタック |
|---|---|---|---|
| 花海甘露の光 | vourukashas_glow | SkillDmgBonus+BurstDmgBonus +0.10 | SkillDmgBonus+BurstDmgBonus +0.08/stack (Stacks(5), None) |

#### ベース（Toggle）+ Stacks（2）

base 効果が条件付きのため、Toggle と Stacks に分離。

| セット | ID | ベース (Toggle) | スタック |
|---|---|---|---|
| 辰砂往生録 | vermillion_hereafter | AtkPercent +0.08 (爆発後) | AtkPercent +0.10/stack (Stacks(4), None) |
| 花の園の喪失 | flower_of_paradise_lost | TransformativeBonus +0.40 ("fopl_bloom_base") | TransformativeBonus +0.10/stack (Stacks(4), None, "fopl_bloom_stacks") |

#### Auto + Team 条件（2）

| セット | ID | モデル |
|---|---|---|
| 金メッキの夢 | gilded_dreams | 6つの ConditionalBuff: same×3 (Auto TeamSameElementCount min:1/2/3 → AtkPercent +0.14 each), diff×3 (Auto TeamDiffElementCount min:1/2/3 → EM +50 each) |
| 英雄の巻物 | scroll_of_the_hero_of_cinder_city | EM +40 (Toggle, target: Team) + 元素別排他Toggle×7 (ElementalDmgBonus per element +0.12, target: Team) |

#### 耐性減少（2）

| セット | ID | ConditionalBuff |
|---|---|---|
| 翠緑の影 | viridescent_venerer | TransformativeBonus +0.60 ("vv_swirl", Toggle), ElementalResReduction ×4元素 -0.40 (排他Toggle, target: Team) |
| 深林の記憶 | deepwood_memories | ElementalResReduction(Dendro) -0.30 (Toggle, target: Team) |

### 2.4 Description のみ（13セット）

計算システムでモデル化できない4pc効果。description テキストはそのまま維持。

| セット | ID | 理由 |
|---|---|---|
| 海染硨磲 | ocean_hued_clam | 回復→物理ダメ変換 |
| 愛される少女 | maiden_beloved | チーム回復量+20% (ダメージ計算外) |
| 悠久の磐岩 | archaic_petra | 結晶拾得→元素DMGメカニクス |
| 昔日の歌 | song_of_days_past | 回復量→チームDMG変換 |
| 栄華の海 | glory_of_the_ancient_sea | フラットダメージ加算（HP超過×24/1000 + 回復超過×32/1000） |
| 追憶の残響 | echoes_of_an_offering | フラットダメージ加算（ATK×70%、確率36%発動） |
| 亡命者 | exile | エネルギー回復 |
| 博徒 | gambler | CDR on kill |
| 学者 | scholar | エネルギー回復 |
| 奇跡 | tiny_miracle | 自身の元素耐性+30% (ダメージ計算外) |
| 守護の心 | defenders_will | 自身の元素耐性 (ダメージ計算外) |

### 2.5 4pc効果なし（4セット）

祈礼系（prayers_for_illumination, prayers_for_destiny, prayers_for_wisdom, prayers_to_springtime）は1pcセットのため4pc効果なし。

## 3. 翠緑の影 耐性減少モデル

拡散元素が動的なため、元素別に4つの排他的 ConditionalBuff を定義:

```rust
conditional_buffs: &[
    ConditionalBuff {
        name: "vv_swirl_bonus",
        stat: TransformativeBonus,
        value: 0.60,
        target: BuffTarget::OnlySelf,
        activation: Activation::Manual(ManualCondition::Toggle),
        ..
    },
    ConditionalBuff {
        name: "vv_res_shred_pyro",
        stat: ElementalResReduction(Element::Pyro),
        value: 0.40,
        target: BuffTarget::Team,
        activation: Activation::Manual(ManualCondition::Toggle),
        ..
    },
    // vv_res_shred_hydro, vv_res_shred_electro, vv_res_shred_cryo 同様
]
```

ユーザーは `.activate("vv_res_shred_pyro")` 等で拡散元素を指定。

## 4. テスト戦略

### core crate

- 新 BuffableStat 5 variant の serde roundtrip
- `is_unconditional()` / `apply_buffs_to_profile()` で新 variant が除外されることの確認

### data crate

- 全52セットの ConditionalBuff 存在確認（stat/value/activation/target 検証）
- eval_manual テスト: stack_values の線形/非線形/境界値/stacks==0ガード
- eval_manual テスト: ConditionalBuff 全体を渡す新シグネチャ
- eval_auto テスト: TeamSameElementCount/TeamDiffElementCount（金メッキ）
- target: Team の ConditionalBuff が正しくチーム全体に適用されるテスト（旧貴族、千岩、教官）
- 翠緑の排他 Toggle + 耐性減少テスト
- ベース+スタック分離パターンのテスト（辰砂、花海甘露、花の園）
- 炎魔女の反応ボーナス Toggle テスト
- description-only セット（13）の conditional_buffs が空であることの確認
- データ整合性テスト（全セット走査）

### 許容誤差

既存慣例通り `< 0.01` で浮動小数点比較。

## 5. 影響範囲

| ファイル | 変更内容 |
|---|---|
| `crates/core/src/buff_types.rs` | BuffableStat +5 variant, `is_unconditional()` 更新 |
| `crates/core/src/team.rs` | `apply_buffs_to_profile` で新 variant 除外 |
| `crates/data/src/buff.rs` | ConditionalBuff +stack_values +target, AutoCondition +TeamSameElementCount +TeamDiffElementCount |
| `crates/data/src/team_builder.rs` | eval_manual シグネチャ変更 + stack_values 対応 + resolve_conditionals の target 対応 |
| `crates/data/src/artifacts.rs` | 全52セットの4pc効果実装 |

### 既存 API への影響

- ConditionalBuff にフィールド追加（既存は `stack_values: None, target: BuffTarget::OnlySelf` で互換）
- BuffableStat に variant 追加（match 文に分岐追加が必要）
- AutoCondition に variant 追加（eval_auto に分岐追加が必要）
- eval_manual シグネチャ変更（内部関数のため外部 API 影響なし）
- resolve_conditionals の target パラメータ削除（内部クロージャのため外部 API 影響なし）
- Stats / DamageInput / StatProfile は変更なし

## 6. セット数サマリ

| 分類 | 数 |
|---|---|
| 実装済み（変更なし） | 2 |
| 実装済み（バフ追加） | 1 |
| Auto条件 | 2 |
| Toggle | 18 |
| Stacks | 5 |
| Stacks+Toggle複合 | 1 |
| ベース(StatBuff)+Toggle | 1 |
| ベース(StatBuff)+Stacks | 1 |
| ベース(Toggle)+Stacks | 2 |
| Auto+Team条件 | 2 |
| 耐性減少 | 2 |
| **ConditionalBuff 小計** | **37** |
| Description のみ | 11 |
| 4pc効果なし | 4 |
| **合計** | **52** |

※ 実装済み3セット含め、ConditionalBuff 作業対象は 1(CW追加) + 34(新規) = 35セット。
