use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(" Seed Input (Numbers Only) ")
        .style(Style::default().fg(Color::Yellow));

    let input_text = if app.dev_seed_input.is_empty() {
        "[Enter a seed or press R for random]".to_string()
    } else {
        app.dev_seed_input.clone()
    };

    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Cyan))
        .block(input_block);

    f.render_widget(input, chunks[0]);

    if let Some(floor) = &app.current_floor {
        let styled_tiles = floor.styled_grid();

        let mut lines = Vec::new();
        let mut current_line = Vec::new();
        let mut prev_y = 0;

        for (_x, y, ch, style) in styled_tiles {
            if y != prev_y {
                lines.push(Line::from(current_line));
                current_line = Vec::new();
                prev_y = y;
            }
            current_line.push(Span::styled(ch.to_string(), style));
        }
        if !current_line.is_empty() {
            lines.push(Line::from(current_line));
        }

        let map_widget = Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title(
            format!(" Cellular Automata Cave (Seed: {}) (120x60) ", floor.seed),
        ));

        f.render_widget(map_widget, chunks[1]);
    } else {
        let placeholder = Paragraph::new("Generate a floor to see the dungeon preview")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Cellular Automata Cave Preview (120x60) "),
            )
            .style(Style::default().fg(Color::DarkGray));

        f.render_widget(placeholder, chunks[1]);
    }

    let current_seed = if let Some(floor) = &app.current_floor {
        floor.seed.to_string()
    } else {
        "None".to_string()
    };

    let seed_display = format!("SEED: {}", current_seed);
    let hints = vec![
        ("R", "Random Roll", Some(Color::Cyan)),
        ("ENTER", "Generate", Some(Color::Green)),
        ("P", "Play Test", Some(Color::Magenta)),
        ("BACKSPACE", "Edit", Some(Color::Yellow)),
        ("ESC", "Back", Some(Color::Red)),
        (seed_display.as_str(), "", None),
    ];

    super::drawing::render_key_hints(f, area, hints);
}
