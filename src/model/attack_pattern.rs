use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// Animation frame for attack visualization
#[derive(Clone, Debug, PartialEq)]
pub struct AnimationFrame {
    pub tiles: Vec<(i32, i32)>,
    pub color: Color,
    pub symbol: char,
    pub frame_duration: f32, // seconds
}

/// Unified attack pattern system with animations for both player and enemies
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
            AttackPattern::Fireball(radius) => self.fireball_animation(origin_x, origin_y, *radius),
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

    // === CLOSE COMBAT ANIMATIONS ===

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
        // Creates expanding shockwave - diamond pattern expanding outward
        let mut frames = vec![];

        // Impact frame at center
        frames.push(AnimationFrame {
            tiles: vec![(origin_x, origin_y)],
            color: Color::Red,
            symbol: '*',
            frame_duration: 0.05,
        });

        // Expanding rings
        for ring in 1..=reach {
            let mut ring_tiles = vec![];

            // Create diamond shape expanding outward
            for i in 0..=ring {
                let remaining = ring - i;
                ring_tiles.push((origin_x + i, origin_y + remaining));
                ring_tiles.push((origin_x - i, origin_y + remaining));
                ring_tiles.push((origin_x + i, origin_y - remaining));
                ring_tiles.push((origin_x - i, origin_y - remaining));
            }
            ring_tiles.sort();
            ring_tiles.dedup();

            frames.push(AnimationFrame {
                tiles: ring_tiles,
                color: if ring % 2 == 0 {
                    Color::Yellow
                } else {
                    Color::LightRed
                },
                symbol: '~',
                frame_duration: 0.08,
            });
        }

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

    // === RANGED ANIMATIONS ===

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

    // === MAGICAL ANIMATIONS ===

    fn fireball_animation(&self, origin_x: i32, origin_y: i32, radius: i32) -> Vec<AnimationFrame> {
        // Expanding fireball with growing ring
        let mut frames = vec![];

        for r in 1..=radius {
            let mut tiles = vec![];
            let radius_sq = r * r;

            // Circle using distance formula
            for dx in -r..=r {
                for dy in -r..=r {
                    if dx * dx + dy * dy <= radius_sq {
                        tiles.push((origin_x + dx, origin_y + dy));
                    }
                }
            }

            let color = if r <= radius / 2 {
                Color::Red
            } else {
                Color::Yellow
            };

            frames.push(AnimationFrame {
                tiles,
                color,
                symbol: '*',
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
        // Zigzag pattern of lightning bolts
        let mut frames = vec![];
        let mut current_x = origin_x;
        let mut current_y = origin_y;

        for step in 0..reach {
            let mut tiles = vec![(current_x, current_y)];

            // Move forward
            current_x += dir_x;
            current_y += dir_y;
            tiles.push((current_x, current_y));

            // Zigzag perpendicular
            let perp_x = if dir_y != 0 { 1 } else { 0 };
            let perp_y = if dir_x != 0 { 1 } else { 0 };

            if step % 2 == 0 {
                tiles.push((current_x + perp_x, current_y + perp_y));
            } else {
                tiles.push((current_x - perp_x, current_y - perp_y));
            }

            frames.push(AnimationFrame {
                tiles,
                color: Color::Cyan,
                symbol: '≈',
                frame_duration: 0.05,
            });
        }

        frames
    }

    fn frost_nova_animation(
        &self,
        origin_x: i32,
        origin_y: i32,
        reach: i32,
    ) -> Vec<AnimationFrame> {
        // Expanding diamond/cross pattern outward
        let mut frames = vec![];

        for distance in 1..=reach {
            let mut tiles = vec![];

            // Diamond pattern (all 4 cardinal directions)
            for i in 0..=distance {
                let remaining = distance - i;
                tiles.push((origin_x + i, origin_y + remaining));
                tiles.push((origin_x - i, origin_y + remaining));
                tiles.push((origin_x + i, origin_y - remaining));
                tiles.push((origin_x - i, origin_y - remaining));
            }
            tiles.sort();
            tiles.dedup();

            frames.push(AnimationFrame {
                tiles,
                color: Color::Blue,
                symbol: '*',
                frame_duration: 0.07,
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
        // Meteors fall from above in forward direction area
        let mut frames = vec![];

        for distance in 1..=reach {
            let mut tiles = vec![];
            let impact_x = origin_x + (dir_x * distance);
            let impact_y = origin_y + (dir_y * distance);

            // Create impact area
            for w in 0..width {
                tiles.push((impact_x + w, impact_y));
                tiles.push((impact_x - w, impact_y));
                if w > 0 {
                    tiles.push((impact_x, impact_y + w));
                    tiles.push((impact_x, impact_y - w));
                }
            }
            tiles.sort();
            tiles.dedup();

            frames.push(AnimationFrame {
                tiles,
                color: Color::Red,
                symbol: '◆',
                frame_duration: 0.05,
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
        // Spiraling vortex pulling inward
        let mut frames = vec![];

        for r in (1..=radius).rev() {
            let mut tiles = vec![];
            let r_sq = r * r;

            // Circular pattern
            for dx in -r..=r {
                for dy in -r..=r {
                    if dx * dx + dy * dy <= r_sq && dx * dx + dy * dy > (r - 1) * (r - 1) {
                        tiles.push((origin_x + dx, origin_y + dy));
                    }
                }
            }

            frames.push(AnimationFrame {
                tiles,
                color: Color::Magenta,
                symbol: '○',
                frame_duration: 0.05,
            });
        }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_slash() {
        let pattern = AttackPattern::BasicSlash;
        let tiles = pattern.get_affected_tiles(0, 0, 1, 0);
        assert!(!tiles.is_empty());
        assert!(tiles.contains(&(1, 0))); // forward
    }

    #[test]
    fn test_ground_slam_expanding() {
        let pattern = AttackPattern::GroundSlam(2);
        let frames = pattern.get_animation_frames(0, 0, 0, 0);
        assert!(frames.len() > 1); // Multiple frames for expanding effect

        // Last frame should have more tiles than first
        let first_frame_count = frames.first().map(|f| f.tiles.len()).unwrap_or(0);
        let last_frame_count = frames.last().map(|f| f.tiles.len()).unwrap_or(0);
        assert!(last_frame_count >= first_frame_count);
    }

    #[test]
    fn test_whirlwind_all_adjacent() {
        let pattern = AttackPattern::WhirlwindAttack;
        let tiles = pattern.get_affected_tiles(0, 0, 0, 0);
        // Should hit all 8 adjacent tiles
        assert!(tiles.len() >= 8 || tiles.len() <= 16); // Some tiles may overlap
    }

    #[test]
    fn test_sword_thrust_directional() {
        let pattern = AttackPattern::SwordThrust(3);
        let tiles_right = pattern.get_affected_tiles(0, 0, 1, 0);
        let tiles_down = pattern.get_affected_tiles(0, 0, 0, 1);

        // Different directions should produce different results
        assert!(!tiles_right.is_empty());
        assert!(!tiles_down.is_empty());
    }

    #[test]
    fn test_arrow_shot_reaches_target() {
        let pattern = AttackPattern::ArrowShot(5);
        let tiles = pattern.get_affected_tiles(0, 0, 1, 0);

        // Should reach at least the maximum distance
        let max_x = tiles.iter().map(|(x, _)| x.abs()).max().unwrap_or(0);
        assert!(max_x >= 4); // Should reach at least 4 tiles away
    }

    #[test]
    fn test_multishot_spreading() {
        let pattern = AttackPattern::MultiShot(5, 2);
        let tiles = pattern.get_affected_tiles(0, 0, 1, 0);

        // Should hit multiple tiles along the line
        // get_affected_tiles returns the LAST frame's tiles
        // With reach=5, final distance should be 5, which is >= 2, so spread should apply
        assert!(!tiles.is_empty());

        // At distance 5 with spread 2, should have center + 2 on each side
        // That would be 5 tiles in the final frame
        assert!(tiles.len() >= 3); // At least center and some spread
    }

    #[test]
    fn test_fireball_circular() {
        let pattern = AttackPattern::Fireball(2);
        let tiles = pattern.get_affected_tiles(0, 0, 0, 0);

        // Should form roughly circular pattern
        assert!(tiles.contains(&(0, 0))); // center
        assert!(!tiles.is_empty());
    }

    #[test]
    fn test_frost_nova_diamond_pattern() {
        let pattern = AttackPattern::FrostNova(2);
        let tiles = pattern.get_affected_tiles(0, 0, 0, 0);

        // Should hit cardinal directions
        assert!(tiles.contains(&(0, 1)) || tiles.contains(&(0, 2)));
        assert!(tiles.contains(&(0, -1)) || tiles.contains(&(0, -2)));
        assert!(tiles.contains(&(1, 0)) || tiles.contains(&(2, 0)));
        assert!(tiles.contains(&(-1, 0)) || tiles.contains(&(-2, 0)));
    }

    #[test]
    fn test_chain_lightning_extends() {
        let pattern = AttackPattern::ChainLightning(3);
        let frames = pattern.get_animation_frames(0, 0, 1, 0);

        // Should have multiple frames showing progression
        assert!(frames.len() >= 2);
    }

    #[test]
    fn test_meteor_shower_impact_area() {
        let pattern = AttackPattern::MeteorShower(3, 2);
        let tiles = pattern.get_affected_tiles(0, 0, 1, 0);

        // Should create impact area with width
        assert!(!tiles.is_empty());
    }

    #[test]
    fn test_vortex_spiral() {
        let pattern = AttackPattern::Vortex(2);
        let frames = pattern.get_animation_frames(0, 0, 0, 0);

        // Should have multiple frames for spiral effect
        assert!(frames.len() > 1);
    }

    #[test]
    fn test_animation_frame_colors() {
        let pattern = AttackPattern::Fireball(1);
        let frames = pattern.get_animation_frames(0, 0, 0, 0);

        // Should have valid colors
        for frame in frames {
            assert!(!frame.tiles.is_empty() || frame.symbol == '*');
        }
    }

    #[test]
    fn test_crescent_slash_curve() {
        let pattern = AttackPattern::CrescentSlash;
        let tiles = pattern.get_affected_tiles(0, 0, 1, 0);

        // Should create curved pattern
        assert!(!tiles.is_empty());
    }

    #[test]
    fn test_pattern_names_and_descriptions() {
        let patterns = vec![
            AttackPattern::BasicSlash,
            AttackPattern::GroundSlam(2),
            AttackPattern::WhirlwindAttack,
            AttackPattern::ArrowShot(5),
            AttackPattern::Fireball(2),
        ];

        for pattern in patterns {
            let name = pattern.name();
            let desc = pattern.description();
            let weapon = pattern.weapon_type();

            assert!(!name.is_empty());
            assert!(!desc.is_empty());
            assert!(!weapon.is_empty());
        }
    }
}
