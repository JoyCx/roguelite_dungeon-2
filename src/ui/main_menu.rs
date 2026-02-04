use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect, pulse_color: Color) {
    app.update_auto_scroll();

    let logo_lines: Vec<&str> = crate::LOGO.lines().collect();
    let diff = app.scroll_offset as i16 - app.last_scroll_offset as i16;
    let velocity =
        super::drawing::calculate_velocity(app.scroll_offset as i16, app.last_scroll_offset as i16);

    if velocity > 0 {
        let ghost_offset = super::drawing::calculate_blur_offset(app.scroll_offset, diff);
        render_logo_layer(
            f,
            &logo_lines,
            ghost_offset,
            area,
            Color::Indexed(234),
            false,
            area.height,
        );
    }

    render_logo_layer(
        f,
        &logo_lines,
        app.scroll_offset,
        area,
        Color::White,
        true,
        area.height,
    );

    let items = ["Start Game", "Saves", "Settings", "Dev Tools", "Exit"];
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|i| ListItem::new(*i).style(Style::default().fg(Color::Gray)))
        .collect();

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(" MAIN MENU "))
        .highlight_style(
            Style::default()
                .bg(pulse_color)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(" >> ");

    let list_width = 30;
    let list_height = (items.len() as u16) + 2;
    let horizontal_padding = 2;

    let menu_area = Rect::new(
        (area.width.saturating_sub(list_width)) / 2,
        (logo_lines.len() as u16 + horizontal_padding).saturating_sub(app.scroll_offset),
        list_width,
        list_height,
    );

    if menu_area.y < area.height.saturating_sub(2) {
        f.render_stateful_widget(list, menu_area, &mut app.main_menu_state);
    }

    let hints = vec![
        ("W/S", "Navigate", Some(Color::Yellow)),
        ("SPACE/ENTER", "Select", Some(Color::Cyan)),
        ("ESC", "Quit", Some(Color::Red)),
    ];
    super::drawing::render_key_hints(f, area, hints);
}

fn render_logo_layer(
    f: &mut Frame,
    lines: &[&str],
    offset: u16,
    area: Rect,
    color: Color,
    use_fade: bool,
    total_height: u16,
) {
    for (i, line) in lines.iter().enumerate() {
        let y = (i as u16).saturating_sub(offset);
        if y >= area.height.saturating_sub(2) {
            continue;
        }

        let mut style = Style::default().fg(color);
        if use_fade {
            let fade_color = super::drawing::calculate_fade_color(y, total_height);
            style = Style::default().fg(fade_color);
        }
        f.render_widget(
            Paragraph::new(*line)
                .style(style)
                .alignment(Alignment::Center),
            Rect::new(0, y, area.width, 1),
        );
    }
}
