use crate::model::consumable::ConsumableInventory;
use crate::model::status_effect::StatusEffectManager;
use crate::model::ultimate::Ultimate;
use crate::model::weapon::WeaponInventory;
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Character {
    pub speed: f32,

    pub name: String,

    pub last_direction: (i32, i32),

    pub dash_cooldown_start: Option<Instant>,

    pub dash_cooldown_duration: f32,

    pub dash_distance: i32,

    // Health system
    pub health: i32,
    pub health_max: i32,

    // Attack stats
    pub attack_damage: i32,
    pub attack_length: i32, // How many blocks forward
    pub attack_width: i32,  // Width of attack area

    pub attack_cooldown_start: Option<Instant>,
    pub attack_cooldown_duration: f32,
    pub last_attack_time: Option<Instant>, // Track attack animation

    // Bow stats
    pub arrow_speed: f32, // Tiles per second
    pub bow_cooldown_start: Option<Instant>,
    pub bow_cooldown_duration: f32,

    // Block stats
    pub block_cooldown_start: Option<Instant>,
    pub block_cooldown_duration: f32,

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
            speed: 5.0,
            name: "Unnamed Hero".to_string(),
            last_direction: (0, 0),
            dash_cooldown_start: None,
            dash_cooldown_duration: 5.0,
            dash_distance: 5,
            health: 100,
            health_max: 100,
            attack_damage: 5,
            attack_length: 2,
            attack_width: 1,
            attack_cooldown_start: None,
            attack_cooldown_duration: 0.5, // Fast attack cooldown
            last_attack_time: None,
            arrow_speed: 8.0, // Tiles per second
            bow_cooldown_start: None,
            bow_cooldown_duration: 0.3, // Fast bow cooldown
            block_cooldown_start: None,
            block_cooldown_duration: 6.0, // 6 second block cooldown
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
        match self.dash_cooldown_start {
            None => true,
            Some(start_time) => start_time.elapsed().as_secs_f32() >= self.dash_cooldown_duration,
        }
    }

    pub fn dash_cooldown_remaining(&self) -> f32 {
        match self.dash_cooldown_start {
            None => 0.0,
            Some(start_time) => {
                let elapsed = start_time.elapsed().as_secs_f32();
                (self.dash_cooldown_duration - elapsed).max(0.0)
            }
        }
    }

    pub fn start_dash_cooldown(&mut self) {
        self.dash_cooldown_start = Some(Instant::now());
    }

    pub fn update_direction(&mut self, dx: i32, dy: i32) {
        if dx != 0 || dy != 0 {
            self.last_direction = (dx, dy);
        }
    }

    pub fn can_attack(&self) -> bool {
        match self.attack_cooldown_start {
            None => true,
            Some(start_time) => start_time.elapsed().as_secs_f32() >= self.attack_cooldown_duration,
        }
    }

    #[allow(dead_code)]
    pub fn attack_cooldown_remaining(&self) -> f32 {
        match self.attack_cooldown_start {
            None => 0.0,
            Some(start_time) => {
                let elapsed = start_time.elapsed().as_secs_f32();
                (self.attack_cooldown_duration - elapsed).max(0.0)
            }
        }
    }

    pub fn start_attack_cooldown(&mut self) {
        self.attack_cooldown_start = Some(Instant::now());
        self.last_attack_time = Some(Instant::now());
    }

    #[allow(dead_code)]
    pub fn is_attack_animating(&self) -> bool {
        if let Some(attack_time) = self.last_attack_time {
            attack_time.elapsed().as_secs_f32() < 0.2 // 200ms animation
        } else {
            false
        }
    }
    pub fn can_shoot(&self) -> bool {
        match self.bow_cooldown_start {
            None => true,
            Some(start_time) => start_time.elapsed().as_secs_f32() >= self.bow_cooldown_duration,
        }
    }

    pub fn start_bow_cooldown(&mut self) {
        self.bow_cooldown_start = Some(Instant::now());
    }

    pub fn can_block(&self) -> bool {
        match self.block_cooldown_start {
            None => true,
            Some(start_time) => start_time.elapsed().as_secs_f32() >= self.block_cooldown_duration,
        }
    }

    pub fn block_cooldown_remaining(&self) -> f32 {
        match self.block_cooldown_start {
            None => 0.0,
            Some(start_time) => {
                let elapsed = start_time.elapsed().as_secs_f32();
                (self.block_cooldown_duration - elapsed).max(0.0)
            }
        }
    }

    pub fn start_block_cooldown(&mut self) {
        self.block_cooldown_start = Some(Instant::now());
    }

    pub fn bow_cooldown_remaining(&self) -> f32 {
        match self.bow_cooldown_start {
            None => 0.0,
            Some(start_time) => {
                let elapsed = start_time.elapsed().as_secs_f32();
                (self.bow_cooldown_duration - elapsed).max(0.0)
            }
        }
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
            damaged_at.elapsed().as_secs_f32() < 1.0
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
