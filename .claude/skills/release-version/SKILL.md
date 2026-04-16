---
name: release-version
description: genshin-calcの新バージョン公開・リリース作業を行うスキル。ユーザーが「リリース」「バージョン公開」「パッケージ公開」「パッチ公開」「パッチとしてパッケージ公開」「マイナー公開」「メジャー公開」「タグ付け」「crates.ioに公開」「npmに公開」「バージョンを上げて公開」などリリース作業に言及したとき、または `/release-version` 実行時に必ず発火させる。バージョンbump → テスト → コミット → タグ → push → GitHub Release → ワークフロー確認までの一貫フロー。
version: 1.1.0
source: local-git-analysis
analyzed_commits: 50
---

# Release Version

genshin-calcの新バージョンをリリースするためのワークフロー。crates.io（core/data/good）とnpm（wasm）への公開を自動化するGitHub Actionsと連携する。

## Usage

```
/release-version <version>
```

**例:**
```
/release-version 0.5.0
/release-version 0.4.3
```

## Trigger

以下のいずれかに該当する場合、このスキルを**必ず**発火すること。類似語・表記ゆれ（ひらがな/カタカナ/英字）も含む。

### 明示トリガー
- `/release-version` スラッシュコマンド実行時

### リリース作業への言及
- 「リリース」「release」「リリースする」「リリースして」
- 「バージョン公開」「バージョンアップ」「バージョンを上げて公開」
- 「パッケージ公開」「パッケージとして公開」「パッケージング」
- 「パッチ公開」「パッチとしてパッケージ公開」「パッチリリース」
- 「マイナー公開」「マイナーリリース」「マイナーバージョン」
- 「メジャー公開」「メジャーリリース」「メジャーバージョン」
- 「タグ付け」「タグを切る」「タグを打つ」「v0.X.Y でタグ」

### 公開先への言及
- 「crates.io に公開」「crates に公開」「cargo publish」
- 「npm に公開」「npm publish」「wasm を公開」
- 「GitHub Release 作成」「リリースノート作成」

### 暗黙トリガー（文脈で判断）
- 機能実装完了後に「公開」「出す」「リリースしたい」と言及
- CHANGELOG 更新後に「次のバージョン」「バージョン上げて」と言及
- ワークフロー失敗の再実行文脈で「リリースやり直し」「タグ移動」と言及

**重要**: 上記キーワードが含まれる場合、他の解釈より優先してこのスキルを発火させる。`/release-version <version>` が明示されていなくても、文脈から適切なバージョン（patch/minor/major）を判断して提案する。

## Prerequisites

- mainブランチに全変更がマージ済み
- `cargo test --workspace` が通ること
- `cargo clippy --workspace -- -D warnings` がクリーンなこと
- `cargo fmt --check` がクリーンなこと

## Workflow

### Step 1: バージョンバンプ

4つのCargo.tomlの`version`を一括更新する:

```bash
# 対象ファイル（全て同じバージョンにすること）
crates/core/Cargo.toml
crates/data/Cargo.toml
crates/good/Cargo.toml
crates/wasm/Cargo.toml
```

**重要**: `crates/data/Cargo.toml`のgenshin-calc-core依存バージョンと`crates/good/Cargo.toml`のgenshin-calc-core/data依存バージョンも確認。ワークスペース内では`path = "../core"`で解決されるが、crates.io公開時にはバージョン指定が使われる。

Editツールで4ファイルを更新:
```
version = "X.Y.Z"
```

### Step 2: ビルド・テスト検証

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check
```

全てパスすることを確認。失敗したら修正してから次へ。

### Step 3: コミット

```bash
git add crates/core/Cargo.toml crates/data/Cargo.toml crates/good/Cargo.toml crates/wasm/Cargo.toml Cargo.lock
git commit -m "chore: bump version to vX.Y.Z"
```

**重要**: `Cargo.lock`を忘れずにコミットすること。バージョンバンプ時にCargoが自動更新するため、コミットから漏れるとCIやcrates.io公開時に不整合が起きる。

### Step 4: タグ作成

```bash
git tag vX.Y.Z
```

**注意**: タグ名は必ず`v`プレフィクス付き（`v0.4.2`等）。GitHub Actionsのトリガーが`v*`パターン。

### Step 5: プッシュ

```bash
git push origin main --tags
```

mainブランチとタグを同時にプッシュ。

### Step 6: GitHub Release 作成

前バージョンのタグからの差分コミットを取得し、リリースノートを作成する:

```bash
# 前バージョンのタグを取得
PREV_TAG=$(git tag --sort=-v:refname | sed -n '2p')

# 差分コミットを確認
git log ${PREV_TAG}..vX.Y.Z --oneline
```

リリースノートの形式:
```markdown
## New Features
- 新機能の説明 (#issue番号)

## Bug Fixes
- バグ修正の説明 (#issue番号)

## Breaking Changes
- 破壊的変更があれば記載
```

リリースノート作成ルール:
- `chore:` コミット（バージョンバンプ、docs削除等）はリリースノートに含めない
- `feat:` → "New Features"、`fix:` → "Bug Fixes"、`refactor:` → 必要に応じて記載
- Issue/PR番号を `(#番号)` で参照する
- 日本語で記述する（ゲームの仕様用語は原文に従う）

```bash
gh release create vX.Y.Z --title "vX.Y.Z" --latest --notes "$(cat <<'EOF'
## New Features
- ...

## Bug Fixes
- ...
EOF
)" --verify-tag
```

### Step 7: ワークフロー確認

```bash
gh run list --limit 5
```

「Release」ワークフローが表示されることを確認。ステータスが`in_progress`または`completed`であればOK。

詳細確認:
```bash
gh run view <run-id>
```

## GitHub Actions パイプライン概要

`.github/workflows/release.yml`が`v*`タグのpushで起動:

1. **validate**: バージョン一致チェック + テスト + clippy + fmt
2. **publish-crates**: crates.ioへ順次公開（core → 30s待機 → data → 30s待機 → good）
3. **publish-npm**: wasm-packビルド → npm公開（`@kotenbu135`スコープ、provenance付き）

## Common Pitfalls

### バージョン不一致エラー

**症状**: Releaseワークフローのvalidateジョブが失敗
**原因**: Cargo.tomlのバージョンとgitタグのバージョンが一致しない
**対処**:
```bash
# タグを削除して修正
git tag -d vX.Y.Z
git push origin :refs/tags/vX.Y.Z
# Cargo.toml修正 → コミット → タグ再作成 → プッシュ
```

### crates.io公開順序エラー

**症状**: `publish-crates`でdata/goodの公開が失敗
**原因**: coreがまだインデックスに反映されていない
**対処**: 30秒の待機が入っているが、稀に不十分な場合はworkflow_dispatchで再実行

### ワークフローが起動しない

**症状**: `gh run list`にReleaseが表示されない
**原因**: タグがpushされていない、またはタグ名が`v`で始まっていない
**対処**:
```bash
git tag -l  # ローカルタグ確認
git ls-remote --tags origin  # リモートタグ確認
```

### dry-run モード

本番公開前にテストしたい場合:
```bash
gh workflow run release.yml --field dry_run=true
```

## Checklist

リリース前の最終チェックリスト:

- [ ] 全Cargo.tomlのバージョンが一致
- [ ] `cargo test --workspace` パス
- [ ] `cargo clippy --workspace -- -D warnings` クリーン
- [ ] `cargo fmt --check` クリーン
- [ ] `Cargo.lock`がコミットに含まれている
- [ ] コミットメッセージが `chore: bump version to vX.Y.Z` 形式
- [ ] タグが `vX.Y.Z` 形式（`v`プレフィクス必須）
- [ ] GitHub Release がリリースノート付きで作成されている
- [ ] `gh run list` でReleaseワークフロー起動確認
