# P1: Talent Buff Expansion Design

## Overview

Expand `TalentBuffDef` coverage from 9 characters to 27 characters using the existing data structures. No core crate changes required. Data crate currently covers v5.8; this expansion adds talent buffs for all expressible characters through v6.4.

## Pre-requisite: Fix Sara ID Bug

`talent_buffs.rs` registers Sara as `"sara"` but `characters/electro.rs` uses `id: "kujou_sara"`. This causes `find_talent_buffs(char_data.id)` to never match Sara's buffs. **Must fix to `"kujou_sara"` before adding C6 entry.**

## Design Decisions

### Scope
- **In scope:** All team stat buffs expressible in current `TalentBuffDef` (味方バフ全般)
- **Out of scope:** Enemy-side debuffs (resistance shred, DEF reduction) deferred to P6
- **Version range:** Data crate is v5.8. This task adds talent buffs for v1.0-v5.8 existing characters + 3 new v5.8-v6.4 characters (Ineffa, Jahoda, Aino)
- **Approach:** TalentBuffDef unchanged, builder absorbs all complexity

### Boundary: P1 vs P0/P6

| P1 (this task) | P0 待ち | P6 待ち |
|----------------|---------|---------|
| 味方ステータスバフ | 元素別CritRate/CritDmg (Sara C6, Illuga A1) | 耐性シュレッド (Zhongli, Faruzan) |
| builder事前計算で条件吸収 | 対象キャラ依存バフ (Raiden E) | DEF減少 (Lisa A4) |
| 命ノ星座ゲート (min_constellation) | 反応ダメバフ (Nilou A4) | |
| | 攻撃種別限定フラットダメ (Xianyun A4) | |

### Shenhe A1 Handling

Shenhe A1 has exclusive press/hold modes. To avoid P0-level complexity:
- Register **press mode only** (SkillDmgBonus + BurstDmgBonus) as default
- Hold mode (Normal/Charged/Plunging +15%) deferred to P0 conditional system

### Sara C6 Approximation

Sara C6 grants +60% Electro CRIT DMG. `BuffableStat` lacks `ElementalCritDmg(Element)`.
- Approximation: Register as generic `CritDmg` +0.60 with `min_constellation: 6`
- Note in description that this is Electro-specific; builder users should only apply to Electro teams
- Accurate implementation deferred to P0 (BuffableStat extension)

### Known Approximations and Limitations

| Entry | Approximation | Impact |
|-------|---------------|--------|
| Sara C6 | Generic CritDmg instead of Electro-specific | Over-buffs non-Electro teams; builder users must apply to Electro only |
| Chevreuse A1 | Unconditional ATK+20%; actual requires Overloaded trigger in Pyro+Electro-only team | Over-buffs teams without Overloaded capability; builder should only use in Pyro+Electro comps |
| Gorou 3-Geo DMG | Always registered; actual requires 3 Geo members | Builder should only include when team has 3+ Geo |
| Yoimiya A4 | Registered as max ATK+20%; actual is 10% per Niwabi hit (1-2 stacks) | Slight over-buff if fewer than 2 stacks; max assumption is standard for calc tools |
| Shenhe A1 | Press mode only (Skill/Burst DMG+15%); hold mode excluded | Hold mode users need P0 |
| Builder-pattern entries | base_value=0.0 requires builder to set actual value | If builder doesn't set value, buff resolves to 0 (no effect, not incorrect) |

## Data Structure

No changes to `TalentBuffDef`, `BuffableStat`, or `ResolvedBuff`.

Three implementation patterns used:

### Pattern 1: Fixed Value
```rust
// Ganyu A4: Cryo DMG +20%
TalentBuffDef {
    name: "Harmony between Heaven and Earth",
    description: "Cryo DMG Bonus +20% for party in burst field",
    stat: BuffableStat::ElementalDmgBonus(Element::Cryo),
    base_value: 0.20,
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}
```

### Pattern 2: Talent Level Scaling
```rust
// Faruzan Burst: Anemo DMG Bonus (Lv1-15)
TalentBuffDef {
    name: "Prayerful Wind's Benefit",
    description: "Anemo DMG Bonus based on burst talent level",
    stat: BuffableStat::ElementalDmgBonus(Element::Anemo),
    base_value: 0.0,
    scales_with_talent: true,
    talent_scaling: Some(&FARUZAN_BURST_ANEMO_SCALING),
    scales_on: None,
    target: BuffTarget::Team,
    source: TalentBuffSource::ElementalBurst,
    min_constellation: 0,
}
```

### Pattern 3: Builder Pre-Computation
```rust
// Sucrose A4: 20% of own EM shared to team
TalentBuffDef {
    name: "Mollis Favonius",
    description: "Shares 20% of Sucrose's EM to party (builder computes EM * 0.20)",
    stat: BuffableStat::ElementalMastery,
    base_value: 0.0, // builder sets value at resolve time
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::TeamExcludeSelf,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}
```

## Full Buff List

### v1.0-v5.8 Characters (existing in data crate)

| Character | Buff | Pattern | stat | Value | source | min_c |
|-----------|------|---------|------|-------|--------|-------|
| Sucrose | Catalyst Conversion (A1) | Fixed | EM | 50.0 | AscensionPassive | 0 |
| Sucrose | Mollis Favonius (A4) | Builder | EM | EM*0.20 | AscensionPassive | 0 |
| Faruzan | Prayerful Wind's Benefit | Scaling | ElementalDmgBonus(Anemo) | Lv1-15 | ElementalBurst | 0 |
| Ganyu | Harmony between Heaven and Earth (A4) | Fixed | ElementalDmgBonus(Cryo) | 0.20 | AscensionPassive | 0 |
| Albedo | Homuncular Nature (A4) | Fixed | EM | 125.0 | AscensionPassive | 0 |
| Diona | Cat's Tail Closing Time (C6) | Fixed | EM | 200.0 | Constellation(6) | 6 |
| Ningguang | Strategic Reserve (A4) | Fixed | ElementalDmgBonus(Geo) | 0.12 | AscensionPassive | 0 |
| Yelan | Adapt With Ease (A4) | Builder | DmgBonus | max 0.50 | AscensionPassive | 0 |
| Candace | Sacred Rite: Wagtail's Tide | Scaling | NormalAtkDmgBonus | Lv1-15 | ElementalBurst | 0 |
| Yoimiya | Summer Night's Dawn (A4) | Fixed | AtkPercent | 0.20 (max) | AscensionPassive | 0 |
| Gorou | Inuzaka All-Round Defense | Scaling | DefFlat | Lv1-15 | ElementalSkill | 0 |
| Gorou | Inuzaka All-Round Defense (3 Geo) | Fixed | ElementalDmgBonus(Geo) | 0.15 | ElementalSkill | 0 |
| Shenhe | Deific Embrace Press (A1) - Skill | Fixed | SkillDmgBonus | 0.15 | AscensionPassive | 0 |
| Shenhe | Deific Embrace Press (A1) - Burst | Fixed | BurstDmgBonus | 0.15 | AscensionPassive | 0 |
| Amber | Let's Dance! (C6) | Fixed | AtkPercent | 0.15 | Constellation(6) | 6 |
| Barbara | Vitality Burst (C2) | Fixed | ElementalDmgBonus(Hydro) | 0.15 | Constellation(2) | 2 |
| Thoma | Burning Heart (C6) - Normal | Fixed | NormalAtkDmgBonus | 0.15 | Constellation(6) | 6 |
| Thoma | Burning Heart (C6) - Charged | Fixed | ChargedAtkDmgBonus | 0.15 | Constellation(6) | 6 |
| Thoma | Burning Heart (C6) - Plunging | Fixed | PlungingAtkDmgBonus | 0.15 | Constellation(6) | 6 |
| Chevreuse | Vanguard's Coordinated Tactics (A1) | Fixed | AtkPercent | 0.20 | AscensionPassive | 0 |
| Dendro Traveler | Verdant Luxury (A4) | Fixed | EM | 60.0 | AscensionPassive | 0 |
| Sara | Subjugation: Koukou Sendou (C6) | Fixed | CritDmg | 0.60 | Constellation(6) | 6 |

### v5.8-v6.4 Characters (CharacterData addition needed for Jahoda/Aino)

| Character | Buff | Pattern | stat | Value | source | min_c |
|-----------|------|---------|------|-------|--------|-------|
| Ineffa | A4 EM Share | Builder | EM | ATK*0.06 | AscensionPassive | 0 |
| Jahoda | A4 EM Buff | Fixed | EM | 100.0 | AscensionPassive | 0 |
| Aino | C1 EM Share | Fixed | EM | 80.0 | Constellation(1) | 1 |

**Total: 25 new buff entries. 18 new characters added to ALL_TALENT_BUFFS + 2 existing characters (Shenhe, Sara) expanded. Post-expansion: 27 characters in ALL_TALENT_BUFFS.**

Note: Talent-level-scaling entries (Faruzan, Candace, Gorou) require `static [f64; 15]` arrays sourced from Wiki. These values must be verified against in-game data at implementation time.

## New CharacterData Required

### Jahoda (v6.2)
- Element: Anemo
- Weapon: Bow
- Rarity: 4-star
- Requires full `CharacterData` struct: `base_hp[4]`, `base_atk[4]`, `base_def[4]`, `ascension_stat`, `weapon_type`, `element`, talent scaling info, constellation boost pattern
- All numeric values sourced from Wiki at implementation time
- Comparable effort to existing 4-star entries (~50-80 lines)

### Aino (v6.0)
- Element: Hydro
- Weapon: Claymore
- Rarity: 4-star
- Same scope as Jahoda above
- Must also register in `ALL_CHARACTERS` in `characters/mod.rs`

## File Changes

| File | Change |
|------|--------|
| `crates/data/src/talent_buffs.rs` | Fix Sara ID `"sara"` → `"kujou_sara"` + 25 new buff entries (18 new chars + 2 expanded) |
| `crates/data/src/characters/anemo.rs` | +Jahoda CharacterData |
| `crates/data/src/characters/hydro.rs` | +Aino CharacterData |
| `crates/data/src/characters/mod.rs` | +Jahoda, Aino in ALL_CHARACTERS |
| `crates/data/src/team_builder.rs` | +tests only (no logic changes) |
| `docs/data-expansion-todo.md` | P1 checklist update |

### Core crate: No changes

## Testing

### Unit Tests (talent_buffs.rs) ~15 tests
- `find_talent_buffs` returns correct entries for each new character
- stat, target, base_value correctness per entry
- talent_scaling spot checks (Lv1, Lv13 for scaling characters)
- `test_all_talent_buffs_have_unique_ids` continues to pass

### Integration Tests (team_builder.rs) ~5 tests
- Faruzan: talent_scaling value at specific levels
- Diona C6: buff absent at C0, present at C6
- Shenhe A1: SkillDmgBonus + BurstDmgBonus present in buffs_provided
- Sara C6: CritDmg absent at C0, present at C6
- Jahoda/Aino: CharacterData builds successfully, buff in buffs_provided

### CharacterData Tests ~2 tests
- Jahoda/Aino base stats are positive and reasonable
- Element and weapon type correct

### Regression
- `cargo test` all existing 173+61 tests pass
- `cargo clippy -- -D warnings` clean

## P0 Deferred Items

These characters/buffs require type extensions and are deferred to P0:

| Character | Buff | Required Extension |
|-----------|------|--------------------|
| Sara C6 | Electro CRIT DMG +60% | `ElementalCritDmg(Element)` in BuffableStat (currently approximated as generic CritDmg) |
| Illuga A1 | Geo CRIT Rate +5% / CRIT DMG +10% | `ElementalCritRate(Element)` + `ElementalCritDmg(Element)` |
| Raiden E | Burst DMG per energy cost | Per-target-stat scaling mechanism |
| Xianyun A4 | Plunging flat DMG from ATK | Attack-type-specific flat DMG |
| Nilou A4 | Bloom reaction DMG bonus | Reaction DMG bonus type in BuffableStat |
| Shenhe A1 Hold | Normal/Charged/Plunging +15% | Exclusive mode selection (press vs hold) |

## Implementation Order

1. Fix Sara ID bug: `"sara"` → `"kujou_sara"` in talent_buffs.rs
2. Jahoda/Aino CharacterData addition (Wiki data)
3. talent_buffs.rs: 25 new buff entries (including scaling arrays from Wiki)
4. Tests: unit + integration + CharacterData
5. Verify: `cargo test && cargo clippy -- -D warnings`
6. Update docs/data-expansion-todo.md
