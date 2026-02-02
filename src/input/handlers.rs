use crate::app::{App, AppState};
use crate::model::settings::Settings;
use crossterm::event::KeyCode;

pub fn handle_main_menu_input(app: &mut App, key: crossterm::event::KeyEvent) {
    // Skip auto-scroll animation on any key press
    app.skip_auto_scroll();

    match key.code {
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
            super::menu::move_selection_up(&mut app.main_menu_state, 5);
        }
        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
            super::menu::move_selection_down(&mut app.main_menu_state, 5);
        }
        KeyCode::Enter | KeyCode::Char(' ') => match app.main_menu_state.selected() {
            Some(0) => app.state = AppState::Game,
            Some(2) => app.state = AppState::Settings,
            Some(3) => app.state = AppState::DevMenu,
            Some(4) => app.should_quit = true,
            _ => {}
        },
        _ => {}
    }
}

pub fn handle_settings_input(app: &mut App, key: crossterm::event::KeyEvent) {
    match app.settings_mode {
        crate::app::SettingsMode::Navigating => match key.code {
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                super::menu::move_selection_up(&mut app.settings_state, 14);
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                super::menu::move_selection_down(&mut app.settings_state, 14);
            }
            KeyCode::PageUp => app.set_scroll(app.scroll_offset.saturating_sub(5)),
            KeyCode::PageDown => app.set_scroll(app.scroll_offset.saturating_add(5)),
            KeyCode::Enter | KeyCode::Char(' ') => {
                handle_settings_selection(app);
            }
            KeyCode::Esc => {
                app.temp_settings = app.settings.clone();
                app.state = AppState::MainMenu;
            }
            _ => {}
        },
        crate::app::SettingsMode::Rebinding => {
            // Escape cancels rebinding without saving
            if key.code == KeyCode::Esc {
                app.settings_mode = crate::app::SettingsMode::Navigating;
                return;
            }

            let k = format!("{:?}", key.code);
            match app.settings_state.selected().unwrap_or(0) {
                0 => app.temp_settings.move_up = k,
                1 => app.temp_settings.move_left = k,
                2 => app.temp_settings.move_down = k,
                3 => app.temp_settings.move_right = k,
                4 => app.temp_settings.attack = k,
                5 => app.temp_settings.dash = k,
                6 => app.temp_settings.block = k,
                7 => app.temp_settings.pick_up = k,
                8 => app.temp_settings.toggle_inv = k,
                9 => app.temp_settings.special_item = k,
                _ => {}
            }
            app.settings_mode = crate::app::SettingsMode::Navigating;
        }
    }
}

fn key_matches(key_code: KeyCode, key_string: &str) -> bool {
    match key_string {
        "W" => matches!(key_code, KeyCode::Char('w') | KeyCode::Char('W')),
        "A" => matches!(key_code, KeyCode::Char('a') | KeyCode::Char('A')),
        "S" => matches!(key_code, KeyCode::Char('s') | KeyCode::Char('S')),
        "D" => matches!(key_code, KeyCode::Char('d') | KeyCode::Char('D')),
        "Q" => matches!(key_code, KeyCode::Char('q') | KeyCode::Char('Q')),
        "E" => matches!(key_code, KeyCode::Char('e') | KeyCode::Char('E')),
        "C" => matches!(key_code, KeyCode::Char('c') | KeyCode::Char('C')),
        "Space" => matches!(key_code, KeyCode::Char(' ')),
        "Enter" => matches!(key_code, KeyCode::Enter),
        "LeftClick" => false,  // Not handled via KeyCode
        "RightClick" => false, // Not handled via KeyCode
        _ => {
            // Try to parse as single char
            if key_string.len() == 1 {
                if let Some(c) = key_string.chars().next() {
                    matches!(key_code, KeyCode::Char(ch) if ch.to_lowercase().eq(c.to_lowercase()))
                } else {
                    false
                }
            } else {
                false
            }
        }
    }
}

pub fn handle_game_input(app: &mut App, key: crossterm::event::KeyEvent) {
    match key.code {
        KeyCode::Esc => app.state = AppState::MainMenu,
        _ => {
            let settings = &app.settings;
            if key_matches(key.code, &settings.move_up) {
                app.move_character(0, -1);
            } else if key_matches(key.code, &settings.move_down) {
                app.move_character(0, 1);
            } else if key_matches(key.code, &settings.move_left) {
                app.move_character(-1, 0);
            } else if key_matches(key.code, &settings.move_right) {
                app.move_character(1, 0);
            } else if key_matches(key.code, &settings.dash) {
                app.dash();
            }
        }
    }
}

fn handle_settings_selection(app: &mut App) {
    let sel = app.settings_state.selected().unwrap_or(0);
    match sel {
        0..=9 => app.settings_mode = crate::app::SettingsMode::Rebinding,
        11 => {
            app.settings = app.temp_settings.clone();
            let _ = app.settings.save();
            app.state = AppState::MainMenu;
        }
        12 => {
            app.temp_settings = app.settings.clone();
            app.state = AppState::MainMenu;
        }
        13 => {
            // Reset to default settings
            app.settings = Settings::default();
            app.temp_settings = app.settings.clone();
            let _ = app.settings.save();
            app.state = AppState::MainMenu;
        }
        _ => {}
    }
}

pub fn handle_dev_menu_input(app: &mut App, key: crossterm::event::KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.state = AppState::MainMenu;
        }
        KeyCode::Char('r') | KeyCode::Char('R') => {
            app.roll_random_seed();
        }
        KeyCode::Char(c) if c.is_ascii_digit() => {
            if app.dev_seed_input.len() < 20 {
                // Max u64 string length
                app.dev_seed_input.push(c);
            }
        }
        KeyCode::Backspace => {
            app.dev_seed_input.pop();
        }
        KeyCode::Enter => {
            app.regenerate_floor();
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            // Enter play mode if a floor has been generated
            if let Some(floor) = &app.current_floor {
                // Find a walkable starting position
                if let Some((x, y)) = floor.find_walkable_tile() {
                    use std::io::Write;
                    let msg = format!("Starting position: ({}, {})\n", x, y);
                    let _ = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("log.txt")
                        .and_then(|mut f| f.write_all(msg.as_bytes()));

                    app.character_position = (x, y);
                    app.update_camera(); // Initialize camera position
                    app.state = AppState::Game;
                } else {
                    use std::io::Write;
                    let _ = std::fs::OpenOptions::new()
                        .create(true)
                        .append(true)
                        .open("log.txt")
                        .and_then(|mut f| f.write_all(b"ERROR: No walkable tile found!\n"));
                }
            }
        }
        _ => {}
    }
}
