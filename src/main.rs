use crossterm::{
    event::{self, Event},
    terminal::*,
};
use ratatui::prelude::*;
use std::{
    io,
    time::{Duration, Instant},
};

mod app;
mod colors;
mod constants;
mod emoji;
mod input;
mod model;
mod ui;

pub const LOGO: &str = include_str!("logo.txt");

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    crossterm::execute!(stdout, EnterAlternateScreen, event::EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let mut app = app::App::new();
    let _ = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    crossterm::execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;
    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut app::App,
) -> Result<(), Box<dyn std::error::Error>>
where
    <B as Backend>::Error: 'static,
{
    let tick_rate = Duration::from_millis(16);
    let mut last_tick = Instant::now();

    loop {
        let size = terminal.size()?;
        app.update_terminal_size(size.width, size.height);

        app.update_camera();

        app.update_camera_smooth();

        app.update_game_logic();

        app.update_arrows();

        // Update audio (fade transitions)
        let elapsed = last_tick.elapsed().as_secs_f32();
        app.audio_manager.update(elapsed);

        terminal.draw(|f| ui::draw(f, app))?;
        app.last_scroll_offset = app.scroll_offset;

        let elapsed = last_tick.elapsed();
        let timeout = if elapsed < tick_rate {
            tick_rate - elapsed
        } else {
            Duration::from_secs(0)
        };

        if event::poll(timeout)? {
            match event::read()? {
                Event::Key(key) => {
                    if key.kind == event::KeyEventKind::Press {
                        input::handle_input(app, key);
                    }
                }
                Event::Mouse(mouse) => {
                    input::handle_mouse_event(app, mouse);
                }
                _ => {}
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.frame_count += 1;
            last_tick = Instant::now();
        }

        if app.should_quit {
            // Save the game before quitting if in game state
            if app.state == app::AppState::Game {
                let _ = app.save_game();
            }
            return Ok(());
        }
    }
}
