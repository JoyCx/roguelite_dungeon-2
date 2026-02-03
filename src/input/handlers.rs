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
            Some(0) => {
                // Start Game - go to character creation
                app.state = AppState::CharacterCreation;
                app.char_name = String::new();
                app.char_name_input_mode = true;
                app.char_creation_selection = 0;
                // Load default difficulty from settings
                app.settings.difficulty = app.settings.default_difficulty.clone();
            }
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
                super::menu::move_selection_up(&mut app.settings_state, 19);
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                super::menu::move_selection_down(&mut app.settings_state, 19);
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

            // Prevent binding to RightClick
            if format!("{:?}", key.code) == "RightClick" {
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
                7 => app.temp_settings.toggle_inv = k,
                8 => app.temp_settings.inventory_up = k,
                9 => app.temp_settings.inventory_down = k,
                10 => app.temp_settings.item_describe = k,
                11 => app.temp_settings.pause = k,
                12 => app.temp_settings.special_item = k,
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
        "R" => matches!(key_code, KeyCode::Char('r') | KeyCode::Char('R')),
        "T" => matches!(key_code, KeyCode::Char('t') | KeyCode::Char('T')),
        "F" => matches!(key_code, KeyCode::Char('f') | KeyCode::Char('F')),
        "Space" => matches!(key_code, KeyCode::Char(' ')),
        "Enter" | "Return" => matches!(key_code, KeyCode::Enter),
        "Up" => matches!(key_code, KeyCode::Up),
        "Down" => matches!(key_code, KeyCode::Down),
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
    // If showing item description, only allow Esc to close it
    if app.showing_item_description {
        if key.code == KeyCode::Esc {
            app.showing_item_description = false;
        }
        return;
    }

    let settings = &app.settings;

    // Check for pause key - can be pressed anytime during gameplay
    if key_matches(key.code, &settings.pause) {
        app.is_paused = !app.is_paused;
        return;
    }

    match key.code {
        KeyCode::Esc => {
            // If inventory is focused, unfocus it instead of going to menu
            if app.inventory_focused {
                app.inventory_focused = false;
                app.inventory_scroll_index = 0;
            } else {
                app.state = AppState::MainMenu;
            }
        }
        KeyCode::Char(c) if c.is_ascii_digit() && c != '0' => {
            // Check if shift is pressed for consumable usage
            if key
                .modifiers
                .contains(crossterm::event::KeyModifiers::SHIFT)
            {
                let slot = c.to_digit(10).unwrap() as usize;
                app.use_consumable(slot - 1); // Convert 1-9 to 0-8
            } else {
                let slot = c.to_digit(10).unwrap() as usize;
                app.switch_weapon(slot);
            }
        }
        _ => {
            let is_movement = key_matches(key.code, &settings.move_up)
                || key_matches(key.code, &settings.move_down)
                || key_matches(key.code, &settings.move_left)
                || key_matches(key.code, &settings.move_right);

            // Handle inventory navigation - always allowed
            if app.inventory_focused && key_matches(key.code, &settings.inventory_up) {
                if app.inventory_scroll_index > 0 {
                    app.inventory_scroll_index -= 1;
                }
                return;
            } else if app.inventory_focused && key_matches(key.code, &settings.inventory_down) {
                app.inventory_scroll_index += 1;
                return;
            } else if app.inventory_focused && key_matches(key.code, &settings.toggle_inv) {
                app.inventory_focused = false;
                app.inventory_scroll_index = 0;
                return;
            } else if app.inventory_focused && key_matches(key.code, &settings.item_describe) {
                app.showing_item_description = true;
                return;
            }

            // Block movement only while inventory focused
            if app.inventory_focused && is_movement {
                return;
            }

            // Process all game actions (works with or without inventory focus)
            if is_movement {
                if key_matches(key.code, &settings.move_up) {
                    app.move_character(0, -1);
                } else if key_matches(key.code, &settings.move_down) {
                    app.move_character(0, 1);
                } else if key_matches(key.code, &settings.move_left) {
                    app.move_character(-1, 0);
                } else if key_matches(key.code, &settings.move_right) {
                    app.move_character(1, 0);
                }
            } else if key_matches(key.code, &settings.dash) {
                app.dash();
            } else if key_matches(key.code, &settings.attack) {
                app.use_current_weapon();
            } else if key_matches(key.code, &settings.special_item) {
                app.use_ultimate();
            } else if key_matches(key.code, &settings.toggle_inv) {
                app.inventory_focused = true;
            } else if key.code == KeyCode::Char('f') || key.code == KeyCode::Char('F') {
                app.block();
            }
        }
    }
}

fn handle_settings_selection(app: &mut App) {
    let sel = app.settings_state.selected().unwrap_or(0);
    match sel {
        0..=12 => app.settings_mode = crate::app::SettingsMode::Rebinding,
        13 => {
            // Difficulty toggle (current difficulty)
            app.temp_settings.difficulty = match &app.temp_settings.difficulty {
                crate::model::item_tier::Difficulty::Easy => {
                    crate::model::item_tier::Difficulty::Normal
                }
                crate::model::item_tier::Difficulty::Normal => {
                    crate::model::item_tier::Difficulty::Hard
                }
                crate::model::item_tier::Difficulty::Hard => {
                    crate::model::item_tier::Difficulty::Death
                }
                crate::model::item_tier::Difficulty::Death => {
                    crate::model::item_tier::Difficulty::Easy
                }
            };
        }
        14 => {
            // Default difficulty toggle
            app.temp_settings.default_difficulty = match &app.temp_settings.default_difficulty {
                crate::model::item_tier::Difficulty::Easy => {
                    crate::model::item_tier::Difficulty::Normal
                }
                crate::model::item_tier::Difficulty::Normal => {
                    crate::model::item_tier::Difficulty::Hard
                }
                crate::model::item_tier::Difficulty::Hard => {
                    crate::model::item_tier::Difficulty::Death
                }
                crate::model::item_tier::Difficulty::Death => {
                    crate::model::item_tier::Difficulty::Easy
                }
            };
        }
        16 => {
            app.settings = app.temp_settings.clone();
            let _ = app.settings.save();
            app.state = AppState::MainMenu;
        }
        17 => {
            app.temp_settings = app.settings.clone();
            app.state = AppState::MainMenu;
        }
        18 => {
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
    // Call the dev menu handler
    crate::ui::dev_menu::handle_input(app, key.code.clone());

    match key.code {
        KeyCode::Esc => {
            app.state = AppState::MainMenu;
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

pub fn handle_character_creation_input(app: &mut App, key: crossterm::event::KeyEvent) {
    if app.char_name_input_mode {
        // In text input mode for character name
        match key.code {
            KeyCode::Tab => {
                // Tab switches to difficulty selection if name is not empty
                if !app.char_name.is_empty() {
                    app.char_name_input_mode = false;
                    app.char_creation_selection = 1;
                }
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                // Down arrow/S also switches to difficulty if name is not empty
                if !app.char_name.is_empty() {
                    app.char_name_input_mode = false;
                    app.char_creation_selection = 1;
                }
            }
            KeyCode::Enter => {
                if !app.char_name.is_empty() {
                    // Move to difficulty selection
                    app.char_name_input_mode = false;
                    app.char_creation_selection = 1;
                }
            }
            KeyCode::Backspace => {
                app.char_name.pop();
            }
            KeyCode::Esc => {
                app.state = AppState::MainMenu;
            }
            KeyCode::Char(c) if app.char_name.len() < 20 => {
                app.char_name.push(c);
            }
            _ => {}
        }
    } else {
        // Navigation mode (difficulty and start button)
        match key.code {
            KeyCode::Tab => {
                // Tab cycles forward through fields
                app.char_creation_selection = (app.char_creation_selection + 1) % 3;
                // If cycling back to name, enter input mode
                if app.char_creation_selection == 0 {
                    app.char_name_input_mode = true;
                }
            }
            KeyCode::BackTab => {
                // Shift+Tab cycles backward through fields
                if app.char_creation_selection == 0 {
                    app.char_creation_selection = 2;
                } else {
                    app.char_creation_selection -= 1;
                }
                // If cycling back to name, enter input mode
                if app.char_creation_selection == 0 {
                    app.char_name_input_mode = true;
                }
            }
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                if app.char_creation_selection > 0 {
                    app.char_creation_selection -= 1;
                    // If going back to name, enter input mode
                    if app.char_creation_selection == 0 {
                        app.char_name_input_mode = true;
                    }
                }
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                if app.char_creation_selection < 2 {
                    app.char_creation_selection += 1;
                }
            }
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                // Cycle difficulty backwards
                if app.char_creation_selection == 1 {
                    app.settings.difficulty = match &app.settings.difficulty {
                        crate::model::item_tier::Difficulty::Easy => {
                            crate::model::item_tier::Difficulty::Death
                        }
                        crate::model::item_tier::Difficulty::Normal => {
                            crate::model::item_tier::Difficulty::Easy
                        }
                        crate::model::item_tier::Difficulty::Hard => {
                            crate::model::item_tier::Difficulty::Normal
                        }
                        crate::model::item_tier::Difficulty::Death => {
                            crate::model::item_tier::Difficulty::Hard
                        }
                    };
                }
            }
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                // Cycle difficulty forwards
                if app.char_creation_selection == 1 {
                    app.settings.difficulty = match &app.settings.difficulty {
                        crate::model::item_tier::Difficulty::Easy => {
                            crate::model::item_tier::Difficulty::Normal
                        }
                        crate::model::item_tier::Difficulty::Normal => {
                            crate::model::item_tier::Difficulty::Hard
                        }
                        crate::model::item_tier::Difficulty::Hard => {
                            crate::model::item_tier::Difficulty::Death
                        }
                        crate::model::item_tier::Difficulty::Death => {
                            crate::model::item_tier::Difficulty::Easy
                        }
                    };
                }
            }
            KeyCode::Enter | KeyCode::Char(' ') => {
                if app.char_creation_selection == 2 {
                    // Start the game
                    app.character.name = app.char_name.clone();
                    app.regenerate_floor();
                    if let Some(floor) = &app.current_floor {
                        // Find a random spawn position in the main connected region
                        if let Some((x, y)) = floor.find_player_spawn() {
                            app.character_position = (x, y);
                            app.update_camera(); // Initialize camera position
                            app.state = AppState::Game;
                        }
                    }
                } else if app.char_creation_selection == 0 {
                    // Go back to name input
                    app.char_name_input_mode = true;
                }
            }
            KeyCode::Esc => {
                app.state = AppState::MainMenu;
            }
            _ => {}
        }
    }
}
