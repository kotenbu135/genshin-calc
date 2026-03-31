# genshin-calc-good

GOOD (Genshin Open Object Description) format importer for [genshin-calc](https://github.com/kotenbu/genshin-calc).

Parses GOOD JSON exported by scanners (Inventory Kamera, etc.) and converts to `CharacterBuild` structs compatible with the genshin-calc calculation engine.

## Features

- GOOD format v1-v3 support
- Automatic key mapping (PascalCase → snake_case)
- Artifact stat aggregation with main stat value lookup
- Artifact set detection (4pc, 2pc+2pc)
- Partial import with warnings for unknown keys

## Usage

```rust
use genshin_calc_good::import_good;

let json = std::fs::read_to_string("good_export.json")?;
let import = import_good(&json)?;

for build in &import.builds {
    println!("{}: Lv{}", build.character.name, build.level);
}
```

## License

MIT
