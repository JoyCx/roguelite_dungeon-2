use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Particle {
    pub x: f32,
    pub y: f32,
    pub glyph: char,
    pub color: ratatui::prelude::Color,
    pub created_at: Instant,
    pub lifetime: f32, // seconds
}

impl Particle {
    pub fn new(x: f32, y: f32, glyph: char, color: ratatui::prelude::Color, lifetime: f32) -> Self {
        Self {
            x,
            y,
            glyph,
            color,
            created_at: Instant::now(),
            lifetime,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.created_at.elapsed().as_secs_f32() < self.lifetime
    }

    pub fn get_alpha(&self) -> f32 {
        let elapsed = self.created_at.elapsed().as_secs_f32();
        ((self.lifetime - elapsed) / self.lifetime)
            .max(0.0)
            .min(1.0)
    }
}

#[derive(Clone, Debug, Default)]
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn emit(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn update(&mut self) {
        self.particles.retain(|p| p.is_alive());
    }

    pub fn emit_impact(&mut self, x: f32, y: f32, radius: i32, color: ratatui::prelude::Color) {
        // Create a burst of particles in a circle
        for angle_steps in 0..8 {
            let angle = (angle_steps as f32 / 8.0) * std::f32::consts::TAU;
            let dx = angle.cos() * radius as f32;
            let dy = angle.sin() * radius as f32;

            let glyph = match angle_steps {
                0 => '→',
                1 => '↘',
                2 => '↓',
                3 => '↙',
                4 => '←',
                5 => '↖',
                6 => '↑',
                7 => '↗',
                _ => '*',
            };

            let particle = Particle::new(x + dx, y + dy, glyph, color, 0.3);
            self.particles.push(particle);
        }
    }

    pub fn emit_crit(&mut self, x: f32, y: f32) {
        // Create upward-flying crit indicators
        for offset in -1..=1 {
            let particle = Particle::new(
                x + offset as f32,
                y,
                '*',
                ratatui::prelude::Color::Yellow,
                0.5,
            );
            self.particles.push(particle);
        }
    }

    pub fn emit_heal(&mut self, x: f32, y: f32) {
        // Create healing indicators
        for i in 0..3 {
            let particle = Particle::new(
                x + (i as f32 - 1.0) * 0.5,
                y,
                '+',
                ratatui::prelude::Color::Green,
                0.4,
            );
            self.particles.push(particle);
        }
    }

    pub fn get_active_particles(&self) -> Vec<(i32, i32, char, ratatui::prelude::Color)> {
        self.particles
            .iter()
            .filter(|p| p.is_alive())
            .map(|p| (p.x.round() as i32, p.y.round() as i32, p.glyph, p.color))
            .collect()
    }
}
