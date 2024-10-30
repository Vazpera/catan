use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::io;
use std::ops::Deref;

use futures::lock::MutexGuard;
use ratatui::{backend::CrosstermBackend, Terminal};
use std::sync::Arc;
use std::sync::Mutex;

use crate::{
    app::{App, AppResult},
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

async fn loop_section(
    app: Arc<Mutex<App>>,
    tui: &mut Tui<CrosstermBackend<io::Stdout>>,
) {
    let held = app.lock().unwrap();
    // Render the user interface.
    // Handle events.
    match tui.events.next().await.unwrap() {
        Event::Tick => held.tick(),
        Event::Key(key_event) => handle_key_events(key_event, app.clone()).await.unwrap(),
        Event::Mouse(_) => {}
        Event::Resize(_, _) => {}
    }
}

async fn main_loop(app: Arc<Mutex<App>>, mut tui: Tui<CrosstermBackend<io::Stdout>>) {
    loop {
        loop_section(app.clone(), tui.borrow_mut()).await;
    }
    // tui.exit().unwrap();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create an application.
    let app = Arc::new(Mutex::new(App::new()));

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stdout());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui: Tui<CrosstermBackend<io::Stdout>> =
        Tui::new(terminal, events);
    tui.init()?;

    tui.draw(app.clone()).unwrap();

    // Start the main loop.

    main_loop(app.clone(), tui).await;

    // Exit the user interface.
    Ok(())
}
