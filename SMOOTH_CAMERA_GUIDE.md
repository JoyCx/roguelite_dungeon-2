# Smooth Camera Implementation

## Overview
The camera now moves smoothly and cinematically when the player moves, similar to ASCIILL. Instead of snapping instantly to follow the player, the camera uses ease-out interpolation to glide towards the target position.

## How It Works

### 1. **Floating-Point Camera Offset**
   - Changed from `camera_offset: (i32, i32)` to `camera_offset: (f32, f32)`
   - This allows sub-pixel positioning during transitions

### 2. **Camera Target**
   - Added `camera_target: (f32, f32)` field
   - When player moves, `update_camera()` calculates where the camera should go
   - This becomes the target for smooth interpolation

### 3. **Smooth Interpolation**
   - New method: `update_camera_smooth()`
   - Moves camera offset towards target at 0.12 ease-out factor per frame
   - At 60 FPS, this provides smooth, cinematic movement
   - When within 0.1 units of target, snaps to exact position to avoid drift

### 4. **Rendering with Floats**
   - Camera offset is converted to integers (floored) only during rendering
   - This keeps visual rendering sharp while maintaining smooth movement

## Configuration

### Adjusting Camera Speed
Edit the `ease_factor` in `update_camera_smooth()` in [app.rs](src/app.rs):

```rust
let ease_factor = 0.12;  // Range: 0.05 (slow) to 0.25 (fast)
```

- **Lower values** (0.05-0.10): Slower, more cinematic movement
- **Higher values** (0.15-0.25): Faster, more responsive camera

### Snap Distance Threshold
Adjust the snap threshold to prevent floating-point drift:

```rust
if dx.abs() > 0.1 || dy.abs() > 0.1 {
    // Smoothly move
} else {
    // Snap to target
}
```

Lower values (0.05) make snapping happen sooner; higher values (0.2) allow more subtle movement.

## Implementation Details

### Key Changes

1. **[src/app.rs](src/app.rs)**
   - Changed camera fields to f32 (floating-point)
   - Modified `update_camera()` to set target, not instant position
   - Added `update_camera_smooth()` for per-frame interpolation

2. **[src/main.rs](src/main.rs)**
   - Calls `update_camera()` to update target
   - Calls `update_camera_smooth()` to interpolate each frame

3. **[src/ui/mod.rs](src/ui/mod.rs)**
   - Floors camera offset when rendering: `camera_offset.0.floor() as i32`

## Performance
- Minimal CPU overhead (one simple interpolation per frame)
- Smooth at 60 FPS with negligible impact on frame time

## Comparison to Previous System
| Aspect | Before | After |
|--------|--------|-------|
| Camera Movement | Instant (snappy) | Smooth (cinematic) |
| Camera Type | Integer position | Floating-point |
| Update Method | Direct assignment | Ease-out interpolation |
| Feel | Robotic | Fluid, game-like |

## Future Enhancements
- Add camera shake on impact
- Different ease functions (ease-in-out, linear, bounce)
- Camera zoom/distance effects
- Lead ahead (camera moves slightly ahead of player direction)
