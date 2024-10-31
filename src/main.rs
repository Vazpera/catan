use std::io;
use std::task::Context;

use backend::peer;
use futures::FutureExt;
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

async fn runtime(app: Arc<App>) -> Result<(), Box<dyn std::error::Error>> {
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui: Tui<CrosstermBackend<io::Stdout>> = Tui::new(terminal, events);
    tui.init()?;

    let held = app.clone();
    // Start the main loop.
    loop {

        if *app.running.lock().unwrap() {
            match tui.events.next().await.unwrap() {
                Event::Tick => {
                    app.tick();
                }
                Event::Key(key_event) => {
                    handle_key_events(key_event, app.clone()).await.unwrap();
                }
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an application.
    let app = Arc::new(App::new());
    let server = peer::peer(std::env::args().collect(), app.clone());
    let runtime = runtime(app.clone());
    tokio::join!(server, runtime);
    // Initialize the terminal user interface.

    Ok(())
}
