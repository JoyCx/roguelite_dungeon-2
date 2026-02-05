use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(area);

    // Header with gold and instructions
    let header = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::Yellow))
        .title(" ⚔️ SKILL TREE - Upgrade Your Stats ");

    let header_text = format!(
        "💰 Gold: {} | Path: {} | [↑↓] Navigate | [ENTER] Purchase | [ESC] Exit",
        app.character.gold,
        if let Some(path) = app.character.skill_tree_path.chosen_path {
            path.name().to_string()
        } else {
            "None".to_string()
        }
    );

    f.render_widget(Paragraph::new(header_text).block(header), chunks[0]);

    // Main skill tree display
    draw_skill_tree(f, app, chunks[1]);

    // Footer with stat display
    let footer = Block::default()
        .borders(Borders::TOP)
        .style(Style::default().fg(Color::Cyan));

    let bonuses = app.character.skill_tree_path.get_total_bonuses();
    let footer_text = format!(
        "Health: {}x | Damage: {}x | Speed: {}x | [T] to close",
        format!("{:.1}%", (bonuses.health_multiplier - 1.0) * 100.0 + 100.0),
        format!("{:.1}%", (bonuses.damage_multiplier - 1.0) * 100.0 + 100.0),
        format!("{:.1}%", (bonuses.speed_multiplier - 1.0) * 100.0 + 100.0),
    );

    f.render_widget(Paragraph::new(footer_text).block(footer), chunks[2]);
}

fn draw_skill_tree(f: &mut Frame, app: &mut App, area: Rect) {
    let paths = app.character.skill_tree_path.get_available_paths();
    let selected_index = app.skill_tree_selection.unwrap_or(0);

    let mut lines = vec![
        Line::from(Span::styled(
            "═══════════════════════════════════════════════════════════════",
            Style::default().fg(Color::Gray),
        )),
        Line::from(""),
    ];

    for (idx, path_type) in paths.iter().enumerate() {
        let node = app.character.skill_tree_path.get_path(*path_type).unwrap();

        let is_selected = idx == selected_index;
        let prefix = if is_selected { "▶ " } else { "  " };
        let status_text = if node.is_unlocked() {
            format!("Level {} (Unlocked)", node.level)
        } else {
            "Locked".to_string()
        };

        let cost_text = if node.is_unlocked() {
            format!("Next: {} gold", node.get_next_cost())
        } else {
            format!("Cost: {} gold", node.get_next_cost())
        };

        let line_style = if is_selected {
            Style::default().fg(Color::Yellow).bold()
        } else if app
            .character
            .skill_tree_path
            .chosen_path
            .is_some_and(|p| path_type.conflicts_with(p) && p != *path_type)
        {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default().fg(Color::Green)
        };

        // Main path line
        lines.push(Line::from(vec![
            Span::styled(prefix, line_style),
            Span::styled(path_type.name(), line_style.bold()),
            Span::raw(" - "),
            Span::styled(status_text.clone(), line_style),
        ]));

        // Description
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(path_type.description(), Style::default().fg(Color::Cyan)),
        ]));

        // Cost
        lines.push(Line::from(vec![
            Span::raw("    "),
            Span::styled(cost_text.clone(), Style::default().fg(Color::Magenta)),
        ]));

        // Stat bonus preview
        let bonus_text = format!(
            "    Bonus: Health {}x | Damage {}x | Speed {}x",
            format!(
                "{:.1}%",
                (node.stat_bonus.health_multiplier - 1.0) * 100.0 + 100.0
            ),
            format!(
                "{:.1}%",
                (node.stat_bonus.damage_multiplier - 1.0) * 100.0 + 100.0
            ),
            format!(
                "{:.1}%",
                (node.stat_bonus.speed_multiplier - 1.0) * 100.0 + 100.0
            ),
        );
        lines.push(Line::from(Span::styled(
            bonus_text,
            Style::default().fg(Color::Blue),
        )));

        // Separator
        if idx < paths.len() - 1 {
            lines.push(Line::from(Span::styled(
                "───────────────────────────────────────────────────────────────",
                Style::default().fg(Color::Gray),
            )));
        }

        lines.push(Line::from(""));
    }

    // Add blocked paths info
    if app.character.skill_tree_path.chosen_path.is_some() {
        lines.push(Line::from(Span::styled(
            "⚠️  One path chosen - conflicting paths are locked!",
            Style::default().fg(Color::Red),
        )));
    }

    let widget = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Upgrade Paths "),
        )
        .scroll((0, 0));

    f.render_widget(widget, area);
}
