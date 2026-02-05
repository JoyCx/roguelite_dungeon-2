pub mod character_creation;
pub mod dev_menu;
pub mod drawing;
pub mod main_menu;
pub mod settings;
pub mod skill_tree;

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

                // Get attack area for highlighting
                let attack_area = app.get_attack_area();
                let is_attacking = app.character.is_attack_animating();

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

                            // Highlight attack area in red if attacking
                            let final_style =
                                if is_attacking && attack_area.contains(&(world_x, world_y)) {
                                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                                } else {
                                    style
                                };

                            current_line.push(Span::styled(glyph_str, final_style));
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
                    .map(|item| (item.x, item.y, item.get_glyph(), item.get_glyph_color()))
                    .collect();
                drawing::render_items(f, game_area, &items, cx, cy);
            }

            // Render enemies
            if let Some(floor) = &app.current_floor {
                let enemies: Vec<(i32, i32, char, Color)> = floor
                    .enemies
                    .iter()
                    .filter(|e| e.is_alive())
                    .map(|enemy| {
                        // If damaged, render in red
                        let color = if enemy.is_damaged_animating() {
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
                        let glyph = enemy.rarity.get_glyph();
                        (enemy.position.x, enemy.position.y, glyph, color)
                    })
                    .collect();
                drawing::render_enemies(f, game_area, &enemies, cx, cy);
            }

            // Render ultimate ability area (only while animating)
            if app.character.ultimate.is_animating() {
                let ultimate_positions: Vec<(i32, i32)> = app
                    .character
                    .ultimate
                    .get_affected_area(px, py)
                    .into_iter()
                    .filter(|(x, y)| {
                        if let Some(floor) = &app.current_floor {
                            floor.is_walkable(*x, *y)
                        } else {
                            false
                        }
                    })
                    .collect();
                drawing::render_ultimate_area(f, game_area, ultimate_positions, cx, cy);
            }

            // Render active attack animations
            drawing::render_animations(f, game_area, &app.active_animations, cx, cy);

            // RENDER PLAYER LAST - so they always appear on top of other entities and effects
            let screen_x = px - cx;
            let screen_y = py - cy;

            if screen_x >= 0
                && screen_x < game_area.width as i32
                && screen_y >= 0
                && screen_y < game_area.height as i32
            {
                drawing::render_character(
                    f,
                    game_area,
                    (screen_x, screen_y),
                    app.character.is_damaged_animating(),
                );
            }
            // Render cooldown bars in right panel
            let bar_height = (right_panel_area.height as usize).saturating_sub(5) / 3;
            let panel_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),                 // Weapon info
                    Constraint::Length(1),                 // Health info
                    Constraint::Length(1),                 // Gold info
                    Constraint::Length(bar_height as u16), // Cooldown bars
                    Constraint::Min(0),                    // Inventory below
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

            // Dash cooldown (vertical)
            let remaining_dash_cooldown = app.character.dash_cooldown_remaining();
            drawing::render_vertical_cooldown_bar(
                f,
                bar_chunks[0],
                "DASH",
                remaining_dash_cooldown,
                app.character.dash_cooldown.duration(),
                Color::Magenta,
            );

            // Attack cooldown (vertical)
            let remaining_attack_cooldown = app.character.attack_cooldown_remaining();
            drawing::render_vertical_cooldown_bar(
                f,
                bar_chunks[1],
                "ATK",
                remaining_attack_cooldown,
                app.character.attack_cooldown.duration(),
                Color::Red,
            );

            // Bow cooldown (vertical)
            let remaining_bow_cooldown = app.character.bow_cooldown_remaining();
            drawing::render_vertical_cooldown_bar(
                f,
                bar_chunks[2],
                "BOW",
                remaining_bow_cooldown,
                app.character.bow_cooldown.duration(),
                Color::Cyan,
            );

            // Block cooldown (vertical)
            let remaining_block_cooldown = app.character.block_cooldown_remaining();
            drawing::render_vertical_cooldown_bar(
                f,
                bar_chunks[3],
                "BLOCK",
                remaining_block_cooldown,
                app.character.block_cooldown.duration(),
                Color::Blue,
            );

            // Inventory display below cooldowns
            drawing::render_consumables_info(
                f,
                panel_chunks[4],
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
        AppState::SkillTree => skill_tree::draw(f, app, area),
    }
}
