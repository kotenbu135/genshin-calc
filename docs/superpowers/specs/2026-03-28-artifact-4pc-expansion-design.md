# P2: 聖遺物4pc効果 全セット実装 設計書

## 概要

既存の `ConditionalBuff` システムを活用し、全52聖遺物セットの4pc効果を実装する。
不足する `BuffableStat` variant を core crate に追加し、反応ボーナス・耐性減少も表現可能にする。

## 決定事項

- **スコープ**: 全52セット網羅
- **アプローチ**: 既存パターン踏襲（BuffableStat追加 + ResolvedBuffで返却）
- **耐性減少**: P2に含める（P6の基盤）
- **スタック値**: `stack_values: Option<&'static [f64]>` で非線形対応
- **計算範囲外の効果**: description のみ（CDR、エネルギー回復、確率、変換系）

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
- `apply_buffs_to_profile()` では除外（ElementalDmgBonus等と同じ扱い）
- `ResolvedBuff` として返却され、consumer が各 Input に適用

### 1.2 ConditionalBuff に stack_values 追加

`crates/data/src/buff.rs`:

```rust
pub struct ConditionalBuff {
    pub name: &'static str,
    pub description: &'static str,
    pub stat: BuffableStat,
    pub value: f64,
    pub refinement_values: Option<[f64; 5]>,
    pub stack_values: Option<&'static [f64]>,  // NEW
    pub activation: Activation,
}
```

セマンティクス:
- `stack_values: None` → `value × stack数`（線形、従来互換）
- `stack_values: Some(&[v1, v2, v3])` → 各段階の累積値を直接使用

### 1.3 eval_manual 変更

`crates/data/src/team_builder.rs`:

```rust
// stack_values 対応
match buff.stack_values {
    Some(values) => values[stacks - 1],  // 累積値を直接参照
    None => buff.value * stacks as f64,  // 従来の線形計算
}
```

## 2. 全52セット分類

### 2.1 実装済み（3セット）

| セット | ID | パターン |
|---|---|---|
| 剣闘士のフィナーレ | gladiators_finale | Auto(WeaponType) → NormalAtkDmgBonus |
| 絶縁の旗印 | emblem_of_severed_fate | Auto(StatScaling) → BurstDmgBonus |
| 燃え盛る炎の魔女 | crimson_witch | Manual(Stacks) → ElementalDmgBonus(Pyro) |

### 2.2 ConditionalBuff 化するセット（38セット）

#### Auto 条件のみ（3）

| セット | ID | 条件 | Stat |
|---|---|---|---|
| 大地を流浪する楽団 | wanderers_troupe | WeaponType(Catalyst,Bow) | ChargedAtkDmgBonus +0.35 |
| 栄華の海 | glory_of_the_ancient_sea | StatScaling(HP), cap有 | NormalAtkDmgBonus + ChargedAtkDmgBonus |
| 年代記 | chronicled_sands_and_water | StatScaling(ER), cap有 | SkillDmgBonus + BurstDmgBonus |

#### Manual(Toggle)（16）

| セット | ID | Stat | 備考 |
|---|---|---|---|
| しめ縄の記憶 | shimenawas_reminiscence | Normal+Charged+Plunging +0.50 | スキル使用後 |
| 沈淪の心 | heart_of_depth | Normal+Charged +0.30 | スキル使用後 |
| 逆飛びの流星 | retracing_bolide | Normal+Charged +0.40 | シールド中 |
| 黄金の劇団 | golden_troupe | SkillDmgBonus +0.25 / +0.25 | フィールド/待機 |
| 砂上の楼閣の史話 | desert_pavilion_chronicle | Normal+Charged+Plunging +0.40 | 重撃ヒット後 |
| 未完の遺響 | unfinished_reverie | DmgBonus +0.50 | 燃焼/烈開花後 |
| 夜語りの織 | nighttime_whispers_in_the_echoing_woods | ElementalDmgBonus(Geo) +0.20 | 夜魂/スキル後 |
| 黒曜の秘典 | obsidian_codex | CritRate +0.40 / CritDmg (ER依存) | 夜魂状態 |
| 勇士の心 | brave_heart | DmgBonus +0.30 | 敵HP50%以上 |
| 狂戦士 | berserker | CritRate +0.24 | 自HP70%以下 |
| 武人 | martial_artist | Normal+Charged +0.25 | スキル/爆発後 |
| 旧貴族のしつけ | noblesse_oblige | AtkPercent +0.20 (Team) | 爆発使用後 |
| 千岩牢固 | tenacity_of_the_millelith | AtkPercent +0.20 + ShieldStrength +0.30 (Team) | スキルヒット後 |
| 教官 | instructor | EM +120 (Team) | 反応トリガー後 |
| 雷のような怒り | thundering_fury | TransformativeBonus +0.40 | CDR部分はdesc |
| 炎魔女（反応部分） | crimson_witch (追加) | AmplifyingBonus +0.15, TransformativeBonus +0.40 | 既存スタックに追加 |

#### Manual(Stacks)（11）

| セット | ID | Max | Stat | stack_values |
|---|---|---|---|---|
| ブリザードストレイヤー | blizzard_strayer | 2 | CritRate | None (0.20/stack) |
| 蒼白の炎 | pale_flame | 2 | AtkPercent +0.09/stack, 2stack時PhysDMG+0.25 | AtkPercent: None(0.09), PhysDMG: Toggle |
| 華館の夢 | husk_of_opulent_dreams | 4 | DefPercent +0.06/stack, GeoDmg +0.06/stack | None (線形) |
| 辰砂往生録 | vermillion_hereafter | 4 | AtkPercent base+0.08, stack+0.10 | Some(&[0.18, 0.28, 0.38, 0.48]) |
| マレショセハンター | marechaussee_hunter | 3 | CritRate +0.12/stack | None (線形) |
| ニンフの夢 | nymphs_dream | 3 | AtkPercent | Some(&[0.08, 0.16, 0.25]) |
| ニンフの夢 | nymphs_dream | 3 | HydroDmg | Some(&[0.04, 0.09, 0.15]) |
| 花海甘露の光 | vourukashas_glow | 5 | DmgBonus base+0.10, stack+0.08 | Some(&[0.18, 0.26, 0.34, 0.42, 0.50]) |
| 調和の断章 | fragment_of_harmonic_whimsy | 3 | DmgBonus +0.18/stack | None (線形) |
| 花の園の喪失 | flower_of_paradise_lost | 4 | TransformativeBonus base+0.40, +0.10/stack | Some(&[0.50, 0.60, 0.70, 0.80]) |
| 追憶の残響 | echoes_of_an_offering | — | NormalAtkDmgBonus +0.70 | Toggle（確率効果を最大値Toggle） |
| 旅人の心 | resolution_of_sojourner | — | CritRate +0.30 | Toggle（重撃時） |

#### Both/Team 条件（3）

| セット | ID | 条件 | Stat |
|---|---|---|---|
| 金メッキの夢 | gilded_dreams | TeamElementCount | 同元素: AtkPercent +0.14/char, 異元素: EM +50/char |
| 英雄の巻物 | scroll_of_the_hero_of_cinder_city | Toggle + TeamElement | 反応後 EM+40(Team), 関連元素DMG+12%(Team) |
| 守護の心 | defenders_will | — | desc only（元素RES、計算外） |

#### 耐性減少（2）

| セット | ID | Stat |
|---|---|---|
| 翠緑の影 | viridescent_venerer | TransformativeBonus(Swirl)+0.60, ElementalResReduction×4元素 -0.40 (排他Toggle) |
| 深林の記憶 | deepwood_memories | ElementalResReduction(Dendro) -0.30 (Toggle) |

### 2.3 Description のみ（8セット）

| セット | ID | 理由 |
|---|---|---|
| 海染硨磲 | ocean_hued_clam | 回復→物理ダメ変換 |
| 愛される少女 | maiden_beloved | チーム回復量+20% |
| 悠久の磐岩 | archaic_petra | 結晶拾得→元素DMGメカニクス |
| 亡命者 | exile | エネルギー回復 |
| 博徒 | gambler | CDR on kill |
| 学者 | scholar | エネルギー回復 |
| 昔日の歌 | song_of_days_past | 回復量→チームDMG変換 |
| 守護の心 | defenders_will | 元素RES（計算外） |

### 2.4 4pc効果なし（4セット）

祈礼系（prayers_for_illumination, prayers_for_destiny, prayers_for_wisdom, prayers_to_springtime）は1pcセットのため4pc効果なし。

## 3. 翠緑の影 耐性減少モデル

拡散元素が動的なため、元素別に4つの排他的 ConditionalBuff を定義:

```rust
// conditional_buffs: &[
//   { name: "vv_swirl_bonus", stat: TransformativeBonus, value: 0.60, Toggle },
//   { name: "vv_res_shred_pyro", stat: ElementalResReduction(Pyro), value: 0.40, Toggle },
//   { name: "vv_res_shred_hydro", stat: ElementalResReduction(Hydro), value: 0.40, Toggle },
//   { name: "vv_res_shred_electro", stat: ElementalResReduction(Electro), value: 0.40, Toggle },
//   { name: "vv_res_shred_cryo", stat: ElementalResReduction(Cryo), value: 0.40, Toggle },
// ]
```

ユーザーは `.activate("vv_res_shred_pyro")` 等で拡散元素を指定。

## 4. テスト戦略

### core crate

- 新 BuffableStat 5 variant の serde roundtrip
- `apply_buffs_to_profile()` で新 variant が除外されることの確認
- `stack_values` 解決ロジック（線形/非線形/境界値）

### data crate

- 全52セットの ConditionalBuff 存在確認（stat/value/activation 検証）
- eval_auto/eval_manual テスト（新パターン含む）
- stack_values 統合テスト（ニンフ等の非線形スタック）
- 翠緑の排他 Toggle テスト
- データ整合性テスト（全セット走査）
- description-only セットの conditional_buffs が空であることの確認

### 許容誤差

既存慣例通り `< 0.01` で浮動小数点比較。

## 5. 影響範囲

| ファイル | 変更内容 |
|---|---|
| `crates/core/src/buff_types.rs` | BuffableStat +5 variant |
| `crates/core/src/team.rs` | apply_buffs_to_profile で新 variant 除外 |
| `crates/data/src/buff.rs` | ConditionalBuff に stack_values フィールド追加 |
| `crates/data/src/team_builder.rs` | eval_manual の stack_values 対応 |
| `crates/data/src/artifacts.rs` | 全52セットの4pc ConditionalBuff 実装 |

### 既存 API への影響

- ConditionalBuff にフィールド追加（既存は `stack_values: None` で互換）
- BuffableStat に variant 追加（match 文に分岐追加が必要）
- Stats / DamageInput / StatProfile は変更なし
