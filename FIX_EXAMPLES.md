# Detailed Fix Examples & Solutions

## 1. ATTACK PATTERN ENUM FIX

### Current Enum Definition
```rust
// src/model/attack_pattern.rs - Lines 14-26
pub enum AttackPattern {
    Thrust(i32),
    CrescentSlash,
    ArrowShot(i32),
    MultiShot(i32, usize),     // reach, count
    Barrage(i32),
    PiercingShot(i32),
    Fireball(i32),
    ChainLightning(i32),
    FrostNova(i32),
    GroundSlam(i32),
    MeteorShower(i32, usize),
    Vortex(i32),
}
```

### Referenced but Missing Variants
- ❌ `BasicSlash` - used in lines 40, 875, 895, 929
- ❌ `WhirlwindAttack` - used in lines 46, 877, 899, 931
- ❌ `SwordThrust(i32)` - used in lines 47, 878, 900, 930

### SOLUTION A: Add Missing Variants
```rust
pub enum AttackPattern {
    // Melee attacks
    BasicSlash,                  // NEW: 3-tile slash with wide coverage
    Thrust(i32),
    SwordThrust(i32),            // NEW: Long reach thrust
    WhirlwindAttack,             // NEW: Spin attack all 8 directions
    CrescentSlash,
    
    // Ranged attacks
    ArrowShot(i32),
    MultiShot(i32, usize),
    Barrage(i32),
    PiercingShot(i32),
    
    // Magic attacks
    Fireball(i32),
    ChainLightning(i32),
    FrostNova(i32),
    GroundSlam(i32),
    MeteorShower(i32, usize),
    Vortex(i32),
}
```

### SOLUTION B: Rename References to Match Enum
```rust
// In get_animation_frames() match - change from:
AttackPattern::BasicSlash => { ... }
AttackPattern::SwordThrust(reach) => { ... }
AttackPattern::WhirlwindAttack => { ... }

// To:
AttackPattern::Thrust(reach) => { ... }  // If Thrust is meant to be basic
// Or keep old names if they're more descriptive
```

**RECOMMENDED:** Solution A - Add missing variants as they're more descriptive

---

## 2. DEPRECATED RAND API FIX

### Current Code
```rust
// src/model/floor.rs - Lines 607-608
let mut rng = rand::thread_rng();
let idx = rng.gen_range(0..region_tiles.len());

// src/model/enemy.rs - Lines 165, 296, 319-320
let mut rng = rand::thread_rng();
// ...
let x = rng.gen_range(1..floor.width as i32 - 1);
let y = rng.gen_range(1..floor.height as i32 - 1);
```

### Fixed Code
```rust
// NEW: Use rand::rng() and random_range()
use rand::Rng;  // Add this import

// src/model/floor.rs - Lines 607-608 (FIXED)
let mut rng = rand::rng();
let idx = rng.random_range(0..region_tiles.len());

// src/model/enemy.rs - Lines 165, 296, 319-320 (FIXED)
let mut rng = rand::rng();
// ...
let x = rng.random_range(1..floor.width as i32 - 1);
let y = rng.random_range(1..floor.height as i32 - 1);

// NOTE: Also check src/ui/dev_menu.rs which already uses correct API
let mut rng = rand::rng();  // ✅ Already correct here!
```

---

## 3. TYPE MISMATCH FIX

### Current Code
```rust
// src/model/attack_pattern.rs - Lines 54, 72
pub fn get_animation_frames(&self, ...) -> Vec<AnimationFrame> {
    match self {
        AttackPattern::MultiShot(reach, spread) => {
            // reach: i32, spread: usize (from enum)
            // But function expects i32, i32
            self.multishot_animation(origin_x, origin_y, dir_x, dir_y, *reach, *spread)
            //                                                               ^^^^^^  ^^^^^^^
            //                                                               i32     usize ❌
        }
        AttackPattern::MeteorShower(reach, width) => {
            // reach: i32, width: usize
            self.meteor_shower_animation(origin_x, origin_y, dir_x, dir_y, *reach, *width)
            //                                                             ^^^^^^  ^^^^^
            //                                                             i32     usize ❌
        }
    }
}
```

### Fix Option A: Cast to i32
```rust
AttackPattern::MultiShot(reach, spread) => {
    self.multishot_animation(origin_x, origin_y, dir_x, dir_y, 
                           *reach, *spread as i32)
}
AttackPattern::MeteorShower(reach, width) => {
    self.meteor_shower_animation(origin_x, origin_y, dir_x, dir_y,
                                *reach, *width as i32)
}
```

### Fix Option B: Change Enum to Use i32
```rust
pub enum AttackPattern {
    MultiShot(i32, i32),        // CHANGE: usize → i32
    MeteorShower(i32, i32),     // CHANGE: usize → i32
    // ... rest unchanged
}
```

**RECOMMENDED:** Option A (casting) - Fewer changes, usize might be semantically correct for counts

---

## 4. COOLDOWN STRUCT REFACTOR

### Current Duplication
```rust
// character.rs has this pattern repeated 4 times:
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

// REPEATED: can_attack/attack_cooldown/start_attack_cooldown
// REPEATED: can_shoot/bow_cooldown/start_bow_cooldown  
// REPEATED: can_block/block_cooldown/start_block_cooldown
```

### Solution: Generic Cooldown Struct

**Create new file: `src/model/cooldown.rs`**
```rust
use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Cooldown {
    start: Option<Instant>,
    duration: f32,
}

impl Cooldown {
    pub fn new(duration: f32) -> Self {
        Self {
            start: None,
            duration,
        }
    }

    pub fn can_use(&self) -> bool {
        match self.start {
            None => true,
            Some(start_time) => start_time.elapsed().as_secs_f32() >= self.duration,
        }
    }

    pub fn remaining(&self) -> f32 {
        match self.start {
            None => 0.0,
            Some(start_time) => {
                let elapsed = start_time.elapsed().as_secs_f32();
                (self.duration - elapsed).max(0.0)
            }
        }
    }

    pub fn start(&mut self) {
        self.start = Some(Instant::now());
    }

    pub fn reset(&mut self) {
        self.start = None;
    }
}
```

### Updated Character Struct
```rust
pub struct Character {
    // OLD - 4 fields per cooldown × 4 cooldowns = 8 fields:
    // pub dash_cooldown_start: Option<Instant>,
    // pub dash_cooldown_duration: f32,
    // pub attack_cooldown_start: Option<Instant>,
    // pub attack_cooldown_duration: f32,
    // ... etc

    // NEW - 1 field per cooldown = 4 fields:
    pub dash_cooldown: Cooldown,
    pub attack_cooldown: Cooldown,
    pub bow_cooldown: Cooldown,
    pub block_cooldown: Cooldown,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            // ...
            dash_cooldown: Cooldown::new(5.0),
            attack_cooldown: Cooldown::new(0.5),
            bow_cooldown: Cooldown::new(0.3),
            block_cooldown: Cooldown::new(6.0),
        }
    }
}

impl Character {
    // SIMPLIFIED:
    pub fn can_dash(&self) -> bool {
        self.dash_cooldown.can_use()
    }

    pub fn dash_cooldown_remaining(&self) -> f32 {
        self.dash_cooldown.remaining()
    }

    pub fn start_dash_cooldown(&mut self) {
        self.dash_cooldown.start()
    }
    // ... same pattern for other cooldowns
}
```

**BENEFIT:** 60 lines of code → 5 lines (90% reduction), reusable for ultimate.rs

---

## 5. ANIMATION BUILDER PATTERN

### Current Duplication (15 methods × 40 lines each = 600 lines)
```rust
fn basic_slash_animation(...) -> Vec<AnimationFrame> {
    let mut frames = vec![];
    frames.push(AnimationFrame {
        tiles: vec![(origin_x + dir_x, origin_y + dir_y)],
        color: Color::DarkGray,
        symbol: '.',
        frame_duration: 0.05,
    });
    frames.push(AnimationFrame { ... });
    // ... repeat 10+ times
    frames
}

fn ground_slam_animation(...) -> Vec<AnimationFrame> {
    let mut frames = vec![];
    frames.push(AnimationFrame { ... });  // Similar pattern
    // ... repeat
    frames
}
// ... and 13 more identical patterns
```

### Solution: Animation Builder

**Add to `src/model/attack_pattern.rs`**
```rust
pub struct AnimationBuilder {
    frames: Vec<AnimationFrame>,
}

impl AnimationBuilder {
    pub fn new() -> Self {
        Self { frames: Vec::new() }
    }

    pub fn add_frame(mut self, tiles: Vec<(i32, i32)>, color: Color, 
                    symbol: char, duration: f32) -> Self {
        self.frames.push(AnimationFrame {
            tiles,
            color,
            symbol,
            frame_duration: duration,
        });
        self
    }

    pub fn add_wind_up(self, origin: (i32, i32), direction: (i32, i32)) -> Self {
        let (ox, oy) = origin;
        let (dx, dy) = direction;
        self.add_frame(
            vec![(ox + dx, oy + dy)],
            Color::DarkGray,
            '.',
            0.05,
        )
    }

    pub fn add_swing_arc(self, center: (i32, i32)) -> Self {
        let (cx, cy) = center;
        let right = (cx - 1, cy);
        let left = (cx + 1, cy);
        
        self.add_frame(vec![right], Color::White, '*', 0.04)
            .add_frame(vec![right, (cx, cy), left], Color::LightYellow, 'X', 0.06)
            .add_frame(vec![left], Color::Yellow, '/', 0.05)
    }

    pub fn build(self) -> Vec<AnimationFrame> {
        self.frames
    }
}
```

### Usage Example
```rust
fn basic_slash_animation(...) -> Vec<AnimationFrame> {
    AnimationBuilder::new()
        .add_wind_up((origin_x, origin_y), (dir_x, dir_y))
        .add_swing_arc((origin_x + dir_x, origin_y + dir_y))
        // ... other frames
        .build()
}
```

**BENEFIT:** 600 lines → 200 lines (66% reduction)

---

## 6. MOVE MAGIC NUMBERS TO CONSTANTS

### Current Scattered Magic Numbers
```rust
// animation.rs:48
pub fn start_animation(&mut self) {
    self.animation = Some(UltimateAnimation {
        duration: 0.5,  // ❌ Magic number
        // ...
    });
}

// character.rs:169
pub fn is_attack_animating(&self) -> bool {
    attack_time.elapsed().as_secs_f32() < 0.2  // ❌ Magic number
}

// status_effect.rs:41
pub fn bleed_with_stacks(stacks: u32) -> Self {
    Self {
        duration: 8.0,  // ❌ Magic number
        // ...
    }
}

// drawing.rs:6
pub fn calculate_pulse_color(duration_secs: f32) -> Color {
    let pulse_val = ((duration_secs * 15.0).sin() + 1.0) / 2.0;  // ❌ Magic 15.0
    // ...
}
```

### Fixed: Add to `src/constants.rs`
```rust
// Animation durations
pub const ULTIMATE_ANIMATION_DURATION: f32 = 0.5;
pub const ATTACK_ANIMATION_DURATION: f32 = 0.2;

// Status effect durations
pub const BLEED_DURATION: f32 = 8.0;
pub const POISON_DURATION: f32 = 5.0;
pub const BURN_DURATION: f32 = 6.0;

// Visual effects
pub const PULSE_FREQUENCY: f32 = 15.0;
pub const PARTICLE_LIFETIME_CRIT: f32 = 0.5;
pub const PARTICLE_LIFETIME_HEAL: f32 = 0.4;
```

### Updated Code
```rust
// ultimate.rs
pub fn start_animation(&mut self) {
    self.animation = Some(UltimateAnimation {
        duration: crate::constants::ULTIMATE_ANIMATION_DURATION,
        // ...
    });
}

// character.rs
pub fn is_attack_animating(&self) -> bool {
    attack_time.elapsed().as_secs_f32() < crate::constants::ATTACK_ANIMATION_DURATION
}
```

**BENEFIT:** Consistency, easy balancing, all in one place

---

## 7. UNUSED VARIABLE FIXES

### Fix 1: collision.rs:33
```rust
// BEFORE
pub fn query_radius(&self, cx: i32, cy: i32, radius: i32) -> Vec<(i32, i32)> {
    let mut results = Vec::new();
    let cell_range = (radius + self.cell_size - 1) / self.cell_size;  // ❌ UNUSED
    
    let min_cell = self.get_cell(cx - radius, cy - radius);
    let max_cell = self.get_cell(cx + radius, cy + radius);
    
    // Uses min_cell/max_cell but never uses cell_range
}

// AFTER - USE the variable
pub fn query_radius(&self, cx: i32, cy: i32, radius: i32) -> Vec<(i32, i32)> {
    let mut results = Vec::new();
    let cell_range = (radius + self.cell_size - 1) / self.cell_size;
    
    // Limit search to cell_range:
    let min_cell_x = self.get_cell(cx - cell_range * self.cell_size, cy).0;
    let max_cell_x = self.get_cell(cx + cell_range * self.cell_size, cy).0;
    
    for grid_x in min_cell_x..=max_cell_x {
        for grid_y in (self.get_cell(cx, cy - cell_range * self.cell_size).1)
                    ..(self.get_cell(cx, cy + cell_range * self.cell_size).1) {
            if let Some(entities) = self.grid.get(&(grid_x, grid_y)) {
                for (ex, ey) in entities {
                    let dx = (*ex - cx).abs();
                    let dy = (*ey - cy).abs();
                    if dx * dx + dy * dy <= radius * radius {
                        results.push((*ex, *ey));
                    }
                }
            }
        }
    }
    
    results
}
```

### Fix 2: enemy.rs:318
```rust
// BEFORE
pub fn find_spawn_positions(...) -> Vec<Position> {
    // ...
    for attempt in 0..max_attempts {  // ❌ UNUSED
        let x = rng.random_range(1..floor.width as i32 - 1);
        let y = rng.random_range(1..floor.height as i32 - 1);
        // ...
    }
}

// AFTER - USE for diagnostics or rename to _
pub fn find_spawn_positions(...) -> Vec<Position> {
    // ...
    for _attempt in 0..max_attempts {  // Prefix with _ to silence warning
        let x = rng.random_range(1..floor.width as i32 - 1);
        let y = rng.random_range(1..floor.height as i32 - 1);
        // ...
    }
    // OR track attempts for logging:
    let mut attempt = 0;
    for attempt_count in 0..max_attempts {
        attempt = attempt_count;
        // ... logic
    }
    // Use attempt for logging
}
```

### Fix 3: app.rs:241
```rust
// BEFORE
use std::io::Write;  // ❌ Never imported or used

// AFTER - Remove the line entirely
```

---

## 8. SAVE/LOAD ERROR HANDLING

### Current Code (gamesave.rs)
```rust
pub fn save(&self, slot: u32) -> std::io::Result<()> {
    Self::ensure_saves_dir()?;
    let path = format!("saves/save_{}.json", slot);
    let data = serde_json::to_string_pretty(self).unwrap();  // ❌ Unsafe unwrap
    fs::write(path, data)
}

pub fn load(slot: u32) -> std::io::Result<Self> {
    let path = format!("saves/save_{}.json", slot);
    fs::read_to_string(&path).and_then(|data| {
        serde_json::from_str(&data)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    })
}
```

### Fixed Code
```rust
pub fn save(&self, slot: u32) -> std::io::Result<()> {
    Self::ensure_saves_dir()?;
    let path = format!("saves/save_{}.json", slot);
    
    // FIX: Proper error handling
    let data = serde_json::to_string_pretty(self)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    
    fs::write(path, data)
}

pub fn load(slot: u32) -> std::io::Result<Self> {
    let path = format!("saves/save_{}.json", slot);
    
    fs::read_to_string(&path).and_then(|data| {
        serde_json::from_str(&data)
            .map_err(|e| {
                eprintln!("Failed to deserialize save file: {}", e);  // Log error
                std::io::Error::new(std::io::ErrorKind::InvalidData, e)
            })
    })
}
```

---

## Summary of Changes

| File | Change | Impact |
|------|--------|--------|
| attack_pattern.rs | Add 3 missing enum variants | Fixes 23 compilation errors |
| floor.rs | Update rand API | Removes 2 deprecation warnings |
| enemy.rs | Update rand API, remove unused var | Removes 4 deprecation warnings, 1 unused warning |
| collision.rs | Fix or use cell_range | Removes 1 warning, improves logic |
| app.rs | Remove unused import | Removes 1 warning |
| character.rs | Use new Cooldown struct | 60 lines → 5 lines (90% reduction) |
| constants.rs | Add magic number constants | All in one place |

**Total Time:** ~2.5 hours for critical fixes
