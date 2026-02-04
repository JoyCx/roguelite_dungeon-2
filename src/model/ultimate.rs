use std::time::Instant;

#[derive(Clone, Debug)]
pub struct UltimateAnimation {
    pub created_at: Instant,
    pub duration: f32, // How long the animation lasts (expand + contract)
    pub max_radius: i32,
}

#[derive(Clone, Debug)]
pub struct Ultimate {
    pub radius: i32,             // Distance/radius of ultimate effect
    pub collision_enabled: bool, // Collides with walls
    pub cooldown_start: Option<Instant>,
    pub cooldown_duration: f32,
    pub damage: i32,
    pub animation: Option<UltimateAnimation>,
}

impl Default for Ultimate {
    fn default() -> Self {
        Self {
            radius: 3,
            collision_enabled: true,
            cooldown_start: None,
            cooldown_duration: 3.0,
            damage: 15,
            animation: None,
        }
    }
}

impl Ultimate {
    pub fn can_use(&self) -> bool {
        match self.cooldown_start {
            None => true,
            Some(start_time) => start_time.elapsed().as_secs_f32() >= self.cooldown_duration,
        }
    }

    pub fn start_cooldown(&mut self) {
        self.cooldown_start = Some(Instant::now());
    }

    pub fn start_animation(&mut self) {
        self.animation = Some(UltimateAnimation {
            created_at: Instant::now(),
            duration: 0.5, // 0.5 second animation
            max_radius: self.radius,
        });
    }

    pub fn is_animating(&self) -> bool {
        if let Some(anim) = &self.animation {
            anim.created_at.elapsed().as_secs_f32() < anim.duration
        } else {
            false
        }
    }

    pub fn get_animation_radius(&self) -> i32 {
        if let Some(anim) = &self.animation {
            let elapsed = anim.created_at.elapsed().as_secs_f32();
            let progress = (elapsed / anim.duration).clamp(0.0, 1.0);
            ((anim.max_radius as f32) * progress).ceil() as i32
        } else {
            0
        }
    }

    #[allow(dead_code)]
    pub fn cooldown_remaining(&self) -> f32 {
        match self.cooldown_start {
            None => 0.0,
            Some(start_time) => {
                let elapsed = start_time.elapsed().as_secs_f32();
                (self.cooldown_duration - elapsed).max(0.0)
            }
        }
    }

    pub fn get_affected_area(&self, player_x: i32, player_y: i32) -> Vec<(i32, i32)> {
        let mut area = Vec::new();
        let current_radius = self.get_animation_radius();

        for dx in -current_radius..=current_radius {
            for dy in -current_radius..=current_radius {
                // Skip player position
                if dx == 0 && dy == 0 {
                    continue;
                }

                let dist_sq = (dx * dx + dy * dy) as f32;
                let radius_sq = (current_radius as f32).powi(2);

                if dist_sq <= radius_sq {
                    area.push((player_x + dx, player_y + dy));
                }
            }
        }

        area
    }
}
