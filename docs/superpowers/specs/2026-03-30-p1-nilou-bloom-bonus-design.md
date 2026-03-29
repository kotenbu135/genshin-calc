# P1 Nilou A2 開花ボーナス — TalentBuffDef追加

## 概要

Nilou A2「夢蓮の舞」をTalentBuffDefとして追加する。Kazuha A4と同じ`base_value: 0.0`パターンで、HP依存の値はconsumer側で計算する。スキーマ変更なし。

## 変更ファイル

| ファイル | 変更内容 |
|---------|---------|
| `crates/data/src/talent_buffs.rs` | NILOU_BUFFS定義 + ALL_TALENT_BUFFSに追加 |
| `docs/data-expansion-todo.md` | Nilou完了マーク |

## 設計詳細

### TalentBuffDef定義

```rust
// ===== Nilou =====
// A2 passive "Dreaming Dance of the Lotuslight":
// For every 1000 HP above 30000, Bloom DMG +9%, max +400%
// Formula: min(floor((total_hp - 30000) / 1000) * 0.09, 4.0)
static NILOU_BUFFS: &[TalentBuffDef] = &[TalentBuffDef {
    name: "Dreaming Dance of the Lotuslight",
    description: "HP30000超過1000ごとにBloom DMG+9%、最大+400%",
    stat: BuffableStat::TransformativeBonus,
    base_value: 0.0, // HP-dependent — consumer computes value
    scales_with_talent: false,
    talent_scaling: None,
    scales_on: None,
    target: BuffTarget::OnlySelf,
    source: TalentBuffSource::AscensionPassive,
    min_constellation: 0,
}];
```

ALL_TALENT_BUFFSに`("nilou", NILOU_BUFFS)`を追加。

### A1「金盞花の宮」の扱い

A1はBloom→BountifulBloomへの変換（チーム水草限定）。ダメージ計算式はBloomと同一（2.0倍率の変態反応）であり、計算上の影響はない。BountifulBloomの有無はUI/フロントエンド側の関心事であるため、TalentBuffDefとしてはモデル化しない。

### base_value: 0.0 パターン

Kazuha A4（EM×0.04%のDmgBonus）と同じパターン。EM/HP等のステータス依存値はconsumer側で計算し、ResolvedBuffのvalueを上書きまたは別途反映する。

消費者向けの計算式:
```
bonus = min(floor((total_hp - 30000.0).max(0.0) / 1000.0) * 0.09, 4.0)
```

## テスト

- `find_talent_buffs("nilou")`が1定義を返すことを確認
- 返されたTalentBuffDefのstat, base_value, source等のフィールド検証
- serde roundtrip
- 既存テスト全通過（`cargo test` + `cargo clippy -- -D warnings`）

## 実装ステップ

- [ ] `crates/data/src/talent_buffs.rs` にNILOU_BUFFS定義追加
- [ ] ALL_TALENT_BUFFSに`("nilou", NILOU_BUFFS)`追加
- [ ] テスト追加（find_talent_buffs + フィールド検証）
- [ ] `cargo test` 全通過確認
- [ ] `cargo clippy -- -D warnings` 通過確認
- [ ] `docs/data-expansion-todo.md` のNilou行にチェックマーク
- [ ] コミット
- [ ] `docs/superpowers/specs/2026-03-30-p1-nilou-bloom-bonus-design.md` 削除
