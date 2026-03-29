# genshin-calc

原神（Genshin Impact）のダメージ・元素反応計算エンジン。Rust製ライブラリ。

## Features

- 単体キャラのダメージ計算（基礎ダメージ、会心、防御補正、耐性補正）
- 元素反応ダメージ計算
  - 増幅反応（蒸発・溶解）
  - 激化反応（超激化・草激化）
  - 固定反応（過負荷・超電導・感電・氷砕き・拡散・開花・超開花・烈開花・燃焼）
  - 月反応（月感電・月開花・月結晶）
- 元素熟知ボーナス計算
- レベル基礎値テーブル（Lv1〜100、データマイニング値）
- 入力バリデーション付き
- serde対応（JSON等でのシリアライズ/デシリアライズ）

## Usage

### ダメージ計算（反応なし）

```rust
use genshin_calc_core::{calculate_damage, DamageInput, DamageType, Stats, Enemy, Element};

let input = DamageInput {
    character_level: 90,
    stats: Stats {
        atk: 2000.0,
        crit_rate: 0.75,
        crit_dmg: 1.50,
        dmg_bonus: 0.466,
        ..Stats::default()
    },
    talent_multiplier: 1.76,
    damage_type: DamageType::Skill,
    element: Some(Element::Pyro),
    reaction: None,
    reaction_bonus: 0.0,
};

let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
let result = calculate_damage(&input, &enemy).unwrap();
println!("Non-crit: {:.1}", result.non_crit);
println!("Crit: {:.1}", result.crit);
println!("Average: {:.1}", result.average);
```

### 増幅反応（蒸発）

```rust
use genshin_calc_core::{calculate_damage, DamageInput, DamageType, Stats, Enemy, Element, Reaction};

let input = DamageInput {
    character_level: 90,
    stats: Stats {
        atk: 1800.0,
        crit_rate: 0.6,
        crit_dmg: 1.2,
        dmg_bonus: 0.466,
        elemental_mastery: 200.0,
        ..Stats::default()
    },
    talent_multiplier: 1.5104,
    damage_type: DamageType::Skill,
    element: Some(Element::Pyro),
    reaction: Some(Reaction::Vaporize),   // 蒸発（Pyro trigger = 1.5x）
    reaction_bonus: 0.15,                 // 魔女4セット
};

let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
let result = calculate_damage(&input, &enemy).unwrap();
```

### 固定反応（過負荷）

```rust
use genshin_calc_core::{calculate_transformative, TransformativeInput, Enemy, Reaction};

let input = TransformativeInput {
    character_level: 90,
    elemental_mastery: 800.0,
    reaction: Reaction::Overloaded,
    reaction_bonus: 0.0,
};

let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
let result = calculate_transformative(&input, &enemy).unwrap();
println!("Overloaded damage: {:.1}", result.damage);
```

### 月反応（月感電）

```rust
use genshin_calc_core::{calculate_lunar, LunarInput, Enemy, Reaction};

let input = LunarInput {
    character_level: 90,
    elemental_mastery: 300.0,
    reaction: Reaction::LunarElectroCharged,
    reaction_bonus: 0.0,
    crit_rate: 0.5,
    crit_dmg: 1.0,
};

let enemy = Enemy { level: 90, resistance: 0.10, def_reduction: 0.0 };
let result = calculate_lunar(&input, &enemy).unwrap();
println!("Non-crit: {:.1}, Crit: {:.1}", result.non_crit, result.crit);
```

## API Overview

| 関数 | 用途 | 反応タイプ |
|------|------|-----------|
| `calculate_damage` | 通常ダメージ + 増幅/激化 | Vaporize, Melt, Aggravate, Spread |
| `calculate_transformative` | 固定反応ダメージ | Overloaded, Superconduct, Swirl, Bloom, etc. |
| `calculate_lunar` | 月反応ダメージ | LunarElectroCharged, LunarBloom, LunarCrystallize |

## Development

```bash
cargo build          # ビルド
cargo test           # テスト実行（95件）
cargo clippy         # lint
cargo fmt --check    # フォーマット確認
```

## License

MIT
