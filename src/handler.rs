use std::sync::{Arc, Mutex};

use crate::app::{App, AppResult};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Handles the key events and updates the state of [`App`].
pub async fn handle_key_events(key_event: KeyEvent, app: Arc<App>) -> AppResult<()> {
    match (key_event.code, *app.is_host.lock().unwrap()) {
        // Exit application on `ESC` or `q`
        (KeyCode::Esc | KeyCode::Char('q'), _) => {
            app.quit();
        }
        // Exit application on `Ctrl-C`
        (KeyCode::Char('c') | KeyCode::Char('C'), _) => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit();
            }
        }
        // Counter handlers
        (KeyCode::Right, _) => {
            app.increment_counter();
        }
        (KeyCode::Left, _) => {
            app.decrement_counter();
        }
        (KeyCode::Up, false) => {
            app.publish_incriment().await;
        }
        // Other handlers you could add here.
        _ => {}
    }
    drop(app);
    Ok(())
}
