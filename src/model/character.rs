use std::time::Instant;

#[derive(Clone, Debug)]
pub struct Character {
    #[allow(dead_code)]
    pub speed: f32,

    pub last_direction: (i32, i32),

    pub dash_cooldown_start: Option<Instant>,

    pub dash_cooldown_duration: f32,

    pub dash_distance: i32,
}

impl Default for Character {
    fn default() -> Self {
        Self {
            speed: 5.0,
            last_direction: (0, 0),
            dash_cooldown_start: None,
            dash_cooldown_duration: 7.0,
            dash_distance: 5,
        }
    }
}

impl Character {
    #[allow(dead_code)]
    pub fn new(speed: f32) -> Self {
        Self {
            speed,
            ..Default::default()
        }
    }

    pub fn can_dash(&self) -> bool {
        match self.dash_cooldown_start {
            None => true,
            Some(start_time) => start_time.elapsed().as_secs_f32() >= self.dash_cooldown_duration,
        }
    }

    pub fn dash_cooldown_remaining(&self) -> f32 {
        match self.dash_cooldown_start {
            None => 0.0,
            Some(start_time) => {
                let elapsed = start_time.elapsed().as_secs_f32();
                (self.dash_cooldown_duration - elapsed).max(0.0)
            }
        }
    }

    pub fn start_dash_cooldown(&mut self) {
        self.dash_cooldown_start = Some(Instant::now());
    }

    pub fn update_direction(&mut self, dx: i32, dy: i32) {
        if dx != 0 || dy != 0 {
            self.last_direction = (dx, dy);
        }
    }
}
