use std::time::Instant;

/// Generic cooldown tracker for abilities and actions
/// Encapsulates the timing logic for managing ability cooldowns
#[derive(Clone, Debug)]
pub struct Cooldown {
    /// When the cooldown was last started
    start_time: Option<Instant>,
    /// Duration of the cooldown in seconds
    duration: f32,
}

impl Cooldown {
    /// Create a new cooldown with the specified duration
    pub fn new(duration: f32) -> Self {
        Self {
            start_time: None,
            duration,
        }
    }

    /// Check if the cooldown is ready (not on cooldown)
    pub fn is_ready(&self) -> bool {
        match self.start_time {
            None => true,
            Some(start) => start.elapsed().as_secs_f32() >= self.duration,
        }
    }

    /// Get remaining cooldown time in seconds (0 if ready)
    pub fn remaining_seconds(&self) -> f32 {
        match self.start_time {
            None => 0.0,
            Some(start) => {
                let elapsed = start.elapsed().as_secs_f32();
                (self.duration - elapsed).max(0.0)
            }
        }
    }

    /// Get cooldown progress as a fraction [0.0, 1.0]
    /// Returns 1.0 when ready, 0.0 when just started
    pub fn progress(&self) -> f32 {
        match self.start_time {
            None => 1.0,
            Some(start) => {
                let elapsed = start.elapsed().as_secs_f32();
                (elapsed / self.duration).min(1.0)
            }
        }
    }

    /// Trigger the cooldown (start the timer)
    pub fn trigger(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Reset the cooldown (make it immediately ready)
    pub fn reset(&mut self) {
        self.start_time = None;
    }

    /// Set the cooldown duration
    pub fn set_duration(&mut self, duration: f32) {
        self.duration = duration;
    }

    /// Get the current duration
    pub fn duration(&self) -> f32 {
        self.duration
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_new_cooldown_is_ready() {
        let cooldown = Cooldown::new(1.0);
        assert!(cooldown.is_ready());
    }

    #[test]
    fn test_triggered_cooldown_not_ready() {
        let mut cooldown = Cooldown::new(0.1);
        cooldown.trigger();
        assert!(!cooldown.is_ready());
    }

    #[test]
    fn test_cooldown_becomes_ready() {
        let mut cooldown = Cooldown::new(0.1);
        cooldown.trigger();
        thread::sleep(std::time::Duration::from_millis(150));
        assert!(cooldown.is_ready());
    }

    #[test]
    fn test_reset_cooldown() {
        let mut cooldown = Cooldown::new(1.0);
        cooldown.trigger();
        assert!(!cooldown.is_ready());
        cooldown.reset();
        assert!(cooldown.is_ready());
    }

    #[test]
    fn test_progress() {
        let cooldown = Cooldown::new(1.0);
        assert_eq!(cooldown.progress(), 1.0);

        let mut cooldown = Cooldown::new(0.2);
        cooldown.trigger();
        thread::sleep(std::time::Duration::from_millis(100));
        let progress = cooldown.progress();
        assert!(progress > 0.4 && progress < 0.6);
    }
}
