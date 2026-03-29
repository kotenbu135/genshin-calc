# P2 2pc聖遺物 — BuffableStat::ElementalRes追加

## 概要

BuffableStatに`ElementalRes(Element)`バリアントを追加し、残り3セットの2pc効果をStatBuffとして実装する。Prayers系4セットは元素影響時間短縮（メカニクス）のため`buffs: &[]`維持。

## 変更ファイル

| ファイル | 変更内容 |
|---------|---------|
| `crates/core/src/buff_types.rs` | `ElementalRes(Element)` バリアント追加 |
| `crates/data/src/artifacts.rs` | 3セットの `two_piece.buffs` にStatBuff追加 |

## 詳細

### core: BuffableStat追加

`HealingBonus`/`ShieldStrength`の隣に追加:

```rust
ElementalRes(Element),
```

計算パイプライン（damage.rs, stat_profile.rs等）への影響なし。データ保持のみ。

### data: 聖遺物3セット

**Thundersoother** — `ElementalRes(Element::Electro)`, value: 0.40
**Lavawalker** — `ElementalRes(Element::Pyro)`, value: 0.40
**Tiny Miracle** — `ElementalRes` × 6元素（Pyro/Hydro/Electro/Cryo/Anemo/Geo）, value: 0.20

### スキップ: Prayers系4セット

元素影響時間短縮はステータスではなくメカニクスのため`buffs: &[]`維持。

## テスト

- BuffableStat::ElementalRes serde roundtripテスト追加
- 既存テスト全通過

## 成功基準

- 2pcカバレッジ: 45→48/52（92%）
- cargo test + clippy通過

## 実装ステップ

- [ ] `crates/core/src/buff_types.rs` に `ElementalRes(Element)` 追加
- [ ] serde roundtripテスト追加
- [ ] `crates/data/src/artifacts.rs` の Thundersoother 2pc buffs 追加
- [ ] Lavawalker 2pc buffs 追加
- [ ] Tiny Miracle 2pc buffs 追加（6元素）
- [ ] `cargo test` 全通過確認
- [ ] `cargo clippy -- -D warnings` 通過確認
- [ ] `docs/data-expansion-todo.md` カバレッジ更新
- [ ] コミット
- [ ] `docs/superpowers/specs/2026-03-29-p2-artifact-2pc-elemental-res-design.md` 削除
