# Attack Pattern Visual Guide

## 🗡️ Close Combat Patterns

### BasicSlash
```
Frame 1: /          Frame 2: \
  X X                 X X
  | |                 | |
Movement: Quick forward slash with perpendicular coverage
Animation: Yellow → White
Coverage: 3 tiles (1 center + 2 perpendicular)
```

### SwordThrust(3)
```
Frame 1:   >        Frame 2:    >       Frame 3:     >
           X                    X                     X
                                X                     X
                                                      X
Movement: Progressive piercing extension
Animation: Magenta (all frames)
Coverage: Center line + side coverage at reach end
```

### GroundSlam(2)
```
Frame 1:   *        Frame 2:    ~       Frame 3:      ~
         (impact)             * * *                * * * *
                              * * *              * * * * * *
                              * * *              * * * * * *
                                                 * * * * * *
Movement: Expanding diamond shockwave
Animation: Red → Yellow → Light Red
Coverage: Growing diamond pattern
```

### WhirlwindAttack
```
Frame 1:   *        Frame 2:    *       Frame 3:    *        Frame 4:    *
         * * *                * * *               * * *                * * *
         * * *                * * *               * * *                * * *
         * * *                * * *               * * *                * * *
Movement: Rotating through adjacent tiles
Animation: Cyan (spinning)
Coverage: All 8 adjacent tiles
```

### CrescentSlash
```
Frame 1:   (        Frame 2:    )
           X                    X
          X X                 X X
Movement: Curved slash pattern
Animation: Magenta → White
Coverage: Crescent moon shape (5 tiles)
```

---

## 🏹 Ranged Patterns

### ArrowShot(4)
```
Frame 1:   ^        Frame 2:    ^        Frame 3:     ^       Frame 4:      ^
           X                   X X                   X X X                  X X X X
Movement: Projectile travels with trail
Animation: Yellow
Coverage: Linear with 1-2 tile trail
```

### MultiShot(4, 2)
```
Frame 1:     ^      Frame 2:      ^       Frame 3:      ^       Frame 4:        ^
             X                   X X                  X X X                    X X X X X
             X X                X X X              X X X X X                X X X X X X X
Movement: Three arrows spreading in fan
Animation: Light Yellow
Coverage: Center + left/right spread (grows by distance)
```

### Barrage(3)
```
Frame 1:   !        Frame 2:    !        Frame 3:     !
           X                    X X                    X X X
Movement: Rapid successive hits
Animation: Light Red (0.02s per frame = fastest)
Coverage: Sequential single tiles (machine gun effect)
```

### PiercingShot(3)
```
Frame 1:   »        Frame 2:    »        Frame 3:     »
           X                    X                      X
                               X X                     X X
                                                       X X X
Movement: Armor-piercing projectile with trail
Animation: White
Coverage: Linear with extended trail (2-3 tiles)
```

---

## ✨ Magical Patterns

### Fireball(2)
```
Frame 1:   *        Frame 2:      *
           X               * * * *
                          * * * *
                           * * *
Movement: Expanding explosion rings
Animation: Red → Yellow (grows larger)
Coverage: Circular area using distance formula
```

### ChainLightning(3)
```
Frame 1:   ≈        Frame 2:    ≈        Frame 3:      ≈
           X                    X X                    X X X
                                 X                      X X
                                                         X
Movement: Zigzag electrical chain
Animation: Cyan
Coverage: Forward line with perpendicular offset
```

### FrostNova(2)
```
Frame 1:       *    Frame 2:        *
             * * *                * * * *
             * * *              * * * * * *
             * * *              * * * * * *
                                 * * * * *
Movement: Expanding frost diamond
Animation: Blue
Coverage: Diamond pattern (cardinal + diagonal)
```

### MeteorShower(2, 2)
```
Frame 1:   ◆        Frame 2:    ◆ ◆ ◆
           X                    X X X
                               X X X X
                               X X X X
Movement: Meteors rain with expanding impact
Animation: Red
Coverage: Forward line with impact area width
```

### Vortex(2)
```
Frame 1:   ○        Frame 2:    ○       (spirals inward)
         ○ ○ ○                ○ ○ ○ ○ ○
         ○   ○              ○ ○     ○ ○
         ○ ○ ○                ○ ○ ○ ○ ○
Movement: Spiraling vortex pulling inward
Animation: Magenta
Coverage: Circular spiral pattern
```

---

## 🎨 Animation Frame Distribution

### Quick Patterns (2 frames)
- BasicSlash: / → \
- CrescentSlash: ( → )
- Fast visual feedback, immediate impact

### Progressive Patterns (Variable frames)
- ArrowShot: 4-6 frames showing travel
- SwordThrust: 3-5 frames showing extension
- Barrage: 3-5 frames rapid fire
- ChainLightning: 3-5 frames zigzag
- FrostNova: 2-4 frames expanding
- Gradual buildup, delayed damage

### Expanding Patterns (Ring-based)
- GroundSlam: Expanding diamond rings
- Fireball: Expanding circular rings
- Vortex: Spiraling rings
- Visual complexity increases with size

### Rotating Patterns (Directional)
- WhirlwindAttack: 4 frames rotating through tiles
- 360° coverage, constant animation

---

## 📏 Coverage Patterns by Shape

### Linear (Straight Line)
```
ArrowShot      SwordThrust    Barrage        PiercingShot
    ↓              ↓            ↓              ↓
    X              X            X              X
    X              X X          X              X X X
    X              X X X        X              X X X X
    X              X X X X      X
```

### Cone (Forward + Spread)
```
MultiShot(reach=3, spread=2)
    X
  X X X
X X X X X
```

### Diamond (Expanding Outward)
```
GroundSlam(2)      FrostNova(2)
    *                  *
   * * *              * * *
  * * * *            * * * *
   * * *              * * *
    *                  *
```

### Circular (Around Origin)
```
Fireball(1)        Fireball(2)        Vortex(2)
  * * *            * * * * *          ○ ○ ○ ○ ○
  * * *          * * * * * * *      ○ ○   ○ ○
  * * *          * * * * * * *      ○ ○   ○ ○
                 * * * * * * *      ○ ○   ○ ○
                   * * * * *          ○ ○ ○ ○ ○
```

### Special Patterns
```
CrescentSlash      ChainLightning     WhirlwindAttack
  (curved)         (zigzag)           (8-directional)
   X                 X                  * * *
  X X               X                   * * *
   X               X X                  * * *
                    X
```

---

## 🎯 Direction Reference

```
        UP (0, -1)
             ↑
             |
LEFT ← (-1,0)●(1,0) → RIGHT
             |
             ↓
        DOWN (0, 1)

Diagonals:
  (-1,-1)↖  ↗(1,-1)
    ↙ ↘
  (-1,1)  (1,1)

All patterns respect these direction vectors!
```

---

## 📊 Pattern Selection by Enemy Type

### Physical Enemies (Zombies, Skeletons)
- BasicSlash - Quick attack
- SwordThrust - Heavy attack
- GroundSlam - AoE attack
- WhirlwindAttack - Defense

### Ghost Enemies (Ethereal)
- ChainLightning - Magical strike
- FrostNova - Debuff area

### Ranged Enemies (Archers, Mages)
- ArrowShot - Single projectile
- MultiShot - Spread pattern
- Barrage - Rapid fire
- PiercingShot - Strong single

### Boss Enemies (High Tier)
- Vortex - Crowd control
- MeteorShower - Massive AoE
- Fireball - Large explosion
- Multiple patterns combined

---

## ⚡ Animation Speed Reference

Frame Duration Values:
```
0.02s ─ Barrage (fastest, rapid fire effect)
0.03s ─ ArrowShot, PiercingShot (projectile travel)
0.04s ─ MultiShot (fan spread)
0.05s ─ ChainLightning, MeteorShower, Vortex
0.06s ─ WhirlwindAttack (spinning rotation)
0.07s ─ FrostNova (gradual expansion)
0.08s ─ GroundSlam (shockwave)
0.10s ─ BasicSlash, SwordThrust, CrescentSlash (impact attacks)

Slower = More dramatic, longer display
Faster = Rapid feedback, immediate effect
```

---

## 🎮 In-Game Integration Timeline

### Attack Initiation
1. Player presses attack button
2. Determine pattern based on weapon/ability
3. Get direction from player facing
4. Call `get_animation_frames()`

### Animation Rendering Loop
```
For each frame in pattern animation:
  For each tile in frame:
    Draw symbol at (x,y) with color
  Sleep for frame_duration
  (render loop continues)
```

### Damage Application
```
After all animation frames complete:
  Get final affected_tiles = get_affected_tiles()
  For each tile:
    If not wall:
      Apply damage at tile
      Apply status effects if any
```

### Cooldown Management
```
When attack completes:
  cooldown_remaining = cooldown_duration
  While cooldown_remaining > 0:
    cooldown_remaining -= delta_time
    Display cooldown bar/icon
```

---

## 🔧 Customization Examples

### Creating a Weapon Variant
```rust
// Original
let fireball = AttackPattern::Fireball(3);

// Weapon-specific variant (visual only)
fn render_fire_variant(pattern: &AttackPattern, frames: &[AnimationFrame]) {
    // Draw with different symbols/colors
    // Same damage calculation
}
```

### Custom Damage Modifier
```rust
let pattern = AttackPattern::SwordThrust(4);
let base_damage = 15;

// Pattern affects reach, not damage directly
let affected_tiles = pattern.get_affected_tiles(x, y, 1, 0);
for tile in affected_tiles {
    let distance = calculate_distance(&tile);
    let damage = base_damage * (1.0 - distance as f32 * 0.1); // Falloff
    apply_damage(&tile, damage);
}
```

### Elemental Application
```rust
let pattern = AttackPattern::Fireball(3);
let tiles = pattern.get_affected_tiles(x, y, 0, 1);

for (tx, ty) in tiles {
    apply_damage(tx, ty, 20);
    apply_status_effect(tx, ty, StatusEffect::Burn(5.0)); // 5 second burn
}
```

---

This visual guide can be printed or referenced while implementing rendering code!
