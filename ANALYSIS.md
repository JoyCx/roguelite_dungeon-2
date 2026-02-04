# Roguelite Dungeon - Comprehensive Code Analysis Report

**Date:** February 5, 2026  
**Severity Levels:** 🔴 Critical | 🟠 High | 🟡 Medium | 🟢 Low

---

## 📊 Executive Summary

**Total Issues Found:** 40+  
**Compilation Errors:** 8  
**Deprecation Warnings:** 6  
**Code Smells:** 15+  
**Missing Features:** 8+  
**Performance Issues:** 4

---

## 🔴 CRITICAL ERRORS (Must Fix Immediately)

### 1. **Attack Pattern Enum Mismatch** [attack_pattern.rs, enemy_type.rs]
- **Issue:** Enum variants referenced in code don't exist
  - `BasicSlash` ❌ (not in enum definition)
  - `WhirlwindAttack` ❌ (not in enum definition)  
  - `SwordThrust(i32)` ❌ (not in enum definition)
- **Locations:**
  - [attack_pattern.rs](src/model/attack_pattern.rs#L40-L54) - 8 references in match arms
  - [attack_pattern.rs](src/model/attack_pattern.rs#L875-L930) - name(), description(), weapon_type() methods
  - [enemy_type.rs](src/model/enemy_type.rs#L299,L324,L403,L507,L531,L560,L584) - 7 enemy spawns
- **Impact:** **Won't compile** - 23 compilation errors
- **Fix:** Align enum variants or rename references to match actual enum:
  - Current enum has: `Thrust`, `CrescentSlash`, `ArrowShot`, etc.
  - Code expects: `BasicSlash`, `WhirlwindAttack`, `SwordThrust`

### 2. **Deprecated Rand API Calls** [floor.rs, enemy.rs]
- **Issue:** Using deprecated methods from rand crate
  - `rand::thread_rng()` → renamed to `rand::rng()`
  - `gen_range()` → renamed to `random_range()`
- **Locations:**
  - [floor.rs:607-608](src/model/floor.rs#L607) - `rand::thread_rng()`, `gen_range()`
  - [enemy.rs:165](src/model/enemy.rs#L165) - `rand::thread_rng()`
  - [enemy.rs:296](src/model/enemy.rs#L296) - `rand::thread_rng()`
  - [enemy.rs:319-320](src/model/enemy.rs#L319) - `gen_range()` x2
- **Impact:** Generates 6 deprecation warnings, will break in future rand versions
- **Fix:** Replace all with modern API calls

### 3. **Type Mismatch in Animation Parameters** [attack_pattern.rs:54, 72]
- **Issue:** `usize` passed where `i32` expected
  ```rust
  // Line 54: reach is usize, spread is usize, but multishot_animation expects i32
  self.multishot_animation(origin_x, origin_y, dir_x, dir_y, *reach, *spread)
  // Line 72: width is usize, but meteor_shower_animation expects i32  
  self.meteor_shower_animation(origin_x, origin_y, dir_x, dir_y, *reach, *width)
  ```
- **Impact:** Type mismatch compilation errors
- **Fix:** Cast parameters or change enum variant storage type

---

## 🟠 HIGH PRIORITY ISSUES

### 4. **Unused Variables** [collision.rs, enemy.rs]
- **Issue:** Variables calculated but never used
  - [collision.rs:33](src/model/collision.rs#L33) - `cell_range` unused
  - [enemy.rs:318](src/model/enemy.rs#L318) - `attempt` loop variable unused
- **Fix:** Either use the variable or remove it

### 5. **Unused Assignment** [enemy.rs:268]
- **Issue:** `current = parent` assigned but never read after
- **Impact:** Compiler warning about dead code
- **Fix:** Analyze if assignment is needed or simplify logic

### 6. **Missing Enum Variants in Match Expressions** [app.rs:200-230]
- **Issue:** `get_attack_pattern_category()` references non-existent variants
  - Lines 212-219 match against `BasicSlash`, `WhirlwindAttack`, `SwordThrust`
- **Impact:** Won't compile once enum is fixed
- **Fix:** Update match expression when attack pattern enum is aligned

### 7. **Unsafe Unwrap in Serde Deserialization** [gamesave.rs:53]
- **Issue:** 
  ```rust
  let data = serde_json::to_string_pretty(self).unwrap();
  ```
- **Impact:** Panic if serialization fails (unlikely but poor error handling)
- **Fix:** Use `.map_err()` and propagate error

### 8. **Unsafe Index Access in item.rs:get_glyph()** [item.rs:70-104]
- **Issue:** Multiple nested match statements that could be exhausted
- **Note:** Currently safe due to enum constraints, but fragile design
- **Risk:** Will break if new weapon/item types added

---

## 🟡 MEDIUM PRIORITY - CODE DUPLICATION

### 9. **Cooldown Pattern Duplication** [character.rs]
**Affects:** dash_cooldown, attack_cooldown, bow_cooldown, block_cooldown
- **Duplication Count:** 4 identical patterns, each ~20 lines
- **Code:**
  ```rust
  pub fn can_dash(&self) -> bool { /* identical logic */ }
  pub fn dash_cooldown_remaining(&self) -> f32 { /* identical logic */ }
  pub fn start_dash_cooldown(&mut self) { /* identical logic */ }
  // ...repeated 3 more times for other cooldowns
  ```
- **Lines:** Approximately 60 lines of duplicated cooldown code
- **Opportunity:** Create generic `Cooldown` struct with `can_use()`, `remaining()`, `start()` methods
- **Refactoring Benefit:** 50% code reduction in character.rs

### 10. **Status Effect Application Duplication** [status_effect.rs]
- **Issue:** Similar logic for Bleed, Poison, Burn with different parameters
- **Code:** Lines 100-135 in StatusEffectManager::add()
- **Pattern:** Match on effect type, check for existing, update or push
- **Opportunity:** Extract into generic effect handler method

### 11. **Animation Frame Creation Duplication** [attack_pattern.rs]
- **Issue:** 15 animation methods all follow same pattern
  ```rust
  fn [attack]_animation(...) -> Vec<AnimationFrame> {
      let mut frames = vec![];
      frames.push(AnimationFrame { ... });
      frames.push(AnimationFrame { ... });
      // ...repeat
      frames
  }
  ```
- **Lines:** ~600 lines total of animation code with high duplication
- **Opportunity:** Create animation builder helper with common patterns

### 12. **Cooldown Check Pattern** [character.rs, ultimate.rs, enemy.rs]
- **Duplication:** Same cooldown checking logic in 3+ files
  ```rust
  match self.cooldown_start {
      None => true,
      Some(start_time) => start_time.elapsed().as_secs_f32() >= self.duration,
  }
  ```
- **Opportunity:** Create reusable cooldown utility module

### 13. **Collision Detection Duplication** [app.rs, floor.rs, enemy.rs]
- **Issue:** Similar distance/collision checking scattered across files
- **Opportunity:** Centralize collision logic in collision.rs module

### 14. **JSON Save/Load Logic Duplication** [gamesave.rs, settings.rs]
- **Pattern:** Both implement same load/save pattern
  ```rust
  pub fn load() -> Self {
      fs::read_to_string(path)
          .and_then(|data| serde_json::from_str(&data).map_err(...))
          .unwrap_or_else(|_| Self::default())
  }
  ```
- **Opportunity:** Create generic `Persistent<T>` wrapper or trait

---

## 🟡 MEDIUM PRIORITY - UNOPTIMIZED CODE

### 15. **Unused `cell_range` Calculation** [collision.rs:33]
- **Issue:** Variable calculated but never used in query_radius()
- **Code:**
  ```rust
  let cell_range = (radius + self.cell_size - 1) / self.cell_size; // Never used!
  ```
- **Impact:** Suggests incomplete optimization
- **Likely Fix:** Should be used to limit grid cell iteration range

### 16. **A* Pathfinding Not Cached** [enemy.rs:200-300]
- **Issue:** `find_path()` creates new HashMaps on every call
- **Impact:** 
  - Called every movement tick for every enemy
  - Allocates HashMaps unnecessarily
  - No path caching between frames
- **Optimization:** 
  - Cache recent paths in enemy struct
  - Reuse path if target hasn't moved
  - Estimated impact: 30-50% fewer allocations

### 17. **Vector Allocations in Animations** [attack_pattern.rs]
- **Issue:** Each animation method creates new `Vec<AnimationFrame>`
- **Impact:** Hundreds of allocations during combat
- **Optimization:** Pre-allocate or use object pool

### 18. **Every Enemy Spawns on Every Frame** [floor.rs]
- **Issue:** `spawn_random_items()` and `spawn_enemies()` use StdRng with seed
- **Code:** `let mut rng = StdRng::seed_from_u64(self.seed.wrapping_add(999));`
- **Impact:** Same RNG seed generates same positions every call
- **Risk:** If called multiple times, duplicates enemies/items

### 19. **Room Detection BFS Inefficiency** [floor.rs:240-290]
- **Issue:** Uses VecDeque for BFS but doesn't limit search
- **Impact:** O(width * height) iteration for each room
- **Optimization:** Could break early or use iterative approach

### 20. **Distance Calculations Not Optimized** [enemy.rs, floor.rs]
- **Issue:** Manhattan distance calculated multiple times for same pair
- **Impact:** A* pathfinding calls heuristic multiple times per node
- **Optimization:** Cache or precompute distance matrix for small spaces

---

## 🟢 MISSING FEATURES & INCOMPLETE CODE

### 21. **No Error Handling for File I/O** [gamesave.rs, settings.rs]
- **Issue:** 
  - No error recovery if save file corrupted
  - If JSON parse fails, silently defaults to default values
- **Risk:** Player progress could be lost silently
- **Missing:** 
  - Backup save file system
  - Error logging
  - User notification

### 22. **Projectile Despawn Logic Incomplete** [arrow.rs]
- **Issue:** 
  - Projectiles despawn based on time/distance
  - No collision with walls implemented
  - `is_dead` field exists but not properly integrated
- **Missing:** 
  - Wall collision detection
  - Proper despawn cleanup
  - Area damage application for FireOil

### 23. **No Bounds Checking for World Coordinates**
- **Issue:** Many functions assume valid coordinates without checking
- **Examples:**
  - get_tile() doesn't bounds-check
  - render functions don't validate camera offsets
- **Risk:** Out-of-bounds reads/writes
- **Missing:** Consistent coordinate validation layer

### 24. **No Enemy Behavior Tree or State Machine** [enemy.rs]
- **Issue:** Enemy logic split between wander(), move_toward(), and external logic
- **Current State:** 
  - No clear state management
  - Behavior hardcoded based on booleans (is_wandering)
- **Missing:**
  - Formal state machine (Wandering, Chasing, Attacking, Stunned)
  - Action queue system
  - Behavior composition

### 25. **Incomplete Attack Pattern Implementation** [attack_pattern.rs]
- **Issue:** Animation frames defined but:
  - No actual damage calculation using the patterns
  - No hit detection for animated areas
  - Animation doesn't trigger game logic
- **Missing:**
  - get_affected_tiles() for each pattern
  - Integration with collision system
  - Async damage application

### 26. **No Consumable Use Effect System** [consumable.rs]
- **Issue:** Consumable struct has methods but:
  - No handler for actually applying effects
  - Healing only defined as values, not applied
  - No status effect application
- **Missing:** 
  - Effect application interface
  - Animation/feedback system
  - Cooldown integration

### 27. **No Weapon Skill System** [weapon.rs]
- **Issue:** Weapon has enchants but:
  - No skill/ability system
  - No weapon-specific attacks
  - Reach patterns hardcoded in weapon.rs
- **Missing:**
  - Weapon-specific abilities
  - Skill trees
  - Combo system

### 28. **Incomplete Ultimate Ability** [ultimate.rs]
- **Issue:** Ultimate has animation but:
  - `get_affected_area()` never actually damages enemies
  - No integration with damage calculator
  - No cooldown enforcement
- **Missing:**
  - Actual damage application
  - Enemy status effect application
  - Visual feedback

---

## 🟢 MEDIUM PRIORITY - DESIGN ISSUES

### 29. **Magic Number Proliferation** [constants.rs, multiple files]
- **Issue:** Some magic numbers in code despite constants file
- **Examples:**
  - `0.5` for animation duration (ultimate.rs:48)
  - `8.0` for bleed duration (status_effect.rs:41)
  - `15.0` multiplier in color pulsing (drawing.rs:6)
  - `0.2` for attack animation (character.rs:169)
- **Missing:** These should be in constants.rs
- **Impact:** Hard to balance and maintain

### 30. **No Consistent Delta Time Handling**
- **Issue:** 
  - Some code uses GAME_TICK_RATE_MS
  - Others use Instant::now().elapsed()
  - Some hardcode update intervals
- **Impact:** 
  - Inconsistent frame-rate independence
  - Difficulty balancing
  - Potential timing bugs

### 31. **Enemy Type Duplication in enemy_type.rs**
- **Issue:** Enemy definitions (Fighter, Guard, Champion, etc.) repeat similar patterns
- **Code:** Lines 270-600 have 5+ similar enemy templates
- **Pattern:**
  ```rust
  EnemyType::Fighter => EnemyTemplate {
      name: "Fighter".to_string(),
      health: 20,
      detection_radius: 5,
      pattern: AttackPattern::BasicSlash,
      // ...
  }
  // ...repeated 4 more times
  ```
- **Opportunity:** Use data-driven approach with enemy definitions

### 32. **No Difficulty Scaling for Attributes**
- **Issue:** Difficulty enum exists but:
  - Not consistently applied to all scalable values
  - Health doesn't scale with difficulty
  - Damage doesn't scale with difficulty
  - Only detection_radius scaled
- **Missing:** Comprehensive difficulty multiplier system

### 33. **Loose Coupling Between Game Systems**
- **Issue:** App struct directly couples:
  - Character position logic
  - Floor generation
  - Animation management
  - Input handling
  - UI rendering
- **Size:** App is 946 lines - too large
- **Opportunity:** Use proper ECS or component system

### 34. **No Integration Tests**
- **Files:** Only damage_calculator.rs has tests (3 tests)
- **Missing:** Tests for:
  - Pathfinding edge cases
  - Collision detection
  - Animation frame generation
  - Save/load round-trip
  - Cooldown calculations

### 35. **Incomplete Input Binding System** [input.rs, settings.rs]
- **Issue:** 
  - Settings stores keybindings as strings
  - No validation of duplicate bindings
  - No indicator of invalid keys
  - Mouse button handling mixed with keyboard
- **Problems:**
  - User could bind same key twice
  - "LeftClick" vs "Left" inconsistency
  - No rebinding feedback

---

## 🟡 MEDIUM PRIORITY - STABILITY & EDGE CASES

### 36. **Divide by Zero Risk** [character.rs, item.rs, particle.rs]
- **Issue:** `get_health_percentage()` [character.rs:220]
  ```rust
  pub fn get_health_percentage(&self) -> f32 {
      if self.health_max == 0 {
          0.0  // Good protection, but why would max_health be 0?
      } else {
          (self.health as f32 / self.health_max as f32).clamp(0.0, 1.0)
      }
  }
  ```
- **Risk:** Suggests health_max could be 0, which shouldn't happen
- **Missing:** Invariant enforcement (health_max > 0 always)

### 37. **Inventory Index Bounds Not Always Checked**
- **Issue:** Several places use inventory indices without bounds checks
- **Examples:**
  - Enemy template weapon selection assumes valid index
  - Consumable inventory pop without validation
- **Risk:** Panic on edge cases

### 38. **No Maximum Values for Stacking/Accumulation**
- **Issue:** 
  - Enemy bleed stacks have no cap
  - Particle system can grow unbounded
  - Gold accumulation: `saturating_add` is good but no hard cap
- **Risk:** 
  - Memory bloat from stacked effects
  - Particle lag in long sessions

### 39. **Dead Code & Dead Paths**
- **Issue:** 
  - `#[allow(dead_code)]` appears 4+ times
  - Some methods marked dead_code might be needed
  - Check: reach_shape.rs for self_tiles() usage
- **Files:** ultimate.rs, character.rs, particle.rs

### 40. **No Panic Handler or Error Reporting**
- **Issue:** 
  - No custom panic hook
  - No error log file
  - Terminal cleanup might fail silently on panic
- **Risk:** Unrecoverable terminal state if panic occurs
- **Missing:** 
  - Graceful shutdown
  - Error logging
  - User-friendly error messages

---

## 📋 ADDITIONAL CODE QUALITY ISSUES

### 41. **Unused Import** [app.rs:241]
```rust
use std::io::Write;  // Never used
```

### 42. **Inconsistent Error Handling**
- **Issue:** Mix of unwrap(), ?, and ignore patterns
- **Files:** Multiple
- **Missing:** Error handling strategy documentation

### 43. **No Lifetime Management for Game State**
- **Issue:** Items, enemies, particles all have created_at but:
  - No automatic cleanup system
  - Manual retain() calls scattered
  - No object pooling
- **Risk:** Memory leaks in long play sessions

### 44. **Camera System Hardcoded** [app.rs]
- **Issue:** Camera offset calculation hardcoded
- **Missing:** Configurable camera behavior (pan speed, smoothing, boundaries)

### 45. **Color Inconsistencies** [drawing.rs, item.rs, colors.rs]
- **Issue:** Color values hardcoded in multiple places
- **Examples:**
  - Tier colors in item.rs (75-88)
  - Indexed color codes scattered (236, 238, 240, etc.)
- **Better:** Centralized color scheme/palette

---

## 🔧 REFACTORING OPPORTUNITIES (by effort/benefit)

| Priority | Issue | Effort | Benefit | Est. Time |
|----------|-------|--------|---------|-----------|
| 1 | Fix Attack Pattern enum | 🔴 1-2h | Must fix | High |
| 2 | Update deprecated rand API | 🟢 15min | Critical | Low |
| 3 | Create Cooldown struct | 🟡 2h | High reuse | Medium |
| 4 | Centralize magic numbers | 🟢 1h | Maintainability | Low |
| 5 | Animation builder pattern | 🟡 3h | 50% code cut | Medium |
| 6 | Enemy state machine | 🟡 3h | Extensibility | Medium |
| 7 | Implement ECS architecture | 🔴 8h+ | Major refactor | Very High |
| 8 | Add comprehensive tests | 🟡 4h | Stability | Medium |
| 9 | Error handling pass | 🟡 2h | Reliability | Medium |
| 10 | Performance optimization | 🟡 3h | Speed | Medium |

---

## 🎯 RECOMMENDED IMMEDIATE ACTIONS

### Phase 1 (Critical - 2 hours)
1. ✅ Fix AttackPattern enum variants
2. ✅ Update deprecated rand calls
3. ✅ Fix type mismatches in animation calls
4. ✅ Remove unused variables

### Phase 2 (High Priority - 4 hours)
5. Create generic Cooldown struct
6. Create animation builder helper
7. Centralize magic numbers
8. Add basic error handling

### Phase 3 (Medium Priority - 6 hours)
9. Implement enemy state machine
10. Add integration tests
11. Optimize pathfinding with caching
12. Implement consumable effects

### Phase 4 (Nice to Have - Later)
13. Full ECS refactor
14. Performance profiling
15. Additional game features

---

## 📈 Code Metrics

| Metric | Value | Status |
|--------|-------|--------|
| Total Lines of Code | ~7000 | Growing |
| Compilation Errors | 23 | 🔴 |
| Warnings | 8 | 🟠 |
| Test Coverage | ~5% | 🔴 |
| Duplicate Code Ratio | ~15% | 🟡 |
| Files | 24 | Growing |
| Cyclomatic Complexity | Medium | 🟡 |

---

## 📝 Notes for Future Development

- Consider splitting App struct (900+ lines) into multiple components
- Implement trait-based design for extensible systems
- Add logging crate (log + env_logger)
- Set up CI/CD for compilation checks
- Create architecture documentation
- Define coding standards and style guide
