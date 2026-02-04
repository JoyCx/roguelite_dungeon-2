use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect, pulse_color: Color) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(area);

    // Title
    let title = Paragraph::new("⚔ CREATE YOUR CHARACTER ⚔")
        .alignment(Alignment::Center)
        .style(
            Style::default()
                .fg(pulse_color)
                .add_modifier(Modifier::BOLD),
        );
    f.render_widget(title, chunks[0]);

    // Main content area
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Length(4),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(chunks[1]);

    // Character name input
    let name_label = if app.char_name_input_mode {
        "[ CHARACTER NAME ] ← ENTER YOUR NAME →"
    } else {
        "CHARACTER NAME"
    };

    let name_display = if app.char_name.is_empty() {
        "___________________".to_string()
    } else {
        app.char_name.clone()
    };

    let name_input_style = if app.char_name_input_mode {
        Style::default()
            .fg(pulse_color)
            .add_modifier(Modifier::BOLD)
    } else if app.char_creation_selection == 0 {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };

    let name_block = Block::default()
        .borders(Borders::ALL)
        .border_style(name_input_style)
        .title(name_label);

    let name_widget = Paragraph::new(name_display)
        .block(name_block)
        .alignment(Alignment::Center)
        .style(name_input_style);

    f.render_widget(name_widget, content_chunks[0]);

    // Difficulty selection
    let difficulty_label = if app.char_creation_selection == 1 {
        "[ DIFFICULTY ] ← Use Arrow Keys →"
    } else {
        "DIFFICULTY"
    };

    let difficulty_style = if app.char_creation_selection == 1 {
        Style::default()
            .fg(pulse_color)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let difficulty_display = match &app.settings.difficulty {
        crate::model::item_tier::Difficulty::Easy => {
            vec![
                Line::from(Span::styled("◄ EASY ►", difficulty_style)),
                Line::from(""),
                Line::from("Fewer rare drops"),
                Line::from("Recommended for learning"),
            ]
        }
        crate::model::item_tier::Difficulty::Normal => {
            vec![
                Line::from(Span::styled("◄ NORMAL ►", difficulty_style)),
                Line::from(""),
                Line::from("Balanced difficulty"),
                Line::from("Standard gameplay"),
            ]
        }
        crate::model::item_tier::Difficulty::Hard => {
            vec![
                Line::from(Span::styled("◄ HARD ►", difficulty_style)),
                Line::from(""),
                Line::from("More rare drops"),
                Line::from("Challenging combat"),
            ]
        }
        crate::model::item_tier::Difficulty::Death => {
            vec![
                Line::from(Span::styled("◄ DEATH ►", difficulty_style)),
                Line::from(""),
                Line::from("Extremely rare drops"),
                Line::from("For the brave or foolish"),
            ]
        }
    };

    let difficulty_block = Block::default()
        .borders(Borders::ALL)
        .border_style(difficulty_style)
        .title(difficulty_label);

    let difficulty_widget = Paragraph::new(difficulty_display)
        .block(difficulty_block)
        .alignment(Alignment::Center);

    f.render_widget(difficulty_widget, content_chunks[1]);

    // Start button
    let start_style = if app.char_creation_selection == 2 {
        Style::default()
            .fg(Color::Black)
            .bg(pulse_color)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };

    let start_block = Block::default()
        .borders(Borders::ALL)
        .border_style(start_style);

    let start_widget = Paragraph::new("[ PRESS ENTER TO START ]")
        .block(start_block)
        .alignment(Alignment::Center)
        .style(start_style);

    f.render_widget(start_widget, content_chunks[2]);

    // Instructions
    let instructions = Paragraph::new(
        "ESC: Back to Menu | TAB: Switch Fields | WASD/Arrows: Navigate | ENTER: Confirm",
    )
    .alignment(Alignment::Center)
    .style(Style::default().fg(Color::DarkGray));
    f.render_widget(instructions, content_chunks[3]);
}
