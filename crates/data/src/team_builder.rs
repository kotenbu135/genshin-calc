use genshin_calc_core::{
    BuffTarget, BuffableStat, CalcError, DamageType, Element, ResolvedBuff, StatProfile,
    TeamMember, WeaponType,
};

use crate::buff::{
    Activation, AutoCondition, AvailableConditional, ConditionalBuff, ManualActivation,
    ManualCondition,
};
use crate::moonsign_chars::is_moonsign_character;
use crate::talent_buffs::find_talent_buffs;
use crate::types::{ArtifactSet, AscensionStat, CharacterData, WeaponData, WeaponSubStat};

/// Builder for constructing a [`TeamMember`] from game data.
pub struct TeamMemberBuilder {
    character: &'static CharacterData,
    weapon: &'static WeaponData,
    artifact_set: Option<&'static ArtifactSet>,
    artifact_stats: StatProfile,
    constellation: u8,
    talent_levels: [u8; 3],
    manual_activations: Vec<(&'static str, ManualActivation)>,
    team_elements: Vec<Element>,
}

impl TeamMemberBuilder {
    /// Creates a new builder with a character and weapon.
    ///
    /// Defaults: no artifact set, empty artifact stats, constellation 0, talents [1, 1, 1].
    pub fn new(character: &'static CharacterData, weapon: &'static WeaponData) -> Self {
        Self {
            character,
            weapon,
            artifact_set: None,
            artifact_stats: StatProfile::default(),
            constellation: 0,
            talent_levels: [1, 1, 1],
            manual_activations: Vec::new(),
            team_elements: Vec::new(),
        }
    }

    /// Sets the artifact set (4-piece).
    pub fn artifact_set(mut self, set: &'static ArtifactSet) -> Self {
        self.artifact_set = Some(set);
        self
    }

    /// Sets the artifact substats.
    pub fn artifact_stats(mut self, stats: StatProfile) -> Self {
        self.artifact_stats = stats;
        self
    }

    /// Sets the constellation level (0-6).
    pub fn constellation(mut self, c: u8) -> Self {
        self.constellation = c;
        self
    }

    /// Sets talent levels [normal, skill, burst] (1-15 each).
    pub fn talent_levels(mut self, levels: [u8; 3]) -> Self {
        self.talent_levels = levels;
        self
    }

    /// Activate a manual conditional buff by name.
    pub fn activate(mut self, name: &'static str) -> Self {
        self.manual_activations
            .push((name, ManualActivation::Active));
        self
    }

    /// Activate a stackable conditional buff with stack count.
    pub fn activate_with_stacks(mut self, name: &'static str, stacks: u8) -> Self {
        self.manual_activations
            .push((name, ManualActivation::Stacks(stacks)));
        self
    }

    /// Set team element composition for Auto team-based conditions.
    pub fn team_elements(mut self, elements: Vec<Element>) -> Self {
        self.team_elements = elements;
        self
    }

    /// Returns available conditional buffs with source context.
    pub fn available_conditionals(&self) -> Vec<AvailableConditional> {
        let mut result = Vec::new();
        if let Some(passive) = &self.weapon.passive {
            for buff in passive.effect.conditional_buffs {
                result.push(AvailableConditional {
                    source: self.weapon.name,
                    buff,
                });
            }
        }
        if let Some(set) = self.artifact_set {
            for buff in set.two_piece.conditional_buffs {
                result.push(AvailableConditional {
                    source: set.name,
                    buff,
                });
            }
            for buff in set.four_piece.conditional_buffs {
                result.push(AvailableConditional {
                    source: set.name,
                    buff,
                });
            }
        }
        result
    }

    /// Builds the [`TeamMember`].
    ///
    /// # Errors
    ///
    /// Returns [`CalcError::InvalidConstellation`] if constellation > 6.
    /// Returns [`CalcError::InvalidTalentLevel`] if any talent level is 0 or > 15.
    pub fn build(self) -> Result<TeamMember, CalcError> {
        if self.constellation > 6 {
            return Err(CalcError::InvalidConstellation(self.constellation));
        }
        for &level in &self.talent_levels {
            if level == 0 || level > 15 {
                return Err(CalcError::InvalidTalentLevel(level));
            }
        }

        let char_data = self.character;
        let weapon = self.weapon;

        // 1. Base stats (Lv90 = index 3)
        let mut profile = StatProfile {
            base_hp: char_data.base_hp[3],
            base_atk: char_data.base_atk[3] + weapon.base_atk[3],
            base_def: char_data.base_def[3],
            ..Default::default()
        };

        // 2. Character ascension stat
        apply_ascension_stat(&mut profile, &char_data.ascension_stat);

        // 3. Weapon sub-stat
        if let Some(sub) = &weapon.sub_stat {
            apply_weapon_sub_stat(&mut profile, sub);
        }

        // 4. Artifact stats (user input) merged
        profile.hp_percent += self.artifact_stats.hp_percent;
        profile.atk_percent += self.artifact_stats.atk_percent;
        profile.def_percent += self.artifact_stats.def_percent;
        profile.hp_flat += self.artifact_stats.hp_flat;
        profile.atk_flat += self.artifact_stats.atk_flat;
        profile.def_flat += self.artifact_stats.def_flat;
        profile.elemental_mastery += self.artifact_stats.elemental_mastery;
        profile.crit_rate += self.artifact_stats.crit_rate;
        profile.crit_dmg += self.artifact_stats.crit_dmg;
        profile.energy_recharge += self.artifact_stats.energy_recharge;
        profile.dmg_bonus += self.artifact_stats.dmg_bonus;

        // 5-7. Collect buffs
        let mut buffs = Vec::new();

        // Weapon passive
        if let Some(passive) = &weapon.passive {
            for stat_buff in passive.effect.buffs {
                buffs.push(ResolvedBuff {
                    source: format!("{} ({})", passive.name, weapon.name),
                    stat: stat_buff.stat,
                    value: stat_buff.value,
                    target: BuffTarget::OnlySelf,
                });
            }
        }

        // Artifact set effects
        if let Some(set) = self.artifact_set {
            for stat_buff in set.two_piece.buffs {
                buffs.push(ResolvedBuff {
                    source: format!("{} 2pc", set.name),
                    stat: stat_buff.stat,
                    value: stat_buff.value,
                    target: BuffTarget::OnlySelf,
                });
            }
            for stat_buff in set.four_piece.buffs {
                buffs.push(ResolvedBuff {
                    source: format!("{} 4pc", set.name),
                    stat: stat_buff.stat,
                    value: stat_buff.value,
                    target: BuffTarget::OnlySelf,
                });
            }
        }

        // 8. Talent buffs
        if let Some(talent_defs) = find_talent_buffs(char_data.id) {
            for def in talent_defs {
                if self.constellation < def.min_constellation {
                    continue;
                }
                // Get scaling value based on talent level
                let raw_value = if def.scales_with_talent {
                    if let Some(scaling) = def.talent_scaling {
                        let (talent_idx, damage_type) = match def.source {
                            crate::talent_buffs::TalentBuffSource::ElementalSkill => {
                                (1, DamageType::Skill)
                            }
                            crate::talent_buffs::TalentBuffSource::ElementalBurst => {
                                (2, DamageType::Burst)
                            }
                            // AscensionPassive and Constellation(u8) map to Normal,
                            // which is never boosted — these sources don't benefit from C3/C5.
                            _ => (0, DamageType::Normal),
                        };
                        let base_level = self.talent_levels[talent_idx];
                        let level = char_data.effective_talent_level(
                            damage_type,
                            base_level,
                            self.constellation,
                        );
                        scaling[(level - 1) as usize]
                    } else {
                        def.base_value
                    }
                } else {
                    def.base_value
                };

                // If scales_on is set, multiply by the provider's base stat
                let value = if let Some(scaling_stat) = def.scales_on {
                    let base = match scaling_stat {
                        genshin_calc_core::ScalingStat::Atk => profile.base_atk,
                        genshin_calc_core::ScalingStat::Hp => profile.base_hp,
                        genshin_calc_core::ScalingStat::Def => profile.base_def,
                        genshin_calc_core::ScalingStat::Em => profile.elemental_mastery,
                    };
                    base * raw_value
                } else {
                    raw_value
                };

                buffs.push(ResolvedBuff {
                    source: def.name.to_string(),
                    stat: def.stat,
                    value,
                    target: def.target,
                });
            }
        }

        // 9. Resolve conditional buffs
        // TODO(P4): use refinement_values[r] when refinement level is available
        let resolve_conditionals =
            |conditional_buffs: &'static [ConditionalBuff],
             source_name: &str,
             target: BuffTarget,
             buffs: &mut Vec<ResolvedBuff>| {
                for cond_buff in conditional_buffs {
                    let resolved_value = match &cond_buff.activation {
                        Activation::Auto(auto) => eval_auto(
                            auto,
                            cond_buff.value,
                            &profile,
                            char_data.weapon_type,
                            char_data.element,
                            &self.team_elements,
                        ),
                        Activation::Manual(manual) => eval_manual(
                            manual,
                            cond_buff.name,
                            cond_buff.value,
                            &self.manual_activations,
                        ),
                        Activation::Both(auto, manual) => eval_auto(
                            auto,
                            cond_buff.value,
                            &profile,
                            char_data.weapon_type,
                            char_data.element,
                            &self.team_elements,
                        )
                        .and_then(|auto_value| {
                            eval_manual(
                                manual,
                                cond_buff.name,
                                auto_value,
                                &self.manual_activations,
                            )
                        }),
                    };

                    if let Some(value) = resolved_value {
                        buffs.push(ResolvedBuff {
                            source: format!("{} ({})", cond_buff.name, source_name),
                            stat: cond_buff.stat,
                            value,
                            target,
                        });
                    }
                }
            };

        // Weapon conditional buffs
        if let Some(passive) = &weapon.passive {
            resolve_conditionals(
                passive.effect.conditional_buffs,
                weapon.name,
                BuffTarget::OnlySelf,
                &mut buffs,
            );
        }

        // Artifact conditional buffs
        if let Some(set) = self.artifact_set {
            resolve_conditionals(
                set.two_piece.conditional_buffs,
                &format!("{} 2pc", set.name),
                BuffTarget::OnlySelf,
                &mut buffs,
            );
            resolve_conditionals(
                set.four_piece.conditional_buffs,
                &format!("{} 4pc", set.name),
                BuffTarget::OnlySelf,
                &mut buffs,
            );
        }

        Ok(TeamMember {
            element: char_data.element,
            weapon_type: char_data.weapon_type,
            stats: profile,
            buffs_provided: buffs,
            is_moonsign: is_moonsign_character(char_data.id),
        })
    }
}

fn apply_ascension_stat(profile: &mut StatProfile, stat: &AscensionStat) {
    match stat {
        AscensionStat::Hp(v) => profile.hp_percent += v,
        AscensionStat::Atk(v) => profile.atk_percent += v,
        AscensionStat::Def(v) => profile.def_percent += v,
        AscensionStat::CritRate(v) => profile.crit_rate += v,
        AscensionStat::CritDmg(v) => profile.crit_dmg += v,
        AscensionStat::ElementalMastery(v) => profile.elemental_mastery += v,
        AscensionStat::EnergyRecharge(v) => profile.energy_recharge += v,
        AscensionStat::ElementalDmgBonus(_, v) => profile.dmg_bonus += v,
        AscensionStat::PhysicalDmgBonus(v) => profile.dmg_bonus += v,
        AscensionStat::HealingBonus(_) => {} // No effect on damage calculation
    }
}

/// Reads the relevant stat value from a profile for StatScaling.
/// BuffableStat here indicates the "stat family" — see AutoCondition::StatScaling doc.
fn read_stat_for_scaling(stat: &BuffableStat, profile: &StatProfile) -> f64 {
    match stat {
        BuffableStat::HpPercent => profile.base_hp * (1.0 + profile.hp_percent) + profile.hp_flat,
        BuffableStat::AtkPercent => {
            profile.base_atk * (1.0 + profile.atk_percent) + profile.atk_flat
        }
        BuffableStat::DefPercent => {
            profile.base_def * (1.0 + profile.def_percent) + profile.def_flat
        }
        BuffableStat::ElementalMastery => profile.elemental_mastery,
        BuffableStat::EnergyRecharge => profile.energy_recharge,
        _ => 0.0,
    }
}

/// Evaluates an Auto condition. Returns Some(value) if condition is met.
fn eval_auto(
    cond: &AutoCondition,
    multiplier: f64,
    profile: &StatProfile,
    weapon_type: WeaponType,
    element: Element,
    team_elements: &[Element],
) -> Option<f64> {
    match cond {
        AutoCondition::WeaponTypeRequired(types) => {
            if types.contains(&weapon_type) {
                Some(multiplier)
            } else {
                None
            }
        }
        AutoCondition::ElementRequired(elements) => {
            if elements.contains(&element) {
                Some(multiplier)
            } else {
                None
            }
        }
        AutoCondition::StatScaling { stat, cap } => {
            let stat_val = read_stat_for_scaling(stat, profile);
            let raw = stat_val * multiplier;
            Some(cap.map_or(raw, |c| raw.min(c)))
        }
        AutoCondition::TeamElementCount {
            element: elem,
            min_count,
        } => {
            if team_elements.is_empty() {
                return None;
            }
            let count = team_elements.iter().filter(|e| *e == elem).count() as u8;
            if count >= *min_count {
                Some(multiplier)
            } else {
                None
            }
        }
        AutoCondition::TeamElementsOnly(allowed) => {
            if team_elements.is_empty() {
                return None;
            }
            if team_elements.iter().all(|e| allowed.contains(e)) {
                Some(multiplier)
            } else {
                None
            }
        }
    }
}

/// Evaluates a Manual condition. Returns Some(value) if user activated it.
fn eval_manual(
    cond: &ManualCondition,
    buff_name: &str,
    value: f64,
    activations: &[(&str, ManualActivation)],
) -> Option<f64> {
    let activation = activations.iter().find(|(name, _)| *name == buff_name);
    match cond {
        ManualCondition::Toggle => match activation {
            Some((_, ManualActivation::Active)) => Some(value),
            _ => None, // Stacks mismatch or not present
        },
        ManualCondition::Stacks(max) => match activation {
            Some((_, ManualActivation::Stacks(n))) => {
                let effective = (*n).min(*max);
                Some(value * f64::from(effective))
            }
            Some((_, ManualActivation::Active)) => {
                Some(value * f64::from(*max)) // Active on Stacks → max stacks
            }
            _ => None,
        },
    }
}

fn apply_weapon_sub_stat(profile: &mut StatProfile, sub: &WeaponSubStat) {
    // Lv90 = index 3
    match sub {
        WeaponSubStat::HpPercent(v) => profile.hp_percent += v[3],
        WeaponSubStat::AtkPercent(v) => profile.atk_percent += v[3],
        WeaponSubStat::DefPercent(v) => profile.def_percent += v[3],
        WeaponSubStat::CritRate(v) => profile.crit_rate += v[3],
        WeaponSubStat::CritDmg(v) => profile.crit_dmg += v[3],
        WeaponSubStat::ElementalMastery(v) => profile.elemental_mastery += v[3],
        WeaponSubStat::EnergyRecharge(v) => profile.energy_recharge += v[3],
        WeaponSubStat::PhysicalDmgBonus(v) => profile.dmg_bonus += v[3],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{find_artifact_set, find_character, find_weapon};

    const EPSILON: f64 = 1e-4;

    #[test]
    fn test_basic_build() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let member = TeamMemberBuilder::new(bennett, weapon).build().unwrap();
        assert_eq!(member.element, genshin_calc_core::Element::Pyro);
        assert!(member.stats.base_atk > 0.0);
        assert!(member.stats.base_hp > 0.0);
    }

    #[test]
    fn test_bennett_burst_buff_at_lv13() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let member = TeamMemberBuilder::new(bennett, weapon)
            .talent_levels([1, 1, 13])
            .build()
            .unwrap();

        // Bennett burst buff should be in buffs_provided
        let burst_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Fantastic Voyage"))
            .unwrap();
        // Lv13 = index 12 = 1.19 (multiplier of base ATK)
        // Expected value = base_atk * 1.19
        let expected = member.stats.base_atk * 1.19;
        assert!((burst_buff.value - expected).abs() < EPSILON);
        assert_eq!(burst_buff.target, BuffTarget::Team);
    }

    #[test]
    fn test_artifact_set_buffs() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let noblesse = find_artifact_set("noblesse_oblige").unwrap();
        let member = TeamMemberBuilder::new(bennett, weapon)
            .artifact_set(noblesse)
            .build()
            .unwrap();

        // Should have weapon passive + noblesse 2pc/4pc buffs
        assert!(
            member
                .buffs_provided
                .iter()
                .any(|b| b.source.contains("2pc")),
            "Should have 2pc buff"
        );
    }

    #[test]
    fn test_invalid_constellation() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let result = TeamMemberBuilder::new(bennett, weapon)
            .constellation(7)
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_talent_level() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let result = TeamMemberBuilder::new(bennett, weapon)
            .talent_levels([0, 1, 1])
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_non_moonsign_character_detected() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let member = TeamMemberBuilder::new(bennett, weapon).build().unwrap();
        assert!(!member.is_moonsign);
    }

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
}

#[cfg(test)]
mod conditional_tests {
    use super::*;
    use crate::buff::*;
    use crate::{find_artifact_set, find_character, find_weapon};

    const EPSILON: f64 = 1e-6;

    // --- eval_auto tests ---

    #[test]
    fn test_eval_auto_weapon_type_match() {
        let cond = AutoCondition::WeaponTypeRequired(&[WeaponType::Sword, WeaponType::Claymore]);
        let result = eval_auto(
            &cond,
            0.35,
            &StatProfile::default(),
            WeaponType::Sword,
            Element::Pyro,
            &[],
        );
        assert!((result.unwrap() - 0.35).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_weapon_type_mismatch() {
        let cond = AutoCondition::WeaponTypeRequired(&[WeaponType::Sword, WeaponType::Claymore]);
        let result = eval_auto(
            &cond,
            0.35,
            &StatProfile::default(),
            WeaponType::Catalyst,
            Element::Pyro,
            &[],
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_element_match() {
        let cond = AutoCondition::ElementRequired(&[Element::Pyro, Element::Hydro]);
        let result = eval_auto(
            &cond,
            0.20,
            &StatProfile::default(),
            WeaponType::Sword,
            Element::Pyro,
            &[],
        );
        assert!((result.unwrap() - 0.20).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_element_mismatch() {
        let cond = AutoCondition::ElementRequired(&[Element::Pyro]);
        let result = eval_auto(
            &cond,
            0.20,
            &StatProfile::default(),
            WeaponType::Sword,
            Element::Cryo,
            &[],
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_stat_scaling_normal() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::EnergyRecharge,
            cap: Some(0.75),
        };
        let profile = StatProfile {
            energy_recharge: 1.80,
            ..Default::default()
        };
        // 1.80 * 0.25 = 0.45 (below cap)
        let result = eval_auto(&cond, 0.25, &profile, WeaponType::Sword, Element::Pyro, &[]);
        assert!((result.unwrap() - 0.45).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_stat_scaling_capped() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::EnergyRecharge,
            cap: Some(0.75),
        };
        let profile = StatProfile {
            energy_recharge: 4.00,
            ..Default::default()
        };
        // 4.00 * 0.25 = 1.00 → capped at 0.75
        let result = eval_auto(&cond, 0.25, &profile, WeaponType::Sword, Element::Pyro, &[]);
        assert!((result.unwrap() - 0.75).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_stat_scaling_hp() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::HpPercent,
            cap: None,
        };
        let profile = StatProfile {
            base_hp: 15000.0,
            hp_percent: 0.466,
            hp_flat: 4780.0,
            ..Default::default()
        };
        // total_hp = 15000 * 1.466 + 4780 = 26770.0; 26770.0 * 0.008 = 214.16
        let result = eval_auto(
            &cond,
            0.008,
            &profile,
            WeaponType::Polearm,
            Element::Pyro,
            &[],
        );
        assert!((result.unwrap() - 214.16).abs() < 0.01);
    }

    #[test]
    fn test_eval_auto_team_element_count_pass() {
        let cond = AutoCondition::TeamElementCount {
            element: Element::Geo,
            min_count: 3,
        };
        let team = vec![Element::Geo, Element::Geo, Element::Geo, Element::Pyro];
        let result = eval_auto(
            &cond,
            0.25,
            &StatProfile::default(),
            WeaponType::Bow,
            Element::Geo,
            &team,
        );
        assert!((result.unwrap() - 0.25).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_team_element_count_fail() {
        let cond = AutoCondition::TeamElementCount {
            element: Element::Geo,
            min_count: 3,
        };
        let team = vec![Element::Geo, Element::Geo, Element::Pyro, Element::Hydro];
        let result = eval_auto(
            &cond,
            0.25,
            &StatProfile::default(),
            WeaponType::Bow,
            Element::Geo,
            &team,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_team_element_count_empty_skips() {
        let cond = AutoCondition::TeamElementCount {
            element: Element::Geo,
            min_count: 3,
        };
        let result = eval_auto(
            &cond,
            0.25,
            &StatProfile::default(),
            WeaponType::Bow,
            Element::Geo,
            &[],
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_team_elements_only_pass() {
        let cond = AutoCondition::TeamElementsOnly(&[Element::Hydro, Element::Dendro]);
        let team = vec![
            Element::Hydro,
            Element::Dendro,
            Element::Hydro,
            Element::Dendro,
        ];
        let result = eval_auto(
            &cond,
            0.10,
            &StatProfile::default(),
            WeaponType::Sword,
            Element::Hydro,
            &team,
        );
        assert!((result.unwrap() - 0.10).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_team_elements_only_fail() {
        let cond = AutoCondition::TeamElementsOnly(&[Element::Hydro, Element::Dendro]);
        let team = vec![
            Element::Hydro,
            Element::Dendro,
            Element::Pyro,
            Element::Dendro,
        ];
        let result = eval_auto(
            &cond,
            0.10,
            &StatProfile::default(),
            WeaponType::Sword,
            Element::Hydro,
            &team,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_team_elements_only_empty_skips() {
        let cond = AutoCondition::TeamElementsOnly(&[Element::Hydro, Element::Dendro]);
        let result = eval_auto(
            &cond,
            0.10,
            &StatProfile::default(),
            WeaponType::Sword,
            Element::Hydro,
            &[],
        );
        assert!(result.is_none());
    }

    // --- eval_manual tests ---

    #[test]
    fn test_eval_manual_toggle_active() {
        let result = eval_manual(
            &ManualCondition::Toggle,
            "test_buff",
            0.15,
            &[("test_buff", ManualActivation::Active)],
        );
        assert!((result.unwrap() - 0.15).abs() < EPSILON);
    }

    #[test]
    fn test_eval_manual_toggle_not_present() {
        let result = eval_manual(&ManualCondition::Toggle, "test_buff", 0.15, &[]);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_manual_toggle_stacks_mismatch() {
        let result = eval_manual(
            &ManualCondition::Toggle,
            "test_buff",
            0.15,
            &[("test_buff", ManualActivation::Stacks(2))],
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_manual_stacks_normal() {
        let result = eval_manual(
            &ManualCondition::Stacks(3),
            "test_buff",
            0.075,
            &[("test_buff", ManualActivation::Stacks(2))],
        );
        assert!((result.unwrap() - 0.15).abs() < EPSILON); // 0.075 * 2
    }

    #[test]
    fn test_eval_manual_stacks_exceeds_max() {
        let result = eval_manual(
            &ManualCondition::Stacks(3),
            "test_buff",
            0.075,
            &[("test_buff", ManualActivation::Stacks(5))],
        );
        assert!((result.unwrap() - 0.225).abs() < EPSILON); // 0.075 * 3 (capped)
    }

    #[test]
    fn test_eval_manual_stacks_not_present() {
        let result = eval_manual(&ManualCondition::Stacks(3), "test_buff", 0.075, &[]);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_manual_stacks_with_active_treated_as_max() {
        let result = eval_manual(
            &ManualCondition::Stacks(3),
            "test_buff",
            0.075,
            &[("test_buff", ManualActivation::Active)],
        );
        assert!((result.unwrap() - 0.225).abs() < EPSILON); // 0.075 * 3 (max)
    }

    // --- Activation::Both tests (unit level) ---

    #[test]
    fn test_both_auto_pass_manual_pass() {
        let auto = AutoCondition::StatScaling {
            stat: BuffableStat::HpPercent,
            cap: None,
        };
        let manual = ManualCondition::Toggle;
        let profile = StatProfile {
            base_hp: 20000.0,
            hp_percent: 0.0,
            ..Default::default()
        };
        // eval_auto: 20000 * 0.01 = 200.0
        let auto_result = eval_auto(
            &auto,
            0.01,
            &profile,
            WeaponType::Polearm,
            Element::Pyro,
            &[],
        );
        assert!(auto_result.is_some());
        // eval_manual with auto_value as input
        let result = auto_result.and_then(|auto_value| {
            eval_manual(
                &manual,
                "test",
                auto_value,
                &[("test", ManualActivation::Active)],
            )
        });
        assert!((result.unwrap() - 200.0).abs() < EPSILON);
    }

    #[test]
    fn test_both_auto_fail_manual_pass() {
        let auto = AutoCondition::WeaponTypeRequired(&[WeaponType::Sword]);
        let manual = ManualCondition::Toggle;
        let auto_result = eval_auto(
            &auto,
            0.35,
            &StatProfile::default(),
            WeaponType::Catalyst,
            Element::Pyro,
            &[],
        );
        assert!(auto_result.is_none());
        let result = auto_result
            .and_then(|v| eval_manual(&manual, "test", v, &[("test", ManualActivation::Active)]));
        assert!(result.is_none());
    }

    #[test]
    fn test_both_auto_pass_manual_fail() {
        let auto = AutoCondition::WeaponTypeRequired(&[WeaponType::Sword]);
        let manual = ManualCondition::Toggle;
        let auto_result = eval_auto(
            &auto,
            0.35,
            &StatProfile::default(),
            WeaponType::Sword,
            Element::Pyro,
            &[],
        );
        assert!(auto_result.is_some());
        let result = auto_result.and_then(|v| {
            eval_manual(&manual, "test", v, &[]) // not activated
        });
        assert!(result.is_none());
    }

    // --- Builder API tests ---

    #[test]
    fn test_builder_activate_noop_for_unknown_name() {
        let char = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let _member = TeamMemberBuilder::new(char, weapon)
            .activate("nonexistent_buff")
            .build()
            .unwrap();
    }

    #[test]
    fn test_builder_activate_with_stacks_noop_for_unknown_name() {
        let char = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let _member = TeamMemberBuilder::new(char, weapon)
            .activate_with_stacks("nonexistent_buff", 2)
            .build()
            .unwrap();
    }

    #[test]
    fn test_builder_team_elements() {
        let char = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let _member = TeamMemberBuilder::new(char, weapon)
            .team_elements(vec![
                Element::Pyro,
                Element::Hydro,
                Element::Cryo,
                Element::Dendro,
            ])
            .build()
            .unwrap();
    }

    #[test]
    fn test_builder_available_conditionals_empty() {
        let char = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let builder = TeamMemberBuilder::new(char, weapon);
        assert!(builder.available_conditionals().is_empty());
    }

    // --- Integration tests: Gladiator, Emblem, CW ---

    #[test]
    fn test_gladiator_4pc_sword_gets_normal_bonus() {
        let char = find_character("bennett").unwrap(); // Sword
        let weapon = find_weapon("aquila_favonia").unwrap();
        let glad = find_artifact_set("gladiators_finale").unwrap();
        let member = TeamMemberBuilder::new(char, weapon)
            .artifact_set(glad)
            .build()
            .unwrap();
        let buff = member
            .buffs_provided
            .iter()
            .find(|b| b.stat == BuffableStat::NormalAtkDmgBonus);
        assert!(buff.is_some());
        assert!((buff.unwrap().value - 0.35).abs() < EPSILON);
    }

    #[test]
    fn test_gladiator_4pc_catalyst_no_bonus() {
        let char = find_character("nahida").unwrap(); // Catalyst
        let weapon = find_weapon("a_thousand_floating_dreams").unwrap();
        let glad = find_artifact_set("gladiators_finale").unwrap();
        let member = TeamMemberBuilder::new(char, weapon)
            .artifact_set(glad)
            .build()
            .unwrap();
        let buff = member
            .buffs_provided
            .iter()
            .find(|b| b.stat == BuffableStat::NormalAtkDmgBonus);
        assert!(buff.is_none());
    }

    #[test]
    fn test_emblem_4pc_burst_bonus_from_er() {
        let char = find_character("raiden_shogun").unwrap();
        let weapon = find_weapon("engulfing_lightning").unwrap();
        let emblem = find_artifact_set("emblem_of_severed_fate").unwrap();
        let member = TeamMemberBuilder::new(char, weapon)
            .artifact_stats(StatProfile {
                energy_recharge: 0.518,
                ..Default::default()
            })
            .artifact_set(emblem)
            .build()
            .unwrap();
        let buff = member
            .buffs_provided
            .iter()
            .find(|b| b.stat == BuffableStat::BurstDmgBonus && b.source.contains("emblem"));
        assert!(buff.is_some());
        let val = buff.unwrap().value;
        assert!(val > 0.0 && val <= 0.75);
    }

    #[test]
    fn test_cw_4pc_pyro_stacks_activated() {
        let char = find_character("hu_tao").unwrap();
        let weapon = find_weapon("staff_of_homa").unwrap();
        let cw = find_artifact_set("crimson_witch").unwrap();
        let member = TeamMemberBuilder::new(char, weapon)
            .artifact_set(cw)
            .activate_with_stacks("cwof_pyro_stacks", 1)
            .build()
            .unwrap();
        let buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("cwof_pyro_stacks"));
        assert!(buff.is_some());
        assert!((buff.unwrap().value - 0.075).abs() < EPSILON); // 1 stack
    }

    #[test]
    fn test_cw_4pc_no_activation_no_buff() {
        let char = find_character("hu_tao").unwrap();
        let weapon = find_weapon("staff_of_homa").unwrap();
        let cw = find_artifact_set("crimson_witch").unwrap();
        let member = TeamMemberBuilder::new(char, weapon)
            .artifact_set(cw)
            .build()
            .unwrap();
        let buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("cwof_pyro_stacks"));
        assert!(buff.is_none());
    }

    #[test]
    fn test_available_conditionals_with_gladiator() {
        let char = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let glad = find_artifact_set("gladiators_finale").unwrap();
        let builder = TeamMemberBuilder::new(char, weapon).artifact_set(glad);
        let conditionals = builder.available_conditionals();
        assert_eq!(conditionals.len(), 1);
        assert_eq!(conditionals[0].buff.name, "gladiator_normal_bonus");
    }

    #[test]
    fn test_existing_bennett_burst_unchanged() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let member = TeamMemberBuilder::new(bennett, weapon)
            .talent_levels([1, 1, 13])
            .build()
            .unwrap();
        let burst_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Fantastic Voyage"))
            .unwrap();
        let expected = member.stats.base_atk * 1.19;
        assert!((burst_buff.value - expected).abs() < 1e-4);
    }
}
