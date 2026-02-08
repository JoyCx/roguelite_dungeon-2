use crate::app::{App, AppState};
use crossterm::event::{self, KeyEvent, MouseButton, MouseEvent, MouseEventKind};

pub mod handlers;
pub mod menu;

pub fn handle_input(app: &mut App, key: KeyEvent) {
    match app.state {
        AppState::MainMenu => handlers::handle_main_menu_input(app, key),
        AppState::CharacterCreation => handlers::handle_character_creation_input(app, key),
        AppState::Settings => handlers::handle_settings_input(app, key),
        AppState::Game => handlers::handle_game_input(app, key),
        AppState::DevMenu => handlers::handle_dev_menu_input(app, key),
        AppState::SkillTree => handlers::handle_skill_tree_input(app, key),
        AppState::DeathScreen => crate::ui::death_screen::handle_input(app, key.code),
    }
}

pub fn handle_mouse_event(app: &mut App, mouse: MouseEvent) {
    match app.state {
        AppState::Settings => match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Down(MouseButton::Right) => {
                if app.settings_mode == crate::app::SettingsMode::Rebinding {
                    let k = match mouse.kind {
                        MouseEventKind::Down(MouseButton::Left) => "Left".to_string(),
                        MouseEventKind::Down(MouseButton::Right) => "Right".to_string(),
                        _ => return,
                    };

                    match app.settings_state.selected().unwrap_or(0) {
                        0 => app.temp_settings.move_up = k,
                        1 => app.temp_settings.move_left = k,
                        2 => app.temp_settings.move_down = k,
                        3 => app.temp_settings.move_right = k,
                        4 => app.temp_settings.attack = k,
                        5 => app.temp_settings.dash = k,
                        6 => app.temp_settings.block = k,
                        7 => app.temp_settings.toggle_inv = k,
                        8 => app.temp_settings.special_item = k,
                        _ => {}
                    }
                    app.settings_mode = crate::app::SettingsMode::Navigating;
                }
            }
            event::MouseEventKind::ScrollUp => app.set_scroll(app.scroll_offset.saturating_sub(2)),
            event::MouseEventKind::ScrollDown => {
                app.set_scroll(app.scroll_offset.saturating_add(2))
            }
            _ => {}
        },
        AppState::Game => match mouse.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                app.use_current_weapon();
            }
            event::MouseEventKind::ScrollUp => app.set_scroll(app.scroll_offset.saturating_sub(2)),
            event::MouseEventKind::ScrollDown => {
                app.set_scroll(app.scroll_offset.saturating_add(2))
            }
            _ => {}
        },
        _ => match mouse.kind {
            event::MouseEventKind::ScrollUp => app.set_scroll(app.scroll_offset.saturating_sub(2)),
            event::MouseEventKind::ScrollDown => {
                app.set_scroll(app.scroll_offset.saturating_add(2))
            }
            _ => {}
        },
    }
}
