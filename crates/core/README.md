# genshin-calc-core

Damage and elemental reaction calculation engine for Genshin Impact.

## Installation

```toml
[dependencies]
genshin-calc-core = "0.1"
```

Minimum supported Rust version: **1.85**

## Features

Three calculation pipelines covering all damage types:

- **`calculate_damage`** — standard damage (normal/charged/skill/burst) with optional amplifying (vaporize/melt) or catalyze (spread/aggravate) reactions
- **`calculate_transformative`** — transformative reactions (overloaded, superconduct, electro-charged, swirl, bloom, etc.) — scales with level and EM, no crit
- **`calculate_lunar`** — lunar reactions (lunar electro-charged, lunar bloom, lunar crystallize) — like transformative but can crit

## Usage

```rust
use genshin_calc_core::*;

let input = DamageInput {
    character_level: 90,
    stats: Stats {
        atk: 2000.0,
        crit_rate: 0.75,
        crit_dmg: 1.50,
        dmg_bonus: 0.466,
        ..Default::default()
    },
    talent_multiplier: 1.76,
    scaling_stat: ScalingStat::Atk,
    damage_type: DamageType::Skill,
    element: Some(Element::Pyro),
    reaction: None,
    reaction_bonus: 0.0,
};
let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
let result = calculate_damage(&input, &enemy).unwrap();

println!("Average damage: {:.0}", result.average);
```

## Supported Reactions

| Category | Reactions |
|----------|-----------|
| Amplifying | Vaporize, Melt |
| Catalyze | Aggravate, Spread |
| Transformative | Overloaded, Superconduct, Electro-Charged, Shattered, Swirl, Bloom, Hyperbloom, Burgeon, Burning |
| Lunar | Lunar Electro-Charged, Lunar Bloom, Lunar Crystallize |

## License

MIT
