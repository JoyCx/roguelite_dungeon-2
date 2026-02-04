use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Clear, Padding, Paragraph};

pub fn calculate_pulse_color(duration_secs: f32) -> Color {
    let pulse_val = ((duration_secs * 15.0).sin() + 1.0) / 2.0;
    if pulse_val > 0.5 {
        Color::Indexed(196)
    } else {
        Color::Indexed(88)
    }
}

pub fn calculate_fade_color(y_pos: u16, total_height: u16) -> Color {
    let score = y_pos as f32 / total_height.max(1) as f32;

    if score < 0.15 {
        Color::Indexed(16)
    } else if score < 0.30 {
        Color::Indexed(52)
    } else if score < 0.45 {
        Color::Indexed(88)
    } else if score < 0.60 {
        Color::Indexed(124)
    } else if score < 0.75 {
        Color::Indexed(160)
    } else if score < 0.90 {
        Color::Indexed(196)
    } else {
        Color::Indexed(202)
    }
}

pub fn calculate_velocity(current: i16, previous: i16) -> u16 {
    (current - previous).unsigned_abs()
}

pub fn calculate_blur_offset(scroll_offset: u16, diff: i16) -> u16 {
    if diff > 0 {
        scroll_offset.saturating_sub(1)
    } else {
        scroll_offset.saturating_add(1)
    }
}

pub fn render_key_hints(f: &mut Frame, area: Rect, hints: Vec<(&str, &str, Option<Color>)>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let mut spans = Vec::new();
    for (i, (key, description, bg_color)) in hints.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("   "));
        }

        let key_style = match bg_color {
            Some(color) => Style::default().fg(Color::Black).bg(*color),
            None => Style::default().fg(Color::Yellow).bg(Color::Indexed(52)),
        };

        spans.push(Span::styled(format!(" {} ", key), key_style));
        spans.push(Span::raw(format!(" {} ", description)));
    }

    let hints_line = Line::from(spans);
    let hint_bar = Paragraph::new(hints_line)
        .alignment(Alignment::Center)
        .style(Style::default().bg(Color::Indexed(234)));

    f.render_widget(hint_bar, chunks[1]);
}

pub fn render_character(f: &mut Frame, area: Rect, position: (i32, i32), color: Color) {
    let character_art = "@";
    let style = Style::default().fg(color);

    let paragraph = Paragraph::new(character_art).style(style);

    let character_area = Rect::new(area.x + position.0 as u16, area.y + position.1 as u16, 1, 1);

    f.render_widget(paragraph, character_area);
}

pub fn render_dash_cooldown_bar(
    f: &mut Frame,
    area: Rect,
    remaining_cooldown: f32,
    max_cooldown: f32,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(2)])
        .split(area);

    let bar_height = chunks[1];

    let charge_percent = 1.0 - (remaining_cooldown / max_cooldown).clamp(0.0, 1.0);

    let bar_width = (bar_height.width as usize).saturating_sub(4);
    let filled_width = ((bar_width as f32) * charge_percent) as usize;
    let empty_width = bar_width.saturating_sub(filled_width);

    let mut bar_line = String::new();
    bar_line.push_str("[ ");

    let fill_char = '█';
    let fill_color = if remaining_cooldown < 0.01 {
        Color::Green
    } else {
        Color::Yellow
    };

    for _ in 0..filled_width {
        bar_line.push(fill_char);
    }

    let empty_char = '░';
    for _ in 0..empty_width {
        bar_line.push(empty_char);
    }

    bar_line.push_str(" ]");

    let status_text = if remaining_cooldown < 0.01 {
        "DASH READY".to_string()
    } else {
        format!("{:.1}s", remaining_cooldown)
    };

    let mut spans = vec![Span::styled(bar_line, Style::default().fg(fill_color))];
    spans.push(Span::raw(" "));
    spans.push(Span::styled(
        status_text,
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD),
    ));

    let cooldown_line = Line::from(spans);
    let cooldown_widget = Paragraph::new(cooldown_line)
        .alignment(Alignment::Center)
        .style(Style::default().bg(Color::Indexed(234)));

    f.render_widget(cooldown_widget, bar_height);
}
#[allow(dead_code)]
pub fn render_attack_area(
    f: &mut Frame,
    area: Rect,
    attack_positions: Vec<(i32, i32)>,
    offset_x: i32,
    offset_y: i32,
) {
    for (x, y) in attack_positions {
        let screen_x = (x - offset_x) as u16;
        let screen_y = (y - offset_y) as u16;

        if screen_x < area.width && screen_y < area.height {
            let pos_area = Rect::new(area.x + screen_x, area.y + screen_y, 1, 1);
            let indicator = Paragraph::new("✦")
                .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));
            f.render_widget(indicator, pos_area);
        }
    }
}

pub fn render_arrows(
    f: &mut Frame,
    area: Rect,
    arrows: &[(f32, f32, &str)],
    offset_x: i32,
    offset_y: i32,
) {
    for (x, y, glyph) in arrows {
        let world_x = x.round() as i32;
        let world_y = y.round() as i32;
        let screen_x = (world_x - offset_x) as u16;
        let screen_y = (world_y - offset_y) as u16;

        if screen_x < area.width && screen_y < area.height {
            let pos_area = Rect::new(area.x + screen_x, area.y + screen_y, 1, 1);
            let indicator = Paragraph::new(*glyph).style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );
            f.render_widget(indicator, pos_area);
        }
    }
}

pub fn render_enemies(
    f: &mut Frame,
    area: Rect,
    enemies: &[(i32, i32, char, Color)],
    offset_x: i32,
    offset_y: i32,
) {
    for (x, y, glyph, color) in enemies {
        let screen_x = (x - offset_x) as u16;
        let screen_y = (y - offset_y) as u16;

        if screen_x < area.width && screen_y < area.height {
            let pos_area = Rect::new(area.x + screen_x, area.y + screen_y, 1, 1);
            let enemy_indicator = Paragraph::new(glyph.to_string())
                .style(Style::default().fg(*color).add_modifier(Modifier::BOLD));
            f.render_widget(enemy_indicator, pos_area);
        }
    }
}

pub fn render_particles(
    f: &mut Frame,
    area: Rect,
    particles: &[(i32, i32, char, Color)],
    offset_x: i32,
    offset_y: i32,
) {
    for (x, y, glyph, color) in particles {
        let screen_x = (*x - offset_x) as u16;
        let screen_y = (*y - offset_y) as u16;

        if screen_x < area.width && screen_y < area.height {
            let pos_area = Rect::new(area.x + screen_x, area.y + screen_y, 1, 1);
            let particle = Paragraph::new(glyph.to_string())
                .style(Style::default().fg(*color).add_modifier(Modifier::BOLD));
            f.render_widget(particle, pos_area);
        }
    }
}

pub fn render_ultimate_area(
    f: &mut Frame,
    area: Rect,
    ultimate_positions: Vec<(i32, i32)>,
    offset_x: i32,
    offset_y: i32,
) {
    for (x, y) in ultimate_positions {
        let screen_x = (x - offset_x) as u16;
        let screen_y = (y - offset_y) as u16;

        if screen_x < area.width && screen_y < area.height {
            let pos_area = Rect::new(area.x + screen_x, area.y + screen_y, 1, 1);
            let indicator = Paragraph::new("◆").style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            );
            f.render_widget(indicator, pos_area);
        }
    }
}

pub fn render_weapon_info(f: &mut Frame, area: Rect, weapon_name: &str) {
    let weapon_widget = Paragraph::new(format!("Equipped: {}", weapon_name))
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Cyan));

    f.render_widget(weapon_widget, area);
}

pub fn render_weapon_inventory(
    f: &mut Frame,
    area: Rect,
    weapon_inventory: &crate::model::weapon::WeaponInventory,
) {
    let mut lines = Vec::new();
    lines.push(Line::from(Span::styled(" WEAPONS (1-9) ", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))));

    for i in 0..9 {
        let slot_num = i + 1;
        let mut style = Style::default().fg(Color::Gray);
        let prefix = if i == weapon_inventory.current_weapon_index {
            style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
            "→"
        } else {
            " "
        };

        let weapon_name = if let Some(weapon) = weapon_inventory.weapons.get(i) {
            weapon.name.clone()
        } else {
            "---".to_string()
        };

        lines.push(Line::from(vec![
            Span::styled(format!("{} {}: ", prefix, slot_num), style),
            Span::styled(weapon_name, style),
        ]));
    }

    let widget = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Cyan)));
    f.render_widget(widget, area);
}

pub fn render_health_info(f: &mut Frame, area: Rect, health: i32, health_max: i32) {
    let health_text = format!("HP: {}/{}", health, health_max);
    let health_color = if health > health_max * 2 / 3 {
        Color::Green
    } else if health > health_max / 3 {
        Color::Yellow
    } else {
        Color::Red
    };

    let health_widget = Paragraph::new(health_text)
        .alignment(Alignment::Left)
        .style(Style::default().fg(health_color));

    f.render_widget(health_widget, area);
}

pub fn render_gold_info(f: &mut Frame, area: Rect, gold: u32) {
    let gold_text = format!("¤ {}", gold);
    let gold_widget = Paragraph::new(gold_text)
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(gold_widget, area);
}

pub fn render_consumables_info(
    f: &mut Frame,
    area: Rect,
    consumables: &crate::model::consumable::ConsumableInventory,
    inventory_focused: bool,
    scroll_index: usize,
    settings: &crate::model::settings::Settings,
) {
    let items = &consumables.items;

    // Build list of items with quantities
    let item_lines: Vec<Line> = if items.is_empty() {
        vec![Line::from("⚠ No items")]
    } else {
        items
            .iter()
            .enumerate()
            .map(|(idx, item)| {
                let selected = inventory_focused && idx == scroll_index;
                let prefix = if selected { "→ " } else { "  " };
                let name_display = if item.name.len() > 12 {
                    format!("{}...", &item.name[..9])
                } else {
                    item.name.clone()
                };

                let text = format!("{}{} x{}", prefix, name_display, item.quantity);
                let style = if selected {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                };
                Line::from(Span::styled(text, style))
            })
            .collect()
    };

    // Apply scrolling
    let visible_items = (area.height as usize).saturating_sub(2); // Reserve space for border/title
    let start_idx = if scroll_index < visible_items {
        0
    } else {
        scroll_index - visible_items + 1
    };

    let visible_lines: Vec<Line> = item_lines
        .into_iter()
        .skip(start_idx)
        .take(visible_items)
        .collect();

    let border_color = if inventory_focused {
        Color::Yellow
    } else {
        Color::Gray
    };

    let title = if inventory_focused {
        " INVENTORY [FOCUSED] "
    } else {
        " INVENTORY "
    };

    let inventory_widget = Paragraph::new(visible_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(title)
                .title_alignment(Alignment::Left),
        )
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::White));

    f.render_widget(inventory_widget, area);

    // Render tooltip below the inventory
    if area.height > 1 {
        let tooltip_area = Rect {
            x: area.x,
            y: area.y.saturating_add(area.height.saturating_sub(1)),
            width: area.width,
            height: 1,
        };
        let subtitle = format!("[{}] to toggle focus", settings.toggle_inv);
        let tooltip = Paragraph::new(subtitle).alignment(Alignment::Right).style(
            Style::default()
                .fg(Color::DarkGray)
                .add_modifier(Modifier::DIM),
        );
        f.render_widget(tooltip, tooltip_area);
    }
}

pub fn render_vertical_cooldown_bar(
    f: &mut Frame,
    area: Rect,
    label: &str,
    remaining: f32,
    max_duration: f32,
    color: Color,
) {
    if area.height < 2 {
        return;
    }

    let charge_percent = 1.0 - (remaining / max_duration).clamp(0.0, 1.0);
    let bar_height = (area.height as usize).saturating_sub(1);
    let filled_height = ((bar_height as f32) * charge_percent).ceil() as usize;

    let mut lines = vec![Line::from(Span::styled(
        label,
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    ))];

    // Draw vertical bar from bottom to top
    for i in (0..bar_height).rev() {
        if i < filled_height {
            lines.push(Line::from(Span::raw("█")));
        } else {
            lines.push(Line::from(Span::raw("░")));
        }
    }

    let bar_widget = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .style(Style::default().fg(color));

    f.render_widget(bar_widget, area);
}

pub fn render_horizontal_ultimate_bar(f: &mut Frame, area: Rect, charge: f32) {
    if area.height < 1 {
        return;
    }

    let bar_width = area.width as usize;
    let filled_width = ((bar_width as f32) * (charge / 100.0)).ceil() as usize;
    let empty_width = bar_width.saturating_sub(filled_width);

    let mut bar_line = String::new();
    bar_line.push('[');
    for _ in 0..filled_width {
        bar_line.push('█');
    }
    for _ in 0..empty_width {
        bar_line.push('░');
    }
    bar_line.push(']');

    let ultimate_widget = Paragraph::new(format!("Ultimate {:.0}%: {}", charge, bar_line))
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::Yellow));

    f.render_widget(ultimate_widget, area);
}
pub fn render_items(
    f: &mut Frame,
    game_area: Rect,
    items: &[(i32, i32, char, Color)],
    camera_x: i32,
    camera_y: i32,
) {
    for (world_x, world_y, glyph, color) in items {
        let screen_x = world_x - camera_x;
        let screen_y = world_y - camera_y;

        if screen_x >= 0
            && screen_x < game_area.width as i32
            && screen_y >= 0
            && screen_y < game_area.height as i32
        {
            let widget = Paragraph::new(glyph.to_string())
                .style(Style::default().fg(*color).add_modifier(Modifier::BOLD));

            let area = Rect {
                x: game_area.x + screen_x as u16,
                y: game_area.y + screen_y as u16,
                width: 1,
                height: 1,
            };

            f.render_widget(widget, area);
        }
    }
}
pub fn render_item_description_popup(
    f: &mut Frame,
    area: Rect,
    consumables: &crate::model::consumable::ConsumableInventory,
    scroll_index: usize,
) {
    let items = &consumables.items;

    // Get the selected item, return early if no items or invalid index
    if items.is_empty() || scroll_index >= items.len() {
        return;
    }

    let selected_item = &items[scroll_index];

    // Build the description text
    let mut description_lines = vec![
        Line::from(format!("{}", selected_item.name)).style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        ),
        Line::from(""),
    ];

    // Add item description
    description_lines.push(Line::from(selected_item.description.clone()).fg(Color::Gray));
    description_lines.push(Line::from(""));

    // Add quantity
    description_lines
        .push(Line::from(format!("Quantity: {}", selected_item.quantity)).fg(Color::Cyan));

    // Create a centered popup area (about 50% of screen width and height)
    let popup_width = (area.width as f32 * 0.6) as u16;
    let popup_height = (area.height as f32 * 0.6) as u16;
    let popup_x = (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = (area.height.saturating_sub(popup_height)) / 2;

    let popup_area = Rect {
        x: area.x + popup_x,
        y: area.y + popup_y,
        width: popup_width,
        height: popup_height,
    };

    // Render dark overlay background to hide the map
    let overlay_block = Block::default().style(Style::default().bg(Color::Black));
    f.render_widget(overlay_block, area);

    // Clear the popup area to remove map characters
    f.render_widget(Clear, popup_area);

    // Create the popup widget
    let popup = Paragraph::new(description_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Yellow))
                .title(" Item Details ")
                .title_alignment(Alignment::Center)
                .padding(Padding::new(1, 1, 1, 1))
                .style(Style::default().bg(Color::Black)),
        )
        .alignment(Alignment::Left)
        .style(Style::default().fg(Color::White).bg(Color::Black));

    f.render_widget(popup, popup_area);

    // Render hint at bottom
    let hint_area = Rect {
        x: popup_area.x,
        y: popup_area.y + popup_area.height.saturating_sub(1),
        width: popup_area.width,
        height: 1,
    };
    let hint = Paragraph::new("[ ESC ] to close")
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Gray)
                .add_modifier(Modifier::DIM)
                .bg(Color::Black),
        );
    f.render_widget(hint, hint_area);
}

pub fn render_pause_indicator(f: &mut Frame, area: Rect) {
    let pause_text = "⏸ PAUSED ⏸";
    let pause_paragraph = Paragraph::new(pause_text)
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        );

    let pause_area = Rect {
        x: area.x + (area.width.saturating_sub(pause_text.len() as u16)) / 2,
        y: area.y + 1,
        width: pause_text.len() as u16,
        height: 1,
    };

    f.render_widget(pause_paragraph, pause_area);
}
