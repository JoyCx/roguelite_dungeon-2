use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Different types of ultimate abilities
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UltimateType {
    /// Rage: Player becomes faster (2x) and stronger (2x damage) for 30 seconds
    Rage,
    /// Shockwave: Deals damage in a wave that expands outward until hitting walls
    Shockwave,
    /// Ghost: Player becomes invulnerable for 10 seconds
    Ghost,
}

impl UltimateType {
    pub fn name(&self) -> &'static str {
        match self {
            UltimateType::Rage => "Rage",
            UltimateType::Shockwave => "Shockwave",
            UltimateType::Ghost => "Ghost",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            UltimateType::Rage => "Become 2x faster and stronger for 30 seconds",
            UltimateType::Shockwave => "Send out a shockwave dealing damage",
            UltimateType::Ghost => "Become invulnerable for 10 seconds",
        }
    }

    pub fn cooldown_duration(&self) -> f32 {
        match self {
            UltimateType::Rage => 45.0,
            UltimateType::Shockwave => 20.0,
            UltimateType::Ghost => 60.0,
        }
    }

    /// How long the ultimate effect lasts
    pub fn effect_duration(&self) -> f32 {
        match self {
            UltimateType::Rage => 30.0,
            UltimateType::Shockwave => 0.5, // Animation duration
            UltimateType::Ghost => 10.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct UltimateAnimation {
    pub created_at: Instant,
    pub duration: f32, // How long the animation lasts (expand + contract)
    pub max_radius: i32,
}

/// Tracks active ultimate effect on the player
#[derive(Clone, Debug)]
pub struct ActiveUltimate {
    pub ultimate_type: UltimateType,
    pub started_at: Instant,
    pub duration: f32,
}

impl ActiveUltimate {
    pub fn new(ultimate_type: UltimateType) -> Self {
        Self {
            ultimate_type: ultimate_type.clone(),
            started_at: Instant::now(),
            duration: ultimate_type.effect_duration(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.started_at.elapsed().as_secs_f32() < self.duration
    }

    pub fn remaining_time(&self) -> f32 {
        let elapsed = self.started_at.elapsed().as_secs_f32();
        (self.duration - elapsed).max(0.0)
    }
}

#[derive(Clone, Debug)]
pub struct Ultimate {
    pub radius: i32,             // Distance/radius of ultimate effect
    pub collision_enabled: bool, // Collides with walls
    pub cooldown_start: Option<Instant>,
    pub cooldown_duration: f32,
    pub damage: i32,
    pub animation: Option<UltimateAnimation>,
    pub current_type: UltimateType,
    pub active_ultimate: Option<ActiveUltimate>,
}

impl Default for Ultimate {
    fn default() -> Self {
        Self {
            radius: 3,
            collision_enabled: true,
            cooldown_start: None,
            cooldown_duration: UltimateType::Shockwave.cooldown_duration(),
            damage: 15,
            animation: None,
            current_type: UltimateType::Shockwave,
            active_ultimate: None,
        }
    }
}

impl Ultimate {
    #[allow(dead_code)]
    pub fn with_type(ultimate_type: UltimateType) -> Self {
        Self {
            radius: 3,
            collision_enabled: true,
            cooldown_start: None,
            cooldown_duration: ultimate_type.cooldown_duration(),
            damage: match ultimate_type {
                UltimateType::Rage => 0,       // Rage doesn't deal direct damage
                UltimateType::Shockwave => 25, // Shockwave deals high damage
                UltimateType::Ghost => 0,      // Ghost doesn't deal damage
            },
            animation: None,
            current_type: ultimate_type,
            active_ultimate: None,
        }
    }

    #[allow(dead_code)]
    pub fn change_type(&mut self, ultimate_type: UltimateType) {
        self.current_type = ultimate_type.clone();
        self.cooldown_duration = ultimate_type.cooldown_duration();
        self.damage = match ultimate_type {
            UltimateType::Rage => 0,
            UltimateType::Shockwave => 25,
            UltimateType::Ghost => 0,
        };
    }

    pub fn can_use(&self) -> bool {
        match self.cooldown_start {
            None => true,
            Some(start_time) => start_time.elapsed().as_secs_f32() >= self.cooldown_duration,
        }
    }

    pub fn start_cooldown(&mut self) {
        self.cooldown_start = Some(Instant::now());
        self.cooldown_duration = self.current_type.cooldown_duration();
    }

    #[allow(dead_code)]
    pub fn activate(&mut self) {
        self.active_ultimate = Some(ActiveUltimate::new(self.current_type.clone()));
        self.start_animation();
    }

    #[allow(dead_code)]
    pub fn is_active(&self) -> bool {
        if let Some(active) = &self.active_ultimate {
            active.is_active()
        } else {
            false
        }
    }

    pub fn get_active_type(&self) -> Option<&UltimateType> {
        if let Some(active) = &self.active_ultimate {
            if active.is_active() {
                return Some(&active.ultimate_type);
            }
        }
        None
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

    /// Get shockwave radius - expands until it hits walls
    #[allow(dead_code)]
    pub fn get_shockwave_reach(
        &self,
        player_x: i32,
        player_y: i32,
        max_reach: i32,
    ) -> Vec<(i32, i32)> {
        // Returns all tiles in expanding radius from player
        // Collision checking should be done by the caller
        let mut area = Vec::new();
        let current_radius = self.get_animation_radius().min(max_reach);

        for dx in -current_radius..=current_radius {
            for dy in -current_radius..=current_radius {
                // Skip center
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

    /// Charge the ultimate ability based on damage dealt
    /// Charging is a percentage of damage (e.g., 5% of damage = 5% charge)
    pub fn charge_on_hit(&self, damage: i32) -> f32 {
        // Charge percentage = 5% of damage dealt
        (damage as f32 * 0.05).min(15.0) // Cap at 15% per hit to prevent infinite charging
    }
}
