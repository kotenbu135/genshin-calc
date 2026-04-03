# @kotenbu135/genshin-calc-wasm

Genshin Impact damage calculator — WASM bindings for the browser.

Built on [genshin-calc](https://crates.io/crates/genshin-calc-core), a Rust-based calculation engine.

## Install

```bash
npm install @kotenbu135/genshin-calc-wasm
```

## Usage

```js
import init, {
  game_version,
  find_character,
  calculate_damage,
  calculate_transformative,
  calculate_lunar,
  resolve_team_stats,
  import_good,
} from "@kotenbu135/genshin-calc-wasm";

// Initialize the WASM module first
await init();

// Game data version
console.log(game_version()); // "5.8"

// Look up a character
const diluc = find_character("diluc");
console.log(diluc.name); // "Diluc"

// Calculate damage
const result = calculate_damage(
  {
    character_level: 90,
    stats: {
      hp: 20000,
      atk: 2000,
      def: 800,
      elemental_mastery: 100,
      crit_rate: 0.75,
      crit_dmg: 1.5,
      energy_recharge: 1.2,
      dmg_bonus: 0.466,
    },
    talent_multiplier: 1.76,
    scaling_stat: "Atk",
    damage_type: "Skill",
    element: "Pyro",
    reaction: null,
    reaction_bonus: 0,
    flat_dmg: 0,
  },
  { level: 90, resistance: 0.1, def_reduction: 0 }
);
console.log(result); // { non_crit, crit, average, reaction }
```

## API

### Data Lookup

| Function | Description |
|---|---|
| `game_version()` | Returns the game data version string |
| `find_character(id)` | Find character by ID (e.g. `"hu_tao"`) |
| `find_weapon(id)` | Find weapon by ID (e.g. `"wolfs_gravestone"`) |
| `find_artifact_set(id)` | Find artifact set by ID (e.g. `"crimson_witch"`) |
| `find_enemy(id)` | Find enemy by ID (e.g. `"hilichurl"`) |
| `characters_by_element(el)` | List characters by element (e.g. `"pyro"`) |
| `weapons_by_type(type)` | List weapons by type (e.g. `"claymore"`) |

### Calculation

| Function | Description |
|---|---|
| `calculate_damage(input, enemy)` | Standard damage (ATK/HP/DEF scaling) |
| `calculate_transformative(input, enemy)` | Transformative reactions (overloaded, swirl, etc.) |
| `calculate_lunar(input, enemy)` | Lunar reactions (Nod-Krai crittable reactions) |
| `resolve_team_stats(members, target_index)` | Resolve team buffs into final stats |

### GOOD Format Import

| Function | Description |
|---|---|
| `import_good(json)` | Import GOOD JSON string → `GoodImport` with character builds |

```js
// Import from Genshin Optimizer, Scanner tools, etc.
const goodJson = '{"format":"GOOD","source":"GenshinOptimizer","version":2,...}';
const imported = import_good(goodJson);
console.log(imported.builds); // Array of CharacterBuild objects
console.log(imported.warnings); // Any import warnings
```

## TypeScript

Type definitions for all input/output objects are available at `types.ts`.

```ts
import type { DamageInput, Enemy, DamageResult } from "@kotenbu135/genshin-calc-wasm/types";
```

## License

MIT
