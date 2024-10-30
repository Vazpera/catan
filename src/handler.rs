use std::sync::{Arc, Mutex, MutexGuard};

use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: Arc<Mutex<App>>) -> AppResult<()> {
    let mut held = app.lock().unwrap();
    match key_event.code {
        // Exit application on `ESC` or `q`
        KeyCode::Esc | KeyCode::Char('q') => {
            held.quit();
        }
        // Exit heldlication on `Ctrl-C`
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                held.quit();
            }
        }
        // Counter handlers
        KeyCode::Right => {
            held.increment_counter();
        }
        KeyCode::Left => {
            held.decrement_counter();
        }
        // Other handlers you could add here.
        _ => {}
    }
    Ok(())
}
