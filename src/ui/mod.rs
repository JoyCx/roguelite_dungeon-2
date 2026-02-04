pub mod character_creation;
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
        AppState::CharacterCreation => character_creation::draw(f, app, area, pulse_color),
        AppState::Settings => settings::draw(f, app, area, pulse_color),
        AppState::Game => {
            f.render_widget(Clear, area);

            // Draw FPS counter in top-right corner
            let elapsed = app.start_time.elapsed().as_secs_f32();
            let fps = if elapsed > 0.0 {
                app.frame_count as f32 / elapsed
            } else {
                0.0
            };
            let fps_text = format!("FPS: {:.1}", fps);
            let fps_para = Paragraph::new(fps_text)
                .style(Style::default().fg(Color::Green))
                .alignment(Alignment::Right);
            let fps_area = Rect {
                x: area.right().saturating_sub(12),
                y: area.top(),
                width: 12,
                height: 1,
            };
            f.render_widget(fps_para, fps_area);

            app.update_terminal_size(area.width, area.height);

            // Split into game area and bottom bar, then split right side into panel
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(0), Constraint::Length(2)])
                .split(area);

            let game_and_panel_area = vertical_chunks[0];
            let ultimate_bar_area = vertical_chunks[1];

            // Split game area and right panel
            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Min(0), Constraint::Length(20)])
                .split(game_and_panel_area);

            let game_area = horizontal_chunks[0];
            let right_panel_area = horizontal_chunks[1];

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

                        if let Some((ch, style)) = floor.get_styled_tile(world_x, world_y, app.frame_count) {
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

            // Render arrows
            let arrows: Vec<(f32, f32, &str)> = app
                .arrows
                .iter()
                .map(|arrow| (arrow.x, arrow.y, arrow.get_glyph()))
                .collect();
            drawing::render_arrows(f, game_area, &arrows, cx, cy);

            // Render particles
            let particles = app.particle_system.get_active_particles();
            drawing::render_particles(f, game_area, &particles, cx, cy);

            // Render items on the floor
            if let Some(floor) = &app.current_floor {
                let items: Vec<(i32, i32, char, Color)> = floor
                    .items
                    .iter()
                    .map(|item| (item.x, item.y, item.get_glyph(), item.get_color()))
                    .collect();
                drawing::render_items(f, game_area, &items, cx, cy);
            }

            // Render active animations
            for anim in &app.active_animations {
                if let Some(frame) = anim.frames.get(anim.current_frame) {
                    for (wx, wy) in &frame.tiles {
                        let sx = wx - cx;
                        let sy = wy - cy;

                        if sx >= 0
                            && sx < game_area.width as i32
                            && sy >= 0
                            && sy < game_area.height as i32
                        {
                            let pos_area = Rect::new(game_area.x + sx as u16, game_area.y + sy as u16, 1, 1);

                            // Check if player is here, prioritize drawing player
                            if *wx == px && *wy == py {
                                continue;
                            }

                            let indicator = Paragraph::new(frame.symbol.to_string())
                                .style(Style::default().fg(frame.color).add_modifier(Modifier::BOLD));
                            f.render_widget(indicator, pos_area);
                        }
                    }
                }
            }

            // Render enemies
            if let Some(floor) = &app.current_floor {
                let enemies: Vec<(i32, i32, char, Color)> = floor
                    .enemies
                    .iter()
                    .filter(|e| e.is_alive())
                    .map(|enemy| {
                        let color = if enemy.damaged_timer > 0.0 {
                            Color::Red
                        } else {
                            match enemy.rarity {
                                crate::model::enemy_type::EnemyRarity::Fighter => Color::Gray,
                                crate::model::enemy_type::EnemyRarity::Guard => Color::Green,
                                crate::model::enemy_type::EnemyRarity::Champion => Color::Cyan,
                                crate::model::enemy_type::EnemyRarity::Elite => Color::Magenta,
                                crate::model::enemy_type::EnemyRarity::Boss => Color::Red,
                            }
                        };
                        (enemy.position.x, enemy.position.y, 'E', color)
                    })
                    .collect();
                drawing::render_enemies(f, game_area, &enemies, cx, cy);
            }

            // Render player (prioritized)
            if screen_x >= 0
                && screen_x < game_area.width as i32
                && screen_y >= 0
                && screen_y < game_area.height as i32
            {
                let player_color = if app.character.damaged_timer > 0.0 {
                    Color::Red
                } else {
                    Color::Yellow
                };
                drawing::render_character(f, game_area, (screen_x, screen_y), player_color);
            }

            // Render panel on the right
            let panel_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),  // Weapon info
                    Constraint::Length(1),  // Health info
                    Constraint::Length(1),  // Gold info
                    Constraint::Length(6),  // Cooldown bars
                    Constraint::Length(11), // Weapon Inventory
                    Constraint::Min(0),     // Consumable Inventory
                ])
                .split(right_panel_area);

            // Weapon info
            if let Some(weapon) = app.character.weapon_inventory.get_current_weapon() {
                drawing::render_weapon_info(f, panel_chunks[0], &weapon.name);
            }

            // Health info
            drawing::render_health_info(
                f,
                panel_chunks[1],
                app.character.health,
                app.character.health_max,
            );

            // Gold info
            drawing::render_gold_info(f, panel_chunks[2], app.character.gold);

            // Create horizontal layout for the 4 cooldown bars
            let bar_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ])
                .split(panel_chunks[3]);

            // Cooldown bars
            drawing::render_vertical_cooldown_bar(
                f,
                bar_chunks[0],
                "DASH",
                app.character.dash_cooldown_remaining(),
                app.character.dash_cooldown_duration,
                Color::Magenta,
            );
            drawing::render_vertical_cooldown_bar(
                f,
                bar_chunks[1],
                "ATK",
                app.character.attack_cooldown_remaining(),
                app.character.attack_cooldown_duration,
                Color::Red,
            );
            drawing::render_vertical_cooldown_bar(
                f,
                bar_chunks[2],
                "BOW",
                app.character.bow_cooldown_remaining(),
                app.character.bow_cooldown_duration,
                Color::Cyan,
            );
            drawing::render_vertical_cooldown_bar(
                f,
                bar_chunks[3],
                "BLOCK",
                app.character.block_cooldown_remaining(),
                app.character.block_cooldown_duration,
                Color::Blue,
            );

            // Weapon Inventory
            drawing::render_weapon_inventory(f, panel_chunks[4], &app.character.weapon_inventory);

            // Consumable Inventory
            drawing::render_consumables_info(
                f,
                panel_chunks[5],
                &app.character.consumable_inventory,
                app.inventory_focused,
                app.inventory_scroll_index,
                &app.settings,
            );

            // Render ultimate charge bar at bottom
            drawing::render_horizontal_ultimate_bar(
                f,
                ultimate_bar_area,
                app.character.ultimate_charge,
            );

            // Render item description popup if showing
            if app.showing_item_description {
                drawing::render_item_description_popup(
                    f,
                    area,
                    &app.character.consumable_inventory,
                    app.inventory_scroll_index,
                );
            }

            // Render pause indicator last (on top of everything)
            if app.is_paused {
                drawing::render_pause_indicator(f, area);
            }
        }
        AppState::DevMenu => dev_menu::draw(f, app, area),
    }
}
