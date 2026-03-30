# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

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
