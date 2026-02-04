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
