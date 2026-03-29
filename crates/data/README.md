# genshin-calc-data

Genshin Impact v5.8 game data as Rust constants for use with
[genshin-calc-core](https://crates.io/crates/genshin-calc-core).

## Installation

```toml
[dependencies]
genshin-calc-data = "0.1"
```

Minimum supported Rust version: **1.85**

## Included Data

| Category | Count |
|----------|-------|
| Characters | 102 |
| Weapons | 230 |
| Artifact Sets | 52 |
| Enemies | 15 |

All data is current as of Genshin Impact **v5.8**.

## Usage

```rust
use genshin_calc_data::*;
use genshin_calc_core::Element;

// Look up a character
let hu_tao = find_character("hu_tao").unwrap();
println!("{} — {:?} {:?}", hu_tao.name, hu_tao.element, hu_tao.weapon_type);

// Find all Pyro characters
let pyro = characters_by_element(Element::Pyro);
println!("{} Pyro characters", pyro.len());

// Convert enemy data for damage calculation
let enemy_data = find_enemy("hilichurl").unwrap();
let enemy = enemy_data.to_enemy(Some(Element::Pyro), 90, 0.0);
```

## License

MIT
