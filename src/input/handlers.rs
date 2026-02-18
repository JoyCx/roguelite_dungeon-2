use crate::app::{App, AppState};
use crate::model::audio::SoundEffect;
use crate::model::settings::Settings;
use crossterm::event::KeyCode;

pub fn handle_main_menu_input(app: &mut App, key: crossterm::event::KeyEvent) {
    // Skip auto-scroll animation on any key press
    app.skip_auto_scroll();

    match key.code {
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
            super::menu::move_selection_up(&mut app.main_menu_state, 5);
            app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
        }
        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
            super::menu::move_selection_down(&mut app.main_menu_state, 5);
            app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            app.audio_manager.play_sound_effect(SoundEffect::MenuPick);
            match app.main_menu_state.selected() {
                Some(0) => {
                    // Start Game - go to character creation
                    app.state = AppState::CharacterCreation;
                    app.char_name = String::new();
                    app.char_name_input_mode = true;
                    app.char_creation_selection = 0;
                    // Load default difficulty from settings
                    app.settings.difficulty = app.settings.default_difficulty.clone();
                }
                Some(1) => {
                    // Load Save - transition to save selection screen (always, even with no saves)
                    if let Ok(saves) = crate::model::gamesave::GameSave::list_saves() {
                        app.available_saves = saves;
                    } else {
                        app.available_saves.clear();
                    }
                    app.save_selection_state.select(Some(0));
                    app.state = AppState::SaveSelection;
                }
                Some(2) => app.state = AppState::Settings,
                Some(3) => app.state = AppState::DevMenu,
                Some(4) => app.should_quit = true,
                _ => {}
            }
        }
        _ => {}
    }
}

pub fn handle_save_selection_input(app: &mut App, key: crossterm::event::KeyEvent) {
    match key.code {
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
            super::menu::move_selection_up(
                &mut app.save_selection_state,
                app.available_saves.len(),
            );
            app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
        }
        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
            super::menu::move_selection_down(
                &mut app.save_selection_state,
                app.available_saves.len(),
            );
            app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            app.audio_manager.play_sound_effect(SoundEffect::MenuPick);
            if let Some(index) = app.save_selection_state.selected() {
                if index < app.available_saves.len() {
                    let save_name = app.available_saves[index].clone();
                    // Try to load the game - proceed regardless due to errors
                    let _ = app.load_game(&save_name);

                    // Always transition to Game state
                    app.state = AppState::Game;
                    app.arrows.clear();
                    app.active_animations.clear();
                    app.particle_system = crate::model::particle::ParticleSystem::new();
                    app.is_paused = false;
                    app.regenerate_floor();
                    app.audio_manager.stop_music();
                    let _ = app.audio_manager.start_music_with_fade_in();
                }
            }
        }
        KeyCode::Esc => {
            app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
            app.state = AppState::MainMenu;
        }
        _ => {}
    }
}

pub fn handle_settings_input(app: &mut App, key: crossterm::event::KeyEvent) {
    match app.settings_mode {
        crate::app::SettingsMode::Navigating => match key.code {
            KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
                super::menu::move_selection_up(&mut app.settings_state, 23);
                app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
            }
            KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
                super::menu::move_selection_down(&mut app.settings_state, 23);
                app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
            }
            KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
                let sel = app.settings_state.selected().unwrap_or(0);
                // Music volume
                if sel == 16 {
                    app.temp_settings.music_volume =
                        (app.temp_settings.music_volume - 0.05).max(0.0);
                    app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
                }
                // Sound volume
                else if sel == 17 {
                    app.temp_settings.sound_volume =
                        (app.temp_settings.sound_volume - 0.05).max(0.0);
                    app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
                }
            }
            KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
                let sel = app.settings_state.selected().unwrap_or(0);
                // Music volume
                if sel == 16 {
                    app.temp_settings.music_volume =
                        (app.temp_settings.music_volume + 0.05).min(1.0);
                    app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
                }
                // Sound volume
                else if sel == 17 {
                    app.temp_settings.sound_volume =
                        (app.temp_settings.sound_volume + 0.05).min(1.0);
                    app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
                }
            }
            KeyCode::PageUp => app.set_scroll(app.scroll_offset.saturating_sub(5)),
            KeyCode::PageDown => app.set_scroll(app.scroll_offset.saturating_add(5)),
            KeyCode::Enter | KeyCode::Char(' ') => {
                app.audio_manager.play_sound_effect(SoundEffect::MenuPick);
                handle_settings_selection(app);
            }
            KeyCode::Esc => {
                app.audio_manager.play_sound_effect(SoundEffect::MenuPick);
                app.temp_settings = app.settings.clone();
                app.state = AppState::MainMenu;
            }
            _ => {}
        },
        crate::app::SettingsMode::Rebinding => {
            // Escape cancels rebinding without saving
            if key.code == KeyCode::Esc {
                app.audio_manager.play_sound_effect(SoundEffect::MenuPick);
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
                8 => app.temp_settings.use_consumable = k,
                9 => app.temp_settings.inventory_up = k,
                10 => app.temp_settings.inventory_down = k,
                11 => app.temp_settings.item_describe = k,
                12 => app.temp_settings.pause = k,
                13 => app.temp_settings.special_item = k,
                _ => {}
            }
            app.audio_manager.play_sound_effect(SoundEffect::MenuPick);
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
        if app.is_paused {
            // Initialize pause menu when entering pause
            app.pause_menu_selection = 0;
            app.pause_submenu = None;
            // Start fade-out: lower volume and muffled effect
            app.audio_manager.start_fade_in(0.5, 0.2); // 0.5 second transition to 0.2 volume
            app.audio_manager.pause_music();
        } else {
            // Resume with fade-in back to settings volume
            app.audio_manager.resume_music();
            app.audio_manager
                .start_fade_in(0.5, app.settings.music_volume); // Fade back to settings volume
        }
        return;
    }

    // If paused, handle pause menu input instead of game input
    if app.is_paused {
        crate::ui::pause_menu::handle_input(app, key.code);
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
            let slot = c.to_digit(10).unwrap() as usize;
            // Check if shift is pressed for consumable usage
            if key
                .modifiers
                .contains(crossterm::event::KeyModifiers::SHIFT)
            {
                app.use_consumable(slot - 1); // Convert 1-9 to 0-8
            } else if key
                .modifiers
                .contains(crossterm::event::KeyModifiers::CONTROL)
            {
                // Ctrl + number drops the weapon in that slot
                app.drop_weapon(slot);
            } else {
                // Just number switches to that weapon
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
            } else if key_matches(key.code, &settings.use_consumable) {
                // Use the selected consumable (or first one if not focused)
                let idx = if app.inventory_focused {
                    app.inventory_scroll_index
                } else {
                    0
                };
                app.use_consumable(idx);
            } else if key_matches(key.code, &settings.special_item) {
                app.use_ultimate();
            } else if key_matches(key.code, &settings.toggle_inv) {
                app.inventory_focused = true;
            } else if key.code == KeyCode::Char('f') || key.code == KeyCode::Char('F') {
                app.block();
            } else if key.code == KeyCode::Char('t') || key.code == KeyCode::Char('T') {
                // Open skill tree during gameplay
                app.previous_state = Some(AppState::Game);
                app.state = AppState::SkillTree;
                app.is_paused = true; // Automatically pause the game
                app.skill_tree_selection = Some(0);
            } else if key.code == KeyCode::Char('y') || key.code == KeyCode::Char('Y') {
                // Open ultimate shop during gameplay
                app.previous_state = Some(AppState::Game);
                app.state = AppState::UltimateShop;
                app.is_paused = true; // Automatically pause the game
            }
        }
    }
}

fn handle_settings_selection(app: &mut App) {
    let sel = app.settings_state.selected().unwrap_or(0);
    match sel {
        0..=13 => app.settings_mode = crate::app::SettingsMode::Rebinding,
        14 => {
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
        15 => {
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
        16 | 17 => {
            // Volume sliders - just navigable with Left/Right arrows
            // No action needed on Enter
        }
        18 => {
            // Skip logo animation toggle
            app.temp_settings.skip_logo_animation = !app.temp_settings.skip_logo_animation;
        }
        20 => {
            // Save changes
            app.settings = app.temp_settings.clone();
            // Sync volume to app and audio manager
            app.music_volume = app.settings.music_volume;
            app.sound_volume = app.settings.sound_volume;
            app.audio_manager.set_music_volume(app.music_volume);
            let _ = app.settings.save();
            app.state = AppState::MainMenu;
        }
        21 => {
            // Discard and back
            app.temp_settings = app.settings.clone();
            app.state = AppState::MainMenu;
        }
        22 => {
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
        KeyCode::Char('t') | KeyCode::Char('T') => {
            // Open skill tree from dev menu
            app.previous_state = Some(AppState::DevMenu);
            app.state = AppState::SkillTree;
            app.skill_tree_selection = Some(0);
        }
        KeyCode::Enter => {
            app.regenerate_floor();
            // Automatically spawn all weapon types and rarities for testing
            if let Some(floor) = &mut app.current_floor {
                crate::ui::dev_menu::spawn_all_weapon_rarities(floor);
            }
        }
        KeyCode::Char('p') | KeyCode::Char('P') => {
            // Enter play mode if a floor has been generated
            if let Some(floor) = &app.current_floor {
                // Find a walkable starting position
                if let Some((x, y)) = floor.find_walkable_tile() {
                    app.character_position = (x, y);
                    app.update_camera(); // Initialize camera position
                    app.game_started_at = Some(std::time::Instant::now());
                    app.state = AppState::Game;
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
                            app.game_started_at = Some(std::time::Instant::now());

                            // Start music with fade-in for new game
                            let _ = app.audio_manager.start_music_with_fade_in();

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

pub fn handle_skill_tree_input(app: &mut App, key: crossterm::event::KeyEvent) {
    use crate::model::skill_tree_path::PathType;

    let available_paths = app.character.skill_tree_path.get_available_paths();
    let num_paths = available_paths.len();

    match key.code {
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
            if let Some(idx) = app.skill_tree_selection {
                let new_idx = if idx > 0 { idx - 1 } else { num_paths - 1 };
                app.skill_tree_selection = Some(new_idx);
            }
        }
        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
            if let Some(idx) = app.skill_tree_selection {
                let new_idx = (idx + 1) % num_paths;
                app.skill_tree_selection = Some(new_idx);
            }
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            if let Some(idx) = app.skill_tree_selection {
                if idx < available_paths.len() {
                    let path_type = available_paths[idx];
                    let _ = app
                        .character
                        .skill_tree_path
                        .purchase_upgrade(path_type, &mut app.character.gold);
                    // Apply the bonuses to character stats
                    app.character.apply_skill_tree_bonuses();
                }
            }
        }
        KeyCode::Char('t') | KeyCode::Char('T') | KeyCode::Esc => {
            // Return to the previous state (Game or DevMenu)
            if let Some(saved_state) = app.previous_state.take() {
                app.state = saved_state;
                // If returning to Game, unpause
                if matches!(app.state, AppState::Game) {
                    app.is_paused = false;
                }
            }
        }
        _ => {}
    }
}

pub fn handle_ultimate_shop_input(app: &mut App, key: crossterm::event::KeyEvent) {
    use crate::model::ultimate_shop::StatUpgradeType;

    match key.code {
        KeyCode::Up | KeyCode::Char('w') | KeyCode::Char('W') => {
            app.ultimate_shop_ui.previous();
            app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
        }
        KeyCode::Down | KeyCode::Char('s') | KeyCode::Char('S') => {
            app.ultimate_shop_ui.next();
            app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
        }
        KeyCode::Tab => {
            app.ultimate_shop_ui.switch_tab();
            app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
        }
        KeyCode::Enter | KeyCode::Char(' ') => {
            match app.ultimate_shop_ui.selected_tab {
                crate::ui::ultimate_shop::ShopTab::Ultimates => {
                    if let Some(idx) = app.ultimate_shop_ui.ultimate_list_state.selected() {
                        if let Some(shop_ult) = app.ultimate_shop.ultimates.get(idx) {
                            let result = app.ultimate_shop.purchase_ultimate(
                                &mut app.character.gold,
                                &mut app.character.shop_inventory,
                                &shop_ult.ultimate_type,
                                app.floor_level,
                            );
                            match result {
                                Ok(msg) => {
                                    app.ultimate_shop_ui.show_message(msg);
                                    app.audio_manager.play_sound_effect(SoundEffect::MenuPick);
                                }
                                Err(err) => {
                                    app.ultimate_shop_ui
                                        .show_message(format!("Cannot purchase: {}", err));
                                    app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
                                }
                            }
                        }
                    }
                }
                crate::ui::ultimate_shop::ShopTab::StatUpgrades => {
                    if let Some(idx) = app.ultimate_shop_ui.upgrade_list_state.selected() {
                        if let Some(shop_upg) = app.ultimate_shop.stat_upgrades.get(idx) {
                            let result = app.ultimate_shop.purchase_stat_upgrade(
                                &mut app.character.gold,
                                &mut app.character.shop_inventory,
                                &shop_upg.upgrade_type,
                                app.floor_level,
                            );
                            match result {
                                Ok(msg) => {
                                    app.ultimate_shop_ui.show_message(msg);
                                    app.audio_manager.play_sound_effect(SoundEffect::MenuPick);
                                    // Apply stat upgrade immediately
                                    apply_stat_upgrade(
                                        app,
                                        shop_upg.upgrade_type,
                                        shop_upg.stack_amount,
                                    );
                                }
                                Err(err) => {
                                    app.ultimate_shop_ui
                                        .show_message(format!("Cannot purchase: {}", err));
                                    app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
                                }
                            }
                        }
                    }
                }
            }
        }
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
            app.ultimate_shop_ui.reset();
            app.state = AppState::Game;
            app.is_paused = true;
            app.audio_manager.play_sound_effect(SoundEffect::MenuSwitch);
        }
        _ => {}
    }
}

/// Apply a stat upgrade to the character
fn apply_stat_upgrade(
    app: &mut App,
    upgrade: crate::model::ultimate_shop::StatUpgradeType,
    amount: f32,
) {
    use crate::model::ultimate_shop::StatUpgradeType;

    match upgrade {
        StatUpgradeType::MaxHealth => {
            app.character.health_max += amount as i32;
            app.character.health = app.character.health_max;
        }
        StatUpgradeType::AttackDamage => {
            app.character.attack_damage += amount as i32;
        }
        StatUpgradeType::AttackSpeed => {
            // Reduce cooldown by this percentage (amount is already in percentage format)
            let reduction = 1.0 - (amount as f32 / 100.0); // Convert to decimal
            let current_duration = app.character.attack_cooldown.duration();
            app.character
                .attack_cooldown
                .set_duration(current_duration * reduction);
        }
        StatUpgradeType::MovementSpeed => {
            app.character.speed += amount;
        }
        StatUpgradeType::DashDistance => {
            app.character.dash_distance += amount as i32;
        }
    }
}
