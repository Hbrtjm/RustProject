mod frontend;
mod converter;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::{
    env,
    error::Error,
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use frontend::events::{AppEvent, AppState, handle_input};
use frontend::ui::draw;

const TICK_RATE: Duration = Duration::from_millis(250);

fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal: Terminal<CrosstermBackend<std::io::Stdout>> = Terminal::new(backend)?;

    // Initialize app state
    let cwd = env::current_dir().unwrap_or_else(|_| {
        dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("/"))
    });
    
    let mut app = AppState::new(cwd);

    // Setup input handling
    let (tx, rx) = mpsc::channel();
    let tick_rate = TICK_RATE;
    
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let Event::Key(key) = event::read().expect("can read events") {
                    tx.send(AppEvent::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(AppEvent::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    // Main loop
    loop {
        terminal.draw(|f: &mut ratatui::Frame | draw::<CrosstermBackend<std::io::Stdout>>(f, &app))?;

        match rx.recv()? {
            AppEvent::Input(key) => {
                // Handle quit with 'q' key specifically
                if key.code == KeyCode::Char('q') && app.command_buffer.is_empty() {
                    break;
                }
                
                if handle_input(&mut app, key) {
                    break;
                }
            }
            AppEvent::Tick => {
                // Handle periodic updates if needed
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // Print conversion summary
    if !app.to_convert.is_empty() {
        println!("Selected files for conversion:");
        for (path, format) in &app.to_convert {
            println!("  {} → {:?}", path.display(), format);
        }
    }

    Ok(())
}