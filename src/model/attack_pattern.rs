use ratatui::style::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq)]
pub struct AnimationFrame {
    pub tiles: Vec<(i32, i32)>,
    pub color: Color,
    pub symbol: char,
    pub frame_duration: f32, // seconds
}

/// Unified attack pattern system for both player and enemies
/// 12+ unique patterns covering melee, ranged, and magical attacks
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum AttackPattern {
    /// Classic melee slash - 3x3 centered around attacker
    BasicSlash,

    /// Ground slam - Creates shockwave expanding outward from player
    GroundSlam(i32), // reach: how many tiles the shockwave travels

    /// Spinning attack - hits in rotating pattern, all 8 directions
    WhirlwindAttack,

    /// Forward thrust - pierces through multiple enemies in a line
    SwordThrust(i32), // reach: distance of the thrust

    /// Single projectile - straight line attack
    ArrowShot(i32), // reach: distance the arrow travels

    /// Multishot - arrows spread in a fan pattern
    MultiShot(i32, i32), // (reach, spread_width): distance and spread

    /// Barrage - rapid hits in a line with staggered tiles
    Barrage(i32), // reach: total distance covered

    /// Piercing shot - goes through walls/obstacles further
    PiercingShot(i32), // reach: distance, ignores obstacles

    /// Fireball - circular explosion with expanding rings
    Fireball(i32), // radius: explosion radius

    /// Chain lightning - hits in zigzag pattern, chains between targets
    ChainLightning(i32), // reach: how far the chain extends

    /// Frost nova - expands in a diamond/cross pattern from caster
    FrostNova(i32), // reach: expansion distance

    /// Meteor shower - falls from above in forward direction area
    MeteorShower(i32, i32), // (reach, width): distance and impact area width

    /// Crescent moon slash - curved attack pattern
    CrescentSlash,

    /// Vortex - spiraling pattern that pulls inward
    Vortex(i32), // radius: size of the vortex
}

impl AttackPattern {
    /// Get all affected tiles with animation frames based on direction
    /// Returns animation frames for visual rendering
    pub fn get_animation_frames(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
    ) -> Vec<AnimationFrame> {
        match self {
            AttackPattern::BasicSlash => {
                self.basic_slash_animation(origin_x, origin_y, dir_x, dir_y)
            }
            AttackPattern::GroundSlam(reach) => {
                self.ground_slam_animation(origin_x, origin_y, *reach)
            }
            AttackPattern::WhirlwindAttack => self.whirlwind_animation(origin_x, origin_y),
            AttackPattern::SwordThrust(reach) => {
                self.sword_thrust_animation(origin_x, origin_y, dir_x, dir_y, *reach)
            }
            AttackPattern::ArrowShot(reach) => {
                self.arrow_shot_animation(origin_x, origin_y, dir_x, dir_y, *reach)
            }
            AttackPattern::MultiShot(reach, spread) => {
                self.multishot_animation(origin_x, origin_y, dir_x, dir_y, *reach, *spread)
            }
            AttackPattern::Barrage(reach) => {
                self.barrage_animation(origin_x, origin_y, dir_x, dir_y, *reach)
            }
            AttackPattern::PiercingShot(reach) => {
                self.piercing_shot_animation(origin_x, origin_y, dir_x, dir_y, *reach)
            }
            AttackPattern::Fireball(radius) => {
                self.fireball_animation(origin_x, origin_y, dir_x, dir_y, *radius)
            }
            AttackPattern::ChainLightning(reach) => {
                self.chain_lightning_animation(origin_x, origin_y, dir_x, dir_y, *reach)
            }
            AttackPattern::FrostNova(reach) => {
                self.frost_nova_animation(origin_x, origin_y, *reach)
            }
            AttackPattern::MeteorShower(reach, width) => {
                self.meteor_shower_animation(origin_x, origin_y, dir_x, dir_y, *reach, *width)
            }
            AttackPattern::CrescentSlash => {
                self.crescent_slash_animation(origin_x, origin_y, dir_x, dir_y)
            }
            AttackPattern::Vortex(radius) => self.vortex_animation(origin_x, origin_y, *radius),
        }
    }

    /// Get all affected tiles in final frame (for damage/collision)
    pub fn get_affected_tiles(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
    ) -> Vec<(i32, i32)> {
        let frames = self.get_animation_frames(origin_x, origin_y, dir_x, dir_y);
        if let Some(last_frame) = frames.last() {
            last_frame.tiles.clone()
        } else {
            vec![]
        }
    }

    fn basic_slash_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
    ) -> Vec<AnimationFrame> {
        // 3-tile slash in direction of attack with spreading color
        let mut tiles = vec![(origin_x + dir_x, origin_y + dir_y)];

        // Add perpendicular tiles (the "slash width")
        let perp_x = if dir_y != 0 { 1 } else { 0 };
        let perp_y = if dir_x != 0 { 1 } else { 0 };
        tiles.push((origin_x + dir_x + perp_x, origin_y + dir_y + perp_y));
        tiles.push((origin_x + dir_x - perp_x, origin_y + dir_y - perp_y));
        tiles.sort();
        tiles.dedup();

        vec![
            AnimationFrame {
                tiles: tiles.clone(),
                color: Color::Yellow,
                symbol: '/',
                frame_duration: 0.1,
            },
            AnimationFrame {
                tiles,
                color: Color::White,
                symbol: '\\',
                frame_duration: 0.1,
            },
        ]
    }

    fn ground_slam_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        reach: i32,
    ) -> Vec<AnimationFrame> {
        // Creates expanding shockwave - ripple/wave then explosion burst
        let mut frames = vec![];

        // Initial impact
        frames.push(AnimationFrame {
            tiles: vec![(origin_x, origin_y)],
            color: Color::Red,
            symbol: '*',
            frame_duration: 0.06,
        });

        // Wave/ripple frames (diamond/ring outward)
        for ring in 1..=reach {
            let mut ring_tiles = vec![];

            // Diamond perimeter: all positions where |dx|+|dy| == ring
            for dx in -ring..=ring {
                let dy = ring - dx.abs();
                ring_tiles.push((origin_x + dx, origin_y + dy));
                ring_tiles.push((origin_x + dx, origin_y - dy));
            }

            ring_tiles.sort();
            ring_tiles.dedup();

            frames.push(AnimationFrame {
                tiles: ring_tiles.clone(),
                color: if ring % 2 == 0 {
                    Color::Yellow
                } else {
                    Color::LightRed
                },
                symbol: '~',
                frame_duration: 0.07 - (ring as f32 * 0.006).min(0.03),
            });

            // Slight debris flash behind the wave for polish
            let mut debris = vec![];
            for (x, y) in ring_tiles.iter().take((ring_tiles.len() / 4).max(1)) {
                debris.push((*x, *y));
            }
            frames.push(AnimationFrame {
                tiles: debris,
                color: Color::LightYellow,
                symbol: '^',
                frame_duration: 0.03,
            });
        }

        // Final explosion: filled diamond (strong visual burst)
        let mut burst = vec![];
        for dx in -reach..=reach {
            for dy in -reach..=reach {
                if dx.abs() + dy.abs() <= reach {
                    burst.push((origin_x + dx, origin_y + dy));
                }
            }
        }
        burst.sort();
        burst.dedup();

        frames.push(AnimationFrame {
            tiles: burst.clone(),
            color: Color::LightRed,
            symbol: '*',
            frame_duration: 0.10,
        });

        // Fade-out embers
        frames.push(AnimationFrame {
            tiles: vec![(origin_x, origin_y)],
            color: Color::Yellow,
            symbol: '.',
            frame_duration: 0.08,
        });

        frames
    }

    fn whirlwind_animation(&self, origin_x: i32, origin_y: i32) -> Vec<AnimationFrame> {
        // Spinning attack hitting all 8 directions
        let mut frames = vec![];

        // Create all 8 adjacent tiles
        let all_adjacent: Vec<(i32, i32)> = vec![
            (origin_x + 1, origin_y),
            (origin_x + 1, origin_y + 1),
            (origin_x, origin_y + 1),
            (origin_x - 1, origin_y + 1),
            (origin_x - 1, origin_y),
            (origin_x - 1, origin_y - 1),
            (origin_x, origin_y - 1),
            (origin_x + 1, origin_y - 1),
        ];

        // Create spinning animation with 4 frames (rotate through all tiles)
        for frame_idx in 0..4 {
            let mut tiles = vec![];
            for i in 0..2 {
                let idx = (frame_idx * 2 + i) % 8;
                tiles.push(all_adjacent[idx]);
            }

            frames.push(AnimationFrame {
                tiles,
                color: Color::Cyan,
                symbol: '*',
                frame_duration: 0.06,
            });
        }

        frames
    }

    fn sword_thrust_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
        reach: i32,
    ) -> Vec<AnimationFrame> {
        // Piercing thrust - extends forward in stages
        let mut frames = vec![];

        for distance in 1..=reach {
            let mut tiles = vec![];

            // Create the thrust area (center line + sides)
            for i in 1..=distance {
                let tile_x = origin_x + (dir_x * i);
                let tile_y = origin_y + (dir_y * i);
                tiles.push((tile_x, tile_y));

                // Add sides for width
                let perp_x = if dir_y != 0 { 1 } else { 0 };
                let perp_y = if dir_x != 0 { 1 } else { 0 };
                if i >= distance - 1 {
                    tiles.push((tile_x + perp_x, tile_y + perp_y));
                    tiles.push((tile_x - perp_x, tile_y - perp_y));
                }
            }
            tiles.sort();
            tiles.dedup();

            frames.push(AnimationFrame {
                tiles,
                color: Color::Magenta,
                symbol: '>',
                frame_duration: 0.05,
            });
        }

        frames
    }

    fn arrow_shot_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
        reach: i32,
    ) -> Vec<AnimationFrame> {
        // Arrow travels in straight line with trailing effect
        let mut frames = vec![];

        for distance in 1..=reach {
            let arrow_x = origin_x + (dir_x * distance);
            let arrow_y = origin_y + (dir_y * distance);

            // Create trailing effect
            let mut tiles = vec![(arrow_x, arrow_y)];
            if distance > 1 {
                let trail_x = origin_x + (dir_x * (distance - 1));
                let trail_y = origin_y + (dir_y * (distance - 1));
                tiles.push((trail_x, trail_y));
            }

            frames.push(AnimationFrame {
                tiles,
                color: Color::Yellow,
                symbol: '^',
                frame_duration: 0.03,
            });
        }

        frames
    }

    fn multishot_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
        reach: i32,
        spread: i32,
    ) -> Vec<AnimationFrame> {
        // Three arrows spreading in a fan pattern
        let mut frames = vec![];

        for distance in 1..=reach {
            let mut tiles = vec![];

            // Center arrow
            tiles.push((origin_x + (dir_x * distance), origin_y + (dir_y * distance)));

            // Spread sideways based on direction
            let perp_x = if dir_y != 0 { 1 } else { 0 };
            let perp_y = if dir_x != 0 { 1 } else { 0 };

            // Left and right arrows - ensure spread appears by distance 2
            let actual_spread = if distance >= 2 { spread } else { 0 };
            for offset in 1..=actual_spread {
                tiles.push((
                    origin_x + (dir_x * distance) + (perp_x * offset),
                    origin_y + (dir_y * distance) + (perp_y * offset),
                ));
                tiles.push((
                    origin_x + (dir_x * distance) - (perp_x * offset),
                    origin_y + (dir_y * distance) - (perp_y * offset),
                ));
            }

            frames.push(AnimationFrame {
                tiles,
                color: Color::LightYellow,
                symbol: '^',
                frame_duration: 0.04,
            });
        }

        frames
    }

    fn barrage_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
        reach: i32,
    ) -> Vec<AnimationFrame> {
        // Rapid successive hits along a line
        let mut frames = vec![];

        for distance in 1..=reach {
            let tile_x = origin_x + (dir_x * distance);
            let tile_y = origin_y + (dir_y * distance);

            frames.push(AnimationFrame {
                tiles: vec![(tile_x, tile_y)],
                color: Color::LightRed,
                symbol: '!',
                frame_duration: 0.02,
            });
        }

        frames
    }

    fn piercing_shot_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
        reach: i32,
    ) -> Vec<AnimationFrame> {
        // Same as arrow but with stronger visual indicator
        let mut frames = vec![];

        for distance in 1..=reach {
            let arrow_x = origin_x + (dir_x * distance);
            let arrow_y = origin_y + (dir_y * distance);

            let mut tiles = vec![(arrow_x, arrow_y)];
            if distance > 2 {
                tiles.push((
                    origin_x + (dir_x * (distance - 1)),
                    origin_y + (dir_y * (distance - 1)),
                ));
                tiles.push((
                    origin_x + (dir_x * (distance - 2)),
                    origin_y + (dir_y * (distance - 2)),
                ));
            }

            frames.push(AnimationFrame {
                tiles,
                color: Color::White,
                symbol: '»',
                frame_duration: 0.03,
            });
        }

        frames
    }

    fn fireball_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
        radius: i32,
    ) -> Vec<AnimationFrame> {
        // Launch three small fireballs in sequence; each travels then explodes
        let mut frames: Vec<AnimationFrame> = Vec::new();

        // Helper: generate filled circle
        let circle = |cx: i32, cy: i32, r: i32| -> Vec<(i32, i32)> {
            let mut tiles = Vec::new();
            let r_sq = r * r;
            for dx in -r..=r {
                for dy in -r..=r {
                    if dx * dx + dy * dy <= r_sq {
                        tiles.push((cx + dx, cy + dy));
                    }
                }
            }
            tiles.sort();
            tiles.dedup();
            tiles
        };

        // If no direction provided, show three centered explosions staggered
        let dir_is_zero = dir_x == 0 && dir_y == 0;

        for i in 0..3 {
            // travel distance before impact (staggered so they come one-after-another)
            let travel = 2 + i * 2; // 2,4,6 steps

            // Travel frames: small tail and head
            for step in 1..=travel {
                let head_x = if dir_is_zero {
                    origin_x
                } else {
                    origin_x + dir_x * step
                };
                let head_y = if dir_is_zero {
                    origin_y
                } else {
                    origin_y + dir_y * step
                };

                let mut tiles = vec![(head_x, head_y)];

                // trailing ember
                if step > 1 {
                    let tail_x = if dir_is_zero {
                        origin_x
                    } else {
                        origin_x + dir_x * (step - 1)
                    };
                    let tail_y = if dir_is_zero {
                        origin_y
                    } else {
                        origin_y + dir_y * (step - 1)
                    };
                    tiles.push((tail_x, tail_y));
                }

                frames.push(AnimationFrame {
                    tiles,
                    color: Color::LightRed,
                    symbol: '•',
                    frame_duration: 0.04,
                });
            }

            // Impact point
            let impact_x = if dir_is_zero {
                origin_x
            } else {
                origin_x + dir_x * travel
            };
            let impact_y = if dir_is_zero {
                origin_y
            } else {
                origin_y + dir_y * travel
            };

            // Explosion: quick inner burst then outer ring
            // Inner burst
            frames.push(AnimationFrame {
                tiles: circle(impact_x, impact_y, 0),
                color: Color::Yellow,
                symbol: '*',
                frame_duration: 0.06,
            });

            // Expanding rings
            for r in 1..=radius {
                let tiles = circle(impact_x, impact_y, r);
                frames.push(AnimationFrame {
                    tiles,
                    color: if r == 1 { Color::LightRed } else { Color::Red },
                    symbol: '*',
                    frame_duration: 0.06 - (r as f32 * 0.005).max(0.02),
                });
            }

            // Small lingering ember frame between fireballs
            frames.push(AnimationFrame {
                tiles: vec![(impact_x, impact_y)],
                color: Color::LightYellow,
                symbol: '.',
                frame_duration: 0.06,
            });
        }

        frames
    }

    fn chain_lightning_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
        reach: i32,
    ) -> Vec<AnimationFrame> {
        // Branching, fading chain lightning with short trails and occasional forks
        let mut frames: Vec<AnimationFrame> = Vec::new();

        let mut heads: Vec<(i32, i32)> = vec![(origin_x, origin_y)];

        for step in 1..=reach {
            let mut next_heads: Vec<(i32, i32)> = Vec::new();

            // For each head, advance and create visible bolt + a faint trail
            for (hx, hy) in heads.iter() {
                let nx = hx + dir_x;
                let ny = hy + dir_y;

                // Main bolt and short trail
                let mut tiles = vec![(nx, ny)];
                tiles.push((*hx, *hy));

                // Occasionally fork perpendicular
                if step % 3 == 0 {
                    let perp_x = if dir_y != 0 { 1 } else { 0 };
                    let perp_y = if dir_x != 0 { 1 } else { 0 };
                    tiles.push((nx + perp_x, ny + perp_y));
                    next_heads.push((nx + perp_x, ny + perp_y));
                }

                next_heads.push((nx, ny));

                frames.push(AnimationFrame {
                    tiles,
                    color: if step % 2 == 0 {
                        Color::LightCyan
                    } else {
                        Color::Cyan
                    },
                    symbol: '≈',
                    frame_duration: 0.04,
                });

                // Short fading after-image (fainter)
                frames.push(AnimationFrame {
                    tiles: vec![(*hx, *hy)],
                    color: Color::White,
                    symbol: '.',
                    frame_duration: 0.03,
                });
            }

            // cap number of simultaneous heads to avoid explosion
            heads = next_heads.into_iter().take(6).collect();
        }

        // Final impact flash at last heads
        let mut final_tiles = Vec::new();
        for (x, y) in heads.iter() {
            final_tiles.push((*x, *y));
            // small surrounding sparks
            final_tiles.push((x + 1, *y));
            final_tiles.push((x - 1, *y));
            final_tiles.push((*x, y + 1));
            final_tiles.push((*x, y - 1));
        }
        final_tiles.sort();
        final_tiles.dedup();

        frames.push(AnimationFrame {
            tiles: final_tiles,
            color: Color::LightBlue,
            symbol: '⚡',
            frame_duration: 0.08,
        });

        frames
    }

    fn frost_nova_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        reach: i32,
    ) -> Vec<AnimationFrame> {
        // Dramatic expanding diamond with icy shard overlays and lingering frost
        let mut frames: Vec<AnimationFrame> = Vec::new();

        // Small initial freeze at center
        frames.push(AnimationFrame {
            tiles: vec![(origin_x, origin_y)],
            color: Color::LightBlue,
            symbol: '✦',
            frame_duration: 0.06,
        });

        for distance in 1..=reach {
            let mut tiles = vec![];

            // Diamond perimeter (Manhattan distance == distance)
            for dx in -distance..=distance {
                let dy = distance - dx.abs();
                tiles.push((origin_x + dx, origin_y + dy));
                tiles.push((origin_x + dx, origin_y - dy));
            }
            tiles.sort();
            tiles.dedup();

            // Ice shards: some cardinal spikes inside the ring
            let mut shards = vec![];
            if distance % 2 == 0 {
                shards.push((origin_x + distance, origin_y));
                shards.push((origin_x - distance, origin_y));
                shards.push((origin_x, origin_y + distance));
                shards.push((origin_x, origin_y - distance));
            }

            frames.push(AnimationFrame {
                tiles: tiles.clone(),
                color: Color::Blue,
                symbol: '*',
                frame_duration: 0.08 - (distance as f32 * 0.005).min(0.04),
            });

            if !shards.is_empty() {
                frames.push(AnimationFrame {
                    tiles: shards,
                    color: Color::LightBlue,
                    symbol: '❄',
                    frame_duration: 0.05,
                });
            }

            // lingering frost embers
            frames.push(AnimationFrame {
                tiles: vec![(origin_x, origin_y)],
                color: Color::White,
                symbol: '.',
                frame_duration: 0.04,
            });
        }

        frames
    }

    fn meteor_shower_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
        reach: i32,
        width: i32,
    ) -> Vec<AnimationFrame> {
        // Staggered falling meteors with trails and wider impact splashes
        let mut frames: Vec<AnimationFrame> = Vec::new();

        // simple circle helper for impacts
        let circle = |cx: i32, cy: i32, r: i32| -> Vec<(i32, i32)> {
            let mut tiles = Vec::new();
            let r_sq = r * r;
            for dx in -r..=r {
                for dy in -r..=r {
                    if dx * dx + dy * dy <= r_sq {
                        tiles.push((cx + dx, cy + dy));
                    }
                }
            }
            tiles.sort();
            tiles.dedup();
            tiles
        };

        for distance in 1..=reach {
            // stagger start offset so meteors arrive in waves
            let stagger = (distance % 3) as i32;

            // meteor start (a few tiles before impact)
            let start_x = origin_x + dir_x * (distance - 3 - stagger);
            let start_y = origin_y + dir_y * (distance - 3 - stagger);

            // falling trail frames toward impact
            for t in 0..3 {
                let tpos_x = start_x + dir_x * t;
                let tpos_y = start_y + dir_y * t;
                frames.push(AnimationFrame {
                    tiles: vec![(tpos_x, tpos_y)],
                    color: Color::LightRed,
                    symbol: '/',
                    frame_duration: 0.03,
                });
            }

            let impact_x = origin_x + (dir_x * distance);
            let impact_y = origin_y + (dir_y * distance);

            // Impact core
            frames.push(AnimationFrame {
                tiles: vec![(impact_x, impact_y)],
                color: Color::Yellow,
                symbol: '✦',
                frame_duration: 0.04,
            });

            // Splash: use width to determine radius (min 1)
            let splash_radius = (width.max(1) / 2).max(1);
            for r in 1..=splash_radius {
                let tiles = circle(impact_x, impact_y, r);
                frames.push(AnimationFrame {
                    tiles,
                    color: if r == 1 { Color::LightRed } else { Color::Red },
                    symbol: '◆',
                    frame_duration: 0.05 - (r as f32 * 0.004).min(0.02),
                });
            }

            // Scattered embers
            frames.push(AnimationFrame {
                tiles: vec![(impact_x + 1, impact_y), (impact_x - 1, impact_y)],
                color: Color::LightYellow,
                symbol: '.',
                frame_duration: 0.04,
            });
        }

        frames
    }

    // === UNIQUE ANIMATIONS ===

    fn crescent_slash_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
    ) -> Vec<AnimationFrame> {
        // Curved slash pattern
        let mut frames = vec![];
        let mut tiles = vec![];

        // Create a curved arc pattern based on direction
        if dir_x != 0 {
            // Horizontal direction - vertical curve
            tiles.push((origin_x + dir_x, origin_y));
            tiles.push((origin_x + dir_x, origin_y + 1));
            tiles.push((origin_x + dir_x, origin_y - 1));
            tiles.push((origin_x + dir_x * 2, origin_y + 1));
            tiles.push((origin_x + dir_x * 2, origin_y - 1));
        } else {
            // Vertical direction - horizontal curve
            tiles.push((origin_x, origin_y + dir_y));
            tiles.push((origin_x + 1, origin_y + dir_y));
            tiles.push((origin_x - 1, origin_y + dir_y));
            tiles.push((origin_x + 1, origin_y + dir_y * 2));
            tiles.push((origin_x - 1, origin_y + dir_y * 2));
        }

        tiles.sort();
        tiles.dedup();

        frames.push(AnimationFrame {
            tiles: tiles.clone(),
            color: Color::Magenta,
            symbol: '(',
            frame_duration: 0.1,
        });

        frames.push(AnimationFrame {
            tiles,
            color: Color::White,
            symbol: ')',
            frame_duration: 0.1,
        });

        frames
    }

    fn vortex_animation(&self, origin_x: i32, origin_y: i32, radius: i32) -> Vec<AnimationFrame> {
        // Rotating spiral that pulls inward with an inner core pulse
        let mut frames: Vec<AnimationFrame> = Vec::new();

        // Build spiral rings from outer to inner, but animate with rotation offsets
        for ring in (1..=radius).rev() {
            let mut tiles = Vec::new();

            // perimeter points approx circle using distance range
            for dx in -ring..=ring {
                for dy in -ring..=ring {
                    let dist_sq = dx * dx + dy * dy;
                    if dist_sq <= ring * ring && dist_sq > (ring - 1) * (ring - 1) {
                        tiles.push((origin_x + dx, origin_y + dy));
                    }
                }
            }

            // create a rotation offset based on ring to stagger appearance
            let symbol = match ring % 3 {
                0 => '○',
                1 => '◐',
                _ => '◓',
            };

            frames.push(AnimationFrame {
                tiles: tiles.clone(),
                color: Color::Magenta,
                symbol,
                frame_duration: 0.05 + (radius - ring) as f32 * 0.01,
            });

            // inward pull: small set of tiles closer to center
            if ring > 1 {
                let mut inner = Vec::new();
                for dx in -(ring - 1)..=(ring - 1) {
                    let dy = (ring - 1) - dx.abs();
                    inner.push((origin_x + dx, origin_y + dy));
                    inner.push((origin_x + dx, origin_y - dy));
                }
                inner.sort();
                inner.dedup();
                frames.push(AnimationFrame {
                    tiles: inner,
                    color: Color::LightRed,
                    symbol: '.',
                    frame_duration: 0.03,
                });
            }
        }

        // Core pulse
        frames.push(AnimationFrame {
            tiles: vec![(origin_x, origin_y)],
            color: Color::Magenta,
            symbol: '@',
            frame_duration: 0.12,
        });

        frames
    }

    /// Get a human-readable name for the pattern
    pub fn name(&self) -> &str {
        match self {
            AttackPattern::BasicSlash => "Basic Slash",
            AttackPattern::GroundSlam(_) => "Ground Slam",
            AttackPattern::WhirlwindAttack => "Whirlwind",
            AttackPattern::SwordThrust(_) => "Sword Thrust",
            AttackPattern::ArrowShot(_) => "Arrow Shot",
            AttackPattern::MultiShot(_, _) => "Multi Shot",
            AttackPattern::Barrage(_) => "Barrage",
            AttackPattern::PiercingShot(_) => "Piercing Shot",
            AttackPattern::Fireball(_) => "Fireball",
            AttackPattern::ChainLightning(_) => "Chain Lightning",
            AttackPattern::FrostNova(_) => "Frost Nova",
            AttackPattern::MeteorShower(_, _) => "Meteor Shower",
            AttackPattern::CrescentSlash => "Crescent Slash",
            AttackPattern::Vortex(_) => "Vortex",
        }
    }

    /// Get description including reach/radius
    pub fn description(&self) -> String {
        match self {
            AttackPattern::BasicSlash => "Quick 3-tile slash with wide coverage".to_string(),
            AttackPattern::GroundSlam(reach) => {
                format!("Shockwave expands {} tiles in all directions", reach)
            }
            AttackPattern::WhirlwindAttack => "Spin attack hitting all 8 directions".to_string(),
            AttackPattern::SwordThrust(reach) => {
                format!("Pierce forward {} tiles with force", reach)
            }
            AttackPattern::ArrowShot(reach) => format!("Single arrow projectile {} tiles", reach),
            AttackPattern::MultiShot(reach, spread) => {
                format!("3 arrows spreading {} tiles, {} spread", reach, spread)
            }
            AttackPattern::Barrage(reach) => format!("Rapid successive hits along {} tiles", reach),
            AttackPattern::PiercingShot(reach) => format!("Armor-piercing shot {} tiles", reach),
            AttackPattern::Fireball(radius) => format!("Explosion with {} tile radius", radius),
            AttackPattern::ChainLightning(reach) => {
                format!("Lightning chains {} tiles in zigzag", reach)
            }
            AttackPattern::FrostNova(reach) => {
                format!("Ice spreads {} tiles in diamond pattern", reach)
            }
            AttackPattern::MeteorShower(reach, width) => {
                format!("Meteors rain {} tiles wide for {} distance", width, reach)
            }
            AttackPattern::CrescentSlash => "Curved slash with arcing pattern".to_string(),
            AttackPattern::Vortex(radius) => {
                format!("Magical vortex with {} tile radius, pulls inward", radius)
            }
        }
    }

    /// Get weapon type this pattern works best with
    pub fn weapon_type(&self) -> &str {
        match self {
            AttackPattern::BasicSlash
            | AttackPattern::SwordThrust(_)
            | AttackPattern::WhirlwindAttack
            | AttackPattern::CrescentSlash => "Melee",
            AttackPattern::ArrowShot(_)
            | AttackPattern::MultiShot(_, _)
            | AttackPattern::Barrage(_)
            | AttackPattern::PiercingShot(_) => "Ranged",
            AttackPattern::GroundSlam(_)
            | AttackPattern::Fireball(_)
            | AttackPattern::ChainLightning(_)
            | AttackPattern::FrostNova(_)
            | AttackPattern::MeteorShower(_, _)
            | AttackPattern::Vortex(_) => "Magic",
        }
    }
}
