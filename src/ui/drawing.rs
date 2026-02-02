use ratatui::prelude::*;
use ratatui::widgets::Paragraph;

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

pub fn render_character(f: &mut Frame, area: Rect, position: (i32, i32)) {
    let character_art = "@";
    let style = Style::default().fg(Color::Yellow);

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
