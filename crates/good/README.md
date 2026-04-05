# genshin-calc-good

GOOD (Genshin Open Object Description) format importer for [genshin-calc](https://github.com/kotenbu135/genshin-calc).

Parses GOOD JSON exported by scanners (Inventory Kamera, Genshin Calculator, etc.) and converts to `CharacterBuild` structs compatible with the genshin-calc calculation engine.

## Features

- GOOD format v1-v3 support
- Automatic key mapping (PascalCase → snake_case)
- Artifact stat aggregation with main stat value lookup
- Artifact set detection (4pc, 2pc+2pc)
- Partial import with warnings for unknown keys

## Installation

```toml
[dependencies]
genshin-calc-good = "0.2"
```

## Quick Start

```rust
use genshin_calc_good::import_good;

let json = std::fs::read_to_string("good_export.json")?;
let import = import_good(&json)?;

for build in &import.builds {
    println!("{}: Lv{}", build.character.name, build.level);
}
```

## Full Example: Build Character and Calculate Damage

```rust
use genshin_calc_core::*;
use genshin_calc_good::import_good;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = std::fs::read_to_string("good_export.json")?;
    let import = import_good(&json)?;

    for build in import.builds {
        // Skip if no weapon equipped
        let weapon = match &build.weapon {
            Some(w) => w,
            None => continue,
        };

        // Build TeamMember with full stats
        let member = genshin_calc_data::TeamMemberBuilder::new(
            build.character,
            weapon.weapon,
        )
        .character_level(build.level)
        .constellation(build.constellation)
        .talent_levels(build.talent_levels)
        .weapon_level(weapon.level)
        .artifact_sets(build.artifacts.sets.iter().copied().collect())
        .artifact_stats(build.artifacts.stats.clone())
        .build()?;

        // Get final stats after all buffs
        let stats = combine_stats(&member.stats)?;

        println!("=== {} (Lv{}) ===", build.character.name, build.level);
        println!("ATK: {:.0}", stats.atk);
        println!("HP: {:.0}", stats.hp);
        println!("CRIT: {:.1}% / {:.1}%", stats.crit_rate * 100.0, stats.crit_dmg * 100.0);

        // Calculate damage
        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };

        let damage = calculate_damage(
            &DamageInput {
                character_level: build.level,
                stats: stats.clone(),
                talent_multiplier: 1.5, // Example: 150% scaling
                scaling_stat: ScalingStat::Atk,
                damage_type: DamageType::Normal,
                element: Some(build.character.element),
                reaction: None,
                reaction_bonus: 0.0,
                flat_dmg: 0.0,
            },
            &enemy,
        )?;

        println!("Damage: {:.0} (avg)", damage.average);
    }

    Ok(())
}
```

## Workflow

1. **Export GOOD JSON** from a scanner app (Inventory Kamera, etc.)
2. **Import**: Use `import_good()` to parse JSON
3. **Build**: Create `TeamMember` using `TeamMemberBuilder`
4. **Calculate**: Use `combine_stats()` and `calculate_damage()` from core

## Output Types

### `GoodImport`
- `source`: Export source (e.g., "Irminsul")
- `version`: GOOD format version
- `builds`: `Vec<CharacterBuild>` - all imported builds
- `warnings`: Import warnings for unknown/missing data

### `CharacterBuild`
- `character`: Reference to `CharacterData` from genshin-calc-data
- `level`: Character level
- `constellation`: Number of constellations (0-6)
- `talent_levels`: [auto, skill, burst] talent levels
- `weapon`: Weapon build (if equipped)
- `artifacts`: Artifact sets and aggregated stats

### `WeaponBuild`
- `weapon`: Reference to `WeaponData`
- `level`: Weapon level (1-90)
- `refinement`: Refinement level (1-5)

### `ArtifactsBuild`
- `sets`: Detected artifact sets (4pc or 2pc+2pc combinations)
- `stats`: Aggregated main+substats as `StatProfile`

## Error Handling

```rust
use genshin_calc_good::import_good;

match import_good(&json) {
    Ok(import) => {
        // Check for warnings
        for warning in &import.warnings {
            eprintln!("Warning: {:?}", warning);
        }
        
        // Process builds
        for build in import.builds { /* ... */ }
    }
    Err(e) => {
        eprintln!("Import failed: {:?}", e);
    }
}
```

## Running the Demo

```bash
# From repository root
cargo run -p genshin-calc-good --example demo
```

## License

MIT
