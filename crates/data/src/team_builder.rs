use genshin_calc_core::{
    BuffTarget, CalcError, ResolvedBuff, StatProfile, TeamMember,
};

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
                        let talent_idx = match def.source {
                            crate::talent_buffs::TalentBuffSource::ElementalSkill => 1,
                            crate::talent_buffs::TalentBuffSource::ElementalBurst => 2,
                            _ => 0,
                        };
                        let level = self.talent_levels[talent_idx];
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

        Ok(TeamMember {
            element: char_data.element,
            weapon_type: char_data.weapon_type,
            stats: profile,
            buffs_provided: buffs,
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
            member.buffs_provided.iter().any(|b| b.source.contains("2pc")),
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
}
