# Character Data Addition Guide

## Patterns
- デュアルスケーリング（ATK+EM等）: `_ATK` と `_EM` の2エントリに分割（Lauma/Nefer パターン）
- Catalyst通常攻撃: `damage_element: Some(Element::Xxx)` 必須
- 月兆タレント強化: `MoonsignTalentEffect` の `GrantReactionCrit` / `StatBuff` / `ReactionDmgBonus` から選択
- TOML テスト: 手計算で expected values を算出。Spread 等の catalyze は `damage.rs` の計算式を確認

## Related Skills
- `.claude/skills/game-update-todo/`: ゲーム更新時の実装TODOリスト生成
- `.claude/skills/honeyhunter-data-fetch/`: Honey Impactからゲームデータ取得・Rust定数変換
