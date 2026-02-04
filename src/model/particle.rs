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
        ((self.lifetime - elapsed) / self.lifetime).clamp(0.0, 1.0)
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

    #[allow(dead_code)] // Part of particle emission API
    pub fn emit(&mut self, particle: Particle) {
        self.particles.push(particle);
    }

    pub fn update(&mut self) {
        self.particles.retain(|p| p.is_alive());
    }

    #[allow(dead_code)] // Will be used when critical hits are displayed
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

    #[allow(dead_code)] // Will be used when healing occurs
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

    pub fn emit_glint(&mut self, x: f32, y: f32, color: ratatui::prelude::Color) {
        // Create a sparkle/glint effect
        let glyphs = &['*', '✦', '✧'];
        for i in 0..2 {
            let angle = (i as f32 / 2.0) * std::f32::consts::TAU;
            let dx = angle.cos() * 0.3;
            let dy = angle.sin() * 0.3;

            let glyph = glyphs[i % glyphs.len()];
            let particle = Particle::new(x + dx, y + dy, glyph, color, 0.3);
            self.particles.push(particle);
        }
    }

    pub fn emit_periodic_glint(&mut self, x: f32, y: f32, color: ratatui::prelude::Color) {
        // Emit glint only with 20% probability (for continuous glow effect)
        if rand::random::<f32>() < 0.2 {
            self.emit_glint(x, y, color);
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
