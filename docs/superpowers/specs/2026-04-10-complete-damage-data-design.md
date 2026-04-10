# Complete Damage Data Design

Date: 2026-04-10

## Goal

Complete the package's damage-related character and artifact data using the existing audit documents under `docs/audit/`.

The implementation should make audited damage-relevant behavior match the HoneyHunter mirror data while preserving existing public compatibility where practical.

## Sources

- Character audits: `docs/audit/*_audit.md`
- Character talent buff audits: `docs/audit/*_buffs_audit.md`
- Artifact buff audit: `docs/audit/artifact_buffs_audit.md`
- Character mirror data: `honeyhunter-mirror/md/characters/*.md`
- Artifact mirror data: `honeyhunter-mirror/md/artifacts/*.md`
- Implementation data: `crates/data/src/characters/*`, `crates/data/src/talent_buffs/*`, `crates/data/src/artifacts.rs`

## Scope

Included:

- Character base stats, weapon type, element, ascension stat, and constellation talent level pattern.
- Character normal, charged, plunging, skill, and burst scaling rows and values.
- Damage-relevant character buffs, debuffs, passives, and constellations.
- Artifact set effects that affect ATK, HP, DEF, Elemental Mastery, CRIT, damage bonuses, reaction damage, and enemy RES/DEF.
- Reaction-specific damage bonuses where category-wide modeling would over-apply.

Excluded:

- Added damage, flat added damage, and independent proc damage correctness work.
- Removal of existing `NormalAtkFlatDmg`, `ChargedAtkFlatDmg`, `PlungingAtkFlatDmg`, `SkillFlatDmg`, and `BurstFlatDmg` implementations.
- Pure healing, defensive resistance, shield strength, energy refund, cooldown, stamina, and attack speed effects unless they are necessary context for a checked damage effect.

## Architecture

### Reaction-Specific Buffs

Add a minimal reaction-specific damage bonus variant to `BuffableStat`:

```rust
ReactionDmgBonus(Reaction)
```

This variant represents a bonus that applies only to the exact reaction. Examples:

- `ReactionDmgBonus(Reaction::Bloom)`
- `ReactionDmgBonus(Reaction::Hyperbloom)`
- `ReactionDmgBonus(Reaction::Aggravate)`
- `ReactionDmgBonus(Reaction::Swirl(Element::Pyro))`
- `ReactionDmgBonus(Reaction::LunarBloom)`

Keep existing broad variants for backward compatibility:

- `AmplifyingBonus`
- `TransformativeBonus`
- `AdditiveBonus`

New and corrected data should use `ReactionDmgBonus` when the mirror effect names specific reactions and a broad category would over-apply.

### Bonus Resolution

Add or update helper logic so reaction bonus resolution can combine:

- Broad category bonuses for existing behavior.
- Exact `ReactionDmgBonus(reaction)` values.

The resolved reaction bonus should feed the existing calculation inputs:

- Standard damage: `DamageInput.reaction_bonus`
- Transformative damage: `TransformativeInput.reaction_bonus`
- Lunar damage: `LunarInput.reaction_bonus`

This avoids duplicating formula logic while making buff application precise.

### WASM And TypeScript

Expose the new `ReactionDmgBonus(Reaction)` serialized shape through the WASM bindings and TypeScript declarations.

Existing serialized values for `TransformativeBonus`, `AdditiveBonus`, and `AmplifyingBonus` remain valid.

## Data Work

### Character Data

Fix audited `FAIL` items in `crates/data/src/characters/<element>/*.rs`.

Priority order:

1. Structural errors: missing rows, collapsed multi-hit rows, wrong weapon type, wrong normal attack element.
2. Base stat arrays that are shifted or scrambled.
3. Wrong ascension stat.
4. Talent scaling values and rounding errors outside the established tolerance.

Lv100-only extrapolation drift is not treated as a failure unless the audit explicitly marks it as in-scope.

### Character Talent Buffs

Fix audited `BUGS`, `HIGH`, `MISSING`, and clear target/stat/value issues in `crates/data/src/talent_buffs/<element>.rs`.

Use `ReactionDmgBonus` for reaction-specific effects such as Bloom-only, Aggravate-only, Swirl-only, Lunar-Bloom-only, or Lunar-Charged-only bonuses.

Where the current model cannot represent detailed conditions such as current active character, enemy HP threshold, Hexerei-only state, or reaction-triggered time windows, keep the numeric effect correct and document the limitation in the buff description or a targeted limitation note.

Do not remove existing flat added damage support as part of this work.

### Artifact Sets

Fix `docs/audit/artifact_buffs_audit.md` discrepancies in `crates/data/src/artifacts.rs`.

Required high-priority changes:

- Swap `Nymph's Dream` 4pc ATK and Hydro DMG stack values to match the mirror.
- Remove the erroneous `Scroll of the Hero of Cinder City` 2pc EM bonus because the mirror 2pc is energy-only and out of damage scope.
- Add missing damage-relevant sets: `Finale of the Deep Galleries`, `Long Night's Oath`, `Adventurer`, and `Lucky Dog`.
- Replace generic reaction bonuses with `ReactionDmgBonus` where broad application is inaccurate, especially `Thundering Fury`, `Viridescent Venerer`, and `Flower of Paradise Lost`.

Condition/description fixes should be made where the value is already correct but the activation text is misleading.

`Traveling Doctor` has no checked damage-related effect, so it is not required for damage-data completion.

## Tests

Add regression tests for the model extension and representative audited fixes.

Required reaction-specific tests:

- `ReactionDmgBonus(Reaction::Bloom)` applies to Bloom and not Overloaded.
- `ReactionDmgBonus(Reaction::Aggravate)` applies to Aggravate and not Spread.
- `ReactionDmgBonus(Reaction::Swirl(Element::Pyro))` applies to Pyro Swirl and not Hydro Swirl.
- `ReactionDmgBonus(Reaction::LunarBloom)` applies to Lunar-Bloom and not Lunar Electro-Charged.

Required artifact tests:

- `Nymph's Dream` stack values match ATK 7%/16%/25% and Hydro DMG 4%/9%/15%.
- `Scroll of the Hero of Cinder City` 2pc does not grant Elemental Mastery.
- Newly added damage-relevant sets can be found by ID.
- Newly added sets expose their audited damage-related buffs.

Required character tests:

- Representative repaired base stat arrays match audited HoneyHunter values at key breakpoints.
- Representative repaired talent rows include the expected number of hits.
- Corrected weapon type, element, and ascension stat mismatches are asserted.

## Verification

Run:

```sh
cargo fmt
cargo test -p genshin-calc-core
cargo test -p genshin-calc-data
cargo test -p genshin-calc-wasm
```

If package-specific tests pass, run:

```sh
cargo test --workspace
```

If a command cannot run in the local environment, record the failure and the reason in the final implementation summary.

## Completion Criteria

The work is complete when:

- Every in-scope `FAIL`, `ISSUE`, `MISSING`, and discrepancy in `docs/audit/` has either been fixed or explicitly documented as out-of-scope because it is added damage/proc damage or non-damage behavior.
- Reaction-specific bonuses no longer over-apply through broad category stats where the audit identified that as inaccurate.
- Added damage implementations are left intact but are not used as proof of completion for this scope.
- Core/data/WASM type surfaces remain compatible except for the additive `ReactionDmgBonus` variant.
- Verification commands pass or any remaining inability to run them is documented.
