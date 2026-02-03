use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ReachShape {
    /// Straight line forward, length n blocks
    Line(i32),
    /// Triangular cone, r blocks deep
    Cone(i32),
    /// 180° sweep in front (melee cleave)
    Arc,
    /// Plus shape centered on target
    Cross,
    /// Square AoE (3x3 centered on player)
    Area,
    /// Around player (3x3 centered on player)
    Self_,
}

impl Default for ReachShape {
    fn default() -> Self {
        ReachShape::Line(1)
    }
}

impl ReachShape {
    /// Calculate all affected tiles based on reach shape
    /// direction: (dx, dy) where each is -1, 0, or 1
    /// player_pos: (x, y) player position
    pub fn get_affected_tiles(
        &self,
        direction: (i32, i32),
        player_pos: (i32, i32),
    ) -> Vec<(i32, i32)> {
        match self {
            ReachShape::Line(length) => Self::line_tiles(*length, direction, player_pos),
            ReachShape::Cone(depth) => Self::cone_tiles(*depth, direction, player_pos),
            ReachShape::Arc => Self::arc_tiles(direction, player_pos),
            ReachShape::Cross => Self::cross_tiles(player_pos),
            ReachShape::Area => Self::area_tiles(player_pos),
            ReachShape::Self_ => Self::self_tiles(player_pos),
        }
    }

    /// LINE(n) - straight line forward, length n blocks
    fn line_tiles(length: i32, direction: (i32, i32), player_pos: (i32, i32)) -> Vec<(i32, i32)> {
        let mut tiles = Vec::new();
        let (dx, dy) = direction;

        for i in 1..=length {
            let x = player_pos.0 + dx * i;
            let y = player_pos.1 + dy * i;
            tiles.push((x, y));
        }

        tiles
    }

    /// CONE(r) - triangular cone, r blocks deep
    fn cone_tiles(depth: i32, direction: (i32, i32), player_pos: (i32, i32)) -> Vec<(i32, i32)> {
        let mut tiles = Vec::new();
        let (dx, dy) = direction;

        // Determine perpendicular direction based on facing direction
        let (perp_x, perp_y) = if dx != 0 {
            // Facing horizontal - perpendicular is vertical
            (0, 1)
        } else {
            // Facing vertical - perpendicular is horizontal
            (1, 0)
        };

        // Build cone expanding outward
        for dist in 1..=depth {
            let base_x = player_pos.0 + dx * dist;
            let base_y = player_pos.1 + dy * dist;

            // Center of cone
            tiles.push((base_x, base_y));

            // Expand cone width as we go deeper
            for width in 1..=dist {
                tiles.push((base_x + perp_x * width, base_y + perp_y * width));
                tiles.push((base_x - perp_x * width, base_y - perp_y * width));
            }
        }

        tiles
    }

    /// ARC(1) - 180° sweep in front (melee cleave)
    fn arc_tiles(direction: (i32, i32), player_pos: (i32, i32)) -> Vec<(i32, i32)> {
        let mut tiles = Vec::new();
        let (dx, dy) = direction;

        // Add the primary direction tile
        tiles.push((player_pos.0 + dx, player_pos.1 + dy));

        // Determine perpendicular directions
        let (perp_x, perp_y) = if dx != 0 {
            // Facing horizontal - sweep up and down
            (0, 1)
        } else {
            // Facing vertical - sweep left and right
            (1, 0)
        };

        // Add perpendicular tiles (one step to each side)
        tiles.push((player_pos.0 + dx + perp_x, player_pos.1 + dy + perp_y));
        tiles.push((player_pos.0 + dx - perp_x, player_pos.1 + dy - perp_y));

        tiles
    }

    /// CROSS(1) - plus shape centered on target
    fn cross_tiles(player_pos: (i32, i32)) -> Vec<(i32, i32)> {
        vec![
            (player_pos.0, player_pos.1),     // Center
            (player_pos.0 + 1, player_pos.1), // Right
            (player_pos.0 - 1, player_pos.1), // Left
            (player_pos.0, player_pos.1 + 1), // Down
            (player_pos.0, player_pos.1 - 1), // Up
        ]
    }

    /// AREA(3x3) - square AoE centered on player
    fn area_tiles(player_pos: (i32, i32)) -> Vec<(i32, i32)> {
        let mut tiles = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                tiles.push((player_pos.0 + dx, player_pos.1 + dy));
            }
        }
        tiles
    }

    /// SELF(3x3) - around player (same as AREA)
    fn self_tiles(player_pos: (i32, i32)) -> Vec<(i32, i32)> {
        let mut tiles = Vec::new();
        for dx in -1..=1 {
            for dy in -1..=1 {
                tiles.push((player_pos.0 + dx, player_pos.1 + dy));
            }
        }
        tiles
    }

    /// Get a descriptive name for this reach shape
    pub fn name(&self) -> &str {
        match self {
            ReachShape::Line(_) => "Line",
            ReachShape::Cone(_) => "Cone",
            ReachShape::Arc => "Arc",
            ReachShape::Cross => "Cross",
            ReachShape::Area => "Area",
            ReachShape::Self_ => "Self",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_right() {
        let shape = ReachShape::Line(3);
        let tiles = shape.get_affected_tiles((1, 0), (0, 0));
        assert_eq!(tiles.len(), 3);
        assert!(tiles.contains(&(1, 0)));
        assert!(tiles.contains(&(2, 0)));
        assert!(tiles.contains(&(3, 0)));
    }

    #[test]
    fn test_arc() {
        let shape = ReachShape::Arc;
        let tiles = shape.get_affected_tiles((1, 0), (0, 0));
        assert_eq!(tiles.len(), 3);
        assert!(tiles.contains(&(1, 0)));
    }

    #[test]
    fn test_cross() {
        let shape = ReachShape::Cross;
        let tiles = shape.get_affected_tiles((0, 0), (5, 5));
        assert_eq!(tiles.len(), 5);
        assert!(tiles.contains(&(5, 5)));
        assert!(tiles.contains(&(6, 5)));
        assert!(tiles.contains(&(4, 5)));
    }

    #[test]
    fn test_area() {
        let shape = ReachShape::Area;
        let tiles = shape.get_affected_tiles((0, 0), (5, 5));
        assert_eq!(tiles.len(), 9);
    }
}
