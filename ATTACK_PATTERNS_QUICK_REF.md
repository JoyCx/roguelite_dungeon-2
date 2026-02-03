# Attack Patterns Quick Reference

## 14 Unique Attack Patterns with Full Animation & Directional Support

### Summary Table

```
┌─────────────────────────────────────────────────────────────────┐
│ MELEE PATTERNS (5)                                              │
├─────────────────────────────────────────────────────────────────┤
│ ✓ BasicSlash          → 3-tile slash with perpendicular width   │
│ ✓ SwordThrust(reach)  → Piercing forward thrust                 │
│ ✓ GroundSlam(reach)   → Expanding diamond shockwave             │
│ ✓ WhirlwindAttack     → 360° spinning attack (8 directions)     │
│ ✓ CrescentSlash       → Curved moon-shaped attack               │
├─────────────────────────────────────────────────────────────────┤
│ RANGED PATTERNS (4)                                             │
├─────────────────────────────────────────────────────────────────┤
│ ✓ ArrowShot(reach)           → Single projectile                │
│ ✓ MultiShot(reach, spread)   → Fan of 3 arrows                  │
│ ✓ Barrage(reach)             → Rapid fire succession            │
│ ✓ PiercingShot(reach)        → Strong armor-piercing shot       │
├─────────────────────────────────────────────────────────────────┤
│ MAGICAL PATTERNS (5)                                            │
├─────────────────────────────────────────────────────────────────┤
│ ✓ Fireball(radius)           → Expanding explosion              │
│ ✓ ChainLightning(reach)      → Zigzag electrical chains         │
│ ✓ FrostNova(reach)           → Diamond frost spread             │
│ ✓ MeteorShower(reach, width) → Rain of meteors                  │
│ ✓ Vortex(radius)             → Spiraling dark vortex            │
└─────────────────────────────────────────────────────────────────┘
```

## Quick Implementation Guide

### Basic Usage
```rust
// Create a pattern
let slash = AttackPattern::BasicSlash;
let fireball = AttackPattern::Fireball(3);

// Get animation frames (for rendering)
let frames = pattern.get_animation_frames(x, y, dir_x, dir_y);

// Get affected tiles (for damage)
let tiles = pattern.get_affected_tiles(x, y, dir_x, dir_y);

// Get info
println!("{}", pattern.name());           // e.g., "Ground Slam"
println!("{}", pattern.description());   // Full description
println!("{}", pattern.weapon_type());   // "Melee", "Ranged", or "Magic"
```

### Direction Vectors
```rust
// Cardinal & Diagonal Directions
right:  (dir_x:  1, dir_y:  0)
left:   (dir_x: -1, dir_y:  0)
down:   (dir_x:  0, dir_y:  1)
up:     (dir_x:  0, dir_y: -1)
down-right: (dir_x:  1, dir_y:  1)
// etc...
```

### Animation Frames
Each frame has:
- `tiles: Vec<(i32, i32)>` - Coordinates to render
- `color: Color` - ratatui Color for display
- `symbol: char` - Character to display ('*', '^', '/', etc.)
- `frame_duration: f32` - How long frame displays (seconds)

### Integration with Enemies
All 12 enemies use these patterns for their attacks:
- Each enemy has 1-5 attacks with different patterns
- Patterns are matched to enemy rarity and type
- Cooldown system prevents rapid firing

## Visual Reference

### Animation Symbols
- `*` Explosion/Impact/Magical
- `^` Arrow/Projectile
- `/` `\` Slash
- `>` Forward Thrust
- `~` Shockwave/Ripple
- `≈` Lightning Chain
- `○` Vortex/Circle
- `◆` Meteor/Falling
- `!` Rapid Fire
- `(` `)` Curved
- `»` Piercing

### Colors Used
- 🟡 Yellow/LightYellow (Ranged, strikes)
- 🔴 Red/LightRed (Impact, fire, explosions)
- 🟦 Blue (Frost, ice, cold)
- 🟪 Cyan (Lightning, electricity)
- 🟣 Magenta (Darkness, curves, voids)
- ⚪ White (Pure force, piercing)

## Pattern Characteristics

### By Reach Type
**Fixed Reach**: ArrowShot, Barrage, PiercingShot, BasicSlash
**Variable Reach**: SwordThrust, GroundSlam, MultiShot, ChainLightning, FrostNova, MeteorShower, Vortex
**No Reach**: WhirlwindAttack, CrescentSlash, Fireball (uses radius instead)

### By Animation Complexity
**Simple (2 frames)**: BasicSlash, CrescentSlash
**Progressive (multiple frames)**: SwordThrust, ArrowShot, MultiShot, Barrage, ChainLightning, FrostNova, MeteorShower
**Expanding (ring pattern)**: GroundSlam, Fireball, Vortex
**Rotating (directional animation)**: WhirlwindAttack

### By Coverage Area
**Single Point**: BasicSlash first frame
**Linear**: ArrowShot, SwordThrust, Barrage, PiercingShot
**Line with Spread**: MultiShot, CrescentSlash
**Circular**: Fireball, Vortex
**Diamond**: GroundSlam, FrostNova
**All Adjacent**: WhirlwindAttack
**Complex Patterns**: ChainLightning, MeteorShower

## Wall Collision Handling

Since patterns return affected tiles, you can:
```rust
// Filter out wall tiles before applying damage
let safe_tiles: Vec<_> = tiles
    .iter()
    .filter(|(x, y)| !floor.get_tile(*x, *y))  // true = wall
    .collect();

// Apply damage only to walkable tiles
for (x, y) in safe_tiles {
    apply_damage_at(x, y);
}
```

## Testing

All patterns include unit tests:
```bash
cargo test --lib attack_pattern::tests
```

Tests verify:
- Directional awareness
- Animation frame generation
- Tile coverage
- Color and symbol validity
- Pattern naming and descriptions

## Performance Notes

- **Minimal overhead**: O(reach²) at most for circular patterns
- **Deterministic**: Same inputs always produce same outputs
- **Memory efficient**: Vectors are deduped and sorted
- **No allocation loops**: Single pass tile generation

## Future Enhancement Ideas

1. **Combo Patterns**: Chain attacks together for extended effects
2. **Elemental Variants**: Burn, freeze, poison status applications
3. **Knockback Patterns**: Push enemies in specific directions
4. **Defensive Patterns**: Shield/barrier creation
5. **Weapon-Specific Variants**: Different looks for sword vs staff
6. **Difficulty Scaling**: Enemy pattern variations by floor difficulty
7. **Custom Animations**: Per-weapon visual customization
8. **Special Enemy Patterns**: Boss-exclusive attack shapes

## Status

✅ 14 unique patterns created
✅ All patterns animated with frames
✅ Full directional support
✅ Color-coded by type
✅ 12 enemies using patterns
✅ All tests passing (30/30)
✅ Zero compilation errors
✅ Integration complete

Ready for game rendering and damage system integration!
