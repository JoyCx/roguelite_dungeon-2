# Roguelite Dungeon - Feature Usage Guide

## Weapon Inventory System (Slots 1-9)

### Finding Weapons
- Weapons drop from defeated enemies
- Walk over weapons to automatically pick them up
- Weapons have colored glyphs based on their rarity tier
- Weapons glow with sparkle effects to attract attention

### Weapon Tiers and Glyphs
Different weapon types have distinct appearances:

**Swords:**
- Common: `s` (small)
- Uncommon: `S` (capital)
- Rare: `⚔` (crossed swords)
- Epic: `⚡` (lightning)
- Legendary: `✦` (star)
- Mythic: `§` (section symbol)
- Godly: `◈` (double diamond)

**Bows:**
- Common: `b` → Godly: `◉` (similar progression)

**Maces:**
- Common: `m` → Godly: `⊙` (similar progression)

### Managing Your Arsenal
- **Switch Weapons:** Press `1-9` to equip weapon in that slot
- **Drop Weapon:** Press `Ctrl+1` through `Ctrl+9` to drop the weapon in that slot
- **Full Inventory:** If you carry 9 weapons, you can't pick up new ones
- **Smart Drop:** If inventory is full but you pick up a weapon, it's automatically dropped back on the ground

### Using Weapons
Once equipped, use your weapon with your normal attack key (usually Click or Spacebar depending on your keybindings)

---

## Enemy Detection and Combat

### How Enemies Detect You
Enemies don't immediately notice you - they have a detection radius:

**Detection Radius by Enemy Type:**
- **Fighter** (ƒ): 5 tiles - smallest, least awareness
- **Guard** (Ψ): 6 tiles
- **Champion** (◆): 8 tiles
- **Elite** (§): 10 tiles
- **Boss** (❋): 15 tiles - can see you from far away!

**Difficulty Modifiers:**
- Easy: 70% of base radius (enemies less aware)
- Normal: 100% baseline
- Hard: 140% of base radius
- Death: 180% of base radius (enemies hyper-aware)

### Engaging in Combat
- Enemies only chase you when you're within their detection radius
- Knockback from your attacks pushes enemies away
- Enemies knocked back will lose momentum and slow down
- When damaged, both you and enemies flash red for 1 second
- Use this visual feedback to track when hits land

### Enemy Visual Markers
Each enemy type has a unique symbol:
- `ƒ` = Fighter (weakest, gray color)
- `Ψ` = Guard (stronger, green color)
- `◆` = Champion (rare, cyan color)
- `§` = Elite (powerful, magenta color)
- `❋` = Boss (most dangerous, red color)

Boss enemies also show sparkle effects around them, warning of danger.

---

## Visual Design and Environment

### Dungeon Walls
Walls are represented with a variety of characters for visual interest:
- `█`, `▓`, `▒`, `▀`, `▄`, `■`, `◆`, `◊`
- Pattern is consistent (same coordinates always use same character)
- No two adjacent walls look identical (random selection)
- Creates more engaging dungeon visual without being overwhelming

### Item Identification by Color
Items drop with colors based on their rarity:
- **Gray:** Common items (worthless, basic)
- **Blue:** Rare items (valuable)
- **Magenta:** Epic items (strong)
- **Yellow:** Exotic items (special)
- **Light Yellow:** Legendary items (rare and powerful)
- **Cyan:** Mythic items (extremely rare)
- **Red:** Godly items (best in game)

### Visual Feedback
- **Sparkles:** Bright weapons and bosses emit sparkle particles (`*`, `✦`, `✧`)
- **Red Flash:** Entities flash red when taking damage (1 second animation)
- **Impact:** Knockback causes visible movement away from attacker

---

## Strategy Tips

### Combat Strategy
1. **Stay Mobile:** Use the detection radius to your advantage
   - Keep enemies at range when possible
   - Lure them into traps or environmental hazards
   - Use distance to avoid overwhelming numbers

2. **Watch for Knockback:** 
   - High-damage weapons create more knockback
   - Use knockback to position enemies or create distance
   - Don't chase knocked-back enemies into walls

3. **Enemy Type Awareness:**
   - Fighters are easiest; can handle multiple
   - Champions and above require caution
   - Bosses (❋) should be approached with full health and weapons

### Loot Management
1. **Inventory Planning:**
   - Keep your most-used weapons in early slots (1-3)
   - Save slots for high-rarity drops
   - Drop low-tier weapons to make room

2. **Identify Upgrades:**
   - Rare weapons (colored blue) are generally better than common
   - Epic (magenta) and above are worth carrying
   - Compare damage values to make sure it's actually an upgrade

3. **Weapon Slots:**
   - Slot 1: Primary weapon (most comfortable/used)
   - Slot 2-3: Secondary weapons (special situations)
   - Slot 4-9: Saved for special drops or experiments

---

## Keyboard Controls Summary

### Movement & Combat
- Arrow Keys or WASD: Move
- Space/Click: Attack with equipped weapon
- Q: Dash (if available)
- F: Block (if available)
- R: Use Ultimate ability (if charged)

### Inventory Management
- I: Toggle inventory view
- 1-9: Switch to weapon in that slot
- Ctrl+1 to Ctrl+9: Drop weapon from that slot
- Shift+1 to Shift+9: Use consumable in that slot
- Up/Down Arrow (in inventory): Scroll through items
- Space (in inventory): Describe selected item

### Game Control
- Esc: Pause/Menu
- Tab: Toggle settings

---

## Visual Cues at a Glance

### What You'll See
| Visual | Meaning |
|--------|---------|
| `@` Yellow | You (player) |
| `ƒ` Gray | Weak enemy (Fighter) |
| `Ψ` Green | Moderate enemy (Guard) |
| `◆` Cyan | Dangerous enemy (Champion) |
| `§` Magenta | Very dangerous (Elite) |
| `❋` Red | Extreme danger (Boss) |
| Sparkle `✦` | Boss or valuable item nearby |
| Red Flash | Something taking damage |
| Knockback Motion | Impact from attack |
| `s` to `◈` Colored | Weapon on ground (color = rarity) |
| `¤` Colored | Gold/currency drop |

---

## Difficulty Scaling

### How Difficulty Affects Gameplay

**Easy Mode:**
- Enemies detect you at 70% normal range
- Great for learning the game
- Loot has fewer rare items (50% drop rate)

**Normal Mode:**
- Standard enemy detection
- Balanced loot distribution
- Recommended first playthrough

**Hard Mode:**
- Enemies detect you at 140% range
- More rare and epic items drop
- Requires better strategy

**Death Mode:**
- Enemies detect you at 180% range (massive!)
- Constant pressure from enemies
- Legendary items drop more frequently
- Severe punishment for mistakes

---

## Advanced Tactics

### Kiting (Hit and Run)
1. Attack an enemy to initiate combat
2. Back away to use their detection radius against them
3. Re-engage when they approach
4. Repeat for safe combat

### Positioning
- Use walls to break line-of-sight
- Push enemies into corners with knockback
- Maintain distance from multiple enemies
- Use the dungeon layout to your advantage

### Resource Management
- Keep health items in easy-access slots
- Maintain weapon variety for different situations
- Don't waste inventory on low-damage weapons
- Plan ahead for boss encounters

---

## Troubleshooting

### "I can't pick up a weapon!"
- Your inventory is full (maximum 9 weapons)
- Solution: Drop a weaker weapon with Ctrl+number, then pick up the new one

### "Enemy isn't attacking me!"
- You're outside their detection radius
- Solution: Move closer to the enemy to engage

### "Weapon dropped disappeared!"
- There was no adjacent empty space
- Solution: Move to a more open area before dropping

### "Visual seems too busy/chaotic"
- Many particle effects and sparkles are rendering
- This is intentional for visual feedback
- The ASCII variety is designed to be rich, not overwhelming

---

Enjoy your adventure in the dungeon!
