# 武器パッシブ拡充 + 精錬値統合 設計書

## 概要

武器パッシブデータの大幅拡充（P3）と精錬値システムの統合（P4）を2段階PRで同時実装する。

## 現状

- 220武器中37本のみパッシブ実装済み（17%カバレッジ）
- 武器のConditionalBuff使用: 0本（P0で型定義済みだが武器では未使用。聖遺物では3件使用中）
- TeamMemberBuilderに精錬レベルフィールドなし（TODOコメント2箇所）
- 実装済み37本は全て`refinement_values: Some([r1..r5])`を持つ

## スコープ

- **星5武器**: 未実装分全て（約25-30本）
- **人気星4武器**: 深境螺旋使用率が高い武器を手動選定（約30-40本）
- StatBuff + ConditionalBuff両方を積極的に使用
- 精錬値R1-R5を全対象武器に入力

## 実装方針: 2段階PR

### PR1: TeamMemberBuilder精錬レベル統合

#### 変更箇所

**`crates/core/src/error.rs`**:
- `CalcError::InvalidRefinement(u8)` variant追加

**`crates/data/src/team_builder.rs`**:
- `TeamMemberBuilder`に`refinement: u8`フィールド追加（デフォルト: 1）
- `.refinement(r: u8) -> Self`ビルダーメソッド追加
- `build()`でバリデーション: `r == 0 || r > 5` → `CalcError::InvalidRefinement`
- 武器パッシブStatBuff解決時: `refinement_values`がSomeなら`values[r-1]`を使用
- ConditionalBuff解決時も同様

#### 精錬値解決ロジック

```rust
fn resolve_value(value: f64, refinement_values: Option<[f64; 5]>, refinement: u8) -> f64 {
    match refinement_values {
        Some(values) => values[(refinement - 1) as usize],
        None => value,
    }
}
```

適用箇所（計6箇所）:
1. 武器パッシブStatBuff
2. 武器パッシブConditionalBuff（`cond_buff.value`を`resolve_value()`で置換してからeval関数に渡す）
3. 聖遺物2pc StatBuff
4. 聖遺物4pc StatBuff
5. 聖遺物2pc ConditionalBuff
6. 聖遺物4pc ConditionalBuff

注: 聖遺物のStatBuff/ConditionalBuffは`refinement_values: None`なので常にフォールバック。
ロジックを統一しておくことで、将来的な拡張に対応。
`refinement`パラメータは武器精錬レベルとして意味的に武器に属するが、
`resolve_value()`を全ソースに統一適用することでソース別の分岐を避ける。

#### ConditionalBuff精錬値解決の適用ポイント

`resolve_conditionals`クロージャ内で、`cond_buff.value`を各eval関数に渡す**前に**精錬値解決する:

```rust
let base_value = resolve_value(cond_buff.value, cond_buff.refinement_values, self.refinement);
// base_valueをeval_auto/eval_manualのmultiplierとして渡す
// StatScaling: stat_total * base_value（精錬済み倍率）
// Stacks: base_value * stack_count
// Both: base_valueがeval_auto→eval_manualに順に流れる
```

これにより、StatScaling/Stacks/Both全パターンで正しい精錬値が使われる。

#### PR1 テスト

- `refinement(1)`: R1の値（Aquila Favonia ATK+20%）
- `refinement(5)`: R5の値（Aquila Favonia ATK+40%）
- `refinement(3)`: 中間値
- デフォルト（未指定）→ R1と同値
- `refinement_values: None` → `value`フォールバック
- ConditionalBuffでの精錬値反映（テスト用モックデータ）
- R0 → `CalcError::InvalidRefinement(0)`
- R6 → `CalcError::InvalidRefinement(6)`
- 既存37武器の`refinement_values[0] == value`不変条件を回帰テストで検証（PR2前にデータ整合性を保証）

### PR2: 武器パッシブデータ拡張

#### 対象武器

**星5（全武器種）**:
- 剣: 磐岩結緑、霧切の廻光、萃光の裁葉、鶴鳴の余韻 等
- 両手剣: 狼の末路、松韻の響く頃、赤角石塵滅砕 等
- 長柄: 護摩の杖、草薙の稲光、赤砂の杖 等
- 弓: 飛雷の鳴弦、若水、冬極の白星 等
- 法器: 神楽の真意、四風原典、千夜浮夢 等

**人気星4**:
- 漁獲（爆発DMG+CR）、流浪楽章（Toggle系）、匣中シリーズ
- 千岩長槍/古剣、ドドコ物語、黒剣、螭龍の剣、死闘の槍
- 黒岩シリーズ、その他深境螺旋高使用率武器

#### データ入力ルール

- 全パーセンテージは小数形式（10.8% → 0.108）
- `refinement_values: Some([r1, r2, r3, r4, r5])` を必ず設定
- `value`はR1の値と一致させる
- 表現不可能な効果（CD短縮、シールド、回復等）は`description`に記載しバフは空のまま

#### ConditionalBuff使用パターン

| 武器 | 効果 | Activation型 |
|------|------|-------------|
| 護摩の杖 | HP基準ATK（常時） | `Auto(StatScaling { stat: HpPercent, cap: None })` |
| 護摩の杖 | HP<50%追加ATK | `Manual(Toggle)` |
| 霧切の廻光 | 元素ダメ+12-28%×3スタック | `Manual(Stacks(3))` |
| 磐岩結緑 | HP→ATK変換 | `Auto(StatScaling { stat: HpPercent, cap: None })` |
| 赤砂の杖 | EM基準ATK% | `Auto(StatScaling { stat: ElementalMastery, cap: None })` |
| 千岩古剣 | 璃月キャラ数依存 | `Manual(Toggle)` （Region条件未対応のためフォールバック） |
| 狼の末路 | HP<30%チームATK | `Manual(Toggle)` |

#### 対象外（StatBuff表現不可）

- 西風シリーズ — パッシブは粒子生成（ダメージ計算に無関係）
- 祭礼シリーズ — パッシブはスキルCD短縮（ダメージ計算に無関係）
- その他、回復/シールド/移動速度等の非ダメージ効果

#### PR2 テスト

**データ整合性テスト**:
- 全武器の`refinement_values`がSomeなら`values[0] == value`（R1一致保証）
- `refinement_values`の5要素が単調非減少（`<=`）（ゲーム仕様: 精錬でバフは下がらない。等間隔が一般的だがR間で同値もありうる）
- ConditionalBuff付き武器のActivation型の妥当性

**回帰確認**:
- `cargo test` 全パス
- character_verification 153ケースが壊れない

## 許容誤差

既存規約通り: 通常 `< 0.01`、ゲーム検証値 `< 1.0`

## 設計原則

- イミュータブル設計の維持
- WASM互換性の維持（`&'static`参照、動的ディスパッチ最小化）
- 数値単位: 全パーセンテージは小数形式
- core crateへの変更は`CalcError` variant追加のみ（最小限）
