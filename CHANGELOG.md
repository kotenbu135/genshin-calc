# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## [0.5.14] - 2026-04-17

### Added
- Direct lunar damage calculation pipeline (`calculate_direct_lunar`) for talent-originated
  damage that is classified as lunar-reaction damage (Ineffa A1/C4/C6,
  Flins burst/C4, Columbina skill, Lauma skill hold, Nefer C6, Zibai skill 2nd hit/constellations)
  - New public types: `DirectLunarInput` (re-uses `LunarResult`)
  - Formula: `base = talent_mult × scaling_value + flat_dmg`, multiplied by
    `(1 + base_dmg_bonus) × (1 + lunar_em_bonus + reaction_bonus) × res_mult`,
    with crit applied; no level base, no DEF multiplier, no elemental DMG%
  - WASM binding `calculate_direct_lunar`
  - TOML test coverage: Ineffa A1 & C6, Lauma skill hold, Zibai skill 2-hit

## [0.1.0] - 2026-03-31

### Added
- Damage calculation engine (`genshin-calc-core`)
  - Standard damage pipeline with ATK/HP/DEF scaling, crit, defense, and resistance
  - Amplifying reactions (Vaporize, Melt)
  - Catalyze reactions (Aggravate, Spread)
  - Transformative reactions (Overloaded, Superconduct, Electro-Charged, Swirl, Bloom, etc.)
  - Lunar reactions (Lunar Electro-Charged, Lunar Bloom, Lunar Crystallize)
  - Moonsign system support
  - Team composition with buff resolution
  - Elemental resonance calculation
  - Stat profile combination
  - Input validation with descriptive errors
  - serde support (Serialize/Deserialize)
- Game data library (`genshin-calc-data`)
  - 102 playable characters
  - 230 weapons with conditional buff system
  - 52 artifact sets with 2pc/4pc effects
  - 15 enemy types with resistance templates
  - Team builder API
  - Moonsign benediction and talent enhancement data
  - Talent buff definitions
