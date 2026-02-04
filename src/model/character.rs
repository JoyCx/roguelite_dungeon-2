use crate::constants::*;
use crate::model::consumable::ConsumableInventory;
use crate::model::cooldown::Cooldown;
use crate::model::status_effect::StatusEffectManager;
use crate::model::ultimate::Ultimate;
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

    // Gold/Currency wallet
    pub gold: u32,

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
            gold: 0,
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

    pub fn apply_knockback(&mut self, dx: f32, dy: f32, force: f32) {
        self.knockback_velocity = (dx * force, dy * force);
    }

    pub fn is_damaged_animating(&self) -> bool {
        if let Some(damaged_at) = self.damaged_at {
            damaged_at.elapsed().as_secs_f32() < PLAYER_DAMAGE_ANIMATION_TIME
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
}
