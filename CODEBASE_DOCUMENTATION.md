# Roguelite Dungeon - Codebase Documentation

## Table of Contents
1. [Project Overview](#project-overview)
2. [Architecture & Structure](#architecture--structure)
3. [Core Systems](#core-systems)
4. [Data Models](#data-models)
5. [Algorithms](#algorithms)
6. [Main Functions & Workflows](#main-functions--workflows)
7. [Dependencies](#dependencies)

---

## Project Overview

**Roguelite Dungeon** is a terminal-based dungeon crawler game built in Rust using the Ratatui framework for UI rendering and Crossterm for terminal input/output handling. The game features:

- **Procedurally generated dungeons** using Cellular Automata
- **Turn-based combat** with various enemy types and boss battles
- **Skill tree system** for character progression
- **Ultimate ability shop** for special powers
- **Weapon loot system** with enchantments and rarities
- **Status effect system** (poison, bleed, burn, stun, etc.)
- **Save/load system** for persistent game state
- **Audio system** with music and sound effects
- **Real-time rendering** at 62.5 FPS (16ms per frame)

---

## Architecture & Structure

### Directory Structure

```
src/
├── main.rs                 # Entry point, event loop
├── app.rs                  # Main App struct & game state management
├── lib.rs                  # Library exports
├── constants.rs            # Game balance constants
├── colors.rs               # Color definitions
├── emoji.rs                # Unicode character definitions
├── input/
│   ├── handlers.rs         # Input event processing
│   └── menu.rs             # Menu navigation utilities
├── model/                  # Data models & game logic
│   ├── floor.rs            # Procedural dungeon generation
│   ├── character.rs        # Player character state
│   ├── enemy.rs            # Enemy entities & pathfinding
│   ├── weapon.rs           # Weapon system & enchantments
│   ├── item.rs             # Item drops & collections
│   ├── skill.rs            # Skill tree & player abilities
│   ├── ultimate.rs         # Ultimate ability system
│   ├── status_effect.rs    # Buff/debuff effects
│   ├── damage_calculator.rs # Damage computation (async)
│   ├── collision.rs        # Spatial hash grid
│   ├── pathfinding_cache.rs # Enemy AI path caching
│   ├── particle.rs         # Visual effects
│   ├── audio.rs            # Audio management
│   ├── gamesave.rs         # Save/load functionality
│   └── ... (other domain models)
└── ui/                     # Rendering & UI
    ├── drawing.rs          # Core rendering functions
    ├── main_menu.rs        # Main menu UI
    ├── character_creation.rs # Character creation
    ├── skill_tree.rs       # Skill tree UI
    ├── pause_menu.rs       # In-game pausing
    └── ... (other screens)
```

### Main App State Machine

The `App` struct manages all game state and transitions between screens:

```
MainMenu → CharacterCreation → Game ← → SkillTree / UltimateShop / Pause
         ↓
      SaveSelection
         ↓
        Game
         ↓
    DeathScreen / VictoryScreen
```

---

## Core Systems

### 1. **Rendering System** (`ui/drawing.rs`)

The rendering system handles all visual output using Ratatui widgets:

**Key Functions:**
- `render_key_hints()` - Display keyboard shortcuts
- `render_character()` - Draw player sprite with damage flash
- `calculate_pulse_color()` - Animated color pulsing effect
- `calculate_fade_color()` - Progressive color transitions
- `calculate_blur_offset()` - Motion blur for scrolling

**Color Scheme:**
- Uses 256-color palette (Color::Indexed)
- Dynamic color calculations based on game state
- Status effect visualization through color changes

### 2. **Game Loop** (`main.rs` & `app.rs`)

**Tick Rate:** 16ms = 62.5 FPS

```rust
loop {
    // Update logic (16ms interval)
    app.update_camera()              // Smooth camera following
    app.update_game_logic()          // Entity updates, collisions
    app.update_arrows()              // Projectile updates
    app.audio_manager.update()       // Audio transitions
    
    // Render frame
    terminal.draw(|f| ui::draw(f, app))?
    
    // Input handling
    if event::poll(timeout)? {
        match event::read()? {
            Event::Key(key) => input::handle_input()
            Event::Mouse(mouse) => input::handle_mouse_event()
        }
    }
    
    // Frame timing (wait if needed)
    if last_tick.elapsed() >= tick_rate { ... }
}
```

**Key App Methods:**
- `update_game_logic()` - Entity updates, pathfinding, combat
- `update_camera()` / `update_camera_smooth()` - Camera tracking
- `update_arrows()` - Projectile physics
- `save_game()` / `load_game()` - Persistence

### 3. **Input System** (`input/handlers.rs`)

Event handlers for different game states:

- `handle_main_menu_input()` - Navigate menu, start game
- `handle_game_input()` - Player movement, attacks, abilities
- `handle_pause_menu_input()` - Volume, settings adjustments
- `handle_skill_tree_input()` - Skill point allocation
- `handle_character_creation_input()` - Name, difficulty selection

### 4. **Dungeon Generation** (`model/floor.rs`)

**Algorithm: Cellular Automata**

```
Initial Phase:
1. Fill 45% of tiles randomly as walls
2. Keep outer border as walls

Cellular Automata Phases (5 iterations):
1. Count neighboring walls within distance 1 and 2
2. First 3 iterations: wall_count_1 >= 5 OR wall_count_2 <= 2 → wall
3. Later iterations: wall_count_1 >= 5 → wall
4. Creates organic cave-like structures

Post-Processing:
1. Detect connected rooms using flood-fill
2. Generate item drops in rooms
3. Spawn enemies in safe areas
4. Create styled tile cache for rendering
```

**Key Data Structures:**
- `tiles: Vec<bool>` - Walkable/wall grid (width × height)
- `rooms: Vec<Room>` - Detected connected areas
- `tile_to_room: Vec<Option<RoomId>>` - Fast room lookup
- `styled_tile_cache: Vec<(char, u8)>` - Cached glyphs & colors

### 5. **Collision & Spatial Hashing** (`model/collision.rs`)

**Data Structure: Spatial Hash Grid**

```rust
pub struct SpatialHash {
    cell_size: i32,
    grid: HashMap<(i32, i32), Vec<(i32, i32)>>
}
```

**Operations:**
- `insert(x, y)` - Add entity to grid cell
- `query_radius(cx, cy, radius)` - Get nearby entities
- `get_cell(x, y)` - Map world coords to grid cell

**Purpose:** O(1) proximity queries for collision detection instead of O(n²)

### 6. **Pathfinding & AI** (`model/enemy.rs` + `model/pathfinding_cache.rs`)

**Enemy Behavior States:**
1. **Wandering** - Random movement in max_range
2. **Chasing** - Move toward player when in detection_radius

**Algorithm: A* with Caching**

```rust
pub struct PathfindingCache {
    cache: HashMap<(from_x, from_y, to_x, to_y), next_step>
    max_size: usize
}

// When cache full, clear and rebuild (simple LRU)
pub fn set(&mut self, from: &Position, to: &Position, next_step) {
    if cache.len() >= max_size { cache.clear(); }
    cache.insert(key, next_step);
}
```

**Movement System:**
- Enemies move every 12 game ticks (192ms)
- Smooth movement with `movement_ticks` accumulator
- Knockback applied per frame with velocity decay

### 7. **Damage Calculation** (`model/damage_calculator.rs`)

**Algorithm: Multi-threaded Damage Formula**

```
Calculation Steps:
1. Apply attacker buffs (Sharpness: +%, BloodFrenzy: ×1.5)
2. Apply type advantage multiplier
3. Apply target armor reduction (capped at 75%)
4. Roll critical strike (1.5× multiplier if successful)
5. Ensure minimum 1 damage
6. Generate breakdown string for UI

Formula:
damage = base_damage
damage *= (1 + attacker_buff%)
damage *= type_multiplier
damage *= (1 - armor_reduction%)
if critical: damage *= 1.5
```

**Threading:**
- `calculate_async()` - Spawns background thread, returns receiver
- `calculate_sync()` - Immediate calculation when needed
- Prevents frame stutters during complex calculations

### 8. **Status Effect System** (`model/status_effect.rs`)

**Effect Types:**

| Effect | Duration | Damage/Sec | Behavior |
|--------|----------|-----------|----------|
| Bleed | 8s | 1 DPS | Stacks additively |
| Poison | Variable | 1 DPS | Refreshes duration |
| Burn | Variable | 2-3 DPS | Spreads to adjacent tiles |
| Stun | Variable | 0 | Cannot act |
| Cripple | Variable | 0 | Reduced movement speed |
| Fear | Variable | 0 | Move away from player |
| Healing | Positive | -HP/sec | Restores health |

**Manager Struct:**
```rust
pub struct StatusEffectManager {
    effects: Vec<StatusEffect>  // Applied to both player & enemies
}
```

### 9. **Audio System** (`model/audio.rs`)

**Features:**
- Multiple audio tracks (music, SFX)
- Fade transitions between tracks
- Thread-safe playback using `parking_lot`
- Sound effects: MenuSwitch, MenuPick, HitPlayer, HitEnemy, etc.

**Key Methods:**
- `play_music()` - Fade to new music track
- `play_sound_effect()` - Fire and forget SFX
- `update()` - Update fade timers each frame

### 10. **Save/Load System** (`model/gamesave.rs`)

**Serialization Format:** JSON

**Saved Data:**
```rust
pub struct GameSave {
    pub player_stats: PlayerStats
    pub character: Character
    pub floor: Floor
    pub timestamp: String
}
```

**File Location:** `./saves/<name>.json`

**Methods:**
- `save_to_file()` - Serialize app state to disk
- `load_from_file()` - Deserialize game from disk
- `list_saves()` - Return available save files

---

## Data Models

### Character (`model/character.rs`)

```rust
pub struct Character {
    // Movement
    pub speed: f32 = 5.0 tiles/sec
    pub dash_cooldown: Cooldown
    pub dash_distance: i32 = 5
    
    // Combat
    pub health: i32
    pub attack_damage: i32 = 10
    pub attack_width: i32 & attack_length: i32
    pub attack_cooldown: Cooldown
    
    // Ranged
    pub arrow_speed: f32 = 8.0 tiles/sec
    pub bow_cooldown: Cooldown
    
    // Equipment
    pub weapon_inventory: WeaponInventory
    pub shield: Option<Shield>
    
    // Progression
    pub skill_tree: SkillTree
    pub ultimate: Ultimate
    pub ultimate_charge: f32 (0-100)
    
    // Status
    pub status_effects: StatusEffectManager
    pub knockback_velocity: (f32, f32)
}
```

### Enemy (`model/enemy.rs`)

```rust
pub struct Enemy {
    pub position: Position
    pub speed: f32                      // tiles per tick
    pub max_range: Option<i32>          // patrol radius
    pub detection_radius: i32 = 5
    pub health: i32, max_health: i32
    pub rarity: EnemyRarity             // Fighter/Guard/Champion/Elite/Boss
    pub attacks: Vec<EnemyAttack>       // attack patterns
    
    // State
    pub is_wandering: bool
    pub movement_ticks: f32
    pub attack_ticks: f32
    pub knockback_velocity: (f32, f32)
}
```

### Weapon (`model/weapon.rs`)

```rust
pub enum WeaponType { Sword, Bow, Mace, Spear, Axe, Staff }

pub struct Weapon {
    pub weapon_type: WeaponType
    pub damage: i32
    pub cooldown: f32
    pub enchants: Vec<Enchant>          // DamageIncrease, RadiusIncrease
    pub rarity: ItemTier                // Common/Uncommon/Rare/Legendary
    pub attack_pattern: AttackPattern   // BasicSlash/ArrowShot/etc
}
```

### Skill (`model/skill.rs`)

```rust
pub enum SkillType { Slash, Pierce, HeavyAttack, Whirlwind, GroundSlam }

pub enum SkillLevel {
    Novice,     // 1.0x damage, 0% cooldown reduction
    Apprentice, // 1.25x damage, 10% cooldown reduction
    Expert,     // 1.5x damage, 25% cooldown reduction
    Master,     // 2.0x damage, 40% cooldown reduction
}

pub struct Skill {
    pub skill_type: SkillType
    pub level: SkillLevel
    pub base_cooldown: f32
}
```

### Item & Loot (`model/item.rs`)

```rust
pub enum ItemDropType {
    Consumable(Consumable),
    Gold(u32),
    Weapon(Weapon),
}

pub struct ItemDrop {
    pub item_type: ItemDropType
    pub x: i32, y: i32
    pub tier: ItemTier              // Affects rarity color
    pub stackable: bool
}
```

---

## Algorithms

### 1. **Cellular Automata Dungeon Generation**

**Purpose:** Create procedurally generated orthogonal dungeons with cave-like features

**Algorithm:**
```
Input: width, height, seed
Output: 2D grid of walls/walkable tiles

Step 1: Random Fill (45% probability)
  For each tile:
    if border → wall
    elif random < 45% → wall
    else → walkable

Step 2: Cellular Automata (5 iterations)
  For each iteration:
    For each tile (except border):
      neighbors_1 = count_walls_distance_1(x,y)
      neighbors_2 = count_walls_distance_2(x,y)
      if iteration < 3:
        new_tile = neighbors_1 >= 5 OR neighbors_2 <= 2
      else:
        new_tile = neighbors_1 >= 5
    Apply changes all at once

Step 3: Room Detection
  Use flood-fill to identify connected components
  Map each tile to its room
```

**Time Complexity:** O(width × height × iterations) = O(n × m × 5)

**Space Complexity:** O(width × height)

---

### 2. **Spatial Hash Grid for Collision**

**Purpose:** Efficient O(1) proximity queries without O(n²) comparisons

**Algorithm:**
```
Insert(entity, x, y):
  cell = (x / cell_size, y / cell_size)
  grid[cell].push(entity)

QueryRadius(cx, cy, radius):
  results = []
  min_cell = ((cx-radius)/cell_size, (cy-radius)/cell_size)
  max_cell = ((cx+radius)/cell_size, (cy+radius)/cell_size)
  
  For each cell in grid range:
    For each entity in cell:
      dist = sqrt((entity.x - cx)² + (entity.y - cy)²)
      if dist <= radius:
        results.push(entity)
  return results
```

**Cell Size:** Typically set to 20×20 or larger based on query radius

**Time Complexity:** O(k) where k = entities in query range (typically small)

**Space Complexity:** O(n + m) where m = grid cells

---

### 3. **Pathfinding with LRU Cache**

**Purpose:** Cache common pathfinding results to avoid expensive A* recalculations

**Algorithm:**
```
Get(from, to):
  key = (from.x, from.y, to.x, to.y)
  return cache.get(key)

Set(from, to, next_step):
  if cache.size >= max_size:
    cache.clear()  # Simple LRU: clear when full
  cache.insert((from.x, from.y, to.x, to.y), next_step)

On Floor Change:
  cache.clear()  # Invalidate when map changes
```

**Hit Rate:** Enemies repeatedly path to same targets = good cache locality

**Time Complexity:** O(1) lookup, O(1) insert

---

### 4. **Damage Calculation with Type System**

**Purpose:** Scale damage based on attacker type, defender type, buffs, and critical strikes

**Algorithm:**
```
Calculate(base_damage, attack_type, target_type, buffs):
  damage = base_damage
  
  // Attacker buffs
  for buff in attacker_buffs:
    if buff == Sharpness(x):
      damage *= (1 + x/100)
    elif buff == BloodFrenzy:
      damage *= 1.5
  
  // Type advantage
  type_mult = get_advantage(attack_type, target_type)
  damage *= type_mult  // Can be 0.5x to 2.0x
  
  // Target armor
  armor = sum(target_buffs.filter(Armor).values)
  armor = clamp(armor, 0, 75)
  damage *= (1 - armor/100)
  
  // Critical strike
  if random() < critical_chance:
    is_critical = true
    damage *= 1.5
  
  return max(ceil(damage), 1)
```

**Damage Factors:**
- Base damage (5-50 typically)
- Type advantage (0.5x to 2.0x)
- Armor reduction (up to 75%)
- Critical multiplier (1.5x)
- Buff multipliers (0.5x to 2.0x)

---

### 5. **Status Effect Management**

**Purpose:** Apply recurring damage, crowd control, and buffs

**Algorithm:**
```
Apply(effect):
  if effect.type == Bleed:
    // Find existing bleed
    if existing_bleed:
      existing_bleed.stacks += effect.stacks
      existing_bleed.duration = max(duration, 8s)
    else:
      effects.push(effect)
  
  elif effect.type == Poison:
    // Refresh or stack
    if existing_poison:
      existing_poison.duration = effect.duration
    else:
      effects.push(effect)
  
  elif effect.type == Burn:
    // Can spread to adjacent tiles
    effects.push(effect)
    spread_to_adjacent_enemies()

Update(dt):
  for effect in effects:
    effect.duration -= dt
    apply_damage_per_sec(effect.damage_per_sec * dt)
    if effect.type == Burn:
      update_spread()
  
  effects.retain(|e| e.duration > 0)
```

**Stacking Rules:**
- Bleed: Stacks additively (1 + 1 + 1 = 3 stacks)
- Poison: Refreshes duration (max 1 active)
- Burn: Spreads to adjacent tiles, creates chain reactions

---

### 6. **Smooth Camera Following**

**Purpose:** Create fluid camera movement to reduce motion sickness

**Algorithm:**
```
Update(player_pos):
  target = player_pos - (screen_width/2, screen_height/2)
  camera_target = target
  
SmoothUpdate():
  smooth_factor = 0.1
  offset.x += (camera_target.x - offset.x) * smooth_factor
  offset.y += (camera_target.y - offset.y) * smooth_factor
```

**Parameters:**
- `CAMERA_SMOOTH_FACTOR: 0.1` - Higher = faster tracking
- Exponential decay interpolation

---

### 7. **Animation Frame Timing**

**Purpose:** Smooth sprite animation independent of frame rate

**Algorithm:**
```
Update(dt):
  timer += dt
  
  if timer >= current_frame.duration:
    timer -= current_frame.duration
    current_frame_idx += 1
  
  if current_frame_idx >= total_frames:
    animation_finished = true

GetCurrentFrame():
  if current_frame_idx < frames.len():
    return frames[current_frame_idx]
  else:
    return None
```

**Frame Duration:** Measured in seconds (0.1s = 100ms per frame)

---

## Main Functions & Workflows

### Game Initialization (`main.rs`)

```rust
fn main() -> io::Result<()> {
    enable_raw_mode()?                          // Terminal setup
    let mut terminal = Terminal::new(...)?      // Create Ratatui backend
    let mut app = app::App::new()               // Initialize app state
    run_app(&mut terminal, &mut app)?           // Start game loop
    disable_raw_mode()?                         // Cleanup
}
```

### Game Loop (`run_app()`)

```rust
fn run_app<B: Backend>(terminal, app) {
    loop {
        app.update_terminal_size(size)          // Handle resize
        app.update_camera()                     // Update camera target
        app.update_camera_smooth()              // Interpolate camera
        app.update_game_logic()                 // Update entities
        app.update_arrows()                     // Projectile physics
        app.audio_manager.update(elapsed)       // Audio fades
        
        terminal.draw(|f| ui::draw(f, app))?
        app.last_scroll_offset = app.scroll_offset
        
        // Input handling with timeout
        if crossterm::event::poll(timeout)? {
            match crossterm::event::read()? {
                Event::Key(key) => input::handle_input(app, key)
                Event::Mouse(mouse) => input::handle_mouse_event(app, mouse)
                _ => {}
            }
        }
        
        if app.should_quit {
            if app.state == Game: app.save_game()
            return Ok(())
        }
    }
}
```

### Character Movement

```rust
// Main update loop processes player input
handle_game_input(app, key) {
    match key.code {
        KeyCode::Up | 'w' => {
            if can_move_up() {
                character.position.y -= 1
                character.last_direction = (0, -1)
                update_walkable_cache()
            }
        }
        // Similar for other directions
    }
}

// Engine applies movement smoothing
app.update_game_logic() {
    // Natural ticks-based movement
    character.movement_ticks += elapsed
    
    if character.movement_ticks >= MOVEMENT_TICKS_REQUIRED {
        character.movement_ticks = 0.0
        // Actually move character on tile grid
    }
}
```

### Combat Flow

```
Player Input: Press Attack Key
    ↓
execute_attack() {
    attack_cooldown.reset()
    last_attack_time = now()
    
    for enemy in get_enemies_in_range() {
        damage = calculate_damage(...)  // Async thread
        enemy.take_damage(damage)
        apply_knockback(enemy, direction, force)
        
        if dead: drop_loot()
        else: spawn_animation()
    }
}
    ↓
Update Animation {
    update_active_animations(dt)
    render_attack_visuals()
}
    ↓
Enemy AI Response {
    if detection_radius.contains(player):
        is_wandering = false
        pathfind_to_player()
    else:
        wander_randomly()
}
    ↓
Enemy Attack {
    if in_range && attack_cooldown_ready:
        player.take_damage(enemy_damage)
        apply_knockback(player, ...)
}
```

### Skill Tree Progression

```
Allocate Skill Point
    ↓
open_skill_tree_ui()
    ↓
player input: select skill
    ↓
skill_tree.upgrade(skill_type) {
    if available_points > 0:
        skill.level = next_level()
        available_points -= 1
        update_cooldowns_and_damage()
}
    ↓
Character stats updated:
    attack_cooldown *= (1 - skill.level.cooldown_reduction())
    attack_damage *= skill.level.damage_multiplier()
```

### Save Game

```
app.save_game() {
    game_save = GameSave {
        player_stats: ...,
        character: app.character.clone(),
        floor: app.current_floor.clone(),
        timestamp: now(),
    }
    
    json = serde_json::to_string(&game_save)?
    file = fs::File::create("./saves/<name>.json")?
    file.write_all(json.as_bytes())?
}
```

### Load Game

```
app.load_game(name: &str) {
    file = fs::File::open("./saves/<name>.json")?
    game_save = serde_json::from_reader(file)?
    
    app.character = game_save.character
    app.current_floor = game_save.floor
    app.floor_level = game_save.floor_level
    
    regenerate_walkable_cache()
}
```

---

## Dependencies

### External Crates

| Crate | Version | Purpose |
|-------|---------|---------|
| `ratatui` | 0.30 | Terminal UI rendering widget library |
| `crossterm` | 0.29 | Terminal input/output handling |
| `serde` | 1.0 | Serialization/deserialization framework |
| `serde_json` | 1.0 | JSON format support |
| `rand` | 0.10 | Random number generation & seeded RNG |
| `rodio` | 0.21 | Audio playback |
| `parking_lot` | 0.12 | Thread-safe synchronization primitives |
| `unicode-width` | 0.2 | Unicode character width calculation |

### Internal Modules

```
lib.rs
├── colors           # Color palette definitions
├── constants        # Game balance tuning
├── model            # Core game logic & data
│   ├── character    # Player state
│   ├── enemy        # Enemy AI & combat
│   ├── floor        # Dungeon generation
│   ├── damage_calculator  # Async damage math
│   ├── status_effect      # Buffs/debuffs
│   ├── skill              # Skill tree system
│   ├── weapon             # Equipment system
│   ├── audio              # Audio management
│   ├── particle           # Visual effects
│   ├── gamesave           # Persistence
│   └── ...
└── ui              # Rendering layer
    ├── drawing     # Core draw functions
    ├── main_menu   # Menu rendering
    ├── skill_tree  # Tree UI
    └── ...
```

---

## Performance Characteristics

### Frame Budget (16ms @ 62.5 FPS)

| System | Time | Notes |
|--------|------|-------|
| Game Logic Update | ~2-3ms | Entity updates, collision checks |
| Camera Calculations | ~0.5ms | Smooth interpolation |
| Pathfinding Cache | <0.1ms | Hash lookup O(1) |
| Damage Calculations | Async | Offloaded to thread pool |
| Audio Update | <1ms | Fade state machine |
| Rendering | ~5-8ms | Ratatui widget tree |
| **Total** | **~12-15ms** | Headroom for spike handling |

### Memory Usage

- **Dungeon Grid:** 180 × 60 = 10.8KB (binary)
- **Styled Cache:** 10.8KB × 2 (glyph + color)
- **Pathfinding Cache:** 1-10MB typically (configurable)
- **Save File:** ~50-200KB JSON (includes full game state)

### Optimization Techniques

1. **Spatial Hashing** - O(1) collision queries
2. **Pathfinding Cache** - Avoid repeated A* calculations
3. **Tile Cache** - Pre-computed colors and glyphs
4. **Async Damage** - Non-blocking calculations
5. **Viewport Culling** - Only render visible entities
6. **Audio Fade System** - Smooth transitions without audio artifacts

---

## Known Limitations & Future Improvements

### Current Limitations

1. **Single-threaded rendering** - Ratatui doesn't support parallel draws
2. **Pathfinding cache invalidation** - Full clear when map changes
3. **Damage calculations** - Limited to available thread pool
4. **Save file format** - No versioning for backward compatibility
5. **Audio** - Limited to basic fade transitions

### Potential Extensions

1. **Procedural skill generation** - Randomized player abilities
2. **Multiplayer support** - Network synchronization
3. **Advanced pathfinding** - Jump point search, HPA*
4. **Texture streaming** - For larger environments
5. **Replay system** - Record and playback gameplay
6. **Modding API** - User-created content support

---

## Testing

Test files located in `tests/`:

- `integration_tests.rs` - Full game flow tests
- `edge_case_tests.rs` - Boundary condition testing

**Run Tests:**
```bash
cargo test
```

---

## Conclusion

This roguelike dungeon crawler demonstrates:
- **Efficient algorithms** for procedural generation and spatial queries
- **Clean architecture** with separated concerns (model, view, input)
- **Performance optimization** through caching and async processing
- **Game design patterns** like state machines and skill trees
- **Rust best practices** with type safety and zero-cost abstractions

The codebase is designed to be extensible for new features like additional enemy types, skills, weapon enchantments, and procedural content generation variations.
