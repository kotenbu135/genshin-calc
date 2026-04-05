---
name: release-version
description: genshin-calcの新バージョン公開フロー（バージョンbump → テスト → タグ → push → ワークフロー確認）
version: 1.0.0
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

- ユーザーが「リリース」「バージョン公開」「タグ付け」「push して公開」などに言及したとき
- `/release-version` スラッシュコマンド実行時

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

### Step 6: ワークフロー確認

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
- [ ] `gh run list` でReleaseワークフロー起動確認
