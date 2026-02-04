use crate::app::App;
use ratatui::{prelude::*, widgets::*};

pub fn draw(f: &mut Frame, app: &mut App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .split(area);

    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(" 🔷 DEV MENU - Testing & Debug ")
        .style(Style::default().fg(Color::Yellow));

    let dev_info = if let Some(floor) = &app.current_floor {
        format!(
            "🎮 SEED: {} | 👥 Enemies: {} | 💰 Gold: {} | ⚔️ Difficulty: {:?}",
            floor.seed,
            floor.enemies.len(),
            app.character.gold,
            app.settings.difficulty
        )
    } else {
        "🔷 No floor loaded - Generate one first".to_string()
    };

    let input_text = if app.dev_seed_input.is_empty() {
        "[Commands: R=Random | ENTER=Generate | E=Spawn Enemy | D=Damage Test | G=Add Gold | H=Cycle Attack Pattern | P=Play | ESC=Back]"
            .to_string()
    } else {
        format!("Seed Input: {}", app.dev_seed_input)
    };

    let quick_stats = format!(
        "🧙 Player HP: {}/{} | Weapon: {:?} | Damage: +{}",
        app.character.health,
        app.character.health_max,
        app.character
            .weapon_inventory
            .get_current_weapon()
            .map(|w| format!("{:?}", w.weapon_type)),
        crate::constants::PLAYER_BASE_DAMAGE
    );

    let input = Paragraph::new(vec![
        Line::from(Span::styled(dev_info, Style::default().fg(Color::Cyan))),
        Line::from(""),
        Line::from(Span::styled(input_text, Style::default().fg(Color::Green))),
        Line::from(Span::styled(
            quick_stats,
            Style::default().fg(Color::Magenta),
        )),
    ])
    .block(input_block);

    f.render_widget(input, chunks[0]);

    // Floor preview and enemy list
    if let Some(floor) = &app.current_floor {
        let preview_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(chunks[1]);

        // Map preview
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

        let map_widget = Paragraph::new(lines).block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 🗺️  Dungeon Map (120x60) "),
        );

        f.render_widget(map_widget, preview_chunks[0]);

        // Render active animations on top of the map
        super::drawing::render_animations(f, preview_chunks[0], &app.active_animations, 0, 0);

        // Enemy list panel
        let mut enemy_lines = vec![
            Line::from(Span::styled(
                format!("📊 ENEMIES ({})", floor.enemies.len()),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
        ];

        for (idx, enemy) in floor.enemies.iter().enumerate() {
            if idx >= 20 {
                enemy_lines.push(Line::from(Span::styled(
                    format!("... and {} more", floor.enemies.len() - idx),
                    Style::default().fg(Color::DarkGray),
                )));
                break;
            }

            let health_bar = if enemy.max_health > 0 {
                let health_ratio = (enemy.health as f32 / enemy.max_health as f32) * 8.0;
                let filled = health_ratio as usize;
                format!(
                    "[{}{}] {}/{}",
                    "█".repeat(filled),
                    "░".repeat((8 as usize).saturating_sub(filled)),
                    enemy.health,
                    enemy.max_health
                )
            } else {
                "[DEAD]".to_string()
            };

            let color = match enemy.rarity {
                crate::model::enemy_type::EnemyRarity::Fighter => Color::Gray,
                crate::model::enemy_type::EnemyRarity::Guard => Color::Green,
                crate::model::enemy_type::EnemyRarity::Champion => Color::Cyan,
                crate::model::enemy_type::EnemyRarity::Elite => Color::Magenta,
                crate::model::enemy_type::EnemyRarity::Boss => Color::Red,
            };

            let rarity_short = match enemy.rarity {
                crate::model::enemy_type::EnemyRarity::Fighter => "Fighter",
                crate::model::enemy_type::EnemyRarity::Guard => "Guard",
                crate::model::enemy_type::EnemyRarity::Champion => "Champion",
                crate::model::enemy_type::EnemyRarity::Elite => "Elite",
                crate::model::enemy_type::EnemyRarity::Boss => "Boss",
            };

            let enemy_info = format!(
                "{}. [{}] {} {}{:2}💰",
                idx + 1,
                rarity_short,
                health_bar,
                if enemy.health <= 0 { "DEAD " } else { "" },
                enemy.base_gold
            );
            enemy_lines.push(Line::from(Span::styled(
                enemy_info,
                Style::default().fg(color),
            )));
        }

        if floor.enemies.is_empty() {
            enemy_lines.push(Line::from(Span::styled(
                "No enemies - Press E to spawn test enemies",
                Style::default().fg(Color::DarkGray),
            )));
        }

        let enemies_widget = Paragraph::new(enemy_lines)
            .block(Block::default().borders(Borders::ALL).title(" 👹 Enemies "))
            .scroll((0, 0));

        f.render_widget(enemies_widget, preview_chunks[1]);
    } else {
        let placeholder = Paragraph::new(vec![
            Line::from(""),
            Line::from("🔷 Generate a floor first with ENTER"),
            Line::from(""),
            Line::from("Then use these commands:"),
            Line::from("  E - Spawn test enemy"),
            Line::from("  D - Test damage (5 HP to all enemies)"),
            Line::from("  G - Add 50 test gold"),
            Line::from("  P - Play the floor"),
        ])
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" 🗺️  Dungeon Map Preview (120x60) "),
        )
        .style(Style::default().fg(Color::DarkGray));

        f.render_widget(placeholder, chunks[1]);
    }

    let hints = vec![
        ("R", "Random", Some(Color::Cyan)),
        ("ENTER", "Generate", Some(Color::Green)),
        ("E", "Enemy", Some(Color::Yellow)),
        ("D", "Damage", Some(Color::LightRed)),
        ("G", "Gold", Some(Color::LightYellow)),
        ("P", "Play", Some(Color::Magenta)),
        ("ESC", "Back", Some(Color::Red)),
    ];

    super::drawing::render_key_hints(f, area, hints);
}

pub fn handle_input(app: &mut App, key: crossterm::event::KeyCode) {
    use crossterm::event::KeyCode;

    match key {
        KeyCode::Char('r') | KeyCode::Char('R') => {
            // Random seed
            use rand::Rng;
            let mut rng = rand::rng();
            let seed = rng.random::<u64>();
            app.dev_seed_input = seed.to_string();
        }
        KeyCode::Char('e') | KeyCode::Char('E') => {
            // Spawn test enemy on map
            if let Some(floor) = &mut app.current_floor {
                if let Some(pos) = find_empty_spawn_position(floor) {
                    spawn_test_enemy(floor, pos.0, pos.1, &app.settings.difficulty);
                }
            }
        }
        KeyCode::Char('d') | KeyCode::Char('D') => {
            // Damage test - hit all enemies for 5 damage
            if let Some(floor) = &mut app.current_floor {
                for enemy in &mut floor.enemies {
                    enemy.take_damage(5);
                }
            }
        }
        KeyCode::Char('g') | KeyCode::Char('G') => {
            // Add gold for testing
            app.character.gold += 50;
        }
        KeyCode::Char('h') | KeyCode::Char('H') => {
            // Cycle attack pattern for testing
            app.cycle_dev_attack_pattern();
        }
        KeyCode::Backspace => {
            // Edit seed
            app.dev_seed_input.pop();
        }
        KeyCode::Char(c) if c.is_numeric() => {
            // Add to seed input
            if app.dev_seed_input.len() < 20 {
                app.dev_seed_input.push(c);
            }
        }
        _ => {}
    }
}

/// Find an empty position to spawn a test enemy
fn find_empty_spawn_position(floor: &crate::model::floor::Floor) -> Option<(i32, i32)> {
    use rand::Rng;
    let mut rng = rand::rng();

    for _ in 0..100 {
        let x = rng.random_range(1..floor.width as i32 - 1);
        let y = rng.random_range(1..floor.height as i32 - 1);

        if !floor.get_tile(x, y) && !floor.enemy_exists_at(x, y) && !floor.item_exists_at(x, y) {
            return Some((x, y));
        }
    }
    None
}

/// Spawn a test enemy for dev testing
fn spawn_test_enemy(
    floor: &mut crate::model::floor::Floor,
    x: i32,
    y: i32,
    difficulty: &crate::model::item_tier::Difficulty,
) {
    use crate::model::enemy_type;

    let templates = enemy_type::get_enemies_for_difficulty(difficulty);
    if templates.is_empty() {
        return;
    }

    use rand::Rng;
    let mut rng = rand::rng();
    let template = templates[rng.random_range(0..templates.len())].clone();

    let mut enemy = crate::model::enemy::Enemy::new(x, y, template.speed);
    enemy.health = template.health;
    enemy.max_health = template.health;
    enemy.rarity = template.rarity.clone();
    enemy.base_gold = template.rarity.calculate_gold_drop(difficulty);

    floor.enemies.push(enemy);
}
