use std::time::Instant;

#[derive(Clone, Debug, PartialEq)]
pub enum ProjectileType {
    Arrow,   // Standard bow arrow
    FireOil, // Thrown fire flask (area damage)
}

#[derive(Clone, Debug)]
pub struct Arrow {
    pub x: f32,
    pub y: f32,
    pub dx: i32,    // Direction x
    pub dy: i32,    // Direction y
    pub speed: f32, // Distance traveled per second
    pub created_at: Instant,
    pub max_distance: f32, // Maximum distance arrow can travel
    pub is_dead: bool,     // Stopped by collision
    pub projectile_type: ProjectileType,
}

impl Arrow {
    pub fn new(x: f32, y: f32, dx: i32, dy: i32, speed: f32) -> Self {
        Self {
            x,
            y,
            dx,
            dy,
            speed,
            created_at: Instant::now(),
            max_distance: 50.0, // Max distance in tiles
            is_dead: false,
            projectile_type: ProjectileType::Arrow,
        }
    }

    pub fn new_with_type(
        x: f32,
        y: f32,
        dx: i32,
        dy: i32,
        speed: f32,
        projectile_type: ProjectileType,
    ) -> Self {
        Self {
            x,
            y,
            dx,
            dy,
            speed,
            created_at: Instant::now(),
            max_distance: 50.0,
            is_dead: false,
            projectile_type,
        }
    }

    pub fn get_glyph(&self) -> &'static str {
        match (self.dx, self.dy) {
            (1, 0) => "→",  // Right arrow
            (-1, 0) => "←", // Left arrow
            (0, 1) => "↓",  // Down arrow
            (0, -1) => "↑", // Up arrow
            _ => "*",
        }
    }

    pub fn is_alive(&self) -> bool {
        if self.is_dead {
            return false;
        }
        let elapsed = self.created_at.elapsed().as_secs_f32();
        let distance_traveled = elapsed * self.speed;
        distance_traveled < self.max_distance
    }

    pub fn update(&mut self, delta_time: f32) {
        if self.is_dead {
            return;
        }
        let movement = self.speed * delta_time;
        self.x += (self.dx as f32) * movement;
        self.y += (self.dy as f32) * movement;
    }

    #[allow(dead_code)]
    pub fn get_position(&self) -> (i32, i32) {
        (self.x.round() as i32, self.y.round() as i32)
    }

    pub fn stop(&mut self) {
        self.is_dead = true;
    }

    /// Get the explosion radius for this projectile when it hits
    pub fn get_impact_radius(&self) -> i32 {
        match self.projectile_type {
            ProjectileType::Arrow => 1,   // Single-target hit
            ProjectileType::FireOil => 4, // 4-tile radius explosion
        }
    }

    /// Calculate all tiles affected by this projectile's area damage
    pub fn get_impact_area(&self) -> Vec<(i32, i32)> {
        let x = self.x.round() as i32;
        let y = self.y.round() as i32;
        let radius = self.get_impact_radius();

        let mut affected = Vec::new();
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                if dx * dx + dy * dy <= radius * radius {
                    affected.push((x + dx, y + dy));
                }
            }
        }
        affected
    }
}
