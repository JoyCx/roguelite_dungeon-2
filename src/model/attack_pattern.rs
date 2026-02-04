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
    // === CLOSE COMBAT PATTERNS ===
    /// Classic melee slash - 3x3 centered around attacker
    BasicSlash,

    /// Ground slam - Creates shockwave expanding outward from player
    GroundSlam(i32), // reach: how many tiles the shockwave travels

    /// Spinning attack - hits in rotating pattern, all 8 directions
    WhirlwindAttack,

    /// Forward thrust - pierces through multiple enemies in a line
    SwordThrust(i32), // reach: distance of the thrust

    // === RANGED PATTERNS ===
    /// Single projectile - straight line attack
    ArrowShot(i32), // reach: distance the arrow travels

    /// Multishot - arrows spread in a fan pattern
    MultiShot(i32, i32), // (reach, spread_width): distance and spread

    /// Barrage - rapid hits in a line with staggered tiles
    Barrage(i32), // reach: total distance covered

    /// Piercing shot - goes through walls/obstacles further
    PiercingShot(i32), // reach: distance, ignores obstacles

    // === MAGICAL PATTERNS ===
    /// Fireball - circular explosion with expanding rings
    Fireball(i32), // radius: explosion radius

    /// Chain lightning - hits in zigzag pattern, chains between targets
    ChainLightning(i32), // reach: how far the chain extends

    /// Frost nova - expands in a diamond/cross pattern from caster
    FrostNova(i32), // reach: expansion distance

    /// Meteor shower - falls from above in forward direction area
    MeteorShower(i32, i32), // (reach, width): distance and impact area width

    // === UNIQUE PATTERNS ===
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
        // For physics, we usually want the area covered by the bulk of the attack.
        // This grabs the union of all tiles involved to ensure nothing is missed,
        // or you could optimize this to only calculate the specific hit zone.
        let frames = self.get_animation_frames(origin_x, origin_y, dir_x, dir_y);
        let mut all_tiles = Vec::new();
        for frame in frames {
            all_tiles.extend(frame.tiles);
        }
        all_tiles.sort();
        all_tiles.dedup();
        all_tiles
    }

    // === CLOSE COMBAT ANIMATIONS ===

    fn basic_slash_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
    ) -> Vec<AnimationFrame> {
        let mut frames = vec![];

        // 1. Wind up (small indicator)
        frames.push(AnimationFrame {
            tiles: vec![(origin_x + dir_x, origin_y + dir_y)],
            color: Color::DarkGray,
            symbol: '.',
            frame_duration: 0.05,
        });

        // 2. The Swing Arc
        // Calculate perpendicular vectors for the "width" of the slash
        let perp_x = if dir_y != 0 { 1 } else { 0 };
        let perp_y = if dir_x != 0 { 1 } else { 0 };

        let center = (origin_x + dir_x, origin_y + dir_y);
        let left = (origin_x + dir_x + perp_x, origin_y + dir_y + perp_y);
        let right = (origin_x + dir_x - perp_x, origin_y + dir_y - perp_y);

        // Frame 2: Start of swing
        frames.push(AnimationFrame {
            tiles: vec![right],
            color: Color::White,
            symbol: '*',
            frame_duration: 0.04,
        });

        // Frame 3: Mid swing (Hit frame)
        frames.push(AnimationFrame {
            tiles: vec![right, center, left],
            color: Color::LightYellow,
            symbol: 'X', // Impact
            frame_duration: 0.06,
        });

        // Frame 4: Follow through
        frames.push(AnimationFrame {
            tiles: vec![left],
            color: Color::Yellow,
            symbol: '/',
            frame_duration: 0.04,
        });

        frames
    }

    fn ground_slam_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        reach: i32,
    ) -> Vec<AnimationFrame> {
        let mut frames = vec![];

        // 1. Build up (Center focuses)
        frames.push(AnimationFrame {
            tiles: vec![(origin_x, origin_y)],
            color: Color::White,
            symbol: '●',
            frame_duration: 0.1,
        });

        // 2. Explosion and expanding wave
        for r in 1..=reach {
            let mut wave_tiles = vec![];
            let mut debris_tiles = vec![];

            // Create a diamond/circle shape
            for dx in -r..=r {
                for dy in -r..=r {
                    let dist = dx.abs() + dy.abs();
                    if dist == r {
                        wave_tiles.push((origin_x + dx, origin_y + dy));
                    } else if dist < r && (dx + dy) % 2 == 0 {
                        debris_tiles.push((origin_x + dx, origin_y + dy));
                    }
                }
            }

            // Combine wave and debris
            let mut all_frame_tiles = wave_tiles.clone();
            all_frame_tiles.extend(debris_tiles);

            frames.push(AnimationFrame {
                tiles: all_frame_tiles,
                color: Color::Indexed(208), // Orange-ish
                symbol: '#',
                frame_duration: 0.05,
            });
        }

        // 3. Lingering cracks
        let mut cracks = vec![];
        for dx in -reach..=reach {
            for dy in -reach..=reach {
                if dx.abs() + dy.abs() <= reach && (dx * dy) % 3 == 0 {
                    cracks.push((origin_x + dx, origin_y + dy));
                }
            }
        }

        frames.push(AnimationFrame {
            tiles: cracks,
            color: Color::DarkGray,
            symbol: '.',
            frame_duration: 0.2,
        });

        frames
    }

    fn whirlwind_animation(&self, origin_x: i32, origin_y: i32) -> Vec<AnimationFrame> {
        let mut frames = vec![];

        // Define the 8 surrounding tiles in clockwise order
        let circle = vec![
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
        ];

        // 3 Full rotations, getting faster
        for rotation in 0..3 {
            for i in 0..8 {
                // Skip frames to make it spin faster
                if rotation == 1 && i % 2 != 0 {
                    continue;
                }
                if rotation == 2 && i % 4 != 0 {
                    continue;
                }

                let (dx, dy) = circle[i];
                let (prev_dx, prev_dy) = circle[(i + 7) % 8]; // Tail

                frames.push(AnimationFrame {
                    tiles: vec![
                        (origin_x + dx, origin_y + dy),
                        (origin_x + prev_dx, origin_y + prev_dy),
                    ],
                    color: Color::Cyan,
                    symbol: if rotation == 2 { '@' } else { '~' },
                    frame_duration: 0.02,
                });
            }
        }

        // Final burst outward
        let mut all_adj = vec![];
        for (dx, dy) in circle.iter() {
            all_adj.push((origin_x + dx, origin_y + dy));
        }
        frames.push(AnimationFrame {
            tiles: all_adj,
            color: Color::White,
            symbol: '*',
            frame_duration: 0.1,
        });

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
        let mut frames = vec![];

        // 1. Charge
        frames.push(AnimationFrame {
            tiles: vec![(origin_x, origin_y)],
            color: Color::White,
            symbol: '+',
            frame_duration: 0.1,
        });

        // 2. The Thrust (Instant line appear)
        let mut thrust_line = vec![];
        for i in 1..=reach {
            thrust_line.push((origin_x + dir_x * i, origin_y + dir_y * i));
        }

        // Flash bright
        frames.push(AnimationFrame {
            tiles: thrust_line.clone(),
            color: Color::Cyan,
            symbol: '≡', // Fast motion lines
            frame_duration: 0.05,
        });

        // 3. The Tip Impact (Heavy damage visual)
        let tip = (origin_x + dir_x * reach, origin_y + dir_y * reach);
        let mut impact_area = vec![tip];
        // Add adjacent to tip for "impact"
        impact_area.push((tip.0 + 1, tip.1));
        impact_area.push((tip.0 - 1, tip.1));
        impact_area.push((tip.0, tip.1 + 1));
        impact_area.push((tip.0, tip.1 - 1));

        frames.push(AnimationFrame {
            tiles: impact_area,
            color: Color::LightCyan,
            symbol: '*',
            frame_duration: 0.1,
        });

        // 4. Fade
        frames.push(AnimationFrame {
            tiles: thrust_line,
            color: Color::Blue,
            symbol: '-',
            frame_duration: 0.05,
        });

        frames
    }

    // === RANGED ANIMATIONS ===

    fn arrow_shot_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
        reach: i32,
    ) -> Vec<AnimationFrame> {
        let mut frames = vec![];

        // Travel Frames
        for i in 1..=reach {
            let current = (origin_x + dir_x * i, origin_y + dir_y * i);
            let prev = (origin_x + dir_x * (i - 1), origin_y + dir_y * (i - 1));

            let mut tiles = vec![current];
            if i > 1 {
                tiles.push(prev);
            } // Trail

            // Pick symbol based on direction
            let symbol = if dir_x.abs() > dir_y.abs() { '-' } else { '|' };

            frames.push(AnimationFrame {
                tiles,
                color: Color::Yellow,
                symbol,
                frame_duration: 0.03, // Fast
            });
        }

        // Impact Frame
        let hit = (origin_x + dir_x * reach, origin_y + dir_y * reach);
        frames.push(AnimationFrame {
            tiles: vec![hit],
            color: Color::LightRed,
            symbol: 'X',
            frame_duration: 0.05,
        });

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
        let mut frames = vec![];

        // Perp vectors
        let perp_x = if dir_y != 0 { 1 } else { 0 };
        let perp_y = if dir_x != 0 { 1 } else { 0 };

        for i in 1..=reach {
            let mut tiles = vec![];

            // Calculate spread factor based on distance (cone shape)
            let current_spread = (i as f32 / reach as f32 * spread as f32).ceil() as i32;

            // Center projectile
            tiles.push((origin_x + dir_x * i, origin_y + dir_y * i));

            // Side projectiles
            if current_spread > 0 {
                tiles.push((
                    origin_x + dir_x * i + perp_x * current_spread,
                    origin_y + dir_y * i + perp_y * current_spread,
                ));
                tiles.push((
                    origin_x + dir_x * i - perp_x * current_spread,
                    origin_y + dir_y * i - perp_y * current_spread,
                ));
            }

            frames.push(AnimationFrame {
                tiles,
                color: Color::LightGreen,
                symbol: 'v',
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
        let mut frames = vec![];

        // Machine-gun style: multiple small hits traveling fast
        for i in 0..reach + 3 {
            let mut tiles = vec![];

            // Generate 3 "bullets" in the air at once
            if i < reach {
                tiles.push((origin_x + dir_x * (i + 1), origin_y + dir_y * (i + 1)));
            }
            if i > 1 && i - 1 < reach {
                tiles.push((origin_x + dir_x * i, origin_y + dir_y * i));
            }
            if i > 3 && i - 3 < reach {
                tiles.push((origin_x + dir_x * (i - 2), origin_y + dir_y * (i - 2)));
            }

            if !tiles.is_empty() {
                frames.push(AnimationFrame {
                    tiles,
                    color: Color::LightYellow,
                    symbol: '•',
                    frame_duration: 0.03,
                });
            }
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
        let mut frames = vec![];

        // Travels normally...
        for i in 1..=reach {
            frames.push(AnimationFrame {
                tiles: vec![(origin_x + dir_x * i, origin_y + dir_y * i)],
                color: Color::Magenta,
                symbol: '»',
                frame_duration: 0.02,
            });
        }

        // ...but leaves a "vacuum" trail behind that lingers
        let mut trail = vec![];
        for i in 1..=reach {
            trail.push((origin_x + dir_x * i, origin_y + dir_y * i));
        }

        frames.push(AnimationFrame {
            tiles: trail,
            color: Color::DarkGray,
            symbol: ':',
            frame_duration: 0.15,
        });

        frames
    }

    // === MAGICAL ANIMATIONS ===

    fn fireball_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        dir_x: i32,
        dir_y: i32,
        radius: i32,
    ) -> Vec<AnimationFrame> {
        let mut frames = vec![];

        // Define target distance (standard cast range)
        let travel_dist = 6;
        let target_x = origin_x + dir_x * travel_dist;
        let target_y = origin_y + dir_y * travel_dist;

        // 1. Travel Phase: Moving Dot
        for i in 1..=travel_dist {
            frames.push(AnimationFrame {
                tiles: vec![(origin_x + dir_x * i, origin_y + dir_y * i)],
                color: Color::Red,
                symbol: 'o',
                frame_duration: 0.04,
            });
        }

        // 2. Impact Phase: Flash
        frames.push(AnimationFrame {
            tiles: vec![(target_x, target_y)],
            color: Color::White,
            symbol: '@',
            frame_duration: 0.05,
        });

        // 3. Explosion Phase: Expanding Circle
        for r in 1..=radius {
            let mut explosion_tiles = vec![];

            // Fill circle
            for dx in -r..=r {
                for dy in -r..=r {
                    if dx * dx + dy * dy <= r * r {
                        explosion_tiles.push((target_x + dx, target_y + dy));
                    }
                }
            }

            frames.push(AnimationFrame {
                tiles: explosion_tiles,
                color: if r % 2 == 0 {
                    Color::Red
                } else {
                    Color::LightRed
                },
                symbol: if r == radius { '#' } else { '*' },
                frame_duration: 0.06,
            });
        }

        // 4. Smoke Phase
        frames.push(AnimationFrame {
            tiles: vec![
                (target_x, target_y),
                (target_x + 1, target_y),
                (target_x - 1, target_y),
                (target_x, target_y + 1),
                (target_x, target_y - 1),
            ],
            color: Color::DarkGray,
            symbol: '%',
            frame_duration: 0.1,
        });

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
        let mut frames = vec![];
        let mut path_points = vec![(origin_x, origin_y)];

        let mut curr_x = origin_x;
        let mut curr_y = origin_y;

        // Generate a random-looking zigzag path
        for i in 1..=reach {
            curr_x += dir_x;
            curr_y += dir_y;

            // Add jitter
            let jitter = if i % 2 == 0 { 1 } else { -1 };
            let j_x = if dir_y != 0 { jitter } else { 0 };
            let j_y = if dir_x != 0 { jitter } else { 0 };

            path_points.push((curr_x + j_x, curr_y + j_y));
        }

        // Frame 1: The Strike (Bright)
        frames.push(AnimationFrame {
            tiles: path_points.clone(),
            color: Color::LightYellow, // Intense white/yellow
            symbol: '⚡',
            frame_duration: 0.05,
        });

        // Frame 2: The Afterimage (Blue)
        frames.push(AnimationFrame {
            tiles: path_points.clone(),
            color: Color::LightBlue,
            symbol: 'z',
            frame_duration: 0.05,
        });

        // Frame 3: Sparks (Scattered)
        let mut sparks = vec![];
        for (x, y) in path_points.iter() {
            if (x + y) % 2 == 0 {
                sparks.push((*x, *y));
            }
        }
        frames.push(AnimationFrame {
            tiles: sparks,
            color: Color::Blue,
            symbol: '.',
            frame_duration: 0.05,
        });

        frames
    }

    fn frost_nova_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        reach: i32,
    ) -> Vec<AnimationFrame> {
        let mut frames = vec![];

        // 1. Charge Up (Suck in)
        frames.push(AnimationFrame {
            tiles: vec![
                (origin_x + 1, origin_y),
                (origin_x - 1, origin_y),
                (origin_x, origin_y + 1),
                (origin_x, origin_y - 1),
            ],
            color: Color::Cyan,
            symbol: '>',
            frame_duration: 0.1,
        });

        // 2. Blast Out
        for r in 1..=reach {
            let mut edge_tiles = vec![];
            for dx in -r..=r {
                for dy in -r..=r {
                    // Manhattan distance for diamond shape
                    if dx.abs() + dy.abs() == r {
                        edge_tiles.push((origin_x + dx, origin_y + dy));
                    }
                }
            }

            frames.push(AnimationFrame {
                tiles: edge_tiles,
                color: Color::White,
                symbol: '*',
                frame_duration: 0.05,
            });
        }

        // 3. Frozen Ground (Lingers)
        let mut frozen_area = vec![];
        for dx in -reach..=reach {
            for dy in -reach..=reach {
                if dx.abs() + dy.abs() <= reach {
                    frozen_area.push((origin_x + dx, origin_y + dy));
                }
            }
        }
        frames.push(AnimationFrame {
            tiles: frozen_area,
            color: Color::LightBlue,
            symbol: '#',
            frame_duration: 0.2, // Stays longer
        });

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
        let mut frames = vec![];

        let target_x = origin_x + dir_x * reach;
        let target_y = origin_y + dir_y * reach;
        let area_radius = width.max(2);

        // Generate 3 random impact points around the target area
        let impacts = vec![
            (target_x, target_y),
            (target_x + 1, target_y - 1),
            (target_x - 1, target_y + 1),
        ];

        for (ix, iy) in impacts {
            // Falling phase (from "sky" - offset Y)
            for h in (0..4).rev() {
                frames.push(AnimationFrame {
                    tiles: vec![(ix, iy - h)],
                    color: Color::LightRed,
                    symbol: '|',
                    frame_duration: 0.03,
                });
            }

            // Impact
            let mut splash = vec![];
            splash.push((ix, iy));
            splash.push((ix + 1, iy));
            splash.push((ix - 1, iy));
            splash.push((ix, iy + 1));
            splash.push((ix, iy - 1));

            frames.push(AnimationFrame {
                tiles: splash,
                color: Color::Yellow,
                symbol: '*', // BOOM
                frame_duration: 0.06,
            });
        }

        // Final crater
        let mut crater = vec![];
        for dx in -area_radius..=area_radius {
            for dy in -area_radius..=area_radius {
                if dx * dx + dy * dy <= area_radius * area_radius {
                    crater.push((target_x + dx, target_y + dy));
                }
            }
        }
        frames.push(AnimationFrame {
            tiles: crater,
            color: Color::DarkGray,
            symbol: '.',
            frame_duration: 0.15,
        });

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
        let mut frames = vec![];

        // Create a moon shape offset by direction
        // If attacking Right (1,0): Draw curve )
        // If attacking Left (-1,0): Draw curve (

        let mut curve_tiles = vec![];
        if dir_x != 0 {
            // Vertical arc
            curve_tiles.push((origin_x + dir_x, origin_y - 1));
            curve_tiles.push((origin_x + dir_x * 2, origin_y)); // Center furthest out
            curve_tiles.push((origin_x + dir_x, origin_y + 1));
        } else {
            // Horizontal arc
            curve_tiles.push((origin_x - 1, origin_y + dir_y));
            curve_tiles.push((origin_x, origin_y + dir_y * 2));
            curve_tiles.push((origin_x + 1, origin_y + dir_y));
        }

        // Frame 1: Flash color 1
        frames.push(AnimationFrame {
            tiles: curve_tiles.clone(),
            color: Color::Magenta,
            symbol: ')', // Placeholder, creates curve look
            frame_duration: 0.06,
        });

        // Frame 2: Flash color 2 (wider)
        let mut wide_tiles = curve_tiles.clone();
        if dir_x != 0 {
            wide_tiles.push((origin_x + dir_x, origin_y));
        } else {
            wide_tiles.push((origin_x, origin_y + dir_y));
        }

        frames.push(AnimationFrame {
            tiles: wide_tiles,
            color: Color::LightMagenta,
            symbol: 'D',
            frame_duration: 0.06,
        });

        frames
    }

    fn vortex_animation(&self, origin_x: i32, origin_y: i32, radius: i32) -> Vec<AnimationFrame> {
        let mut frames = vec![];

        // 3 pulses of sucking in
        for _ in 0..3 {
            // Outer ring
            let mut outer = vec![];
            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    if dx * dx + dy * dy <= radius * radius
                        && dx * dx + dy * dy > (radius - 2) * (radius - 2)
                    {
                        outer.push((origin_x + dx, origin_y + dy));
                    }
                }
            }
            frames.push(AnimationFrame {
                tiles: outer,
                color: Color::Magenta,
                symbol: '%',
                frame_duration: 0.05,
            });

            // Middle ring
            let mut mid = vec![];
            for dx in -radius..=radius {
                for dy in -radius..=radius {
                    if dx * dx + dy * dy <= (radius - 2) * (radius - 2) {
                        mid.push((origin_x + dx, origin_y + dy));
                    }
                }
            }
            frames.push(AnimationFrame {
                tiles: mid,
                color: Color::DarkGray,
                symbol: '@',
                frame_duration: 0.05,
            });
        }

        // Singularity (Center pop)
        frames.push(AnimationFrame {
            tiles: vec![(origin_x, origin_y)],
            color: Color::Black,
            symbol: 'Ø', // Void symbol
            frame_duration: 0.1,
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
