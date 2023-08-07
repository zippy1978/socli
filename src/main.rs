use std::{
    io::{self},
    sync::Arc,
    time::Duration,
};

use clap::Parser;
use crossterm::{execute, terminal::LeaveAlternateScreen};
use log::LevelFilter;
use socli::{
    app::{
        input::{events::Events, InputEvent},
        io::{handler::IoAsyncHandler, IoEvent},
        ui::{check_window_size, draw},
        App, AppReturn,
    },
    core::{service::player::PlayerService, setup_container},
    resolve_trait,
};
use ratatui::{backend::CrosstermBackend, Terminal};

/// SoCli - A Sorare NBA 🏀 CLI
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Strategy scripts folder path
    #[arg(short, long)]
    strategies: String,
    /// Reset stored data
    #[clap(long, short, action)]
    reset: bool,
}

pub async fn start_ui(app: &Arc<tokio::sync::Mutex<App>>) -> io::Result<()> {
    // Configure Crossterm backend for tui
    let stdout = io::stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // User event handler
    let tick_rate = Duration::from_millis(200);
    let mut events = Events::new(tick_rate);

    // Trigger state change from Init to Initialized
    {
        let mut app = app.lock().await;
        // Here we assume the the first load is a long task
        app.dispatch(IoEvent::Initialize).await;
    }

    loop {
        let mut app = app.lock().await;

        // Check terminal size
        if let Err(msg) = check_window_size(&terminal.size().unwrap()) {
            println!("{}", msg);
            break;
        }

        // Render
        terminal.draw(|rect| draw(rect, &app))?;

        // Handle inputs
        let result = match events.next().await {
            // Let's process that event
            InputEvent::Input(key) => app.do_action(key).await,
            // Handle no user input
            //InputEvent::Tick => app.update_on_tick().await,
            InputEvent::Tick => AppReturn::Continue,
        };
        // Check if we should exit
        if result == AppReturn::Exit {
            events.close();
            break;
        }
    }

    // Restore the terminal and close application
    terminal.flush()?;
    terminal.clear()?;
    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Parse args
    let args = Args::parse();

    // Configure log
    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);

    // Init core container
    setup_container(&args.strategies)
        .await
        .expect("failed to intialize container");

    // Reset storage
    if args.reset {
        resolve_trait!(PlayerService)
            .clear_storage()
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    }

    // Create a channel for IoEvent
    let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel::<IoEvent>(100);

    // Create app
    let app = Arc::new(tokio::sync::Mutex::new(App::new(sync_io_tx.clone())));
    let app_ui = Arc::clone(&app);

    // Handle IO in a specifc thread
    tokio::spawn(async move {
        let mut handler = IoAsyncHandler::new(app);
        while let Some(io_event) = sync_io_rx.recv().await {
            handler.handle_io_event(io_event).await;
        }
    });

    start_ui(&app_ui).await?;

    Ok(())
}
