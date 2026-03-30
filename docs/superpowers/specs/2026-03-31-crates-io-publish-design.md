# crates.io 公開準備 — 設計仕様

## 概要

`genshin-calc-core` と `genshin-calc-data` を 0.1.0 で crates.io に同時公開する。

## 前提条件

- crates.io アカウント作成済み
- `cargo login <API_TOKEN>` 実行済み（`~/.cargo/credentials.toml` にトークン設定）
- crate名 `genshin-calc-core` / `genshin-calc-data` が未使用であること（確認済み）

## 注意事項

- `edition = "2024"` + `rust-version = "1.85"` を使用。ユーザー層は限定されるが、初期リリースなので問題なし
- `categories` スラグは crates.io/category_slugs で有効性を確認すること

## スコープ

### 作業項目

#### 1. CHANGELOG.md 追加
- ルートに `CHANGELOG.md` を作成（[Keep a Changelog](https://keepachangelog.com/) 形式）
- 0.1.0 の初回エントリ：主要機能の要約

#### 2. Doc comments 充実
- 全公開API（pub fn, pub struct, pub enum）に doc comment を付与
- 主要な関数には `# Examples` セクション付き（`cargo test --doc` でテストされる）
- 各crateのlib.rsに `#![deny(missing_docs)]` を追加して漏れを防止

#### 3. `cargo publish --dry-run` で検証
- 両crateでパッケージング問題を事前検出
- 不要ファイルが含まれないか確認
- 必要に応じて `exclude` フィールドを Cargo.toml に追加
- `categories` スラグの有効性を確認

#### 4. Git tag
- `v0.1.0` タグを作成（ソースコードとcrateバージョンの対応付け）

#### 5. 公開実行
- `cargo publish -p genshin-calc-core` → 成功を確認
- **インデックス反映を待つ**: `cargo search genshin-calc-core` で 0.1.0 が表示されるまで待機（1〜5分）
- `genshin-calc-data` の Cargo.toml で core への path 依存に `version = "0.1.0"` が既にあることを確認
- `cargo publish -p genshin-calc-data`

### 公開順序の制約

```
genshin-calc-core (publish 1st)
       ↑
  wait for index propagation (~1-5 min)
       ↑
genshin-calc-data (publish 2nd, depends on core)
```

### やらないこと
- CI/GitHub Actions
- 1.0.0 への bump
- crate名変更
