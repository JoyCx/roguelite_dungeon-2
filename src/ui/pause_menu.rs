use crate::app::{App, PauseSubmenu, SettingsMode};
use ratatui::{prelude::*, widgets::*};

const MAIN_MENU_ITEMS: &[&str] = &["Resume", "Volume", "Settings", "Quit"];

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    // Clear background
    f.render_widget(Clear, area);

    // Dim the background
    let dim_block = Block::default().style(Style::default().bg(Color::Black));
    f.render_widget(dim_block, area);

    // Create centered area for pause menu
    let popup_width = 60;
    let popup_height = 18;
    let popup_area = centered_rect(popup_width, popup_height, area);

    match app.pause_submenu {
        None => draw_main_pause_menu(f, app, popup_area),
        Some(PauseSubmenu::Volume) => draw_volume_menu(f, app, popup_area),
        Some(PauseSubmenu::Settings) => draw_settings_menu(f, app, popup_area),
    }
}

fn draw_main_pause_menu(f: &mut Frame, app: &mut App, area: Rect) {
    let mut lines = vec![
        Line::from(Span::styled(
            "⏸  PAUSE MENU",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
    ];

    for (idx, item) in MAIN_MENU_ITEMS.iter().enumerate() {
        let selected = idx == app.pause_menu_selection;
        let style = if selected {
            Style::default()
                .fg(Color::Black)
                .bg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };

        let prefix = if selected { "> " } else { "  " };
        lines.push(Line::from(Span::styled(
            format!("{}{}", prefix, item),
            style,
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "↑ ↓ to navigate | ENTER to select | ESC to resume",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
    )));

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" PAUSE ")
        .style(Style::default().fg(Color::Yellow).bg(Color::Black));

    let paragraph = Paragraph::new(lines)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn draw_volume_menu(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
        ])
        .split(area);

    // Title
    let title = Paragraph::new(Span::styled(
        "🔊 VOLUME SETTINGS",
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    ))
    .alignment(Alignment::Center);

    f.render_widget(title, chunks[0]);

    // Music volume - left label, right slider
    let music_selected = app.pause_volume_selection == 0;
    let music_filled = (app.music_volume * 20.0) as usize;
    let music_empty = 20 - music_filled;
    let music_bar = format!("[{}{}]", "█".repeat(music_filled), "░".repeat(music_empty));
    let music_percent = format!("{:.0}%", app.music_volume * 100.0);

    let music_style = if music_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Cyan)
    };

    let mut music_line_spans = vec![];
    music_line_spans.push(Span::styled("Music Volume", music_style));

    // Calculate padding to align right
    let label_len = 12; // Length of "Music Volume"
    let bar_len = 22; // Length of "[████████████████████]"
    let space_len = 1; // The space before the percent
    let content_width = label_len + bar_len + space_len + music_percent.len();
    let padding = (area.width as usize)
        .saturating_sub(2)
        .saturating_sub(content_width);
    if padding > 0 {
        music_line_spans.push(Span::raw(" ".repeat(padding)));
    }

    music_line_spans.push(Span::styled(music_bar, music_style));
    music_line_spans.push(Span::styled(" ", music_style));
    music_line_spans.push(Span::styled(music_percent, music_style));

    let music_block = if music_selected {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Blue))
    } else {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
    };

    let music_para = Paragraph::new(Line::from(music_line_spans)).block(music_block);
    f.render_widget(music_para, chunks[1]);

    // Sound volume - left label, right slider
    let sound_selected = app.pause_volume_selection == 1;
    let sound_filled = (app.sound_volume * 20.0) as usize;
    let sound_empty = 20 - sound_filled;
    let sound_bar = format!("[{}{}]", "█".repeat(sound_filled), "░".repeat(sound_empty));
    let sound_percent = format!("{:.0}%", app.sound_volume * 100.0);

    let sound_style = if sound_selected {
        Style::default()
            .fg(Color::Black)
            .bg(Color::Magenta)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Magenta)
    };

    let mut sound_line_spans = vec![];
    sound_line_spans.push(Span::styled("Sound Volume", sound_style));

    let label_len = 12; // Length of "Sound Volume"
    let bar_len = 22; // Length of "[████████████████████]"
    let space_len = 1; // The space before the percent
    let content_width = label_len + bar_len + space_len + sound_percent.len();
    let padding = (area.width as usize)
        .saturating_sub(2)
        .saturating_sub(content_width);
    if padding > 0 {
        sound_line_spans.push(Span::raw(" ".repeat(padding)));
    }

    sound_line_spans.push(Span::styled(sound_bar, sound_style));
    sound_line_spans.push(Span::styled(" ", sound_style));
    sound_line_spans.push(Span::styled(sound_percent, sound_style));

    let sound_block = if sound_selected {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .style(Style::default().bg(Color::Blue))
    } else {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
    };

    let sound_para = Paragraph::new(Line::from(sound_line_spans)).block(sound_block);
    f.render_widget(sound_para, chunks[2]);

    // Instructions
    let instructions = Paragraph::new(Span::styled(
        "↑ ↓ to select | A/D or ← → to adjust | BACKSPACE to back",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::ITALIC),
    ));

    f.render_widget(instructions, chunks[3]);
}

fn draw_settings_menu(f: &mut Frame, app: &mut App, area: Rect) {
    let s = &app.pause_temp_settings;
    let items = vec![
        format!("Move Up:         [{}]", s.move_up),
        format!("Move Left:       [{}]", s.move_left),
        format!("Move Down:       [{}]", s.move_down),
        format!("Move Right:      [{}]", s.move_right),
        format!("Attack:          [{}]", s.attack),
        format!("Dash:            [{}]", s.dash),
        format!("Block:           [{}]", s.block),
        format!("Inventory Focus: [{}]", s.toggle_inv),
        format!("Use Consumable:  [{}]", s.use_consumable),
        format!("Inv Up:          [{}]", s.inventory_up),
        format!("Inv Down:        [{}]", s.inventory_down),
        format!("Item Describe:   [{}]", s.item_describe),
        format!("Pause:           [{}]", s.pause),
        format!("Special:         [{}]", s.special_item),
        format!("Difficulty:      [{}]", s.difficulty.name()),
        format!("Default Difficulty: [{}]", s.default_difficulty.name()),
        "-------------------".to_string(),
        "SAVE CHANGES".to_string(),
        "DISCARD & BACK".to_string(),
        "RESET TO DEFAULT".to_string(),
    ];

    let list_items: Vec<ListItem> = items
        .iter()
        .enumerate()
        .map(|(i, text)| {
            let mut style = Style::default().fg(Color::Gray);
            if i == 17 {
                style = style.fg(Color::Green);
            } else if i == 18 {
                style = style.fg(Color::Yellow);
            } else if i == 19 {
                style = style.fg(Color::Red);
            }
            ListItem::new(text.as_str()).style(style)
        })
        .collect();

    let title = if let SettingsMode::Rebinding = app.pause_rebinding_mode {
        " [ PRESS ANY KEY ] "
    } else {
        " SETTINGS "
    };

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    f.render_stateful_widget(list, area, &mut app.pause_settings_state);
}

/// Helper function to create a centered rect inside another rect
fn centered_rect(width: u16, height: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(r.height.saturating_sub(height) / 2),
            Constraint::Length(height),
            Constraint::Length(r.height.saturating_sub(height) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(r.width.saturating_sub(width) / 2),
            Constraint::Length(width),
            Constraint::Length(r.width.saturating_sub(width) / 2),
        ])
        .split(popup_layout[1])[1]
}

pub fn handle_input(app: &mut App, key: crossterm::event::KeyCode) {
    use crossterm::event::KeyCode;

    match app.pause_submenu {
        None => handle_main_menu_input(app, key),
        Some(crate::app::PauseSubmenu::Volume) => handle_volume_input(app, key),
        Some(crate::app::PauseSubmenu::Settings) => handle_settings_input(app, key),
    }
}

fn handle_main_menu_input(app: &mut App, key: crossterm::event::KeyCode) {
    use crossterm::event::KeyCode;

    match key {
        KeyCode::Esc => {
            // Resume game
            app.is_paused = false;
            app.pause_menu_selection = 0;
            app.pause_submenu = None;
            // Fade music back to settings volume
            app.audio_manager.resume_music();
            app.audio_manager
                .start_fade_in(0.5, app.settings.music_volume);
        }
        KeyCode::Up => {
            if app.pause_menu_selection > 0 {
                app.pause_menu_selection -= 1;
            } else {
                app.pause_menu_selection = MAIN_MENU_ITEMS.len() - 1;
            }
        }
        KeyCode::Down => {
            app.pause_menu_selection = (app.pause_menu_selection + 1) % MAIN_MENU_ITEMS.len();
        }
        KeyCode::Enter => {
            match app.pause_menu_selection {
                0 => {
                    // Resume
                    app.is_paused = false;
                    app.pause_menu_selection = 0;
                    app.pause_submenu = None;
                    // Fade music back to settings volume
                    app.audio_manager.resume_music();
                    app.audio_manager
                        .start_fade_in(0.5, app.settings.music_volume);
                }
                1 => {
                    // Volume
                    app.pause_volume_selection = 0;
                    app.pause_submenu = Some(PauseSubmenu::Volume);
                }
                2 => {
                    // Settings
                    app.pause_temp_settings = app.settings.clone();
                    app.pause_settings_state.select(Some(0));
                    app.pause_rebinding_mode = SettingsMode::Navigating;
                    app.pause_submenu = Some(PauseSubmenu::Settings);
                }
                3 => {
                    // Quit - save game before returning to menu
                    let _ = app.save_game();
                    app.state = crate::app::AppState::MainMenu;
                    app.is_paused = false;
                    app.pause_menu_selection = 0;
                    app.pause_submenu = None;
                }
                _ => {}
            }
        }
        _ => {}
    }
}

fn handle_volume_input(app: &mut App, key: crossterm::event::KeyCode) {
    use crossterm::event::KeyCode;

    match key {
        KeyCode::Up => {
            // Move to music volume
            app.pause_volume_selection = 0;
        }
        KeyCode::Down => {
            // Move to sound volume
            app.pause_volume_selection = 1;
        }
        KeyCode::Left | KeyCode::Char('a') | KeyCode::Char('A') => {
            // Decrease selected volume
            match app.pause_volume_selection {
                0 => {
                    app.music_volume = (app.music_volume - 0.05).max(0.0);
                    // Sync to audio manager immediately
                    app.audio_manager.set_music_volume(app.music_volume);
                }
                1 => {
                    app.sound_volume = (app.sound_volume - 0.05).max(0.0);
                    // Sync to audio manager immediately
                    app.audio_manager.set_sound_volume(app.sound_volume);
                }
                _ => {}
            }
        }
        KeyCode::Right | KeyCode::Char('d') | KeyCode::Char('D') => {
            // Increase selected volume
            match app.pause_volume_selection {
                0 => {
                    app.music_volume = (app.music_volume + 0.05).min(1.0);
                    // Sync to audio manager immediately
                    app.audio_manager.set_music_volume(app.music_volume);
                }
                1 => {
                    app.sound_volume = (app.sound_volume + 0.05).min(1.0);
                    // Sync to audio manager immediately
                    app.audio_manager.set_sound_volume(app.sound_volume);
                }
                _ => {}
            }
        }
        KeyCode::Backspace => {
            // Go back to main pause menu
            app.pause_submenu = None;
        }
        _ => {}
    }
}

fn handle_settings_input(app: &mut App, key: crossterm::event::KeyCode) {
    use crossterm::event::KeyCode;

    if let SettingsMode::Rebinding = app.pause_rebinding_mode {
        // In rebinding mode, any key press rebinds
        let selected = app.pause_settings_state.selected().unwrap_or(0);
        let key_string = key_code_to_string(key);

        match selected {
            0 => app.pause_temp_settings.move_up = key_string,
            1 => app.pause_temp_settings.move_left = key_string,
            2 => app.pause_temp_settings.move_down = key_string,
            3 => app.pause_temp_settings.move_right = key_string,
            4 => app.pause_temp_settings.attack = key_string,
            5 => app.pause_temp_settings.dash = key_string,
            6 => app.pause_temp_settings.block = key_string,
            7 => app.pause_temp_settings.toggle_inv = key_string,
            8 => app.pause_temp_settings.use_consumable = key_string,
            9 => app.pause_temp_settings.inventory_up = key_string,
            10 => app.pause_temp_settings.inventory_down = key_string,
            11 => app.pause_temp_settings.item_describe = key_string,
            12 => app.pause_temp_settings.pause = key_string,
            13 => app.pause_temp_settings.special_item = key_string,
            _ => {}
        }

        app.pause_rebinding_mode = SettingsMode::Navigating;
    } else {
        // Normal navigation mode
        match key {
            KeyCode::Up => {
                let current = app.pause_settings_state.selected().unwrap_or(0);
                let new_selection = if current == 0 { 19 } else { current - 1 };
                app.pause_settings_state.select(Some(new_selection));
            }
            KeyCode::Down => {
                let current = app.pause_settings_state.selected().unwrap_or(0);
                let new_selection = (current + 1) % 20;
                app.pause_settings_state.select(Some(new_selection));
            }
            KeyCode::Enter => {
                let selected = app.pause_settings_state.selected().unwrap_or(0);
                match selected {
                    0..=15 => {
                        // Rebindable keys
                        app.pause_rebinding_mode = SettingsMode::Rebinding;
                    }
                    17 => {
                        // Save changes
                        app.settings = app.pause_temp_settings.clone();
                        // Update volume settings
                        app.settings.music_volume = app.music_volume;
                        app.settings.sound_volume = app.sound_volume;
                        let _ = app.settings.save();
                        app.pause_submenu = None;
                    }
                    18 => {
                        // Discard & back
                        app.pause_submenu = None;
                    }
                    19 => {
                        // Reset to default
                        app.pause_temp_settings = crate::model::settings::Settings::default();
                        app.pause_settings_state.select(Some(0));
                    }
                    _ => {}
                }
            }
            KeyCode::Backspace => {
                // Go back without saving
                app.pause_submenu = None;
            }
            _ => {}
        }
    }
}

fn key_code_to_string(key: crossterm::event::KeyCode) -> String {
    use crossterm::event::KeyCode;
    match key {
        KeyCode::Char(c) => c.to_uppercase().to_string(),
        KeyCode::F(n) => format!("F{}", n),
        KeyCode::Up => "Up".to_string(),
        KeyCode::Down => "Down".to_string(),
        KeyCode::Left => "Left".to_string(),
        KeyCode::Right => "Right".to_string(),
        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Backspace => "Backspace".to_string(),
        KeyCode::Esc => "Esc".to_string(),
        KeyCode::Tab => "Tab".to_string(),
        KeyCode::BackTab => "BackTab".to_string(),
        KeyCode::Delete => "Delete".to_string(),
        KeyCode::Home => "Home".to_string(),
        KeyCode::End => "End".to_string(),
        KeyCode::PageUp => "PageUp".to_string(),
        KeyCode::PageDown => "PageDown".to_string(),
        _ => "?".to_string(),
    }
}
