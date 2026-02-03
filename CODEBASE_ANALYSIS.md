# Comprehensive Roguelite Dungeon Codebase Analysis

**Analysis Date:** February 3, 2026  
**Project:** Roguelite Dungeon (Rust, Ratatui Terminal Game)

---

## 1. MOVEMENT SPEED CONSTANTS

### Player Movement Speed

**Location:** [src/constants.rs](src/constants.rs#L28)
```rust
pub const PLAYER_MOVEMENT_SPEED: f32 = 1.0; // Tiles per tick (controlled by GAME_TICK_RATE_MS)
pub const PLAYER_MOVEMENT_TICKS_REQUIRED: u32 = 3; // Require 3 game ticks between moves (48ms per move = ~21 moves/sec)
```
- **Status:** ⚠️ **DEAD CODE** - `PLAYER_MOVEMENT_SPEED` is defined but never used
- **Actual Movement Control:** Via `PLAYER_MOVEMENT_TICKS_REQUIRED` (3 ticks)
- **Effective Speed:** ~21 moves/sec with 16ms tick rate = 48ms per move
- **Location Used:** [src/app.rs](src/app.rs#L200-210)

### Enemy Movement Speed

**Location:** [src/constants.rs](src/constants.rs#L29)
```rust
pub const ENEMY_MOVEMENT_TICKS_REQUIRED: u32 = 7; // Enemies move every 7 ticks (112ms per move = ~9 moves/sec)
```
- **Effective Speed:** ~9 moves/sec with 16ms tick rate = 112ms per move
- **Actual Implementation:** [src/app.rs](src/app.rs#L640) uses direct tick counter instead of using this constant
- **Issue:** Enemy movement uses `enemy.movement_ticks` directly instead of the constant

### Arrow/Projectile Speed

**Location:** [src/constants.rs](src/constants.rs#L41-L43)
```rust
pub const ARROW_SPEED: f32 = 8.0;        // Tiles per second
pub const THROW_SPEED: f32 = 10.0;
```
- **Usage:** [src/model/arrow.rs](src/model/arrow.rs#L15) - Arrow struct has `speed: f32`
- **Character Default:** [src/model/character.rs](src/model/character.rs#L36) - `pub arrow_speed: f32 = 8.0`

### Game Tick Rate

**Location:** [src/constants.rs](src/constants.rs#L5)
```rust
pub const GAME_TICK_RATE_MS: u128 = 16; // 16ms = 62.5 FPS
```
- **Usage:** [src/app.rs](src/app.rs#L27) - `game_tick_rate_ms: GAME_TICK_RATE_MS`
- **Impact:** All movement is gated by this value via `should_tick()` mechanism

### Character Speed Stat (Unused)

**Location:** [src/model/character.rs](src/model/character.rs#L9-L10)
```rust
#[allow(dead_code)]
pub speed: f32,  // Initialized to 5.0 in Default
```
- **Status:** ⚠️ **DEAD CODE** - Never used
- **Purpose:** Likely intended for future movement scaling
- **Constructor:** [src/model/character.rs](src/model/character.rs#L97) - Has `new(speed: f32)` but never called

---

## 2. DEAD CODE & UNUSED IMPLEMENTATIONS

### Character Methods with #[allow(dead_code)]

| Item | Location | Status | Notes |
|------|----------|--------|-------|
| `Character::speed` field | [char.rs#L9](src/model/character.rs#L9) | 🔴 UNUSED | Field initialized but never read |
| `Character::new(speed)` | [char.rs#L97](src/model/character.rs#L97) | 🔴 UNUSED | Constructor never called, uses Default instead |
| `Character::attack_cooldown_remaining()` | [char.rs#L139](src/model/character.rs#L139) | 🔴 UNUSED | Method defined but not called anywhere |
| `Character::is_attack_animating()` | [char.rs#L155](src/model/character.rs#L155) | 🔴 UNUSED | Animation check exists but not integrated |

### Constants

| Item | Location | Status | Notes |
|------|----------|--------|-------|
| `PLAYER_MOVEMENT_SPEED` | [const.rs#L28](src/constants.rs#L28) | 🔴 UNUSED | Defined but never referenced (compiler warning) |

### UI Drawing Functions

| Item | Location | Status | Notes |
|------|----------|--------|-------|
| `ui::drawing::calculate_velocity()` | [drawing.rs#L33](src/ui/drawing.rs#L33) | ⚠️ UNUSED | Helper function not called |
| `ui::drawing::calculate_blur_offset()` | [drawing.rs#L37](src/ui/drawing.rs#L37) | ⚠️ UNUSED | Helper function not called |
| `render_attack_cooldown_bar()` | [drawing.rs#L148](src/ui/drawing.rs#L148) | ⚠️ UNUSED | Marked as dead code with #[allow(dead_code)] |

### Floor Methods

| Item | Location | Status | Notes |
|------|----------|--------|-------|
| `Floor::as_string()` | [floor.rs#L48](src/model/floor.rs#L48) | ⚠️ DEBUG | Debugging utility, not used in game |
| `Floor::get_styled_tile()` | [floor.rs#L379](src/model/floor.rs#L379) | ⚠️ UNUSED | Method exists but not called |

### Game App Methods

| Item | Location | Status | Notes |
|------|----------|--------|-------|
| `App::get_attack_area()` | [app.rs#L332](src/app.rs#L332) | ⚠️ UNUSED | Marked with #[allow(dead_code)] |
| `App::get_ultimate_area()` | [app.rs#L759](src/app.rs#L759) | ⚠️ PARTIAL | Defined but usage unclear |
| `App::is_paused` field | [app.rs](src/app.rs) | ⚠️ UNUSED | Pause field exists but pause logic not fully implemented |

### Ultimate Cooldown

| Item | Location | Status | Notes |
|------|----------|--------|-------|
| `Ultimate::cooldown_remaining()` | [ultimate.rs#L72](src/model/ultimate.rs#L72) | ⚠️ UNUSED | Method defined but likely not called |

### GameSave

| Item | Location | Status | Notes |
|------|----------|--------|-------|
| `GameSave::save_exists()` | [gamesave.rs#L72](src/model/gamesave.rs#L72) | ⚠️ UNUSED | Marked with #[allow(dead_code)] |

### Arrow Methods

| Item | Location | Status | Notes |
|------|----------|--------|-------|
| `Arrow::stop()` method | [arrow.rs#L86](src/model/arrow.rs#L86) | ⚠️ UNUSED | Marked with #[allow(dead_code)] |

---

## 3. DUPLICATE CODE PATTERNS & LOGIC

### A. Cooldown Pattern Duplication

**Issue:** Multiple nearly-identical cooldown implementations across different structs.

#### Pattern 1: Character Cooldowns (4 identical patterns)
- Location: [character.rs#L107-L121](src/model/character.rs#L107-L121), etc.
- Affected: `dash_cooldown`, `attack_cooldown`, `bow_cooldown`, `block_cooldown`
- **Code Pattern:**
  ```rust
  pub fn can_<action>(&self) -> bool {
      match self.<action>_cooldown_start {
          None => true,
          Some(start_time) => start_time.elapsed().as_secs_f32() >= self.<action>_cooldown_duration,
      }
  }
  
  pub fn <action>_cooldown_remaining(&self) -> f32 {
      match self.<action>_cooldown_start {
          None => 0.0,
          Some(start_time) => {
              let elapsed = start_time.elapsed().as_secs_f32();
              (self.<action>_cooldown_duration - elapsed).max(0.0)
          }
      }
  }
  
  pub fn start_<action>_cooldown(&mut self) {
      self.<action>_cooldown_start = Some(Instant::now());
  }
  ```

#### Pattern 2: Enemy Attacks & Ultimates (2 identical patterns)
- **EnemyAttack:** [enemy_type.rs#L124-L145](src/model/enemy_type.rs#L124-L145)
- **EnemyUltimate:** [enemy_type.rs#L222-L243](src/model/enemy_type.rs#L222-L243)
- Same methods duplicated: `is_available()`, `use_attack()/use_ultimate()`, `update_cooldown()`, `cooldown_percent()`

#### Pattern 3: Tile Calculation Duplicates
- **ReachShape::area_tiles()** vs **ReachShape::self_tiles()**: [reach_shape.rs#L127-L147](src/model/reach_shape.rs#L127-L147)
  - Both methods produce identical 3x3 grid result
  - Comment admits: "SELF(3x3) - around player (same as AREA)"
  - **Recommendation:** Consolidate to single method

### B. Movement Tick Handling Duplication

**Pattern:** Both player and enemy movement increment `movement_ticks` identically:

**Player:**
```rust
self.movement_tick_counter += 1;
if self.movement_tick_counter < PLAYER_MOVEMENT_TICKS_REQUIRED { return; }
self.movement_tick_counter = 0;
```
**Location:** [app.rs#L200-L208](src/app.rs#L200-L208)

**Enemy:**
```rust
enemy.movement_ticks += 1.0;
if enemy.movement_ticks >= ENEMY_MOVEMENT_TICKS_REQUIRED { ... }
enemy.movement_ticks = 0.0;
```
**Location:** [app.rs#L635-L645](src/app.rs#L635-L645)

### C. File Logging Duplication

**Issue:** Identical logging pattern repeated 15+ times throughout [app.rs](src/app.rs)

**Pattern:**
```rust
let _ = std::fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open("log.txt")
    .and_then(|mut f| f.write_all(msg.as_bytes()));
```

**Locations:**
- [app.rs#L213](src/app.rs#L213) - Move attempt
- [app.rs#L220](src/app.rs#L220) - Move success
- [app.rs#L230](src/app.rs#L230) - Move blocked
- [app.rs#L265](src/app.rs#L265) - Dash attempt
- [app.rs#L272](src/app.rs#L272) - Dash success
- [app.rs#L282](src/app.rs#L282) - Dash blocked
- [app.rs#L310](src/app.rs#L310) - Attack
- [app.rs#L354](src/app.rs#L354) - Shoot
- Multiple more locations

**Recommendation:** Extract to utility function `fn log_event(msg: &str)`

### D. Direction Defaulting Pattern

Appears in 3+ methods:
```rust
let (dx, dy) = self.character.last_direction;
let (dir_x, dir_y) = if dx == 0 && dy == 0 {
    (0, 1) // Default: forward
} else {
    (dx, dy)
};
```

**Locations:**
- [app.rs#L301-L307](src/app.rs#L301-L307) - Attack
- [app.rs#L340-L348](src/app.rs#L340-L348) - Shoot
- [app.rs#L332-L338](src/app.rs#L332-L338) - Get attack area

---

## 4. REACHSHAPE VS ATTACKPATTERN: RELATIONSHIP ANALYSIS

### ReachShape Struct
**Location:** [src/model/reach_shape.rs](src/model/reach_shape.rs#L4)

**Purpose:** Defines geometric shapes for collision detection
- **6 Variants:** Line, Cone, Arc, Cross, Area, Self_
- **Method:** `get_affected_tiles(direction, player_pos) -> Vec<(i32, i32)>`
- **Scope:** Pure geometry calculation, no animation or visualization

**Characteristics:**
- Direction-aware (respects facing direction)
- Static pattern generation
- Used for hit detection

### AttackPattern Enum
**Location:** [src/model/attack_pattern.rs](src/model/attack_pattern.rs#L16)

**Purpose:** Defines complete attack system with animations
- **14+ Variants:** BasicSlash, GroundSlam, Whirlwind, SwordThrust, ArrowShot, MultiShot, Barrage, PiercingShot, Fireball, ChainLightning, FrostNova, MeteorShower, CrescentSlash, Vortex
- **Key Methods:** 
  - `get_animation_frames()` - Returns animation sequence
  - `get_affected_tiles()` - Returns final hit area
- **Scope:** Animation + geometry + visual representation

**Characteristics:**
- Animation frame-based
- Includes color, symbol, timing info
- Much larger complexity (899 lines total)

### Relationship

#### Architecture Issue ⚠️

**Problem:** ReachShape and AttackPattern solve overlapping problems with different approaches:

1. **ReachShape** is a lightweight geometric primitive system
   - Used by weapons: [weapon.rs#L28](src/model/weapon.rs#L28) stores `pub reach_shape: ReachShape`
   - Pure calculation focus

2. **AttackPattern** is a monolithic animation system
   - Used for enemy attacks but has redundant geometry
   - Gets affected tiles by computing final animation frame

3. **No Integration:** They're never used together
   - Weapons use ReachShape
   - EnemyAttacks use AttackPattern in their pattern field: [enemy_type.rs#L123](src/model/enemy_type.rs#L123)
   - Player attack uses neither systematically

#### Design Issues

| Issue | Impact | Recommendation |
|-------|--------|-----------------|
| Redundant geometry logic | Code duplication | Extract ReachShape logic into both systems |
| AttackPattern too large | Maintenance burden | Split into Animation + Geometry modules |
| No common interface | Inconsistent attacks | Create trait-based attack system |
| EnemyAttack.pattern unused | Dead code | Integrate pattern visual system |

#### Suggested Refactor

```rust
// New unified trait approach
pub trait AttackShape {
    fn get_tiles(&self, direction: (i32, i32), origin: (i32, i32)) -> Vec<(i32, i32)>;
    fn get_animation(&self) -> Option<Vec<AnimationFrame>>;
}

// ReachShape becomes AttackShape implementer
impl AttackShape for ReachShape { ... }

// AttackPattern keeps animation but delegates geometry to ReachShape
pub enum AttackPattern {
    Melee(ReachShape),
    Ranged { reach: i32, pattern: ReachShape },
    Magical { reach: i32, animation: Animation },
}
```

---

## 5. MISSING FEATURES & INCOMPLETE IMPLEMENTATIONS

### A. Player Features Not Fully Implemented

| Feature | Status | Location | Notes |
|---------|--------|----------|-------|
| Block ability | ⚠️ PARTIAL | [char.rs#L173-L190](src/model/character.rs#L173-L190) | Can start cooldown but no damage reduction logic |
| Ultimate ability | ⚠️ PARTIAL | [ultimate.rs](src/model/ultimate.rs) | Animation exists, damage application missing |
| Animation system | ⚠️ BASIC | [char.rs#L155-L161](src/model/character.rs#L155-L161) | Simple 200ms check, not integrated with rendering |
| Weapon system | ⚠️ BASIC | [weapon.rs](src/model/weapon.rs) | Only damage/cooldown, no passive effects |
| Status effects | ✅ COMPLETE | [status_effect.rs](src/model/status_effect.rs) | Well implemented, not fully used |

### B. Enemy Features Not Fully Implemented

| Feature | Status | Location | Notes |
|---------|--------|----------|-------|
| Attack pattern usage | ⚠️ UNUSED | [enemy_type.rs#L123](src/model/enemy_type.rs#L123) | Pattern stored but visual never rendered |
| Buff spells | 🔴 UNUSED | [enemy_type.rs#L255](src/model/enemy_type.rs#L255) | BuffSpell enum defined, creation functions exist, never applied |
| Effect application | 🔴 UNUSED | [enemy_type.rs#L147](src/model/enemy_type.rs#L147) | Effect field exists, update_cooldown() not integrated |
| Ultimate ability | 🔴 UNUSED | [enemy_type.rs#L196](src/model/enemy_type.rs#L196) | EnemyUltimate struct, never used in game logic |
| Pathfinding cache | ⚠️ UNUSED | [pathfinding_cache.rs](src/model/pathfinding_cache.rs) | Cache system exists but enemies use A* without caching |

### C. Game System Gaps

| Feature | Status | Notes |
|---------|--------|-------|
| Pause system | 🔴 INCOMPLETE | `is_paused` field exists [app.rs](src/app.rs), no pause logic |
| Save/Load system | ⚠️ PARTIAL | GameSave structure exists but saving never called |
| Item system | ⚠️ BASIC | Items spawn but limited interaction |
| Consumable usage | ⚠️ PARTIAL | Consumables defined, use mechanics incomplete |

---

## 6. PERFORMANCE ISSUES & POTENTIAL BUGS

### A. Performance Issues

#### 1. **Inefficient Logging in Hot Path**
- **Issue:** File I/O on every movement attempt in game loop
- **Location:** [app.rs#L200-L282](src/app.rs#L200-L282)
- **Impact:** ~15 file opens/writes per player action
- **Recommendation:** Use `log` crate or disable in release mode

#### 2. **Pathfinding Cache Not Used**
- **Issue:** Enemies compute A* path every move, cache system ignored
- **Location:** [enemy.rs#L160-L250](src/model/enemy.rs#L160-L250), but pathfinding_cache.rs never called
- **Impact:** O(n) pathfinding on every enemy update
- **Recommendation:** Actually use the PathfindingCache

#### 3. **Duplicate Lookups**
- **Issue:** Walking tiles computed twice: once to render, once for collision
- **Location:** [app.rs#L620](src/app.rs#L620) - `walkable_tiles` HashSet computed every frame
- **Recommendation:** Cache walkable tiles per frame

#### 4. **String Formatting Overhead**
- **Issue:** Multiple `format!()` calls for logging even when not writing
- **Locations:** [app.rs#L211](src/app.rs#L211), [app.rs#L264](src/app.rs#L264), etc.
- **Recommendation:** Use lazy formatting or conditional compilation

#### 5. **Enemy Movement Tick Counter as Float**
- **Issue:** Using `f32` for tick counting introduces floating-point precision issues
- **Location:** [enemy.rs#L29](src/model/enemy.rs#L29) - `pub movement_ticks: f32`
- **Recommendation:** Use `u32` like player movement does
- **Risk:** Over time, floating-point error could cause missed movement checks

### B. Potential Bugs

#### 1. **Integer Overflow in Damage Calculation**
- **Issue:** No bounds on damage stacking
- **Location:** [damage_calculator.rs](src/model/damage_calculator.rs)
- **Risk:** Unlimited damage scaling with equipment could overflow

#### 2. **Attack Pattern/Direction Mismatch**
- **Issue:** Attack assumes direction is always set
- **Code:**
  ```rust
  let (dx, dy) = self.character.last_direction;
  if dx == 0 && dy == 0 { 
      (0, 1) // Default
  }
  ```
- **Risk:** Initialization direction always (0, 0), so first attack goes forward unintentionally
- **Location:** [app.rs#L301-L307](src/app.rs#L301-L307)

#### 3. **Floating-Point Tick Accumulation**
- **Issue:** Enemy movement uses `+=` on f32 without clamping
- **Code:** [enemy.rs#L101](src/model/enemy.rs#L101) - `self.movement_ticks += self.speed`
- **Risk:** Ticks can accumulate unboundedly if speed < 1.0
- **Recommendation:** Use modulo/wrapping after check

#### 4. **Position Cloning in Pathfinding**
- **Issue:** Multiple `.clone()` calls in hot path
- **Location:** [enemy.rs#L103](src/model/enemy.rs#L103) - `find_path(&self.position.clone(), ...)`
- **Impact:** Unnecessary allocations in game loop
- **Fix:** Use references instead

#### 5. **Enemy Collision With Other Enemies Missing**
- **Issue:** `collision_with_player` field exists but no enemy-enemy collision
- **Location:** [enemy.rs#L27](src/model/enemy.rs#L27) and [app.rs#L640-L660](src/app.rs#L640-L660)
- **Result:** Enemies can stack on same tile
- **Recommendation:** Implement grid-based collision detection

#### 6. **Undefined Behavior With Out-of-Bounds Access**
- **Issue:** Floor tiles access could panic on bad input
- **Location:** [floor.rs#L104-L112](src/model/floor.rs#L104-L112)
- **Mitigation:** Good bounds checking exists, but enemy spawn could bypass
- **Check:** [enemy.rs#L76-L96](src/model/enemy.rs#L76-L96) does check bounds

#### 7. **Ultimate Charge Never Incremented**
- **Issue:** `character.ultimate_charge` is initialized but never updated
- **Location:** [character.rs#L57](src/model/character.rs#L57)
- **Result:** Ultimate always starts at 0%, not usable
- **Fix:** Add charge increment in update_game_logic()

---

## 7. CODE ORGANIZATION ISSUES

### A. Module Structure Problems

#### 1. **enemy_type.rs Too Large**
- **Size:** 937 lines
- **Content:** Mix of enums, enemy factories, attack systems, buff systems
- **Recommendation:** Split into:
  - `enemy_type.rs` - EnemyRarity, EnemyType enums only
  - `enemy_attack.rs` - EnemyAttack struct and methods
  - `enemy_buff.rs` - EnemyBuff and EnemyEffect
  - `enemy_ultimate.rs` - EnemyUltimate struct
  - `enemy_templates.rs` - All creator functions

#### 2. **attack_pattern.rs Too Large**
- **Size:** 899 lines (mostly animation code)
- **Problems:**
  - Duplicated reach calculation from ReachShape
  - Monolithic animation system hard to extend
- **Recommendation:** Extract to:
  - `attack_pattern.rs` - Enum variants only
  - `attack_animation.rs` - Animation frame generation
  - `attack_geometry.rs` - Tile calculation

#### 3. **app.rs Monolithic**
- **Size:** 771 lines
- **Contains:** Game state, input handling, rendering dispatch, game logic
- **Recommendation:** Move logic to:
  - `game_logic.rs` - All game update methods
  - `input_handler.rs` - All input processing
  - Keep `app.rs` for state definition only

### B. Separation of Concerns Issues

#### 1. **File Logging in Game Logic**
- **Problem:** [app.rs#L200-L282](src/app.rs#L200-L282) has debug logging mixed in
- **Impact:** Pollutes game logic, hard to disable
- **Solution:** Use `log` crate with feature flags

#### 2. **UI Rendering Logic in Model**
- **Problem:** AttackPattern includes visual info (color, symbol)
- **Should be:** In separate AnimationFrame type only
- **Issue:** Makes patterns hard to serialize/reuse

#### 3. **No Clear Game State Machine**
- **Problem:** AppState enum has 5 states but transitions scattered
- **Locations:** Input handlers, UI draw functions, game logic
- **Recommendation:** Implement explicit state machine with transition handlers

### C. Naming & Convention Issues

#### 1. **Inconsistent Naming**
| Pattern | Example | Problem |
|---------|---------|---------|
| `can_<action>()` | `can_dash()`, `can_attack()` | Good, consistent |
| `start_<action>_cooldown()` | Multiple instances | Good, consistent |
| `<action>_cooldown_remaining()` | Inconsistent usage | Should be private or removed |
| `pub speed` vs `pub movement_ticks` | Different units | Confusing |

#### 2. **Field Visibility Issues**
- **Problem:** Most game state is `pub`, allowing external modification
- **Example:** `pub character.health` can be set directly, bypassing `take_damage()`
- **Risk:** Inconsistent state (health > max, negative values)
- **Recommendation:** Make fields private, provide methods only

#### 3. **Magic Numbers Scattered**
| Magic Number | Locations | Recommendation |
|--------------|-----------|-----------------|
| `3` (ticks) | [app.rs#L205](src/app.rs#L205), constants | Move to constant ✓ |
| `7` (ticks) | [app.rs#L635](src/app.rs#L635), constants | Move to constant ✓ |
| `0.2` (attack anim) | [char.rs#L161](src/model/character.rs#L161) | Should be constant |
| `200ms` | Same location | Same |
| `5` (dash distance) | [char.rs#L74](src/model/character.rs#L74), constants | Move to constant ✓ |

---

## SUMMARY TABLE

| Category | Issue Count | Severity | Priority |
|----------|-------------|----------|----------|
| Dead Code | 15+ | 🟡 Medium | High (cleanup) |
| Duplicated Logic | 5 major patterns | 🟡 Medium | Medium (refactor) |
| Missing Features | 8 incomplete | 🔴 High | Medium (feature complete) |
| Performance Issues | 5 patterns | 🔴 High | Low (gameplay first) |
| Potential Bugs | 7 identified | 🔴 High | High (fix before release) |
| Organization Issues | 8 problems | 🟡 Medium | Low (future refactor) |

---

## RECOMMENDATIONS (Priority Order)

### Immediate (Before Release)
1. ✅ Remove `#[allow(dead_code)]` attributes and delete truly dead code
2. ✅ Fix ultimate charge increment logic (broken feature)
3. ✅ Use enemy collision system or document why stacking is allowed
4. ✅ Extract logging to utility function or disable in release builds

### Short Term (Next Sprint)
1. Fix floating-point tick accumulation in enemy movement (use u32)
2. Implement pathfinding cache usage for performance
3. Integrate attack pattern visual rendering system
4. Consolidate cooldown pattern into generic trait

### Medium Term
1. Split large modules (enemy_type.rs, attack_pattern.rs)
2. Refactor ReachShape/AttackPattern overlap
3. Implement proper state machine for AppState
4. Add bounds checking and validation for all public APIs

### Long Term
1. Refactor app.rs into separate modules
2. Make game fields private with accessor methods
3. Implement comprehensive logging system
4. Add performance profiling and optimization

---

## FILES WITH MOST ISSUES

| File | Issues | Size | Priority |
|------|--------|------|----------|
| [src/app.rs](src/app.rs) | 12 | 771 lines | High (needs refactor) |
| [src/model/enemy_type.rs](src/model/enemy_type.rs) | 8 | 937 lines | High (too large) |
| [src/model/attack_pattern.rs](src/model/attack_pattern.rs) | 6 | 899 lines | Medium (refactor) |
| [src/model/character.rs](src/model/character.rs) | 5 | 241 lines | Medium (clean up) |
| [src/model/enemy.rs](src/model/enemy.rs) | 4 | 495 lines | Medium (performance) |

---

## QUICK FIXES CHECKLIST

- [ ] Delete `PLAYER_MOVEMENT_SPEED` constant
- [ ] Remove `Character::speed` field
- [ ] Delete unused `Character::new(speed)` constructor
- [ ] Merge `ReachShape::area_tiles()` and `self_tiles()`
- [ ] Extract logging to utility function
- [ ] Implement ultimate charge increment
- [ ] Change enemy movement_ticks from f32 to u32
- [ ] Add `#[non_exhaustive]` to public enums for future expansion
- [ ] Document why `is_paused` exists without logic

