# ASCII Filter Overlay System

## Overview
A visual enhancement system that overlays random ASCII characters on attack pattern animations while maintaining original colors. This creates distinct visual identities for different attack types without changing the underlying color scheme.

## Categories & Character Sets

### 1. **Close Combat** `[/, \, |, -, +, X, *]`
Patterns: BasicSlash, GroundSlam, WhirlwindAttack, SwordThrust, CrescentSlash

Characters represent melee strikes and physical impacts:
- `/` and `\` - Slashing motions
- `|` and `-` - Cross strikes  
- `+` and `X` - Impact zones
- `*` - Area effects

### 2. **Ranged Combat** `[>, <, ^, v, →, ←, ↑]`
Patterns: ArrowShot, MultiShot, Barrage, PiercingShot

Characters represent projectile motion and direction:
- `>`, `<`, `^`, `v` - Simple directional indicators
- `→`, `←`, `↑` - Arrow-like directional symbols
- Creates visual flow showing arrow/projectile paths

### 3. **Magic** `[~, §, ¤, ✦, ◆, ●, #]`
Patterns: Fireball, ChainLightning, FrostNova, MeteorShower, Vortex

Characters represent magical energy and supernatural effects:
- `~` - Wave/arcane energy
- `§` and `¤` - Magical symbols
- `✦` and `◆` - Enchanted/mystical sparkles
- `●` - Energy orb/core
- `#` - Chaotic magical field

## Implementation Details

### AnimationCategory Enum
```rust
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum AnimationCategory {
    CloseCombat,
    RangedCombat,
    Magic,
}
```

### Character Randomization
- Each frame randomly selects a character from its category set
- Characters are chosen via `AnimationCategory::get_random_character()`
- Different characters appear every frame for organic, dynamic visuals
- Colors remain unchanged - only symbols vary

### ActiveAnimation Enhancement
```rust
pub struct ActiveAnimation {
    pub frames: Vec<AnimationFrame>,
    pub current_frame_idx: usize,
    pub timer: f32,
    pub category: AnimationCategory,  // NEW: Category for ASCII filtering
}
```

### Pattern Classification
The system automatically classifies attack patterns:
```rust
pub fn get_attack_pattern_category(pattern: &AttackPattern) -> AnimationCategory
```

Maps each pattern to appropriate category based on mechanics.

## How It Works

1. **Animation Creation**: When attack/trigger_dev_animation() creates frames
2. **Category Assignment**: Pattern is automatically classified into one of 3 categories
3. **Animation Rendering**: Each frame uses a random character from its category
4. **Color Preservation**: Original frame colors (Red, Cyan, Yellow, etc.) are maintained
5. **Visual Result**: Dynamic ASCII overlay that changes every frame while keeping consistent colors

## Example Visuals

### Close Combat (Red color, varying symbols)
```
/\/\/\
|X X|
\/\/\/
```

### Ranged Combat (Yellow color, arrow symbols)
```
>>>>>>>>
<<>>>>>
^^^^^v^v
```

### Magic (Cyan color, sparkle symbols)
```
~~✦~~
§◆◆§
~#~#~
```

## Files Modified

1. **src/app.rs**
   - Added `AnimationCategory` enum and impl
   - Enhanced `ActiveAnimation` struct with category field
   - Added `get_attack_pattern_category()` classifier function
   - Updated `attack()` and `trigger_dev_animation()` to use categories

2. **src/ui/drawing.rs**
   - Enhanced `render_animations()` to use random ASCII characters
   - Characters selected from category sets with preserved colors

## Benefits

- **Visual Distinction**: Instantly identify attack type from animation style
- **Dynamic Feel**: Random characters create organic, changing visuals
- **Color Consistency**: Original color scheme remains for gameplay clarity
- **Immersive**: Makes animations feel more alive and varied
- **Accessible**: Enhanced visual feedback without overwhelming changes
