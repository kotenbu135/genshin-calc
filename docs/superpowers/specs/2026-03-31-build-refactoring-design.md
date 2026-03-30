# `build()` 228-line Refactoring Design

## Overview

`TeamMemberBuilder::build()` (crates/data/src/team_builder.rs:137-364) is 228 lines handling 7 distinct phases inline. Refactor by extracting private methods so `build()` becomes a ~10-line "table of contents". No behavioral changes.

## Approach

Method extraction into `impl TeamMemberBuilder`. Existing free functions (`eval_auto`, `eval_manual`, `resolve_value`, `apply_ascension_stat`, `apply_weapon_sub_stat`) remain unchanged.

## Refactored `build()`

```rust
pub fn build(self) -> Result<TeamMember, CalcError> {
    self.validate()?;
    let mut profile = self.build_base_profile();
    self.merge_artifact_stats(&mut profile);
    let mut buffs = self.collect_static_buffs();
    self.collect_talent_buffs(&profile, &mut buffs);
    self.resolve_conditional_buffs(&profile, &mut buffs);
    Ok(self.into_team_member(profile, buffs))
}
```

## Extracted Methods

### `validate(&self) -> Result<(), CalcError>`

**Source:** Lines 138-148

Validates constellation (0-6), talent levels (1-15), and refinement (1-5). Returns early with `CalcError` on invalid input.

### `build_base_profile(&self) -> StatProfile`

**Source:** Lines 150-167

Combines three initialization steps into one:
1. Base stats from character + weapon (Lv90 = index 3)
2. Character ascension stat via `apply_ascension_stat()`
3. Weapon sub-stat via `apply_weapon_sub_stat()`

Rationale: All three are "initial profile construction" — same responsibility.

### `merge_artifact_stats(&self, profile: &mut StatProfile)`

**Source:** Lines 170-181

Adds 11 artifact stat fields to the profile. Separate from `build_base_profile` because this merges user-provided input rather than deriving from game data.

### `collect_static_buffs(&self) -> Vec<ResolvedBuff>`

**Source:** Lines 183-219

Collects unconditional (static) buffs from:
- Weapon passive `buffs`
- Artifact set 2-piece `buffs`
- Artifact set 4-piece `buffs`

All use the same `resolve_value()` → `ResolvedBuff` pattern.

### `collect_talent_buffs(&self, profile: &StatProfile, buffs: &mut Vec<ResolvedBuff>)`

**Source:** Lines 222-275

Resolves talent-based buffs:
1. Checks constellation gate (`min_constellation`)
2. Resolves raw value from talent scaling or base value
3. Applies stat scaling (`scales_on`) if present
4. Pushes `ResolvedBuff`

### `resolve_conditional_buffs(&self, profile: &StatProfile, buffs: &mut Vec<ResolvedBuff>)`

**Source:** Lines 278-355

Replaces the inline closure `resolve_conditionals`. Calls a private helper method `resolve_conditionals_for_source()` three times:
1. Weapon conditional buffs
2. Artifact 2-piece conditional buffs
3. Artifact 4-piece conditional buffs

#### `resolve_conditionals_for_source(&self, conditional_buffs: &'static [ConditionalBuff], source_name: &str, refinement: u8, profile: &StatProfile, buffs: &mut Vec<ResolvedBuff>)`

Extracted from the closure at lines 278-329. Evaluates each conditional buff through `Activation::Auto`, `Manual`, or `Both` paths.

### `into_team_member(self, stats: StatProfile, buffs: Vec<ResolvedBuff>) -> TeamMember`

**Source:** Lines 357-364

Consumes `self` and constructs the final `TeamMember`.

## Design Decisions

1. **`validate` takes `&self`** — Called before consuming `self`, so validation failure returns `Err` without moving ownership.
2. **`build_base_profile` groups base + ascension + weapon sub-stat** — Single "initialize profile" responsibility. Three separate methods would be over-granular.
3. **`merge_artifact_stats` is separate** — Different responsibility (user input vs. game data).
4. **`collect_static_buffs` groups weapon + artifact set buffs** — Same pattern: iterate `buffs` slice, resolve values, push `ResolvedBuff`.
5. **Existing free functions unchanged** — `eval_auto`, `eval_manual`, `resolve_value`, `apply_ascension_stat`, `apply_weapon_sub_stat`, `find_talent_buffs`, `is_moonsign_character` stay as module-level functions.
6. **Closure → method** — The `resolve_conditionals` closure captured `&profile`, `char_data`, `&self.team_elements`, etc. As a method on `self`, these are accessed via `self.*` fields directly. `profile` is passed as parameter.
7. **Local alias elimination** — The original `let char_data = self.character; let weapon = self.weapon;` locals (lines 150-151) are removed. All extracted methods use `self.character` and `self.weapon` directly.

## Test Strategy

- No new tests. This is a pure refactoring with no behavioral changes.
- Verification: `cargo test -p genshin-calc-data` — all existing tests must pass.
- Secondary: `cargo clippy -- -D warnings` and `cargo fmt --check`.

## Risk Assessment

- **Breaking changes:** None. Public API unchanged.
- **Behavioral changes:** None. Method extraction only.
- **Risk:** Low. Each method is a direct cut-paste of the original code block with minimal signature adaptation.
