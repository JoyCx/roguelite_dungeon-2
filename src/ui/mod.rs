pub mod dev_menu;
pub mod drawing;
pub mod main_menu;
pub mod settings;

use crate::app::{App, AppState};
use ratatui::prelude::*;
use ratatui::widgets::{Clear, Paragraph};

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let duration = app.start_time.elapsed().as_secs_f32();
    let pulse_color = drawing::calculate_pulse_color(duration);

    match app.state {
        AppState::MainMenu => main_menu::draw(f, app, area, pulse_color),
        AppState::Settings => settings::draw(f, app, area, pulse_color),
        AppState::Game => {
            f.render_widget(Clear, area);

            app.update_terminal_size(area.width, area.height);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(2)])
                .split(area);

            let game_area = chunks[0];
            let cooldown_area = chunks[1];

            if let Some(floor) = &app.current_floor {
                let mut lines = Vec::new();

                let viewport_width = game_area.width as i32;
                let viewport_height = game_area.height as i32;
                let camera_x = app.camera_offset.0.floor() as i32;
                let camera_y = app.camera_offset.1.floor() as i32;

                for screen_y in 0..viewport_height {
                    let world_y = camera_y + screen_y;
                    let mut current_line = Vec::new();

                    for screen_x in 0..viewport_width {
                        let world_x = camera_x + screen_x;

                        if let Some((ch, style)) = floor.get_styled_tile(world_x, world_y) {
                            let glyph_str = if ch == ' ' {
                                "·".to_string()
                            } else {
                                ch.to_string()
                            };
                            current_line.push(Span::styled(glyph_str, style));
                        } else {
                            current_line.push(Span::raw(" "));
                        }
                    }

                    lines.push(Line::from(current_line));
                }

                let map_widget = Paragraph::new(lines);
                f.render_widget(map_widget, game_area);
            }

            let (px, py) = app.character_position;
            let (cx, cy) = (
                app.camera_offset.0.floor() as i32,
                app.camera_offset.1.floor() as i32,
            );

            let screen_x = px - cx;
            let screen_y = py - cy;

            if screen_x >= 0
                && screen_x < game_area.width as i32
                && screen_y >= 0
                && screen_y < game_area.height as i32
            {
                drawing::render_character(f, game_area, (screen_x, screen_y));
            }

            let remaining_cooldown = app.character.dash_cooldown_remaining();
            drawing::render_dash_cooldown_bar(
                f,
                cooldown_area,
                remaining_cooldown,
                app.character.dash_cooldown_duration,
            );
        }
        AppState::DevMenu => dev_menu::draw(f, app, area),
    }
}
