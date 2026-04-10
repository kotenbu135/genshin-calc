# Artifact Damage-Related Set Effects Audit

Audit date: 2026-04-10

Sources checked:
- Mirror: `honeyhunter-mirror/md/artifacts/*.md`
- Implementation: `crates/data/src/artifacts.rs`

Scope:
- Checked damage-related set performance: ATK/HP/DEF stats, EM, damage bonuses, CRIT, reaction bonuses, and enemy RES reduction.
- Excluded added-damage / proc / flat added damage effects per request. Examples: Ocean-Hued Clam foam damage, Echoes of an Offering Valley Rite, Song of Days Past flat added damage.
- Pure healing, defensive RES, shield strength, energy refund, cooldown, stamina, and attack speed effects are noted only when they affect a checked damage effect.

SubAgent split:
- Agent 1: `adventurer` through `finale_of_the_deep_galleries`
- Agent 2: `flower_of_paradise_lost` through `maiden_beloved`
- Agent 3: `marechaussee_hunter` through `shimenawa_s_reminiscence`
- Agent 4: `song_of_days_past` through `wanderer_s_troupe`

## Summary

- HoneyHunter artifact md files checked: 51
- Damage-related discrepancies in implemented sets: 12
- Damage-related missing sets/effects from HoneyHunter md: 4
- Missing set with no damage-related checked effect: 1
- Added-damage effects excluded from discrepancy count: 3
- Implemented sets without matching HoneyHunter md source in this folder: 10

## High-Priority Issues

### Nymph's Dream

**Problem**: 4pc ATK and Hydro DMG stack values are swapped.

- Mirror: ATK +7%/+16%/+25%, Hydro DMG +4%/+9%/+15%.
- Rust: `AtkPercent` uses 4%/9%/15%; `ElementalDmgBonus(Hydro)` uses 7%/16%/25%.
- Refs: `crates/data/src/artifacts.rs:1179`, `crates/data/src/artifacts.rs:1185`, `crates/data/src/artifacts.rs:1196`

**Implementation status:** Fixed in this branch.

### Scroll of the Hero of Cinder City

**Problem**: 2pc is implemented as EM +80, but the mirror says Elemental Energy +6 after nearby party Nightsoul Burst.

- Mirror 2pc: energy refund only, out of damage scope.
- Rust 2pc: `ElementalMastery +80`, which is a damage-relevant erroneous stat.
- Mirror 4pc: reaction-related team Elemental DMG +12%, plus additional +28% if the wearer is in Nightsoul's Blessing, total +40%.
- Rust 4pc: values are represented as `0.12` with `nightsoul_value: Some(0.40)`, but the description/trigger duration is outdated.
- Refs: `crates/data/src/artifacts.rs:1533`, `crates/data/src/artifacts.rs:1535`, `crates/data/src/artifacts.rs:1543`

**Implementation status:** Fixed in this branch.

### Finale of the Deep Galleries

**Problem**: Entire damage-relevant set is missing.

- Mirror 2pc: Cryo DMG +15%.
- Mirror 4pc: at 0 Elemental Energy, Normal Attack DMG +60% and Elemental Burst DMG +60%, with mutually exclusive cooldown behavior after dealing those damage types.
- Rust: no `finale_of_the_deep_galleries` artifact set and no registration in `ALL_ARTIFACT_SETS`.
- Refs: `crates/data/src/artifacts.rs:2549`

**Implementation status:** Fixed in this branch.

### Long Night's Oath

**Problem**: Entire damage-relevant set is missing.

- Mirror 2pc: Plunging Attack DMG +25%.
- Mirror 4pc: Plunging/Charged/Skill hits grant 1/2/2 stacks; Plunging Attack DMG +15% per stack, max 5.
- Rust: no `long_night_s_oath` artifact set and no registration in `ALL_ARTIFACT_SETS`.
- Refs: `crates/data/src/artifacts.rs:2549`

**Implementation status:** Fixed in this branch.

## Other Discrepancies

### Archaic Petra

Values are correct, but the trigger text is incomplete.

- Mirror: obtaining a Crystallize shard or triggering Lunar-Crystallize grants party +35% DMG Bonus for the corresponding element.
- Rust: implements Pyro/Hydro/Electro/Cryo Elemental DMG +35% for Team, but the description only covers normal Crystallize shard pickup.
- Refs: `crates/data/src/artifacts.rs:393`

**Implementation status:** Fixed in this branch.

### Flower of Paradise Lost

Lunar-Bloom handling is not represented accurately.

- Mirror: Bloom/Hyperbloom/Burgeon +40%, Lunar-Bloom +10%; after triggering those reactions, the listed effects gain +25% per stack, max 4.
- Rust: `TransformativeBonus +40%` and `TransformativeBonus +10%/stack`.
- Issues: Lunar-Bloom base +10% and its +25% stack scaling are not separately modeled; generic `TransformativeBonus` may apply wider than the listed reactions.
- Refs: `crates/data/src/artifacts.rs:1133`, `crates/data/src/artifacts.rs:1140`, `crates/data/src/artifacts.rs:1151`

**Implementation status:** Fixed in this branch.

### Fragment of Harmonic Whimsy

Values are correct, but the activation text is wrong.

- Mirror: Bond of Life value increases/decreases -> DMG +18% per stack, max 3.
- Rust: `DmgBonus +18%` per stack, max 3, but the description says HP changes.
- Refs: `crates/data/src/artifacts.rs:1480`, `crates/data/src/artifacts.rs:1485`

**Implementation status:** Fixed in this branch.

### Gilded Dreams

Values are correct, but activation timing is simplified as always-on team composition.

- Mirror: within 8s of triggering an Elemental Reaction, ATK +14% per same-element teammate and EM +50 per different-element teammate, each up to 3.
- Rust: same values, but uses auto team same/different element counts rather than the reaction-triggered 8s window.
- Refs: `crates/data/src/artifacts.rs:989`, `crates/data/src/artifacts.rs:995`, `crates/data/src/artifacts.rs:1028`

**Implementation status:** Fixed in this branch.

### Martial Artist

Values are correct, but the activation condition is too broad.

- Mirror: after using Elemental Skill, Normal/Charged Attack DMG +25%.
- Rust: description/condition says after Skill/Burst use.
- Refs: `crates/data/src/artifacts.rs:1934`, `crates/data/src/artifacts.rs:1940`, `crates/data/src/artifacts.rs:1951`

**Implementation status:** Fixed in this branch.

### Nighttime Whispers in the Echoing Woods

4pc condition and value are wrong for the enhanced portion.

- Mirror: after Skill, Geo DMG +20%; under Crystallize shield or near Lunar-Crystallize Moondrifts, the above effect is increased by 150%, so total is +50% Geo DMG.
- Rust: base +20% is correct, but the extra condition is Nightsoul point consumption and the extra value is +20%, total +40%.
- Refs: `crates/data/src/artifacts.rs:1431`, `crates/data/src/artifacts.rs:1448`

**Implementation status:** Fixed in this branch.

### Obsidian Codex

Values are correct, but the 4pc trigger text is outdated.

- Mirror: after consuming 1 Nightsoul point while on field, CRIT Rate +40% for 6s.
- Rust: `CritRate +40%`, but the description says Nightsoul points below 50%.
- Refs: `crates/data/src/artifacts.rs:1651`, `crates/data/src/artifacts.rs:1656`

**Implementation status:** Fixed in this branch.

### Thundering Fury

Reaction coverage is too generic and Lunar-Charged is not represented separately.

- Mirror: Overloaded/Electro-Charged/Superconduct/Hyperbloom +40%, Aggravate +20%, Lunar-Charged +20%.
- Rust: `TransformativeBonus +40%` and `AdditiveBonus +20%`.
- Issues: generic transformative/additive stats can apply outside the listed reactions; Spread may receive the +20% even though only Aggravate is listed; Lunar-Charged +20% is not distinct and may be missing or over-buffed depending on engine mapping.
- Refs: `crates/data/src/artifacts.rs:217`, `crates/data/src/artifacts.rs:223`, `crates/data/src/artifacts.rs:235`

**Implementation status:** Fixed in this branch.

### Unfinished Reverie

4pc condition/maintenance logic is outdated.

- Mirror: after leaving combat for 3s, DMG +50%; in combat, Burning enemy presence increases/decreases the bonus by 10% per second between 0% and 50%; works off-field.
- Rust: `DmgBonus +50%`, but description/condition is after Burning/Burgeon reaction, with different duration/off-field behavior.
- Refs: `crates/data/src/artifacts.rs:1511`, `crates/data/src/artifacts.rs:1516`

**Implementation status:** Fixed in this branch.

### Viridescent Venerer

Swirl damage bonus is too generic.

- Mirror: Swirl DMG +60%; RES -40% to the element infused in Swirl.
- Rust: Anemo DMG +15% and Pyro/Hydro/Electro/Cryo RES -40% are correct, but `TransformativeBonus +60%` may apply to all transformative reactions instead of Swirl only.
- Refs: `crates/data/src/artifacts.rs:294`, `crates/data/src/artifacts.rs:299`

**Implementation status:** Fixed in this branch.

## Missing Sets From HoneyHunter md

### Adventurer

- Mirror: 2pc Max HP +1000. This can be damage-relevant for HP-scaling characters.
- Rust: no `adventurer` set.
- 4pc healing is out of scope.
- Refs: `crates/data/src/artifacts.rs:2549`

**Implementation status:** Fixed in this branch.

### Lucky Dog

- Mirror: 2pc DEF +100. This can be damage-relevant for DEF-scaling characters.
- Rust: no `lucky_dog` set.
- 4pc healing is out of scope.
- Refs: `crates/data/src/artifacts.rs:2549`

**Implementation status:** Fixed in this branch.

### Finale of the Deep Galleries

See High-Priority Issues.

### Long Night's Oath

See High-Priority Issues.

### Traveling Doctor

- Mirror: incoming healing +20%, Burst use restores HP.
- Rust: no `traveling_doctor` set.
- Assessment: missing set, but no checked damage-related effect.
- Refs: `crates/data/src/artifacts.rs:2549`

**Implementation status:** Out of scope for this pass because this is an added-damage/proc-damage or non-damage effect.

## Added-Damage / Proc Effects Excluded

These were checked against the request and intentionally excluded from discrepancy counts.

| Set | Mirror effect | Rust status |
| --- | --- | --- |
| Echoes of an Offering | Valley Rite increases Normal Attack DMG by 70% of ATK | Implemented as `NormalAtkFlatDmg`; 2pc ATK +18% is OK |
| Ocean-Hued Clam | Sea-Dyed Foam deals separate damage based on healing | 2pc Healing Bonus is implemented; 4pc is out of scope |
| Song of Days Past | healing record adds flat damage to Normal/Charged/Plunging/Skill/Burst hits | Implemented as flat-damage buffs; excluded |

**Implementation status:** Out of scope for this pass because this is an added-damage/proc-damage or non-damage effect.

## Full Per-File Status

| HoneyHunter md | Status | Notes |
| --- | --- | --- |
| `adventurer.md` | Missing | 2pc Max HP +1000 missing; 4pc healing out of scope |
| `archaic_petra.md` | Discrepancy | Values OK; Lunar-Crystallize trigger text missing |
| `berserker.md` | OK | CRIT Rate +12%, conditional +24% |
| `blizzard_strayer.md` | OK | Cryo DMG +15%, CRIT Rate +20%/+20% |
| `bloodstained_chivalry.md` | OK | Physical DMG +25%, Charged DMG +50% |
| `brave_heart.md` | OK | ATK +18%, DMG +30% vs HP >50% |
| `crimson_witch_of_flames.md` | OK | Pyro DMG, reaction bonuses, skill stacks match |
| `deepwood_memories.md` | OK | Dendro DMG +15%, Dendro RES -30% |
| `defender_s_will.md` | OK | DEF +30%; 4pc defensive RES out of scope |
| `desert_pavilion_chronicle.md` | OK | Anemo DMG +15%, NA/CA/Plunge DMG +40% |
| `echoes_of_an_offering.md` | Excluded | 2pc OK; 4pc added damage excluded |
| `emblem_of_severed_fate.md` | OK | ER +20%, Burst DMG = 25% ER capped at 75% |
| `finale_of_the_deep_galleries.md` | Missing | Cryo DMG +15%, NA/Burst DMG +60% missing |
| `flower_of_paradise_lost.md` | Discrepancy | Lunar-Bloom and reaction-specific scoping not represented |
| `fragment_of_harmonic_whimsy.md` | Discrepancy | Value OK; condition text says HP changes instead of Bond of Life |
| `gambler.md` | OK | Skill DMG +20%; 4pc cooldown out of scope |
| `gilded_dreams.md` | Discrepancy | Values OK; reaction-triggered window simplified to auto team composition |
| `gladiator_s_finale.md` | OK | ATK +18%, weapon-gated Normal DMG +35% |
| `golden_troupe.md` | OK | Skill DMG +20%, +25%, off-field +25% |
| `heart_of_depth.md` | OK | Hydro DMG +15%, NA/CA DMG +30% |
| `husk_of_opulent_dreams.md` | OK | DEF +30%, DEF/Geo stacks +6% up to 4 |
| `instructor.md` | OK | EM +80, team EM +120 |
| `lavawalker.md` | OK | 4pc DMG +35%; 2pc defensive RES out of scope |
| `long_night_s_oath.md` | Missing | Plunging DMG +25%, stackable Plunging DMG +15% missing |
| `lucky_dog.md` | Missing | DEF +100 missing; 4pc healing out of scope |
| `maiden_beloved.md` | Out of scope | Healing only |
| `marechaussee_hunter.md` | OK | NA/CA DMG +15%, CRIT Rate +12% up to 3 |
| `martial_artist.md` | Discrepancy | Values OK; Rust condition includes Burst but mirror only Skill |
| `nighttime_whispers_in_the_echoing_woods.md` | Discrepancy | Enhanced Geo DMG should be +30% extra via Crystallize/Lunar-Crystallize, not +20% via Nightsoul |
| `noblesse_oblige.md` | OK | Burst DMG +20%, team ATK +20% |
| `nymph_s_dream.md` | Discrepancy | ATK and Hydro DMG stack values swapped |
| `obsidian_codex.md` | Discrepancy | Values OK; 4pc trigger text outdated |
| `ocean_hued_clam.md` | Excluded | Healing and separate damage proc |
| `pale_flame.md` | OK | Physical DMG +25%, ATK stacks, doubled 2pc represented |
| `resolution_of_sojourner.md` | OK | ATK +18%, Charged CRIT Rate +30% |
| `retracing_bolide.md` | OK | NA/CA DMG +40%; Shield Strength out of scope |
| `scholar.md` | OK | ER +20%; 4pc energy refund out of scope |
| `scroll_of_the_hero_of_cinder_city.md` | Discrepancy | 2pc incorrectly EM +80; 4pc values mostly OK but text/trigger stale |
| `shimenawa_s_reminiscence.md` | OK | ATK +18%, NA/CA/Plunge DMG +50% |
| `song_of_days_past.md` | Excluded | Flat added damage excluded |
| `tenacity_of_the_millelith.md` | OK | HP +20%, team ATK +20%; shield out of scope |
| `the_exile.md` | OK | ER +20%; energy refund out of scope |
| `thundering_fury.md` | Discrepancy | Reaction scoping too generic; Lunar-Charged not separately modeled |
| `thundersoother.md` | OK | DMG +35%; defensive RES out of scope |
| `tiny_miracle.md` | Out of scope | Defensive RES only |
| `traveling_doctor.md` | Missing out of scope | Missing set, but healing only |
| `unfinished_reverie.md` | Discrepancy | 4pc condition/maintenance logic does not match mirror |
| `vermillion_hereafter.md` | OK | ATK +18%, +8%, +10% stacks |
| `viridescent_venerer.md` | Discrepancy | Swirl bonus modeled as generic transformative bonus |
| `vourukasha_s_glow.md` | OK | HP +20%, Skill/Burst +10%, +8% stacks |
| `wanderer_s_troupe.md` | OK | EM +80, Bow/Catalyst Charged DMG +35% |

## Implemented Sets Without Matching HoneyHunter md Source

These exist in `crates/data/src/artifacts.rs` but do not have a corresponding file under `honeyhunter-mirror/md/artifacts/`, so they were not source-verified in this audit:

- `prayers_for_illumination`
- `prayers_for_destiny`
- `prayers_for_wisdom`
- `prayers_to_springtime`
- `glory_of_the_ancient_sea`
- `chronicled_sands_and_water`
- `night_of_the_skys_unveiling`
- `silken_moons_serenade`
- `aubade_of_morningstar_and_moon`
- `a_day_carved_from_rising_winds`
