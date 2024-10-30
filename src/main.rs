use std::io;

use ratatui::{backend::CrosstermBackend, Terminal};
use std::sync::Arc;
use std::sync::Mutex;

use crate::{
    app::App,
    event::{Event, EventHandler},
    handler::handle_key_events,
    tui::Tui,
};

mod backend;

pub mod app;
pub mod event;
pub mod handler;
pub mod tui;
pub mod ui;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an application.
    let app = Arc::new(Mutex::new(App::new()));

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui: Tui<CrosstermBackend<io::Stdout>> = Tui::new(terminal, events);
    tui.init()?;

    let held = app.clone();
    // Start the main loop.
    loop {
        let x = held.lock().unwrap();
        if x.running {
            match tui.events.next().await.unwrap() {
                Event::Tick => {
                    x.tick();
                    drop(x);
                }
                Event::Key(key_event) => {
                    drop(x);
                    handle_key_events(key_event, app.clone()).await.unwrap();
                }
                Event::Mouse(_) => drop(x),
                Event::Resize(_, _) => drop(x),
            }
            tui.draw(app.clone())?;
        } else {
            break;
        }
    }
    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
