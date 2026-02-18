pub mod character_creation;
pub mod death_screen;
pub mod dev_menu;
pub mod drawing;
pub mod main_menu;
pub mod pause_menu;
pub mod settings;
pub mod skill_tree;
pub mod ultimate_shop;
pub mod victory_screen;

use crate::app::{App, AppState};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

pub fn draw(f: &mut Frame, app: &mut App) {
    let area = f.area();
    let duration = app.start_time.elapsed().as_secs_f32();
    let pulse_color = drawing::calculate_pulse_color(duration);

    match app.state {
        AppState::MainMenu => main_menu::draw(f, app, area, pulse_color),
        AppState::SaveSelection => {
            // Draw save selection menu with similar styling to main menu
            main_menu::draw_save_selection(f, app, area, pulse_color)
        }
        AppState::CharacterCreation => character_creation::draw(f, app, area, pulse_color),
        AppState::Settings => settings::draw(f, app, area, pulse_color),
        AppState::UltimateShop => {
            // Draw game in background then shop overlay
            app.update_terminal_size(area.width, area.height);

            // Draw background game elements first
            if let Some(ref floor) = app.current_floor {
                // You can add minimal background rendering here if desired
            }

            // Create shop area (centered modal)
            let shop_width = (area.width as usize).min(100).max(60) as u16;
            let shop_height = (area.height as usize).min(40).max(20) as u16;
            let shop_x = (area.width.saturating_sub(shop_width)) / 2;
            let shop_y = (area.height.saturating_sub(shop_height)) / 2;

            let shop_area = Rect {
                x: shop_x,
                y: shop_y,
                width: shop_width,
                height: shop_height,
            };

            ultimate_shop::draw(
                f,
                app.character.gold,
                app.floor_level,
                &app.ultimate_shop,
                &mut app.ultimate_shop_ui,
                shop_area,
            );
        }
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

            // Split into game area, ultimate bar, and weapon slots at bottom
            let vertical_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(2),
                    Constraint::Length(1),
                    Constraint::Length(2),
                ])
                .split(area);

            let game_and_panel_area = vertical_chunks[0];
            let ultimate_bar_area = vertical_chunks[1];
            let weapon_slots_area = vertical_chunks[2];
            let weapon_names_area = vertical_chunks[3];

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

                // Get attack area for highlighting - convert to HashSet for O(1) lookups
                let attack_area_set: std::collections::HashSet<_> =
                    app.get_attack_area().into_iter().collect();
                let is_attacking = app.character.is_attack_animating();

                // Pre-allocate cached strings for common glyphs to avoid repeated allocations
                let space_bullet = "·".to_string();
                let dot_str = ".".to_string();

                for screen_y in 0..viewport_height {
                    let world_y = camera_y + screen_y;
                    let mut current_line = Vec::with_capacity(viewport_width as usize);

                    for screen_x in 0..viewport_width {
                        let world_x = camera_x + screen_x;

                        if let Some((ch, color_idx)) =
                            floor.get_styled_tile_cached(world_x, world_y)
                        {
                            // Highlight attack area in red if attacking
                            let is_attacked =
                                is_attacking && attack_area_set.contains(&(world_x, world_y));

                            if is_attacked {
                                // Only allocate string if we need red highlighting
                                let glyph_str = if ch == ' ' {
                                    "·".to_string()
                                } else {
                                    ch.to_string()
                                };
                                current_line.push(Span::styled(
                                    glyph_str,
                                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                                ));
                            } else {
                                // Use pre-rendered strings for non-highlighted tiles
                                let glyph = if ch == ' ' {
                                    space_bullet.clone()
                                } else if ch == '.' {
                                    dot_str.clone()
                                } else {
                                    ch.to_string()
                                };
                                current_line.push(Span::styled(
                                    glyph,
                                    Style::default().fg(Color::Indexed(color_idx)),
                                ));
                            }
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
                let items: Vec<(i32, i32, &str, Color)> = floor
                    .items
                    .iter()
                    .map(|item| (item.x, item.y, item.get_glyph(), item.get_glyph_color()))
                    .collect();
                drawing::render_items(f, game_area, &items, cx, cy);
            }

            // Render enemies
            if let Some(floor) = &app.current_floor {
                let enemies: Vec<(i32, i32, String, Color)> = floor
                    .enemies
                    .iter()
                    .filter(|e| e.is_alive())
                    .map(|enemy| {
                        // If damaged, render in red
                        let color = if enemy.is_damaged_animating() {
                            Color::Red
                        } else {
                            match enemy.rarity {
                                crate::model::enemy_type::EnemyRarity::Fighter => Color::White,
                                crate::model::enemy_type::EnemyRarity::Guard => Color::Green,
                                crate::model::enemy_type::EnemyRarity::Champion => Color::Yellow,
                                crate::model::enemy_type::EnemyRarity::Elite => Color::LightRed,
                                crate::model::enemy_type::EnemyRarity::Boss => Color::LightMagenta,
                            }
                        };
                        let glyph = enemy.rarity.get_glyph();
                        (enemy.position.x, enemy.position.y, glyph.to_string(), color)
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
            let panel_chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1), // Health info
                    Constraint::Length(1), // Gold info
                    Constraint::Length(6), // Cooldown bars area
                    Constraint::Min(0),    // Inventory below
                ])
                .split(right_panel_area);

            // Health info
            drawing::render_health_info(
                f,
                panel_chunks[0],
                app.character.health,
                app.character.health_max,
            );

            // Gold info
            drawing::render_gold_info(f, panel_chunks[1], app.character.gold);

            // Create dedicated area for cooldown bars with border
            let cooldown_area = panel_chunks[2];
            let bar_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                    Constraint::Percentage(25),
                ])
                .split(cooldown_area);

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
                panel_chunks[3],
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

            // Render weapon slots bar
            drawing::render_weapon_slots(f, weapon_slots_area, &app.character.weapon_inventory);

            // Render weapon names tooltip
            drawing::render_weapon_names_tooltip(
                f,
                weapon_names_area,
                &app.character.weapon_inventory,
            );

            // Render empty slot warning if active
            drawing::render_empty_slot_warning(f, weapon_slots_area, app.empty_slot_message_timer);

            // Render item description popup if showing
            if app.showing_item_description {
                drawing::render_item_description_popup(
                    f,
                    area,
                    &app.character.consumable_inventory,
                    app.inventory_scroll_index,
                );
            }

            // Render weapon pickup notification
            if let Some((weapon_name, rarity)) = &app.last_weapon_pickup {
                let rarity_color = rarity.get_color();
                let box_width = 40.min(area.width.saturating_sub(4));
                let box_x = (area.width.saturating_sub(box_width)) / 2;
                let weapon_rarity = rarity.name();
                let lines = vec![
                    Line::from(Span::styled(
                        "Weapon Picked Up",
                        Style::default()
                            .fg(rarity_color)
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(""),
                    Line::from(Span::styled(
                        format!("{} {} ", weapon_rarity, weapon_name),
                        Style::default().fg(rarity_color),
                    )),
                ];

                let notification_area = Rect {
                    x: area.x + box_x,
                    y: area.y + 1,
                    width: box_width,
                    height: 5,
                };

                // Render dark overlay to cleanly show notification
                f.render_widget(Clear, notification_area);

                let notification = Paragraph::new(lines)
                    .block(
                        Block::default()
                            .borders(Borders::ALL)
                            .border_style(Style::default().fg(rarity_color))
                            .style(Style::default().bg(Color::Black)),
                    )
                    .alignment(Alignment::Center)
                    .style(Style::default().bg(Color::Black));

                f.render_widget(notification, notification_area);
            }

            // Render pause indicator last (on top of everything)
            if app.is_paused {
                drawing::render_pause_indicator(f, area);
                // Render pause menu if paused
                pause_menu::draw(f, app, area);
            }
        }
        AppState::DevMenu => dev_menu::draw(f, app, area),
        AppState::SkillTree => skill_tree::draw(f, app, area),
        AppState::DeathScreen => death_screen::draw(f, app, area),
        AppState::VictoryScreen => victory_screen::draw(f, app, area),
    }
}

/// Show load save menu from main menu
pub fn show_load_save_menu(app: &mut App) {
    use crate::model::gamesave::GameSave;

    if let Ok(saves) = GameSave::list_saves() {
        if !saves.is_empty() {
            // For now, load the first save (in a full implementation, you'd show a menu to select)
            if let Ok(()) = app.load_game(&saves[0]) {
                app.state = AppState::Game;
                app.restart_game(); // Initialize the game with loaded state
            }
        }
    }
}
