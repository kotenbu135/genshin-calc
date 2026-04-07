# resolve_team_stats に resonance_activations パラメータを追加

**Issue:** #52
**Date:** 2026-04-07

## 背景

`resolve_team_stats` は4人パーティ時に元素共鳴を自動検出し、無条件バフ（炎 ATK+25%、水 HP+25%、草 EM+50）を適用している。しかし条件付き共鳴バフ（氷・岩・草追加分）はスキップされており、フロントエンドからON/OFFを制御する手段がない。

## 設計

### API変更

`resolve_team_stats` / `resolve_team_stats_detailed` に第3引数を追加:

```rust
pub fn resolve_team_stats(
    team: &[TeamMember],
    target_index: usize,
    resonance_activations: &[(ElementalResonance, bool)],
) -> Result<TeamResolveResult, CalcError>
```

- `&[]` を渡せば既存動作と完全互換（条件付き共鳴バフなし）
- `ElementalResonance` enumをキーに使うため型安全（typo不可能）
- 現在の条件付き共鳴は全てToggleなので `bool` で十分

### 条件付き共鳴バフ定義

`resonance.rs` に新関数を追加:

```rust
/// Returns conditional stat buffs for a resonance.
/// These require user activation (e.g. enemy affected by Cryo, shielded).
pub fn resonance_conditional_buffs(resonance: ElementalResonance) -> Vec<(BuffableStat, f64)> {
    match resonance {
        ElementalResonance::ShatteringIce => vec![(BuffableStat::CritRate, 0.15)],
        ElementalResonance::EnduringRock => vec![(BuffableStat::DmgBonus, 0.15)],
        ElementalResonance::SprawlingGreenery => vec![(BuffableStat::ElementalMastery, 30.0)],
        _ => vec![],
    }
}
```

既存の `resonance_buffs()` は無条件バフ用としてそのまま維持。

| 共鳴 | 無条件 (`resonance_buffs`) | 条件付き (`resonance_conditional_buffs`) |
|---|---|---|
| ShatteringIce (氷×2) | なし | CritRate +0.15 |
| EnduringRock (岩×2) | なし | DmgBonus +0.15 |
| SprawlingGreenery (草×2) | EM +50 | EM +30（反応後追加分） |
| FerventFlames (炎×2) | ATK +25% | なし |
| SoothingWater (水×2) | HP +25% | なし |
| その他 | なし | なし |

### collect_buffs の変更

`collect_buffs` が `resonance_activations` を受け取るよう変更:

```rust
fn collect_buffs(
    team: &[TeamMember],
    target_index: usize,
    resonance_activations: &[(ElementalResonance, bool)],
) -> Vec<ResolvedBuff>
```

既存の無条件バフ追加ループの後に、条件付きバフ追加ロジックを挿入:

1. 検出された共鳴を走査
2. `resonance_activations` 内で `(resonance, true)` のエントリがあるか確認
3. あれば `resonance_conditional_buffs()` の結果を `ResolvedBuff` として追加
4. source名は `"ShatteringIce(conditional)"` 等で無条件バフと区別

activationsに含まれない共鳴や `false` の共鳴は無視。検出されていない共鳴のactivationも無視（4人未満で氷共鳴をactivateしても効果なし）。

### 呼び出し箇所の修正

全ての既存呼び出しを機械的に `&[]` 付きに変更:

- `crates/core/src/team.rs` テスト
- `crates/core/src/enemy.rs` テスト
- `crates/data/tests/team_integration.rs`
- `crates/data/tests/meta_team_edge_cases.rs`
- `crates/data/tests/meta_team_verification.rs`
- `crates/wasm/src/lib.rs`

## テスト計画

1. **既存テスト**: `&[]` 追加で全パス確認（動作互換）
2. **条件付き共鳴の単体テスト**: `resonance_conditional_buffs()` の戻り値検証
3. **統合テスト**: 4人チームで各条件付き共鳴をactivate → `applied_buffs` に含まれること、`final_stats` に反映されること
4. **非activateテスト**: `&[]` や `(resonance, false)` で条件付きバフが含まれないこと
5. **4人未満テスト**: 共鳴未検出時にactivationを渡しても無視されること
