use crate::constants::GAME_TICK_RATE_MS;
use crate::model::arrow::Arrow;
use crate::model::character::Character;
use crate::model::floor::Floor;
use crate::model::gamesave::{GameSave, PlayerStats};
use crate::model::particle::ParticleSystem;
use crate::model::pathfinding_cache::PathfindingCache;
use crate::model::settings::Settings;
use ratatui::prelude::Color;
use ratatui::widgets::ListState;
use std::time::Instant;

pub enum AppState {
    MainMenu,
    CharacterCreation,
    Settings,
    Game,
    DevMenu,
}

#[derive(PartialEq)]
pub enum SettingsMode {
    Navigating,
    Rebinding,
}

pub struct App {
    pub state: AppState,
    pub settings: Settings,
    pub temp_settings: Settings,
    pub settings_mode: SettingsMode,
    pub main_menu_state: ListState,
    pub settings_state: ListState,
    pub should_quit: bool,
    pub scroll_offset: u16,
    pub last_scroll_offset: u16,
    pub start_time: Instant,
    pub frame_count: u64,
    pub is_auto_scrolling: bool,
    pub scroll_target: f32,
    pub dev_seed_input: String,
    pub current_floor: Option<Floor>,
    pub character_position: (i32, i32),
    pub character: Character,
    pub terminal_size: (u16, u16),
    pub camera_offset: (f32, f32),
    pub camera_target: (f32, f32),
    pub last_game_tick: Instant,
    pub game_tick_rate_ms: u128,
    pub floor_level: u32,
    pub arrows: Vec<Arrow>,
    pub inventory_focused: bool,
    pub inventory_scroll_index: usize,
    pub showing_item_description: bool,
    pub is_paused: bool,
    pub particle_system: ParticleSystem,
    pub pathfinding_cache: PathfindingCache,
    pub movement_tick_counter: u32, // Counter to track movement cooldown
    // Character creation state
    pub char_name: String,
    pub char_name_input_mode: bool,
    pub char_creation_selection: usize, // 0 = name field, 1 = difficulty, 2 = start button
}

impl App {
    pub fn new() -> Self {
        let s = Settings::load();
        let mut menu_s = ListState::default();
        menu_s.select(Some(0));
        let mut set_s = ListState::default();
        set_s.select(Some(0));

        let now = Instant::now();

        Self {
            state: AppState::MainMenu,
            settings: s.clone(),
            temp_settings: s,
            settings_mode: SettingsMode::Navigating,
            main_menu_state: menu_s,
            settings_state: set_s,
            should_quit: false,
            scroll_offset: 0,
            is_auto_scrolling: true,
            scroll_target: 60.0,
            last_scroll_offset: 0,
            start_time: now,
            frame_count: 0,
            dev_seed_input: String::new(),
            current_floor: None,
            character_position: (0, 0),
            character: Character::default(),
            terminal_size: (0, 0),
            camera_offset: (0.0, 0.0),
            camera_target: (0.0, 0.0),
            last_game_tick: now,
            game_tick_rate_ms: GAME_TICK_RATE_MS,
            floor_level: 1,
            arrows: Vec::new(),
            inventory_focused: false,
            inventory_scroll_index: 0,
            showing_item_description: false,
            is_paused: false,
            particle_system: ParticleSystem::new(),
            pathfinding_cache: PathfindingCache::new(500), // Cache up to 500 pathfinding queries
            char_name: String::new(),
            char_name_input_mode: false,
            char_creation_selection: 0,
            movement_tick_counter: 0,
        }
    }

    pub fn set_scroll(&mut self, offset: u16) {
        let max_scroll = 60;
        self.scroll_offset = offset.min(max_scroll);
        self.last_scroll_offset = self.scroll_offset;
        self.is_auto_scrolling = false;
    }

    pub fn update_auto_scroll(&mut self) {
        if !self.is_auto_scrolling {
            return;
        }

        let distance = self.scroll_target - self.scroll_offset as f32;

        let speed_factor = 0.05;

        if distance.abs() > 0.5 {
            self.last_scroll_offset = self.scroll_offset;
            let step = distance * speed_factor;
            self.scroll_offset = (self.scroll_offset as f32 + step).round() as u16;
        } else {
            self.last_scroll_offset = self.scroll_offset;
            self.scroll_offset = self.scroll_target as u16;
            self.is_auto_scrolling = false;
        }
    }

    pub fn skip_auto_scroll(&mut self) {
        if self.is_auto_scrolling {
            self.last_scroll_offset = self.scroll_offset;
            self.scroll_offset = self.scroll_target as u16;
            self.is_auto_scrolling = false;
        }
    }

    pub fn regenerate_floor(&mut self) {
        let seed = self.dev_seed_input.parse::<u64>().unwrap_or(0);
        let mut floor = Floor::new(180, 60, seed);

        // Spawn random items on the floor
        let difficulty = self.settings.difficulty.clone();
        floor.spawn_random_items(10, &difficulty); // Spawn 10 random consumable items

        // Spawn enemies scaled by difficulty
        floor.spawn_enemies(&difficulty);

        self.current_floor = Some(floor);

        if let Some(floor) = &self.current_floor {
            // Find a random spawn position in the main connected region
            if let Some((x, y)) = floor.find_player_spawn() {
                self.character_position = (x, y);
                self.update_camera();
            }
        }
    }

    pub fn roll_random_seed(&mut self) {
        use rand::Rng;
        let new_seed: u64 = rand::rng().random_range(0..=u64::MAX);
        self.dev_seed_input = new_seed.to_string();
        self.regenerate_floor();
    }

    pub fn update_terminal_size(&mut self, width: u16, height: u16) {
        self.terminal_size = (width, height);
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if let Some(floor) = &self.current_floor {
            floor.is_walkable(x, y)
        } else {
            true
        }
    }

    pub fn should_tick(&self) -> bool {
        !self.is_paused && self.last_game_tick.elapsed().as_millis() >= self.game_tick_rate_ms
    }

    pub fn consume_tick(&mut self) {
        self.last_game_tick = Instant::now();
    }

    pub fn move_character(&mut self, dx: i32, dy: i32) {
        use std::io::Write;

        // Increment movement counter
        self.movement_tick_counter += 1;

        // Only allow movement every N ticks
        if self.movement_tick_counter < crate::constants::PLAYER_MOVEMENT_TICKS_REQUIRED {
            return;
        }

        self.movement_tick_counter = 0;

        let new_x = self.character_position.0 + dx;
        let new_y = self.character_position.1 + dy;

        let msg = format!(
            "\n=== MOVE ATTEMPT ===\nCurrent pos: ({}, {})\nTarget pos: ({}, {})\n",
            self.character_position.0, self.character_position.1, new_x, new_y
        );
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")
            .and_then(|mut f| f.write_all(msg.as_bytes()));

        if self.is_walkable(new_x, new_y) {
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .and_then(|mut f| f.write_all(b"MOVE SUCCESSFUL\n"));

            self.character_position = (new_x, new_y);
            self.character.update_direction(dx, dy);
            self.pickup_items();
            self.update_camera();
            self.consume_tick();
        } else {
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .and_then(|mut f| f.write_all(b"MOVE BLOCKED\n"));
        }
    }

    pub fn dash(&mut self) {
        use std::io::Write;

        if !self.character.can_dash() || !self.should_tick() {
            return;
        }

        let (dx, dy) = self.character.last_direction;

        if dx == 0 && dy == 0 {
            return;
        }

        let dash_dist = self.character.dash_distance;
        let new_x = self.character_position.0 + (dx * dash_dist);
        let new_y = self.character_position.1 + (dy * dash_dist);

        let msg = format!(
            "\n=== DASH ATTEMPT ===\nCurrent pos: ({}, {})\nDash dir: ({}, {})\nTarget pos: ({}, {})\n",
            self.character_position.0, self.character_position.1, dx, dy, new_x, new_y
        );
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")
            .and_then(|mut f| f.write_all(msg.as_bytes()));

        if self.is_walkable(new_x, new_y) {
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .and_then(|mut f| f.write_all(b"DASH SUCCESSFUL\n"));

            self.character_position = (new_x, new_y);
            self.character.start_dash_cooldown();
            self.update_camera();
            self.consume_tick();
        } else {
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .and_then(|mut f| f.write_all(b"DASH BLOCKED BY WALL\n"));
        }
    }

    pub fn attack(&mut self) {
        use std::io::Write;

        if !self.character.can_attack() || !self.should_tick() {
            return;
        }

        let (dx, dy) = self.character.last_direction;

        // Default to forward direction if no direction set
        let (attack_dx, attack_dy) = if dx == 0 && dy == 0 {
            (0, 1) // Default: attack forward
        } else {
            (dx, dy)
        };

        let damage = self.character.attack_damage;
        let length = self.character.attack_length;
        let width = self.character.attack_width;

        let msg = format!(
            "\n=== ATTACK ===\nPosition: ({}, {})\nDirection: ({}, {})\nDamage: {}\nRange: {} blocks, Width: {}\n",
            self.character_position.0,
            self.character_position.1,
            attack_dx,
            attack_dy,
            damage,
            length,
            width
        );
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")
            .and_then(|mut f| f.write_all(msg.as_bytes()));

        // Start the attack animation and cooldown
        self.character.start_attack_cooldown();
        self.consume_tick();
    }

    #[allow(dead_code)]
    pub fn get_attack_area(&self) -> Vec<(i32, i32)> {
        if let Some(weapon) = self.character.weapon_inventory.get_current_weapon() {
            let (dx, dy) = self.character.last_direction;
            let (attack_dx, attack_dy) = if dx == 0 && dy == 0 { (0, 1) } else { (dx, dy) };

            weapon
                .reach_shape
                .get_affected_tiles((attack_dx, attack_dy), self.character_position)
        } else {
            Vec::new()
        }
    }

    pub fn shoot(&mut self) {
        use std::io::Write;

        if !self.character.can_shoot() || !self.should_tick() {
            return;
        }

        let (dx, dy) = self.character.last_direction;
        let (shoot_dx, shoot_dy) = if dx == 0 && dy == 0 {
            (0, 1) // Default: shoot forward
        } else {
            (dx, dy)
        };

        let arrow = Arrow::new(
            self.character_position.0 as f32,
            self.character_position.1 as f32,
            shoot_dx,
            shoot_dy,
            self.character.arrow_speed,
        );

        self.arrows.push(arrow);
        self.character.start_bow_cooldown();
        self.consume_tick();

        let msg = format!(
            "\n=== BOW SHOT ===\nPosition: ({}, {})\nDirection: ({}, {})\nArrow Speed: {}\n",
            self.character_position.0,
            self.character_position.1,
            shoot_dx,
            shoot_dy,
            self.character.arrow_speed
        );
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")
            .and_then(|mut f| f.write_all(msg.as_bytes()));
    }

    pub fn update_arrows(&mut self) {
        let frame_time = (self.game_tick_rate_ms as f32) / 1000.0;

        // Update arrows and check collisions
        for arrow in self.arrows.iter_mut() {
            arrow.update(frame_time);
        }

        // Check for wall collisions (separate from update to avoid borrow issues)
        let mut indices_to_stop = Vec::new();
        for (idx, arrow) in self.arrows.iter().enumerate() {
            let pos = arrow.get_position();
            if !self.is_walkable(pos.0, pos.1) {
                indices_to_stop.push(idx);
            }
        }

        // Stop arrows that hit walls
        for idx in indices_to_stop {
            if idx < self.arrows.len() {
                self.arrows[idx].stop();
            }
        }

        // Remove dead arrows
        self.arrows.retain(|arrow| arrow.is_alive());
    }

    pub fn use_ultimate(&mut self) {
        use std::io::Write;

        if !self.character.ultimate.can_use() || !self.should_tick() {
            return;
        }

        self.character.ultimate.start_animation();
        self.character.ultimate.start_cooldown();
        self.consume_tick();

        let msg = format!(
            "\n=== ULTIMATE ABILITY ===\nPosition: ({}, {})\nRadius: {}\n",
            self.character_position.0, self.character_position.1, self.character.ultimate.radius
        );
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")
            .and_then(|mut f| f.write_all(msg.as_bytes()));
    }

    pub fn get_ultimate_area(&self) -> Vec<(i32, i32)> {
        self.character
            .ultimate
            .get_affected_area(self.character_position.0, self.character_position.1)
    }

    pub fn block(&mut self) {
        if !self.character.can_block() || !self.should_tick() {
            return;
        }

        self.character.start_block_cooldown();
        self.consume_tick();

        use std::io::Write;
        let msg = format!(
            "\n=== BLOCK ===\nPosition: ({}, {})\n",
            self.character_position.0, self.character_position.1
        );
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")
            .and_then(|mut f| f.write_all(msg.as_bytes()));
    }

    pub fn switch_weapon(&mut self, slot: usize) {
        self.character.weapon_inventory.switch_weapon(slot - 1);
    }

    pub fn use_current_weapon(&mut self) {
        if let Some(weapon) = self.character.weapon_inventory.get_current_weapon() {
            match weapon.weapon_type {
                crate::model::weapon::WeaponType::Sword => {
                    self.attack();
                }
                crate::model::weapon::WeaponType::Bow => {
                    self.shoot();
                }
                crate::model::weapon::WeaponType::Mace => {
                    // Similar to attack but with mace properties
                    self.attack();
                }
            }
        }
    }

    pub fn pickup_items(&mut self) {
        if let Some(floor) = &mut self.current_floor {
            let (char_x, char_y) = self.character_position;
            let item_count = floor.items_at(char_x, char_y).len();

            for _ in 0..item_count {
                if let Some(item) = floor.pickup_item(char_x, char_y) {
                    // Match on the item type and add to inventory
                    use crate::model::item::ItemDropType;
                    match item.item_type {
                        ItemDropType::Consumable(consumable) => {
                            self.character.consumable_inventory.add(consumable);
                        }
                        ItemDropType::Gold(amount) => {
                            self.character.add_gold(amount);
                        }
                    }
                }
            }
        }
    }

    pub fn use_consumable(&mut self, index: usize) {
        if let Some(consumable) = self.character.consumable_inventory.use_item(index) {
            use crate::model::consumable::ConsumableType;
            use crate::model::status_effect::StatusEffect;

            match consumable.consumable_type {
                ConsumableType::WeakHealingDraught => {
                    // Healing over 5 seconds at 2 HP/sec
                    // For now, heal instantly for simplicity - can add timed healing later
                    self.character.heal(10);
                }
                ConsumableType::BandageRoll => {
                    // Remove bleed and heal 6 HP
                    self.character
                        .status_effects
                        .remove_type(&crate::model::status_effect::StatusEffectType::Bleed);
                    self.character.heal(6);
                }
                ConsumableType::AntitoxinVial => {
                    // Remove poison and grant 5 seconds immunity
                    self.character
                        .status_effects
                        .remove_type(&crate::model::status_effect::StatusEffectType::Poison);
                    self.character
                        .status_effects
                        .add(StatusEffect::poison_immunity(5.0));
                }
                ConsumableType::FireOilFlask => {
                    // Throw fire oil - creates an arrow-like projectile with area damage
                    let (dx, dy) = self.character.last_direction;
                    if dx != 0 || dy != 0 {
                        // Normalize diagonal directions to single tile distance
                        let norm_dx = if dx > 0 {
                            1
                        } else if dx < 0 {
                            -1
                        } else {
                            0
                        };
                        let norm_dy = if dy > 0 {
                            1
                        } else if dy < 0 {
                            -1
                        } else {
                            0
                        };

                        let arrow = Arrow::new_with_type(
                            self.character_position.0 as f32,
                            self.character_position.1 as f32,
                            norm_dx,
                            norm_dy,
                            10.0, // Slightly faster throw than arrows
                            crate::model::arrow::ProjectileType::FireOil,
                        );
                        self.arrows.push(arrow);
                    }
                }
                ConsumableType::BlessedBread => {
                    // Slow healing: 1 HP/sec for 8 seconds
                    // For now, heal over time amount instantly
                    self.character.heal(8);
                }
            }
        }
    }

    pub fn update_camera(&mut self) {
        let vw = self.terminal_size.0 as f32;
        let vh = self.terminal_size.1 as f32;

        let mut target_x = self.character_position.0 as f32 - vw / 2.0;
        let mut target_y = self.character_position.1 as f32 - vh / 2.0;

        if let Some(floor) = &self.current_floor {
            target_x = target_x.clamp(0.0, (floor.width as f32 - vw).max(0.0));
            target_y = target_y.clamp(0.0, (floor.height as f32 - vh).max(0.0));
        }

        self.camera_target = (target_x, target_y);
        // On first frame (when offset is 0,0), immediately sync to target to avoid rendering glitch
        if self.camera_offset.0 == 0.0
            && self.camera_offset.1 == 0.0
            && (target_x != 0.0 || target_y != 0.0)
        {
            self.camera_offset = self.camera_target;
        }
    }

    pub fn update_game_logic(&mut self) {
        // Update status effects
        let delta = (self.game_tick_rate_ms as f32) / 1000.0;
        self.character.status_effects.update(delta);

        // Apply status effect damage
        let damage = (self.character.status_effects.get_total_damage_per_sec() * delta) as i32;
        if damage > 0 {
            self.character.take_damage(damage);
        }

        // Capture values before mutable borrow
        let player_pos = crate::model::enemy::Position::new(
            self.character_position.0,
            self.character_position.1,
        );
        let player_attack_area = self.get_attack_area();
        let mut attacks_on_player: Vec<i32> = Vec::new();
        let mut hit_enemy_indices: Vec<usize> = Vec::new();

        // Update floor items and enemies
        if let Some(floor) = &mut self.current_floor {
            floor.update_items(delta);

            // Get a copy of floor data for pathfinding
            let walkable_tiles: std::collections::HashSet<(i32, i32)> = (0..floor.width as i32)
                .flat_map(|x| (0..floor.height as i32).map(move |y| (x, y)))
                .filter(|(x, y)| floor.is_walkable(*x, *y))
                .collect();

            // First pass: move enemies and collect attacks
            for (enemy_idx, enemy) in floor.enemies.iter_mut().enumerate() {
                if !enemy.is_alive() {
                    continue;
                }

                // Increment movement counter
                enemy.movement_ticks += 1.0;
                // Increment attack counter
                enemy.attack_ticks += 1.0;

                // Simple movement: move toward player if far enough and enough ticks have passed
                let distance = enemy.position.distance_to(&player_pos);
                if distance > 1
                    && enemy.movement_ticks
                        >= crate::constants::ENEMY_MOVEMENT_TICKS_REQUIRED as f32
                {
                    enemy.movement_ticks = 0.0;
                    // Move one step closer to player
                    let dx = (player_pos.x - enemy.position.x).signum();
                    let dy = (player_pos.y - enemy.position.y).signum();

                    let new_x = enemy.position.x + dx;
                    let new_y = enemy.position.y + dy;

                    // Check if the new position is walkable
                    if walkable_tiles.contains(&(new_x, new_y)) {
                        enemy.position.x = new_x;
                        enemy.position.y = new_y;
                    } else {
                        // If directly towards player is blocked, try moving in one dimension
                        if dx != 0
                            && walkable_tiles.contains(&(enemy.position.x + dx, enemy.position.y))
                        {
                            enemy.position.x += dx;
                        } else if dy != 0
                            && walkable_tiles.contains(&(enemy.position.x, enemy.position.y + dy))
                        {
                            enemy.position.y += dy;
                        }
                        // If both dimensions blocked, stay in place (wait for path to clear)
                    }
                }

                // Check if enemy is adjacent to player and can attack
                let distance = enemy.position.distance_to(&player_pos);
                if distance <= 1 && enemy.attack_ticks >= 5.0 {
                    // Enemy attacks player - scale damage by rarity
                    // Attack cooldown: 5 ticks = 80ms (slower than movement)
                    enemy.attack_ticks = 0.0;

                    let rarity_damage = match enemy.rarity {
                        crate::model::enemy_type::EnemyRarity::Fighter => 3,
                        crate::model::enemy_type::EnemyRarity::Guard => 5,
                        crate::model::enemy_type::EnemyRarity::Champion => 8,
                        crate::model::enemy_type::EnemyRarity::Elite => 12,
                        crate::model::enemy_type::EnemyRarity::Boss => 20,
                    };
                    attacks_on_player.push(rarity_damage);
                }

                // Check if player's attack hitbox overlaps this enemy
                if player_attack_area.contains(&(enemy.position.x, enemy.position.y)) {
                    hit_enemy_indices.push(enemy_idx);
                }
            }

            // Apply player attack damage to hit enemies
            for idx in hit_enemy_indices {
                if idx < floor.enemies.len() {
                    let damage = self.character.attack_damage;
                    floor.enemies[idx].take_damage(damage);

                    // Emit particle effect at enemy position
                    self.particle_system.emit_impact(
                        floor.enemies[idx].position.x as f32,
                        floor.enemies[idx].position.y as f32,
                        2,
                        Color::Yellow,
                    );
                }
            }

            // Remove dead enemies
            floor.enemies.retain(|e| e.is_alive());
        }

        // Apply damage outside of borrow
        for attack_damage in attacks_on_player {
            self.character.take_damage(attack_damage);
            self.particle_system.emit_impact(
                self.character_position.0 as f32,
                self.character_position.1 as f32,
                2,
                Color::Red,
            );
        }

        // Update particles
        self.particle_system.update();
    }

    pub fn update_camera_smooth(&mut self) {
        let dx = self.camera_target.0 - self.camera_offset.0;
        let dy = self.camera_target.1 - self.camera_offset.1;

        let ease_factor = 0.05;

        if dx.abs() > 0.1 || dy.abs() > 0.1 {
            self.camera_offset.0 += dx * ease_factor;
            self.camera_offset.1 += dy * ease_factor;
        } else {
            self.camera_offset = self.camera_target;
        }
    }

    #[allow(dead_code)]
    pub fn save_game(&self, slot: u32) -> std::io::Result<()> {
        let save = GameSave {
            player_stats: PlayerStats {
                attack_damage: self.character.attack_damage,
                attack_length: self.character.attack_length,
                attack_width: self.character.attack_width,
                dash_distance: self.character.dash_distance,
                health: 100,
                max_health: 100,
            },
            floor_level: self.floor_level,
            position_x: self.character_position.0,
            position_y: self.character_position.1,
        };
        save.save(slot)
    }

    #[allow(dead_code)]
    pub fn load_game(&mut self, slot: u32) -> std::io::Result<()> {
        let save = GameSave::load(slot)?;
        self.character.attack_damage = save.player_stats.attack_damage;
        self.character.attack_length = save.player_stats.attack_length;
        self.character.attack_width = save.player_stats.attack_width;
        self.character.dash_distance = save.player_stats.dash_distance;
        self.floor_level = save.floor_level;
        self.character_position = (save.position_x, save.position_y);
        Ok(())
    }
}
