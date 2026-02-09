use crate::app::{App, SettingsMode};
use ratatui::{prelude::*, widgets::*};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect, pulse: Color) {
    let s = &app.temp_settings;
    let skip_anim_check = if s.skip_logo_animation { "☑" } else { "☐" };
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
        format_volume_bar("Music Volume", s.music_volume),
        format_volume_bar("Sound Volume", s.sound_volume),
        format!("Skip Logo Animation: {}", skip_anim_check),
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
            if i == 20 {
                style = style.fg(Color::Green);
            } else if i == 21 {
                style = style.fg(Color::Yellow);
            } else if i == 22 {
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

fn format_volume_bar(label: &str, volume: f32) -> String {
    let filled = (volume * 20.0) as usize;
    let empty = 20 - filled;
    let bar = format!("[{}{}]", "█".repeat(filled), "░".repeat(empty));
    let percent = format!("{:.0}%", volume * 100.0);
    format!("{:<20} {}{}", label, bar, percent)
}
