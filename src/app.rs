use crate::model::character::Character;
use crate::model::floor::Floor;
use crate::model::settings::Settings;
use ratatui::widgets::ListState;
use std::time::Instant;

pub enum AppState {
    MainMenu,
    Settings,
    Game,
    DevMenu,
}

#[derive(PartialEq)]
pub enum SettingsMode {
    Navigating,
    Rebinding,
}

pub struct App {
    pub state: AppState,
    pub settings: Settings,
    pub temp_settings: Settings,
    pub settings_mode: SettingsMode,
    pub main_menu_state: ListState,
    pub settings_state: ListState,
    pub should_quit: bool,
    pub scroll_offset: u16,
    pub last_scroll_offset: u16,
    pub start_time: Instant,
    pub frame_count: u64,
    pub is_auto_scrolling: bool,
    pub scroll_target: f32,
    pub dev_seed_input: String,
    pub current_floor: Option<Floor>,
    pub character_position: (i32, i32),
    pub character: Character,
    pub terminal_size: (u16, u16),
    pub camera_offset: (f32, f32),
    pub camera_target: (f32, f32),
    pub last_game_tick: Instant,
    pub game_tick_rate_ms: u128,
}

impl App {
    pub fn new() -> Self {
        let s = Settings::load();
        let mut menu_s = ListState::default();
        menu_s.select(Some(0));
        let mut set_s = ListState::default();
        set_s.select(Some(0));

        let now = Instant::now();

        Self {
            state: AppState::MainMenu,
            settings: s.clone(),
            temp_settings: s,
            settings_mode: SettingsMode::Navigating,
            main_menu_state: menu_s,
            settings_state: set_s,
            should_quit: false,
            scroll_offset: 0,
            is_auto_scrolling: true,
            scroll_target: 60.0,
            last_scroll_offset: 0,
            start_time: now,
            frame_count: 0,
            dev_seed_input: String::new(),
            current_floor: None,
            character_position: (0, 0),
            character: Character::default(),
            terminal_size: (0, 0),
            camera_offset: (0.0, 0.0),
            camera_target: (0.0, 0.0),
            last_game_tick: now,
            game_tick_rate_ms: 100,
        }
    }

    pub fn set_scroll(&mut self, offset: u16) {
        let max_scroll = 60;
        self.scroll_offset = offset.min(max_scroll);
        self.last_scroll_offset = self.scroll_offset;
        self.is_auto_scrolling = false;
    }

    pub fn update_auto_scroll(&mut self) {
        if !self.is_auto_scrolling {
            return;
        }

        let distance = self.scroll_target - self.scroll_offset as f32;

        let speed_factor = 0.05;

        if distance.abs() > 0.5 {
            self.last_scroll_offset = self.scroll_offset;
            let step = distance * speed_factor;
            self.scroll_offset = (self.scroll_offset as f32 + step).round() as u16;
        } else {
            self.last_scroll_offset = self.scroll_offset;
            self.scroll_offset = self.scroll_target as u16;
            self.is_auto_scrolling = false;
        }
    }

    pub fn skip_auto_scroll(&mut self) {
        if self.is_auto_scrolling {
            self.last_scroll_offset = self.scroll_offset;
            self.scroll_offset = self.scroll_target as u16;
            self.is_auto_scrolling = false;
        }
    }

    pub fn regenerate_floor(&mut self) {
        let seed = self.dev_seed_input.parse::<u64>().unwrap_or(0);
        self.current_floor = Some(Floor::new(180, 60, seed));

        if let Some(floor) = &self.current_floor {
            let mut found = false;
            for y in 0..floor.height {
                for x in 0..floor.width {
                    if floor.is_walkable(x, y) {
                        self.character_position = (x, y);
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }

            self.update_camera();
        }
    }

    pub fn roll_random_seed(&mut self) {
        use rand::Rng;
        let new_seed: u64 = rand::rng().random_range(0..=u64::MAX);
        self.dev_seed_input = new_seed.to_string();
        self.regenerate_floor();
    }

    pub fn update_terminal_size(&mut self, width: u16, height: u16) {
        self.terminal_size = (width, height);
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        if let Some(floor) = &self.current_floor {
            floor.is_walkable(x, y)
        } else {
            true
        }
    }

    pub fn should_tick(&self) -> bool {
        self.last_game_tick.elapsed().as_millis() >= self.game_tick_rate_ms
    }

    pub fn consume_tick(&mut self) {
        self.last_game_tick = Instant::now();
    }

    pub fn move_character(&mut self, dx: i32, dy: i32) {
        use std::io::Write;

        if !self.should_tick() {
            return;
        }

        let new_x = self.character_position.0 + dx;
        let new_y = self.character_position.1 + dy;

        let msg = format!(
            "\n=== MOVE ATTEMPT ===\nCurrent pos: ({}, {})\nTarget pos: ({}, {})\n",
            self.character_position.0, self.character_position.1, new_x, new_y
        );
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")
            .and_then(|mut f| f.write_all(msg.as_bytes()));

        if self.is_walkable(new_x, new_y) {
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .and_then(|mut f| f.write_all(b"MOVE SUCCESSFUL\n"));

            self.character_position = (new_x, new_y);
            self.character.update_direction(dx, dy);
            self.update_camera();
            self.consume_tick();
        } else {
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .and_then(|mut f| f.write_all(b"MOVE BLOCKED\n"));
        }
    }

    pub fn dash(&mut self) {
        use std::io::Write;

        if !self.character.can_dash() || !self.should_tick() {
            return;
        }

        let (dx, dy) = self.character.last_direction;

        if dx == 0 && dy == 0 {
            return;
        }

        let dash_dist = self.character.dash_distance;
        let new_x = self.character_position.0 + (dx * dash_dist);
        let new_y = self.character_position.1 + (dy * dash_dist);

        let msg = format!(
            "\n=== DASH ATTEMPT ===\nCurrent pos: ({}, {})\nDash dir: ({}, {})\nTarget pos: ({}, {})\n",
            self.character_position.0, self.character_position.1, dx, dy, new_x, new_y
        );
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("log.txt")
            .and_then(|mut f| f.write_all(msg.as_bytes()));

        if self.is_walkable(new_x, new_y) {
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .and_then(|mut f| f.write_all(b"DASH SUCCESSFUL\n"));

            self.character_position = (new_x, new_y);
            self.character.start_dash_cooldown();
            self.update_camera();
            self.consume_tick();
        } else {
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.txt")
                .and_then(|mut f| f.write_all(b"DASH BLOCKED BY WALL\n"));
        }
    }

    pub fn update_camera(&mut self) {
        let vw = self.terminal_size.0 as f32;
        let vh = self.terminal_size.1 as f32;

        let mut target_x = self.character_position.0 as f32 - vw / 2.0;
        let mut target_y = self.character_position.1 as f32 - vh / 2.0;

        if let Some(floor) = &self.current_floor {
            target_x = target_x.clamp(0.0, (floor.width as f32 - vw).max(0.0));
            target_y = target_y.clamp(0.0, (floor.height as f32 - vh).max(0.0));
        }

        self.camera_target = (target_x, target_y);
    }

    pub fn update_camera_smooth(&mut self) {
        let dx = self.camera_target.0 - self.camera_offset.0;
        let dy = self.camera_target.1 - self.camera_offset.1;

        let ease_factor = 0.05;

        if dx.abs() > 0.1 || dy.abs() > 0.1 {
            self.camera_offset.0 += dx * ease_factor;
            self.camera_offset.1 += dy * ease_factor;
        } else {
            self.camera_offset = self.camera_target;
        }
    }
}
