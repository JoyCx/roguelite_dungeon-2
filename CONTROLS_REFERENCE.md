# Quick Reference: New Features

## Player Controls

### Movement
- **W** - Move Up
- **A** - Move Left
- **S** - Move Down
- **D** - Move Right

### Abilities
- **Space** - Dash (in last movement direction)

## Mechanics

### Speed & Movement
- **Tick Rate:** 100ms (10 movement checks per second)
- **Movement:** 1 tile per tick
- **Direction:** Remembers last direction for dashing

### Dash Ability
- **Cost:** Tick (same as normal move)
- **Distance:** 5 tiles
- **Direction:** Last movement direction
- **Cooldown:** 7 seconds
- **Wall Blocking:** Cannot dash into walls

### Cooldown Bar
- Bottom of screen during gameplay
- Shows real-time recharge progress
- Yellow = Charging, Green = Ready
- Displays remaining seconds

## System Features

### Tick-Based Movement
✅ Consistent across all FPS  
✅ Same gameplay feel on 30 FPS and 240 FPS  
✅ No system-dependent bugs  

### Collision System
✅ Player cannot clip through walls  
✅ Dashes blocked by walls  
✅ Movement blocked by walls  

### Character Attributes
- Speed: 5.0 tiles/second (for future use)
- Dash Distance: 5 tiles
- Dash Cooldown: 7 seconds

## Configuration

To adjust movement speed, edit `App::new()` in src/app.rs:
```rust
game_tick_rate_ms: 100,  // Lower = faster, Higher = slower
```

To adjust dash cooldown, edit Character::default() in src/model/character.rs:
```rust
dash_cooldown_duration: 7.0,  // Seconds
dash_distance: 5,             // Tiles
```
