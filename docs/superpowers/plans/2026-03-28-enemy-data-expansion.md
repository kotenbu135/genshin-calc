# P5: 敵データ拡充 実装プラン

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 既存パターンで25体の敵データ（週ボス・フィールドボス・精鋭）と13の耐性テンプレートを追加する

**Architecture:** `crates/data/src/enemies.rs` のみ変更。既存の `ResistanceTemplate` const + `EnemyData` const パターンに従い、データ定数を追加する。API変更なし。

**Tech Stack:** Rust, genshin-calc-data crate

**Spec:** `docs/superpowers/specs/2026-03-28-enemy-data-expansion-design.md`

---

### Task 1: 耐性テンプレート追加（13個）+ テスト

**Files:**
- Modify: `crates/data/src/enemies.rs:161` (既存テンプレートの後に追加)

- [ ] **Step 1: テスト追加 — 新テンプレートの耐性値検証**

`crates/data/src/enemies.rs` の `mod tests` 内に以下を追加:

```rust
    #[test]
    fn test_hydro_50_rest_0_template() {
        let eps = 1e-10;
        assert!((HYDRO_50_REST_0.hydro - 0.50).abs() < eps);
        assert!((HYDRO_50_REST_0.physical - 0.0).abs() < eps);
        assert!((HYDRO_50_REST_0.pyro - 0.0).abs() < eps);
    }

    #[test]
    fn test_hydro_70_electro_70_rest_0_template() {
        let eps = 1e-10;
        assert!((HYDRO_70_ELECTRO_70_REST_0.hydro - 0.70).abs() < eps);
        assert!((HYDRO_70_ELECTRO_70_REST_0.electro - 0.70).abs() < eps);
        assert!((HYDRO_70_ELECTRO_70_REST_0.physical - 0.0).abs() < eps);
        assert!((HYDRO_70_ELECTRO_70_REST_0.pyro - 0.0).abs() < eps);
    }

    #[test]
    fn test_geo_70_phys_40_rest_10_template() {
        let eps = 1e-10;
        assert!((GEO_70_PHYS_40_REST_10.geo - 0.70).abs() < eps);
        assert!((GEO_70_PHYS_40_REST_10.physical - 0.40).abs() < eps);
        assert!((GEO_70_PHYS_40_REST_10.pyro - 0.10).abs() < eps);
    }

    #[test]
    fn test_pyro_70_cryo_50_rest_10_template() {
        let eps = 1e-10;
        assert!((PYRO_70_CRYO_50_REST_10.pyro - 0.70).abs() < eps);
        assert!((PYRO_70_CRYO_50_REST_10.cryo - 0.50).abs() < eps);
        assert!((PYRO_70_CRYO_50_REST_10.hydro - 0.10).abs() < eps);
    }

    #[test]
    fn test_electro_90_rest_30_template() {
        let eps = 1e-10;
        assert!((ELECTRO_90_REST_30.electro - 0.90).abs() < eps);
        assert!((ELECTRO_90_REST_30.physical - 0.30).abs() < eps);
        assert!((ELECTRO_90_REST_30.pyro - 0.30).abs() < eps);
    }

    #[test]
    fn test_immune_templates() {
        let eps = 1e-10;
        // Immune = 10.0 (1000% resistance)
        assert!((CRYO_IMMUNE_PHYS_0_REST_10.cryo - 10.0).abs() < eps);
        assert!((CRYO_IMMUNE_PHYS_0_REST_10.physical - 0.0).abs() < eps);
        assert!((CRYO_IMMUNE_PHYS_0_REST_10.pyro - 0.10).abs() < eps);

        assert!((ELECTRO_IMMUNE_PHYS_0_REST_10.electro - 10.0).abs() < eps);
        assert!((ELECTRO_IMMUNE_PHYS_0_REST_10.physical - 0.0).abs() < eps);

        assert!((DENDRO_IMMUNE_PHYS_0_REST_10.dendro - 10.0).abs() < eps);
        assert!((DENDRO_IMMUNE_PHYS_0_REST_10.physical - 0.0).abs() < eps);
    }
```

- [ ] **Step 2: テスト実行 — FAILを確認**

Run: `cargo test -p genshin-calc-data -- test_hydro_50_rest_0`
Expected: FAIL — `HYDRO_50_REST_0` が未定義

- [ ] **Step 3: 13テンプレートを実装**

`crates/data/src/enemies.rs` の `ANEMO_70_REST_10` 定義（L151-161）の後に追加:

```rust
// ---------------------------------------------------------------------------
// 追加テンプレート: 週ボス・フィールドボス用
// ---------------------------------------------------------------------------

/// 水50% 他0% (タルタリヤ・第1段階)
pub const HYDRO_50_REST_0: ResistanceTemplate = ResistanceTemplate {
    name: "水50% 他0%",
    pyro: 0.0,
    hydro: 0.50,
    electro: 0.0,
    cryo: 0.0,
    dendro: 0.0,
    anemo: 0.0,
    geo: 0.0,
    physical: 0.0,
};

/// 雷50% 他0% (タルタリヤ・第2段階)
pub const ELECTRO_50_REST_0: ResistanceTemplate = ResistanceTemplate {
    name: "雷50% 他0%",
    pyro: 0.0,
    hydro: 0.0,
    electro: 0.50,
    cryo: 0.0,
    dendro: 0.0,
    anemo: 0.0,
    geo: 0.0,
    physical: 0.0,
};

/// 水70% 雷70% 他0% (タルタリヤ・第3段階)
pub const HYDRO_70_ELECTRO_70_REST_0: ResistanceTemplate = ResistanceTemplate {
    name: "水70% 雷70% 他0%",
    pyro: 0.0,
    hydro: 0.70,
    electro: 0.70,
    cryo: 0.0,
    dendro: 0.0,
    anemo: 0.0,
    geo: 0.0,
    physical: 0.0,
};

/// 岩70% 物理40% 他10% (若陀龍王)
pub const GEO_70_PHYS_40_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "岩70% 物理40% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.70,
    physical: 0.40,
};

/// 氷50% 他10% (シニョーラ・第1段階)
pub const CRYO_50_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "氷50% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.50,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 炎70% 氷50% 他10% (シニョーラ・第2段階)
pub const PYRO_70_CRYO_50_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "炎70% 氷50% 他10%",
    pyro: 0.70,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.50,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 雷50% 他10% (散兵・第1段階)
pub const ELECTRO_50_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "雷50% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.50,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 雷90% 他30% (散兵・第2段階)
pub const ELECTRO_90_REST_30: ResistanceTemplate = ResistanceTemplate {
    name: "雷90% 他30%",
    pyro: 0.30,
    hydro: 0.30,
    electro: 0.90,
    cryo: 0.30,
    dendro: 0.30,
    anemo: 0.30,
    geo: 0.30,
    physical: 0.30,
};

/// 草50% 他10% (アペプ・第2段階)
pub const DENDRO_50_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "草50% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 0.50,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.10,
};

/// 雷70% 物理30% 他10% (雷レジスヴァイン)
pub const ELECTRO_70_PHYS_30_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "雷70% 物理30% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.70,
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.30,
};

/// 氷免疫 物理0% 他10% (無相の氷) — Immune: cryo = 10.0
pub const CRYO_IMMUNE_PHYS_0_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "氷免疫 物理0% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 10.0, // Immune
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.0,
};

/// 雷免疫 物理0% 他10% (無相の雷) — Immune: electro = 10.0
pub const ELECTRO_IMMUNE_PHYS_0_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "雷免疫 物理0% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 10.0, // Immune
    cryo: 0.10,
    dendro: 0.10,
    anemo: 0.10,
    geo: 0.10,
    physical: 0.0,
};

/// 草免疫 物理0% 他10% (無相の草) — Immune: dendro = 10.0
pub const DENDRO_IMMUNE_PHYS_0_REST_10: ResistanceTemplate = ResistanceTemplate {
    name: "草免疫 物理0% 他10%",
    pyro: 0.10,
    hydro: 0.10,
    electro: 0.10,
    cryo: 0.10,
    dendro: 10.0, // Immune
    anemo: 0.10,
    geo: 0.10,
    physical: 0.0,
};
```

- [ ] **Step 4: テスト実行 — PASSを確認**

Run: `cargo test -p genshin-calc-data -- test_hydro_50 test_hydro_70_electro test_geo_70 test_pyro_70_cryo test_electro_90 test_immune`
Expected: All PASS

- [ ] **Step 5: コミット**

```bash
git add crates/data/src/enemies.rs
git commit -m "feat(data): add 13 resistance templates for bosses"
```

---

### Task 2: 週ボス敵データ追加（15エントリ）

**Files:**
- Modify: `crates/data/src/enemies.rs` (既存 `ANEMO_HYPOSTASIS` の後に追加)

- [ ] **Step 1: テスト追加 — 週ボスの `to_enemy()` 変換検証**

```rust
    #[test]
    fn test_childe_phase1_hydro_resistance() {
        let eps = 1e-10;
        let enemy = CHILDE_PHASE1.to_enemy(Some(Element::Hydro), 90, 0.0);
        assert!((enemy.resistance - 0.50).abs() < eps);
    }

    #[test]
    fn test_childe_phase1_non_hydro_resistance() {
        let eps = 1e-10;
        let enemy = CHILDE_PHASE1.to_enemy(Some(Element::Pyro), 90, 0.0);
        assert!((enemy.resistance - 0.0).abs() < eps);
    }

    #[test]
    fn test_childe_phase3_dual_resistance() {
        let eps = 1e-10;
        let enemy = CHILDE_PHASE3.to_enemy(Some(Element::Hydro), 90, 0.0);
        assert!((enemy.resistance - 0.70).abs() < eps);
        let enemy = CHILDE_PHASE3.to_enemy(Some(Element::Electro), 90, 0.0);
        assert!((enemy.resistance - 0.70).abs() < eps);
        let enemy = CHILDE_PHASE3.to_enemy(Some(Element::Pyro), 90, 0.0);
        assert!((enemy.resistance - 0.0).abs() < eps);
    }

    #[test]
    fn test_azhdaha_geo_and_physical() {
        let eps = 1e-10;
        let enemy = AZHDAHA.to_enemy(Some(Element::Geo), 90, 0.0);
        assert!((enemy.resistance - 0.70).abs() < eps);
        let enemy = AZHDAHA.to_enemy(None, 90, 0.0);
        assert!((enemy.resistance - 0.40).abs() < eps);
    }

    #[test]
    fn test_signora_phase2_dual_element() {
        let eps = 1e-10;
        let enemy = SIGNORA_PHASE2.to_enemy(Some(Element::Pyro), 90, 0.0);
        assert!((enemy.resistance - 0.70).abs() < eps);
        let enemy = SIGNORA_PHASE2.to_enemy(Some(Element::Cryo), 90, 0.0);
        assert!((enemy.resistance - 0.50).abs() < eps);
    }

    #[test]
    fn test_scaramouche_phase2_high_electro() {
        let eps = 1e-10;
        let enemy = SCARAMOUCHE_PHASE2.to_enemy(Some(Element::Electro), 90, 0.0);
        assert!((enemy.resistance - 0.90).abs() < eps);
        let enemy = SCARAMOUCHE_PHASE2.to_enemy(Some(Element::Pyro), 90, 0.0);
        assert!((enemy.resistance - 0.30).abs() < eps);
    }
```

- [ ] **Step 2: テスト実行 — FAILを確認**

Run: `cargo test -p genshin-calc-data -- test_childe`
Expected: FAIL — `CHILDE_PHASE1` が未定義

- [ ] **Step 3: 15体の週ボスを実装**

`ANEMO_HYPOSTASIS` の後に追加:

```rust
// ---------------------------------------------------------------------------
// 週ボス
// ---------------------------------------------------------------------------

pub const DVALIN: EnemyData = EnemyData {
    id: "dvalin",
    name: "風魔龍トワリン",
    resistance: &ALL_10,
};

pub const CHILDE_PHASE1: EnemyData = EnemyData {
    id: "childe_phase1",
    name: "タルタリヤ・第1段階",
    resistance: &HYDRO_50_REST_0,
};

pub const CHILDE_PHASE2: EnemyData = EnemyData {
    id: "childe_phase2",
    name: "タルタリヤ・第2段階",
    resistance: &ELECTRO_50_REST_0,
};

pub const CHILDE_PHASE3: EnemyData = EnemyData {
    id: "childe_phase3",
    name: "タルタリヤ・第3段階",
    resistance: &HYDRO_70_ELECTRO_70_REST_0,
};

pub const AZHDAHA: EnemyData = EnemyData {
    id: "azhdaha",
    name: "若陀龍王",
    resistance: &GEO_70_PHYS_40_REST_10,
};

pub const SIGNORA_PHASE1: EnemyData = EnemyData {
    id: "signora_phase1",
    name: "シニョーラ・第1段階",
    resistance: &CRYO_50_REST_10,
};

pub const SIGNORA_PHASE2: EnemyData = EnemyData {
    id: "signora_phase2",
    name: "シニョーラ・第2段階",
    resistance: &PYRO_70_CRYO_50_REST_10,
};

pub const RAIDEN_SHOGUN_WEEKLY: EnemyData = EnemyData {
    id: "raiden_shogun_weekly",
    name: "禍津御建鳴神命",
    resistance: &ALL_10,
};

pub const SCARAMOUCHE_PHASE1: EnemyData = EnemyData {
    id: "scaramouche_phase1",
    name: "正機の神・第1段階",
    resistance: &ELECTRO_50_REST_10,
};

pub const SCARAMOUCHE_PHASE2: EnemyData = EnemyData {
    id: "scaramouche_phase2",
    name: "正機の神・第2段階",
    resistance: &ELECTRO_90_REST_30,
};

pub const APEP_PHASE1: EnemyData = EnemyData {
    id: "apep_phase1",
    name: "アペプ・第1段階",
    resistance: &DENDRO_70_REST_10,
};

pub const APEP_PHASE2: EnemyData = EnemyData {
    id: "apep_phase2",
    name: "アペプ・第2段階",
    resistance: &DENDRO_50_REST_10,
};

pub const APEP_PHASE3: EnemyData = EnemyData {
    id: "apep_phase3",
    name: "アペプ・第3段階",
    resistance: &DENDRO_70_REST_10,
};

pub const NARWHAL_PHASE1: EnemyData = EnemyData {
    id: "narwhal_phase1",
    name: "呑星の鯨・第1段階",
    resistance: &HYDRO_70_REST_10,
};

pub const NARWHAL_PHASE2: EnemyData = EnemyData {
    id: "narwhal_phase2",
    name: "呑星の鯨・第2段階",
    resistance: &ELECTRO_70_REST_10,
};
```

- [ ] **Step 4: テスト実行 — PASSを確認**

Run: `cargo test -p genshin-calc-data -- test_childe test_azhdaha test_signora test_scaramouche`
Expected: All PASS

- [ ] **Step 5: コミット**

```bash
git add crates/data/src/enemies.rs
git commit -m "feat(data): add 15 weekly boss enemy entries"
```

---

### Task 3: フィールドボス・精鋭の敵データ追加（10エントリ）

**Files:**
- Modify: `crates/data/src/enemies.rs` (週ボスの後に追加)

- [ ] **Step 1: テスト追加 — 免疫とフィールドボスの検証**

```rust
    #[test]
    fn test_cryo_hypostasis_immune() {
        let eps = 1e-10;
        let enemy = CRYO_HYPOSTASIS.to_enemy(Some(Element::Cryo), 90, 0.0);
        assert!((enemy.resistance - 10.0).abs() < eps);
        // Non-immune element
        let enemy = CRYO_HYPOSTASIS.to_enemy(Some(Element::Pyro), 90, 0.0);
        assert!((enemy.resistance - 0.10).abs() < eps);
        // Physical
        let enemy = CRYO_HYPOSTASIS.to_enemy(None, 90, 0.0);
        assert!((enemy.resistance - 0.0).abs() < eps);
    }

    #[test]
    fn test_electro_hypostasis_immune() {
        let eps = 1e-10;
        let enemy = ELECTRO_HYPOSTASIS.to_enemy(Some(Element::Electro), 90, 0.0);
        assert!((enemy.resistance - 10.0).abs() < eps);
    }

    #[test]
    fn test_dendro_hypostasis_immune() {
        let eps = 1e-10;
        let enemy = DENDRO_HYPOSTASIS.to_enemy(Some(Element::Dendro), 90, 0.0);
        assert!((enemy.resistance - 10.0).abs() < eps);
    }

    #[test]
    fn test_electro_regisvine_resistances() {
        let eps = 1e-10;
        let enemy = ELECTRO_REGISVINE.to_enemy(Some(Element::Electro), 90, 0.0);
        assert!((enemy.resistance - 0.70).abs() < eps);
        let enemy = ELECTRO_REGISVINE.to_enemy(None, 90, 0.0);
        assert!((enemy.resistance - 0.30).abs() < eps);
    }

    #[test]
    fn test_eremite_negative_physical() {
        let eps = 1e-10;
        let enemy = EREMITE_ELITE.to_enemy(None, 90, 0.0);
        assert!((enemy.resistance - (-0.20)).abs() < eps);
        let enemy = EREMITE_ELITE.to_enemy(Some(Element::Pyro), 90, 0.0);
        assert!((enemy.resistance - 0.10).abs() < eps);
    }
```

- [ ] **Step 2: テスト実行 — FAILを確認**

Run: `cargo test -p genshin-calc-data -- test_cryo_hypostasis`
Expected: FAIL — `CRYO_HYPOSTASIS` が未定義

- [ ] **Step 3: 10体のフィールドボス・精鋭を実装**

```rust
// ---------------------------------------------------------------------------
// フィールドボス
// ---------------------------------------------------------------------------

pub const ELECTRO_REGISVINE: EnemyData = EnemyData {
    id: "electro_regisvine",
    name: "雷レジスヴァイン",
    resistance: &ELECTRO_70_PHYS_30_REST_10,
};

pub const MAGUU_KENKI: EnemyData = EnemyData {
    id: "maguu_kenki",
    name: "魔偶剣鬼",
    resistance: &ALL_10,
};

pub const CRYO_HYPOSTASIS: EnemyData = EnemyData {
    id: "cryo_hypostasis",
    name: "無相の氷",
    resistance: &CRYO_IMMUNE_PHYS_0_REST_10,
};

pub const ELECTRO_HYPOSTASIS: EnemyData = EnemyData {
    id: "electro_hypostasis",
    name: "無相の雷",
    resistance: &ELECTRO_IMMUNE_PHYS_0_REST_10,
};

pub const DENDRO_HYPOSTASIS: EnemyData = EnemyData {
    id: "dendro_hypostasis",
    name: "無相の草",
    resistance: &DENDRO_IMMUNE_PHYS_0_REST_10,
};

// ---------------------------------------------------------------------------
// 精鋭
// ---------------------------------------------------------------------------

pub const ABYSS_HERALD: EnemyData = EnemyData {
    id: "abyss_herald",
    name: "アビスの使徒",
    resistance: &ALL_10,
};

pub const ABYSS_LECTOR: EnemyData = EnemyData {
    id: "abyss_lector",
    name: "アビスの詠唱者",
    resistance: &ALL_10,
};

pub const RUIN_DRAKE_SKYWATCH: EnemyData = EnemyData {
    id: "ruin_drake_skywatch",
    name: "遺跡ドレイク・飛空",
    resistance: &PHYS_50_ELEM_10,
};

pub const RUIN_DRAKE_EARTHGUARD: EnemyData = EnemyData {
    id: "ruin_drake_earthguard",
    name: "遺跡ドレイク・陸行",
    resistance: &PHYS_50_ELEM_10,
};

pub const EREMITE_ELITE: EnemyData = EnemyData {
    id: "eremite_elite",
    name: "エルマイト旅団・精鋭",
    resistance: &PHYS_MINUS20_ELEM_10,
};
```

- [ ] **Step 4: テスト実行 — PASSを確認**

Run: `cargo test -p genshin-calc-data -- test_cryo_hypostasis test_electro_hypostasis test_dendro_hypostasis test_electro_regisvine test_eremite`
Expected: All PASS

- [ ] **Step 5: コミット**

```bash
git add crates/data/src/enemies.rs
git commit -m "feat(data): add 10 field boss and elite enemy entries"
```

---

### Task 4: 配列更新・カウントテスト・最終検証

**Files:**
- Modify: `crates/data/src/enemies.rs` (`ALL_TEMPLATES`, `ALL_ENEMIES` 配列 + テストのカウント値)

- [ ] **Step 1: `ALL_TEMPLATES` 配列に13テンプレートを追加**

```rust
pub const ALL_TEMPLATES: &[&ResistanceTemplate] = &[
    // 既存12
    &ALL_10,
    &PHYS_50_ELEM_10,
    &PHYS_70_ELEM_10,
    &PHYS_30_ELEM_10,
    &PHYS_MINUS20_ELEM_10,
    &PYRO_70_REST_10,
    &CRYO_70_REST_10,
    &ELECTRO_70_REST_10,
    &HYDRO_70_REST_10,
    &DENDRO_70_REST_10,
    &GEO_70_REST_10,
    &ANEMO_70_REST_10,
    // 新規13
    &HYDRO_50_REST_0,
    &ELECTRO_50_REST_0,
    &HYDRO_70_ELECTRO_70_REST_0,
    &GEO_70_PHYS_40_REST_10,
    &CRYO_50_REST_10,
    &PYRO_70_CRYO_50_REST_10,
    &ELECTRO_50_REST_10,
    &ELECTRO_90_REST_30,
    &DENDRO_50_REST_10,
    &ELECTRO_70_PHYS_30_REST_10,
    &CRYO_IMMUNE_PHYS_0_REST_10,
    &ELECTRO_IMMUNE_PHYS_0_REST_10,
    &DENDRO_IMMUNE_PHYS_0_REST_10,
];
```

- [ ] **Step 2: `ALL_ENEMIES` 配列に25敵を追加**

```rust
pub const ALL_ENEMIES: &[&EnemyData] = &[
    // 既存15
    &HILICHURL,
    &MITACHURL,
    &LAWACHURL,
    &TREASURE_HOARDER,
    &FATUI_SKIRMISHER,
    &FATUI_AGENT,
    &RUIN_GUARD,
    &RUIN_HUNTER,
    &RUIN_GRADER,
    &PYRO_REGISVINE,
    &CRYO_REGISVINE,
    &THUNDER_MANIFESTATION,
    &HYDRO_HYPOSTASIS,
    &GEO_HYPOSTASIS,
    &ANEMO_HYPOSTASIS,
    // 週ボス
    &DVALIN,
    &CHILDE_PHASE1,
    &CHILDE_PHASE2,
    &CHILDE_PHASE3,
    &AZHDAHA,
    &SIGNORA_PHASE1,
    &SIGNORA_PHASE2,
    &RAIDEN_SHOGUN_WEEKLY,
    &SCARAMOUCHE_PHASE1,
    &SCARAMOUCHE_PHASE2,
    &APEP_PHASE1,
    &APEP_PHASE2,
    &APEP_PHASE3,
    &NARWHAL_PHASE1,
    &NARWHAL_PHASE2,
    // フィールドボス
    &ELECTRO_REGISVINE,
    &MAGUU_KENKI,
    &CRYO_HYPOSTASIS,
    &ELECTRO_HYPOSTASIS,
    &DENDRO_HYPOSTASIS,
    // 精鋭
    &ABYSS_HERALD,
    &ABYSS_LECTOR,
    &RUIN_DRAKE_SKYWATCH,
    &RUIN_DRAKE_EARTHGUARD,
    &EREMITE_ELITE,
];
```

- [ ] **Step 3: カウントテストを更新**

既存テストの値を更新:

```rust
    #[test]
    fn test_all_templates_count() {
        assert_eq!(ALL_TEMPLATES.len(), 25);
    }

    #[test]
    fn test_all_enemies_count() {
        assert_eq!(ALL_ENEMIES.len(), 40);
    }
```

- [ ] **Step 4: 全テスト実行 — 全PASSを確認**

Run: `cargo test -p genshin-calc-data`
Expected: All PASS (既存61テスト + 新規テスト全て)

- [ ] **Step 5: clippy + fmt 確認**

Run: `cargo clippy -p genshin-calc-data -- -D warnings && cargo fmt --check`
Expected: No warnings, no formatting issues

- [ ] **Step 6: coreテストも全パス確認**

Run: `cargo test`
Expected: All PASS (core + data 全テスト)

- [ ] **Step 7: コミット**

```bash
git add crates/data/src/enemies.rs
git commit -m "feat(data): update arrays and counts for 25 new enemies"
```

---

### Task 5: TODOリスト更新

**Files:**
- Modify: `docs/data-expansion-todo.md`

- [ ] **Step 1: P5セクションのチェックボックスを完了にマーク**

`docs/data-expansion-todo.md` の P5 セクションで、実装済みアイテムに `[x]` を付ける。
水/草レジスヴァインは存在しないため `[x] スキップ (ゲーム内に存在しない)` と記載。

- [ ] **Step 2: 現状サマリの敵データ行を更新**

敵データの実装済みを 15 → 40 に更新。

- [ ] **Step 3: コミット**

```bash
git add docs/data-expansion-todo.md
git commit -m "docs: update P5 enemy data checklist as completed"
```
