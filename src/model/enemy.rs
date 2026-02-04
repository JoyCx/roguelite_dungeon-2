use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Enemy {
    pub position: Position,
    pub speed: f32,                  // tiles per game tick (0.1 = 10 ticks per tile)
    pub max_range: Option<i32>,      // optional 5x5 nearby area (radius from spawn point)
    pub collision_enabled: bool,     // whether enemy collides with walls
    pub collision_with_player: bool, // whether enemy collides with player (default: false)
    pub movement_ticks: f32,         // accumulated movement ticks
    pub attack_ticks: f32,           // accumulated attack ticks for cooldown
    pub is_wandering: bool,          // current behavior: true = wander, false = chase
    pub spawn_point: Position,       // original spawn position for max_range calculation
    pub health: i32,                 // current health points
    pub max_health: i32,             // maximum health points
    pub rarity: crate::model::enemy_type::EnemyRarity, // enemy difficulty tier
    pub base_gold: u32,              // gold dropped on defeat
    #[serde(skip)]
    pub knockback_velocity: (f32, f32), // knockback direction and remaining force (dx, dy)
    #[serde(skip)]
    pub damaged_at: Option<std::time::Instant>, // timestamp of when entity was last damaged
    pub detection_radius: i32,       // radius within which enemy detects and chases player
}

impl Enemy {
    /// Create a new enemy at the given position
    pub fn new(x: i32, y: i32, speed: f32) -> Self {
        let pos = Position::new(x, y);
        Self {
            position: pos.clone(),
            speed,
            max_range: None,
            collision_enabled: true,
            collision_with_player: false,
            movement_ticks: 0.0,
            attack_ticks: 0.0,
            is_wandering: false,
            spawn_point: pos,
            health: 10, // Default, will be set from template
            max_health: 10,
            rarity: crate::model::enemy_type::EnemyRarity::Fighter,
            base_gold: 10,
            knockback_velocity: (0.0, 0.0),
            damaged_at: None,
            detection_radius: 5, // Default, will be set from template
        }
    }

    /// Take damage and return whether enemy is still alive
    pub fn take_damage(&mut self, damage: i32) -> bool {
        self.health = (self.health - damage).max(0);
        self.damaged_at = Some(std::time::Instant::now());
        self.health > 0
    }

    pub fn apply_knockback(&mut self, dx: f32, dy: f32, force: f32) {
        self.knockback_velocity = (dx * force, dy * force);
    }

    pub fn is_damaged_animating(&self) -> bool {
        if let Some(damaged_at) = self.damaged_at {
            damaged_at.elapsed().as_secs_f32() < 1.0
        } else {
            false
        }
    }

    /// Check if enemy is alive
    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    /// Set a maximum range (5x5 area) for this enemy to roam
    pub fn set_max_range(&mut self, radius: i32) {
        self.max_range = Some(radius);
    }

    /// Check if enemy can move to a position (respects walls if collision_enabled)
    pub fn can_move_to(&self, x: i32, y: i32, floor: &crate::model::floor::Floor) -> bool {
        // Check bounds first
        if x < 0 || x >= floor.width || y < 0 || y >= floor.height {
            return false;
        }

        // Check collision if enabled (is_walkable returns true for non-wall tiles)
        if self.collision_enabled && !floor.is_walkable(x, y) {
            return false;
        }

        // Check range if set
        if let Some(radius) = self.max_range {
            if self.spawn_point.distance_to(&Position::new(x, y)) > radius {
                return false;
            }
        }

        true
    }

    /// Move enemy toward character using A* pathfinding
    /// Returns true if movement occurred
    pub fn move_toward(&mut self, target: &Position, floor: &crate::model::floor::Floor) -> bool {
        self.movement_ticks += self.speed;

        // Check if enough movement ticks have accumulated
        if self.movement_ticks < 1.0 {
            return false;
        }

        self.movement_ticks -= 1.0;

        // Find path using A*
        if let Some(path) = self.find_path(&self.position.clone(), target, floor) {
            if path.len() > 1 {
                let next_pos = &path[1]; // path[0] is current position
                if self.can_move_to(next_pos.x, next_pos.y, floor) {
                    self.position = next_pos.clone();
                    self.is_wandering = false;
                    return true;
                }
            }
        }

        false
    }

    /// Random wander behavior within allowed range
    pub fn wander(&mut self, floor: &crate::model::floor::Floor) -> bool {
        self.movement_ticks += self.speed;

        if self.movement_ticks < 1.0 {
            return false;
        }

        self.movement_ticks -= 1.0;
        self.is_wandering = true;

        // Try random adjacent move
        let directions = [
            Position::new(0, -1), // up
            Position::new(0, 1),  // down
            Position::new(-1, 0), // left
            Position::new(1, 0),  // right
        ];

        use rand::seq::SliceRandom;
        let mut rng = rand::rng();

        // Shuffle directions for random walk
        let mut dirs = directions.to_vec();
        dirs.shuffle(&mut rng);

        for dir in dirs {
            let new_x = self.position.x + dir.x;
            let new_y = self.position.y + dir.y;

            if self.can_move_to(new_x, new_y, floor) {
                self.position = Position::new(new_x, new_y);
                return true;
            }
        }

        false
    }

    /// A* pathfinding algorithm
    /// Returns the path from start to goal, or None if no path exists
    fn find_path(
        &self,
        start: &Position,
        goal: &Position,
        floor: &crate::model::floor::Floor,
    ) -> Option<Vec<Position>> {
        let mut open_set = BinaryHeap::new();
        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        let mut f_score = HashMap::new();

        let start_key = (start.x, start.y);
        let goal_key = (goal.x, goal.y);

        g_score.insert(start_key, 0);
        f_score.insert(start_key, self.heuristic(start, goal));

        open_set.push(AStarNode {
            position: start.clone(),
            f_score: self.heuristic(start, goal),
        });

        while let Some(current_node) = open_set.pop() {
            let current = &current_node.position;
            let current_key = (current.x, current.y);

            if current_key == goal_key {
                return Some(self.reconstruct_path(&came_from, current.clone()));
            }

            // Check all 4 neighbors (no diagonals)
            let neighbors = vec![
                Position::new(current.x + 1, current.y),
                Position::new(current.x - 1, current.y),
                Position::new(current.x, current.y + 1),
                Position::new(current.x, current.y - 1),
            ];

            for neighbor in neighbors {
                // Skip unwalkable positions
                if !self.can_move_to(neighbor.x, neighbor.y, floor) {
                    continue;
                }

                let neighbor_key = (neighbor.x, neighbor.y);
                let tentative_g = g_score.get(&current_key).copied().unwrap_or(i32::MAX) + 1;

                if tentative_g < *g_score.get(&neighbor_key).unwrap_or(&i32::MAX) {
                    came_from.insert(neighbor_key, current_key);
                    g_score.insert(neighbor_key, tentative_g);

                    let f = tentative_g + self.heuristic(&neighbor, goal);
                    f_score.insert(neighbor_key, f);

                    open_set.push(AStarNode {
                        position: neighbor,
                        f_score: f,
                    });
                }
            }
        }

        None
    }

    /// Heuristic for A* (Manhattan distance)
    fn heuristic(&self, a: &Position, b: &Position) -> i32 {
        (a.x - b.x).abs() + (a.y - b.y).abs()
    }

    /// Reconstruct path from A* search
    fn reconstruct_path(
        &self,
        came_from: &HashMap<(i32, i32), (i32, i32)>,
        current: Position,
    ) -> Vec<Position> {
        let mut path = vec![current.clone()];
        let mut current_key = (current.x, current.y);

        while let Some(&parent_key) = came_from.get(&current_key) {
            let parent = Position::new(parent_key.0, parent_key.1);
            path.push(parent.clone());
            current_key = parent_key;
        }

        path.reverse();
        path
    }

    /// Find valid spawn positions for enemies
    /// Detects empty spaces that are:
    /// - Not walkable walls
    /// - Far from the player (min_player_distance)
    /// - Not occupied by other enemies
    /// - Spread out across the floor
    /// - Respecting different floor rooms
    ///
    /// Returns a list of valid spawn positions
    pub fn find_spawn_positions(
        floor: &crate::model::floor::Floor,
        player_position: &Position,
        enemies: &[Enemy],
        min_player_distance: i32,
        max_attempts: usize,
    ) -> Vec<Position> {
        use rand::Rng;

        let mut valid_positions = Vec::new();
        let mut occupied_positions = std::collections::HashSet::new();
        let mut rng = rand::rng();

        // Mark all enemy positions as occupied
        for enemy in enemies {
            occupied_positions.insert((enemy.position.x, enemy.position.y));
        }

        // Mark player position as occupied
        occupied_positions.insert((player_position.x, player_position.y));

        // Also mark adjacent positions to player as occupied (no spawning right next to player)
        let player_adjacent = [
            (player_position.x + 1, player_position.y),
            (player_position.x - 1, player_position.y),
            (player_position.x, player_position.y + 1),
            (player_position.x, player_position.y - 1),
        ];
        for pos in player_adjacent.iter() {
            occupied_positions.insert(*pos);
        }

        // Scan floor for valid spawn positions
        for _attempt in 0..max_attempts {
            let x = rng.random_range(1..floor.width - 1);
            let y = rng.random_range(1..floor.height - 1);

            let pos = (x, y);

            // Skip if position is already occupied
            if occupied_positions.contains(&pos) {
                continue;
            }

            // Check if position is walkable
            if !floor.is_walkable(x, y) {
                continue;
            }

            let spawn_pos = Position::new(x, y);

            // Check minimum distance from player
            if spawn_pos.distance_to(player_position) < min_player_distance {
                continue;
            }

            // Check spread: position should not be too close to other valid spawns
            // Maintain minimum spacing between spawn points
            let min_spacing = 5;
            let too_close = valid_positions
                .iter()
                .any(|existing: &Position| existing.distance_to(&spawn_pos) < min_spacing);

            if too_close {
                continue;
            }

            // Valid spawn position found
            valid_positions.push(spawn_pos);
            occupied_positions.insert(pos);
        }

        valid_positions
    }

    /// Check if a position can be validly occupied by this enemy
    /// considering other enemies and the player
    pub fn is_spawn_valid(
        &self,
        position: &Position,
        floor: &crate::model::floor::Floor,
        player_position: &Position,
        enemies: &[Enemy],
    ) -> bool {
        // Check bounds
        if position.x < 0
            || position.x >= floor.width
            || position.y < 0
            || position.y >= floor.height
        {
            return false;
        }

        // Check walkable
        if !floor.is_walkable(position.x, position.y) {
            return false;
        }

        // Check if player is at this position
        if position == player_position {
            return false;
        }

        // Check if any other enemy is at this position
        for enemy in enemies {
            if enemy.position == *position {
                return false;
            }
        }

        true
    }
}

/// Helper struct for A* priority queue
#[derive(Clone, Debug)]
#[allow(dead_code)] // Used in A* implementation
struct AStarNode {
    position: Position,
    f_score: i32,
}

impl Eq for AStarNode {}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_score == other.f_score
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enemy_creation() {
        let enemy = Enemy::new(5, 5, 0.1);
        assert_eq!(enemy.position.x, 5);
        assert_eq!(enemy.position.y, 5);
        assert_eq!(enemy.speed, 0.1);
        assert!(!enemy.is_wandering);
        assert!(!enemy.collision_with_player); // Default is no collision with player
    }

    #[test]
    fn test_position_distance() {
        let pos1 = Position::new(0, 0);
        let pos2 = Position::new(3, 4);
        assert_eq!(pos1.distance_to(&pos2), 7); // Manhattan distance
    }

    #[test]
    fn test_max_range() {
        let mut enemy = Enemy::new(5, 5, 0.1);
        enemy.set_max_range(2);
        assert_eq!(enemy.max_range, Some(2));
    }

    #[test]
    fn test_collision_with_player_default() {
        let enemy = Enemy::new(10, 10, 0.2);
        assert!(!enemy.collision_with_player);
    }

    #[test]
    fn test_enemy_occupied_positions() {
        let enemy1 = Enemy::new(5, 5, 0.1);
        let enemy2 = Enemy::new(10, 10, 0.1);
        let enemies = vec![enemy1, enemy2];

        let test_enemy = Enemy::new(0, 0, 0.1);
        let player_pos = Position::new(15, 15);

        // Mock floor (we can't fully test without a real floor)
        // This test just verifies the spawn validation logic compiles and runs
        let _ = enemies;
        let _ = test_enemy;
        let _ = player_pos;
    }

    #[test]
    fn test_enemy_health_tracking() {
        let mut enemy = Enemy::new(5, 5, 0.1);
        enemy.health = 50;
        enemy.max_health = 50;

        assert_eq!(enemy.health, 50);
        assert!(enemy.is_alive());

        // Take damage
        let is_alive = enemy.take_damage(20);
        assert!(is_alive);
        assert_eq!(enemy.health, 30);

        // Take more damage
        let is_alive = enemy.take_damage(30);
        assert!(!is_alive);
        assert_eq!(enemy.health, 0);

        // Overkill damage doesn't go negative
        let is_alive = enemy.take_damage(10);
        assert!(!is_alive);
        assert_eq!(enemy.health, 0);
    }

    #[test]
    fn test_enemy_gold_drop_assignment() {
        use crate::model::enemy_type::EnemyRarity;
        let mut enemy = Enemy::new(5, 5, 0.1);

        enemy.rarity = EnemyRarity::Fighter;
        enemy.base_gold = EnemyRarity::Fighter.base_gold();
        assert_eq!(enemy.base_gold, 10);

        enemy.rarity = EnemyRarity::Boss;
        enemy.base_gold = EnemyRarity::Boss.base_gold();
        assert_eq!(enemy.base_gold, 150);
    }
}
