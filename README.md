# genshin-calc

原神（Genshin Impact）のダメージ計算エンジン。Rust製ライブラリ。

## Features

- 単体キャラのダメージ計算（基礎ダメージ、会心、防御補正、耐性補正）
- 入力バリデーション付き
- serde対応（JSON等でのシリアライズ/デシリアライズ）

## Usage

```rust
use genshin_calc_core::{calculate_damage, DamageInput, DamageType, Stats, Enemy, Element};

let stats = Stats {
    atk: 2000.0,
    crit_rate: 0.75,
    crit_dmg: 1.50,
    dmg_bonus: 0.466,
    ..Stats::default()
};

let input = DamageInput {
    character_level: 90,
    stats,
    talent_multiplier: 1.76,
    damage_type: DamageType::Skill,
    element: Some(Element::Pyro),
};

let enemy = Enemy {
    level: 90,
    resistance: 0.10,
    def_reduction: 0.0,
};

let result = calculate_damage(&input, &enemy).unwrap();
println!("Non-crit: {:.1}", result.non_crit);
println!("Crit: {:.1}", result.crit);
println!("Average: {:.1}", result.average);
```

## Development

```bash
cargo build          # ビルド
cargo test           # テスト実行
cargo clippy         # lint
cargo fmt --check    # フォーマット確認
```

## License

MIT
