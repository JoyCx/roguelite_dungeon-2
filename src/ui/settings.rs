use crate::app::{App, SettingsMode};
use ratatui::{prelude::*, widgets::*};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect, pulse: Color) {
    let s = &app.temp_settings;
    let items = vec![
        format!("Move Up:    [{}]", s.move_up),
        format!("Move Left:  [{}]", s.move_left),
        format!("Move Down:  [{}]", s.move_down),
        format!("Move Right: [{}]", s.move_right),
        format!("Attack:     [{}]", s.attack),
        format!("Dash:       [{}]", s.dash),
        format!("Block:      [{}]", s.block),
        format!("Pick Up:    [{}]", s.pick_up),
        format!("Inventory:  [{}]", s.toggle_inv),
        format!("Special:    [{}]", s.special_item),
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
            if i == 11 {
                style = style.fg(Color::Green);
            } else if i == 12 {
                style = style.fg(Color::Yellow);
            } else if i == 13 {
                style = style.fg(Color::Red);
            }
            ListItem::new(text.as_str()).style(style)
        })
        .collect();

    let title = if let SettingsMode::Rebinding = app.settings_mode {
        " [ PRESS ANY KEY ] "
    } else {
        " SETTINGS "
    };
    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().bg(pulse).fg(Color::White));

    f.render_stateful_widget(list, area, &mut app.settings_state);
}
