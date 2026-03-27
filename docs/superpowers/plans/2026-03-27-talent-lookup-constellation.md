# Talent Lookup API + Constellation Level Boost Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add constellation level boost data and talent multiplier lookup API to CharacterData, enabling automatic DamageInput construction from character data.

**Architecture:** ConstellationPattern enum added to data crate's types.rs. Lookup methods implemented as CharacterData methods. build_damage_input takes pre-resolved Stats from the existing team pipeline. No changes to core crate.

**Tech Stack:** Rust, serde, genshin-calc-core, genshin-calc-data

**Spec:** `docs/superpowers/specs/2026-03-27-talent-lookup-constellation-design.md`

---

### Task 1: Add ConstellationPattern enum and CharacterData field

**Files:**
- Modify: `crates/data/src/types.rs:1-2` (add import) and `:44-126` (add enum + field)

- [ ] **Step 1: Add ConstellationPattern enum to types.rs**

Add after `ArtifactRarity` enum (line 52), before the Talent Types section:

```rust
/// Constellation pattern for talent level boosts.
///
/// In Genshin Impact, every character's C3 boosts one of Skill/Burst by +3 levels,
/// and C5 boosts the other. This enum encodes the two possible patterns,
/// eliminating invalid states by construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConstellationPattern {
    /// C3 boosts Elemental Skill +3, C5 boosts Elemental Burst +3.
    C3SkillC5Burst,
    /// C3 boosts Elemental Burst +3, C5 boosts Elemental Skill +3.
    C3BurstC5Skill,
}
```

- [ ] **Step 2: Add constellation_pattern field to CharacterData**

Add after `talents: TalentSet,` (line 125):

```rust
    /// Constellation pattern for talent level boosts (C3 and C5).
    pub constellation_pattern: ConstellationPattern,
```

- [ ] **Step 3: Add DamageType import to types.rs**

Update line 1:

```rust
use genshin_calc_core::{DamageType, Element, ScalingStat};
```

- [ ] **Step 4: Verify compilation fails (expected — 102 const definitions need updating)**

Run: `cargo build -p genshin-calc-data 2>&1 | head -5`
Expected: compilation errors about missing `constellation_pattern` field

---

### Task 2: Add constellation_pattern to all 102 characters

**Files:**
- Modify: `crates/data/src/characters/pyro.rs` (16 chars)
- Modify: `crates/data/src/characters/hydro.rs` (14 chars)
- Modify: `crates/data/src/characters/electro.rs` (17 chars)
- Modify: `crates/data/src/characters/cryo.rs` (18 chars)
- Modify: `crates/data/src/characters/dendro.rs` (11 chars)
- Modify: `crates/data/src/characters/anemo.rs` (15 chars)
- Modify: `crates/data/src/characters/geo.rs` (11 chars)

**Reference data for verified characters:**
| Character | Pattern |
|-----------|---------|
| Freminet  | C3SkillC5Burst |
| Diluc     | C3SkillC5Burst |
| Ganyu     | C3BurstC5Skill |
| Raiden    | C3BurstC5Skill |
| Yanfei    | C3SkillC5Burst |

**Note:** The spec mentions "incremental" addition, but since `constellation_pattern` is a required field on `CharacterData`, all 102 characters must be updated at once for compilation to succeed. Data *accuracy* is verified for 5 characters; the remaining 97 need wiki verification.

For the remaining 97 characters, look up each character's C3/C5 on the Genshin Impact wiki to determine the correct pattern. C3 description says "Raises the Level of [Skill or Burst] by 3". Every character file uses the same structure — add `constellation_pattern: ConstellationPattern::C3SkillC5Burst,` or `::C3BurstC5Skill,` as the last field before the closing `};`.

- [ ] **Step 1: Add `use crate::types::ConstellationPattern;` to each character file's imports**

Each character file already imports from `crate::types::*`, so `ConstellationPattern` is available. No import changes needed if using glob import. If using explicit imports, add `ConstellationPattern` to the import list.

Check: `grep 'use crate::types' crates/data/src/characters/pyro.rs`

- [ ] **Step 2: Add constellation_pattern to pyro.rs (16 characters)**

For each `CharacterData` const in `pyro.rs`, add `constellation_pattern` as the last field before `};`. Example for Diluc:

```rust
pub const DILUC: CharacterData = CharacterData {
    // ... existing fields ...
    talents: TalentSet { ... },
    constellation_pattern: ConstellationPattern::C3SkillC5Burst,
};
```

Pyro characters and their patterns (verify with wiki):
- Diluc: C3SkillC5Burst (C3: 逆焔の刃 = Skill)
- Amber: C3BurstC5Skill (C3: 矢の雨 = Burst)
- Arlecchino: C3BurstC5Skill (C3: 極炎の夜 = Burst)
- Bennett: C3SkillC5Burst (C3: 溢れる情熱 = Skill)
- Chevreuse: C3BurstC5Skill (C3: 近接支援射撃 = Burst)
- Dehya: C3BurstC5Skill (C3: 焔の獅子 = Burst)
- Gaming: C3BurstC5Skill (C3: 瑞獣の祝福 = Burst)
- Hu Tao: C3SkillC5Burst (C3: 蝶導来世 = Skill)
- Klee: C3BurstC5Skill (C3: ドッカンフラワー = Burst)
- Lyney: C3SkillC5Burst (C3: 奇術的驚奇 = Skill)
- Mavuika: C3BurstC5Skill (C3: 炎の奔流 = Burst)
- Thoma: C3SkillC5Burst (C3: 烈火侍魂 = Skill)
- Xiangling: C3BurstC5Skill (C3: グゥオパァー出撃 = Burst)
- Xinyan: C3SkillC5Burst (C3: 掃キックの旋律 = Skill)
- Yanfei: C3SkillC5Burst (C3: 封印の判決 = Skill)
- Yoimiya: C3BurstC5Skill (C3: 琉金雲閒 = Burst)

- [ ] **Step 3: Add constellation_pattern to hydro.rs (14 characters)**

Hydro characters and their patterns:
- Barbara: C3BurstC5Skill (C3: 煌めく聖歌 = Burst)
- Candace: C3BurstC5Skill (C3: 聖なる儀典 = Burst)
- Furina: C3BurstC5Skill (C3: 至高の独唱 = Burst)
- Kokomi: C3BurstC5Skill (C3: 海月の遊宴 = Burst)
- Mona: C3BurstC5Skill (C3: 虚実流転 = Burst)
- Mualani: C3SkillC5Burst (C3: サーフシャーク・ウェーブライダー = Skill)
- Neuvillette: C3BurstC5Skill (C3: 古海の秩序 = Burst)
- Nilou: C3BurstC5Skill (C3: 睡蓮の舞 = Burst)
- Sigewinne: C3SkillC5Burst (C3: 応急処置 = Skill)
- Tartaglia: C3SkillC5Burst (C3: 海淵の断裂 = Skill)
- Xingqiu: C3BurstC5Skill (C3: 古華剣・裁雨留虹 = Burst)
- Yelan: C3BurstC5Skill (C3: 深閉の眼 = Burst)
- Ayato: C3BurstC5Skill (C3: 神里流・鏡花 = Burst)
- Dahlia: C3BurstC5Skill (C3 = Burst, verify with wiki)

- [ ] **Step 4: Add constellation_pattern to electro.rs (17 characters)**

- Beidou: C3BurstC5Skill (C3: 潮の逆流 = Burst)
- Clorinde: C3SkillC5Burst (C3: 裁きの刃 = Skill)
- Cyno: C3BurstC5Skill (C3: 聖儀の裁き = Burst)
- Dori: C3BurstC5Skill (C3: アルカサルザの契約 = Burst)
- Fischl: C3BurstC5Skill (C3: 常夜の幻 = Burst)
- Iansan: C3SkillC5Burst (C3 = Skill, verify with wiki)
- Ineffa: C3BurstC5Skill (C3 = Burst, verify with wiki)
- Keqing: C3BurstC5Skill (C3: 天街巡遊 = Burst)
- Kujou Sara: C3BurstC5Skill (C3: 鎮めの礼 = Burst)
- Kuki Shinobu: C3SkillC5Burst (C3: 越祓雷草の輪 = Skill)
- Lisa: C3BurstC5Skill (C3: 薔薇の雷光 = Burst)
- Ororon: C3BurstC5Skill (C3 = Burst, verify with wiki)
- Raiden Shogun: C3BurstC5Skill (C3: 夢想真説 = Burst)
- Razor: C3BurstC5Skill (C3: 雷牙 = Burst)
- Sethos: C3BurstC5Skill (C3 = Burst, verify with wiki)
- Varesa: C3SkillC5Burst (C3 = Skill, verify with wiki)
- Yae Miko: C3BurstC5Skill (C3: 大密法 = Burst)

- [ ] **Step 5: Add constellation_pattern to cryo.rs (18 characters)**

- Aloy: C3BurstC5Skill (C3 = Burst)
- Ayaka: C3BurstC5Skill (C3: 神里流・霜滅 = Burst)
- Charlotte: C3BurstC5Skill (C3: 仲冬の速報 = Burst)
- Chongyun: C3BurstC5Skill (C3: 霊刃・雲開星落 = Burst)
- Citlali: C3BurstC5Skill (C3 = Burst, verify with wiki)
- Diona: C3BurstC5Skill (C3: シグネチャーミックス = Burst)
- Escoffier: C3BurstC5Skill (C3 = Burst, verify with wiki)
- Eula: C3BurstC5Skill (C3: 氷浪の光剣 = Burst)
- Freminet: C3SkillC5Burst (C3: 圧力変動型高出力射撃 = Skill)
- Ganyu: C3BurstC5Skill (C3: 降天帰命 = Burst)
- Kaeya: C3BurstC5Skill (C3: 凛冽なる氷の柱 = Burst)
- Layla: C3BurstC5Skill (C3: 安眠の星の帳 = Burst)
- Mika: C3BurstC5Skill (C3: 星光の箭陣 = Burst)
- Qiqi: C3BurstC5Skill (C3: 仙法・寒病鬼差 = Burst)
- Rosaria: C3SkillC5Burst (C3: 懺悔の儀式 = Skill)
- Shenhe: C3BurstC5Skill (C3: 神女遣霊真訣 = Burst)
- Skirk: C3SkillC5Burst (C3 = Skill, verify with wiki)
- Wriothesley: C3SkillC5Burst (C3: 拳闘の掟 = Skill)

- [ ] **Step 6: Add constellation_pattern to dendro.rs (11 characters)**

- Alhaitham: C3BurstC5Skill (C3: 殊境・顕象の結末 = Burst)
- Baizhu: C3BurstC5Skill (C3: 太素の癒し = Burst)
- Collei: C3BurstC5Skill (C3: 猫猫の秘宝 = Burst)
- Emilie: C3SkillC5Burst (C3: 香水の調合 = Skill)
- Kaveh: C3BurstC5Skill (C3: 壁画師の妄想 = Burst)
- Kirara: C3SkillC5Burst (C3: 猫箱急便 = Skill)
- Nahida: C3BurstC5Skill (C3: 夢の境域 = Burst)
- Tighnari: C3BurstC5Skill (C3: 造生の夢 = Burst)
- Traveler Dendro: C3SkillC5Burst (C3 = Skill, verify; const name: TRAVELER_DENDRO)
- Yaoyao: C3BurstC5Skill (C3: 桂子仙機 = Burst)
- Kinich: C3SkillC5Burst (C3: 巡夜の斥候 = Skill)

- [ ] **Step 7: Add constellation_pattern to anemo.rs (15 characters)**

- Chasca: C3SkillC5Burst (C3: 多重弾射撃 = Skill)
- Faruzan: C3BurstC5Skill (C3: 七窟遺智 = Burst)
- Heizou: C3BurstC5Skill (C3: 勠力の風 = Burst)
- Ifa: C3BurstC5Skill (C3 = Burst, verify with wiki)
- Jean: C3BurstC5Skill (C3: 蒲公英の風 = Burst)
- Kazuha: C3BurstC5Skill (C3: 万葉の一刀 = Burst)
- Lan Yan: C3BurstC5Skill (C3 = Burst, verify with wiki)
- Lynette: C3BurstC5Skill (C3: マジカルキャット = Burst)
- Mizuki: C3BurstC5Skill (C3 = Burst, verify with wiki)
- Sayu: C3BurstC5Skill (C3: 呑獣の意味 = Burst)
- Sucrose: C3BurstC5Skill (C3: 禁忌・滅風の創造 = Burst)
- Venti: C3BurstC5Skill (C3: 風神の詩 = Burst)
- Wanderer: C3BurstC5Skill (C3: 狂言「式舞」 = Burst)
- Xianyun: C3BurstC5Skill (C3: 白雲閑歩 = Burst)
- Xiao: C3BurstC5Skill (C3: 靖妖儺舞 = Burst)

- [ ] **Step 8: Add constellation_pattern to geo.rs (11 characters)**

- Albedo: C3SkillC5Burst (C3: 創生の陽光 = Skill)
- Chiori: C3SkillC5Burst (C3: 四切り仕立て = Skill)
- Gorou: C3BurstC5Skill (C3: 犬坂の大将 = Burst)
- Itto: C3BurstC5Skill (C3: 荒瀧第一 = Burst)
- Kachina: C3SkillC5Burst (C3 = Skill, verify with wiki)
- Navia: C3BurstC5Skill (C3: 砲弾洗礼 = Burst)
- Ningguang: C3BurstC5Skill (C3: 天権の崩玉 = Burst)
- Noelle: C3BurstC5Skill (C3: メイドの誠心 = Burst)
- Xilonen: C3SkillC5Burst (C3 = Skill, verify with wiki)
- Yun Jin: C3BurstC5Skill (C3: 破冰の旋舞 = Burst)
- Zhongli: C3BurstC5Skill (C3: 天星 = Burst)

- [ ] **Step 9: Verify compilation passes**

Run: `cargo build -p genshin-calc-data`
Expected: success

- [ ] **Step 10: Run existing tests to verify no regressions**

Run: `cargo test`
Expected: all tests pass (239+)

- [ ] **Step 11: Commit**

```bash
git add crates/data/src/types.rs crates/data/src/characters/
cargo fmt && cargo clippy -- -D warnings
git commit -m "feat(data): add ConstellationPattern to CharacterData for 102 characters"
```

---

### Task 3: Implement effective_talent_level with tests

**Files:**
- Modify: `crates/data/src/types.rs` (add impl block)

- [ ] **Step 1: Write failing tests for effective_talent_level**

Add to bottom of `crates/data/src/types.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use genshin_calc_core::DamageType;

    // Helper to create minimal CharacterData for testing
    fn test_char(pattern: ConstellationPattern) -> CharacterData {
        use genshin_calc_core::Element;
        static EMPTY_HITS: &[TalentScaling] = &[];
        CharacterData {
            id: "test",
            name: "Test",
            element: Element::Pyro,
            weapon_type: WeaponType::Sword,
            rarity: Rarity::Star4,
            region: Region::Mondstadt,
            base_hp: [0.0; 4],
            base_atk: [0.0; 4],
            base_def: [0.0; 4],
            ascension_stat: AscensionStat::Atk(0.0),
            talents: TalentSet {
                normal_attack: NormalAttackData {
                    name: "test",
                    hits: EMPTY_HITS,
                    charged: EMPTY_HITS,
                    plunging: EMPTY_HITS,
                },
                elemental_skill: TalentData {
                    name: "test_skill",
                    scalings: EMPTY_HITS,
                },
                elemental_burst: TalentData {
                    name: "test_burst",
                    scalings: EMPTY_HITS,
                },
            },
            constellation_pattern: pattern,
        }
    }

    #[test]
    fn test_effective_level_no_constellation() {
        let char = test_char(ConstellationPattern::C3SkillC5Burst);
        assert_eq!(char.effective_talent_level(DamageType::Skill, 9, 0), 9);
        assert_eq!(char.effective_talent_level(DamageType::Burst, 9, 0), 9);
        assert_eq!(char.effective_talent_level(DamageType::Normal, 9, 0), 9);
    }

    #[test]
    fn test_effective_level_c3_skill_c5_burst() {
        let char = test_char(ConstellationPattern::C3SkillC5Burst);
        // C3: Skill +3
        assert_eq!(char.effective_talent_level(DamageType::Skill, 9, 3), 12);
        assert_eq!(char.effective_talent_level(DamageType::Burst, 9, 3), 9);
        // C5: Burst +3
        assert_eq!(char.effective_talent_level(DamageType::Burst, 9, 5), 12);
        // C6: both boosted
        assert_eq!(char.effective_talent_level(DamageType::Skill, 9, 6), 12);
        assert_eq!(char.effective_talent_level(DamageType::Burst, 9, 6), 12);
    }

    #[test]
    fn test_effective_level_c3_burst_c5_skill() {
        let char = test_char(ConstellationPattern::C3BurstC5Skill);
        // C3: Burst +3
        assert_eq!(char.effective_talent_level(DamageType::Burst, 9, 3), 12);
        assert_eq!(char.effective_talent_level(DamageType::Skill, 9, 3), 9);
        // C5: Skill +3
        assert_eq!(char.effective_talent_level(DamageType::Skill, 9, 5), 12);
    }

    #[test]
    fn test_effective_level_cap_at_15() {
        let char = test_char(ConstellationPattern::C3SkillC5Burst);
        assert_eq!(char.effective_talent_level(DamageType::Skill, 13, 3), 15);
        assert_eq!(char.effective_talent_level(DamageType::Skill, 15, 3), 15);
    }

    #[test]
    fn test_effective_level_normal_never_boosted() {
        let char = test_char(ConstellationPattern::C3SkillC5Burst);
        assert_eq!(char.effective_talent_level(DamageType::Normal, 9, 6), 9);
        assert_eq!(char.effective_talent_level(DamageType::Charged, 9, 6), 9);
        assert_eq!(char.effective_talent_level(DamageType::Plunging, 9, 6), 9);
    }
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data -- types::tests`
Expected: FAIL — `effective_talent_level` not found

- [ ] **Step 3: Implement effective_talent_level**

Add `impl CharacterData` block in `types.rs` (after the struct definition, before the Weapon section):

```rust
impl CharacterData {
    /// Returns the effective talent level after constellation boosts.
    ///
    /// Normal/Charged/Plunging are never boosted. Skill and Burst get +3
    /// based on the character's constellation pattern at C3/C5.
    /// Capped at 15.
    pub fn effective_talent_level(
        &self,
        damage_type: DamageType,
        talent_level: u8,
        constellation: u8,
    ) -> u8 {
        let boost = match (damage_type, self.constellation_pattern) {
            (DamageType::Skill, ConstellationPattern::C3SkillC5Burst)
                if constellation >= 3 => 3,
            (DamageType::Skill, ConstellationPattern::C3BurstC5Skill)
                if constellation >= 5 => 3,
            (DamageType::Burst, ConstellationPattern::C3SkillC5Burst)
                if constellation >= 5 => 3,
            (DamageType::Burst, ConstellationPattern::C3BurstC5Skill)
                if constellation >= 3 => 3,
            _ => 0,
        };
        (talent_level + boost).min(15)
    }
}
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data -- types::tests`
Expected: all PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/types.rs
cargo fmt && cargo clippy -- -D warnings
git commit -m "feat(data): implement effective_talent_level with constellation boost"
```

---

### Task 4: Implement talent_scaling and talent_multiplier with tests

**Files:**
- Modify: `crates/data/src/types.rs` (extend impl block + tests)

- [ ] **Step 1: Write failing tests for talent_scaling and talent_multiplier**

Add to the existing `tests` module in `types.rs`:

```rust
    #[test]
    fn test_talent_scaling_diluc_skill() {
        let diluc = crate::find_character("diluc").unwrap();
        let scaling = diluc.talent_scaling(DamageType::Skill, 0).unwrap();
        assert_eq!(scaling.name, "1段ダメージ");
        assert_eq!(scaling.scaling_stat, ScalingStat::Atk);
        assert_eq!(scaling.damage_element, Some(Element::Pyro));
    }

    #[test]
    fn test_talent_scaling_out_of_range() {
        let diluc = crate::find_character("diluc").unwrap();
        assert!(diluc.talent_scaling(DamageType::Skill, 999).is_none());
    }

    #[test]
    fn test_talent_scaling_normal_charged_plunging() {
        let diluc = crate::find_character("diluc").unwrap();
        // Normal hit
        let s = diluc.talent_scaling(DamageType::Normal, 0).unwrap();
        assert_eq!(s.damage_element, None); // Physical
        // Charged
        let s = diluc.talent_scaling(DamageType::Charged, 0).unwrap();
        assert_eq!(s.damage_element, None);
        // Plunging
        let s = diluc.talent_scaling(DamageType::Plunging, 0).unwrap();
        assert_eq!(s.damage_element, None);
    }

    #[test]
    fn test_talent_multiplier_diluc_skill_lv9() {
        let diluc = crate::find_character("diluc").unwrap();
        let mult = diluc.talent_multiplier(DamageType::Skill, 0, 9, 0).unwrap();
        // Diluc skill 1st hit Lv9 = 1.6048
        assert!((mult - 1.6048).abs() < 1e-4);
    }

    #[test]
    fn test_talent_multiplier_with_constellation_boost() {
        let diluc = crate::find_character("diluc").unwrap();
        // Diluc C3 = Skill +3, so Lv9 becomes Lv12
        let mult = diluc.talent_multiplier(DamageType::Skill, 0, 9, 3).unwrap();
        // Diluc skill 1st hit Lv12 = 1.8880
        assert!((mult - 1.8880).abs() < 1e-4);
    }

    #[test]
    fn test_talent_multiplier_ganyu_burst_c3() {
        let ganyu = crate::find_character("ganyu").unwrap();
        // Ganyu C3 = Burst +3 (C3BurstC5Skill pattern)
        let no_boost = ganyu.talent_multiplier(DamageType::Burst, 0, 9, 0).unwrap();
        let with_c3 = ganyu.talent_multiplier(DamageType::Burst, 0, 9, 3).unwrap();
        assert!(with_c3 > no_boost);
    }

    #[test]
    fn test_talent_multiplier_out_of_range() {
        let diluc = crate::find_character("diluc").unwrap();
        assert!(diluc.talent_multiplier(DamageType::Skill, 999, 9, 0).is_none());
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data -- types::tests`
Expected: FAIL — `talent_scaling` and `talent_multiplier` not found

- [ ] **Step 3: Implement talent_scaling and talent_multiplier**

Add to the existing `impl CharacterData` block in `types.rs`:

```rust
    /// Returns the TalentScaling entry for a specific talent hit.
    pub fn talent_scaling(
        &self,
        damage_type: DamageType,
        hit_index: usize,
    ) -> Option<&TalentScaling> {
        let scalings: &[TalentScaling] = match damage_type {
            DamageType::Normal => self.talents.normal_attack.hits,
            DamageType::Charged => self.talents.normal_attack.charged,
            DamageType::Plunging => self.talents.normal_attack.plunging,
            DamageType::Skill => self.talents.elemental_skill.scalings,
            DamageType::Burst => self.talents.elemental_burst.scalings,
        };
        scalings.get(hit_index)
    }

    /// Returns the talent multiplier with constellation boost applied.
    pub fn talent_multiplier(
        &self,
        damage_type: DamageType,
        hit_index: usize,
        talent_level: u8,
        constellation: u8,
    ) -> Option<f64> {
        let scaling = self.talent_scaling(damage_type, hit_index)?;
        let effective = self.effective_talent_level(damage_type, talent_level, constellation);
        Some(scaling.values[(effective - 1) as usize])
    }
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data -- types::tests`
Expected: all PASS

- [ ] **Step 5: Run full test suite**

Run: `cargo test`
Expected: all pass

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/types.rs
cargo fmt && cargo clippy -- -D warnings
git commit -m "feat(data): implement talent_scaling and talent_multiplier on CharacterData"
```

---

### Task 5: Update team_builder to use effective_talent_level

**Files:**
- Modify: `crates/data/src/team_builder.rs:146-154`

- [ ] **Step 1: Write failing test for constellation-boosted talent buff**

Add test to `crates/data/src/team_builder.rs` tests module:

```rust
    #[test]
    fn test_bennett_c5_burst_buff_boosted() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();

        // C5 Bennett with burst Lv10 → effective Lv13
        let member = TeamMemberBuilder::new(bennett, weapon)
            .constellation(5)
            .talent_levels([1, 1, 10])
            .build()
            .unwrap();

        let burst_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Fantastic Voyage"))
            .unwrap();

        // Lv13 scaling = 1.19 (index 12 in the array)
        let expected = member.stats.base_atk * 1.19;
        assert!((burst_buff.value - expected).abs() < 1e-4);
    }

    #[test]
    fn test_bennett_c0_burst_buff_no_boost() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();

        // C0 Bennett with burst Lv10 → effective Lv10 (no boost)
        let member = TeamMemberBuilder::new(bennett, weapon)
            .constellation(0)
            .talent_levels([1, 1, 10])
            .build()
            .unwrap();

        let burst_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Fantastic Voyage"))
            .unwrap();

        // Lv10 scaling = 1.008 (index 9)
        let expected = member.stats.base_atk * 1.008;
        assert!((burst_buff.value - expected).abs() < 1e-4);
    }
```

- [ ] **Step 2: Run tests to verify the C5 test fails**

Run: `cargo test -p genshin-calc-data -- team_builder::tests::test_bennett_c5`
Expected: FAIL — C5 test should fail because current code doesn't apply constellation boost

- [ ] **Step 3: Update build() to use effective_talent_level**

In `crates/data/src/team_builder.rs`, add import at top:

```rust
use genshin_calc_core::DamageType;
```

Then replace lines 148-153 (the talent level lookup):

```rust
                        let talent_idx = match def.source {
                            crate::talent_buffs::TalentBuffSource::ElementalSkill => 1,
                            crate::talent_buffs::TalentBuffSource::ElementalBurst => 2,
                            _ => 0,
                        };
                        let level = self.talent_levels[talent_idx];
```

With:

```rust
                        let (talent_idx, damage_type) = match def.source {
                            crate::talent_buffs::TalentBuffSource::ElementalSkill => {
                                (1, DamageType::Skill)
                            }
                            crate::talent_buffs::TalentBuffSource::ElementalBurst => {
                                (2, DamageType::Burst)
                            }
                            _ => (0, DamageType::Normal),
                        // AscensionPassive and Constellation(u8) map to Normal,
                        // which is never boosted — these sources don't benefit from C3/C5.
                        };
                        let base_level = self.talent_levels[talent_idx];
                        let level = char_data.effective_talent_level(
                            damage_type,
                            base_level,
                            self.constellation,
                        );
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data -- team_builder::tests`
Expected: all PASS (including new C5 test)

- [ ] **Step 5: Run full test suite**

Run: `cargo test`
Expected: all pass

- [ ] **Step 6: Commit**

```bash
git add crates/data/src/team_builder.rs
cargo fmt && cargo clippy -- -D warnings
git commit -m "fix(data): apply constellation boost to talent buff scaling in TeamMemberBuilder"
```

---

### Task 6: Implement build_damage_input with tests

**Files:**
- Modify: `crates/data/src/types.rs` (add method + tests)

- [ ] **Step 1: Write failing tests for build_damage_input**

Add to the existing `tests` module in `types.rs`:

```rust
    #[test]
    fn test_build_damage_input_diluc_skill() {
        use genshin_calc_core::{DamageInput, Stats};

        let diluc = crate::find_character("diluc").unwrap();
        let stats = Stats {
            atk: 2000.0,
            crit_rate: 0.75,
            crit_dmg: 1.50,
            dmg_bonus: 0.466,
            ..Stats::default()
        };

        let input = diluc
            .build_damage_input(stats.clone(), 90, DamageType::Skill, 0, 9, 0, None, 0.0)
            .unwrap();

        assert_eq!(input.character_level, 90);
        assert_eq!(input.damage_type, DamageType::Skill);
        assert_eq!(input.scaling_stat, ScalingStat::Atk);
        assert_eq!(input.element, Some(Element::Pyro));
        assert!((input.talent_multiplier - 1.6048).abs() < 1e-4);
        assert!(input.reaction.is_none());
        assert!((input.stats.atk - 2000.0).abs() < 1e-4);
    }

    #[test]
    fn test_build_damage_input_with_reaction() {
        use genshin_calc_core::{Reaction, Stats};

        let diluc = crate::find_character("diluc").unwrap();
        let stats = Stats {
            atk: 2000.0,
            ..Stats::default()
        };

        let input = diluc
            .build_damage_input(
                stats, 90, DamageType::Skill, 0, 9, 0,
                Some(Reaction::Vaporize), 0.15,
            )
            .unwrap();

        assert_eq!(input.reaction, Some(Reaction::Vaporize));
        assert!((input.reaction_bonus - 0.15).abs() < 1e-4);
    }

    #[test]
    fn test_build_damage_input_with_constellation() {
        use genshin_calc_core::Stats;

        let diluc = crate::find_character("diluc").unwrap();
        let stats = Stats::default();

        // C3 Diluc = Skill +3, so Lv9 → Lv12
        let input = diluc
            .build_damage_input(stats, 90, DamageType::Skill, 0, 9, 3, None, 0.0)
            .unwrap();

        // Lv12 = 1.8880
        assert!((input.talent_multiplier - 1.8880).abs() < 1e-4);
    }

    #[test]
    fn test_build_damage_input_normal_physical() {
        use genshin_calc_core::Stats;

        let diluc = crate::find_character("diluc").unwrap();
        let stats = Stats::default();

        let input = diluc
            .build_damage_input(stats, 90, DamageType::Normal, 0, 9, 0, None, 0.0)
            .unwrap();

        assert_eq!(input.element, None); // Physical
        assert_eq!(input.scaling_stat, ScalingStat::Atk);
    }

    #[test]
    fn test_build_damage_input_out_of_range() {
        use genshin_calc_core::Stats;

        let diluc = crate::find_character("diluc").unwrap();
        let stats = Stats::default();

        assert!(diluc
            .build_damage_input(stats, 90, DamageType::Skill, 999, 9, 0, None, 0.0)
            .is_none());
    }
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `cargo test -p genshin-calc-data -- types::tests::test_build_damage_input`
Expected: FAIL — `build_damage_input` not found

- [ ] **Step 3: Implement build_damage_input**

Update the import line at top of `types.rs` (which was already modified in Task 1 Step 3) to its final form:

```rust
use genshin_calc_core::{DamageInput, DamageType, Element, Reaction, ScalingStat, Stats};
```

Add to the existing `impl CharacterData` block:

```rust
    /// Builds a DamageInput for a specific talent hit.
    ///
    /// Auto-fills talent_multiplier, scaling_stat, element, and damage_type
    /// from the character's talent data. Applies constellation boost.
    ///
    /// Returns `None` if `hit_index` is out of range.
    pub fn build_damage_input(
        &self,
        stats: Stats,
        character_level: u32,
        damage_type: DamageType,
        hit_index: usize,
        talent_level: u8,
        constellation: u8,
        reaction: Option<Reaction>,
        reaction_bonus: f64,
    ) -> Option<DamageInput> {
        let scaling = self.talent_scaling(damage_type, hit_index)?;
        let multiplier = self.talent_multiplier(damage_type, hit_index, talent_level, constellation)?;
        Some(DamageInput {
            character_level,
            stats,
            talent_multiplier: multiplier,
            scaling_stat: scaling.scaling_stat,
            damage_type,
            element: scaling.damage_element,
            reaction,
            reaction_bonus,
        })
    }
```

- [ ] **Step 4: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data -- types::tests`
Expected: all PASS

- [ ] **Step 5: Commit**

```bash
git add crates/data/src/types.rs
cargo fmt && cargo clippy -- -D warnings
git commit -m "feat(data): implement build_damage_input on CharacterData"
```

---

### Task 7: End-to-end integration test

**Files:**
- Modify: `crates/data/src/types.rs` (add integration test to tests module)

- [ ] **Step 1: Write end-to-end test**

Add to the `tests` module in `types.rs`:

```rust
    #[test]
    fn test_e2e_build_damage_input_to_calculate_damage() {
        use genshin_calc_core::{calculate_damage, Enemy, Stats};

        let diluc = crate::find_character("diluc").unwrap();
        let stats = Stats {
            atk: 2000.0,
            crit_rate: 0.75,
            crit_dmg: 1.50,
            dmg_bonus: 0.466,
            ..Stats::default()
        };

        let input = diluc
            .build_damage_input(stats, 90, DamageType::Skill, 0, 9, 0, None, 0.0)
            .unwrap();

        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };

        let result = calculate_damage(&input, &enemy).unwrap();

        // Verify result is reasonable (positive, crit > non_crit)
        assert!(result.non_crit > 0.0);
        assert!(result.crit > result.non_crit);
        assert!(result.average > result.non_crit);
        assert!(result.average < result.crit);

        // Golden value check:
        // base_dmg = 2000 * 1.6048 = 3209.6
        // with dmg_bonus: 3209.6 * (1 + 0.466) = 4705.2736
        // defense_mult = (90 + 100) / ((90 + 100) + (90 + 100)) = 0.5
        // resistance_mult = 1 - 0.10 = 0.90
        // non_crit = 4705.2736 * 0.5 * 0.9 = 2117.3731
        assert!((result.non_crit - 2117.3731).abs() < 1.0);
    }

    #[test]
    fn test_e2e_with_vaporize() {
        use genshin_calc_core::{calculate_damage, Enemy, Reaction, Stats};

        let diluc = crate::find_character("diluc").unwrap();
        let stats = Stats {
            atk: 2000.0,
            crit_rate: 0.75,
            crit_dmg: 1.50,
            dmg_bonus: 0.466,
            elemental_mastery: 200.0,
            ..Stats::default()
        };

        let input = diluc
            .build_damage_input(
                stats, 90, DamageType::Skill, 0, 9, 0,
                Some(Reaction::Vaporize), 0.15,
            )
            .unwrap();

        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
        };

        let result = calculate_damage(&input, &enemy).unwrap();

        // Vaporize should amplify damage significantly
        // Pyro trigger = 1.5x base, plus EM bonus + reaction bonus
        assert!(result.non_crit > 2117.0); // More than non-reaction
        assert_eq!(result.reaction, Some(Reaction::Vaporize));
    }
```

- [ ] **Step 2: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data -- types::tests::test_e2e`
Expected: all PASS

- [ ] **Step 3: Run full test suite + clippy + fmt**

Run: `cargo test && cargo clippy -- -D warnings && cargo fmt --check`
Expected: all pass, no warnings

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/types.rs
git commit -m "test(data): add end-to-end tests for build_damage_input pipeline"
```

---

### Task 8: Data integrity test for all characters

**Files:**
- Modify: `crates/data/src/types.rs` (add data integrity test)

- [ ] **Step 1: Write data integrity test**

Add to the `tests` module:

```rust
    #[test]
    fn test_all_characters_have_valid_talent_data() {
        // Verify every character has at least one normal attack hit,
        // one skill scaling, and one burst scaling
        for char in crate::characters::ALL_CHARACTERS {
            assert!(
                !char.talents.normal_attack.hits.is_empty(),
                "{} has no normal attack hits",
                char.name
            );
            assert!(
                !char.talents.elemental_skill.scalings.is_empty(),
                "{} has no skill scalings",
                char.name
            );
            assert!(
                !char.talents.elemental_burst.scalings.is_empty(),
                "{} has no burst scalings",
                char.name
            );

            // Verify talent_multiplier works for first hit of each type
            assert!(
                char.talent_multiplier(DamageType::Normal, 0, 1, 0).is_some(),
                "{} normal talent_multiplier failed",
                char.name
            );
            assert!(
                char.talent_multiplier(DamageType::Skill, 0, 1, 0).is_some(),
                "{} skill talent_multiplier failed",
                char.name
            );
            assert!(
                char.talent_multiplier(DamageType::Burst, 0, 1, 0).is_some(),
                "{} burst talent_multiplier failed",
                char.name
            );
        }
    }

    #[test]
    fn test_verified_characters_constellation_patterns() {
        // Golden data: verified against Genshin wiki
        let cases = [
            ("freminet", ConstellationPattern::C3SkillC5Burst),
            ("diluc", ConstellationPattern::C3SkillC5Burst),
            ("ganyu", ConstellationPattern::C3BurstC5Skill),
            ("raiden_shogun", ConstellationPattern::C3BurstC5Skill),
            ("yanfei", ConstellationPattern::C3SkillC5Burst),
        ];
        for (id, expected_pattern) in cases {
            let char = crate::find_character(id).unwrap();
            assert_eq!(
                char.constellation_pattern, expected_pattern,
                "{} has wrong constellation pattern",
                char.name
            );
        }
    }
```

- [ ] **Step 2: Run tests to verify they pass**

Run: `cargo test -p genshin-calc-data -- types::tests::test_all_characters`
Expected: all PASS

- [ ] **Step 3: Run full test suite**

Run: `cargo test`
Expected: all pass

- [ ] **Step 4: Commit**

```bash
git add crates/data/src/types.rs
cargo fmt && cargo clippy -- -D warnings
git commit -m "test(data): add data integrity tests for talent lookup and constellation patterns"
```
