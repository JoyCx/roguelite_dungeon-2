use serde::{Deserialize, Serialize};

/// Represents a stat boost path in the skill tree
#[derive(Clone, Debug, Copy, Eq, PartialEq, Serialize, Deserialize)]
pub enum PathType {
    /// Warrior path: +15% health per level, blocks Mage and Rogue paths
    Warrior,
    /// Mage path: +20% attack damage per level, blocks Warrior and Rogue paths
    Mage,
    /// Rogue path: +25% speed per level, blocks Warrior and Mage paths
    Rogue,
    /// Balanced path: +8% all stats per level, blocks other paths
    Balanced,
}

impl PathType {
    pub fn name(&self) -> &'static str {
        match self {
            PathType::Warrior => "Warrior",
            PathType::Mage => "Mage",
            PathType::Rogue => "Rogue",
            PathType::Balanced => "Balanced",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            PathType::Warrior => "Increase maximum health by 15% per level",
            PathType::Mage => "Increase attack damage by 20% per level",
            PathType::Rogue => "Increase movement speed by 25% per level",
            PathType::Balanced => "Increase all stats by 8% per level",
        }
    }

    pub fn cost_at_level(&self, level: u32) -> u32 {
        // Exponential cost: 100 gold base, increases per level
        100 + (level * 50)
    }

    /// Get all available paths
    pub fn all_paths() -> Vec<PathType> {
        vec![
            PathType::Warrior,
            PathType::Mage,
            PathType::Rogue,
            PathType::Balanced,
        ]
    }

    /// Check if this path conflicts with another (blocks progress if opposite is chosen)
    pub fn conflicts_with(&self, other: PathType) -> bool {
        match (self, other) {
            (PathType::Warrior, PathType::Mage) | (PathType::Mage, PathType::Warrior) => true,
            (PathType::Warrior, PathType::Rogue) | (PathType::Rogue, PathType::Warrior) => true,
            (PathType::Mage, PathType::Rogue) | (PathType::Rogue, PathType::Mage) => true,
            _ => false,
        }
    }
}

/// Represents player progress along a skill tree path
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillPathNode {
    pub path_type: PathType,
    pub level: u32,      // 0 = not purchased, 1+ = purchased
    pub total_cost: u32, // Total gold spent on this path
    pub stat_bonus: StatBonus,
}

/// Stat bonuses applied by a skill path
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct StatBonus {
    pub health_multiplier: f32, // e.g., 1.15 = +15%
    pub damage_multiplier: f32, // e.g., 1.20 = +20%
    pub speed_multiplier: f32,  // e.g., 1.25 = +25%
}

impl Default for StatBonus {
    fn default() -> Self {
        StatBonus {
            health_multiplier: 1.0,
            damage_multiplier: 1.0,
            speed_multiplier: 1.0,
        }
    }
}

impl SkillPathNode {
    /// Create a new path node
    pub fn new(path_type: PathType) -> Self {
        Self {
            path_type,
            level: 0,
            total_cost: 0,
            stat_bonus: StatBonus::default(),
        }
    }

    /// Try to upgrade this path with given gold
    pub fn try_upgrade(&mut self, gold: &mut u32) -> bool {
        let cost = self.path_type.cost_at_level(self.level);
        if *gold >= cost {
            *gold -= cost;
            self.level += 1;
            self.total_cost += cost;
            self.update_stat_bonus();
            true
        } else {
            false
        }
    }

    /// Update stat bonus based on current level
    fn update_stat_bonus(&mut self) {
        self.stat_bonus = match self.path_type {
            PathType::Warrior => StatBonus {
                health_multiplier: 1.0 + (0.15 * self.level as f32),
                damage_multiplier: 1.0,
                speed_multiplier: 1.0,
            },
            PathType::Mage => StatBonus {
                health_multiplier: 1.0,
                damage_multiplier: 1.0 + (0.20 * self.level as f32),
                speed_multiplier: 1.0,
            },
            PathType::Rogue => StatBonus {
                health_multiplier: 1.0,
                damage_multiplier: 1.0,
                speed_multiplier: 1.0 + (0.25 * self.level as f32),
            },
            PathType::Balanced => StatBonus {
                health_multiplier: 1.0 + (0.08 * self.level as f32),
                damage_multiplier: 1.0 + (0.08 * self.level as f32),
                speed_multiplier: 1.0 + (0.08 * self.level as f32),
            },
        };
    }

    pub fn is_unlocked(&self) -> bool {
        self.level > 0
    }

    pub fn can_upgrade(&self, gold: u32) -> bool {
        gold >= self.path_type.cost_at_level(self.level)
    }

    pub fn get_next_cost(&self) -> u32 {
        self.path_type.cost_at_level(self.level)
    }
}

/// Complete skill tree system for the player
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkillTreeManager {
    pub paths: Vec<SkillPathNode>,
    pub chosen_path: Option<PathType>, // Only one path can be chosen to block others
}

impl SkillTreeManager {
    /// Create new skill tree with all paths available
    pub fn new() -> Self {
        let paths = PathType::all_paths()
            .into_iter()
            .map(SkillPathNode::new)
            .collect();

        Self {
            paths,
            chosen_path: None,
        }
    }

    /// Try to purchase an upgrade for a specific path
    pub fn purchase_upgrade(&mut self, path_type: PathType, gold: &mut u32) -> bool {
        // Check if this path conflicts with chosen path
        if let Some(chosen) = self.chosen_path {
            if path_type.conflicts_with(chosen) {
                return false; // Cannot upgrade conflicting path
            }
        }

        // Find and upgrade the path
        if let Some(node) = self.paths.iter_mut().find(|p| p.path_type == path_type) {
            if node.try_upgrade(gold) {
                // Set as chosen path if first upgrade
                if self.chosen_path.is_none() {
                    self.chosen_path = Some(path_type);
                }
                return true;
            }
        }

        false
    }

    /// Get a specific path node
    pub fn get_path(&self, path_type: PathType) -> Option<&SkillPathNode> {
        self.paths.iter().find(|p| p.path_type == path_type)
    }

    /// Get total combined stat bonuses
    pub fn get_total_bonuses(&self) -> StatBonus {
        let mut total = StatBonus::default();

        for node in &self.paths {
            if node.is_unlocked() {
                total.health_multiplier *= node.stat_bonus.health_multiplier;
                total.damage_multiplier *= node.stat_bonus.damage_multiplier;
                total.speed_multiplier *= node.stat_bonus.speed_multiplier;
            }
        }

        total
    }

    /// Get available paths to purchase (not blocked)
    pub fn get_available_paths(&self) -> Vec<PathType> {
        PathType::all_paths()
            .into_iter()
            .filter(|path| {
                if let Some(chosen) = self.chosen_path {
                    !path.conflicts_with(chosen) || *path == chosen
                } else {
                    true
                }
            })
            .collect()
    }
}

impl Default for SkillTreeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_creation() {
        let node = SkillPathNode::new(PathType::Warrior);
        assert_eq!(node.level, 0);
        assert!(!node.is_unlocked());
    }

    #[test]
    fn test_upgrade_with_gold() {
        let mut node = SkillPathNode::new(PathType::Warrior);
        let mut gold = 150;

        assert!(node.try_upgrade(&mut gold));
        assert_eq!(node.level, 1);
        assert_eq!(gold, 50); // 150 - 100
        assert!(node.is_unlocked());
    }

    #[test]
    fn test_insufficient_gold() {
        let mut node = SkillPathNode::new(PathType::Warrior);
        let mut gold = 50;

        assert!(!node.try_upgrade(&mut gold));
        assert_eq!(node.level, 0);
        assert_eq!(gold, 50); // Unchanged
    }

    #[test]
    fn test_path_conflicts() {
        assert!(PathType::Warrior.conflicts_with(PathType::Mage));
        assert!(PathType::Warrior.conflicts_with(PathType::Rogue));
        assert!(PathType::Mage.conflicts_with(PathType::Rogue));
        assert!(!PathType::Balanced.conflicts_with(PathType::Warrior));
    }

    #[test]
    fn test_skill_tree_manager() {
        let mut manager = SkillTreeManager::new();
        let mut gold = 300;

        // Purchase first upgrade in Warrior path
        assert!(manager.purchase_upgrade(PathType::Warrior, &mut gold));
        assert_eq!(manager.chosen_path, Some(PathType::Warrior));

        // Cannot purchase conflicting Mage path
        assert!(!manager.purchase_upgrade(PathType::Mage, &mut gold));

        // Can purchase same path again
        assert!(manager.purchase_upgrade(PathType::Warrior, &mut gold));
        assert_eq!(manager.get_path(PathType::Warrior).unwrap().level, 2);
    }

    #[test]
    fn test_stat_bonuses() {
        let mut node = SkillPathNode::new(PathType::Mage);
        let mut gold = 500;

        node.try_upgrade(&mut gold); // Level 1
        assert!((node.stat_bonus.damage_multiplier - 1.20).abs() < 0.01);

        node.try_upgrade(&mut gold); // Level 2
        assert!((node.stat_bonus.damage_multiplier - 1.40).abs() < 0.01);
    }
}
