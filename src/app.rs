use crate::constants::GAME_TICK_RATE_MS;
use crate::model::arrow::Arrow;
use crate::model::attack_pattern::AnimationFrame; //
use crate::model::audio::{AudioManager, SoundEffect};
use crate::model::character::Character;
use crate::model::floor::Floor;
use crate::model::gamesave::{GameSave, PlayerStats};
use crate::model::particle::ParticleSystem;
use crate::model::pathfinding_cache::PathfindingCache;
use crate::model::settings::Settings;
use crate::model::ultimate_shop::UltimateShop;
use crate::ui::ultimate_shop::UltimateShopUI;
use rand::RngExt;
use ratatui::prelude::Color;
use ratatui::widgets::ListState;
use std::time::Instant;

#[derive(Clone, Copy, PartialEq)]
pub enum AppState {
    MainMenu,
    CharacterCreation,
    Settings,
    Game,
    DevMenu,
    SkillTree,
    UltimateShop,
    DeathScreen,
    VictoryScreen,
}

#[derive(PartialEq)]
pub enum SettingsMode {
    Navigating,
    Rebinding,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PauseSubmenu {
    Volume,
    Settings,
}

/// Attack animation categories for ASCII character filtering
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AnimationCategory {
    CloseCombat,
    RangedCombat,
    Magic,
}

impl AnimationCategory {
    /// Get random ASCII character from the category's character set
    pub fn get_random_character(&self) -> char {
        use rand::{Rng, RngExt};
        let chars = match self {
            AnimationCategory::CloseCombat => {
                vec!['/', '\\', '|', '-', '+', 'X', '*']
            }
            AnimationCategory::RangedCombat => {
                vec!['>', '<', '^', 'v', '→', '←', '↑']
            }
            AnimationCategory::Magic => {
                vec!['~', '§', '¤', '✦', '◆', '●', '#']
            }
        };
        let mut rng = rand::rng();
        chars[rng.random_range(0..chars.len())]
    }
}

/// Represents an animation currently playing on the screen
pub struct ActiveAnimation {
    pub frames: Vec<AnimationFrame>,
    pub current_frame_idx: usize,
    pub timer: f32,
    pub category: AnimationCategory,
}

impl ActiveAnimation {
    pub fn new(frames: Vec<AnimationFrame>) -> Self {
        Self {
            frames,
            current_frame_idx: 0,
            timer: 0.0,
            category: AnimationCategory::CloseCombat,
        }
    }

    pub fn new_with_category(frames: Vec<AnimationFrame>, category: AnimationCategory) -> Self {
        Self {
            frames,
            current_frame_idx: 0,
            timer: 0.0,
            category,
        }
    }

    /// Returns true if animation is finished
    pub fn update(&mut self, dt: f32) -> bool {
        if self.current_frame_idx >= self.frames.len() {
            return true;
        }

        self.timer += dt;

        let current_duration = self.frames[self.current_frame_idx].frame_duration;

        if self.timer >= current_duration {
            self.timer -= current_duration;
            self.current_frame_idx += 1;
        }

        self.current_frame_idx >= self.frames.len()
    }

    pub fn get_current_frame(&self) -> Option<&AnimationFrame> {
        if self.current_frame_idx < self.frames.len() {
            Some(&self.frames[self.current_frame_idx])
        } else {
            None
        }
    }
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
    pub player_has_acted: bool, // Track if player has moved/attacked this level (gates enemy attacks)
    pub arrows: Vec<Arrow>,
    pub inventory_focused: bool,
    pub inventory_scroll_index: usize,
    pub showing_item_description: bool,
    pub is_paused: bool,
    pub particle_system: ParticleSystem,
    pub pathfinding_cache: PathfindingCache,
    pub movement_tick_counter: u32,
    pub char_name: String,
    pub char_name_input_mode: bool,
    pub char_creation_selection: usize,
    pub dev_attack_pattern: crate::model::attack_pattern::AttackPattern,
    pub active_animations: Vec<ActiveAnimation>,
    pub skill_tree_selection: Option<usize>, // For skill tree UI navigation
    pub previous_state: Option<AppState>,    // To track where we came from when opening skill tree
    pub game_started_at: Option<Instant>,    // Track when current run started
    pub death_time_elapsed: f32,             // Time elapsed when death occurred
    pub levels_passed_before_death: u32,     // Levels completed before death
    pub death_screen_fade_timer: f32,        // Fade animation timer
    pub pause_menu_selection: usize,         // Current pause menu item selection
    pub pause_submenu: Option<PauseSubmenu>, // Which submenu is open
    pub pause_volume_selection: usize,       // Which volume is selected (0=music, 1=sound)
    pub pause_settings_state: ListState,     // For settings menu navigation
    pub pause_temp_settings: Settings,       // Temporary settings copy for pause menu
    pub pause_rebinding_mode: SettingsMode,  // Rebinding mode for pause settings
    pub music_volume: f32,                   // Music volume 0.0 - 1.0
    pub sound_volume: f32,                   // Sound effects volume 0.0 - 1.0
    pub audio_manager: AudioManager,         // Audio playback manager
    pub max_levels: u32,                     // Maximum levels before boss based on difficulty
    pub is_boss_level: bool,                 // Whether current level is a boss fight
    pub victory_win_time: f32,               // Time elapsed when victory occurred
    pub last_weapon_pickup: Option<(String, crate::model::item_rarity::ItemRarity)>, // Weapon name and rarity
    pub weapon_pickup_timer: f32, // Timer for weapon pickup notification display
    pub ultimate_shop: UltimateShop, // The shop system for ultimates and upgrades
    pub ultimate_shop_ui: UltimateShopUI, // UI state for the ultimate shop
}

impl App {
    pub fn new() -> Self {
        let s = Settings::load();
        let mut menu_s = ListState::default();
        menu_s.select(Some(0));
        let mut set_s = ListState::default();
        set_s.select(Some(0));
        let mut pause_s = ListState::default();
        pause_s.select(Some(0));

        let now = Instant::now();
        let mut audio_mgr = AudioManager::new();

        // Start music with fade-in on app startup
        let _ = audio_mgr.start_music_with_fade_in();

        Self {
            state: AppState::MainMenu,
            settings: s.clone(),
            temp_settings: s.clone(),
            settings_mode: SettingsMode::Navigating,
            main_menu_state: menu_s,
            settings_state: set_s,
            should_quit: false,
            scroll_offset: 0,
            is_auto_scrolling: !s.skip_logo_animation,
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
            player_has_acted: false,
            arrows: Vec::new(),
            inventory_focused: false,
            inventory_scroll_index: 0,
            showing_item_description: false,
            is_paused: false,
            particle_system: ParticleSystem::new(),
            pathfinding_cache: PathfindingCache::new(500),
            char_name: String::new(),
            char_name_input_mode: false,
            char_creation_selection: 0,
            movement_tick_counter: 0,
            dev_attack_pattern: crate::model::attack_pattern::AttackPattern::BasicSlash,
            active_animations: Vec::new(),
            skill_tree_selection: Some(0), // Initialize for skill tree UI
            previous_state: None,          // No previous state initially
            game_started_at: None,
            death_time_elapsed: 0.0,
            levels_passed_before_death: 0,
            death_screen_fade_timer: 0.0,
            pause_menu_selection: 0,
            pause_submenu: None,
            pause_volume_selection: 0,
            pause_settings_state: pause_s,
            pause_temp_settings: s.clone(),
            pause_rebinding_mode: SettingsMode::Navigating,
            music_volume: 0.5,
            sound_volume: 0.5,
            audio_manager: audio_mgr,
            max_levels: 5, // Default, will be updated when game starts
            is_boss_level: false,
            victory_win_time: 0.0,
            last_weapon_pickup: None,
            weapon_pickup_timer: 0.0,
            ultimate_shop: UltimateShop::new(),
            ultimate_shop_ui: UltimateShopUI::new(),
        }
    }

    /// Get the maximum number of levels before boss based on difficulty
    pub fn get_max_levels_for_difficulty(&self) -> u32 {
        use crate::model::item_tier::Difficulty;
        match &self.settings.difficulty {
            Difficulty::Easy => 5,
            Difficulty::Normal => 10,
            Difficulty::Hard => 15,
            Difficulty::Death => 20,
        }
    }

    /// Check if the current floor level is the boss level
    pub fn is_current_level_boss(&self) -> bool {
        self.floor_level == self.max_levels
    }

    /// Get difficulty scaling multiplier for enemy health and damage
    pub fn get_enemy_difficulty_multiplier(&self) -> f32 {
        use crate::model::item_tier::Difficulty;
        match &self.settings.difficulty {
            Difficulty::Easy => 0.8,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 1.4,
            Difficulty::Death => 2.0,
        }
    }

    /// Classify an attack pattern into a category for ASCII filter rendering
    pub fn get_attack_pattern_category(
        pattern: &crate::model::attack_pattern::AttackPattern,
    ) -> AnimationCategory {
        use crate::model::attack_pattern::AttackPattern;
        match pattern {
            // Close combat patterns
            AttackPattern::BasicSlash
            | AttackPattern::GroundSlam(_)
            | AttackPattern::WhirlwindAttack
            | AttackPattern::SwordThrust(_)
            | AttackPattern::CrescentSlash => AnimationCategory::CloseCombat,

            // Ranged combat patterns
            AttackPattern::ArrowShot(_)
            | AttackPattern::MultiShot(_, _)
            | AttackPattern::Barrage(_)
            | AttackPattern::PiercingShot(_) => AnimationCategory::RangedCombat,

            // Magic patterns
            AttackPattern::Fireball(_)
            | AttackPattern::ChainLightning(_)
            | AttackPattern::FrostNova(_)
            | AttackPattern::MeteorShower(_, _)
            | AttackPattern::Vortex(_) => AnimationCategory::Magic,
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
        let speed_factor = crate::constants::LOGO_ANIMATION_SPEED;

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

        let difficulty = self.settings.difficulty.clone();
        floor.spawn_random_items(10, &difficulty);

        // Check if this is a boss level
        if self.is_current_level_boss() {
            // Spawn a boss instead of regular enemies
            use crate::model::boss::BossType;
            use rand::Rng;

            let boss_types = [
                BossType::GoblinOverlord,
                BossType::SkeletalKnight,
                BossType::FlameSorcerer,
                BossType::ShadowAssassin,
                BossType::CorruptedWarden,
            ];

            let random_boss = boss_types[rand::rng().random_range(0..boss_types.len())];
            floor.spawn_boss(random_boss);
            self.is_boss_level = true;
        } else {
            // Regular floor with normal enemies
            floor.spawn_enemies(&difficulty);
            self.is_boss_level = false;
        }

        self.current_floor = Some(floor);
        self.player_has_acted = false; // Reset action state for new level

        if let Some(floor) = &self.current_floor {
            if let Some((x, y)) = floor.find_player_spawn() {
                self.character_position = (x, y);
                self.update_camera();
            }
        }
    }

    pub fn roll_random_seed(&mut self) {
        use rand::{Rng, RngExt};
        let new_seed: u64 = rand::rng().random_range(0..=u64::MAX);
        self.dev_seed_input = new_seed.to_string();
        self.regenerate_floor();
    }

    pub fn update_terminal_size(&mut self, width: u16, height: u16) {
        self.terminal_size = (width, height);
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if let Some(floor) = &self.current_floor {
            // Hard boundary check - prevent any movement out of bounds
            if x < 0 || x >= floor.width || y < 0 || y >= floor.height {
                return false;
            }

            // Check if floor tile is walkable
            if !floor.is_walkable(x, y) {
                return false;
            }

            // Check if an enemy is at this position
            for enemy in &floor.enemies {
                if enemy.is_alive() && enemy.position.x == x && enemy.position.y == y {
                    return false; // Can't walk into enemy
                }
            }
            true
        } else {
            true
        }
    }

    /// Check if the player is dead and transition to death screen if necessary
    pub fn check_and_handle_death(&mut self) {
        if self.character.health <= 0 && self.state == AppState::Game {
            // Record death stats
            self.death_time_elapsed = if let Some(started_at) = self.game_started_at {
                started_at.elapsed().as_secs_f32()
            } else {
                0.0
            };
            self.levels_passed_before_death = self.floor_level.saturating_sub(1);

            // Reset fade timer for death screen animation
            self.death_screen_fade_timer = 0.0;

            // Play death sound and apply music muffling
            self.audio_manager.play_sound_effect(SoundEffect::Death);
            self.audio_manager.start_death_fade_out(1.0); // 1 second muffling fade

            // Transition to death screen
            self.state = AppState::DeathScreen;
        }
    }

    /// Restart the game with a fresh character and new floor
    pub fn restart_game(&mut self) {
        // Reset character
        self.character = Character::default();
        self.character_position = (0, 0);

        // Reset game state
        self.floor_level = 1;
        self.player_has_acted = false; // Reset action state for new level
        self.current_floor = None;
        self.dev_seed_input = String::new();
        self.arrows.clear();
        self.active_animations.clear();
        self.particle_system = ParticleSystem::new();
        self.is_paused = false;

        // Set max levels based on selected difficulty
        self.max_levels = self.get_max_levels_for_difficulty();
        self.is_boss_level = false;

        // Reset stats
        self.game_started_at = Some(Instant::now());
        self.death_time_elapsed = 0.0;
        self.levels_passed_before_death = 0;
        self.death_screen_fade_timer = 0.0;
        self.victory_win_time = 0.0;

        // Reset pause menu state
        self.pause_menu_selection = 0;
        self.pause_submenu = None;

        // Generate new floor
        self.regenerate_floor();

        // Restart music with fade-in for new run
        let _ = self.audio_manager.start_music_with_fade_in();

        // Transition to game state
        self.state = AppState::Game;
    }

    pub fn should_tick(&self) -> bool {
        !self.is_paused && self.last_game_tick.elapsed().as_millis() >= self.game_tick_rate_ms
    }

    pub fn consume_tick(&mut self) {
        self.last_game_tick = Instant::now();
    }

    pub fn move_character(&mut self, dx: i32, dy: i32) {
        // Prevent movement while attacking to avoid animation desync
        if self.character.is_attack_animating() {
            return;
        }

        self.movement_tick_counter += 1;

        let speed_adjusted_requirement = (crate::constants::PLAYER_MOVEMENT_TICKS_REQUIRED as f32
            / self.settings.player_speed)
            .ceil() as u32;

        if self.movement_tick_counter < speed_adjusted_requirement {
            return;
        }

        self.movement_tick_counter = 0;

        let new_x = self.character_position.0 + dx;
        let new_y = self.character_position.1 + dy;

        // Update direction regardless of movement success (allows turning in 1x1 blocks)
        if dx != 0 || dy != 0 {
            self.character.update_direction(dx, dy);
        }

        if self.is_walkable(new_x, new_y) {
            self.character_position = (new_x, new_y);
            self.pickup_items();
            self.update_camera();
            self.player_has_acted = true; // Player has moved - enable enemy attacks
            self.consume_tick();
        }
    }

    pub fn dash(&mut self) {
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

        if self.is_walkable(new_x, new_y) {
            self.character_position = (new_x, new_y);
            self.character.start_dash_cooldown();
            self.update_camera();
            self.consume_tick();
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

        // --- NEW: Generate and store the animation ---
        // We use the current dev_attack_pattern, but in a real game you might switch based on weapon
        let frames = self.dev_attack_pattern.get_animation_frames(
            self.character_position.0,
            self.character_position.1,
            attack_dx,
            attack_dy,
        );

        if !frames.is_empty() {
            let category = Self::get_attack_pattern_category(&self.dev_attack_pattern);
            self.active_animations
                .push(ActiveAnimation::new_with_category(frames, category));
        }
        // ---------------------------------------------

        // Start the attack cooldown
        self.character.start_attack_cooldown();
        self.player_has_acted = true; // Player has attacked - enable enemy attacks
        self.consume_tick();
    }

    // Helper to visualize patterns in dev menu without cooldowns/movement
    pub fn trigger_dev_animation(&mut self) {
        let (dx, dy) = self.character.last_direction;
        let (attack_dx, attack_dy) = if dx == 0 && dy == 0 { (0, 1) } else { (dx, dy) };

        let frames = self.dev_attack_pattern.get_animation_frames(
            self.character_position.0,
            self.character_position.1,
            attack_dx,
            attack_dy,
        );

        if !frames.is_empty() {
            let category = Self::get_attack_pattern_category(&self.dev_attack_pattern);
            self.active_animations
                .push(ActiveAnimation::new_with_category(frames, category));
        }
    }

    pub fn get_attack_area(&self) -> Vec<(i32, i32)> {
        let (dx, dy) = self.character.last_direction;
        let (attack_dx, attack_dy) = if dx == 0 && dy == 0 { (0, 1) } else { (dx, dy) };

        self.dev_attack_pattern.get_affected_tiles(
            self.character_position.0,
            self.character_position.1,
            attack_dx,
            attack_dy,
        )
    }

    /// Get the attack area based on the current animation frame
    /// This ensures hitboxes match what's visually displayed
    pub fn get_current_attack_area(&self) -> Vec<(i32, i32)> {
        if let Some(attack_time) = self.character.last_attack_time {
            let elapsed = attack_time.elapsed().as_secs_f32();

            // Get all frames for current attack
            let (dx, dy) = self.character.last_direction;
            let (attack_dx, attack_dy) = if dx == 0 && dy == 0 { (0, 1) } else { (dx, dy) };
            let frames = self.dev_attack_pattern.get_animation_frames(
                self.character_position.0,
                self.character_position.1,
                attack_dx,
                attack_dy,
            );

            // Find which frame should be displayed based on elapsed time
            let mut accumulated_time = 0.0;
            for frame in &frames {
                accumulated_time += frame.frame_duration;
                if elapsed < accumulated_time {
                    return frame.tiles.clone();
                }
            }

            // If we've gone past all frames, return the last frame's tiles
            frames.last().map(|f| f.tiles.clone()).unwrap_or_default()
        } else {
            vec![]
        }
    }

    pub fn shoot(&mut self) {
        if !self.character.can_shoot() || !self.should_tick() {
            return;
        }

        let (dx, dy) = self.character.last_direction;
        let (shoot_dx, shoot_dy) = if dx == 0 && dy == 0 { (0, 1) } else { (dx, dy) };

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
    }

    pub fn update_arrows(&mut self) {
        let frame_time = (self.game_tick_rate_ms as f32) / 1000.0;

        for arrow in self.arrows.iter_mut() {
            arrow.update(frame_time);
        }

        let mut indices_to_stop = Vec::new();
        for (idx, arrow) in self.arrows.iter().enumerate() {
            let pos = arrow.get_position();
            if !self.is_walkable(pos.0, pos.1) {
                indices_to_stop.push(idx);
            }
        }

        for idx in indices_to_stop {
            if idx < self.arrows.len() {
                self.arrows[idx].stop();
            }
        }

        self.arrows.retain(|arrow| arrow.is_alive());
    }

    pub fn use_ultimate(&mut self) {
        if !self.character.ultimate.can_use() || !self.should_tick() {
            return;
        }

        self.character.ultimate.start_animation();
        self.character.ultimate.start_cooldown();
        self.consume_tick();
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
    }

    pub fn switch_weapon(&mut self, slot: usize) {
        self.character.weapon_inventory.switch_weapon(slot - 1);
        self.audio_manager.play_sound_effect(SoundEffect::ItemEquip);
    }

    pub fn drop_weapon(&mut self, slot: usize) {
        if slot == 0 || slot > 9 {
            return;
        }

        if let Some(weapon) = self.character.weapon_inventory.remove_weapon(slot - 1) {
            if let Some(floor) = &mut self.current_floor {
                let (char_x, char_y) = self.character_position;
                let weapon_item =
                    crate::model::item::ItemDrop::weapon(weapon.clone(), char_x, char_y);
                if !floor.try_drop_item_adjacent(weapon_item, char_x, char_y) {
                    self.character
                        .weapon_inventory
                        .weapons
                        .insert(slot - 1, weapon);
                }
            }
        }
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
                    self.attack();
                }
                crate::model::weapon::WeaponType::Spear => {
                    self.attack();
                }
                crate::model::weapon::WeaponType::Axe => {
                    self.attack();
                }
                crate::model::weapon::WeaponType::Staff => {
                    self.shoot();
                }
            }
        }
    }

    pub fn cycle_dev_attack_pattern(&mut self) {
        use crate::model::attack_pattern::AttackPattern;

        let patterns = vec![
            AttackPattern::BasicSlash,
            AttackPattern::GroundSlam(3),
            AttackPattern::WhirlwindAttack,
            AttackPattern::SwordThrust(2),
            AttackPattern::ArrowShot(4),
            AttackPattern::MultiShot(4, 2),
            AttackPattern::Barrage(3),
            AttackPattern::PiercingShot(5),
            AttackPattern::Fireball(2),
            AttackPattern::ChainLightning(3),
            AttackPattern::FrostNova(2),
            AttackPattern::MeteorShower(3, 2),
            AttackPattern::CrescentSlash,
            AttackPattern::Vortex(2),
        ];

        let current_index = patterns
            .iter()
            .position(|p| p == &self.dev_attack_pattern)
            .unwrap_or(0);
        let next_index = (current_index + 1) % patterns.len();
        self.dev_attack_pattern = patterns[next_index].clone();

        // Trigger a visual test immediately upon switching
        self.trigger_dev_animation();
    }

    pub fn pickup_items(&mut self) {
        if let Some(floor) = &mut self.current_floor {
            let (char_x, char_y) = self.character_position;
            let item_count = floor.items_at(char_x, char_y).len();

            for _ in 0..item_count {
                if let Some(item) = floor.pickup_item(char_x, char_y) {
                    use crate::model::item::ItemDropType;
                    match item.item_type {
                        ItemDropType::Consumable(consumable) => {
                            self.character.consumable_inventory.add(consumable);
                            self.audio_manager
                                .play_sound_effect(SoundEffect::PickedUpItem);
                        }
                        ItemDropType::Gold(amount) => {
                            self.character.add_gold(amount);
                            self.audio_manager.play_gold_sound();
                        }
                        ItemDropType::Weapon(weapon) => {
                            if self.character.weapon_inventory.weapons.len() < 9 {
                                // Store weapon pickup notification
                                let weapon_name = format!("{:?}", weapon.weapon_type);
                                self.last_weapon_pickup =
                                    Some((weapon_name, weapon.rarity.clone()));
                                self.weapon_pickup_timer = 3.0; // Show for 3 seconds

                                self.character.weapon_inventory.add_weapon(weapon);
                                self.audio_manager
                                    .play_sound_effect(SoundEffect::PickedUpItem);
                            } else {
                                let weapon_item =
                                    crate::model::item::ItemDrop::weapon(weapon, char_x, char_y);
                                let _ = floor.try_drop_item_adjacent(weapon_item, char_x, char_y);
                            }
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
                    self.character.heal(10);
                }
                ConsumableType::BandageRoll => {
                    self.character
                        .status_effects
                        .remove_type(&crate::model::status_effect::StatusEffectType::Bleed);
                    self.character.heal(6);
                }
                ConsumableType::AntitoxinVial => {
                    self.character
                        .status_effects
                        .remove_type(&crate::model::status_effect::StatusEffectType::Poison);
                    self.character
                        .status_effects
                        .add(StatusEffect::poison_immunity(5.0));
                }
                ConsumableType::FireOilFlask => {
                    let (dx, dy) = self.character.last_direction;
                    if dx != 0 || dy != 0 {
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
                            10.0,
                            crate::model::arrow::ProjectileType::FireOil,
                        );
                        self.arrows.push(arrow);
                    }
                }
                ConsumableType::BlessedBread => {
                    self.character.heal(8);
                }
            }
        }
    }

    pub fn update_camera(&mut self) {
        let vw = self.terminal_size.0 as f32;
        // Subtract 2 for the ultimate bar at the bottom (UI reserved space)
        let vh = (self.terminal_size.1 as f32 - 2.0).max(1.0);

        let mut target_x = self.character_position.0 as f32 - vw / 2.0;
        let mut target_y = self.character_position.1 as f32 - vh / 2.0;

        if let Some(floor) = &self.current_floor {
            target_x = target_x.clamp(0.0, (floor.width as f32 - vw).max(0.0));
            target_y = target_y.clamp(0.0, (floor.height as f32 - vh).max(0.0));
        }

        self.camera_target = (target_x, target_y);
        if self.camera_offset.0 == 0.0
            && self.camera_offset.1 == 0.0
            && (target_x != 0.0 || target_y != 0.0)
        {
            self.camera_offset = self.camera_target;
        }
    }

    pub fn update_game_logic(&mut self) {
        let delta = (self.game_tick_rate_ms as f32) / 1000.0;

        // Update weapon pickup notification timer
        if self.weapon_pickup_timer > 0.0 {
            self.weapon_pickup_timer -= delta;
            if self.weapon_pickup_timer <= 0.0 {
                self.last_weapon_pickup = None;
            }
        }

        self.character.status_effects.update(delta);

        // --- NEW: Update active animations ---
        // Iterate backwards to safely remove finished animations
        let mut i = 0;
        while i < self.active_animations.len() {
            let finished = self.active_animations[i].update(delta);
            if finished {
                self.active_animations.swap_remove(i);
            } else {
                i += 1;
            }
        }
        // -------------------------------------

        let damage = (self.character.status_effects.get_total_damage_per_sec() * delta) as i32;
        if damage > 0 {
            self.character.take_damage(damage);
            self.audio_manager.play_damaged_sound();
        }

        if self.character.knockback_velocity != (0.0, 0.0) {
            if let Some(floor) = &self.current_floor {
                let (kb_x, kb_y) = self.character.knockback_velocity;
                let new_x = (self.character_position.0 as f32 + kb_x).round() as i32;
                let new_y = (self.character_position.1 as f32 + kb_y).round() as i32;

                // Clamp to map boundaries
                let clamped_x = new_x.clamp(0, floor.width as i32 - 1);
                let clamped_y = new_y.clamp(0, floor.height as i32 - 1);

                // Try to move to clamped position if it's walkable
                if self.is_walkable(clamped_x, clamped_y) {
                    self.character_position = (clamped_x, clamped_y);
                    // If knocked out of bounds, zero out knockback in that direction
                    if clamped_x != new_x {
                        self.character.knockback_velocity.0 = 0.0;
                    }
                    if clamped_y != new_y {
                        self.character.knockback_velocity.1 = 0.0;
                    }
                } else {
                    // Try moving along each axis separately (respect directions)
                    let can_move_x = self.is_walkable(clamped_x, self.character_position.1);
                    let can_move_y = self.is_walkable(self.character_position.0, clamped_y);

                    if can_move_x {
                        self.character_position.0 = clamped_x;
                        if clamped_x != new_x {
                            self.character.knockback_velocity.0 = 0.0;
                        }
                    } else {
                        self.character.knockback_velocity.0 = 0.0;
                    }

                    if can_move_y {
                        self.character_position.1 = clamped_y;
                        if clamped_y != new_y {
                            self.character.knockback_velocity.1 = 0.0;
                        }
                    } else {
                        self.character.knockback_velocity.1 = 0.0;
                    }
                }

                self.character.knockback_velocity.0 *= 0.7;
                self.character.knockback_velocity.1 *= 0.7;
                if self.character.knockback_velocity.0.abs() < 0.1
                    && self.character.knockback_velocity.1.abs() < 0.1
                {
                    self.character.knockback_velocity = (0.0, 0.0);
                }
            }
        }

        let player_pos = crate::model::enemy::Position::new(
            self.character_position.0,
            self.character_position.1,
        );
        let player_pos_f32 = (
            self.character_position.0 as f32,
            self.character_position.1 as f32,
        );
        let mut attacks_on_player: Vec<(i32, f32, f32)> = Vec::new();
        let mut hit_enemy_indices: Vec<usize> = Vec::new();

        // Get current attack area before borrowing floor mutably
        let current_attack_area = if self.character.is_attacking_animating() {
            self.get_current_attack_area()
        } else {
            vec![]
        };

        if let Some(floor) = &mut self.current_floor {
            floor.update_items(delta);

            for item in &floor.items {
                if matches!(item.item_type, crate::model::item::ItemDropType::Weapon(_)) {
                    self.particle_system.emit_periodic_glint(
                        item.x as f32,
                        item.y as f32,
                        Color::Yellow,
                    );
                }
            }

            let walkable_tiles: std::collections::HashSet<(i32, i32)> = (0..floor.width)
                .flat_map(|x| (0..floor.height).map(move |y| (x, y)))
                .filter(|(x, y)| floor.is_walkable(*x, *y))
                .collect();

            for (enemy_idx, enemy) in floor.enemies.iter_mut().enumerate() {
                if !enemy.is_alive() {
                    continue;
                }

                if enemy.knockback_velocity != (0.0, 0.0) {
                    let (kb_x, kb_y) = enemy.knockback_velocity;
                    let new_x = (enemy.position.x as f32 + kb_x).round() as i32;
                    let new_y = (enemy.position.y as f32 + kb_y).round() as i32;

                    // Clamp to map boundaries
                    let clamped_x = new_x.clamp(0, floor.width as i32 - 1);
                    let clamped_y = new_y.clamp(0, floor.height as i32 - 1);

                    // Try to move to clamped position if it's walkable and not the player's position
                    if (clamped_x, clamped_y) != (player_pos.x, player_pos.y)
                        && walkable_tiles.contains(&(clamped_x, clamped_y))
                    {
                        enemy.position.x = clamped_x;
                        enemy.position.y = clamped_y;
                        // If knocked out of bounds, zero out knockback in that direction
                        if clamped_x != new_x {
                            enemy.knockback_velocity.0 = 0.0;
                        }
                        if clamped_y != new_y {
                            enemy.knockback_velocity.1 = 0.0;
                        }
                    } else {
                        // Try moving along each axis separately (respect directions), avoiding player
                        let can_move_x = (clamped_x, enemy.position.y)
                            != (player_pos.x, player_pos.y)
                            && walkable_tiles.contains(&(clamped_x, enemy.position.y));
                        let can_move_y = (enemy.position.x, clamped_y)
                            != (player_pos.x, player_pos.y)
                            && walkable_tiles.contains(&(enemy.position.x, clamped_y));

                        if can_move_x {
                            enemy.position.x = clamped_x;
                            if clamped_x != new_x {
                                enemy.knockback_velocity.0 = 0.0;
                            }
                        } else {
                            enemy.knockback_velocity.0 = 0.0;
                        }

                        if can_move_y {
                            enemy.position.y = clamped_y;
                            if clamped_y != new_y {
                                enemy.knockback_velocity.1 = 0.0;
                            }
                        } else {
                            enemy.knockback_velocity.1 = 0.0;
                        }
                    }

                    enemy.knockback_velocity.0 *= 0.7;
                    enemy.knockback_velocity.1 *= 0.7;
                    if enemy.knockback_velocity.0.abs() < 0.1
                        && enemy.knockback_velocity.1.abs() < 0.1
                    {
                        enemy.knockback_velocity = (0.0, 0.0);
                    }
                }

                if matches!(enemy.rarity, crate::model::enemy_type::EnemyRarity::Boss) {
                    self.particle_system.emit_periodic_glint(
                        enemy.position.x as f32,
                        enemy.position.y as f32,
                        Color::Red,
                    );
                }

                enemy.movement_ticks += enemy.speed; // Use enemy's speed for movement
                enemy.attack_ticks += 1.0;

                let distance = enemy.position.distance_to(&player_pos);
                if distance > 1 && distance <= enemy.detection_radius && enemy.movement_ticks >= 1.0
                // Use 1.0 threshold for consistency with speed
                {
                    enemy.movement_ticks -= 1.0; // Deduct movement cost
                    let dx = (player_pos.x - enemy.position.x).signum();
                    let dy = (player_pos.y - enemy.position.y).signum();

                    let new_x = enemy.position.x + dx;
                    let new_y = enemy.position.y + dy;

                    // Check if target position is walkable AND not occupied by player
                    if (new_x, new_y) != (player_pos.x, player_pos.y)
                        && walkable_tiles.contains(&(new_x, new_y))
                    {
                        enemy.position.x = new_x;
                        enemy.position.y = new_y;
                    } else {
                        // Try moving along each axis separately, but still avoid player
                        if dx != 0
                            && (enemy.position.x + dx, enemy.position.y)
                                != (player_pos.x, player_pos.y)
                            && walkable_tiles.contains(&(enemy.position.x + dx, enemy.position.y))
                        {
                            enemy.position.x += dx;
                        } else if dy != 0
                            && (enemy.position.x, enemy.position.y + dy)
                                != (player_pos.x, player_pos.y)
                            && walkable_tiles.contains(&(enemy.position.x, enemy.position.y + dy))
                        {
                            enemy.position.y += dy;
                        }
                    }
                } else if distance > enemy.detection_radius {
                    // Enemy is out of detection range - will wander in separate pass below
                }

                let distance = enemy.position.distance_to(&player_pos);
                if distance <= 1 && enemy.attack_ticks >= 65.0 && self.player_has_acted {
                    enemy.attack_ticks = 0.0;

                    let rarity_damage = match enemy.rarity {
                        crate::model::enemy_type::EnemyRarity::Fighter => 3,
                        crate::model::enemy_type::EnemyRarity::Guard => 5,
                        crate::model::enemy_type::EnemyRarity::Champion => 8,
                        crate::model::enemy_type::EnemyRarity::Elite => 12,
                        crate::model::enemy_type::EnemyRarity::Boss => 20,
                    };

                    let enemy_pos_f32 = (enemy.position.x as f32, enemy.position.y as f32);
                    let diff_x = player_pos_f32.0 - enemy_pos_f32.0;
                    let diff_y = player_pos_f32.1 - enemy_pos_f32.1;
                    let distance_sq = diff_x * diff_x + diff_y * diff_y;

                    // Normalize direction to avoid diagonal bias in knockback
                    let (dx, dy) = if distance_sq > 0.0 {
                        let distance_f = distance_sq.sqrt();
                        (diff_x / distance_f, diff_y / distance_f)
                    } else {
                        (0.0, 1.0)
                    };

                    // Generate attack animation for enemy
                    let attack_pattern = match enemy.rarity {
                        crate::model::enemy_type::EnemyRarity::Fighter => {
                            crate::model::attack_pattern::AttackPattern::BasicSlash
                        }
                        crate::model::enemy_type::EnemyRarity::Guard => {
                            crate::model::attack_pattern::AttackPattern::BasicSlash
                        }
                        crate::model::enemy_type::EnemyRarity::Champion => {
                            crate::model::attack_pattern::AttackPattern::WhirlwindAttack
                        }
                        crate::model::enemy_type::EnemyRarity::Elite => {
                            crate::model::attack_pattern::AttackPattern::GroundSlam(1)
                        }
                        crate::model::enemy_type::EnemyRarity::Boss => {
                            match enemy.attack_pattern_cycle {
                                0 => crate::model::attack_pattern::AttackPattern::Fireball(3),
                                1 => crate::model::attack_pattern::AttackPattern::GroundSlam(2),
                                2 => crate::model::attack_pattern::AttackPattern::WhirlwindAttack,
                                _ => crate::model::attack_pattern::AttackPattern::BasicSlash,
                            }
                        }
                    };

                    let attack_dir_x = (dx.signum()) as i32;
                    let attack_dir_y = (dy.signum()) as i32;
                    let frames = attack_pattern.get_animation_frames(
                        enemy.position.x,
                        enemy.position.y,
                        attack_dir_x,
                        attack_dir_y,
                    );

                    if !frames.is_empty() {
                        let category = Self::get_attack_pattern_category(&attack_pattern);
                        self.active_animations
                            .push(ActiveAnimation::new_with_category(frames, category));
                    }

                    attacks_on_player.push((rarity_damage, dx, dy));

                    // Cycle attack pattern for bosses
                    if matches!(enemy.rarity, crate::model::enemy_type::EnemyRarity::Boss) {
                        enemy.attack_pattern_cycle = (enemy.attack_pattern_cycle + 1) % 3;
                    }
                }

                // Only register hits if player is in attack animation
                if !current_attack_area.is_empty()
                    && current_attack_area.contains(&(enemy.position.x, enemy.position.y))
                {
                    hit_enemy_indices.push(enemy_idx);
                }
            }

            // Second pass: handle wandering for enemies out of detection range
            for enemy in &mut floor.enemies {
                if !enemy.is_alive() {
                    continue;
                }
                let distance = enemy.position.distance_to(&player_pos);
                if distance > enemy.detection_radius && enemy.movement_ticks >= 1.0 {
                    // Inline wander behavior (single random direction instead of A*)
                    enemy.movement_ticks -= 1.0; // Deduct movement cost based on speed
                    enemy.is_wandering = true;

                    let directions = [
                        (0, -1), // up
                        (0, 1),  // down
                        (-1, 0), // left
                        (1, 0),  // right
                    ];

                    use rand::seq::SliceRandom;
                    let mut rng = rand::rng();
                    let mut shuffled = directions.to_vec();
                    shuffled.shuffle(&mut rng);

                    for (dx, dy) in shuffled {
                        let new_x = enemy.position.x + dx;
                        let new_y = enemy.position.y + dy;

                        // Prevent wandering into player's position
                        if (new_x, new_y) == (player_pos.x, player_pos.y) {
                            continue;
                        }

                        if new_x >= 0
                            && new_x < floor.width as i32
                            && new_y >= 0
                            && new_y < floor.height as i32
                            && walkable_tiles.contains(&(new_x, new_y))
                        {
                            enemy.position.x = new_x;
                            enemy.position.y = new_y;
                            break;
                        }
                    }
                }
            }

            let _player_pos = (
                self.character_position.0 as f32,
                self.character_position.1 as f32,
            );

            for idx in hit_enemy_indices {
                if idx < floor.enemies.len() {
                    let damage = self.character.get_effective_attack_damage();
                    let knockback_force = 1.0; // Exactly 1 block knockback per hit

                    // Use player's facing direction for knockback, not direction to enemy
                    let (player_dir_x, player_dir_y) = self.character.last_direction;
                    let (dx, dy) = if player_dir_x == 0 && player_dir_y == 0 {
                        (0.0, 1.0) // default direction if no direction set
                    } else {
                        (player_dir_x as f32, player_dir_y as f32)
                    };

                    floor.enemies[idx].apply_knockback(dx, dy, knockback_force);
                    floor.enemies[idx].take_damage(damage);
                    // Charge player's ultimate based on damage dealt
                    self.character.charge_ultimate(damage);
                    // Play hit sound when enemy is damaged
                    self.audio_manager.play_sound_effect(SoundEffect::Hit);
                }
            }

            let mut dead_enemies = Vec::new();
            for (idx, enemy) in floor.enemies.iter().enumerate() {
                if !enemy.is_alive() {
                    dead_enemies.push(idx);
                }
            }

            for idx in dead_enemies.iter().rev() {
                let enemy = floor.enemies.remove(*idx);
                // Play death sound when enemy is killed
                self.audio_manager.play_sound_effect(SoundEffect::Death);
                // Increment kill counter
                self.character.enemies_killed += 1;

                let enemy_x = enemy.position.x;
                let enemy_y = enemy.position.y;

                // Always drop gold - guaranteed success
                let gold_drop = enemy.base_gold;
                let mut gold_item = crate::model::item::ItemDrop::gold(gold_drop, enemy_x, enemy_y);

                // Try to drop at adjacent position first, otherwise place at enemy location
                if !floor.try_drop_item_adjacent(gold_item, enemy_x, enemy_y) {
                    gold_item = crate::model::item::ItemDrop::gold(gold_drop, enemy_x, enemy_y);
                    floor.add_item(gold_item);
                }

                // Weapon drops with 33% chance
                if rand::random::<f32>() < 0.33 {
                    // Determine weapon rarity based on difficulty
                    use crate::model::item_tier::{Difficulty, ItemTier};
                    use rand::prelude::IndexedRandom;

                    let rarity = match self.settings.difficulty {
                        Difficulty::Easy => {
                            let rarities = vec![ItemTier::Common, ItemTier::Rare];
                            rarities.choose(&mut rand::rng()).unwrap().clone()
                        }
                        Difficulty::Normal => {
                            let rarities = vec![ItemTier::Rare, ItemTier::Epic];
                            rarities.choose(&mut rand::rng()).unwrap().clone()
                        }
                        Difficulty::Hard => {
                            let rarities = vec![ItemTier::Epic, ItemTier::Exotic];
                            rarities.choose(&mut rand::rng()).unwrap().clone()
                        }
                        Difficulty::Death => {
                            let rarities =
                                vec![ItemTier::Exotic, ItemTier::Legendary, ItemTier::Mythic];
                            rarities.choose(&mut rand::rng()).unwrap().clone()
                        }
                    };

                    let weapon = crate::model::weapon::Weapon::random_for_rarity(&rarity);
                    let mut weapon_drop =
                        crate::model::item::ItemDrop::weapon(weapon.clone(), enemy_x, enemy_y);

                    if !floor.try_drop_item_adjacent(weapon_drop, enemy_x, enemy_y) {
                        weapon_drop =
                            crate::model::item::ItemDrop::weapon(weapon, enemy_x, enemy_y);
                        floor.add_item(weapon_drop);
                    }
                }
            }
        }

        for (attack_damage, dx, dy) in attacks_on_player {
            self.character.apply_knockback(dx, dy, 0.5);
            self.character.take_damage(attack_damage);
            // Player gains ultimate charge when damaged by enemies
            self.character.charge_ultimate(attack_damage);
            self.audio_manager.play_damaged_sound();
        }

        self.particle_system.update();

        // Check for player death
        self.check_and_handle_death();

        // Check if all enemies on current floor are defeated - advance to next floor
        if let Some(floor) = &self.current_floor {
            if floor.enemies.is_empty() && self.player_has_acted {
                // All enemies defeated - check if this was the boss level
                if self.is_boss_level {
                    // Victory! All levels and boss defeated
                    self.victory_win_time = if let Some(started_at) = self.game_started_at {
                        started_at.elapsed().as_secs_f32()
                    } else {
                        0.0
                    };
                    self.state = AppState::VictoryScreen;
                    self.audio_manager
                        .play_sound_effect(SoundEffect::AdvanceLevel);
                } else {
                    // Play level advance sound and go to next floor
                    self.audio_manager
                        .play_sound_effect(SoundEffect::AdvanceLevel);
                    self.floor_level += 1;
                    self.player_has_acted = false;
                    self.regenerate_floor();
                }
            }
        }
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
    pub fn save_game(&self) -> std::io::Result<()> {
        use crate::model::gamesave::{GameSave, InventoryData};

        let time = if let Some(started) = self.game_started_at {
            started.elapsed().as_secs_f32()
        } else {
            0.0
        };

        let save = GameSave {
            player_name: self.char_name.clone(),
            player_stats: PlayerStats {
                attack_damage: self.character.attack_damage,
                attack_length: self.character.attack_length,
                attack_width: self.character.attack_width,
                dash_distance: self.character.dash_distance,
                health: self.character.health,
                max_health: self.character.health_max,
                gold: self.character.gold,
                enemies_killed: self.character.enemies_killed,
            },
            inventory_data: InventoryData {
                weapon_slots: vec![None; 9], // Simplified for now
                consumables: vec![],
            },
            floor_level: self.floor_level,
            max_levels: self.max_levels,
            position_x: self.character_position.0,
            position_y: self.character_position.1,
            difficulty: self.settings.difficulty.name().to_string(),
            time_elapsed: time,
        };
        save.save()
    }

    pub fn load_game(&mut self, player_name: &str) -> std::io::Result<()> {
        use crate::model::gamesave::GameSave;

        let save = GameSave::load(player_name)?;

        // Restore character stats
        self.char_name = save.player_name.clone();
        self.character.attack_damage = save.player_stats.attack_damage;
        self.character.attack_length = save.player_stats.attack_length;
        self.character.attack_width = save.player_stats.attack_width;
        self.character.dash_distance = save.player_stats.dash_distance;
        self.character.health = save.player_stats.health;
        self.character.health_max = save.player_stats.max_health;
        self.character.gold = save.player_stats.gold;
        self.character.enemies_killed = save.player_stats.enemies_killed;

        // Restore game state
        self.floor_level = save.floor_level;
        self.max_levels = save.max_levels;
        self.character_position = (save.position_x, save.position_y);

        Ok(())
    }
}
