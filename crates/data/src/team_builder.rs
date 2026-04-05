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
use crate::types::{
    ArtifactSet, ArtifactSetEntry, AscensionStat, CharacterData, WeaponData, WeaponSubStat,
};

fn weapon_stat_index(level: u32) -> usize {
    match level {
        1..=20 => 0,
        21..=40 => 1,
        41..=70 => 2,
        _ => 3,
    }
}

/// Builder for constructing a [`TeamMember`] from game data.
pub struct TeamMemberBuilder {
    character: &'static CharacterData,
    weapon: &'static WeaponData,
    artifact_sets: Vec<ArtifactSetEntry>,
    artifact_stats: StatProfile,
    constellation: u8,
    talent_levels: [u8; 3],
    manual_activations: Vec<(String, ManualActivation)>,
    team_elements: Vec<Element>,
    team_regions: Vec<crate::types::Region>,
    refinement: u8,
    character_level: u32,
    weapon_level: u32,
}

impl TeamMemberBuilder {
    /// Creates a new builder with a character and weapon.
    ///
    /// Defaults: no artifact set, empty artifact stats, constellation 0, talents [1, 1, 1], refinement 1, level 90.
    pub fn new(character: &'static CharacterData, weapon: &'static WeaponData) -> Self {
        Self {
            character,
            weapon,
            artifact_sets: Vec::new(),
            artifact_stats: StatProfile::default(),
            constellation: 0,
            talent_levels: [1, 1, 1],
            manual_activations: Vec::new(),
            team_elements: Vec::new(),
            team_regions: Vec::new(),
            refinement: 1,
            character_level: 90,
            weapon_level: 90,
        }
    }

    /// Sets the weapon refinement level (1-5).
    pub fn refinement(mut self, r: u8) -> Self {
        self.refinement = r;
        self
    }

    /// Sets the artifact sets with piece counts.
    pub fn artifact_sets(mut self, sets: Vec<ArtifactSetEntry>) -> Self {
        self.artifact_sets = sets;
        self
    }

    /// Sets a single artifact set as 4-piece (convenience method).
    pub fn artifact_set(mut self, set: &'static ArtifactSet) -> Self {
        self.artifact_sets = vec![ArtifactSetEntry {
            set,
            piece_count: 4,
        }];
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
    pub fn activate(mut self, name: &str) -> Self {
        self.manual_activations
            .push((name.to_string(), ManualActivation::Active));
        self
    }

    /// Activate a stackable conditional buff with stack count.
    pub fn activate_with_stacks(mut self, name: &str, stacks: u16) -> Self {
        self.manual_activations
            .push((name.to_string(), ManualActivation::Stacks(stacks)));
        self
    }

    /// Set team element composition for Auto team-based conditions.
    pub fn team_elements(mut self, elements: Vec<Element>) -> Self {
        self.team_elements = elements;
        self
    }

    /// Set team region composition for Auto region-based conditions.
    pub fn team_regions(mut self, regions: Vec<crate::types::Region>) -> Self {
        self.team_regions = regions;
        self
    }

    /// Sets the character level (1-100). Defaults to 90.
    pub fn character_level(mut self, level: u32) -> Self {
        self.character_level = level.clamp(1, 100);
        self
    }

    /// Sets the weapon level (1-90). Defaults to 90.
    pub fn weapon_level(mut self, level: u32) -> Self {
        self.weapon_level = level.clamp(1, 90);
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
        for entry in &self.artifact_sets {
            for buff in entry.set.two_piece.conditional_buffs {
                result.push(AvailableConditional {
                    source: entry.set.name,
                    buff,
                });
            }
            if entry.piece_count >= 4 {
                for buff in entry.set.four_piece.conditional_buffs {
                    result.push(AvailableConditional {
                        source: entry.set.name,
                        buff,
                    });
                }
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
        self.validate()?;

        // 1-3. Base stats, ascension stat, weapon sub-stat
        let mut profile = self.build_base_profile();

        // 4. Artifact stats (user input) merged
        self.merge_artifact_stats(&mut profile);

        // 5-7. Collect buffs
        let mut buffs = self.collect_static_buffs();

        // 8. Talent buffs
        self.collect_talent_buffs(&profile, &mut buffs);

        // 9. Resolve conditional buffs
        self.resolve_conditional_buffs(&profile, &mut buffs);

        Ok(self.into_team_member(profile, buffs))
    }

    fn into_team_member(self, stats: StatProfile, buffs: Vec<ResolvedBuff>) -> TeamMember {
        TeamMember {
            element: self.character.element,
            weapon_type: self.character.weapon_type,
            stats,
            buffs_provided: buffs,
            is_moonsign: is_moonsign_character(self.character.id),
        }
    }

    fn merge_artifact_stats(&self, profile: &mut StatProfile) {
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
        profile.pyro_dmg_bonus += self.artifact_stats.pyro_dmg_bonus;
        profile.hydro_dmg_bonus += self.artifact_stats.hydro_dmg_bonus;
        profile.electro_dmg_bonus += self.artifact_stats.electro_dmg_bonus;
        profile.cryo_dmg_bonus += self.artifact_stats.cryo_dmg_bonus;
        profile.dendro_dmg_bonus += self.artifact_stats.dendro_dmg_bonus;
        profile.anemo_dmg_bonus += self.artifact_stats.anemo_dmg_bonus;
        profile.geo_dmg_bonus += self.artifact_stats.geo_dmg_bonus;
        profile.physical_dmg_bonus += self.artifact_stats.physical_dmg_bonus;
    }

    fn build_base_profile(&self) -> StatProfile {
        let weapon_idx = weapon_stat_index(self.weapon_level);
        let mut profile = StatProfile {
            base_hp: self.character.base_hp_at_level(self.character_level),
            base_atk: self.character.base_atk_at_level(self.character_level)
                + self.weapon.base_atk[weapon_idx],
            base_def: self.character.base_def_at_level(self.character_level),
            crit_rate: 0.05,
            crit_dmg: 0.50,
            energy_recharge: 1.0,
            ..Default::default()
        };
        apply_ascension_stat(&mut profile, &self.character.ascension_stat);
        if let Some(sub) = &self.weapon.sub_stat {
            apply_weapon_sub_stat_at_level(&mut profile, sub, weapon_idx);
        }
        profile
    }

    fn resolve_conditionals_for_source(
        &self,
        conditional_buffs: &'static [ConditionalBuff],
        source_name: &str,
        refinement: u8,
        profile: &StatProfile,
        buffs: &mut Vec<ResolvedBuff>,
    ) {
        for cond_buff in conditional_buffs {
            let effective_value =
                resolve_value(cond_buff.value, cond_buff.refinement_values, refinement);
            let resolved_value = match &cond_buff.activation {
                Activation::Auto(auto) => eval_auto(
                    auto,
                    effective_value,
                    profile,
                    self.character.weapon_type,
                    self.character.element,
                    &self.team_elements,
                    &self.team_regions,
                    refinement,
                ),
                Activation::Manual(manual) => {
                    eval_manual(manual, cond_buff, &self.manual_activations, effective_value)
                }
                Activation::Both(auto, manual) => {
                    let auto_result = eval_auto(
                        auto,
                        effective_value,
                        profile,
                        self.character.weapon_type,
                        self.character.element,
                        &self.team_elements,
                        &self.team_regions,
                        refinement,
                    );
                    auto_result
                        .and_then(|av| eval_manual(manual, cond_buff, &self.manual_activations, av))
                }
            };

            if let Some(value) = resolved_value {
                buffs.push(ResolvedBuff {
                    source: format!("{} ({})", cond_buff.name, source_name),
                    stat: cond_buff.stat,
                    value,
                    target: cond_buff.target,
                });
            }
        }
    }

    fn resolve_conditional_buffs(&self, profile: &StatProfile, buffs: &mut Vec<ResolvedBuff>) {
        // Weapon conditional buffs
        if let Some(passive) = &self.weapon.passive {
            self.resolve_conditionals_for_source(
                passive.effect.conditional_buffs,
                self.weapon.name,
                self.refinement,
                profile,
                buffs,
            );
        }

        // Artifact conditional buffs
        for entry in &self.artifact_sets {
            // 2pc conditional: always apply
            self.resolve_conditionals_for_source(
                entry.set.two_piece.conditional_buffs,
                &format!("{} 2pc", entry.set.name),
                1,
                profile,
                buffs,
            );
            // 4pc conditional: only if piece_count >= 4
            if entry.piece_count >= 4 {
                self.resolve_conditionals_for_source(
                    entry.set.four_piece.conditional_buffs,
                    &format!("{} 4pc", entry.set.name),
                    1,
                    profile,
                    buffs,
                );
            }
        }
    }

    fn collect_talent_buffs(&self, profile: &StatProfile, buffs: &mut Vec<ResolvedBuff>) {
        let Some(talent_defs) = find_talent_buffs(self.character.id) else {
            return;
        };
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
                    let level = self.character.effective_talent_level(
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
                let raw = match scaling_stat {
                    genshin_calc_core::ScalingStat::Atk => profile.base_atk * raw_value,
                    // TotalAtk: best approximation at StatProfile level is base_atk
                    genshin_calc_core::ScalingStat::TotalAtk => profile.base_atk * raw_value,
                    genshin_calc_core::ScalingStat::Hp => profile.base_hp * raw_value,
                    genshin_calc_core::ScalingStat::Def => profile.base_def * raw_value,
                    genshin_calc_core::ScalingStat::Em => profile.elemental_mastery * raw_value,
                    genshin_calc_core::ScalingStat::CritRate => profile.crit_rate * raw_value,
                };
                match def.cap {
                    Some(cap) => raw.min(cap),
                    None => raw,
                }
            } else {
                raw_value
            };

            let final_value = match &def.activation {
                None => Some(value),
                Some(activation) => match activation {
                    crate::buff::Activation::Manual(cond) => {
                        eval_talent_manual(cond, def.name, &self.manual_activations, value)
                    }
                    crate::buff::Activation::Auto(cond) => eval_auto(
                        cond,
                        value,
                        profile,
                        self.character.weapon_type,
                        self.character.element,
                        &self.team_elements,
                        &self.team_regions,
                        self.refinement,
                    ),
                    crate::buff::Activation::Both(auto_cond, manual_cond) => {
                        let auto_val = eval_auto(
                            auto_cond,
                            value,
                            profile,
                            self.character.weapon_type,
                            self.character.element,
                            &self.team_elements,
                            &self.team_regions,
                            self.refinement,
                        );
                        match auto_val {
                            Some(base) => eval_talent_manual(
                                manual_cond,
                                def.name,
                                &self.manual_activations,
                                base,
                            ),
                            None => None,
                        }
                    }
                },
            };
            if let Some(fv) = final_value {
                buffs.push(ResolvedBuff {
                    source: def.name.to_string(),
                    stat: def.stat,
                    value: fv,
                    target: def.target,
                });
            }
        }
    }

    fn collect_static_buffs(&self) -> Vec<ResolvedBuff> {
        let mut buffs = Vec::new();

        // Weapon passive
        if let Some(passive) = &self.weapon.passive {
            for stat_buff in passive.effect.buffs {
                buffs.push(ResolvedBuff {
                    source: format!("{} ({})", passive.name, self.weapon.name),
                    stat: stat_buff.stat,
                    value: resolve_value(
                        stat_buff.value,
                        stat_buff.refinement_values,
                        self.refinement,
                    ),
                    target: BuffTarget::OnlySelf,
                });
            }
        }

        // Artifact set effects
        for entry in &self.artifact_sets {
            // 2pc buffs: always apply (piece_count >= 2)
            for stat_buff in entry.set.two_piece.buffs {
                buffs.push(ResolvedBuff {
                    source: format!("{} 2pc", entry.set.name),
                    stat: stat_buff.stat,
                    value: resolve_value(stat_buff.value, stat_buff.refinement_values, 1),
                    target: BuffTarget::OnlySelf,
                });
            }
            // 4pc buffs: only if piece_count >= 4
            if entry.piece_count >= 4 {
                for stat_buff in entry.set.four_piece.buffs {
                    buffs.push(ResolvedBuff {
                        source: format!("{} 4pc", entry.set.name),
                        stat: stat_buff.stat,
                        value: resolve_value(stat_buff.value, stat_buff.refinement_values, 1),
                        target: BuffTarget::OnlySelf,
                    });
                }
            }
        }

        buffs
    }

    fn validate(&self) -> Result<(), CalcError> {
        if self.constellation > 6 {
            return Err(CalcError::InvalidConstellation(self.constellation));
        }
        for &level in &self.talent_levels {
            if level == 0 || level > 15 {
                return Err(CalcError::InvalidTalentLevel(level));
            }
        }
        if self.refinement == 0 || self.refinement > 5 {
            return Err(CalcError::InvalidRefinement(self.refinement));
        }
        Ok(())
    }
}

/// Resolves the effective buff value for a given refinement level.
///
/// Uses `refinement_values[refinement - 1]` when available, otherwise falls back to `value`.
fn resolve_value(value: f64, refinement_values: Option<[f64; 5]>, refinement: u8) -> f64 {
    match refinement_values {
        Some(values) => values[(refinement.saturating_sub(1).min(4)) as usize],
        None => value,
    }
}

pub fn apply_ascension_stat(profile: &mut StatProfile, stat: &AscensionStat) {
    match stat {
        AscensionStat::Hp(v) => profile.hp_percent += v,
        AscensionStat::Atk(v) => profile.atk_percent += v,
        AscensionStat::Def(v) => profile.def_percent += v,
        AscensionStat::CritRate(v) => profile.crit_rate += v,
        AscensionStat::CritDmg(v) => profile.crit_dmg += v,
        AscensionStat::ElementalMastery(v) => profile.elemental_mastery += v,
        AscensionStat::EnergyRecharge(v) => profile.energy_recharge += v,
        AscensionStat::ElementalDmgBonus(elem, v) => match elem {
            Element::Pyro => profile.pyro_dmg_bonus += v,
            Element::Hydro => profile.hydro_dmg_bonus += v,
            Element::Electro => profile.electro_dmg_bonus += v,
            Element::Cryo => profile.cryo_dmg_bonus += v,
            Element::Dendro => profile.dendro_dmg_bonus += v,
            Element::Anemo => profile.anemo_dmg_bonus += v,
            Element::Geo => profile.geo_dmg_bonus += v,
        },
        AscensionStat::PhysicalDmgBonus(v) => profile.physical_dmg_bonus += v,
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
        BuffableStat::DefPercentRaw => profile.def_percent,
        BuffableStat::ElementalMastery => profile.elemental_mastery,
        BuffableStat::EnergyRecharge => profile.energy_recharge,
        // Not valid as StatScaling sources — these stats are not readable
        // from StatProfile as a single scalar for scaling purposes.
        BuffableStat::HpFlat
        | BuffableStat::AtkFlat
        | BuffableStat::DefFlat
        | BuffableStat::CritRate
        | BuffableStat::CritDmg
        | BuffableStat::DmgBonus
        | BuffableStat::ElementalDmgBonus(_)
        | BuffableStat::PhysicalDmgBonus
        | BuffableStat::NormalAtkDmgBonus
        | BuffableStat::ChargedAtkDmgBonus
        | BuffableStat::PlungingAtkDmgBonus
        | BuffableStat::SkillDmgBonus
        | BuffableStat::BurstDmgBonus
        | BuffableStat::HealingBonus
        | BuffableStat::ShieldStrength
        | BuffableStat::AmplifyingBonus
        | BuffableStat::TransformativeBonus
        | BuffableStat::AdditiveBonus
        | BuffableStat::ElementalRes(_)
        | BuffableStat::ElementalResReduction(_)
        | BuffableStat::PhysicalResReduction
        | BuffableStat::DefReduction
        | BuffableStat::NormalAtkFlatDmg
        | BuffableStat::ChargedAtkFlatDmg
        | BuffableStat::PlungingAtkFlatDmg
        | BuffableStat::SkillFlatDmg
        | BuffableStat::BurstFlatDmg => 0.0,
    }
}

/// Evaluates an Auto condition. Returns Some(value) if condition is met.
#[allow(clippy::too_many_arguments)]
fn eval_auto(
    cond: &AutoCondition,
    multiplier: f64,
    profile: &StatProfile,
    weapon_type: WeaponType,
    element: Element,
    team_elements: &[Element],
    team_regions: &[crate::types::Region],
    refinement: u8, // 1-based: 1=R1, 5=R5
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
        AutoCondition::StatScaling { stat, offset, cap } => {
            let stat_val = read_stat_for_scaling(stat, profile);
            let effective = (stat_val - offset.unwrap_or(0.0)).max(0.0);
            let raw = effective * multiplier;
            let idx = refinement.saturating_sub(1).min(4) as usize;
            Some(cap.map_or(raw, |caps| raw.min(caps[idx])))
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
        AutoCondition::TeamSameElementCount { min_count } => {
            if team_elements.is_empty() {
                return None;
            }
            let count = team_elements.iter().filter(|e| **e == element).count() as u8;
            if count >= *min_count {
                Some(multiplier)
            } else {
                None
            }
        }
        AutoCondition::TeamDiffElementCount { min_count } => {
            if team_elements.is_empty() {
                return None;
            }
            let count = team_elements.iter().filter(|e| **e != element).count() as u8;
            if count >= *min_count {
                Some(multiplier)
            } else {
                None
            }
        }
        AutoCondition::TeamRegionCount { region } => {
            if team_regions.is_empty() {
                return None;
            }
            let count = team_regions.iter().filter(|r| **r == *region).count();
            if count > 0 {
                Some(multiplier * count as f64)
            } else {
                None
            }
        }
    }
}

/// Evaluates a Manual condition. Returns Some(value) if user activated it.
fn eval_manual(
    cond: &ManualCondition,
    buff: &ConditionalBuff,
    activations: &[(String, ManualActivation)],
    base_value: f64,
) -> Option<f64> {
    let activation = activations.iter().find(|(name, _)| *name == buff.name);
    match cond {
        ManualCondition::Toggle => match activation {
            Some((_, ManualActivation::Active)) => Some(base_value),
            _ => None,
        },
        ManualCondition::Stacks(max) => match activation {
            Some((_, ManualActivation::Stacks(n))) => {
                let effective = (*n).min(*max);
                if effective == 0 {
                    return None;
                }
                match buff.stack_values {
                    Some(values) if !values.is_empty() => {
                        Some(values[(effective as usize).min(values.len()) - 1])
                    }
                    Some(_) => None,
                    None => Some(base_value * f64::from(effective)),
                }
            }
            Some((_, ManualActivation::Active)) => {
                let effective = *max;
                match buff.stack_values {
                    Some(values) if !values.is_empty() => {
                        Some(values[(effective as usize).min(values.len()) - 1])
                    }
                    Some(_) => None,
                    None => Some(base_value * f64::from(effective)),
                }
            }
            _ => None,
        },
    }
}

/// Evaluates a Manual condition for a talent buff. Returns Some(value) if user activated it.
///
/// Unlike `eval_manual`, this operates on talent buffs where `computed_value` is the
/// MAX-stacks value. For Stacks(max), per-stack = computed_value / max, so
/// n stacks = computed_value * n / max.
fn eval_talent_manual(
    cond: &ManualCondition,
    name: &str,
    activations: &[(String, ManualActivation)],
    computed_value: f64,
) -> Option<f64> {
    let activation = activations.iter().find(|(n, _)| n.as_str() == name);
    match cond {
        ManualCondition::Toggle => match activation {
            Some((_, ManualActivation::Active)) => Some(computed_value),
            _ => None,
        },
        ManualCondition::Stacks(max) => match activation {
            Some((_, ManualActivation::Stacks(n))) => {
                let effective = (*n).min(*max);
                if effective == 0 {
                    return None;
                }
                // computed_value is the MAX value; scale by n/max to get n-stack value
                Some(computed_value * f64::from(effective) / f64::from(*max))
            }
            Some((_, ManualActivation::Active)) => {
                // Active means full max stacks = full value
                Some(computed_value)
            }
            _ => None,
        },
    }
}

pub fn apply_weapon_sub_stat(profile: &mut StatProfile, sub: &WeaponSubStat) {
    apply_weapon_sub_stat_at_level(profile, sub, 3); // Default to Lv90
}

pub fn apply_weapon_sub_stat_at_level(
    profile: &mut StatProfile,
    sub: &WeaponSubStat,
    level_idx: usize,
) {
    match sub {
        WeaponSubStat::HpPercent(v) => profile.hp_percent += v[level_idx],
        WeaponSubStat::AtkPercent(v) => profile.atk_percent += v[level_idx],
        WeaponSubStat::DefPercent(v) => profile.def_percent += v[level_idx],
        WeaponSubStat::CritRate(v) => profile.crit_rate += v[level_idx],
        WeaponSubStat::CritDmg(v) => profile.crit_dmg += v[level_idx],
        WeaponSubStat::ElementalMastery(v) => profile.elemental_mastery += v[level_idx],
        WeaponSubStat::EnergyRecharge(v) => profile.energy_recharge += v[level_idx],
        WeaponSubStat::PhysicalDmgBonus(v) => profile.physical_dmg_bonus += v[level_idx],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{find_artifact_set, find_character, find_weapon};

    const EPSILON: f64 = 1e-4;

    // ---- eval_talent_manual unit tests ----

    #[test]
    fn test_eval_talent_manual_toggle_active() {
        let activations = vec![("my_buff".to_string(), ManualActivation::Active)];
        let result = eval_talent_manual(&ManualCondition::Toggle, "my_buff", &activations, 0.25);
        assert!((result.unwrap() - 0.25).abs() < EPSILON);
    }

    #[test]
    fn test_eval_talent_manual_toggle_inactive() {
        let activations: Vec<(String, ManualActivation)> = vec![];
        let result = eval_talent_manual(&ManualCondition::Toggle, "my_buff", &activations, 0.25);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_talent_manual_stacks_partial() {
        // computed_value = 0.3 (= 3 stacks max). With 2 stacks: 0.3 * 2/3 = 0.2
        let activations = vec![("my_buff".to_string(), ManualActivation::Stacks(2))];
        let result = eval_talent_manual(&ManualCondition::Stacks(3), "my_buff", &activations, 0.3);
        assert!((result.unwrap() - 0.2).abs() < EPSILON);
    }

    #[test]
    fn test_eval_talent_manual_stacks_max() {
        // Stacks(3) with max=3 → full value
        let activations = vec![("my_buff".to_string(), ManualActivation::Stacks(3))];
        let result = eval_talent_manual(&ManualCondition::Stacks(3), "my_buff", &activations, 0.3);
        assert!((result.unwrap() - 0.3).abs() < EPSILON);
    }

    #[test]
    fn test_eval_talent_manual_stacks_zero_returns_none() {
        let activations = vec![("my_buff".to_string(), ManualActivation::Stacks(0))];
        let result = eval_talent_manual(&ManualCondition::Stacks(3), "my_buff", &activations, 0.3);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_talent_manual_stacks_active_full_value() {
        // Active = max stacks = full computed_value
        let activations = vec![("my_buff".to_string(), ManualActivation::Active)];
        let result = eval_talent_manual(&ManualCondition::Stacks(3), "my_buff", &activations, 0.3);
        assert!((result.unwrap() - 0.3).abs() < EPSILON);
    }

    #[test]
    fn test_eval_talent_manual_stacks_clamped_to_max() {
        // n=5 but max=3 → effective=3 → full value
        let activations = vec![("my_buff".to_string(), ManualActivation::Stacks(5))];
        let result = eval_talent_manual(&ManualCondition::Stacks(3), "my_buff", &activations, 0.3);
        assert!((result.unwrap() - 0.3).abs() < EPSILON);
    }

    #[test]
    fn test_eval_talent_manual_wrong_name_returns_none() {
        let activations = vec![("other_buff".to_string(), ManualActivation::Active)];
        let result = eval_talent_manual(&ManualCondition::Toggle, "my_buff", &activations, 0.25);
        assert!(result.is_none());
    }

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
            .activate("Fantastic Voyage ATK Bonus")
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

    // --- Refinement tests ---

    #[test]
    fn test_refinement_default_is_r1() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        // Aquila Favonia: ATK% R1=0.20, R3=0.30, R5=0.40
        let member = TeamMemberBuilder::new(bennett, weapon).build().unwrap();
        let passive_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Aquila Favonia"))
            .unwrap();
        assert!(
            (passive_buff.value - 0.20).abs() < EPSILON,
            "R1 ATK% should be 0.20"
        );
    }

    #[test]
    fn test_refinement_r3() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let member = TeamMemberBuilder::new(bennett, weapon)
            .refinement(3)
            .build()
            .unwrap();
        let passive_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Aquila Favonia"))
            .unwrap();
        assert!(
            (passive_buff.value - 0.30).abs() < EPSILON,
            "R3 ATK% should be 0.30"
        );
    }

    #[test]
    fn test_refinement_r5() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let member = TeamMemberBuilder::new(bennett, weapon)
            .refinement(5)
            .build()
            .unwrap();
        let passive_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Aquila Favonia"))
            .unwrap();
        assert!(
            (passive_buff.value - 0.40).abs() < EPSILON,
            "R5 ATK% should be 0.40"
        );
    }

    #[test]
    fn test_refinement_r0_invalid() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let result = TeamMemberBuilder::new(bennett, weapon)
            .refinement(0)
            .build();
        assert!(matches!(result, Err(CalcError::InvalidRefinement(0))));
    }

    #[test]
    fn test_refinement_r6_invalid() {
        let bennett = find_character("bennett").unwrap();
        let weapon = find_weapon("aquila_favonia").unwrap();
        let result = TeamMemberBuilder::new(bennett, weapon)
            .refinement(6)
            .build();
        assert!(matches!(result, Err(CalcError::InvalidRefinement(6))));
    }

    #[test]
    fn test_refinement_values_none_falls_back_to_value() {
        // Weapons without refinement_values should use value directly regardless of refinement
        // Use a weapon where some buff has refinement_values: None
        // Check resolve_value directly
        assert!((resolve_value(0.15, None, 3) - 0.15).abs() < EPSILON);
        assert!((resolve_value(0.15, None, 5) - 0.15).abs() < EPSILON);
    }

    #[test]
    fn test_all_37_weapons_r1_value_matches_refinement_values_r1() {
        // Invariant: for any weapon with refinement_values, refinement_values[0] == value
        use crate::find_weapon;
        let weapon_ids = [
            "absolution",
            "aquila_favonia",
            "staff_of_homa",
            "engulfing_lightning",
            "freedom_sworn",
            "mistsplitter_reforged",
            "light_of_foliar_incision",
            "key_of_khaj_nisut",
            "splendor_of_tranquil_waters",
            "a_thousand_floating_dreams",
            "calamity_queller",
            "skyward_blade",
            "summit_shaper",
        ];
        for id in weapon_ids {
            if let Some(weapon) = find_weapon(id) {
                if let Some(passive) = &weapon.passive {
                    for buff in passive.effect.buffs {
                        if let Some(rv) = buff.refinement_values {
                            assert!(
                                (rv[0] - buff.value).abs() < EPSILON,
                                "Weapon '{}' buff {:?}: refinement_values[0]={} != value={}",
                                id,
                                buff.stat,
                                rv[0],
                                buff.value
                            );
                        }
                    }
                    for cond_buff in passive.effect.conditional_buffs {
                        if let Some(rv) = cond_buff.refinement_values {
                            assert!(
                                (rv[0] - cond_buff.value).abs() < EPSILON,
                                "Weapon '{}' cond_buff '{}': refinement_values[0]={} != value={}",
                                id,
                                cond_buff.name,
                                rv[0],
                                cond_buff.value
                            );
                        }
                    }
                }
            }
        }
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
            .activate("Fantastic Voyage ATK Bonus")
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
            .activate("Fantastic Voyage ATK Bonus")
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

    #[test]
    fn test_faruzan_burst_buff_at_lv13() {
        let faruzan = find_character("faruzan").unwrap();
        let weapon = find_weapon("favonius_warbow").unwrap();
        let member = TeamMemberBuilder::new(faruzan, weapon)
            .talent_levels([1, 1, 13])
            .activate("Prayerful Wind's Benefit")
            .build()
            .unwrap();

        let buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Prayerful Wind"))
            .expect("Should have Faruzan burst buff");
        assert_eq!(
            buff.stat,
            genshin_calc_core::BuffableStat::ElementalDmgBonus(genshin_calc_core::Element::Anemo)
        );
        assert!(buff.value > 0.0, "Buff value should be positive");
    }

    #[test]
    fn test_diona_c6_constellation_gate() {
        let diona = find_character("diona").unwrap();
        let weapon = find_weapon("favonius_warbow").unwrap();

        // C0: no EM buff
        let c0 = TeamMemberBuilder::new(diona, weapon)
            .constellation(0)
            .build()
            .unwrap();
        assert!(
            !c0.buffs_provided
                .iter()
                .any(|b| b.source.contains("Cat's Tail")),
            "C0 Diona should not have C6 buff"
        );

        // C6: EM+200 buff present
        let c6 = TeamMemberBuilder::new(diona, weapon)
            .constellation(6)
            .build()
            .unwrap();
        let buff = c6
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Cat's Tail"))
            .expect("C6 Diona should have EM buff");
        assert!((buff.value - 200.0).abs() < EPSILON);
    }

    #[test]
    fn test_shenhe_a1_press_in_buffs_provided() {
        let shenhe = find_character("shenhe").unwrap();
        let weapon = find_weapon("calamity_queller").unwrap();
        let member = TeamMemberBuilder::new(shenhe, weapon)
            .activate("Deific Embrace Press - Skill DMG")
            .activate("Deific Embrace Press - Burst DMG")
            .activate_with_stacks("Spring Spirit Summoning Normal ATK Flat DMG", 5)
            .activate_with_stacks("Spring Spirit Summoning Charged ATK Flat DMG", 5)
            .activate_with_stacks("Spring Spirit Summoning Plunging ATK Flat DMG", 5)
            .activate_with_stacks("Spring Spirit Summoning Skill Flat DMG", 5)
            .activate_with_stacks("Spring Spirit Summoning Burst Flat DMG", 5)
            .build()
            .unwrap();

        assert!(
            member
                .buffs_provided
                .iter()
                .any(|b| b.stat == genshin_calc_core::BuffableStat::SkillDmgBonus),
            "Should have SkillDmgBonus from A4"
        );
        assert!(
            member
                .buffs_provided
                .iter()
                .any(|b| b.stat == genshin_calc_core::BuffableStat::BurstDmgBonus),
            "Should have BurstDmgBonus from A4"
        );
        // Skill should produce 5 FlatDmg entries, not AtkFlat
        assert!(
            !member
                .buffs_provided
                .iter()
                .any(|b| b.stat == genshin_calc_core::BuffableStat::AtkFlat
                    && b.source.contains("Spring Spirit")),
            "Shenhe skill should not produce AtkFlat"
        );
        assert!(
            member
                .buffs_provided
                .iter()
                .any(|b| b.stat == genshin_calc_core::BuffableStat::NormalAtkFlatDmg),
            "Should have NormalAtkFlatDmg from skill"
        );
    }

    #[test]
    fn test_sara_c6_crit_dmg_gate() {
        let sara = find_character("kujou_sara").unwrap();
        let weapon = find_weapon("favonius_warbow").unwrap();

        // C0: only ATK buff, no CritDmg
        let c0 = TeamMemberBuilder::new(sara, weapon)
            .constellation(0)
            .build()
            .unwrap();
        assert!(
            !c0.buffs_provided
                .iter()
                .any(|b| b.stat == genshin_calc_core::BuffableStat::CritDmg),
            "C0 Sara should not have C6 CritDmg buff"
        );

        // C6: CritDmg present
        let c6 = TeamMemberBuilder::new(sara, weapon)
            .constellation(6)
            .build()
            .unwrap();
        let buff = c6
            .buffs_provided
            .iter()
            .find(|b| b.stat == genshin_calc_core::BuffableStat::CritDmg)
            .expect("C6 Sara should have CritDmg buff");
        assert!((buff.value - 0.60).abs() < EPSILON);
    }

    #[test]
    fn test_jahoda_builds_with_talent_buff() {
        let jahoda = find_character("jahoda").unwrap();
        let weapon = find_weapon("favonius_warbow").unwrap();
        let member = TeamMemberBuilder::new(jahoda, weapon).build().unwrap();
        assert!(
            member
                .buffs_provided
                .iter()
                .any(|b| b.source.contains("Jahoda A4")),
            "Should have Jahoda A4 EM buff"
        );
    }

    #[test]
    fn test_aino_c1_builds_with_talent_buff() {
        let aino = find_character("aino").unwrap();
        let weapon = find_weapon("favonius_greatsword").unwrap();

        // C0: no C1 buff
        let c0 = TeamMemberBuilder::new(aino, weapon)
            .constellation(0)
            .build()
            .unwrap();
        assert!(
            !c0.buffs_provided
                .iter()
                .any(|b| b.source.contains("Aino C1")),
            "C0 Aino should not have C1 buff"
        );

        // C1: EM+80 present
        let c1 = TeamMemberBuilder::new(aino, weapon)
            .constellation(1)
            .build()
            .unwrap();
        let buff = c1
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("Aino C1"))
            .expect("C1 Aino should have EM buff");
        assert!((buff.value - 80.0).abs() < EPSILON);
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
            &[],
            1,
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
            &[],
            1,
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
            &[],
            1,
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
            &[],
            1,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_stat_scaling_normal() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::EnergyRecharge,
            offset: None,
            cap: Some([0.75; 5]),
        };
        let profile = StatProfile {
            energy_recharge: 1.80,
            ..Default::default()
        };
        // 1.80 * 0.25 = 0.45 (below cap)
        let result = eval_auto(
            &cond,
            0.25,
            &profile,
            WeaponType::Sword,
            Element::Pyro,
            &[],
            &[],
            1,
        );
        assert!((result.unwrap() - 0.45).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_stat_scaling_capped() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::EnergyRecharge,
            offset: None,
            cap: Some([0.75; 5]),
        };
        let profile = StatProfile {
            energy_recharge: 4.00,
            ..Default::default()
        };
        // 4.00 * 0.25 = 1.00 → capped at 0.75
        let result = eval_auto(
            &cond,
            0.25,
            &profile,
            WeaponType::Sword,
            Element::Pyro,
            &[],
            &[],
            1,
        );
        assert!((result.unwrap() - 0.75).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_stat_scaling_hp() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::HpPercent,
            offset: None,
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
            &[],
            1,
        );
        assert!((result.unwrap() - 214.16).abs() < 0.01);
    }

    #[test]
    fn test_eval_auto_stat_scaling_with_offset() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::EnergyRecharge,
            offset: Some(1.0),
            cap: None,
        };
        let profile = StatProfile {
            energy_recharge: 1.80,
            ..Default::default()
        };
        // (1.80 - 1.0) * 0.28 = 0.224
        let result = eval_auto(
            &cond,
            0.28,
            &profile,
            WeaponType::Polearm,
            Element::Electro,
            &[],
            &[],
            1,
        );
        assert!((result.unwrap() - 0.224).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_stat_scaling_offset_clamps_negative() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::EnergyRecharge,
            offset: Some(1.0),
            cap: None,
        };
        let profile = StatProfile {
            energy_recharge: 0.80,
            ..Default::default()
        };
        // (0.80 - 1.0).max(0.0) * 0.28 = 0.0
        let result = eval_auto(
            &cond,
            0.28,
            &profile,
            WeaponType::Polearm,
            Element::Electro,
            &[],
            &[],
            1,
        );
        assert!((result.unwrap() - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_stat_scaling_refinement_cap() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::EnergyRecharge,
            offset: Some(1.0),
            cap: Some([0.80, 0.90, 1.00, 1.10, 1.20]),
        };
        let profile = StatProfile {
            energy_recharge: 5.00,
            ..Default::default()
        };
        // R1: (5.00 - 1.0) * 0.28 = 1.12 → capped at 0.80
        let result = eval_auto(
            &cond,
            0.28,
            &profile,
            WeaponType::Polearm,
            Element::Electro,
            &[],
            &[],
            1,
        );
        assert!((result.unwrap() - 0.80).abs() < EPSILON);

        // R5: (5.00 - 1.0) * 0.56 = 2.24 → capped at 1.20
        let result_r5 = eval_auto(
            &cond,
            0.56,
            &profile,
            WeaponType::Polearm,
            Element::Electro,
            &[],
            &[],
            5,
        );
        assert!((result_r5.unwrap() - 1.20).abs() < EPSILON);
    }

    #[test]
    fn test_eval_auto_stat_scaling_def_percent_raw() {
        let cond = AutoCondition::StatScaling {
            stat: BuffableStat::DefPercentRaw,
            offset: None,
            cap: None,
        };
        let profile = StatProfile {
            base_def: 800.0,
            def_percent: 0.50,
            ..Default::default()
        };
        // DefPercentRaw reads 0.50 directly: 0.50 * 0.18 = 0.09
        let result = eval_auto(
            &cond,
            0.18,
            &profile,
            WeaponType::Sword,
            Element::Geo,
            &[],
            &[],
            1,
        );
        assert!((result.unwrap() - 0.09).abs() < EPSILON);
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
            &[],
            1,
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
            &[],
            1,
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
            &[],
            1,
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
            &[],
            1,
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
            &[],
            1,
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
            &[],
            1,
        );
        assert!(result.is_none());
    }

    // --- eval_auto TeamRegionCount tests ---

    #[test]
    fn test_eval_auto_team_region_count_liyue_2() {
        let cond = AutoCondition::TeamRegionCount {
            region: crate::types::Region::Liyue,
        };
        let regions = vec![
            crate::types::Region::Liyue,
            crate::types::Region::Mondstadt,
            crate::types::Region::Liyue,
        ];
        let result = eval_auto(
            &cond,
            0.07,
            &StatProfile::default(),
            WeaponType::Claymore,
            Element::Pyro,
            &[],
            &regions,
            1,
        );
        assert!((result.unwrap() - 0.14).abs() < EPSILON); // 0.07 * 2
    }

    #[test]
    fn test_eval_auto_team_region_count_zero() {
        let cond = AutoCondition::TeamRegionCount {
            region: crate::types::Region::Liyue,
        };
        let regions = vec![crate::types::Region::Mondstadt];
        let result = eval_auto(
            &cond,
            0.07,
            &StatProfile::default(),
            WeaponType::Claymore,
            Element::Pyro,
            &[],
            &regions,
            1,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_auto_team_region_count_empty() {
        let cond = AutoCondition::TeamRegionCount {
            region: crate::types::Region::Liyue,
        };
        let result = eval_auto(
            &cond,
            0.07,
            &StatProfile::default(),
            WeaponType::Claymore,
            Element::Pyro,
            &[],
            &[],
            1,
        );
        assert!(result.is_none());
    }

    // --- eval_manual tests ---

    fn make_test_buff(name: &'static str, value: f64, cond: ManualCondition) -> ConditionalBuff {
        ConditionalBuff {
            name,
            description: "test",
            stat: BuffableStat::AtkPercent,
            value,
            refinement_values: None,
            stack_values: None,
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(cond),
        }
    }

    #[test]
    fn test_eval_manual_toggle_active() {
        let buff = make_test_buff("test_buff", 0.15, ManualCondition::Toggle);
        let result = eval_manual(
            &ManualCondition::Toggle,
            &buff,
            &[("test_buff".to_string(), ManualActivation::Active)],
            buff.value,
        );
        assert!((result.unwrap() - 0.15).abs() < EPSILON);
    }

    #[test]
    fn test_eval_manual_toggle_not_present() {
        let buff = make_test_buff("test_buff", 0.15, ManualCondition::Toggle);
        let result = eval_manual(&ManualCondition::Toggle, &buff, &[], buff.value);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_manual_toggle_stacks_mismatch() {
        let buff = make_test_buff("test_buff", 0.15, ManualCondition::Toggle);
        let result = eval_manual(
            &ManualCondition::Toggle,
            &buff,
            &[("test_buff".to_string(), ManualActivation::Stacks(2))],
            buff.value,
        );
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_manual_stacks_normal() {
        let buff = make_test_buff("test_buff", 0.075, ManualCondition::Stacks(3));
        let result = eval_manual(
            &ManualCondition::Stacks(3),
            &buff,
            &[("test_buff".to_string(), ManualActivation::Stacks(2))],
            buff.value,
        );
        assert!((result.unwrap() - 0.15).abs() < EPSILON); // 0.075 * 2
    }

    #[test]
    fn test_eval_manual_stacks_exceeds_max() {
        let buff = make_test_buff("test_buff", 0.075, ManualCondition::Stacks(3));
        let result = eval_manual(
            &ManualCondition::Stacks(3),
            &buff,
            &[("test_buff".to_string(), ManualActivation::Stacks(5))],
            buff.value,
        );
        assert!((result.unwrap() - 0.225).abs() < EPSILON); // 0.075 * 3 (capped)
    }

    #[test]
    fn test_eval_manual_stacks_not_present() {
        let buff = make_test_buff("test_buff", 0.075, ManualCondition::Stacks(3));
        let result = eval_manual(&ManualCondition::Stacks(3), &buff, &[], buff.value);
        assert!(result.is_none());
    }

    #[test]
    fn test_eval_manual_stacks_with_active_treated_as_max() {
        let buff = make_test_buff("test_buff", 0.075, ManualCondition::Stacks(3));
        let result = eval_manual(
            &ManualCondition::Stacks(3),
            &buff,
            &[("test_buff".to_string(), ManualActivation::Active)],
            buff.value,
        );
        assert!((result.unwrap() - 0.225).abs() < EPSILON); // 0.075 * 3 (max)
    }

    #[test]
    fn test_eval_manual_toggle_uses_base_value() {
        let buff = make_test_buff("test_buff", 0.15, ManualCondition::Toggle);
        // base_value differs from buff.value — should return base_value
        let result = eval_manual(
            &ManualCondition::Toggle,
            &buff,
            &[("test_buff".to_string(), ManualActivation::Active)],
            0.30,
        );
        assert!((result.unwrap() - 0.30).abs() < EPSILON);
    }

    #[test]
    fn test_eval_manual_stacks_uses_base_value() {
        let buff = make_test_buff("test_buff", 0.05, ManualCondition::Stacks(3));
        let result = eval_manual(
            &ManualCondition::Stacks(3),
            &buff,
            &[("test_buff".to_string(), ManualActivation::Stacks(2))],
            0.09,
        );
        assert!((result.unwrap() - 0.18).abs() < EPSILON); // 0.09 * 2
    }

    #[test]
    fn test_empty_stack_values_returns_none() {
        let buff = ConditionalBuff {
            name: "test_empty",
            description: "test",
            stat: BuffableStat::AtkPercent,
            value: 0.10,
            refinement_values: None,
            stack_values: Some(&[]),
            target: BuffTarget::OnlySelf,
            activation: Activation::Manual(ManualCondition::Stacks(3)),
        };
        let activations = vec![("test_empty".to_string(), ManualActivation::Stacks(2))];
        let result = eval_manual(&ManualCondition::Stacks(3), &buff, &activations, 0.10);
        assert_eq!(result, None);
    }

    // --- Activation::Both tests (unit level) ---

    #[test]
    fn test_both_auto_pass_manual_pass() {
        // Both should pass auto_val to eval_manual as base_value
        let auto = AutoCondition::StatScaling {
            stat: BuffableStat::HpPercent,
            offset: None,
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
            &[],
            1,
        );
        let auto_val = auto_result.unwrap();
        assert!((auto_val - 200.0).abs() < EPSILON);
        // Both passes auto_val (200.0) as base_value to eval_manual
        let buff = make_test_buff("test", 0.01, ManualCondition::Toggle);
        let result = auto_result.and_then(|av| {
            eval_manual(
                &manual,
                &buff,
                &[("test".to_string(), ManualActivation::Active)],
                av,
            )
        });
        assert!((result.unwrap() - 200.0).abs() < EPSILON);
    }

    #[test]
    fn test_both_stat_scaling_with_stacks() {
        let auto = AutoCondition::StatScaling {
            stat: BuffableStat::ElementalMastery,
            offset: None,
            cap: None,
        };
        let manual = ManualCondition::Stacks(2);
        let profile = StatProfile {
            elemental_mastery: 200.0,
            ..Default::default()
        };
        // eval_auto: 200 * 0.28 = 56.0
        let auto_result = eval_auto(
            &auto,
            0.28,
            &profile,
            WeaponType::Polearm,
            Element::Pyro,
            &[],
            &[],
            1,
        );
        let auto_val = auto_result.unwrap();
        assert!((auto_val - 56.0).abs() < EPSILON);
        // Both: auto_val * 2 stacks = 112.0
        let buff = make_test_buff("test", 0.28, ManualCondition::Stacks(2));
        let result = auto_result.and_then(|av| {
            eval_manual(
                &manual,
                &buff,
                &[("test".to_string(), ManualActivation::Stacks(2))],
                av,
            )
        });
        assert!((result.unwrap() - 112.0).abs() < EPSILON);
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
            &[],
            1,
        );
        assert!(auto_result.is_none());
        let buff = make_test_buff("test", 0.35, ManualCondition::Toggle);
        let result = auto_result.and_then(|av| {
            eval_manual(
                &manual,
                &buff,
                &[("test".to_string(), ManualActivation::Active)],
                av,
            )
        });
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
            &[],
            1,
        );
        assert!(auto_result.is_some());
        let buff = make_test_buff("test", 0.35, ManualCondition::Toggle);
        let result = auto_result.and_then(|av| {
            eval_manual(&manual, &buff, &[], av) // not activated
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
            .activate("Fantastic Voyage ATK Bonus")
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

    // --- Task 11: Data integrity tests ---

    #[test]
    fn test_all_weapon_refinement_values_non_decreasing() {
        for weapon in crate::weapons::ALL_WEAPONS {
            if let Some(passive) = &weapon.passive {
                for (i, stat_buff) in passive.effect.buffs.iter().enumerate() {
                    if let Some(rv) = stat_buff.refinement_values {
                        for w in rv.windows(2) {
                            assert!(
                                w[0] <= w[1],
                                "Weapon '{}' StatBuff[{}]: refinement values not non-decreasing: {:?}",
                                weapon.name,
                                i,
                                rv
                            );
                        }
                    }
                }
                for (i, cond_buff) in passive.effect.conditional_buffs.iter().enumerate() {
                    if let Some(rv) = cond_buff.refinement_values {
                        for w in rv.windows(2) {
                            assert!(
                                w[0] <= w[1],
                                "Weapon '{}' ConditionalBuff[{}] '{}': refinement values not non-decreasing: {:?}",
                                weapon.name,
                                i,
                                cond_buff.name,
                                rv
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_all_weapon_refinement_values_r1_invariant() {
        for weapon in crate::weapons::ALL_WEAPONS {
            if let Some(passive) = &weapon.passive {
                for (i, stat_buff) in passive.effect.buffs.iter().enumerate() {
                    if let Some(rv) = stat_buff.refinement_values {
                        assert!(
                            (rv[0] - stat_buff.value).abs() < 1e-10,
                            "Weapon '{}' StatBuff[{}]: refinement_values[0]={} != value={}",
                            weapon.name,
                            i,
                            rv[0],
                            stat_buff.value
                        );
                    }
                }
                for (i, cond_buff) in passive.effect.conditional_buffs.iter().enumerate() {
                    if let Some(rv) = cond_buff.refinement_values {
                        assert!(
                            (rv[0] - cond_buff.value).abs() < 1e-10,
                            "Weapon '{}' ConditionalBuff[{}] '{}': refinement_values[0]={} != value={}",
                            weapon.name,
                            i,
                            cond_buff.name,
                            rv[0],
                            cond_buff.value
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_jade_cutter_hp_atk_conditional_refinement() {
        let keqing = find_character("keqing").unwrap();
        let weapon = find_weapon("primordial_jade_cutter").unwrap();

        // R1: jade_cutter_hp_atk multiplier = 0.012
        let r1 = TeamMemberBuilder::new(keqing, weapon)
            .refinement(1)
            .build()
            .unwrap();

        // R5: jade_cutter_hp_atk multiplier = 0.024
        let r5 = TeamMemberBuilder::new(keqing, weapon)
            .refinement(5)
            .build()
            .unwrap();

        // HP% StatBuff should scale: R1=0.20, R5=0.40
        let r1_hp = r1
            .buffs_provided
            .iter()
            .find(|b| {
                b.stat == crate::buff::BuffableStat::HpPercent && b.source.contains("Jade Cutter")
            })
            .unwrap();
        let r5_hp = r5
            .buffs_provided
            .iter()
            .find(|b| {
                b.stat == crate::buff::BuffableStat::HpPercent && b.source.contains("Jade Cutter")
            })
            .unwrap();
        assert!((r1_hp.value - 0.20).abs() < EPSILON);
        assert!((r5_hp.value - 0.40).abs() < EPSILON);

        // HP->ATK ConditionalBuff should yield higher value at R5 than R1
        // (multiplier is larger, same total_hp, so r5_atk > r1_atk)
        let r1_atk = r1
            .buffs_provided
            .iter()
            .find(|b| {
                b.stat == crate::buff::BuffableStat::AtkFlat
                    && b.source.contains("jade_cutter_hp_atk")
            })
            .unwrap();
        let r5_atk = r5
            .buffs_provided
            .iter()
            .find(|b| {
                b.stat == crate::buff::BuffableStat::AtkFlat
                    && b.source.contains("jade_cutter_hp_atk")
            })
            .unwrap();
        assert!(
            r5_atk.value > r1_atk.value,
            "R5 should give more ATK than R1"
        );
        assert!(r1_atk.value > 0.0, "ATK buff should be positive");
    }

    #[test]
    fn test_lithic_blade_with_liyue_team() {
        use crate::types::Region;
        let beidou = find_character("beidou").unwrap();
        let lithic = find_weapon("lithic_blade").unwrap();
        let member = TeamMemberBuilder::new(beidou, lithic)
            .team_regions(vec![Region::Liyue, Region::Mondstadt, Region::Liyue])
            .build()
            .unwrap();

        let atk_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("lithic_blade_atk"))
            .expect("Should have lithic_blade_atk buff");
        // R1: 0.07 * 2 Liyue members = 0.14
        assert!(
            (atk_buff.value - 0.14).abs() < EPSILON,
            "ATK% should be 0.14, got {}",
            atk_buff.value
        );

        let crit_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("lithic_blade_crit"))
            .expect("Should have lithic_blade_crit buff");
        // R1: 0.03 * 2 = 0.06
        assert!(
            (crit_buff.value - 0.06).abs() < EPSILON,
            "CR should be 0.06, got {}",
            crit_buff.value
        );
    }

    #[test]
    fn test_scarlet_sands_with_em_and_stacks() {
        let cyno = find_character("cyno").unwrap();
        let weapon = find_weapon("staff_of_the_scarlet_sands").unwrap();
        let em_stats = StatProfile {
            elemental_mastery: 100.0,
            ..Default::default()
        };
        let member = TeamMemberBuilder::new(cyno, weapon)
            .artifact_stats(em_stats)
            .activate_with_stacks("scarlet_sands_skill_stacks", 2)
            .build()
            .unwrap();

        // Primary: Auto(StatScaling EM) should exist
        let primary = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("scarlet_sands_em_atk"))
            .expect("Should have primary EM ATK buff");
        assert!(primary.value > 0.0, "Primary buff should be positive");

        // Secondary: Both(StatScaling EM, Stacks(2)) should exist
        let secondary = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("scarlet_sands_skill_stacks"))
            .expect("Should have skill stacks buff");
        assert!(secondary.value > 0.0, "Secondary buff should be positive");
    }

    #[test]
    fn test_engulfing_burst_er_toggle() {
        let raiden = find_character("raiden_shogun").unwrap();
        let weapon = find_weapon("engulfing_lightning").unwrap();
        let member = TeamMemberBuilder::new(raiden, weapon)
            .activate("engulfing_burst_er")
            .build()
            .unwrap();

        let er_buff = member
            .buffs_provided
            .iter()
            .find(|b| b.source.contains("engulfing_burst_er"))
            .expect("Should have burst ER buff");
        assert!(
            (er_buff.value - 0.30).abs() < EPSILON,
            "ER should be 0.30, got {}",
            er_buff.value
        );
    }
}
