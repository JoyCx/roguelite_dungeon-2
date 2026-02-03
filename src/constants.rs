// Game constants - centralized magic numbers for maintainability

// Game loop timing
pub const GAME_TICK_RATE_MS: u128 = 16; // 16ms = 62.5 FPS

// Game world
pub const FLOOR_WIDTH: i32 = 180;
pub const FLOOR_HEIGHT: i32 = 60;
pub const DEFAULT_SEED: u64 = 12345;

// Dungeon generation
pub const FILL_PROBABILITY: u32 = 45; // Percentage chance of wall tile
pub const CELLULAR_AUTOMATA_ITERATIONS: u32 = 5;
pub const CUTOFF_BIG_AREA: i32 = 3;
pub const NEIGHBOR_THRESHOLD_BIG: u32 = 3;
pub const NEIGHBOR_THRESHOLD_SMALL: u32 = 3;

// Spawning
pub const ITEMS_PER_FLOOR: usize = 10;
pub const ENEMY_SPAWN_RADIUS: i32 = 20;

// Combat
pub const PLAYER_BASE_DAMAGE: i32 = 5;
pub const PLAYER_ATTACK_COOLDOWN: f32 = 0.5;
pub const PLAYER_BOW_COOLDOWN: f32 = 0.3;
pub const PLAYER_DASH_COOLDOWN: f32 = 5.0;
pub const PLAYER_BLOCK_COOLDOWN: f32 = 6.0;
pub const PLAYER_MOVEMENT_SPEED: f32 = 1.0; // Tiles per tick (controlled by GAME_TICK_RATE_MS)
pub const PLAYER_MOVEMENT_TICKS_REQUIRED: u32 = 3; // Require 3 game ticks between moves (48ms per move = ~21 moves/sec)
pub const ENEMY_MOVEMENT_TICKS_REQUIRED: u32 = 7; // Enemies move every 7 ticks (112ms per move = ~9 moves/sec)

// Enemy combat and gold drops
pub const ENEMY_BASE_HEALTH: i32 = 20;
pub const FIGHTER_BASE_GOLD: u32 = 10;
pub const GUARD_BASE_GOLD: u32 = 15;
pub const CHAMPION_BASE_GOLD: u32 = 25;
pub const ELITE_BASE_GOLD: u32 = 50;
pub const BOSS_BASE_GOLD: u32 = 150;

// Projectiles
pub const ARROW_SPEED: f32 = 8.0;
pub const ARROW_MAX_DISTANCE: f32 = 50.0;
pub const THROW_SPEED: f32 = 10.0;
pub const FIRE_OIL_IMPACT_RADIUS: i32 = 4;

// Camera
pub const CAMERA_SMOOTH_FACTOR: f32 = 0.1;

// UI
pub const HEALTH_BAR_WIDTH: u16 = 20;
pub const INVENTORY_MAX_ITEMS: usize = 5;
pub const COOLDOWN_BAR_HEIGHT: u16 = 3;

// Colors (for consistency)
pub const COLOR_HEALTH_GOOD: &str = "green";
pub const COLOR_HEALTH_WARNING: &str = "yellow";
pub const COLOR_HEALTH_CRITICAL: &str = "red";
pub const COLOR_GOLD: &str = "yellow";
pub const COLOR_ATTACK: &str = "red";
pub const COLOR_DASH: &str = "magenta";
pub const COLOR_BOW: &str = "cyan";
pub const COLOR_BLOCK: &str = "blue";
