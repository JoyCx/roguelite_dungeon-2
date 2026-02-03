# Attack Patterns System

## Overview

The game now features a comprehensive **12+ unique attack patterns** with full animation support, directional awareness, and color-coded visual effects. Each pattern has specific reach limitations, distinct visual animations, and proper collision handling.

## Pattern Categories

### 🗡️ Close Combat Patterns (Melee/Sword-Based)

#### 1. **BasicSlash**
- **Description**: Quick 3-tile slash with wide coverage
- **Type**: Melee
- **Animation**: 2-frame slash effect (Yellow → White)
- **Coverage**: Attacker position + perpendicular width
- **Color**: Yellow → White
- **Symbol**: `/` and `\`
- **Best For**: Quick basic attacks, low cooldown abilities
- **Enemy Usage**: Footsoldier, primary attack pattern

#### 2. **SwordThrust**
- **Description**: Pierce forward with distance reach
- **Type**: Melee
- **Parameters**: `reach` (distance tiles)
- **Animation**: Progressive extension from 1 to reach tiles
- **Coverage**: Center line + side coverage at end
- **Color**: Magenta
- **Symbol**: `>`
- **Best For**: Piercing through enemies, directional power attacks
- **Enemy Usage**: Sentinel, Watcher

#### 3. **GroundSlam**
- **Description**: Shockwave expands outward from player in all directions
- **Type**: Melee/Magic Hybrid
- **Parameters**: `reach` (expansion distance)
- **Animation**: Impact at center, then expanding diamond rings
- **Coverage**: Diamond pattern expanding outward
- **Color**: Red → Yellow → Light Red
- **Symbol**: `*` then `~`
- **Best For**: Area denial, crowd control, ultimate ability
- **Enemy Usage**: Doorwarden (primary), Ossuary King

#### 4. **WhirlwindAttack**
- **Description**: Spin attack hitting all 8 directions
- **Type**: Melee
- **Animation**: 4-frame spinning pattern rotating through all tiles
- **Coverage**: All 8 adjacent tiles (3x3 minus center)
- **Color**: Cyan
- **Symbol**: `*`
- **Best For**: 360° protection, boss patterns
- **Enemy Usage**: Grave Scrabbler, Blight Captain, Veilbound Duelist

#### 5. **CrescentSlash**
- **Description**: Curved slash with arcing pattern
- **Type**: Melee
- **Animation**: 2-frame curved attack
- **Coverage**: Crescent moon shape relative to direction
- **Color**: Magenta → White
- **Symbol**: `(` and `)`
- **Best For**: Stylish attacks, curved movement patterns
- **Enemy Usage**: Advanced melee enemies

---

### 🏹 Ranged Patterns (Projectile-Based)

#### 6. **ArrowShot**
- **Description**: Single arrow projectile traveling in straight line
- **Type**: Ranged
- **Parameters**: `reach` (distance tiles)
- **Animation**: Progressive movement from origin with trail
- **Coverage**: Linear path with 1-2 tile trail
- **Color**: Yellow
- **Symbol**: `^`
- **Best For**: Single target ranged attacks, basic projectiles
- **Enemy Usage**: Shade, various ranged enemies

#### 7. **MultiShot**
- **Description**: 3 arrows spreading in fan pattern
- **Type**: Ranged
- **Parameters**: `reach` (distance), `spread` (width at end)
- **Animation**: Progressive spread from distance 2+
- **Coverage**: Center arrow + left/right spread
- **Color**: Light Yellow
- **Symbol**: `^`
- **Best For**: Area coverage, spray attacks
- **Enemy Usage**: Watchers, ranged bosses

#### 8. **Barrage**
- **Description**: Rapid successive hits along a line
- **Type**: Ranged
- **Parameters**: `reach` (total distance)
- **Animation**: Single frame per distance tile (rapid fire)
- **Coverage**: Individual tiles along line
- **Color**: Light Red
- **Symbol**: `!`
- **Frame Duration**: 0.02s (fastest animation)
- **Best For**: Machine gun style attacks, rapid fire
- **Enemy Usage**: Lantern Haunt (rapid projectile spam)

#### 9. **PiercingShot**
- **Description**: Armor-piercing shot with strong visual trail
- **Type**: Ranged
- **Parameters**: `reach` (distance)
- **Animation**: Progressive with 2-3 tile trail
- **Coverage**: Linear with lingering trail effect
- **Color**: White
- **Symbol**: `»`
- **Best For**: Penetrating attacks, strong single projectiles
- **Enemy Usage**: Crypt Sentinel, armored enemies

---

### ✨ Magical Patterns (Spell-Based)

#### 10. **Fireball**
- **Description**: Expanding explosion with growing radius
- **Type**: Magic
- **Parameters**: `radius` (explosion size)
- **Animation**: Expanding rings from 1 to radius
- **Coverage**: Circular area with expanding rings
- **Color**: Red → Yellow (color gradient by size)
- **Symbol**: `*`
- **Best For**: Area damage, explosion effects
- **Enemy Usage**: Blight Captain (secondary), impact-based attacks

#### 11. **ChainLightning**
- **Description**: Lightning bolts chain in zigzag pattern
- **Type**: Magic (Electric)
- **Parameters**: `reach` (chain distance)
- **Animation**: Progressive zigzag advancement
- **Coverage**: Forward line with perpendicular offset every step
- **Color**: Cyan
- **Symbol**: `≈`
- **Best For**: Chaining effects, magical tracking
- **Enemy Usage**: Whispering Shade, electrical enemies

#### 12. **FrostNova**
- **Description**: Ice spreads outward in diamond/cross pattern
- **Type**: Magic (Frost)
- **Parameters**: `reach` (expansion distance)
- **Animation**: Expanding diamond rings
- **Coverage**: Diamond pattern (4 cardinal directions + diagonals)
- **Color**: Blue
- **Symbol**: `*`
- **Best For**: Freeze effects, ice magic
- **Enemy Usage**: Grave Scrabbler alternative, elemental attacks

#### 13. **MeteorShower**
- **Description**: Meteors rain from above in impact area
- **Type**: Magic (Physical Impact)
- **Parameters**: `reach` (distance forward), `width` (impact area)
- **Animation**: Progressive forward advancement with impact areas
- **Coverage**: Forward line with impact radius at each step
- **Color**: Red
- **Symbol**: `◆`
- **Best For**: Catastrophic area effects, large radius magic
- **Enemy Usage**: Mourning Bell (ultimate-level), boss patterns

#### 14. **Vortex**
- **Description**: Magical vortex pulling inward with spiral effect
- **Type**: Magic (Void/Dark)
- **Parameters**: `radius` (vortex size)
- **Animation**: Spiraling inward from outside to center
- **Coverage**: Circular spiral pattern
- **Color**: Magenta
- **Symbol**: `○`
- **Best For**: Crowd control, pulling effects, boss attacks
- **Enemy Usage**: Ossuary King (secondary), advanced magic

---

## Technical Features

### Directional Awareness
All patterns respect attack direction vectors:
- `dir_x`, `dir_y` in range [-1, 0, 1]
- Proper 8-directional support
- Perpendicular calculation for spread/width
- Cardinal directions for cross/diamond patterns

### Animation System
Each pattern includes:
- **Multiple frames** for smooth animation
- **Color gradients** showing progression
- **Unique symbols** for visual distinction
- **Frame duration** control (0.02s - 0.1s typical)

### Reach & Collision
- **Reach limitations**: Each pattern has defined maximum distance
- **Circular calculations**: Proper distance formula for AOE patterns
- **Overlap prevention**: Deduplication of tile positions
- **Wall collision**: Respects level layout when applied

### Color Palette
- 🟡 **Yellow/Light Yellow**: Ranged, strikes, basic attacks
- 🔴 **Red/Light Red**: Impact, damage, fire effects
- 🟦 **Blue**: Frost, ice, cold magic
- 🟪 **Cyan**: Lightning, electricity, chains
- 🟣 **Magenta**: Darkness, void, curved attacks
- ⚪ **White**: Pure force, piercing, ultimate effects

### Symbol Meanings
- `*` - Explosion, impact, magical effect
- `^` - Arrow, projectile direction
- `/` `\` - Slash direction
- `>` - Forward thrust, piercing
- `~` - Shockwave, ripple effect
- `≈` - Lightning, zigzag pattern
- `○` - Vortex, spiral pattern
- `◆` - Meteor, falling projectile
- `!` - Rapid fire, staccato effect
- `(` `)` - Curved slash
- `»` - Piercing projectile

---

## Integration with Game Systems

### Enemy Usage
All 12 enemy templates use these patterns:
- **Footsoldier**: BasicSlash
- **Grave Scrabbler**: WhirlwindAttack (main), FrostNova (alternate)
- **Whispering Shade**: ChainLightning
- **Crypt Sentinel**: PiercingShot
- **Watcher**: SwordThrust
- **Doorwarden**: GroundSlam
- **Blight Captain**: Fireball, WhirlwindAttack
- **Veilbound Duelist**: WhirlwindAttack
- **Corpse Abomination**: SwordThrust, Barrage
- **Lantern Haunt**: Barrage (rapid fire)
- **Ossuary King**: Fireball, Vortex
- **Mourning Bell**: MeteorShower (ultimate-level)

### Player Integration
Patterns can be assigned to:
- Weapon special abilities
- Magic spells
- Ultimate abilities
- Different weapon types (sword, staff, bow)

### Cooldown System
Each attack has:
- `cooldown_duration`: Time between uses (seconds)
- `cooldown_remaining`: Current countdown timer
- `is_available()`: Check if ready to use
- `update_cooldown(delta_time)`: Reduce remaining cooldown

---

## Pattern Statistics

| Pattern | Type | Reach/Radius | Animation Frames | Color | Best Used |
|---------|------|-------------|------------------|-------|-----------|
| BasicSlash | Melee | 1 | 2 | Yellow | Quick attacks |
| SwordThrust | Melee | Variable | Variable | Magenta | Power attacks |
| GroundSlam | Melee | Variable | Expanding | Red | Ultimate/AoE |
| Whirlwind | Melee | 1 | 4 | Cyan | 360° defense |
| CrescentSlash | Melee | 2 | 2 | Magenta | Curved hits |
| ArrowShot | Ranged | Variable | Variable | Yellow | Single target |
| MultiShot | Ranged | Variable | Variable | Light Yellow | Fan attack |
| Barrage | Ranged | Variable | Variable | Light Red | Rapid fire |
| PiercingShot | Ranged | Variable | Variable | White | Strong pierce |
| Fireball | Magic | Variable | Variable | Red | Area explosion |
| ChainLightning | Magic | Variable | Variable | Cyan | Chain effect |
| FrostNova | Magic | Variable | Variable | Blue | Frost spread |
| MeteorShower | Magic | Variable | Variable | Red | Large AoE |
| Vortex | Magic | Variable | Variable | Magenta | Crowd control |

---

## Usage Example

```rust
// Create an attack pattern
let pattern = AttackPattern::Fireball(3);

// Get animation frames for visualization
let frames = pattern.get_animation_frames(player_x, player_y, dir_x, dir_y);

// Render all frames
for frame in frames {
    println!("Color: {:?}, Symbol: {}", frame.color, frame.symbol);
    for (x, y) in &frame.tiles {
        // Draw at position
    }
}

// Get final affected tiles for damage calculation
let damage_tiles = pattern.get_affected_tiles(player_x, player_y, dir_x, dir_y);

// Apply damage to all tiles
for (x, y) in damage_tiles {
    apply_damage_at(x, y);
}
```

---

## Animation Principles

1. **Progression**: Attacks build from origin outward
2. **Color Coding**: Visual feedback for attack type
3. **Speed Variation**: Frame duration reflects attack speed
4. **Overlap Prevention**: No duplicate tiles in effects
5. **Direction Respect**: All effects follow attack direction
6. **Wall Awareness**: Can be combined with collision detection

---

## Future Expansion Ideas

- Combo patterns (chain attacks together)
- Elemental effects (burn, freeze, poison)
- Knockback patterns (push enemies away)
- Defensive patterns (shields, barriers)
- Weapon-specific variants
- Enemy pattern variations by difficulty
- Animation customization per weapon
- Pattern randomization for special enemies

