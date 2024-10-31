use std::{error, sync::Mutex};

use futures::{
    channel::mpsc::{self, Receiver, Sender},
    SinkExt,
};

use crate::backend::message::Message;
/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    pub running: Mutex<bool>,
    pub counter: Mutex<u8>,
    pub is_host: Mutex<bool>,
    pub queued_instructions: (Sender<Message>, Mutex<Receiver<Message>>),
}

impl Default for App {
    fn default() -> Self {
        let (tx, rx) = mpsc::channel(1024);
        Self {
            running: Mutex::new(true),
            counter: Mutex::new(0),
            is_host: Mutex::new(false),
            queued_instructions: (tx, Mutex::new(rx)),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&self) {
        *self.running.lock().unwrap() = false;
    }

    pub fn increment_counter(&self) {
        let mut cnter = self.counter.lock().unwrap();
        if let Some(res) = cnter.checked_add(1) {
            *cnter = res;
        }
    }

    pub fn decrement_counter(&self) {
        let mut cnter = self.counter.lock().unwrap();
        if let Some(res) = cnter.checked_sub(1) {
            *cnter = res;
        }
    }
    pub async fn publish_incriment(&self) {
        self.queued_instructions.0.clone().send(Message::Incriment).await;
    }
}
