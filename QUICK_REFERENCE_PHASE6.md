# Quick Reference: Phase 6 Features

## Enemy Health & Gold System

### How It Works

When enemies spawn on a floor:
1. They're assigned a rarity tier (Fighter/Guard/Champion/Elite/Boss)
2. Their health is set based on rarity (10-120 HP)
3. Their gold drop amount is calculated:
   - **Base Gold** = Rarity tier specific (10, 15, 25, 50, 150)
   - **Final Gold** = Base Gold × Difficulty Multiplier

### Gold Drop Values

```
Difficulty Multipliers:
├─ Easy:   1.0x (base values)
├─ Normal: 1.5x 
├─ Hard:   2.0x
└─ Death:  3.0x

Example - Fighter enemy:
├─ Easy:   10 gold
├─ Normal: 15 gold  
├─ Hard:   20 gold
└─ Death:  30 gold

Example - Boss enemy:
├─ Easy:   150 gold
├─ Normal: 225 gold
├─ Hard:   300 gold
└─ Death:  450 gold
```

### Enemy Health by Rarity

| Rarity | HP Range | Example Enemy |
|--------|----------|---------------|
| Fighter | 10-15 | Rotting Footsoldier (15 HP) |
| Guard | 14-20 | Crypt Sentinel (20 HP) |
| Champion | 30-35 | Blight Captain (35 HP) |
| Elite | 45-55 | Corpse Abomination (55 HP) |
| Boss | 100-120 | Ossuary King (120 HP) |

## Dev Menu Quick Start

### Accessing Dev Menu
1. Start the game
2. At main menu, select **"Dev Menu"** (option 4)

### Available Commands

| Key | Action | Effect |
|-----|--------|--------|
| `R` | Random Seed | Generates a random floor seed |
| `0-9` | Manual Seed | Type a number to manually set seed |
| `BACKSPACE` | Delete | Remove last digit from seed |
| `ENTER` | Generate | Create floor with current/entered seed |
| `E` | Spawn Enemy | Add a test enemy to the current floor |
| `D` | Damage Test | Deal 5 damage to all enemies |
| `G` | Add Gold | Add 50 gold to player inventory |
| `P` | Play | Enter gameplay on generated floor |
| `ESC` | Back | Return to main menu |

### Typical Workflow

```
1. Press R to generate random seed
2. Press ENTER to generate the floor
3. Press E multiple times to spawn test enemies
4. Press D to test damage on enemies
5. Press G to add test gold
6. Press P to enter play mode
7. Test mechanics in actual gameplay
8. Press ESC to return to dev menu
```

## Dev Menu Display

### Information Shown

**Top Panel:**
- Current seed number
- Number of enemies on floor
- Player's current gold
- Current difficulty level
- Player HP and equipped weapon

**Map Panel (Left Side):**
- Visual representation of dungeon
- Shows walls, floors, items, and enemies

**Enemy List Panel (Right Side):**
- Number of enemies alive
- For each enemy:
  - Index number
  - Rarity (color-coded)
  - Health bar with current/max HP
  - Gold drop amount

### Enemy Rarity Colors

- **Gray** = Fighter (weakest)
- **Green** = Guard
- **Cyan** = Champion
- **Magenta** = Elite  
- **Red** = Boss (strongest)

## Testing Scenarios

### Test 1: Verify Gold Calculation
1. Generate floor with Easy difficulty
2. Spawn a Champion enemy (press E)
3. Note it shows 25 gold in the list
4. Change to Death difficulty (if available)
5. Spawn another Champion
6. Verify it shows 75 gold (25 × 3.0)

### Test 2: Enemy Damage System
1. Spawn multiple enemies (E key)
2. Press D to damage all (5 HP each)
3. Watch their health bars decrease
4. Press D multiple times to kill them
5. Verify they show 0 HP when dead

### Test 3: Gold Accumulation
1. Press G multiple times (adds 50 each time)
2. Watch player gold increase in top panel
3. Verify gold display updates in real-time

### Test 4: Difficulty Scaling
1. Generate floor at Easy difficulty
2. Note enemy gold amounts
3. Generate new floor at Hard difficulty
4. Spawn same enemy type
5. Verify gold is correctly 2x higher

## Common Issues & Solutions

### Enemy Won't Spawn
- Floor may be full of items/walls
- Try with a different seed (press R)
- Or clear some items first (generate new floor)

### Can't See Enemies
- Enemy list only shows first 20 enemies
- Scroll limit is intentional for UI space
- "... and X more" message shows remaining

### Gold Amount Seems Wrong
- Gold is calculated: base_gold × difficulty_multiplier
- Easy is always 1.0x multiplier
- Check current difficulty in top panel

## Code Integration Points

### Where Gold Drops Would Be Used
Currently, enemies show their gold drop amount in the dev menu. For full integration:

```rust
// When enemy dies in gameplay:
let gold_drop = enemy.base_gold;
player_gold += gold_drop;

// Item drop (for later):
let item = ItemDrop::gold(enemy.position.x, enemy.position.y, gold_drop);
floor.add_item(item);
```

### Where Health Is Used
```rust
// Taking damage:
if enemy.take_damage(damage) {
    // Enemy survived
} else {
    // Enemy is dead (health <= 0)
    // Remove from floor, drop items, etc.
}

// Checking if alive:
if enemy.is_alive() {
    // Continue combat
}
```

## Performance Notes

- Health tracking: O(1) per enemy
- Gold calculation: O(1) at spawn time
- Dev menu: Zero impact on gameplay FPS
- No memory leaks: Proper struct ownership

## Future Enhancements

Coming in Phase 7:
- [ ] Gold items drop on enemy death
- [ ] Death animations and effects
- [ ] Dead enemy removal from floor
- [ ] Damage number popups
- [ ] Combat feedback sounds

Coming in Phase 8+:
- [ ] Enemy scaling by floor depth
- [ ] Dynamic difficulty adjustments
- [ ] Loot tables by enemy type
- [ ] Gold spending mechanics
