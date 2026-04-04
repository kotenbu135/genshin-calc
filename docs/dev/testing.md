# Testing Guide

## Structure
- core: ユニットテスト + 統合テスト2種（character_verification + moonsign_integration）
- data: 検索API、serde roundtrip、データ整合性、core統合、チーム統合、聖遺物・武器ConditionalBuff
- v0.3.0でStatProfile合算 + ScalingStatテスト追加

## Verified Characters
ゲーム検証済み: Freminet（完全一致）、Diluc、Ganyu、Raiden、Yanfei蒸発

## Golden Tests
手計算値との照合（各モジュールに `test_golden_*` テスト）

## Data-Driven Tests
- `tests/data/characters/*.toml`（v6.4全キャラ対応）
- `cargo test --test character_verification` で実行
- 新キャラ追加: TOMLファイル1つ追加するだけ

## Tolerance
- 通常: `< 0.01`
- ゲーム検証: `< 1.0`（floor丸め考慮）
