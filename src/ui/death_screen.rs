use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    // Update fade timer
    let fade_speed = 0.015; // Controls how fast the fade happens
    let max_fade = 1.0;
    if app.death_screen_fade_timer < max_fade {
        app.death_screen_fade_timer += fade_speed;
    }

    // Calculate fade color - starts transparent, becomes more opaque
    let alpha_progress = (app.death_screen_fade_timer).min(1.0);
    let fade_color = if alpha_progress < 0.3 {
        Color::Red
    } else if alpha_progress < 0.6 {
        Color::LightRed
    } else {
        Color::Yellow
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(8)])
        .split(area);

    // Main death screen content
    let mut lines = vec![
        Line::from(""),
        Line::from(Span::styled(
            "YOU DIED, AGAIN?",
            Style::default()
                .fg(fade_color)
                .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        )),
        Line::from(""),
    ];

    // Death stats
    let time_minutes = app.death_time_elapsed as i32 / 60;
    let time_seconds = (app.death_time_elapsed as i32) % 60;

    lines.push(Line::from(Span::styled(
        format!("⏱️  Time Survived: {}m {}s", time_minutes, time_seconds),
        Style::default().fg(Color::Yellow),
    )));

    lines.push(Line::from(Span::styled(
        format!("⚔️  Enemies Slain: {}", app.character.enemies_killed),
        Style::default().fg(Color::Cyan),
    )));

    lines.push(Line::from(Span::styled(
        format!("📈 Levels Passed: {}", app.levels_passed_before_death),
        Style::default().fg(Color::Green),
    )));

    lines.push(Line::from(Span::styled(
        format!("💰 Gold Collected: {}", app.character.gold),
        Style::default().fg(Color::LightYellow),
    )));

    lines.push(Line::from(""));

    let death_block = Block::default()
        .borders(Borders::ALL)
        .title(" ☠️  DEATH SCREEN ☠️ ")
        .title_alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));

    let death_widget = Paragraph::new(lines)
        .alignment(Alignment::Center)
        .block(death_block);

    f.render_widget(death_widget, chunks[0]);

    // Bottom control hints - only show if fade is complete
    if alpha_progress > 0.7 {
        let hints = vec![
            ("R", "Retry Floor", Some(Color::Green)),
            ("N", "New Game", Some(Color::Yellow)),
            ("ESC", "Main Menu", Some(Color::Red)),
        ];

        super::drawing::render_key_hints(f, area, hints);
    }
}

pub fn handle_input(app: &mut App, key: crossterm::event::KeyCode) {
    use crossterm::event::KeyCode;

    match key {
        KeyCode::Char('r') | KeyCode::Char('R') => {
            // Retry current floor with reset health and position
            app.retry_current_floor();
        }
        KeyCode::Char('n') | KeyCode::Char('N') => {
            // Start a completely new game
            app.state = crate::app::AppState::CharacterCreation;
            app.char_name = String::new();
            app.char_name_input_mode = true;
            app.char_creation_selection = 0;
            app.settings.difficulty = app.settings.default_difficulty.clone();
        }
        KeyCode::Esc => {
            app.state = crate::app::AppState::MainMenu;
            app.main_menu_state.select(Some(0));
        }
        _ => {}
    }
}
