# 全キャラ対応 Character Verification 実装プラン

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 全プレイアブルキャラ（v5.8、102体）のTOML駆動ダメージ検証テストを完成させる

**Architecture:** 既存インフラ（test_types.rs, character_verification.rs）はそのまま使用。87体分のTOMLファイルとgenerate_expected.rsへの追記のみ。元素バッチ方式で7段階に分けて追加。

**Tech Stack:** Rust, toml 0.8, glob 0.3（既存dev-dependencies）

**Spec:** `docs/superpowers/specs/2026-03-26-full-character-coverage-design.md`

---

## 共通パターン（全バッチ共通）

各バッチは以下の手順を踏む:

1. **Webリサーチ**: 各キャラの代表天賦の倍率をKQM/Genshin Wiki/Honey Impactから取得
2. **generate_expected.rs追記**: キャラごとの `#[test] #[ignore] fn generate_<name>()` 関数を追加
3. **期待値生成**: `cargo test -p genshin-calc-core --test generate_expected -- --nocapture --ignored` で実行
4. **TOMLファイル作成**: 期待値を使って `crates/core/tests/data/characters/<name>.toml` を作成
5. **テスト実行**: `cargo test -p genshin-calc-core --test character_verification -- --nocapture` で全パス確認
6. **コミット**

### TOML作成ルール

- **通常ケース**: `type = "normal"`, 代表天賦の通常ダメージ
- **反応ケース**: 元素に応じた反応テスト
  - Pyro → `type = "amplifying"`, reaction = "Vaporize" (1.5x) or "Melt" (2.0x)
  - Hydro → `type = "amplifying"`, reaction = "Vaporize" (2.0x)
  - Cryo → `type = "amplifying"`, reaction = "Melt" (1.5x)
  - Electro → `type = "catalyze"`, reaction = "Aggravate"
  - Dendro → `type = "catalyze"`, reaction = "Spread"
  - Anemo/Geo → 通常のみ（反応テストはKazuhaでカバー済み）
- **反応ケースのEM**: 100-200程度の現実的な値を設定
- **tolerance**: 反応ケースは `tolerance = 0.1`

### ステータス設定基準

| カテゴリ | ATK | HP | DEF | CR | CD | DMG% | EM |
|---------|-----|----|----|----|----|------|----|
| ATK DPS (5★) | 1800-2200 | - | - | 0.60-0.70 | 1.20-1.80 | 0.466 | 0 |
| ATK DPS (4★) | 1400-1700 | - | - | 0.55-0.65 | 1.00-1.40 | 0.466 | 0 |
| HP DPS | - | 30000-60000 | - | 0.50-0.70 | 1.00-2.00 | 0.466 | 0 |
| DEF DPS | - | - | 2000-2500 | 0.60-0.70 | 1.50-1.80 | 0.466 | 0 |
| サポート | 1200-1500 | - | - | 0.50 | 1.00 | 0.466 | 0 |
| EM特化 | 800-1000 | - | - | 0.30 | 0.60 | 0.15 | 800 |

### 既存ファイル参考

- `crates/core/tests/data/characters/diluc.toml` — ATK DPS + 蒸発/溶解の参考
- `crates/core/tests/data/characters/yelan.toml` — HP scaling の参考
- `crates/core/tests/data/characters/itto.toml` — DEF scaling の参考
- `crates/core/tests/data/characters/kazuha.toml` — Transformative(Swirl)の参考
- `crates/core/tests/data/characters/nahida.toml` — Catalyze(Spread)の参考

---

## Task 1: Batch 1 — Pyro（14体）

**Files:**
- Modify: `crates/core/tests/generate_expected.rs`
- Create: `crates/core/tests/data/characters/{amber,arlecchino,bennett,chevreuse,dehya,gaming,klee,lyney,mavuika,thoma,xiangling,xinyan,yanfei,yoimiya}.toml`

**キャラデータ（Webリサーチで天賦倍率を取得すること）:**

| キャラ | 武器 | ★ | 代表天賦 | DamageType | ScalingStat | 反応テスト |
|--------|------|---|---------|------------|-------------|-----------|
| Amber | Bow | 4 | Fiery Rain (Burst) | Burst | Atk | Vaporize |
| Arlecchino | Polearm | 5 | Normal (Blood-Debt) | Normal | Atk | Vaporize |
| Bennett | Sword | 4 | Passion Overload (Skill) | Skill | Atk | Melt |
| Chevreuse | Polearm | 4 | Short-Range Rapid Fire (Skill) | Skill | Hp | — |
| Dehya | Claymore | 5 | Kick (Skill coord) | Skill | Atk | Vaporize |
| Gaming | Claymore | 4 | Suanni Plunge (Skill) | Plunging | Atk | Vaporize |
| Klee | Catalyst | 5 | Charged Attack | Charged | Atk | Vaporize |
| Lyney | Bow | 5 | Prop Arrow (Charged) | Charged | Atk | Vaporize |
| Mavuika | Claymore | 5 | Flamestrider Normal | Normal | Atk | Vaporize |
| Thoma | Polearm | 4 | Blazing Blessing (Burst coord) | Burst | Atk | — |
| Xiangling | Polearm | 4 | Pyronado (Burst) | Burst | Atk | Vaporize |
| Xinyan | Claymore | 4 | Riff Revolution (Burst) | Burst | Atk | — |
| Yanfei | Catalyst | 4 | Charged Attack | Charged | Atk | Vaporize |
| Yoimiya | Bow | 5 | Niwabi Fire-Dance Normal | Normal | Atk | Vaporize |

- [ ] **Step 1: Webリサーチで各キャラの天賦Lv10倍率を取得**
- [ ] **Step 2: generate_expected.rsに14体分のgenerate関数を追加**
- [ ] **Step 3: 期待値生成を実行**
- [ ] **Step 4: 14体分のTOMLファイル作成**
- [ ] **Step 5: テスト実行確認**

Run: `cargo test -p genshin-calc-core --test character_verification -- --nocapture`
Expected: `29 characters, XX cases — all passed`

- [ ] **Step 6: コミット**

```bash
git add crates/core/tests/generate_expected.rs crates/core/tests/data/characters/
git commit -m "test: add Pyro character verification data (14 characters)"
```

---

## Task 2: Batch 2 — Hydro（11体）

**Files:**
- Modify: `crates/core/tests/generate_expected.rs`
- Create: `crates/core/tests/data/characters/{barbara,candace,dahlia,furina,kokomi,mona,mualani,neuvillette,sigewinne,tartaglia,xingqiu}.toml`

**キャラデータ:**

| キャラ | 武器 | ★ | 代表天賦 | DamageType | ScalingStat | 反応テスト |
|--------|------|---|---------|------------|-------------|-----------|
| Barbara | Catalyst | 4 | Normal Attack | Normal | Atk | — |
| Candace | Polearm | 4 | Sacred Rite (Burst) | Burst | Hp | — |
| Dahlia | Sword | 4 | Skill | Skill | Atk | Vaporize |
| Furina | Sword | 5 | Salon Solitaire (Skill) | Skill | Hp | — |
| Kokomi | Catalyst | 5 | Normal (Burst mode) | Normal | Hp | — |
| Mona | Catalyst | 5 | Stellaris Phantasm (Burst) | Burst | Atk | Vaporize |
| Mualani | Catalyst | 5 | Sharky Surge (Normal) | Normal | Hp | Vaporize |
| Neuvillette | Catalyst | 5 | Charged Attack | Charged | Hp | Vaporize |
| Sigewinne | Bow | 5 | Bolstering Bubblebalm (Skill) | Skill | Hp | Vaporize |
| Tartaglia | Bow | 5 | Melee Normal | Normal | Atk | Vaporize |
| Xingqiu | Sword | 4 | Raincutter (Burst) | Burst | Atk | Vaporize |

- [ ] **Step 1-5: 共通パターンに従って実行**

Expected: `40 characters, XX cases — all passed`

- [ ] **Step 6: コミット**

```bash
git commit -m "test: add Hydro character verification data (11 characters)"
```

---

## Task 3: Batch 3 — Cryo（16体）

**Files:**
- Modify: `crates/core/tests/generate_expected.rs`
- Create: `crates/core/tests/data/characters/{aloy,ayaka,charlotte,chongyun,citlali,diona,escoffier,eula,kaeya,layla,mika,qiqi,rosaria,shenhe,skirk,wriothesley}.toml`

**キャラデータ:**

| キャラ | 武器 | ★ | 代表天賦 | DamageType | ScalingStat | 反応テスト |
|--------|------|---|---------|------------|-------------|-----------|
| Aloy | Bow | 5 | Freeze Bomb (Skill) | Skill | Atk | Melt |
| Ayaka | Sword | 5 | Soumetsu (Burst) | Burst | Atk | Melt |
| Charlotte | Catalyst | 4 | Skill | Skill | Atk | — |
| Chongyun | Claymore | 4 | Spirit Blade (Burst) | Burst | Atk | Melt |
| Citlali | Catalyst | 5 | Itzpapa Skill | Skill | Hp | Melt |
| Diona | Bow | 4 | Icy Paws (Skill) | Skill | Atk | — |
| Escoffier | Polearm | 5 | Skill | Skill | Atk | Melt |
| Eula | Claymore | 5 | Lightfall Sword (Burst) | Burst | Atk | — |
| Kaeya | Sword | 4 | Glacial Waltz (Burst) | Burst | Atk | Melt |
| Layla | Sword | 4 | Skill (shield) | Skill | Hp | — |
| Mika | Polearm | 4 | Skill | Skill | Atk | — |
| Qiqi | Sword | 5 | Normal Attack | Normal | Atk | — |
| Rosaria | Polearm | 4 | Rites of Termination (Burst) | Burst | Atk | Melt |
| Shenhe | Polearm | 5 | Divine Maiden (Skill, press) | Skill | Atk | — |
| Skirk | Sword | 5 | Skill | Skill | Atk | Melt |
| Wriothesley | Catalyst | 5 | Normal Attack | Normal | Atk | Melt |

注: Eulaは物理DPS。Burstは物理ダメージ（element=None）で通常ダメージのみ。

- [ ] **Step 1-5: 共通パターンに従って実行**

Expected: `56 characters, XX cases — all passed`

- [ ] **Step 6: コミット**

```bash
git commit -m "test: add Cryo character verification data (16 characters)"
```

---

## Task 4: Batch 4 — Electro（14体）

**Files:**
- Modify: `crates/core/tests/generate_expected.rs`
- Create: `crates/core/tests/data/characters/{beidou,clorinde,cyno,dori,iansan,ineffa,keqing,kuki_shinobu,lisa,ororon,razor,kujou_sara,sethos,varesa}.toml`

**キャラデータ:**

| キャラ | 武器 | ★ | 代表天賦 | DamageType | ScalingStat | 反応テスト |
|--------|------|---|---------|------------|-------------|-----------|
| Beidou | Claymore | 4 | Stormbreaker (Burst) | Burst | Atk | Aggravate |
| Clorinde | Sword | 5 | Pistol Normal (Skill) | Normal | Atk | Aggravate |
| Cyno | Polearm | 5 | Burst Normal | Normal | Atk | Aggravate |
| Dori | Claymore | 4 | Skill | Skill | Atk | — |
| Iansan | Polearm | 4 | Skill | Skill | Atk | Aggravate |
| Ineffa | Polearm | 5 | Skill | Skill | Atk | Aggravate |
| Keqing | Sword | 5 | Charged Attack | Charged | Atk | Aggravate |
| Kuki Shinobu | Sword | 4 | Skill | Skill | Hp | — |
| Lisa | Catalyst | 4 | Violet Arc Hold (Skill) | Skill | Atk | Aggravate |
| Ororon | Bow | 4 | Skill | Skill | Atk | Aggravate |
| Razor | Claymore | 4 | Normal (Burst mode) | Normal | Atk | — |
| Kujou Sara | Bow | 4 | Charged (Crowfeather) | Charged | Atk | Aggravate |
| Sethos | Bow | 4 | Charged Attack | Charged | Atk | Aggravate |
| Varesa | Catalyst | 5 | Skill | Skill | Atk | Aggravate |

- [ ] **Step 1-5: 共通パターンに従って実行**

Expected: `70 characters, XX cases — all passed`

- [ ] **Step 6: コミット**

```bash
git commit -m "test: add Electro character verification data (14 characters)"
```

---

## Task 5: Batch 5 — Dendro（8体）

**Files:**
- Modify: `crates/core/tests/generate_expected.rs`
- Create: `crates/core/tests/data/characters/{alhaitham,baizhu,collei,emilie,kaveh,kinich,kirara,yaoyao}.toml`

**キャラデータ:**

| キャラ | 武器 | ★ | 代表天賦 | DamageType | ScalingStat | 反応テスト |
|--------|------|---|---------|------------|-------------|-----------|
| Alhaitham | Sword | 5 | Projection Attack (Skill) | Skill | Atk | Spread |
| Baizhu | Catalyst | 5 | Skill | Skill | Atk | Spread |
| Collei | Bow | 4 | Skill | Skill | Atk | Spread |
| Emilie | Polearm | 5 | Skill | Skill | Atk | Spread |
| Kaveh | Claymore | 4 | Skill | Skill | Atk | Spread |
| Kinich | Claymore | 5 | Skill | Skill | Atk | Spread |
| Kirara | Sword | 4 | Skill | Skill | Hp | — |
| Yaoyao | Polearm | 4 | Skill | Skill | Atk | — |

- [ ] **Step 1-5: 共通パターンに従って実行**

Expected: `78 characters, XX cases — all passed`

- [ ] **Step 6: コミット**

```bash
git commit -m "test: add Dendro character verification data (8 characters)"
```

---

## Task 6: Batch 6 — Anemo（13体）

**Files:**
- Modify: `crates/core/tests/generate_expected.rs`
- Create: `crates/core/tests/data/characters/{chasca,faruzan,heizou,ifa,jean,lan_yan,lynette,sayu,sucrose,venti,wanderer,xianyun,mizuki}.toml`

**キャラデータ:**

| キャラ | 武器 | ★ | 代表天賦 | DamageType | ScalingStat | 反応テスト |
|--------|------|---|---------|------------|-------------|-----------|
| Chasca | Bow | 5 | Multitarget Fire (Skill) | Skill | Atk | — |
| Faruzan | Bow | 4 | Charged (Pressurized Collapse) | Charged | Atk | — |
| Heizou | Catalyst | 4 | Heartstopper Strike (Skill) | Skill | Atk | — |
| Ifa | Catalyst | 4 | Skill | Skill | Atk | — |
| Jean | Sword | 5 | Gale Blade (Skill) | Skill | Atk | — |
| Lan Yan | Catalyst | 4 | Skill | Skill | Atk | — |
| Lynette | Sword | 4 | Skill | Skill | Atk | — |
| Sayu | Claymore | 4 | Yoohoo Art (Burst) | Burst | Atk | — |
| Sucrose | Catalyst | 4 | Astable Anemohypostasis (Skill) | Skill | Atk | — |
| Venti | Bow | 5 | Skyward Sonnet (Skill) | Skill | Atk | — |
| Wanderer | Catalyst | 5 | Normal (Windfavored) | Normal | Atk | — |
| Xianyun | Catalyst | 5 | Skill | Skill | Atk | — |
| Mizuki | Catalyst | 5 | Skill | Skill | Hp | — |

Anemoは通常ダメージのみ（1ケース/キャラ）。拡散テストはKazuhaでカバー済み。

- [ ] **Step 1-5: 共通パターンに従って実行**

Expected: `91 characters, XX cases — all passed`

- [ ] **Step 6: コミット**

```bash
git commit -m "test: add Anemo character verification data (13 characters)"
```

---

## Task 7: Batch 7 — Geo（10体）+ 旅人

**Files:**
- Modify: `crates/core/tests/generate_expected.rs`
- Create: `crates/core/tests/data/characters/{albedo,chiori,gorou,kachina,navia,ningguang,noelle,xilonen,yun_jin,zhongli,traveler}.toml`

**キャラデータ:**

| キャラ | 武器 | ★ | 代表天賦 | DamageType | ScalingStat | 反応テスト |
|--------|------|---|---------|------------|-------------|-----------|
| Albedo | Sword | 5 | Transient Blossom (Skill) | Skill | Def | — |
| Chiori | Sword | 5 | Tamoto (Skill) | Skill | Atk | — |
| Gorou | Bow | 4 | Skill | Skill | Atk | — |
| Kachina | Polearm | 4 | Turbo Twirly (Skill) | Skill | Def | — |
| Navia | Claymore | 5 | Skill (Rosula Shardshots) | Skill | Atk | — |
| Ningguang | Catalyst | 4 | Star Shatter (Burst) | Burst | Atk | — |
| Noelle | Claymore | 4 | Normal (Burst mode) | Normal | Def | — |
| Xilonen | Sword | 5 | Normal Attack | Normal | Def | — |
| Yun Jin | Polearm | 4 | Opening Flourish (Skill) | Skill | Def | — |
| Zhongli | Polearm | 5 | Planet Befall (Burst) | Burst | Atk | — |
| Traveler (Dendro) | Sword | 5 | Razorgrass Blade (Skill) | Skill | Atk | Spread |

Geoは反応テストなし（1ケース/キャラ）。旅人はDendro版でSpreadテスト付き。

- [ ] **Step 1-5: 共通パターンに従って実行**

Expected: `102 characters, ~181 cases — all passed`

- [ ] **Step 6: コミット**

```bash
git commit -m "test: add Geo + Traveler character verification data (11 characters)"
```

---

## Task 8: 最終検証 + CLAUDE.md更新

- [ ] **Step 1: 全テスト実行**

Run: `cargo test -p genshin-calc-core`
Expected: 127 unit tests + 1 integration test (~181 cases) = all pass

- [ ] **Step 2: clippy + fmt確認**

Run: `cargo clippy -p genshin-calc-core -- -D warnings && cargo fmt -p genshin-calc-core --check`

- [ ] **Step 3: CLAUDE.md更新**

Testingセクションの「15キャラ・31ケース」を最終数に更新。

- [ ] **Step 4: コミット**

```bash
git commit -m "test: complete full character verification (102 chars, ~181 cases)"
```
