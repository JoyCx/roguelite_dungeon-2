# Game Mechanics Implementation Guide

## Overview
This document outlines the newly implemented game mechanics: tick-rate system, character speed control, dash ability with cooldown, and wall collision prevention.

---

## 1. Time-Based Tick Rate System

### What Changed
- Replaced frame-rate-dependent movement with **time-based ticks**
- Movement is now consistent across all FPS values
- No more weird behavior on different systems

### Implementation Details

**In [src/app.rs](src/app.rs):**
```rust
pub last_game_tick: Instant,          // Last time character could move
pub game_tick_rate_ms: u128,          // Milliseconds between game ticks
```

**Tick checking:**
```rust
pub fn should_tick(&self) -> bool {
    self.last_game_tick.elapsed().as_millis() >= self.game_tick_rate_ms
}

pub fn consume_tick(&mut self) {
    self.last_game_tick = Instant::now();
}
```

### Adjusting Movement Speed

Edit `game_tick_rate_ms` in `App::new()`:
- **50ms** = 20 ticks/sec (very fast)
- **100ms** = 10 ticks/sec (normal) ← Default
- **200ms** = 5 ticks/sec (slow)

---

## 2. Character Stats & Attributes

### Character Struct

Created new [src/model/character.rs](src/model/character.rs):

```rust
pub struct Character {
    pub speed: f32,                    // Movement speed (tiles per second)
    pub last_direction: (i32, i32),    // Last movement direction (for dash)
    pub dash_cooldown_start: Option<Instant>,
    pub dash_cooldown_duration: f32,   // 7 seconds
    pub dash_distance: i32,            // 5 tiles per dash
}
```

### Default Values
- Speed: 5.0 tiles/second (for future use)
- Dash Distance: 5 tiles
- Dash Cooldown: 7 seconds

---

## 3. Movement & Collision

### Movement System

Movement now respects tick rate:

```rust
pub fn move_character(&mut self, dx: i32, dy: i32) {
    if !self.should_tick() {
        return;  // Wait for next tick
    }
    
    // ... collision check ...
    // ... actual movement ...
    
    self.consume_tick();  // Consume the tick after movement
}
```

### Wall Collision

The system checks `floor.is_walkable()` before moving:
- ✅ Prevents clipping through walls
- ✅ Blocks movement into solid tiles
- ✅ Player stops at walls

---

## 4. Dash Ability

### How It Works

1. **Direction Tracking**: Every move updates `character.last_direction`
2. **Dash Direction**: Dashes in the direction you were last moving
3. **Dash Distance**: Moves 5 tiles in that direction
4. **Cooldown**: 7-second cooldown after each dash
5. **Wall Collision**: Can't dash through walls (blocked if destination is a wall)

### Input Binding

In [src/input/handlers.rs](src/input/handlers.rs):
```rust
else if key_matches(key.code, &settings.dash) {
    app.dash();
}
```

Default keybind from [settings.json](settings.json): **Space**

### Dash Implementation

```rust
pub fn dash(&mut self) {
    if !self.character.can_dash() || !self.should_tick() {
        return;
    }
    
    let (dx, dy) = self.character.last_direction;
    // Dashes 5 tiles in that direction
    // Blocked by walls (checked via is_walkable)
    
    self.character.start_dash_cooldown();
    self.consume_tick();
}
```

---

## 5. Dash Cooldown Bar

### Visual Display

The cooldown bar appears at the bottom of the game screen:

```
[ ████████████░░░░░░░░░░░░░░░ ] 2.3s
```

- **Left Side**: Game map
- **Bottom Bar**: Animated cooldown indicator
- **Filled (█)**: Charge progress
- **Empty (░)**: Remaining cooldown
- **Status**: Shows time remaining or "DASH READY"

### Appearance

**While Charging:**
```
[ ███░░░░░░░░░░░░░░░░░░░░░░░░ ] 5.2s
```
- Yellow bar (charging)

**When Ready:**
```
[ ██████████████████████████████ ] DASH READY
```
- Green bar (ready to use)

### Implementation

**In [src/ui/drawing.rs](src/ui/drawing.rs):**
```rust
pub fn render_dash_cooldown_bar(
    f: &mut Frame,
    area: Rect,
    remaining_cooldown: f32,
    max_cooldown: f32,
)
```

**In [src/ui/mod.rs](src/ui/mod.rs):**
```rust
drawing::render_dash_cooldown_bar(
    f,
    cooldown_area,
    remaining_cooldown,
    app.character.dash_cooldown_duration,
);
```

---

## Game Flow

### Movement Sequence

1. Player presses movement key (W/A/S/D)
2. Input handler calls `app.move_character(dx, dy)`
3. System checks if tick has elapsed
4. If tick available:
   - Check if destination is walkable
   - Move character if safe
   - Update direction for dash
   - Consume tick

### Dash Sequence

1. Player presses dash key (Space)
2. Input handler calls `app.dash()`
3. System checks:
   - ✓ Dash available (not on cooldown)
   - ✓ Tick has elapsed
   - ✓ Direction was previously set
4. If all checks pass:
   - Move 5 tiles in last direction
   - Start cooldown
   - Consume tick
5. Cooldown bar animates for 7 seconds

---

## Configuration & Tuning

### Adjust Movement Speed

**File:** [src/app.rs](src/app.rs), `App::new()` method

```rust
pub game_tick_rate_ms: u128,  // Change this value
```

### Adjust Dash Distance

**File:** [src/model/character.rs](src/model/character.rs), `Default` implementation

```rust
pub dash_distance: i32,  // Currently 5 tiles
```

### Adjust Dash Cooldown

**File:** [src/model/character.rs](src/model/character.rs), `Default` implementation

```rust
pub dash_cooldown_duration: f32,  // Currently 7.0 seconds
```

---

## Features Implemented

| Feature | Status | Details |
|---------|--------|---------|
| Time-based tick rate | ✅ Complete | 100ms default (10 ticks/sec) |
| Character speed stat | ✅ Complete | Ready for future movement scaling |
| Dash ability | ✅ Complete | 5 tiles, space key, tracks direction |
| Dash cooldown | ✅ Complete | 7 seconds, resets after use |
| Cooldown bar (ASCII) | ✅ Complete | Animated, shows remaining time |
| Wall collision | ✅ Complete | Blocks movement and dashes |
| Direction tracking | ✅ Complete | Stores last movement direction |

---

## Technical Details

### File Changes

- **Created:** [src/model/character.rs](src/model/character.rs)
- **Modified:** [src/app.rs](src/app.rs)
- **Modified:** [src/model/mod.rs](src/model/mod.rs)
- **Modified:** [src/input/handlers.rs](src/input/handlers.rs)
- **Modified:** [src/ui/mod.rs](src/ui/mod.rs)
- **Modified:** [src/ui/drawing.rs](src/ui/drawing.rs)

### Performance

- Minimal CPU overhead (simple time comparisons)
- Smooth at any FPS (time-based, not frame-based)
- No visual artifacts or stuttering

---

## Future Enhancements

- [ ] Dash through enemies (with damage)
- [ ] Different dash types (forward, blink, charge)
- [ ] Speed stat affects movement tick rate
- [ ] Dash chain mechanic (reduced cooldown on consecutive dashes)
- [ ] Dash momentum/inertia
- [ ] Cooldown bar sounds/feedback
- [ ] Special abilities (greater cooldowns, unique effects)
