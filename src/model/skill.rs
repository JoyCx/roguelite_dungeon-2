use std::time::Instant;

/// Player skill types for different attack techniques
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SkillType {
    /// Basic slash - quick, moderate damage
    Slash,
    /// Piercing attack - single target, high damage
    Pierce,
    /// Heavy attack - slow, high damage, knocks back
    HeavyAttack,
    /// Whirlwind - spin attack hitting all around
    Whirlwind,
    /// Ground slam - shockwave attack
    GroundSlam,
}

/// Skill level affects damage and cooldown
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum SkillLevel {
    /// Level 1 - Basic execution
    Novice,
    /// Level 2 - More damage/range
    Apprentice,
    /// Level 3 - Mastered
    Expert,
    /// Level 4 - Specialized
    Master,
}

impl SkillLevel {
    /// Get damage multiplier for this skill level
    pub fn damage_multiplier(&self) -> f32 {
        match self {
            SkillLevel::Novice => 1.0,
            SkillLevel::Apprentice => 1.25,
            SkillLevel::Expert => 1.5,
            SkillLevel::Master => 2.0,
        }
    }

    /// Get cooldown reduction as percentage (0.0 = no reduction, 1.0 = instant)
    pub fn cooldown_reduction(&self) -> f32 {
        match self {
            SkillLevel::Novice => 0.0,
            SkillLevel::Apprentice => 0.1,
            SkillLevel::Expert => 0.25,
            SkillLevel::Master => 0.4,
        }
    }

    /// Get experience cost to reach next level
    pub fn experience_to_next(&self) -> i32 {
        match self {
            SkillLevel::Novice => 50,
            SkillLevel::Apprentice => 100,
            SkillLevel::Expert => 200,
            SkillLevel::Master => 0, // Max level
        }
    }
}

/// Represents a learned skill with cooldown and level
#[derive(Clone, Debug)]
pub struct Skill {
    pub skill_type: SkillType,
    pub level: SkillLevel,
    pub last_used: Option<Instant>,
    pub base_cooldown: f32, // In seconds
}

impl Skill {
    /// Create a new skill at novice level
    pub fn new(skill_type: SkillType) -> Self {
        let base_cooldown = match skill_type {
            SkillType::Slash => 0.8,
            SkillType::Pierce => 1.2,
            SkillType::HeavyAttack => 2.0,
            SkillType::Whirlwind => 3.0,
            SkillType::GroundSlam => 4.0,
        };

        Self {
            skill_type,
            level: SkillLevel::Novice,
            last_used: None,
            base_cooldown,
        }
    }

    /// Check if skill is ready to use
    pub fn is_ready(&self) -> bool {
        match self.last_used {
            None => true,
            Some(last_time) => {
                let effective_cooldown = self.get_effective_cooldown();
                last_time.elapsed().as_secs_f32() >= effective_cooldown
            }
        }
    }

    /// Get cooldown with level reduction applied
    pub fn get_effective_cooldown(&self) -> f32 {
        let reduction = self.level.cooldown_reduction();
        self.base_cooldown * (1.0 - reduction)
    }

    /// Get remaining cooldown time (0 if ready)
    pub fn remaining_cooldown(&self) -> f32 {
        match self.last_used {
            None => 0.0,
            Some(last_time) => {
                let elapsed = last_time.elapsed().as_secs_f32();
                let effective = self.get_effective_cooldown();
                (effective - elapsed).max(0.0)
            }
        }
    }

    /// Get cooldown progress (0.0 to 1.0)
    pub fn cooldown_progress(&self) -> f32 {
        match self.last_used {
            None => 1.0,
            Some(last_time) => {
                let elapsed = last_time.elapsed().as_secs_f32();
                let effective = self.get_effective_cooldown();
                (elapsed / effective).min(1.0)
            }
        }
    }

    /// Mark skill as used (reset cooldown)
    pub fn use_skill(&mut self) {
        if self.is_ready() {
            self.last_used = Some(Instant::now());
        }
    }

    /// Get damage multiplier for this skill at current level
    pub fn get_damage_multiplier(&self) -> f32 {
        let base_multiplier = match self.skill_type {
            SkillType::Slash => 1.0,
            SkillType::Pierce => 1.3,
            SkillType::HeavyAttack => 1.8,
            SkillType::Whirlwind => 0.9, // Lower because hits multiple enemies
            SkillType::GroundSlam => 1.5,
        };

        base_multiplier * self.level.damage_multiplier()
    }

    /// Get area of effect radius
    pub fn get_aoe_radius(&self) -> i32 {
        match self.skill_type {
            SkillType::Slash => 1,
            SkillType::Pierce => 0, // Single target
            SkillType::HeavyAttack => 1,
            SkillType::Whirlwind => 2,
            SkillType::GroundSlam => 3,
        }
    }

    /// Level up the skill if possible
    pub fn level_up(&mut self) -> bool {
        let next_level = match self.level {
            SkillLevel::Novice => Some(SkillLevel::Apprentice),
            SkillLevel::Apprentice => Some(SkillLevel::Expert),
            SkillLevel::Expert => Some(SkillLevel::Master),
            SkillLevel::Master => None,
        };

        if let Some(new_level) = next_level {
            self.level = new_level;
            true
        } else {
            false
        }
    }

    /// Get description of this skill
    pub fn description(&self) -> &'static str {
        match self.skill_type {
            SkillType::Slash => "Quick slashing attack with moderate damage",
            SkillType::Pierce => "Single-target piercing attack with high damage",
            SkillType::HeavyAttack => "Slow but powerful attack that knocks back",
            SkillType::Whirlwind => "Spin attack hitting all enemies around you",
            SkillType::GroundSlam => "Slam ground creating shockwave area damage",
        }
    }
}

/// Skill tree manager for the player
#[derive(Clone, Debug)]
pub struct SkillTree {
    pub skills: Vec<Skill>,
}

impl SkillTree {
    /// Create new skill tree with all skills available
    pub fn new() -> Self {
        let skills = vec![
            Skill::new(SkillType::Slash),
            Skill::new(SkillType::Pierce),
            Skill::new(SkillType::HeavyAttack),
            Skill::new(SkillType::Whirlwind),
            Skill::new(SkillType::GroundSlam),
        ];

        Self { skills }
    }

    /// Get skill by type
    pub fn get_skill(&self, skill_type: SkillType) -> Option<&Skill> {
        self.skills.iter().find(|s| s.skill_type == skill_type)
    }

    /// Get mutable skill by type
    pub fn get_skill_mut(&mut self, skill_type: SkillType) -> Option<&mut Skill> {
        self.skills.iter_mut().find(|s| s.skill_type == skill_type)
    }

    /// Get all ready skills
    pub fn get_ready_skills(&self) -> Vec<&Skill> {
        self.skills.iter().filter(|s| s.is_ready()).collect()
    }

    /// Get total skill experience (sum of all levels)
    pub fn get_total_experience(&self) -> i32 {
        self.skills
            .iter()
            .map(|s| match s.level {
                SkillLevel::Novice => 0,
                SkillLevel::Apprentice => 50,
                SkillLevel::Expert => 150,
                SkillLevel::Master => 350,
            })
            .sum()
    }
}

impl Default for SkillTree {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_skill_creation() {
        let skill = Skill::new(SkillType::Slash);
        assert_eq!(skill.skill_type, SkillType::Slash);
        assert_eq!(skill.level, SkillLevel::Novice);
        assert!(skill.last_used.is_none());
    }

    #[test]
    fn test_skill_cooldown() {
        let mut skill = Skill::new(SkillType::Slash);
        assert!(skill.is_ready());

        skill.use_skill();
        assert!(!skill.is_ready());
    }

    #[test]
    fn test_cooldown_progress() {
        let mut skill = Skill::new(SkillType::Slash);
        skill.use_skill();
        let progress = skill.cooldown_progress();
        assert!(progress >= 0.0 && progress <= 1.0);
    }

    #[test]
    fn test_level_up() {
        let mut skill = Skill::new(SkillType::Pierce);
        assert_eq!(skill.level, SkillLevel::Novice);

        assert!(skill.level_up());
        assert_eq!(skill.level, SkillLevel::Apprentice);

        assert!(skill.level_up());
        assert_eq!(skill.level, SkillLevel::Expert);

        assert!(skill.level_up());
        assert_eq!(skill.level, SkillLevel::Master);

        assert!(!skill.level_up()); // Can't level up past master
        assert_eq!(skill.level, SkillLevel::Master);
    }

    #[test]
    fn test_damage_multiplier_progression() {
        let mut skill = Skill::new(SkillType::HeavyAttack);

        let novice_damage = skill.get_damage_multiplier();
        skill.level_up();
        let apprentice_damage = skill.get_damage_multiplier();
        skill.level_up();
        let expert_damage = skill.get_damage_multiplier();

        assert!(apprentice_damage > novice_damage);
        assert!(expert_damage > apprentice_damage);
    }

    #[test]
    fn test_cooldown_reduction() {
        let mut skill = Skill::new(SkillType::Whirlwind);
        let novice_cooldown = skill.get_effective_cooldown();

        skill.level = SkillLevel::Master;
        let master_cooldown = skill.get_effective_cooldown();

        assert!(master_cooldown < novice_cooldown);
    }

    #[test]
    fn test_skill_level_damage_multiplier() {
        assert_eq!(SkillLevel::Novice.damage_multiplier(), 1.0);
        assert_eq!(SkillLevel::Apprentice.damage_multiplier(), 1.25);
        assert_eq!(SkillLevel::Expert.damage_multiplier(), 1.5);
        assert_eq!(SkillLevel::Master.damage_multiplier(), 2.0);
    }

    #[test]
    fn test_skill_tree_creation() {
        let tree = SkillTree::new();
        assert_eq!(tree.skills.len(), 5);

        assert!(tree.get_skill(SkillType::Slash).is_some());
        assert!(tree.get_skill(SkillType::Pierce).is_some());
    }

    #[test]
    fn test_skill_tree_get_ready_skills() {
        let tree = SkillTree::new();
        let ready = tree.get_ready_skills();
        assert_eq!(ready.len(), 5); // All should be ready initially
    }

    #[test]
    fn test_aoe_radius() {
        let slash = Skill::new(SkillType::Slash);
        let pierce = Skill::new(SkillType::Pierce);
        let whirlwind = Skill::new(SkillType::Whirlwind);

        assert_eq!(slash.get_aoe_radius(), 1);
        assert_eq!(pierce.get_aoe_radius(), 0); // Single target
        assert_eq!(whirlwind.get_aoe_radius(), 2);
    }
}
