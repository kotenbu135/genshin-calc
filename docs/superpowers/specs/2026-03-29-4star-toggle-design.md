# P3: 4-Star Weapon ConditionalBuff Design

**Date**: 2026-03-29
**Status**: Approved
**Depends on**: P0 (ConditionalBuff system)
**Scope**: 28 four-star weapons with ConditionalBuff (data-only, no core changes)

## Overview

Add ConditionalBuff definitions to 28 four-star weapons. Most use `ManualCondition::Toggle`, with exceptions using `Stacks` or `Auto(StatScaling)`. All use existing framework patterns. No core crate changes required.

## Patterns

### Pattern A: Toggle (single buff) — 17 weapons

```rust
ConditionalBuff {
    name: "<weapon_id>_<stat>",
    description: "<Japanese description>",
    stat: BuffableStat::Xxx,
    value: <r1_value>,
    refinement_values: Some([R1, R2, R3, R4, R5]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Manual(ManualCondition::Toggle),
}
```

### Pattern B: Toggle (multiple buffs) — 7 weapons

Same as Pattern A but with 2-3 ConditionalBuff entries per weapon.

### Pattern C: Auto(StatScaling) — 2 weapons

DEF-scaling flat damage. Auto-computes DEF × multiplier. No Toggle needed because the target stat (e.g. `SkillFlatDmg`) is only applied during the relevant damage calculation.

```rust
ConditionalBuff {
    name: "<weapon_id>_def_flat",
    description: "<Japanese description>",
    stat: BuffableStat::SkillFlatDmg, // or appropriate FlatDmg variant
    value: <r1_multiplier>,
    refinement_values: Some([R1, R2, R3, R4, R5]),
    stack_values: None,
    target: BuffTarget::OnlySelf,
    activation: Activation::Auto(AutoCondition::StatScaling {
        stat: BuffableStat::DefPercent,
        offset: None,
        cap: None,
    }),
}
```

Note: `Both(StatScaling, Toggle)` was considered but the current `Both` implementation discards the auto-computed value (`and_then(|_| ...)`), returning only `buff.value`. `Auto(StatScaling)` correctly computes DEF × multiplier. Same pattern as Staff of Homa's always-on HP scaling buff.

### Pattern D: Stacks — 1 weapon

Non-linear multi-state effect using `ManualCondition::Stacks`.

```rust
// Fading Twilight: non-linear stacks (different value per state)
ConditionalBuff {
    activation: Activation::Manual(ManualCondition::Stacks(3)),
    stack_values: Some(&[state1, state2, state3]), // R1 values only
    refinement_values: None, // stack_values are R1-fixed (same as Nymph's Dream pattern)
    ..
}
```

Note on `stack_values` + `refinement_values` interaction: The existing codebase (Nymph's Dream) uses `stack_values: Some(...)` with `refinement_values: None` because artifacts don't have refinement. For weapons with refinement AND non-linear stacks, `stack_values` stores R1 values only, and `refinement_values` is set to `None`. If precise per-refinement non-linear values are needed, a core change would be required (out of scope). For Fading Twilight, R1 values are sufficient since it's an event weapon (R5 fixed).

### Pattern E: StatBuff + Toggle hybrid — 1 weapon

Hamayumi: unconditional base effect in `buffs`, energy-full bonus in `conditional_buffs`.

## 28 Weapon Definitions

### Swords (9)

**1. Cinnabar Spindle** — DEF×倍率をスキルフラットダメージに加算 [Pattern C]
```
name: "cinnabar_skill_def_flat"
stat: SkillFlatDmg
activation: Auto(StatScaling{DefPercent, offset: None, cap: None})
R1-R5: [0.40, 0.50, 0.60, 0.70, 0.80]
```

**2. Festering Desire** — スキル会心率アップ [Pattern A]
```
name: "festering_skill_cr", stat: CritRate, R1-R5: [0.06, 0.075, 0.09, 0.105, 0.12]
```
Note: SkillDmgBonus is already implemented as unconditional StatBuff. Only CritRate is new.

**3. Flute of Ezpitzal** — DEF×倍率をフラットダメージに加算 [Pattern C]
```
name: "flute_ezpitzal_def_flat"
stat: NormalAtkFlatDmg (TBV — verify which attack types are buffed)
activation: Auto(StatScaling{DefPercent, offset: None, cap: None})
R1-R5: TBV
```

**4. Lion's Roar** — 炎/雷影響下の敵へDMGアップ [Pattern A]
```
name: "lions_roar_dmg", stat: DmgBonus, R1-R5: [0.20, 0.24, 0.28, 0.32, 0.36]
```

**5. Prized Isshin Blade** — スキル命中後ATK%アップ [Pattern A]
```
name: "prized_isshin_atk", stat: AtkPercent, R1-R5: TBV
```

**6. Royal Longsword** — 会心率スタック（会心リセット、最大5スタックToggle近似） [Pattern A]
```
name: "royal_longsword_cr", stat: CritRate, R1-R5: [0.40, 0.50, 0.60, 0.70, 0.80]
```
Note: 1スタック=8%(R1)、最大5スタック=40%(R1)。Toggle値は最大5スタック合計値。

**7. Serenity's Call** — スキル後DMGアップ [Pattern A]
```
name: "serenitys_call_dmg", stat: DmgBonus, R1-R5: TBV
```

**8. Sturdy Bone** — ダッシュ後通常攻撃DMGアップ [Pattern A]
```
name: "sturdy_bone_na_dmg", stat: NormalAtkDmgBonus, R1-R5: TBV
```

**9. Toukabou Shigure** — 落下攻撃命中後DMGアップ [Pattern A]
```
name: "toukabou_dmg", stat: DmgBonus, R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
```

### Claymores (2)

**10. Earth Shaker** — スキル命中後元素爆発DMGアップ [Pattern A]
```
name: "earth_shaker_burst_dmg", stat: BurstDmgBonus, R1-R5: TBV
```

**11. Master Key** — スキル後元素熟知アップ [Pattern A]
```
name: "master_key_em", stat: ElementalMastery, R1-R5: TBV
```

Note: Luxurious Sea-Lord removed — BurstDmgBonus already implemented as unconditional StatBuff. Tuna proc is out of scope.

### Bows (8)

**12. Fading Twilight** — 3段階サイクルDMGアップ [Pattern D]
```
name: "fading_twilight_dmg", stat: DmgBonus
activation: Stacks(3)
stack_values: Some(&[0.06, 0.10, 0.14])  (R1 values)
refinement_values: None
value: 0.06  (R1 state 1)
```
Note: R5固定のイベント武器。stack_valuesにR1値を格納。

**13. Hamayumi** — 常時NA/CA DMG + エネルギー満タン追加 [Pattern E]
```
StatBuff (unconditional, add to existing `buffs` array):
  buff1: stat: NormalAtkDmgBonus, R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
  buff2: stat: ChargedAtkDmgBonus, R1-R5: [0.12, 0.15, 0.18, 0.21, 0.24]

ConditionalBuff (Toggle — energy full):
  buff3: name: "hamayumi_full_energy_na", stat: NormalAtkDmgBonus, R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
  buff4: name: "hamayumi_full_energy_ca", stat: ChargedAtkDmgBonus, R1-R5: [0.12, 0.15, 0.18, 0.21, 0.24]
```

**14. Mitternachts Waltz** — 通常攻撃→スキルDMG / スキル→通常攻撃DMG [Pattern B]
```
buff1: name: "mitternachts_skill_dmg", stat: SkillDmgBonus, R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
buff2: name: "mitternachts_na_dmg", stat: NormalAtkDmgBonus, R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
```

**15. Predator** — 氷元素命中後NA/CA DMGアップ（精錬固定） [Pattern B]
```
buff1: name: "predator_na_dmg", stat: NormalAtkDmgBonus, R1-R5: [0.10, 0.10, 0.10, 0.10, 0.10]
buff2: name: "predator_ca_dmg", stat: ChargedAtkDmgBonus, R1-R5: [0.10, 0.10, 0.10, 0.10, 0.10]
```
Note: PS限定武器。精錬不可、全精錬で固定10%。

**16. Prototype Crescent** — 弱点命中後ATK%アップ [Pattern A]
```
name: "prototype_crescent_atk", stat: AtkPercent, R1-R5: [0.36, 0.45, 0.54, 0.63, 0.72]
```

**17. Sequence of Solitude** — HP変動時スキル/爆発DMGアップ [Pattern B]
```
buff1: name: "sequence_skill_dmg", stat: SkillDmgBonus, R1-R5: TBV
buff2: name: "sequence_burst_dmg", stat: BurstDmgBonus, R1-R5: TBV
```

**18. Snare Hook** — スキル命中後DMGアップ [Pattern A]
```
name: "snare_hook_dmg", stat: DmgBonus, R1-R5: TBV
```

**19. Windblume Ode** — スキル後ATK%アップ [Pattern A]
```
name: "windblume_atk", stat: AtkPercent, R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
```

Note: Amos' Bow removed — it is a 5-star weapon, not 4-star.

### Catalysts (9)

**20. Blackmarrow Lantern** — 夜魂力消費時DMGアップ [Pattern A]
```
name: "blackmarrow_dmg", stat: DmgBonus, R1-R5: TBV
```

**21. Dawning Frost** — スキル命中後DMGアップ [Pattern A]
```
name: "dawning_frost_dmg", stat: DmgBonus, R1-R5: TBV
```

**22. Dodoco Tales** — NA命中後CA DMG / CA命中後ATK [Pattern B]
```
buff1: name: "dodoco_ca_dmg", stat: ChargedAtkDmgBonus, R1-R5: [0.16, 0.20, 0.24, 0.28, 0.32]
buff2: name: "dodoco_atk", stat: AtkPercent, R1-R5: [0.08, 0.10, 0.12, 0.14, 0.16]
```

**23. Etherlight Spindlelute** — 爆発後DMGアップ [Pattern A]
```
name: "etherlight_dmg", stat: DmgBonus, R1-R5: TBV
```

**24. Flowing Purity** — HP消費時DMGアップ [Pattern A]
```
name: "flowing_purity_dmg", stat: DmgBonus, R1-R5: TBV
```

**25. Sacrificial Jade** — フィールド上でHP% + EM [Pattern B]
```
buff1: name: "sacrificial_jade_hp", stat: HpPercent, R1-R5: [0.32, 0.40, 0.48, 0.56, 0.64]
buff2: name: "sacrificial_jade_em", stat: ElementalMastery, R1-R5: [40.0, 50.0, 60.0, 70.0, 80.0]
```

**26. Solar Pearl** — NA→スキル/爆発DMG / スキル/爆発→NA DMG [Pattern B]
```
buff1: name: "solar_pearl_skill_dmg", stat: SkillDmgBonus, R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
buff2: name: "solar_pearl_burst_dmg", stat: BurstDmgBonus, R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
buff3: name: "solar_pearl_na_dmg", stat: NormalAtkDmgBonus, R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
```
Note: buff1+buff2 replace the previous DmgBonus approximation. Using SkillDmgBonus+BurstDmgBonus is more accurate than DmgBonus (which over-buffs NA).

**27. The Widsith** — ランダムバフ（ATK% / 元素DMG / EM） [Pattern B]
```
buff1: name: "widsith_atk", stat: AtkPercent, R1-R5: [0.60, 0.75, 0.90, 1.05, 1.20]
buff2: name: "widsith_dmg", stat: DmgBonus, R1-R5: [0.48, 0.60, 0.72, 0.84, 0.96]
buff3: name: "widsith_em", stat: ElementalMastery, R1-R5: [240.0, 300.0, 360.0, 420.0, 480.0]
```
Note: 3つのうち1つがランダム発動。ユーザーがシミュレーションしたいものをToggle。

**28. Wine and Song** — ダッシュ後ATK%アップ [Pattern A]
```
name: "wine_and_song_atk", stat: AtkPercent, R1-R5: [0.20, 0.25, 0.30, 0.35, 0.40]
```

Note: Mappa Mare removed — already has ConditionalBuff (Stacks) implemented.

## Design Decisions

1. **Cinnabar Spindle / Flute of Ezpitzal**: `Auto(StatScaling{DefPercent})` で正確にDEF×倍率を自動計算。`Both(StatScaling, Toggle)` は auto値を破棄するため不適切。SkillFlatDmg等はスキルダメージ計算時のみ適用されるので Toggle 不要。
2. **Fading Twilight**: `Stacks(3)` + `stack_values` でR1の3状態値を格納。R5固定イベント武器のため `refinement_values: None`。
3. **Hamayumi**: 常時効果は `StatBuff`（`buffs` フィールド）、エネルギー満タンボーナスは `ConditionalBuff` Toggle。
4. **Festering Desire**: SkillDmgBonus は既存 StatBuff。CritRate のみ ConditionalBuff として追加。
5. **Royal Longsword**: 会心リセット機構はToggle最大値（5スタック合計）で近似。R1=40%。
6. **Solar Pearl**: DmgBonus近似をやめ、SkillDmgBonus + BurstDmgBonus の2バフで正確にモデル。
7. **TBV値（9武器）**: 実装フェーズでゲームデータソースから確定。
8. **Flute of Ezpitzal stat TBV**: stat種別（NormalAtkFlatDmg等）も実装時に確定。複数攻撃種の場合はPattern B+Cの複合になる可能性あり。

## Removed from Original Scope

| Weapon | Reason |
|--------|--------|
| Amos' Bow | 5-star weapon (not 4-star) |
| Luxurious Sea-Lord | BurstDmgBonus already in StatBuff; tuna proc is out of scope |
| Mappa Mare | Already has ConditionalBuff (Stacks) |

## Impact

### Files Modified

| File | Weapons | Count |
|------|---------|-------|
| `crates/data/src/weapons/sword.rs` | #1-9 | 9 |
| `crates/data/src/weapons/claymore.rs` | #10-11 | 2 |
| `crates/data/src/weapons/bow.rs` | #12-19 | 8 |
| `crates/data/src/weapons/catalyst.rs` | #20-28 (+Mappa Mare skip) | 9 |

### Core Crate

No changes required. All patterns use existing types.

## Test Plan

28 unit tests (one per weapon). Each test verifies:
- ConditionalBuff count
- Stat type (`BuffableStat` variant)
- R1 value (or stack_values[0] for Stacks)
- Activation type (Toggle / Stacks / Auto(StatScaling))
- For Stacks: max stack count and stack_values
- For Hamayumi: StatBuff count + ConditionalBuff count
- For Festering Desire: existing StatBuff preserved + new ConditionalBuff added

## Out of Scope

- 5-star weapons (Amos' Bow etc.)
- Proc damage weapons (追加ダメージ, 範囲ダメージ, ダメージ弾) — 21本
- Particle generation (Favonius series)
- CD reset (Sacrificial series)
- HP recovery / Shield generation
- Placeholder weapons (7 polearms)
- Mappa Mare, Iron Sting (already implemented)
- Luxurious Sea-Lord (already fully implemented)
- `Both(StatScaling, Toggle)` value propagation fix (separate issue)
