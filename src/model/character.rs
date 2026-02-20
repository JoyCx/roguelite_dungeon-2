use crate::constants::*;
use crate::model::consumable::{ConsumableInventory, ConsumableType};
use crate::model::cooldown::Cooldown;
use crate::model::skill::{SkillTree, SkillType};
use crate::model::skill_tree_path::SkillTreeManager;
use crate::model::status_effect::{StatusEffect, StatusEffectManager, StatusEffectType};
use crate::model::ultimate::Ultimate;
use crate::model::ultimate_shop::UltimateShopInventory;
use crate::model::weapon::WeaponInventory;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Character {
    pub speed: f32,

    pub name: String,

    pub last_direction: (i32, i32),

    pub dash_cooldown: Cooldown,

    pub dash_distance: i32,

    // Health system
    pub health: i32,
    pub health_max: i32,

    // Attack stats
    pub attack_damage: i32,
    pub attack_length: i32, // How many blocks forward
    pub attack_width: i32,  // Width of attack area

    pub attack_cooldown: Cooldown,
    pub last_attack_time: Option<Instant>, // Track attack animation

    // Bow stats
    pub arrow_speed: f32, // Tiles per second
    pub bow_cooldown: Cooldown,

    // Block stats
    pub block_cooldown: Cooldown,

    // Weapon system
    pub weapon_inventory: WeaponInventory,

    // Consumables
    pub consumable_inventory: ConsumableInventory,

    // Status effects
    pub status_effects: StatusEffectManager,

    // Ultimate ability
    pub ultimate: Ultimate,

    // Ultimate charge (0.0 to 100.0)
    pub ultimate_charge: f32,

    // Ultimate Shop Inventory (owned ultimates and upgrades)
    pub shop_inventory: UltimateShopInventory,

    // Skill Tree
    pub skill_tree: SkillTree,

    // Skill Tree Path System
    pub skill_tree_path: SkillTreeManager,

    // Gold/Currency wallet
    pub gold: u32,

    // Stats tracking
    pub enemies_killed: u32,

    // Knockback system
    pub knockback_velocity: (f32, f32), // knockback direction and remaining force (dx, dy)
    pub damaged_at: Option<Instant>,    // timestamp of when entity was last damaged
}

impl Default for Character {
    fn default() -> Self {
        Self {
            speed: PLAYER_BASE_SPEED,
            name: "Unnamed Hero".to_string(),
            last_direction: (0, 0),
            dash_cooldown: Cooldown::new(PLAYER_DASH_COOLDOWN),
            dash_distance: PLAYER_DASH_DISTANCE,
            health: PLAYER_BASE_HEALTH,
            health_max: PLAYER_BASE_HEALTH,
            attack_damage: PLAYER_BASE_DAMAGE,
            attack_length: PLAYER_ATTACK_LENGTH,
            attack_width: PLAYER_ATTACK_WIDTH,
            attack_cooldown: Cooldown::new(PLAYER_ATTACK_COOLDOWN),
            last_attack_time: None,
            arrow_speed: PLAYER_ARROW_SPEED,
            bow_cooldown: Cooldown::new(PLAYER_BOW_COOLDOWN),
            block_cooldown: Cooldown::new(PLAYER_BLOCK_COOLDOWN),
            weapon_inventory: WeaponInventory::default(),
            consumable_inventory: ConsumableInventory::default(),
            status_effects: StatusEffectManager::default(),
            ultimate: Ultimate::default(),
            ultimate_charge: 0.0,
            shop_inventory: UltimateShopInventory::default(),
            skill_tree: SkillTree::new(),
            skill_tree_path: SkillTreeManager::new(),
            gold: 0,
            enemies_killed: 0,
            knockback_velocity: (0.0, 0.0),
            damaged_at: None,
        }
    }
}

impl Character {
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            ..Default::default()
        }
    }

    pub fn can_dash(&self) -> bool {
        self.dash_cooldown.is_ready()
    }

    pub fn dash_cooldown_remaining(&self) -> f32 {
        self.dash_cooldown.remaining_seconds()
    }

    pub fn start_dash_cooldown(&mut self) {
        self.dash_cooldown.trigger();
    }

    pub fn update_direction(&mut self, dx: i32, dy: i32) {
        if dx != 0 || dy != 0 {
            self.last_direction = (dx, dy);
        }
    }

    pub fn can_attack(&self) -> bool {
        self.attack_cooldown.is_ready()
    }

    #[allow(dead_code)]
    pub fn attack_cooldown_remaining(&self) -> f32 {
        self.attack_cooldown.remaining_seconds()
    }

    pub fn start_attack_cooldown(&mut self) {
        self.attack_cooldown.trigger();
        self.last_attack_time = Some(Instant::now());
    }

    #[allow(dead_code)]
    pub fn is_attack_animating(&self) -> bool {
        if let Some(attack_time) = self.last_attack_time {
            attack_time.elapsed().as_secs_f32() < PLAYER_ATTACK_ANIMATION_TIME
        } else {
            false
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.bow_cooldown.is_ready()
    }

    pub fn start_bow_cooldown(&mut self) {
        self.bow_cooldown.trigger();
    }

    pub fn can_block(&self) -> bool {
        self.block_cooldown.is_ready()
    }

    pub fn block_cooldown_remaining(&self) -> f32 {
        self.block_cooldown.remaining_seconds()
    }

    pub fn start_block_cooldown(&mut self) {
        self.block_cooldown.trigger();
    }

    pub fn bow_cooldown_remaining(&self) -> f32 {
        self.bow_cooldown.remaining_seconds()
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.health_max);
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.health = (self.health - amount).max(0);
        self.damaged_at = Some(Instant::now());
    }

    /// Charge ultimate ability based on damage dealt
    pub fn charge_ultimate(&mut self, damage: i32) {
        let charge_amount = self.ultimate.charge_on_hit(damage);
        self.ultimate_charge = (self.ultimate_charge + charge_amount).min(100.0);
    }

    /// Use the ultimate ability if charged
    pub fn use_ultimate(&mut self) -> bool {
        if self.ultimate_charge >= 100.0 && self.ultimate.can_use() {
            self.ultimate_charge = 0.0;
            self.ultimate.activate();
            self.ultimate.start_cooldown();
            true
        } else {
            false
        }
    }

    /// Check if player is in a Rage ultimate
    #[allow(dead_code)]
    pub fn is_raging(&self) -> bool {
        matches!(
            self.ultimate.get_active_type(),
            Some(crate::model::ultimate::UltimateType::Rage)
        )
    }

    /// Check if player is in Ghost ultimate (invulnerable)
    pub fn is_ghost(&self) -> bool {
        matches!(
            self.ultimate.get_active_type(),
            Some(crate::model::ultimate::UltimateType::Ghost)
        )
    }

    /// Get effective speed multiplier based on active ultimate
    #[allow(dead_code)]
    pub fn get_ultimate_speed_multiplier(&self) -> f32 {
        if self.is_raging() {
            2.0 // Double speed during Rage
        } else {
            1.0
        }
    }

    /// Get damage multiplier based on active ultimate
    #[allow(dead_code)]
    pub fn get_ultimate_damage_multiplier(&self) -> f32 {
        if self.is_raging() {
            2.0 // Double damage during Rage
        } else {
            1.0
        }
    }

    pub fn apply_knockback(&mut self, dx: f32, dy: f32, force: f32) {
        // Normalize direction to prevent diagonal knockback from being stronger
        // Only apply knockback in the dominant direction
        let abs_dx = dx.abs();
        let abs_dy = dy.abs();

        if abs_dx > abs_dy {
            // Knockback primarily in x direction
            self.knockback_velocity = (dx * force, 0.0);
        } else if abs_dy > abs_dx {
            // Knockback primarily in y direction
            self.knockback_velocity = (0.0, dy * force);
        } else {
            // Equal in both directions - apply diagonal
            self.knockback_velocity = (dx * force, dy * force);
        }
    }

    pub fn is_damaged_animating(&self) -> bool {
        if let Some(damaged_at) = self.damaged_at {
            damaged_at.elapsed().as_secs_f32() < PLAYER_DAMAGE_ANIMATION_TIME
        } else {
            false
        }
    }

    pub fn is_attacking_animating(&self) -> bool {
        if let Some(attack_time) = self.last_attack_time {
            attack_time.elapsed().as_secs_f32() < PLAYER_ATTACK_ANIMATION_TIME
        } else {
            false
        }
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn get_health_percentage(&self) -> f32 {
        if self.health_max == 0 {
            0.0
        } else {
            (self.health as f32 / self.health_max as f32).clamp(0.0, 1.0)
        }
    }

    pub fn add_gold(&mut self, amount: u32) {
        self.gold = self.gold.saturating_add(amount);
    }

    pub fn spend_gold(&mut self, amount: u32) -> bool {
        if self.gold >= amount {
            self.gold -= amount;
            true
        } else {
            false
        }
    }

    pub fn get_gold(&self) -> u32 {
        self.gold
    }

    /// Apply a consumable's effects to this character
    pub fn apply_consumable(&mut self, consumable_type: &ConsumableType) {
        match consumable_type {
            ConsumableType::WeakHealingDraught => {
                // Healing over 5 seconds at 2.0 hp/sec = 10 total
                self.status_effects.add(StatusEffect {
                    effect_type: StatusEffectType::Healing,
                    duration: 5.0,
                    damage_per_sec: -2.0, // Negative damage = healing
                    stacks: 1,
                });
            }
            ConsumableType::BandageRoll => {
                // Instant heal + remove bleed
                self.heal(20);
                self.status_effects.remove_type(&StatusEffectType::Bleed);
            }
            ConsumableType::AntitoxinVial => {
                // Remove poison and apply immunity
                self.status_effects.remove_type(&StatusEffectType::Poison);
                self.status_effects.add(StatusEffect::poison_immunity(10.0));
            }
            ConsumableType::FireOilFlask => {
                // This is typically thrown, not consumed directly by player
                // Can add burn effect for testing purposes
                self.status_effects.add(StatusEffect::burn(4.0));
            }
            ConsumableType::BlessedBread => {
                // Slow healing over 8 seconds at 1.0 hp/sec = 8 total
                self.status_effects.add(StatusEffect {
                    effect_type: StatusEffectType::Healing,
                    duration: 8.0,
                    damage_per_sec: -1.0,
                    stacks: 1,
                });
            }
        }
    }

    /// Apply healing from healing status effects (negative damage per sec)
    pub fn apply_healing_effects(&mut self, delta: f32) {
        let healing_amount: i32 = self
            .status_effects
            .effects
            .iter()
            .filter(|e| e.effect_type == StatusEffectType::Healing)
            .map(|e| -(e.damage_per_sec * delta * e.stacks as f32) as i32)
            .sum();

        if healing_amount > 0 {
            self.heal(healing_amount);
        }
    }

    /// Get total attack damage including weapon enchants
    pub fn get_total_attack_damage(&self) -> i32 {
        let weapon_bonus = self
            .weapon_inventory
            .get_current_weapon()
            .map(|w| w.get_total_damage() - w.damage) // Get enchant bonus only
            .unwrap_or(0);
        self.attack_damage + weapon_bonus
    }

    /// Get attack area radius including weapon bonuses
    pub fn get_attack_radius_bonus(&self) -> i32 {
        self.weapon_inventory
            .get_current_weapon()
            .map(|w| w.get_radius_bonus())
            .unwrap_or(0)
    }

    /// Calculate damage for ultimate ability (includes charge multiplier)
    pub fn calculate_ultimate_damage(&self) -> i32 {
        // Base ultimate damage plus charge bonus
        // Charge goes from 0 to 100, so multiply by (1.0 + charge/100 * 0.5) for 0.5x bonus at full charge
        let charge_multiplier = 1.0 + (self.ultimate_charge / 100.0) * 0.5;
        (self.ultimate.damage as f32 * charge_multiplier) as i32
    }

    /// Use a skill if it's ready
    pub fn use_skill(&mut self, skill_type: SkillType) -> bool {
        if let Some(skill) = self.skill_tree.get_skill_mut(skill_type) {
            if skill.is_ready() {
                skill.use_skill();
                return true;
            }
        }
        false
    }

    /// Get skill damage multiplier (applies to attack damage)
    pub fn get_skill_damage_multiplier(&self, skill_type: SkillType) -> f32 {
        self.skill_tree
            .get_skill(skill_type)
            .map(|s| s.get_damage_multiplier())
            .unwrap_or(1.0)
    }

    /// Get AoE radius from skill
    pub fn get_skill_aoe_radius(&self, skill_type: SkillType) -> i32 {
        self.skill_tree
            .get_skill(skill_type)
            .map(|s| s.get_aoe_radius())
            .unwrap_or(0)
    }

    /// Check if a skill is ready to use
    pub fn is_skill_ready(&self, skill_type: SkillType) -> bool {
        self.skill_tree
            .get_skill(skill_type)
            .map(|s| s.is_ready())
            .unwrap_or(false)
    }

    /// Get all ready skills (can use immediately)
    pub fn get_ready_skills(&self) -> Vec<SkillType> {
        self.skill_tree
            .get_ready_skills()
            .iter()
            .map(|s| s.skill_type)
            .collect()
    }

    /// Level up a skill with experience
    pub fn level_up_skill(&mut self, skill_type: SkillType) -> bool {
        if let Some(skill) = self.skill_tree.get_skill_mut(skill_type) {
            skill.level_up()
        } else {
            false
        }
    }

    /// Get ultimate radius (area of effect)
    pub fn get_ultimate_radius(&self) -> i32 {
        self.ultimate.radius
    }

    /// Get effective attack damage with skill tree bonuses applied
    pub fn get_effective_attack_damage(&self) -> i32 {
        let bonuses = self.skill_tree_path.get_total_bonuses();
        let multiplier = bonuses.damage_multiplier.max(1.0); // Ensure minimum 1.0x multiplier
        (self.attack_damage as f32 * multiplier) as i32
    }

    /// Get effective max health with skill tree bonuses applied
    pub fn get_effective_max_health(&self) -> i32 {
        let bonuses = self.skill_tree_path.get_total_bonuses();
        let multiplier = bonuses.health_multiplier.max(1.0); // Ensure minimum 1.0x multiplier
        (self.health_max as f32 * multiplier) as i32
    }

    /// Get effective speed with skill tree bonuses applied
    pub fn get_effective_speed(&self) -> f32 {
        let bonuses = self.skill_tree_path.get_total_bonuses();
        let multiplier = bonuses.speed_multiplier.max(1.0); // Ensure minimum 1.0x multiplier
        self.speed * multiplier
    }

    /// Apply skill tree stat bonuses to actual stats
    pub fn apply_skill_tree_bonuses(&mut self) {
        let bonuses = self.skill_tree_path.get_total_bonuses();

        // Apply health bonus
        let new_health_max = (PLAYER_BASE_HEALTH as f32 * bonuses.health_multiplier) as i32;
        if new_health_max > self.health_max {
            // Heal the difference when max health increases
            let difference = new_health_max - self.health_max;
            self.health = (self.health + difference).min(new_health_max);
            self.health_max = new_health_max;
        } else if new_health_max < self.health_max {
            // Cap current health if max health decreases
            self.health_max = new_health_max;
            self.health = self.health.min(new_health_max);
        }

        // Update actual speed
        self.speed = PLAYER_BASE_SPEED * bonuses.speed_multiplier;

        // Note: attack_damage is calculated via get_effective_attack_damage() when needed
        // so we don't modify it directly here
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_character_creation() {
        let character = Character::default();
        assert_eq!(character.health, PLAYER_BASE_HEALTH);
        assert_eq!(character.health_max, PLAYER_BASE_HEALTH);
        assert_eq!(character.gold, 0);
        assert!(character.is_alive());
    }

    #[test]
    fn test_cooldown_ready() {
        let character = Character::default();
        assert!(character.can_dash());
        assert!(character.can_attack());
        assert!(character.can_shoot());
        assert!(character.can_block());
    }

    #[test]
    fn test_start_cooldown() {
        let mut character = Character::default();
        assert!(character.can_dash());
        character.start_dash_cooldown();
        assert!(!character.can_dash());
    }

    #[test]
    fn test_health_management() {
        let mut character = Character::default();
        let initial_health = character.health;

        character.take_damage(10);
        assert_eq!(character.health, initial_health - 10);
        assert!(character.is_alive());

        character.heal(5);
        assert_eq!(character.health, initial_health - 5);
    }

    #[test]
    fn test_health_capped_at_max() {
        let mut character = Character::default();
        character.heal(1000);
        assert_eq!(character.health, character.health_max);
    }

    #[test]
    fn test_death_at_zero_health() {
        let mut character = Character::default();
        character.take_damage(character.health + 100);
        assert_eq!(character.health, 0);
        assert!(!character.is_alive());
    }

    #[test]
    fn test_gold_management() {
        let mut character = Character::default();
        assert_eq!(character.get_gold(), 0);

        character.add_gold(50);
        assert_eq!(character.get_gold(), 50);

        assert!(character.spend_gold(30));
        assert_eq!(character.get_gold(), 20);

        assert!(!character.spend_gold(100));
        assert_eq!(character.get_gold(), 20);
    }

    #[test]
    fn test_consumable_application() {
        let mut character = Character::default();
        let initial_health = character.health;

        // Test healing draught
        character.take_damage(20);
        assert_eq!(character.health, initial_health - 20);

        character.apply_consumable(&ConsumableType::WeakHealingDraught);
        assert!(character
            .status_effects
            .has_effect(&StatusEffectType::Healing));
    }

    #[test]
    fn test_bandage_roll_consumable() {
        let mut character = Character::default();
        character.take_damage(30);
        character
            .status_effects
            .add(StatusEffect::bleed_with_stacks(2));

        assert!(character
            .status_effects
            .has_effect(&StatusEffectType::Bleed));

        character.apply_consumable(&ConsumableType::BandageRoll);

        assert!(!character
            .status_effects
            .has_effect(&StatusEffectType::Bleed));
        assert_eq!(character.health, PLAYER_BASE_HEALTH - 10); // 30 - 20 heal
    }

    #[test]
    fn test_total_attack_damage() {
        let character = Character::default();
        assert_eq!(character.get_total_attack_damage(), PLAYER_BASE_DAMAGE);
    }

    #[test]
    fn test_ultimate_damage_calculation() {
        let mut character = Character::default();

        // At 0 charge, should be base damage
        let damage_no_charge = character.calculate_ultimate_damage();
        assert_eq!(damage_no_charge, character.ultimate.damage);

        // At full charge (100), should be 1.5x base damage
        character.ultimate_charge = 100.0;
        let damage_full_charge = character.calculate_ultimate_damage();
        assert_eq!(
            damage_full_charge,
            (character.ultimate.damage as f32 * 1.5) as i32
        );
    }

    #[test]
    fn test_direction_update() {
        let mut character = Character::default();
        assert_eq!(character.last_direction, (0, 0));

        character.update_direction(1, 0);
        assert_eq!(character.last_direction, (1, 0));

        character.update_direction(0, 0); // No change for (0, 0)
        assert_eq!(character.last_direction, (1, 0));
    }

    #[test]
    fn test_knockback() {
        let mut character = Character::default();
        character.apply_knockback(1.0, 0.0, 2.0);
        assert_eq!(character.knockback_velocity, (2.0, 0.0));
    }

    #[test]
    fn test_health_percentage() {
        let mut character = Character::default();
        assert_eq!(character.get_health_percentage(), 1.0); // 100%

        character.health = 50;
        assert_eq!(character.get_health_percentage(), 0.5); // 50%

        character.health = 0;
        assert_eq!(character.get_health_percentage(), 0.0); // 0%
    }

    #[test]
    fn test_skill_integration() {
        let mut character = Character::default();

        // Slash skill should be ready initially
        assert!(character.is_skill_ready(SkillType::Slash));

        // Use skill
        assert!(character.use_skill(SkillType::Slash));

        assert!(!character.is_skill_ready(SkillType::Slash));
    }

    #[test]
    fn test_skill_damage_multiplier() {
        let mut character = Character::default();

        let novice_multiplier = character.get_skill_damage_multiplier(SkillType::Pierce);
        assert!(novice_multiplier > 1.0); // Pierce does base 1.3x

        // Level up Pierce
        character.level_up_skill(SkillType::Pierce);
        let apprentice_multiplier = character.get_skill_damage_multiplier(SkillType::Pierce);

        assert!(apprentice_multiplier > novice_multiplier);
    }

    #[test]
    fn test_get_ready_skills() {
        let character = Character::default();
        let ready = character.get_ready_skills();

        // All 5 skills should be ready initially
        assert_eq!(ready.len(), 5);
    }

    #[test]
    fn test_skill_aoe_radius() {
        let character = Character::default();

        let slash_radius = character.get_skill_aoe_radius(SkillType::Slash);
        let whirlwind_radius = character.get_skill_aoe_radius(SkillType::Whirlwind);

        // Whirlwind should have larger AoE
        assert!(whirlwind_radius > slash_radius);
    }
}
