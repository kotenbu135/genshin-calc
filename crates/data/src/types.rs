use genshin_calc_core::{DamageInput, DamageType, Element, Reaction, ScalingStat, Stats};
use serde::{Deserialize, Serialize};

/// Re-export of [`genshin_calc_core::WeaponType`] for convenience.
pub use genshin_calc_core::WeaponType;

// -- Core Enums --

/// Character or weapon rarity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rarity {
    Star1,
    Star2,
    Star3,
    Star4,
    Star5,
}

/// Character region of origin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Region {
    Mondstadt,
    Liyue,
    Inazuma,
    Sumeru,
    Fontaine,
    Natlan,
    Snezhnaya,
    NodKrai,
    Other,
}

/// Stat gained from character ascension.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AscensionStat {
    Hp(f64),
    Atk(f64),
    Def(f64),
    CritRate(f64),
    CritDmg(f64),
    ElementalMastery(f64),
    EnergyRecharge(f64),
    ElementalDmgBonus(Element, f64),
    PhysicalDmgBonus(f64),
    HealingBonus(f64),
}

/// An artifact set with its detected piece count.
///
/// `Serialize` only — `Deserialize` is not derived because the `set` field
/// is a `&'static` reference (per project convention: `&'static` types are Serialize-only).
#[derive(Debug, Clone, serde::Serialize)]
pub struct ArtifactSetEntry {
    /// Artifact set data.
    pub set: &'static ArtifactSet,
    /// Effective piece threshold: always exactly 2 or 4.
    /// Represents the highest activated set bonus, not the raw equipped count
    /// (e.g., 5 pieces of the same set → piece_count = 4).
    pub piece_count: u8,
}

/// Artifact set rarity availability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArtifactRarity {
    Star4,
    Star5,
    Star4And5,
}

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

// -- Talent Types --

/// 動的天賦ボーナス（スタック/ゲージ消費で天賦倍率に加算される効果）
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DynamicTalentBonus {
    /// ボーナス名（"闘志", "諸願百目の輪"等）
    pub name: &'static str,
    /// 最大スタック/ゲージ数
    pub max_stacks: u16,
    /// 天賦レベル別のスタック1あたりの加算倍率（小数形式）
    /// per_stack[talent_level - 1] × stacks を基本倍率に加算
    pub per_stack: [f64; 15],
}

/// Talent scaling entry with multipliers at each talent level.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentScaling {
    /// Scaling name (e.g. "1-Hit DMG").
    pub name: &'static str,
    /// Stat used for scaling.
    pub scaling_stat: ScalingStat,
    /// Element of the talent damage. `None` for physical.
    pub damage_element: Option<Element>,
    /// Multiplier values at talent levels 1-15.
    pub values: [f64; 15],
    /// 動的天賦ボーナス。Noneなら静的倍率のみ。
    pub dynamic_bonus: Option<&'static DynamicTalentBonus>,
}

/// Talent data for an elemental skill or burst.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentData {
    /// Talent name.
    pub name: &'static str,
    /// Scaling entries.
    pub scalings: &'static [TalentScaling],
}

/// Normal attack data including charged and plunging attacks.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct NormalAttackData {
    /// Attack name.
    pub name: &'static str,
    /// Normal attack hit scalings.
    pub hits: &'static [TalentScaling],
    /// Charged attack scalings.
    pub charged: &'static [TalentScaling],
    /// Plunging attack scalings.
    pub plunging: &'static [TalentScaling],
}

/// Complete talent set for a character.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TalentSet {
    pub normal_attack: NormalAttackData,
    pub elemental_skill: TalentData,
    pub elemental_burst: TalentData,
}

/// Base stat values at key level breakpoints.
/// [Lv1, Lv20, Lv20+, Lv40, Lv40+, Lv50(pre-A3), Lv50+, Lv60, Lv60+, Lv70, Lv70+, Lv80, Lv80+, Lv90, Lv90+, Lv95, Lv95+, Lv100]
///
/// Note: Lv50+ (突破後) is at index 6, and Lv50- (突破前) is at index 5.
/// This design allows level=50 to return the post-ascension value (Lv50+) for user convenience.
/// Lv95/Lv100 unlocked with 無主の星屑 (Stardust of the Unowned).
pub type BaseStatKeypoints = [f64; 18];

/// Returns the interpolated stat value at a given level.
///
/// Uses linear interpolation between key breakpoints.
/// Level 50 returns the post-ascension value (Lv50+) for convenience.
/// Use level 49 for the pre-ascension Lv50 value.
pub fn interpolate_base_stat(keypoints: &[f64; 18], level: u32) -> f64 {
    debug_assert_eq!(keypoints.len(), 18, "Expected 18 keypoints");
    let level = level.clamp(1, 100);

    if level <= 1 {
        return keypoints[0];
    }
    if level >= 100 {
        return keypoints[17];
    }

    // Breakpoints: [1, 20, 20, 40, 40, 49, 50, 60, 60, 70, 70, 80, 80, 90, 90, 95, 95, 100]
    // Index mapping:
    //   0: Lv1     = keypoints[0]
    //   1: Lv20    = keypoints[1]
    //   2: Lv20+   = keypoints[2]
    //   3: Lv40    = keypoints[3]
    //   4: Lv40+   = keypoints[4]
    //   5: Lv49    = keypoints[5] (Lv50 突破前)
    //   6: Lv50    = keypoints[6] (Lv50+ 突破後)
    //   7: Lv60    = keypoints[7]
    //   ...
    const BREAKPOINTS: [u32; 18] = [
        1, 20, 20, 40, 40, 49, 50, 60, 60, 70, 70, 80, 80, 90, 90, 95, 95, 100,
    ];

    for i in 0..17 {
        if level >= BREAKPOINTS[i] && level <= BREAKPOINTS[i + 1] {
            let start_level = BREAKPOINTS[i];
            let end_level = BREAKPOINTS[i + 1];
            let t = if start_level == end_level {
                0.0
            } else {
                (level - start_level) as f64 / (end_level - start_level) as f64
            };
            return keypoints[i] + (keypoints[i + 1] - keypoints[i]) * t;
        }
    }

    keypoints[17]
}

// -- Character --

/// Character data including base stats, talents, and metadata.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct CharacterData {
    /// Unique identifier (lowercase, e.g. `"hu_tao"`).
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Character element.
    pub element: Element,
    /// Weapon type.
    pub weapon_type: WeaponType,
    /// Rarity (4-star or 5-star).
    pub rarity: Rarity,
    /// Region of origin.
    pub region: Region,
    /// Base HP at key level breakpoints [Lv1, Lv20, Lv40, Lv50, Lv60, Lv70, Lv80, Lv90, Lv95, Lv100].
    pub base_hp: BaseStatKeypoints,
    /// Base ATK at key level breakpoints.
    pub base_atk: BaseStatKeypoints,
    /// Base DEF at key level breakpoints.
    pub base_def: BaseStatKeypoints,
    /// Stat gained from ascension.
    pub ascension_stat: AscensionStat,
    /// Character talent set.
    pub talents: TalentSet,
    /// Constellation pattern for talent level boosts (C3 and C5).
    pub constellation_pattern: ConstellationPattern,
}

impl CharacterData {
    /// Returns the effective talent level after constellation boosts.
    ///
    /// Normal/Charged/Plunging are never boosted. Skill and Burst get +3
    /// based on the character's constellation pattern at C3/C5. Capped at 15.
    ///
    /// `talent_level` must be 1-15. Passing 0 will cause a panic in `talent_multiplier`.
    pub fn effective_talent_level(
        &self,
        damage_type: DamageType,
        talent_level: u8,
        constellation: u8,
    ) -> u8 {
        let boost = match (damage_type, self.constellation_pattern) {
            (DamageType::Skill, ConstellationPattern::C3SkillC5Burst) if constellation >= 3 => 3,
            (DamageType::Skill, ConstellationPattern::C3BurstC5Skill) if constellation >= 5 => 3,
            (DamageType::Burst, ConstellationPattern::C3SkillC5Burst) if constellation >= 5 => 3,
            (DamageType::Burst, ConstellationPattern::C3BurstC5Skill) if constellation >= 3 => 3,
            _ => 0,
        };
        (talent_level + boost).min(15)
    }

    /// Returns the base HP at the specified character level.
    ///
    /// Uses linear interpolation between key breakpoints [Lv1, Lv20, Lv40, Lv50, Lv60, Lv70, Lv80, Lv90, Lv95, Lv100].
    pub fn base_hp_at_level(&self, level: u32) -> f64 {
        interpolate_base_stat(&self.base_hp, level)
    }

    /// Returns the base ATK at the specified character level.
    pub fn base_atk_at_level(&self, level: u32) -> f64 {
        interpolate_base_stat(&self.base_atk, level)
    }

    /// Returns the base DEF at the specified character level.
    pub fn base_def_at_level(&self, level: u32) -> f64 {
        interpolate_base_stat(&self.base_def, level)
    }

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
        if talent_level == 0 {
            return None;
        }
        let scaling = self.talent_scaling(damage_type, hit_index)?;
        let effective = self.effective_talent_level(damage_type, talent_level, constellation);
        Some(scaling.values[(effective - 1) as usize])
    }

    /// Builds a DamageInput for a specific talent hit.
    ///
    /// Auto-fills talent_multiplier, scaling_stat, element, and damage_type
    /// from the character's talent data. Applies constellation boost.
    ///
    /// Returns `None` if `hit_index` is out of range.
    #[allow(clippy::too_many_arguments)]
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
        if talent_level == 0 {
            return None;
        }
        let scaling = self.talent_scaling(damage_type, hit_index)?;
        let effective = self.effective_talent_level(damage_type, talent_level, constellation);
        let multiplier = scaling.values[(effective - 1) as usize];
        Some(DamageInput {
            character_level,
            stats,
            talent_multiplier: multiplier,
            scaling_stat: scaling.scaling_stat,
            damage_type,
            element: scaling.damage_element,
            reaction,
            reaction_bonus,
            flat_dmg: 0.0,
        })
    }
}

// -- Weapon --

/// Weapon sub-stat type with values at each ascension breakpoint.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum WeaponSubStat {
    HpPercent([f64; 4]),
    AtkPercent([f64; 4]),
    DefPercent([f64; 4]),
    CritRate([f64; 4]),
    CritDmg([f64; 4]),
    ElementalMastery([f64; 4]),
    EnergyRecharge([f64; 4]),
    PhysicalDmgBonus([f64; 4]),
}

/// Weapon passive effect.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeaponPassive {
    /// Passive name.
    pub name: &'static str,
    /// Effect details.
    pub effect: crate::buff::PassiveEffect,
}

/// Weapon data including base stats, passives, and metadata.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct WeaponData {
    /// Unique identifier.
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Weapon type.
    pub weapon_type: WeaponType,
    /// Rarity.
    pub rarity: Rarity,
    /// Base ATK at ascension breakpoints.
    pub base_atk: [f64; 4],
    /// Sub-stat with values at ascension breakpoints.
    pub sub_stat: Option<WeaponSubStat>,
    /// Weapon passive effect.
    pub passive: Option<WeaponPassive>,
}

// -- Artifact --

/// Artifact set effect for 2-piece or 4-piece bonus.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct SetEffect {
    /// Effect description.
    pub description: &'static str,
    /// Stat buffs provided.
    pub buffs: &'static [crate::buff::StatBuff],
    /// Conditional buffs that require activation.
    pub conditional_buffs: &'static [crate::buff::ConditionalBuff],
}

/// Artifact set data.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ArtifactSet {
    /// Unique identifier.
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Rarity availability.
    pub rarity: ArtifactRarity,
    /// 2-piece set bonus.
    pub two_piece: SetEffect,
    /// 4-piece set bonus.
    pub four_piece: SetEffect,
}

// -- Enemy --

/// Elemental resistance template for an enemy type.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ResistanceTemplate {
    pub name: &'static str,
    pub pyro: f64,
    pub hydro: f64,
    pub electro: f64,
    pub cryo: f64,
    pub dendro: f64,
    pub anemo: f64,
    pub geo: f64,
    pub physical: f64,
}

/// Enemy data with resistance template.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct EnemyData {
    /// Unique identifier.
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Resistance values.
    pub resistance: &'static ResistanceTemplate,
}

impl ResistanceTemplate {
    /// Returns the resistance value for the given element, or physical if `None`.
    pub fn get(&self, element: Option<Element>) -> f64 {
        match element {
            Some(Element::Pyro) => self.pyro,
            Some(Element::Hydro) => self.hydro,
            Some(Element::Electro) => self.electro,
            Some(Element::Cryo) => self.cryo,
            Some(Element::Dendro) => self.dendro,
            Some(Element::Anemo) => self.anemo,
            Some(Element::Geo) => self.geo,
            None => self.physical,
        }
    }
}

impl EnemyData {
    /// Converts to a [`genshin_calc_core::Enemy`] for use in damage calculations.
    pub fn to_enemy(
        &self,
        element: Option<Element>,
        level: u32,
        def_reduction: f64,
    ) -> genshin_calc_core::Enemy {
        genshin_calc_core::Enemy {
            level,
            resistance: self.resistance.get(element),
            def_reduction,
            def_ignore: 0.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genshin_calc_core::DamageType;

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
            base_hp: [0.0; 18],
            base_atk: [0.0; 18],
            base_def: [0.0; 18],
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
        let c = test_char(ConstellationPattern::C3SkillC5Burst);
        assert_eq!(c.effective_talent_level(DamageType::Skill, 9, 0), 9);
        assert_eq!(c.effective_talent_level(DamageType::Burst, 9, 0), 9);
        assert_eq!(c.effective_talent_level(DamageType::Normal, 9, 0), 9);
    }

    #[test]
    fn test_effective_level_c3_skill_c5_burst() {
        let c = test_char(ConstellationPattern::C3SkillC5Burst);
        assert_eq!(c.effective_talent_level(DamageType::Skill, 9, 3), 12);
        assert_eq!(c.effective_talent_level(DamageType::Burst, 9, 3), 9);
        assert_eq!(c.effective_talent_level(DamageType::Burst, 9, 5), 12);
        assert_eq!(c.effective_talent_level(DamageType::Skill, 9, 6), 12);
        assert_eq!(c.effective_talent_level(DamageType::Burst, 9, 6), 12);
    }

    #[test]
    fn test_effective_level_c3_burst_c5_skill() {
        let c = test_char(ConstellationPattern::C3BurstC5Skill);
        assert_eq!(c.effective_talent_level(DamageType::Burst, 9, 3), 12);
        assert_eq!(c.effective_talent_level(DamageType::Skill, 9, 3), 9);
        assert_eq!(c.effective_talent_level(DamageType::Skill, 9, 5), 12);
    }

    #[test]
    fn test_effective_level_cap_at_15() {
        let c = test_char(ConstellationPattern::C3SkillC5Burst);
        assert_eq!(c.effective_talent_level(DamageType::Skill, 13, 3), 15);
        assert_eq!(c.effective_talent_level(DamageType::Skill, 15, 3), 15);
    }

    #[test]
    fn test_effective_level_normal_never_boosted() {
        let c = test_char(ConstellationPattern::C3SkillC5Burst);
        assert_eq!(c.effective_talent_level(DamageType::Normal, 9, 6), 9);
        assert_eq!(c.effective_talent_level(DamageType::Charged, 9, 6), 9);
        assert_eq!(c.effective_talent_level(DamageType::Plunging, 9, 6), 9);
    }

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
        let s = diluc.talent_scaling(DamageType::Normal, 0).unwrap();
        assert_eq!(s.damage_element, None);
        let s = diluc.talent_scaling(DamageType::Charged, 0).unwrap();
        assert_eq!(s.damage_element, None);
        let s = diluc.talent_scaling(DamageType::Plunging, 0).unwrap();
        assert_eq!(s.damage_element, None);
    }

    #[test]
    fn test_talent_multiplier_diluc_skill_lv9() {
        let diluc = crate::find_character("diluc").unwrap();
        let mult = diluc.talent_multiplier(DamageType::Skill, 0, 9, 0).unwrap();
        assert!((mult - 1.6048).abs() < 1e-4);
    }

    #[test]
    fn test_talent_multiplier_with_constellation_boost() {
        let diluc = crate::find_character("diluc").unwrap();
        let mult = diluc.talent_multiplier(DamageType::Skill, 0, 9, 3).unwrap();
        assert!((mult - 1.8880).abs() < 1e-4);
    }

    #[test]
    fn test_talent_multiplier_ganyu_burst_c3() {
        let ganyu = crate::find_character("ganyu").unwrap();
        let no_boost = ganyu.talent_multiplier(DamageType::Burst, 0, 9, 0).unwrap();
        let with_c3 = ganyu.talent_multiplier(DamageType::Burst, 0, 9, 3).unwrap();
        assert!(with_c3 > no_boost);
    }

    #[test]
    fn test_talent_multiplier_out_of_range() {
        let diluc = crate::find_character("diluc").unwrap();
        assert!(
            diluc
                .talent_multiplier(DamageType::Skill, 999, 9, 0)
                .is_none()
        );
    }

    // -- build_damage_input tests --

    #[test]
    fn test_build_damage_input_diluc_skill() {
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
        let diluc = crate::find_character("diluc").unwrap();
        let stats = Stats {
            atk: 2000.0,
            ..Stats::default()
        };

        let input = diluc
            .build_damage_input(
                stats,
                90,
                DamageType::Skill,
                0,
                9,
                0,
                Some(Reaction::Vaporize),
                0.15,
            )
            .unwrap();

        assert_eq!(input.reaction, Some(Reaction::Vaporize));
        assert!((input.reaction_bonus - 0.15).abs() < 1e-4);
    }

    #[test]
    fn test_build_damage_input_with_constellation() {
        let diluc = crate::find_character("diluc").unwrap();
        let stats = Stats::default();

        let input = diluc
            .build_damage_input(stats, 90, DamageType::Skill, 0, 9, 3, None, 0.0)
            .unwrap();

        assert!((input.talent_multiplier - 1.8880).abs() < 1e-4);
    }

    #[test]
    fn test_build_damage_input_normal_physical() {
        let diluc = crate::find_character("diluc").unwrap();
        let stats = Stats::default();

        let input = diluc
            .build_damage_input(stats, 90, DamageType::Normal, 0, 9, 0, None, 0.0)
            .unwrap();

        assert_eq!(input.element, None);
        assert_eq!(input.scaling_stat, ScalingStat::Atk);
    }

    #[test]
    fn test_build_damage_input_out_of_range() {
        let diluc = crate::find_character("diluc").unwrap();
        let stats = Stats::default();

        assert!(
            diluc
                .build_damage_input(stats, 90, DamageType::Skill, 999, 9, 0, None, 0.0)
                .is_none()
        );
    }

    // -- E2E integration tests --

    #[test]
    fn test_e2e_build_damage_input_to_calculate_damage() {
        use genshin_calc_core::{Enemy, calculate_damage};

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
            def_ignore: 0.0,
        };

        let result = calculate_damage(&input, &enemy).unwrap();

        assert!(result.non_crit > 0.0);
        assert!(result.crit > result.non_crit);
        assert!(result.average > result.non_crit);
        assert!(result.average < result.crit);

        // Golden value: 2000 * 1.6048 * (1 + 0.466) * 0.5 * 0.9 = 2117.3731
        assert!((result.non_crit - 2117.3731).abs() < 1.0);
    }

    #[test]
    fn test_e2e_with_vaporize() {
        use genshin_calc_core::{Enemy, calculate_damage};

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
                stats,
                90,
                DamageType::Skill,
                0,
                9,
                0,
                Some(Reaction::Vaporize),
                0.15,
            )
            .unwrap();

        let enemy = Enemy {
            level: 90,
            resistance: 0.10,
            def_reduction: 0.0,
            def_ignore: 0.0,
        };

        let result = calculate_damage(&input, &enemy).unwrap();
        assert!(result.non_crit > 2117.0);
        assert_eq!(result.reaction, Some(Reaction::Vaporize));
    }

    // -- Data integrity tests --

    #[test]
    fn test_all_characters_have_valid_talent_data() {
        // Characters whose burst is a self-buff or heal with no damage scalings.
        let no_burst_scalings: &[&str] = &["mika", "nahida", "xiao"];

        for c in crate::characters::all_characters() {
            assert!(
                !c.talents.normal_attack.hits.is_empty(),
                "{} has no normal attack hits",
                c.name
            );
            assert!(
                !c.talents.elemental_skill.scalings.is_empty(),
                "{} has no skill scalings",
                c.name
            );
            if !no_burst_scalings.contains(&c.id) {
                assert!(
                    !c.talents.elemental_burst.scalings.is_empty(),
                    "{} has no burst scalings",
                    c.name
                );
            }

            assert!(
                c.talent_multiplier(DamageType::Normal, 0, 1, 0).is_some(),
                "{} normal talent_multiplier failed",
                c.name
            );
            assert!(
                c.talent_multiplier(DamageType::Skill, 0, 1, 0).is_some(),
                "{} skill talent_multiplier failed",
                c.name
            );
            if !no_burst_scalings.contains(&c.id) {
                assert!(
                    c.talent_multiplier(DamageType::Burst, 0, 1, 0).is_some(),
                    "{} burst talent_multiplier failed",
                    c.name
                );
            }
        }
    }

    // -- DynamicTalentBonus tests --

    #[test]
    fn test_dynamic_bonus_serialize_some() {
        let mavuika = crate::find_character("mavuika").unwrap();
        let burst = mavuika.talent_scaling(DamageType::Burst, 0).unwrap();
        let bonus = burst.dynamic_bonus.unwrap();
        assert_eq!(bonus.name, "闘志");
        assert_eq!(bonus.max_stacks, 200);
        assert!((bonus.per_stack[9] - 0.0288).abs() < 1e-6); // Lv10

        let json = serde_json::to_string(burst).unwrap();
        assert!(json.contains("\"dynamic_bonus\""));
        assert!(json.contains("\"闘志\""));
        assert!(json.contains("\"max_stacks\":200"));
    }

    #[test]
    fn test_dynamic_bonus_serialize_none() {
        let diluc = crate::find_character("diluc").unwrap();
        let skill = diluc.talent_scaling(DamageType::Skill, 0).unwrap();
        assert!(skill.dynamic_bonus.is_none());

        let json = serde_json::to_string(skill).unwrap();
        assert!(json.contains("\"dynamic_bonus\":null"));
    }

    #[test]
    fn test_mavuika_burst_with_fighting_spirit() {
        // Lv10, 闘志200: 8.0064 + 200 × 0.0288 = 13.7664
        let mavuika = crate::find_character("mavuika").unwrap();
        let burst = mavuika.talent_scaling(DamageType::Burst, 0).unwrap();
        let bonus = burst.dynamic_bonus.unwrap();
        let talent_level: usize = 10;
        let stacks: f64 = 200.0;
        let adjusted = burst.values[talent_level - 1] + stacks * bonus.per_stack[talent_level - 1];
        assert!((adjusted - 13.7664).abs() < 1e-4);
    }

    #[test]
    fn test_raiden_burst_with_resolve() {
        // 初撃 Lv10, 願力60: 7.2144 + 60 × 0.069984 = 11.41344
        let raiden = crate::find_character("raiden_shogun").unwrap();
        let musou = raiden.talent_scaling(DamageType::Burst, 0).unwrap();
        let bonus = musou.dynamic_bonus.unwrap();
        assert_eq!(bonus.max_stacks, 60);
        let adjusted = musou.values[9] + 60.0 * bonus.per_stack[9];
        assert!((adjusted - 11.41344).abs() < 1e-4);
    }

    #[test]
    fn test_raiden_burst_normal_with_resolve() {
        // 通常1段 Lv10, 願力60: 0.7982 + 60 × 0.013071 = 1.58246
        let raiden = crate::find_character("raiden_shogun").unwrap();
        let n1 = raiden.talent_scaling(DamageType::Burst, 1).unwrap();
        let bonus = n1.dynamic_bonus.unwrap();
        let adjusted = n1.values[9] + 60.0 * bonus.per_stack[9];
        assert!((adjusted - 1.58246).abs() < 1e-4);
    }

    #[test]
    fn test_verified_characters_constellation_patterns() {
        let cases = [
            ("freminet", ConstellationPattern::C3SkillC5Burst),
            ("diluc", ConstellationPattern::C3SkillC5Burst),
            ("ganyu", ConstellationPattern::C3BurstC5Skill),
            ("raiden_shogun", ConstellationPattern::C3BurstC5Skill),
            ("yanfei", ConstellationPattern::C3SkillC5Burst),
        ];
        for (id, expected_pattern) in cases {
            let c = crate::find_character(id).unwrap();
            assert_eq!(
                c.constellation_pattern, expected_pattern,
                "{} has wrong constellation pattern",
                c.name
            );
        }
    }
}
